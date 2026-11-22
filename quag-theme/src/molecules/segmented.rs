//! MOLECULE: Segmented.

use crate::atoms::Button;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::Widget,
};

/// One choice among a few short options, as joined buttons: `[15m] 1h 24h 7d`.
/// For modes and ranges (time range, view mode). Many or long options → `Select`.
pub struct Segmented<'a> {
    pub options: &'a [&'a str],
    pub active: usize,
    pub focused: bool,
}

impl Segmented<'_> {
    pub fn width(&self) -> u16 {
        self.options.iter().map(|o| Button::new(o).compact(true).width() + 1).sum::<u16>().saturating_sub(1)
    }
}

impl Widget for Segmented<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut x = area.x;
        for (i, o) in self.options.iter().enumerate() {
            let b = Button::new(o).compact(true);
            let b = if i == self.active { b.primary().focused(self.focused) } else { b };
            let w = b.width();
            if x + w > area.right() { break; }
            b.render(Rect::new(x, area.y, w, 1), buf);
            x += w + 1;
        }
    }
}
