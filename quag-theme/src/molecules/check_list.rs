//! MOLECULE: CheckList.

use crate::atoms::{self};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::Widget,
};

/// A column of checkboxes. `focused` = index with keyboard focus.
pub struct CheckList<'a> {
    pub items: &'a [(&'a str, bool)],
    pub focused: Option<usize>,
}

impl Widget for CheckList<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        for (i, (label, checked)) in self.items.iter().enumerate().take(area.height as usize) {
            atoms::Checkbox::new(label, *checked)
                .focused(self.focused == Some(i))
                .render(Rect { y: area.y + i as u16, height: 1, ..area }, buf);
        }
    }
}
