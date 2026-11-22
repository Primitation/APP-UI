/**
 * CONSTANTS: every number the parts share, named once (desktop side).
 * Colors and spacing are CSS tokens in `web/quag.css` (`--q-*`). Same values as the Rust `quag_theme::consts`:
 * change one, change the other.
 */

// ── Rules: thresholds that decide a color or a state (DESIGN.md) ──
/** Usage of a limit: yellow from here, red at USAGE_ERR. */
export const USAGE_WARN = 0.8;
export const USAGE_ERR = 1.0;
/** Live data older than this (seconds) is "stale" (yellow). */
export const STALE_AFTER_SECS = 30;
/** Restart loop: more than this many restarts within the window → warn even while running. */
export const FLAPPING_RESTARTS = 3;
export const FLAPPING_WINDOW_SECS = 600;
/** Split panes: share moved per arrow key, and the smallest share a pane keeps. */
export const SPLIT_STEP = 0.05;
export const SPLIT_MIN = 0.15;

// ── Formats ──
/** SecretField: characters left visible at the end; most mask dots shown. */
export const SECRET_TAIL = 4;
export const SECRET_MASK_MAX = 12;
export const MASK = "•";
/** Ids and digests are shortened to this many characters. */
export const SHORT_ID_LEN = 12;
/** Binary units; below ONE_DECIMAL_BELOW one decimal is shown (`1.4 GiB`), above none (`412 MiB`). */
export const BYTES_STEP = 1024;
export const BYTE_UNITS = ["B", "KiB", "MiB", "GiB", "TiB"] as const;
export const ONE_DECIMAL_BELOW = 10;
/** Time, in seconds. Younger than NOW_SECS reads "now". */
export const MINUTE = 60;
export const HOUR = 60 * MINUTE;
export const DAY = 24 * HOUR;
export const NOW_SECS = 5;
/** Badge: counts above this show as `99+`. */
export const BADGE_MAX = 99;
/** Breadcrumbs: more parts than this collapse the middle to `…`. */
export const BREADCRUMBS_MAX = 4;
/** fmtNum: ≥ 100 no decimals, ≥ 10 one, else two (≤ 3 significant digits). */
export const NUM_NO_DECIMALS_FROM = 100;
export const NUM_ONE_DECIMAL_FROM = 10;

// ── Timing (ms) ──
/** Toasts go away by themselves after this. */
export const TOAST_MS = 4000;
/** "Copied" feedback on copy buttons. */
export const COPIED_MS = 1500;

// ── Input ──
/** NumberField: Shift multiplies the step; dragging moves one step per DRAG_PX_PER_STEP pixels;
 *  a press that moved less than CLICK_SLOP_PX is a click (focus), not a drag. */
export const SHIFT_STEP_MULT = 10;
export const DRAG_PX_PER_STEP = 4;
export const CLICK_SLOP_PX = 3;

// ── Sizes (px) ──
/** LogViewer: row height (matches `.q-log` line-height in quag.css), rows drawn above / below the view,
 *  height used before the first measure. */
export const LOG_ROW_PX = 22;
export const LOG_OVERSCAN_ABOVE = 5;
export const LOG_OVERSCAN_BELOW = 10;
export const LOG_INITIAL_HEIGHT = 400;
/** Charts: default sizes; sparkline end dot radius and padding from the top / bottom edge. */
export const HISTOGRAM_W = 240;
export const HISTOGRAM_H = 60;
export const SPARKLINE_W = 120;
export const SPARKLINE_H = 20;
export const SPARK_DOT_R = 2.5;
export const SPARK_PAD = 1;

// ── Game design editing ──
/** Key moves, as a share of the value's range: arrow = NUDGE, Shift+arrow = NUDGE_FINE. */
export const NUDGE = 0.05;
export const NUDGE_FINE = 0.01;
/** Weights: one step of the raw weight. */
export const WEIGHT_STEP = 1;
/** Timeline: aim for one ruler label every RULER_TICK_PX pixels. */
export const RULER_TICK_PX = 90;
/** Curve: drawing box (SVG viewBox units) and key radius (selected is bigger). */
export const CURVE_VIEW_W = 1000;
export const CURVE_VIEW_H = 400;
export const CURVE_KEY_R = 5;
export const CURVE_KEY_R_SELECTED = 8;

// ── Node graphs (px) ──
export const NODE_W = 170;
export const NODE_HEAD_H = 30;
export const NODE_ROW_H = 22;
export const NODE_PAD_Y = 6;
/** A wire bends out of its port at least this far, so short wires still read as curves. */
export const WIRE_BEND_MIN = 40;
/** Arrows move the selected node; Shift moves it faster. */
export const NODE_STEP_PX = 10;
export const NODE_STEP_FAST_PX = 40;

/** Motion: one animation tick (= Rust `consts::motion::TICK_MS`, CSS `--q-tick`); a spinner turn is 10 ticks. */
export const TICK_MS = 80;
export const SPIN_TURN_MS = 10 * TICK_MS;
