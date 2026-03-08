use crate::state::AppState;
use crate::ui::draw_tui;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    Terminal, 
    prelude::Backend
};
use std::io;

pub fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app_state: &mut AppState,
) -> Result<(), io::Error> {
    loop {
        terminal.draw(|frame| draw_tui(frame, app_state))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Up => app_state.move_up(),
                KeyCode::Down => app_state.move_down(),
                _ => {}
            }
        }
    }
    Ok(())
}