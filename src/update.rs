use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::app::{App, CurrentScreen};

pub fn update(app: &mut App, key_event: KeyEvent) {
    match app.current_screen {
        CurrentScreen::Main => match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => app.quit(),
            _ => {}
        },
        CurrentScreen::Editing => match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => app.quit(),
            _ => {}
        },
        CurrentScreen::Exiting => println!("fuck you i am out"),
    }
}
