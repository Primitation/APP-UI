# New-app templates

Two starting points that already follow DESIGN.md and STRUCTURE.md: the shell, one page with its keymap
(hints, `?` help, Ctrl K palette), one element with its empty state, and a consts file.

| Template | Stack | Uses |
|---|---|---|
| `tui-app/` | Rust · ratatui | `quag-theme` (path dependency) |
| `desktop-app/` | Solid · Vite (put Tauri around it for a desktop window) | `quag-solid` source + `web/quag.css` (alias) |

## Start an app

```sh
cp -r /home/primitation/APP-UI/templates/tui-app ~/code/my-tool        # or desktop-app
cd ~/code/my-tool
# rename: Cargo.toml `name` / package.json `name`, and the `app` shown in AppHeader
cargo run                     # tui-app
npm install && npm run dev    # desktop-app
```

The paths to APP-UI are absolute, so the copy works anywhere on this machine.

- **Private / personal app?** Turn the mascot on: `features = ["mascot"]` (Rust) or `EmptyState mascot={…}` (Solid).
  Public apps never show it (DESIGN.md §6).
- **Desktop window:** `npm create tauri-app` style setup, or `cargo tauri init` in the copy, pointing at `npm run dev` / `dist/`.
- **Adding UI:** reuse library parts first; new app parts go in `elements`; pages only place elements;
  numbers go in `consts`. The checklist is at the end of STRUCTURE.md.
