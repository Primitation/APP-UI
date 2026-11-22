#![cfg(feature = "mascot")]
use ratatui::{Terminal, backend::TestBackend};
#[test]
fn mascot_renders_halfblocks() {
    let mut p = ratatui_image::picker::Picker::halfblocks();
    p.set_background_color(Some(image::Rgba([0x0E, 0x16, 0x20, 0xFF])));
    let mut m = quag_theme::mascot::Mascot::new(p).unwrap();
    let mut t = Terminal::new(TestBackend::new(40, 16)).unwrap();
    for _ in 0..3 {
        t.draw(|f| m.render(f, f.area())).unwrap();
    }
    let buf = t.backend().buffer();
    let painted = buf.content().iter().filter(|c| c.symbol() != " ").count();
    assert!(painted > 50, "only {painted} cells painted");
    assert!(m.next_frame_in() <= std::time::Duration::from_secs(1));
}
