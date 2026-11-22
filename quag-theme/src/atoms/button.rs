//! ATOM: Button.

use super::*;
use crate::{
    self as q,
    consts::{glyph as G, layout as L},
};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Widget,
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonKind {
    /// Max one per screen.
    Primary,
    #[default]
    Secondary,
    Danger,
    Ghost,
}

#[derive(Clone, Debug)]
pub struct Button<'a> {
    pub label: &'a str,
    pub kind: ButtonKind,
    pub state: State,
    /// 1 cell of padding instead of 2 (molecules switch this on when space is tight).
    pub compact: bool,
}

impl<'a> Button<'a> {
    pub fn new(label: &'a str) -> Self {
        Self { label, kind: ButtonKind::Secondary, state: State::Normal, compact: false }
    }
    pub fn primary(mut self) -> Self { self.kind = ButtonKind::Primary; self }
    pub fn danger(mut self) -> Self { self.kind = ButtonKind::Danger; self }
    pub fn ghost(mut self) -> Self { self.kind = ButtonKind::Ghost; self }
    pub fn focused(mut self, on: bool) -> Self { if on { self.state = State::Focused } self }
    pub fn disabled(mut self, on: bool) -> Self { if on { self.state = State::Disabled } self }

    pub fn compact(mut self, on: bool) -> Self { self.compact = on; self }

    /// Cells it takes: 2 padding each side (1 when compact).
    pub fn width(&self) -> u16 {
        Span::raw(self.label).width() as u16 + if self.compact { L::BUTTON_PAD_COMPACT } else { L::BUTTON_PAD }
    }

    pub fn style(&self) -> Style {
        if self.state == State::Disabled {
            return Style::new().fg(q::c(q::FAINT)).bg(q::c(q::SURFACE));
        }
        let focused = self.state == State::Focused;
        match self.kind {
            ButtonKind::Primary => Style::new()
                .fg(q::c(q::BG))
                .bg(q::c(if focused { q::QUAG_HI } else { q::QUAG }))
                .add_modifier(Modifier::BOLD),
            ButtonKind::Secondary => Style::new()
                .fg(q::c(if focused { q::QUAG_HI } else { q::TEXT }))
                .bg(q::c(q::BORDER)),
            ButtonKind::Danger => Style::new().fg(q::c(q::ERR)).bg(q::c(q::BORDER)).add_modifier(Modifier::BOLD),
            ButtonKind::Ghost => Style::new().fg(q::c(if focused { q::QUAG_HI } else { q::MUTED })),
        }
        .add_modifier(if focused { Modifier::BOLD | Modifier::UNDERLINED } else { Modifier::empty() })
    }

    pub fn to_span(&self) -> Span<'static> {
        let pad = if self.compact { " " } else { "  " };
        // keyboard focus: a `▸` in the left padding (same width), like the `▌` of a selected row
        let lead = match (self.state == State::Focused, self.compact) { (true, true) => G::FOCUS_MARK.to_string(), (true, false) => format!("{} ", G::FOCUS_MARK), _ => pad.to_string() };
        Span::styled(format!("{lead}{}{pad}", self.label), self.style())
    }
}

impl Widget for Button<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Line::from(self.to_span()).render(area, buf);
    }
}
