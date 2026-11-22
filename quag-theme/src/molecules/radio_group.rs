//! MOLECULE: RadioGroup.

use super::*;
use crate::atoms::{self};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::Widget,
};

/// One choice among a few options, in a row (or a column when it doesn't fit).
pub struct RadioGroup<'a> {
    pub options: &'a [&'a str],
    pub selected: usize,
    pub focused: Option<usize>,
}

impl RadioGroup<'_> {
    /// Rows needed at this width (1 when it fits in a row, else one per option).
    pub fn height(&self, width: u16) -> u16 {
        let w: u16 = self.options.iter().map(|o| atoms::Radio::new(o, false).width() + GAP).sum();
        if w.saturating_sub(GAP) <= width { 1 } else { self.options.len() as u16 }
    }
}

impl Widget for RadioGroup<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let row = self.height(area.width) == 1;
        let (mut x, mut y) = (area.x, area.y);
        for (i, o) in self.options.iter().enumerate() {
            if y >= area.bottom() { break; }
            let r = atoms::Radio::new(o, i == self.selected).focused(self.focused == Some(i));
            let w = r.width().min(area.right().saturating_sub(x));
            r.render(Rect::new(x, y, w, 1), buf);
            if row { x += w + GAP } else { y += 1 }
        }
    }
}
