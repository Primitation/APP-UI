//! Quagsire mascot — **private / personal apps only** (see DESIGN.md §6).
//!
//! Plays the embedded GIF in a ratatui area using real terminal graphics
//! (kitty / sixel / iTerm2) and falls back to colored half-blocks elsewhere.
//!
//! ```ignore
//! // once, BEFORE entering raw mode / alternate screen:
//! let picker = quag_theme::mascot::picker();
//! let mut quag = quag_theme::mascot::Mascot::new(picker)?;
//! // in your draw closure:
//! quag.render(frame, area);
//! // in your event loop, poll with a timeout of `quag.next_frame_in()`
//! ```

use image::{AnimationDecoder, DynamicImage, Rgba, codecs::gif::GifDecoder};
use ratatui::{Frame, layout::Rect};
use ratatui_image::{Image, Resize, picker::Picker, protocol::Protocol};
use std::{
    io::Cursor,
    time::{Duration, Instant},
};

/// The GIF, embedded in the binary so every app has it.
pub static GIF: &[u8] = include_bytes!("../assets/quagsire.gif");

/// Detects the terminal's graphics protocol. Call before raw mode.
/// Falls back to half-blocks if the query fails.
pub fn picker() -> Picker {
    let mut p = Picker::from_query_stdio().unwrap_or_else(|_| Picker::halfblocks());
    // Transparent pixels blend into the app background, not black.
    p.set_background_color(Some(Rgba([0x0E, 0x16, 0x20, 0xFF])));
    p
}

pub struct Mascot {
    picker: Picker,
    frames: Vec<(DynamicImage, Duration)>,
    /// Encoded frames per drawing size (width, height), so moving between pages never re-encodes.
    encoded: Vec<((u16, u16), Vec<Protocol>)>,
    current: usize,
    last_switch: Instant,
    paused: bool,
}

impl Mascot {
    pub fn new(picker: Picker) -> Result<Self, Box<dyn std::error::Error>> {
        let decoder = GifDecoder::new(Cursor::new(GIF))?;
        let frames = decoder
            .into_frames()
            .collect_frames()?
            .into_iter()
            .map(|f| {
                let (n, d) = f.delay().numer_denom_ms();
                let ms = n.checked_div(d).map_or(100, |v| v.max(20));
                // Flatten transparency onto the app background (BG) so no black box shows.
                let mut buf = f.into_buffer();
                for px in buf.pixels_mut() {
                    let a = px[3] as u16;
                    for (i, bg) in [0x0Eu16, 0x16, 0x20].into_iter().enumerate() {
                        px[i] = ((px[i] as u16 * a + bg * (255 - a)) / 255) as u8;
                    }
                    px[3] = 255;
                }
                (DynamicImage::ImageRgba8(buf), Duration::from_millis(ms as u64))
            })
            .collect();
        Ok(Self {
            picker,
            frames,
            encoded: Vec::new(),
            current: 0,
            last_switch: Instant::now(),
            paused: false,
        })
    }

    /// Freeze on the current frame (e.g. when the app is busy or idle-timeout reached).
    pub fn set_paused(&mut self, paused: bool) {
        self.paused = paused;
    }

    /// How long until the next frame is due — use as your event-poll timeout.
    pub fn next_frame_in(&self) -> Duration {
        if self.paused || self.frames.is_empty() {
            return Duration::from_secs(3600);
        }
        self.frames[self.current]
            .1
            .saturating_sub(self.last_switch.elapsed())
    }

    fn advance(&mut self) {
        if self.paused || self.frames.is_empty() {
            return;
        }
        while self.last_switch.elapsed() >= self.frames[self.current].1 {
            self.last_switch += self.frames[self.current].1;
            self.current = (self.current + 1) % self.frames.len();
        }
    }

    /// Draws the current frame centred inside `area` (keeps aspect ratio).
    pub fn render(&mut self, f: &mut Frame, area: Rect) {
        if area.width < 4 || area.height < 2 {
            return;
        }
        let size = (area.width, area.height);
        if !self.encoded.iter().any(|(s, _)| *s == size) {
            // Encode all frames for this size once; afterwards drawing is cheap. Keep the last few sizes.
            let frames: Vec<Protocol> = self
                .frames
                .iter()
                .filter_map(|(img, _)| {
                    self.picker
                        .new_protocol(img.clone(), Rect::new(0, 0, size.0, size.1).into(), Resize::Fit(None))
                        .ok()
                })
                .collect();
            if self.encoded.len() >= crate::consts::layout::MASCOT_SIZES_KEPT { self.encoded.remove(0); }
            self.encoded.push((size, frames));
        }
        self.advance();
        let frames = &self.encoded.iter().find(|(s, _)| *s == size).expect("encoded just above").1;
        if let Some(p) = frames.get(self.current) {
            let r = p.size();
            let x = area.x + area.width.saturating_sub(r.width) / 2;
            let y = area.y + area.height.saturating_sub(r.height) / 2;
            let target = Rect::new(x, y, r.width.min(area.width), r.height.min(area.height));
            f.render_widget(Image::new(p), target);
        }
    }
}
