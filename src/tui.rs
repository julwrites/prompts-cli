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
use prompts_core::{load_prompts, Prompt};

enum InputMode {
    Normal,
    Editing,
}

struct TuiApp {
    prompts: Vec<Prompt>,
    list_state: ListState,
    input_mode: InputMode,
    selected_prompt_text: String,
}

impl TuiApp {
    fn new(prompts: Vec<Prompt>) -> TuiApp {
        let mut list_state = ListState::default();
        if !prompts.is_empty() {
            list_state.select(Some(0));
        }
        TuiApp {
            prompts,
            list_state,
            input_mode: InputMode::Normal,
            selected_prompt_text: String::new(),
        }
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
        } else {
            self.selected_prompt_text = String::new();
        }
    }
}

pub fn run(file: &str) -> Result<()> {
    let prompts = load_prompts(file)?;
    let mut app = TuiApp::new(prompts);
    app.update_selected_prompt_text();

    let mut terminal = setup_terminal()?;
    let mut should_quit = false;

    while !should_quit {
        terminal.draw(|frame| ui(frame, &mut app))?;
        should_quit = handle_events(&mut app)?;
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

    let prompt_text = Paragraph::new(app.selected_prompt_text.clone())
        .block(Block::default().title("Prompt Text").borders(Borders::ALL));

    let chunks = ratatui::prelude::Layout::default()
        .direction(ratatui::prelude::Direction::Horizontal)
        .constraints([
            ratatui::prelude::Constraint::Percentage(30),
            ratatui::prelude::Constraint::Percentage(70),
        ])
        .split(frame.size());

    frame.render_stateful_widget(list, chunks[0], &mut app.list_state);
    frame.render_widget(prompt_text, chunks[1]);
}

fn handle_events(app: &mut TuiApp) -> Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('q') => return Ok(true),
                    KeyCode::Down => app.next(),
                    KeyCode::Up => app.previous(),
                    _ => {},
                },
                InputMode::Editing => {
                    // Handle editing mode input here later
                }
            }
        }
    }
    Ok(false)
}
