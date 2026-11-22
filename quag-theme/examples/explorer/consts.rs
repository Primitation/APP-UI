//! Numbers of the demo apps (pond, dock): timing and limits of the simulation, not of the design system.
//! The design system's own numbers are in `quag_theme::consts`.

use std::time::Duration;

use quag_theme::consts::motion::{TICK, ticks_in};

/// Redraw at least this often: every animation tick (spinners, settling states, the deploy run).
pub const FRAME: Duration = TICK;
/// How long a toast stays.
pub const TOAST: Duration = Duration::from_secs(3);

/// pond · settings: reminder lead time, in minutes.
pub const LEAD_STEP_MIN: f64 = 5.0;
pub const LEAD_MIN: f64 = 0.0;
pub const LEAD_MAX: f64 = 120.0;
/// pond · workspace: the editor keeps between these shares of the width.
pub const SPLIT_MIN: f32 = 0.2;
pub const SPLIT_MAX: f32 = 0.9;
/// pond · log page: lines in the demo log.
pub const LOG_LINES: usize = 20_000;

/// dock · how long a container takes to restart / stop / start (2.5 s / 2 s), in ticks.
pub const RESTART_TICKS: usize = ticks_in(2_500);
pub const STOP_START_TICKS: usize = ticks_in(2_000);
/// dock · deploy: ticks per step (1.5 s), and the step that fails in "demo: fail" mode (health check).
pub const DEPLOY_TICKS_PER_STEP: usize = ticks_in(1_500);
pub const DEPLOY_FAIL_STEP: usize = 3;
/// dock · ages shown by the demo's Freshness: fresh, and stale (hosts page).
pub const AGE_FRESH_SECS: u64 = 3;
pub const AGE_STALE_SECS: u64 = 45;

/// forge · balance: a unit whose DPS per gold is this many times the median (or below median / this) is an outlier.
pub const OUTLIER_RATIO: f64 = 1.25;
/// forge · sequence: the playhead moves in real time while playing; per ← / → press it moves this much (seconds).
pub const SCRUB_SECS: f64 = 0.5;
/// forge · graph: ctrl + arrows pan the view this many cells.
pub const PAN_STEP: i32 = 4;
