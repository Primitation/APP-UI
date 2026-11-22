//! MOLECULE: ButtonGroup.

use super::*;
use crate::atoms::{self, Button};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::Widget,
};

/// One button of a row, written ONCE: what it says, what it does (a keymap action id) and how it looks.
/// The row is drawn from the table (`ActionButton::group`) and pressed from it (`keymap::ButtonRow`),
/// so a label can never end up on another button's action.
#[derive(Clone, Copy, Debug)]
pub struct ActionButton<'a> {
    pub label: &'a str,
    pub action: &'a str,
    pub kind: crate::atoms::ButtonKind,
}

impl<'a> ActionButton<'a> {
    pub const fn new(label: &'a str, action: &'a str) -> Self { Self { label, action, kind: crate::atoms::ButtonKind::Secondary } }
    pub const fn primary(mut self) -> Self { self.kind = crate::atoms::ButtonKind::Primary; self }
    pub const fn danger(mut self) -> Self { self.kind = crate::atoms::ButtonKind::Danger; self }
    pub const fn ghost(mut self) -> Self { self.kind = crate::atoms::ButtonKind::Ghost; self }

    /// The row as a ButtonGroup, `focused` = the button the keyboard is on.
    pub fn group(row: &[ActionButton<'a>], focused: Option<usize>) -> ButtonGroup<'a> {
        ButtonGroup::new(row.iter().enumerate().map(|(i, b)| {
            Button { kind: b.kind, ..Button::new(b.label) }.focused(focused == Some(i))
        }).collect())
    }
}

/// An array of buttons in a row, optionally inside a titled box.
/// Order rule (STRUCTURE.md): in the terminal the Primary button comes FIRST.
#[derive(Clone, Debug, Default)]
pub struct ButtonGroup<'a> {
    pub buttons: Vec<Button<'a>>,
    pub align: Align,
    /// `Some(title)` draws a panel around the group ("" = untitled box).
    pub boxed: Option<&'a str>,
}

impl<'a> ButtonGroup<'a> {
    pub fn new(buttons: Vec<Button<'a>>) -> Self { Self { buttons, ..Default::default() } }
    pub fn align(mut self, a: Align) -> Self { self.align = a; self }
    pub fn boxed(mut self, title: &'a str) -> Self { self.boxed = Some(title); self }

    /// Width of the buttons row (without the box).
    pub fn width(&self) -> u16 {
        let n = self.buttons.len() as u16;
        let gap = if self.buttons.first().is_some_and(|b| b.compact) { 1 } else { GAP };
        self.buttons.iter().map(|b| b.width()).sum::<u16>() + gap * n.saturating_sub(1)
    }
    /// Height needed: 1, or 3 when boxed.
    pub fn height(&self) -> u16 { if self.boxed.is_some() { 3 } else { 1 } }
}

impl Widget for ButtonGroup<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let inner = match self.boxed {
            Some(t) => {
                let b = atoms::panel(t, false);
                let i = b.inner(area);
                b.render(area, buf);
                i
            }
            None => area,
        };
        // Never drop a button silently: tighten padding first, then wrap to the next row.
        let mut group = self;
        if group.width() > inner.width {
            group.buttons = group.buttons.into_iter().map(|b| b.compact(true)).collect();
        }
        let gap = if group.buttons.first().is_some_and(|b| b.compact) { 1 } else { GAP };
        let w = group.width().min(inner.width);
        let start = match group.align {
            Align::Left => inner.x,
            Align::Right => inner.right().saturating_sub(w),
        };
        let (mut x, mut y) = (start, inner.y);
        for b in group.buttons {
            let bw = b.width().min(inner.width);
            if x + bw > inner.right() && x != start {
                x = start;
                y += 1;
            }
            if y >= inner.bottom() { break; }
            b.render(Rect::new(x, y, bw, 1), buf);
            x += bw + gap;
        }
    }
}
