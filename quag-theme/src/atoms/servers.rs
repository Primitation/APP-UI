//! ATOM: Servers: RunState, Env, short_id, diff.

use crate::{
    self as q, Status,
    consts::{format as F, glyph as G},
};
use ratatui::{
    style::Modifier,
    text::{Line, Span},
};

/// State of a container / service / job. Stopped is NOT an error (grey); only crashes are red.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RunState {
    Running,
    Starting,
    Stopping,
    Restarting,
    Stopped,
    Paused,
    /// Exited with this code (0 = finished fine, shown like Stopped).
    Exited(i32),
    /// Running, but restarted this many times recently: more than `rules::FLAPPING_RESTARTS`
    /// within `rules::FLAPPING_WINDOW_SECS` (restart loop). The app decides; this only shows it.
    Flapping(u32),
}

impl RunState {
    pub fn label(self) -> String {
        match self {
            RunState::Running => "running".into(),
            RunState::Starting => "starting".into(),
            RunState::Stopping => "stopping".into(),
            RunState::Restarting => "restarting".into(),
            RunState::Stopped => "stopped".into(),
            RunState::Paused => "paused".into(),
            RunState::Exited(0) => "exited 0".into(),
            RunState::Exited(c) => format!("crashed {c}"),
            RunState::Flapping(n) => format!("restarted {n}×"),
        }
    }
    pub fn is_busy(self) -> bool { matches!(self, RunState::Starting | RunState::Stopping | RunState::Restarting) }
    pub fn is_up(self) -> bool { matches!(self, RunState::Running | RunState::Flapping(_)) }
    /// Ok / Warn / Err, or None for neutral (stopped, paused, exited 0).
    pub fn status(self) -> Option<Status> {
        match self {
            RunState::Running => Some(Status::Ok),
            RunState::Starting | RunState::Stopping | RunState::Restarting | RunState::Flapping(_) => Some(Status::Warn),
            RunState::Exited(c) if c != 0 => Some(Status::Err),
            _ => None,
        }
    }
}

/// `● running` · `⠋ starting` · `○ stopped` · `‖ paused` · `✗ crashed 137` · `! restarted 5×`.
/// Busy states animate with `tick`. Icon + word, never color alone.
pub fn run_state(s: RunState, tick: usize) -> Vec<Span<'static>> {
    let icon = match s {
        RunState::Running => "●",
        _ if s.is_busy() => G::SPINNER[tick % G::SPINNER.len()],
        RunState::Paused => "‖",
        RunState::Exited(c) if c != 0 => "✗",
        RunState::Flapping(_) => "!",
        _ => "○",
    };
    let style = s.status().map_or(q::muted(), Status::style);
    vec![Span::styled(icon, style), Span::raw(" "), Span::styled(s.label(), style)]
}

/// Deployment environment, shown in the header of every page so you always know where you are.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Env {
    Prod,
    Staging,
    Dev,
}

/// `[prod]` bold red · `[staging]` yellow · `[dev]` grey.
pub fn env_tag(e: Env) -> Span<'static> {
    match e {
        Env::Prod => Span::styled("[prod]", Status::Err.style().add_modifier(Modifier::BOLD)),
        Env::Staging => Span::styled("[staging]", Status::Warn.style()),
        Env::Dev => Span::styled("[dev]", q::muted()),
    }
}

/// An id or digest shortened to 12 characters (`sha256:` dropped), faint so it doesn't compete with names.
pub fn short_id(id: &str) -> Span<'static> {
    let id = id.strip_prefix("sha256:").unwrap_or(id);
    Span::styled(id.chars().take(F::SHORT_ID_LEN).collect::<String>(), q::faint())
}

/// One line of a diff.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DiffKind {
    Same,
    Add,
    Del,
    /// `@@ services.api @@`: where the next lines are.
    Hunk,
}

/// `+ added` (green +, normal text) · `- removed` (red −, muted text) · `  same` (faint) · `@@ hunk` (Gill).
pub fn diff_line(kind: DiffKind, text: &str) -> Line<'static> {
    match kind {
        DiffKind::Add => Line::from(vec![Span::styled("+ ", Status::Ok.style()), Span::styled(text.to_string(), q::text())]),
        DiffKind::Del => Line::from(vec![Span::styled("− ", Status::Err.style()), Span::styled(text.to_string(), q::muted().add_modifier(Modifier::CROSSED_OUT))]),
        DiffKind::Same => Line::from(vec![Span::raw("  "), Span::styled(text.to_string(), q::faint())]),
        DiffKind::Hunk => Line::from(Span::styled(text.to_string(), q::special())),
    }
}

/// State of one step in a pipeline.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StepState {
    Waiting,
    Running,
    Done,
    Failed,
    Skipped,
}

/// `✓` done · spinner running · `○` waiting · `✗` failed · `–` skipped.
pub fn step_icon(s: StepState, tick: usize) -> Span<'static> {
    match s {
        StepState::Done => Span::styled("✓", Status::Ok.style()),
        StepState::Running => Span::styled(G::SPINNER[tick % G::SPINNER.len()], q::accent()),
        StepState::Waiting => Span::styled("○", q::faint()),
        StepState::Failed => Span::styled("✗", Status::Err.style()),
        StepState::Skipped => Span::styled("–", q::faint()),
    }
}
