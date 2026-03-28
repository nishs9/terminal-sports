//! In-game situation: diamond, runners, count (B/S), and outs — drawn with [`Canvas`].
//!
//! Terminal cells are much **taller** than they are wide, so we scale down world **y** so the
//! infield reads as a square diamond instead of a tall arrow.

use crate::model::GameSummary;
use ratatui::{
    style::Color,
    symbols::Marker,
    widgets::{
        Widget, canvas::{Canvas, Circle, Context, Line, Points, Rectangle}
    },
};

const LINE_COLOR: Color = Color::LightBlue;
const EMPTY: Color = Color::DarkGray;
const ON: Color = Color::Yellow;

/// Squash world Y so horizontal and vertical distances look balanced on screen.
const Y_SCALE: f64 = 0.52;

#[inline]
fn yw(y: f64) -> f64 {
    y * Y_SCALE
}

/// Home at origin; 1st east; 2nd north; 3rd west (`y` increases upward in canvas space).
fn draw_diamond(ctx: &mut Context) {
    let h = (0.0, yw(0.0));
    let first = (9.0, yw(0.0));
    let second = (0.0, yw(9.0));
    let third = (-9.0, yw(0.0));
    ctx.draw(&Line::new(h.0, h.1, first.0, first.1, LINE_COLOR));
    ctx.draw(&Line::new(first.0, first.1, second.0, second.1, LINE_COLOR));
    ctx.draw(&Line::new(second.0, second.1, third.0, third.1, LINE_COLOR));
    ctx.draw(&Line::new(third.0, third.1, h.0, h.1, LINE_COLOR));
}

fn draw_base(ctx: &mut Context, x: f64, y: f64, occupied: bool) {
    let color = if occupied { ON } else { EMPTY };
    ctx.draw(&Rectangle {
        x,
        y,
        width: 3.0,
        height: 2.0,
        color,
    });
}

/// `filled` = how many leading circles use `ON` (rest `EMPTY`).
fn draw_dot_row(ctx: &mut Context, y: f64, start_x: f64, spacing: f64, max: usize, filled: usize) {
    let filled = filled.min(max);
    for i in 0..max {
        let x = start_x + spacing * i as f64;
        let color = if i < filled { ON } else { EMPTY };
        ctx.draw(&Points {
            coords: &[(x, y)],
            color,
        });
    }
}

/// World Y for B / S / O rows — **must match** the `y` passed to `draw_dot_row` for each row.
const Y_ROW_BALLS: f64 = 0.0;
const Y_ROW_STRIKES: f64 = -4.0;
const Y_ROW_OUTS: f64 = -8.0;

fn draw_labels(ctx: &mut Context) {
    // Same y as the dot rows so labels line up with their indicators (print + draw share coords).
    ctx.print(-13.5, yw(Y_ROW_BALLS), "B");
    ctx.print(-13.5, yw(Y_ROW_STRIKES), "S");
    ctx.print(-13.5, yw(Y_ROW_OUTS), "O");
}

/// Builds a canvas widget for the current pitch / base situation.
pub fn situation_canvas(game: &GameSummary) -> impl Widget {
    let balls = game.balls.unwrap_or(0).min(4) as usize;
    let strikes = game.strikes.unwrap_or(0).min(3) as usize;
    let outs = game.outs.unwrap_or(0).min(3) as usize;
    let (on1, on2, on3) = match &game.bases {
        Some(b) => (b.on_first, b.on_second, b.on_third),
        None => (false, false, false),
    };

    Canvas::default()
        .x_bounds([-18.0, 18.0])
        .y_bounds([-10.0, 8.0])
        // Dots read as round “lights”; Braille often looks like tall streaks in narrow panes.
        .marker(Marker::Block)
        .paint(move |ctx: &mut Context<'_>| {
            draw_base(ctx, 4.5, yw(5.0), on1);
            draw_base(ctx, 0.0, yw(9.0), on2);
            draw_base(ctx, -4.5, yw(5.0), on3);

            draw_labels(ctx);
            draw_dot_row(ctx, yw(Y_ROW_BALLS), -4.0, 2.0, 4, balls);
            draw_dot_row(ctx, yw(Y_ROW_STRIKES), -4.0, 2.0, 3, strikes);
            draw_dot_row(ctx, yw(Y_ROW_OUTS), -4.0, 2.0, 3, outs);
        })
}
