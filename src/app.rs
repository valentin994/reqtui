use std::error::Error;

use tui_input::Input;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum CurrentScreen {
    #[default]
    Main,
    Editing,
    Exiting,
}

#[derive(Debug, Default)]
pub struct App {
    pub current_screen: CurrentScreen,
    pub url: Input,
    pub response: String,
    pub https: bool,
    pub should_quit: bool,
}

impl App {
    pub fn quit(&mut self) {
        self.should_quit = true;
    }
    // TODO: save history of the requests

    // TODO: handler to send a http request
    // TODO: do the response in json
    // TODO: make it able to work for multiple types of requests
    pub async fn send_request(&mut self) -> Result<(), Box<dyn Error>> {
        let body = reqwest::get(format!("https://{}", self.url))
            .await?
            .text()
            .await?;
        self.response = body;
        Ok(())
    }

    pub fn print_json(&self) -> serde_json::Result<()> {
        println!("hell yeah");
        Ok(())
    }
}
