//! ELEMENT: AppHeader.

use crate::{
    atoms::{self, Tone},
    molecules::Tabs,
    ui,
};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    text::{Line, Span},
    widgets::Widget,
};

/// `◆ app  location ··················· tab  tab  tab`
pub struct AppHeader<'a> {
    pub app: &'a str,
    pub location: &'a str,
    pub tabs: Option<Tabs<'a>>,
    /// Server tools: the environment, always visible (`[prod]` in red).
    pub env: Option<atoms::Env>,
}

impl Widget for AppHeader<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [left, right] = ui::header_split(area);
        let mut l = vec![atoms::label(&format!("◆ {}", self.app), Tone::Title), Span::raw("  ")];
        if let Some(e) = self.env { l.extend([atoms::env_tag(e), Span::raw("  ")]); }
        l.push(atoms::label(self.location, Tone::Muted));
        Line::from(l).render(left, buf);
        if let Some(t) = self.tabs {
            t.to_line().alignment(Alignment::Right).render(right, buf);
        }
    }
}
