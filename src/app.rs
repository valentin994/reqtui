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
    pub https: bool,
    pub currently_editing: bool,
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

    pub fn print_json(&self) -> serde_json::Result<()> {
        println!("hell yeah");
        Ok(())
    }
}
