//! ATOM: Slider.

use super::*;
use crate::{
    self as q,
    consts::glyph as G,
};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Widget,
};

/// `━━━━━━●──────` A value between min and max. Put the number next to it (NumberField or a label).
#[derive(Clone, Copy, Debug)]
pub struct Slider {
    pub value: f64,
    pub min: f64,
    pub max: f64,
    pub state: State,
}

impl Slider {
    pub fn new(value: f64, min: f64, max: f64) -> Self { Self { value, min, max, state: State::Normal } }
    pub fn focused(mut self, on: bool) -> Self { if on { self.state = State::Focused } self }
    pub fn disabled(mut self, on: bool) -> Self { if on { self.state = State::Disabled } self }
}

impl Widget for Slider {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let w = area.width as usize;
        if w < 2 { return; }
        let f = ((self.value - self.min) / (self.max - self.min).max(f64::EPSILON)).clamp(0.0, 1.0);
        let pos = (f * (w - 1) as f64).round() as usize;
        let (fill, knob) = match self.state {
            State::Disabled => (q::faint(), q::faint()),
            State::Focused => (q::accent(), Style::new().fg(q::c(q::QUAG_HI)).add_modifier(Modifier::BOLD)),
            State::Normal => (q::accent(), q::accent()),
        };
        Line::from(vec![
            Span::styled(G::LINE_FULL.repeat(pos), fill),
            Span::styled(if self.state == State::Focused { "◆" } else { "●" }, knob),
            Span::styled("─".repeat(w - pos - 1), Style::new().fg(q::c(q::BORDER))),
        ])
        .render(Rect { height: 1, ..area }, buf);
    }
}
