use crate::model::*;
use crate::utils::get_mock_data;

pub fn fetch_games(_league: &League) -> Result<Vec<GameSummary>, String> {
    Ok(get_mock_data())
}