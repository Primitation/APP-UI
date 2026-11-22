//! This app's own numbers (timing, limits). The design system's numbers are in `crate::quag::consts`:
//! use those for anything about the look (sizes, thresholds, formats). No magic numbers in the code.

use std::time::Duration;

/// Redraw at least this often (spinners, live data).
pub const FRAME: Duration = Duration::from_millis(250);
