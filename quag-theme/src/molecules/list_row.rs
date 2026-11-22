//! MOLECULE: ListRow.

use crate::{
    self as q, Status,
    atoms::{self},
};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    text::{Line, Span},
    widgets::Widget,
};
use crate::consts::layout as L;

/// One row of a list: `▌ [✓] title            #tag  ! due`.
/// Left: optional checkbox + title. Right (aligned): tags, status icon, meta.
#[derive(Clone, Debug, Default)]
pub struct ListRow<'a> {
    pub title: &'a str,
    pub check: Option<bool>,
    pub tags: &'a [&'a str],
    pub status: Option<Status>,
    pub meta: &'a str,
    pub meta_accent: bool,
    /// Reserve a column for the status icon even when `status` is None (keeps columns aligned in task lists).
    pub status_slot: bool,
    /// Right-align `meta` in this many cells (0 = its own width).
    pub meta_width: usize,
    pub selected: bool,
    pub list_focused: bool,
    /// A leading badge before the title (e.g. `atoms::severity_badge`).
    pub lead: Option<Span<'static>>,
}

impl Widget for &ListRow<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area = Rect { height: 1, ..area };
        let done = self.check == Some(true);
        if self.selected {
            let s = if self.list_focused { q::selected() } else { q::selected_unfocused() };
            buf.set_style(area, s);
            buf[(area.x, area.y)].set_symbol("▌").set_fg(q::c(if self.list_focused { q::QUAG } else { q::MUTED }));
        }
        // right side first, so we know how much room the title gets
        let mut right: Vec<Span> = self.tags.iter().map(|t| atoms::tag(t)).flat_map(|t| [t, Span::raw("  ")]).collect();
        match (self.status, self.status_slot) {
            (Some(s), _) => right.extend([atoms::status_icon(s), Span::raw("  ")]),
            (None, true) => right.push(Span::raw("   ")),
            (None, false) => {}
        }
        let w = self.meta_width;
        right.push(Span::styled(format!("{:>w$}", self.meta), if self.meta_accent { q::accent() } else { q::muted() }));
        let right = Line::from(right);
        let rw = right.width() as u16;
        let [l, r] = Layout::horizontal([Constraint::Min(0), Constraint::Length(rw + L::GAP)]).areas(Rect {
            x: area.x + L::ROW_INDENT,
            width: area.width.saturating_sub(L::ROW_INDENT),
            ..area
        });

        let mut left = Vec::new();
        if let Some(b) = &self.lead {
            left.extend([b.clone(), Span::raw(" ")]);
        }
        if let Some(c) = self.check {
            left.push(Span::styled(if c { "✓ " } else { "○ " }, if c { Status::Ok.style() } else { q::faint() }));
        }
        let title_style = if done {
            q::faint()
        } else if self.selected && self.list_focused {
            q::selected()
        } else {
            q::text()
        };
        // Overflow rule: truncate with … (the full text lives in the inspector / tooltip)
        let room = (l.width as usize).saturating_sub(left.iter().map(|s| s.width()).sum::<usize>());
        let title: String = if Span::raw(self.title).width() > room && room > 1 {
            let mut t = String::new();
            for ch in self.title.chars() {
                if Span::raw(format!("{t}{ch}…")).width() > room { break; }
                t.push(ch);
            }
            format!("{t}…")
        } else {
            self.title.to_string()
        };
        left.push(Span::styled(title, title_style));
        Line::from(left).render(l, buf);
        // keep selection bg behind the right part
        let mut right = right;
        if self.selected {
            let bg = if self.list_focused { q::selected() } else { q::selected_unfocused() };
            right = right.patch_style(ratatui::style::Style::new().bg(bg.bg.unwrap_or(q::c(q::BG))));
        }
        right.alignment(ratatui::layout::Alignment::Right).render(r, buf);
    }
}
