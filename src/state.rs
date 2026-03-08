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