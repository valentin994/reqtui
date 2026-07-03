use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Flex, Layout},
    style::{Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, Padding, Paragraph},
};
use ratatui_textarea::WrapMode;

use crate::app::{ActiveEditField, App, CurrentScreen};
use crate::theme::THEME;

// TODO: UI revamp
// TODO: moduliraze the UI

pub fn render(app: &mut App, frame: &mut Frame) {
    let [request_layout, hero_layout, footer_layout] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .areas(frame.area());

    let [protocol, request] =
        Layout::horizontal([Constraint::Length(12), Constraint::Percentage(100)])
            .flex(Flex::Start)
            .areas(request_layout);

    let [collection, response, body] = Layout::horizontal([
        Constraint::Length(20),
        Constraint::Percentage(100),
        Constraint::Length(30),
    ])
    .flex(Flex::Start)
    .areas(hero_layout);

    let collection_lines: Vec<String> = app.history.iter().map(|req| format!("{}", req)).collect();
    let history_list = List::new(collection_lines)
        .block(
            Block::default()
                .title("Default")
                .border_style(THEME.text)
                .borders(Borders::ALL),
        )
        .style(THEME.text)
        .highlight_style(Modifier::REVERSED)
        .highlight_symbol("> ");
    frame.render_stateful_widget(history_list, collection, &mut app.history_state);

    let request_type_block = Block::default()
        .border_style(THEME.text)
        .borders(Borders::ALL)
        .padding(Padding::horizontal(2));

    let request_type_paragraph = Paragraph::new(Text::styled(
        format!("{:?}", app.request_type),
        Style::default()
            .bold()
            .fg(THEME.derive_request(app.request_type)),
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
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(THEME.text),
        )
        .scroll((app.scroll_response, 0));

    let [mode, help] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(12), Constraint::Percentage(100)])
        .areas(footer_layout);

    let current_screen_name = app.current_screen.name();
    let current_screen_color = app.current_screen.color();
    let current_screen_help = app.current_screen.help();

    let [centered] = Layout::horizontal([Constraint::Length(12)])
        .flex(Flex::Center)
        .areas(mode);

    let nav = Paragraph::new(Span::styled(
        current_screen_name,
        Style::default().fg(THEME.text).bold(),
    ))
    .alignment(Alignment::Center)
    .bg(current_screen_color);

    let current_screen_help = Paragraph::new(Line::from(current_screen_help)).block(
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

    frame.render_widget(request_type_paragraph, protocol);
    frame.render_widget(url_paragraph, request);
    frame.render_widget(response_paragraph, response);
    frame.render_widget(parsed_json_paragraph, body);
    frame.render_widget(nav, centered);
    frame.render_widget(current_screen_help, help);

    if app.current_screen == CurrentScreen::Editing {
        let url_focus = app.active_edit_field == ActiveEditField::Url;

        let centered_area = frame
            .area()
            .centered(Constraint::Percentage(60), Constraint::Percentage(80));

        let [edit_url, edit_body] = Layout::vertical([Constraint::Length(3), Constraint::Min(3)])
            .flex(Flex::Start)
            .areas(centered_area);

        let url_block = Block::bordered()
            .title("Enter URL")
            .border_style(if url_focus {
                THEME.text
            } else {
                THEME.background
            });

        app.body.set_line_number_style(Style::default());
        app.body.set_wrap_mode(WrapMode::Word);
        frame.render_widget(Clear, centered_area);
        if url_focus {
            app.body.set_cursor_style(Style::default());
        } else {
            app.body
                .set_cursor_style(Style::default().add_modifier(Modifier::REVERSED));
        }
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
        frame.render_widget(paragraph, edit_url);

        if url_focus {
            let x = app.url.visual_cursor().max(scroll) - scroll + 1;
            frame.set_cursor_position((edit_url.x + x as u16, edit_url.y + 1));
        }

        frame.render_widget(&app.body, edit_body);
    }

    if app.current_screen == CurrentScreen::Collection {
        let centered_area = frame
            .area()
            .centered(Constraint::Percentage(60), Constraint::Percentage(60));
        frame.render_widget(Clear, centered_area);

        let [add_collection, edit_body] =
            Layout::vertical([Constraint::Length(3), Constraint::Min(3)])
                .flex(Flex::Start)
                .areas(centered_area);

        let collection_lines: Vec<String> = app
            .collection_store
            .collections
            .iter()
            .map(|collection| format!("{}", collection))
            .collect();

        if !app.collection_store.collections.is_empty() && app.collection_state.selected().is_none()
        {
            app.collection_state.select(Some(0));
        }
        let collection_list = List::new(collection_lines)
            .block(
                Block::default()
                    .border_style(THEME.text)
                    .borders(Borders::ALL),
            )
            .style(THEME.text)
            .highlight_style(Modifier::REVERSED)
            .highlight_symbol("> ");
        frame.render_stateful_widget(collection_list, a, &mut app.collection_state);
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
        let popup = Paragraph::new(format!("{}", app.url)).block(Block::bordered());
        frame.render_widget(Clear, area);
        frame.render_widget(popup, area);
        frame.render_stateful_widget(loading_widget, area, &mut app.throbber_state);
    }
}
