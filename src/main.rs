mod input;
mod model;
mod msg;
mod tui;
mod update;
mod view;

use crate::model::Model;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let mut term = tui::init_terminal()?;
    let mut model = Model {
        blocks: vec![String::from("Item 1"), String::from("Item 2")],
        running: true,
        ..Default::default()
    };

    while model.running {
        term.draw(|frame| view::view(&model, frame))?;
        let mut current_msg = input::handle_event(&model)?;
        while current_msg.is_some() {
            current_msg = update::update(&mut model, current_msg.unwrap());
        }
    }

    tui::restore_terminal()?;
    Ok(())
}
