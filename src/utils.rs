use crate::model::{
    GameSummary, GameStatus, League, BaseState
};

pub fn get_mock_data() -> Vec<GameSummary> {
    vec![
        GameSummary {
            game_id: "1".to_string(),
            game_date: "2026-03-28T18:15Z".to_string(),
            odds: None,
            league: League::Wbc,
            away_team_abbrev: "PAN".to_string(),
            home_team_abbrev: "PUR".to_string(),
            away_team_score: Some(2),
            home_team_score: Some(0),
            game_status: GameStatus::InProgress,
            status_text: "Top 8".to_string(),
            balls: Some(0),
            strikes: Some(2),
            outs: Some(2),
            bases: Some(BaseState {
                on_first: true,
                on_second: false,
                on_third: false,
            }),
        },
        GameSummary {
            game_id: "2".to_string(),
            game_date: "2026-03-28T18:15Z".to_string(),
            odds: None,
            league: League::Wbc,
            away_team_abbrev: "COL".to_string(),
            home_team_abbrev: "CAN".to_string(),
            away_team_score: Some(3),
            home_team_score: Some(8),
            game_status: GameStatus::Final,
            status_text: "Final".to_string(),
            balls: None,
            strikes: None,
            outs: None,
            bases: None,
        },
        GameSummary {
            game_id: "3".to_string(),
            game_date: "2026-03-28T18:15Z".to_string(),
            odds: None,
            league: League::Wbc,
            away_team_abbrev: "TPE".to_string(),
            home_team_abbrev: "KOR".to_string(),
            away_team_score: None,
            home_team_score: None,
            game_status: GameStatus::Scheduled,
            status_text: "10:00 PM".to_string(),
            balls: None,
            strikes: None,
            outs: None,
            bases: None,
        },
    ]
}