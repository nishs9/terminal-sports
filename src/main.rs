mod api;
mod model;
mod state;
mod ui;
mod app;
mod utils;

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
    // enable raw mode to capture keystrokes
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = app::run_app(&mut terminal);
    
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}
