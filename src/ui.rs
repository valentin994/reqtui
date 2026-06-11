use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Flex, Layout},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, Padding, Paragraph},
};

use crate::app::{ActiveEditField, App, CurrentScreen};
use crate::theme::THEME;

// TODO: UI revamp
// TODO: moduliraze the UI
// TODO: new layout
// TODO: scrollbar for response part
// TODO: use a theme
// TODO: update footer with flex layout
// TODO: color of request type

pub fn render(app: &mut App, frame: &mut Frame) {
    // Main chunks of the area that are going to be displayed
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(frame.area());

    let request_layout = Layout::horizontal([Constraint::Length(12), Constraint::Percentage(100)])
        .flex(Flex::Start)
        .split(chunks[0]);

    let request_type_block = Block::default()
        .border_style(THEME.text)
        .borders(Borders::ALL)
        .padding(Padding::horizontal(2));

    let protocol_paragraph = Paragraph::new(Text::styled(
        format!("{:?}", app.request_type),
        Style::default().bold().fg(THEME.secondary),
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

    let response = Paragraph::new(lines)
        .style(Style::default().fg(THEME.text))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(THEME.text),
        )
        .scroll((app.scroll_response, 0));

    let footer_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(12), Constraint::Percentage(100)])
        .split(chunks[2]);

    let current_screen_name = app.current_screen.name();
    let current_screen_color = app.current_screen.color();
    let current_screen_help = app.current_screen.help();

    let [centered] = Layout::horizontal([Constraint::Length(12)])
        .flex(Flex::Center)
        .areas(footer_layout[0]);

    let nav = Paragraph::new(Span::styled(
        current_screen_name,
        Style::default().fg(THEME.text).bold(),
    ))
    .alignment(Alignment::Center)
    .bg(current_screen_color);

    // TODO: do the shadowing to other variables as well, like current_screen_help
    let current_screen_help = Paragraph::new(Line::from(current_screen_help)).block(
        Block::default()
            .bg(THEME.secondary)
            .padding(Padding::left(2)),
    );

    frame.render_widget(protocol_paragraph, request_layout[0]);
    frame.render_widget(url_paragraph, request_layout[1]);
    frame.render_widget(response, chunks[1]);
    frame.render_widget(nav, centered);
    frame.render_widget(current_screen_help, footer_layout[1]);

    if app.current_screen == CurrentScreen::Editing {
        let url_focus = app.active_edit_field == ActiveEditField::Url;

        let centered_area = frame
            .area()
            .centered(Constraint::Percentage(60), Constraint::Percentage(80));

        let editing_layout = Layout::vertical([Constraint::Length(3), Constraint::Min(3)])
            .flex(Flex::Start)
            .split(centered_area);

        let url_block = Block::bordered()
            .title("Enter URL")
            .border_style(if url_focus {
                THEME.text
            } else {
                THEME.background
            });

        frame.render_widget(Clear, centered_area);
        if url_focus {
            app.body.set_cursor_style(Style::default());
        } else {
            app.body
                .set_cursor_style(Style::default().add_modifier(Modifier::REVERSED));
        }
        app.body.set_line_number_style(Style::default());
        app.body.set_block(
            Block::bordered()
                .style(if url_focus {
                    THEME.background
                } else {
                    THEME.text
                })
                .title("Body".to_string()),
        );

        let width = centered_area.width.max(3) - 3;
        let scroll = app.url.visual_scroll(width as usize);
        let paragraph = Paragraph::new(app.url.value())
            .scroll((0, scroll as u16))
            .style(if url_focus {
                THEME.text
            } else {
                THEME.background
            })
            .block(url_block);
        frame.render_widget(paragraph, editing_layout[0]);

        if url_focus {
            let x = app.url.visual_cursor().max(scroll) - scroll + 1;
            frame.set_cursor_position((editing_layout[0].x + x as u16, editing_layout[0].y + 1));
        }

        frame.render_widget(&app.body, editing_layout[1]);
    }
    // TODO: make a new input field in history for search
    // TODO: if the url is not correct make the border red
    if app.current_screen == CurrentScreen::History {
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

    if app.loading {
        let loading_widget = throbber_widgets_tui::Throbber::default()
            .label("Sending request to")
            .style(Style::default().fg(Color::Cyan))
            .throbber_style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
            .throbber_set(throbber_widgets_tui::BRAILLE_SIX);

        let area = frame
            .area()
            .centered(Constraint::Percentage(50), Constraint::Length(3));
        let popup = Paragraph::new(format!("{}", app.url)).block(Block::bordered());
        frame.render_widget(Clear, area);
        frame.render_widget(popup, area);
        frame.render_stateful_widget(loading_widget, area, &mut app.throbber_state);
    }
}
