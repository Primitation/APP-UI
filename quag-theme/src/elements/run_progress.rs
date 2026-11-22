//! ELEMENT: RunProgress.

use super::*;
use crate::{
    Status,
    atoms::{self, Button, Tone},
    consts::layout as L,
    molecules::{Step, StepList, ButtonGroup},
};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    text::Line,
    widgets::Widget,
};

/// A running job (deploy, backup, migration): steps on top, its live output below, then Cancel or Retry.
/// It never blocks the app: it's a zone of a page, not a modal.
pub struct RunProgress<'a> {
    pub title: &'a str,
    pub steps: &'a [Step<'a>],
    pub output: &'a [LogEntry<'a>],
    pub tick: usize,
    pub focused: bool,
}

impl RunProgress<'_> {
    pub fn failed(&self) -> bool { self.steps.iter().any(|s| s.state == atoms::StepState::Failed) }
    pub fn finished(&self) -> bool { self.steps.iter().all(|s| matches!(s.state, atoms::StepState::Done | atoms::StepState::Skipped)) }
}

impl Widget for RunProgress<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let (failed, finished) = (self.failed(), self.finished());
        let block = atoms::panel(self.title, self.focused);
        let inner = block.inner(area);
        block.render(area, buf);
        let [steps, _, out, _, btn] = Layout::vertical([
            Constraint::Length(1), Constraint::Length(1), Constraint::Min(L::PROGRESS_OUTPUT_MIN_ROWS), Constraint::Length(1), Constraint::Length(1),
        ]).areas(inner);
        StepList { steps: self.steps, inline: true, tick: self.tick }.render(steps, buf);
        LogViewer { title: "Output", lines: self.output, filter: "", filter_focused: false, follow: true, scroll: 0, focused: false, only: None }.render(out, buf);
        let buttons = if failed {
            vec![Button::new("Retry").primary(), Button::new("Roll back")]
        } else if finished {
            vec![Button::new("Close").primary()]
        } else {
            vec![Button::new("Cancel")]
        };
        let mut l = vec![];
        if failed { l.push(atoms::status_label("failed", Status::Err)) } else if finished { l.push(atoms::status_label("done", Status::Ok)) } else { l.extend([atoms::spinner(self.tick), atoms::label(" running", Tone::Muted)]) }
        let [s, b] = Layout::horizontal([Constraint::Min(0), Constraint::Length(L::PROGRESS_BUTTONS_COLS)]).areas(btn);
        Line::from(l).render(s, buf);
        ButtonGroup::new(buttons).align(crate::molecules::Align::Right).render(b, buf);
    }
}
