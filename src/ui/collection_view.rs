use ratatui::{
    Frame,
    layout::{Constraint, Flex, Layout},
    style::Modifier,
    widgets::{Block, Borders, Clear, List, Padding, Paragraph},
};

use crate::{
    app::{ActiveCollectionField, App},
    theme::THEME,
};

pub fn render(app: &mut App, frame: &mut Frame) {
    let centered_area = frame
        .area()
        .centered(Constraint::Percentage(60), Constraint::Percentage(60));
    frame.render_widget(Clear, centered_area);

    let collection_focus = app.active_collection_field == ActiveCollectionField::CollectionList;

    let [collection_list_layout, add_to_collection_layout] =
        Layout::vertical([Constraint::Min(3), Constraint::Length(3)])
            .flex(Flex::Start)
            .areas(centered_area);

    let collection_lines: Vec<String> = app
        .collection
        .collections
        .iter()
        .map(|collection| format!("{}", collection))
        .collect();

    if app.collection_state.selected().is_none() {
        app.collection_state.select(Some(0));
    }
    let collection_list = List::new(collection_lines)
        .block(
            Block::default()
                .border_style(THEME.text)
                .borders(Borders::ALL),
        )
        .style(if collection_focus {
            THEME.text
        } else {
            THEME.background
        })
        .highlight_style(Modifier::REVERSED)
        .highlight_symbol("> ");

    let width = centered_area.width.max(3) - 3;
    let scroll = app.collection_name.visual_scroll(width as usize);

    let collection_block = Block::default()
        .borders(Borders::ALL)
        .title("Enter new collection name")
        .padding(Padding::left(1))
        .border_style(THEME.text)
        .style(if collection_focus {
            THEME.primary
        } else {
            THEME.text
        });

    let paragraph = Paragraph::new(app.collection_name.value())
        .scroll((0, scroll as u16))
        .style(if collection_focus {
            THEME.background
        } else {
            THEME.text
        })
        .block(collection_block);

    if !collection_focus {
        let x = app.collection_name.visual_cursor().max(scroll) - scroll + 2;
        frame.set_cursor_position((
            add_to_collection_layout.x + x as u16,
            add_to_collection_layout.y + 1,
        ));
    }
    frame.render_stateful_widget(
        collection_list,
        collection_list_layout,
        &mut app.collection_state,
    );

    frame.render_widget(paragraph, add_to_collection_layout);
}
