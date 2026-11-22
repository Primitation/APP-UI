//! ATOM: Charts.

use crate::{
    self as q, Status,
    consts::{glyph as G, rules as R},
};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::Widget,
};

const EIGHTHS: [&str; 9] = G::EIGHTHS_UP;

/// One-row trend: `▂▃▅▇▆▃▂`. The last `width` values are shown.
/// `budget`: values above it are drawn in the error color (frame time over 16.6 ms, CPU over 80 %…).
#[derive(Clone, Copy, Debug)]
pub struct Sparkline<'a> {
    pub values: &'a [f64],
    pub max: Option<f64>,
    pub budget: Option<f64>,
}

impl<'a> Sparkline<'a> {
    pub fn new(values: &'a [f64]) -> Self { Self { values, max: None, budget: None } }
    pub fn max(mut self, m: f64) -> Self { self.max = Some(m); self }
    pub fn budget(mut self, b: f64) -> Self { self.budget = Some(b); self }
}

impl Widget for Sparkline<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Histogram { values: self.values, max: self.max, budget: self.budget }.render(Rect { height: 1, ..area }, buf);
    }
}

/// Vertical bars, several rows tall (eighth-block precision). Use for distributions and frame graphs.
/// With a `budget`, a faint `┄` line marks it and bars above it turn red.
#[derive(Clone, Copy, Debug)]
pub struct Histogram<'a> {
    pub values: &'a [f64],
    pub max: Option<f64>,
    pub budget: Option<f64>,
}

impl Widget for Histogram<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width == 0 || area.height == 0 { return; }
        let vals = &self.values[self.values.len().saturating_sub(area.width as usize)..];
        let max = self.max.unwrap_or_else(|| vals.iter().cloned().fold(0.0, f64::max)).max(f64::EPSILON);
        let h = area.height as f64;
        if let Some(b) = self.budget.filter(|b| *b < max) {
            let row = area.bottom() - 1 - ((b / max * h).floor() as u16).min(area.height - 1);
            for x in area.x..area.right() {
                buf[(x, row)].set_symbol("┄").set_style(Style::new().fg(q::c(q::BORDER)));
            }
        }
        for (i, v) in vals.iter().enumerate() {
            let x = area.x + i as u16;
            let over = self.budget.is_some_and(|b| *v > b);
            let style = if over { Status::Err.style() } else { q::accent() };
            let eighths = ((v / max).clamp(0.0, 1.0) * h * 8.0).round() as usize;
            for r in 0..area.height as usize {
                let e = eighths.saturating_sub(r * 8).min(8);
                if e == 0 { break; }
                buf[(x, area.bottom() - 1 - r as u16)].set_symbol(EIGHTHS[e]).set_style(style);
            }
        }
    }
}

/// One horizontal bar (for bar charts): `██████▌` with eighth precision, on a faint track.
#[derive(Clone, Copy, Debug)]
pub struct Bar {
    pub fraction: f64,
    /// Over budget → error color.
    pub over: bool,
}

impl Widget for Bar {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let w = area.width as usize;
        let e = (self.fraction.clamp(0.0, 1.0) * w as f64 * G::EIGHTHS as f64).round() as usize;
        let style = if self.over { Status::Err.style() } else { q::accent() };
        bar_line(e, w, style).render(Rect { height: 1, ..area }, buf);
    }
}

fn bar_line(e: usize, w: usize, style: Style) -> Line<'static> {
    const H: [&str; 9] = G::EIGHTHS_RIGHT;
    const N: usize = G::EIGHTHS;
    {
        let mut s = "█".repeat(e / N);
        s.push_str(H[e % N]);
        let used = e / N + usize::from(!e.is_multiple_of(N));
        Line::from(vec![Span::styled(s, style), Span::styled("·".repeat(w.saturating_sub(used)), Style::new().fg(q::c(q::BORDER)))])
    }
}

/// Usage of a limit (memory, disk, CPU quota): accent below 80 %, yellow from 80 %, red at or over the limit.
#[derive(Clone, Copy, Debug)]
pub struct UsageBar(pub f64);

impl UsageBar {
    pub const WARN_AT: f64 = R::USAGE_WARN;
    pub fn status(self) -> Option<Status> {
        if self.0 >= R::USAGE_ERR { Some(Status::Err) } else if self.0 >= Self::WARN_AT { Some(Status::Warn) } else { None }
    }
}

impl Widget for UsageBar {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Thin line (like Slider), half-cell precision: calmer than solid blocks for many meters on one screen.
        let w = area.width as usize;
        let halves = (self.0.clamp(0.0, 1.0) * w as f64 * 2.0).round() as usize;
        let style = self.status().map_or(q::accent(), Status::style);
        let mut s = G::LINE_FULL.repeat(halves / 2);
        if halves % 2 == 1 { s.push(G::LINE_HALF); }
        let used = halves / 2 + halves % 2;
        Line::from(vec![Span::styled(s, style), Span::styled(G::LINE_TRACK.repeat(w.saturating_sub(used)), Style::new().fg(q::c(q::BORDER)))])
            .render(Rect { height: 1, ..area }, buf);
    }
}

/// Change between two measures: `▲ +12.0%`. Green when it went the good way, red when not.
/// `higher_is_better`: true for FPS / score, false for frame time / memory / errors.
pub fn delta_badge(change: f64, higher_is_better: bool) -> Span<'static> {
    if change.abs() < 0.0005 {
        return Span::styled("= 0%", q::muted());
    }
    let good = (change > 0.0) == higher_is_better;
    let arrow = if change > 0.0 { "▲" } else { "▼" };
    let style = if good { Status::Ok.style() } else { Status::Err.style() };
    Span::styled(format!("{arrow} {:+.1}%", change * 100.0), style)
}
