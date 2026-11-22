//! MOLECULE: Table.

use super::*;
use crate::{
    self as q,
    atoms::{self, Tone},
};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    text::Line,
    widgets::Widget,
};
use crate::consts::layout as L;

/// One column: header, width (0 = take the rest), right-aligned (numbers).
#[derive(Clone, Copy, Debug)]
pub struct Column<'a> {
    pub header: &'a str,
    pub width: u16,
    pub right: bool,
}

/// Rows and columns with a header line. Cells truncate with `…`; columns never move.
pub struct Table<'a> {
    pub columns: &'a [Column<'a>],
    pub rows: &'a [&'a [&'a str]],
    pub selected: Option<usize>,
    pub focused: bool,
}

impl Table<'_> {
    /// The cell areas of data row `row` (0 = first row under the header), the same columns `render` uses.
    /// For elements that put atoms in cells (a state icon, a sparkline, a meter): render the Table with
    /// "" in those cells, then draw the atoms into these areas.
    pub fn cell_areas(&self, area: Rect, row: usize) -> Vec<Rect> {
        let cons: Vec<Constraint> = self.columns.iter().map(|c| if c.width == 0 { Constraint::Min(0) } else { Constraint::Length(c.width) }).collect();
        let body = Rect { x: area.x + L::ROW_INDENT, width: area.width.saturating_sub(L::ROW_INDENT), y: area.y + 1 + row as u16, height: 1 };
        Layout::horizontal(cons).spacing(GAP).split(body).to_vec()
    }
}

impl Widget for Table<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let cons: Vec<Constraint> = self.columns.iter().map(|c| if c.width == 0 { Constraint::Min(0) } else { Constraint::Length(c.width) }).collect();
        let body = Rect { x: area.x + L::ROW_INDENT, width: area.width.saturating_sub(L::ROW_INDENT), ..area };
        let cells = |y: u16| Layout::horizontal(cons.clone()).spacing(GAP).split(Rect { y, height: 1, ..body });
        let cell = |text: &str, r: Rect, right: bool, tone: Tone, buf: &mut Buffer| {
            let t = truncate(text, r.width as usize);
            let l = Line::from(atoms::label(&t, tone));
            if right { l.alignment(ratatui::layout::Alignment::Right).render(r, buf) } else { l.render(r, buf) }
        };
        if area.height == 0 { return; }
        for (c, r) in self.columns.iter().zip(cells(area.y).iter()) {
            cell(c.header, *r, c.right, Tone::Heading, buf);
        }
        for (i, row) in self.rows.iter().enumerate().take(area.height.saturating_sub(1) as usize) {
            let y = area.y + 1 + i as u16;
            let sel = self.selected == Some(i);
            if sel {
                (&ListRow { selected: true, list_focused: self.focused, ..Default::default() }).render(Rect { y, height: 1, ..area }, buf);
            }
            for ((c, r), text) in self.columns.iter().zip(cells(y).iter()).zip(row.iter()) {
                let tone = if c.right { Tone::Muted } else { Tone::Text };
                cell(text, *r, c.right, tone, buf);
            }
            if sel {
                let s = if self.focused { q::selected() } else { q::selected_unfocused() };
                buf.set_style(Rect { x: area.x + 1, y, width: area.width.saturating_sub(1), height: 1 }, ratatui::style::Style::new().bg(s.bg.unwrap_or(q::c(q::BG))));
            }
        }
    }
}
