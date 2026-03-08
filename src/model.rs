#[derive(Clone)]
pub enum League {
    Wbc,
    Mlb,
}

pub enum GameStatus {
    Scheduled,
    InProgress,
    Final,
    // Postponed,
    // Canceled,
    // Delayed,
}

pub struct BaseState {
    pub on_first: bool,
    pub on_second: bool,
    pub on_third: bool,
}

pub struct GameSummary {
    pub game_id: String,
    pub league: League,

    pub away_team_abbrev: String,
    pub home_team_abbrev: String,

    pub away_team_score: Option<u8>,
    pub home_team_score: Option<u8>,

    pub game_status: GameStatus,
    pub status_text: String,

    pub balls: Option<u8>,
    pub strikes: Option<u8>,
    pub outs: Option<u8>,

    pub bases: Option<BaseState>,
}
