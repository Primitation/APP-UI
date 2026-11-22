//! KEYMAP: the keys of a page, written ONCE. Three parts read it, so they can't drift apart:
//! - `StatusBar` key hints  ← `keymap.hints()`   (the few marked `hint`)
//! - `HelpOverlay`          ← the whole keymap, by group
//! - `CommandPalette`       ← `keymap.commands()` (every binding with a command name)
//!
//! The app matches key events against the same list (`keymap.find(code)`), so what's shown is what works.

use crate::molecules::MenuItem;

#[derive(Clone, Copy, Debug, Default)]
pub struct Binding<'a> {
    /// What's displayed: "ctrl+k", "↑ ↓", "d".
    pub key: &'a str,
    /// What it does, in the help: "delete (asks first)".
    pub what: &'a str,
    /// Help section: "Move", "Act", "App".
    pub group: &'a str,
    /// Short form for the status bar; "" = not in the status bar.
    pub hint: &'a str,
    /// Name in the command palette; "" = not a command (e.g. arrow keys).
    pub command: &'a str,
    /// Destroys something (red in menus, last).
    pub danger: bool,
    /// The app's action id (match on it after `find`).
    pub action: &'a str,
}

impl<'a> Binding<'a> {
    pub const fn new(key: &'a str, what: &'a str, group: &'a str) -> Self {
        Self { key, what, group, hint: "", command: "", danger: false, action: "" }
    }
    pub const fn hint(mut self, h: &'a str) -> Self { self.hint = h; self }
    pub const fn command(mut self, c: &'a str) -> Self { self.command = c; self }
    pub const fn danger(mut self) -> Self { self.danger = true; self }
    pub const fn action(mut self, a: &'a str) -> Self { self.action = a; self }
}

#[derive(Clone, Copy, Debug)]
pub struct Keymap<'a>(pub &'a [Binding<'a>]);

impl<'a> Keymap<'a> {
    /// For `StatusBar { hints: KeyHints(&keymap.hints()) }`.
    pub fn hints(&self) -> Vec<(&'a str, &'a str)> {
        self.0.iter().filter(|b| !b.hint.is_empty()).map(|b| (b.key, b.hint)).collect()
    }
    /// For `CommandPalette { items: &keymap.commands() }`; danger ones last. Filter by `query` (case-insensitive).
    pub fn commands(&self, query: &str) -> Vec<MenuItem<'a>> {
        let q = query.to_lowercase();
        let mut v: Vec<_> = self.0.iter()
            .filter(|b| !b.command.is_empty() && b.command.to_lowercase().contains(&q))
            .map(|b| MenuItem { label: b.command, key: b.key, danger: b.danger })
            .collect();
        v.sort_by_key(|m| m.danger);
        v
    }
    /// Groups in first-seen order, for `HelpOverlay`.
    pub fn groups(&self) -> Vec<(&'a str, Vec<&'a Binding<'a>>)> {
        let mut out: Vec<(&str, Vec<&Binding>)> = Vec::new();
        for b in self.0 {
            match out.iter_mut().find(|(g, _)| *g == b.group) {
                Some((_, v)) => v.push(b),
                None => out.push((b.group, vec![b])),
            }
        }
        out
    }
    /// The binding whose displayed key is `key` (one of its space-separated alternatives).
    pub fn find(&self, key: &str) -> Option<&'a Binding<'a>> {
        self.0.iter().find(|b| b.key == key || b.key.split(' ').any(|k| k == key))
    }
}

/// The keys of a row of buttons, for the help of any page that has one.
pub const BUTTONS: Binding<'static> = Binding::new("← → enter", "in a row of buttons: pick one, press it", "Buttons");
/// The same for a single button (nothing to pick).
pub const BUTTON: Binding<'static> = Binding::new("enter", "press the focused button", "Buttons");

/// A row of buttons the keyboard can use, from the same table it's drawn from (`ActionButton`).
/// Each button's action is the action id of a binding of the page, so pressing the button (Enter)
/// and pressing its key run the same code; they can't drift apart.
/// The app keeps the focused index; ← → move it (stopping at the ends), Enter presses it.
#[derive(Clone, Copy, Debug)]
pub struct ButtonRow<'a>(pub &'a [crate::molecules::ActionButton<'a>]);
impl<'a> ButtonRow<'a> {
    /// The focused index after `key` ("←" / "→"; anything else leaves it).
    pub fn step(&self, i: usize, key: &str) -> usize {
        let last = self.0.len().saturating_sub(1);
        match key { "←" => i.min(last).saturating_sub(1), "→" => (i + 1).min(last), _ => i.min(last) }
    }
    /// The action of the focused button.
    pub fn press(&self, i: usize) -> Option<&'a str> { self.0.get(i.min(self.0.len().saturating_sub(1))).map(|b| b.action) }
}

#[cfg(test)]
mod tests {
    use super::*;
    const KM: Keymap = Keymap(&[
        Binding::new("↑ ↓", "move", "Move").hint("move"),
        Binding::new("d", "delete (asks first)", "Act").hint("delete").command("Delete task").danger().action("delete"),
        Binding::new("n", "new task", "Act").command("New task").action("new"),
        Binding::new("?", "this help", "App").hint("keys"),
    ]);

    #[test]
    fn a_button_row_moves_and_presses_actions() {
        use crate::molecules::ActionButton as B;
        let table = [B::new("Mark done", "done").primary(), B::new("Edit", "edit"), B::new("Delete", "delete").danger()];
        let row = ButtonRow(&table);
        assert_eq!(row.step(0, "←"), 0, "stops at the left end");
        assert_eq!(row.step(1, "→"), 2);
        assert_eq!(row.step(2, "→"), 2, "stops at the right end");
        assert_eq!(row.press(2), Some("delete"));
        assert_eq!(row.press(9), Some("delete"), "a stale index presses the last one, never nothing");
        assert_eq!(ButtonRow(&[]).press(0), None);
        // drawn from the same table: same labels, same order, focus where the keyboard is
        let g = B::group(&table, Some(1));
        assert_eq!(g.buttons.iter().map(|b| b.label).collect::<Vec<_>>(), vec!["Mark done", "Edit", "Delete"]);
        assert_eq!(g.buttons.iter().map(|b| b.state == crate::atoms::State::Focused).collect::<Vec<_>>(), vec![false, true, false]);
        assert_eq!(g.buttons[2].kind, crate::atoms::ButtonKind::Danger);
    }

    #[test]
    fn one_source_three_views() {
        assert_eq!(KM.hints(), vec![("↑ ↓", "move"), ("d", "delete"), ("?", "keys")]);
        let c = KM.commands("");
        assert_eq!(c.iter().map(|m| m.label).collect::<Vec<_>>(), vec!["New task", "Delete task"], "danger last");
        assert_eq!(KM.commands("new").len(), 1);
        assert_eq!(KM.groups().iter().map(|g| g.0).collect::<Vec<_>>(), vec!["Move", "Act", "App"]);
        assert_eq!(KM.find("↓").map(|b| b.what), Some("move"));
        assert_eq!(KM.find("d").map(|b| b.action), Some("delete"));
    }
}
