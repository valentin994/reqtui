use directories::ProjectDirs;
use indexmap::IndexSet;
use ratatui::{style::Color, widgets::ListState};
use ratatui_textarea::TextArea;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{error::Error, fs, path::PathBuf};
use tokio::task::JoinHandle;
use tui_input::Input;

use crate::{
    api::{Protocol, Request, RequestType},
    theme::THEME,
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum CurrentScreen {
    #[default]
    Main,
    Editing,
    History,
}

impl CurrentScreen {
    pub fn name(self) -> &'static str {
        match self {
            CurrentScreen::Main => "Main",
            CurrentScreen::Editing => "Editing",
            CurrentScreen::History => "History",
        }
    }

    pub fn color(self) -> Color {
        match self {
            CurrentScreen::Main => THEME.primary,
            CurrentScreen::Editing => THEME.error,
            CurrentScreen::History => THEME.success,
        }
    }

    // TODO: update the help info
    pub fn help(self) -> &'static str {
        match self {
            CurrentScreen::Main => {
                "(q) / (esc) quit, (e) edit url, (tab) change request type, (p) change protocol, (enter) send request"
            }
            CurrentScreen::Editing => "(esc) / (alt + s) save value",
            CurrentScreen::History => "(esc) cancel, (q) quit",
        }
    }
}

#[derive(Debug, Default, PartialEq)]
pub enum ActiveEditField {
    #[default]
    Url,
    Body,
}

// TODO: file search for postman collections
// TODO: change up the hotkeys and way of selecting request type

// TODO: write to json
// TODO: make it possible to select collections
#[derive(Debug, Default)]
pub struct CollectionStore {
    collections: Vec<Collection>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Collection {
    pub name: String,
    pub requests: Vec<Request>,
}

impl CollectionStore {
    fn get_config_path() -> Option<PathBuf> {
        let proj = ProjectDirs::from("com", "you", "reqtui")?;
        Some(proj.config_dir().join("collections"))
    }

    fn load_collection_to_history() -> Result<IndexSet<Request>, Box<dyn Error>> {
        let Some(file_path) = Self::get_config_path() else {
            return Ok(IndexSet::new());
        };
        // INFO: hardcoded default for now, after adding support to multiple collections rework
        let file = fs::read_to_string(file_path.join("default.json"))?;
        let collection: Collection = serde_json::from_str(&file)?;
        let history = collection.requests.into_iter().collect();
        Ok(history)
    }

    fn write_to_collection(history: IndexSet<Request>) -> Result<(), Box<dyn Error>> {
        let Some(file_path) = Self::get_config_path() else {
            return Err("could not find collection".into());
        };
        // INFO: hardcoded default for now, after adding support to multiple collections rework
        let full_path = file_path.join("default.json");
        let file = fs::read_to_string(&full_path)?;
        let mut collection: Collection = serde_json::from_str(&file)?;
        collection.requests = history.iter().cloned().collect();
        fs::write(full_path, serde_json::to_string_pretty(&collection)?)?;
        Ok(())
    }

    fn list_collections() -> Self {
        let Some(config_path) = Self::get_config_path() else {
            return Self {
                collections: vec![],
            };
        };

        if !config_path.exists() {
            if let Err(_e) = fs::create_dir_all(&config_path) {
                return Self {
                    collections: vec![],
                };
            }
        }

        let default_file = config_path.join("default.json");
        if !default_file.exists() {
            let default_file_content = r#"{
                "name": "Default",
                "requests": []
            }"#;

            let _ = fs::write(&default_file, default_file_content);
        }

        let mut collections = Vec::new();

        match fs::read_dir(&config_path) {
            Ok(entries) => {
                for entry in entries.into_iter().flatten() {
                    let path = entry.path();

                    if path.is_file() && path.extension().map_or(false, |ext| ext == "json") {
                        if let Some(name) = path.file_stem().and_then(|n| n.to_str()) {
                            collections.push(Collection {
                                name: name.to_string(),
                                requests: vec![],
                            });
                        }
                    }
                }
            }
            Err(_) => {}
        }

        Self { collections }
    }
}

#[derive(Debug, Default)]
pub struct App {
    pub current_screen: CurrentScreen,
    pub url: Input,
    pub response: String,
    pub request_type: RequestType,
    pub protocol: Protocol,
    pub body: TextArea<'static>,
    pub scroll_response: u16,
    pub active_edit_field: ActiveEditField,

    pub client: Client,
    pub pending_tasks: Option<JoinHandle<Result<String, String>>>,

    pub history: IndexSet<Request>,
    pub history_state: ListState,
    pub throbber_state: throbber_widgets_tui::ThrobberState,
    pub loading: bool,

    pub collections: CollectionStore,

    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        App {
            body: TextArea::from(vec!["{}".to_string()]),
            history: CollectionStore::load_collection_to_history().unwrap_or(IndexSet::new()),
            collections: CollectionStore::list_collections(),
            ..Default::default()
        }
    }
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    // INFO: possible duplication with Request struct and url, right now overkill
    // TODO: add logging
    pub fn send_request(&mut self) -> Result<(), Box<dyn Error>> {
        self.loading = true;
        let request = Request {
            name: "Untitled".to_string(),
            protocol: self.protocol,
            request_type: self.request_type,
            url: self.url.to_string(),
            body: self.body.lines().join("\n"),
        };
        let client = self.client.clone();
        self.history.insert(request.clone());
        CollectionStore::write_to_collection(self.history.clone());
        self.pending_tasks = Some(tokio::spawn(async move {
            request.send(&client).await.map_err(|e| e.to_string())
        }));
        Ok(())
    }

    pub fn poll_requests(&mut self) {
        let finished = self
            .pending_tasks
            .as_ref()
            .map(|f| f.is_finished())
            .unwrap_or(false);

        if finished {
            let handle = self.pending_tasks.take().unwrap();

            match futures::executor::block_on(handle) {
                Ok(Ok(body)) => self.response = body,
                Ok(Err(e)) => self.response = e,
                Err(_) => self.response = "Request couldn't be executed".to_string(),
            }
            self.loading = false;
        }
    }

    pub fn set_request(&mut self) {
        let Some(req) = self.selected_request().cloned() else {
            return;
        };
        self.url = Input::new(req.url);
        self.protocol = req.protocol;
        self.request_type = req.request_type;
        self.body = TextArea::from(req.body.lines());
    }

    pub fn selected_request(&mut self) -> Option<&Request> {
        self.history_state
            .selected()
            .and_then(|i| self.history.get_index(i))
    }

    pub fn toggle_active_field(&mut self) {
        match self.active_edit_field {
            ActiveEditField::Url => self.active_edit_field = ActiveEditField::Body,
            ActiveEditField::Body => self.active_edit_field = ActiveEditField::Url,
        }
    }
    // TODO: load testing feature
}
