//! Tests of the molecules.
use super::*;
use crate::atoms::{Button, Input};
use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

fn text_at(buf: &Buffer, y: u16) -> String {
    (0..buf.area.width).map(|x| buf[(x, y)].symbol().to_string()).collect()
}

#[test]
fn nudge_keeps_keys_in_order_and_range() {
    let mut p = vec![(0.0, 1.0), (0.5, 1.5), (1.0, 2.0)];
    nudge_point(&mut p, 1, 0.9, 5.0, [0.0, 1.0], [0.0, 3.0]);
    assert_eq!(p[1], (1.0, 3.0), "clamped to next key's x and to y max");
    nudge_point(&mut p, 0, 0.3, -9.0, [0.0, 1.0], [0.0, 3.0]);
    assert_eq!(p[0], (0.0, 0.0), "first key keeps its x");
}

#[test]
fn wires_turn_with_round_corners() {
    let r = route((0, 0), (6, 3));
    assert!(r.contains(&(3, 0, "╮")) && r.contains(&(3, 3, "╰")), "{r:?}");
    assert!(r.contains(&(0, 0, "─")) && r.contains(&(6, 3, "─")) && r.contains(&(3, 1, "│")));
    let up = route((0, 3), (6, 0));
    assert!(up.contains(&(3, 3, "╯")) && up.contains(&(3, 0, "╭")));
    assert_eq!(route((0, 2), (4, 2)).len(), 5, "same row: a straight line");
}

#[test]
fn graph_cuts_nodes_at_the_edge_instead_of_hiding_them() {
    let nodes = [NodeSpec { title: "A", inputs: &[], outputs: &["out"] }, NodeSpec { title: "B", inputs: &["in"], outputs: &[] }];
    let at = [(0, 0), (20, 4)];
    let wires = [Wire { from: (0, 0), to: (1, 0) }];
    let area = Rect::new(0, 0, 40, 6); // B is 3 rows tall from y 4: only its top 2 rows fit
    let mut buf = Buffer::empty(area);
    NodeGraph { nodes: &nodes, positions: &at, wires: &wires, selected: None, pan: (0, 0) }.render(area, &mut buf);
    assert_eq!(buf[(20, 4)].symbol(), "╭", "B's top-left corner is drawn although B doesn't fit");
    assert_eq!(buf[(20, 5)].symbol(), crate::consts::glyph::PORT, "the wire ends at B's port");
}

#[test]
fn shares_always_total_one() {
    let s = shares(&[50.0, 30.0, 15.0, 5.0]);
    assert!((s.iter().sum::<f64>() - 1.0).abs() < 1e-9);
    assert_eq!(shares(&[0.0, 0.0]), vec![0.0, 0.0]);
}

#[test]
fn ruler_steps_are_nice() {
    assert_eq!(Timeline::ruler_step(60.0, 100), 10.0);
    assert_eq!(Timeline::ruler_step(12.0, 120), 1.0);
    assert_eq!(Timeline::ruler_step(30.0, 40), 10.0);
}

#[test]
fn field_never_overlaps_when_squeezed() {
    for h in 1..=3u16 {
        let mut buf = Buffer::empty(Rect::new(0, 0, 30, h));
        Field::new("Label", Input::new("value")).help("help").render(buf.area, &mut buf);
        let all: String = (0..h).map(|y| text_at(&buf, y)).collect();
        assert!(all.contains("value"), "input must always show (h={h})");
        if h < 3 { assert!(!all.contains("help"), "help must drop first (h={h})"); }
    }
}

#[test]
fn button_group_never_drops_a_button() {
    let group = || ButtonGroup::new(vec![Button::new("Mark done").primary(), Button::new("Edit"), Button::new("Delete").danger()]);
    for w in [40u16, 26, 20] {
        let mut buf = Buffer::empty(Rect::new(0, 0, w, 3));
        group().render(buf.area, &mut buf);
        let all: String = (0..3).map(|y| text_at(&buf, y)).collect();
        for label in ["Mark done", "Edit", "Delete"] {
            assert!(all.contains(label), "{label} missing at width {w}");
        }
    }
}

#[test]
fn key_hints_drop_whole_pairs() {
    // "q quit" = 6 cells; "q quit  ·  / search" = 19 cells → at 12 only the first fits
    let mut buf = Buffer::empty(Rect::new(0, 0, 12, 1));
    KeyHints(&[("q", "quit"), ("/", "search"), ("?", "help")]).render(buf.area, &mut buf);
    let t = text_at(&buf, 0);
    assert!(t.contains("q quit") && !t.contains("sear"), "got {t:?}");
}
