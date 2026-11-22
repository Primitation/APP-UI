//! "dock": a second demo app, for server / Docker administration (DESIGN.md "Servers and containers").
//! Data, elements and pages of the four admin pages live here so pond's files stay about pond.
//! Same rules: elements wire data to molecules, pages only place elements, colors come from atoms.

use crate::consts as C;
use crate::pages::{frame_with, overlays, PageState, BACK, HELP, PALETTE};
use quag_theme::molecules::ActionButton as AB;
use quag_theme::{
    Status,
    atoms::{self, DiffKind, Env, RunState, StepState, Tone},
    elements::{ConfirmDialog, DiffView, EmptyState, ErrorState, LogEntry, RunProgress, TypeToConfirm},
    fmt,
    keymap::{Binding, Keymap},
    molecules::{StatusLine, freshness, BarChart, Column, KeyValueRows, ListRow, Meter, PortMap, PropertyList, Step, StepList, Table, TreeItem, TreeView, flatten},
    ui::Zones,
};
use ratatui::{
    Frame,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    text::{Line, Span},
    widgets::Widget,
};

// ───────────────────────────── data ─────────────────────────────

const MIB: u64 = 1024 * 1024;
const GIB: u64 = 1024 * MIB;

pub struct Container {
    pub name: &'static str,
    pub stack: &'static str,
    pub image: &'static str,
    pub id: &'static str,
    pub state: RunState,
    pub cpu: &'static [f64],
    pub mem: u64,
    pub limit: u64,
    pub ports: &'static [PortMap<'static>],
    pub up: u64,
    pub env: &'static [(&'static str, &'static str, bool)],
    pub volumes: &'static [&'static str],
}

const API_ENV: &[(&str, &str, bool)] = &[
    ("RUST_LOG", "info", false), ("DATABASE_URL", "postgres://shop:hunter2@db:5432/shop", true),
    ("STRIPE_KEY", "sk_live_51HxQ2bK9vTz7m3f9a", true), ("WORKERS", "4", false),
];
const P_HTTPS: &[PortMap] = &[PortMap { host: Some(443), container: 443, proto: "tcp", public: true }, PortMap { host: Some(80), container: 80, proto: "tcp", public: true }];
const P_API: &[PortMap] = &[PortMap { host: None, container: 8080, proto: "tcp", public: false }];
const P_REDIS: &[PortMap] = &[PortMap { host: Some(6379), container: 6379, proto: "tcp", public: true }];
const P_PG: &[PortMap] = &[PortMap { host: None, container: 5432, proto: "tcp", public: false }];
const P_GRAF: &[PortMap] = &[PortMap { host: Some(3000), container: 3000, proto: "tcp", public: false }];

pub const CONTAINERS: &[Container] = &[
    Container { name: "proxy", stack: "shop", image: "traefik:3.1", id: "sha256:7be41aa03f9a", state: RunState::Running, cpu: &[3.0, 4.0, 3.0, 5.0, 6.0, 4.0, 3.0, 4.0, 5.0, 4.0], mem: 61 * MIB, limit: 256 * MIB, ports: P_HTTPS, up: 3_542_400, env: &[("TZ", "Europe/Paris", false)], volumes: &["certs:/letsencrypt"] },
    Container { name: "api-1", stack: "shop", image: "shop/api:1.8.2", id: "sha256:3f9a1c2b7e41", state: RunState::Running, cpu: &[22.0, 30.0, 28.0, 35.0, 41.0, 38.0, 44.0, 52.0, 47.0, 42.0], mem: 412 * MIB, limit: 512 * MIB, ports: P_API, up: 187_200, env: API_ENV, volumes: &["uploads:/srv/uploads"] },
    Container { name: "api-2", stack: "shop", image: "shop/api:1.8.2", id: "sha256:91c2b7e413f9", state: RunState::Flapping(5), cpu: &[60.0, 5.0, 70.0, 2.0, 80.0, 4.0, 75.0, 3.0, 88.0, 6.0], mem: 498 * MIB, limit: 512 * MIB, ports: P_API, up: 140, env: API_ENV, volumes: &["uploads:/srv/uploads"] },
    Container { name: "worker", stack: "shop", image: "shop/worker:1.8.2", id: "sha256:b7e413f9a1c2", state: RunState::Running, cpu: &[12.0, 14.0, 13.0, 15.0, 18.0, 16.0, 14.0, 13.0, 15.0, 14.0], mem: 210 * MIB, limit: GIB, ports: &[], up: 187_200, env: API_ENV, volumes: &[] },
    Container { name: "db", stack: "shop", image: "postgres:16.4", id: "sha256:e413f9a1c2b7", state: RunState::Running, cpu: &[8.0, 9.0, 8.0, 12.0, 10.0, 9.0, 11.0, 10.0, 9.0, 8.0], mem: 1_503_238_553, limit: 2 * GIB, ports: P_PG, up: 3_542_400, env: &[("POSTGRES_PASSWORD", "hunter2-but-longer", true)], volumes: &["pgdata:/var/lib/postgresql/data"] },
    Container { name: "cache", stack: "shop", image: "redis:7.4", id: "sha256:13f9a1c2b7e4", state: RunState::Running, cpu: &[2.0, 2.0, 3.0, 2.0, 2.0, 3.0, 2.0, 2.0, 2.0, 3.0], mem: 88 * MIB, limit: 256 * MIB, ports: P_REDIS, up: 3_542_400, env: &[], volumes: &[] },
    Container { name: "mailer", stack: "shop", image: "shop/mailer:0.4.0", id: "sha256:c2b7e413f9a1", state: RunState::Paused, cpu: &[0.0; 10], mem: 34 * MIB, limit: 128 * MIB, ports: &[], up: 86_400, env: &[], volumes: &[] },
    Container { name: "prometheus", stack: "monitoring", image: "prom/prometheus:2.54", id: "sha256:a1c2b7e413f9", state: RunState::Exited(137), cpu: &[30.0, 45.0, 60.0, 80.0, 95.0, 0.0, 0.0, 0.0, 0.0, 0.0], mem: 0, limit: GIB, ports: &[], up: 0, env: &[], volumes: &["prom:/prometheus"] },
    Container { name: "grafana", stack: "monitoring", image: "grafana/grafana:11.2", id: "sha256:f9a1c2b7e413", state: RunState::Running, cpu: &[4.0, 5.0, 4.0, 6.0, 5.0, 4.0, 5.0, 6.0, 4.0, 5.0], mem: 140 * MIB, limit: 512 * MIB, ports: P_GRAF, up: 3_542_400, env: &[("GF_SECURITY_ADMIN_PASSWORD", "grafana-admin-pw", true)], volumes: &["grafana:/var/lib/grafana"] },
    Container { name: "restic", stack: "backup", image: "restic/restic:0.17", id: "sha256:2b7e413f9a1c", state: RunState::Exited(0), cpu: &[0.0; 10], mem: 0, limit: 512 * MIB, ports: &[], up: 0, env: &[("RESTIC_PASSWORD", "correct-horse-battery", true)], volumes: &["pgdata:/data:ro"] },
];

pub const STACKS: &[TreeItem] = &[
    TreeItem { id: "shop", label: "shop", meta: "6/7", status: Some(Status::Warn), children: &[] },
    TreeItem { id: "monitoring", label: "monitoring", meta: "1/2", status: Some(Status::Err), children: &[] },
    TreeItem { id: "backup", label: "backup", meta: "0/1", status: None, children: &[] },
];

pub struct Host {
    pub name: &'static str,
    pub role: &'static str,
    pub reachable: bool,
    pub cpu: &'static [f64],
    pub mem: (u64, u64),
    pub disk: (u64, u64),
    pub up: (usize, usize),
    pub uptime: u64,
}

pub const HOSTS: &[Host] = &[
    Host { name: "prod-1", role: "shop · monitoring", reachable: true, cpu: &[35.0, 42.0, 38.0, 51.0, 47.0, 55.0, 61.0, 58.0, 52.0, 49.0, 54.0, 57.0], mem: (5_100 * MIB, 8 * GIB), disk: (41 * GIB, 80 * GIB), up: (8, 10), uptime: 3_542_400 },
    Host { name: "prod-2", role: "shop (replica)", reachable: true, cpu: &[20.0, 22.0, 25.0, 21.0, 24.0, 26.0, 23.0, 22.0, 25.0, 24.0, 23.0, 21.0], mem: (3_300 * MIB, 8 * GIB), disk: (33 * GIB, 80 * GIB), up: (5, 5), uptime: 3_542_400 },
    Host { name: "backup-1", role: "backup", reachable: true, cpu: &[5.0, 4.0, 6.0, 90.0, 95.0, 8.0, 5.0, 4.0, 5.0, 4.0, 6.0, 5.0], mem: (900 * MIB, 2 * GIB), disk: (455 * GIB, 500 * GIB), up: (1, 2), uptime: 1_209_600 },
    Host { name: "db-2", role: "postgres replica", reachable: false, cpu: &[], mem: (0, 0), disk: (0, 0), up: (0, 0), uptime: 0 },
];

pub const DIFF: &[(DiffKind, &str)] = &[
    (DiffKind::Hunk, "@@ services.api @@"),
    (DiffKind::Same, "api:"),
    (DiffKind::Del, "  image: shop/api:1.8.2"),
    (DiffKind::Add, "  image: shop/api:1.9.0"),
    (DiffKind::Same, "  deploy:"),
    (DiffKind::Same, "    replicas: 2"),
    (DiffKind::Del, "    resources: { limits: { memory: 512M } }"),
    (DiffKind::Add, "    resources: { limits: { memory: 768M } }"),
    (DiffKind::Same, "  environment:"),
    (DiffKind::Add, "    FEATURE_INVOICES: \"on\""),
    (DiffKind::Hunk, "@@ services.worker @@"),
    (DiffKind::Del, "  image: shop/worker:1.8.2"),
    (DiffKind::Add, "  image: shop/worker:1.9.0"),
];

pub const STEP_NAMES: &[&str] = &["pull", "migrate", "start new", "health check", "switch", "stop old"];
const STEP_TOOK: &[&str] = &["8s", "3s", "6s", "12s", "1s", "4s"];
pub const DEPLOY_LOG: &[(&str, Status, &str)] = &[
    ("14:02:01", Status::Info, "pull shop/api:1.9.0 · 212 MiB"),
    ("14:02:09", Status::Ok, "pulled shop/api:1.9.0, shop/worker:1.9.0"),
    ("14:02:10", Status::Info, "migrate: 2 pending (0042_invoices, 0043_invoice_index)"),
    ("14:02:13", Status::Ok, "migrations applied"),
    ("14:02:14", Status::Info, "start api-3, api-4 (1.9.0)"),
    ("14:02:20", Status::Info, "health check GET /healthz on api-3, api-4"),
    ("14:02:32", Status::Ok, "api-3 healthy · api-4 healthy"),
    ("14:02:33", Status::Info, "switch traffic → api-3, api-4"),
    ("14:02:34", Status::Info, "stop api-1, api-2 (1.8.2)"),
    ("14:02:38", Status::Ok, "deployed shop 1.9.0 in 37s"),
];
const FAIL_LOG: (&str, Status, &str) = ("14:02:32", Status::Err, "api-3 unhealthy: GET /healthz → 503 (3 tries) · old version still serving");

/// (image, id, size, age secs, used by)
pub const IMAGES: &[(&str, &str, u64, u64, usize)] = &[
    ("shop/api:1.8.2", "sha256:3f9a1c2b7e41", 212 * MIB, 864_000, 2),
    ("shop/worker:1.8.2", "sha256:b7e413f9a1c2", 198 * MIB, 864_000, 1),
    ("postgres:16.4", "sha256:e413f9a1c2b7", 432 * MIB, 3_542_400, 1),
    ("shop/api:1.7.9", "sha256:77e41aa03f9a", 209 * MIB, 2_592_000, 0),
    ("shop/api:1.7.8", "sha256:aa03f9a77e41", 207 * MIB, 3_196_800, 0),
    ("node:20-alpine", "sha256:41aa03f9a77e", 1_288 * MIB, 5_184_000, 0),
    ("redis:7.4", "sha256:13f9a1c2b7e4", 117 * MIB, 3_542_400, 1),
    ("grafana/grafana:11.2", "sha256:f9a1c2b7e413", 455 * MIB, 3_542_400, 1),
    ("<none>", "sha256:0bad1dea5e11", 640 * MIB, 6_048_000, 0),
];

/// The state of each container after the user's actions (restart → busy, then settles).
pub fn initial_states() -> Vec<RunState> { CONTAINERS.iter().map(|c| c.state).collect() }

// ───────────────────────────── elements ─────────────────────────────

/// Columns by priority: Name and State always; Image only when there's room (DESIGN.md: drop, don't squeeze).
fn ct_cols(width: u16) -> Vec<Column<'static>> {
    let mut c = vec![Column { header: "State", width: 15, right: false }, Column { header: "Name", width: 0, right: false }];
    if width >= 100 { c.push(Column { header: "Image", width: 20, right: false }); }
    c.extend([
        Column { header: "CPU", width: 10, right: false },
        Column { header: "Memory", width: 18, right: false },
        Column { header: "Up", width: 7, right: true },
    ]);
    c
}

/// ContainerTable: every container of the host, with state, CPU trend, memory of its limit.
/// States: daemon unreachable (error), no containers (empty), loading (skeleton rows), many (only what fits).
pub struct ContainerTable<'a> {
    pub states: &'a [RunState],
    pub selected: usize,
    pub focused: bool,
    pub tick: usize,
    pub reachable: bool,
    pub loading: bool,
}

impl Widget for ContainerTable<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let up = self.states.iter().filter(|s| s.is_up()).count();
        let title = format!("Containers · {up}/{} up", self.states.len());
        let block = atoms::panel(&title, self.focused);
        let inner = block.inner(area);
        block.render(area, buf);
        if !self.reachable {
            ErrorState { what: "Can't reach the Docker daemon on prod-1.", why: "ssh: connect to prod-1:22 timed out after 10s", retry_focused: true }.render(inner, buf);
            return;
        }
        if self.loading {
            for i in 0..inner.height.min(8) { atoms::Skeleton(0.4 + 0.07 * (i % 5) as f64).render(Rect { y: inner.y + i * 2, height: 1, ..inner }, buf); }
            return;
        }
        if self.states.is_empty() {
            EmptyState { message: "No containers on this host.", hint: Some(("d", "to deploy a stack")) }.render(inner, buf);
            return;
        }
        let cols = ct_cols(inner.width);
        let wide = cols.len() == 6;
        let rows: Vec<Vec<String>> = CONTAINERS.iter().map(|c| {
            let mut r = vec![String::new(), format!("{}/{}", c.stack, c.name)];
            if wide { r.push(c.image.into()); }
            r.extend([String::new(), String::new(), if c.up > 0 { fmt::uptime(c.up) } else { "–".into() }]);
            r
        }).collect();
        let row_refs: Vec<Vec<&str>> = rows.iter().map(|r| r.iter().map(String::as_str).collect()).collect();
        let row_slices: Vec<&[&str]> = row_refs.iter().map(Vec::as_slice).collect();
        let table = Table { columns: &cols, rows: &row_slices, selected: Some(self.selected), focused: self.focused };
        let cells: Vec<Vec<Rect>> = (0..CONTAINERS.len()).map(|i| table.cell_areas(inner, i)).collect();
        table.render(inner, buf);
        let (cpu_i, mem_i) = if wide { (3, 4) } else { (2, 3) };
        for ((c, st), cell) in CONTAINERS.iter().zip(self.states).zip(cells).take(inner.height.saturating_sub(1) as usize) {
            Line::from(atoms::run_state(*st, self.tick)).render(cell[0], buf);
            if st.is_up() {
                atoms::Sparkline::new(c.cpu).max(100.0).budget(80.0).render(cell[cpu_i], buf);
                let v = fmt::bytes(c.mem);
                Meter { label: "", label_width: 0, fraction: c.mem as f64 / c.limit as f64, value: &v, value_width: 8 }.render(cell[mem_i], buf);
            } else {
                Line::from(atoms::label("–", Tone::Faint)).render(cell[cpu_i], buf);
            }
        }
    }
}

/// ContainerInspector: the selected container. Facts, usage, ports, environment (secrets masked), actions.
pub struct ContainerInspector<'a> {
    pub container: Option<&'a Container>,
    pub state: RunState,
    pub revealed: bool,
    pub tick: usize,
    pub focused: bool,
    /// The focused button (Some while the zone has focus).
    pub button: Option<usize>,
}

impl ContainerInspector<'_> {
    /// The buttons (the containers keymap's action ids); Stop or Start by state.
    pub fn buttons(up: bool) -> &'static [AB<'static>] {
        const UP: &[AB<'static>] = &[AB::new("Restart", "restart").primary(), AB::new("Stop", "stop"), AB::new("Logs", "logs"), AB::new("Remove", "remove").danger()];
        const DOWN: &[AB<'static>] = &[AB::new("Restart", "restart").primary(), AB::new("Start", "stop"), AB::new("Logs", "logs"), AB::new("Remove", "remove").danger()];
        if up { UP } else { DOWN }
    }
}

impl Widget for ContainerInspector<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = atoms::panel("Container", self.focused);
        let inner = block.inner(area);
        block.render(area, buf);
        let Some(c) = self.container else {
            EmptyState { message: "Select a container.", hint: None }.render(inner, buf);
            return;
        };
        let env_h = c.env.len().max(1) as u16;
        let [name, state, _, facts, _, cpu, mem, _, ports_l, ports, _, env_l, env, _, vols, rest] = Layout::vertical([
            Constraint::Length(1), Constraint::Length(1), Constraint::Length(1), Constraint::Length(4), Constraint::Length(1),
            Constraint::Length(1), Constraint::Length(1), Constraint::Length(1), Constraint::Length(1), Constraint::Length(c.ports.len().max(1) as u16),
            Constraint::Length(1), Constraint::Length(1), Constraint::Length(env_h), Constraint::Length(1), Constraint::Length(1), Constraint::Min(0),
        ]).areas(inner);
        Line::from(vec![atoms::label(&format!("{} / {}", c.stack, c.name), Tone::Title)]).render(name, buf);
        Line::from(atoms::run_state(self.state, self.tick)).render(state, buf);
        let id: String = c.id.trim_start_matches("sha256:").chars().take(12).collect();
        let up = if self.state.is_up() { fmt::uptime(c.up) } else { "–".into() };
        PropertyList(&[("Image", c.image), ("Id", &id), ("Uptime", &up), ("Host", "prod-1")]).render(facts, buf);
        let cpu_now = c.cpu.last().copied().unwrap_or(0.0);
        let cpu_v = format!("{cpu_now:.0} %");
        Meter { label: "CPU", label_width: 7, fraction: cpu_now / 100.0, value: &cpu_v, value_width: 12 }.render(cpu, buf);
        let mem_v = fmt::bytes_of(c.mem, c.limit);
        Meter { label: "Memory", label_width: 7, fraction: c.mem as f64 / c.limit as f64, value: &mem_v, value_width: 12 }.render(mem, buf);
        Line::from(atoms::label("Ports", Tone::Heading)).render(ports_l, buf);
        if c.ports.is_empty() { Line::from(atoms::label("none published", Tone::Faint)).render(ports, buf); }
        for (i, p) in c.ports.iter().enumerate() {
            Line::from(p.spans()).render(Rect { y: ports.y + i as u16, height: 1, ..ports }, buf);
        }
        Line::from(vec![atoms::label("Environment", Tone::Heading), atoms::label(if self.revealed { "  e hide" } else { "  e show secrets" }, Tone::Faint)]).render(env_l, buf);
        if c.env.is_empty() { Line::from(atoms::label("none", Tone::Faint)).render(env, buf); }
        KeyValueRows { rows: c.env, key_width: 14, revealed: self.revealed }.render(env, buf);
        Line::from(vec![atoms::label("Volumes  ", Tone::Muted), atoms::label(&c.volumes.join(", "), Tone::Text)]).render(vols, buf);
        if rest.height >= 2 {
            let b = Rect { y: rest.bottom() - 1, height: 1, ..rest };
            AB::group(Self::buttons(self.state.is_up()), self.button).render(b, buf);
        }
    }
}

/// StackTree: stacks with how many containers are up, worst state first in color.
pub struct StackTree {
    pub focused: bool,
}

impl Widget for StackTree {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = atoms::panel("Stacks", self.focused);
        let inner = block.inner(area);
        block.render(area, buf);
        let rows = flatten(STACKS, &|_| false);
        TreeView { rows: &rows, selected: 0, focused: self.focused }.render(inner, buf);
    }
}

/// HostCard: one host. Usage meters, containers up, CPU trend. Unreachable → ErrorState with Retry.
pub struct HostCard<'a> {
    pub host: &'a Host,
    pub selected: bool,
}

impl Widget for HostCard<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let h = self.host;
        let block = atoms::panel(h.name, self.selected);
        let inner = block.inner(area);
        block.render(area, buf);
        if !h.reachable {
            ErrorState { what: "Can't reach db-2.", why: "no answer for 4 min · last seen 13:58", retry_focused: self.selected }.render(inner, buf);
            return;
        }
        let [role, _, cpu, mem, disk, _, trend] = Layout::vertical([Constraint::Length(1); 7]).areas(inner);
        let st = if h.up.0 == h.up.1 { atoms::status_label(&format!("{}/{} up", h.up.0, h.up.1), Status::Ok) } else { atoms::status_label(&format!("{}/{} up", h.up.0, h.up.1), Status::Warn) };
        Line::from(vec![atoms::label(h.role, Tone::Muted), Span::raw("  "), st, atoms::label(&format!("  · {}", fmt::uptime(h.uptime)), Tone::Faint)]).render(role, buf);
        let c = h.cpu.last().copied().unwrap_or(0.0);
        let cv = format!("{c:.0} %");
        Meter { label: "CPU", label_width: 6, fraction: c / 100.0, value: &cv, value_width: 14 }.render(cpu, buf);
        let mv = fmt::bytes_of(h.mem.0, h.mem.1);
        Meter { label: "Memory", label_width: 6, fraction: h.mem.0 as f64 / h.mem.1 as f64, value: &mv, value_width: 14 }.render(mem, buf);
        let dv = fmt::bytes_of(h.disk.0, h.disk.1);
        Meter { label: "Disk", label_width: 6, fraction: h.disk.0 as f64 / h.disk.1 as f64, value: &dv, value_width: 14 }.render(disk, buf);
        let [l, s] = Layout::horizontal([Constraint::Length(7), Constraint::Min(0)]).areas(trend);
        Line::from(atoms::label("1h", Tone::Faint)).render(l, buf);
        atoms::Sparkline::new(h.cpu).max(100.0).budget(80.0).render(s, buf);
    }
}

/// NeedsAttention: what's wrong across hosts, worst first, each with where it is.
pub struct NeedsAttention;

impl Widget for NeedsAttention {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = atoms::panel("Needs attention", false);
        let inner = block.inner(area);
        block.render(area, buf);
        let items: [(Status, &str, &str); 5] = [
            (Status::Err, "db-2 unreachable", "4 min"),
            (Status::Err, "prometheus crashed: out of memory (137)", "prod-1"),
            (Status::Warn, "api-2 restarted 5× in 10 min", "prod-1"),
            (Status::Warn, "backup-1 disk at 91 %", "backup-1"),
            (Status::Warn, "port 6379 (redis) public", "prod-1"),
        ];
        for (i, (k, t, m)) in items.iter().enumerate().take(inner.height as usize) {
            (&ListRow { lead: Some(atoms::status_icon(*k)), title: t, meta: m, meta_width: 9, ..Default::default() }).render(Rect { y: inner.y + i as u16, height: 1, ..inner }, buf);
        }
    }
}

/// Release: what is being deployed, where, and the steps (vertical).
pub struct Release<'a> {
    pub steps: &'a [Step<'a>],
    pub tick: usize,
}

impl Widget for Release<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = atoms::panel("Release", false);
        let inner = block.inner(area);
        block.render(area, buf);
        let [facts, _, l, steps] = Layout::vertical([Constraint::Length(5), Constraint::Length(1), Constraint::Length(1), Constraint::Min(0)]).areas(inner);
        PropertyList(&[("Stack", "shop"), ("From", "1.8.2"), ("To", "1.9.0"), ("Hosts", "prod-1, prod-2"), ("Strategy", "start new, then stop old")]).render(facts, buf);
        Line::from(atoms::label("Steps", Tone::Heading)).render(l, buf);
        StepList { steps: self.steps, inline: false, tick: self.tick }.render(steps, buf);
    }
}

/// ImageTable: images with size, age and how many containers use them; unused ones say so.
pub struct ImageTable {
    pub selected: usize,
    pub focused: bool,
}

impl Widget for ImageTable {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let unused: u64 = IMAGES.iter().filter(|i| i.4 == 0).map(|i| i.2).sum();
        let title = format!("Images · {} unused, {}", IMAGES.iter().filter(|i| i.4 == 0).count(), fmt::bytes(unused));
        let block = atoms::panel(&title, self.focused);
        let inner = block.inner(area);
        block.render(area, buf);
        let cols = [
            Column { header: "Image", width: 0, right: false }, Column { header: "Id", width: 12, right: false },
            Column { header: "Size", width: 8, right: true }, Column { header: "Created", width: 9, right: true }, Column { header: "Used by", width: 8, right: false },
        ];
        let rows: Vec<[String; 5]> = IMAGES.iter().map(|i| [i.0.into(), String::new(), fmt::bytes(i.2), fmt::age(i.3), String::new()]).collect();
        let refs: Vec<Vec<&str>> = rows.iter().map(|r| r.iter().map(String::as_str).collect()).collect();
        let sl: Vec<&[&str]> = refs.iter().map(Vec::as_slice).collect();
        let t = Table { columns: &cols, rows: &sl, selected: Some(self.selected), focused: self.focused };
        let cells: Vec<Vec<Rect>> = (0..IMAGES.len()).map(|i| t.cell_areas(inner, i)).collect();
        t.render(inner, buf);
        for (i, cell) in IMAGES.iter().zip(cells).take(inner.height.saturating_sub(1) as usize) {
            Line::from(atoms::short_id(i.1)).render(cell[1], buf);
            let used = if i.4 == 0 { atoms::label("unused", Tone::Faint) } else { atoms::label(&format!("{} ctr", i.4), Tone::Muted) };
            Line::from(used).render(cell[4], buf);
        }
    }
}

/// DiskUse: where the host's disk goes, and the prune action.
pub struct DiskUse {
    /// The focused button (Some while the zone has focus).
    pub button: Option<usize>,
}

impl DiskUse {
    pub const BUTTONS: &'static [AB<'static>] = &[AB::new("Prune…", "prune").danger()];
}

impl Widget for DiskUse {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = atoms::panel("Disk · prod-1", self.button.is_some());
        let inner = block.inner(area);
        block.render(area, buf);
        let [m, _, chart, _, note, rest] = Layout::vertical([Constraint::Length(1), Constraint::Length(1), Constraint::Length(4), Constraint::Length(1), Constraint::Length(2), Constraint::Min(0)]).areas(inner);
        let v = fmt::bytes_of(41 * GIB, 80 * GIB);
        Meter { label: "Used", label_width: 5, fraction: 41.0 / 80.0, value: &v, value_width: 12 }.render(m, buf);
        BarChart { items: &[("volumes", 18.2), ("images", 6.1), ("build cache", 2.3), ("containers", 0.4)], unit: "GiB", max: None, budget: None }.render(chart, buf);
        let unused: u64 = IMAGES.iter().filter(|i| i.4 == 0).map(|i| i.2).sum();
        ratatui::widgets::Paragraph::new(atoms::label(&format!("Prune removes {} unused images and the build cache: {}.", IMAGES.iter().filter(|i| i.4 == 0).count(), fmt::bytes(unused + 2_469_606_195)), Tone::Muted))
            .wrap(ratatui::widgets::Wrap { trim: true }).render(note, buf);
        if rest.height >= 2 {
            AB::group(Self::BUTTONS, self.button).render(Rect { y: rest.bottom() - 1, height: 1, ..rest }, buf);
        }
    }
}

// ───────────────────────────── pages ─────────────────────────────

const LIVE: Binding = Binding::new("↑ ↓", "previous / next", "Move").hint("move").action("move");

pub const CONTAINERS_KEYS: Keymap = Keymap(&[
    LIVE,
    Binding::new("r", "restart", "Act").hint("restart").command("Restart container").action("restart"),
    Binding::new("s", "stop / start", "Act").hint("stop/start").command("Stop or start container").action("stop"),
    Binding::new("l", "follow its logs", "Act").command("Show logs").action("logs"),
    Binding::new("e", "show / hide secret env values", "Act").command("Show secrets").action("reveal"),
    Binding::new("del", "remove (asks first)", "Act").command("Remove container").danger().action("remove"),
    crate::pages::DETAILS, quag_theme::keymap::BUTTONS, HELP, PALETTE, BACK,
]);
pub const HOSTS_KEYS: Keymap = Keymap(&[
    Binding::new("← →", "previous / next host", "Move").hint("move").action("move"),
    Binding::new("enter", "open host", "Move").hint("open").action("open"),
    Binding::new("r", "refresh now", "Act").hint("refresh").command("Refresh").action("refresh"),
    HELP, PALETTE, BACK,
]);
pub const DEPLOY_KEYS: Keymap = Keymap(&[
    Binding::new("d", "deploy (type the stack name: prod)", "Act").hint("deploy").command("Deploy shop 1.9.0").danger().action("deploy"),
    Binding::new("z", "roll back to 1.8.2 (asks first)", "Act").hint("roll back").command("Roll back").action("rollback"),
    Binding::new("f", "demo: make the health check fail", "Demo").command("Demo: fail next deploy").action("fail"),
    HELP, PALETTE, BACK,
]);
pub const IMAGES_KEYS: Keymap = Keymap(&[
    LIVE,
    Binding::new("p", "prune unused (type to confirm)", "Act").hint("prune").command("Prune unused images").danger().action("prune"),
    crate::pages::DETAILS, quag_theme::keymap::BUTTON, HELP, PALETTE, BACK,
]);

/// The status bar's right side: how fresh the live data is (yellow once stale).
fn fresh(age: u64) -> (Status, String) {
    let stale = quag_theme::consts::rules::STALE_AFTER_SECS;
    (if age > stale { Status::Warn } else { Status::Ok }, freshness(age, stale).content.into_owned())
}

/// Settle busy containers (restarting → running) after a few ticks, like the daemon would.
fn settle(st: &mut PageState) {
    
    for (i, p) in st.ct_pending.iter_mut().enumerate() {
        if let Some((at, to)) = *p && st.tick >= at {
            st.ct_states[i] = to;
            *p = None;
        }
    }
}

/// List → Detail: stacks · containers · the selected container.
pub fn containers(f: &mut Frame, area: Rect, st: &mut PageState) {
    settle(st);
    let fr = fresh(C::AGE_FRESH_SECS);
    let Some(s) = frame_with(f, area, Zones { nav: true, inspector: true }, "dock", Some(Env::Prod), "containers · prod-1", None, CONTAINERS_KEYS, Some(StatusLine { kind: fr.0, message: &fr.1 })) else { return };
    if let Some(nav) = s.nav { f.render_widget(StackTree { focused: false }, nav); }
    let sel = st.ct_sel.min(CONTAINERS.len() - 1);
    f.render_widget(ContainerTable { states: &st.ct_states, selected: sel, focused: st.focus != 2, tick: st.tick, reachable: true, loading: false }, s.content);
    if let Some(i) = s.inspector {
        f.render_widget(ContainerInspector { container: CONTAINERS.get(sel), state: st.ct_states[sel], revealed: st.revealed, tick: st.tick, focused: st.focus == 2, button: st.button_in(2) }, i);
    }
    if st.confirm_delete {
        let detail = format!("“{}” is removed; its volumes are kept.", CONTAINERS[sel].name);
        let d = ConfirmDialog { title: "Remove container", question: "Remove this container?", detail: &detail, confirm: "Remove", cancel: "Keep", danger: true, focused: st.confirm_btn };
        let r = d.area(area);
        f.render_widget(d, r);
    }
    let msg = st.toast_msg.clone();
    overlays(f, area, st, CONTAINERS_KEYS, &msg);
}

/// Dashboard: every host, and what needs attention.
pub fn hosts(f: &mut Frame, area: Rect, st: &mut PageState) {
    settle(st);
    let fr = fresh(C::AGE_STALE_SECS);
    let Some(s) = frame_with(f, area, Zones { nav: false, inspector: false }, "dock", Some(Env::Prod), "hosts", None, HOSTS_KEYS, Some(StatusLine { kind: fr.0, message: &fr.1 })) else { return };
    let [cards, _, att] = Layout::vertical([Constraint::Length(20), Constraint::Length(1), Constraint::Min(4)]).areas(s.content);
    let [r1, r2] = Layout::vertical([Constraint::Length(10); 2]).areas(cards);
    let mut slots = vec![];
    for r in [r1, r2] {
        let [a, _, b] = Layout::horizontal([Constraint::Fill(1), Constraint::Length(1), Constraint::Fill(1)]).areas(r);
        slots.extend([a, b]);
    }
    for (i, (h, r)) in HOSTS.iter().zip(slots).enumerate() {
        f.render_widget(HostCard { host: h, selected: i == st.host_sel.min(HOSTS.len() - 1) }, r);
    }
    f.render_widget(NeedsAttention, att);
    let msg = st.toast_msg.clone();
    overlays(f, area, st, HOSTS_KEYS, &msg);
}

/// Workspace: review the change → run it → result. The run is a zone, not a modal: the app stays usable.
pub fn deploy(f: &mut Frame, area: Rect, st: &mut PageState) {
    settle(st);
    // how far the deploy got: one step every 1.5 s
    let done = st.deploy_start.map(|t0| (st.tick.saturating_sub(t0)) / C::DEPLOY_TICKS_PER_STEP);
    let fail_at = if st.deploy_fail { Some(C::DEPLOY_FAIL_STEP) } else { None };
    let steps: Vec<Step> = STEP_NAMES.iter().enumerate().map(|(i, n)| {
        let state = match done {
            None => StepState::Waiting,
            Some(d) if fail_at.is_some_and(|fa| i == fa && d > fa) => StepState::Failed,
            Some(d) if fail_at.is_some_and(|fa| i > fa && d > fa) => StepState::Skipped,
            Some(d) if i < d => StepState::Done,
            Some(d) if i == d => StepState::Running,
            _ => StepState::Waiting,
        };
        Step { name: n, state, took: if state == StepState::Done { STEP_TOOK[i] } else { "" } }
    }).collect();
    let status = match done {
        None => StatusLine { kind: Status::Ok, message: "updated now" },
        Some(_) if steps.iter().any(|s| s.state == StepState::Failed) => StatusLine { kind: Status::Err, message: "failed · 1.8.2 still live" },
        Some(d) if d >= STEP_NAMES.len() => StatusLine { kind: Status::Ok, message: "shop 1.9.0 live" },
        Some(_) => StatusLine { kind: Status::Info, message: "deploying…" },
    };
    let Some(s) = frame_with(f, area, Zones { nav: false, inspector: true }, "dock", Some(Env::Prod), "deploy · shop", None, DEPLOY_KEYS, Some(status)) else { return };
    match done {
        None => f.render_widget(DiffView { title: "Changes · compose.yaml", lines: DIFF, scroll: 0, focused: true }, s.content),
        Some(d) => {
            let n_lines = (d * 2).min(DEPLOY_LOG.len());
            let mut log: Vec<LogEntry> = DEPLOY_LOG[..n_lines].iter().map(|(t, k, m)| LogEntry { time: t, kind: *k, message: m }).collect();
            if steps.iter().any(|s| s.state == StepState::Failed) {
                log.truncate(6);
                log.push(LogEntry { time: FAIL_LOG.0, kind: FAIL_LOG.1, message: FAIL_LOG.2 });
            }
            f.render_widget(RunProgress { title: "Deploy shop 1.9.0 → prod", steps: &steps, output: &log, tick: st.tick, focused: true }, s.content);
        }
    }
    if let Some(i) = s.inspector { f.render_widget(Release { steps: &steps, tick: st.tick }, i); }
    if let Some(t) = &st.confirm_text {
        let d = TypeToConfirm { title: "Deploy to prod", question: "Deploy shop 1.9.0 to prod-1 and prod-2?", name: st.confirm_name, typed: t, confirm: "Deploy" };
        let r = d.area(area);
        f.render_widget(d, r);
    }
    if st.confirm_delete {
        let d = ConfirmDialog { title: "Roll back", question: "Roll back shop to 1.8.2?", detail: "Containers restart one by one; no downtime.", confirm: "Roll back", cancel: "Cancel", danger: false, focused: st.confirm_btn };
        let r = d.area(area);
        f.render_widget(d, r);
    }
    let msg = st.toast_msg.clone();
    overlays(f, area, st, DEPLOY_KEYS, &msg);
}

/// List → Detail: images · disk use and prune.
pub fn images(f: &mut Frame, area: Rect, st: &mut PageState) {
    settle(st);
    let fr = fresh(C::AGE_FRESH_SECS);
    let Some(s) = frame_with(f, area, Zones { nav: false, inspector: true }, "dock", Some(Env::Prod), "images · prod-1", None, IMAGES_KEYS, Some(StatusLine { kind: fr.0, message: &fr.1 })) else { return };
    f.render_widget(ImageTable { selected: st.img_sel.min(IMAGES.len() - 1), focused: st.focus != 2 }, s.content);
    if let Some(i) = s.inspector { f.render_widget(DiskUse { button: st.button_in(2) }, i); }
    if let Some(t) = &st.confirm_text {
        let d = TypeToConfirm { title: "Prune images", question: "Delete 4 unused images and the build cache?", name: st.confirm_name, typed: t, confirm: "Prune" };
        let r = d.area(area);
        f.render_widget(d, r);
    }
    let msg = st.toast_msg.clone();
    overlays(f, area, st, IMAGES_KEYS, &msg);
}

/// A ConfirmDialog of a dock page was confirmed.
pub fn confirmed(st: &mut PageState, page: &str) -> bool {
    match page {
        "containers" => { let n = CONTAINERS[st.ct_sel.min(CONTAINERS.len() - 1)].name; st.toast(&format!("Removed {n} (demo: it stays listed)")); }
        "deploy" => { st.deploy_start = None; st.toast("Rolled back to 1.8.2"); }
        _ => return false,
    }
    true
}

/// Key actions of the dock pages (called from main.rs `page_key`).
pub fn key(st: &mut PageState, page: &str, action: &str, name: &str) -> bool {
    let toast = |st: &mut PageState, m: &str| { st.toast_msg = m.into(); st.overlay = crate::pages::Overlay::Toast; };
    let sel = st.ct_sel.min(CONTAINERS.len() - 1);
    match (page, action) {
        ("containers", "move") => st.ct_sel = if name == "↓" { (st.ct_sel + 1).min(CONTAINERS.len() - 1) } else { st.ct_sel.saturating_sub(1) },
        ("containers", "restart") => {
            st.ct_states[sel] = RunState::Restarting;
            st.ct_pending[sel] = Some((st.tick + C::RESTART_TICKS, RunState::Running));
            toast(st, &format!("Restarting {}", CONTAINERS[sel].name));
        }
        ("containers", "stop") => {
            let up = st.ct_states[sel].is_up();
            st.ct_states[sel] = if up { RunState::Stopping } else { RunState::Starting };
            st.ct_pending[sel] = Some((st.tick + C::STOP_START_TICKS, if up { RunState::Stopped } else { RunState::Running }));
        }
        ("containers", "reveal") => st.revealed = !st.revealed,
        ("containers", "logs") => toast(st, &format!("Following logs of {}", CONTAINERS[sel].name)),
        ("containers", "remove") => st.ask(true),
        ("hosts", "move") => st.host_sel = if name == "→" { (st.host_sel + 1).min(HOSTS.len() - 1) } else { st.host_sel.saturating_sub(1) },
        ("hosts", "open") => toast(st, "Host page: not in the demo"),
        ("hosts", "refresh") => toast(st, "Refreshed 4 hosts"),
        ("deploy", "deploy") => { st.confirm_text = Some(String::new()); st.confirm_name = "shop"; st.confirm_for = "deploy"; }
        ("deploy", "rollback") => st.ask(false),
        ("deploy", "fail") => { st.deploy_fail = !st.deploy_fail; toast(st, if st.deploy_fail { "Next deploy fails its health check" } else { "Next deploy succeeds" }) }
        ("images", "move") => st.img_sel = if name == "↓" { (st.img_sel + 1).min(IMAGES.len() - 1) } else { st.img_sel.saturating_sub(1) },
        ("images", "prune") => { st.confirm_text = Some(String::new()); st.confirm_name = "prune"; st.confirm_for = "prune"; }
        _ => return false,
    }
    true
}
