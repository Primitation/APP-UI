//! ATOM: Input.

use super::*;
use crate::{
    self as q, Status,
    consts::layout as L,
};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::Widget,
};

/// One-line text input. Fills its whole area width.
#[derive(Clone, Debug, Default)]
pub struct Input<'a> {
    pub value: &'a str,
    pub placeholder: &'a str,
    pub state: State,
    pub invalid: bool,
}

impl<'a> Input<'a> {
    pub fn new(value: &'a str) -> Self { Self { value, ..Default::default() } }
    pub fn placeholder(mut self, p: &'a str) -> Self { self.placeholder = p; self }
    pub fn focused(mut self, on: bool) -> Self { if on { self.state = State::Focused } self }
    pub fn disabled(mut self, on: bool) -> Self { if on { self.state = State::Disabled } self }
    pub fn invalid(mut self, on: bool) -> Self { self.invalid = on; self }
}

impl Widget for Input<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area = Rect { height: 1, ..area };
        let focused = self.state == State::Focused;
        let bg = q::c(if focused { q::RAISED } else { q::SURFACE });
        buf.set_style(area, Style::new().bg(bg));
        let mut spans = vec![Span::raw(" ")];
        if self.value.is_empty() {
            spans.push(Span::styled(self.placeholder.to_string(), q::faint()));
        } else {
            let s = if self.state == State::Disabled { q::faint() } else { q::text() };
            spans.push(Span::styled(self.value.to_string(), s));
        }
        if focused {
            spans.push(Span::styled("▏", q::accent()));
        }
        Line::from(spans).render(area, buf);
        if self.invalid && area.width > L::FIELD_ICON {
            buf.set_string(area.right() - L::FIELD_ICON, area.y, "✗", Status::Err.style().bg(bg));
        }
        // focus / error underline: last row hint via a thin left bar
        let bar = if self.invalid { q::c(q::ERR) } else if focused { q::c(q::QUAG) } else { bg };
        buf[(area.x, area.y)].set_symbol("▎").set_fg(bar);
    }
}
