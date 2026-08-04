use std::time::Duration;

use ratatui::{
    Frame,
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode},
    layout::{Constraint, Layout, Margin, Rect},
    prelude::Stylize,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Widget},
};
use ratatui_textarea::TextArea;
use tui_widget_list::{ListBuilder, ListView};

#[allow(clippy::large_enum_variant)]
enum ItemWidget<'a> {
    Text(Line<'a>),
    Edit(TextArea<'a>),
}

impl Widget for ItemWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self {
            ItemWidget::Text(line) => line.render(area, buf),
            ItemWidget::Edit(textarea) => (&textarea).render(area, buf),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct CursorPosition {
    line: u16,
    col: u16,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct EditData {
    cursor_position: CursorPosition,
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
    StopEditing,
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
        term.draw(|frame| view(&model, frame))?;
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
        Mode::Editing(edit_data) => render_edit_mode(model, &edit_data, frame),
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

    let mut list_state = ratatui::widgets::ListState::default().with_selected(Some(model.selected));

    let list = List::new(items)
        .style(Color::White)
        .highlight_style(Modifier::REVERSED)
        .block(content_block);

    frame.render_stateful_widget(list, blocks_area, &mut list_state);
}

fn render_edit_mode(model: &Model, edit_data: &EditData, frame: &mut Frame) {
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

    let builder = ListBuilder::new(|context| {
        if context.is_selected {
            let textarea_block = Block::default().borders(Borders::all());
            let mut textarea = TextArea::from([model.blocks[context.index].clone()]);
            textarea.set_cursor_line_style(Style::default());
            textarea.set_block(textarea_block);
            textarea.move_cursor(ratatui_textarea::CursorMove::Jump(
                edit_data.cursor_position.line,
                edit_data.cursor_position.col,
            ));
            return (ItemWidget::Edit(textarea), 3);
        }

        let line = Line::from(vec![
            "󰝥".into(),
            " ".into(),
            Span::from(model.blocks[context.index].clone()),
        ]);
        (ItemWidget::Text(line), 1)
    });

    let list_view = ListView::new(builder, model.blocks.len()).block(content_block);
    let mut list_state = tui_widget_list::ListState::new_with_index(Some(model.selected));
    frame.render_stateful_widget(list_view, blocks_area, &mut list_state);
}

fn handle_event(model: &Model) -> color_eyre::Result<Option<Message>> {
    if event::poll(Duration::from_millis(250))?
        && let Event::Key(key) = event::read()?
        && key.kind == event::KeyEventKind::Press
    {
        return Ok(handle_key(model, key));
    }

    Ok(None)
}

fn handle_key(model: &Model, key: event::KeyEvent) -> Option<Message> {
    match model.mode {
        Mode::Normal => match key.code {
            KeyCode::Char('j') => Some(Message::MoveDown),
            KeyCode::Char('k') => Some(Message::MoveUp),
            KeyCode::Char('q') => Some(Message::Quit),
            KeyCode::Char('A') => Some(Message::InsertAtTheEnd),
            _ => None,
        },
        Mode::Editing(_) => match key.code {
            KeyCode::Esc => Some(Message::StopEditing),
            _ => None,
        },
        Mode::Quit => None,
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
                cursor_position: CursorPosition {
                    line: 0,
                    col: model.blocks[model.selected].len() as u16,
                },
            });
            None
        }
        Message::StopEditing => {
            model.mode = Mode::Normal;
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
