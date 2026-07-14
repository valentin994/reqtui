use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::Modifier,
    widgets::{Block, Borders, Clear, List},
};

use crate::{app::App, theme::THEME};

pub fn render(app: &mut App, frame: &mut Frame) {
    let [added_requests_layout, history_layout, footer_layout] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
            Constraint::Length(1),
        ])
        .areas(frame.area());

    let added_requests_block = Block::new().style(THEME.text).borders(Borders::ALL);

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

    frame.render_widget(Clear, added_requests_layout);
    frame.render_widget(Clear, history_layout);

    frame.render_widget(added_requests_block, added_requests_layout);
    frame.render_stateful_widget(history_list, history_layout, &mut app.history_state);
}
