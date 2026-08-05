#[derive(PartialEq)]
pub enum Message {
    MoveUp,
    MoveDown,
    InsertAtTheEnd,
    StopEditing,
    InsertChar(char),
    Backspace,
    MoveCursorLeft,
    MoveCursorRight,
    Quit,
}
