//! MOLECULE: Servers.

use super::*;
use crate::{
    Status,
    atoms::{self, Tone},
};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    text::{Line, Span},
    widgets::Widget,
};
use crate::consts::layout as L;

/// `mem  ███████▒▒▒  412 / 512 MiB`: usage of a limit. Colors come from `UsageBar` (80 % yellow, 100 % red);
/// past 80 % the value also gets the status icon, so it's not color alone.
pub struct Meter<'a> {
    pub label: &'a str,
    pub label_width: u16,
    pub fraction: f64,
    /// Already formatted, e.g. `fmt::bytes_of(used, limit)` or `"42 %"`.
    pub value: &'a str,
    pub value_width: u16,
}

impl Widget for Meter<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area = Rect { height: 1, ..area };
        let bar = atoms::UsageBar(self.fraction);
        let lw = if self.label.is_empty() { 0 } else { self.label_width };
        let [l, b, v] = Layout::horizontal([Constraint::Length(lw), Constraint::Min(L::BAR_MIN_COLS), Constraint::Length(self.value_width + L::FIELD_ICON)]).spacing(1).areas(area);
        Line::from(atoms::label(self.label, Tone::Muted)).render(l, buf);
        bar.render(b, buf);
        let mut spans = vec![];
        if let Some(st) = bar.status() { spans.extend([atoms::status_icon(st), Span::raw(" ")]); }
        spans.push(atoms::label(self.value, Tone::Text));
        Line::from(spans).alignment(ratatui::layout::Alignment::Right).render(v, buf);
    }
}

/// `8080 → 80/tcp`. Published on all interfaces (`public`) gets a `!`: it's reachable from outside.
#[derive(Clone, Copy, Debug)]
pub struct PortMap<'a> {
    pub host: Option<u16>,
    pub container: u16,
    pub proto: &'a str,
    pub public: bool,
}

impl PortMap<'_> {
    pub fn spans(&self) -> Vec<Span<'static>> {
        let mut v = vec![];
        if self.public { v.extend([atoms::status_icon(Status::Warn), Span::raw(" ")]); }
        if let Some(h) = self.host { v.extend([atoms::label(&h.to_string(), Tone::Text), atoms::label(" → ", Tone::Faint)]); }
        v.push(atoms::label(&format!("{}/{}", self.container, self.proto), Tone::Muted));
        v
    }
    pub fn width(&self) -> u16 { Line::from(self.spans()).width() as u16 }
}

/// One step of a pipeline (build, push, deploy, health check).
#[derive(Clone, Copy, Debug)]
pub struct Step<'a> {
    pub name: &'a str,
    pub state: atoms::StepState,
    /// `"12s"`, or "" while waiting.
    pub took: &'a str,
}

/// Steps of a job. Vertical: one row per step with its time. Inline: `✓ build › ⠋ deploy › ○ check` (headers, rows).
pub struct StepList<'a> {
    pub steps: &'a [Step<'a>],
    pub inline: bool,
    pub tick: usize,
}

impl StepList<'_> {
    pub fn to_line(&self) -> Line<'static> {
        let mut v = vec![];
        for (i, st) in self.steps.iter().enumerate() {
            if i > 0 { v.push(atoms::label(" › ", Tone::Faint)); }
            let tone = if st.state == atoms::StepState::Waiting || st.state == atoms::StepState::Skipped { Tone::Faint } else { Tone::Text };
            v.extend([atoms::step_icon(st.state, self.tick), Span::raw(" "), atoms::label(st.name, tone)]);
        }
        Line::from(v)
    }
}

impl Widget for StepList<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.inline {
            self.to_line().render(Rect { height: 1, ..area }, buf);
            return;
        }
        for (i, st) in self.steps.iter().enumerate().take(area.height as usize) {
            let r = Rect { y: area.y + i as u16, height: 1, ..area };
            let tone = match st.state { atoms::StepState::Waiting | atoms::StepState::Skipped => Tone::Faint, atoms::StepState::Running => Tone::Accent, _ => Tone::Text };
            Line::from(vec![atoms::step_icon(st.state, self.tick), Span::raw(" "), atoms::label(st.name, tone)]).render(r, buf);
            Line::from(atoms::label(st.took, Tone::Faint)).alignment(ratatui::layout::Alignment::Right).render(r, buf);
        }
    }
}

/// Key = value rows (environment variables, labels). Secret values are masked (`SecretField`) unless `revealed`.
pub struct KeyValueRows<'a> {
    /// (key, value, secret)
    pub rows: &'a [(&'a str, &'a str, bool)],
    pub key_width: u16,
    pub revealed: bool,
}

impl Widget for KeyValueRows<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        for (i, (k, v, secret)) in self.rows.iter().enumerate().take(area.height as usize) {
            let r = Rect { y: area.y + i as u16, height: 1, ..area };
            let [kr, vr] = Layout::horizontal([Constraint::Length(self.key_width), Constraint::Min(0)]).spacing(1).areas(r);
            Line::from(atoms::label(&truncate(k, kr.width as usize), Tone::Muted)).render(kr, buf);
            if *secret {
                atoms::SecretField::new(v).revealed(self.revealed).render(vr, buf);
            } else {
                Line::from(atoms::label(&truncate(v, vr.width as usize), Tone::Text)).render(vr, buf);
            }
        }
    }
}

/// How fresh live data is: `updated 3s ago` (muted), or `! stale 45s` (yellow) once older than `stale_after`
/// (normally `consts::rules::STALE_AFTER_SECS`).
pub fn freshness(age_secs: u64, stale_after: u64) -> Span<'static> {
    if age_secs > stale_after {
        atoms::status_label(&format!("stale {age_secs}s"), Status::Warn)
    } else {
        atoms::label(&format!("updated {}", crate::fmt::age(age_secs)), Tone::Muted)
    }
}
