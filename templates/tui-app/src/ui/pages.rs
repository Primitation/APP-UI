//! PAGES: pick elements, place them in the shell's zones. No atoms, no colors here.
//! Each page declares its keys ONCE (a Keymap): hints, help and palette are read from it.

use super::elements::ItemList;
use crate::app::App;
use quag_theme::{
    self as q, Status,
    elements::{AppHeader, CommandPalette, HelpOverlay, StatusBar, Toast},
    keymap::{Binding, Keymap},
    molecules::{KeyHints, StatusLine},
    ui::{self, Zones},
};
use ratatui::{Frame, widgets::Block};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Overlay {
    None,
    Help,
    Palette,
    Toast,
}

pub const HOME_KEYS: Keymap = Keymap(&[
    Binding::new("↑ ↓", "previous / next", "Move").hint("move").action("move"),
    Binding::new("x", "mark done / reopen", "Act").hint("done").command("Mark done").action("done"),
    Binding::new("?", "this help", "App").hint("keys").command("Show keys").action("help"),
    Binding::new("ctrl+k", "commands", "App").hint("commands").action("palette"),
    Binding::new("q", "quit", "App").hint("quit").action("quit"),
]);

/// List page: items in the content zone.
pub fn home(f: &mut Frame, app: &App) {
    let area = f.area();
    f.render_widget(Block::new().style(q::base()), area);
    let s = ui::shell(area, Zones { nav: false, inspector: false });
    if s.too_small {
        ui::render_too_small(f, area);
        return;
    }
    f.render_widget(AppHeader { app: "quag-app", location: "home", tabs: None, env: None }, s.header);
    let hints = HOME_KEYS.hints();
    f.render_widget(StatusBar { hints: KeyHints(&hints), status: Some(StatusLine { kind: Status::Ok, message: "Ready" }) }, s.footer);
    f.render_widget(ItemList { items: &app.items, selected: app.selected, focused: true }, s.content);

    match app.overlay {
        Overlay::None => {}
        Overlay::Help => {
            let h = HelpOverlay { keymap: HOME_KEYS };
            let r = h.area(area);
            f.render_widget(h, r);
        }
        Overlay::Palette => {
            let items = HOME_KEYS.commands("");
            let p = CommandPalette { query: "", items: &items, selected: 0 };
            let r = p.area(area);
            f.render_widget(p, r);
        }
        Overlay::Toast => {
            let t = Toast { kind: Status::Ok, message: &app.toast };
            let r = t.area(area);
            f.render_widget(t, r);
        }
    }
}
