mod api;
mod model;
mod state;
mod ui;
mod app;
mod logger;

use ratatui::{
    backend::CrosstermBackend,
    Terminal
};
use crossterm::{
    execute,
    terminal::{
        disable_raw_mode, 
        enable_raw_mode, 
        EnterAlternateScreen, 
        LeaveAlternateScreen,
    },
};
use std::io;

fn main() -> Result<(), io::Error> {
    logger::init_logger().expect("Failed to initialize service logger");
    log::info!("Starting live MLB scoreboard service!");
    
    // enable raw mode to capture keystrokes
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = app::run_app(&mut terminal);

    log::info!("Shutting down live MLB scoreboard service!");
    
    // Always run teardown logic to restore terminal state, even on error
    if let Err(e) = (|| -> Result<(), io::Error> {
        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        terminal.show_cursor()?;
        Ok(())
    })() {
        eprintln!("Failed to restore terminal state: {}", e);
    }

    result
}
