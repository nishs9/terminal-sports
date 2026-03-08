use crate::model::{League, GameSummary};
use std::time::Instant;

pub struct AppState {
    pub league: League,
    pub games: Vec<GameSummary>,
    pub selected_game_idx: usize,

    pub last_refresh: Option<Instant>,
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
            is_refreshing: false,
            last_error: None,
        }
    }

    pub fn set_games(&mut self, games: Vec<GameSummary>) {
        self.games = games;
        self.selected_game_idx = 0;
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
}