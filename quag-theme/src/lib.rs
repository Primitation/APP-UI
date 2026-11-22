//! # quag-theme
//!
//! The house style for every terminal app ("Quag": calm & modern, dark only, Quagsire colors),
//! organised as an **atomic system** (STRUCTURE.md):
//!
//! | Level | Module | What |
//! |---|---|---|
//! | Tokens | crate root | colors + text roles (`q::QUAG`, `q::text()`…): the matter atoms are made of |
//! | Atom | [`atoms`] | one control: `Button`, `Checkbox`, `Input`, `tag`, `kbd`, `panel`… |
//! | Molecule | [`molecules`] | atoms arranged in a box: `ButtonGroup`, `Field`, `SearchBar`, `Tabs`, `ListRow`… |
//! | Element | [`elements`] | a molecule put to use. Shared ones live here (`AppHeader`, `StatusBar`, `EmptyState`, `ConfirmDialog`); app-specific ones live in the app |
//! | UI | [`ui`] | a page arranging elements: `ui::shell`, breakpoints, page layouts |
//!
//! Rule: each level only uses the levels below it. Only atoms set colors.

use ratatui::style::{Color, Modifier, Style};
use std::sync::OnceLock;

pub mod atoms;
pub mod consts;
pub mod elements;
pub mod fmt;
pub mod keymap;
pub mod molecules;
pub mod ui;

#[cfg(feature = "mascot")]
pub mod mascot;

// ───────────────────────────── Palette ─────────────────────────────
//
// Raw palette. Apps should normally use the *role* helpers further down
// (`text()`, `accent()`, `panel()`...), not these constants directly.

/// Hex values, handy for docs, configs, or non-ratatui code.
pub mod hex {
    // Water (backgrounds) — deep pond, tinted blue, never pure black.
    pub const BG: &str = "#0E1620";
    pub const SURFACE: &str = "#152230";
    pub const RAISED: &str = "#1C2D3F";
    pub const BORDER: &str = "#2A3F55";
    // Ink (text)
    pub const TEXT: &str = "#E4F3FB";
    pub const MUTED: &str = "#93ADC2";
    pub const FAINT: &str = "#5F778D";
    // Quagsire (brand)
    pub const QUAG: &str = "#84D6F7"; // light body — THE accent
    pub const QUAG_HI: &str = "#ADE7FF"; // face highlight — hover/pressed
    pub const BODY: &str = "#639CBD"; // main body — secondary
    pub const GILL: &str = "#A897D6"; // gills/tail, lifted for dark bg — tertiary
    pub const GILL_DEEP: &str = "#6B5A8C"; // raw gill — decoration only
    pub const NAVY: &str = "#425284"; // sprite outline — decoration only
    // Status
    pub const OK: &str = "#7DD3A8";
    pub const WARN: &str = "#F2C46D";
    pub const ERR: &str = "#F27D7F";
}

const fn rgb(h: u32) -> Color {
    Color::Rgb((h >> 16) as u8, (h >> 8) as u8, h as u8)
}

pub const BG: Color = rgb(0x0E1620);
pub const SURFACE: Color = rgb(0x152230);
pub const RAISED: Color = rgb(0x1C2D3F);
pub const BORDER: Color = rgb(0x2A3F55);
pub const TEXT: Color = rgb(0xE4F3FB);
pub const MUTED: Color = rgb(0x93ADC2);
pub const FAINT: Color = rgb(0x5F778D);
pub const QUAG: Color = rgb(0x84D6F7);
pub const QUAG_HI: Color = rgb(0xADE7FF);
pub const BODY: Color = rgb(0x639CBD);
pub const GILL: Color = rgb(0xA897D6);
pub const GILL_DEEP: Color = rgb(0x6B5A8C);
pub const NAVY: Color = rgb(0x425284);
pub const OK: Color = rgb(0x7DD3A8);
pub const WARN: Color = rgb(0xF2C46D);
pub const ERR: Color = rgb(0xF27D7F);

// ───────────────────── Truecolor / 256-color fallback ─────────────────────

fn truecolor() -> bool {
    static TC: OnceLock<bool> = OnceLock::new();
    *TC.get_or_init(|| {
        if std::env::var_os("QUAG_FORCE_256").is_some() {
            return false;
        }
        let ct = std::env::var("COLORTERM").unwrap_or_default();
        let term = std::env::var("TERM").unwrap_or_default();
        ct.contains("truecolor")
            || ct.contains("24bit")
            || term.contains("kitty")
            || term.contains("direct")
            || std::env::var_os("WT_SESSION").is_some()
    })
}

/// Returns the color as-is on truecolor terminals, or the nearest xterm-256
/// color otherwise. All helpers in this crate already go through this.
pub fn c(color: Color) -> Color {
    match color {
        Color::Rgb(r, g, b) if !truecolor() => Color::Indexed(nearest_256(r, g, b)),
        other => other,
    }
}

fn nearest_256(r: u8, g: u8, b: u8) -> u8 {
    const STEPS: [u8; 6] = [0, 95, 135, 175, 215, 255];
    let idx = |v: u8| -> usize {
        STEPS
            .iter()
            .enumerate()
            .min_by_key(|(_, s)| (**s as i32 - v as i32).abs())
            .map(|(i, _)| i)
            .unwrap()
    };
    let (ri, gi, bi) = (idx(r), idx(g), idx(b));
    let cube = 16 + 36 * ri + 6 * gi + bi;
    let cube_rgb = (STEPS[ri], STEPS[gi], STEPS[bi]);
    let avg = (r as u32 + g as u32 + b as u32) / 3;
    let gi_ = ((avg.saturating_sub(8)) / 10).min(23) as u8;
    let gray = 8 + 10 * gi_;
    let d = |(a, b2, c2): (u8, u8, u8)| {
        let f = |x: u8, y: u8| (x as i32 - y as i32).pow(2);
        f(a, r) + f(b2, g) + f(c2, b)
    };
    if d((gray, gray, gray)) < d(cube_rgb) {
        232 + gi_
    } else {
        cube as u8
    }
}

// ───────────────────────────── Roles ─────────────────────────────
// Each function = one role from DESIGN.md. If you need a style that is not
// here, the design is probably asking for something it shouldn't.

/// Whole-screen base. Paint this first: `f.render_widget(Block::new().style(q::base()), f.area())`.
pub fn base() -> Style {
    Style::new().fg(c(TEXT)).bg(c(BG))
}
/// Normal body text.
pub fn text() -> Style {
    Style::new().fg(c(TEXT))
}
/// Secondary info: descriptions, timestamps, counts, inactive tabs.
pub fn muted() -> Style {
    Style::new().fg(c(MUTED))
}
/// Placeholders, disabled items, separators. Never for text that must be read.
pub fn faint() -> Style {
    Style::new().fg(c(FAINT))
}
/// The ONE thing that matters most right now (focused item, primary action, links).
pub fn accent() -> Style {
    Style::new().fg(c(QUAG))
}
/// Screen / panel titles.
pub fn title() -> Style {
    Style::new().fg(c(QUAG)).add_modifier(Modifier::BOLD)
}
/// Section headers inside a panel, table headers.
pub fn heading() -> Style {
    Style::new().fg(c(BODY)).add_modifier(Modifier::BOLD)
}
/// Keyboard keys, tags, "special" highlights (search matches, badges).
pub fn special() -> Style {
    Style::new().fg(c(GILL))
}
/// Selected row in a list/table (focused list).
pub fn selected() -> Style {
    Style::new()
        .fg(c(QUAG_HI))
        .bg(c(RAISED))
        .add_modifier(Modifier::BOLD)
}
/// Selected row when the list does NOT have focus.
pub fn selected_unfocused() -> Style {
    Style::new().fg(c(TEXT)).bg(c(SURFACE))
}
/// Primary button / call-to-action: dark text on Quag blue. Max one per screen.
pub fn button_primary() -> Style {
    Style::new()
        .fg(c(BG))
        .bg(c(QUAG))
        .add_modifier(Modifier::BOLD)
}
/// Secondary button. Uses the BORDER tone so it stays visible on BG, SURFACE *and* RAISED (popups).
pub fn button() -> Style {
    Style::new().fg(c(TEXT)).bg(c(BORDER))
}

/// Status kinds — only for real states, never for decoration.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    Ok,
    Warn,
    Err,
    Info,
}
impl Status {
    pub fn color(self) -> Color {
        c(match self {
            Status::Ok => OK,
            Status::Warn => WARN,
            Status::Err => ERR,
            Status::Info => BODY,
        })
    }
    /// Icon so meaning never relies on color alone.
    pub fn icon(self) -> &'static str {
        match self {
            Status::Ok => "✓",
            Status::Warn => "!",
            Status::Err => "✗",
            Status::Info => "i",
        }
    }
    pub fn style(self) -> Style {
        Style::new().fg(self.color())
    }
}

/// Gauge / progress bar style (fg = filled part).
pub fn gauge() -> Style {
    Style::new().fg(c(QUAG)).bg(c(SURFACE))
}

/// Series colors for charts/sparklines, in the order to use them.
pub fn series() -> [Color; 5] {
    [c(QUAG), c(GILL), c(OK), c(WARN), c(BODY)]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fallback_maps_into_range() {
        for (r, g, b) in [(0x0E, 0x16, 0x20), (0x84, 0xD6, 0xF7), (255, 255, 255)] {
            assert!(nearest_256(r, g, b) >= 16);
        }
    }
}
