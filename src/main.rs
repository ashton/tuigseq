use std::time::Duration;

use ratatui::{
    Frame,
    crossterm::event::{self, Event, KeyCode},
    layout::{Constraint, Layout, Margin},
    prelude::Stylize,
    style::{Color, Modifier},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState},
};

#[derive(Debug, PartialEq, Eq)]
enum CursorPosition {
    Begining,
    End,
    Above,
    Below,
}

#[derive(Debug, PartialEq, Eq)]
struct EditData {
    position: CursorPosition,
}

#[derive(Debug, Default, PartialEq, Eq)]
enum Mode {
    #[default]
    Normal,
    Editing(EditData),
    Quit,
}

#[derive(PartialEq)]
enum Message {
    MoveUp,
    MoveDown,
    InsertAtTheEnd,
    Quit,
}

#[derive(Debug, Default)]
struct Model {
    mode: Mode,
    blocks: Vec<String>,
    selected: usize,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let mut term = tui::init_terminal()?;
    let mut model = Model {
        blocks: vec![String::from("Item 1"), String::from("Item 2")],
        ..Default::default()
    };

    while model.mode != Mode::Quit {
        term.draw(|frame| view(&mut model, frame))?;
        let mut current_msg = handle_event(&model)?;
        while current_msg.is_some() {
            current_msg = update(&mut model, current_msg.unwrap());
        }
    }

    tui::restore_terminal()?;
    Ok(())
}

fn view(model: &Model, frame: &mut Frame) {
    match model.mode {
        Mode::Normal => render_normal_mode(model, frame),
        Mode::Editing(_) => render_edit_mode(model, frame),
        Mode::Quit => (),
    }
}

fn render_normal_mode(model: &Model, frame: &mut Frame) {
    let root_layout = Layout::horizontal([Constraint::Percentage(25), Constraint::Percentage(75)]);
    let content_layout = Layout::vertical([Constraint::Fill(1)]);
    let [menu_area, main_area] = frame.area().layout(&root_layout);
    let [blocks_area] = main_area.layout(&content_layout);

    frame.render_widget(
        Block::default().borders(Borders::all()),
        menu_area.inner(Margin {
            vertical: 1,
            horizontal: 2,
        }),
    );

    let content_block = Block::new()
        .title("Page title".bold())
        .borders(Borders::all());

    let items: Vec<ListItem> = model
        .blocks
        .iter()
        .map(|item| {
            let line = Line::from(vec!["󰝥".into(), " ".into(), Span::from(item)]);
            ListItem::new(line)
        })
        .collect();

    let mut list_state = ListState::default().with_selected(Some(model.selected));

    let list = List::new(items)
        .style(Color::White)
        .highlight_style(Modifier::REVERSED)
        .block(content_block);

    frame.render_stateful_widget(list, blocks_area, &mut list_state);
}

fn render_edit_mode(model: &Model, frame: &mut Frame) {
    let root_layout = Layout::horizontal([Constraint::Percentage(25), Constraint::Percentage(75)]);
    let content_layout = Layout::vertical([Constraint::Fill(1)]);
    let [menu_area, main_area] = frame.area().layout(&root_layout);
    let [blocks_area] = main_area.layout(&content_layout);

    frame.render_widget(
        Block::default().borders(Borders::all()),
        menu_area.inner(Margin {
            vertical: 1,
            horizontal: 2,
        }),
    );

    let content_block = Block::new()
        .title("Page title".bold())
        .borders(Borders::all());

    let items: Vec<ListItem> = model
        .blocks
        .iter()
        .map(|item| {
            let line = Line::from(vec!["󰝥".into(), " ".into(), Span::from(item)]);
            ListItem::new(line)
        })
        .collect();

    let mut list_state = ListState::default().with_selected(Some(model.selected));

    let list = List::new(items)
        .style(Color::White)
        .highlight_style(Modifier::REVERSED)
        .block(content_block);

    frame.render_stateful_widget(list, blocks_area, &mut list_state);
}

fn handle_event(_: &Model) -> color_eyre::Result<Option<Message>> {
    if event::poll(Duration::from_millis(250))?
        && let Event::Key(key) = event::read()?
        && key.kind == event::KeyEventKind::Press
    {
        return Ok(handle_key(key));
    }

    Ok(None)
}

fn handle_key(key: event::KeyEvent) -> Option<Message> {
    match key.code {
        KeyCode::Char('j') => Some(Message::MoveDown),
        KeyCode::Char('k') => Some(Message::MoveUp),
        KeyCode::Char('q') => Some(Message::Quit),
        KeyCode::Char('A') => Some(Message::InsertAtTheEnd),
        _ => None,
    }
}

fn update(model: &mut Model, msg: Message) -> Option<Message> {
    match msg {
        Message::Quit => {
            model.mode = Mode::Quit;
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
            model.mode = Mode::Editing(EditData {
                position: CursorPosition::End,
            });
            None
        }
    }
}

mod tui {
    use std::io::stdout;

    use ratatui::{
        Terminal,
        backend::CrosstermBackend,
        crossterm::{
            ExecutableCommand,
            terminal::{
                EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
            },
        },
    };

    pub fn init_terminal() -> color_eyre::Result<ratatui::DefaultTerminal> {
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;
        let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        Ok(terminal)
    }

    pub fn restore_terminal() -> color_eyre::Result<()> {
        stdout().execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }
}
