use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::{CrosstermBackend, Terminal},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    style::{Style, Modifier, Color},
};
use std::io;
use prompts_core::{load_prompts, Prompt, MockTextGenerator, TextGenerator};

enum InputMode {
    Normal,
    Editing,
    Generating,
}

struct TuiApp {
    prompts: Vec<prompts_core::Prompt>,
    list_state: ListState,
    input_mode: InputMode,
    selected_prompt_text: String,
    cursor_position: usize,
    generated_text: String,
}

impl TuiApp {
    fn new(prompts: Vec<prompts_core::Prompt>) -> TuiApp {
        let mut list_state = ListState::default();
        if !prompts.is_empty() {
            list_state.select(Some(0));
        }
        let mut app = TuiApp {
            prompts,
            list_state,
            input_mode: InputMode::Normal,
            selected_prompt_text: String::new(),
            cursor_position: 0,
            generated_text: String::new(),
        };
        app.update_selected_prompt_text();
        app
    }

    fn next(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.prompts.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
        self.update_selected_prompt_text();
    }

    fn previous(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.prompts.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
        self.update_selected_prompt_text();
    }

    fn update_selected_prompt_text(&mut self) {
        if let Some(selected) = self.list_state.selected() {
            self.selected_prompt_text = self.prompts[selected].text.clone();
            self.cursor_position = self.selected_prompt_text.len();
        } else {
            self.selected_prompt_text = String::new();
            self.cursor_position = 0;
        }
    }

    fn enter_editing_mode(&mut self) {
        self.input_mode = InputMode::Editing;
    }

    fn exit_editing_mode(&mut self) {
        if let Some(selected) = self.list_state.selected() {
            self.prompts[selected].text = self.selected_prompt_text.clone();
        }
        self.input_mode = InputMode::Normal;
    }

    fn discard_editing_mode(&mut self) {
        self.update_selected_prompt_text(); // Revert changes
        self.input_mode = InputMode::Normal;
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.cursor_position.saturating_sub(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.cursor_position.saturating_add(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_right);
    }

    fn enter_char(&mut self, new_char: char) {
        self.selected_prompt_text.insert(self.cursor_position, new_char);
        self.move_cursor_right();
    }

    fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.cursor_position != 0;
        if is_not_cursor_leftmost {
            // `drain` does not `panic` if the index is out of bounds.
            // `saturating_sub` makes sure that `from` is always at least `0`
            let current_index = self.cursor_position;
            let from = current_index.saturating_sub(1);
            let to = current_index;
            self.selected_prompt_text.drain(from..to);
            self.move_cursor_left();
        }
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.selected_prompt_text.len())
    }

    fn reset_cursor(&mut self) {
        self.cursor_position = 0;
    }

    async fn generate_text(&mut self) {
        if let Some(selected) = self.list_state.selected() {
            let prompt = &self.prompts[selected];
            let generated = MockTextGenerator.generate(&prompt.text).await;
            self.generated_text = generated;
        } else {
            self.generated_text = "No prompt selected for generation.".to_string();
        }
        self.input_mode = InputMode::Generating;
    }
}

pub async fn run(file: &str) -> Result<()> {
    let prompts = prompts_core::load_prompts(file)?;
    let mut app = TuiApp::new(prompts);

    let mut terminal = setup_terminal()?;
    let mut should_quit = false;

    while !should_quit {
        terminal.draw(|frame| ui(frame, &mut app))?;
        should_quit = handle_events(&mut app).await?;
    }

    restore_terminal()?;

    Ok(())
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    Ok(Terminal::new(CrosstermBackend::new(stdout))?)
}

fn restore_terminal() -> Result<()> {
    disable_raw_mode()?;
    execute!(
        io::stdout(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    Ok(())
}

fn ui(frame: &mut ratatui::Frame, app: &mut TuiApp) {
    let items: Vec<ListItem> = app.prompts
        .iter()
        .map(|p| ListItem::new(p.name.clone()))
        .collect();

    let list = List::new(items)
        .block(Block::default().title("Prompts").borders(Borders::ALL))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD).fg(Color::Green))
        .highlight_symbol("> ");

    let prompt_text_block = Block::default().title("Prompt Text").borders(Borders::ALL);
    let prompt_text = Paragraph::new(app.selected_prompt_text.clone())
        .block(prompt_text_block.clone());

    let generated_text_block = Block::default().title("Generated Text").borders(Borders::ALL);
    let generated_text = Paragraph::new(app.generated_text.clone())
        .block(generated_text_block.clone());

    let chunks = ratatui::prelude::Layout::default()
        .direction(ratatui::prelude::Direction::Horizontal)
        .constraints([
            ratatui::prelude::Constraint::Percentage(30),
            ratatui::prelude::Constraint::Percentage(70),
        ])
        .split(frame.size());

    frame.render_stateful_widget(list, chunks[0], &mut app.list_state);

    match app.input_mode {
        InputMode::Normal | InputMode::Editing => {
            frame.render_widget(prompt_text, chunks[1]);
            if let InputMode::Editing = app.input_mode {
                frame.set_cursor(
                    chunks[1].x + app.cursor_position as u16 + prompt_text_block.inner(chunks[1]).x,
                    chunks[1].y + prompt_text_block.inner(chunks[1]).y,
                );
            }
        },
        InputMode::Generating => {
            frame.render_widget(generated_text, chunks[1]);
        }
    }
}

async fn handle_events(app: &mut TuiApp) -> Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('q') => return Ok(true),
                    KeyCode::Down => app.next(),
                    KeyCode::Up => app.previous(),
                    KeyCode::Char('e') => app.enter_editing_mode(),
                    KeyCode::Char('g') => {
                        app.generate_text().await;
                    },
                    _ => {},
                },
                InputMode::Editing => match key.code {
                    KeyCode::Enter => app.exit_editing_mode(),
                    KeyCode::Esc => app.discard_editing_mode(),
                    KeyCode::Left => app.move_cursor_left(),
                    KeyCode::Right => app.move_cursor_right(),
                    KeyCode::Backspace => app.delete_char(),
                    KeyCode::Char(to_insert) => {
                        app.enter_char(to_insert);
                    }
                    _ => {},
                },
                InputMode::Generating => match key.code {
                    KeyCode::Esc => {
                        app.input_mode = InputMode::Normal;
                    },
                    _ => {},
                },
            }
        }
    }
    Ok(false)
}
