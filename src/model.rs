#[derive(Debug)]
pub struct EditData {
    pub text: String,
    pub cursor: usize,
}

#[derive(Debug, Default)]
pub enum Mode {
    #[default]
    Normal,
    Editing(EditData),
}

#[derive(Debug, Default)]
pub struct Model {
    pub mode: Mode,
    pub blocks: Vec<String>,
    pub selected: usize,
    pub running: bool,
}
