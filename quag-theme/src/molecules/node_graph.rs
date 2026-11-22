//! MOLECULE: Node graph.

use crate::atoms::{self, Tone};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::{Line, Span},
    widgets::Widget,
};
use crate::consts::layout as L;

/// One node: a title, its input and output port names.
#[derive(Clone, Copy, Debug)]
pub struct NodeSpec<'a> {
    pub title: &'a str,
    pub inputs: &'a [&'a str],
    pub outputs: &'a [&'a str],
}

impl NodeSpec<'_> {
    /// Outer size in cells (border included).
    pub fn size(&self) -> (u16, u16) {
        let w = |v: &[&str]| v.iter().map(|s| Span::raw(*s).width()).max().unwrap_or(0) as u16;
        let ports = w(self.inputs) + w(self.outputs) + L::NODE_PORT_GAP + 4;
        let title = Span::raw(self.title).width() as u16 + 4;
        (ports.max(title).max(L::NODE_MIN_COLS), self.inputs.len().max(self.outputs.len()).max(1) as u16 + 2)
    }
}

/// A wire: (node, output port) → (node, input port).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Wire {
    pub from: (usize, usize),
    pub to: (usize, usize),
}

/// Cells of a wire from `a` (just right of an output) to `b` (just left of an input), as (x, y, glyph):
/// across to the middle, up or down, across again; rounded corners.
pub fn route(a: (i32, i32), b: (i32, i32)) -> Vec<(i32, i32, &'static str)> {
    let mut out = Vec::new();
    let mid = (a.0 + b.0) / 2;
    let hline = |out: &mut Vec<_>, y: i32, x1: i32, x2: i32| { for x in x1.min(x2)..=x1.max(x2) { out.push((x, y, "─")); } };
    if a.1 == b.1 { hline(&mut out, a.1, a.0, b.0); return out; }
    let down = b.1 > a.1;
    hline(&mut out, a.1, a.0, mid);
    hline(&mut out, b.1, mid, b.0);
    for y in a.1.min(b.1) + 1..a.1.max(b.1) { out.push((mid, y, "│")); }
    // corner 1: joins the first line (from a's side) and the vertical (towards b)
    let from_left = a.0 <= mid;
    let c1 = match (from_left, down) { (true, true) => "╮", (true, false) => "╯", (false, true) => "╭", (false, false) => "╰" };
    // corner 2: joins the vertical (from a's side) and the last line (towards b)
    let to_right = b.0 >= mid;
    let c2 = match (to_right, down) { (true, true) => "╰", (true, false) => "╭", (false, true) => "╯", (false, false) => "╮" };
    out.retain(|c| !((c.0, c.1) == (mid, a.1) || (c.0, c.1) == (mid, b.1)));
    out.push((mid, a.1, c1));
    out.push((mid, b.1, c2));
    out
}

/// Nodes and the wires between them (abilities, dialogue, shaders, AI). Positions are cells from the
/// graph's top-left, minus `pan`. Wires are drawn first, nodes on top; the selected node and its wires are Quag.
pub struct NodeGraph<'a> {
    pub nodes: &'a [NodeSpec<'a>],
    pub positions: &'a [(i32, i32)],
    pub wires: &'a [Wire],
    pub selected: Option<usize>,
    pub pan: (i32, i32),
}

impl NodeGraph<'_> {
    /// Where node `i`'s output / input port `p` is (the cell on its border).
    pub fn port_cell(&self, i: usize, p: usize, output: bool) -> (i32, i32) {
        let (x, y) = self.positions[i];
        let (w, _) = self.nodes[i].size();
        (x + if output { w as i32 - 1 } else { 0 }, y + 1 + p as i32)
    }
}

impl Widget for NodeGraph<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let to_screen = |(x, y): (i32, i32)| -> Option<(u16, u16)> {
            let (sx, sy) = (area.x as i32 + x - self.pan.0, area.y as i32 + y - self.pan.1);
            (sx >= area.x as i32 && sy >= area.y as i32 && sx < area.right() as i32 && sy < area.bottom() as i32).then_some((sx as u16, sy as u16))
        };
        // wires
        for w in self.wires {
            let active = self.selected.is_some_and(|s| s == w.from.0 || s == w.to.0);
            let (a, b) = (self.port_cell(w.from.0, w.from.1, true), self.port_cell(w.to.0, w.to.1, false));
            for (x, y, g) in route((a.0 + 1, a.1), (b.0 - 1, b.1)) {
                if let Some((sx, sy)) = to_screen((x, y)) { buf.set_string(sx, sy, g, atoms::wire(active)); }
            }
        }
        // nodes: each drawn whole in its own buffer, then only the cells inside `area` are copied,
        // so a node half out of view is cut at the edge (and its wires still end at it)
        for (i, n) in self.nodes.iter().enumerate() {
            let sel = self.selected == Some(i);
            let (w, h) = n.size();
            let local = Rect::new(0, 0, w, h);
            let mut nb = Buffer::empty(local);
            let block = atoms::node_frame(n.title, sel);
            let inner = block.inner(local);
            block.render(local, &mut nb);
            let wired = |p: usize, out: bool| self.wires.iter().any(|wr| if out { wr.from == (i, p) } else { wr.to == (i, p) });
            for (p, name) in n.inputs.iter().enumerate().take(inner.height as usize) {
                let row = inner.y + p as u16;
                let pt = atoms::port(wired(p, false), sel);
                nb.set_string(0, row, pt.content.as_ref(), pt.style);
                Line::from(atoms::label(name, Tone::Muted)).render(Rect { x: inner.x + 1, y: row, width: inner.width.saturating_sub(1), height: 1 }, &mut nb);
            }
            for (p, name) in n.outputs.iter().enumerate().take(inner.height as usize) {
                let row = inner.y + p as u16;
                let pt = atoms::port(wired(p, true), sel);
                nb.set_string(w - 1, row, pt.content.as_ref(), pt.style);
                Line::from(atoms::label(name, Tone::Muted)).alignment(ratatui::layout::Alignment::Right).render(Rect { x: inner.x, y: row, width: inner.width.saturating_sub(1), height: 1 }, &mut nb);
            }
            let (x, y) = self.positions[i];
            for cy in 0..h {
                for cx in 0..w {
                    if let Some((sx, sy)) = to_screen((x + cx as i32, y + cy as i32)) { buf[(sx, sy)] = nb[(cx, cy)].clone(); }
                }
            }
        }
    }
}
