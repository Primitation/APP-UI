//! System checks: the stacks agree with each other, and the palette stays readable.
//! - Colors: Rust palette = CSS tokens (web/quag.css) = Unreal (QuagStyle.h).
//! - Constants: every name defined in both `src/consts.rs` and `solid/src/constants.ts` has the same value
//!   (same for the two demo consts files).
//! - Contrast: WCAG ratios for each text role on each background it's used on (DESIGN.md roles).

use quag_theme::hex;
use std::collections::BTreeMap;

const ROOT: &str = env!("CARGO_MANIFEST_DIR");

fn read(rel: &str) -> String {
    std::fs::read_to_string(format!("{ROOT}/{rel}")).unwrap_or_else(|e| panic!("{rel}: {e}"))
}

/// The Rust palette, by name.
fn palette() -> BTreeMap<&'static str, &'static str> {
    BTreeMap::from([
        ("BG", hex::BG), ("SURFACE", hex::SURFACE), ("RAISED", hex::RAISED), ("BORDER", hex::BORDER),
        ("TEXT", hex::TEXT), ("MUTED", hex::MUTED), ("FAINT", hex::FAINT),
        ("QUAG", hex::QUAG), ("QUAG_HI", hex::QUAG_HI), ("BODY", hex::BODY), ("GILL", hex::GILL),
        ("GILL_DEEP", hex::GILL_DEEP), ("NAVY", hex::NAVY), ("OK", hex::OK), ("WARN", hex::WARN), ("ERR", hex::ERR),
    ])
}

fn rgb(hex: &str) -> u32 { u32::from_str_radix(hex.trim_start_matches('#'), 16).unwrap() }

// ───────────── colors ─────────────

#[test]
fn css_tokens_match_palette() {
    let css = read("web/quag.css");
    for (name, value) in palette() {
        let token = format!("--q-{}:", name.to_lowercase().replace('_', "-"));
        let line = css.lines().find(|l| l.trim_start().starts_with(&token)).unwrap_or_else(|| panic!("quag.css has no {token}"));
        let got = line.split(':').nth(1).unwrap().split(';').next().unwrap().trim();
        assert_eq!(got.to_uppercase(), value.to_uppercase(), "{token} in quag.css ≠ Rust {name}");
    }
}

#[test]
fn unreal_colors_match_palette() {
    let h = read("unreal/QuagStyle/Source/QuagStyle/Public/QuagStyle.h");
    let mut checked = 0;
    for line in h.lines() {
        // `inline FLinearColor QuagHi()   { return Hex(0xAD, 0xE7, 0xFF); }` (alpha variants are tints: skipped)
        let Some((head, rest)) = line.split_once("() { return Hex(").or_else(|| line.split_once("()   { return Hex(")).or_else(|| {
            let i = line.find("{ return Hex(")?; Some((&line[..i], &line[i + 13..]))
        }) else { continue };
        let name = head.split_whitespace().last().unwrap().trim_end_matches("()");
        let args: Vec<&str> = rest.split(')').next().unwrap().split(',').map(str::trim).collect();
        if args.len() != 3 { continue }
        let snake: String = name.chars().enumerate().flat_map(|(i, c)| {
            if c.is_uppercase() && i > 0 { vec!['_', c] } else { vec![c.to_ascii_uppercase()] }
        }).collect();
        let Some(want) = palette().get(snake.as_str()).copied() else { continue };
        let got: String = args.iter().map(|a| a.trim_start_matches("0x").to_uppercase()).collect();
        assert_eq!(format!("#{got}"), want.to_uppercase(), "Unreal {name}() ≠ Rust {snake}");
        checked += 1;
    }
    assert!(checked >= 12, "only {checked} Unreal colors found: did the header's format change?");
}

// ───────────── constants ─────────────

/// `NAME → value` for simple literal constants (numbers, strings). Expressions (`60 * MINUTE`) are skipped.
fn literals(src: &str, prefix: &str) -> BTreeMap<String, String> {
    let mut m = BTreeMap::new();
    for line in src.lines().map(str::trim) {
        let Some(rest) = line.strip_prefix(prefix) else { continue };
        let Some((lhs, rhs)) = rest.split_once('=') else { continue };
        let name = lhs.split(':').next().unwrap().trim().to_string();
        let v = rhs.trim().trim_end_matches(';').trim().trim_end_matches(" as const").to_string();
        let v = v.strip_prefix("Duration::from_millis(").map(|x| x.trim_end_matches(')').to_string()).unwrap_or(v);
        let num = v.replace('_', "").parse::<f64>();
        if let Ok(n) = num { m.insert(name, format!("{n}")); }
        else if v.starts_with('"') && v.ends_with('"') || v.starts_with('\'') && v.ends_with('\'') { m.insert(name, v.trim_matches(['"', '\'']).to_string()); }
    }
    m
}

fn same_values(rust: &str, ts: &str, min_shared: usize) {
    let r = literals(&read(rust), "pub const ");
    let t = literals(&read(ts), "export const ");
    let shared: Vec<_> = r.keys().filter(|k| t.contains_key(*k)).collect();
    assert!(shared.len() >= min_shared, "{rust} and {ts} share only {} constants: {shared:?}", shared.len());
    for k in shared {
        assert_eq!(r[k], t[k], "{k}: {rust} has {} but {ts} has {}", r[k], t[k]);
    }
}

/// Unreal: `constexpr float UsageWarn = 0.8f;` → `USAGE_WARN → 0.8`.
fn unreal_literals(src: &str) -> BTreeMap<String, String> {
    let mut m = BTreeMap::new();
    for line in src.lines().map(str::trim) {
        let Some(rest) = line.strip_prefix("constexpr ") else { continue };
        let Some((lhs, rhs)) = rest.split_once('=') else { continue };
        let name = lhs.split_whitespace().last().unwrap();
        let snake: String = name.chars().enumerate().flat_map(|(i, c)| {
            if c.is_uppercase() && i > 0 { vec!['_', c] } else { vec![c.to_ascii_uppercase()] }
        }).collect();
        let v = rhs.trim().trim_end_matches(';').trim().trim_end_matches('f');
        if let Ok(n) = v.parse::<f64>() { m.insert(snake, format!("{n}")); }
    }
    m
}

#[test]
fn unreal_constants_match_rust() {
    let r = literals(&read("src/consts.rs"), "pub const ");
    let u = unreal_literals(&read("unreal/QuagStyle/Source/QuagStyle/Public/QuagStyle.h"));
    assert!(u.len() >= 7, "only {} QuagConsts found: did the header's format change?", u.len());
    for (k, v) in &u {
        let want = r.get(k).unwrap_or_else(|| panic!("QuagConsts has {k}, which src/consts.rs doesn't"));
        assert_eq!(v, want, "{k}: Unreal has {v} but src/consts.rs has {want}");
    }
}

#[test]
fn library_constants_match_between_stacks() {
    same_values("src/consts.rs", "solid/src/constants.ts", 10);
}

#[test]
fn demo_constants_match_between_stacks() {
    same_values("examples/explorer/consts.rs", "solid/explorer/consts.ts", 5);
}

// ───────────── contrast ─────────────

fn luminance(c: u32) -> f64 {
    let ch = |v: u32| {
        let s = v as f64 / 255.0;
        if s <= 0.03928 { s / 12.92 } else { ((s + 0.055) / 1.055).powf(2.4) }
    };
    0.2126 * ch(c >> 16 & 255) + 0.7152 * ch(c >> 8 & 255) + 0.0722 * ch(c & 255)
}

fn contrast(a: &str, b: &str) -> f64 {
    let (x, y) = (luminance(rgb(a)), luminance(rgb(b)));
    (x.max(y) + 0.05) / (x.min(y) + 0.05)
}

/// WCAG AA for normal text.
const AA_TEXT: f64 = 4.5;
/// WCAG for large text, icons and UI parts; also our floor for placeholder / disabled text.
const AA_LARGE: f64 = 3.0;

#[test]
fn text_roles_are_readable_on_every_background() {
    let p = palette();
    let backgrounds = ["BG", "SURFACE", "RAISED"];
    // every color DESIGN.md allows for text
    for fg in ["TEXT", "MUTED", "QUAG", "BODY", "GILL", "OK", "WARN", "ERR"] {
        for bg in backgrounds {
            let r = contrast(p[fg], p[bg]);
            assert!(r >= AA_TEXT, "{fg} on {bg} = {r:.2}:1, needs {AA_TEXT}");
        }
    }
    // FAINT: placeholders and disabled only (WCAG exempts disabled; we still keep 3:1)
    for bg in backgrounds {
        let r = contrast(p["FAINT"], p[bg]);
        assert!(r >= AA_LARGE, "FAINT on {bg} = {r:.2}:1, needs {AA_LARGE}");
    }
}

#[test]
fn dark_text_on_colored_fills_is_readable() {
    let p = palette();
    // primary button, CRIT badge, Segmented active, secondary button (TEXT on BORDER)
    for (fg, bg) in [("BG", "QUAG"), ("BG", "QUAG_HI"), ("BG", "ERR"), ("TEXT", "BORDER"), ("MUTED", "BORDER")] {
        let r = contrast(p[fg], p[bg]);
        assert!(r >= AA_TEXT, "{fg} on {bg} = {r:.2}:1, needs {AA_TEXT}");
    }
}

#[test]
fn focus_ring_stands_out() {
    let p = palette();
    for bg in ["BG", "SURFACE", "RAISED"] {
        assert!(contrast(p["QUAG"], p[bg]) >= AA_LARGE, "focus (QUAG) on {bg}");
    }
}

/// One animation clock: Rust `motion::TICK_MS` = CSS `--q-tick` = Solid `TICK_MS`, so spinners turn at one speed everywhere.
#[test]
fn motion_tick_matches() {
    let tick = quag_theme::consts::motion::TICK_MS;
    assert!(read("web/quag.css").contains(&format!("--q-tick: {tick}ms;")), "quag.css --q-tick ≠ Rust TICK_MS");
    assert!(read("solid/src/constants.ts").contains(&format!("export const TICK_MS = {tick};")), "constants.ts TICK_MS ≠ Rust");
}
