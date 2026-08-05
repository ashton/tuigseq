use ratatui::{
    Frame,
    layout::{Constraint, Layout, Margin},
    prelude::Stylize,
    text::{Line, Span},
    widgets::{Block, Borders},
};
use tui_widget_list::{ListBuilder, ListView};

use crate::model::{EditData, Model};
use crate::view::ItemWidget;
use crate::view::TextInputWidget;

pub fn render(model: &Model, edit_data: &EditData, frame: &mut Frame) {
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

    let builder = ListBuilder::new(|context| {
        if context.is_selected {
            let widget = TextInputWidget {
                text: &edit_data.text,
                cursor: edit_data.cursor,
            };
            return (ItemWidget::Edit(widget), 3);
        }

        let line = Line::from(vec![
            "󰝥".into(),
            " ".into(),
            Span::from(model.blocks[context.index].clone()),
        ]);
        (ItemWidget::Text(line), 1)
    });

    let list_view = ListView::new(builder, model.blocks.len()).block(content_block);
    let mut list_state = tui_widget_list::ListState::new_with_index(Some(model.selected));
    frame.render_stateful_widget(list_view, blocks_area, &mut list_state);
}
