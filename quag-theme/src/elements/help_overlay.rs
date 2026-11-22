//! ELEMENT: HelpOverlay.

use crate::{
    atoms::{self, Tone},
    consts::layout as L,
    keymap::Keymap,
    ui,
};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    text::{Line, Span},
    widgets::{Clear, Widget},
};

/// `?`: every key of the current page, grouped, read from the page's `Keymap`. Esc closes it.
pub struct HelpOverlay<'a> {
    pub keymap: Keymap<'a>,
}

impl HelpOverlay<'_> {
    /// Width of the key column.
    fn key_cols(&self) -> u16 {
        self.keymap.0.iter().map(|b| Span::raw(b.key).width()).max().unwrap_or(1) as u16 + L::HELP_KEY_PAD
    }
    pub fn area(&self, screen: Rect) -> Rect {
        let groups = self.keymap.groups();
        let text_w = L::HELP_COLS.saturating_sub(self.key_cols()).max(1) as usize;
        let rows: u16 = groups.iter().map(|(_, k)| 1 + k.iter().map(|b| wrap_words(b.what, text_w).len() as u16).sum::<u16>()).sum::<u16>()
            + groups.len().saturating_sub(1) as u16;
        let (w, h) = atoms::popup_size(L::HELP_COLS, rows);
        ui::centered(screen, w, h)
    }
}

/// Split `text` into lines of at most `w` columns, at spaces (a longer word gets its own line).
pub fn wrap_words(text: &str, w: usize) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut line = String::new();
    for word in text.split(' ') {
        let len = Span::raw(line.as_str()).width();
        if !line.is_empty() && len + 1 + Span::raw(word).width() > w {
            out.push(std::mem::take(&mut line));
        }
        if !line.is_empty() { line.push(' '); }
        line.push_str(word);
    }
    out.push(line);
    out
}

impl Widget for HelpOverlay<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Clear.render(area, buf);
        let block = atoms::popup("Keys");
        let inner = block.inner(area);
        block.render(area, buf);
        let kw = self.key_cols();
        let mut y = inner.y;
        for (gi, (title, keys)) in self.keymap.groups().iter().enumerate() {
            if gi > 0 { y += 1; }
            if y >= inner.bottom() { break; }
            Line::from(atoms::label(title, Tone::Heading)).render(Rect { y, height: 1, ..inner }, buf);
            y += 1;
            for b in keys {
                if y >= inner.bottom() { break; }
                let [kr, wr] = Layout::horizontal([Constraint::Length(kw), Constraint::Min(0)]).areas(Rect { y, height: 1, ..inner });
                Line::from(atoms::kbd(b.key)).render(kr, buf);
                // long descriptions wrap under themselves, never cut
                for line in wrap_words(b.what, wr.width.max(1) as usize) {
                    if y >= inner.bottom() { break; }
                    Line::from(atoms::label(&line, Tone::Text)).render(Rect { y, ..wr }, buf);
                    y += 1;
                }
            }
        }
    }
}
