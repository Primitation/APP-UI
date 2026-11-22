//! MOLECULE: PropertyList.

use super::*;
use crate::atoms::{self, Tone};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    text::{Line, Span},
    widgets::Widget,
};

/// Read-only facts as `key   value` rows, keys aligned (inspector, about boxes).
pub struct PropertyList<'a>(pub &'a [(&'a str, &'a str)]);

impl Widget for PropertyList<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let kw = self.0.iter().map(|(k, _)| Span::raw(*k).width()).max().unwrap_or(0) as u16;
        for (i, (k, v)) in self.0.iter().enumerate().take(area.height as usize) {
            let [kr, _, vr] = Layout::horizontal([Constraint::Length(kw), Constraint::Length(GAP), Constraint::Min(0)])
                .areas(Rect { y: area.y + i as u16, height: 1, ..area });
            Line::from(atoms::label(k, Tone::Muted)).render(kr, buf);
            Line::from(atoms::label(&truncate(v, vr.width as usize), Tone::Text)).render(vr, buf);
        }
    }
}
