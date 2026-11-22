//! ELEMENTS of this app: molecules wired to its data. Each owns its states (empty, etc.). Never sets colors.

use crate::app::Item;
use quag_theme::{
    atoms,
    elements::EmptyState,
    molecules::ListRow,
};
use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

/// ItemList: the items, one ListRow each. Empty → EmptyState with the key to add one.
pub struct ItemList<'a> {
    pub items: &'a [Item],
    pub selected: usize,
    pub focused: bool,
}

impl Widget for ItemList<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = atoms::panel("Items", self.focused);
        let inner = block.inner(area);
        block.render(area, buf);
        if self.items.is_empty() {
            EmptyState { message: "Nothing here yet.", hint: Some(("n", "to add one")) }.render(inner, buf);
            return;
        }
        for (i, it) in self.items.iter().enumerate().take(inner.height as usize) {
            (&ListRow {
                title: &it.title,
                meta: &it.meta,
                check: Some(it.done),
                selected: i == self.selected,
                list_focused: self.focused,
                ..Default::default()
            })
                .render(Rect { y: inner.y + i as u16, height: 1, ..inner }, buf);
        }
    }
}
