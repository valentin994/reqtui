use ratatui::crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use tui_input::backend::crossterm::EventHandler;

use crate::api::Protocol;
use crate::app::{ActiveCollectionField, ActiveEditField, App, CurrentScreen};

pub async fn update(app: &mut App, key_event: KeyEvent) {
    match app.current_screen {
        CurrentScreen::Main => match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => app.quit(),
            KeyCode::Char('e') => app.current_screen = CurrentScreen::Editing,
            KeyCode::Char('h') => app.current_screen = CurrentScreen::History,
            KeyCode::Char('c') => app.current_screen = CurrentScreen::Collection,
            KeyCode::Char('p') => {
                app.protocol = match app.protocol {
                    Protocol::HTTP => Protocol::HTTPS,
                    Protocol::HTTPS => Protocol::HTTP,
                }
            }
            KeyCode::Up | KeyCode::Char('k') => {
                app.scroll_response = app.scroll_response.saturating_sub(1)
            }
            KeyCode::Down | KeyCode::Char('j') => {
                app.scroll_response = app.scroll_response.saturating_add(1)
            }
            KeyCode::BackTab => app.request_type = app.request_type.previous(),
            KeyCode::Tab => app.request_type = app.request_type.next(),
            KeyCode::Enter => app.send_request().expect("failed to send request"),
            _ => {}
        },
        CurrentScreen::Editing => match key_event.code {
            KeyCode::Tab => app.active_edit_field = app.active_edit_field.next(),
            KeyCode::Esc => app.current_screen = CurrentScreen::Main,
            KeyCode::Char('s') if key_event.modifiers.contains(KeyModifiers::ALT) => {
                app.current_screen = CurrentScreen::Main
            }
            _ => match app.active_edit_field {
                ActiveEditField::Name => {
                    app.request_name.handle_event(&Event::Key(key_event));
                }
                ActiveEditField::Url => {
                    app.url.handle_event(&Event::Key(key_event));
                }
                ActiveEditField::Body => {
                    app.body.input(key_event);
                }
            },
        },
        CurrentScreen::History => match key_event.code {
            KeyCode::Esc => app.current_screen = CurrentScreen::Main,
            KeyCode::Char('q') => app.quit(),
            KeyCode::Char('d') => app.delete_request().expect("unable to delete request"),
            KeyCode::Up | KeyCode::Char('k') => app.history_state.select_previous(),
            KeyCode::Down | KeyCode::Char('j') => app.history_state.select_next(),
            KeyCode::Enter => {
                app.set_request();
            }
            _ => {}
        },
        CurrentScreen::Collection => match key_event.code {
            KeyCode::Tab => {
                app.active_collection_field = match app.active_collection_field {
                    ActiveCollectionField::AddCollection => ActiveCollectionField::CollectionList,
                    ActiveCollectionField::CollectionList => ActiveCollectionField::AddCollection,
                }
            }
            KeyCode::Esc => app.current_screen = CurrentScreen::Main,
            KeyCode::Up | KeyCode::Char('k')
                if app.active_collection_field == ActiveCollectionField::CollectionList =>
            {
                app.collection_state.select_previous()
            }
            KeyCode::Char('d')
                if app.active_collection_field == ActiveCollectionField::CollectionList =>
            {
                app.delete_collection();
            }
            KeyCode::Char('q')
                if app.active_collection_field == ActiveCollectionField::CollectionList =>
            {
                app.quit();
            }
            KeyCode::Down | KeyCode::Char('j')
                if app.active_collection_field == ActiveCollectionField::CollectionList =>
            {
                app.collection_state.select_next()
            }
            KeyCode::Enter => match app.active_collection_field {
                ActiveCollectionField::CollectionList => {
                    let _ = app.set_collection();
                }
                ActiveCollectionField::AddCollection => {
                    app.add_collection();
                }
            },
            _ => match app.active_collection_field {
                ActiveCollectionField::AddCollection => {
                    app.collection_name.handle_event(&Event::Key(key_event));
                }
                ActiveCollectionField::CollectionList => {}
            },
        },
    }
}
