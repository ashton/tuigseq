use std::time::Duration;

use ratatui::{
    Frame,
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Margin, Rect},
    prelude::Stylize,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Widget},
};
use tui_widget_list::{ListBuilder, ListView};

enum ItemWidget<'a> {
    Text(Line<'a>),
    Edit(TextInputWidget<'a>),
}

impl Widget for ItemWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self {
            ItemWidget::Text(line) => line.render(area, buf),
            ItemWidget::Edit(input) => input.render(area, buf),
        }
    }
}

struct TextInputWidget<'a> {
    text: &'a str,
    cursor: usize,
}

impl Widget for TextInputWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default().borders(Borders::all());
        let inner = block.inner(area);
        block.render(area, buf);

        Paragraph::new(self.text).render(inner, buf);

        if inner.width == 0 || inner.height == 0 {
            return;
        }

        let cursor_col_offset = self.text[..self.cursor].chars().count() as u16;
        let cursor_x = inner.x.saturating_add(cursor_col_offset);
        let cursor_y = inner.y;

        if cursor_x < inner.x.saturating_add(inner.width) {
            if let Some(cell) = buf.cell_mut((cursor_x, cursor_y)) {
                cell.set_style(Style::default().add_modifier(Modifier::REVERSED));
            }
        }
    }
}

#[derive(Debug)]
struct EditData {
    text: String,
    cursor: usize,
}

#[derive(Debug, Default)]
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
    InsertChar(char),
    Backspace,
    MoveCursorLeft,
    MoveCursorRight,
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

    while !matches!(model.mode, Mode::Quit) {
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
    match &model.mode {
        Mode::Normal => render_normal_mode(model, frame),
        Mode::Editing(edit_data) => render_edit_mode(model, edit_data, frame),
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
            let widget = TextInputWidget {
                text: &edit_data.text,
                cursor: edit_data.cursor,
            };
            return (ItemWidget::Edit(widget), 3);
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
    if event::poll(Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Release {
                return Ok(None);
            }
            return Ok(handle_key(model, key));
        }
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
            if let Mode::Editing(edit_data) = &mut model.mode {
                if let Some((prev_idx, _)) = edit_data.text[..edit_data.cursor]
                    .char_indices()
                    .next_back()
                {
                    edit_data.text.remove(prev_idx);
                    edit_data.cursor = prev_idx;
                }
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
