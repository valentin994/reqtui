use std::error::Error;

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
}

impl App {
    pub fn quit(&mut self) {
        self.should_quit = true;
    }
    // TODO: save history of the requests
    // TODO: remove everything that is hardcoded
    // TODO: handler to send a http request
    // TODO: do the response in json
    // TODO: make it able to work for multiple types of requests
    pub async fn send_request(&mut self) -> Result<(), Box<dyn Error>> {
        let body = reqwest::get(format!("http://{}", self.url))
            .await?
            .text()
            .await?;
        self.response = body;
        Ok(())
    }

    // TODO: maybe use another function to print the response, it panics on a lot of requests
    pub fn print_json(&self) -> serde_json::Result<()> {
        println!("hell yeah");
        Ok(())
    }
}
