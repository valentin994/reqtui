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

    pub fn start_updating_url(&mut self) {
        self.current_screen = CurrentScreen::Editing;
    }

    pub fn end_updating_url(&mut self) {
        self.current_screen = CurrentScreen::Main;
    }

    // TODO: handler to send a htt prequest
    pub fn send_request() {}

    pub fn print_json(&self) -> serde_json::Result<()> {
        println!("hell yeah");
        Ok(())
    }
}
