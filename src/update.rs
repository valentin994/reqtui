use ratatui::crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use tui_input::backend::crossterm::EventHandler;

use crate::api::Protocol;
use crate::app::{ActiveCollectionField, ActiveEditField, App, CurrentScreen};
use crate::config::CollectionStore;

pub async fn update(app: &mut App, key_event: KeyEvent) {
    match app.current_screen {
        CurrentScreen::Main => match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => app.quit(),
            KeyCode::Char('e') => app.current_screen = CurrentScreen::Editing,
            KeyCode::Char('h') => app.current_screen = CurrentScreen::History,
            KeyCode::Char('c') => app.current_screen = CurrentScreen::Collection,
            // INFO:if I implement more protocols see RequestType iterator example for cycling
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
            KeyCode::Tab => app.request_type = app.request_type.next(),
            KeyCode::Enter => app.send_request().expect("failed to send request"),
            _ => {}
        },
        CurrentScreen::Editing => match key_event.code {
            // TODO: on escape don't save URL
            KeyCode::Tab => app.toggle_active_field(),
            KeyCode::Esc => app.current_screen = CurrentScreen::Main,
            KeyCode::Char('s') if key_event.modifiers.contains(KeyModifiers::ALT) => {
                app.current_screen = CurrentScreen::Main
            }
            _ => match app.active_edit_field {
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
            KeyCode::Up | KeyCode::Char('k') => app.history_state.select_previous(),
            KeyCode::Down | KeyCode::Char('j') => app.history_state.select_next(),
            KeyCode::Enter => {
                app.set_request();
                app.current_screen = CurrentScreen::Main;
            }
            _ => {}
        },
        CurrentScreen::Collection => match key_event.code {
            KeyCode::Tab => app.toggle_active_field(),
            KeyCode::Esc => app.current_screen = CurrentScreen::Main,
            KeyCode::Up | KeyCode::Char('k')
                if app.active_collection_field == ActiveCollectionField::CollectionList =>
            {
                app.collection_state.select_previous()
            }
            KeyCode::Down | KeyCode::Char('j')
                if app.active_collection_field == ActiveCollectionField::CollectionList =>
            {
                app.collection_state.select_next()
            }
            KeyCode::Enter => match app.active_collection_field {
                ActiveCollectionField::CollectionList => todo!(),
                ActiveCollectionField::AddCollection => {
                    CollectionStore::add_collection(app.collection_name.to_string())
                        .expect("Failed to add collection")
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
