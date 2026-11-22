//! ELEMENT: EmptyState.

use crate::{
    atoms::{self, Tone},
    consts::layout as L,
    ui,
};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    text::Line,
    widgets::{Paragraph, Widget},
};

/// Centered message + one hint. With the mascot feature, call `mascot_area` first
/// and render the mascot there (it needs the `Frame`, so the app draws it).
pub struct EmptyState<'a> {
    pub message: &'a str,
    /// e.g. `[("N", "to add a task")]` → "Press N to add a task."
    pub hint: Option<(&'a str, &'a str)>,
}

impl EmptyState<'_> {
    /// Splits the zone into `[mascot art, text]` (art is 0-height when `with_mascot` is false).
    pub fn layout(area: Rect, with_mascot: bool) -> [Rect; 2] {
        let art_h = if with_mascot { L::EMPTY_ART_ROWS.min(area.height.saturating_sub(L::EMPTY_TEXT_ROWS + 1)) } else { 0 };
        let col = ui::focus_column(area, art_h + L::EMPTY_TEXT_ROWS);
        let [art, _, text] =
            Layout::vertical([Constraint::Length(art_h), Constraint::Length(1), Constraint::Length(2)]).areas(col);
        [art, text]
    }
}

impl Widget for EmptyState<'_> {
    /// Render into the *text* rect from `layout`, or into the whole zone (no mascot).
    fn render(self, area: Rect, buf: &mut Buffer) {
        let text = if area.height > 3 { Self::layout(area, false)[1] } else { area };
        let mut lines = vec![Line::from(atoms::label(self.message, Tone::Muted))];
        if let Some((key, rest)) = self.hint {
            lines.push(Line::from(vec![
                atoms::label("Press ", Tone::Faint),
                atoms::kbd(key),
                atoms::label(&format!(" {rest}."), Tone::Faint),
            ]));
        }
        Paragraph::new(lines).alignment(Alignment::Center).render(text, buf);
    }
}
