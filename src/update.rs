use ratatui::crossterm::event::{Event, KeyCode, KeyEvent};
use tui_input::backend::crossterm::EventHandler;

use crate::app::{App, CurrentScreen};

pub fn update(app: &mut App, key_event: KeyEvent) {
    match app.current_screen {
        CurrentScreen::Main => match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => app.quit(),
            KeyCode::Char('e') => app.current_screen = CurrentScreen::Editing,
            _ => {}
        },
        CurrentScreen::Editing => match key_event.code {
            // TODO: on escape don't save URL
            KeyCode::Esc | KeyCode::Enter => app.current_screen = CurrentScreen::Main,
            _ => {
                app.url.handle_event(&Event::Key(key_event));
            }
        },
        CurrentScreen::Exiting => println!("fuck you i am out"),
    }
}
