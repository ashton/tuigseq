use ratatui::{
    Frame,
    buffer::Buffer,
    layout::{Constraint, Layout, Margin, Rect},
    prelude::Stylize,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Widget},
};
use tui_widget_list::{ListBuilder, ListView};

use crate::model::{EditData, Mode, Model};

enum ItemWidget<'a> {
    Text(Line<'a>),
    Edit(TextInputWidget<'a>),
}

impl Widget for ItemWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self {
            ItemWidget::Text(line) => line.render(area, buf),
            ItemWidget::Edit(input) => input.render(area, buf),
        }
    }
}

struct TextInputWidget<'a> {
    text: &'a str,
    cursor: usize,
}

impl Widget for TextInputWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default().borders(Borders::all());
        let inner = block.inner(area);
        block.render(area, buf);

        Paragraph::new(self.text).render(inner, buf);

        if inner.width == 0 || inner.height == 0 {
            return;
        }

        let cursor_col_offset = self.text[..self.cursor].chars().count() as u16;
        let cursor_x = inner.x.saturating_add(cursor_col_offset);
        let cursor_y = inner.y;

        if cursor_x < inner.x.saturating_add(inner.width)
            && let Some(cell) = buf.cell_mut((cursor_x, cursor_y))
        {
            cell.set_style(Style::default().add_modifier(Modifier::REVERSED));
        }
    }
}

pub fn view(model: &Model, frame: &mut Frame) {
    match &model.mode {
        Mode::Normal => render_normal_mode(model, frame),
        Mode::Editing(edit_data) => render_edit_mode(model, edit_data, frame),
    }
}

fn render_normal_mode(model: &Model, frame: &mut Frame) {
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

fn render_edit_mode(model: &Model, edit_data: &EditData, frame: &mut Frame) {
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
