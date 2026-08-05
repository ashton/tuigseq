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
        Message::StopEditing => {
            model.mode = Mode::Normal;
            None
        }
        Message::InsertChar(c) => {
            if let Mode::Editing(edit_data) = &mut model.mode {
                edit_data.text.insert(edit_data.cursor, c);
                edit_data.cursor += c.len_utf8();
            }
            None
        }
        Message::Backspace => {
            if let Mode::Editing(edit_data) = &mut model.mode
                && let Some((prev_idx, _)) = edit_data.text[..edit_data.cursor]
                    .char_indices()
                    .next_back()
            {
                edit_data.text.remove(prev_idx);
                edit_data.cursor = prev_idx;
            }
            None
        }
        Message::MoveCursorLeft => {
            if let Mode::Editing(edit_data) = &mut model.mode
                && let Some((prev_idx, _)) = edit_data.text[..edit_data.cursor]
                    .char_indices()
                    .next_back()
            {
                edit_data.cursor = prev_idx;
            }
            None
        }
        Message::MoveCursorRight => {
            if let Mode::Editing(edit_data) = &mut model.mode {
                let rest = &edit_data.text[edit_data.cursor..];
                if let Some(c) = rest.chars().next() {
                    edit_data.cursor += c.len_utf8();
                }
            }
            None
        }
    }
}
