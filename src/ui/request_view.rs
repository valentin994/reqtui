use ratatui::{
    Frame,
    layout::{Constraint, Flex, Layout},
    style::{Modifier, Style},
    widgets::{Block, Clear, Paragraph},
};
use ratatui_textarea::WrapMode;

use crate::{
    app::{ActiveEditField, App},
    theme::THEME,
};

pub fn render(app: &mut App, frame: &mut Frame) {
    let centered_area = frame
        .area()
        .centered(Constraint::Percentage(60), Constraint::Percentage(80));

    let [edit_name, edit_url, edit_body] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Min(3),
    ])
    .flex(Flex::Start)
    .areas(centered_area);

    let request_name_block = Block::bordered().title("Enter request name").border_style(
        if app.active_edit_field == ActiveEditField::Name {
            THEME.text
        } else {
            THEME.background
        },
    );

    let url_block = Block::bordered().title("Enter URL").border_style(
        if app.active_edit_field == ActiveEditField::Url {
            THEME.text
        } else {
            THEME.background
        },
    );

    app.body.set_line_number_style(Style::default());
    app.body.set_wrap_mode(WrapMode::Word);
    frame.render_widget(Clear, centered_area);

    if app.active_edit_field == ActiveEditField::Name
        || app.active_edit_field == ActiveEditField::Url
    {
        app.body.set_cursor_style(Style::default());
    } else {
        app.body
            .set_cursor_style(Style::default().add_modifier(Modifier::REVERSED));
    }

    app.body.set_block(
        Block::bordered()
            .style(if app.active_edit_field == ActiveEditField::Body {
                THEME.text
            } else {
                THEME.background
            })
            .title("Body".to_string()),
    );

    let width = centered_area.width.max(3) - 3;

    let request_name_scroll = app.request_name.visual_scroll(width as usize);
    let scroll = app.url.visual_scroll(width as usize);

    let request_name_paragraph = Paragraph::new(app.request_name.value())
        .scroll((0, request_name_scroll as u16))
        .style(if app.active_edit_field == ActiveEditField::Name {
            THEME.text
        } else {
            THEME.background
        })
        .block(request_name_block);

    let url_paragraph = Paragraph::new(app.url.value())
        .scroll((0, scroll as u16))
        .style(if app.active_edit_field == ActiveEditField::Url {
            THEME.text
        } else {
            THEME.background
        })
        .block(url_block);

    frame.render_widget(request_name_paragraph, edit_name);
    frame.render_widget(url_paragraph, edit_url);

    if app.active_edit_field == ActiveEditField::Url {
        let x = app.url.visual_cursor().max(scroll) - scroll + 1;
        frame.set_cursor_position((edit_url.x + x as u16, edit_url.y + 1));
    } else if app.active_edit_field == ActiveEditField::Name {
        let x = app.request_name.visual_cursor().max(request_name_scroll) - request_name_scroll + 1;
        frame.set_cursor_position((edit_name.x + x as u16, edit_name.y + 1));
    }

    frame.render_widget(&app.body, edit_body);
}
