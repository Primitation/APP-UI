//! Quag parts explorer: the test TUI of the atomic system.
//! Left: every part, grouped by level. Right: a live preview of the selected one.
//! UI pages preview small (they adapt to the space), and Enter opens one full screen.
//!
//!   cargo run --example explorer                     (public build)
//!   cargo run --example explorer --features mascot   (private build, with Quagsire)
//!   … -- --dump <dir>                                (headless screenshots)
//!
//! Folder = the layout of a real app:
//!   data.rs      app data
//!   elements.rs  ELEMENTS (TaskActions, TaskList, …)
//!   pages.rs     UI pages (one per layout)
//!   catalog.rs   the explorer's list of parts + previews
//! (ATOMS and MOLECULES come from quag_theme.)

mod catalog;
mod consts;
mod data;
mod dock;
mod forge;
mod elements;
mod pages;

use catalog::{Entry, Kind, Level, catalog};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use pages::{Overlay, PAGES, PageState, TASKS_KEYS};
use quag_theme::{
    self as q,
    atoms::{self, Divider, Tone},
    elements::{AppHeader, StatusBar},
    molecules::{KeyHints, ListRow},
    ui::{self, Zones},
};
use ratatui::{
    Frame, Terminal,
    backend::TestBackend,
    layout::{Constraint, Layout, Rect},
    style::Modifier,
    text::{Line, Span},
    widgets::{Block, Paragraph, Wrap},
};

/// The explorer's own keys.
const EXPLORER_KEYS: quag_theme::keymap::Keymap = quag_theme::keymap::Keymap(&[
    quag_theme::keymap::Binding::new("↑ ↓", "browse the parts", "Move").hint("browse"),
    quag_theme::keymap::Binding::new("pgup pgdn", "previous / next level", "Move").hint("level"),
    quag_theme::keymap::Binding::new("home end", "first / last part", "Move"),
    quag_theme::keymap::Binding::new("/", "filter the parts by name", "Move").hint("filter"),
    quag_theme::keymap::Binding::new("enter", "open the UI page full screen", "Move").hint("open page"),
    quag_theme::keymap::Binding::new("q esc", "quit (esc clears the filter first)", "App").hint("quit"),
]);

struct Explorer {
    entries: Vec<Entry>,
    selected: usize,
    /// Some(page index) = a UI page is open full screen.
    full: Option<usize>,
    page: PageState,
    /// The parts filter (`/`), and whether keys type into it.
    filter: String,
    filtering: bool,
    /// When the current toast appeared (it closes by itself).
    toast_at: std::time::Instant,
}

impl Explorer {
    /// Entry indices in display order (grouped by level), only those matching the filter.
    fn order(&self) -> Vec<usize> {
        let q = self.filter.to_lowercase();
        Level::ALL.iter()
            .flat_map(|lv| self.entries.iter().enumerate().filter(move |(_, e)| e.level == *lv).map(|(i, _)| i))
            .filter(|&i| q.is_empty() || self.entries[i].name.to_lowercase().contains(&q))
            .collect()
    }
    /// Keys of the parts list.
    fn list_key(&mut self, k: crossterm::event::KeyEvent) -> bool {
        if self.filtering {
            match k.code {
                KeyCode::Char(c) => self.filter.push(c),
                KeyCode::Backspace => { self.filter.pop(); }
                KeyCode::Esc => { self.filter.clear(); self.filtering = false; }
                KeyCode::Enter | KeyCode::Down | KeyCode::Up => self.filtering = false,
                _ => {}
            }
            // keep the selection on a matching part
            let order = self.order();
            if !order.contains(&self.selected) && let Some(&first) = order.first() { self.selected = first; }
            return true;
        }
        let order = self.order();
        let n = order.len();
        let at = order.iter().position(|&i| i == self.selected).unwrap_or(0);
        let level = |i: usize| self.entries[i].level;
        let to = match k.code {
            KeyCode::Char('q') => return false,
            KeyCode::Esc if self.filter.is_empty() => return false,
            KeyCode::Esc => { self.filter.clear(); Some(at) }
            KeyCode::Char('/') => { self.filtering = true; None }
            KeyCode::Down | KeyCode::Char('j') if n > 0 => Some((at + 1) % n),
            KeyCode::Up | KeyCode::Char('k') if n > 0 => Some((at + n - 1) % n),
            KeyCode::Home => Some(0),
            KeyCode::End => Some(n.saturating_sub(1)),
            // first part of the next / previous level
            KeyCode::PageDown => order.iter().position(|&i| level(i) != level(self.selected) && order.iter().position(|&j| j == i) > Some(at)).or(Some(at)),
            KeyCode::PageUp => {
                let lv = order.get(at).map(|&i| level(i));
                let prev = order[..at].iter().rev().find(|&&i| Some(level(i)) != lv).map(|&i| level(i));
                prev.and_then(|p| order.iter().position(|&i| level(i) == p)).or(Some(at))
            }
            KeyCode::Enter => {
                if let Kind::Page(p) = self.entries[self.selected].kind { self.page.open_page(); self.full = Some(p) }
                None
            }
            _ => None,
        };
        if let Some(t) = to && let Some(&i) = order.get(t) { self.selected = i; }
        true
    }
    fn new(#[cfg(feature = "mascot")] picker: ratatui_image::picker::Picker) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            entries: catalog(),
            selected: 0,
            full: None,
            filter: String::new(),
            filtering: false,
            toast_at: std::time::Instant::now(),
            page: PageState::new(
                #[cfg(feature = "mascot")]
                quag_theme::mascot::Mascot::new(picker)?,
            ),
        })
    }
}

// ── the explorer is itself a UI page, built from shared elements ──
fn draw(f: &mut Frame, ex: &mut Explorer) {
    if let Some(p) = ex.full {
        return (PAGES[p].render)(f, f.area(), &mut ex.page);
    }
    f.render_widget(Block::new().style(q::base()), f.area());
    let s = ui::shell(f.area(), Zones { nav: true, inspector: false });
    if s.too_small {
        return ui::render_too_small(f, f.area());
    }
    let entry = &ex.entries[ex.selected];
    let is_page = matches!(entry.kind, Kind::Page(_));
    f.render_widget(AppHeader { app: "Quag", location: "parts explorer", tabs: None, env: None }, s.header);
    let hints: Vec<_> = EXPLORER_KEYS.hints().into_iter().filter(|(k, _)| is_page || *k != "enter").collect();
    f.render_widget(StatusBar { hints: KeyHints(&hints), status: None }, s.footer);

    // Nav: wider than the default so part names fit
    let nav_w = 32.min(f.area().width / 3);
    let [nav, content] = Layout::horizontal([Constraint::Length(nav_w), Constraint::Min(0)])
        .areas(Rect { y: s.content.y, height: s.content.height, x: f.area().x, width: f.area().width });
    parts_list(f, nav, ex);
    preview(f, content, ex);
}

enum NavRow {
    Header(Level),
    Item(usize),
}

fn parts_list(f: &mut Frame, area: Rect, ex: &Explorer) {
    let block = atoms::panel("Parts", true);
    let mut inner = block.inner(area);
    f.render_widget(block, area);
    // the filter (`/`) sits on top while it's set
    if ex.filtering || !ex.filter.is_empty() {
        let [bar, _, rest] = Layout::vertical([Constraint::Length(1), Constraint::Length(1), Constraint::Min(0)]).areas(inner);
        f.render_widget(quag_theme::molecules::SearchBar { query: &ex.filter, focused: ex.filtering }, bar);
        inner = rest;
    }
    let order = ex.order();
    if order.is_empty() {
        f.render_widget(Paragraph::new(Line::from(atoms::label("No part matches.", Tone::Faint))), inner);
        return;
    }
    let mut rows = Vec::new();
    for lv in Level::ALL {
        let items: Vec<_> = order.iter().filter(|&&i| ex.entries[i].level == lv).collect();
        if items.is_empty() { continue; }
        if !rows.is_empty() {
            rows.push(None);
        }
        rows.push(Some(NavRow::Header(lv)));
        rows.extend(items.into_iter().map(|&i| Some(NavRow::Item(i))));
    }
    // Tight: drop the blank lines between levels before scrolling
    if rows.len() > inner.height as usize {
        rows.retain(|r| r.is_some());
    }
    // Scroll so the selected row stays visible
    let sel_row = rows.iter().position(|r| matches!(r, Some(NavRow::Item(i)) if *i == ex.selected)).unwrap_or(0);
    let h = inner.height as usize;
    let offset = sel_row.saturating_sub(h.saturating_sub(3));
    for (y, row) in rows.iter().skip(offset).take(h).enumerate() {
        let r = Rect { y: inner.y + y as u16, height: 1, ..inner };
        match row {
            Some(NavRow::Header(lv)) => f.render_widget(Paragraph::new(Line::from(atoms::label(lv.title(), Tone::Heading))), r),
            Some(NavRow::Item(i)) => f.render_widget(
                &ListRow { title: ex.entries[*i].name, selected: *i == ex.selected, list_focused: true, ..Default::default() },
                r,
            ),
            None => {}
        }
    }
}

fn preview(f: &mut Frame, area: Rect, ex: &mut Explorer) {
    let entry = &ex.entries[ex.selected];
    let block = atoms::panel(entry.name, false);
    let inner = block.inner(area);
    f.render_widget(block, area);
    let [info, _, line, _, stage] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Length(0),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Min(0),
    ])
    .areas(inner);
    let mut lines = vec![
        Line::from(vec![
            atoms::tag(match entry.level { Level::Atom => "atom", Level::Molecule => "molecule", Level::Element => "element", Level::Ui => "ui" }),
            Span::raw("  "),
            atoms::label(entry.level.rule(), Tone::Faint),
        ]),
        Line::from(atoms::label(entry.what, Tone::Muted)),
    ];
    if !entry.from.is_empty() {
        lines.push(Line::from(vec![atoms::label("built from  ", Tone::Faint), atoms::label(entry.from, Tone::Heading)]));
    }
    f.render_widget(Paragraph::new(lines).wrap(Wrap { trim: true }), info);
    f.render_widget(Divider, line);

    match entry.kind {
        Kind::Part(render) => render(f, stage, &mut ex.page),
        Kind::Page(p) => {
            // The page itself, live, at preview size: it adapts with its own breakpoints.
            let [page, _, hint] = Layout::vertical([Constraint::Min(0), Constraint::Length(1), Constraint::Length(1)]).areas(stage);
            (PAGES[p].render)(f, page, &mut ex.page);
            f.render_widget(
                Paragraph::new(Line::from(vec![
                    atoms::label("live preview at this size · ", Tone::Faint),
                    atoms::kbd("enter"),
                    atoms::label(" opens it full screen", Tone::Faint),
                ])),
                hint,
            );
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if let Some(i) = args.iter().position(|a| a == "--dump") {
        return dump(&args[i + 1]);
    }
    #[cfg(feature = "mascot")]
    let picker = quag_theme::mascot::picker();
    let mut ex = Explorer::new(
        #[cfg(feature = "mascot")]
        picker,
    )?;
    let mut terminal = ratatui::init();
    let clock = std::time::Instant::now();
    let res = (|| -> std::io::Result<()> {
        loop {
            ex.page.tick = quag_theme::consts::motion::ticks(clock.elapsed());
            terminal.draw(|f| draw(f, &mut ex))?;
            #[cfg(feature = "mascot")]
            let timeout = ex.page.quag.next_frame_in().min(consts::FRAME);
            #[cfg(not(feature = "mascot"))]
            let timeout = consts::FRAME;
            // a toast closes by itself
            if ex.page.overlay == Overlay::Toast && ex.toast_at.elapsed() >= consts::TOAST { ex.page.overlay = Overlay::None; }
            if !event::poll(timeout)? { continue; }
            let Event::Key(k) = event::read()? else { continue };
            if k.kind != KeyEventKind::Press { continue; }
            if let Some(p) = ex.full {
                page_key(&mut ex.page, PAGES[p].id, k);
                // any toast still up after a key is a new one
                if ex.page.overlay == Overlay::Toast { ex.toast_at = std::time::Instant::now(); }
                if ex.page.overlay == Overlay::Back { ex.page.overlay = Overlay::None; ex.full = None; }
                // a page opens another (dashboard → tasks)
                if let Some(id) = ex.page.goto.take() && let Some(i) = PAGES.iter().position(|p| p.id == id) {
                    let project = ex.page.project;
                    ex.page.open_page();
                    ex.page.project = project;
                    ex.page.selected = 0;
                    ex.full = Some(i);
                }
                continue;
            }
            if !ex.list_key(k) { return Ok(()); }
        }
    })();
    ratatui::restore();
    Ok(res?)
}

/// Display name of a key press, as written in keymaps ("ctrl+k", "?", "↓", "esc").
fn key_name(k: &crossterm::event::KeyEvent) -> String {
    let base = match k.code {
        KeyCode::Char(' ') => "space".into(),
        KeyCode::Char(c) => c.to_string(),
        KeyCode::Up => "↑".into(),
        KeyCode::Down => "↓".into(),
        KeyCode::Left => "←".into(),
        KeyCode::Right => "→".into(),
        KeyCode::Tab => "tab".into(),
        KeyCode::Esc => "esc".into(),
        KeyCode::Enter => "enter".into(),
        KeyCode::End => "end".into(),
        KeyCode::Home => "home".into(),
        KeyCode::PageUp => "pgup".into(),
        KeyCode::PageDown => "pgdn".into(),
        KeyCode::BackTab => "shift+tab".into(),
        KeyCode::Backspace => "backspace".into(),
        KeyCode::Delete => "del".into(),
        _ => String::new(),
    };
    let arrow = matches!(k.code, KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right);
    if k.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) { format!("ctrl+{base}") }
    else if arrow && k.modifiers.contains(crossterm::event::KeyModifiers::SHIFT) { format!("shift+{base}") }
    else { base }
}

/// One step down (or up) in a list of `n`, staying inside it.
fn step(i: usize, down: bool, n: usize) -> usize { if down { (i + 1).min(n.saturating_sub(1)) } else { i.saturating_sub(1) } }

/// Keys inside an open page: everything goes through the page's keymap (the same list its
/// status bar, help and palette show). Only typing into the log filter is free text.
/// Whatever is open on top owns the keys first: TypeToConfirm, ConfirmDialog, palette, help.
fn page_key(st: &mut PageState, page: &str, k: crossterm::event::KeyEvent) {
    let name = key_name(&k);
    let keys = PAGES.iter().find(|p| p.id == page).map(|p| p.keys).unwrap_or(TASKS_KEYS);
    // TypeToConfirm: type, backspace, enter (only when it matches), esc
    if let Some(t) = st.confirm_text.as_mut() {
        match k.code {
            KeyCode::Char(c) => t.push(c),
            KeyCode::Backspace => { t.pop(); }
            KeyCode::Enter if t == st.confirm_name => {
                st.confirm_text = None;
                match st.confirm_for {
                    "deploy" => st.deploy_start = Some(st.tick),
                    "prune" => st.toast("Pruned 4.6 GiB"),
                    _ => st.toast("Key revoked"),
                }
            }
            KeyCode::Esc => st.confirm_text = None,
            _ => {}
        }
        return;
    }
    // ConfirmDialog: ← → / tab move between the buttons, enter presses the focused one, esc cancels
    if st.confirm_delete {
        match k.code {
            KeyCode::Left | KeyCode::Right | KeyCode::Tab | KeyCode::BackTab => st.confirm_btn = 1 - st.confirm_btn,
            KeyCode::Enter => {
                st.confirm_delete = false;
                if st.confirm_btn == 0 { confirmed(st, page) }
            }
            KeyCode::Esc => st.confirm_delete = false,
            _ => {}
        }
        return;
    }
    match st.overlay {
        Overlay::Help => {
            if matches!(k.code, KeyCode::Esc | KeyCode::Char('?')) { st.overlay = Overlay::None }
            return;
        }
        Overlay::Palette => return palette_key(st, page, keys, k),
        // a toast never blocks: it closes and the key still works
        Overlay::Toast => st.overlay = Overlay::None,
        _ => {}
    }
    // a row of buttons with focus: ← → pick one, enter presses it (= its key: same action, same code)
    if let Some(actions) = button_row(st, page) {
        let row = quag_theme::keymap::ButtonRow(actions);
        match name.as_str() {
            "←" | "→" => { st.btn = row.step(st.btn, &name); return; }
            "enter" => {
                if let Some(a) = row.press(st.btn) {
                    let key = keys.0.iter().find(|b| b.action == a).and_then(|b| b.key.split(' ').next()).unwrap_or("");
                    act(st, page, a, key, None);
                }
                return;
            }
            _ => {}
        }
    }
    let action = keys.find(&name).map(|b| b.action).unwrap_or("");
    let typed = if let KeyCode::Char(c) = k.code && k.modifiers.difference(crossterm::event::KeyModifiers::SHIFT).is_empty() { Some(c) } else { None };
    act(st, page, action, &name, typed);
}

/// The buttons the keyboard is on, if any: the focused zone's row (each button = an action id of the page's keymap).
/// The same tables the elements draw from, so what's pressed is what's shown.
fn button_row(st: &PageState, page: &str) -> Option<&'static [quag_theme::molecules::ActionButton<'static>]> {
    use elements::{FindingDetail, SettingsActions, TaskActions, WelcomeMessage};
    match page {
        "tasks" if st.focus == 2 => st.task_list().get(st.selected).map(|t| TaskActions::buttons(t)),
        "audit" if st.focus == 2 => data::findings_in(st.range).get(st.finding).map(|f| FindingDetail::buttons(f)),
        "containers" if st.focus == 2 => Some(dock::ContainerInspector::buttons(st.ct_states[st.ct_sel.min(dock::CONTAINERS.len() - 1)].is_up())),
        "images" if st.focus == 2 => Some(dock::DiskUse::BUTTONS),
        "curves" if st.focus == 2 => Some(forge::KeyInspector::BUTTONS),
        "settings" if st.settings_field == pages::SETTINGS_ACTIONS => Some(SettingsActions::BUTTONS),
        "welcome" => Some(WelcomeMessage::BUTTONS),
        _ => None,
    }
}

/// CommandPalette: type to filter, ↑ ↓ pick, enter runs it (like pressing its key), esc closes.
fn palette_key(st: &mut PageState, page: &str, keys: quag_theme::keymap::Keymap<'static>, k: crossterm::event::KeyEvent) {
    let items = keys.commands(&st.palette_query);
    match k.code {
        KeyCode::Esc => st.overlay = Overlay::None,
        KeyCode::Down => st.palette_sel = step(st.palette_sel, true, items.len()),
        KeyCode::Up => st.palette_sel = step(st.palette_sel, false, items.len()),
        KeyCode::Backspace => { st.palette_query.pop(); st.palette_sel = 0; }
        KeyCode::Enter => {
            st.overlay = Overlay::None;
            let chosen = items.get(st.palette_sel).and_then(|m| keys.0.iter().find(|b| b.command == m.label));
            if let Some(b) = chosen {
                // run it as its first key would
                act(st, page, b.action, b.key.split(' ').next().unwrap_or(""), None);
            }
        }
        KeyCode::Char(c) if !k.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) => { st.palette_query.push(c); st.palette_sel = 0; }
        _ => {}
    }
}

/// A ConfirmDialog was confirmed.
fn confirmed(st: &mut PageState, page: &str) {
    if dock::confirmed(st, page) { return; }
    if page == "tasks" {
        let list = st.task_list();
        if let Some(t) = list.get(st.selected.min(list.len().saturating_sub(1))) {
            let title = t.title;
            st.tasks.retain(|x| x.title != title);
            st.toast("Task deleted");
        }
    }
}

/// What a page action does. `name` is the key as written in keymaps; `typed` the character, for free text.
fn act(st: &mut PageState, page: &str, action: &str, name: &str, typed: Option<char>) {
    act_on(st, page, action, name, typed);
    // a task that left the list (done, deleted, other project) never leaves the selection outside it
    st.selected = st.selected.min(st.task_list().len().saturating_sub(1));
}

fn act_on(st: &mut PageState, page: &str, action: &str, name: &str, typed: Option<char>) {
    let rows = st.tree_rows();
    if dock::key(st, page, action, name) || forge::key(st, page, action, name) { return; }
    let back = name == "shift+tab";
    match (page, action) {
        // every page
        (_, "help") => st.overlay = Overlay::Help,
        (_, "palette") => { st.palette_query.clear(); st.palette_sel = 0; st.overlay = Overlay::Palette }
        (_, "back") => st.overlay = Overlay::Back,
        ("audit", "save") => st.toast("Fix scheduled"),
        ("settings" | "workspace", "save") => st.toast("Saved"),
        ("dashboard", "refresh") => st.toast("Refreshed"),
        ("tasks" | "welcome", "new") => st.toast("New task (demo)"),
        // tasks
        // ↑ ↓ move in the focused list: the projects (nav) or the tasks
        ("tasks", "move") if st.focus == 0 => { st.project = step(st.project, name == "↓", data::PROJECTS.len()); st.selected = 0; }
        ("tasks", "move") => st.selected = step(st.selected, name == "↓", st.task_list().len()),
        ("tasks", "project") => { st.project = step(st.project, name == "pgdn", data::PROJECTS.len()); st.selected = 0; }
        ("tasks", "focus") => { st.focus = (st.focus + if back { 2 } else { 1 }) % 3; st.btn = 0; }
        ("tasks", "edit") => st.toast("Edit task (demo)"),
        // list ⇄ details (and its buttons)
        ("containers" | "images" | "audit" | "curves", "focus") => { st.focus = if st.focus == 2 { 1 } else { 2 }; st.btn = 0; }
        ("settings", "cancel") => { st.settings_field = 0; st.toast("Changes discarded"); }
        ("tasks", "tab") => { st.tab = name.parse::<usize>().unwrap_or(1) - 1; st.selected = 0; }
        ("tasks", "done") => {
            let list = st.task_list();
            if let Some(t) = list.get(st.selected.min(list.len().saturating_sub(1))) {
                let title = t.title;
                let done = !t.done;
                if let Some(x) = st.tasks.iter_mut().find(|x| x.title == title) { x.done = done; }
                st.toast(if done { "Task marked done" } else { "Task reopened" });
            }
        }
        // only asks when there's a task to delete
        ("tasks", "delete") => if !st.task_list().is_empty() { st.ask(true) },
        // dashboard
        ("dashboard", "move") => st.dash_sel = step(st.dash_sel, name == "↓", data::PROJECT_ROWS.len()),
        ("dashboard", "open") => { st.project = data::project_of_row(st.dash_sel); st.tab = 0; st.goto = Some("tasks"); }
        // log
        ("log", "scroll") if name == "↑" => { st.log_follow = false; st.log_scroll += 1 }
        ("log", "scroll") => { st.log_scroll = st.log_scroll.saturating_sub(1); st.log_follow = st.log_scroll == 0 }
        ("log", "follow") => { st.log_follow = true; st.log_scroll = 0 }
        ("log", "erase") => { st.log_filter.pop(); }
        ("log", "") => if let Some(c) = typed { st.log_filter.push(c) },
        // workspace
        ("workspace", "focus") => st.focus = if st.focus == 0 { 1 } else { 0 },
        ("workspace", "resize") => st.split = (st.split + if name == "[" { -quag_theme::ui::SPLIT_STEP } else { quag_theme::ui::SPLIT_STEP }).clamp(consts::SPLIT_MIN, consts::SPLIT_MAX),
        ("workspace", "move") => st.tree_sel = step(st.tree_sel, name == "↓", rows.len()),
        // like any file tree: → opens a folder (then goes into it), ← closes it (or goes up to the parent)
        ("workspace", "fold") => if let Some(r) = rows.get(st.tree_sel) {
            match (name, r.open) {
                ("→", Some(false)) => { st.open.insert(r.item.id); }
                ("→", Some(true)) => st.tree_sel = step(st.tree_sel, true, rows.len()),
                ("←", Some(true)) => { st.open.remove(r.item.id); }
                ("←", _) => if let Some(p) = rows[..st.tree_sel].iter().rposition(|p| p.depth < r.depth) { st.tree_sel = p },
                _ => {}
            }
        },
        // audit
        ("audit", "move") => st.finding = step(st.finding, name == "↓", data::findings_in(st.range).len()),
        ("audit", "range") => { st.range = name.parse::<usize>().unwrap_or(3) - 1; st.finding = 0 }
        ("audit", "reveal") => st.revealed = !st.revealed,
        ("audit", "ignore") => st.toast("Ignored"),
        ("audit", "revoke") => { st.confirm_text = Some(String::new()); st.confirm_name = "api-1"; st.confirm_for = "revoke"; }
        // settings
        ("settings", "focus") => { st.settings_field = (st.settings_field + if back { pages::SETTINGS_FIELDS - 1 } else { 1 }) % pages::SETTINGS_FIELDS; st.btn = 0; }
        ("settings", "change") if st.settings_field == pages::FIELD_LEAD =>
            st.lead = (st.lead + if name == "→" { consts::LEAD_STEP_MIN } else { -consts::LEAD_STEP_MIN }).clamp(consts::LEAD_MIN, consts::LEAD_MAX),
        _ => {}
    }
}

/// Headless screenshots: the explorer on a few parts, and every page full screen.
fn dump(dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all(dir)?;
    let mk = || Explorer::new(
        #[cfg(feature = "mascot")]
        {
            let mut p = ratatui_image::picker::Picker::halfblocks();
            p.set_background_color(Some(image::Rgba([0x0E, 0x16, 0x20, 0xFF])));
            p
        },
    );
    let mut shots: Vec<(String, Option<usize>, Option<usize>, Overlay)> = Vec::new();
    let ex0 = mk()?;
    for id in ["button", "buttongroup", "taskactions", "tasklist", "emptystate", "tasks", "radio", "badge", "skeleton", "progressbars", "radiogroup", "breadcrumbs", "menu", "table", "proplist", "editor", "projecttable", "errorstate", "toast", "palette", "help", "number", "select", "charts", "split", "tree", "paramrow", "barchart", "logviewer", "perf", "material", "activity", "severity", "secret", "segmented", "typeconfirm", "findings", "runstate", "meter", "steps", "diff", "runprogress", "containers", "hostcards", "curve", "weights", "timeline", "balance", "nodegraph"] {
        let i = ex0.entries.iter().position(|e| e.id == id).unwrap();
        shots.push((format!("explorer-{id}"), Some(i), None, Overlay::None));
    }
    for (i, p) in PAGES.iter().enumerate() {
        shots.push((format!("page-{}", p.id), None, Some(i), Overlay::None));
    }
    for (n, o) in [("palette", Overlay::Palette), ("help", Overlay::Help), ("toast", Overlay::Toast)] {
        shots.push((format!("page-tasks-{n}"), None, Some(0), o));
    }
    let audit = PAGES.iter().position(|p| p.id == "audit").unwrap();
    shots.push(("page-audit-help".into(), None, Some(audit), Overlay::Help));
    shots.push(("page-audit-revoke".into(), None, Some(audit), Overlay::None));
    let deploy = PAGES.iter().position(|p| p.id == "deploy").unwrap();
    for n in ["page-deploy-confirm", "page-deploy-running", "page-deploy-failed"] { shots.push((n.into(), None, Some(deploy), Overlay::None)); }
    shots.push(("page-containers-help".into(), None, PAGES.iter().position(|p| p.id == "containers"), Overlay::Help));
    for n in ["page-balance-loading", "page-balance-failed"] { shots.push((n.into(), None, PAGES.iter().position(|p| p.id == "balance"), Overlay::None)); }
    shots.push(("page-graph-help".into(), None, PAGES.iter().position(|p| p.id == "graph"), Overlay::Help));
    shots.push(("page-tasks-buttons".into(), None, Some(0), Overlay::None));
    shots.push(("page-containers-buttons".into(), None, PAGES.iter().position(|p| p.id == "containers"), Overlay::None));
    for (name, sel, full, overlay) in shots {
        let mut ex = mk()?;
        if let Some(s) = sel { ex.selected = s; }
        ex.full = full;
        ex.page.overlay = overlay;
        if name.ends_with("revoke") { ex.page.revealed = true; ex.page.confirm_text = Some("api".into()); }
        if name.ends_with("deploy-confirm") { ex.page.confirm_text = Some("sh".into()); ex.page.confirm_name = "shop"; }
        if name.ends_with("deploy-running") { ex.page.deploy_start = Some(0); ex.page.tick = 2 * consts::DEPLOY_TICKS_PER_STEP + 3; }
        // the keyboard on a row of buttons: the second one focused
        if name.ends_with("-buttons") { ex.page.focus = 2; ex.page.btn = 1; }
        if name.ends_with("progressbars") { ex.page.tick = 37; } // mid-run: 37 %, step 3 of 5
        if name.ends_with("balance-loading") { ex.page.forge_load = forge::Load::Loading; }
        if name.ends_with("balance-failed") { ex.page.forge_load = forge::Load::Failed; }
        if name.ends_with("deploy-failed") { ex.page.deploy_start = Some(0); ex.page.tick = (consts::DEPLOY_FAIL_STEP + 2) * consts::DEPLOY_TICKS_PER_STEP; ex.page.deploy_fail = true; }
        let mut t = Terminal::new(TestBackend::new(140, 38))?;
        t.draw(|f| draw(f, &mut ex))?;
        let buf = t.backend().buffer();
        let mut out = String::new();
        for y in 0..buf.area.height {
            for x in 0..buf.area.width {
                let c = &buf[(x, y)];
                out.push_str(&format!(
                    "{x}\t{y}\t{}\t{:?}\t{:?}\t{}\t{}\n",
                    c.symbol(), c.fg, c.bg,
                    c.modifier.contains(Modifier::BOLD) as u8,
                    c.modifier.contains(Modifier::UNDERLINED) as u8
                ));
            }
        }
        std::fs::write(format!("{dir}/{name}.cells"), out)?;
    }
    Ok(())
}

/// The explorer's navigation, driven through the real key handlers (`cargo test --examples`).
/// Every page, every key its keymap declares: if it's in the help, it must do something.
#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyEvent, KeyModifiers};
    use ratatui::buffer::Buffer;

    const W: u16 = 160;
    const H: u16 = 50;

    fn state() -> PageState { PageState::new() }

    /// A key press from its keymap name ("ctrl+k", "shift+←", "pgdn", "x").
    fn key(name: &str) -> KeyEvent {
        let (mods, base) = match name.split_once('+') {
            Some(("ctrl", b)) if !b.is_empty() => (KeyModifiers::CONTROL, b),
            Some(("shift", b)) if !b.is_empty() => (KeyModifiers::SHIFT, b),
            _ => (KeyModifiers::NONE, name),
        };
        let code = match base {
            "↑" => KeyCode::Up, "↓" => KeyCode::Down, "←" => KeyCode::Left, "→" => KeyCode::Right,
            "tab" if mods == KeyModifiers::SHIFT => KeyCode::BackTab,
            "tab" => KeyCode::Tab, "esc" => KeyCode::Esc, "enter" => KeyCode::Enter, "end" => KeyCode::End,
            "home" => KeyCode::Home, "pgup" => KeyCode::PageUp, "pgdn" => KeyCode::PageDown,
            "backspace" => KeyCode::Backspace, "del" => KeyCode::Delete, "space" => KeyCode::Char(' '),
            c => KeyCode::Char(c.chars().next().expect("a key")),
        };
        KeyEvent::new(code, mods)
    }

    fn render(st: &mut PageState, page: usize) -> Buffer {
        let mut t = Terminal::new(TestBackend::new(W, H)).unwrap();
        t.draw(|f| (PAGES[page].render)(f, f.area(), st)).unwrap();
        t.backend().buffer().clone()
    }

    /// Known starting points some keys need (documented, not hidden).
    fn setup(st: &mut PageState, page: &str) {
        match page {
            "settings" => st.settings_field = pages::FIELD_LEAD, // ← → change the number field
            "log" => { st.log_filter = "up".into(); st.log_follow = false; st.log_scroll = 3; } // backspace, end
            _ => {}
        }
    }

    /// Put the keyboard on the page's row of buttons (if it has one).
    fn on_buttons(st: &mut PageState, page: &str) {
        match page {
            "settings" => st.settings_field = pages::SETTINGS_ACTIONS,
            "tasks" | "audit" | "containers" | "images" | "curves" => st.focus = 2,
            _ => {}
        }
    }

    /// Did pressing `name` change anything: the screen, an overlay, or a page switch?
    fn changes(page: usize, before: &[&str], name: &str, buttons: bool) -> bool {
        let id = PAGES[page].id;
        let mut st = state();
        st.open_page();
        setup(&mut st, id);
        if buttons { on_buttons(&mut st, id); }
        for b in before { page_key(&mut st, id, key(b)); st.overlay = Overlay::None; }
        let a = render(&mut st, page);
        let overlay = st.overlay;
        page_key(&mut st, id, key(name));
        let b = render(&mut st, page);
        a != b || st.overlay != overlay || st.goto.is_some()
    }

    #[test]
    fn every_declared_key_does_something() {
        let mut dead = Vec::new();
        for (p, info) in PAGES.iter().enumerate() {
            for b in info.keys.0 {
                if b.key == "type" { continue; } // free text, tested in `log_filter_types`
                let alts: Vec<&str> = b.key.split(' ').collect();
                for &a in &alts {
                    // from the start, or after its partner keys (↑ needs a ↓ first at the top of a list)
                    let others: Vec<&str> = alts.iter().copied().filter(|o| *o != a).collect();
                    // also with the keyboard on the page's buttons (← → enter work there)
                    let any = [false, true].iter().any(|&b| changes(p, &[], a, b) || changes(p, &others, a, b));
                    if !any { dead.push(format!("{} {a}", info.id)); }
                }
            }
        }
        assert!(dead.is_empty(), "declared keys that do nothing: {dead:?}");
    }

    /// Enter on a focused button does exactly what its key does: same action, same state after.
    #[test]
    fn a_button_does_what_its_key_does() {
        use std::fmt::Write;
        let fingerprint = |st: &mut PageState, p: usize| {
            let mut s = String::new();
            let _ = write!(s, "{:?}{}{:?}{:?}{}{}", st.overlay, st.toast_msg, st.confirm_text, st.ct_states, st.confirm_delete, st.points.len());
            let _ = write!(s, "{}", st.tasks.iter().filter(|t| t.done).count());
            let _ = p;
            s
        };
        for (p, info) in PAGES.iter().enumerate() {
            let mut st = state();
            st.open_page();
            on_buttons(&mut st, info.id);
            let Some(actions) = button_row(&st, info.id) else { continue };
            for (i, action) in actions.iter().map(|b| b.action).enumerate() {
                // by the button: ← → to it, enter
                let mut by_button = state();
                by_button.open_page();
                on_buttons(&mut by_button, info.id);
                for _ in 0..i { page_key(&mut by_button, info.id, key("→")); }
                assert_eq!(by_button.btn, i);
                page_key(&mut by_button, info.id, key("enter"));
                // by its key (or the palette's action, for a button without one)
                let mut by_key = state();
                by_key.open_page();
                on_buttons(&mut by_key, info.id);
                match info.keys.0.iter().find(|b| b.action == action) {
                    Some(b) => page_key(&mut by_key, info.id, key(b.key.split(' ').next().unwrap())),
                    None => act(&mut by_key, info.id, action, "", None),
                }
                assert_eq!(fingerprint(&mut by_button, p), fingerprint(&mut by_key, p), "{} button {i} ({action}) ≠ its key", info.id);
            }
        }
    }

    #[test]
    fn tab_reaches_the_buttons() {
        let mut st = state();
        st.open_page();
        for _ in 0..pages::SETTINGS_ACTIONS { page_key(&mut st, "settings", key("tab")); }
        assert_eq!(button_row(&st, "settings").map(|r| r.iter().map(|b| b.label).collect::<Vec<_>>()), Some(vec!["Save", "Cancel"]));
        page_key(&mut st, "settings", key("→"));
        page_key(&mut st, "settings", key("enter"));
        assert_eq!(st.toast_msg, "Changes discarded", "→ enter pressed Cancel");
        let mut st = state();
        st.open_page();
        page_key(&mut st, "containers", key("tab"));
        assert!(button_row(&st, "containers").is_some());
        let sel = st.ct_sel;
        page_key(&mut st, "containers", key("enter"));
        assert_eq!(st.ct_states[sel], q::atoms::RunState::Restarting, "enter on the first button: Restart");
    }

    #[test]
    fn esc_closes_what_is_open_before_leaving() {
        for (p, info) in PAGES.iter().enumerate() {
            for opener in ["?", "ctrl+k"] {
                let mut st = state();
                st.open_page();
                page_key(&mut st, info.id, key(opener));
                assert!(matches!(st.overlay, Overlay::Help | Overlay::Palette), "{} {opener} opens", info.id);
                render(&mut st, p);
                page_key(&mut st, info.id, key("esc"));
                assert_eq!(st.overlay, Overlay::None, "{}: esc closes {opener}, stays on the page", info.id);
                page_key(&mut st, info.id, key("esc"));
                assert_eq!(st.overlay, Overlay::Back, "{}: the next esc leaves", info.id);
            }
        }
    }

    #[test]
    fn help_stays_open_until_closed() {
        let mut st = state();
        page_key(&mut st, "tasks", key("?"));
        page_key(&mut st, "tasks", key("↓"));
        assert_eq!(st.overlay, Overlay::Help, "other keys don't close it or act behind it");
        page_key(&mut st, "tasks", key("?"));
        assert_eq!(st.overlay, Overlay::None);
    }

    #[test]
    fn palette_filters_and_runs_a_command() {
        let mut st = state();
        st.open_page();
        let sel = st.ct_sel;
        page_key(&mut st, "containers", key("ctrl+k"));
        for c in "restart".chars() { page_key(&mut st, "containers", key(&c.to_string())); }
        assert_eq!(st.overlay, Overlay::Palette, "typing filters, it doesn't close");
        assert_eq!(dock::CONTAINERS_KEYS.commands(&st.palette_query).len(), 1);
        page_key(&mut st, "containers", key("enter"));
        assert_eq!(st.ct_states[sel], q::atoms::RunState::Restarting, "enter ran Restart");
        // ↓ moves the highlight, esc closes without running
        page_key(&mut st, "tasks", key("ctrl+k"));
        page_key(&mut st, "tasks", key("↓"));
        assert_eq!(st.palette_sel, 1);
        page_key(&mut st, "tasks", key("esc"));
        assert_eq!(st.overlay, Overlay::None);
    }

    #[test]
    fn confirm_dialog_enter_presses_the_focused_button() {
        let mut st = state();
        st.open_page();
        let n = st.tasks.len();
        page_key(&mut st, "tasks", key("del"));
        assert!(st.confirm_delete && st.confirm_btn == 1, "danger: focus starts on Keep");
        page_key(&mut st, "tasks", key("enter"));
        assert_eq!(st.tasks.len(), n, "enter on Keep keeps it");
        page_key(&mut st, "tasks", key("del"));
        page_key(&mut st, "tasks", key("←"));
        page_key(&mut st, "tasks", key("enter"));
        assert_eq!(st.tasks.len(), n - 1, "← then enter deletes");
        // nothing to delete: nothing asks
        st.tab = 2;
        st.tasks.retain(|t| !t.done);
        page_key(&mut st, "tasks", key("del"));
        assert!(!st.confirm_delete);
    }

    #[test]
    fn a_toast_never_eats_a_key() {
        let mut st = state();
        st.open_page();
        page_key(&mut st, "workspace", key("ctrl+s"));
        assert_eq!(st.overlay, Overlay::Toast);
        let sel = st.tree_sel;
        page_key(&mut st, "workspace", key("tab"));
        page_key(&mut st, "workspace", key("ctrl+s"));
        page_key(&mut st, "workspace", key("↓"));
        assert_ne!(st.tree_sel, sel, "↓ moved even with a toast up");
    }

    #[test]
    fn tasks_tabs_follow_the_data() {
        let mut st = state();
        for (tab, want) in [(0, 2), (1, 3), (2, 1)] {
            st.tab = tab;
            assert_eq!(st.task_list().len(), want, "tab {tab}");
        }
        st.tab = 0;
        page_key(&mut st, "tasks", key("x"));
        assert_eq!(st.task_list().len(), 1, "done leaves Today");
        st.tab = 2;
        assert_eq!(st.task_list().len(), 2, "and shows in Done");
    }

    #[test]
    fn dashboard_enter_opens_the_project() {
        let mut st = state();
        page_key(&mut st, "dashboard", key("↓"));
        page_key(&mut st, "dashboard", key("enter"));
        assert_eq!((st.goto, data::PROJECTS[st.project]), (Some("tasks"), "web-gl-shooter"));
    }

    #[test]
    fn opening_a_page_starts_it_fresh() {
        let mut st = state();
        page_key(&mut st, "tasks", key("tab"));
        page_key(&mut st, "tasks", key("tab"));
        page_key(&mut st, "tasks", key("?"));
        st.open_page();
        assert_eq!((st.focus, st.overlay), (1, Overlay::None));
    }

    #[test]
    fn log_filter_types() {
        let mut st = state();
        for c in ["u", "p", "backspace"] { page_key(&mut st, "log", key(c)); }
        assert_eq!(st.log_filter, "u");
    }

    /// Thousands of random keys on every page: no panic, every selection stays inside its list.
    #[test]
    fn random_keys_never_break_a_page() {
        let names = ["↑", "↓", "←", "→", "shift+←", "shift+↑", "ctrl+←", "ctrl+↓", "tab", "shift+tab", "enter", "esc", "space",
            "pgup", "pgdn", "home", "end", "del", "backspace", "[", "]", "1", "2", "3", "4", "5", "a", "d", "e", "f", "i", "l",
            "n", "p", "r", "s", "x", "z", "?", "ctrl+k", "ctrl+s"];
        let mut seed: u64 = 0x5eed;
        let mut rnd = |n: usize| { seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407); (seed >> 33) as usize % n };
        for (p, info) in PAGES.iter().enumerate() {
            let mut st = state();
            st.open_page();
            for i in 0..1500 {
                page_key(&mut st, info.id, key(names[rnd(names.len())]));
                if st.overlay == Overlay::Back { st.overlay = Overlay::None; }
                st.goto = None;
                if i % 25 == 0 { render(&mut st, p); }
                assert!(st.finding <= data::findings_in(st.range).len().saturating_sub(1), "{}: audit finding", info.id);
                assert!(st.tree_sel < st.tree_rows().len(), "{}: tree", info.id);
                assert!(st.project < data::PROJECTS.len() && st.dash_sel < data::PROJECT_ROWS.len(), "{}: projects", info.id);
                assert!(st.selected <= st.task_list().len().saturating_sub(1), "{}: task", info.id);
                assert!(st.settings_field < pages::SETTINGS_FIELDS, "{}: settings", info.id);
                assert!(st.key < st.points.len() && st.node < forge::GRAPH.len(), "{}: forge", info.id);
            }
            render(&mut st, p);
        }
    }

    #[test]
    fn explorer_list_keys() {
        let mut ex = Explorer::new().unwrap();
        let first = |ex: &Explorer| ex.order()[0];
        assert_eq!(ex.selected, first(&ex));
        ex.list_key(key("pgdn"));
        assert_eq!(ex.entries[ex.selected].level, Level::Molecule, "pgdn: next level");
        ex.list_key(key("pgup"));
        assert_eq!(ex.selected, first(&ex), "pgup: back to the first atom");
        ex.list_key(key("end"));
        assert_eq!(ex.entries[ex.selected].level, Level::Ui);
        ex.list_key(key("/"));
        for c in "graph".chars() { ex.list_key(key(&c.to_string())); }
        assert!(ex.order().iter().all(|&i| ex.entries[i].name.to_lowercase().contains("graph")));
        assert!(ex.order().contains(&ex.selected), "the selection follows the filter");
        assert!(ex.list_key(key("q")), "q types into the filter, it doesn't quit");
        ex.list_key(key("esc"));
        assert!(ex.filter.is_empty() && !ex.filtering);
        assert!(!ex.list_key(key("q")), "q quits outside the filter");
    }
}
