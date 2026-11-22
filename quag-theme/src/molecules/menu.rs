//! MOLECULE: Menu.

use super::*;
use crate::{
    Status,
    atoms::{self},
};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::{Line, Span},
    widgets::Widget,
};
use crate::consts::layout as L;

/// One action of a menu: label, its shortcut key ("" = none), and whether it destroys something.
#[derive(Clone, Copy, Debug, Default)]
pub struct MenuItem<'a> {
    pub label: &'a str,
    pub key: &'a str,
    pub danger: bool,
}

/// A column of actions with their keys, in a popup box. Danger items are red and come last.
pub struct Menu<'a> {
    pub title: &'a str,
    pub items: &'a [MenuItem<'a>],
    pub selected: usize,
}

impl Menu<'_> {
    /// Size it needs (fits its longest label + key).
    pub fn size(&self) -> (u16, u16) {
        let w = self.items.iter().map(|i| Span::raw(i.label).width() + Span::raw(i.key).width() + L::MENU_ITEM_PAD).max().unwrap_or(0) as u16;
        atoms::popup_size(w.max(Span::raw(self.title).width() as u16 + L::MENU_TITLE_PAD).max(L::MENU_MIN_COLS), self.items.len() as u16)
    }
}

impl Widget for Menu<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        ratatui::widgets::Clear.render(area, buf);
        let block = atoms::popup(self.title);
        let inner = block.inner(area);
        block.render(area, buf);
        self.render_rows(inner, buf);
    }
}

impl Menu<'_> {
    /// Just the rows, no box (for molecules/elements that bring their own box).
    pub fn render_rows(&self, inner: Rect, buf: &mut Buffer) {
        for (i, it) in self.items.iter().enumerate().take(inner.height as usize) {
            let r = Rect { y: inner.y + i as u16, height: 1, ..inner };
            let row = ListRow {
                title: it.label,
                meta: it.key,
                selected: i == self.selected,
                list_focused: true,
                ..Default::default()
            };
            (&row).render(r, buf);
            if it.danger {
                // the label in the danger tone (an atom decides the color)
                let label = atoms::status_label(it.label, Status::Err);
                Line::from(label).render(Rect { x: r.x + L::ROW_INDENT, width: r.width.saturating_sub(L::ROW_INDENT), ..r }, buf);
            }
        }
    }
}
