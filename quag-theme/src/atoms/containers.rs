//! ATOM: Containers (the "box").

use crate::{
    self as q,
    consts::layout as L,
};
use ratatui::{
    layout::Rect,
    style::Style,
    text::Span,
    widgets::{Block, BorderType, Borders, Padding},
};

/// Panel frame: rounded border, 1-cell padding. Focused = Quag border + bright title.
pub fn panel<'a>(title_text: &'a str, focused: bool) -> Block<'a> {
    let (border_c, title_s) = if focused { (q::c(q::QUAG), q::title()) } else { (q::c(q::BORDER), q::muted()) };
    let b = Block::new()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::new().fg(border_c))
        .style(Style::new().bg(q::c(q::BG)).fg(q::c(q::TEXT)))
        .padding(Padding::horizontal(L::PANEL_PAD_X));
    if title_text.is_empty() { b } else { b.title(Span::styled(format!(" {title_text} "), title_s)) }
}

/// Popup frame: raised background, Quag border, roomy padding.
/// Outer size of a popup whose content needs `w × h` (adds border + padding).
pub fn popup_size(w: u16, h: u16) -> (u16, u16) {
    let probe = Rect::new(0, 0, 200, 200);
    let inner = popup("").inner(probe);
    (w + probe.width - inner.width, h + probe.height - inner.height)
}

pub fn popup<'a>(title_text: &'a str) -> Block<'a> {
    Block::new()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::new().fg(q::c(q::QUAG)))
        .title(Span::styled(format!(" {title_text} "), q::title()))
        .style(Style::new().bg(q::c(q::RAISED)).fg(q::c(q::TEXT)))
        .padding(Padding::new(L::POPUP_PAD_X, L::POPUP_PAD_X, L::POPUP_PAD_Y, L::POPUP_PAD_Y))
}
