use crate::state::AppState;
use crate::model::BaseState;
use ratatui::{
    prelude::{
        Frame, Layout, 
        Direction, Constraint,
        Rect, Line, Text,
        Style, Modifier
    },
    widgets::{
        Block, Borders,
        List, ListItem,
        ListState, Paragraph
    }
};
use crate::model::{League, GameSummary, GameStatus};

pub fn draw_tui(frame: &mut Frame, app_state: &AppState) {
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

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(inner);

    let list_area = chunks[0];
    let status_area = chunks[1];

    if app_state.games.is_empty() {
        let empty = Paragraph::new("No games found");
        frame.render_widget(empty, inner);
        return;
    } else {
        let items: Vec<ListItem> = app_state
        .games
        .iter()
        .map(|game| ListItem::new(format_game_summary(game)))
        .collect();

        let list = List::new(items)
            .highlight_symbol("> ")
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::REVERSED | Modifier::BOLD),
            );

        let mut list_state = ListState::default();
        list_state.select(Some(app_state.selected_game_idx));

        frame.render_stateful_widget(list, list_area, &mut list_state);
    }

    let status_bar_text = build_status_line(app_state);
    let status = Paragraph::new(status_bar_text);
    frame.render_widget(status, status_area);
}

fn build_status_line(app_state: &AppState) -> String {
    let league = match app_state.league {
        League::Wbc => "WBC",
        League::MlbSpring => "MLB Spring Training",
        League::MlbRegSzn => "MLB Regular Season",
    };

    let refresh_status = if app_state.is_refreshing {
        "Refreshing...".to_string()
    } else if let Some(err) = &app_state.last_error {
        format!("Error: {}", err)
    } else if app_state.last_refresh.is_some() {
        "Last refresh just now!".to_string()
    } else {
        "Last refresh: Never".to_string()
    };

    format!(
        "League: {} | {} | ↑/↓ move | r - refresh | q -quit",
        league,
        refresh_status,
    )
}

fn format_game_summary(game: &GameSummary) -> Text<'static> {
    let mut lines = Vec::new();

    // TODO: Clean up how we are formatting/rendering games here
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
            
            generate_and_set_base_string(&game.bases, &mut lines);
        }
        GameStatus::Final => {
            let away_score = game.away_team_score.unwrap_or(0);
            let home_score = game.home_team_score.unwrap_or(0);

            lines.push(Line::from(format!(
                "{} {} @ {} {} | Final",
                game.away_team_abbrev, away_score,
                game.home_team_abbrev, home_score,
            )));

            generate_and_set_base_string(&game.bases, &mut lines);
        }
        GameStatus::Scheduled => {
            lines.push(Line::from(format!(
                "{} @ {} | {} | Scheduled",
                game.away_team_abbrev, 
                game.home_team_abbrev,
                game.status_text,
            )));

            generate_and_set_base_string(&game.bases, &mut lines);
        }
    }
    Text::from(lines)
}

fn generate_and_set_base_string(bases: &Option<BaseState>, lines: &mut Vec<Line>) {
    match bases {
        Some(bases) => {
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
        None => {
            lines.push(Line::from(format!(
                "      {}",
                base_char(false)
            )));
            lines.push(Line::from(format!(
                "    {}  {}",
                base_char(false),
                base_char(false)
            )));
        }
    }
}

fn base_char(occupied: bool) -> &'static str {
    if occupied {
        "⬛"
    } else {
        "⬜"
    }
}