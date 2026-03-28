use crate::model::{GameSummary, League, BaseState, GameStatus, GameOdds};
use crate::utils::get_mock_data;
use rand::RngExt;
use serde_json::Value;
use std::{
    fs,
    path::PathBuf
};
use regex::Regex;
use std::sync::LazyLock;

pub struct ApiClient {
    req_counter: u64,
    next_snapshot: u64,
    snapshot_dir: PathBuf,
}

impl ApiClient {
    pub fn new(snapshot_dir: impl Into<PathBuf>) -> Self {
        let mut rng = rand::rng();
        Self {
            req_counter: 0,
            next_snapshot: rng.random_range(5..=10),
            snapshot_dir: snapshot_dir.into(),
        }
    }

    pub fn fetch_games(&mut self, league: &League) -> Result<Vec<GameSummary>, String> {
        self.req_counter += 1;

        let url = get_league_url(league);
        log::info!("Fetching {:?} scoreboard from {}", league, url);

        match fetch_scoreboard_json(url)
            .and_then(|json| create_game_summaries(&json, league)) {
                Ok(games) => {
                    log::info!("Successfully fetched games for {:?}", league);
                    if self.req_counter >= self.next_snapshot {
                        if let Err(err) = self.save_snapshot(league, &games) {
                            log::error!("Failed to save snapshot for {:?}: {}", league, err);
                        } else {
                            log::info!("Saved snapshot for {:?}", league);
                        }

                        let mut rng = rand::rng();
                        self.next_snapshot = rng.random_range(5..=10);
                        log::debug!("Next snapshot in {} requests", self.next_snapshot);
                    }
                    Ok(games)
                }
                Err(err) => {
                    log::error!("Failed to fetch games for {:?}: {}", league, err);

                    match self.load_snapshot(league) {
                        Ok(games) => {
                            log::warn!("Using cached snapshot for {:?}", league);
                            Ok(games)
                        }
                        Err(err) => Err(format!(
                            "Failed to load cached snapshot for {:?}: {}", league, err)
                        ),
                    }
                }
            }
    }

    pub fn fetch_mock_games(&mut self) -> Result<Vec<GameSummary>, String> {
        Ok(get_mock_data())
    }

    fn save_snapshot(&mut self, league: &League, games: &[GameSummary]) -> Result<(), String> {
        fs::create_dir_all(&self.snapshot_dir)
            .map_err(|err| format!("Failed to create snapshot directory: {}", err))?;

        let snapshot_path = self.get_snapshot_path(league);
        let tmp_path = snapshot_path.with_extension("tmp");

        let json = serde_json::to_string_pretty(games)
            .map_err(|err| format!("Failed to serialize snapshot: {}", err))?;

        fs::write(&tmp_path, json)
            .map_err(|err| format!("Failed to write snapshot: {}", err))?;
        fs::rename(&tmp_path, &snapshot_path)
            .map_err(|err| format!("Failed to rename snapshot: {}", err))?;

        Ok(())
    }

    fn load_snapshot(&self, league: &League) -> Result<Vec<GameSummary>, String> {
        let path = self.get_snapshot_path(league);

        let contents = fs::read_to_string(&path)
            .map_err(|err| format!("Failed to read snapshot: {}", err))?;
        
        let games = serde_json::from_str(&contents)
            .map_err(|err| format!("Failed to parse snapshot: {}", err))?;

        Ok(games)
    }

    fn get_snapshot_path(&self, league: &League) -> PathBuf {
        let filename = match league {
            League::Wbc => "wbc_scoreboard.json",
            League::Mlb => "mlb_scoreboard.json",
        };

        self.snapshot_dir.join(filename)
    }
}

const GAME_STATUS_IN: &str = "in";
const GAME_STATUS_PRE: &str = "pre";
const GAME_STATUS_POST: &str = "post";

fn parse_score(v: &Value) -> Option<u64> {
    v.as_u64()
        .or_else(|| v.as_str().and_then(|s| s.parse().ok()))
}

fn get_game_status(game_status: &str) -> GameStatus {
    match game_status {
        GAME_STATUS_IN => GameStatus::InProgress,
        GAME_STATUS_PRE => GameStatus::Scheduled,
        GAME_STATUS_POST => GameStatus::Final,
        _ => panic!("Invalid game status: {}", game_status),
    }
}
fn get_league_url(league: &League) -> &'static str {
    match league {
        League::Wbc => "https://site.api.espn.com/apis/site/v2/sports/baseball/world-baseball-classic/scoreboard",
        League::Mlb => "https://site.api.espn.com/apis/site/v2/sports/baseball/mlb/scoreboard",
    }
}

fn fetch_scoreboard_json(url: &str) -> Result<Value, String> {
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(url)
        .send()
        .map_err(|err| format!("Request failed: {}", err))?;

    let response = response
        .error_for_status()
        .map_err(|err| format!("Bad HTTP Status: {}", err))?;

    let json = response
        .json::<Value>()
        .map_err(|err| format!("Failed to parse JSON: {}", err))?;

    log::info!("Successfully fetched JSON!");
    Ok(json)
}

fn create_game_summaries(root: &Value, league: &League) -> Result<Vec<GameSummary>, String> {
    let events = root["events"]
        .as_array()
        .ok_or("missing events array")?;

    let games: Vec<GameSummary> = events
        .iter()
        .filter_map(|event| parse_game_data(event, league))
        .collect();

    Ok(games)
}

static GAME_DATE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"^(\d{4})-(\d{2})-(\d{2})T(\d{2}):(\d{2})(?::(\d{2}))?(Z|[+-]\d{2}:?\d{2})$",
    )
    .expect("GAME_DATE_RE")
});

/// Parse + format the game date from the ESPN API
fn parse_game_date(date: String) -> Result<String, String> {
    log::debug!("Parsing game date: {}", date);
    let caps = GAME_DATE_RE
        .captures(&date)
        .ok_or_else(|| format!("unrecognized date format: {date}"))?;

    let m = caps[2].parse::<i32>().map_err(|e| e.to_string())?;
    let d = caps[3].parse::<i32>().map_err(|e| e.to_string())?;
    let mut hour24 = caps[4].parse::<i32>().map_err(|e| e.to_string())?;
    // temporary hack: adjusting UTC -> PST local time
    hour24 -= 7;
    if hour24 < 0 {
        hour24 += 24;
    }
    let minute = caps[5].parse::<i32>().map_err(|e| e.to_string())?;

    let (hour12, am_pm) = if hour24 % 12 == 0 {
        (12i32, if hour24 < 12 { "AM" } else { "PM" })
    } else {
        (hour24 % 12, if hour24 < 12 { "AM" } else { "PM" })
    };

    Ok(format!(
        "{m:02}-{d:02} @ {hour12:02}:{minute:02} {am_pm} PST"
    ))
}

fn parse_game_odds(odds: &Value) -> Option<GameOdds> {
    // Check for existence and type of all required fields
    // Return None (fail parsing odds) if anything isn't good

    // Moneyline
    let moneyline_val = &odds["details"];
    log::debug!("Odds: {:?}", moneyline_val);
    let moneyline = match moneyline_val.as_str() {
        Some(s) => s.to_string(),
        None => {
            log::warn!("Failed to parse moneyline from odds: {:?}", odds);
            return None;
        }
    };

    // Spread
    let spread_val = &odds["spread"];
    log::debug!("Odds: {:?}", spread_val);
    let spread = match spread_val.as_number() {
        Some(s) => s.to_string(),
        None => {
            log::warn!("Failed to parse spread from odds: {:?}", odds);
            return None;
        }
    };

    // Over/Under
    let over_under_val = &odds["overUnder"];
    log::debug!("Odds: {:?}", over_under_val);
    let over_under = match over_under_val.as_number() {
        Some(s) => s.to_string(),
        None => {
            log::warn!("Failed to parse overUnder from odds: {:?}", odds);
            return None;
        }
    };

    Some(GameOdds {
        moneyline,
        spread,
        over_under,
    })
}

fn parse_game_data(event: &Value, league: &League) -> Option<GameSummary> {
    let game_id = event["id"].as_str()?.to_string();
    let raw_date = event["date"].as_str()?.to_string();
    let game_date = match parse_game_date(raw_date) {
        Ok(date) => date,
        Err(err) => {
            log::error!("Failed to parse game date: {}", err);
            return None;
        }
    };

    let odds = parse_game_odds(&event["competitions"][0]["odds"][0]);

    let competition = &event["competitions"][0];
    let home_team = &competition["competitors"][0];
    let away_team = &competition["competitors"][1];

    let home_team_abbrev = home_team["team"]["abbreviation"].as_str()?.to_string();
    let away_team_abbrev = away_team["team"]["abbreviation"].as_str()?.to_string();
    let away_team_score = parse_score(&away_team["score"]);
    let home_team_score = parse_score(&home_team["score"]);

    let game_status = get_game_status(competition["status"]["type"]["state"].as_str()?);
    let status_text = competition["status"]["type"]["description"]
        .as_str()
        .unwrap_or("")
        .to_string();

    let balls: u64 = 0;
    let strikes: u64 = 0;
    let outs: u64 = 0;
    let bases: BaseState = BaseState {
        on_first: false,
        on_second: false,
        on_third: false,
    };

    let game_summary = match game_status {
        GameStatus::InProgress => {
            let situation = &competition["situation"];
            let balls = situation["balls"].as_u64();
            let strikes = situation["strikes"].as_u64();
            let outs = situation["outs"].as_u64();
            let bases = BaseState {
                on_first: situation["onFirst"].as_bool().unwrap_or(false),
                on_second: situation["onSecond"].as_bool().unwrap_or(false),
                on_third: situation["onThird"].as_bool().unwrap_or(false),
            };
            Some(GameSummary {
                game_id,
                game_date,
                league: league.clone(),
                odds,
                away_team_abbrev,
                home_team_abbrev,
                away_team_score,
                home_team_score,
                game_status,
                status_text: status_text.clone(),
                balls,
                strikes,
                outs,
                bases: Some(bases),
            })
        },
        _ => Some(GameSummary {
            game_id,
            game_date,
            league: league.clone(),
            odds,
            away_team_abbrev,
            home_team_abbrev,
            away_team_score,
            home_team_score,
            game_status,
            status_text: status_text.clone(),
            balls: Some(balls),
            strikes: Some(strikes),
            outs: Some(outs),
            bases: Some(bases),
        })
    };
    log::debug!("Parsed game summary: {:?}", game_summary);
    game_summary
}