use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::{CrosstermBackend, Stylize, Terminal},
    widgets::{Block, Borders, Paragraph},
};
use std::io;

pub fn run(file: &str) -> Result<()> {
    let mut terminal = setup_terminal()?;
    let mut should_quit = false;

    while !should_quit {
        terminal.draw(ui)?;
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

fn ui(frame: &mut ratatui::Frame) {
    frame.render_widget(
        Paragraph::new("Hello, TUI!")
            .block(Block::default().title("Prompts").borders(Borders::ALL)),
        frame.size(),
    );
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
