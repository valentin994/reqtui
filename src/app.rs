use indexmap::IndexSet;
use ratatui::{style::Color, widgets::ListState};
use ratatui_textarea::TextArea;
use reqwest::Client;
use std::error::Error;
use tokio::task::JoinHandle;
use tui_input::Input;

use crate::{
    api::{Protocol, Request, RequestType, ResponseData},
    collections::{Collection, CollectionStore},
    config::{AppConfig, update_config_name},
    theme::THEME,
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum CurrentScreen {
    #[default]
    Main,
    Editing,
    History,
    Collection,
    Error,
    Testing,
}

// TODO: change up the hotkeys and way of selecting request type
impl CurrentScreen {
    pub fn name(self) -> &'static str {
        match self {
            CurrentScreen::Main => "Main",
            CurrentScreen::Editing => "Editing",
            CurrentScreen::History => "History",
            CurrentScreen::Collection => "Collection",
            CurrentScreen::Error => "Error",
            CurrentScreen::Testing => "Testing",
        }
    }

    pub fn color(self) -> Color {
        match self {
            CurrentScreen::Main => THEME.primary,
            CurrentScreen::Editing => THEME.error,
            CurrentScreen::History => THEME.success,
            CurrentScreen::Collection => THEME.accent,
            CurrentScreen::Error => THEME.error,
            CurrentScreen::Testing => THEME.patch,
        }
    }

    pub fn help(self) -> &'static str {
        match self {
            CurrentScreen::Main => {
                "(q) / (esc) quit, (e) edit url, (tab) change request type, (p) change protocol, (enter) send request"
            }
            CurrentScreen::Editing => "(esc) / (alt + s) save value, (tab) switch window",
            CurrentScreen::History => "(esc) cancel, (q) quit, (enter) select request ",
            CurrentScreen::Collection => {
                "(esc) cancel / (q) quit / (enter) add or select collection / (d) delete collection"
            }
            CurrentScreen::Error => "(esc) back to main screen, (q) quit",
            CurrentScreen::Testing => "(esc) back to main screen, (q) quit",
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum ActiveEditField {
    #[default]
    Name,
    Url,
    Body,
}

impl ActiveEditField {
    const VARIANTS: [ActiveEditField; 3] = [
        ActiveEditField::Name,
        ActiveEditField::Url,
        ActiveEditField::Body,
    ];
    pub fn next(self) -> ActiveEditField {
        let idx = ActiveEditField::VARIANTS
            .iter()
            .position(|&r| r == self)
            .unwrap();
        ActiveEditField::VARIANTS[(idx + 1) % ActiveEditField::VARIANTS.len()]
    }
}

#[derive(Debug, Default, PartialEq)]
pub enum ActiveCollectionField {
    #[default]
    CollectionList,
    AddCollection,
}

#[derive(Debug, Default)]
pub struct App {
    pub config: AppConfig,

    pub current_screen: CurrentScreen,
    pub request_name: Input,
    pub url: Input,
    pub response: String,
    pub request_type: RequestType,
    pub protocol: Protocol,
    pub body: TextArea<'static>,
    pub scroll_response: u16,

    pub status_code: u16,
    pub http_version: String,

    pub active_edit_field: ActiveEditField,

    pub client: Client,
    pub pending_tasks: Option<JoinHandle<Result<ResponseData, String>>>,

    pub history: IndexSet<Request>,
    pub history_state: ListState,
    pub throbber_state: throbber_widgets_tui::ThrobberState,
    pub loading: bool,

    pub collection: CollectionStore,
    pub collection_state: ListState,
    pub collection_name: Input,
    pub active_collection_field: ActiveCollectionField,

    pub error: String,
    pub should_quit: bool,
}

impl App {
    pub fn new(config: AppConfig) -> Self {
        App {
            body: TextArea::from(vec!["{}".to_string()]),
            history: CollectionStore::load_collection_to_history(&format!(
                "{}.json",
                config.collection_name
            ))
            .unwrap_or_default(),
            collection: CollectionStore::list_collections().expect("Couldn't load collection"),
            config,
            ..Default::default()
        }
    }
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    // TODO: add logging
    pub fn send_request(&mut self) -> Result<(), Box<dyn Error>> {
        let request = Request {
            name: self.request_name.value().to_string(),
            protocol: self.protocol,
            request_type: self.request_type,
            url: self.url.to_string(),
            body: self.body.lines().join("\n"),
        };

        match request.is_valid() {
            true => {
                self.loading = true;
                let client = self.client.clone();
                let timeout = self.config.request_timeout;
                self.history.insert(request.clone());
                CollectionStore::write_to_collection(
                    self.history.clone(),
                    &self.config.collection_name,
                )?;
                self.pending_tasks = Some(tokio::spawn(async move {
                    request
                        .send(&client, timeout)
                        .await
                        .map_err(|e| e.to_string())
                }));
                Ok(())
            }
            false => {
                self.error = "Invalid url".to_string();
                self.current_screen = CurrentScreen::Error;
                Ok(())
            }
        }
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
                Ok(Ok(resp)) => {
                    self.response = resp.body;
                    self.status_code = resp.status;
                    self.http_version = resp.version;
                }
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
        self.request_name = req.name.into();
        self.request_type = req.request_type;
        self.body = TextArea::from(req.body.lines());
        self.current_screen = CurrentScreen::Main;
    }

    pub fn delete_request(&mut self) -> Result<(), Box<dyn Error>> {
        let Some(req) = self.selected_request().cloned() else {
            return Err("cannot delete request".into());
        };
        self.history.swap_remove(&req);
        CollectionStore::write_to_collection(self.history.clone(), &self.config.collection_name)?;
        Ok(())
    }

    pub fn selected_request(&mut self) -> Option<&Request> {
        self.history_state
            .selected()
            .and_then(|i| self.history.get_index(i))
    }

    pub fn set_collection(&mut self) -> Result<(), Box<dyn Error>> {
        let Some(collection) = self.selected_collection().cloned() else {
            return Err("Could not set collection".into());
        };
        self.config.collection_name = collection.name;
        update_config_name(&self.config.collection_name)?;
        self.history = CollectionStore::load_collection_to_history(&format!(
            "{}.json",
            self.config.collection_name
        ))
        .unwrap_or_default();
        self.collection = CollectionStore::list_collections().expect("Couldnt load collection");
        self.current_screen = CurrentScreen::Main;
        Ok(())
    }

    pub fn selected_collection(&mut self) -> Option<&Collection> {
        self.collection_state
            .selected()
            .and_then(|i| self.collection.collections.get(i))
    }

    pub fn add_collection(&mut self) {
        CollectionStore::add_collection(self.collection_name.to_string())
            .expect("Failed to add collection");
        self.collection_name.value_and_reset();
        self.collection = CollectionStore::list_collections().expect("Couldnt load collection");
        self.active_collection_field = ActiveCollectionField::CollectionList;
    }

    pub fn delete_collection(&mut self) {
        let Some(collection) = self.selected_collection().cloned() else {
            return;
        };
        CollectionStore::delete_collection(collection.name).expect("Failed to delete collection");
        self.collection = CollectionStore::list_collections().expect("Couldn't load collection");
    }

    // TODO: load testing feature
}
