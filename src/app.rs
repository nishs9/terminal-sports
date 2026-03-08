use crate::state::AppState;
use crate::ui::draw_tui;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    Terminal, 
    prelude::Backend
};
use crate::model::League;
use crate::api;
use std::io;
use std::time::{Instant, Duration};

pub fn run_app<B: Backend>(
    terminal: &mut Terminal<B>
) -> Result<(), io::Error> {
    let init_games = api::fetch_games(&League::Wbc).unwrap_or_default();
    let mut app_state = AppState::new(init_games);
    loop {
        terminal.draw(|frame| draw_tui(frame, &app_state))?;

        if should_auto_refresh(&mut app_state) {
            refresh_games(&mut app_state);
        }

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('r') => refresh_games(&mut app_state),
                    KeyCode::Up => app_state.move_up(),
                    KeyCode::Down => app_state.move_down(),
                    _ => {}
                }
            }
        }
    }
    Ok(())
}

fn refresh_games(app_state: &mut AppState) {
    app_state.start_refresh();

    match api::fetch_games(&app_state.league) {
        Ok(games) => {
            app_state.set_games(games);
            app_state.complete_refresh();
        }
        Err(err) => {
            app_state.set_refresh_err(err);
        }
    }
}

fn should_auto_refresh(app_state: &mut AppState) -> bool {
    if app_state.is_refreshing {
        return false;
    }

    match app_state.last_refresh {
        Some(last_refresh) => {
            if last_refresh.elapsed() >= Duration::from_secs(10) {
                true
            } else {
                false
            }
        },
        None => true,
    }
}