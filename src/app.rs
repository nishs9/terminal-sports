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
use std::time::Duration;

const AUTO_REFRESH_INTERVAL: u64 = 60;

pub fn run_app<B: Backend>(
    terminal: &mut Terminal<B>
) -> Result<(), io::Error> {
    let mut client = api::ApiClient::new("snapshots");
    let init_games = client.fetch_games(&League::Mlb).unwrap_or_default();
    let mut app_state = AppState::new(League::Mlb, init_games);
    loop {
        terminal.draw(|frame| draw_tui(frame, &app_state))?;

        if should_auto_refresh(&mut app_state) {
            refresh_games(&mut app_state, &mut client);
        }

        if event::poll(Duration::from_millis(100))?
            && let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Char('r') => {
                    // Prevents a user from spamming the manual refresh
                    let time_since_refresh = match app_state.last_refresh {
                        Some(last_refresh) => last_refresh.elapsed(),
                        None => Duration::from_secs(0),
                    };
                    if time_since_refresh > Duration::from_secs(10) {
                        refresh_games(&mut app_state, &mut client);
                    }
                }
                KeyCode::Char('l') => app_state.toggle_league(),
                KeyCode::Up => app_state.move_up(),
                KeyCode::Down => app_state.move_down(),
                _ => {}
            }
        }
    }
    Ok(())
}

fn refresh_games(app_state: &mut AppState, client: &mut api::ApiClient) {
    app_state.start_refresh();

    match client.fetch_games(&app_state.league) {
        Ok(games) => {
            log::debug!("Refreshed games: {:?}", games.len());
            log::debug!("Selected game index: {:?}", app_state.selected_game_idx);
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
        Some(last_refresh) => 
            last_refresh.elapsed() >= Duration::from_secs(AUTO_REFRESH_INTERVAL),
        None => true,
    }
}