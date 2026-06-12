pub mod api;
pub mod app;
pub mod event;
pub mod theme;
pub mod tui;
pub mod ui;
pub mod update;

use std::{error::Error, io};

use event::{Event, EventHandler};
use ratatui::{Terminal, backend::CrosstermBackend};
use tui::Tui;
use update::update;

use crate::app::App;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    let mut app = App::new();

    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(100);

    // create app and run it
    let mut tui = Tui::new(terminal, events);

    tui.enter()?;
    while !app.should_quit {
        tui.draw(&mut app)?;

        match tui.events.next()? {
            Event::Tick => {
                if app.loading {
                    app.throbber_state.calc_next();
                }
                app.poll_requests();
            }
            Event::Key(key_event) => update(&mut app, key_event).await,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }
    tui.exit()?;
    Ok(())
}
