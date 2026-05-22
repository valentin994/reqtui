use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Clear, Paragraph},
};

use crate::app::App;

pub fn render(app: &mut App, frame: &mut Frame) {
    // Chunks of the area that are going to be displayed
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let contraints = Constraint::from_percentages([10, 70, 20]);
    let request = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(contraints)
        .split(chunks[0]);

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title =
        Paragraph::new(Text::styled("GET", Style::default().fg(Color::Green))).block(title_block);

    let search_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let protocol = match app.https {
        true => "https",
        false => "http",
    };

    let divider = Paragraph::new(Text::styled(
        format!("{}://{}", protocol, app.url),
        Style::default().fg(Color::Cyan),
    ))
    .block(search_block);

    frame.render_widget(title, request[0]);
    frame.render_widget(divider, request[1]);

    // Editing

    if app.currently_editing {
        let popup_block = Block::bordered().title("Popup");
        let centered_area = frame
            .area()
            .centered(Constraint::Percentage(60), Constraint::Percentage(20));
        // clears out any background in the area before rendering the popup
        frame.render_widget(Clear, centered_area);
        let paragraph = Paragraph::new("Lorem ipsum").block(popup_block);
        frame.render_widget(paragraph, centered_area);
    }
}
