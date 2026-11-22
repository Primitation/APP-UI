//! MOLECULE: SearchBar.

use crate::atoms::{self, Input};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    text::Line,
    widgets::Widget,
};

/// `/` key + input. Height: 1.
pub struct SearchBar<'a> {
    pub query: &'a str,
    pub focused: bool,
}

impl Widget for SearchBar<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [k, i] = Layout::horizontal([Constraint::Length(2), Constraint::Min(0)]).areas(area);
        Line::from(atoms::kbd("/")).render(k, buf);
        Input::new(self.query).placeholder("Search…").focused(self.focused).render(i, buf);
    }
}
