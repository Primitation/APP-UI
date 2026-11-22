# House Style: "Quag"

The single UI style for **every** app I build. It's calm and modern, dark only,
and the colors come from the Quagsire sprite.
If an app looks different from this, the app is wrong, not the guide.

- Terminal apps (Rust / ratatui): `quag-theme/` crate. See §7.
- Desktop apps for Windows / Linux (Tauri, any web view): `quag-theme/web/quag.css`. See §8.
- Unreal Engine editor plugins (Slate): `quag-theme/unreal/QuagStyle/` plugin. See §9.
- **How UIs are built** (atom → molecule → element → UI): `STRUCTURE.md`. This file is the look; that one is the build.
- Previews: `preview/*.png`. Live parts explorers: `cd quag-theme && cargo run --example explorer` (terminal) and `cd quag-theme/solid && npm run dev` (desktop)

---

## 1. Style in one sentence

> **A dark pond with one light-blue Quagsire in it.** The background is quiet and the
> text is easy to read. The only bright thing on screen is what matters right now.

Principles, in priority order:

1. **Readable first.** All text meets WCAG AA (≥ 4.5:1) on its background. *Muted* text does too.
2. **One accent.** The Quag blue marks one thing per area: focus, the primary action, or a link.
   When everything is highlighted, nothing stands out.
3. **Calm.** No pure black and no pure white. Avoid heavy borders, blinking, and rainbow colors.
4. **Color means something.** Green, yellow and red are used for status only. Every status also has an icon, so color is never the only signal.
5. **Space over lines.** Separate things with padding first, then with a faint border. Use a bright border only on the focused panel.

---

## 2. Palette

### Water: backgrounds (60% of the screen)
| Token | Hex | Use for |
|---|---|---|
| `BG` | `#0E1620` | App background. Everything starts here. |
| `SURFACE` | `#152230` | Secondary areas: sidebars, status bar, unfocused selection, gauge track |
| `RAISED` | `#1C2D3F` | Things on top: popups, menus, focused selection row |
| `BORDER` | `#2A3F55` | Unfocused panel borders, dividers, secondary button bg |

### Ink: text (30%)
| Token | Hex | Contrast on BG | Use for |
|---|---|---|---|
| `TEXT` | `#E4F3FB` | 16.0 | Body text, values, anything you must read |
| `MUTED` | `#93ADC2` | 7.8 | Descriptions, timestamps, hints, inactive tabs, unfocused titles |
| `FAINT` | `#5F778D` | 3.9 | Placeholders, disabled items, separators (`·`). **Never** for important text |

### Quagsire: brand (10%, used sparingly)
| Token | Hex | Contrast | Role | Use for |
|---|---|---|---|---|
| `QUAG` | `#84D6F7` | 11.2 | **Primary accent** | Focused border, panel titles, active tab, primary button bg, links, progress fill, cursor |
| `QUAG_HI` | `#ADE7FF` | 13.6 | Accent, pressed/selected | Text of the selected row, hover/pressed state |
| `BODY` | `#639CBD` | 6.1 | Secondary | Section headings, table headers, "info" status |
| `GILL` | `#A897D6` | 7.0 | Tertiary / special | Keyboard keys in hints, tags, badges, search matches |
| `GILL_DEEP` | `#6B5A8C` | 3.0 | Decoration only | Logo or art details, never text |
| `NAVY` | `#425284` | 2.4 | Decoration only | Logo or art outlines, never text |

### Status (only for real states)
| Token | Hex | Icon | Use for |
|---|---|---|---|
| `OK` | `#7DD3A8` | ✓ | Success, saved, connected, passing |
| `WARN` | `#F2C46D` | ! | Needs attention, but nothing has failed |
| `ERR` | `#F27D7F` | ✗ | Failed, destructive, invalid input |
| Info | `BODY` | i | Neutral notices |

### Chart series (in this order)
`QUAG` → `GILL` → `OK` → `WARN` → `BODY`. If a chart needs more than 5 series, change the chart.

---

### Numbers, units and charts
- **Digits line up**: tabular figures, numbers right-aligned in tables and fields.
- **≤ 3 significant digits** after the point, trailing zeros dropped: `16.6 ms`, `812 MB`, `0.45`, `1.2 GB` (`fmt_num` / `fmtNum`).
- **A space before the unit**, always the same unit within a column.
- **Budgets**: a value over its budget turns **error red**; the budget is a faint dashed line. Nothing else in a chart is red.
- **Deltas** (`delta_badge` / `DeltaBadge`): green when the change went the *good* way for that measure (frame time ▼, FPS ▲), red otherwise, grey when 0.
- **Charts are Quag blue** by default; several series use the chart series order above. Never color a single series by value except for budgets.

### Severity (findings, lint, audits)

Five levels on the four status colors, so they must differ by more than color:

| Level | Look | Use for |
|---|---|---|
| **CRIT** | filled red badge, dark text | exploitable now: leaked key, open admin port |
| **HIGH** | bold red (outlined on desktop) | fix today |
| **MED** | yellow | fix this week |
| **LOW** | Body blue | when convenient |
| **INFO** | muted grey | nothing to do, just know |

All badges are the same width so titles line up; lists sort worst first. Secrets (`SecretField`) are masked
except the last 4 characters; when revealed the field turns yellow (it's exposed). Irreversible actions on
something important use `TypeToConfirm`: type the name, the danger button stays disabled until it matches.

### Servers and containers

| State | Look | Note |
|---|---|---|
| running | green `●` | |
| starting / stopping / restarting | yellow spinner | settles by itself; never block the screen while it does |
| stopped · paused · exited 0 | grey `○` `‖` | **not an error**: someone or something stopped it on purpose |
| crashed (exit ≠ 0) | red `✗` + the code (`crashed 137`) | |
| restart loop (> 3 restarts / 10 min) | yellow `!` `restarted 5×` | warns even while "running" |

- **Environment always visible:** `EnvTag` in the header of every page: **prod** red, **staging** yellow, **dev** grey.
  The one place red is used without an error, on purpose: you must not forget you are on prod.
- **Confirmation scales with the damage:**
  - start / stop / restart: no confirmation (easy to undo); a toast says what happened.
  - remove a container, roll back: `ConfirmDialog`.
  - remove a volume, prune, deploy to prod, rotate a secret: `TypeToConfirm` (type the name).
- **Long actions are a zone, not a modal** (`RunProgress`): steps, live output, Cancel; then Retry / Roll back on failure.
  Before applying a change, show it (`DiffView`).
- **Live data shows its age** (`Freshness`): "updated 3s ago"; yellow "stale 45s" past 30 s, so frozen numbers are never trusted.
- **Units:**
  - Memory and disk in binary units (`fmt::bytes`: MiB, GiB), as `used / limit` in the limit's unit.
  - CPU and memory are a percentage of the container's **limit**, not the host.
  - `Meter` turns yellow from 80 % and red at the limit, with an icon (not color alone).
  - Ids and digests: 12 characters, monospace, faint, copyable (`ShortId`).
- **Ports:** `8080 → 80/tcp`. A port published on all interfaces gets a `!`.
- **Secrets in env vars** are masked (`KeyValueRows` → `SecretField`).

### Game design data (curves, drop tables, balance, sequences)

- **Curves:** the line is Quag; keys are dots; the **selected key is Gill** (special, like the playhead), never red/green.
  Always show the ranges (y labels left: max / mid / min, x labels under) and a readout of the selected key with units.
  The first and last keys hold the range ends: they move only up / down and can't be deleted.
- **Editing by keys:** arrows move the selected thing by 5 % of its range, Shift + arrows by 1 % (`NUDGE`, `NUDGE_FINE`).
  `[` `]` pick the previous / next key or clip. These go in the page's keymap like every other key.
- **Weights** are stored raw and shown three ways: raw (editable), **% of the total** (always sums to 100 %), and odds
  `1 in N`. A table whose weights are all 0 says "nothing can drop" in yellow.
- **Balance against the last version:** every changed number gets a `DeltaBadge` that knows which way is good
  (more HP ▲ green, more cost ▲ red). Outliers are **yellow "strong" / blue "weak"**: they need a look, they are not errors.
- **Timelines:** ruler on top with round steps (1, 2, 5 × 10ⁿ), one row per track, clips on raised blocks
  (selected = Quag fill), the playhead **Gill** over everything. Play / pause is Space; ← → scrub.
- **Node graphs** (abilities, dialogue, shaders, AI): nodes are raised cards with the title on top; inputs on the
  left edge, outputs on the right, so wires run left → right. Wires are faint and drawn **under** the nodes; the selected
  node and **its** wires turn Quag, so you can follow what it touches. A wired port is filled, a free one hollow.
  Arrange chains left → right and start each event on a new row, so wires don't cross nodes.
  Keys: Tab next node, arrows move it (Shift: faster); the mouse drags titles and pans the background.
- **Project data has three states, shown the same everywhere:** loading (skeleton rows), can't read (`ErrorState`:
  which file, the parser's reason, Retry), ready. Totals and KPIs show `–` until the data is there: never stale numbers.

## 3. Where and when to use each color

| Situation | Do this |
|---|---|
| Screen background | `BG` |
| A panel that has focus | `QUAG` rounded border with a **bold** `QUAG` title |
| A panel without focus | `BORDER` border with a `MUTED` title |
| Selected list row (focused list) | `QUAG_HI` bold text on `RAISED` with a `▌` marker |
| Selected list row (unfocused list) | `TEXT` on `SURFACE` |
| Primary action (Save, Run, Confirm) | `BG` text on a `QUAG` background. **Max one per screen** |
| Other actions | `TEXT` on `BORDER` (visible on every background, including popups) |
| Destructive action (Delete) | `ERR` bold text on `BORDER`, plus a confirm popup. Never a red background by default |
| Popup / modal | `RAISED` bg with a `QUAG` border and 2-cell padding. Clear the area under it |
| Title of the app / screen | `QUAG` bold, with `◆` before the app name |
| Section heading | `BODY` bold |
| Secondary info | `MUTED` |
| Disabled | `FAINT` (do not also dim with a modifier) |
| Key hints (bottom bar) | key in bold `GILL`, label in `MUTED`, separated by `·` in `FAINT` |
| Tabs | active: bold, underlined `QUAG`. inactive: `MUTED` |
| Link / clickable text | underlined `QUAG` |
| Search match / tag / badge | `GILL` |
| Progress / gauge | `QUAG` fill on a `SURFACE` track. Put the numbers next to it, not on top |
| Progress that moves by small amounts (download, copy) | `SmoothProgress`: fills in eighths of a cell (`█████▋`); on the desktop the fill glides |
| Progress by known steps (deploy, install, wizard) | `StepProgress`: one segment per step, the current one filling, a failed step `ERR`. Say the step next to it: `Step 3 of 5 · Migrate` |
| A button with keyboard focus | `▸` in its left padding, bold and underlined (same width). Enter presses it |
| Status messages | status color + icon + short text: `✓ Saved`, `✗ Sync failed` |
| Loading | `MUTED` text plus a spinner in `QUAG`: `⠋ Loading…`. One turn = 0.8 s (10 frames of one 80 ms tick, `consts::motion`), from a clock, never from redraws |
| Empty state | centered `MUTED` sentence plus one `QUAG` action hint. Private apps also show the mascot (§6) |

The **60 / 30 / 10 rule**: about 60% water (backgrounds), 30% ink (text), and
at most 10% Quagsire and status colors combined. If a screenshot looks
mostly blue, there are too many accents.

---

## 4. Layout and type (terminal)

- **Structure:** a 1–2 line header (app name and tabs), the body (panels), and a 1-line key-hint bar at the bottom. Every app uses this layout.
- **Borders:** always `Rounded`. Never `Double` or `Thick`.
- **Padding:** panels get 1 cell left and right. Popups get 2 cells left and right and 1 line top and bottom.
- **Gaps:** panels are placed edge-to-edge (their borders are the gap). Leave 1 blank line between sections inside a panel.
- **Bold:** use it for titles, headings, the selected row, the primary button, and keys in hints. Nothing else.
- **Italic:** don't use it, because many terminal fonts render it badly.
- **Underline:** use it for links and the active tab only.
- **Reverse video:** never use it. Use `selected()` instead.
- **Icons:** keep to a small set: `◆ ✓ ✗ ! i ▌ · → ⠋`. Avoid emoji in the UI chrome.
- **Truecolor:** this is the default. On terminals without truecolor, the crate picks the nearest
  256 colors automatically (`QUAG_FORCE_256=1` forces this for testing).

---

## 5. Do / Don't

| ✅ Do | ❌ Don't |
|---|---|
| Build from the library's atoms and molecules | Write `Color::Rgb(..)` or `Color::Blue` in app code |
| Highlight one focused thing | Use a bright border on every panel |
| Pair status color with an icon | Use red/green as the only signal |
| Use `MUTED` for supporting text | Use `FAINT` for text the user needs |
| Give dark text on a `QUAG` button | Put white text on `QUAG` (it fails contrast) |
| Keep backgrounds `BG` / `SURFACE` / `RAISED` | Invent new grays or use terminal default bg |

---

## 6. The Quagsire mascot: private apps only

`quag-theme/assets/quagsire.gif` (200×191, 41 frames, from Tenor, sticker by *muffinmonkey*).

**When to show it.** Only in **private / personal tools** (apps just for me, never published or shared).
Turn it on with the cargo feature: `quag-theme = { path = "...", features = ["mascot"] }`.
Public apps never enable the feature, so the GIF is not even compiled in. The GIF belongs to its
artist, so public apps can't ship it.

**Where to show it** (pick one per screen, never more):
1. **Empty states**, e.g. "Nothing here yet". The mascot goes above the message. *(main spot)*
2. **Splash / welcome screen** on start, for at most 1–2 s, or until the first keypress.
3. **About / help screen** (`?`).
4. **Idle / done**, e.g. after a long task finishes successfully, next to `✓ Done`.

**Rules:**
- Size: about 12–16 rows tall. It must never be bigger than the content it accompanies.
- Never put it over working content, and never let it block input. It is decoration.
- Pause it (`set_paused(true)`) when it is not the focus of the screen.
- No mascot in error states. Quagsire is happy and the user isn't.
- Rendering: kitty/sixel/iTerm2 graphics when available, otherwise colored half-blocks. Call
  `mascot::picker()` **before** `ratatui::init()`.

---

## 7. Using it in a new app (terminal)

```toml
# Cargo.toml
[dependencies]
quag-theme = { path = "/home/primitation/APP-UI/quag-theme" }               # public app
# quag-theme = { path = "/home/primitation/APP-UI/quag-theme", features = ["mascot"] }  # private app
```

```rust
use quag_theme::{atoms::Button, molecules::ButtonGroup, elements::{AppHeader, StatusBar}, ui};

// ELEMENT (in your app): a molecule put to use
struct TaskActions<'a> { task: &'a Task }
impl Widget for TaskActions<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let main = if self.task.done { Button::new("Reopen") } else { Button::new("Mark done").primary() };
        ButtonGroup::new(vec![main, Button::new("Edit"), Button::new("Delete").danger()]).render(area, buf);
    }
}

// UI (in your app): the page places elements
fn tasks_page(f: &mut Frame, app: &App) {
    let s = ui::shell(f.area(), ui::Zones { nav: true, inspector: true });
    f.render_widget(AppHeader { app: "pond", location: "tasks", tabs: None }, s.header);
    // …TaskList in s.content, TaskInspector (with TaskActions) in s.inspector, StatusBar in s.footer
}
```

Full template: `quag-theme/examples/explorer/` (`data.rs`, `elements.rs`, `pages.rs`).

---

## 8. Desktop apps (Windows / Linux, Tauri + web view)

The palette, roles and mascot rules are the same as above. Only the shapes change, because the
desktop has pixels instead of character cells.

**Setup (Solid apps)**: use the components in `quag-theme/solid/src` (see its README). Import `quag.css` once
at the root; build elements and pages from `Button`, `ButtonGroup`, `Field`, `AppShell`… Never style raw HTML.

**Setup (plain CSS)**
1. Copy `quag-theme/web/quag.css` (and `quagsire.gif` + `quagsire-still.png` for **private** apps)
   into the UI's `src/`, then import it **before** any other CSS.
2. In `tauri.conf.json`, give each window a dark title bar and stop the white flash on start:
   ```json
   "app": { "windows": [{ "theme": "Dark", "backgroundColor": "#0E1620" }] }
   ```
3. Use the `.q-*` classes. In app CSS, use `var(--q-*)` and never raw hex values. The short
   aliases (`--bg --panel --fg --accent --muted --line --danger --ok --category-N`) also work, so older apps pick up the theme
   by swapping their theme file.

**Shape rules**
| Thing | Rule |
|---|---|
| Layout | Sidebar (232px, `SURFACE`) + top bar (tabs, search) + main (`BG`) + optional status bar |
| Spacing | Use the 4 / 8 / 12 / 16 / 24 / 32 px scale only (`--q-space-1…6`) |
| Corners | 4px (tags, kbd) · 6px (buttons, inputs) · 10px (cards, dialogs) |
| Text | 14px body, 20px page title, 16px dialog title, 12px uppercase `BODY` section labels |
| Font | Inter / system UI; JetBrains Mono for code, numbers in tables, `kbd` |
| Cards | `SURFACE` + 1px `BORDER`. A focused or editing card gets a `QUAG` border |
| Selected row / nav | `RAISED` bg (or 12% Quag tint for nav) + 3px `QUAG` bar on the left |
| Hover | One step lighter background (`RAISED`). Never change size or bold on hover |
| Focus (keyboard) | 2px `QUAG` outline. Inputs get a `QUAG` border plus a soft 3px glow |
| Shadows | Only on things that float (dialogs, menus, toasts). Flat everything else |
| Motion | 120ms ease-out on color/background only; progress fills glide one tick (`--q-tick`, 80 ms) and the spinner turns in `--q-spin-turn` (0.8 s), the same clock as the TUI. Respect `prefers-reduced-motion` |
| Dialog buttons | Right-aligned: secondary/ghost first, primary (or danger) last |
| Errors in forms | `ERR` border on the input + `ERR` help text under it, never a popup |
| Mascot | `<img class="q-mascot" src="quagsire.gif">` inside `.q-empty`, 160px, `image-rendering: pixelated` |

---

## 9. Unreal Engine editor plugins (Slate)

**This is the one exception to "everything is Quag".** An editor plugin lives *inside someone else's
app*, the Unreal Editor. A navy panel in the gray editor would look broken. So editor plugins
**blend in** and keep Quag as an accent:

| From the **editor** (`FAppStyle` / `FStyleColors`) | From **Quag** (`FQuagStyle`) |
|---|---|
| Panel / tab / window backgrounds | ◆ brand title of the tool (`MakeTitle`) |
| Fonts and base text size | Primary button: Quag blue with dark text (`EQuagButton::Primary`) |
| Inputs, combo boxes, checkboxes, details panels | Selection in *my own* lists (`Quag.TableRow`), focused card outline |
| Toolbar, menus, scrollbars | Section headings (Body blue), tags (Gill), muted text |
| Global selection color (never override it) | Status: ✓ ✗ ! i with Ok/Warn/Err/Info colors |
| | Mascot in empty states, **private plugins only** |

Rules:
- Layout of a tool tab: a header row (title left, max one Primary button right), the body in a `MakeCard`
  (12px padding), and an optional status line at the bottom. Use 12px outer padding and a 4/8/12 spacing scale.
- Use the editor's radius (4px) for buttons and 6px for cards.
- Never restyle editor widgets globally (no `FAppStyle` overrides). Only style what the plugin creates.
- **Standalone windows / in-game (runtime) UI** made with Slate/UMG are *my own app*, so they use the
  full Quag palette, including `Quag.Window` (`BG`) backgrounds, like §2–§3.
- Public plugins: delete `Resources/Mascot/`. Fab plugins: copy the style into the plugin's own module (see the plugin README).
- **Data parts** come from `FQuagStyle` like in the other stacks: `MakeSeverityBadge`, `MakeDeltaBadge`, `MakeUsageBar`,
  `MakeMeter`, `MakeStat`, `MakePropertyList`, `MakeWeightsEditor`, `MakeErrorState`, `MakeLoading` (the editor's own
  throbber). Thresholds and steps come from `QuagConsts` (= Rust `consts`, checked by `tests/system.rs`).
- **Curves, timelines and node graphs: use the engine's editors**, never a hand-made copy: curve assets
  (`UCurveFloat` + the curve editor), **Sequencer**, and the graph editor (`SGraphEditor` / an `UEdGraph` schema).
  People already know them, and they come with undo, copy/paste and zoom. Quag only adds the accents those editors
  let you set: the curve color (`QuagColors::Quag()`), and pin / wire colors of your own graph schema
  (`QuagColors::Body()` for data, `QuagColors::Quag()` for the selected path). Never restyle them globally.
