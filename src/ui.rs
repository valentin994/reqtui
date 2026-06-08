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
            Constraint::Length(1),
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

    // WARNING: not sure how to keep this, should the app.response be other type than string?
    // maybe there should be a part where you can select your data type and parse pased on that,
    // like key value view, idk
    let pretty_json = serde_json::from_str::<serde_json::Value>(&app.response)
        .map(|v| serde_json::to_string_pretty(&v).unwrap())
        .unwrap_or_else(|_| app.response.clone()); // fallback to raw if not valid JSON

    let lines: Vec<Line> = pretty_json
        .lines()
        .map(|line| Line::from(line.to_owned()))
        .collect();
    let response = Paragraph::new(lines).block(Block::default().borders(Borders::ALL));

    let footer_contraints = Constraint::from_percentages([10, 90]);
    let footer_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(footer_contraints)
        .split(chunks[2]);

    let current_navigation_text = match app.current_screen {
        CurrentScreen::Main => Span::styled("Main", Style::default().fg(Color::Green)),
        CurrentScreen::Editing => Span::styled("Editing", Style::default().fg(Color::Yellow)),
        CurrentScreen::History => Span::styled("History", Style::default().fg(Color::Blue)),
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
                "(esc) cancel, (enter) save value",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::History => {
                Span::styled("(esc) cancel, (q) quit", Style::default().fg(Color::Red))
            }
        }
    };

    let current_screen_help = Paragraph::new(Line::from(current_key_help)).block(Block::default());

    frame.render_widget(protocol_paragraph, request_layout[0]);
    frame.render_widget(url_paragraph, request_layout[1]);
    frame.render_widget(response, chunks[1]);
    frame.render_widget(navigation_text_paragraph, footer_layout[0]);
    frame.render_widget(current_screen_help, footer_layout[1]);

    // WARNING: should refactor, the popup block can be shared
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
    // WARNING: should refactor
    if app.current_screen == CurrentScreen::History {
        let popup_block = Block::bordered().title("Last requests");
        let centered_area = frame
            .area()
            .centered(Constraint::Percentage(60), Constraint::Percentage(20));
        frame.render_widget(Clear, centered_area);

        let width = centered_area.width.max(3) - 3;
        let scroll = app.url.visual_scroll(width as usize);
        // clears out any background in the area before rendering the popup
        let lines: Vec<String> = app.history.iter().map(|req| format!("{:?}", req)).collect();
        let paragraph = Paragraph::new(lines.join("\n"))
            .scroll((0, scroll as u16))
            .block(popup_block);
        frame.render_widget(paragraph, centered_area);
    }
}
