mod api;
mod model;
mod state;
mod ui;
mod app;
use ratatui::{
    backend::CrosstermBackend,
    prelude::*,
    widgets::{
        Block, Borders,
        List, ListItem,
        ListState, Paragraph
    },
    Terminal
};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{
        disable_raw_mode, 
        enable_raw_mode, 
        EnterAlternateScreen, 
        LeaveAlternateScreen,
    },
};
use model::{BaseState, GameStatus, GameSummary, League};
use state::AppState;
use std::io;

fn main() -> Result<(), io::Error> {
    // enable raw mode to capture keystrokes
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mock_games = mock_games();
    let mut app_state = AppState::new(mock_games);

    let result = run_app(&mut terminal, &mut app_state);
    
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app_state: &mut AppState,
) -> Result<(), io::Error> {
    loop {
        terminal.draw(|frame| draw_tui(frame, app_state))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Up => app_state.move_up(),
                KeyCode::Down => app_state.move_down(),
                _ => {}
            }
        }
    }
    Ok(())
}

fn draw_tui(frame: &mut Frame, app_state: &AppState) {
    let area = frame.size();

    let outer = Block::default()
        .title("Live Baseball Scoreboard TUI")
        .borders(Borders::ALL);

    frame.render_widget(outer, area);

    let inner = Rect {
        x: area.x + 1,
        y: area.y + 1,
        width: area.width.saturating_sub(2),
        height: area.height.saturating_sub(2),
    };

    if app_state.games.is_empty() {
        let empty = Paragraph::new("No games found");
        frame.render_widget(empty, inner);
        return;
    }

    let items: Vec<ListItem> = app_state
        .games
        .iter()
        .map(|game| ListItem::new(format_game_summary(game)))
        .collect();

    let list = List::new(items)
        .highlight_symbol("> ")
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED));

    let mut list_state = ListState::default();
    list_state.select(Some(app_state.selected_game_idx));

    frame.render_stateful_widget(list, inner, &mut list_state);
}

fn format_game_summary(game: &GameSummary) -> Text<'static> {
    let mut lines = Vec::new();

    match game.game_status {
        GameStatus::InProgress => {
            let away_score = game.away_team_score.unwrap_or(0);
            let home_score = game.home_team_score.unwrap_or(0);
            
            let out = game.outs.unwrap_or(0);
            let balls = game.balls.unwrap_or(0);
            let strikes = game.strikes.unwrap_or(0);

            lines.push(Line::from(format!(
                "{} {} @ {} {} | {} | {} outs | {}-{}",
                game.away_team_abbrev, away_score,
                game.home_team_abbrev, home_score,
                game.status_text,
                out,
                balls,
                strikes,
            )));
            
            if let Some(bases) = &game.bases {
                lines.push(Line::from(format!(
                    "      {}",
                    base_char(bases.on_second)
                )));
                lines.push(Line::from(format!(
                    "    {}  {}",
                    base_char(bases.on_third),
                    base_char(bases.on_first)
                )));
            }
        }
        GameStatus::Final => {
            let away_score = game.away_team_score.unwrap_or(0);
            let home_score = game.home_team_score.unwrap_or(0);

            lines.push(Line::from(format!(
                "{} {} @ {} {} | Final",
                game.away_team_abbrev, away_score,
                game.home_team_abbrev, home_score,
            )));
        }
        GameStatus::Scheduled => {
            lines.push(Line::from(format!(
                "{} @ {} | {} | Scheduled",
                game.away_team_abbrev, 
                game.home_team_abbrev,
                game.status_text,
            )));
        }
    }
    Text::from(lines)
}

fn base_char(occupied: bool) -> &'static str {
    if occupied {
        "⬛"
    } else {
        "⬜"
    }
}

fn mock_games() -> Vec<GameSummary> {
    vec![
        GameSummary {
            game_id: "1".to_string(),
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
