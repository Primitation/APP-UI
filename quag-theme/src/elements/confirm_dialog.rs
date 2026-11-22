//! ELEMENT: ConfirmDialog.

use crate::{
    atoms::{self, Button, Tone},
    consts::layout as L,
    molecules::ButtonGroup,
    ui,
};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    text::{Line, Span},
    widgets::{Clear, Widget},
};

/// "Are you sure?" popup. `focused` = index of the focused button (0 = confirm).
pub struct ConfirmDialog<'a> {
    pub title: &'a str,
    pub question: &'a str,
    pub detail: &'a str,
    pub confirm: &'a str,
    pub cancel: &'a str,
    pub danger: bool,
    pub focused: usize,
}

impl ConfirmDialog<'_> {
    /// Where it goes: centered over the whole screen, as wide as its text needs.
    pub fn area(&self, screen: Rect) -> Rect {
        let text = Span::raw(self.question).width().max(Span::raw(self.detail).width()) as u16;
        ui::centered(screen, (text + L::DIALOG_TEXT_PAD).clamp(L::DIALOG_MIN_COLS, L::DIALOG_MAX_COLS), L::DIALOG_ROWS)
    }
}

impl Widget for ConfirmDialog<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Clear.render(area, buf);
        let block = atoms::popup(self.title);
        let inner = block.inner(area);
        block.render(area, buf);
        let [q_, d, _, b] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .areas(inner);
        Line::from(atoms::label(self.question, Tone::Text)).render(q_, buf);
        Line::from(atoms::label(self.detail, Tone::Muted)).render(d, buf);
        let confirm = Button::new(self.confirm).focused(self.focused == 0);
        let confirm = if self.danger { confirm.danger() } else { confirm.primary() };
        ButtonGroup::new(vec![confirm, Button::new(self.cancel).focused(self.focused == 1)])
            .render(b, buf);
    }
}
