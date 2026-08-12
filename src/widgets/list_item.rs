use crate::widgets::text_input::TextInputWidget;
use ratatui::{buffer::Buffer, layout::Rect, text::Line, widgets::Widget};

pub enum ListItemWidget<'a> {
    Text(Line<'a>),
    Edit(TextInputWidget<'a>),
}

impl Widget for ListItemWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self {
            ListItemWidget::Text(line) => line.render(area, buf),
            ListItemWidget::Edit(input) => input.render(area, buf),
        }
    }
}
