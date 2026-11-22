//! App data. Elements read from here; atoms and molecules never do.
use quag_theme::Status;

#[derive(Clone, Copy)]
pub struct Task {
    pub done: bool,
    pub title: &'static str,
    pub tag: &'static str,
    pub due: &'static str,
    pub state: Option<Status>,
    pub notes: &'static str,
    /// Index in PROJECTS (never 0: "All" is not a project).
    pub project: usize,
}

pub const TASKS: &[Task] = &[
    Task { done: false, title: "Fix save crash on empty file", tag: "bug", due: "today", state: Some(Status::Err), notes: "Repro: open, clear, Ctrl+S.", project: 1 },
    Task { done: false, title: "Write README for quag-theme", tag: "docs", due: "today", state: None, notes: "Keep the API tiny; link STRUCTURE.md.", project: 1 },
    Task { done: false, title: "Port settings screen to theme", tag: "ui", due: "tomorrow", state: Some(Status::Warn), notes: "", project: 1 },
    Task { done: true, title: "Pick a color palette", tag: "ui", due: "done", state: Some(Status::Ok), notes: "Quagsire.", project: 1 },
    Task { done: false, title: "Backup tablet partitions", tag: "chore", due: "fri", state: None, notes: "", project: 3 },
    Task { done: false, title: "Shader test in Godot", tag: "game", due: "sat", state: Some(Status::Info), notes: "", project: 2 },
];

/// Projects in the nav; the first entry shows every task.
pub const PROJECTS: &[&str] = &["All", "quag-theme", "web-gl-shooter", "Tablet"];

/// Is this task in project `p` (0 = All)?
pub fn in_project(t: &Task, p: usize) -> bool { p == 0 || t.project == p }

/// Open tasks in project `p`.
pub fn open_in(tasks: &[Task], p: usize) -> usize { tasks.iter().filter(|t| !t.done && in_project(t, p)).count() }

/// The tasks page's tabs: Today (open, due today), Upcoming (open, due later), Done.
pub const TABS: &[&str] = &["Today", "Upcoming", "Done"];
pub fn in_tab(t: &Task, tab: usize) -> bool {
    match tab { 0 => !t.done && t.due == "today", 1 => !t.done && t.due != "today", _ => t.done }
}
/// What an empty tab says, and the key that helps.
pub const TAB_EMPTY: &[(&str, (&str, &str))] = &[
    ("Nothing due today.", ("2", "to see what's next")),
    ("Nothing coming up.", ("n", "to add a task")),
    ("Nothing finished yet.", ("1", "to see what's on")),
];

/// Dashboard: the project a row of PROJECT_ROWS opens (Inbox = every task).
pub fn project_of_row(row: usize) -> usize {
    PROJECT_ROWS.get(row).and_then(|r| PROJECTS.iter().position(|p| *p == r[0])).unwrap_or(0)
}

pub const LOG: &[(&str, Status, &str)] = &[
    ("12:01:03", Status::Info, "Connected to sync server"),
    ("12:01:09", Status::Warn, "Slow response (812 ms)"),
    ("12:01:12", Status::Err, "Upload failed: timeout after 30 s"),
    ("12:01:15", Status::Info, "Retrying upload (1/3)"),
    ("12:01:17", Status::Ok, "Upload complete: 4 tasks"),
    ("12:02:40", Status::Info, "Project 'quag-theme' opened"),
    ("12:03:02", Status::Ok, "Saved"),
];

/// The note open in the Workspace page (markdown source).
pub const NOTE: &[&str] = &[
    "# README for quag-theme",
    "",
    "Quag is the house style: calm, dark, Quagsire blue.",
    "",
    "## Use it",
    "- depend on quag-theme (path dependency)",
    "- build pages from elements, never from atoms",
    "- only atoms set colors",
    "",
    "## Levels",
    "atom → molecule → element → UI",
];

/// Projects table: name, open, done, last change.
pub const PROJECT_ROWS: &[&[&str]] = &[
    &["Inbox", "4", "12", "today"],
    &["quag-theme", "2", "31", "today"],
    &["web-gl-shooter", "7", "9", "3 days ago"],
    &["Tablet", "1", "4", "last week"],
];

/// Tasks done per day, last 14 days (dashboard sparkline).
pub const DONE_PER_DAY: &[f64] = &[3.0, 4.0, 2.0, 5.0, 3.0, 2.0, 5.0, 4.0, 5.0, 3.0, 6.0, 3.0, 4.0, 3.0];

/// Frame times in ms (optimisation demo), with a few spikes.
pub const FRAME_MS: &[f64] = &[
    12.1, 12.4, 11.9, 13.0, 12.2, 12.8, 14.1, 12.0, 11.8, 12.5, 19.8, 12.3, 12.1, 12.6, 13.2, 12.9, 12.0, 11.7, 12.4, 12.2,
    12.8, 13.5, 12.1, 24.6, 13.0, 12.4, 12.2, 11.9, 12.7, 12.3, 12.5, 12.0, 13.1, 12.6, 12.2, 17.2, 12.4, 12.1, 11.8, 12.6,
    12.9, 12.3, 12.0, 12.5, 13.3, 12.7, 12.2, 12.4, 12.1, 12.8, 12.0, 12.6, 18.4, 12.3, 12.1, 12.5, 12.2, 12.9, 12.4, 12.0,
];

/// GPU memory per asset type in MB (bar chart demo).
pub const MEMORY_MB: &[(&str, f64)] = &[("Textures", 812.0), ("Meshes", 236.0), ("Render targets", 410.0), ("Audio", 64.0), ("Other", 38.0)];

/// The project tree (workspace nav).
use quag_theme::molecules::TreeItem;
pub const TREE: &[TreeItem] = &[
    TreeItem::branch("inbox", "Inbox", &[TreeItem::leaf("inbox/crash", "Fix save crash"), TreeItem::leaf("inbox/backup", "Backup tablet")]),
    TreeItem::branch("qt", "quag-theme", &[
        TreeItem::branch("qt/docs", "docs", &[TreeItem::leaf("qt/docs/readme", "README.md"), TreeItem::leaf("qt/docs/structure", "STRUCTURE.md")]),
        TreeItem::branch("qt/src", "src", &[TreeItem::leaf("qt/src/atoms", "atoms.rs"), TreeItem::leaf("qt/src/molecules", "molecules.rs")]),
    ]),
    TreeItem::branch("gl", "web-gl-shooter", &[TreeItem::leaf("gl/shader", "Shader test")]),
    TreeItem::leaf("tablet", "Tablet"),
];

/// A big generated log (LogViewer demo): `n` lines.
pub fn big_log(n: usize) -> Vec<(String, Status, String)> {
    const MSG: &[(Status, &str)] = &[
        (Status::Info, "GET /api/tasks 200 in 12 ms"),
        (Status::Info, "GET /api/projects 200 in 8 ms"),
        (Status::Ok, "Sync complete: 4 tasks"),
        (Status::Warn, "Slow response from db (812 ms)"),
        (Status::Info, "POST /api/tasks 201 in 21 ms"),
        (Status::Err, "Upload failed: timeout after 30 s"),
        (Status::Warn, "Failed login for user 'admin' from 203.0.113.7"),
        (Status::Info, "Cache hit ratio 94 %"),
    ];
    (0..n)
        .map(|i| {
            let s = 8 * 3600 + i;
            let (k, m) = MSG[(i * 7 + i / 13) % MSG.len()];
            (format!("{:02}:{:02}:{:02}", s / 3600 % 24, s / 60 % 60, s % 60), k, m.to_string())
        })
        .collect()
}

/// Security findings (audit page): severity, title, host, age, detail, fix.
use quag_theme::atoms::Severity;
pub struct Finding {
    pub sev: Severity,
    pub title: &'static str,
    pub host: &'static str,
    pub age: &'static str,
    pub detail: &'static str,
    pub secret: Option<&'static str>,
}
pub const FINDINGS: &[Finding] = &[
    Finding { sev: Severity::Critical, title: "API key committed to repository", host: "api-1", age: "10m", detail: "Found in config/prod.env (commit 3f9a1c2).", secret: Some("sk_live_51HxQ2bK9vTz7m3f9a") },
    Finding { sev: Severity::High, title: "SSH allows password login", host: "db-1", age: "1d", detail: "PasswordAuthentication yes in sshd_config.", secret: None },
    Finding { sev: Severity::High, title: "12 failed admin logins from one IP", host: "api-1", age: "40m", detail: "203.0.113.7, 12 attempts in 4 minutes.", secret: None },
    Finding { sev: Severity::Medium, title: "TLS certificate expires in 9 days", host: "web-1", age: "6h", detail: "pond.example.net, Let's Encrypt, auto-renew failed.", secret: None },
    Finding { sev: Severity::Medium, title: "Port 6379 (Redis) open to the internet", host: "cache-1", age: "2d", detail: "Bound to 0.0.0.0, no auth.", secret: None },
    Finding { sev: Severity::Low, title: "Kernel update available", host: "db-1", age: "4d", detail: "6.8.0-45 → 6.8.0-48 (security fixes).", secret: None },
    Finding { sev: Severity::Info, title: "New user 'deploy' added", host: "web-1", age: "12d", detail: "By root, via ansible.", secret: None },
];

/// Firewall rules: #, action, port, source, hits.
pub const RULES: &[&[&str]] = &[
    &["1", "allow", "443/tcp", "any", "1.2 M"],
    &["2", "allow", "22/tcp", "10.0.0.0/8", "3 412"],
    &["3", "deny", "22/tcp", "any", "18 905"],
    &["4", "allow", "6379/tcp", "any", "211"],
    &["5", "deny", "all", "any", "52 117"],
];

pub const RANGES: &[&str] = &["15m", "1h", "24h", "7d", "30d"];

/// Audit: the time ranges (keys 1 – 5), in hours.
pub const RANGE_HOURS: &[f64] = &[0.25, 1.0, 24.0, 168.0, 720.0];
/// Age text ("40m", "6h", "2d") in hours.
pub fn hours(age: &str) -> f64 {
    let n: f64 = age[..age.len() - 1].parse().unwrap_or(0.0);
    match age.chars().last() { Some('m') => n / 60.0, Some('d') => n * 24.0, _ => n }
}
/// Findings inside time range `range` (index in RANGE_HOURS).
pub fn findings_in(range: usize) -> Vec<&'static Finding> {
    let max = RANGE_HOURS[range.min(RANGE_HOURS.len() - 1)];
    FINDINGS.iter().filter(|f| hours(f.age) <= max).collect()
}
