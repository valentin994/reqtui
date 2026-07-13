use ratatui::{
    Frame,
    layout::Constraint,
    style::{Modifier, Style},
    widgets::{Block, Clear, Paragraph},
};

use crate::{app::App, theme::THEME};

pub fn render(app: &mut App, frame: &mut Frame) {
    let loading_widget = throbber_widgets_tui::Throbber::default()
        .label("Sending request to")
        .style(Style::default().fg(THEME.text))
        .throbber_style(
            Style::default()
                .fg(THEME.primary)
                .add_modifier(Modifier::BOLD),
        )
        .throbber_set(throbber_widgets_tui::BRAILLE_SIX);

    let area = frame
        .area()
        .centered(Constraint::Percentage(50), Constraint::Length(3));
    let popup_paragraph = Paragraph::new(format!("{}", app.url)).block(Block::bordered());
    frame.render_widget(Clear, area);
    frame.render_widget(popup_paragraph, area);
    frame.render_stateful_widget(loading_widget, area, &mut app.throbber_state);
}
