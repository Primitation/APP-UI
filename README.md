# APP-UI · Quag

The reference for every UI I build: one look, one way to build it, for terminal, desktop and Unreal.

![pond, the demo app, welcoming you with Quagsire](preview/tui-page-welcome.png)

- **[DESIGN.md](DESIGN.md)**: the look. "Quag": calm, modern, dark only, the Quagsire palette.
- **[STRUCTURE.md](STRUCTURE.md)**: the build. Atom → molecule → element → UI page; reuse parts first,
  only atoms set colors, pages only place elements, one keymap per page.

## Quick start (any PC)

```sh
git clone https://github.com/Primitation/APP-UI      # anywhere you like
node APP-UI/tools/quag.mjs setup                     # once: tells Claude Code where this clone is
```

That's it. From then on, ask Claude Code for a UI in any project ("make me a terminal tool that lists my
containers") and it follows Quag: it starts the app from a template and copies in the parts it needs.

Or by hand:

```sh
node APP-UI/tools/quag.mjs new tui ~/code/my-tool             # terminal app (Rust / ratatui)
node APP-UI/tools/quag.mjs new desktop ~/code/my-app          # desktop app (Solid; wrap in Tauri for a window)
node APP-UI/tools/quag.mjs add ~/code/my-tool ButtonGroup Table   # more parts later
```

**Needs:** Node 18+ (for the tool). Plus Rust (`cargo`) for terminal apps, npm for desktop apps, Unreal 5.3+ for the plugin.

## Apps copy parts, they don't depend on APP-UI

An app never points at this repo (no path dependency, no alias, no absolute import). It **copies** the parts it
uses into its own `src/quag/` and owns that copy, like a vendored folder: the app builds on any machine without
APP-UI, and a change here never breaks an app by surprise.

| Command | Does |
|---|---|
| `node tools/quag.mjs setup` | Writes the Quag rules into `~/.claude/CLAUDE.md` with this clone's path |
| `node tools/quag.mjs list [tui\|desktop]` | Every part you can copy, by level |
| `node tools/quag.mjs new <tui\|desktop> <dir> [--mascot]` | New app from a template, with the parts it uses |
| `node tools/quag.mjs add <app-dir> <part>... [--force]` | Copies parts + everything they need into `<app>/src/quag/` |
| `node tools/quag.mjs unreal <project-dir> [--mascot]` | Copies the QuagStyle plugin into `<project>/Plugins/` |
| `node tools/quag.mjs check` | Self-test: every part copies into an empty app and builds on its own |

- **Naming a part:** its id (`molecules/table`, `atoms/Button`), its file name, or a name it exports (`EmptyState`).
  `list` shows them; the explorers (below) show them live.
- **What gets copied:** always the core (colors and text roles, consts, fmt, keymap, page shell; on desktop also
  `quag.css`), then the parts you ask for and the parts *they* use. The app's `quag.json` lists them.
- **Using them:** terminal apps use `crate::quag::{atoms, molecules, elements, ui, keymap, consts}`;
  desktop apps `import … from "./quag"` and `import "./quag/quag.css"`.
- **Updating a copy:** `add` again only writes what's missing or out of date. Files the app changed are **kept**
  and listed; `--force` replaces them with APP-UI's current version. That's how an app picks up a fix made here.
- **Mascot (private apps only, DESIGN.md §6):** `new … --mascot` or `add <app> mascot`, then build Rust apps with
  `--features mascot`. Unreal: `unreal … --mascot`. Public apps never get the GIF.

## Use it with Claude Code

`setup` writes the rules between `<!-- quag:start -->` and `<!-- quag:end -->` in `~/.claude/CLAUDE.md`, leaving
the rest of the file alone. Every session then reads DESIGN.md and STRUCTURE.md before touching a UI, and uses
`quag.mjs` to start apps and copy parts (never importing from APP-UI). Running it again is safe; run it again after
moving the clone or pulling a new version of the rules. The text it writes is
[`tools/claude-block.md`](tools/claude-block.md) (`{{APP_UI}}` becomes the clone's path).

## What's in the repo

| Folder | What |
|---|---|
| `quag-theme/` | Rust / ratatui parts, one file per part (`src/atoms/`, `src/molecules/`, `src/elements/`), explorer and tests |
| `quag-theme/solid/` | Solid desktop parts, one file per part (`src/atoms/`, …), explorer and tests |
| `quag-theme/web/` | `quag.css` (the desktop look: `.q-*` classes, `var(--q-*)` tokens) and the mascot images |
| `quag-theme/unreal/QuagStyle/` | Unreal Engine editor plugin (Slate): `FQuagStyle` helpers |
| `templates/` | Starter apps: `tui-app` (Rust) and `desktop-app` (Solid); `new` copies them |
| `tools/` | `quag.mjs` (the copy tool, no dependencies) and `claude-block.md` (what `setup` writes) |

**Explorers** (every part live, plus demo apps): `cargo run --example explorer` in `quag-theme/`
(`--features mascot` for the private build), and `npm install && npm run dev` in `quag-theme/solid/`.

## Changing APP-UI

Keep both stacks in step (STRUCTURE.md §6 lists every part in each), then run:

```sh
cd quag-theme && cargo test --all-targets          # Rust parts, explorer, cross-stack color/consts checks
cd quag-theme/solid && npm test                    # Solid parts
node tools/quag.mjs check                          # every part copies and builds on its own; templates build
```

A new part is one file. In Rust it reaches other parts by name; in Solid it imports the exact file
(`../atoms/small`, never `../atoms`), so `add` can follow what it needs.
