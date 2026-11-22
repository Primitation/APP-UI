//! MOLECULE: TreeView.

use super::*;
use crate::{
    Status,
    atoms::{self, Tone},
};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::Line,
    widgets::Widget,
};

/// One node of a tree (static or built by the app). `id` must be unique; it's how expansion is remembered.
#[derive(Clone, Copy, Debug)]
pub struct TreeItem<'a> {
    pub id: &'a str,
    pub label: &'a str,
    pub meta: &'a str,
    pub status: Option<Status>,
    pub children: &'a [TreeItem<'a>],
}

impl<'a> TreeItem<'a> {
    pub const fn leaf(id: &'a str, label: &'a str) -> Self { Self { id, label, meta: "", status: None, children: &[] } }
    pub const fn branch(id: &'a str, label: &'a str, children: &'a [TreeItem<'a>]) -> Self { Self { id, label, meta: "", status: None, children } }
}

/// A visible row of a tree, after expansion.
#[derive(Clone, Copy, Debug)]
pub struct TreeRow<'a> {
    pub item: &'a TreeItem<'a>,
    pub depth: usize,
    /// None = leaf, Some(open)
    pub open: Option<bool>,
    /// For the guide lines: is this the last child of its parent?
    pub last: bool,
}

/// The rows to show, in order. `is_open(id)` comes from the app's state (e.g. a HashSet of ids).
pub fn flatten<'a>(items: &'a [TreeItem<'a>], is_open: &dyn Fn(&str) -> bool) -> Vec<TreeRow<'a>> {
    fn walk<'a>(items: &'a [TreeItem<'a>], depth: usize, is_open: &dyn Fn(&str) -> bool, out: &mut Vec<TreeRow<'a>>) {
        for (i, it) in items.iter().enumerate() {
            let open = (!it.children.is_empty()).then(|| is_open(it.id));
            out.push(TreeRow { item: it, depth, open, last: i == items.len() - 1 });
            if open == Some(true) {
                walk(it.children, depth + 1, is_open, out);
            }
        }
    }
    let mut out = Vec::new();
    walk(items, 0, is_open, &mut out);
    out
}

/// A tree: `▾ folder` / `▸ folder` / `  file`, indented 2 per level, with selection and scrolling.
/// The app owns expansion and selection (→ opens, ← closes, ↑↓ moves).
pub struct TreeView<'a> {
    pub rows: &'a [TreeRow<'a>],
    pub selected: usize,
    pub focused: bool,
}

impl Widget for TreeView<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let h = area.height as usize;
        let offset = self.selected.saturating_sub(h.saturating_sub(2)).min(self.rows.len().saturating_sub(h));
        for (y, (i, row)) in self.rows.iter().enumerate().skip(offset).take(h).enumerate() {
            let r = Rect { y: area.y + y as u16, height: 1, ..area };
            let arrow = match row.open { Some(true) => "▾ ", Some(false) => "▸ ", None => "  " };
            let indent = "  ".repeat(row.depth);
            let title = format!("{indent}{arrow}{}", row.item.label);
            let lr = ListRow {
                title: &title,
                status: row.item.status,
                meta: row.item.meta,
                selected: i == self.selected,
                list_focused: self.focused,
                ..Default::default()
            };
            (&lr).render(r, buf);
        }
        if self.rows.is_empty() {
            Line::from(atoms::label("Empty.", Tone::Faint)).render(Rect { height: 1, ..area }, buf);
        }
    }
}
