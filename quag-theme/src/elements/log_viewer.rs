//! ELEMENT: LogViewer.

use crate::{
    Status,
    atoms::{self, Tone},
    consts::layout as L,
    molecules::SearchBar,
};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    text::{Line, Span},
    widgets::Widget,
};

/// One line of any log the app has.
#[derive(Clone, Copy, Debug)]
pub struct LogEntry<'a> {
    pub time: &'a str,
    pub kind: Status,
    pub message: &'a str,
}

/// A big log: filter line, match count, only the visible lines are drawn (fine with 100k+ lines).
/// `follow`: stick to the newest line (End). Otherwise `scroll` = lines up from the bottom.
/// Matches of `filter` are highlighted like search matches (Gill).
pub struct LogViewer<'a> {
    pub title: &'a str,
    pub lines: &'a [LogEntry<'a>],
    pub filter: &'a str,
    pub filter_focused: bool,
    pub follow: bool,
    pub scroll: usize,
    pub focused: bool,
    /// Only show these kinds (None = all).
    pub only: Option<&'a [Status]>,
}

impl LogViewer<'_> {
    /// Indices of the lines that pass the filter (the app can use it for scrolling bounds).
    pub fn matching(&self) -> Vec<usize> {
        let f = self.filter.to_lowercase();
        self.lines.iter().enumerate()
            .filter(|(_, l)| self.only.is_none_or(|k| k.contains(&l.kind)))
            .filter(|(_, l)| f.is_empty() || l.message.to_lowercase().contains(&f))
            .map(|(i, _)| i)
            .collect()
    }
}

impl Widget for LogViewer<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = atoms::panel(self.title, self.focused);
        let inner = block.inner(area);
        block.render(area, buf);
        let [top, _, body] = Layout::vertical([Constraint::Length(1), Constraint::Length(1), Constraint::Min(0)]).areas(inner);
        let m = self.matching();
        let [search, count] = Layout::horizontal([Constraint::Max(L::SEARCH_MAX_COLS), Constraint::Min(0)]).spacing(L::GAP).areas(top);
        SearchBar { query: self.filter, focused: self.filter_focused }.render(search, buf);
        let state = if self.follow { atoms::label("● following", Tone::Accent) } else { atoms::label(&format!("↑ {} lines up", self.scroll), Tone::Muted) };
        Line::from(vec![atoms::label(&format!("{} / {} lines   ", m.len(), self.lines.len()), Tone::Faint), state])
            .alignment(Alignment::Right).render(count, buf);
        if m.is_empty() {
            Line::from(atoms::label("No line matches the filter.", Tone::Faint)).render(Rect { height: 1, ..body }, buf);
            return;
        }
        let h = body.height as usize;
        let scroll = if self.follow { 0 } else { self.scroll.min(m.len().saturating_sub(h)) };
        let end = m.len() - scroll;
        let start = end.saturating_sub(h);
        let f = self.filter.to_lowercase();
        for (y, &i) in m[start..end].iter().enumerate() {
            let l = &self.lines[i];
            let mut spans = vec![atoms::label(l.time, Tone::Faint), Span::raw("  "), atoms::status_icon(l.kind), Span::raw("  ")];
            // highlight the first match
            let lower = l.message.to_lowercase();
            match (!f.is_empty()).then(|| lower.find(&f)).flatten().filter(|p| l.message.is_char_boundary(*p) && l.message.is_char_boundary(p + f.len())) {
                Some(p) => {
                    spans.push(atoms::label(&l.message[..p], Tone::Text));
                    spans.push(atoms::mark(&l.message[p..p + f.len()]));
                    spans.push(atoms::label(&l.message[p + f.len()..], Tone::Text));
                }
                None => spans.push(atoms::label(l.message, Tone::Text)),
            }
            Line::from(spans).render(Rect { y: body.y + y as u16, height: 1, ..body }, buf);
        }
    }
}
