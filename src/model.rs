use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum League {
    Mlb,
    Wbc,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum GameStatus {
    Scheduled,
    InProgress,
    Final,
    // Postponed,
    // Canceled,
    // Delayed,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct BaseState {
    pub on_first: bool,
    pub on_second: bool,
    pub on_third: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct GameSummary {
    pub game_id: String,
    pub game_date: String,
    pub league: League,

    pub odds: Option<GameOdds>,
    pub details: Option<GameDetails>,

    pub away_team_abbrev: String,
    pub home_team_abbrev: String,

    pub short_inning: String,
    pub full_inning: String,

    pub away_team_score: Option<u64>,
    pub home_team_score: Option<u64>,

    pub home_linescore: Vec<u64>,
    pub away_linescore: Vec<u64>,

    pub game_status: GameStatus,
    pub status_text: String,

    pub balls: Option<u64>,
    pub strikes: Option<u64>,
    pub outs: Option<u64>,

    pub bases: Option<BaseState>,

    pub winning_pitcher: Option<String>,
    pub losing_pitcher: Option<String>
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct GameOdds {
    pub moneyline: String,
    pub spread: String,
    pub over_under: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct GameDetails {
    pub last_play: String,
    pub pitcher: Player,
    pub batter: Player,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Player {
    pub id: String,
    pub full_name: String
}
