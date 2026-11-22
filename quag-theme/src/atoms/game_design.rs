//! ATOM: Game design: Curve, Clip, playhead.

use crate::{
    self as q,
    consts::glyph as G,
};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, Widget},
};

/// A curve through key points (difficulty, XP, falloff): line in Quag, keys as `•`, the selected key `◆` in Gill.
/// `x` / `y` are the value ranges shown. Points must be sorted by x. Linear between keys.
pub struct Curve<'a> {
    pub points: &'a [(f64, f64)],
    pub x: [f64; 2],
    pub y: [f64; 2],
    pub selected: Option<usize>,
}

impl Widget for Curve<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        use ratatui::{symbols::Marker, widgets::canvas::{Canvas, Line as CLine}};
        let pts = self.points;
        // own background: the canvas resets its cells otherwise (transparent in terminals with background opacity)
        Canvas::default()
            .background_color(q::c(q::BG))
            .marker(Marker::Braille)
            .x_bounds(self.x)
            .y_bounds(self.y)
            .paint(|ctx| {
                // baseline at the bottom of the range
                ctx.draw(&CLine { x1: self.x[0], y1: self.y[0], x2: self.x[1], y2: self.y[0], color: q::c(q::BORDER) });
                for w in pts.windows(2) {
                    ctx.draw(&CLine { x1: w[0].0, y1: w[0].1, x2: w[1].0, y2: w[1].1, color: q::c(q::QUAG) });
                }
            })
            .render(area, buf);
        // keys: in the very cell the braille line passes through (same grid math as the canvas: 2×4 dots per cell)
        if area.width == 0 || area.height == 0 { return; }
        let (dw, dh) = (area.width as f64 * 2.0 - 1.0, area.height as f64 * 4.0 - 1.0);
        for (i, (x, y)) in pts.iter().enumerate() {
            let fx = (x - self.x[0]) / (self.x[1] - self.x[0]);
            let fy = (self.y[1] - y) / (self.y[1] - self.y[0]);
            if !(0.0..=1.0).contains(&fx) || !(0.0..=1.0).contains(&fy) { continue; }
            let (cx, cy) = (area.x + (fx * dw) as u16 / 2, area.y + (fy * dh) as u16 / 4);
            let sel = self.selected == Some(i);
            let style = if sel { q::special().add_modifier(Modifier::BOLD) } else { Style::new().fg(q::c(q::QUAG_HI)) };
            buf.set_string(cx, cy, if sel { G::CURVE_SELECTED } else { G::CURVE_POINT }, style);
        }
    }
}

/// One clip on a timeline track: label on a raised block; selected = Quag fill.
pub struct Clip<'a> {
    pub label: &'a str,
    pub selected: bool,
}

impl Widget for Clip<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width == 0 { return; }
        let area = Rect { height: 1, ..area };
        let style = if self.selected { Style::new().bg(q::c(q::QUAG)).fg(q::c(q::BG)).add_modifier(Modifier::BOLD) } else { Style::new().bg(q::c(q::RAISED)).fg(q::c(q::TEXT)) };
        buf.set_style(area, style);
        // left edge marks where the clip starts, even when the label doesn't fit
        buf[(area.x, area.y)].set_symbol("▏").set_fg(q::c(if self.selected { q::BG } else { q::BODY }));
        if area.width > 2 {
            let room = (area.width - 2) as usize;
            let label: String = if self.label.chars().count() > room { self.label.chars().take(room.saturating_sub(1)).chain(['…']).collect() } else { self.label.into() };
            buf.set_string(area.x + 1, area.y, label, style);
        }
    }
}

/// A node's frame in a graph: raised, rounded, title in the top border; Quag border when selected.
pub fn node_frame(title: &str, selected: bool) -> Block<'_> {
    let (border_c, title_s) = if selected { (q::c(q::QUAG), q::title()) } else { (q::c(q::BORDER), Style::new().fg(q::c(q::TEXT)).add_modifier(Modifier::BOLD)) };
    Block::new()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::new().fg(border_c))
        .style(Style::new().bg(q::c(q::RAISED)).fg(q::c(q::TEXT)))
        .title(Span::styled(format!(" {title} "), title_s))
}

/// A port on a node's edge: `●` connected, `○` free. Body blue; Quag when its node is selected.
pub fn port(connected: bool, active: bool) -> Span<'static> {
    Span::styled(if connected { G::PORT } else { G::PORT_FREE }, Style::new().fg(q::c(if active { q::QUAG } else { q::BODY })))
}

/// The style of a wire between nodes: faint, or Quag when it touches the selected node.
pub fn wire(active: bool) -> Style {
    Style::new().fg(q::c(if active { q::QUAG } else { q::FAINT }))
}

/// The playhead: `▼` on the ruler, `│` across the tracks. Gill, so it reads over clips.
pub fn playhead(head: bool) -> Span<'static> {
    Span::styled(if head { G::PLAYHEAD_HEAD } else { G::PLAYHEAD_LINE }, q::special().add_modifier(Modifier::BOLD))
}
