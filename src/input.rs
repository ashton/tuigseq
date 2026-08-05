use std::time::Duration;

use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent};

use crate::model::{Mode, Model};
use crate::msg::Message;

pub fn handle_event(model: &Model) -> color_eyre::Result<Option<Message>> {
    if event::poll(Duration::from_millis(50))?
        && let Event::Key(key) = event::read()?
    {
        return Ok(handle_key(model, key));
    }

    Ok(None)
}

fn handle_key(model: &Model, key: KeyEvent) -> Option<Message> {
    match &model.mode {
        Mode::Normal => match key.code {
            KeyCode::Char('j') => Some(Message::MoveDown),
            KeyCode::Char('k') => Some(Message::MoveUp),
            KeyCode::Char('q') => Some(Message::Quit),
            KeyCode::Char('A') => Some(Message::InsertAtTheEnd),
            _ => None,
        },
        Mode::Editing(_) => match key.code {
            KeyCode::Esc => Some(Message::StopEditing),
            KeyCode::Backspace => Some(Message::Backspace),
            KeyCode::Left => Some(Message::MoveCursorLeft),
            KeyCode::Right => Some(Message::MoveCursorRight),
            KeyCode::Char(c) => Some(Message::InsertChar(c)),
            _ => None,
        },
    }
}
