# APP-UI · Quag

The reference for every UI I build: one look, one way to build it, for terminal, desktop and Unreal.

![pond, the demo app, welcoming you with Quagsire](preview/tui-page-welcome.png)

- **[DESIGN.md](DESIGN.md)**: the look. "Quag": calm, modern, dark only, the Quagsire palette.
- **[STRUCTURE.md](STRUCTURE.md)**: the build. Atom → molecule → element → UI page; reuse library parts first,
  only atoms set colors, pages only place elements, one keymap per page.

| Folder | What |
|---|---|
| `quag-theme/` | Rust / ratatui library (`quag-theme`), its explorer (`cargo run --example explorer`) and tests |
| `quag-theme/solid/` | Solid desktop library + explorer (`npm run dev`), using `quag-theme/web/quag.css` |
| `quag-theme/unreal/QuagStyle/` | Unreal Engine editor plugin (Slate): `FQuagStyle` helpers |
| `templates/` | Starter apps: `tui-app` (Rust) and `desktop-app` (Solid) |

Checks: `cargo test --all-targets` (in `quag-theme/`), `npm test` (in `quag-theme/solid/`).

## Use it with Claude Code

Paste this into `~/.claude/CLAUDE.md` so every app follows Quag. It assumes the repo is cloned at
`/home/primitation/APP-UI`; if it's somewhere else, replace that path everywhere.

```markdown
# UI design (all my apps)
Before building or changing ANY user interface, read and follow `/home/primitation/APP-UI/DESIGN.md` (the look: "Quag", calm & modern, dark only, Quagsire palette) and `/home/primitation/APP-UI/STRUCTURE.md` (the build: atomic system atom → molecule → element → UI; reuse library parts first, only atoms set colors, pages only place elements, numbers in a consts file).
APP-UI is the reference for all my UIs; it's the private GitHub repo `Primitation/APP-UI` (commit / push only when I ask).
- New app: start from a template in `/home/primitation/APP-UI/templates/` (`tui-app` Rust, `desktop-app` Solid), see its README.
- Keys: one keymap per page (hints, `?` help and Ctrl K palette read it); use the standard keys of STRUCTURE.md §7.2b, the same in every stack.
- Rust/TUI apps: depend on `quag-theme = { path = "/home/primitation/APP-UI/quag-theme" }` and use its helpers; never hand-write colors.
- Desktop apps (Windows/Linux, Tauri/web view): use the Solid parts in `/home/primitation/APP-UI/quag-theme/solid` (`quag-solid`) with `/home/primitation/APP-UI/quag-theme/web/quag.css` (`.q-*` classes, `var(--q-*)` tokens), per DESIGN.md §8.
- Unreal Engine editor plugins (Slate): depend on the `QuagStyle` plugin (`/home/primitation/APP-UI/quag-theme/unreal/QuagStyle`) and use `FQuagStyle` helpers; blend with the editor per DESIGN.md §9 (editor backgrounds, Quag accents only).
- Other stacks: use the exact hex tokens and roles from DESIGN.md.
- Quagsire GIF mascot: ONLY in private/personal apps (`features = ["mascot"]`), placed per DESIGN.md §6. Ask if unsure whether an app is private.
- Changing APP-UI itself: keep both stacks in step, then run `cargo test --all-targets` (in `quag-theme/`) and `npm test` (in `quag-theme/solid/`).
```
