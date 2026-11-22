//! MOLECULE: KeyHints.

use crate::{
    self as q,
    atoms::{self},
};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::{Line, Span},
    widgets::Widget,
};

/// `q quit · / search · ? help`
pub struct KeyHints<'a>(pub &'a [(&'a str, &'a str)]);

impl KeyHints<'_> {
    pub fn to_line(&self) -> Line<'static> {
        let mut spans = Vec::new();
        for (i, (key, label)) in self.0.iter().enumerate() {
            if i > 0 { spans.push(Span::styled("  ·  ", q::faint())); }
            spans.push(atoms::kbd(key));
            spans.push(Span::styled(format!(" {label}"), q::muted()));
        }
        Line::from(spans)
    }
}

impl Widget for KeyHints<'_> {
    /// Drops whole hints from the end instead of cutting one in half.
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut n = self.0.len();
        while n > 0 && KeyHints(&self.0[..n]).to_line().width() as u16 > area.width { n -= 1; }
        KeyHints(&self.0[..n]).to_line().render(area, buf)
    }
}
