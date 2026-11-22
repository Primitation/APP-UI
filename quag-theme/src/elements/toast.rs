//! ELEMENT: Toast.

use crate::{
    Status,
    atoms::{self},
    consts::layout as L,
    molecules::StatusLine,
};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::Span,
    widgets::{Clear, Widget},
};

/// A short-lived result message in a small box, bottom-right above the status bar.
/// Use for results that matter more than the status line (desktop uses it more).
pub struct Toast<'a> {
    pub kind: Status,
    pub message: &'a str,
}

impl Toast<'_> {
    pub fn area(&self, screen: Rect) -> Rect {
        let (w, h) = atoms::popup_size(Span::raw(self.message).width() as u16 + L::TOAST_ICON, 1);
        let w = w.min(screen.width.saturating_sub(L::SCREEN_MARGIN));
        // bottom-right, leaving the status bar row free
        Rect { x: screen.right().saturating_sub(w + 1), y: screen.bottom().saturating_sub(h + 1), width: w, height: h }
    }
}

impl Widget for Toast<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Clear.render(area, buf);
        let block = atoms::popup("");
        let inner = block.inner(area);
        block.render(area, buf);
        StatusLine { kind: self.kind, message: self.message }.render(Rect { x: inner.x + 1, width: inner.width.saturating_sub(1), ..inner }, buf);
    }
}
