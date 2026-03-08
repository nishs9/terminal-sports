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
use state::AppState;
use std::io;

fn main() -> Result<(), io::Error> {
    // enable raw mode to capture keystrokes
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let init_games = utils::get_mock_data();
    let mut app_state = AppState::new(init_games);

    let result = app::run_app(&mut terminal, &mut app_state);
    
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}
