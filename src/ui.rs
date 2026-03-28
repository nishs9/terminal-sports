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
        .title("Terminal Sports")
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

    let list_rect = Rect {
        x: list_area.x,
        y: list_area.y,
        width: list_area.width,
        height: list_area.height,
    };
    let list_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(10),
            Constraint::Percentage(40),
            Constraint::Percentage(50),
        ])
        .split(list_rect);
    let list_sidebar = list_chunks[0];
    let list_main = list_chunks[1];
    let list_game_details = list_chunks[2];

    if app_state.games.is_empty() {
        let empty = Paragraph::new("No games found");
        frame.render_widget(empty, list_main);
        return;
    } else {
        let sidebar_items: Vec<ListItem> = vec![
            ListItem::new("MLB"),
            ListItem::new("WBC")
        ];

        let sidebar_list = List::new(sidebar_items)
            .highlight_symbol("# ")
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::UNDERLINED | Modifier::BOLD),
            );

        let mut sidebar_list_state = ListState::default();
        match app_state.league {
            League::Mlb => sidebar_list_state.select(Some(0)),
            League::Wbc => sidebar_list_state.select(Some(1)),
        }

        frame.render_stateful_widget(sidebar_list, list_sidebar, &mut sidebar_list_state);

        let game_items: Vec<ListItem> = app_state
        .games
        .iter()
        .map(|game| ListItem::new(format_game_summary(game)))
        .collect();

        let game_list = List::new(game_items)
            .highlight_symbol("> ")
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::REVERSED | Modifier::BOLD),
            );

        let mut list_state = ListState::default();
        list_state.select(Some(app_state.selected_game_idx));

        let game_details_box = Block::default()
            .title("Game Details")
            .borders(Borders::ALL);

        let selected_game = &app_state.games[app_state.selected_game_idx];
        let _game_details = Paragraph::new(format_game_details(selected_game)).block(game_details_box);

        frame.render_stateful_widget(game_list, list_main, &mut list_state);
        frame.render_widget(_game_details, list_game_details);
    }

    let status_bar_text = build_status_line(app_state);
    let status = Paragraph::new(status_bar_text);
    frame.render_widget(status, status_area);
}

fn format_game_details(game: &GameSummary) -> Text<'static> {
    let mut lines = Vec::new();
    match &game.odds {
        Some(odds) => {
            lines.push(Line::from(format!(
                "Moneyline: {}", odds.moneyline
            )));
            lines.push(Line::from(format!(
                "Spread: {}", odds.spread
            )));
            lines.push(Line::from(format!(
                "O/U: {}", odds.over_under
            )));
        }
        None => {
            lines.push(Line::from("No odds available at this time"));
        }
    }
    Text::from(lines)
}

fn build_status_line(app_state: &AppState) -> String {
    let league = match app_state.league {
        League::Wbc => "WBC",
        League::Mlb => "MLB",
    };

    let refresh_status = if app_state.is_refreshing {
        "Refreshing...".to_string()
    } else if let Some(err) = &app_state.last_error {
        format!("Error: {}", err)
    } else if app_state.last_refresh.is_some() {
        let last_timestamp = app_state.last_timestamp.unwrap();
        let formatted_timestamp = last_timestamp.format("%Y-%m-%d %H:%M:%S").to_string();
        format!("Last refresh - {}", formatted_timestamp)
    } else {
        "Last refresh: Never".to_string()
    };

    format!(
        "Current League: {} | {} | ↑/↓ move | r - refresh | l - toggle league (WBC/MLB) | q - quit",
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
                "{} @ {} | {} | {}",
                game.away_team_abbrev, 
                game.home_team_abbrev,
                game.status_text,
                game.game_date,
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