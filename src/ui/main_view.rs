use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Flex, Layout},
    prelude::Stylize,
    style::{Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, Padding, Paragraph},
};

use crate::{app::App, theme::THEME};

pub fn render(app: &mut App, frame: &mut Frame) {
    let [request_layout, hero_layout, footer_layout] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .areas(frame.area());

    let [protocol_layout, request_layout] =
        Layout::horizontal([Constraint::Length(12), Constraint::Fill(100)])
            .flex(Flex::Start)
            .areas(request_layout);

    let [collection_layout, response_layout, body_layout] = Layout::horizontal([
        Constraint::Length(20),
        Constraint::Percentage(100),
        Constraint::Length(30),
    ])
    .flex(Flex::Start)
    .areas(hero_layout);

    let response_layout_block = Block::new().borders(Borders::ALL).style(THEME.text);

    let inner_area = response_layout_block.inner(response_layout);

    let [response_details_layout, response_body_layout] =
        Layout::vertical([Constraint::Length(2), Constraint::Fill(1)]).areas(inner_area);

    let response_details_block = Block::new();

    let response_details_paragraph = if app.status_code != 0 {
        Paragraph::new(format!(
            "Status {}, Version: {}",
            app.status_code, app.http_version
        ))
        .block(response_details_block)
        .style(Style::default())
        .fg(THEME.secondary)
        .add_modifier(Modifier::BOLD)
    } else {
        Paragraph::new("").block(response_details_block)
    };
    let collection_lines: Vec<String> = app.history.iter().map(|req| format!("{}", req)).collect();
    let history_list = List::new(collection_lines)
        .block(
            Block::default()
                .title(app.config.collection_name.clone())
                .border_style(THEME.text)
                .borders(Borders::ALL),
        )
        .style(THEME.text)
        .highlight_style(Modifier::REVERSED)
        .highlight_symbol("> ");
    frame.render_stateful_widget(history_list, collection_layout, &mut app.history_state);

    let request_type_block = Block::default()
        .border_style(THEME.text)
        .borders(Borders::ALL)
        .padding(Padding::horizontal(2));

    let request_type_paragraph = Paragraph::new(Text::styled(
        format!("{:?}", app.request_type),
        Style::default()
            .fg(THEME.derive_request(app.request_type))
            .bold(),
    ))
    .block(request_type_block);

    let url_block = Block::default()
        .borders(Borders::ALL)
        .padding(Padding::left(1))
        .border_style(THEME.text)
        .style(Style::default());

    let url_paragraph = Paragraph::new(Text::styled(
        format!("{}://{}", app.protocol, app.url),
        Style::default().fg(THEME.text),
    ))
    .block(url_block);

    let pretty_json = serde_json::from_str::<serde_json::Value>(&app.response)
        .map(|v| serde_json::to_string_pretty(&v).unwrap())
        .unwrap_or_else(|_| app.response.clone()); // fallback to raw if not valid JSON

    let lines: Vec<Line> = pretty_json
        .lines()
        .map(|line| Line::from(line.to_owned()))
        .collect();

    let response_paragraph = Paragraph::new(lines)
        .style(Style::default().fg(THEME.text))
        .scroll((app.scroll_response, 0));

    let [mode_layout, help_layout] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(12), Constraint::Percentage(100)])
        .areas(footer_layout);

    let current_screen_name = app.current_screen.name();
    let current_screen_color = app.current_screen.color();
    let current_screen_help = app.current_screen.help();

    let [centered_layout] = Layout::horizontal([Constraint::Length(12)])
        .flex(Flex::Center)
        .areas(mode_layout);

    let navigation_paragraph = Paragraph::new(Span::styled(
        current_screen_name,
        Style::default().fg(THEME.text).bold(),
    ))
    .alignment(Alignment::Center)
    .bg(current_screen_color);

    let current_screen_help_paragraph = Paragraph::new(Line::from(current_screen_help)).block(
        Block::default()
            .bg(THEME.secondary)
            .padding(Padding::left(2)),
    );

    let parsed_body = serde_json::from_str::<serde_json::Value>(&app.body.lines().join("\n"))
        .map(|v| serde_json::to_string_pretty(&v).unwrap())
        .unwrap_or_else(|_| "Invalid json".to_string()); // fallback to raw if not valid JSON

    let parsed_json_paragraph = Paragraph::new(format!("Body:\n\n{parsed_body}")).block(
        Block::default()
            .borders(Borders::ALL)
            .style(THEME.derive_body(app.request_type)),
    );

    frame.render_widget(request_type_paragraph, protocol_layout);
    frame.render_widget(url_paragraph, request_layout);
    frame.render_widget(response_layout_block, response_layout);
    frame.render_widget(response_paragraph, response_body_layout);
    frame.render_widget(parsed_json_paragraph, body_layout);
    frame.render_widget(response_details_paragraph, response_details_layout);
    frame.render_widget(navigation_paragraph, centered_layout);
    frame.render_widget(current_screen_help_paragraph, help_layout);
}
