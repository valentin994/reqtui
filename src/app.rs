use reqwest::Client;
use std::error::Error;
use tokio::task::JoinHandle;
use tui_input::Input;

use crate::api::{Protocol, Request, RequestType};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum CurrentScreen {
    #[default]
    Main,
    Editing,
    History,
}

#[derive(Debug, Default)]
pub struct App {
    pub current_screen: CurrentScreen,
    pub url: Input,
    pub response: String,
    pub request_type: RequestType,
    pub protocol: Protocol,
    pub should_quit: bool,
    pub client: Client,
    pub history: Vec<Request>,
    pub throbber_state: throbber_widgets_tui::ThrobberState,
    pub loading: bool,
    pub pending_tasks: Option<JoinHandle<Result<String, String>>>,
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
    // TODO: load testing feature
}
