//! MOLECULE: Breadcrumbs.

use crate::atoms::{self, Tone};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::Line,
    widgets::Widget,
};

/// Where you are: `pond › quag-theme › Write README`. The last crumb is the current place.
/// Too long: the middle crumbs collapse to `…`, the first and last always stay.
pub struct Breadcrumbs<'a>(pub &'a [&'a str]);

impl Widget for Breadcrumbs<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let n = self.0.len();
        let build = |skip_mid: bool| -> Line<'static> {
            let mut s = Vec::new();
            for (i, c) in self.0.iter().enumerate() {
                if skip_mid && i > 0 && i < n - 1 {
                    if i == 1 { s.extend([atoms::label("  ›  ", Tone::Faint), atoms::label("…", Tone::Faint)]); }
                    continue;
                }
                if i > 0 { s.push(atoms::label("  ›  ", Tone::Faint)); }
                s.push(atoms::label(c, if i == n - 1 { Tone::Text } else { Tone::Muted }));
            }
            Line::from(s)
        };
        let full = build(false);
        let line = if full.width() as u16 <= area.width || n < 3 { full } else { build(true) };
        line.render(Rect { height: 1, ..area }, buf);
    }
}
