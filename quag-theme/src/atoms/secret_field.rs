//! ATOM: SecretField.

use super::*;
use crate::{
    self as q, Status,
    consts::{format as F, glyph as G, layout as L},
};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::Widget,
};

/// A secret (API key, password): masked by default `••••••••••••3f9a`, revealed on demand.
/// Only the last 4 characters show while masked, so you can tell keys apart.
#[derive(Clone, Debug, Default)]
pub struct SecretField<'a> {
    pub value: &'a str,
    pub revealed: bool,
    pub state: State,
}

impl<'a> SecretField<'a> {
    pub fn new(value: &'a str) -> Self { Self { value, ..Default::default() } }
    pub fn revealed(mut self, on: bool) -> Self { self.revealed = on; self }
    pub fn focused(mut self, on: bool) -> Self { if on { self.state = State::Focused } self }

    pub fn masked(&self) -> String {
        let n = self.value.chars().count();
        let tail: String = self.value.chars().skip(n.saturating_sub(F::SECRET_TAIL)).collect();
        format!("{}{tail}", G::MASK.repeat(n.saturating_sub(F::SECRET_TAIL).min(F::SECRET_MASK_MAX)))
    }
}

impl Widget for SecretField<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area = Rect { height: 1, ..area };
        let focused = self.state == State::Focused;
        let bg = q::c(if focused { q::RAISED } else { q::SURFACE });
        buf.set_style(area, Style::new().bg(bg));
        let text = if self.revealed { self.value.to_string() } else { self.masked() };
        // The show/hide label keeps its place; the value gives way (keeps its end: the part that tells keys apart)
        let room = area.width.saturating_sub(if area.width > L::SECRET_TAG_COLS { L::SECRET_TAG_COLS } else { 1 }) as usize;
        let n = text.chars().count();
        let text: String = if n > room { format!("…{}", text.chars().skip(n + 1 - room.max(1)).collect::<String>()) } else { text };
        Line::from(vec![Span::raw(" "), Span::styled(text, if self.revealed { Status::Warn.style() } else { q::text() })]).render(area, buf);
        let tag = if self.revealed { "hide" } else { "show" };
        if area.width > L::SECRET_TAG_COLS {
            buf.set_string(area.right() - L::SECRET_TAG_RIGHT, area.y, tag, (if focused { q::accent() } else { q::faint() }).bg(bg));
        }
        buf[(area.x, area.y)].set_symbol("▎").set_fg(if self.revealed { q::c(q::WARN) } else if focused { q::c(q::QUAG) } else { bg });
    }
}
