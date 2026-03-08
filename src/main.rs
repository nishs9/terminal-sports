mod api;
mod model;
mod state;
mod ui;
mod app;

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{
        disable_raw_mode, 
        enable_raw_mode, 
        EnterAlternateScreen, 
        LeaveAlternateScreen,
    },
};
use std::io;
use ratatui::{backend::CrosstermBackend, Terminal};

fn main() -> Result<(), io::Error> {
    // enable raw mode to capture keystrokes
    enable_raw_mode()?;

    let mut stdout = io::stdout();

    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run_app(&mut terminal);
    
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
) -> Result<(), io::Error> {
    loop {
        terminal.draw(|frame| {
            let size = frame.size();

            let block = ratatui::widgets::Block::default()
                .title("Live Baseball Scoreboard TUI")
                .borders(ratatui::widgets::Borders::ALL);

            frame.render_widget(block, size);
        })?;

        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') {
                break;
            }
        }
    }
    Ok(())
}
