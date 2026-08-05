use ratatui::Frame;

use crate::model::Mode;
use crate::model::Model;

mod edit_mode;
mod normal_mode;

pub fn render(model: &Model, frame: &mut Frame) {
    match &model.mode {
        Mode::Normal => normal_mode::render(model, frame),
        Mode::Editing(edit_data) => edit_mode::render(model, edit_data, frame),
    }
}
