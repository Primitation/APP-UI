//! ATOM: SplitHandle.

use crate::{
    self as q,
};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    text::Span,
    widgets::Widget,
};

/// The line between two resizable panes. Focused = Quag (then ←/→ or [ ] resize).
pub struct SplitHandle {
    pub vertical: bool,
    pub focused: bool,
}

impl Widget for SplitHandle {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let s = Style::new().fg(q::c(if self.focused { q::QUAG } else { q::BORDER }));
        let sym = if self.vertical { "│" } else { "─" };
        for y in area.y..area.bottom() {
            for x in area.x..area.right() {
                buf[(x, y)].set_symbol(sym).set_style(s);
            }
        }
        if self.focused {
            let (x, y) = (area.x + area.width / 2, area.y + area.height / 2);
            buf[(x, y)].set_symbol(if self.vertical { "┃" } else { "━" });
        }
    }
}

/// A search match inside text (Gill, bold): the highlight of filters and searches.
pub fn mark(text: &str) -> Span<'static> {
    Span::styled(text.to_string(), q::special().add_modifier(Modifier::BOLD | Modifier::UNDERLINED))
}
