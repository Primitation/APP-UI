//! A new Quag terminal app. Read APP-UI/DESIGN.md and STRUCTURE.md before adding UI.
//! Levels: atoms → molecules (copied from APP-UI into src/quag/, owned by this app) → elements (src/ui/elements.rs)
//! → pages (src/ui/pages.rs). More parts: `node <APP-UI>/tools/quag.mjs add . <part>`.

mod app;
mod consts;
mod quag;
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
