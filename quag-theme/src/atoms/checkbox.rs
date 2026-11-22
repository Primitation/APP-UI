//! ATOM: Checkbox.

use super::*;
use crate::{
    self as q,
    consts::layout as L,
};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::{Line, Span},
    widgets::Widget,
};

#[derive(Clone, Debug)]
pub struct Checkbox<'a> {
    pub label: &'a str,
    pub checked: bool,
    pub state: State,
}

impl<'a> Checkbox<'a> {
    pub fn new(label: &'a str, checked: bool) -> Self { Self { label, checked, state: State::Normal } }
    pub fn focused(mut self, on: bool) -> Self { if on { self.state = State::Focused } self }
    pub fn disabled(mut self, on: bool) -> Self { if on { self.state = State::Disabled } self }
    pub fn width(&self) -> u16 { Span::raw(self.label).width() as u16 + L::CHOICE_MARK }

    pub fn to_line(&self) -> Line<'static> {
        let (boxs, label) = match self.state {
            State::Disabled => (q::faint(), q::faint()),
            State::Focused => (q::accent(), q::selected().bg(q::c(q::BG))),
            State::Normal => (if self.checked { q::accent() } else { q::faint() }, q::text()),
        };
        Line::from(vec![
            Span::styled(if self.checked { "[✓]" } else { "[ ]" }, boxs),
            Span::raw(" "),
            Span::styled(self.label.to_string(), label),
        ])
    }
}

impl Widget for Checkbox<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) { self.to_line().render(area, buf) }
}
