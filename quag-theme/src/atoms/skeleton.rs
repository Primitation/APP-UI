//! ATOM: Skeleton.

use crate::{
    self as q,
    consts::glyph as G,
};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::Line,
    widgets::Widget,
};

/// Loading placeholder: a faint bar where text will appear. `fill` = share of the width (0–1).
/// Keeps the layout stable while loading (STRUCTURE.md §7.4: never a blank zone).
pub struct Skeleton(pub f64);

impl Widget for Skeleton {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let w = ((area.width as f64) * self.0.clamp(0.0, 1.0)).round() as usize;
        Line::styled(G::LINE_FULL.repeat(w), Style::new().fg(q::c(q::BORDER))).render(Rect { height: 1, ..area }, buf);
    }
}
