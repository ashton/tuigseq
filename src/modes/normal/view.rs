use ratatui::{
    Frame,
    layout::{Constraint, Layout, Margin},
    prelude::Stylize,
    style::{Color, Modifier},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem},
};

use crate::model::Model;

pub fn render(model: &Model, frame: &mut Frame) {
    let root_layout = Layout::horizontal([Constraint::Percentage(25), Constraint::Percentage(75)]);
    let content_layout = Layout::vertical([Constraint::Fill(1)]);
    let [menu_area, main_area] = frame.area().layout(&root_layout);
    let [blocks_area] = main_area.layout(&content_layout);

    frame.render_widget(
        Block::default().borders(Borders::all()),
        menu_area.inner(Margin {
            vertical: 1,
            horizontal: 2,
        }),
    );

    let content_block = Block::new()
        .title("Page title".bold())
        .borders(Borders::all());

    let items: Vec<ListItem> = model
        .blocks
        .iter()
        .map(|item| {
            let line = Line::from(vec!["󰝥".into(), " ".into(), Span::from(item)]);
            ListItem::new(line)
        })
        .collect();

    let mut list_state = ratatui::widgets::ListState::default().with_selected(Some(model.selected));

    let list = List::new(items)
        .style(Color::White)
        .highlight_style(Modifier::REVERSED)
        .block(content_block);

    frame.render_stateful_widget(list, blocks_area, &mut list_state);
}
