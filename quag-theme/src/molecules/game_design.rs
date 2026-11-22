//! MOLECULE: Game design.

use super::*;
use crate::{
    self as q, Status,
    atoms::{self, Tone},
};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    text::{Line, Span},
    widgets::Widget,
};
use crate::consts::{glyph as G, layout as L};

/// A curve to edit: y labels left (max / mid / min), the curve, x labels under it, then a readout of the
/// selected key. Moving keys is the app's job (`nudge_point` keeps them in order and in range).
pub struct CurveEditor<'a> {
    pub points: &'a [(f64, f64)],
    pub x: [f64; 2],
    pub y: [f64; 2],
    pub x_unit: &'a str,
    pub y_unit: &'a str,
    pub selected: Option<usize>,
}

impl Widget for CurveEditor<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [plot_row, xl, read] = Layout::vertical([Constraint::Min(3), Constraint::Length(1), Constraint::Length(1)]).areas(area);
        let [yl, plot] = Layout::horizontal([Constraint::Length(L::CURVE_YLABEL_COLS), Constraint::Min(0)]).spacing(1).areas(plot_row);
        let ylab = |v: f64| fmt_num(v, self.y_unit);
        let mid = (self.y[0] + self.y[1]) / 2.0;
        for (row, v) in [(yl.y, self.y[1]), (yl.y + yl.height / 2, mid), (yl.bottom().saturating_sub(1), self.y[0])] {
            Line::from(atoms::label(&ylab(v), Tone::Faint)).alignment(ratatui::layout::Alignment::Right).render(Rect { y: row, height: 1, ..yl }, buf);
        }
        atoms::Curve { points: self.points, x: self.x, y: self.y, selected: self.selected }.render(plot, buf);
        let xr = Rect { x: plot.x, width: plot.width, ..xl };
        Line::from(atoms::label(&fmt_num(self.x[0], self.x_unit), Tone::Faint)).render(xr, buf);
        Line::from(atoms::label(&fmt_num(self.x[1], self.x_unit), Tone::Faint)).alignment(ratatui::layout::Alignment::Right).render(xr, buf);
        if let Some((i, (x, y))) = self.selected.and_then(|i| self.points.get(i).map(|p| (i, p))) {
            Line::from(vec![
                atoms::label(&format!("{} key {}/{}", G::CURVE_SELECTED, i + 1, self.points.len()), Tone::Accent),
                atoms::label(&format!("   x {}   y {}", fmt_num(*x, self.x_unit), fmt_num(*y, self.y_unit)), Tone::Muted),
            ]).render(Rect { x: plot.x, width: plot.width, ..read }, buf);
        }
    }
}

/// Moves key `i` by (dx, dy), keeping it between its neighbours on x (keys stay sorted) and inside the ranges.
/// The first and last keys keep their x (the curve always spans the whole range).
pub fn nudge_point(points: &mut [(f64, f64)], i: usize, dx: f64, dy: f64, x: [f64; 2], y: [f64; 2]) {
    let n = points.len();
    if i >= n { return; }
    let lo = if i == 0 { x[0] } else { points[i - 1].0 };
    let hi = if i + 1 == n { x[1] } else { points[i + 1].0 };
    if i != 0 && i + 1 != n { points[i].0 = (points[i].0 + dx).clamp(lo, hi); }
    points[i].1 = (points[i].1 + dy).clamp(y[0], y[1]);
}

/// Shares of each weight in the total (0..1). All zero → all zero.
pub fn shares(weights: &[f64]) -> Vec<f64> {
    let total: f64 = weights.iter().sum();
    weights.iter().map(|w| if total > 0.0 { w / total } else { 0.0 }).collect()
}

/// Weighted choices (loot, spawn tables): raw weights, shown as % of the total (so it always sums to 100 %)
/// and as odds `1 in N`. The selected row's weight is the one the keys change.
pub struct WeightsEditor<'a> {
    pub items: &'a [(&'a str, f64)],
    pub selected: usize,
    pub focused: bool,
}

impl Widget for WeightsEditor<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let w: Vec<f64> = self.items.iter().map(|i| i.1).collect();
        let sh = shares(&w);
        let top = sh.iter().cloned().fold(0.0, f64::max).max(f64::EPSILON);
        let lw = self.items.iter().map(|(l, _)| Span::raw(*l).width()).max().unwrap_or(0).min(L::CHART_LABEL_MAX) as u16 + L::ROW_INDENT;
        let n = self.items.len() as u16;
        let [rows, _, total] = Layout::vertical([Constraint::Length(n), Constraint::Length(1), Constraint::Length(1)]).areas(area);
        for (i, ((label, raw), s)) in self.items.iter().zip(&sh).enumerate().take(rows.height as usize) {
            let r = Rect { y: rows.y + i as u16, height: 1, ..rows };
            let sel = i == self.selected;
            if sel { (&ListRow { selected: true, list_focused: self.focused, ..Default::default() }).render(r, buf); }
            let [l, b, raw_c, p, o] = Layout::horizontal([
                Constraint::Length(lw), Constraint::Min(L::BAR_MIN_COLS), Constraint::Length(L::WEIGHT_PCT_COLS - 1),
                Constraint::Length(L::WEIGHT_PCT_COLS), Constraint::Length(L::WEIGHT_ODDS_COLS),
            ]).spacing(1).areas(r);
            Line::from(atoms::label(&truncate(label, (l.width - L::ROW_INDENT) as usize), if sel { Tone::Title } else { Tone::Text })).render(Rect { x: l.x + L::ROW_INDENT, width: l.width - L::ROW_INDENT, ..l }, buf);
            atoms::Bar { fraction: s / top, over: false }.render(b, buf);
            Line::from(atoms::label(&fmt_num(*raw, ""), Tone::Faint)).alignment(ratatui::layout::Alignment::Right).render(raw_c, buf);
            Line::from(atoms::label(&format!("{:.1} %", s * 100.0), Tone::Text)).alignment(ratatui::layout::Alignment::Right).render(p, buf);
            let odds = if *s > 0.0 { format!("1 in {}", fmt_num(1.0 / s, "")) } else { "never".into() };
            Line::from(atoms::label(&odds, Tone::Muted)).alignment(ratatui::layout::Alignment::Right).render(o, buf);
        }
        let sum: f64 = w.iter().sum();
        Line::from(vec![
            atoms::label(&format!("{} entries · weights {} · ", self.items.len(), fmt_num(sum, "")), Tone::Faint),
            atoms::status_label(if sum > 0.0 { "100 %" } else { "nothing can drop" }, if sum > 0.0 { Status::Ok } else { Status::Warn }),
        ]).render(total, buf);
    }
}

/// One clip: start and length in the timeline's unit.
#[derive(Clone, Copy, Debug)]
pub struct ClipSpec<'a> {
    pub start: f64,
    pub len: f64,
    pub label: &'a str,
}

/// One track: a name and its clips.
#[derive(Clone, Copy, Debug)]
pub struct Track<'a> {
    pub name: &'a str,
    pub clips: &'a [ClipSpec<'a>],
}

/// Tracks of clips over time (cutscenes, animation events, waves): ruler on top, one row per track,
/// the playhead across all. `span` = visible time range; clips outside are cut at the edges.
pub struct Timeline<'a> {
    pub tracks: &'a [Track<'a>],
    pub span: [f64; 2],
    pub playhead: f64,
    pub unit: &'a str,
    /// (track, clip)
    pub selected: Option<(usize, usize)>,
}

impl Timeline<'_> {
    /// A "nice" ruler step (1, 2, 5 × 10ⁿ) so labels are about RULER_TICK_COLS cells apart.
    pub fn ruler_step(span: f64, cols: u16) -> f64 {
        let raw = span / (cols as f64 / L::RULER_TICK_COLS).max(1.0);
        let p = 10f64.powf(raw.log10().floor());
        [1.0, 2.0, 5.0, 10.0].iter().map(|m| m * p).find(|s| *s >= raw).unwrap_or(10.0 * p)
    }
}

impl Widget for Timeline<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let nw = self.tracks.iter().map(|t| Span::raw(t.name).width()).max().unwrap_or(0).min(L::TRACK_NAME_MAX as usize) as u16;
        let [names, lane] = Layout::horizontal([Constraint::Length(nw), Constraint::Min(0)]).spacing(1).areas(area);
        if lane.width < 2 || self.span[1] <= self.span[0] { return; }
        let span = self.span[1] - self.span[0];
        let col = |t: f64| -> i32 { ((t - self.span[0]) / span * (lane.width - 1) as f64).round() as i32 };
        // ruler
        let step = Self::ruler_step(span, lane.width);
        let mut t = (self.span[0] / step).ceil() * step;
        while t <= self.span[1] + f64::EPSILON {
            let x = lane.x + col(t) as u16;
            let lab = fmt_num(t, self.unit);
            buf.set_string(x, lane.y, G::RULER_TICK, q::faint());
            if x + 1 + lab.len() as u16 <= lane.right() { buf.set_string(x + 1, lane.y, &lab, q::faint()); }
            t += step;
        }
        // tracks: one row each, a blank row between
        for (ti, tr) in self.tracks.iter().enumerate() {
            let y = lane.y + 2 + 2 * ti as u16;
            if y >= area.bottom() { break; }
            Line::from(atoms::label(&truncate(tr.name, nw as usize), Tone::Muted)).render(Rect { y, height: 1, ..names }, buf);
            for (ci, c) in tr.clips.iter().enumerate() {
                let (a, b) = (col(c.start).max(0), col(c.start + c.len).min(lane.width as i32 - 1));
                if b < 0 || a >= lane.width as i32 { continue; }
                let w = (b - a).max(1) as u16;
                atoms::Clip { label: c.label, selected: self.selected == Some((ti, ci)) }.render(Rect { x: lane.x + a as u16, y, width: w, height: 1 }, buf);
            }
        }
        // playhead over everything
        if (self.span[0]..=self.span[1]).contains(&self.playhead) {
            let x = lane.x + col(self.playhead) as u16;
            let ph = atoms::playhead(true);
            buf.set_string(x, lane.y, ph.content.as_ref(), ph.style);
            let line = atoms::playhead(false);
            for y in lane.y + 1..area.bottom() {
                buf[(x, y)].set_symbol(&line.content).set_fg(line.style.fg.unwrap_or_default());
            }
        }
    }
}
