use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::{CrosstermBackend, Terminal},
    widgets::{Block, Borders, List, ListItem},
};
use std::io;
use prompts_core::{load_prompts, Prompt};

pub fn run(file: &str) -> Result<()> {
    let prompts = load_prompts(file)?;
    let mut terminal = setup_terminal()?;
    let mut should_quit = false;

    while !should_quit {
        terminal.draw(|frame| ui(frame, &prompts))?;
        should_quit = handle_events()?;
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

fn ui(frame: &mut ratatui::Frame, prompts: &[Prompt]) {
    let items: Vec<ListItem> = prompts
        .iter()
        .map(|p| ListItem::new(p.name.clone()))
        .collect();

    let list = List::new(items)
        .block(Block::default().title("Prompts").borders(Borders::ALL));

    frame.render_widget(list, frame.size());
}

fn handle_events() -> Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(true);
            }
        }
    }
    Ok(false)
}
