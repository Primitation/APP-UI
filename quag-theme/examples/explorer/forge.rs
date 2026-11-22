//! "forge": demo app for game design data (curves, loot tables, unit balance, sequences).
//! Data, elements and pages here; same rules: elements wire data to molecules, pages place elements.

use crate::consts as C;
use crate::pages::{frame_with, overlays, PageState, BACK, HELP, PALETTE};
use quag_theme::molecules::ActionButton as AB;
use quag_theme::{
    Status,
    atoms::{self, Tone},
    consts::input,
    elements::{EmptyState, ErrorState},
    keymap::{Binding, Keymap},
    molecules::{fmt_num, ClipSpec, NodeGraph, NodeSpec, Wire, Column, CurveEditor, ListRow, PropertyList, Stat, StatusLine, Table, Timeline, Track, WeightsEditor},
    ui::Zones,
};
use ratatui::{
    Frame,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    text::Line,
    widgets::Widget,
};

// ───────────────────────────── data ─────────────────────────────

pub struct CurveAsset {
    pub name: &'static str,
    pub what: &'static str,
    pub x: [f64; 2],
    pub y: [f64; 2],
    pub x_unit: &'static str,
    pub y_unit: &'static str,
    pub points: &'static [(f64, f64)],
}

pub const CURVES: &[CurveAsset] = &[
    CurveAsset { name: "difficulty", what: "Enemy stat multiplier over a run.", x: [0.0, 60.0], y: [0.5, 3.0], x_unit: "min", y_unit: "×", points: &[(0.0, 0.8), (10.0, 1.0), (25.0, 1.4), (40.0, 2.1), (50.0, 2.2), (60.0, 2.8)] },
    CurveAsset { name: "xp to next level", what: "XP needed for each level.", x: [1.0, 50.0], y: [0.0, 50_000.0], x_unit: "", y_unit: "xp", points: &[(1.0, 100.0), (10.0, 2_400.0), (20.0, 8_000.0), (35.0, 22_000.0), (50.0, 48_000.0)] },
    CurveAsset { name: "damage falloff", what: "Damage share by distance.", x: [0.0, 40.0], y: [0.0, 1.0], x_unit: "m", y_unit: "×", points: &[(0.0, 1.0), (8.0, 1.0), (20.0, 0.55), (32.0, 0.2), (40.0, 0.1)] },
];

pub const LOOT: &[(&str, &[(&str, f64)])] = &[
    ("Slime", &[("Gel", 50.0), ("Coin pouch", 30.0), ("Small potion", 15.0), ("Slime crown", 5.0)]),
    ("Golem", &[("Stone", 60.0), ("Iron ore", 25.0), ("Golem core", 10.0), ("Ancient rune", 4.0), ("Nothing", 1.0)]),
    ("Mire King (boss)", &[("Crown of reeds", 40.0), ("Bog blade", 30.0), ("Mire scale ×5", 25.0), ("Quagsire charm", 5.0)]),
];

/// name, class, (hp, dps, cost) now, (hp, dps, cost) last patch
pub const UNITS: &[(&str, &str, [f64; 3], [f64; 3])] = &[
    ("Pikeman", "infantry", [420.0, 18.0, 60.0], [400.0, 18.0, 60.0]),
    ("Archer", "ranged", [260.0, 22.0, 75.0], [260.0, 20.0, 75.0]),
    ("Knight", "cavalry", [780.0, 30.0, 140.0], [780.0, 34.0, 140.0]),
    ("Mage", "ranged", [220.0, 48.0, 120.0], [220.0, 36.0, 110.0]),
    ("Golem", "siege", [1500.0, 25.0, 200.0], [1500.0, 25.0, 220.0]),
    ("Scout", "cavalry", [300.0, 12.0, 50.0], [300.0, 12.0, 50.0]),
];

pub const SEQUENCE: &[Track] = &[
    Track { name: "Camera", clips: &[ClipSpec { start: 0.0, len: 6.0, label: "wide: bog" }, ClipSpec { start: 6.0, len: 8.0, label: "dolly → king" }, ClipSpec { start: 14.0, len: 10.0, label: "close-up" }, ClipSpec { start: 24.0, len: 12.0, label: "orbit fight" }] },
    Track { name: "Dialogue", clips: &[ClipSpec { start: 4.0, len: 5.0, label: "KING: Who…" }, ClipSpec { start: 15.0, len: 7.0, label: "HERO: Your reign ends" }] },
    Track { name: "Music", clips: &[ClipSpec { start: 0.0, len: 14.0, label: "mire_ambient" }, ClipSpec { start: 14.0, len: 26.0, label: "boss_theme" }] },
    Track { name: "SFX", clips: &[ClipSpec { start: 2.0, len: 1.5, label: "splash" }, ClipSpec { start: 13.0, len: 2.0, label: "roar" }, ClipSpec { start: 24.0, len: 1.0, label: "hit" }] },
    Track { name: "Events", clips: &[ClipSpec { start: 23.5, len: 1.0, label: "spawn adds" }, ClipSpec { start: 36.0, len: 1.0, label: "phase 2" }] },
];
pub const SEQ_SPAN: [f64; 2] = [0.0, 40.0];

/// The "fireball" ability: two events, each a chain left → right.
pub const GRAPH: &[NodeSpec] = &[
    NodeSpec { title: "On Cast", inputs: &[], outputs: &["exec", "caster"] },
    NodeSpec { title: "Has mana?", inputs: &["exec", "caster"], outputs: &["true", "false"] },
    NodeSpec { title: "Spend mana", inputs: &["exec"], outputs: &["exec"] },
    NodeSpec { title: "Spawn projectile", inputs: &["exec"], outputs: &["exec"] },
    NodeSpec { title: "Say \"No mana\"", inputs: &["exec"], outputs: &[] },
    NodeSpec { title: "On Hit", inputs: &[], outputs: &["exec", "target"] },
    NodeSpec { title: "Deal damage", inputs: &["exec", "target"], outputs: &["exec", "target"] },
    NodeSpec { title: "Apply burn", inputs: &["exec", "target"], outputs: &[] },
];
pub const GRAPH_AT: &[(i32, i32)] = &[(1, 1), (21, 1), (43, 1), (63, 1), (43, 6), (1, 11), (21, 11), (43, 11)];
pub const WIRES: &[Wire] = &[
    Wire { from: (0, 0), to: (1, 0) }, Wire { from: (0, 1), to: (1, 1) },
    Wire { from: (1, 0), to: (2, 0) }, Wire { from: (1, 1), to: (4, 0) }, Wire { from: (2, 0), to: (3, 0) },
    Wire { from: (5, 0), to: (6, 0) }, Wire { from: (5, 1), to: (6, 1) },
    Wire { from: (6, 0), to: (7, 0) }, Wire { from: (6, 1), to: (7, 1) },
];
/// Each node's settings, shown in the inspector.
pub const NODE_PARAMS: &[&[(&str, &str)]] = &[
    &[("Trigger", "ability key pressed")], &[("Needs", "20 mana")], &[("Cost", "20 mana")],
    &[("Speed", "18 m/s"), ("Lifetime", "2 s")], &[("Line", "Not enough mana.")], &[("Trigger", "projectile hits")],
    &[("Amount", "35"), ("Type", "fire")], &[("Damage", "4 / s"), ("Duration", "3 s")],
];

/// Where the project data stands: shown by every forge panel the same way.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Load {
    #[default]
    Ready,
    Loading,
    Failed,
}

/// Loading → skeleton rows; Failed → ErrorState (what + why + Retry). Returns true when it drew a state.
fn load_state(load: Load, file: &str, inner: Rect, buf: &mut Buffer) -> bool {
    match load {
        Load::Ready => false,
        Load::Loading => {
            for i in 0..inner.height.min(8) / 2 { atoms::Skeleton(0.4 + 0.08 * (i % 5) as f64).render(Rect { y: inner.y + 1 + i * 2, height: 1, ..inner }, buf); }
            true
        }
        Load::Failed => {
            let what = format!("Can't read {file}.");
            ErrorState { what: &what, why: "line 14: expected 4 columns, found 3", retry_focused: true }.render(inner, buf);
            true
        }
    }
}

// ───────────────────────────── elements ─────────────────────────────

/// AssetList: the curves (or loot tables) of the project, the selected one highlighted.
pub struct AssetList<'a> {
    pub title: &'a str,
    pub names: &'a [&'a str],
    pub selected: usize,
}

impl Widget for AssetList<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = atoms::panel(self.title, false);
        let inner = block.inner(area);
        block.render(area, buf);
        for (i, n) in self.names.iter().enumerate().take(inner.height as usize) {
            (&ListRow { title: n, selected: i == self.selected, ..Default::default() }).render(Rect { y: inner.y + i as u16, height: 1, ..inner }, buf);
        }
    }
}

/// CurvePanel: the curve being edited.
pub struct CurvePanel<'a> {
    pub load: Load,
    pub asset: &'a CurveAsset,
    pub points: &'a [(f64, f64)],
    pub selected: usize,
}

impl Widget for CurvePanel<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = format!("Curve · {}", self.asset.name);
        let block = atoms::panel(&title, true);
        let inner = block.inner(area);
        block.render(area, buf);
        if load_state(self.load, "data/curves.csv", inner, buf) { return; }
        let a = self.asset;
        let [what, _, ed] = Layout::vertical([Constraint::Length(1), Constraint::Length(1), Constraint::Min(0)]).areas(inner);
        Line::from(atoms::label(a.what, Tone::Muted)).render(what, buf);
        CurveEditor { points: self.points, x: a.x, y: a.y, x_unit: a.x_unit, y_unit: a.y_unit, selected: Some(self.selected) }.render(ed, buf);
    }
}

/// KeyInspector: the selected key's values, neighbours, and actions.
pub struct KeyInspector<'a> {
    pub asset: &'a CurveAsset,
    pub points: &'a [(f64, f64)],
    pub selected: usize,
    /// The focused button (Some while the zone has focus).
    pub button: Option<usize>,
}

impl KeyInspector<'_> {
    pub const BUTTONS: &'static [AB<'static>] = &[AB::new("Add key", "add"), AB::new("Delete key", "delete").danger()];
}

impl Widget for KeyInspector<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = atoms::panel("Key", self.button.is_some());
        let inner = block.inner(area);
        block.render(area, buf);
        let Some((x, y)) = self.points.get(self.selected) else {
            EmptyState { message: "No key selected.", hint: Some(("a", "to add one")) }.render(inner, buf);
            return;
        };
        let a = self.asset;
        let [facts, _, slope, _, rest] = Layout::vertical([Constraint::Length(3), Constraint::Length(1), Constraint::Length(2), Constraint::Length(1), Constraint::Min(0)]).areas(inner);
        let (xs, ys, n) = (fmt_num(*x, a.x_unit), fmt_num(*y, a.y_unit), format!("{} of {}", self.selected + 1, self.points.len()));
        PropertyList(&[("Key", &n), ("x", &xs), ("y", &ys)]).render(facts, buf);
        let rise = |i: usize| self.points.get(i + 1).map(|p| (p.1 - self.points[i].1) / (p.0 - self.points[i].0).max(f64::EPSILON));
        let s = rise(self.selected).map(|r| format!("{} {} per {}", if r >= 0.0 { "▲" } else { "▼" }, fmt_num(r.abs(), a.y_unit), if a.x_unit.is_empty() { "step" } else { a.x_unit })).unwrap_or("last key".into());
        Line::from(vec![atoms::label("Slope to next  ", Tone::Muted), atoms::label(&s, Tone::Text)]).render(slope, buf);
        if rest.height >= 1 {
            AB::group(Self::BUTTONS, self.button).render(Rect { y: rest.bottom() - 1, height: 1, ..rest }, buf);
        }
    }
}

/// LootPanel: one drop table's weights.
pub struct LootPanel<'a> {
    pub load: Load,
    pub name: &'a str,
    pub items: &'a [(&'a str, f64)],
    pub selected: usize,
}

impl Widget for LootPanel<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = format!("Drops · {}", self.name);
        let block = atoms::panel(&title, true);
        let inner = block.inner(area);
        block.render(area, buf);
        if load_state(self.load, "data/loot.csv", inner, buf) { return; }
        if self.items.is_empty() {
            EmptyState { message: "This table drops nothing.", hint: Some(("a", "to add an item")) }.render(inner, buf);
            return;
        }
        WeightsEditor { items: self.items, selected: self.selected, focused: true }.render(inner, buf);
    }
}

/// PerHundred: what 100 kills give, on average.
pub struct PerHundred<'a> {
    pub items: &'a [(&'a str, f64)],
}

impl Widget for PerHundred<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = atoms::panel("Per 100 kills", false);
        let inner = block.inner(area);
        block.render(area, buf);
        let sh = quag_theme::molecules::shares(&self.items.iter().map(|i| i.1).collect::<Vec<_>>());
        let rows: Vec<(&str, f64)> = self.items.iter().zip(sh).map(|((l, _), s)| (*l, s * 100.0)).collect();
        quag_theme::molecules::BarChart { items: &rows, unit: "", max: None, budget: None }.render(inner, buf);
    }
}

/// BalanceTable: unit stats against last patch, value for cost, outliers marked.
pub struct BalanceTable {
    pub selected: usize,
    pub load: Load,
}

impl BalanceTable {
    /// DPS per 100 gold, the median of all units, and whether a unit is an outlier (above C::OUTLIER_RATIO × median).
    pub fn value(u: &[f64; 3]) -> f64 { u[1] / u[2] * 100.0 }
    /// (strong, weak) outliers: value for cost above / below the median by OUTLIER_RATIO.
    pub fn outliers() -> (usize, usize) {
        let m = Self::median();
        let v = || UNITS.iter().map(|u| Self::value(&u.2));
        (v().filter(|x| *x > m * C::OUTLIER_RATIO).count(), v().filter(|x| *x < m / C::OUTLIER_RATIO).count())
    }
    pub fn median() -> f64 {
        let mut v: Vec<f64> = UNITS.iter().map(|u| Self::value(&u.2)).collect();
        v.sort_by(f64::total_cmp);
        v[v.len() / 2]
    }
}

impl Widget for BalanceTable {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = atoms::panel("Units · vs last patch", true);
        let inner = block.inner(area);
        block.render(area, buf);
        if load_state(self.load, "data/units.csv", inner, buf) { return; }
        let cols = [
            Column { header: "Unit", width: 0, right: false }, Column { header: "Class", width: 9, right: false },
            Column { header: "HP", width: 5, right: true }, Column { header: "", width: 8, right: false },
            Column { header: "DPS", width: 4, right: true }, Column { header: "", width: 9, right: false },
            Column { header: "Cost", width: 5, right: true }, Column { header: "", width: 9, right: false },
            Column { header: "DPS/100g", width: 9, right: true }, Column { header: "", width: 10, right: false },
        ];
        let rows: Vec<Vec<String>> = UNITS.iter().map(|(n, c, now, _)| vec![
            n.to_string(), c.to_string(), fmt_num(now[0], ""), String::new(), fmt_num(now[1], ""), String::new(), fmt_num(now[2], ""), String::new(), format!("{:.1}", Self::value(now)), String::new(),
        ]).collect();
        let refs: Vec<Vec<&str>> = rows.iter().map(|r| r.iter().map(String::as_str).collect()).collect();
        let sl: Vec<&[&str]> = refs.iter().map(Vec::as_slice).collect();
        let t = Table { columns: &cols, rows: &sl, selected: Some(self.selected), focused: true };
        let cells: Vec<Vec<Rect>> = (0..UNITS.len()).map(|i| t.cell_areas(inner, i)).collect();
        t.render(inner, buf);
        let med = Self::median();
        for ((_, _, now, prev), cell) in UNITS.iter().zip(cells).take(inner.height.saturating_sub(1) as usize) {
            // HP and DPS: higher is better; cost: lower is better
            for (k, ci, higher) in [(0, 3, true), (1, 5, true), (2, 7, false)] {
                let ch = now[k] / prev[k] - 1.0;
                if ch.abs() >= 0.0005 { Line::from(atoms::delta_badge(ch, higher)).render(cell[ci], buf); }
            }
            let v = Self::value(now);
            if v > med * C::OUTLIER_RATIO { Line::from(atoms::status_label("strong", Status::Warn)).render(cell[9], buf); }
            else if v < med / C::OUTLIER_RATIO { Line::from(atoms::status_label("weak", Status::Info)).render(cell[9], buf); }
        }
    }
}

/// SequencePanel: the cutscene's tracks.
pub struct SequencePanel {
    pub playhead: f64,
    pub selected: (usize, usize),
    pub playing: bool,
    pub tick: usize,
}

impl Widget for SequencePanel {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = format!("Sequence · boss_intro   {} {}", if self.playing { "▶" } else { "‖" }, fmt_num(self.playhead, "s"));
        let block = atoms::panel(&title, true);
        let inner = block.inner(area);
        block.render(area, buf);
        let _ = self.tick;
        Timeline { tracks: SEQUENCE, span: SEQ_SPAN, playhead: self.playhead, unit: "s", selected: Some(self.selected) }.render(inner, buf);
    }
}

/// ClipInspector: the selected clip.
pub struct ClipInspector {
    pub selected: (usize, usize),
}

impl Widget for ClipInspector {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = atoms::panel("Clip", false);
        let inner = block.inner(area);
        block.render(area, buf);
        let Some((tr, c)) = SEQUENCE.get(self.selected.0).and_then(|t| t.clips.get(self.selected.1).map(|c| (t, c))) else {
            EmptyState { message: "Select a clip.", hint: None }.render(inner, buf);
            return;
        };
        let (s, l, e) = (fmt_num(c.start, "s"), fmt_num(c.len, "s"), fmt_num(c.start + c.len, "s"));
        PropertyList(&[("Track", tr.name), ("Clip", c.label), ("Start", &s), ("Length", &l), ("End", &e)]).render(inner, buf);
    }
}

// ───────────────────────────── pages ─────────────────────────────

pub const CURVES_KEYS: Keymap = Keymap(&[
    Binding::new("pgup pgdn", "previous / next curve", "Move").hint("curve").action("next"),
    Binding::new("[ ]", "previous / next key", "Move").hint("key").action("key"),
    Binding::new("← → ↑ ↓", "move the key", "Edit").hint("move key").action("nudge"),
    Binding::new("shift+← shift+→ shift+↑ shift+↓", "move the key a little", "Edit").action("nudge_fine"),
    Binding::new("a", "add a key after this one", "Edit").command("Add key").action("add"),
    Binding::new("del", "delete the key", "Edit").command("Delete key").danger().action("delete"),
    crate::pages::DETAILS, quag_theme::keymap::BUTTONS, HELP, PALETTE, BACK,
]);
pub const LOOT_KEYS: Keymap = Keymap(&[
    Binding::new("pgup pgdn", "previous / next table", "Move").hint("table").action("next"),
    Binding::new("↑ ↓", "previous / next item", "Move").hint("move").action("move"),
    Binding::new("← →", "less / more weight", "Edit").hint("weight").action("weight"),
    HELP, PALETTE, BACK,
]);
pub const BALANCE_KEYS: Keymap = Keymap(&[
    Binding::new("↑ ↓", "previous / next unit", "Move").hint("move").action("move"),
    Binding::new("r", "reload the data", "Act").hint("reload").command("Reload data").action("reload"),
    HELP, PALETTE, BACK,
]);
pub const SEQUENCE_KEYS: Keymap = Keymap(&[
    Binding::new("space", "play / pause", "Play").hint("play").command("Play / pause").action("play"),
    Binding::new("← →", "move the playhead", "Play").hint("scrub").action("scrub"),
    Binding::new("↑ ↓", "previous / next track", "Move").hint("track").action("track"),
    Binding::new("[ ]", "previous / next clip", "Move").hint("clip").action("clip"),
    HELP, PALETTE, BACK,
]);

const STATUS_SAVED: StatusLine<'static> = StatusLine { kind: Status::Ok, message: "saved" };

/// Workspace: curves · the curve · the selected key.
pub fn curves(f: &mut Frame, area: Rect, st: &mut PageState) {
    let Some(s) = frame_with(f, area, Zones { nav: true, inspector: true }, "forge", None, "curves", None, CURVES_KEYS, Some(STATUS_SAVED)) else { return };
    let a = &CURVES[st.curve % CURVES.len()];
    let names: Vec<&str> = CURVES.iter().map(|c| c.name).collect();
    if let Some(nav) = s.nav { f.render_widget(AssetList { title: "Curves", names: &names, selected: st.curve % CURVES.len() }, nav); }
    f.render_widget(CurvePanel { load: st.forge_load, asset: a, points: &st.points, selected: st.key }, s.content);
    if let Some(i) = s.inspector { f.render_widget(KeyInspector { asset: a, points: &st.points, selected: st.key, button: st.button_in(2) }, i); }
    let msg = st.toast_msg.clone();
    overlays(f, area, st, CURVES_KEYS, &msg);
}

/// List → Detail: drop tables · weights · what 100 kills give.
pub fn loot(f: &mut Frame, area: Rect, st: &mut PageState) {
    let Some(s) = frame_with(f, area, Zones { nav: true, inspector: true }, "forge", None, "loot tables", None, LOOT_KEYS, Some(STATUS_SAVED)) else { return };
    let t = st.loot_table % LOOT.len();
    let names: Vec<&str> = LOOT.iter().map(|l| l.0).collect();
    if let Some(nav) = s.nav { f.render_widget(AssetList { title: "Tables", names: &names, selected: t }, nav); }
    let items: Vec<(&str, f64)> = LOOT[t].1.iter().zip(&st.weights[t]).map(|((n, _), w)| (*n, *w)).collect();
    f.render_widget(LootPanel { load: st.forge_load, name: LOOT[t].0, items: &items, selected: st.loot_sel.min(items.len().saturating_sub(1)) }, s.content);
    if let Some(i) = s.inspector { f.render_widget(PerHundred { items: &items }, i); }
    let msg = st.toast_msg.clone();
    overlays(f, area, st, LOOT_KEYS, &msg);
}

/// Dashboard: the roster's numbers, then every unit against last patch.
pub fn balance(f: &mut Frame, area: Rect, st: &mut PageState) {
    let (strong, weak) = BalanceTable::outliers();
    let msg_status = match st.forge_load { Load::Ready => format!("{strong} strong · {weak} weak"), Load::Loading => "loading…".into(), Load::Failed => "data/units.csv unreadable".into() };
    let kind = match st.forge_load { Load::Failed => Status::Err, Load::Loading => Status::Info, Load::Ready if strong > 0 => Status::Warn, Load::Ready => Status::Ok };
    let status = StatusLine { kind, message: &msg_status };
    let Some(s) = frame_with(f, area, Zones { nav: false, inspector: false }, "forge", None, "balance · patch 0.9", None, BALANCE_KEYS, Some(status)) else { return };
    let [kpis, _, table] = Layout::vertical([Constraint::Length(Stat::HEIGHT), Constraint::Length(1), Constraint::Min(0)]).areas(s.content);
    let cols = Layout::horizontal([Constraint::Fill(1); 4]).spacing(2).split(kpis);
    let med = format!("{:.1}", BalanceTable::median());
    let changed = UNITS.iter().filter(|u| u.2 != u.3).count().to_string();
    for (r, (v, l)) in cols.iter().zip([(UNITS.len().to_string(), "units"), (changed, "changed this patch"), (med, "median DPS / 100 gold"), (strong.to_string(), "strong outliers")]) {
        // no numbers until the data is there
        f.render_widget(Stat { value: if st.forge_load == Load::Ready { &v } else { "–" }, label: l }, *r);
    }
    f.render_widget(BalanceTable { selected: st.unit_sel.min(UNITS.len() - 1), load: st.forge_load }, table);
    let msg = st.toast_msg.clone();
    overlays(f, area, st, BALANCE_KEYS, &msg);
}

/// Timeline: the sequence · the selected clip.
pub fn sequence(f: &mut Frame, area: Rect, st: &mut PageState) {
    // real time: the ticks since the last frame, however often it redraws
    let dt = st.tick.saturating_sub(st.play_tick) as f64 * quag_theme::consts::motion::TICK.as_secs_f64();
    st.play_tick = st.tick;
    if st.playing {
        st.playhead += dt;
        if st.playhead > SEQ_SPAN[1] { st.playhead = SEQ_SPAN[0]; }
    }
    let Some(s) = frame_with(f, area, Zones { nav: false, inspector: true }, "forge", None, "sequences", None, SEQUENCE_KEYS, Some(STATUS_SAVED)) else { return };
    f.render_widget(SequencePanel { playhead: st.playhead, selected: st.clip, playing: st.playing, tick: st.tick }, s.content);
    if let Some(i) = s.inspector { f.render_widget(ClipInspector { selected: st.clip }, i); }
    let msg = st.toast_msg.clone();
    overlays(f, area, st, SEQUENCE_KEYS, &msg);
}

/// Key actions of the forge pages (called from main.rs `page_key`).
pub fn key(st: &mut PageState, page: &str, action: &str, name: &str) -> bool {
    let dir = |n: &str| -> (f64, f64) {
        match n.trim_start_matches("shift+") { "←" => (-1.0, 0.0), "→" => (1.0, 0.0), "↑" => (0.0, 1.0), "↓" => (0.0, -1.0), _ => (0.0, 0.0) }
    };
    // pgup / pgdn: previous / next item, wrapping
    let cycle = |i: usize, n: usize| if name == "pgup" { (i + n - 1) % n } else { (i + 1) % n };
    match (page, action) {
        ("curves", "next") => { st.curve = cycle(st.curve, CURVES.len()); st.points = CURVES[st.curve].points.to_vec(); st.key = 0; }
        ("curves", "key") => st.key = if name == "]" { (st.key + 1).min(st.points.len().saturating_sub(1)) } else { st.key.saturating_sub(1) },
        ("curves", "nudge" | "nudge_fine") => {
            let a = &CURVES[st.curve];
            let share = if action == "nudge" { input::NUDGE } else { input::NUDGE_FINE };
            let (dx, dy) = dir(name);
            quag_theme::molecules::nudge_point(&mut st.points, st.key, dx * share * (a.x[1] - a.x[0]), dy * share * (a.y[1] - a.y[0]), a.x, a.y);
        }
        ("curves", "add") => {
            if let Some(&(x1, y1)) = st.points.get(st.key) && let Some(&(x2, y2)) = st.points.get(st.key + 1) {
                st.points.insert(st.key + 1, ((x1 + x2) / 2.0, (y1 + y2) / 2.0));
                st.key += 1;
            }
        }
        // the first and last keys hold the range ends: they stay
        ("curves", "delete") => if st.key > 0 && st.key + 1 < st.points.len() { st.points.remove(st.key); st.key -= 1; },
        ("loot", "next") => { st.loot_table = cycle(st.loot_table, LOOT.len()); st.loot_sel = 0; }
        ("loot", "move") => st.loot_sel = if name == "↓" { (st.loot_sel + 1).min(LOOT[st.loot_table].1.len() - 1) } else { st.loot_sel.saturating_sub(1) },
        ("loot", "weight") => {
            let w = &mut st.weights[st.loot_table][st.loot_sel];
            *w = (*w + if name == "→" { input::WEIGHT_STEP } else { -input::WEIGHT_STEP }).max(0.0);
        }
        ("balance", "reload") => { st.forge_load = Load::Ready; st.toast("Reloaded data/units.csv"); }
        ("graph", "next") => st.node = cycle(st.node, GRAPH.len()),
        ("graph", "move" | "move_fast") => {
            let step = if action == "move" { input::NODE_STEP } else { input::NODE_STEP_FAST };
            let (dx, dy) = dir(name);
            let p = &mut st.node_at[st.node];
            p.0 = (p.0 + dx as i32 * step).max(0);
            p.1 = (p.1 - dy as i32 * step).max(0);
        }
        ("graph", "pan") => {
            let (dx, dy) = dir(name.trim_start_matches("ctrl+"));
            st.pan = ((st.pan.0 + dx as i32 * C::PAN_STEP).max(0), (st.pan.1 - dy as i32 * C::PAN_STEP).max(0));
        }
        ("balance", "move") => st.unit_sel = if name == "↓" { (st.unit_sel + 1).min(UNITS.len() - 1) } else { st.unit_sel.saturating_sub(1) },
        ("sequence", "play") => st.playing = !st.playing,
        ("sequence", "scrub") => st.playhead = (st.playhead + if name == "→" { C::SCRUB_SECS } else { -C::SCRUB_SECS }).clamp(SEQ_SPAN[0], SEQ_SPAN[1]),
        ("sequence", "track") => {
            let t = if name == "↓" { (st.clip.0 + 1).min(SEQUENCE.len() - 1) } else { st.clip.0.saturating_sub(1) };
            st.clip = (t, st.clip.1.min(SEQUENCE[t].clips.len() - 1));
        }
        ("sequence", "clip") => {
            let n = SEQUENCE[st.clip.0].clips.len();
            st.clip.1 = if name == "]" { (st.clip.1 + 1).min(n - 1) } else { st.clip.1.saturating_sub(1) };
        }
        _ => return false,
    }
    true
}

/// GraphPanel: the ability's nodes and wires.
pub struct GraphPanel<'a> {
    pub at: &'a [(i32, i32)],
    pub selected: usize,
    pub pan: (i32, i32),
    pub load: Load,
}

impl Widget for GraphPanel<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = atoms::panel("Ability · fireball", true);
        let inner = block.inner(area);
        block.render(area, buf);
        if load_state(self.load, "abilities/fireball.graph", inner, buf) { return; }
        NodeGraph { nodes: GRAPH, positions: self.at, wires: WIRES, selected: Some(self.selected), pan: self.pan }.render(inner, buf);
    }
}

/// NodeInspector: the selected node's settings and what it is wired to.
pub struct NodeInspector {
    pub selected: usize,
}

impl Widget for NodeInspector {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = atoms::panel("Node", false);
        let inner = block.inner(area);
        block.render(area, buf);
        let Some(n) = GRAPH.get(self.selected) else {
            EmptyState { message: "Select a node.", hint: Some(("tab", "to pick one")) }.render(inner, buf);
            return;
        };
        let mut y = inner.y;
        Line::from(atoms::label(n.title, Tone::Title)).render(Rect { y, height: 1, ..inner }, buf);
        y += 2;
        let params = NODE_PARAMS[self.selected];
        PropertyList(params).render(Rect { y, height: params.len() as u16, ..inner }, buf);
        y += params.len() as u16 + 1;
        let mut links: Vec<String> = Vec::new();
        for w in WIRES {
            if w.to.0 == self.selected { links.push(format!("{} ← {}", n.inputs[w.to.1], GRAPH[w.from.0].title)); }
            if w.from.0 == self.selected { links.push(format!("{} → {}", n.outputs[w.from.1], GRAPH[w.to.0].title)); }
        }
        Line::from(atoms::label("Wired to", Tone::Muted)).render(Rect { y, height: 1, ..inner }, buf);
        y += 1;
        for l in links.iter().take(inner.bottom().saturating_sub(y) as usize) {
            Line::from(atoms::label(l, Tone::Text)).render(Rect { y, height: 1, ..inner }, buf);
            y += 1;
        }
    }
}

pub const GRAPH_KEYS: Keymap = Keymap(&[
    Binding::new("pgup pgdn", "previous / next node", "Move").hint("node").action("next"),
    Binding::new("← → ↑ ↓", "move the node", "Edit").hint("move node").action("move"),
    Binding::new("shift+← shift+→ shift+↑ shift+↓", "move the node faster", "Edit").action("move_fast"),
    Binding::new("ctrl+← ctrl+→ ctrl+↑ ctrl+↓", "pan the view", "View").hint("pan").action("pan"),
    HELP, PALETTE, BACK,
]);

/// Canvas: the graph · the selected node.
pub fn graph(f: &mut Frame, area: Rect, st: &mut PageState) {
    let Some(s) = frame_with(f, area, Zones { nav: false, inspector: true }, "forge", None, "abilities", None, GRAPH_KEYS, Some(STATUS_SAVED)) else { return };
    f.render_widget(GraphPanel { at: &st.node_at, selected: st.node, pan: st.pan, load: st.forge_load }, s.content);
    if let Some(i) = s.inspector { f.render_widget(NodeInspector { selected: st.node }, i); }
    let msg = st.toast_msg.clone();
    overlays(f, area, st, GRAPH_KEYS, &msg);
}

/// Starting weights, editable per table.
pub fn initial_weights() -> Vec<Vec<f64>> { LOOT.iter().map(|t| t.1.iter().map(|i| i.1).collect()).collect() }
