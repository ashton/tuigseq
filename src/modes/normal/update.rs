use crate::model::{EditData, Mode, Model};
use crate::msg::Message;

pub fn update(model: &mut Model, msg: Message) -> Option<Message> {
    match msg {
        Message::Quit => {
            model.running = false;
            None
        }

        Message::MoveUp => {
            if model.selected == 0 {
                model.selected = model.blocks.len() - 1;
                return None;
            }

            model.selected -= 1;
            None
        }

        Message::MoveDown => {
            if model.selected == model.blocks.len() - 1 {
                model.selected = 0;
                return None;
            }

            model.selected += 1;
            None
        }

        Message::InsertAtTheEnd => {
            let block = &model.blocks[model.selected];
            model.mode = Mode::Editing(EditData {
                text: block.clone(),
                cursor: block.len(),
            });
            None
        }

        _ => None,
    }
}
