//! The explorer's catalog: every part by level, each with a live preview.
//! When you add a part to the library or the app, add it here too.

use crate::{
    data::TASKS,
    elements::{
        FindingDetail, FindingsList, FirewallRules, Activity, Attention, EditorTools, FrameBudget, GpuMemory, MaterialParams, ProjectTree, ServerLoad, SyncSettings, GeneralSettings, NoteEditor, NotificationSettings, ProjectNav, ProjectTable, SyncLog, TaskActions,
        TaskInspector, TaskList, TaskProperties, WeekStats,
    },
    pages::{PAGES, PageState},
};
use quag_theme::{
    Status,
    atoms::{self, Severity, SecretField, Bar, Button, Checkbox, Divider, Histogram, Input, NumberField, Progress, Radio, Select, Skeleton, Slider, SplitHandle, Sparkline, Tone},
    elements::{TypeToConfirm, AppHeader, CommandPalette, ConfirmDialog, EmptyState, ErrorState, HelpOverlay, LogEntry, LogViewer, StatusBar, Toast},
    molecules::{
        Segmented, BarChart, Breadcrumbs, ButtonGroup, CheckList, Column, Field, KeyHints, ListRow, LogLine, Menu, MenuItem, ParamRow, PropertyList,
        RadioGroup, SearchBar, Stat, StatusLine, Table, Tabs, TreeView, flatten,
    },
};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    text::{Line, Span},
    widgets::Paragraph,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Level {
    Atom,
    Molecule,
    Element,
    Ui,
}

impl Level {
    pub const ALL: [Level; 4] = [Level::Atom, Level::Molecule, Level::Element, Level::Ui];
    pub fn title(self) -> &'static str {
        match self {
            Level::Atom => "1 · ATOMS",
            Level::Molecule => "2 · MOLECULES",
            Level::Element => "3 · ELEMENTS",
            Level::Ui => "4 · UI",
        }
    }
    pub fn rule(self) -> &'static str {
        match self {
            Level::Atom => "one control · only atoms set colors",
            Level::Molecule => "atoms arranged in a box · generic",
            Level::Element => "a molecule put to use · wired to data",
            Level::Ui => "a page placing elements",
        }
    }
}

pub type Preview = fn(&mut Frame, Rect, &mut PageState);

pub enum Kind {
    Part(Preview),
    /// Index into `pages::PAGES`.
    Page(usize),
}

pub struct Entry {
    pub id: &'static str,
    pub level: Level,
    pub name: &'static str,
    pub what: &'static str,
    pub from: &'static str,
    pub kind: Kind,
}

// ── preview helpers ──
fn rows(area: Rect, n: usize) -> Vec<Rect> {
    (0..n as u16).map(|i| Rect { y: area.y + i, height: 1, ..area }).filter(|r| r.y < area.bottom()).collect()
}
fn cap(f: &mut Frame, r: Rect, text: &str) {
    f.render_widget(Paragraph::new(Line::from(atoms::label(text, Tone::Faint))), r);
}
fn at(area: Rect, dy: u16, h: u16, w: u16) -> Rect {
    Rect { x: area.x, y: area.y + dy, width: w.min(area.width), height: h.min(area.height.saturating_sub(dy)) }
}
fn buttons_row(f: &mut Frame, r: Rect, bs: Vec<Button<'static>>) {
    let mut x = r.x;
    for b in bs {
        let w = b.width();
        if x + w > r.right() { break; }
        f.render_widget(b, Rect::new(x, r.y, w, 1));
        x += w + 2;
    }
}

// ── previews: ATOMS ──
fn p_button(f: &mut Frame, a: Rect, _: &mut PageState) {
    let r = rows(a, 8);
    buttons_row(f, r[0], vec![Button::new("Primary").primary(), Button::new("Secondary"), Button::new("Danger").danger(), Button::new("Ghost").ghost()]);
    cap(f, r[1], "kinds: primary (max one per screen) · secondary · danger · ghost");
    buttons_row(f, r[3], vec![Button::new("Focused").focused(true), Button::new("Focused").primary().focused(true), Button::new("Disabled").disabled(true)]);
    cap(f, r[4], "states: focused · focused primary · disabled");
    buttons_row(f, r[6], vec![Button::new("Compact").compact(true), Button::new("Compact").primary().compact(true)]);
    cap(f, r[7], "compact: molecules switch this on when space is tight");
}
fn p_checkbox(f: &mut Frame, a: Rect, _: &mut PageState) {
    let r = rows(a, 5);
    f.render_widget(Checkbox::new("Checked", true), r[0]);
    f.render_widget(Checkbox::new("Unchecked", false), r[1]);
    f.render_widget(Checkbox::new("Focused", false).focused(true), r[2]);
    f.render_widget(Checkbox::new("Disabled", true).disabled(true), r[3]);
}
fn p_input(f: &mut Frame, a: Rect, _: &mut PageState) {
    let r = rows(a, 8);
    let w = |r: Rect| Rect { width: r.width.min(36), ..r };
    f.render_widget(Input::new("").placeholder("Placeholder…"), w(r[0]));
    cap(f, r[1], "empty");
    f.render_widget(Input::new("Focused value").focused(true), w(r[2]));
    cap(f, r[3], "focused: accent bar + cursor");
    f.render_widget(Input::new("tomorow").invalid(true), w(r[4]));
    cap(f, r[5], "invalid: red bar + ✗ (Field adds the message)");
    f.render_widget(Input::new("Disabled").disabled(true), w(r[6]));
}
fn p_text(f: &mut Frame, a: Rect, _: &mut PageState) {
    let tones = [("◆ Title", Tone::Title), ("Heading", Tone::Heading), ("Body text", Tone::Text), ("Muted: secondary info", Tone::Muted), ("Faint: placeholders, disabled", Tone::Faint), ("Accent: the one thing", Tone::Accent)];
    for (r, (t, tone)) in rows(a, 6).into_iter().zip(tones) {
        f.render_widget(Paragraph::new(Line::from(atoms::label(t, tone))), r);
    }
}
fn p_small(f: &mut Frame, a: Rect, _: &mut PageState) {
    let r = rows(a, 7);
    f.render_widget(Paragraph::new(Line::from(vec![atoms::tag("docs"), Span::raw("   "), atoms::kbd("/"), Span::raw("  "), atoms::kbd("ctrl+s")])), r[0]);
    cap(f, r[1], "tag · kbd");
    f.render_widget(Paragraph::new(Line::from(vec![
        atoms::status_icon(Status::Ok), Span::raw("  "), atoms::status_icon(Status::Warn), Span::raw("  "),
        atoms::status_icon(Status::Err), Span::raw("  "), atoms::status_icon(Status::Info), Span::raw("     "), atoms::spinner(2),
    ])), r[3]);
    cap(f, r[4], "status icons ok · warn · err · info   spinner");
    f.render_widget(Progress(0.58), Rect { width: r[6].width.min(30), ..r[6] });
}
fn p_box(f: &mut Frame, a: Rect, _: &mut PageState) {
    let [l, _, m, _, rr] = Layout::horizontal([Constraint::Length(22), Constraint::Length(2), Constraint::Length(22), Constraint::Length(2), Constraint::Length(26)]).areas(at(a, 0, 6, a.width));
    f.render_widget(atoms::panel("Panel", false), l);
    f.render_widget(atoms::panel("Focused", true), m);
    f.render_widget(atoms::popup("Popup"), rr);
    f.render_widget(Divider, at(a, 7, 1, 72));
    cap(f, at(a, 8, 1, a.width), "panel · focused panel · popup · divider: the empty boxes");
}

// ── previews: MOLECULES ──
fn p_buttongroup(f: &mut Frame, a: Rect, _: &mut PageState) {
    f.render_widget(ButtonGroup::new(vec![Button::new("Button A").primary(), Button::new("Button B"), Button::new("Button C")]), at(a, 0, 1, a.width));
    cap(f, at(a, 1, 1, a.width), "an array of buttons");
    f.render_widget(ButtonGroup::new(vec![Button::new("Button A").primary(), Button::new("Button B"), Button::new("Button C")]).boxed("Actions"), at(a, 3, 3, 44));
    cap(f, at(a, 6, 1, a.width), "… in a box");
    f.render_widget(ButtonGroup::new(vec![Button::new("Mark done").primary(), Button::new("Edit"), Button::new("Delete").danger()]), at(a, 8, 2, 24));
    cap(f, at(a, 10, 1, a.width), "tight space: compact first, then wraps. Never drops a button");
}
fn p_field(f: &mut Frame, a: Rect, _: &mut PageState) {
    f.render_widget(Field::new("Name", Input::new("pond")).help("Shown in the title."), at(a, 0, 3, 40));
    f.render_widget(Field::new("Due", Input::new("tomorow").focused(true)).error("Can't read that date."), at(a, 4, 3, 40));
    cap(f, at(a, 8, 1, a.width), "label + Input + help · with error");
}
fn p_search(f: &mut Frame, a: Rect, _: &mut PageState) {
    f.render_widget(SearchBar { query: "", focused: false }, at(a, 0, 1, 40));
    f.render_widget(SearchBar { query: "readme", focused: true }, at(a, 2, 1, 40));
    cap(f, at(a, 3, 1, a.width), "kbd + Input · idle and focused");
}
fn p_tabs(f: &mut Frame, a: Rect, _: &mut PageState) {
    f.render_widget(Tabs { names: &["Today", "Upcoming", "Done"], active: 0 }, at(a, 0, 1, a.width));
    f.render_widget(KeyHints(&[("↑↓", "move"), ("/", "search"), ("q", "quit")]), at(a, 2, 1, a.width));
    f.render_widget(StatusLine { kind: Status::Ok, message: "Saved 2s ago" }, at(a, 4, 1, a.width));
    cap(f, at(a, 6, 1, a.width), "Tabs · KeyHints (drops whole hints when narrow) · StatusLine");
}
fn p_listrow(f: &mut Frame, a: Rect, _: &mut PageState) {
    let w = a.width.min(64);
    let row = |title, check, tags: &'static [&'static str], status, meta, selected| ListRow { title, check: Some(check), tags, status, meta, selected, list_focused: true, status_slot: true, meta_width: 8, meta_accent: meta == "today", lead: None };
    let rs = [
        row("Open row", false, &["docs"], None, "today", false),
        row("Selected row", false, &["ui"], Some(Status::Warn), "fri", true),
        row("Done row", true, &["ui"], Some(Status::Ok), "done", false),
        row("A very long title that will not fit in the space it has and gets cut", false, &["bug"], Some(Status::Err), "sat", false),
    ];
    for (i, r) in rs.iter().enumerate() {
        f.render_widget(r, at(a, i as u16, 1, w));
    }
    cap(f, at(a, 5, 1, a.width), "check + title (… when long) · tags · status · meta right-aligned");
}
fn p_stat(f: &mut Frame, a: Rect, _: &mut PageState) {
    f.render_widget(Stat { value: "12", label: "open tasks" }, at(a, 0, 4, 20));
    f.render_widget(CheckList { items: &[("Remind me", true), ("Daily summary", false), ("Sounds", true)], focused: Some(1) }, at(a, 5, 3, 30));
    f.render_widget(LogLine { time: "12:01:12", kind: Status::Err, message: "Upload failed: timeout" }, at(a, 9, 1, a.width));
    cap(f, at(a, 11, 1, a.width), "Stat · CheckList · LogLine");
}

// ── previews: ELEMENTS ──
fn p_taskactions(f: &mut Frame, a: Rect, _: &mut PageState) {
    cap(f, at(a, 0, 1, a.width), "open task:");
    f.render_widget(TaskActions { task: &TASKS[1], focused: None }, at(a, 1, 1, a.width));
    cap(f, at(a, 3, 1, a.width), "done task: same ButtonGroup, the element picks “Reopen” from the data");
    f.render_widget(TaskActions { task: &TASKS[3], focused: Some(0) }, at(a, 4, 1, a.width));
}
fn p_tasklist(f: &mut Frame, a: Rect, _: &mut PageState) {
    let tasks: Vec<_> = TASKS.iter().take(4).collect();
    let [l, _, r] = Layout::horizontal([Constraint::Percentage(62), Constraint::Length(2), Constraint::Min(0)]).areas(at(a, 0, 9, a.width));
    f.render_widget(TaskList { title: "ideal", tasks: &tasks, selected: 1, focused: true, query: "" }, l);
    f.render_widget(TaskList { title: "empty", tasks: &[], selected: 0, focused: false, query: "" }, r);
}
fn p_inspector(f: &mut Frame, a: Rect, _: &mut PageState) {
    let [l, _, r] = Layout::horizontal([Constraint::Length(42), Constraint::Length(2), Constraint::Length(30)]).areas(at(a, 0, 14, a.width));
    f.render_widget(TaskInspector { task: Some(&TASKS[2]), focused: true, button: Some(0) }, l);
    f.render_widget(TaskInspector { task: None, focused: false, button: None }, r);
}
fn p_projectnav(f: &mut Frame, a: Rect, _: &mut PageState) {
    f.render_widget(ProjectNav { tasks: TASKS, selected: 0, focused: true, week: (7, 12) }, at(a, 0, 12, 28));
}
fn p_dash(f: &mut Frame, a: Rect, _: &mut PageState) {
    f.render_widget(WeekStats, at(a, 0, 4, a.width));
    f.render_widget(Attention, at(a, 5, 5, a.width.min(60)));
    f.render_widget(crate::elements::Recent, at(a, 11, 5, a.width.min(60)));
}
fn p_settings(f: &mut Frame, a: Rect, _: &mut PageState) {
    f.render_widget(GeneralSettings { focused: Some(0) }, at(a, 0, 10, 48));
    f.render_widget(NotificationSettings { focused: Some(2) }, at(a, 11, 5, 48));
}
fn p_synclog(f: &mut Frame, a: Rect, _: &mut PageState) {
    f.render_widget(SyncLog, at(a, 0, 9, a.width.min(64)));
}
fn p_header(f: &mut Frame, a: Rect, _: &mut PageState) {
    f.render_widget(AppHeader { app: "pond", location: "tasks", tabs: Some(Tabs { names: &["Today", "Done"], active: 0 }), env: None }, at(a, 0, 2, a.width));
    cap(f, at(a, 2, 1, a.width), "AppHeader");
    f.render_widget(StatusBar { hints: KeyHints(&[("↑↓", "move"), ("q", "quit")]), status: Some(StatusLine { kind: Status::Ok, message: "Saved" }) }, at(a, 4, 1, a.width));
    cap(f, at(a, 5, 1, a.width), "StatusBar");
}
fn p_empty(f: &mut Frame, a: Rect, st: &mut PageState) {
    let area = at(a, 0, a.height.min(20), a.width.min(60));
    let block = atoms::panel("", false);
    let inner = block.inner(area);
    f.render_widget(block, area);
    let [art, text] = EmptyState::layout(inner, cfg!(feature = "mascot"));
    #[cfg(feature = "mascot")]
    st.quag.render(f, art);
    let _ = (art, &st);
    f.render_widget(EmptyState { message: "Nothing here yet.", hint: Some(("N", "to add a task")) }, text);
}
fn p_confirm(f: &mut Frame, a: Rect, _: &mut PageState) {
    let d = ConfirmDialog { title: "Delete task", question: "Delete this task?", detail: "“Write README” can't be restored.", confirm: "Delete", cancel: "Keep", danger: true, focused: 1 };
    let r = d.area(at(a, 0, 10, a.width.min(64)));
    f.render_widget(d, r);
}

// ── previews: added parts ──
fn p_radio(f: &mut Frame, a: Rect, _: &mut PageState) {
    let r = rows(a, 5);
    f.render_widget(Radio::new("Selected", true), r[0]);
    f.render_widget(Radio::new("Not selected", false), r[1]);
    f.render_widget(Radio::new("Focused", false).focused(true), r[2]);
    f.render_widget(Radio::new("Disabled", false).disabled(true), r[3]);
}
fn p_badge(f: &mut Frame, a: Rect, _: &mut PageState) {
    let r = rows(a, 8);
    f.render_widget(Paragraph::new(Line::from(vec![atoms::label("Inbox ", Tone::Text), atoms::badge(3), Span::raw("     "), atoms::label("Mentions ", Tone::Text), atoms::badge(128)])), r[0]);
    cap(f, r[1], "badge: a count · 0 shows nothing · over 99 shows 99+");
    f.render_widget(Paragraph::new(Line::from(vec![atoms::label("See ", Tone::Muted), atoms::link("STRUCTURE.md"), atoms::label(" for the rules.", Tone::Muted)])), r[3]);
    cap(f, r[4], "link");
    f.render_widget(Paragraph::new(Line::from(vec![atoms::status_label("Upload failed", Status::Err), Span::raw("   "), atoms::status_label("Saved", Status::Ok)])), r[6]);
    cap(f, r[7], "status_label: text in a status color");
}
/// The two progress bars, live: they run on the app's clock (`st.tick`), like the spinner.
fn p_progressbars(f: &mut Frame, a: Rect, st: &mut PageState) {
    use quag_theme::atoms::{SmoothProgress, StepProgress};
    let w = a.width.min(52);
    let bar_w = w.saturating_sub(8);
    // smooth: 0 → 100 % in 8 s, then again
    let frac = (st.tick % 100) as f64 / 100.0;
    f.render_widget(Line::from(atoms::label("Downloading model.bin", Tone::Text)), at(a, 0, 1, w));
    f.render_widget(SmoothProgress(frac), at(a, 1, 1, bar_w));
    f.render_widget(Line::from(atoms::label(&format!("{:>3.0} %", frac * 100.0), Tone::Muted)), Rect { x: a.x + bar_w + 2, ..at(a, 1, 1, 6) });
    // steps: 5 steps of 1.2 s each, the current one filling
    const STEPS: [&str; 5] = ["Pull", "Build image", "Migrate", "Health check", "Switch traffic"];
    let per = 15;
    let t = st.tick % (per * (STEPS.len() + 1));
    let (done, current) = ((t / per).min(STEPS.len()), (t % per) as f64 / per as f64);
    let label = if done == STEPS.len() { "Done · 5 of 5".to_string() } else { format!("Step {} of 5 · {}", done + 1, STEPS[done]) };
    f.render_widget(Line::from(atoms::label(&label, Tone::Text)), at(a, 3, 1, w));
    f.render_widget(StepProgress { total: STEPS.len(), done, current, failed: false }, at(a, 4, 1, bar_w));
    f.render_widget(Line::from(atoms::label("Step 4 of 5 · Health check failed", Tone::Text)), at(a, 6, 1, w));
    f.render_widget(StepProgress { total: STEPS.len(), done: 3, current: 0.6, failed: true }, at(a, 7, 1, bar_w));
    f.render_widget(Line::from(vec![atoms::spinner(st.tick), atoms::label(" Loading…", Tone::Muted)]), at(a, 9, 1, w));
    cap(f, at(a, 11, 1, a.width), "smooth: eighths of a cell · steps: one segment per step · the spinner turns once per 0.8 s");
}
fn p_skeleton(f: &mut Frame, a: Rect, _: &mut PageState) {
    let w = a.width.min(48);
    for (i, fill) in [0.6, 0.9, 0.75, 0.4].into_iter().enumerate() {
        f.render_widget(Skeleton(fill), at(a, i as u16, 1, w));
    }
    cap(f, at(a, 5, 1, a.width), "loading placeholder: keeps the layout while data comes (never a blank zone)");
}
fn p_radiogroup(f: &mut Frame, a: Rect, _: &mut PageState) {
    let opts: &[&str] = &["Low", "Normal", "High"];
    f.render_widget(RadioGroup { options: opts, selected: 1, focused: Some(1) }, at(a, 0, 1, a.width));
    cap(f, at(a, 1, 1, a.width), "a row when it fits…");
    f.render_widget(RadioGroup { options: opts, selected: 2, focused: None }, at(a, 3, 3, 20));
    cap(f, at(a, 6, 1, a.width), "…a column when it doesn't");
}
fn p_breadcrumbs(f: &mut Frame, a: Rect, _: &mut PageState) {
    let path: &[&str] = &["pond", "quag-theme", "Write README", "README.md"];
    f.render_widget(Breadcrumbs(path), at(a, 0, 1, a.width));
    f.render_widget(Breadcrumbs(path), at(a, 2, 1, 32));
    cap(f, at(a, 3, 1, a.width), "too narrow: the middle collapses to …, first and last stay");
}
const MENU: &[MenuItem] = &[
    MenuItem { label: "Open", key: "enter", danger: false },
    MenuItem { label: "Rename", key: "r", danger: false },
    MenuItem { label: "Duplicate", key: "ctrl+d", danger: false },
    MenuItem { label: "Delete", key: "d", danger: true },
];
fn p_menu(f: &mut Frame, a: Rect, _: &mut PageState) {
    let m = Menu { title: "Task", items: MENU, selected: 1 };
    let (w, h) = m.size();
    f.render_widget(m, at(a, 0, h, w));
    cap(f, at(a, h + 1, 1, a.width), "actions + keys in a popup · danger last, in red");
}
fn p_table(f: &mut Frame, a: Rect, _: &mut PageState) {
    let cols = [
        Column { header: "Name", width: 0, right: false },
        Column { header: "Size", width: 8, right: true },
        Column { header: "Changed", width: 10, right: true },
    ];
    let rows: &[&[&str]] = &[&["README.md", "4 KB", "today"], &["Cargo.toml", "1 KB", "today"], &["a_very_long_file_name_that_will_be_cut.rs", "12 KB", "yesterday"]];
    f.render_widget(Table { columns: &cols, rows, selected: Some(1), focused: true }, at(a, 0, 4, a.width.min(56)));
    cap(f, at(a, 5, 1, a.width), "header in heading tone · numbers right-aligned · cells cut with …");
}
fn p_proplist(f: &mut Frame, a: Rect, _: &mut PageState) {
    f.render_widget(PropertyList(&[("Name", "pond"), ("Version", "0.4.2"), ("Path", "~/apps/pond"), ("Size", "1.2 MB")]), at(a, 0, 4, 40));
    cap(f, at(a, 5, 1, a.width), "read-only facts, keys aligned");
}
fn p_error(f: &mut Frame, a: Rect, _: &mut PageState) {
    let area = at(a, 0, a.height.min(10), a.width.min(60));
    let block = atoms::panel("Today", false);
    let inner = block.inner(area);
    f.render_widget(block, area);
    f.render_widget(ErrorState { what: "Couldn't load tasks", why: "The sync server didn't answer (timeout).", retry_focused: true }, inner);
}
fn p_toast(f: &mut Frame, a: Rect, _: &mut PageState) {
    let stage = at(a, 0, a.height.min(10), a.width.min(60));
    f.render_widget(atoms::panel("screen", false), stage);
    let t = Toast { kind: Status::Ok, message: "Task marked done" };
    let r = t.area(stage);
    f.render_widget(t, r);
    cap(f, at(a, stage.height + 1, 1, a.width), "bottom-right, above the status bar · goes away by itself");
}
const COMMANDS: &[MenuItem] = &[
    MenuItem { label: "New task", key: "n", danger: false },
    MenuItem { label: "Go to tab: Today", key: "1", danger: false },
    MenuItem { label: "Delete task", key: "d", danger: true },
];
fn p_palette(f: &mut Frame, a: Rect, _: &mut PageState) {
    let p = CommandPalette { query: "ta", items: COMMANDS, selected: 0 };
    let h = p.area(a).height;
    f.render_widget(p, at(a, 0, h, a.width.min(60)));
}
fn p_help(f: &mut Frame, a: Rect, _: &mut PageState) {
    let h = HelpOverlay { keymap: crate::pages::TASKS_KEYS };
    let r = h.area(a);
    f.render_widget(h, at(a, 0, r.height, r.width));
}
fn p_editor(f: &mut Frame, a: Rect, _: &mut PageState) {
    let [t, _, e, _, p] = Layout::horizontal([Constraint::Length(5), Constraint::Length(1), Constraint::Min(30), Constraint::Length(1), Constraint::Length(30)]).areas(at(a, 0, 16, a.width));
    f.render_widget(EditorTools { active: 1 }, t);
    f.render_widget(NoteEditor { focused: true, cursor_line: 5 }, e);
    f.render_widget(TaskProperties { task: &TASKS[1] }, p);
}
fn p_projecttable(f: &mut Frame, a: Rect, _: &mut PageState) {
    f.render_widget(ProjectTable { selected: Some(1), focused: false }, at(a, 0, 7, a.width.min(70)));
}

// ── previews: tool parts ──
fn p_number(f: &mut Frame, a: Rect, _: &mut PageState) {
    let w = |r: Rect| Rect { width: r.width.min(22), ..r };
    let r = rows(a, 8);
    f.render_widget(NumberField::new(16.6, "ms").decimals(1), w(r[0]));
    f.render_widget(NumberField::new(0.45, "").decimals(2).range(0.0, 1.0).focused(true), w(r[1]));
    f.render_widget(NumberField::new(0.0, "%").range(0.0, 100.0), w(r[2]));
    f.render_widget(NumberField::new(-3.0, "°C").invalid(true), w(r[3]));
    cap(f, r[4], "normal · focused (←→ steps) · at its min (◂ dims) · invalid");
    f.render_widget(Slider::new(0.45, 0.0, 1.0), Rect { width: r[6].width.min(30), ..r[6] });
    f.render_widget(Slider::new(0.8, 0.0, 1.0).focused(true), Rect { width: r[7].width.min(30), ..r[7] });
}
fn p_select(f: &mut Frame, a: Rect, _: &mut PageState) {
    f.render_widget(Select::new("5 minutes"), at(a, 0, 1, 30));
    f.render_widget(Select::new("5 minutes").focused(true), at(a, 2, 1, 30));
    f.render_widget(Select::new("5 minutes").open(true), at(a, 4, 1, 30));
    let opts = [MenuItem { label: "1 minute", key: "", danger: false }, MenuItem { label: "5 minutes", key: "", danger: false }, MenuItem { label: "Hourly", key: "", danger: false }];
    let m = Menu { title: "", items: &opts, selected: 1 };
    let (mw, mh) = m.size();
    f.render_widget(m, at(a, 5, mh, mw.max(30)));
    cap(f, at(a, 5 + mh + 1, 1, a.width), "closed · focused · open = Select + a Menu under it");
}
fn p_charts(f: &mut Frame, a: Rect, _: &mut PageState) {
    let fm = crate::data::FRAME_MS;
    f.render_widget(Sparkline::new(fm).max(25.0).budget(16.6), at(a, 0, 1, 60));
    cap(f, at(a, 1, 1, a.width), "Sparkline: one row · with a budget, values over it are red");
    f.render_widget(Histogram { values: fm, max: Some(25.0), budget: Some(16.6) }, at(a, 3, 5, 60));
    cap(f, at(a, 8, 1, a.width), "Histogram: several rows, eighth-block precision, ┄ budget line");
    f.render_widget(Bar { fraction: 0.62, over: false }, at(a, 10, 1, 30));
    f.render_widget(Bar { fraction: 0.91, over: true }, at(a, 11, 1, 30));
    f.render_widget(Paragraph::new(Line::from(vec![
        atoms::delta_badge(-0.12, false), Span::raw("   "), atoms::delta_badge(0.08, false), Span::raw("   "),
        atoms::delta_badge(0.15, true), Span::raw("   "), atoms::delta_badge(0.0, true),
    ])), at(a, 13, 1, a.width));
    cap(f, at(a, 14, 1, a.width), "Bar · delta_badge: green when it went the good way (frame time ▼ = good, FPS ▲ = good)");
}
fn p_split(f: &mut Frame, a: Rect, _: &mut PageState) {
    let area = at(a, 0, 10, a.width.min(70));
    let [l, h, r] = quag_theme::ui::split(area, 0.4, true, 10);
    f.render_widget(atoms::panel("left", false), l);
    f.render_widget(SplitHandle { vertical: true, focused: true }, h);
    f.render_widget(atoms::panel("right", false), r);
    cap(f, at(a, 11, 1, a.width), "ui::split + SplitHandle · [ ] resize by 5 % · each pane keeps a minimum");
}
fn p_tree(f: &mut Frame, a: Rect, st: &mut PageState) {
    let rows = flatten(crate::data::TREE, &|id| st.open.contains(id));
    f.render_widget(TreeView { rows: &rows, selected: 4, focused: true }, at(a, 0, 12, 40));
    cap(f, at(a, 13, 1, a.width), "▸ closed · ▾ open · → opens, ← closes · the app keeps which ids are open");
}
fn p_paramrow(f: &mut Frame, a: Rect, _: &mut PageState) {
    f.render_widget(ParamRow { label: "Roughness", label_width: 14, field: NumberField::new(0.45, "").decimals(2).range(0.0, 1.0) }, at(a, 0, 1, 64));
    f.render_widget(ParamRow { label: "Exposure", label_width: 14, field: NumberField::new(1.5, "EV").decimals(1).range(-4.0, 4.0).focused(true) }, at(a, 1, 1, 64));
    f.render_widget(ParamRow { label: "Exposure", label_width: 14, field: NumberField::new(1.5, "EV").decimals(1).range(-4.0, 4.0) }, at(a, 3, 1, 30));
    cap(f, at(a, 4, 1, a.width), "narrow: the slider goes, the number stays");
}
fn p_barchart(f: &mut Frame, a: Rect, _: &mut PageState) {
    f.render_widget(BarChart { items: crate::data::MEMORY_MB, unit: "MB", max: Some(1000.0), budget: Some(800.0) }, at(a, 0, 5, a.width.min(64)));
    cap(f, at(a, 6, 1, a.width), "label · bar · value, budget 800 MB");
}
fn p_logviewer(f: &mut Frame, a: Rect, st: &mut PageState) {
    let lines: Vec<LogEntry> = st.log.iter().map(|(t, k, m)| LogEntry { time: t, kind: *k, message: m }).collect();
    f.render_widget(LogViewer { title: "api-1.log", lines: &lines, filter: "failed", filter_focused: true, follow: true, scroll: 0, focused: true, only: None }, at(a, 0, 14, a.width));
}
fn p_perf(f: &mut Frame, a: Rect, _: &mut PageState) {
    f.render_widget(FrameBudget, at(a, 0, 9, a.width.min(70)));
    f.render_widget(GpuMemory, at(a, 10, 7, a.width.min(70)));
}
fn p_material(f: &mut Frame, a: Rect, _: &mut PageState) {
    f.render_widget(MaterialParams { focused: 0 }, at(a, 0, 6, a.width.min(64)));
    let cpu1 = [22.0, 30.0, 28.0, 35.0, 41.0, 38.0, 44.0, 52.0, 47.0, 42.0, 39.0, 42.0];
    let cpu2 = [55.0, 62.0, 70.0, 78.0, 84.0, 91.0, 88.0, 86.0, 82.0, 79.0, 85.0, 87.0];
    f.render_widget(ServerLoad { name: "api-1", cpu: &cpu1 }, at(a, 8, 1, 40));
    f.render_widget(ServerLoad { name: "db-1", cpu: &cpu2 }, at(a, 9, 1, 40));
    cap(f, at(a, 10, 1, a.width), "ServerLoad: Sparkline with an 80 % budget");
}
fn p_activity(f: &mut Frame, a: Rect, st: &mut PageState) {
    f.render_widget(Activity, at(a, 0, 7, a.width.min(50)));
    f.render_widget(SyncSettings { focused: Some(1), lead_minutes: st.lead }, at(a, 8, SyncSettings::HEIGHT, a.width.min(56)));
}
fn p_projecttree(f: &mut Frame, a: Rect, st: &mut PageState) {
    let rows = st.tree_rows();
    f.render_widget(ProjectTree { rows: &rows, selected: st.tree_sel, focused: true }, at(a, 0, 13, 36));
}

// ── previews: security parts ──
fn p_severity(f: &mut Frame, a: Rect, _: &mut PageState) {
    let mut spans = Vec::new();
    for s in Severity::ALL { spans.extend([atoms::severity_badge(s), Span::raw("  ")]); }
    f.render_widget(Paragraph::new(Line::from(spans)), at(a, 0, 1, a.width));
    cap(f, at(a, 1, 1, a.width), "5 levels on 4 colors: CRIT is filled, HIGH bold. Same width so lists line up");
    for (i, (s, t)) in [(Severity::Critical, "API key committed to repository"), (Severity::Medium, "TLS certificate expires in 9 days"), (Severity::Info, "New user 'deploy' added")].iter().enumerate() {
        f.render_widget(&ListRow { lead: Some(atoms::severity_badge(*s)), title: t, meta: "api-1", ..Default::default() }, at(a, 3 + i as u16, 1, a.width.min(64)));
    }
    cap(f, at(a, 7, 1, a.width), "in a ListRow: `lead: Some(severity_badge(..))`");
}
fn p_secret(f: &mut Frame, a: Rect, _: &mut PageState) {
    f.render_widget(SecretField::new("sk_live_51HxQ2bK9vTz7m3f9a"), at(a, 0, 1, 40));
    f.render_widget(SecretField::new("sk_live_51HxQ2bK9vTz7m3f9a").focused(true), at(a, 2, 1, 40));
    f.render_widget(SecretField::new("sk_live_51HxQ2bK9vTz7m3f9a").revealed(true), at(a, 4, 1, 40));
    cap(f, at(a, 5, 1, a.width), "masked (last 4 show, to tell keys apart) · focused · revealed (yellow: it's exposed)");
}
fn p_segmented(f: &mut Frame, a: Rect, _: &mut PageState) {
    f.render_widget(Segmented { options: crate::data::RANGES, active: 2, focused: false }, at(a, 0, 1, a.width));
    f.render_widget(Segmented { options: &["List", "Grid"], active: 0, focused: true }, at(a, 2, 1, a.width));
    cap(f, at(a, 3, 1, a.width), "a few short options as joined buttons · time ranges, view modes");
}
fn p_typeconfirm(f: &mut Frame, a: Rect, _: &mut PageState) {
    let d = TypeToConfirm { title: "Delete server", question: "Delete api-1 and all its data?", name: "api-1", typed: "api", confirm: "Delete" };
    let r = d.area(at(a, 0, 12, a.width.min(70)));
    f.render_widget(d, Rect { y: a.y, ..r });
    cap(f, at(a, r.height + 1, 1, a.width), "the danger button stays disabled until the name matches");
}
fn p_findings(f: &mut Frame, a: Rect, _: &mut PageState) {
    let list: Vec<_> = crate::data::FINDINGS.iter().collect();
    let [l, _, r] = Layout::horizontal([Constraint::Min(40), Constraint::Length(1), Constraint::Length(36)]).areas(at(a, 0, 14, a.width));
    f.render_widget(FindingsList { findings: &list, selected: 0, focused: true, range: 3 }, l);
    f.render_widget(FindingDetail { finding: Some(list[0]), revealed: false, button: None }, r);
}
fn p_firewall(f: &mut Frame, a: Rect, _: &mut PageState) {
    f.render_widget(FirewallRules, at(a, 0, 8, a.width.min(64)));
}

// ── previews: server parts ──
fn p_runstate(f: &mut Frame, a: Rect, st: &mut PageState) {
    use atoms::RunState as R;
    for (i, r) in [R::Running, R::Starting, R::Restarting, R::Stopped, R::Paused, R::Exited(0), R::Exited(137), R::Flapping(5)].iter().enumerate() {
        f.render_widget(Line::from(atoms::run_state(*r, st.tick)), at(a, i as u16, 1, 24));
    }
    cap(f, at(a, 8, 1, a.width), "stopped / paused / exited 0 are grey, not errors · busy states spin · restart loop warns");
    f.render_widget(Line::from(vec![atoms::env_tag(atoms::Env::Prod), Span::raw("  "), atoms::env_tag(atoms::Env::Staging), Span::raw("  "), atoms::env_tag(atoms::Env::Dev), Span::raw("     "), atoms::short_id("sha256:3f9a1c2b7e41aa00ff")]), at(a, 10, 1, a.width));
    cap(f, at(a, 11, 1, a.width), "env_tag: always in the header · short_id: 12 chars, faint");
}
fn p_meter(f: &mut Frame, a: Rect, _: &mut PageState) {
    use quag_theme::{fmt, molecules::Meter};
    const M: u64 = 1024 * 1024;
    for (i, (l, u, lim)) in [("api-1", 210 * M, 512 * M), ("api-2", 430 * M, 512 * M), ("worker", 530 * M, 512 * M)].iter().enumerate() {
        let v = fmt::bytes_of(*u, *lim);
        f.render_widget(Meter { label: l, label_width: 7, fraction: *u as f64 / *lim as f64, value: &v, value_width: 13 }, at(a, i as u16, 1, a.width.min(56)));
    }
    cap(f, at(a, 3, 1, a.width), "UsageBar atom: accent, then yellow from 80 %, red at the limit (+ icon, not color alone)");
    let ports = [quag_theme::molecules::PortMap { host: Some(8080), container: 80, proto: "tcp", public: false }, quag_theme::molecules::PortMap { host: Some(6379), container: 6379, proto: "tcp", public: true }, quag_theme::molecules::PortMap { host: None, container: 5432, proto: "tcp", public: false }];
    for (i, p) in ports.iter().enumerate() { f.render_widget(Line::from(p.spans()), at(a, 5 + i as u16, 1, 30)); }
    cap(f, at(a, 8, 1, a.width), "PortMap: host → container/proto · ! = published on all interfaces");
    for (i, age) in [3u64, 190, 45].iter().enumerate() {
        f.render_widget(Line::from(quag_theme::molecules::freshness(*age, if i == 1 { 600 } else { 30 })), at(a, 10 + i as u16, 1, 30));
    }
    cap(f, at(a, 13, 1, a.width), "freshness: live data says how old it is; yellow once stale");
}
fn p_steps(f: &mut Frame, a: Rect, st: &mut PageState) {
    use atoms::StepState as S;
    use quag_theme::molecules::{Step, StepList, KeyValueRows};
    let steps = [Step { name: "pull", state: S::Done, took: "8s" }, Step { name: "migrate", state: S::Done, took: "3s" }, Step { name: "health check", state: S::Running, took: "" }, Step { name: "switch", state: S::Waiting, took: "" }];
    f.render_widget(StepList { steps: &steps, inline: true, tick: st.tick }, at(a, 0, 1, a.width));
    f.render_widget(StepList { steps: &steps, inline: false, tick: st.tick }, at(a, 2, 4, 30));
    let failed = [Step { name: "pull", state: S::Done, took: "8s" }, Step { name: "health check", state: S::Failed, took: "" }, Step { name: "switch", state: S::Skipped, took: "" }];
    f.render_widget(StepList { steps: &failed, inline: true, tick: st.tick }, at(a, 7, 1, a.width));
    cap(f, at(a, 8, 1, a.width), "StepList inline (headers, rows) or vertical with times · failed → the rest is skipped");
    f.render_widget(KeyValueRows { rows: &[("RUST_LOG", "info", false), ("DATABASE_URL", "postgres://shop:hunter2@db/shop", true), ("WORKERS", "4", false)], key_width: 13, revealed: false }, at(a, 10, 3, a.width.min(56)));
    cap(f, at(a, 13, 1, a.width), "KeyValueRows: env vars, secret values masked (SecretField)");
}
fn p_diff(f: &mut Frame, a: Rect, _: &mut PageState) {
    f.render_widget(quag_theme::elements::DiffView { title: "Changes · compose.yaml", lines: crate::dock::DIFF, scroll: 0, focused: true }, at(a, 0, 17, a.width.min(70)));
}
fn p_runprogress(f: &mut Frame, a: Rect, st: &mut PageState) {
    use atoms::StepState as S;
    use quag_theme::molecules::Step;
    let steps = [Step { name: "pull", state: S::Done, took: "8s" }, Step { name: "migrate", state: S::Done, took: "3s" }, Step { name: "start new", state: S::Running, took: "" }, Step { name: "health check", state: S::Waiting, took: "" }];
    let log: Vec<LogEntry> = crate::dock::DEPLOY_LOG[..5].iter().map(|(t, k, m)| LogEntry { time: t, kind: *k, message: m }).collect();
    f.render_widget(quag_theme::elements::RunProgress { title: "Deploy shop 1.9.0", steps: &steps, output: &log, tick: st.tick, focused: true }, at(a, 0, 14, a.width.min(84)));
    cap(f, at(a, 15, 1, a.width), "a zone of the page, not a modal: the app stays usable while it runs");
}
fn p_containers(f: &mut Frame, a: Rect, st: &mut PageState) {
    let states = crate::dock::initial_states();
    let [t, _, i] = Layout::horizontal([Constraint::Min(60), Constraint::Length(1), Constraint::Length(40)]).areas(at(a, 0, 26, a.width));
    f.render_widget(crate::dock::ContainerTable { states: &states, selected: 1, focused: true, tick: st.tick, reachable: true, loading: false }, Rect { height: 13, ..t });
    f.render_widget(crate::dock::ContainerTable { states: &states, selected: 1, focused: false, tick: st.tick, reachable: false, loading: false }, Rect { y: t.y + 14, height: 8, ..t });
    f.render_widget(crate::dock::ContainerInspector { container: crate::dock::CONTAINERS.get(1), state: states[1], revealed: false, tick: st.tick, focused: false, button: None }, i);
}
fn p_hosts(f: &mut Frame, a: Rect, _: &mut PageState) {
    let [l, _, r] = Layout::horizontal([Constraint::Fill(1), Constraint::Length(1), Constraint::Fill(1)]).areas(at(a, 0, 10, a.width.min(110)));
    f.render_widget(crate::dock::HostCard { host: &crate::dock::HOSTS[2], selected: true }, l);
    f.render_widget(crate::dock::HostCard { host: &crate::dock::HOSTS[3], selected: false }, r);
    f.render_widget(crate::dock::NeedsAttention, at(a, 11, 7, a.width.min(110)));
}

// ── previews: game design ──
fn p_curve(f: &mut Frame, a: Rect, _: &mut PageState) {
    let c = &crate::forge::CURVES[0];
    f.render_widget(quag_theme::molecules::CurveEditor { points: c.points, x: c.x, y: c.y, x_unit: c.x_unit, y_unit: c.y_unit, selected: Some(3) }, at(a, 0, 14, a.width.min(80)));
    cap(f, at(a, 15, 1, a.width), "Curve atom (braille line, keys ●, selected ◆) + labels + readout · nudge_point keeps keys in order");
}
fn p_weights(f: &mut Frame, a: Rect, _: &mut PageState) {
    f.render_widget(quag_theme::molecules::WeightsEditor { items: crate::forge::LOOT[0].1, selected: 3, focused: true }, at(a, 0, 6, a.width.min(80)));
    cap(f, at(a, 7, 1, a.width), "raw weights → % of total (always 100 %) and odds; ← → change the selected weight");
}
fn p_timeline(f: &mut Frame, a: Rect, _: &mut PageState) {
    f.render_widget(quag_theme::molecules::Timeline { tracks: crate::forge::SEQUENCE, span: crate::forge::SEQ_SPAN, playhead: 12.5, unit: "s", selected: Some((0, 2)) }, at(a, 0, 12, a.width.min(100)));
    cap(f, at(a, 13, 1, a.width), "ruler with nice steps · Clip atoms (selected = Quag) · playhead ▼│ in Gill over everything");
}
fn p_balance(f: &mut Frame, a: Rect, _: &mut PageState) {
    f.render_widget(crate::forge::BalanceTable { selected: 3, load: crate::forge::Load::Ready }, at(a, 0, 9, a.width.min(110)));
    cap(f, at(a, 10, 1, a.width), "deltas know which way is good (cost ▼ = green) · outliers: strong = yellow, weak = blue (not errors)");
}

fn p_nodegraph(f: &mut Frame, a: Rect, _: &mut PageState) {
    use crate::forge::{GRAPH, GRAPH_AT, WIRES};
    f.render_widget(quag_theme::molecules::NodeGraph { nodes: GRAPH, positions: GRAPH_AT, wires: WIRES, selected: Some(1), pan: (0, 0) }, at(a, 0, 17, a.width));
    cap(f, at(a, 18, 1, a.width), "node_frame + port + wire (atoms) · wires drawn first, nodes on top · selected node and its wires in Quag");
}

pub fn catalog() -> Vec<Entry> {
    use Kind::Part;
    let mut v = vec![
        Entry { id: "button", level: Level::Atom, name: "Button", what: "One button. Kinds are props, not new atoms.", from: "", kind: Part(p_button) },
        Entry { id: "checkbox", level: Level::Atom, name: "Checkbox", what: "A box with a label. States: checked, unchecked, focused, disabled.", from: "", kind: Part(p_checkbox) },
        Entry { id: "input", level: Level::Atom, name: "Input", what: "One-line text input.", from: "", kind: Part(p_input) },
        Entry { id: "text", level: Level::Atom, name: "label (Tone)", what: "Text in one of six tones. Use it instead of choosing a color.", from: "", kind: Part(p_text) },
        Entry { id: "small", level: Level::Atom, name: "tag · kbd · status", what: "Small atoms: tag, key, status icon, spinner, progress.", from: "", kind: Part(p_small) },
        Entry { id: "radio", level: Level::Atom, name: "Radio", what: "One option of a single choice.", from: "", kind: Part(p_radio) },
        Entry { id: "badge", level: Level::Atom, name: "badge · link · status_label", what: "A count, a link, text in a status color.", from: "", kind: Part(p_badge) },
        Entry { id: "progressbars", level: Level::Atom, name: "SmoothProgress · StepProgress", what: "Progress that moves smoothly (eighths of a cell), and progress by steps. Numbers next to the bar.", from: "", kind: Part(p_progressbars) },
        Entry { id: "skeleton", level: Level::Atom, name: "Skeleton", what: "Loading placeholder where text will be.", from: "", kind: Part(p_skeleton) },
        Entry { id: "number", level: Level::Atom, name: "NumberField · Slider", what: "A number with its unit (←→ steps), and its slider.", from: "", kind: Part(p_number) },
        Entry { id: "select", level: Level::Atom, name: "Select", what: "A closed dropdown; open, the app shows a Menu under it.", from: "", kind: Part(p_select) },
        Entry { id: "charts", level: Level::Atom, name: "Sparkline · Histogram · Bar · delta", what: "The chart atoms. Budgets turn values red; deltas know which way is good.", from: "", kind: Part(p_charts) },
        Entry { id: "split", level: Level::Atom, name: "SplitHandle (+ ui::split)", what: "Resizable panes.", from: "", kind: Part(p_split) },
        Entry { id: "severity", level: Level::Atom, name: "severity_badge", what: "Five severity levels for findings, lint, audits.", from: "", kind: Part(p_severity) },
        Entry { id: "secret", level: Level::Atom, name: "SecretField", what: "A key or password: masked, revealed on demand.", from: "", kind: Part(p_secret) },
        Entry { id: "runstate", level: Level::Atom, name: "run_state · env_tag · short_id", what: "Servers: container state, environment, ids.", from: "", kind: Part(p_runstate) },
        Entry { id: "box", level: Level::Atom, name: "panel · popup", what: "The boxes: containers with nothing inside yet.", from: "", kind: Part(p_box) },
        Entry { id: "buttongroup", level: Level::Molecule, name: "ButtonGroup", what: "An array of buttons in a box. Doesn't know what they do.", from: "Button × n + panel", kind: Part(p_buttongroup) },
        Entry { id: "field", level: Level::Molecule, name: "Field", what: "Label above an input, help or error under it.", from: "label + Input + label", kind: Part(p_field) },
        Entry { id: "searchbar", level: Level::Molecule, name: "SearchBar", what: "The search line.", from: "kbd + Input", kind: Part(p_search) },
        Entry { id: "tabs", level: Level::Molecule, name: "Tabs · KeyHints · StatusLine", what: "Rows of small atoms.", from: "label / kbd / status_icon", kind: Part(p_tabs) },
        Entry { id: "listrow", level: Level::Molecule, name: "ListRow", what: "One row of any list.", from: "check + label + tag + status_icon", kind: Part(p_listrow) },
        Entry { id: "stat", level: Level::Molecule, name: "Stat · CheckList · LogLine", what: "KPI tile, column of checkboxes, one log line.", from: "panel + label · Checkbox × n · label + status_icon", kind: Part(p_stat) },
        Entry { id: "radiogroup", level: Level::Molecule, name: "RadioGroup", what: "One choice among a few. Row, or column when tight.", from: "Radio × n", kind: Part(p_radiogroup) },
        Entry { id: "breadcrumbs", level: Level::Molecule, name: "Breadcrumbs", what: "Where you are. Collapses the middle when tight.", from: "label × n", kind: Part(p_breadcrumbs) },
        Entry { id: "menu", level: Level::Molecule, name: "Menu", what: "Actions with their keys, in a popup.", from: "popup + ListRow × n", kind: Part(p_menu) },
        Entry { id: "table", level: Level::Molecule, name: "Table", what: "Rows and columns with a header line.", from: "label × cells + ListRow selection", kind: Part(p_table) },
        Entry { id: "proplist", level: Level::Molecule, name: "PropertyList", what: "Read-only key / value facts.", from: "label × 2 per row", kind: Part(p_proplist) },
        Entry { id: "tree", level: Level::Molecule, name: "TreeView", what: "Any hierarchy: files, scenes, rules, call trees.", from: "ListRow × n (indent + ▸▾)", kind: Part(p_tree) },
        Entry { id: "paramrow", level: Level::Molecule, name: "ParamRow", what: "A tunable number: label, slider, value.", from: "label + Slider + NumberField", kind: Part(p_paramrow) },
        Entry { id: "barchart", level: Level::Molecule, name: "BarChart", what: "Compare a few amounts. Over budget = red.", from: "label + Bar + label", kind: Part(p_barchart) },
        Entry { id: "segmented", level: Level::Molecule, name: "Segmented", what: "One of a few short options (time range, view).", from: "Button × n (compact)", kind: Part(p_segmented) },
        Entry { id: "meter", level: Level::Molecule, name: "Meter · PortMap · freshness", what: "Usage of a limit; a published port; how old live data is.", from: "label + UsageBar + status_icon · labels", kind: Part(p_meter) },
        Entry { id: "steps", level: Level::Molecule, name: "StepList · KeyValueRows", what: "Steps of a job; env vars with masked secrets.", from: "step_icon + label · label + SecretField", kind: Part(p_steps) },
        Entry { id: "curve", level: Level::Molecule, name: "Curve · CurveEditor", what: "Game design: a curve to shape (difficulty, XP, falloff).", from: "Curve (atom) + labels", kind: Part(p_curve) },
        Entry { id: "weights", level: Level::Molecule, name: "WeightsEditor", what: "Weighted choices (loot, spawns): %, odds, always 100 %.", from: "ListRow + Bar + labels", kind: Part(p_weights) },
        Entry { id: "timeline", level: Level::Molecule, name: "Timeline", what: "Tracks of clips over time (cutscenes, events, waves).", from: "Clip + playhead (atoms) + ruler", kind: Part(p_timeline) },
        Entry { id: "nodegraph", level: Level::Molecule, name: "NodeGraph", what: "Nodes and wires (abilities, dialogue, shaders, AI).", from: "node_frame + port + wire (atoms) + route", kind: Part(p_nodegraph) },
        Entry { id: "taskactions", level: Level::Element, name: "TaskActions", what: "Your example: the ButtonGroup put to use for the selected task.", from: "ButtonGroup + task", kind: Part(p_taskactions) },
        Entry { id: "tasklist", level: Level::Element, name: "TaskList", what: "The tasks. Owns its states: ideal, empty.", from: "panel + SearchBar + ListRow × n, EmptyState", kind: Part(p_tasklist) },
        Entry { id: "taskinspector", level: Level::Element, name: "TaskInspector", what: "Details of the selection. State: nothing selected.", from: "Field × 2 + TaskActions", kind: Part(p_inspector) },
        Entry { id: "projectnav", level: Level::Element, name: "ProjectNav", what: "Pond's sidebar.", from: "ListRow × n + Progress", kind: Part(p_projectnav) },
        Entry { id: "weekstats", level: Level::Element, name: "WeekStats · Attention · Recent", what: "Dashboard elements fed from the tasks.", from: "Stat × 4 · StatusLine × n", kind: Part(p_dash) },
        Entry { id: "settingsel", level: Level::Element, name: "General/NotificationSettings", what: "Settings groups for the form page.", from: "Field × 2 · CheckList", kind: Part(p_settings) },
        Entry { id: "synclog", level: Level::Element, name: "SyncLog", what: "The sync events, newest at the bottom.", from: "LogLine × n", kind: Part(p_synclog) },
        Entry { id: "editor", level: Level::Element, name: "EditorTools · NoteEditor · TaskProperties", what: "The workspace elements.", from: "ListRow · Breadcrumbs + label · PropertyList + RadioGroup", kind: Part(p_editor) },
        Entry { id: "projecttable", level: Level::Element, name: "ProjectTable", what: "Every project with its counts.", from: "panel + Table", kind: Part(p_projecttable) },
        Entry { id: "projecttree", level: Level::Element, name: "ProjectTree", what: "The workspace's file tree.", from: "panel + TreeView", kind: Part(p_projecttree) },
        Entry { id: "activity", level: Level::Element, name: "Activity · SyncSettings", what: "Dashboard trend and a settings group.", from: "Histogram + delta_badge · Select + ParamRow", kind: Part(p_activity) },
        Entry { id: "perf", level: Level::Element, name: "FrameBudget · GpuMemory", what: "Optimisation: frame times vs 16.6 ms, memory per asset type.", from: "Histogram + delta_badge · BarChart", kind: Part(p_perf) },
        Entry { id: "material", level: Level::Element, name: "MaterialParams · ServerLoad", what: "Tech art tuning panel; server CPU trend.", from: "ParamRow × n · Sparkline", kind: Part(p_material) },
        Entry { id: "appheader", level: Level::Element, name: "AppHeader · StatusBar", what: "Shared elements every app has.", from: "label + Tabs · KeyHints + StatusLine", kind: Part(p_header) },
        Entry { id: "emptystate", level: Level::Element, name: "EmptyState", what: "Shared: empty zone, with the mascot in private builds.", from: "label + kbd (+ mascot)", kind: Part(p_empty) },
        Entry { id: "errorstate", level: Level::Element, name: "ErrorState", what: "Shared: a zone failed. What + why + Retry; the rest stays usable.", from: "status_label + label + Button", kind: Part(p_error) },
        Entry { id: "toast", level: Level::Element, name: "Toast", what: "Shared: a result that should be noticed.", from: "popup + StatusLine", kind: Part(p_toast) },
        Entry { id: "palette", level: Level::Element, name: "CommandPalette", what: "Shared: ctrl+k, every action by name.", from: "popup + SearchBar + Menu rows", kind: Part(p_palette) },
        Entry { id: "help", level: Level::Element, name: "HelpOverlay", what: "Shared: ?, every key of the page.", from: "popup + label + kbd", kind: Part(p_help) },
        Entry { id: "logviewer", level: Level::Element, name: "LogViewer", what: "Shared: big logs. Filter, highlight, follow; draws only what's visible.", from: "panel + SearchBar + LogLine-like rows + mark", kind: Part(p_logviewer) },
        Entry { id: "typeconfirm", level: Level::Element, name: "TypeToConfirm", what: "Shared: type the name before an action that can't be undone.", from: "popup + kbd + Input + ButtonGroup", kind: Part(p_typeconfirm) },
        Entry { id: "findings", level: Level::Element, name: "FindingsList · FindingDetail", what: "Security: findings worst first, and the selected one.", from: "Segmented + ListRow(lead: severity) · PropertyList + SecretField + ButtonGroup", kind: Part(p_findings) },
        Entry { id: "firewall", level: Level::Element, name: "FirewallRules", what: "Security: rules in order, with hit counts.", from: "panel + Table", kind: Part(p_firewall) },
        Entry { id: "diff", level: Level::Element, name: "DiffView", what: "Shared: review a change before applying it.", from: "panel + diff_line + status_label + EmptyState", kind: Part(p_diff) },
        Entry { id: "runprogress", level: Level::Element, name: "RunProgress", what: "Shared: a running job, its output, Cancel / Retry.", from: "StepList + LogViewer + ButtonGroup", kind: Part(p_runprogress) },
        Entry { id: "containers", level: Level::Element, name: "ContainerTable · ContainerInspector", what: "dock: containers of a host, and the selected one.", from: "Table (+ cell_areas) + run_state + Sparkline + Meter · PropertyList + PortMap + KeyValueRows", kind: Part(p_containers) },
        Entry { id: "hostcards", level: Level::Element, name: "HostCard · NeedsAttention", what: "dock: one host (or why it can't be reached); what's wrong.", from: "Meter + Sparkline / ErrorState · ListRow(lead)", kind: Part(p_hosts) },
        Entry { id: "balance", level: Level::Element, name: "BalanceTable", what: "forge: units vs last patch, value for cost, outliers.", from: "Table (+ cell_areas) + delta_badge + status_label", kind: Part(p_balance) },
        Entry { id: "confirm", level: Level::Element, name: "ConfirmDialog", what: "Shared: “are you sure?”. Focus starts on the safe button.", from: "popup + label + ButtonGroup", kind: Part(p_confirm) },
    ];
    for (i, p) in PAGES.iter().enumerate() {
        v.push(Entry { id: p.id, level: Level::Ui, name: p.name, what: p.layout, from: "elements only", kind: Kind::Page(i) });
    }
    v
}
