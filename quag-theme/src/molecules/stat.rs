//! MOLECULE: Stat.

use crate::{
    self as q,
    atoms::{self, Tone},
};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Modifier,
    text::{Line, Span},
    widgets::Widget,
};

/// A number with its label, in a box (dashboard KPI). Height: 4.
pub struct Stat<'a> {
    pub value: &'a str,
    pub label: &'a str,
}

impl Stat<'_> {
    pub const HEIGHT: u16 = 4;
}

impl Widget for Stat<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let b = atoms::panel("", false);
        let inner = b.inner(area);
        b.render(area, buf);
        let [v, l] = Layout::vertical([Constraint::Length(1), Constraint::Length(1)]).areas(inner);
        Line::from(Span::styled(self.value.to_string(), q::text().add_modifier(Modifier::BOLD))).render(v, buf);
        Line::from(atoms::label(self.label, Tone::Muted)).render(l, buf);
    }
}
