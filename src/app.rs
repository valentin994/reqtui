use std::error::Error;

use reqwest::Client;
use std::fmt;
use tui_input::Input;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum CurrentScreen {
    #[default]
    Main,
    Editing,
    Exiting,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum RequestType {
    #[default]
    GET,
    POST,
    DELETE,
    PATCH,
    PUT,
}

impl RequestType {
    pub fn next(self) -> RequestType {
        let idx = RequestType::VARIANTS
            .iter()
            .position(|&r| r == self)
            .unwrap();
        RequestType::VARIANTS[(idx + 1) % RequestType::VARIANTS.len()]
    }
}

impl RequestType {
    const VARIANTS: [RequestType; 5] = [
        RequestType::GET,
        RequestType::POST,
        RequestType::DELETE,
        RequestType::PATCH,
        RequestType::PUT,
    ];
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Protocol {
    #[default]
    HTTP,
    HTTPS,
}

impl fmt::Display for Protocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Protocol::HTTP => write!(f, "http"),
            Protocol::HTTPS => write!(f, "https"),
        }
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
}

impl App {
    pub fn quit(&mut self) {
        self.should_quit = true;
    }
    // TODO: save history of the requests
    // TODO: remove everything that is hardcoded
    // TODO: handler to send a http request
    // TODO: do the response in json, better response handler
    pub async fn send_request(&mut self) -> Result<(), Box<dyn Error>> {
        let prepare_request = match self.request_type {
            RequestType::GET => self.client.get(format!("{}://{}", self.protocol, self.url)),
            RequestType::POST => self
                .client
                .post(format!("{}://{}", self.protocol, self.url)),
            RequestType::PUT => self.client.put(format!("{}://{}", self.protocol, self.url)),
            RequestType::PATCH => self
                .client
                .patch(format!("{}://{}", self.protocol, self.url)),
            RequestType::DELETE => self
                .client
                .delete(format!("{}://{}", self.protocol, self.url)),
        };
        let req = prepare_request.send().await?;
        self.response = req.text().await?;
        Ok(())
    }

    // TODO: maybe use another function to print the response, it panics on a lot of requests
    pub fn print_json(&self) -> serde_json::Result<()> {
        println!("hell yeah");
        Ok(())
    }
}
