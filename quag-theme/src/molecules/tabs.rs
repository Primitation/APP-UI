//! MOLECULE: Tabs.

use crate::{
    self as q,
};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Modifier,
    text::{Line, Span},
    widgets::Widget,
};

/// Row of tab labels; the active one is accent + underlined.
pub struct Tabs<'a> {
    pub names: &'a [&'a str],
    pub active: usize,
}

impl Tabs<'_> {
    pub fn to_line(&self) -> Line<'static> {
        let mut spans = Vec::new();
        for (i, n) in self.names.iter().enumerate() {
            if i > 0 { spans.push(Span::raw("   ")); }
            let s = if i == self.active {
                q::accent().add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
            } else {
                q::muted()
            };
            spans.push(Span::styled(n.to_string(), s));
        }
        Line::from(spans)
    }
    pub fn width(&self) -> u16 { self.to_line().width() as u16 }
}

impl Widget for Tabs<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) { self.to_line().render(area, buf) }
}
