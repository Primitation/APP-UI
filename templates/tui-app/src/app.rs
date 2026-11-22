//! App state and what each key does. Keys are looked up in the page's keymap (`ui::pages`),
//! so the status bar hints, `?` help and ctrl+k palette always match what the keys really do.

use crate::ui::pages::{Overlay, HOME_KEYS};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub struct Item {
    pub title: String,
    pub meta: String,
    pub done: bool,
}

pub struct App {
    pub items: Vec<Item>,
    pub selected: usize,
    pub overlay: Overlay,
    pub toast: String,
    pub quit: bool,
}

impl App {
    pub fn new() -> Self {
        let items = ["First thing", "Second thing", "Third thing"]
            .iter()
            .map(|t| Item { title: t.to_string(), meta: "today".into(), done: false })
            .collect();
        Self { items, selected: 0, overlay: Overlay::None, toast: String::new(), quit: false }
    }

    pub fn key(&mut self, k: KeyEvent) {
        // An open overlay takes the next key and closes.
        if self.overlay != Overlay::None {
            self.overlay = Overlay::None;
            return;
        }
        let name = key_name(&k);
        let action = HOME_KEYS.find(&name).map(|b| b.action).unwrap_or("");
        match action {
            "move" if name == "↓" => self.selected = (self.selected + 1).min(self.items.len().saturating_sub(1)),
            "move" => self.selected = self.selected.saturating_sub(1),
            "done" => {
                if let Some(i) = self.items.get_mut(self.selected) {
                    i.done = !i.done;
                    self.toast = if i.done { "Marked done".into() } else { "Reopened".into() };
                    self.overlay = Overlay::Toast;
                }
            }
            "help" => self.overlay = Overlay::Help,
            "palette" => self.overlay = Overlay::Palette,
            "quit" => self.quit = true,
            _ => {}
        }
    }
}

/// The name a `Binding` matches: "↑", "ctrl+k", "x", "?".
fn key_name(k: &KeyEvent) -> String {
    let ctrl = if k.modifiers.contains(KeyModifiers::CONTROL) { "ctrl+" } else { "" };
    let base = match k.code {
        KeyCode::Up => "↑".into(),
        KeyCode::Down => "↓".into(),
        KeyCode::Enter => "enter".into(),
        KeyCode::Esc => "esc".into(),
        KeyCode::Char(c) => c.to_string(),
        _ => String::new(),
    };
    format!("{ctrl}{base}")
}
