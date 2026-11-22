//! UI: the page level. A page arranges ELEMENTS into zones; it never draws atoms itself.
//! These helpers give every page the same skeleton and breakpoints (STRUCTURE.md §5),
//! so apps never hand-roll them.

use crate as q;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Flex, Layout, Rect},
    text::Line,
    widgets::{Block, Paragraph},
};

use crate::consts::layout as L;

/// Minimum usable terminal size (STRUCTURE.md §6). Kept here too for existing callers.
pub use crate::consts::layout::{MIN_COLS, MIN_ROWS};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SizeClass {
    /// ≥ 120 cols: nav · content · inspector side by side.
    Wide,
    /// 80–119 cols: nav · content, inspector stacked under content.
    Medium,
    /// 60–79 cols: content only; app shows nav / inspector as full panels on demand.
    Narrow,
}

impl SizeClass {
    pub fn of(area: Rect) -> SizeClass {
        match area.width {
            w if w >= L::WIDE_COLS => SizeClass::Wide,
            w if w >= L::MEDIUM_COLS => SizeClass::Medium,
            _ => SizeClass::Narrow,
        }
    }
}

/// Which optional zones this screen wants.
#[derive(Clone, Copy, Debug, Default)]
pub struct Zones {
    pub nav: bool,
    pub inspector: bool,
}

/// The five zones of STRUCTURE.md §1, already resolved for the current size.
#[derive(Clone, Copy, Debug)]
pub struct Shell {
    pub header: Rect,
    /// `None` when not requested or when the terminal is Narrow.
    pub nav: Option<Rect>,
    pub content: Rect,
    /// `None` when not requested or when Narrow (show it full-screen on Enter instead).
    pub inspector: Option<Rect>,
    pub footer: Rect,
    pub class: SizeClass,
    /// Below MIN_COLS × MIN_ROWS: call `render_too_small` and return.
    pub too_small: bool,
}

/// Splits the screen into the skeleton. Header = 2 rows, footer = 1 row.
/// Nav = 24 cols (22–28 by width). Inspector = 36% (Wide) or 40% height under content (Medium).
pub fn shell(area: Rect, zones: Zones) -> Shell {
    let too_small = area.width < MIN_COLS || area.height < MIN_ROWS;
    let class = SizeClass::of(area);
    let [header, body, footer] = Layout::vertical([
        Constraint::Length(L::HEADER_ROWS),
        Constraint::Min(0),
        Constraint::Length(L::FOOTER_ROWS),
    ])
    .areas(area);

    let nav_w = (area.width / L::NAV_SHARE).clamp(L::NAV_MIN_COLS, L::NAV_MAX_COLS);
    let (nav, rest) = if zones.nav && class != SizeClass::Narrow {
        let [n, r] = Layout::horizontal([Constraint::Length(nav_w), Constraint::Min(0)]).areas(body);
        (Some(n), r)
    } else {
        (None, body)
    };

    let (content, inspector) = match (zones.inspector, class) {
        (true, SizeClass::Wide) => {
            let [c, i] = Layout::horizontal([Constraint::Percentage(100 - L::INSPECTOR_PCT_WIDE), Constraint::Percentage(L::INSPECTOR_PCT_WIDE)]).areas(rest);
            (c, Some(i))
        }
        (true, SizeClass::Medium) => {
            let [c, i] = Layout::vertical([Constraint::Percentage(100 - L::INSPECTOR_PCT_MEDIUM), Constraint::Percentage(L::INSPECTOR_PCT_MEDIUM)]).areas(rest);
            (c, Some(i))
        }
        _ => (rest, None),
    };

    Shell { header, nav, content, inspector, footer, class, too_small }
}

/// Header row split: `[identity (◆ app + location), right side (tabs / search)]`.
pub fn header_split(header: Rect) -> [Rect; 2] {
    let row = Rect { height: 1, ..header };
    let [l, r] = Layout::horizontal([Constraint::Min(0), Constraint::Min(0)]).areas(row);
    [Rect { x: l.x + L::EDGE, width: l.width.saturating_sub(L::EDGE), ..l }, Rect { width: r.width.saturating_sub(L::EDGE), ..r }]
}

/// Footer split: `[key hints (left), status (right, 28 cols)]`.
pub fn footer_split(footer: Rect) -> [Rect; 2] {
    let [l, r] = Layout::horizontal([Constraint::Min(0), Constraint::Length(L::FOOTER_STATUS_COLS)]).areas(footer);
    [Rect { x: l.x + L::EDGE, width: l.width.saturating_sub(L::EDGE), ..l }, Rect { width: r.width.saturating_sub(L::EDGE), ..r }]
}

/// Content top row (title / toolbar) and the rest, inside an already-bordered panel's inner area.
pub fn toolbar_split(inner: Rect) -> [Rect; 2] {
    let [t, _, b] = Layout::vertical([Constraint::Length(1), Constraint::Length(1), Constraint::Min(0)]).areas(inner);
    [t, b]
}

/// Form / Settings column: left-aligned, max 72 cols wide (STRUCTURE.md §3D).
pub fn form_column(area: Rect) -> Rect {
    Rect { width: area.width.min(L::FORM_MAX_COLS), ..area }
}

/// Focus layout: centered column, max 50 cols, `rows` tall (STRUCTURE.md §3E).
pub fn focus_column(area: Rect, rows: u16) -> Rect {
    centered(area, L::FOCUS_MAX_COLS, rows)
}

/// Centered rect of at most `w` × `h`, clamped to `area`. Use for popups.
pub fn centered(area: Rect, w: u16, h: u16) -> Rect {
    let [r] = Layout::horizontal([Constraint::Length(w.min(area.width))]).flex(Flex::Center).areas(area);
    let [r] = Layout::vertical([Constraint::Length(h.min(area.height))]).flex(Flex::Center).areas(r);
    r
}

/// Dashboard: `[kpi_row (4 rows: bordered value + label), body]` (STRUCTURE.md §3B).
pub fn dashboard(area: Rect) -> [Rect; 2] {
    Layout::vertical([Constraint::Length(4), Constraint::Min(0)]).areas(area)
}

/// Splits a row into `n` equal columns (KPI tiles, dashboard cards).
pub fn columns(area: Rect, n: u16) -> Vec<Rect> {
    Layout::horizontal((0..n).map(|_| Constraint::Ratio(1, n as u32)))
        .split(area)
        .to_vec()
}

/// Splits an area into a 2-wide grid of `rows` rows (dashboard body, max 6 cards = 3 rows).
pub fn grid2(area: Rect, rows: u16) -> Vec<Rect> {
    let rws = Layout::vertical((0..rows).map(|_| Constraint::Ratio(1, rows as u32))).split(area);
    rws.iter().flat_map(|r| columns(*r, 2)).collect()
}

/// Workspace: `[tools strip (5 cols), canvas, output (collapsed = 0 rows)]` (STRUCTURE.md §3C).
pub fn workspace(area: Rect, output_open: bool) -> [Rect; 3] {
    let [tools, main] = Layout::horizontal([Constraint::Length(L::TOOLS_COLS), Constraint::Min(0)]).areas(area);
    let [canvas, output] = Layout::vertical([
        Constraint::Min(0),
        Constraint::Length(if output_open { (area.height / L::OUTPUT_SHARE).max(L::OUTPUT_MIN_ROWS) } else { 0 }),
    ])
    .areas(main);
    [tools, canvas, output]
}

/// Two resizable panes side by side (`vertical` handle) or stacked. `ratio` = share of the first pane.
/// Returns `[first, handle, second]`; draw `atoms::SplitHandle` in the handle. Each pane keeps at least `min`.
pub fn split(area: Rect, ratio: f32, vertical: bool, min: u16) -> [Rect; 3] {
    let total = if vertical { area.width } else { area.height };
    let room = total.saturating_sub(L::SPLIT_HANDLE);
    let first = ((room as f32 * ratio.clamp(0.0, 1.0)).round() as u16).clamp(min.min(room / 2), room.saturating_sub(min.min(room / 2)));
    let cons = [Constraint::Length(first), Constraint::Length(L::SPLIT_HANDLE), Constraint::Min(0)];
    if vertical { Layout::horizontal(cons).areas(area) } else { Layout::vertical(cons).areas(area) }
}

/// Step for keyboard resizing of a split (`[` / `]`): 5 %.
pub use crate::consts::rules::SPLIT_STEP;

/// The "too small" Focus screen.
pub fn render_too_small(f: &mut Frame, area: Rect) {
    f.render_widget(Block::new().style(q::base()), area);
    let r = centered(area, area.width, 3);
    f.render_widget(
        Paragraph::new(vec![
            Line::styled("Window too small", q::text()),
            Line::styled(format!("need {MIN_COLS}×{MIN_ROWS}, have {}×{}", area.width, area.height), q::muted()),
        ])
        .alignment(Alignment::Center),
        r,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn breakpoints() {
        let z = Zones { nav: true, inspector: true };
        let wide = shell(Rect::new(0, 0, 140, 40), z);
        assert_eq!(wide.class, SizeClass::Wide);
        assert!(wide.nav.is_some() && wide.inspector.is_some());
        assert_eq!(wide.inspector.unwrap().y, wide.content.y); // side by side

        let med = shell(Rect::new(0, 0, 100, 40), z);
        assert_eq!(med.class, SizeClass::Medium);
        assert!(med.inspector.unwrap().y > med.content.y); // stacked under

        let narrow = shell(Rect::new(0, 0, 70, 30), z);
        assert!(narrow.nav.is_none() && narrow.inspector.is_none());
        assert_eq!(narrow.content.width, 70);

        assert!(shell(Rect::new(0, 0, 50, 30), z).too_small);
        assert!(shell(Rect::new(0, 0, 100, 15), z).too_small);
    }

    #[test]
    fn form_and_focus_limits() {
        let a = Rect::new(0, 0, 200, 50);
        assert_eq!(form_column(a).width, 72);
        assert_eq!(form_column(a).x, 0);
        let fc = focus_column(a, 10);
        assert_eq!((fc.width, fc.height), (50, 10));
        assert_eq!(fc.x, 75);
    }
}
