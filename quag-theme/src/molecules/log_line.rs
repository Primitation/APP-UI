//! MOLECULE: LogLine.

use crate::{
    Status,
    atoms::{self, Tone},
};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::{Line, Span},
    widgets::Widget,
};

/// One line of a stream: `12:01:03  ✗  message`. Time faint, icon colored, text normal.
pub struct LogLine<'a> {
    pub time: &'a str,
    pub kind: Status,
    pub message: &'a str,
}

impl Widget for LogLine<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Line::from(vec![
            atoms::label(self.time, Tone::Faint),
            Span::raw("  "),
            atoms::status_icon(self.kind),
            Span::raw("  "),
            atoms::label(self.message, Tone::Text),
        ])
        .render(area, buf);
    }
}
