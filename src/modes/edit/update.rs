use crate::model::{Mode, Model};
use crate::msg::Message;

pub fn update(model: &mut Model, msg: Message) -> Option<Message> {
    match msg {
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

        _ => None,
    }
}
