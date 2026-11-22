//! MOLECULE: StatusLine.

use crate::Status;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::{Line, Span},
    widgets::Widget,
};

/// `✓ Saved 2s ago`: icon + message, both in the status color.
pub struct StatusLine<'a> {
    pub kind: Status,
    pub message: &'a str,
}

impl StatusLine<'_> {
    pub fn to_span(&self) -> Span<'static> {
        Span::styled(format!("{} {}", self.kind.icon(), self.message), self.kind.style())
    }
}

impl Widget for StatusLine<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) { Line::from(self.to_span()).render(area, buf) }
}
