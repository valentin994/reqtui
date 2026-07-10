use ratatui::{
    Frame,
    layout::Constraint,
    style::Modifier,
    widgets::{Block, Borders, Clear, List},
};

use crate::{app::App, theme::THEME};

// TODO: make a new input field in history for search
pub fn render(app: &mut App, frame: &mut Frame) {
    let centered_area = frame
        .area()
        .centered(Constraint::Percentage(60), Constraint::Percentage(60));
    frame.render_widget(Clear, centered_area);

    if !app.history.is_empty() && app.history_state.selected().is_none() {
        app.history_state.select(Some(0));
    }

    let lines: Vec<String> = app.history.iter().map(|req| format!("{}", req)).collect();
    let history_list = List::new(lines)
        .block(
            Block::default()
                .border_style(THEME.text)
                .borders(Borders::ALL),
        )
        .style(THEME.text)
        .highlight_style(Modifier::REVERSED)
        .highlight_symbol("> ");
    frame.render_stateful_widget(history_list, centered_area, &mut app.history_state);
}
