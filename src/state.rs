use crate::model::{League, GameSummary};
use std::time::Instant;
use chrono::{DateTime, Local};

pub struct AppState {
    pub league: League,
    pub games: Vec<GameSummary>,
    pub selected_game_idx: usize,

    pub last_refresh: Option<Instant>,
    pub last_timestamp: Option<DateTime<Local>>,
    pub is_refreshing: bool,
    pub last_error: Option<String>,
}

impl AppState {
    pub fn new(games: Vec<GameSummary>) -> Self {
        Self {
            league: League::Wbc,
            games,
            selected_game_idx: 0,
            last_refresh: None,
            last_timestamp: None,
            is_refreshing: false,
            last_error: None,
        }
    }

    pub fn set_games(&mut self, games: Vec<GameSummary>) {
        self.games = games;
        log::info!("Setting games: selected game index is {}", self.selected_game_idx);
        log::info!("Games: {:?}", self.games.len());
        if self.selected_game_idx >= self.games.len() {
            self.selected_game_idx = self.games.len() - 1;
        }
    }

    pub fn move_up(&mut self) {
        if self.games.is_empty() {
            return;
        }

        if self.selected_game_idx > 0 {
            self.selected_game_idx -= 1;
        }
    }

    pub fn move_down(&mut self) {
        if self.games.is_empty() {
            return;
        }

        if self.selected_game_idx + 1 < self.games.len() {
            self.selected_game_idx += 1;
        }
    }

    pub fn start_refresh(&mut self) {
        self.is_refreshing = true;
        self.last_error = None;
        self.last_timestamp = None;
    }

    pub fn complete_refresh(&mut self) {
        self.is_refreshing = false;
        self.last_refresh = Some(Instant::now());
        self.last_timestamp = Some(chrono::offset::Local::now());
    }

    pub fn set_refresh_err(&mut self, err: String) {
        self.is_refreshing = false;
        self.last_error = Some(err);
        self.last_refresh = Some(Instant::now());
        self.last_timestamp = Some(chrono::offset::Local::now());
    }

    pub fn toggle_league(&mut self) {
        self.league = match self.league {
            League::Wbc => League::Mlb,
            League::Mlb => League::Wbc,
        };
        log::info!("Toggled league to: {:?}", self.league);
    }
}