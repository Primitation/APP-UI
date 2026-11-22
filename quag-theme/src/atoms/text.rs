//! ATOM: Small text atoms.

use crate::{
    self as q, Status,
    consts::glyph as G,
};
use ratatui::{
    style::Modifier,
    text::Span,
};

/// `#tag` in Gill purple.
pub fn tag(text: &str) -> Span<'static> {
    Span::styled(format!("#{text}"), q::special())
}

/// A keyboard key, e.g. `kbd("/")`.
pub fn kbd(key: &str) -> Span<'static> {
    Span::styled(key.to_string(), q::special().add_modifier(Modifier::BOLD))
}

/// Just the colored status icon (✓ ! ✗ i).
pub fn status_icon(kind: Status) -> Span<'static> {
    Span::styled(kind.icon(), kind.style())
}

/// Text tones. Use instead of picking styles by hand.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tone {
    Title,
    Heading,
    Text,
    Muted,
    Faint,
    Accent,
}

pub fn label(text: &str, tone: Tone) -> Span<'static> {
    let s = match tone {
        Tone::Title => q::title(),
        Tone::Heading => q::heading(),
        Tone::Text => q::text(),
        Tone::Muted => q::muted(),
        Tone::Faint => q::faint(),
        Tone::Accent => q::accent(),
    };
    Span::styled(text.to_string(), s)
}

/// Spinner frame for `tick` (from `consts::motion::ticks(clock.elapsed())`: never count redraws).
pub fn spinner(tick: usize) -> Span<'static> {
    Span::styled(G::SPINNER[tick % G::SPINNER.len()], q::accent())
}
