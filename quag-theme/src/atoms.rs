//! ATOMS: the smallest parts. One control or one visual, with its own look and states.
//! An atom knows nothing about the app: `Button`, never `SaveButton`.
//! See STRUCTURE.md §2.

use crate::{
    self as q, Status,
    consts::{format as F, glyph as G, layout as L, rules as R},
};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Padding, Widget},
};

/// Interaction state shared by all interactive atoms.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum State {
    #[default]
    Normal,
    Focused,
    Disabled,
}

// ───────────────────────────── Button ─────────────────────────────

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonKind {
    /// Max one per screen.
    Primary,
    #[default]
    Secondary,
    Danger,
    Ghost,
}

#[derive(Clone, Debug)]
pub struct Button<'a> {
    pub label: &'a str,
    pub kind: ButtonKind,
    pub state: State,
    /// 1 cell of padding instead of 2 (molecules switch this on when space is tight).
    pub compact: bool,
}

impl<'a> Button<'a> {
    pub fn new(label: &'a str) -> Self {
        Self { label, kind: ButtonKind::Secondary, state: State::Normal, compact: false }
    }
    pub fn primary(mut self) -> Self { self.kind = ButtonKind::Primary; self }
    pub fn danger(mut self) -> Self { self.kind = ButtonKind::Danger; self }
    pub fn ghost(mut self) -> Self { self.kind = ButtonKind::Ghost; self }
    pub fn focused(mut self, on: bool) -> Self { if on { self.state = State::Focused } self }
    pub fn disabled(mut self, on: bool) -> Self { if on { self.state = State::Disabled } self }

    pub fn compact(mut self, on: bool) -> Self { self.compact = on; self }

    /// Cells it takes: 2 padding each side (1 when compact).
    pub fn width(&self) -> u16 {
        Span::raw(self.label).width() as u16 + if self.compact { L::BUTTON_PAD_COMPACT } else { L::BUTTON_PAD }
    }

    pub fn style(&self) -> Style {
        if self.state == State::Disabled {
            return Style::new().fg(q::c(q::FAINT)).bg(q::c(q::SURFACE));
        }
        let focused = self.state == State::Focused;
        match self.kind {
            ButtonKind::Primary => Style::new()
                .fg(q::c(q::BG))
                .bg(q::c(if focused { q::QUAG_HI } else { q::QUAG }))
                .add_modifier(Modifier::BOLD),
            ButtonKind::Secondary => Style::new()
                .fg(q::c(if focused { q::QUAG_HI } else { q::TEXT }))
                .bg(q::c(q::BORDER)),
            ButtonKind::Danger => Style::new().fg(q::c(q::ERR)).bg(q::c(q::BORDER)).add_modifier(Modifier::BOLD),
            ButtonKind::Ghost => Style::new().fg(q::c(if focused { q::QUAG_HI } else { q::MUTED })),
        }
        .add_modifier(if focused { Modifier::BOLD | Modifier::UNDERLINED } else { Modifier::empty() })
    }

    pub fn to_span(&self) -> Span<'static> {
        let pad = if self.compact { " " } else { "  " };
        // keyboard focus: a `▸` in the left padding (same width), like the `▌` of a selected row
        let lead = match (self.state == State::Focused, self.compact) { (true, true) => G::FOCUS_MARK.to_string(), (true, false) => format!("{} ", G::FOCUS_MARK), _ => pad.to_string() };
        Span::styled(format!("{lead}{}{pad}", self.label), self.style())
    }
}

impl Widget for Button<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Line::from(self.to_span()).render(area, buf);
    }
}

// ───────────────────────────── Checkbox ─────────────────────────────

#[derive(Clone, Debug)]
pub struct Checkbox<'a> {
    pub label: &'a str,
    pub checked: bool,
    pub state: State,
}

impl<'a> Checkbox<'a> {
    pub fn new(label: &'a str, checked: bool) -> Self { Self { label, checked, state: State::Normal } }
    pub fn focused(mut self, on: bool) -> Self { if on { self.state = State::Focused } self }
    pub fn disabled(mut self, on: bool) -> Self { if on { self.state = State::Disabled } self }
    pub fn width(&self) -> u16 { Span::raw(self.label).width() as u16 + L::CHOICE_MARK }

    pub fn to_line(&self) -> Line<'static> {
        let (boxs, label) = match self.state {
            State::Disabled => (q::faint(), q::faint()),
            State::Focused => (q::accent(), q::selected().bg(q::c(q::BG))),
            State::Normal => (if self.checked { q::accent() } else { q::faint() }, q::text()),
        };
        Line::from(vec![
            Span::styled(if self.checked { "[✓]" } else { "[ ]" }, boxs),
            Span::raw(" "),
            Span::styled(self.label.to_string(), label),
        ])
    }
}

impl Widget for Checkbox<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) { self.to_line().render(area, buf) }
}

// ───────────────────────────── Input ─────────────────────────────

/// One-line text input. Fills its whole area width.
#[derive(Clone, Debug, Default)]
pub struct Input<'a> {
    pub value: &'a str,
    pub placeholder: &'a str,
    pub state: State,
    pub invalid: bool,
}

impl<'a> Input<'a> {
    pub fn new(value: &'a str) -> Self { Self { value, ..Default::default() } }
    pub fn placeholder(mut self, p: &'a str) -> Self { self.placeholder = p; self }
    pub fn focused(mut self, on: bool) -> Self { if on { self.state = State::Focused } self }
    pub fn disabled(mut self, on: bool) -> Self { if on { self.state = State::Disabled } self }
    pub fn invalid(mut self, on: bool) -> Self { self.invalid = on; self }
}

impl Widget for Input<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area = Rect { height: 1, ..area };
        let focused = self.state == State::Focused;
        let bg = q::c(if focused { q::RAISED } else { q::SURFACE });
        buf.set_style(area, Style::new().bg(bg));
        let mut spans = vec![Span::raw(" ")];
        if self.value.is_empty() {
            spans.push(Span::styled(self.placeholder.to_string(), q::faint()));
        } else {
            let s = if self.state == State::Disabled { q::faint() } else { q::text() };
            spans.push(Span::styled(self.value.to_string(), s));
        }
        if focused {
            spans.push(Span::styled("▏", q::accent()));
        }
        Line::from(spans).render(area, buf);
        if self.invalid && area.width > L::FIELD_ICON {
            buf.set_string(area.right() - L::FIELD_ICON, area.y, "✗", Status::Err.style().bg(bg));
        }
        // focus / error underline: last row hint via a thin left bar
        let bar = if self.invalid { q::c(q::ERR) } else if focused { q::c(q::QUAG) } else { bg };
        buf[(area.x, area.y)].set_symbol("▎").set_fg(bar);
    }
}

// ───────────────────────────── Small text atoms ─────────────────────────────

/// `#tag` in Gill purple.
pub fn tag(text: &str) -> Span<'static> {
    Span::styled(format!("#{text}"), q::special())
}

/// A keyboard key, e.g. `kbd("/")`.
pub fn kbd(key: &str) -> Span<'static> {
    Span::styled(key.to_string(), q::special().add_modifier(Modifier::BOLD))
}

/// Just the colored status icon (✓ ! ✗ i).
pub fn status_icon(kind: Status) -> Span<'static> {
    Span::styled(kind.icon(), kind.style())
}

/// Text tones. Use instead of picking styles by hand.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tone {
    Title,
    Heading,
    Text,
    Muted,
    Faint,
    Accent,
}

pub fn label(text: &str, tone: Tone) -> Span<'static> {
    let s = match tone {
        Tone::Title => q::title(),
        Tone::Heading => q::heading(),
        Tone::Text => q::text(),
        Tone::Muted => q::muted(),
        Tone::Faint => q::faint(),
        Tone::Accent => q::accent(),
    };
    Span::styled(text.to_string(), s)
}

/// Spinner frame for `tick` (from `consts::motion::ticks(clock.elapsed())`: never count redraws).
pub fn spinner(tick: usize) -> Span<'static> {
    Span::styled(G::SPINNER[tick % G::SPINNER.len()], q::accent())
}

// ───────────────────────────── Divider & Progress ─────────────────────────────

pub struct Divider;
impl Widget for Divider {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Line::styled("─".repeat(area.width as usize), Style::new().fg(q::c(q::BORDER)))
            .render(Rect { height: 1, ..area }, buf);
    }
}

/// Thin progress bar: `━━━━━━────` (numbers go next to it, never on it).
pub struct Progress(pub f64);
impl Widget for Progress {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let w = area.width as usize;
        let filled = ((self.0.clamp(0.0, 1.0)) * w as f64).round() as usize;
        Line::from(vec![
            Span::styled(G::LINE_FULL.repeat(filled), q::accent()),
            Span::styled(G::LINE_FULL.repeat(w - filled), Style::new().fg(q::c(q::BORDER))),
        ])
        .render(Rect { height: 1, ..area }, buf);
    }
}

/// Smooth progress: fills in eighths of a cell (`█████▋`), the Quag fill on a surface track.
/// For anything that moves by small amounts (downloads, copies). Numbers go next to it, never on it.
pub struct SmoothProgress(pub f64);
impl SmoothProgress {
    /// Filled eighths for a bar `width` cells wide.
    pub fn eighths(&self, width: u16) -> usize {
        (self.0.clamp(0.0, 1.0) * width as f64 * G::EIGHTHS as f64).round() as usize
    }
}
impl Widget for SmoothProgress {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let w = area.width as usize;
        let e = self.eighths(area.width);
        let mut s = G::EIGHTHS_RIGHT[G::EIGHTHS].repeat(e / G::EIGHTHS);
        s.push_str(G::EIGHTHS_RIGHT[e % G::EIGHTHS]);
        let used = e / G::EIGHTHS + usize::from(!e.is_multiple_of(G::EIGHTHS));
        let track = Style::new().bg(q::c(q::SURFACE));
        Line::from(vec![Span::styled(s, q::accent().patch(track)), Span::styled(" ".repeat(w.saturating_sub(used)), track)])
            .render(Rect { height: 1, ..area }, buf);
    }
}

/// Stepped progress: one segment per step (`━━━━ ━━━━ ━━╸─ ──── ────`). Done steps are full,
/// the current one fills with its own progress (`current`, 0–1), the rest are track. `failed`: the current step is red.
/// For jobs with known steps (deploy, install, wizard). Say which step next to it: "Step 3 of 5 · Health check".
pub struct StepProgress {
    pub total: usize,
    pub done: usize,
    pub current: f64,
    pub failed: bool,
}
impl StepProgress {
    /// Each step's cells `(x, width)` in a bar `width` wide, a 1-cell gap between steps.
    pub fn segments(&self, width: u16) -> Vec<(u16, u16)> {
        let n = self.total.max(1) as u16;
        let room = width.saturating_sub(n - 1);
        (0..n).map(|i| {
            let x0 = i * room / n;
            let x1 = (i + 1) * room / n;
            (x0 + i, x1 - x0)
        }).collect()
    }
}
impl Widget for StepProgress {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let track = Style::new().fg(q::c(q::BORDER));
        let fill = if self.failed { Status::Err.style() } else { q::accent() };
        for (i, (x, w)) in self.segments(area.width).into_iter().enumerate() {
            let r = Rect { x: area.x + x, y: area.y, width: w, height: 1 };
            let (full, half) = match i.cmp(&self.done) {
                std::cmp::Ordering::Less => (w as usize, false),
                std::cmp::Ordering::Equal => {
                    let halves = (self.current.clamp(0.0, 1.0) * w as f64 * 2.0).round() as usize;
                    (halves / 2, halves % 2 == 1)
                }
                std::cmp::Ordering::Greater => (0, false),
            };
            let style = if i < self.done { q::accent() } else { fill };
            let mut spans = vec![Span::styled(G::LINE_FULL.repeat(full), style)];
            if half { spans.push(Span::styled(G::LINE_HALF.to_string(), style)); }
            spans.push(Span::styled(G::LINE_TRACK.repeat((w as usize).saturating_sub(full + usize::from(half))), track));
            Line::from(spans).render(r, buf);
        }
    }
}

// ───────────────────────────── Containers (the "box") ─────────────────────────────

/// Panel frame: rounded border, 1-cell padding. Focused = Quag border + bright title.
pub fn panel<'a>(title_text: &'a str, focused: bool) -> Block<'a> {
    let (border_c, title_s) = if focused { (q::c(q::QUAG), q::title()) } else { (q::c(q::BORDER), q::muted()) };
    let b = Block::new()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::new().fg(border_c))
        .style(Style::new().bg(q::c(q::BG)).fg(q::c(q::TEXT)))
        .padding(Padding::horizontal(L::PANEL_PAD_X));
    if title_text.is_empty() { b } else { b.title(Span::styled(format!(" {title_text} "), title_s)) }
}

/// Popup frame: raised background, Quag border, roomy padding.
/// Outer size of a popup whose content needs `w × h` (adds border + padding).
pub fn popup_size(w: u16, h: u16) -> (u16, u16) {
    let probe = Rect::new(0, 0, 200, 200);
    let inner = popup("").inner(probe);
    (w + probe.width - inner.width, h + probe.height - inner.height)
}

pub fn popup<'a>(title_text: &'a str) -> Block<'a> {
    Block::new()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::new().fg(q::c(q::QUAG)))
        .title(Span::styled(format!(" {title_text} "), q::title()))
        .style(Style::new().bg(q::c(q::RAISED)).fg(q::c(q::TEXT)))
        .padding(Padding::new(L::POPUP_PAD_X, L::POPUP_PAD_X, L::POPUP_PAD_Y, L::POPUP_PAD_Y))
}

// ───────────────────────────── Radio ─────────────────────────────

/// One option of a single choice: `(●) label` / `( ) label`.
#[derive(Clone, Debug)]
pub struct Radio<'a> {
    pub label: &'a str,
    pub selected: bool,
    pub state: State,
}

impl<'a> Radio<'a> {
    pub fn new(label: &'a str, selected: bool) -> Self { Self { label, selected, state: State::Normal } }
    pub fn focused(mut self, on: bool) -> Self { if on { self.state = State::Focused } self }
    pub fn disabled(mut self, on: bool) -> Self { if on { self.state = State::Disabled } self }
    pub fn width(&self) -> u16 { Span::raw(self.label).width() as u16 + L::CHOICE_MARK }

    pub fn to_line(&self) -> Line<'static> {
        let (dot, label) = match self.state {
            State::Disabled => (q::faint(), q::faint()),
            State::Focused => (q::accent(), q::selected().bg(q::c(q::BG))),
            State::Normal => (if self.selected { q::accent() } else { q::faint() }, q::text()),
        };
        Line::from(vec![
            Span::styled(if self.selected { "(●)" } else { "( )" }, dot),
            Span::raw(" "),
            Span::styled(self.label.to_string(), label),
        ])
    }
}

impl Widget for Radio<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) { self.to_line().render(area, buf) }
}

// ───────────────────────────── Badge ─────────────────────────────

/// A small count, e.g. unread items: ` 3 `. 0 renders nothing; 100+ shows `99+`.
pub fn badge(count: usize) -> Span<'static> {
    if count == 0 {
        return Span::raw("");
    }
    let text = if count > 99 { "99+".to_string() } else { count.to_string() };
    Span::styled(format!(" {text} "), Style::new().fg(q::c(q::QUAG_HI)).bg(q::c(q::RAISED)).add_modifier(Modifier::BOLD))
}

// ───────────────────────────── Skeleton ─────────────────────────────

/// Loading placeholder: a faint bar where text will appear. `fill` = share of the width (0–1).
/// Keeps the layout stable while loading (STRUCTURE.md §7.4: never a blank zone).
pub struct Skeleton(pub f64);

impl Widget for Skeleton {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let w = ((area.width as f64) * self.0.clamp(0.0, 1.0)).round() as usize;
        Line::styled(G::LINE_FULL.repeat(w), Style::new().fg(q::c(q::BORDER))).render(Rect { height: 1, ..area }, buf);
    }
}

// ───────────────────────────── Link ─────────────────────────────

/// Clickable / openable text: underlined Quag blue.
pub fn link(text: &str) -> Span<'static> {
    Span::styled(text.to_string(), q::accent().add_modifier(Modifier::UNDERLINED))
}

/// Text in a status color (e.g. a danger menu item, a failure title).
pub fn status_label(text: &str, kind: Status) -> Span<'static> {
    Span::styled(text.to_string(), kind.style())
}

// ───────────────────────────── NumberField ─────────────────────────────

/// A number with its unit: `◂ 16.6 ms ▸`. ←/→ (or drag on desktop) changes it by `step`.
/// Digits are right-aligned so values don't jump while changing.
#[derive(Clone, Debug)]
pub struct NumberField<'a> {
    pub value: f64,
    pub unit: &'a str,
    pub decimals: usize,
    pub min: f64,
    pub max: f64,
    pub state: State,
    pub invalid: bool,
}

impl<'a> NumberField<'a> {
    pub fn new(value: f64, unit: &'a str) -> Self {
        Self { value, unit, decimals: 0, min: f64::MIN, max: f64::MAX, state: State::Normal, invalid: false }
    }
    pub fn decimals(mut self, d: usize) -> Self { self.decimals = d; self }
    pub fn range(mut self, min: f64, max: f64) -> Self { self.min = min; self.max = max; self }
    pub fn focused(mut self, on: bool) -> Self { if on { self.state = State::Focused } self }
    pub fn disabled(mut self, on: bool) -> Self { if on { self.state = State::Disabled } self }
    pub fn invalid(mut self, on: bool) -> Self { self.invalid = on; self }
    /// The value stepped by `delta`, clamped to the range (the app calls this on ←/→).
    pub fn stepped(&self, delta: f64) -> f64 { (self.value + delta).clamp(self.min, self.max) }
    pub fn text(&self) -> String {
        let n = format!("{:.*}", self.decimals, self.value);
        if self.unit.is_empty() { n } else { format!("{n} {}", self.unit) }
    }
}

impl Widget for NumberField<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area = Rect { height: 1, ..area };
        let focused = self.state == State::Focused;
        let bg = q::c(if focused { q::RAISED } else { q::SURFACE });
        buf.set_style(area, Style::new().bg(bg));
        let arrows = if focused { q::accent() } else { q::faint() };
        let at_min = self.value <= self.min;
        let at_max = self.value >= self.max;
        let text_s = if self.state == State::Disabled { q::faint() } else { q::text() };
        let t = self.text();
        let w = area.width as usize;
        let inner = w.saturating_sub(L::NUMBER_ARROWS as usize);
        Line::from(vec![
            Span::styled(" ◂ ", if at_min { q::faint() } else { arrows }),
            Span::styled(format!("{t:>inner$}"), text_s),
            Span::styled(" ▸ ", if at_max { q::faint() } else { arrows }),
        ])
        .render(area, buf);
        let bar = if self.invalid { q::c(q::ERR) } else if focused { q::c(q::QUAG) } else { bg };
        buf[(area.x, area.y)].set_symbol("▎").set_fg(bar);
    }
}

// ───────────────────────────── Slider ─────────────────────────────

/// `━━━━━━●──────` A value between min and max. Put the number next to it (NumberField or a label).
#[derive(Clone, Copy, Debug)]
pub struct Slider {
    pub value: f64,
    pub min: f64,
    pub max: f64,
    pub state: State,
}

impl Slider {
    pub fn new(value: f64, min: f64, max: f64) -> Self { Self { value, min, max, state: State::Normal } }
    pub fn focused(mut self, on: bool) -> Self { if on { self.state = State::Focused } self }
    pub fn disabled(mut self, on: bool) -> Self { if on { self.state = State::Disabled } self }
}

impl Widget for Slider {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let w = area.width as usize;
        if w < 2 { return; }
        let f = ((self.value - self.min) / (self.max - self.min).max(f64::EPSILON)).clamp(0.0, 1.0);
        let pos = (f * (w - 1) as f64).round() as usize;
        let (fill, knob) = match self.state {
            State::Disabled => (q::faint(), q::faint()),
            State::Focused => (q::accent(), Style::new().fg(q::c(q::QUAG_HI)).add_modifier(Modifier::BOLD)),
            State::Normal => (q::accent(), q::accent()),
        };
        Line::from(vec![
            Span::styled(G::LINE_FULL.repeat(pos), fill),
            Span::styled(if self.state == State::Focused { "◆" } else { "●" }, knob),
            Span::styled("─".repeat(w - pos - 1), Style::new().fg(q::c(q::BORDER))),
        ])
        .render(Rect { height: 1, ..area }, buf);
    }
}

// ───────────────────────────── Select ─────────────────────────────

/// A closed dropdown: ` Every 5 min        ▾`. When open, the app shows a `Menu` under it.
#[derive(Clone, Debug, Default)]
pub struct Select<'a> {
    pub value: &'a str,
    pub placeholder: &'a str,
    pub state: State,
    pub open: bool,
}

impl<'a> Select<'a> {
    pub fn new(value: &'a str) -> Self { Self { value, ..Default::default() } }
    pub fn focused(mut self, on: bool) -> Self { if on { self.state = State::Focused } self }
    pub fn disabled(mut self, on: bool) -> Self { if on { self.state = State::Disabled } self }
    pub fn open(mut self, on: bool) -> Self { self.open = on; self }
}

impl Widget for Select<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area = Rect { height: 1, ..area };
        let focused = self.state == State::Focused || self.open;
        let bg = q::c(if focused { q::RAISED } else { q::SURFACE });
        buf.set_style(area, Style::new().bg(bg));
        let (t, s) = if self.value.is_empty() { (self.placeholder, q::faint()) } else { (self.value, if self.state == State::Disabled { q::faint() } else { q::text() }) };
        Line::from(vec![Span::raw(" "), Span::styled(t.to_string(), s)]).render(area, buf);
        if area.width > L::FIELD_ICON {
            buf.set_string(area.right() - L::FIELD_ICON, area.y, if self.open { "▴" } else { "▾" }, if focused { q::accent() } else { q::muted() });
        }
        buf[(area.x, area.y)].set_symbol("▎").set_fg(if focused { q::c(q::QUAG) } else { bg });
    }
}

// ───────────────────────────── Charts ─────────────────────────────

const EIGHTHS: [&str; 9] = G::EIGHTHS_UP;

/// One-row trend: `▂▃▅▇▆▃▂`. The last `width` values are shown.
/// `budget`: values above it are drawn in the error color (frame time over 16.6 ms, CPU over 80 %…).
#[derive(Clone, Copy, Debug)]
pub struct Sparkline<'a> {
    pub values: &'a [f64],
    pub max: Option<f64>,
    pub budget: Option<f64>,
}

impl<'a> Sparkline<'a> {
    pub fn new(values: &'a [f64]) -> Self { Self { values, max: None, budget: None } }
    pub fn max(mut self, m: f64) -> Self { self.max = Some(m); self }
    pub fn budget(mut self, b: f64) -> Self { self.budget = Some(b); self }
}

impl Widget for Sparkline<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Histogram { values: self.values, max: self.max, budget: self.budget }.render(Rect { height: 1, ..area }, buf);
    }
}

/// Vertical bars, several rows tall (eighth-block precision). Use for distributions and frame graphs.
/// With a `budget`, a faint `┄` line marks it and bars above it turn red.
#[derive(Clone, Copy, Debug)]
pub struct Histogram<'a> {
    pub values: &'a [f64],
    pub max: Option<f64>,
    pub budget: Option<f64>,
}

impl Widget for Histogram<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width == 0 || area.height == 0 { return; }
        let vals = &self.values[self.values.len().saturating_sub(area.width as usize)..];
        let max = self.max.unwrap_or_else(|| vals.iter().cloned().fold(0.0, f64::max)).max(f64::EPSILON);
        let h = area.height as f64;
        if let Some(b) = self.budget.filter(|b| *b < max) {
            let row = area.bottom() - 1 - ((b / max * h).floor() as u16).min(area.height - 1);
            for x in area.x..area.right() {
                buf[(x, row)].set_symbol("┄").set_style(Style::new().fg(q::c(q::BORDER)));
            }
        }
        for (i, v) in vals.iter().enumerate() {
            let x = area.x + i as u16;
            let over = self.budget.is_some_and(|b| *v > b);
            let style = if over { Status::Err.style() } else { q::accent() };
            let eighths = ((v / max).clamp(0.0, 1.0) * h * 8.0).round() as usize;
            for r in 0..area.height as usize {
                let e = eighths.saturating_sub(r * 8).min(8);
                if e == 0 { break; }
                buf[(x, area.bottom() - 1 - r as u16)].set_symbol(EIGHTHS[e]).set_style(style);
            }
        }
    }
}

/// One horizontal bar (for bar charts): `██████▌` with eighth precision, on a faint track.
#[derive(Clone, Copy, Debug)]
pub struct Bar {
    pub fraction: f64,
    /// Over budget → error color.
    pub over: bool,
}

impl Widget for Bar {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let w = area.width as usize;
        let e = (self.fraction.clamp(0.0, 1.0) * w as f64 * G::EIGHTHS as f64).round() as usize;
        let style = if self.over { Status::Err.style() } else { q::accent() };
        bar_line(e, w, style).render(Rect { height: 1, ..area }, buf);
    }
}

fn bar_line(e: usize, w: usize, style: Style) -> Line<'static> {
    const H: [&str; 9] = G::EIGHTHS_RIGHT;
    const N: usize = G::EIGHTHS;
    {
        let mut s = "█".repeat(e / N);
        s.push_str(H[e % N]);
        let used = e / N + usize::from(!e.is_multiple_of(N));
        Line::from(vec![Span::styled(s, style), Span::styled("·".repeat(w.saturating_sub(used)), Style::new().fg(q::c(q::BORDER)))])
    }
}

/// Usage of a limit (memory, disk, CPU quota): accent below 80 %, yellow from 80 %, red at or over the limit.
#[derive(Clone, Copy, Debug)]
pub struct UsageBar(pub f64);

impl UsageBar {
    pub const WARN_AT: f64 = R::USAGE_WARN;
    pub fn status(self) -> Option<Status> {
        if self.0 >= R::USAGE_ERR { Some(Status::Err) } else if self.0 >= Self::WARN_AT { Some(Status::Warn) } else { None }
    }
}

impl Widget for UsageBar {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Thin line (like Slider), half-cell precision: calmer than solid blocks for many meters on one screen.
        let w = area.width as usize;
        let halves = (self.0.clamp(0.0, 1.0) * w as f64 * 2.0).round() as usize;
        let style = self.status().map_or(q::accent(), Status::style);
        let mut s = G::LINE_FULL.repeat(halves / 2);
        if halves % 2 == 1 { s.push(G::LINE_HALF); }
        let used = halves / 2 + halves % 2;
        Line::from(vec![Span::styled(s, style), Span::styled(G::LINE_TRACK.repeat(w.saturating_sub(used)), Style::new().fg(q::c(q::BORDER)))])
            .render(Rect { height: 1, ..area }, buf);
    }
}

/// Change between two measures: `▲ +12.0%`. Green when it went the good way, red when not.
/// `higher_is_better`: true for FPS / score, false for frame time / memory / errors.
pub fn delta_badge(change: f64, higher_is_better: bool) -> Span<'static> {
    if change.abs() < 0.0005 {
        return Span::styled("= 0%", q::muted());
    }
    let good = (change > 0.0) == higher_is_better;
    let arrow = if change > 0.0 { "▲" } else { "▼" };
    let style = if good { Status::Ok.style() } else { Status::Err.style() };
    Span::styled(format!("{arrow} {:+.1}%", change * 100.0), style)
}

// ───────────────────────────── SplitHandle ─────────────────────────────

/// The line between two resizable panes. Focused = Quag (then ←/→ or [ ] resize).
pub struct SplitHandle {
    pub vertical: bool,
    pub focused: bool,
}

impl Widget for SplitHandle {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let s = Style::new().fg(q::c(if self.focused { q::QUAG } else { q::BORDER }));
        let sym = if self.vertical { "│" } else { "─" };
        for y in area.y..area.bottom() {
            for x in area.x..area.right() {
                buf[(x, y)].set_symbol(sym).set_style(s);
            }
        }
        if self.focused {
            let (x, y) = (area.x + area.width / 2, area.y + area.height / 2);
            buf[(x, y)].set_symbol(if self.vertical { "┃" } else { "━" });
        }
    }
}

/// A search match inside text (Gill, bold): the highlight of filters and searches.
pub fn mark(text: &str) -> Span<'static> {
    Span::styled(text.to_string(), q::special().add_modifier(Modifier::BOLD | Modifier::UNDERLINED))
}

// ───────────────────────────── Severity ─────────────────────────────

/// Five-level severity (security findings, lint, audits). DESIGN.md "Severity":
/// 5 levels on 4 status colors, told apart by fill and weight too, never by color alone.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

impl Severity {
    pub const ALL: [Severity; 5] = [Severity::Critical, Severity::High, Severity::Medium, Severity::Low, Severity::Info];
    pub fn label(self) -> &'static str {
        match self { Severity::Critical => "CRIT", Severity::High => "HIGH", Severity::Medium => "MED", Severity::Low => "LOW", Severity::Info => "INFO" }
    }
}

/// `▐CRIT▌` filled red · `HIGH` bold red · `MED` yellow · `LOW` blue · `INFO` grey. Always 6 cells wide.
pub fn severity_badge(s: Severity) -> Span<'static> {
    let style = match s {
        Severity::Critical => Style::new().fg(q::c(q::BG)).bg(q::c(q::ERR)).add_modifier(Modifier::BOLD),
        Severity::High => Status::Err.style().add_modifier(Modifier::BOLD),
        Severity::Medium => Status::Warn.style(),
        Severity::Low => Style::new().fg(q::c(q::BODY)),
        Severity::Info => q::muted(),
    };
    Span::styled(format!(" {:<4} ", s.label()), style)
}

// ───────────────────────────── SecretField ─────────────────────────────

/// A secret (API key, password): masked by default `••••••••••••3f9a`, revealed on demand.
/// Only the last 4 characters show while masked, so you can tell keys apart.
#[derive(Clone, Debug, Default)]
pub struct SecretField<'a> {
    pub value: &'a str,
    pub revealed: bool,
    pub state: State,
}

impl<'a> SecretField<'a> {
    pub fn new(value: &'a str) -> Self { Self { value, ..Default::default() } }
    pub fn revealed(mut self, on: bool) -> Self { self.revealed = on; self }
    pub fn focused(mut self, on: bool) -> Self { if on { self.state = State::Focused } self }

    pub fn masked(&self) -> String {
        let n = self.value.chars().count();
        let tail: String = self.value.chars().skip(n.saturating_sub(F::SECRET_TAIL)).collect();
        format!("{}{tail}", G::MASK.repeat(n.saturating_sub(F::SECRET_TAIL).min(F::SECRET_MASK_MAX)))
    }
}

impl Widget for SecretField<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area = Rect { height: 1, ..area };
        let focused = self.state == State::Focused;
        let bg = q::c(if focused { q::RAISED } else { q::SURFACE });
        buf.set_style(area, Style::new().bg(bg));
        let text = if self.revealed { self.value.to_string() } else { self.masked() };
        // The show/hide label keeps its place; the value gives way (keeps its end: the part that tells keys apart)
        let room = area.width.saturating_sub(if area.width > L::SECRET_TAG_COLS { L::SECRET_TAG_COLS } else { 1 }) as usize;
        let n = text.chars().count();
        let text: String = if n > room { format!("…{}", text.chars().skip(n + 1 - room.max(1)).collect::<String>()) } else { text };
        Line::from(vec![Span::raw(" "), Span::styled(text, if self.revealed { Status::Warn.style() } else { q::text() })]).render(area, buf);
        let tag = if self.revealed { "hide" } else { "show" };
        if area.width > L::SECRET_TAG_COLS {
            buf.set_string(area.right() - L::SECRET_TAG_RIGHT, area.y, tag, (if focused { q::accent() } else { q::faint() }).bg(bg));
        }
        buf[(area.x, area.y)].set_symbol("▎").set_fg(if self.revealed { q::c(q::WARN) } else if focused { q::c(q::QUAG) } else { bg });
    }
}

// ───────────────────────────── Servers: RunState, Env, short_id, diff ─────────────────────────────

/// State of a container / service / job. Stopped is NOT an error (grey); only crashes are red.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RunState {
    Running,
    Starting,
    Stopping,
    Restarting,
    Stopped,
    Paused,
    /// Exited with this code (0 = finished fine, shown like Stopped).
    Exited(i32),
    /// Running, but restarted this many times recently: more than `rules::FLAPPING_RESTARTS`
    /// within `rules::FLAPPING_WINDOW_SECS` (restart loop). The app decides; this only shows it.
    Flapping(u32),
}

impl RunState {
    pub fn label(self) -> String {
        match self {
            RunState::Running => "running".into(),
            RunState::Starting => "starting".into(),
            RunState::Stopping => "stopping".into(),
            RunState::Restarting => "restarting".into(),
            RunState::Stopped => "stopped".into(),
            RunState::Paused => "paused".into(),
            RunState::Exited(0) => "exited 0".into(),
            RunState::Exited(c) => format!("crashed {c}"),
            RunState::Flapping(n) => format!("restarted {n}×"),
        }
    }
    pub fn is_busy(self) -> bool { matches!(self, RunState::Starting | RunState::Stopping | RunState::Restarting) }
    pub fn is_up(self) -> bool { matches!(self, RunState::Running | RunState::Flapping(_)) }
    /// Ok / Warn / Err, or None for neutral (stopped, paused, exited 0).
    pub fn status(self) -> Option<Status> {
        match self {
            RunState::Running => Some(Status::Ok),
            RunState::Starting | RunState::Stopping | RunState::Restarting | RunState::Flapping(_) => Some(Status::Warn),
            RunState::Exited(c) if c != 0 => Some(Status::Err),
            _ => None,
        }
    }
}

/// `● running` · `⠋ starting` · `○ stopped` · `‖ paused` · `✗ crashed 137` · `! restarted 5×`.
/// Busy states animate with `tick`. Icon + word, never color alone.
pub fn run_state(s: RunState, tick: usize) -> Vec<Span<'static>> {
    let icon = match s {
        RunState::Running => "●",
        _ if s.is_busy() => G::SPINNER[tick % G::SPINNER.len()],
        RunState::Paused => "‖",
        RunState::Exited(c) if c != 0 => "✗",
        RunState::Flapping(_) => "!",
        _ => "○",
    };
    let style = s.status().map_or(q::muted(), Status::style);
    vec![Span::styled(icon, style), Span::raw(" "), Span::styled(s.label(), style)]
}

/// Deployment environment, shown in the header of every page so you always know where you are.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Env {
    Prod,
    Staging,
    Dev,
}

/// `[prod]` bold red · `[staging]` yellow · `[dev]` grey.
pub fn env_tag(e: Env) -> Span<'static> {
    match e {
        Env::Prod => Span::styled("[prod]", Status::Err.style().add_modifier(Modifier::BOLD)),
        Env::Staging => Span::styled("[staging]", Status::Warn.style()),
        Env::Dev => Span::styled("[dev]", q::muted()),
    }
}

/// An id or digest shortened to 12 characters (`sha256:` dropped), faint so it doesn't compete with names.
pub fn short_id(id: &str) -> Span<'static> {
    let id = id.strip_prefix("sha256:").unwrap_or(id);
    Span::styled(id.chars().take(F::SHORT_ID_LEN).collect::<String>(), q::faint())
}

/// One line of a diff.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DiffKind {
    Same,
    Add,
    Del,
    /// `@@ services.api @@`: where the next lines are.
    Hunk,
}

/// `+ added` (green +, normal text) · `- removed` (red −, muted text) · `  same` (faint) · `@@ hunk` (Gill).
pub fn diff_line(kind: DiffKind, text: &str) -> Line<'static> {
    match kind {
        DiffKind::Add => Line::from(vec![Span::styled("+ ", Status::Ok.style()), Span::styled(text.to_string(), q::text())]),
        DiffKind::Del => Line::from(vec![Span::styled("− ", Status::Err.style()), Span::styled(text.to_string(), q::muted().add_modifier(Modifier::CROSSED_OUT))]),
        DiffKind::Same => Line::from(vec![Span::raw("  "), Span::styled(text.to_string(), q::faint())]),
        DiffKind::Hunk => Line::from(Span::styled(text.to_string(), q::special())),
    }
}

/// State of one step in a pipeline.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StepState {
    Waiting,
    Running,
    Done,
    Failed,
    Skipped,
}

/// `✓` done · spinner running · `○` waiting · `✗` failed · `–` skipped.
pub fn step_icon(s: StepState, tick: usize) -> Span<'static> {
    match s {
        StepState::Done => Span::styled("✓", Status::Ok.style()),
        StepState::Running => Span::styled(G::SPINNER[tick % G::SPINNER.len()], q::accent()),
        StepState::Waiting => Span::styled("○", q::faint()),
        StepState::Failed => Span::styled("✗", Status::Err.style()),
        StepState::Skipped => Span::styled("–", q::faint()),
    }
}

// ───────────────────────────── Game design: Curve, Clip, playhead ─────────────────────────────

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

#[cfg(test)]
mod tests {
    use super::*;

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
}
