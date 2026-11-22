# New-app templates

Two starting points that already follow DESIGN.md and STRUCTURE.md: the shell, one page with its keymap
(hints, `?` help, Ctrl K palette), one element with its empty state, and a consts file.

| Template | Stack | Quag parts |
|---|---|---|
| `tui-app/` | Rust · ratatui | copied into `src/quag/`, used as `crate::quag::…` |
| `desktop-app/` | Solid · Vite (put Tauri around it for a desktop window) | copied into `src/quag/`, imported from `./quag` (+ `quag/quag.css`) |

The templates here have no `src/quag/` of their own: `quag.json` lists the parts they use, and the tool copies
them in when it creates the app. The new app depends on nothing in APP-UI.

## Start an app

```sh
node <APP-UI>/tools/quag.mjs new tui ~/code/my-tool       # or: new desktop ~/code/my-app
cd ~/code/my-tool
cargo run                     # tui-app
npm install && npm run dev    # desktop-app
```

`new` names the package after the folder. Still rename the `app` shown in `AppHeader`.

- **More parts:** `node <APP-UI>/tools/quag.mjs add . ButtonGroup Table` (in the app folder). `list` shows them all.
- **Private / personal app?** `new … --mascot` (or later `add . mascot`), then `default = ["mascot"]` in Cargo.toml
  (Rust) or `EmptyState mascot={…}` with `./quag/quagsire.gif` (Solid). Public apps never show it (DESIGN.md §6).
- **Desktop window:** `npm create tauri-app` style setup, or `cargo tauri init` in the copy, pointing at `npm run dev` / `dist/`.
- **Adding UI:** reuse parts first (copy them with `add`); new app parts go in `elements`; pages only place elements;
  numbers go in `consts`. The checklist is at the end of STRUCTURE.md.
- **The copy is yours:** change a part in `src/quag/` if this app needs it. A fix every app should get goes into
  APP-UI, then `add . <Part> --force` brings it back here.
