//! ATOM: NumberField.

use super::*;
use crate::{
    self as q,
    consts::layout as L,
};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::Widget,
};

/// A number with its unit: `◂ 16.6 ms ▸`. ←/→ (or drag on desktop) changes it by `step`.
/// Digits are right-aligned so values don't jump while changing.
#[derive(Clone, Debug)]
pub struct NumberField<'a> {
    pub value: f64,
    pub unit: &'a str,
    pub decimals: usize,
    pub min: f64,
    pub max: f64,
    pub state: State,
    pub invalid: bool,
}

impl<'a> NumberField<'a> {
    pub fn new(value: f64, unit: &'a str) -> Self {
        Self { value, unit, decimals: 0, min: f64::MIN, max: f64::MAX, state: State::Normal, invalid: false }
    }
    pub fn decimals(mut self, d: usize) -> Self { self.decimals = d; self }
    pub fn range(mut self, min: f64, max: f64) -> Self { self.min = min; self.max = max; self }
    pub fn focused(mut self, on: bool) -> Self { if on { self.state = State::Focused } self }
    pub fn disabled(mut self, on: bool) -> Self { if on { self.state = State::Disabled } self }
    pub fn invalid(mut self, on: bool) -> Self { self.invalid = on; self }
    /// The value stepped by `delta`, clamped to the range (the app calls this on ←/→).
    pub fn stepped(&self, delta: f64) -> f64 { (self.value + delta).clamp(self.min, self.max) }
    pub fn text(&self) -> String {
        let n = format!("{:.*}", self.decimals, self.value);
        if self.unit.is_empty() { n } else { format!("{n} {}", self.unit) }
    }
}

impl Widget for NumberField<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area = Rect { height: 1, ..area };
        let focused = self.state == State::Focused;
        let bg = q::c(if focused { q::RAISED } else { q::SURFACE });
        buf.set_style(area, Style::new().bg(bg));
        let arrows = if focused { q::accent() } else { q::faint() };
        let at_min = self.value <= self.min;
        let at_max = self.value >= self.max;
        let text_s = if self.state == State::Disabled { q::faint() } else { q::text() };
        let t = self.text();
        let w = area.width as usize;
        let inner = w.saturating_sub(L::NUMBER_ARROWS as usize);
        Line::from(vec![
            Span::styled(" ◂ ", if at_min { q::faint() } else { arrows }),
            Span::styled(format!("{t:>inner$}"), text_s),
            Span::styled(" ▸ ", if at_max { q::faint() } else { arrows }),
        ])
        .render(area, buf);
        let bar = if self.invalid { q::c(q::ERR) } else if focused { q::c(q::QUAG) } else { bg };
        buf[(area.x, area.y)].set_symbol("▎").set_fg(bar);
    }
}
