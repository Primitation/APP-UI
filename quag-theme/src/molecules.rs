//! MOLECULES: a few atoms arranged together, usually in a box.
//! Still generic: a molecule has no idea what app it's in. It takes values and
//! labels as props; it doesn't fetch data or decide what buttons do.
//! See STRUCTURE.md §3.

use crate::{
    self as q, Status,
    atoms::{self, Button, Input, Tone},
};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Modifier,
    text::{Line, Span},
    widgets::Widget,
};

/// Gap between atoms inside one molecule (STRUCTURE.md: "inside a group").
pub use crate::consts::layout::GAP;
use crate::consts::{glyph as G, layout as L};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Align {
    #[default]
    Left,
    Right,
}

// ───────────────────────────── ButtonGroup ─────────────────────────────

/// One button of a row, written ONCE: what it says, what it does (a keymap action id) and how it looks.
/// The row is drawn from the table (`ActionButton::group`) and pressed from it (`keymap::ButtonRow`),
/// so a label can never end up on another button's action.
#[derive(Clone, Copy, Debug)]
pub struct ActionButton<'a> {
    pub label: &'a str,
    pub action: &'a str,
    pub kind: crate::atoms::ButtonKind,
}

impl<'a> ActionButton<'a> {
    pub const fn new(label: &'a str, action: &'a str) -> Self { Self { label, action, kind: crate::atoms::ButtonKind::Secondary } }
    pub const fn primary(mut self) -> Self { self.kind = crate::atoms::ButtonKind::Primary; self }
    pub const fn danger(mut self) -> Self { self.kind = crate::atoms::ButtonKind::Danger; self }
    pub const fn ghost(mut self) -> Self { self.kind = crate::atoms::ButtonKind::Ghost; self }

    /// The row as a ButtonGroup, `focused` = the button the keyboard is on.
    pub fn group(row: &[ActionButton<'a>], focused: Option<usize>) -> ButtonGroup<'a> {
        ButtonGroup::new(row.iter().enumerate().map(|(i, b)| {
            Button { kind: b.kind, ..Button::new(b.label) }.focused(focused == Some(i))
        }).collect())
    }
}

/// An array of buttons in a row, optionally inside a titled box.
/// Order rule (STRUCTURE.md): in the terminal the Primary button comes FIRST.
#[derive(Clone, Debug, Default)]
pub struct ButtonGroup<'a> {
    pub buttons: Vec<Button<'a>>,
    pub align: Align,
    /// `Some(title)` draws a panel around the group ("" = untitled box).
    pub boxed: Option<&'a str>,
}

impl<'a> ButtonGroup<'a> {
    pub fn new(buttons: Vec<Button<'a>>) -> Self { Self { buttons, ..Default::default() } }
    pub fn align(mut self, a: Align) -> Self { self.align = a; self }
    pub fn boxed(mut self, title: &'a str) -> Self { self.boxed = Some(title); self }

    /// Width of the buttons row (without the box).
    pub fn width(&self) -> u16 {
        let n = self.buttons.len() as u16;
        let gap = if self.buttons.first().is_some_and(|b| b.compact) { 1 } else { GAP };
        self.buttons.iter().map(|b| b.width()).sum::<u16>() + gap * n.saturating_sub(1)
    }
    /// Height needed: 1, or 3 when boxed.
    pub fn height(&self) -> u16 { if self.boxed.is_some() { 3 } else { 1 } }
}

impl Widget for ButtonGroup<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let inner = match self.boxed {
            Some(t) => {
                let b = atoms::panel(t, false);
                let i = b.inner(area);
                b.render(area, buf);
                i
            }
            None => area,
        };
        // Never drop a button silently: tighten padding first, then wrap to the next row.
        let mut group = self;
        if group.width() > inner.width {
            group.buttons = group.buttons.into_iter().map(|b| b.compact(true)).collect();
        }
        let gap = if group.buttons.first().is_some_and(|b| b.compact) { 1 } else { GAP };
        let w = group.width().min(inner.width);
        let start = match group.align {
            Align::Left => inner.x,
            Align::Right => inner.right().saturating_sub(w),
        };
        let (mut x, mut y) = (start, inner.y);
        for b in group.buttons {
            let bw = b.width().min(inner.width);
            if x + bw > inner.right() && x != start {
                x = start;
                y += 1;
            }
            if y >= inner.bottom() { break; }
            b.render(Rect::new(x, y, bw, 1), buf);
            x += bw + gap;
        }
    }
}

// ───────────────────────────── Field ─────────────────────────────

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

// ───────────────────────────── SearchBar ─────────────────────────────

/// `/` key + input. Height: 1.
pub struct SearchBar<'a> {
    pub query: &'a str,
    pub focused: bool,
}

impl Widget for SearchBar<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [k, i] = Layout::horizontal([Constraint::Length(2), Constraint::Min(0)]).areas(area);
        Line::from(atoms::kbd("/")).render(k, buf);
        Input::new(self.query).placeholder("Search…").focused(self.focused).render(i, buf);
    }
}

// ───────────────────────────── Tabs ─────────────────────────────

/// Row of tab labels; the active one is accent + underlined.
pub struct Tabs<'a> {
    pub names: &'a [&'a str],
    pub active: usize,
}

impl Tabs<'_> {
    pub fn to_line(&self) -> Line<'static> {
        let mut spans = Vec::new();
        for (i, n) in self.names.iter().enumerate() {
            if i > 0 { spans.push(Span::raw("   ")); }
            let s = if i == self.active {
                q::accent().add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
            } else {
                q::muted()
            };
            spans.push(Span::styled(n.to_string(), s));
        }
        Line::from(spans)
    }
    pub fn width(&self) -> u16 { self.to_line().width() as u16 }
}

impl Widget for Tabs<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) { self.to_line().render(area, buf) }
}

// ───────────────────────────── KeyHints ─────────────────────────────

/// `q quit · / search · ? help`
pub struct KeyHints<'a>(pub &'a [(&'a str, &'a str)]);

impl KeyHints<'_> {
    pub fn to_line(&self) -> Line<'static> {
        let mut spans = Vec::new();
        for (i, (key, label)) in self.0.iter().enumerate() {
            if i > 0 { spans.push(Span::styled("  ·  ", q::faint())); }
            spans.push(atoms::kbd(key));
            spans.push(Span::styled(format!(" {label}"), q::muted()));
        }
        Line::from(spans)
    }
}

impl Widget for KeyHints<'_> {
    /// Drops whole hints from the end instead of cutting one in half.
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut n = self.0.len();
        while n > 0 && KeyHints(&self.0[..n]).to_line().width() as u16 > area.width { n -= 1; }
        KeyHints(&self.0[..n]).to_line().render(area, buf)
    }
}

// ───────────────────────────── StatusLine ─────────────────────────────

/// `✓ Saved 2s ago`: icon + message, both in the status color.
pub struct StatusLine<'a> {
    pub kind: Status,
    pub message: &'a str,
}

impl StatusLine<'_> {
    pub fn to_span(&self) -> Span<'static> {
        Span::styled(format!("{} {}", self.kind.icon(), self.message), self.kind.style())
    }
}

impl Widget for StatusLine<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) { Line::from(self.to_span()).render(area, buf) }
}

// ───────────────────────────── ListRow ─────────────────────────────

/// One row of a list: `▌ [✓] title            #tag  ! due`.
/// Left: optional checkbox + title. Right (aligned): tags, status icon, meta.
#[derive(Clone, Debug, Default)]
pub struct ListRow<'a> {
    pub title: &'a str,
    pub check: Option<bool>,
    pub tags: &'a [&'a str],
    pub status: Option<Status>,
    pub meta: &'a str,
    pub meta_accent: bool,
    /// Reserve a column for the status icon even when `status` is None (keeps columns aligned in task lists).
    pub status_slot: bool,
    /// Right-align `meta` in this many cells (0 = its own width).
    pub meta_width: usize,
    pub selected: bool,
    pub list_focused: bool,
    /// A leading badge before the title (e.g. `atoms::severity_badge`).
    pub lead: Option<Span<'static>>,
}

impl Widget for &ListRow<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area = Rect { height: 1, ..area };
        let done = self.check == Some(true);
        if self.selected {
            let s = if self.list_focused { q::selected() } else { q::selected_unfocused() };
            buf.set_style(area, s);
            buf[(area.x, area.y)].set_symbol("▌").set_fg(q::c(if self.list_focused { q::QUAG } else { q::MUTED }));
        }
        // right side first, so we know how much room the title gets
        let mut right: Vec<Span> = self.tags.iter().map(|t| atoms::tag(t)).flat_map(|t| [t, Span::raw("  ")]).collect();
        match (self.status, self.status_slot) {
            (Some(s), _) => right.extend([atoms::status_icon(s), Span::raw("  ")]),
            (None, true) => right.push(Span::raw("   ")),
            (None, false) => {}
        }
        let w = self.meta_width;
        right.push(Span::styled(format!("{:>w$}", self.meta), if self.meta_accent { q::accent() } else { q::muted() }));
        let right = Line::from(right);
        let rw = right.width() as u16;
        let [l, r] = Layout::horizontal([Constraint::Min(0), Constraint::Length(rw + L::GAP)]).areas(Rect {
            x: area.x + L::ROW_INDENT,
            width: area.width.saturating_sub(L::ROW_INDENT),
            ..area
        });

        let mut left = Vec::new();
        if let Some(b) = &self.lead {
            left.extend([b.clone(), Span::raw(" ")]);
        }
        if let Some(c) = self.check {
            left.push(Span::styled(if c { "✓ " } else { "○ " }, if c { Status::Ok.style() } else { q::faint() }));
        }
        let title_style = if done {
            q::faint()
        } else if self.selected && self.list_focused {
            q::selected()
        } else {
            q::text()
        };
        // Overflow rule: truncate with … (the full text lives in the inspector / tooltip)
        let room = (l.width as usize).saturating_sub(left.iter().map(|s| s.width()).sum::<usize>());
        let title: String = if Span::raw(self.title).width() > room && room > 1 {
            let mut t = String::new();
            for ch in self.title.chars() {
                if Span::raw(format!("{t}{ch}…")).width() > room { break; }
                t.push(ch);
            }
            format!("{t}…")
        } else {
            self.title.to_string()
        };
        left.push(Span::styled(title, title_style));
        Line::from(left).render(l, buf);
        // keep selection bg behind the right part
        let mut right = right;
        if self.selected {
            let bg = if self.list_focused { q::selected() } else { q::selected_unfocused() };
            right = right.patch_style(ratatui::style::Style::new().bg(bg.bg.unwrap_or(q::c(q::BG))));
        }
        right.alignment(ratatui::layout::Alignment::Right).render(r, buf);
    }
}

// ───────────────────────────── Stat ─────────────────────────────

/// A number with its label, in a box (dashboard KPI). Height: 4.
pub struct Stat<'a> {
    pub value: &'a str,
    pub label: &'a str,
}

impl Stat<'_> {
    pub const HEIGHT: u16 = 4;
}

impl Widget for Stat<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let b = atoms::panel("", false);
        let inner = b.inner(area);
        b.render(area, buf);
        let [v, l] = Layout::vertical([Constraint::Length(1), Constraint::Length(1)]).areas(inner);
        Line::from(Span::styled(self.value.to_string(), q::text().add_modifier(Modifier::BOLD))).render(v, buf);
        Line::from(atoms::label(self.label, Tone::Muted)).render(l, buf);
    }
}

// ───────────────────────────── CheckList ─────────────────────────────

/// A column of checkboxes. `focused` = index with keyboard focus.
pub struct CheckList<'a> {
    pub items: &'a [(&'a str, bool)],
    pub focused: Option<usize>,
}

impl Widget for CheckList<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        for (i, (label, checked)) in self.items.iter().enumerate().take(area.height as usize) {
            atoms::Checkbox::new(label, *checked)
                .focused(self.focused == Some(i))
                .render(Rect { y: area.y + i as u16, height: 1, ..area }, buf);
        }
    }
}

// ───────────────────────────── LogLine ─────────────────────────────

/// One line of a stream: `12:01:03  ✗  message`. Time faint, icon colored, text normal.
pub struct LogLine<'a> {
    pub time: &'a str,
    pub kind: Status,
    pub message: &'a str,
}

impl Widget for LogLine<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Line::from(vec![
            atoms::label(self.time, Tone::Faint),
            Span::raw("  "),
            atoms::status_icon(self.kind),
            Span::raw("  "),
            atoms::label(self.message, Tone::Text),
        ])
        .render(area, buf);
    }
}

// ───────────────────────────── helpers ─────────────────────────────

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

// ───────────────────────────── RadioGroup ─────────────────────────────

/// One choice among a few options, in a row (or a column when it doesn't fit).
pub struct RadioGroup<'a> {
    pub options: &'a [&'a str],
    pub selected: usize,
    pub focused: Option<usize>,
}

impl RadioGroup<'_> {
    /// Rows needed at this width (1 when it fits in a row, else one per option).
    pub fn height(&self, width: u16) -> u16 {
        let w: u16 = self.options.iter().map(|o| atoms::Radio::new(o, false).width() + GAP).sum();
        if w.saturating_sub(GAP) <= width { 1 } else { self.options.len() as u16 }
    }
}

impl Widget for RadioGroup<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let row = self.height(area.width) == 1;
        let (mut x, mut y) = (area.x, area.y);
        for (i, o) in self.options.iter().enumerate() {
            if y >= area.bottom() { break; }
            let r = atoms::Radio::new(o, i == self.selected).focused(self.focused == Some(i));
            let w = r.width().min(area.right().saturating_sub(x));
            r.render(Rect::new(x, y, w, 1), buf);
            if row { x += w + GAP } else { y += 1 }
        }
    }
}

// ───────────────────────────── Breadcrumbs ─────────────────────────────

/// Where you are: `pond › quag-theme › Write README`. The last crumb is the current place.
/// Too long: the middle crumbs collapse to `…`, the first and last always stay.
pub struct Breadcrumbs<'a>(pub &'a [&'a str]);

impl Widget for Breadcrumbs<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let n = self.0.len();
        let build = |skip_mid: bool| -> Line<'static> {
            let mut s = Vec::new();
            for (i, c) in self.0.iter().enumerate() {
                if skip_mid && i > 0 && i < n - 1 {
                    if i == 1 { s.extend([atoms::label("  ›  ", Tone::Faint), atoms::label("…", Tone::Faint)]); }
                    continue;
                }
                if i > 0 { s.push(atoms::label("  ›  ", Tone::Faint)); }
                s.push(atoms::label(c, if i == n - 1 { Tone::Text } else { Tone::Muted }));
            }
            Line::from(s)
        };
        let full = build(false);
        let line = if full.width() as u16 <= area.width || n < 3 { full } else { build(true) };
        line.render(Rect { height: 1, ..area }, buf);
    }
}

// ───────────────────────────── Menu ─────────────────────────────

/// One action of a menu: label, its shortcut key ("" = none), and whether it destroys something.
#[derive(Clone, Copy, Debug, Default)]
pub struct MenuItem<'a> {
    pub label: &'a str,
    pub key: &'a str,
    pub danger: bool,
}

/// A column of actions with their keys, in a popup box. Danger items are red and come last.
pub struct Menu<'a> {
    pub title: &'a str,
    pub items: &'a [MenuItem<'a>],
    pub selected: usize,
}

impl Menu<'_> {
    /// Size it needs (fits its longest label + key).
    pub fn size(&self) -> (u16, u16) {
        let w = self.items.iter().map(|i| Span::raw(i.label).width() + Span::raw(i.key).width() + L::MENU_ITEM_PAD).max().unwrap_or(0) as u16;
        atoms::popup_size(w.max(Span::raw(self.title).width() as u16 + L::MENU_TITLE_PAD).max(L::MENU_MIN_COLS), self.items.len() as u16)
    }
}

impl Widget for Menu<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        ratatui::widgets::Clear.render(area, buf);
        let block = atoms::popup(self.title);
        let inner = block.inner(area);
        block.render(area, buf);
        self.render_rows(inner, buf);
    }
}

impl Menu<'_> {
    /// Just the rows, no box (for molecules/elements that bring their own box).
    pub fn render_rows(&self, inner: Rect, buf: &mut Buffer) {
        for (i, it) in self.items.iter().enumerate().take(inner.height as usize) {
            let r = Rect { y: inner.y + i as u16, height: 1, ..inner };
            let row = ListRow {
                title: it.label,
                meta: it.key,
                selected: i == self.selected,
                list_focused: true,
                ..Default::default()
            };
            (&row).render(r, buf);
            if it.danger {
                // the label in the danger tone (an atom decides the color)
                let label = atoms::status_label(it.label, Status::Err);
                Line::from(label).render(Rect { x: r.x + L::ROW_INDENT, width: r.width.saturating_sub(L::ROW_INDENT), ..r }, buf);
            }
        }
    }
}

// ───────────────────────────── Table ─────────────────────────────

/// One column: header, width (0 = take the rest), right-aligned (numbers).
#[derive(Clone, Copy, Debug)]
pub struct Column<'a> {
    pub header: &'a str,
    pub width: u16,
    pub right: bool,
}

/// Rows and columns with a header line. Cells truncate with `…`; columns never move.
pub struct Table<'a> {
    pub columns: &'a [Column<'a>],
    pub rows: &'a [&'a [&'a str]],
    pub selected: Option<usize>,
    pub focused: bool,
}

impl Table<'_> {
    /// The cell areas of data row `row` (0 = first row under the header), the same columns `render` uses.
    /// For elements that put atoms in cells (a state icon, a sparkline, a meter): render the Table with
    /// "" in those cells, then draw the atoms into these areas.
    pub fn cell_areas(&self, area: Rect, row: usize) -> Vec<Rect> {
        let cons: Vec<Constraint> = self.columns.iter().map(|c| if c.width == 0 { Constraint::Min(0) } else { Constraint::Length(c.width) }).collect();
        let body = Rect { x: area.x + L::ROW_INDENT, width: area.width.saturating_sub(L::ROW_INDENT), y: area.y + 1 + row as u16, height: 1 };
        Layout::horizontal(cons).spacing(GAP).split(body).to_vec()
    }
}

impl Widget for Table<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let cons: Vec<Constraint> = self.columns.iter().map(|c| if c.width == 0 { Constraint::Min(0) } else { Constraint::Length(c.width) }).collect();
        let body = Rect { x: area.x + L::ROW_INDENT, width: area.width.saturating_sub(L::ROW_INDENT), ..area };
        let cells = |y: u16| Layout::horizontal(cons.clone()).spacing(GAP).split(Rect { y, height: 1, ..body });
        let cell = |text: &str, r: Rect, right: bool, tone: Tone, buf: &mut Buffer| {
            let t = truncate(text, r.width as usize);
            let l = Line::from(atoms::label(&t, tone));
            if right { l.alignment(ratatui::layout::Alignment::Right).render(r, buf) } else { l.render(r, buf) }
        };
        if area.height == 0 { return; }
        for (c, r) in self.columns.iter().zip(cells(area.y).iter()) {
            cell(c.header, *r, c.right, Tone::Heading, buf);
        }
        for (i, row) in self.rows.iter().enumerate().take(area.height.saturating_sub(1) as usize) {
            let y = area.y + 1 + i as u16;
            let sel = self.selected == Some(i);
            if sel {
                (&ListRow { selected: true, list_focused: self.focused, ..Default::default() }).render(Rect { y, height: 1, ..area }, buf);
            }
            for ((c, r), text) in self.columns.iter().zip(cells(y).iter()).zip(row.iter()) {
                let tone = if c.right { Tone::Muted } else { Tone::Text };
                cell(text, *r, c.right, tone, buf);
            }
            if sel {
                let s = if self.focused { q::selected() } else { q::selected_unfocused() };
                buf.set_style(Rect { x: area.x + 1, y, width: area.width.saturating_sub(1), height: 1 }, ratatui::style::Style::new().bg(s.bg.unwrap_or(q::c(q::BG))));
            }
        }
    }
}

// ───────────────────────────── PropertyList ─────────────────────────────

/// Read-only facts as `key   value` rows, keys aligned (inspector, about boxes).
pub struct PropertyList<'a>(pub &'a [(&'a str, &'a str)]);

impl Widget for PropertyList<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let kw = self.0.iter().map(|(k, _)| Span::raw(*k).width()).max().unwrap_or(0) as u16;
        for (i, (k, v)) in self.0.iter().enumerate().take(area.height as usize) {
            let [kr, _, vr] = Layout::horizontal([Constraint::Length(kw), Constraint::Length(GAP), Constraint::Min(0)])
                .areas(Rect { y: area.y + i as u16, height: 1, ..area });
            Line::from(atoms::label(k, Tone::Muted)).render(kr, buf);
            Line::from(atoms::label(&truncate(v, vr.width as usize), Tone::Text)).render(vr, buf);
        }
    }
}

// ───────────────────────────── TreeView ─────────────────────────────

/// One node of a tree (static or built by the app). `id` must be unique; it's how expansion is remembered.
#[derive(Clone, Copy, Debug)]
pub struct TreeItem<'a> {
    pub id: &'a str,
    pub label: &'a str,
    pub meta: &'a str,
    pub status: Option<Status>,
    pub children: &'a [TreeItem<'a>],
}

impl<'a> TreeItem<'a> {
    pub const fn leaf(id: &'a str, label: &'a str) -> Self { Self { id, label, meta: "", status: None, children: &[] } }
    pub const fn branch(id: &'a str, label: &'a str, children: &'a [TreeItem<'a>]) -> Self { Self { id, label, meta: "", status: None, children } }
}

/// A visible row of a tree, after expansion.
#[derive(Clone, Copy, Debug)]
pub struct TreeRow<'a> {
    pub item: &'a TreeItem<'a>,
    pub depth: usize,
    /// None = leaf, Some(open)
    pub open: Option<bool>,
    /// For the guide lines: is this the last child of its parent?
    pub last: bool,
}

/// The rows to show, in order. `is_open(id)` comes from the app's state (e.g. a HashSet of ids).
pub fn flatten<'a>(items: &'a [TreeItem<'a>], is_open: &dyn Fn(&str) -> bool) -> Vec<TreeRow<'a>> {
    fn walk<'a>(items: &'a [TreeItem<'a>], depth: usize, is_open: &dyn Fn(&str) -> bool, out: &mut Vec<TreeRow<'a>>) {
        for (i, it) in items.iter().enumerate() {
            let open = (!it.children.is_empty()).then(|| is_open(it.id));
            out.push(TreeRow { item: it, depth, open, last: i == items.len() - 1 });
            if open == Some(true) {
                walk(it.children, depth + 1, is_open, out);
            }
        }
    }
    let mut out = Vec::new();
    walk(items, 0, is_open, &mut out);
    out
}

/// A tree: `▾ folder` / `▸ folder` / `  file`, indented 2 per level, with selection and scrolling.
/// The app owns expansion and selection (→ opens, ← closes, ↑↓ moves).
pub struct TreeView<'a> {
    pub rows: &'a [TreeRow<'a>],
    pub selected: usize,
    pub focused: bool,
}

impl Widget for TreeView<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let h = area.height as usize;
        let offset = self.selected.saturating_sub(h.saturating_sub(2)).min(self.rows.len().saturating_sub(h));
        for (y, (i, row)) in self.rows.iter().enumerate().skip(offset).take(h).enumerate() {
            let r = Rect { y: area.y + y as u16, height: 1, ..area };
            let arrow = match row.open { Some(true) => "▾ ", Some(false) => "▸ ", None => "  " };
            let indent = "  ".repeat(row.depth);
            let title = format!("{indent}{arrow}{}", row.item.label);
            let lr = ListRow {
                title: &title,
                status: row.item.status,
                meta: row.item.meta,
                selected: i == self.selected,
                list_focused: self.focused,
                ..Default::default()
            };
            (&lr).render(r, buf);
        }
        if self.rows.is_empty() {
            Line::from(atoms::label("Empty.", Tone::Faint)).render(Rect { height: 1, ..area }, buf);
        }
    }
}

// ───────────────────────────── ParamRow ─────────────────────────────

/// A numeric parameter: `Roughness   ━━━━●─────  ◂  0.45 ▸`. Label · Slider · NumberField.
/// The tech-art / tuning row. Narrow: the slider goes first, the number always stays.
pub struct ParamRow<'a> {
    pub label: &'a str,
    pub label_width: u16,
    pub field: atoms::NumberField<'a>,
}

impl Widget for ParamRow<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area = Rect { height: 1, ..area };
        let num_w = (self.field.text().chars().count() as u16 + L::PARAM_NUMBER_PAD).max(L::PARAM_NUMBER_MIN);
        let show_slider = area.width >= self.label_width + num_w + L::PARAM_SLIDER_MIN;
        let [l, s, n] = Layout::horizontal([
            Constraint::Length(self.label_width),
            Constraint::Min(if show_slider { L::PARAM_SLIDER_MIN_COLS } else { 0 }),
            Constraint::Length(num_w),
        ])
        .spacing(GAP)
        .areas(area);
        let focused = self.field.state == atoms::State::Focused;
        Line::from(atoms::label(&truncate(self.label, l.width as usize), if focused { Tone::Accent } else { Tone::Muted })).render(l, buf);
        if show_slider {
            atoms::Slider { value: self.field.value, min: self.field.min, max: self.field.max, state: self.field.state }.render(s, buf);
        }
        self.field.render(n, buf);
    }
}

// ───────────────────────────── BarChart ─────────────────────────────

/// Horizontal bars with labels and values: compare a few things (memory per asset type, time per system).
/// `budget`: bars over it turn red. Rows: one per item.
pub struct BarChart<'a> {
    pub items: &'a [(&'a str, f64)],
    pub unit: &'a str,
    pub max: Option<f64>,
    pub budget: Option<f64>,
}

impl Widget for BarChart<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let max = self.max.unwrap_or_else(|| self.items.iter().map(|i| i.1).fold(0.0, f64::max)).max(f64::EPSILON);
        let lw = self.items.iter().map(|(l, _)| Span::raw(*l).width()).max().unwrap_or(0).min(L::CHART_LABEL_MAX) as u16;
        let vals: Vec<String> = self.items.iter().map(|(_, v)| fmt_num(*v, self.unit)).collect();
        let vw = vals.iter().map(|v| v.len()).max().unwrap_or(0) as u16;
        for (i, ((label, v), vs)) in self.items.iter().zip(&vals).enumerate().take(area.height as usize) {
            let [l, b, n] = Layout::horizontal([Constraint::Length(lw), Constraint::Min(L::BAR_MIN_COLS), Constraint::Length(vw)])
                .spacing(GAP)
                .areas(Rect { y: area.y + i as u16, height: 1, ..area });
            Line::from(atoms::label(&truncate(label, l.width as usize), Tone::Muted)).render(l, buf);
            atoms::Bar { fraction: v / max, over: self.budget.is_some_and(|bd| *v > bd) }.render(b, buf);
            Line::from(atoms::label(vs, Tone::Text)).alignment(ratatui::layout::Alignment::Right).render(n, buf);
        }
    }
}

/// Number + unit the house way (DESIGN.md "Numbers"): ≤ 3 significant digits after the point, thin unit.
pub fn fmt_num(v: f64, unit: &str) -> String {
    let n = if v.abs() >= 100.0 { format!("{v:.0}") } else if v.abs() >= 10.0 { format!("{v:.1}") } else { format!("{v:.2}") };
    let n = if n.contains('.') { n.trim_end_matches('0').trim_end_matches('.').to_string() } else { n };
    if unit.is_empty() { n } else { format!("{n} {unit}") }
}

// ───────────────────────────── Segmented ─────────────────────────────

/// One choice among a few short options, as joined buttons: `[15m] 1h 24h 7d`.
/// For modes and ranges (time range, view mode). Many or long options → `Select`.
pub struct Segmented<'a> {
    pub options: &'a [&'a str],
    pub active: usize,
    pub focused: bool,
}

impl Segmented<'_> {
    pub fn width(&self) -> u16 {
        self.options.iter().map(|o| Button::new(o).compact(true).width() + 1).sum::<u16>().saturating_sub(1)
    }
}

impl Widget for Segmented<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut x = area.x;
        for (i, o) in self.options.iter().enumerate() {
            let b = Button::new(o).compact(true);
            let b = if i == self.active { b.primary().focused(self.focused) } else { b };
            let w = b.width();
            if x + w > area.right() { break; }
            b.render(Rect::new(x, area.y, w, 1), buf);
            x += w + 1;
        }
    }
}

// ───────────────────────────── Servers ─────────────────────────────

/// `mem  ███████▒▒▒  412 / 512 MiB`: usage of a limit. Colors come from `UsageBar` (80 % yellow, 100 % red);
/// past 80 % the value also gets the status icon, so it's not color alone.
pub struct Meter<'a> {
    pub label: &'a str,
    pub label_width: u16,
    pub fraction: f64,
    /// Already formatted, e.g. `fmt::bytes_of(used, limit)` or `"42 %"`.
    pub value: &'a str,
    pub value_width: u16,
}

impl Widget for Meter<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area = Rect { height: 1, ..area };
        let bar = atoms::UsageBar(self.fraction);
        let lw = if self.label.is_empty() { 0 } else { self.label_width };
        let [l, b, v] = Layout::horizontal([Constraint::Length(lw), Constraint::Min(L::BAR_MIN_COLS), Constraint::Length(self.value_width + L::FIELD_ICON)]).spacing(1).areas(area);
        Line::from(atoms::label(self.label, Tone::Muted)).render(l, buf);
        bar.render(b, buf);
        let mut spans = vec![];
        if let Some(st) = bar.status() { spans.extend([atoms::status_icon(st), Span::raw(" ")]); }
        spans.push(atoms::label(self.value, Tone::Text));
        Line::from(spans).alignment(ratatui::layout::Alignment::Right).render(v, buf);
    }
}

/// `8080 → 80/tcp`. Published on all interfaces (`public`) gets a `!`: it's reachable from outside.
#[derive(Clone, Copy, Debug)]
pub struct PortMap<'a> {
    pub host: Option<u16>,
    pub container: u16,
    pub proto: &'a str,
    pub public: bool,
}

impl PortMap<'_> {
    pub fn spans(&self) -> Vec<Span<'static>> {
        let mut v = vec![];
        if self.public { v.extend([atoms::status_icon(Status::Warn), Span::raw(" ")]); }
        if let Some(h) = self.host { v.extend([atoms::label(&h.to_string(), Tone::Text), atoms::label(" → ", Tone::Faint)]); }
        v.push(atoms::label(&format!("{}/{}", self.container, self.proto), Tone::Muted));
        v
    }
    pub fn width(&self) -> u16 { Line::from(self.spans()).width() as u16 }
}

/// One step of a pipeline (build, push, deploy, health check).
#[derive(Clone, Copy, Debug)]
pub struct Step<'a> {
    pub name: &'a str,
    pub state: atoms::StepState,
    /// `"12s"`, or "" while waiting.
    pub took: &'a str,
}

/// Steps of a job. Vertical: one row per step with its time. Inline: `✓ build › ⠋ deploy › ○ check` (headers, rows).
pub struct StepList<'a> {
    pub steps: &'a [Step<'a>],
    pub inline: bool,
    pub tick: usize,
}

impl StepList<'_> {
    pub fn to_line(&self) -> Line<'static> {
        let mut v = vec![];
        for (i, st) in self.steps.iter().enumerate() {
            if i > 0 { v.push(atoms::label(" › ", Tone::Faint)); }
            let tone = if st.state == atoms::StepState::Waiting || st.state == atoms::StepState::Skipped { Tone::Faint } else { Tone::Text };
            v.extend([atoms::step_icon(st.state, self.tick), Span::raw(" "), atoms::label(st.name, tone)]);
        }
        Line::from(v)
    }
}

impl Widget for StepList<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.inline {
            self.to_line().render(Rect { height: 1, ..area }, buf);
            return;
        }
        for (i, st) in self.steps.iter().enumerate().take(area.height as usize) {
            let r = Rect { y: area.y + i as u16, height: 1, ..area };
            let tone = match st.state { atoms::StepState::Waiting | atoms::StepState::Skipped => Tone::Faint, atoms::StepState::Running => Tone::Accent, _ => Tone::Text };
            Line::from(vec![atoms::step_icon(st.state, self.tick), Span::raw(" "), atoms::label(st.name, tone)]).render(r, buf);
            Line::from(atoms::label(st.took, Tone::Faint)).alignment(ratatui::layout::Alignment::Right).render(r, buf);
        }
    }
}

/// Key = value rows (environment variables, labels). Secret values are masked (`SecretField`) unless `revealed`.
pub struct KeyValueRows<'a> {
    /// (key, value, secret)
    pub rows: &'a [(&'a str, &'a str, bool)],
    pub key_width: u16,
    pub revealed: bool,
}

impl Widget for KeyValueRows<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        for (i, (k, v, secret)) in self.rows.iter().enumerate().take(area.height as usize) {
            let r = Rect { y: area.y + i as u16, height: 1, ..area };
            let [kr, vr] = Layout::horizontal([Constraint::Length(self.key_width), Constraint::Min(0)]).spacing(1).areas(r);
            Line::from(atoms::label(&truncate(k, kr.width as usize), Tone::Muted)).render(kr, buf);
            if *secret {
                atoms::SecretField::new(v).revealed(self.revealed).render(vr, buf);
            } else {
                Line::from(atoms::label(&truncate(v, vr.width as usize), Tone::Text)).render(vr, buf);
            }
        }
    }
}

/// How fresh live data is: `updated 3s ago` (muted), or `! stale 45s` (yellow) once older than `stale_after`
/// (normally `consts::rules::STALE_AFTER_SECS`).
pub fn freshness(age_secs: u64, stale_after: u64) -> Span<'static> {
    if age_secs > stale_after {
        atoms::status_label(&format!("stale {age_secs}s"), Status::Warn)
    } else {
        atoms::label(&format!("updated {}", crate::fmt::age(age_secs)), Tone::Muted)
    }
}

// ───────────────────────────── Game design ─────────────────────────────

/// A curve to edit: y labels left (max / mid / min), the curve, x labels under it, then a readout of the
/// selected key. Moving keys is the app's job (`nudge_point` keeps them in order and in range).
pub struct CurveEditor<'a> {
    pub points: &'a [(f64, f64)],
    pub x: [f64; 2],
    pub y: [f64; 2],
    pub x_unit: &'a str,
    pub y_unit: &'a str,
    pub selected: Option<usize>,
}

impl Widget for CurveEditor<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [plot_row, xl, read] = Layout::vertical([Constraint::Min(3), Constraint::Length(1), Constraint::Length(1)]).areas(area);
        let [yl, plot] = Layout::horizontal([Constraint::Length(L::CURVE_YLABEL_COLS), Constraint::Min(0)]).spacing(1).areas(plot_row);
        let ylab = |v: f64| fmt_num(v, self.y_unit);
        let mid = (self.y[0] + self.y[1]) / 2.0;
        for (row, v) in [(yl.y, self.y[1]), (yl.y + yl.height / 2, mid), (yl.bottom().saturating_sub(1), self.y[0])] {
            Line::from(atoms::label(&ylab(v), Tone::Faint)).alignment(ratatui::layout::Alignment::Right).render(Rect { y: row, height: 1, ..yl }, buf);
        }
        atoms::Curve { points: self.points, x: self.x, y: self.y, selected: self.selected }.render(plot, buf);
        let xr = Rect { x: plot.x, width: plot.width, ..xl };
        Line::from(atoms::label(&fmt_num(self.x[0], self.x_unit), Tone::Faint)).render(xr, buf);
        Line::from(atoms::label(&fmt_num(self.x[1], self.x_unit), Tone::Faint)).alignment(ratatui::layout::Alignment::Right).render(xr, buf);
        if let Some((i, (x, y))) = self.selected.and_then(|i| self.points.get(i).map(|p| (i, p))) {
            Line::from(vec![
                atoms::label(&format!("{} key {}/{}", G::CURVE_SELECTED, i + 1, self.points.len()), Tone::Accent),
                atoms::label(&format!("   x {}   y {}", fmt_num(*x, self.x_unit), fmt_num(*y, self.y_unit)), Tone::Muted),
            ]).render(Rect { x: plot.x, width: plot.width, ..read }, buf);
        }
    }
}

/// Moves key `i` by (dx, dy), keeping it between its neighbours on x (keys stay sorted) and inside the ranges.
/// The first and last keys keep their x (the curve always spans the whole range).
pub fn nudge_point(points: &mut [(f64, f64)], i: usize, dx: f64, dy: f64, x: [f64; 2], y: [f64; 2]) {
    let n = points.len();
    if i >= n { return; }
    let lo = if i == 0 { x[0] } else { points[i - 1].0 };
    let hi = if i + 1 == n { x[1] } else { points[i + 1].0 };
    if i != 0 && i + 1 != n { points[i].0 = (points[i].0 + dx).clamp(lo, hi); }
    points[i].1 = (points[i].1 + dy).clamp(y[0], y[1]);
}

/// Shares of each weight in the total (0..1). All zero → all zero.
pub fn shares(weights: &[f64]) -> Vec<f64> {
    let total: f64 = weights.iter().sum();
    weights.iter().map(|w| if total > 0.0 { w / total } else { 0.0 }).collect()
}

/// Weighted choices (loot, spawn tables): raw weights, shown as % of the total (so it always sums to 100 %)
/// and as odds `1 in N`. The selected row's weight is the one the keys change.
pub struct WeightsEditor<'a> {
    pub items: &'a [(&'a str, f64)],
    pub selected: usize,
    pub focused: bool,
}

impl Widget for WeightsEditor<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let w: Vec<f64> = self.items.iter().map(|i| i.1).collect();
        let sh = shares(&w);
        let top = sh.iter().cloned().fold(0.0, f64::max).max(f64::EPSILON);
        let lw = self.items.iter().map(|(l, _)| Span::raw(*l).width()).max().unwrap_or(0).min(L::CHART_LABEL_MAX) as u16 + L::ROW_INDENT;
        let n = self.items.len() as u16;
        let [rows, _, total] = Layout::vertical([Constraint::Length(n), Constraint::Length(1), Constraint::Length(1)]).areas(area);
        for (i, ((label, raw), s)) in self.items.iter().zip(&sh).enumerate().take(rows.height as usize) {
            let r = Rect { y: rows.y + i as u16, height: 1, ..rows };
            let sel = i == self.selected;
            if sel { (&ListRow { selected: true, list_focused: self.focused, ..Default::default() }).render(r, buf); }
            let [l, b, raw_c, p, o] = Layout::horizontal([
                Constraint::Length(lw), Constraint::Min(L::BAR_MIN_COLS), Constraint::Length(L::WEIGHT_PCT_COLS - 1),
                Constraint::Length(L::WEIGHT_PCT_COLS), Constraint::Length(L::WEIGHT_ODDS_COLS),
            ]).spacing(1).areas(r);
            Line::from(atoms::label(&truncate(label, (l.width - L::ROW_INDENT) as usize), if sel { Tone::Title } else { Tone::Text })).render(Rect { x: l.x + L::ROW_INDENT, width: l.width - L::ROW_INDENT, ..l }, buf);
            atoms::Bar { fraction: s / top, over: false }.render(b, buf);
            Line::from(atoms::label(&fmt_num(*raw, ""), Tone::Faint)).alignment(ratatui::layout::Alignment::Right).render(raw_c, buf);
            Line::from(atoms::label(&format!("{:.1} %", s * 100.0), Tone::Text)).alignment(ratatui::layout::Alignment::Right).render(p, buf);
            let odds = if *s > 0.0 { format!("1 in {}", fmt_num(1.0 / s, "")) } else { "never".into() };
            Line::from(atoms::label(&odds, Tone::Muted)).alignment(ratatui::layout::Alignment::Right).render(o, buf);
        }
        let sum: f64 = w.iter().sum();
        Line::from(vec![
            atoms::label(&format!("{} entries · weights {} · ", self.items.len(), fmt_num(sum, "")), Tone::Faint),
            atoms::status_label(if sum > 0.0 { "100 %" } else { "nothing can drop" }, if sum > 0.0 { Status::Ok } else { Status::Warn }),
        ]).render(total, buf);
    }
}

/// One clip: start and length in the timeline's unit.
#[derive(Clone, Copy, Debug)]
pub struct ClipSpec<'a> {
    pub start: f64,
    pub len: f64,
    pub label: &'a str,
}

/// One track: a name and its clips.
#[derive(Clone, Copy, Debug)]
pub struct Track<'a> {
    pub name: &'a str,
    pub clips: &'a [ClipSpec<'a>],
}

/// Tracks of clips over time (cutscenes, animation events, waves): ruler on top, one row per track,
/// the playhead across all. `span` = visible time range; clips outside are cut at the edges.
pub struct Timeline<'a> {
    pub tracks: &'a [Track<'a>],
    pub span: [f64; 2],
    pub playhead: f64,
    pub unit: &'a str,
    /// (track, clip)
    pub selected: Option<(usize, usize)>,
}

impl Timeline<'_> {
    /// A "nice" ruler step (1, 2, 5 × 10ⁿ) so labels are about RULER_TICK_COLS cells apart.
    pub fn ruler_step(span: f64, cols: u16) -> f64 {
        let raw = span / (cols as f64 / L::RULER_TICK_COLS).max(1.0);
        let p = 10f64.powf(raw.log10().floor());
        [1.0, 2.0, 5.0, 10.0].iter().map(|m| m * p).find(|s| *s >= raw).unwrap_or(10.0 * p)
    }
}

impl Widget for Timeline<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let nw = self.tracks.iter().map(|t| Span::raw(t.name).width()).max().unwrap_or(0).min(L::TRACK_NAME_MAX as usize) as u16;
        let [names, lane] = Layout::horizontal([Constraint::Length(nw), Constraint::Min(0)]).spacing(1).areas(area);
        if lane.width < 2 || self.span[1] <= self.span[0] { return; }
        let span = self.span[1] - self.span[0];
        let col = |t: f64| -> i32 { ((t - self.span[0]) / span * (lane.width - 1) as f64).round() as i32 };
        // ruler
        let step = Self::ruler_step(span, lane.width);
        let mut t = (self.span[0] / step).ceil() * step;
        while t <= self.span[1] + f64::EPSILON {
            let x = lane.x + col(t) as u16;
            let lab = fmt_num(t, self.unit);
            buf.set_string(x, lane.y, G::RULER_TICK, q::faint());
            if x + 1 + lab.len() as u16 <= lane.right() { buf.set_string(x + 1, lane.y, &lab, q::faint()); }
            t += step;
        }
        // tracks: one row each, a blank row between
        for (ti, tr) in self.tracks.iter().enumerate() {
            let y = lane.y + 2 + 2 * ti as u16;
            if y >= area.bottom() { break; }
            Line::from(atoms::label(&truncate(tr.name, nw as usize), Tone::Muted)).render(Rect { y, height: 1, ..names }, buf);
            for (ci, c) in tr.clips.iter().enumerate() {
                let (a, b) = (col(c.start).max(0), col(c.start + c.len).min(lane.width as i32 - 1));
                if b < 0 || a >= lane.width as i32 { continue; }
                let w = (b - a).max(1) as u16;
                atoms::Clip { label: c.label, selected: self.selected == Some((ti, ci)) }.render(Rect { x: lane.x + a as u16, y, width: w, height: 1 }, buf);
            }
        }
        // playhead over everything
        if (self.span[0]..=self.span[1]).contains(&self.playhead) {
            let x = lane.x + col(self.playhead) as u16;
            let ph = atoms::playhead(true);
            buf.set_string(x, lane.y, ph.content.as_ref(), ph.style);
            let line = atoms::playhead(false);
            for y in lane.y + 1..area.bottom() {
                buf[(x, y)].set_symbol(&line.content).set_fg(line.style.fg.unwrap_or_default());
            }
        }
    }
}

// ───────────────────────────── Node graph ─────────────────────────────

/// One node: a title, its input and output port names.
#[derive(Clone, Copy, Debug)]
pub struct NodeSpec<'a> {
    pub title: &'a str,
    pub inputs: &'a [&'a str],
    pub outputs: &'a [&'a str],
}

impl NodeSpec<'_> {
    /// Outer size in cells (border included).
    pub fn size(&self) -> (u16, u16) {
        let w = |v: &[&str]| v.iter().map(|s| Span::raw(*s).width()).max().unwrap_or(0) as u16;
        let ports = w(self.inputs) + w(self.outputs) + L::NODE_PORT_GAP + 4;
        let title = Span::raw(self.title).width() as u16 + 4;
        (ports.max(title).max(L::NODE_MIN_COLS), self.inputs.len().max(self.outputs.len()).max(1) as u16 + 2)
    }
}

/// A wire: (node, output port) → (node, input port).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Wire {
    pub from: (usize, usize),
    pub to: (usize, usize),
}

/// Cells of a wire from `a` (just right of an output) to `b` (just left of an input), as (x, y, glyph):
/// across to the middle, up or down, across again; rounded corners.
pub fn route(a: (i32, i32), b: (i32, i32)) -> Vec<(i32, i32, &'static str)> {
    let mut out = Vec::new();
    let mid = (a.0 + b.0) / 2;
    let hline = |out: &mut Vec<_>, y: i32, x1: i32, x2: i32| { for x in x1.min(x2)..=x1.max(x2) { out.push((x, y, "─")); } };
    if a.1 == b.1 { hline(&mut out, a.1, a.0, b.0); return out; }
    let down = b.1 > a.1;
    hline(&mut out, a.1, a.0, mid);
    hline(&mut out, b.1, mid, b.0);
    for y in a.1.min(b.1) + 1..a.1.max(b.1) { out.push((mid, y, "│")); }
    // corner 1: joins the first line (from a's side) and the vertical (towards b)
    let from_left = a.0 <= mid;
    let c1 = match (from_left, down) { (true, true) => "╮", (true, false) => "╯", (false, true) => "╭", (false, false) => "╰" };
    // corner 2: joins the vertical (from a's side) and the last line (towards b)
    let to_right = b.0 >= mid;
    let c2 = match (to_right, down) { (true, true) => "╰", (true, false) => "╭", (false, true) => "╯", (false, false) => "╮" };
    out.retain(|c| !((c.0, c.1) == (mid, a.1) || (c.0, c.1) == (mid, b.1)));
    out.push((mid, a.1, c1));
    out.push((mid, b.1, c2));
    out
}

/// Nodes and the wires between them (abilities, dialogue, shaders, AI). Positions are cells from the
/// graph's top-left, minus `pan`. Wires are drawn first, nodes on top; the selected node and its wires are Quag.
pub struct NodeGraph<'a> {
    pub nodes: &'a [NodeSpec<'a>],
    pub positions: &'a [(i32, i32)],
    pub wires: &'a [Wire],
    pub selected: Option<usize>,
    pub pan: (i32, i32),
}

impl NodeGraph<'_> {
    /// Where node `i`'s output / input port `p` is (the cell on its border).
    pub fn port_cell(&self, i: usize, p: usize, output: bool) -> (i32, i32) {
        let (x, y) = self.positions[i];
        let (w, _) = self.nodes[i].size();
        (x + if output { w as i32 - 1 } else { 0 }, y + 1 + p as i32)
    }
}

impl Widget for NodeGraph<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let to_screen = |(x, y): (i32, i32)| -> Option<(u16, u16)> {
            let (sx, sy) = (area.x as i32 + x - self.pan.0, area.y as i32 + y - self.pan.1);
            (sx >= area.x as i32 && sy >= area.y as i32 && sx < area.right() as i32 && sy < area.bottom() as i32).then_some((sx as u16, sy as u16))
        };
        // wires
        for w in self.wires {
            let active = self.selected.is_some_and(|s| s == w.from.0 || s == w.to.0);
            let (a, b) = (self.port_cell(w.from.0, w.from.1, true), self.port_cell(w.to.0, w.to.1, false));
            for (x, y, g) in route((a.0 + 1, a.1), (b.0 - 1, b.1)) {
                if let Some((sx, sy)) = to_screen((x, y)) { buf.set_string(sx, sy, g, atoms::wire(active)); }
            }
        }
        // nodes: each drawn whole in its own buffer, then only the cells inside `area` are copied,
        // so a node half out of view is cut at the edge (and its wires still end at it)
        for (i, n) in self.nodes.iter().enumerate() {
            let sel = self.selected == Some(i);
            let (w, h) = n.size();
            let local = Rect::new(0, 0, w, h);
            let mut nb = Buffer::empty(local);
            let block = atoms::node_frame(n.title, sel);
            let inner = block.inner(local);
            block.render(local, &mut nb);
            let wired = |p: usize, out: bool| self.wires.iter().any(|wr| if out { wr.from == (i, p) } else { wr.to == (i, p) });
            for (p, name) in n.inputs.iter().enumerate().take(inner.height as usize) {
                let row = inner.y + p as u16;
                let pt = atoms::port(wired(p, false), sel);
                nb.set_string(0, row, pt.content.as_ref(), pt.style);
                Line::from(atoms::label(name, Tone::Muted)).render(Rect { x: inner.x + 1, y: row, width: inner.width.saturating_sub(1), height: 1 }, &mut nb);
            }
            for (p, name) in n.outputs.iter().enumerate().take(inner.height as usize) {
                let row = inner.y + p as u16;
                let pt = atoms::port(wired(p, true), sel);
                nb.set_string(w - 1, row, pt.content.as_ref(), pt.style);
                Line::from(atoms::label(name, Tone::Muted)).alignment(ratatui::layout::Alignment::Right).render(Rect { x: inner.x, y: row, width: inner.width.saturating_sub(1), height: 1 }, &mut nb);
            }
            let (x, y) = self.positions[i];
            for cy in 0..h {
                for cx in 0..w {
                    if let Some((sx, sy)) = to_screen((x + cx as i32, y + cy as i32)) { buf[(sx, sy)] = nb[(cx, cy)].clone(); }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::buffer::Buffer;

    fn text_at(buf: &Buffer, y: u16) -> String {
        (0..buf.area.width).map(|x| buf[(x, y)].symbol().to_string()).collect()
    }

    #[test]
    fn nudge_keeps_keys_in_order_and_range() {
        let mut p = vec![(0.0, 1.0), (0.5, 1.5), (1.0, 2.0)];
        nudge_point(&mut p, 1, 0.9, 5.0, [0.0, 1.0], [0.0, 3.0]);
        assert_eq!(p[1], (1.0, 3.0), "clamped to next key's x and to y max");
        nudge_point(&mut p, 0, 0.3, -9.0, [0.0, 1.0], [0.0, 3.0]);
        assert_eq!(p[0], (0.0, 0.0), "first key keeps its x");
    }

    #[test]
    fn wires_turn_with_round_corners() {
        let r = route((0, 0), (6, 3));
        assert!(r.contains(&(3, 0, "╮")) && r.contains(&(3, 3, "╰")), "{r:?}");
        assert!(r.contains(&(0, 0, "─")) && r.contains(&(6, 3, "─")) && r.contains(&(3, 1, "│")));
        let up = route((0, 3), (6, 0));
        assert!(up.contains(&(3, 3, "╯")) && up.contains(&(3, 0, "╭")));
        assert_eq!(route((0, 2), (4, 2)).len(), 5, "same row: a straight line");
    }

    #[test]
    fn graph_cuts_nodes_at_the_edge_instead_of_hiding_them() {
        let nodes = [NodeSpec { title: "A", inputs: &[], outputs: &["out"] }, NodeSpec { title: "B", inputs: &["in"], outputs: &[] }];
        let at = [(0, 0), (20, 4)];
        let wires = [Wire { from: (0, 0), to: (1, 0) }];
        let area = Rect::new(0, 0, 40, 6); // B is 3 rows tall from y 4: only its top 2 rows fit
        let mut buf = Buffer::empty(area);
        NodeGraph { nodes: &nodes, positions: &at, wires: &wires, selected: None, pan: (0, 0) }.render(area, &mut buf);
        assert_eq!(buf[(20, 4)].symbol(), "╭", "B's top-left corner is drawn although B doesn't fit");
        assert_eq!(buf[(20, 5)].symbol(), crate::consts::glyph::PORT, "the wire ends at B's port");
    }

    #[test]
    fn shares_always_total_one() {
        let s = shares(&[50.0, 30.0, 15.0, 5.0]);
        assert!((s.iter().sum::<f64>() - 1.0).abs() < 1e-9);
        assert_eq!(shares(&[0.0, 0.0]), vec![0.0, 0.0]);
    }

    #[test]
    fn ruler_steps_are_nice() {
        assert_eq!(Timeline::ruler_step(60.0, 100), 10.0);
        assert_eq!(Timeline::ruler_step(12.0, 120), 1.0);
        assert_eq!(Timeline::ruler_step(30.0, 40), 10.0);
    }

    #[test]
    fn field_never_overlaps_when_squeezed() {
        for h in 1..=3u16 {
            let mut buf = Buffer::empty(Rect::new(0, 0, 30, h));
            Field::new("Label", Input::new("value")).help("help").render(buf.area, &mut buf);
            let all: String = (0..h).map(|y| text_at(&buf, y)).collect();
            assert!(all.contains("value"), "input must always show (h={h})");
            if h < 3 { assert!(!all.contains("help"), "help must drop first (h={h})"); }
        }
    }

    #[test]
    fn button_group_never_drops_a_button() {
        let group = || ButtonGroup::new(vec![Button::new("Mark done").primary(), Button::new("Edit"), Button::new("Delete").danger()]);
        for w in [40u16, 26, 20] {
            let mut buf = Buffer::empty(Rect::new(0, 0, w, 3));
            group().render(buf.area, &mut buf);
            let all: String = (0..3).map(|y| text_at(&buf, y)).collect();
            for label in ["Mark done", "Edit", "Delete"] {
                assert!(all.contains(label), "{label} missing at width {w}");
            }
        }
    }

    #[test]
    fn key_hints_drop_whole_pairs() {
        // "q quit" = 6 cells; "q quit  ·  / search" = 19 cells → at 12 only the first fits
        let mut buf = Buffer::empty(Rect::new(0, 0, 12, 1));
        KeyHints(&[("q", "quit"), ("/", "search"), ("?", "help")]).render(buf.area, &mut buf);
        let t = text_at(&buf, 0);
        assert!(t.contains("q quit") && !t.contains("sear"), "got {t:?}");
    }
}
