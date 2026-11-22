//! A new Quag terminal app. Read APP-UI/DESIGN.md and STRUCTURE.md before adding UI.
//! Levels: atoms → molecules (from quag_theme) → elements (src/ui/elements.rs) → pages (src/ui/pages.rs).

mod app;
mod consts;
mod ui;

use crossterm::event::{self, Event, KeyEventKind};

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let mut app = app::App::new();
    let res = (|| -> std::io::Result<()> {
        while !app.quit {
            terminal.draw(|f| ui::pages::home(f, &app))?;
            if !event::poll(consts::FRAME)? { continue; }
            if let Event::Key(k) = event::read()? && k.kind == KeyEventKind::Press {
                app.key(k);
            }
        }
        Ok(())
    })();
    ratatui::restore();
    res
}
