use indexmap::IndexSet;
use ratatui::{style::Color, widgets::ListState};
use ratatui_textarea::TextArea;
use reqwest::Client;
use std::error::Error;
use tokio::task::JoinHandle;
use tui_input::Input;

use crate::{
    api::{Protocol, Request, RequestType},
    config::CollectionStore,
    theme::THEME,
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum CurrentScreen {
    #[default]
    Main,
    Editing,
    History,
    Collection,
}

// TODO: change up the hotkeys and way of selecting request type
impl CurrentScreen {
    pub fn name(self) -> &'static str {
        match self {
            CurrentScreen::Main => "Main",
            CurrentScreen::Editing => "Editing",
            CurrentScreen::History => "History",
            CurrentScreen::Collection => "Collection",
        }
    }

    pub fn color(self) -> Color {
        match self {
            CurrentScreen::Main => THEME.primary,
            CurrentScreen::Editing => THEME.error,
            CurrentScreen::History => THEME.success,
            CurrentScreen::Collection => THEME.accent,
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
            CurrentScreen::Collection => "(esc) cancel",
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
    pub current_screen: CurrentScreen,
    pub request_name: Input,
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

    pub collection_store: CollectionStore,
    pub collection_state: ListState,
    pub collection_name: Input,
    pub active_collection_field: ActiveCollectionField,

    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        App {
            body: TextArea::from(vec!["{}".to_string()]),
            history: CollectionStore::load_collection_to_history("default.json".to_string())
                .unwrap_or_default(),
            collection_store: CollectionStore::list_collections(),
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
            name: self.request_name.value().to_string(),
            protocol: self.protocol,
            request_type: self.request_type,
            url: self.url.to_string(),
            body: self.body.lines().join("\n"),
        };
        let client = self.client.clone();
        self.history.insert(request.clone());
        CollectionStore::write_to_collection(self.history.clone())?;
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

    // TODO: load testing feature
}
