pub mod collection_view;
pub mod error_view;
pub mod history_view;
pub mod loading_view;
pub mod main_view;
pub mod request_view;
pub mod testing_view;

use ratatui::Frame;

use crate::app::{App, CurrentScreen};

// TODO: check the naming, make a naming convention

pub fn render(app: &mut App, frame: &mut Frame) {
    main_view::render(app, frame);
    match app.current_screen {
        CurrentScreen::Editing => request_view::render(app, frame),
        CurrentScreen::History => history_view::render(app, frame),
        CurrentScreen::Collection => collection_view::render(app, frame),
        CurrentScreen::Error => error_view::render(app, frame),
        CurrentScreen::Testing => testing_view::render(app, frame),
        _ => {}
    }

    if app.loading {
        loading_view::render(app, frame);
    }
}
