//! UI: the pages of the demo app "pond", one per page layout (STRUCTURE.md §7.3).
//! A page picks elements and places them in zones. It never draws atoms or sets colors.
//! Every page renders into any `area`, so the explorer can show it small (preview) or full screen.

use crate::{
    data::{TASKS, Task},
    elements::{
        Activity, Attention, EditorTools, GeneralSettings, NoteEditor, NotificationSettings, ProjectNav, ProjectTable,
        FindingDetail, FindingsList, FirewallRules, ProjectTree, ServerLoad, SettingsActions, SyncLog, SyncSettings, TaskInspector, TaskList, TaskProperties, WeekStats, WelcomeMessage,
    },
};
use quag_theme::{
    self as q, Status,
    elements::{AppHeader, CommandPalette, ConfirmDialog, EmptyState, HelpOverlay, LogEntry, LogViewer, StatusBar, Toast, TypeToConfirm},
    atoms::SplitHandle,
    keymap::{Binding, Keymap},
    molecules::{KeyHints, StatusLine, Tabs, TreeRow, flatten},
    ui::{self, Zones},
};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    widgets::{Block, Clear},
};

/// State shared by the pages (what a real app would keep in its App struct).
pub struct PageState {
    pub tab: usize,
    pub focus: usize, // 0 nav, 1 content, 2 inspector
    /// tasks: the project picked in the nav (0 = All).
    pub project: usize,
    pub selected: usize,
    /// tasks: the app's tasks (x marks done, del deletes).
    pub tasks: Vec<Task>,
    /// A ConfirmDialog is open; which of its buttons has focus (0 confirm, 1 cancel).
    pub confirm_delete: bool,
    pub confirm_btn: usize,
    pub overlay: Overlay,
    /// CommandPalette: what's typed, the highlighted command.
    pub palette_query: String,
    pub palette_sel: usize,
    /// settings: the focused field (SETTINGS_FIELDS of them); dashboard: the selected project row.
    pub settings_field: usize,
    pub dash_sel: usize,
    /// The focused button in the focused zone's row of buttons (ButtonRow).
    pub btn: usize,
    /// A page asks the explorer to open another page (dashboard → tasks).
    pub goto: Option<&'static str>,
    /// Workspace: open tree ids, selected tree row, editor/output split.
    pub open: std::collections::HashSet<&'static str>,
    pub tree_sel: usize,
    pub split: f32,
    /// Log page: the big log, its filter and scroll.
    pub log: Vec<(String, Status, String)>,
    pub log_filter: String,
    pub log_follow: bool,
    pub log_scroll: usize,
    /// Settings: reminder lead time.
    pub lead: f64,
    /// Audit: selected finding, time range, secret shown, text typed in TypeToConfirm (Some = open).
    pub finding: usize,
    pub range: usize,
    pub revealed: bool,
    pub confirm_text: Option<String>,
    /// What TypeToConfirm wants typed, and what it confirms ("revoke", "deploy", "prune").
    pub confirm_name: &'static str,
    pub confirm_for: &'static str,
    /// Message of the next toast (dock pages).
    pub toast_msg: String,
    /// Animation tick from the app's clock (`motion::ticks`), never a redraw count; and the tick the sequence last played at.
    pub tick: usize,
    pub play_tick: usize,
    /// dock: containers (selection, states after actions, pending state changes), hosts, deploy, images.
    pub ct_sel: usize,
    pub ct_states: Vec<q::atoms::RunState>,
    pub ct_pending: Vec<Option<(usize, q::atoms::RunState)>>,
    pub host_sel: usize,
    pub deploy_start: Option<usize>,
    pub deploy_fail: bool,
    pub img_sel: usize,
    /// forge: curve and its editable keys, selected key; loot table, weights, item; balance unit; sequence.
    pub curve: usize,
    pub points: Vec<(f64, f64)>,
    pub key: usize,
    pub loot_table: usize,
    pub weights: Vec<Vec<f64>>,
    pub loot_sel: usize,
    pub unit_sel: usize,
    pub playhead: f64,
    pub playing: bool,
    pub clip: (usize, usize),
    /// forge graph: selected node, node positions, view pan; and how the project data stands (all forge pages).
    pub node: usize,
    pub node_at: Vec<(i32, i32)>,
    pub pan: (i32, i32),
    pub forge_load: crate::forge::Load,
    #[cfg(feature = "mascot")]
    pub quag: quag_theme::mascot::Mascot,
}

impl PageState {
    pub fn new(#[cfg(feature = "mascot")] quag: quag_theme::mascot::Mascot) -> Self {
        Self {
            tab: 0,
            focus: 1,
            project: 0,
            selected: 1,
            tasks: TASKS.to_vec(),
            confirm_delete: false,
            confirm_btn: 1,
            overlay: Overlay::None,
            palette_query: String::new(),
            palette_sel: 0,
            settings_field: 0,
            dash_sel: 1,
            btn: 0,
            goto: None,
            open: ["qt", "qt/docs"].into_iter().collect(),
            tree_sel: 4,
            split: 0.68,
            log: crate::data::big_log(crate::consts::LOG_LINES),
            log_filter: String::new(),
            log_follow: true,
            log_scroll: 0,
            lead: 15.0,
            finding: 0,
            range: 2,
            revealed: false,
            confirm_text: None,
            confirm_name: "api-1",
            confirm_for: "revoke",
            toast_msg: String::new(),
            tick: 0,
            play_tick: 0,
            ct_sel: 1,
            ct_states: crate::dock::initial_states(),
            ct_pending: vec![None; crate::dock::CONTAINERS.len()],
            host_sel: 0,
            deploy_start: None,
            deploy_fail: false,
            img_sel: 3,
            curve: 0,
            points: crate::forge::CURVES[0].points.to_vec(),
            key: 3,
            loot_table: 0,
            weights: crate::forge::initial_weights(),
            loot_sel: 3,
            unit_sel: 3,
            playhead: 12.5,
            playing: false,
            clip: (0, 2),
            node: 1,
            node_at: crate::forge::GRAPH_AT.to_vec(),
            pan: (0, 0),
            forge_load: crate::forge::Load::Ready,
            #[cfg(feature = "mascot")]
            quag,
        }
    }
    /// Opening a page starts it fresh: its default focus, nothing open over it.
    /// (The app's data, like tasks or weights, stays.)
    pub fn open_page(&mut self) {
        self.focus = 1;
        self.overlay = Overlay::None;
        self.confirm_delete = false;
        self.confirm_text = None;
        self.palette_query.clear();
        self.palette_sel = 0;
        self.settings_field = 0;
        self.btn = 0;
        self.playing = false;
        self.goto = None;
    }
    /// The focused button of zone `zone`'s buttons, while that zone has focus.
    pub fn button_in(&self, zone: usize) -> Option<usize> { (self.focus == zone).then_some(self.btn) }
    /// Ask before an action: a ConfirmDialog, focus on "cancel" when it destroys something.
    pub fn ask(&mut self, danger: bool) {
        self.confirm_delete = true;
        self.confirm_btn = if danger { 1 } else { 0 };
    }
    /// Show a toast.
    pub fn toast(&mut self, msg: &str) {
        self.toast_msg = msg.into();
        self.overlay = Overlay::Toast;
    }
    /// The tasks page's list: current tab and project.
    pub fn task_list(&self) -> Vec<&Task> {
        self.tasks.iter().filter(|t| crate::data::in_tab(t, self.tab) && crate::data::in_project(t, self.project)).collect()
    }
    pub fn tree_rows(&self) -> Vec<TreeRow<'static>> {
        flatten(crate::data::TREE, &|id| self.open.contains(id))
    }
}

/// What floats over the tasks page.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Overlay {
    #[default]
    None,
    /// ctrl+k
    Palette,
    /// ?
    Help,
    /// after an action
    Toast,
    /// (signal to the explorer: leave the page)
    Back,
}

pub type PageFn = fn(&mut Frame, Rect, &mut PageState);

pub struct PageInfo {
    pub keys: Keymap<'static>,
    pub id: &'static str,
    pub name: &'static str,
    pub layout: &'static str,
    pub render: PageFn,
}

pub const PAGES: &[PageInfo] = &[
    PageInfo { keys: TASKS_KEYS, id: "tasks", name: "TasksPage", layout: "List → Detail", render: tasks },
    PageInfo { keys: DASHBOARD_KEYS, id: "dashboard", name: "DashboardPage", layout: "Dashboard", render: dashboard },
    PageInfo { keys: WORKSPACE_KEYS, id: "workspace", name: "WorkspacePage", layout: "Workspace", render: workspace },
    PageInfo { keys: SETTINGS_KEYS, id: "settings", name: "SettingsPage", layout: "Form", render: settings },
    PageInfo { keys: LOG_KEYS, id: "log", name: "LogPage", layout: "Stream", render: log },
    PageInfo { keys: AUDIT_KEYS, id: "audit", name: "AuditPage", layout: "List → Detail", render: audit },
    PageInfo { keys: crate::dock::CONTAINERS_KEYS, id: "containers", name: "ContainersPage (dock)", layout: "List → Detail", render: crate::dock::containers },
    PageInfo { keys: crate::dock::HOSTS_KEYS, id: "hosts", name: "HostsPage (dock)", layout: "Dashboard", render: crate::dock::hosts },
    PageInfo { keys: crate::dock::DEPLOY_KEYS, id: "deploy", name: "DeployPage (dock)", layout: "Workspace", render: crate::dock::deploy },
    PageInfo { keys: crate::dock::IMAGES_KEYS, id: "images", name: "ImagesPage (dock)", layout: "List → Detail", render: crate::dock::images },
    PageInfo { keys: crate::forge::CURVES_KEYS, id: "curves", name: "CurvesPage (forge)", layout: "Workspace", render: crate::forge::curves },
    PageInfo { keys: crate::forge::LOOT_KEYS, id: "loot", name: "LootPage (forge)", layout: "List → Detail", render: crate::forge::loot },
    PageInfo { keys: crate::forge::BALANCE_KEYS, id: "balance", name: "BalancePage (forge)", layout: "Dashboard", render: crate::forge::balance },
    PageInfo { keys: crate::forge::SEQUENCE_KEYS, id: "sequence", name: "SequencePage (forge)", layout: "Timeline", render: crate::forge::sequence },
    PageInfo { keys: crate::forge::GRAPH_KEYS, id: "graph", name: "GraphPage (forge)", layout: "Canvas", render: crate::forge::graph },
    PageInfo { keys: WELCOME_KEYS, id: "welcome", name: "WelcomePage", layout: "Focus", render: welcome },
];

/// Common frame of every page: background, header, footer. Returns the shell.
fn frame(f: &mut Frame, area: Rect, zones: Zones, location: &str, tabs: Option<Tabs<'_>>, keys: Keymap<'_>, status: Option<StatusLine<'_>>) -> Option<ui::Shell> {
    frame_with(f, area, zones, "pond", None, location, tabs, keys, status)
}

/// `frame` for another app / environment (dock shows `[prod]` in the header).
#[allow(clippy::too_many_arguments)]
pub fn frame_with(f: &mut Frame, area: Rect, zones: Zones, app: &str, env: Option<q::atoms::Env>, location: &str, tabs: Option<Tabs<'_>>, keys: Keymap<'_>, status: Option<StatusLine<'_>>) -> Option<ui::Shell> {
    f.render_widget(Clear, area);
    f.render_widget(Block::new().style(q::base()), area);
    let s = ui::shell(area, zones);
    if s.too_small {
        ui::render_too_small(f, area);
        return None;
    }
    f.render_widget(AppHeader { app, location, tabs, env }, s.header);
    let hints = keys.hints();
    f.render_widget(StatusBar { hints: KeyHints(&hints), status }, s.footer);
    Some(s)
}

const TABS: &[&str] = &["1 Today", "2 Upcoming", "3 Done"];
/// Settings: the stops Tab walks through (2 general, 3 notifications, 2 sync, then the Save / Cancel buttons).
pub const SETTINGS_FIELDS: usize = 8;
/// The reminder lead: the field ← → change.
pub const FIELD_LEAD: usize = 6;
/// The Save / Cancel row.
pub const SETTINGS_ACTIONS: usize = 7;

/// List → Detail
pub fn tasks(f: &mut Frame, area: Rect, st: &mut PageState) {
    let Some(s) = frame(
        f, area, Zones { nav: true, inspector: true }, "tasks",
        Some(Tabs { names: TABS, active: st.tab }),
        TASKS_KEYS,
        Some(StatusLine { kind: Status::Ok, message: "Saved 2s ago" }),
    ) else { return };

    if let Some(nav) = s.nav {
        f.render_widget(ProjectNav { tasks: &st.tasks, selected: st.project, focused: st.focus == 0, week: (7, 12) }, nav);
    }
    // copied out, so the page state stays free (the mascot draws with it)
    let owned: Vec<Task> = st.task_list().into_iter().copied().collect();
    let list: Vec<&Task> = owned.iter().collect();
    let sel = st.selected.min(list.len().saturating_sub(1));
    let title = match st.project { 0 => crate::data::TABS[st.tab].to_string(), p => format!("{} · {}", crate::data::TABS[st.tab], crate::data::PROJECTS[p]) };
    let title = title.as_str();

    if list.is_empty() {
        // Empty state of the content zone (+ mascot in private builds)
        let block = q::atoms::panel(title, st.focus == 1);
        let inner = block.inner(s.content);
        f.render_widget(block, s.content);
        let [art, text] = EmptyState::layout(inner, cfg!(feature = "mascot"));
        #[cfg(feature = "mascot")]
        st.quag.render(f, art);
        let _ = art;
        let (message, hint) = crate::data::TAB_EMPTY[st.tab];
        f.render_widget(EmptyState { message, hint: Some(hint) }, text);
    } else {
        f.render_widget(TaskList { title, tasks: &list, selected: sel, focused: st.focus == 1, query: "" }, s.content);
    }
    if let Some(insp) = s.inspector {
        f.render_widget(TaskInspector { task: list.get(sel).copied(), focused: st.focus == 2, button: st.button_in(2) }, insp);
    }
    if st.confirm_delete && let Some(t) = list.get(sel) {
        let detail = format!("“{}” can't be restored.", t.title);
        let d = ConfirmDialog { title: "Delete task", question: "Delete this task?", detail: &detail, confirm: "Delete", cancel: "Keep", danger: true, focused: st.confirm_btn };
        let r = d.area(area);
        f.render_widget(d, r);
    }
    let msg = st.toast_msg.clone();
    overlays(f, area, st, TASKS_KEYS, &msg);
}

/// What floats over any page: command palette, help, toast. Every page calls this last,
/// with its own keymap, so `?` and `ctrl+k` work everywhere and always match the page's keys.
pub fn overlays(f: &mut Frame, area: Rect, st: &PageState, keys: Keymap<'_>, toast: &str) {
    match st.overlay {
        Overlay::None | Overlay::Back => {}
        Overlay::Palette => {
            let items = keys.commands(&st.palette_query);
            let p = CommandPalette { query: &st.palette_query, items: &items, selected: st.palette_sel.min(items.len().saturating_sub(1)) };
            let r = p.area(area);
            f.render_widget(p, r);
        }
        Overlay::Help => {
            let h = HelpOverlay { keymap: keys };
            let r = h.area(area);
            f.render_widget(h, r);
        }
        Overlay::Toast => {
            let t = Toast { kind: Status::Ok, message: if st.toast_msg.is_empty() { toast } else { &st.toast_msg } };
            let r = t.area(area);
            f.render_widget(t, r);
        }
    }
}

/// The tasks page's keys, written once: status bar hints, help (?) and command palette (ctrl+k) all read this,
/// and main.rs dispatches key presses through `TASKS_KEYS.find(..)`.
pub const TASKS_KEYS: Keymap = Keymap(&[
    Binding::new("↑ ↓", "move in the focused list (projects or tasks)", "Move").hint("move").action("move"),
    Binding::new("pgup pgdn", "previous / next project", "Move").hint("project").action("project"),
    Binding::new("tab shift+tab", "next / previous zone", "Move").hint("focus").action("focus"),
    Binding::new("1 2 3", "switch tab: today, upcoming, done", "Move").hint("tabs").action("tab"),
    Binding::new("n", "new task", "Act").command("New task").action("new"),
    Binding::new("x", "mark done / reopen", "Act").command("Mark task done").action("done"),
    Binding::new("e", "edit the task", "Act").command("Edit task").action("edit"),
    Binding::new("del", "delete (asks first)", "Act").hint("delete").command("Delete task").danger().action("delete"),
    quag_theme::keymap::BUTTONS,
    PALETTE,
    HELP,
    BACK,
]);

// The keys every page has (put them last in each keymap).
pub const PALETTE: Binding = Binding::new("ctrl+k", "commands", "App").hint("commands").action("palette");
pub const HELP: Binding = Binding::new("?", "this help", "App").hint("keys").command("Show keys").action("help");
pub const BACK: Binding = Binding::new("esc", "close / back", "App").action("back");
/// List ⇄ details, on List → Detail pages whose details have buttons.
pub const DETAILS: Binding = Binding::new("tab shift+tab", "list ⇄ details (and its buttons)", "Move").hint("focus").action("focus");

pub const DASHBOARD_KEYS: Keymap = Keymap(&[
    Binding::new("↑ ↓", "previous / next project", "Move").hint("move").action("move"),
    Binding::new("enter", "open the project's tasks", "Act").hint("open").command("Open project").action("open"),
    Binding::new("r", "refresh", "Act").command("Refresh").action("refresh"),
    HELP, PALETTE, BACK,
]);
pub const SETTINGS_KEYS: Keymap = Keymap(&[
    Binding::new("tab shift+tab", "next / previous field", "Move").hint("next field").action("focus"),
    Binding::new("← →", "change a number (Remind before)", "Edit").hint("change").action("change"),
    Binding::new("ctrl+s", "save", "Edit").command("Save settings").action("save"),
    quag_theme::keymap::BUTTONS, HELP, PALETTE, BACK,
]);
pub const LOG_KEYS: Keymap = Keymap(&[
    Binding::new("type", "filter the lines", "Filter").hint("filter"),
    Binding::new("backspace", "erase a filter letter", "Filter").action("erase"),
    Binding::new("↑ ↓", "scroll", "Move").hint("scroll").action("scroll"),
    Binding::new("end", "follow the newest line", "Move").hint("follow").command("Follow newest").action("follow"),
    HELP, PALETTE, BACK,
]);
pub const WELCOME_KEYS: Keymap = Keymap(&[
    Binding::new("n", "create a task", "Act").hint("new task").command("New task").action("new"),
    quag_theme::keymap::BUTTON, HELP, PALETTE, BACK,
]);
pub const WORKSPACE_KEYS: Keymap = Keymap(&[
    Binding::new("tab shift+tab", "files ⇄ editor", "Move").hint("focus").action("focus"),
    Binding::new("↑ ↓", "move in the file tree", "Move").action("move"),
    Binding::new("← →", "close / open a folder (← on a file: its folder)", "Move").hint("fold").action("fold"),
    Binding::new("[ ]", "resize editor / output", "View").hint("resize").command("Resize panes").action("resize"),
    Binding::new("ctrl+s", "save", "Act").command("Save note").action("save"),
    HELP, PALETTE, BACK,
]);

/// Dashboard
pub fn dashboard(f: &mut Frame, area: Rect, st: &mut PageState) {
    let Some(s) = frame(f, area, Zones { nav: true, inspector: false }, "overview", None, DASHBOARD_KEYS, Some(StatusLine { kind: Status::Ok, message: "Synced" })) else { return };
    if let Some(nav) = s.nav {
        f.render_widget(ProjectNav { tasks: &st.tasks, selected: crate::data::project_of_row(st.dash_sel), focused: false, week: (7, 12) }, nav);
    }
    let [kpis, body] = ui::dashboard(s.content);
    f.render_widget(WeekStats, kpis);
    let [cards, _, table] = Layout::vertical([Constraint::Length(body.height.min(7)), Constraint::Length(1), Constraint::Max(7)]).areas(body);
    let cards = ui::columns(cards, 2);
    f.render_widget(Attention, cards[0]);
    f.render_widget(Activity, cards[1]);
    f.render_widget(ProjectTable { selected: Some(st.dash_sel), focused: true }, table);
    overlays(f, area, st, DASHBOARD_KEYS, "Refreshed");
}

/// Form
pub fn settings(f: &mut Frame, area: Rect, st: &mut PageState) {
    let Some(s) = frame(f, area, Zones { nav: true, inspector: false }, "settings", None, SETTINGS_KEYS, None) else { return };
    if let Some(nav) = s.nav {
        f.render_widget(ProjectNav { tasks: &st.tasks, selected: usize::MAX, focused: false, week: (7, 12) }, nav);
    }
    let block = q::atoms::panel("Settings", true);
    let inner = block.inner(s.content);
    f.render_widget(block, s.content);
    let col = ui::form_column(inner);
    let [general, _, notif, _, sync, _, actions] = Layout::vertical([
        Constraint::Length(10),
        Constraint::Length(1),
        Constraint::Length(NotificationSettings::HEIGHT),
        Constraint::Length(1),
        Constraint::Length(SyncSettings::HEIGHT),
        Constraint::Length(1),
        Constraint::Length(1),
    ])
    .areas(col);
    // one focused field across the three groups: 0–1 general, 2–4 notifications, 5–6 sync
    let in_group = |from: usize, n: usize| (from..from + n).contains(&st.settings_field).then(|| st.settings_field - from);
    f.render_widget(SyncSettings { focused: in_group(5, 2), lead_minutes: st.lead }, sync);
    f.render_widget(GeneralSettings { focused: in_group(0, 2) }, general);
    f.render_widget(NotificationSettings { focused: in_group(2, 3) }, notif);
    f.render_widget(SettingsActions { focused: (st.settings_field == SETTINGS_ACTIONS).then_some(st.btn) }, actions);
    overlays(f, area, st, SETTINGS_KEYS, "Settings saved");
}

/// Stream
pub fn log(f: &mut Frame, area: Rect, st: &mut PageState) {
    let Some(s) = frame(f, area, Zones::default(), "server log", None, LOG_KEYS, Some(StatusLine { kind: Status::Info, message: "api-1 · live" })) else { return };
    let lines: Vec<LogEntry> = st.log.iter().map(|(t, k, m)| LogEntry { time: t, kind: *k, message: m }).collect();
    f.render_widget(
        LogViewer { title: "api-1.log", lines: &lines, filter: &st.log_filter, filter_focused: true, follow: st.log_follow, scroll: st.log_scroll, focused: true, only: None },
        s.content,
    );
    overlays(f, area, st, LOG_KEYS, "Following");
}

/// Focus
pub fn welcome(f: &mut Frame, area: Rect, st: &mut PageState) {
    let Some(s) = frame(f, area, Zones::default(), "", None, WELCOME_KEYS, None) else { return };
    let with_mascot = cfg!(feature = "mascot");
    let art_h = if with_mascot { 14.min(s.content.height.saturating_sub(6)) } else { 0 };
    let col = ui::focus_column(s.content, art_h + 5);
    let [art, _, text] = Layout::vertical([Constraint::Length(art_h), Constraint::Length(1), Constraint::Length(4)]).areas(col);
    #[cfg(feature = "mascot")]
    st.quag.render(f, art);
    let _ = (art, &st);
    f.render_widget(WelcomeMessage, text);
    overlays(f, area, st, WELCOME_KEYS, "Task created");
}

/// Workspace: tools strip · canvas · inspector · output
pub fn workspace(f: &mut Frame, area: Rect, st: &mut PageState) {
    let Some(s) = frame(
        f, area, Zones { nav: true, inspector: true }, "editor", None,
        WORKSPACE_KEYS,
        Some(StatusLine { kind: Status::Info, message: "Unsaved changes" }),
    ) else { return };
    if let Some(nav) = s.nav {
        let rows = st.tree_rows();
        f.render_widget(ProjectTree { rows: &rows, selected: st.tree_sel.min(rows.len().saturating_sub(1)), focused: st.focus == 0 }, nav);
    }
    let [tools, main, _] = ui::workspace(s.content, false);
    f.render_widget(EditorTools { active: 1 }, tools);
    // Canvas over output, resizable with [ ]
    let [canvas, handle, output] = ui::split(main, st.split, false, 5);
    f.render_widget(NoteEditor { focused: st.focus == 1, cursor_line: 5 }, canvas);
    f.render_widget(SplitHandle { vertical: false, focused: st.focus == 1 }, handle);
    f.render_widget(SyncLog, output);
    if let Some(insp) = s.inspector {
        f.render_widget(TaskProperties { task: &TASKS[1] }, insp);
    }
    overlays(f, area, st, WORKSPACE_KEYS, "Note saved");
}

pub const AUDIT_KEYS: Keymap = Keymap(&[
    Binding::new("↑ ↓", "previous / next finding", "Move").hint("move").action("move"),
    Binding::new("1 2 3 4 5", "time range 15m … 30d", "Move").hint("range").action("range"),
    Binding::new("s", "show / hide the secret", "Act").hint("show key").command("Show / hide secret").action("reveal"),
    Binding::new("f", "fix", "Act").command("Fix finding").action("save"),
    Binding::new("i", "ignore", "Act").command("Ignore finding").action("ignore"),
    Binding::new("x", "revoke the key (type to confirm)", "Act").hint("revoke").command("Revoke key").danger().action("revoke"),
    DETAILS, quag_theme::keymap::BUTTONS, HELP, PALETTE, BACK,
]);

/// List → Detail, security flavour: servers · findings · detail. Shows the security parts working together.
pub fn audit(f: &mut Frame, area: Rect, st: &mut PageState) {
    let Some(s) = frame(f, area, Zones { nav: true, inspector: true }, "security audit", None, AUDIT_KEYS,
        Some(StatusLine { kind: Status::Warn, message: "1 critical" })) else { return };
    if let Some(nav) = s.nav {
        let block = q::atoms::panel("Servers", false);
        let inner = block.inner(nav);
        f.render_widget(block, nav);
        let cpu: [(&str, &[f64]); 4] = [
            ("api-1", &[22.0, 30.0, 28.0, 35.0, 41.0, 38.0, 44.0, 52.0, 47.0, 42.0]),
            ("db-1", &[55.0, 62.0, 70.0, 78.0, 84.0, 91.0, 88.0, 86.0, 82.0, 87.0]),
            ("web-1", &[12.0, 14.0, 13.0, 15.0, 18.0, 16.0, 14.0, 13.0, 15.0, 14.0]),
            ("cache-1", &[8.0, 9.0, 8.0, 12.0, 10.0, 9.0, 11.0, 10.0, 9.0, 8.0]),
        ];
        for (i, (n, c)) in cpu.iter().enumerate().take(inner.height as usize) {
            f.render_widget(ServerLoad { name: n, cpu: c }, Rect { y: inner.y + i as u16, height: 1, ..inner });
        }
    }
    // Only findings inside the time range (data ages: h < 24h < d)
    let list = crate::data::findings_in(st.range);
    let sel = st.finding.min(list.len().saturating_sub(1));
    let [findings, _, rules] = Layout::vertical([Constraint::Min(8), Constraint::Length(1), Constraint::Length(8)]).areas(s.content);
    f.render_widget(FindingsList { findings: &list, selected: sel, focused: st.focus != 2, range: st.range }, if s.content.height >= 26 { findings } else { s.content });
    if s.content.height >= 26 {
        f.render_widget(FirewallRules, rules);
    }
    if let Some(insp) = s.inspector {
        f.render_widget(FindingDetail { finding: list.get(sel).copied(), revealed: st.revealed, button: st.button_in(2) }, insp);
    }
    if let Some(t) = &st.confirm_text {
        let d = TypeToConfirm { title: "Revoke key", question: "Revoke this API key? Every client using it stops working.", name: "api-1", typed: t, confirm: "Revoke" };
        let r = d.area(area);
        f.render_widget(d, r);
    }
    overlays(f, area, st, AUDIT_KEYS, "Done");
}
