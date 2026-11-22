//! ELEMENTS: molecules put to use. An element has a job in the app: it is wired
//! to data and actions, and it owns its states (ideal / empty / loading / error).
//!
//! Most elements are app-specific and live in the app (`src/ui/elements/`), e.g.
//! `TaskActions` = a `ButtonGroup` with "Mark done / Edit / Delete" for the selected task.
//! The ones below are needed by every app, so they live here.
//! See STRUCTURE.md §4.

use crate::{
    Status,
    atoms::{self, Button, Tone},
    consts::layout as L,
    keymap::Keymap,
    molecules::{Step, StepList, ButtonGroup, KeyHints, Menu, MenuItem, SearchBar, StatusLine, Tabs},
    ui,
};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    text::{Line, Span},
    widgets::{Clear, Paragraph, Widget},
};

// ───────────────────────────── AppHeader ─────────────────────────────

/// `◆ app  location ··················· tab  tab  tab`
pub struct AppHeader<'a> {
    pub app: &'a str,
    pub location: &'a str,
    pub tabs: Option<Tabs<'a>>,
    /// Server tools: the environment, always visible (`[prod]` in red).
    pub env: Option<atoms::Env>,
}

impl Widget for AppHeader<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [left, right] = ui::header_split(area);
        let mut l = vec![atoms::label(&format!("◆ {}", self.app), Tone::Title), Span::raw("  ")];
        if let Some(e) = self.env { l.extend([atoms::env_tag(e), Span::raw("  ")]); }
        l.push(atoms::label(self.location, Tone::Muted));
        Line::from(l).render(left, buf);
        if let Some(t) = self.tabs {
            t.to_line().alignment(Alignment::Right).render(right, buf);
        }
    }
}

// ───────────────────────────── StatusBar ─────────────────────────────

/// Key hints left, last result right.
pub struct StatusBar<'a> {
    pub hints: KeyHints<'a>,
    pub status: Option<StatusLine<'a>>,
}

impl Widget for StatusBar<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [l, r] = ui::footer_split(area);
        self.hints.render(l, buf);
        if let Some(s) = self.status {
            Line::from(s.to_span()).alignment(Alignment::Right).render(r, buf);
        }
    }
}

// ───────────────────────────── EmptyState ─────────────────────────────

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

// ───────────────────────────── ConfirmDialog ─────────────────────────────

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

// ───────────────────────────── ErrorState ─────────────────────────────

/// The error state of a zone (STRUCTURE.md §7.4): `✗` + what failed + why + "Retry".
/// Only the zone that failed shows it; the rest of the page stays usable.
pub struct ErrorState<'a> {
    pub what: &'a str,
    pub why: &'a str,
    pub retry_focused: bool,
}

impl Widget for ErrorState<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let col = ui::focus_column(area, 4);
        let [t, d, _, b] = Layout::vertical([Constraint::Length(1); 4]).areas(col);
        Line::from(vec![atoms::status_icon(Status::Err), Span::raw("  "), atoms::status_label(self.what, Status::Err)])
            .alignment(Alignment::Center)
            .render(t, buf);
        Line::from(atoms::label(self.why, Tone::Muted)).alignment(Alignment::Center).render(d, buf);
        let btn = Button::new("Retry").focused(self.retry_focused);
        let w = btn.width();
        btn.render(Rect { x: b.x + b.width.saturating_sub(w) / 2, width: w.min(b.width), ..b }, buf);
    }
}

// ───────────────────────────── Toast ─────────────────────────────

/// A short-lived result message in a small box, bottom-right above the status bar.
/// Use for results that matter more than the status line (desktop uses it more).
pub struct Toast<'a> {
    pub kind: Status,
    pub message: &'a str,
}

impl Toast<'_> {
    pub fn area(&self, screen: Rect) -> Rect {
        let (w, h) = atoms::popup_size(Span::raw(self.message).width() as u16 + L::TOAST_ICON, 1);
        let w = w.min(screen.width.saturating_sub(L::SCREEN_MARGIN));
        // bottom-right, leaving the status bar row free
        Rect { x: screen.right().saturating_sub(w + 1), y: screen.bottom().saturating_sub(h + 1), width: w, height: h }
    }
}

impl Widget for Toast<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Clear.render(area, buf);
        let block = atoms::popup("");
        let inner = block.inner(area);
        block.render(area, buf);
        StatusLine { kind: self.kind, message: self.message }.render(Rect { x: inner.x + 1, width: inner.width.saturating_sub(1), ..inner }, buf);
    }
}

// ───────────────────────────── CommandPalette ─────────────────────────────

/// `ctrl+k`: search every action of the app by name. A SearchBar over a Menu.
pub struct CommandPalette<'a> {
    pub query: &'a str,
    /// Already filtered by the app.
    pub items: &'a [MenuItem<'a>],
    pub selected: usize,
}

impl CommandPalette<'_> {
    /// Top third of the screen, 60 wide (it grows down as you'd read).
    pub fn area(&self, screen: Rect) -> Rect {
        let w = L::PALETTE_COLS.min(screen.width.saturating_sub(2 * L::SCREEN_MARGIN));
        let h = atoms::popup_size(0, self.items.len().max(1) as u16 + L::PALETTE_HEAD_ROWS).1.min(screen.height.saturating_sub(L::SCREEN_MARGIN));
        Rect { x: screen.x + (screen.width - w) / 2, y: screen.y + screen.height / L::PALETTE_TOP_SHARE, width: w, height: h }
    }
}

impl Widget for CommandPalette<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Clear.render(area, buf);
        let block = atoms::popup("Commands");
        let inner = block.inner(area);
        block.render(area, buf);
        let [search, _, list] = Layout::vertical([Constraint::Length(1), Constraint::Length(1), Constraint::Min(0)]).areas(inner);
        SearchBar { query: self.query, focused: true }.render(search, buf);
        if self.items.is_empty() {
            Line::from(atoms::label("No command matches.", Tone::Faint)).render(list, buf);
            return;
        }
        // The list part of a Menu, without its own box
        Menu { title: "", items: self.items, selected: self.selected }.render_rows(list, buf);
    }
}

// ───────────────────────────── HelpOverlay ─────────────────────────────

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

// ───────────────────────────── LogViewer ─────────────────────────────

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

// ───────────────────────────── TypeToConfirm ─────────────────────────────

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

// ───────────────────────────── DiffView ─────────────────────────────

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

// ───────────────────────────── RunProgress ─────────────────────────────

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn help_wraps_long_descriptions() {
        assert_eq!(wrap_words("move in the focused list", 10), vec!["move in", "the", "focused", "list"]);
        assert_eq!(wrap_words("short", 10), vec!["short"]);
        assert_eq!(wrap_words("averyveryverylongword x", 5), vec!["averyveryverylongword", "x"]);
    }

    #[test]
    fn type_to_confirm_needs_the_exact_name() {
        let d = |typed| TypeToConfirm { title: "", question: "", name: "api-1", typed, confirm: "Delete" };
        assert!(!d("").matches());
        assert!(!d("api").matches());
        assert!(!d("API-1").matches(), "case matters");
        assert!(d("api-1").matches());
    }
}
