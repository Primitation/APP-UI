//! ATOM: Divider & Progress.

use crate::{
    self as q, Status,
    consts::glyph as G,
};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::Widget,
};

pub struct Divider;
impl Widget for Divider {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Line::styled("─".repeat(area.width as usize), Style::new().fg(q::c(q::BORDER)))
            .render(Rect { height: 1, ..area }, buf);
    }
}

/// Thin progress bar: `━━━━━━────` (numbers go next to it, never on it).
pub struct Progress(pub f64);
impl Widget for Progress {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let w = area.width as usize;
        let filled = ((self.0.clamp(0.0, 1.0)) * w as f64).round() as usize;
        Line::from(vec![
            Span::styled(G::LINE_FULL.repeat(filled), q::accent()),
            Span::styled(G::LINE_FULL.repeat(w - filled), Style::new().fg(q::c(q::BORDER))),
        ])
        .render(Rect { height: 1, ..area }, buf);
    }
}

/// Smooth progress: fills in eighths of a cell (`█████▋`), the Quag fill on a surface track.
/// For anything that moves by small amounts (downloads, copies). Numbers go next to it, never on it.
pub struct SmoothProgress(pub f64);
impl SmoothProgress {
    /// Filled eighths for a bar `width` cells wide.
    pub fn eighths(&self, width: u16) -> usize {
        (self.0.clamp(0.0, 1.0) * width as f64 * G::EIGHTHS as f64).round() as usize
    }
}
impl Widget for SmoothProgress {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let w = area.width as usize;
        let e = self.eighths(area.width);
        let mut s = G::EIGHTHS_RIGHT[G::EIGHTHS].repeat(e / G::EIGHTHS);
        s.push_str(G::EIGHTHS_RIGHT[e % G::EIGHTHS]);
        let used = e / G::EIGHTHS + usize::from(!e.is_multiple_of(G::EIGHTHS));
        let track = Style::new().bg(q::c(q::SURFACE));
        Line::from(vec![Span::styled(s, q::accent().patch(track)), Span::styled(" ".repeat(w.saturating_sub(used)), track)])
            .render(Rect { height: 1, ..area }, buf);
    }
}

/// Stepped progress: one segment per step (`━━━━ ━━━━ ━━╸─ ──── ────`). Done steps are full,
/// the current one fills with its own progress (`current`, 0–1), the rest are track. `failed`: the current step is red.
/// For jobs with known steps (deploy, install, wizard). Say which step next to it: "Step 3 of 5 · Health check".
pub struct StepProgress {
    pub total: usize,
    pub done: usize,
    pub current: f64,
    pub failed: bool,
}
impl StepProgress {
    /// Each step's cells `(x, width)` in a bar `width` wide, a 1-cell gap between steps.
    pub fn segments(&self, width: u16) -> Vec<(u16, u16)> {
        let n = self.total.max(1) as u16;
        let room = width.saturating_sub(n - 1);
        (0..n).map(|i| {
            let x0 = i * room / n;
            let x1 = (i + 1) * room / n;
            (x0 + i, x1 - x0)
        }).collect()
    }
}
impl Widget for StepProgress {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let track = Style::new().fg(q::c(q::BORDER));
        let fill = if self.failed { Status::Err.style() } else { q::accent() };
        for (i, (x, w)) in self.segments(area.width).into_iter().enumerate() {
            let r = Rect { x: area.x + x, y: area.y, width: w, height: 1 };
            let (full, half) = match i.cmp(&self.done) {
                std::cmp::Ordering::Less => (w as usize, false),
                std::cmp::Ordering::Equal => {
                    let halves = (self.current.clamp(0.0, 1.0) * w as f64 * 2.0).round() as usize;
                    (halves / 2, halves % 2 == 1)
                }
                std::cmp::Ordering::Greater => (0, false),
            };
            let style = if i < self.done { q::accent() } else { fill };
            let mut spans = vec![Span::styled(G::LINE_FULL.repeat(full), style)];
            if half { spans.push(Span::styled(G::LINE_HALF.to_string(), style)); }
            spans.push(Span::styled(G::LINE_TRACK.repeat((w as usize).saturating_sub(full + usize::from(half))), track));
            Line::from(spans).render(r, buf);
        }
    }
}
