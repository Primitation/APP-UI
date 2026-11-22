//! MOLECULE: ParamRow.

use super::*;
use crate::atoms::{self, Tone};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    text::Line,
    widgets::Widget,
};
use crate::consts::layout as L;

/// A numeric parameter: `Roughness   ━━━━●─────  ◂  0.45 ▸`. Label · Slider · NumberField.
/// The tech-art / tuning row. Narrow: the slider goes first, the number always stays.
pub struct ParamRow<'a> {
    pub label: &'a str,
    pub label_width: u16,
    pub field: atoms::NumberField<'a>,
}

impl Widget for ParamRow<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area = Rect { height: 1, ..area };
        let num_w = (self.field.text().chars().count() as u16 + L::PARAM_NUMBER_PAD).max(L::PARAM_NUMBER_MIN);
        let show_slider = area.width >= self.label_width + num_w + L::PARAM_SLIDER_MIN;
        let [l, s, n] = Layout::horizontal([
            Constraint::Length(self.label_width),
            Constraint::Min(if show_slider { L::PARAM_SLIDER_MIN_COLS } else { 0 }),
            Constraint::Length(num_w),
        ])
        .spacing(GAP)
        .areas(area);
        let focused = self.field.state == atoms::State::Focused;
        Line::from(atoms::label(&truncate(self.label, l.width as usize), if focused { Tone::Accent } else { Tone::Muted })).render(l, buf);
        if show_slider {
            atoms::Slider { value: self.field.value, min: self.field.min, max: self.field.max, state: self.field.state }.render(s, buf);
        }
        self.field.render(n, buf);
    }
}
