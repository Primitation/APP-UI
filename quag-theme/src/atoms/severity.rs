//! ATOM: Severity.

use crate::{
    self as q, Status,
};
use ratatui::{
    style::{Modifier, Style},
    text::Span,
};

/// Five-level severity (security findings, lint, audits). DESIGN.md "Severity":
/// 5 levels on 4 status colors, told apart by fill and weight too, never by color alone.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

impl Severity {
    pub const ALL: [Severity; 5] = [Severity::Critical, Severity::High, Severity::Medium, Severity::Low, Severity::Info];
    pub fn label(self) -> &'static str {
        match self { Severity::Critical => "CRIT", Severity::High => "HIGH", Severity::Medium => "MED", Severity::Low => "LOW", Severity::Info => "INFO" }
    }
}

/// `▐CRIT▌` filled red · `HIGH` bold red · `MED` yellow · `LOW` blue · `INFO` grey. Always 6 cells wide.
pub fn severity_badge(s: Severity) -> Span<'static> {
    let style = match s {
        Severity::Critical => Style::new().fg(q::c(q::BG)).bg(q::c(q::ERR)).add_modifier(Modifier::BOLD),
        Severity::High => Status::Err.style().add_modifier(Modifier::BOLD),
        Severity::Medium => Status::Warn.style(),
        Severity::Low => Style::new().fg(q::c(q::BODY)),
        Severity::Info => q::muted(),
    };
    Span::styled(format!(" {:<4} ", s.label()), style)
}
