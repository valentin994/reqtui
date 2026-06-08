use std::error::Error;

use reqwest::Client;
use tui_input::Input;

use crate::api::{Protocol, Request, RequestType};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum CurrentScreen {
    #[default]
    Main,
    Editing,
    History,
}

impl RequestType {
    const VARIANTS: [RequestType; 5] = [
        RequestType::GET,
        RequestType::POST,
        RequestType::DELETE,
        RequestType::PATCH,
        RequestType::PUT,
    ];
    pub fn next(self) -> RequestType {
        let idx = RequestType::VARIANTS
            .iter()
            .position(|&r| r == self)
            .unwrap();
        RequestType::VARIANTS[(idx + 1) % RequestType::VARIANTS.len()]
    }
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
}

impl App {
    pub fn quit(&mut self) {
        self.should_quit = true;
    }
    // TODO: handler to send a http request
    // TODO: do the response in json, prettyprint, better response handler
    // INFO: possible duplication with Request struct and url, request, and protocol
    pub async fn send_request(&mut self) -> Result<(), Box<dyn Error>> {
        let request = Request {
            protocol: self.protocol,
            request_type: self.request_type,
            url: self.url.to_string(),
        };
        self.response = request.send(&self.client).await?;
        self.history.insert(0, request);
        Ok(())
    }
}
