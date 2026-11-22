//! ATOM: Radio.

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

/// One option of a single choice: `(●) label` / `( ) label`.
#[derive(Clone, Debug)]
pub struct Radio<'a> {
    pub label: &'a str,
    pub selected: bool,
    pub state: State,
}

impl<'a> Radio<'a> {
    pub fn new(label: &'a str, selected: bool) -> Self { Self { label, selected, state: State::Normal } }
    pub fn focused(mut self, on: bool) -> Self { if on { self.state = State::Focused } self }
    pub fn disabled(mut self, on: bool) -> Self { if on { self.state = State::Disabled } self }
    pub fn width(&self) -> u16 { Span::raw(self.label).width() as u16 + L::CHOICE_MARK }

    pub fn to_line(&self) -> Line<'static> {
        let (dot, label) = match self.state {
            State::Disabled => (q::faint(), q::faint()),
            State::Focused => (q::accent(), q::selected().bg(q::c(q::BG))),
            State::Normal => (if self.selected { q::accent() } else { q::faint() }, q::text()),
        };
        Line::from(vec![
            Span::styled(if self.selected { "(●)" } else { "( )" }, dot),
            Span::raw(" "),
            Span::styled(self.label.to_string(), label),
        ])
    }
}

impl Widget for Radio<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) { self.to_line().render(area, buf) }
}
