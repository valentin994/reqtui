use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, Paragraph},
};

use crate::app::{App, CurrentScreen};

// TODO: UI revamp
// TODO: moduliraze the UI
// TODO: new layout
// TODO: scrollbar for response part
// TODO: use a theme
// TODO: update footer with flex layout

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

    let request_contraints = Constraint::from_percentages([10, 90]);
    let request_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(request_contraints)
        .split(chunks[0]);

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let protocol_paragraph = Paragraph::new(Text::styled(
        format!("{:?}", app.request_type),
        Style::default().fg(Color::Green),
    ))
    .block(title_block);

    let url_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let url_paragraph = Paragraph::new(Text::styled(
        format!("{}://{}", app.protocol, app.url),
        Style::default().fg(Color::Cyan),
    ))
    .block(url_block);

    let response =
        Paragraph::new(app.response.to_string()).block(Block::default().borders(Borders::ALL));

    let footer_contraints = Constraint::from_percentages([10, 90]);
    let footer_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(footer_contraints)
        .split(chunks[2]);

    let current_navigation_text = match app.current_screen {
        CurrentScreen::Main => Span::styled("Main", Style::default().fg(Color::Green)),
        CurrentScreen::Editing => Span::styled("Editing", Style::default().fg(Color::Yellow)),
        CurrentScreen::Exiting => Span::styled("Exiting", Style::default().fg(Color::LightRed)),
    }
    .to_owned();

    let navigation_text_paragraph = Paragraph::new(current_navigation_text).block(Block::default());

    let current_key_help = {
        match app.current_screen {
            CurrentScreen::Main => Span::styled(
                "(q) / (esc) quit, (e) edit url, (tab) change request type, (p) change protocol, (enter) send request",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Editing => Span::styled(
                "(esc) to cancel, (enter) to save value",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Exiting => Span::styled(
                "(esc) to cancel, (enter) to save value",
                Style::default().fg(Color::Red),
            ),
        }
    };

    let current_screen_help = Paragraph::new(Line::from(current_key_help)).block(Block::default());

    frame.render_widget(protocol_paragraph, request_layout[0]);
    frame.render_widget(url_paragraph, request_layout[1]);
    frame.render_widget(response, chunks[1]);
    frame.render_widget(navigation_text_paragraph, footer_layout[0]);
    frame.render_widget(current_screen_help, footer_layout[1]);

    // Editing

    if app.current_screen == CurrentScreen::Editing {
        let popup_block = Block::bordered().title("Enter URL");
        let centered_area = frame
            .area()
            .centered(Constraint::Percentage(60), Constraint::Percentage(20));
        frame.render_widget(Clear, centered_area);

        let width = centered_area.width.max(3) - 3;
        let scroll = app.url.visual_scroll(width as usize);
        // clears out any background in the area before rendering the popup
        let paragraph = Paragraph::new(app.url.value())
            .scroll((0, scroll as u16))
            .block(popup_block);
        frame.render_widget(paragraph, centered_area);
        let x = app.url.visual_cursor().max(scroll) - scroll + 1;
        frame.set_cursor_position((centered_area.x + x as u16, centered_area.y + 1));
    }
}
