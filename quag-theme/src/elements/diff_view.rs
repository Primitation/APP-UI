//! ELEMENT: DiffView.

use super::*;
use crate::{
    Status,
    atoms::{self, Tone},
};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    text::{Line, Span},
    widgets::Widget,
};

/// Before / after of a config or compose file, to review before applying. `+3 −1` in the title line.
/// States: no changes → "Nothing to apply"; long diffs scroll (`scroll` = first line shown).
pub struct DiffView<'a> {
    pub title: &'a str,
    pub lines: &'a [(atoms::DiffKind, &'a str)],
    pub scroll: usize,
    pub focused: bool,
}

impl Widget for DiffView<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = atoms::panel(self.title, self.focused);
        let inner = block.inner(area);
        block.render(area, buf);
        let add = self.lines.iter().filter(|l| l.0 == atoms::DiffKind::Add).count();
        let del = self.lines.iter().filter(|l| l.0 == atoms::DiffKind::Del).count();
        let [top, _, body] = Layout::vertical([Constraint::Length(1), Constraint::Length(1), Constraint::Min(0)]).areas(inner);
        if add + del == 0 {
            EmptyState { message: "Nothing to apply: no changes.", hint: None }.render(body, buf);
            return;
        }
        Line::from(vec![
            atoms::status_label(&format!("+{add}"), Status::Ok), Span::raw(" "),
            atoms::status_label(&format!("−{del}"), Status::Err),
            atoms::label(&format!("   {} lines", self.lines.len()), Tone::Faint),
        ]).render(top, buf);
        for (i, (k, t)) in self.lines.iter().skip(self.scroll).enumerate().take(body.height as usize) {
            atoms::diff_line(*k, t).render(Rect { y: body.y + i as u16, height: 1, ..body }, buf);
        }
    }
}
