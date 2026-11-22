//! ELEMENT: ErrorState.

use crate::{
    Status,
    atoms::{self, Button, Tone},
    ui,
};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    text::{Line, Span},
    widgets::Widget,
};

/// The error state of a zone (STRUCTURE.md §7.4): `✗` + what failed + why + "Retry".
/// Only the zone that failed shows it; the rest of the page stays usable.
pub struct ErrorState<'a> {
    pub what: &'a str,
    pub why: &'a str,
    pub retry_focused: bool,
}

impl Widget for ErrorState<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let col = ui::focus_column(area, 4);
        let [t, d, _, b] = Layout::vertical([Constraint::Length(1); 4]).areas(col);
        Line::from(vec![atoms::status_icon(Status::Err), Span::raw("  "), atoms::status_label(self.what, Status::Err)])
            .alignment(Alignment::Center)
            .render(t, buf);
        Line::from(atoms::label(self.why, Tone::Muted)).alignment(Alignment::Center).render(d, buf);
        let btn = Button::new("Retry").focused(self.retry_focused);
        let w = btn.width();
        btn.render(Rect { x: b.x + b.width.saturating_sub(w) / 2, width: w.min(b.width), ..b }, buf);
    }
}
