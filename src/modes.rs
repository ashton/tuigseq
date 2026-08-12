use ratatui::Frame;

use crate::model::{Mode, Model};
use crate::msg::Message;

pub mod edit;
pub mod normal;

pub fn update(model: &mut Model, msg: Message) -> Option<Message> {
    match &model.mode {
        Mode::Normal => normal::update::update(model, msg),
        Mode::Editing(_) => edit::update::update(model, msg),
    }
}

pub fn view(model: &Model, frame: &mut Frame) {
    match &model.mode {
        Mode::Normal => normal::view::render(model, frame),
        Mode::Editing(edit_data) => edit::view::render(model, edit_data, frame),
    }
}
