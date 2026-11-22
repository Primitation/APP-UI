//! ATOM: Select.

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

/// A closed dropdown: ` Every 5 min        ▾`. When open, the app shows a `Menu` under it.
#[derive(Clone, Debug, Default)]
pub struct Select<'a> {
    pub value: &'a str,
    pub placeholder: &'a str,
    pub state: State,
    pub open: bool,
}

impl<'a> Select<'a> {
    pub fn new(value: &'a str) -> Self { Self { value, ..Default::default() } }
    pub fn focused(mut self, on: bool) -> Self { if on { self.state = State::Focused } self }
    pub fn disabled(mut self, on: bool) -> Self { if on { self.state = State::Disabled } self }
    pub fn open(mut self, on: bool) -> Self { self.open = on; self }
}

impl Widget for Select<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area = Rect { height: 1, ..area };
        let focused = self.state == State::Focused || self.open;
        let bg = q::c(if focused { q::RAISED } else { q::SURFACE });
        buf.set_style(area, Style::new().bg(bg));
        let (t, s) = if self.value.is_empty() { (self.placeholder, q::faint()) } else { (self.value, if self.state == State::Disabled { q::faint() } else { q::text() }) };
        Line::from(vec![Span::raw(" "), Span::styled(t.to_string(), s)]).render(area, buf);
        if area.width > L::FIELD_ICON {
            buf.set_string(area.right() - L::FIELD_ICON, area.y, if self.open { "▴" } else { "▾" }, if focused { q::accent() } else { q::muted() });
        }
        buf[(area.x, area.y)].set_symbol("▎").set_fg(if focused { q::c(q::QUAG) } else { bg });
    }
}
