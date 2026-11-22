//! ELEMENT: CommandPalette.

use crate::{
    atoms::{self, Tone},
    consts::layout as L,
    molecules::{Menu, MenuItem, SearchBar},
};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    text::Line,
    widgets::{Clear, Widget},
};

/// `ctrl+k`: search every action of the app by name. A SearchBar over a Menu.
pub struct CommandPalette<'a> {
    pub query: &'a str,
    /// Already filtered by the app.
    pub items: &'a [MenuItem<'a>],
    pub selected: usize,
}

impl CommandPalette<'_> {
    /// Top third of the screen, 60 wide (it grows down as you'd read).
    pub fn area(&self, screen: Rect) -> Rect {
        let w = L::PALETTE_COLS.min(screen.width.saturating_sub(2 * L::SCREEN_MARGIN));
        let h = atoms::popup_size(0, self.items.len().max(1) as u16 + L::PALETTE_HEAD_ROWS).1.min(screen.height.saturating_sub(L::SCREEN_MARGIN));
        Rect { x: screen.x + (screen.width - w) / 2, y: screen.y + screen.height / L::PALETTE_TOP_SHARE, width: w, height: h }
    }
}

impl Widget for CommandPalette<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Clear.render(area, buf);
        let block = atoms::popup("Commands");
        let inner = block.inner(area);
        block.render(area, buf);
        let [search, _, list] = Layout::vertical([Constraint::Length(1), Constraint::Length(1), Constraint::Min(0)]).areas(inner);
        SearchBar { query: self.query, focused: true }.render(search, buf);
        if self.items.is_empty() {
            Line::from(atoms::label("No command matches.", Tone::Faint)).render(list, buf);
            return;
        }
        // The list part of a Menu, without its own box
        Menu { title: "", items: self.items, selected: self.selected }.render_rows(list, buf);
    }
}
