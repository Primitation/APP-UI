//! ATOM: Link.

use crate::{
    self as q, Status,
};
use ratatui::{
    style::Modifier,
    text::Span,
};

/// Clickable / openable text: underlined Quag blue.
pub fn link(text: &str) -> Span<'static> {
    Span::styled(text.to_string(), q::accent().add_modifier(Modifier::UNDERLINED))
}

/// Text in a status color (e.g. a danger menu item, a failure title).
pub fn status_label(text: &str, kind: Status) -> Span<'static> {
    Span::styled(text.to_string(), kind.style())
}
