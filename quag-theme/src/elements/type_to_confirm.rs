//! ELEMENT: TypeToConfirm.

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

/// For actions that can't be undone on something important (delete a server, revoke a key):
/// the user types the thing's name. The danger button stays disabled until it matches.
pub struct TypeToConfirm<'a> {
    pub title: &'a str,
    pub question: &'a str,
    /// What must be typed, e.g. the server name.
    pub name: &'a str,
    pub typed: &'a str,
    pub confirm: &'a str,
}

impl TypeToConfirm<'_> {
    pub fn matches(&self) -> bool { self.typed == self.name }
    pub fn area(&self, screen: Rect) -> Rect {
        let w = Span::raw(self.question).width().max(L::TYPE_CONFIRM_MIN_COLS as usize) as u16;
        let (w, h) = atoms::popup_size(w.min(L::TYPE_CONFIRM_MAX_COLS), L::TYPE_CONFIRM_ROWS);
        ui::centered(screen, w, h)
    }
}

impl Widget for TypeToConfirm<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Clear.render(area, buf);
        let block = atoms::popup(self.title);
        let inner = block.inner(area);
        block.render(area, buf);
        let [q_, _, l, i, _, b] = Layout::vertical([Constraint::Length(1); 6]).areas(inner);
        Line::from(atoms::label(self.question, Tone::Text)).render(q_, buf);
        Line::from(vec![atoms::label("Type ", Tone::Muted), atoms::kbd(self.name), atoms::label(" to confirm:", Tone::Muted)]).render(l, buf);
        atoms::Input::new(self.typed).focused(true).invalid(!self.typed.is_empty() && !self.name.starts_with(self.typed)).render(i, buf);
        let ok = self.matches();
        ButtonGroup::new(vec![Button::new(self.confirm).danger().disabled(!ok), Button::new("Cancel")]).render(b, buf);
    }
}
