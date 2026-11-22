//! CONSTANTS: every number the parts share, named once.
//! Colors live in `lib.rs` (the palette). This file holds sizes, rules, formats, glyphs and timing.
//! The same values exist for the desktop in `solid/src/constants.ts` (and as CSS tokens in `web/quag.css`):
//! change one, change the other.

/// Sizes of the terminal layout, in cells.
pub mod layout {
    /// Below this the app shows "terminal too small" (STRUCTURE.md §6).
    pub const MIN_COLS: u16 = 60;
    pub const MIN_ROWS: u16 = 20;
    /// Size classes: Wide ≥ 120 cols (nav · content · inspector), Medium ≥ 80 (inspector under content), else Narrow.
    pub const WIDE_COLS: u16 = 120;
    pub const MEDIUM_COLS: u16 = 80;

    /// Header = title row + blank row. Footer = one row of key hints and status.
    pub const HEADER_ROWS: u16 = 2;
    pub const FOOTER_ROWS: u16 = 1;
    /// Space kept between the screen edge and the header / footer text.
    pub const EDGE: u16 = 1;
    /// Right side of the footer (last result / status).
    pub const FOOTER_STATUS_COLS: u16 = 28;

    /// Nav width = screen width / NAV_SHARE, kept between NAV_MIN_COLS and NAV_MAX_COLS.
    pub const NAV_SHARE: u16 = 6;
    pub const NAV_MIN_COLS: u16 = 22;
    pub const NAV_MAX_COLS: u16 = 28;
    /// Inspector: share of the width (Wide, beside content) or of the height (Medium, under content).
    pub const INSPECTOR_PCT_WIDE: u16 = 36;
    pub const INSPECTOR_PCT_MEDIUM: u16 = 40;

    /// Form column and Focus column widths (STRUCTURE.md §3D, §3E).
    pub const FORM_MAX_COLS: u16 = 72;
    pub const FOCUS_MAX_COLS: u16 = 50;
    /// Workspace: tools strip width; output pane = height / OUTPUT_SHARE, at least OUTPUT_MIN_ROWS.
    pub const TOOLS_COLS: u16 = 5;
    pub const OUTPUT_SHARE: u16 = 3;
    pub const OUTPUT_MIN_ROWS: u16 = 5;
    /// Split panes: one cell for the handle.
    pub const SPLIT_HANDLE: u16 = 1;

    /// Gap between atoms inside one molecule ("inside a group").
    pub const GAP: u16 = 2;
    /// Left room in lists and tables for the selection bar `▎` and a space.
    pub const ROW_INDENT: u16 = 2;
    /// Panel inner padding (left/right) and popup padding.
    pub const PANEL_PAD_X: u16 = 1;
    pub const POPUP_PAD_X: u16 = 2;
    pub const POPUP_PAD_Y: u16 = 1;

    /// Extra cells around a button label: `  Save  ` (normal) or ` Save ` (compact).
    pub const BUTTON_PAD: u16 = 4;
    pub const BUTTON_PAD_COMPACT: u16 = 2;
    /// `[x] ` / `(•) ` before a checkbox or radio label.
    pub const CHOICE_MARK: u16 = 4;
    /// Room an icon takes at the right end of a field (`✗`, `▾`): icon + one space.
    pub const FIELD_ICON: u16 = 2;
    /// NumberField arrows ` ◂ ` and ` ▸ `.
    pub const NUMBER_ARROWS: u16 = 6;
    /// SecretField `show` / `hide` tag at the right (with its spacing), and its offset from the right edge.
    pub const SECRET_TAG_COLS: u16 = 8;
    pub const SECRET_TAG_RIGHT: u16 = 5;

    /// ConfirmDialog: width = text + DIALOG_TEXT_PAD, between DIALOG_MIN_COLS and DIALOG_MAX_COLS.
    pub const DIALOG_TEXT_PAD: u16 = 8;
    pub const DIALOG_MIN_COLS: u16 = 40;
    pub const DIALOG_MAX_COLS: u16 = 72;
    pub const DIALOG_ROWS: u16 = 8;
    /// TypeToConfirm: content width between these; 6 rows (question, blank, label, input, blank, buttons).
    pub const TYPE_CONFIRM_MIN_COLS: u16 = 36;
    pub const TYPE_CONFIRM_MAX_COLS: u16 = 64;
    pub const TYPE_CONFIRM_ROWS: u16 = 6;
    /// CommandPalette: width, and its top at screen height / PALETTE_TOP_SHARE. Search row + blank row above the items.
    pub const PALETTE_COLS: u16 = 60;
    pub const PALETTE_TOP_SHARE: u16 = 6;
    pub const PALETTE_HEAD_ROWS: u16 = 2;
    /// HelpOverlay content width; space after the widest key.
    pub const HELP_COLS: u16 = 48;
    pub const HELP_KEY_PAD: u16 = 2;
    /// Toast: room for its icon and a space before the message.
    pub const TOAST_ICON: u16 = 3;
    /// Menu: room for the key column and spacing; smallest width; title padding.
    pub const MENU_ITEM_PAD: usize = 6;
    pub const MENU_MIN_COLS: u16 = 14;
    pub const MENU_TITLE_PAD: u16 = 4;
    /// Popups keep this far from the screen edges.
    pub const SCREEN_MARGIN: u16 = 2;

    /// EmptyState: mascot height, and the rows of message + hint (+1 spacing).
    pub const EMPTY_ART_ROWS: u16 = 14;
    pub const EMPTY_TEXT_ROWS: u16 = 3;
    /// Search box: never wider than this.
    pub const SEARCH_MAX_COLS: u16 = 40;
    /// RunProgress: width of its button area; smallest output height.
    pub const PROGRESS_BUTTONS_COLS: u16 = 30;
    pub const PROGRESS_OUTPUT_MIN_ROWS: u16 = 3;

    /// Charts: label column max, smallest bar width.
    pub const CHART_LABEL_MAX: usize = 20;
    pub const BAR_MIN_COLS: u16 = 4;
    /// ParamRow: number field = text + PARAM_NUMBER_PAD, at least PARAM_NUMBER_MIN; the slider shows when
    /// PARAM_SLIDER_MIN more cells are free.
    pub const PARAM_NUMBER_PAD: u16 = 7;
    pub const PARAM_NUMBER_MIN: u16 = 12;
    pub const PARAM_SLIDER_MIN: u16 = 8;
    pub const PARAM_SLIDER_MIN_COLS: u16 = 6;

    /// Timeline: widest track name; aim for one ruler label every RULER_TICK_COLS cells.
    pub const TRACK_NAME_MAX: u16 = 14;
    pub const RULER_TICK_COLS: f64 = 10.0;
    /// CurveEditor: the y labels column (`1.25×`), and the rows of the x labels + readout under the curve.
    pub const CURVE_YLABEL_COLS: u16 = 7;
    pub const CURVE_FOOT_ROWS: u16 = 2;
    /// WeightsEditor: share and odds columns (`42.0 %`, `1 in 2.4`).
    pub const WEIGHT_PCT_COLS: u16 = 7;
    pub const WEIGHT_ODDS_COLS: u16 = 10;
    /// NodeGraph: a node is at least NODE_MIN_COLS wide; the gap between input and output names.
    pub const NODE_MIN_COLS: u16 = 16;
    pub const NODE_PORT_GAP: u16 = 3;
    /// Mascot: how many drawing sizes keep their encoded frames (empty state, welcome page, preview…).
    pub const MASCOT_SIZES_KEPT: usize = 4;
}

/// Editing steps (keys), as a share of the value's range: arrow = NUDGE, Shift+arrow = NUDGE_FINE.
pub mod input {
    pub const NUDGE: f64 = 0.05;
    pub const NUDGE_FINE: f64 = 0.01;
    /// Weights: one key press adds / removes this much raw weight.
    pub const WEIGHT_STEP: f64 = 1.0;
    /// Node graphs: arrows move the selected node this many cells (Shift: NODE_STEP_FAST).
    pub const NODE_STEP: i32 = 1;
    pub const NODE_STEP_FAST: i32 = 4;
}

/// Rules: thresholds and limits that decide a color or a state (DESIGN.md).
pub mod rules {
    /// Usage of a limit: yellow from here, red at USAGE_ERR.
    pub const USAGE_WARN: f64 = 0.8;
    pub const USAGE_ERR: f64 = 1.0;
    /// Live data older than this is "stale" (yellow).
    pub const STALE_AFTER_SECS: u64 = 30;
    /// Restart loop: more than this many restarts within the window → warn even while running.
    pub const FLAPPING_RESTARTS: u32 = 3;
    pub const FLAPPING_WINDOW_SECS: u64 = 600;
    /// Split panes move by this share per key press.
    pub const SPLIT_STEP: f32 = 0.05;
}

/// Formats of text values.
pub mod format {
    /// SecretField: characters left visible at the end, and the most mask dots shown.
    pub const SECRET_TAIL: usize = 4;
    pub const SECRET_MASK_MAX: usize = 12;
    /// Ids and digests are shortened to this many characters.
    pub const SHORT_ID_LEN: usize = 12;
    /// Binary units.
    pub const BYTES_STEP: f64 = 1024.0;
    pub const BYTE_UNITS: [&str; 5] = ["B", "KiB", "MiB", "GiB", "TiB"];
    /// Below this, one decimal is shown (`1.4 GiB`); above, none (`412 MiB`).
    pub const ONE_DECIMAL_BELOW: f64 = 10.0;
    /// Time.
    pub const MINUTE: u64 = 60;
    pub const HOUR: u64 = 60 * MINUTE;
    pub const DAY: u64 = 24 * HOUR;
    /// Younger than this reads "now".
    pub const NOW_SECS: u64 = 5;
}

/// Glyphs used by more than one part.
/// Motion: one clock for every animation, so a spinner turns at the same speed however often the app redraws.
pub mod motion {
    use std::time::Duration;
    /// One animation tick: a spinner frame. 10 frames = one turn in 0.8 s, like the desktop's `q-spin .8s`.
    pub const TICK_MS: u64 = 80;
    pub const TICK: Duration = Duration::from_millis(TICK_MS);
    /// The tick count after `elapsed` (derive it from a clock, never count redraws).
    pub fn ticks(elapsed: Duration) -> usize { (elapsed.as_millis() / TICK_MS as u128) as usize }
    /// How many ticks `ms` milliseconds last (for timings written in time, used in ticks).
    pub const fn ticks_in(ms: u64) -> usize { (ms / TICK_MS) as usize }
}

pub mod glyph {
    /// Braille spinner frames (one per `motion::TICK`).
    pub const SPINNER: [&str; 10] = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
    /// Vertical eighths (histograms), from empty to full.
    pub const EIGHTHS_UP: [&str; 9] = [" ", "▁", "▂", "▃", "▄", "▅", "▆", "▇", "█"];
    /// Horizontal eighths (bars), from empty to full.
    pub const EIGHTHS_RIGHT: [&str; 9] = ["", "▏", "▎", "▍", "▌", "▋", "▊", "▉", "█"];
    /// Steps per cell of the eighths above.
    pub const EIGHTHS: usize = 8;
    /// Thin meter line: full cell, half cell, track.
    pub const LINE_FULL: &str = "━";
    pub const LINE_HALF: char = '╸';
    pub const LINE_TRACK: &str = "─";
    /// A button with keyboard focus.
    pub const FOCUS_MARK: char = '▸';
    /// Mask character for secrets.
    pub const MASK: &str = "•";
    /// Curves: a key point, the selected key point.
    pub const CURVE_POINT: &str = "●";
    pub const CURVE_SELECTED: &str = "◆";
    /// Timelines: playhead head (on the ruler) and line, ruler tick.
    pub const PLAYHEAD_HEAD: &str = "▼";
    pub const PLAYHEAD_LINE: &str = "│";
    pub const RULER_TICK: &str = "╵";
    /// Node graphs: a port (connected / free).
    pub const PORT: &str = "●";
    pub const PORT_FREE: &str = "○";
}
