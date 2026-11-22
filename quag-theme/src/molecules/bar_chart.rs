//! MOLECULE: BarChart.

use super::*;
use crate::atoms::{self, Tone};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    text::{Line, Span},
    widgets::Widget,
};
use crate::consts::layout as L;

/// Horizontal bars with labels and values: compare a few things (memory per asset type, time per system).
/// `budget`: bars over it turn red. Rows: one per item.
pub struct BarChart<'a> {
    pub items: &'a [(&'a str, f64)],
    pub unit: &'a str,
    pub max: Option<f64>,
    pub budget: Option<f64>,
}

impl Widget for BarChart<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let max = self.max.unwrap_or_else(|| self.items.iter().map(|i| i.1).fold(0.0, f64::max)).max(f64::EPSILON);
        let lw = self.items.iter().map(|(l, _)| Span::raw(*l).width()).max().unwrap_or(0).min(L::CHART_LABEL_MAX) as u16;
        let vals: Vec<String> = self.items.iter().map(|(_, v)| fmt_num(*v, self.unit)).collect();
        let vw = vals.iter().map(|v| v.len()).max().unwrap_or(0) as u16;
        for (i, ((label, v), vs)) in self.items.iter().zip(&vals).enumerate().take(area.height as usize) {
            let [l, b, n] = Layout::horizontal([Constraint::Length(lw), Constraint::Min(L::BAR_MIN_COLS), Constraint::Length(vw)])
                .spacing(GAP)
                .areas(Rect { y: area.y + i as u16, height: 1, ..area });
            Line::from(atoms::label(&truncate(label, l.width as usize), Tone::Muted)).render(l, buf);
            atoms::Bar { fraction: v / max, over: self.budget.is_some_and(|bd| *v > bd) }.render(b, buf);
            Line::from(atoms::label(vs, Tone::Text)).alignment(ratatui::layout::Alignment::Right).render(n, buf);
        }
    }
}

/// Number + unit the house way (DESIGN.md "Numbers"): ≤ 3 significant digits after the point, thin unit.
pub fn fmt_num(v: f64, unit: &str) -> String {
    let n = if v.abs() >= 100.0 { format!("{v:.0}") } else if v.abs() >= 10.0 { format!("{v:.1}") } else { format!("{v:.2}") };
    let n = if n.contains('.') { n.trim_end_matches('0').trim_end_matches('.').to_string() } else { n };
    if unit.is_empty() { n } else { format!("{n} {unit}") }
}
