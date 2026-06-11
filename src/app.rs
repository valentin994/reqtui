use ratatui::{style::Color, widgets::ListState};
use ratatui_textarea::TextArea;
use reqwest::Client;
use std::error::Error;
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
            CurrentScreen::Editing => "(esc) cancel, (enter) save value",
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

#[derive(Debug, Default)]
pub struct App {
    pub current_screen: CurrentScreen,
    pub url: Input,
    pub response: String,
    pub request_type: RequestType,
    pub protocol: Protocol,
    pub body: TextArea<'static>,
    pub active_edit_field: ActiveEditField,
    pub should_quit: bool,
    pub client: Client,
    pub history: Vec<Request>,
    pub history_state: ListState,
    pub throbber_state: throbber_widgets_tui::ThrobberState,
    pub loading: bool,
    pub pending_tasks: Option<JoinHandle<Result<String, String>>>,
    pub scroll_response: u16,
}

impl App {
    pub fn quit(&mut self) {
        self.should_quit = true;
    }
    // TODO: do the response in json, prettyprint, better response handler
    // INFO: possible duplication with Request struct and url, right now overkill
    // TODO: add logging
    // TODO: postman collection import
    pub fn send_request(&mut self) -> Result<(), Box<dyn Error>> {
        self.loading = true;
        let request = Request {
            protocol: self.protocol,
            request_type: self.request_type,
            url: self.url.to_string(),
            body: self.body.lines().join("\n"),
        };
        let client = self.client.clone();
        self.history.insert(0, request.clone());
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
    }

    pub fn selected_request(&mut self) -> Option<&Request> {
        self.history_state
            .selected()
            .and_then(|i| self.history.get(i))
    }

    pub fn toggle_active_field(&mut self) {
        match self.active_edit_field {
            ActiveEditField::Url => self.active_edit_field = ActiveEditField::Body,
            ActiveEditField::Body => self.active_edit_field = ActiveEditField::Url,
        }
    }
    // TODO: load testing feature
}
