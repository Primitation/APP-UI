//! Tests of the atoms.
use super::*;
use crate::Status;
use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

#[test]
fn a_focused_button_is_marked_and_keeps_its_width() {
    let b = Button::new("Edit");
    assert_eq!(b.to_span().content, "  Edit  ");
    assert_eq!(b.clone().focused(true).to_span().content, "▸ Edit  ");
    assert_eq!(b.clone().compact(true).focused(true).to_span().content, "▸Edit ");
}

#[test]
fn smooth_progress_moves_by_eighths() {
    assert_eq!(SmoothProgress(0.5).eighths(10), 40);
    assert_eq!(SmoothProgress(0.01).eighths(10), 1, "1 % already shows");
    assert_eq!(SmoothProgress(2.0).eighths(10), 80, "clamped");
}

#[test]
fn step_segments_fill_the_width_with_gaps() {
    let s = StepProgress { total: 5, done: 2, current: 0.5, failed: false };
    let seg = s.segments(29);
    assert_eq!(seg.len(), 5);
    assert_eq!(seg.iter().map(|x| x.1).sum::<u16>() + 4, 29, "segments + 4 gaps = the width");
    assert!(seg.windows(2).all(|w| w[1].0 == w[0].0 + w[0].1 + 1), "1-cell gaps");
    let mut buf = Buffer::empty(Rect::new(0, 0, 29, 1));
    s.render(buf.area, &mut buf);
    let line: String = (0..29).map(|x| buf[(x, 0)].symbol().to_string()).collect();
    assert_eq!(line, "━━━━━ ━━━━━ ━━╸── ───── ─────");
}

#[test]
fn secret_masks_all_but_last_four() {
    let s = SecretField::new("sk_live_51HxQ2bK9vTz7m3f9a");
    assert!(s.masked().ends_with("3f9a"));
    assert!(!s.masked().contains("sk_live"));
    assert_eq!(SecretField::new("abc").masked(), "abc", "short secrets: nothing to hide behind");
}

#[test]
fn severity_badges_line_up() {
    let w: Vec<usize> = Severity::ALL.iter().map(|s| severity_badge(*s).width()).collect();
    assert!(w.iter().all(|x| *x == w[0]), "all badges same width: {w:?}");
    assert!(Severity::Critical < Severity::Info, "sorts worst first");
}

#[test]
fn usage_bar_thresholds() {
    assert_eq!(UsageBar(0.5).status(), None);
    assert_eq!(UsageBar(0.8).status(), Some(Status::Warn));
    assert_eq!(UsageBar(1.0).status(), Some(Status::Err));
}

#[test]
fn stopped_is_not_an_error() {
    assert_eq!(RunState::Stopped.status(), None);
    assert_eq!(RunState::Exited(0).status(), None);
    assert_eq!(RunState::Exited(137).status(), Some(Status::Err));
    assert_eq!(RunState::Flapping(5).status(), Some(Status::Warn), "restart loop warns even while up");
}

#[test]
fn short_id_drops_prefix() {
    assert_eq!(short_id("sha256:3f9a1c2b7e41aa00ff").content, "3f9a1c2b7e41");
}

#[test]
fn delta_knows_which_way_is_good() {
    assert_eq!(delta_badge(-0.1, false).style, Status::Ok.style(), "frame time down = good");
    assert_eq!(delta_badge(-0.1, true).style, Status::Err.style(), "fps down = bad");
}
