use ratatui::{
    Frame,
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::model::Model;

mod modes;

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
    modes::render(model, frame);
}
