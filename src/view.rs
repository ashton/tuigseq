use crate::model::Model;
use ratatui::Frame;

mod modes;
pub mod widgets;

pub fn view(model: &Model, frame: &mut Frame) {
    modes::render(model, frame);
}
