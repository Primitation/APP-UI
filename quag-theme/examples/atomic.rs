#![allow(dead_code)] // shares pond's modules; not every part is used here
//! The atomic sheet: one picture of the four levels, using pond's real parts.
//!   cargo run --example atomic                  (interactive view, q to quit)
//!   cargo run --example atomic -- --dump <dir>  (headless, for screenshots)

#[path = "explorer/consts.rs"]
mod consts;
#[path = "explorer/data.rs"]
mod data;
#[path = "explorer/elements.rs"]
mod elements;

use crossterm::event::{self, Event, KeyCode};
use data::TASKS;
use elements::{TaskActions, TaskInspector};
use quag_theme::{
    self as q, Status,
    atoms::{self, Button, Checkbox, Input, Progress, Tone},
    elements::{AppHeader, StatusBar},
    molecules::{ButtonGroup, Field, KeyHints, ListRow, SearchBar, StatusLine, Tabs},
};
use ratatui::{
    Frame, Terminal,
    backend::TestBackend,
    layout::{Constraint, Layout, Rect},
    style::Modifier,
    text::{Line, Span},
    widgets::{Block, Paragraph},
};

fn band_label(f: &mut Frame, area: Rect, n: &str, name: &str, what: &[&str]) {
    let mut lines = vec![Line::from(vec![
        atoms::label(n, Tone::Faint),
        Span::raw(" "),
        atoms::label(name, Tone::Title),
    ])];
    lines.extend(what.iter().map(|w| Line::from(atoms::label(w, Tone::Muted))));
    f.render_widget(Paragraph::new(lines), area);
}

fn caption(f: &mut Frame, area: Rect, text: &str) {
    f.render_widget(Paragraph::new(Line::from(atoms::label(text, Tone::Faint))), area);
}

fn arrow(f: &mut Frame, area: Rect) {
    f.render_widget(Paragraph::new(Line::styled("        ↓ used by", q::faint())), area);
}

fn sheet(f: &mut Frame) {
    f.render_widget(Block::new().style(q::base()), f.area());
    let area = Rect { x: 2, y: 1, width: f.area().width - 4, height: f.area().height - 2 };
    let [h, _, a, a2, b, b2, c, c2, d] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(6),
        Constraint::Length(1),
        Constraint::Length(6),
        Constraint::Length(1),
        Constraint::Length(13),
        Constraint::Length(1),
        Constraint::Length(10),
    ])
    .areas(area);
    f.render_widget(
        Paragraph::new(Line::from(vec![
            atoms::label("◆ Quag atomic system", Tone::Title),
            atoms::label("   atom → molecule → element → UI", Tone::Muted),
        ])),
        h,
    );
    let split = |r: Rect| -> [Rect; 2] { Layout::horizontal([Constraint::Length(26), Constraint::Min(0)]).areas(r) };

    // ── 1 ATOM
    let [l, r] = split(a);
    band_label(f, l, "1", "ATOM", &["one control or visual", "generic: Button, not", "SaveButton. has states"]);
    let rows = Layout::vertical([Constraint::Length(1), Constraint::Length(1), Constraint::Length(1), Constraint::Length(1), Constraint::Length(1)]).split(r);
    let mut x = rows[0].x;
    for (b, cap) in [
        (Button::new("Mark done").primary(), ""),
        (Button::new("Edit"), ""),
        (Button::new("Delete").danger(), ""),
        (Button::new("Cancel").ghost(), ""),
        (Button::new("Edit").focused(true), ""),
        (Button::new("Edit").disabled(true), ""),
    ] {
        let w = b.width();
        f.render_widget(b, Rect::new(x, rows[0].y, w, 1));
        x += w + 2;
        let _ = cap;
    }
    caption(f, rows[1], "Button: primary · secondary · danger · ghost · focused · disabled");
    let [c1, c2_, c3, c4] = Layout::horizontal([Constraint::Length(16), Constraint::Length(16), Constraint::Length(26), Constraint::Min(0)]).areas(rows[2]);
    f.render_widget(Checkbox::new("Notify", true), c1);
    f.render_widget(Checkbox::new("Archive", false), c2_);
    f.render_widget(Input::new("").placeholder("Input…").focused(true), Rect { width: 24, ..c3 });
    f.render_widget(
        Paragraph::new(Line::from(vec![
            atoms::tag("docs"), Span::raw("  "), atoms::kbd("/"), Span::raw("  "),
            atoms::status_icon(Status::Ok), Span::raw(" "), atoms::status_icon(Status::Warn), Span::raw(" "),
            atoms::status_icon(Status::Err), Span::raw(" "), atoms::status_icon(Status::Info), Span::raw("  "),
            atoms::spinner(3),
        ])),
        c4,
    );
    caption(f, rows[3], "Checkbox · Checkbox · Input · tag · kbd · status icons · spinner");
    f.render_widget(Progress(0.6), Rect { width: 30, ..rows[4] });
    arrow(f, a2);

    // ── 2 MOLECULE
    let [l, r] = split(b);
    band_label(f, l, "2", "MOLECULE", &["atoms arranged in a box", "still generic: takes", "labels & values as props"]);
    let [m1, _, m2] = Layout::horizontal([Constraint::Length(40), Constraint::Length(4), Constraint::Min(0)]).areas(r);
    let [g, gc, _] = Layout::vertical([Constraint::Length(3), Constraint::Length(1), Constraint::Min(0)]).areas(m1);
    f.render_widget(
        ButtonGroup::new(vec![Button::new("Button A").primary(), Button::new("Button B"), Button::new("Button C")]).boxed(""),
        g,
    );
    caption(f, gc, "ButtonGroup = [Button] × n in a box");
    let [fl, fc, sb, lr] = Layout::vertical([Constraint::Length(3), Constraint::Length(1), Constraint::Length(1), Constraint::Length(1)]).areas(m2);
    f.render_widget(Field::new("Label", Input::new("value")).help("help text"), Rect { width: 34, ..fl });
    caption(f, fc, "Field = label + Input + help");
    f.render_widget(SearchBar { query: "", focused: false }, Rect { width: 34, ..sb });
    f.render_widget(
        &ListRow { title: "ListRow = check + text + tag + status", check: Some(false), tags: &["tag"], status: Some(Status::Warn), meta: "meta", ..Default::default() },
        lr,
    );
    arrow(f, b2);

    // ── 3 ELEMENT
    let [l, r] = split(c);
    band_label(f, l, "3", "ELEMENT", &["a molecule put to use:", "wired to data + actions,", "owns its states.", "named for its job"]);
    let [e1, _, e2] = Layout::horizontal([Constraint::Length(46), Constraint::Length(4), Constraint::Min(0)]).areas(r);
    let [t1, a1, _, t2, a2_, _, ec] = Layout::vertical([
        Constraint::Length(1), Constraint::Length(1), Constraint::Length(1),
        Constraint::Length(1), Constraint::Length(1), Constraint::Length(1), Constraint::Min(0),
    ])
    .areas(e1);
    let open = &TASKS[1];
    let done = &TASKS[3];
    f.render_widget(Paragraph::new(Line::from(vec![atoms::label("TaskActions", Tone::Heading), atoms::label(" for an open task", Tone::Muted)])), t1);
    f.render_widget(TaskActions { task: open, focused: None }, a1);
    f.render_widget(Paragraph::new(Line::from(vec![atoms::label("TaskActions", Tone::Heading), atoms::label(" for a done task", Tone::Muted)])), t2);
    f.render_widget(TaskActions { task: done, focused: None }, a2_);
    f.render_widget(
        Paragraph::new(vec![
            Line::from(atoms::label("same ButtonGroup; the element decides", Tone::Faint)),
            Line::from(atoms::label("the buttons from the data (done → Reopen)", Tone::Faint)),
        ]),
        ec,
    );
    f.render_widget(TaskInspector { task: Some(open), focused: false, button: None }, Rect { height: c.height, ..e2 });
    arrow(f, c2);

    // ── 4 UI
    let [l, r] = split(d);
    band_label(f, l, "4", "UI", &["a page: arranges", "elements in zones.", "never draws atoms", "or sets colors"]);
    let [hh, body, ff] = Layout::vertical([Constraint::Length(1), Constraint::Min(0), Constraint::Length(1)]).areas(r);
    f.render_widget(AppHeader { app: "pond", location: "AppHeader", tabs: Some(Tabs { names: &["Today", "Done"], active: 0 }), env: None }, hh);
    let [n, ct, ins] = Layout::horizontal([Constraint::Length(18), Constraint::Min(0), Constraint::Length(30)]).areas(body);
    for (rect, name, sub, focus) in [
        (n, "ProjectNav", "nav zone", false),
        (ct, "TaskList", "content zone", true),
        (ins, "TaskInspector", "inspector · has TaskActions", false),
    ] {
        let blk = atoms::panel(name, focus);
        let inner = blk.inner(rect);
        f.render_widget(blk, rect);
        f.render_widget(Paragraph::new(Line::from(atoms::label(sub, Tone::Faint))), inner);
    }
    f.render_widget(
        StatusBar { hints: KeyHints(&[("StatusBar", "")]), status: Some(StatusLine { kind: Status::Ok, message: "status" }) },
        ff,
    );
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if let Some(i) = args.iter().position(|a| a == "--dump") {
        let dir = &args[i + 1];
        std::fs::create_dir_all(dir)?;
        let mut t = Terminal::new(TestBackend::new(132, 45))?;
        t.draw(sheet)?;
        let buf = t.backend().buffer();
        let mut out = String::new();
        for y in 0..buf.area.height {
            for x in 0..buf.area.width {
                let c = &buf[(x, y)];
                out.push_str(&format!(
                    "{x}\t{y}\t{}\t{:?}\t{:?}\t{}\t{}\n",
                    c.symbol(), c.fg, c.bg,
                    c.modifier.contains(Modifier::BOLD) as u8,
                    c.modifier.contains(Modifier::UNDERLINED) as u8
                ));
            }
        }
        std::fs::write(format!("{dir}/atomic-sheet.cells"), out)?;
        return Ok(());
    }
    let mut terminal = ratatui::init();
    loop {
        terminal.draw(sheet)?;
        if let Event::Key(k) = event::read()? && matches!(k.code, KeyCode::Char('q') | KeyCode::Esc) { break; }
    }
    ratatui::restore();
    Ok(())
}
