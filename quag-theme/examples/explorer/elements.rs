//! ELEMENTS of this app: molecules put to use, wired to pond's data.
//! Each element = one job. It picks the molecules, feeds them data, and owns its
//! states (empty, etc). It never sets colors: that's the atoms' job.

use crate::data::{PROJECTS, Task};
use quag_theme::molecules::ActionButton as AB;
use quag_theme::{
    Status,
    atoms::{self, Button, SecretField, Histogram, Input, NumberField, Progress, Select, Sparkline, Tone},
    elements::EmptyState,
    molecules::{Align, BarChart, Segmented, Breadcrumbs, ParamRow, TreeRow, TreeView, ButtonGroup, CheckList, Column, Field, ListRow, LogLine, PropertyList, RadioGroup, SearchBar, Stat, StatusLine, Table},
    ui,
};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    text::Line,
    widgets::Widget,
};

// ── TaskActions: the user's example. ButtonGroup (molecule) + the selected task = element ──

/// What can be done with the selected task. The *element* decides which buttons
/// exist and their labels from the data (done → "Reopen"); the molecule just lays them out.
pub struct TaskActions<'a> {
    pub task: &'a Task,
    /// Index of the focused button, if the actions row has focus.
    pub focused: Option<usize>,
}

/// The rows, written once: label, the tasks keymap's action (Enter on a button = its key), look.
const OPEN_TASK: &[AB<'static>] = &[AB::new("Mark done", "done").primary(), AB::new("Edit", "edit"), AB::new("Delete", "delete").danger()];
const DONE_TASK: &[AB<'static>] = &[AB::new("Reopen", "done"), AB::new("Edit", "edit"), AB::new("Delete", "delete").danger()];

impl TaskActions<'_> {
    /// The buttons for this task (done → "Reopen").
    pub fn buttons(task: &Task) -> &'static [AB<'static>] { if task.done { DONE_TASK } else { OPEN_TASK } }
    pub fn group(&self) -> ButtonGroup<'static> { AB::group(Self::buttons(self.task), self.focused) }
}

impl Widget for TaskActions<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.group().align(Align::Left).render(area, buf);
    }
}

// ── TaskList ──

pub struct TaskList<'a> {
    pub title: &'a str,
    pub tasks: &'a [&'a Task],
    pub selected: usize,
    pub focused: bool,
    pub query: &'a str,
}

impl Widget for TaskList<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = atoms::panel(self.title, self.focused);
        let inner = block.inner(area);
        block.render(area, buf);
        let [bar, rows] = ui::toolbar_split(inner);
        SearchBar { query: self.query, focused: false }.render(Rect { width: bar.width.min(40), ..bar }, buf);

        // State: empty
        if self.tasks.is_empty() {
            EmptyState { message: "Nothing here yet.", hint: Some(("N", "to add a task")) }.render(rows, buf);
            return;
        }
        // State: ideal
        for (i, t) in self.tasks.iter().enumerate().take(rows.height as usize) {
            let row = Rect { y: rows.y + i as u16, height: 1, ..rows };
            (&ListRow {
                title: t.title,
                check: Some(t.done),
                tags: std::slice::from_ref(&t.tag),
                status: t.state,
                meta: t.due,
                meta_accent: t.due == "today",
                status_slot: true,
                meta_width: 8,
                selected: i == self.selected,
                list_focused: self.focused,
                lead: None,
            })
                .render(row, buf);
        }
    }
}

// ── TaskInspector: Fields + TaskActions ──

pub struct TaskInspector<'a> {
    pub task: Option<&'a Task>,
    pub focused: bool,
    /// The focused button of its actions row (Some while the zone has focus).
    pub button: Option<usize>,
}

impl Widget for TaskInspector<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = atoms::panel("Details", self.focused);
        let inner = block.inner(area);
        block.render(area, buf);
        // State: nothing selected → muted hint, not an empty box
        let Some(task) = self.task else {
            Line::from(atoms::label("Select a task", Tone::Muted)).render(inner, buf);
            return;
        };
        // Overflow state: drop the spacing before dropping any content
        let gap = if inner.height >= 12 { 1 } else { 0 };
        let [title, _, f1, _, f2, _, actions] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(gap),
            Constraint::Length(Field::HEIGHT),
            Constraint::Length(gap),
            Constraint::Length(Field::HEIGHT),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .areas(inner);
        Line::from(atoms::label(task.title, Tone::Heading)).render(title, buf);
        Field::new("Notes", Input::new(task.notes).placeholder("Add notes…").focused(self.focused && self.button.is_none()))
            .help("Markdown is fine.")
            .render(f1, buf);
        let bad_date = task.due == "tomorrow";
        let due = Field::new("Due", Input::new(if bad_date { "tomorow" } else { task.due }));
        let due = if bad_date { due.error("Can't read that date.") } else { due };
        due.render(f2, buf);
        TaskActions { task, focused: self.button }.render(actions, buf);
    }
}

// ── ProjectNav ──

pub struct ProjectNav<'a> {
    /// The app's current tasks (the counts follow done / deleted ones).
    pub tasks: &'a [crate::data::Task],
    pub selected: usize,
    pub focused: bool,
    pub week: (u32, u32),
}

impl Widget for ProjectNav<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = atoms::panel("Projects", self.focused);
        let inner = block.inner(area);
        block.render(area, buf);
        let [list, _, label, bar] = Layout::vertical([
            Constraint::Min(0),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .areas(inner);
        for (i, name) in PROJECTS.iter().enumerate().take(list.height as usize) {
            let row = Rect { y: list.y + i as u16, height: 1, ..list };
            let meta = crate::data::open_in(self.tasks, i).to_string();
            (&ListRow { title: name, meta: &meta, selected: i == self.selected, list_focused: self.focused, ..Default::default() })
                .render(row, buf);
        }
        Line::from(vec![
            atoms::label("This week  ", Tone::Heading),
            atoms::label(&format!("{} / {}", self.week.0, self.week.1), Tone::Muted),
        ])
        .render(label, buf);
        Progress(self.week.0 as f64 / self.week.1 as f64).render(bar, buf);
    }
}

// ── Dashboard elements ──

/// WeekStats: four Stat molecules fed from the data.
pub struct WeekStats;

impl Widget for WeekStats {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let open = crate::data::TASKS.iter().filter(|t| !t.done).count().to_string();
        let stats = [(open.as_str(), "open tasks"), ("7/12", "done this week"), ("2", "overdue"), ("4", "projects")];
        for (rect, (v, l)) in ui::columns(area, 4).into_iter().zip(stats) {
            Stat { value: v, label: l }.render(rect, buf);
        }
    }
}

/// Attention: tasks with an error or warning, as status lines in a panel.
pub struct Attention;

impl Widget for Attention {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = atoms::panel("Needs attention", false);
        let inner = block.inner(area);
        block.render(area, buf);
        let items: Vec<_> = crate::data::TASKS.iter().filter(|t| matches!(t.state, Some(Status::Err | Status::Warn))).collect();
        if items.is_empty() {
            Line::from(atoms::label("All good.", Tone::Muted)).render(inner, buf);
            return;
        }
        for (i, t) in items.iter().enumerate().take(inner.height as usize) {
            StatusLine { kind: t.state.unwrap(), message: t.title }.render(Rect { y: inner.y + i as u16, height: 1, ..inner }, buf);
        }
    }
}

/// Recent: last finished tasks.
pub struct Recent;

impl Widget for Recent {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = atoms::panel("Recent", false);
        let inner = block.inner(area);
        block.render(area, buf);
        for (i, t) in crate::data::TASKS.iter().filter(|t| t.done).enumerate().take(inner.height as usize) {
            StatusLine { kind: Status::Ok, message: t.title }.render(Rect { y: inner.y + i as u16, height: 1, ..inner }, buf);
        }
    }
}

// ── Settings elements ──

/// GeneralSettings: two fields, one of them with a validation error.
pub struct GeneralSettings {
    pub focused: Option<usize>,
}

impl Widget for GeneralSettings {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [h, _, f1, _, f2] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(Field::HEIGHT),
            Constraint::Length(1),
            Constraint::Length(Field::HEIGHT),
        ])
        .areas(area);
        Line::from(atoms::label("General", Tone::Heading)).render(h, buf);
        Field::new("Workspace name", Input::new("My pond").focused(self.focused == Some(0)))
            .help("Shown in the window title.")
            .render(f1, buf);
        Field::new("Week starts on", Input::new("Friday").focused(self.focused == Some(1)))
            .error("Pick Monday or Sunday.")
            .render(f2, buf);
    }
}

/// NotificationSettings: a CheckList fed from the settings.
pub struct NotificationSettings {
    pub focused: Option<usize>,
}

impl NotificationSettings {
    pub const HEIGHT: u16 = 5;
}

impl Widget for NotificationSettings {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [h, _, list] = Layout::vertical([Constraint::Length(1), Constraint::Length(1), Constraint::Min(0)]).areas(area);
        Line::from(atoms::label("Notifications", Tone::Heading)).render(h, buf);
        CheckList {
            items: &[("Remind me when a task is due", true), ("Daily summary at 9:00", false), ("Sounds", true)],
            focused: self.focused,
        }
        .render(list, buf);
    }
}

/// SettingsActions: Save / Cancel for the settings form (the last stop of Tab).
pub struct SettingsActions {
    pub focused: Option<usize>,
}

impl SettingsActions {
    pub const BUTTONS: &'static [AB<'static>] = &[AB::new("Save", "save").primary(), AB::new("Cancel", "cancel").ghost()];
}

impl Widget for SettingsActions {
    fn render(self, area: Rect, buf: &mut Buffer) {
        AB::group(Self::BUTTONS, self.focused).render(area, buf);
    }
}

// ── Stream element ──

/// SyncLog: the sync events, newest at the bottom.
pub struct SyncLog;

impl Widget for SyncLog {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = atoms::panel("Sync log", false);
        let inner = block.inner(area);
        block.render(area, buf);
        let log = crate::data::LOG;
        let skip = log.len().saturating_sub(inner.height as usize);
        for (i, (time, kind, msg)) in log.iter().skip(skip).enumerate() {
            LogLine { time, kind: *kind, message: msg }.render(Rect { y: inner.y + i as u16, height: 1, ..inner }, buf);
        }
    }
}

// ── Welcome element ──

/// WelcomeMessage: the text + call to action of the welcome page (the page draws the mascot above it).
pub struct WelcomeMessage;

impl WelcomeMessage {
    /// Its one button, always focused: Enter presses it.
    pub const BUTTONS: &'static [AB<'static>] = &[AB::new("Create a task", "new").primary()];
}

impl Widget for WelcomeMessage {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [t, h, _, b] = Layout::vertical([Constraint::Length(1), Constraint::Length(1), Constraint::Length(1), Constraint::Length(1)]).areas(area);
        Line::from(atoms::label("Welcome to pond.", Tone::Text)).centered().render(t, buf);
        Line::from(vec![
            atoms::label("Your tasks, calm and in one place. Press ", Tone::Muted),
            atoms::kbd("N"),
            atoms::label(".", Tone::Muted),
        ])
        .centered()
        .render(h, buf);
        let row = AB::group(Self::BUTTONS, Some(0));
        let w = row.width();
        row.render(Rect { x: b.x + b.width.saturating_sub(w) / 2, width: w, ..b }, buf);
    }
}

// ── Workspace elements (the note editor) ──

/// EditorTools: the tools strip of the editor. One ListRow per tool; the active one is selected.
pub struct EditorTools {
    pub active: usize,
}

pub const TOOLS: &[&str] = &["¶", "B", "I", "•", "#"];

impl Widget for EditorTools {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // A thin strip: no box (a box would leave no room), one compact button per tool.
        let inner = area;
        // One compact button per tool; the active tool is the primary one.
        for (i, t) in TOOLS.iter().enumerate().take(inner.height as usize / 2 + 1) {
            let b = Button::new(t).compact(true);
            let b = if i == self.active { b.primary() } else { b.ghost() };
            let w = b.width().min(inner.width);
            b.render(Rect { x: inner.x + inner.width.saturating_sub(w) / 2, y: inner.y + 2 * i as u16, width: w, height: 1 }, buf);
        }
    }
}

/// NoteEditor: where you are (Breadcrumbs) + the note text with line numbers.
pub struct NoteEditor {
    pub focused: bool,
    pub cursor_line: usize,
}

impl Widget for NoteEditor {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = atoms::panel("README.md", self.focused);
        let inner = block.inner(area);
        block.render(area, buf);
        let [crumbs, _, text] = Layout::vertical([Constraint::Length(1), Constraint::Length(1), Constraint::Min(0)]).areas(inner);
        Breadcrumbs(&["pond", "quag-theme", "Write README", "README.md"]).render(crumbs, buf);
        for (i, l) in crate::data::NOTE.iter().enumerate().take(text.height as usize) {
            let r = Rect { y: text.y + i as u16, height: 1, ..text };
            let tone = if l.starts_with('#') { Tone::Heading } else { Tone::Text };
            let mut spans = vec![atoms::label(&format!("{:>3}  ", i + 1), Tone::Faint), atoms::label(l, tone)];
            if i == self.cursor_line && self.focused {
                spans.push(atoms::label("▏", Tone::Accent));
            }
            Line::from(spans).render(r, buf);
        }
    }
}

/// TaskProperties: the inspector of the workspace. Facts (PropertyList) + one choice (RadioGroup).
pub struct TaskProperties<'a> {
    pub task: &'a Task,
}

impl Widget for TaskProperties<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = atoms::panel("Properties", false);
        let inner = block.inner(area);
        block.render(area, buf);
        let words = crate::data::NOTE.iter().map(|l| l.split_whitespace().count()).sum::<usize>().to_string();
        let lines = crate::data::NOTE.len().to_string();
        let facts = [("Task", self.task.title), ("Tag", self.task.tag), ("Due", self.task.due), ("Lines", lines.as_str()), ("Words", words.as_str())];
        let radio = RadioGroup { options: &["Low", "Normal", "High"], selected: 1, focused: None };
        let rh = radio.height(inner.width);
        let [p, _, l, r] = Layout::vertical([Constraint::Length(facts.len() as u16), Constraint::Length(1), Constraint::Length(1), Constraint::Length(rh)]).areas(inner);
        PropertyList(&facts).render(p, buf);
        Line::from(atoms::label("Priority", Tone::Muted)).render(l, buf);
        radio.render(r, buf);
    }
}

// ── Dashboard: a table ──

/// ProjectTable: every project with its counts. A Table fed from the data.
pub struct ProjectTable {
    pub selected: Option<usize>,
    pub focused: bool,
}

impl Widget for ProjectTable {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = atoms::panel("Projects", self.focused);
        let inner = block.inner(area);
        block.render(area, buf);
        let cols = [
            Column { header: "Project", width: 0, right: false },
            Column { header: "Open", width: 5, right: true },
            Column { header: "Done", width: 5, right: true },
            Column { header: "Changed", width: 11, right: true },
        ];
        Table { columns: &cols, rows: crate::data::PROJECT_ROWS, selected: self.selected, focused: self.focused }.render(inner, buf);
    }
}

// ── Dashboard: activity ──

/// Activity: tasks done per day (Sparkline) and the change vs the week before (delta_badge).
pub struct Activity;

impl Widget for Activity {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = atoms::panel("Activity", false);
        let inner = block.inner(area);
        block.render(area, buf);
        let d = crate::data::DONE_PER_DAY;
        let (prev, this): (f64, f64) = (d[..7].iter().sum(), d[7..].iter().sum());
        let [l, g, _, c] = Layout::vertical([Constraint::Length(1), Constraint::Length(2), Constraint::Length(1), Constraint::Length(1)]).areas(inner);
        Line::from(atoms::label("done per day · 14 days", Tone::Muted)).render(l, buf);
        Histogram { values: d, max: None, budget: None }.render(Rect { width: g.width.min(28), ..g }, buf);
        Line::from(vec![
            atoms::label(&format!("{this:.0} this week  "), Tone::Text),
            atoms::delta_badge(this / prev - 1.0, true),
            atoms::label("  vs last week", Tone::Faint),
        ])
        .render(c, buf);
    }
}

// ── Settings: sync ──

/// SyncSettings: how often to sync (Select) and how early to remind (ParamRow).
pub struct SyncSettings {
    pub focused: Option<usize>,
    pub lead_minutes: f64,
}

impl SyncSettings {
    pub const HEIGHT: u16 = 6;
}

impl Widget for SyncSettings {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [h, _, l, sel, _, p] = Layout::vertical([Constraint::Length(1), Constraint::Length(1), Constraint::Length(1), Constraint::Length(1), Constraint::Length(1), Constraint::Length(1)]).areas(area);
        Line::from(atoms::label("Sync", Tone::Heading)).render(h, buf);
        Line::from(atoms::label("Sync every", if self.focused == Some(0) { Tone::Accent } else { Tone::Muted })).render(l, buf);
        Select::new("5 minutes").focused(self.focused == Some(0)).render(Rect { width: sel.width.min(30), ..sel }, buf);
        ParamRow { label: "Remind before", label_width: 14, field: NumberField::new(self.lead_minutes, "min").range(crate::consts::LEAD_MIN, crate::consts::LEAD_MAX).focused(self.focused == Some(1)) }.render(p, buf);
    }
}

// ── Workspace: the project tree ──

/// ProjectTree: projects and their files (TreeView). The app keeps which ids are open.
pub struct ProjectTree<'a> {
    pub rows: &'a [TreeRow<'a>],
    pub selected: usize,
    pub focused: bool,
}

impl Widget for ProjectTree<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = atoms::panel("Files", self.focused);
        let inner = block.inner(area);
        block.render(area, buf);
        TreeView { rows: self.rows, selected: self.selected, focused: self.focused }.render(inner, buf);
    }
}

// ── Optimisation demo elements (catalog) ──

/// FrameBudget: frame times against the 60 FPS budget (16.6 ms), spikes in red, plus the summary.
pub struct FrameBudget;

impl Widget for FrameBudget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = atoms::panel("Frame time", false);
        let inner = block.inner(area);
        block.render(area, buf);
        let f = crate::data::FRAME_MS;
        let avg = f.iter().sum::<f64>() / f.len() as f64;
        let over = f.iter().filter(|v| **v > 16.6).count();
        let [head, g] = Layout::vertical([Constraint::Length(1), Constraint::Min(0)]).areas(inner);
        Line::from(vec![
            atoms::label(&format!("avg {avg:.1} ms   "), Tone::Text),
            atoms::status_label(&format!("{over} frames over 16.6 ms"), quag_theme::Status::Err),
            atoms::label("   ", Tone::Text),
            atoms::delta_badge(-0.083, false),
            atoms::label(" vs last build", Tone::Faint),
        ])
        .render(head, buf);
        Histogram { values: f, max: Some(25.0), budget: Some(16.6) }.render(g, buf);
    }
}

/// GpuMemory: memory per asset type (BarChart), budget 800 MB per type.
pub struct GpuMemory;

impl Widget for GpuMemory {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = atoms::panel("GPU memory", false);
        let inner = block.inner(area);
        block.render(area, buf);
        BarChart { items: crate::data::MEMORY_MB, unit: "MB", max: Some(1000.0), budget: Some(800.0) }.render(inner, buf);
    }
}

/// MaterialParams: a tech-art panel of tunable numbers (ParamRow × n).
pub struct MaterialParams {
    pub focused: usize,
}

impl Widget for MaterialParams {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = atoms::panel("Material", false);
        let inner = block.inner(area);
        block.render(area, buf);
        let rows = [
            ("Roughness", NumberField::new(0.45, "").decimals(2).range(0.0, 1.0)),
            ("Metallic", NumberField::new(0.0, "").decimals(2).range(0.0, 1.0)),
            ("Normal strength", NumberField::new(1.2, "").decimals(1).range(0.0, 4.0)),
            ("Tiling", NumberField::new(4.0, "×").range(1.0, 16.0)),
        ];
        for (i, (l, f)) in rows.into_iter().enumerate().take(inner.height as usize) {
            ParamRow { label: l, label_width: 16, field: f.focused(i == self.focused) }.render(Rect { y: inner.y + i as u16, height: 1, ..inner }, buf);
        }
    }
}

/// Sparkline row for a server: `api-1  ▂▃▅▇▆▃  42 %` (CPU, budget 80 %).
pub struct ServerLoad<'a> {
    pub name: &'a str,
    pub cpu: &'a [f64],
}

impl Widget for ServerLoad<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Name and value always fit; the sparkline takes what's left (or goes when there's no room)
        let nw = self.name.chars().count() as u16;
        let [n, s, v] = Layout::horizontal([Constraint::Length(nw), Constraint::Min(0), Constraint::Length(4)]).spacing(1).areas(Rect { height: 1, ..area });
        Line::from(atoms::label(self.name, Tone::Muted)).render(n, buf);
        Sparkline::new(self.cpu).max(100.0).budget(80.0).render(s, buf);
        let last = self.cpu.last().copied().unwrap_or(0.0);
        Line::from(atoms::label(&format!("{last:.0} %"), Tone::Text)).alignment(ratatui::layout::Alignment::Right).render(v, buf);
    }
}

// ── Security (audit page) ──

/// FindingsList: findings, worst first, with the time range on top (Segmented).
pub struct FindingsList<'a> {
    pub findings: &'a [&'a crate::data::Finding],
    pub selected: usize,
    pub focused: bool,
    pub range: usize,
}

impl Widget for FindingsList<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = atoms::panel("Findings", self.focused);
        let inner = block.inner(area);
        block.render(area, buf);
        let [top, _, list] = Layout::vertical([Constraint::Length(1), Constraint::Length(1), Constraint::Min(0)]).areas(inner);
        let seg = Segmented { options: crate::data::RANGES, active: self.range, focused: false };
        let sw = seg.width();
        let [count, r] = Layout::horizontal([Constraint::Min(0), Constraint::Length(sw)]).areas(top);
        seg.render(r, buf);
        let crit = self.findings.iter().filter(|f| f.sev <= atoms::Severity::High).count();
        Line::from(vec![
            atoms::label(&format!("{} findings  ", self.findings.len()), Tone::Text),
            atoms::status_label(&format!("{crit} high or worse"), quag_theme::Status::Err),
        ]).render(count, buf);
        if self.findings.is_empty() {
            EmptyState { message: "No findings in this range.", hint: Some(("5", "for the last 30 days")) }.render(list, buf);
            return;
        }
        for (i, f) in self.findings.iter().enumerate().take(list.height as usize) {
            let meta = format!("{} · {}", f.host, f.age);
            (&ListRow { lead: Some(atoms::severity_badge(f.sev)), title: f.title, meta: &meta, meta_width: 14, selected: i == self.selected, list_focused: self.focused, ..Default::default() })
                .render(Rect { y: list.y + i as u16, height: 1, ..list }, buf);
        }
    }
}

/// FindingDetail: the selected finding. Facts, the leaked secret (masked), and the actions.
pub struct FindingDetail<'a> {
    pub finding: Option<&'a crate::data::Finding>,
    pub revealed: bool,
    /// The focused button (Some while the zone has focus).
    pub button: Option<usize>,
}

impl FindingDetail<'_> {
    /// The buttons: a leaked key can be revoked, anything else fixed.
    pub fn buttons(f: &crate::data::Finding) -> &'static [AB<'static>] {
        const SECRET: &[AB<'static>] = &[AB::new("Revoke key", "revoke").danger(), AB::new("Ignore", "ignore")];
        const OTHER: &[AB<'static>] = &[AB::new("Fix", "save").primary(), AB::new("Ignore", "ignore")];
        if f.secret.is_some() { SECRET } else { OTHER }
    }
}

impl Widget for FindingDetail<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = atoms::panel("Details", self.button.is_some());
        let inner = block.inner(area);
        block.render(area, buf);
        let Some(f) = self.finding else {
            EmptyState { message: "Select a finding.", hint: None }.render(inner, buf);
            return;
        };
        let [sev, title, _, facts, _, det, _, sec_l, sec, rest] = Layout::vertical([
            Constraint::Length(1), Constraint::Length(1), Constraint::Length(1), Constraint::Length(3), Constraint::Length(1),
            Constraint::Length(2), Constraint::Length(1), Constraint::Length(1), Constraint::Length(1), Constraint::Min(0),
        ]).areas(inner);
        Line::from(vec![atoms::severity_badge(f.sev)]).render(sev, buf);
        Line::from(atoms::label(f.title, Tone::Title)).render(title, buf);
        PropertyList(&[("Host", f.host), ("Found", f.age), ("Status", "open")]).render(facts, buf);
        ratatui::widgets::Paragraph::new(atoms::label(f.detail, Tone::Muted)).wrap(ratatui::widgets::Wrap { trim: true }).render(det, buf);
        let mut actions = rest;
        if let Some(secret) = f.secret {
            Line::from(atoms::label("Leaked key", Tone::Muted)).render(sec_l, buf);
            SecretField::new(secret).revealed(self.revealed).render(sec, buf);
        } else {
            actions = Rect { y: sec_l.y, height: rest.bottom() - sec_l.y, ..rest };
        }
        // Actions at the bottom, but never on top of the content above (overflow: they just go last)
        let y = actions.bottom().saturating_sub(1).max(if f.secret.is_some() { sec.bottom() + 1 } else { det.bottom() + 1 });
        if y >= inner.bottom() { return; }
        let a = Rect { y, height: 1, ..inner };
        AB::group(Self::buttons(f), self.button).render(a, buf);
    }
}

/// FirewallRules: the host's rules in order, with hit counts (Table). Risky rule = allow from any on a non-web port.
pub struct FirewallRules;

impl Widget for FirewallRules {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = atoms::panel("Firewall · cache-1", false);
        let inner = block.inner(area);
        block.render(area, buf);
        let cols = [
            Column { header: "#", width: 2, right: true },
            Column { header: "Action", width: 6, right: false },
            Column { header: "Port", width: 9, right: false },
            Column { header: "Source", width: 0, right: false },
            Column { header: "Hits", width: 7, right: true },
        ];
        Table { columns: &cols, rows: crate::data::RULES, selected: Some(3), focused: true }.render(inner, buf);
    }
}
