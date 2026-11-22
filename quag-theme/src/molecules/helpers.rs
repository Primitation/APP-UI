//! MOLECULE: helpers.

use ratatui::text::Span;

/// Cut `text` to `room` cells, ending with `…` (the overflow rule).
pub fn truncate(text: &str, room: usize) -> String {
    if Span::raw(text).width() <= room {
        return text.to_string();
    }
    if room == 0 {
        return String::new();
    }
    let mut t = String::new();
    for ch in text.chars() {
        if Span::raw(format!("{t}{ch}…")).width() > room { break; }
        t.push(ch);
    }
    format!("{t}…")
}
