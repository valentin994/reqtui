use ratatui::{
    Frame,
    layout::Constraint,
    widgets::{Block, Borders, Clear, Paragraph},
};

use crate::{app::App, theme::THEME};

pub fn render(app: &mut App, frame: &mut Frame) {
    let centered_area = frame
        .area()
        .centered(Constraint::Percentage(60), Constraint::Length(3));
    frame.render_widget(Clear, centered_area);

    let error_block = Block::new().borders(Borders::ALL).style(THEME.error);
    let error_paragraph = Paragraph::new(format!("{}", app.error))
        .centered()
        .block(error_block);

    frame.render_widget(error_paragraph, centered_area);
}
