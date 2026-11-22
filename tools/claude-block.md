# UI design (all my apps)
Before building or changing ANY user interface, read and follow `{{APP_UI}}/DESIGN.md` (the look: "Quag", calm & modern, dark only, Quagsire palette) and `{{APP_UI}}/STRUCTURE.md` (the build: atomic system atom → molecule → element → UI; reuse library parts first, only atoms set colors, pages only place elements, numbers in a consts file).
APP-UI (cloned at `{{APP_UI}}`) is the reference for all my UIs; it's the private GitHub repo `Primitation/APP-UI` (commit / push only when I ask).
**Apps never depend on APP-UI.** They copy the parts they use into their own `src/quag/` with the `quag` tool, and own that copy:
- New app: `node {{APP_UI}}/tools/quag.mjs new tui <dir>` (Rust / ratatui) or `new desktop <dir>` (Solid). See `{{APP_UI}}/templates/README.md`.
- Need a part: `node {{APP_UI}}/tools/quag.mjs list` to see them, then `node {{APP_UI}}/tools/quag.mjs add <app-dir> <Part>...` (e.g. `ButtonGroup EmptyState`). It copies the part plus everything it needs and records it in the app's `quag.json`. Never import from or path-depend on APP-UI, never copy parts by hand.
- Rust/TUI apps use `crate::quag::{atoms, molecules, elements, ui, keymap, consts}`; Solid apps import from `./quag` (relative) and `quag/quag.css` (`.q-*` classes, `var(--q-*)` tokens, DESIGN.md §8). Never hand-write colors.
- A copied part is the app's code: fix it there when the app needs it. A fix or new generic part that every app should get goes into APP-UI first (both stacks), then `add <app-dir> <Part> --force` refreshes the copy (it otherwise keeps files the app changed and lists them).
- Keys: one keymap per page (hints, `?` help and Ctrl K palette read it); use the standard keys of STRUCTURE.md §7.2b, the same in every stack.
- Unreal Engine editor plugins (Slate): `node {{APP_UI}}/tools/quag.mjs unreal <project-dir>` copies the `QuagStyle` plugin into the project; use `FQuagStyle` helpers; blend with the editor per DESIGN.md §9 (editor backgrounds, Quag accents only).
- Other stacks: use the exact hex tokens and roles from DESIGN.md.
- Quagsire GIF mascot: ONLY in private/personal apps (`quag add <app-dir> mascot`, Rust: `--features mascot`; Unreal: `unreal … --mascot`), placed per DESIGN.md §6. Ask if unsure whether an app is private.
- Changing APP-UI itself: keep both stacks in step, then run `cargo test --all-targets` (in `quag-theme/`), `npm test` (in `quag-theme/solid/`) and `node tools/quag.mjs check` (every part copies and builds on its own).
