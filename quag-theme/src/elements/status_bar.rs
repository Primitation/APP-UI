//! ELEMENT: StatusBar.

use crate::{
    molecules::{KeyHints, StatusLine},
    ui,
};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    text::Line,
    widgets::Widget,
};

/// Key hints left, last result right.
pub struct StatusBar<'a> {
    pub hints: KeyHints<'a>,
    pub status: Option<StatusLine<'a>>,
}

impl Widget for StatusBar<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [l, r] = ui::footer_split(area);
        self.hints.render(l, buf);
        if let Some(s) = self.status {
            Line::from(s.to_span()).alignment(Alignment::Right).render(r, buf);
        }
    }
}
