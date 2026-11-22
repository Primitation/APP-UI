//! MOLECULE: Field.

use crate::{
    Status,
    atoms::{self, Input, Tone},
};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::{Line, Span},
    widgets::Widget,
};

/// Label above an input, with help or error text under it. Height: 3.
#[derive(Clone, Debug)]
pub struct Field<'a> {
    pub label: &'a str,
    pub input: Input<'a>,
    pub help: Option<&'a str>,
    pub error: Option<&'a str>,
}

impl<'a> Field<'a> {
    pub fn new(label: &'a str, input: Input<'a>) -> Self { Self { label, input, help: None, error: None } }
    pub fn help(mut self, h: &'a str) -> Self { self.help = Some(h); self }
    pub fn error(mut self, e: &'a str) -> Self { self.error = Some(e); self }
    pub const HEIGHT: u16 = 3;
}

impl Widget for Field<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Tight space: drop help first, then the label. The input always stays, and rows never overlap.
        let row = |n: u16| (n < area.height).then(|| Rect { y: area.y + n, height: 1, ..area });
        let (l, i, h) = match area.height {
            0 => return,
            1 => (None, row(0), None),
            2 => (row(0), row(1), None),
            _ => (row(0), row(1), row(2)),
        };
        let focused = self.input.state == atoms::State::Focused;
        if let Some(l) = l {
            Line::from(atoms::label(self.label, if focused { Tone::Accent } else { Tone::Muted })).render(l, buf);
        }
        let invalid = self.error.is_some();
        if let Some(i) = i {
            self.input.invalid(invalid).render(i, buf);
        }
        let Some(h) = h else { return };
        if let Some(e) = self.error {
            Line::from(Span::styled(e.to_string(), Status::Err.style())).render(h, buf);
        } else if let Some(t) = self.help {
            Line::from(atoms::label(t, Tone::Faint)).render(h, buf);
        }
    }
}
