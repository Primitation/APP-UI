/**
 * Numbers of the demo apps (pond, dock): timing and sizes of the simulation, not of the design system.
 * The design system's own numbers are in `src/constants.ts`. Same values as `examples/explorer/consts.rs`.
 */

/** pond · lines in the log page's demo log, and in the LogViewer catalog preview. */
export const LOG_LINES = 20_000;
export const LOG_LINES_PREVIEW = 100_000;
/** pond · workspace: the editor keeps between these shares of the width. */
export const SPLIT_MIN = 0.2;
export const SPLIT_MAX = 0.9;

/** dock · how long a container takes to restart / stop / start (ms). */
export const SETTLE_MS = 1800;
/** dock · deploy: time per step (ms), and the step that fails in "demo: fail" mode (health check). */
export const DEPLOY_STEP_MS = 1200;
export const DEPLOY_FAIL_STEP = 3;
/** dock · ages shown by the demo's Freshness: fresh, and stale (hosts page). */
export const AGE_FRESH_SECS = 3;
export const AGE_STALE_SECS = 45;

/** forge · balance: DPS per gold this many times the median (or below median / this) = outlier. */
export const OUTLIER_RATIO = 1.25;
/** forge · sequence: playhead step per frame while playing, per ← / → press (seconds), and the frame. */
export const PLAY_STEP_SECS = 0.25;
export const SCRUB_SECS = 0.5;
export const PLAY_FRAME_MS = 250;
