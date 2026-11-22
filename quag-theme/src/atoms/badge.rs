//! ATOM: Badge.

use crate::{
    self as q,
};
use ratatui::{
    style::{Modifier, Style},
    text::Span,
};

/// A small count, e.g. unread items: ` 3 `. 0 renders nothing; 100+ shows `99+`.
pub fn badge(count: usize) -> Span<'static> {
    if count == 0 {
        return Span::raw("");
    }
    let text = if count > 99 { "99+".to_string() } else { count.to_string() };
    Span::styled(format!(" {text} "), Style::new().fg(q::c(q::QUAG_HI)).bg(q::c(q::RAISED)).add_modifier(Modifier::BOLD))
}
