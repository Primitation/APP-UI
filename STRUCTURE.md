# Structure: the atomic system

`DESIGN.md` covers how things **look**. This file covers how UIs are **built**: from small parts up.
It applies to every app (terminal, desktop and Unreal).

```
 TOKENS        colors, text roles, spacing        the matter          (DESIGN.md)
   ↓
 1 ATOM        Button, Checkbox, Input, Tag        one control
   ↓
 2 MOLECULE    ButtonGroup, Field, SearchBar       atoms arranged in a box
   ↓
 3 ELEMENT     TaskActions, TaskList, Inspector    a molecule put to use
   ↓
 4 UI          Tasks page, Settings page           elements arranged on a page
```

**See every part live in the explorers**: the parts list on the left, the preview on the right, and each UI page openable on its own:
- Terminal: `cd quag-theme && cargo run --example explorer` (add `--features mascot` for the private build)
- Desktop: `cd quag-theme/solid && npm run dev`
- One-picture overview: `preview/atomic-sheet.png` (`cargo run --example atomic`)

---

## 1. The four rules

1. **Only build upwards.** Each level uses only the levels **below** it. An atom never contains a molecule,
   and a page never draws a button directly.
2. **Only atoms set colors.** Molecules arrange, elements wire, pages place. If a molecule, element or
   page writes a color or a style, it's wrong. Make (or fix) an atom instead.
3. **Meaning appears at level 3.** Atoms and molecules are generic (`Button`, `ButtonGroup`). Elements are
   named after their job in the app (`TaskActions`). If an atom's name mentions the app, it's in the wrong level.
4. **Reuse before you make.** Check the inventory (§6) first. Make a new part only when nothing fits, and put it
   at the lowest level that makes sense.

---

## 2. ATOM: one control

> **An atom is one control or one visual, on its own.** Examples: a button, a checkbox.

| It has | It doesn't have |
|---|---|
| Its look (from tokens) | Other atoms inside it |
| Its states: normal, focused, disabled (+ hover/pressed on desktop) | Data, or knowledge of the app |
| Its props: label, checked, value, kind | Its own position. The level above places it |

- **Name:** what it *is*: `Button`, `Checkbox`, `Input`, `Tag`, `Kbd`, `StatusIcon`, `Spinner`, `Progress`, `Divider`.
- **Variants** are props, not new atoms: `Button` + `kind: Primary | Secondary | Danger | Ghost`, not `PrimaryButton`.
- **Containers count as atoms.** The plain *box* (`panel`, `popup`, `card`) is an atom: a box with nothing in it.
- **Every atom shows all its states** in the explorers. A state that isn't designed doesn't exist.

## 3. MOLECULE: atoms arranged in a box

> **A molecule is a few atoms arranged together, usually in a box.** Example: an array of buttons in a box.

| It does | It doesn't |
|---|---|
| Arranges atoms (one row, one column, or label-over-input) | Know what the buttons *do* |
| Sets the spacing **between** its atoms (the "inside a group" gap) | Fetch or choose data |
| Takes labels, values and callbacks as props | Set colors (its atoms do) |
| Handles tight space (compact, wrap, truncate with `…`) | Silently drop a part that doesn't fit |

- **Name:** the arrangement: `ButtonGroup`, `Field`, `SearchBar`, `Tabs`, `KeyHints`, `StatusLine`, `ListRow`, `CardHeader`.
- **One job of layout.** If it needs two directions (a row *and* a column), it's probably two molecules.
- **Spacing inside:** 2 cells / 8px between atoms, 0 between a label and its input.

## 4. ELEMENT: the molecule put to use

> **An element is a molecule (or a few) given a job in the app.** Example: the ButtonGroup used as the
> actions for the selected task.

```
TaskActions (element)
 └─ ButtonGroup (molecule)
     ├─ Button "Mark done"  primary   ← chosen by TaskActions: task.done ? "Reopen" : "Mark done"
     ├─ Button "Edit"                 ← click → open the editor
     └─ Button "Delete"     danger    ← click → ask ConfirmDialog
```

| It does | It doesn't |
|---|---|
| Reads app data and decides **which** atoms/labels appear | Know where it sits on the page |
| Wires actions (what each button does) | Set colors or spacing between atoms |
| Owns its **states**: ideal, empty, loading, error, overflow (§7.4) | Lay out other elements around it |

- **Name:** its job, in app words: `TaskActions`, `TaskList`, `TaskInspector`, `ProjectNav`, `AssetIssues`.
- **Elements may contain smaller elements** (`TaskInspector` contains `TaskActions`), but an element never
  knows its own position or size. The level above gives it a box.
- **Shared elements** that every app has are in the library: `AppHeader`, `StatusBar`, `EmptyState`, `ConfirmDialog`.
  App-specific ones live in the app.

## 5. UI: the page

> **The UI is a page: several elements arranged together.**

| It does | It doesn't |
|---|---|
| Picks which elements appear | Draw atoms or molecules directly |
| Places them in zones (§7) | Set colors or styles |
| Decides focus order and keyboard navigation | Contain data logic (elements do) |
| Adapts to window size (breakpoints) | |

- **Name:** the screen: `TasksPage`, `SettingsPage`, `AuditorTab`.
- A page should read like a table of contents. If a page file is long, parts of it belong in an element.

---

## 6. Inventory (what exists today)

| Level | Terminal: `quag-theme/src` (Rust) | Desktop: `quag-theme/solid/src` (Solid + `quag.css`) | Unreal: `FQuagStyle` |
|---|---|---|---|
| **Atoms** | `atoms::{Button, Checkbox, Radio, Input, NumberField, Slider, Select, label(Tone), status_label, tag, kbd, badge, link, mark, status_icon, spinner, Skeleton, Progress, SmoothProgress, StepProgress, Sparkline, Histogram, Bar, delta_badge, Severity + severity_badge, SecretField, Curve, Clip, playhead, node_frame, port, wire, RunState + run_state, Env + env_tag, short_id, UsageBar, diff_line, step_icon, SplitHandle, Divider, panel, popup, popup_size}` | `Button`, `Checkbox` (+ toggle), `Radio`, `Input`, `Textarea`, `NumberField`, `Slider`, `Select`, `Card`, `Text` (tones + ok/warn/err), `Tag`, `Kbd`, `Badge`, `Link`, `Mark`, `Status`, `Progress`, `SmoothProgress`, `StepProgress`, `Spinner`, `Skeleton`, `Sparkline`, `Histogram`, `Bar`, `DeltaBadge`, `SeverityBadge`, `SecretField`, `Curve`, `Clip`, `Playhead`, `NodeFrame`, `NodePort`, `Wire`, `RunStateLabel`, `EnvTag`, `ShortId`, `UsageBar`, `DiffLine`, `StepIcon`, `Divider`, `Label`, `Help` | `MakeButton`, `MakeTag`, `MakeStatus`, `MakeHeading`, `MakeSeverityBadge`, `MakeDeltaBadge`, `MakeUsageBar`, `Quag.Text.*`, `Quag.Card` |
| **Molecules** | `molecules::{ButtonGroup, ActionButton, Field, SearchBar, Tabs, KeyHints, StatusLine, ListRow, Stat, CheckList, LogLine, RadioGroup, Breadcrumbs, Menu, Table, PropertyList, TreeView (+ flatten), ParamRow, BarChart, Segmented, Meter, PortMap, StepList, KeyValueRows, freshness, CurveEditor (+ nudge_point), WeightsEditor (+ shares), Timeline, NodeGraph (+ route)}` (+ `Table::cell_areas` for atom cells) | `ButtonGroup`, `Field`, `SearchBar`, `Tabs`, `NavItem`, `ListRow`, `List`, `RadioGroup`, `Breadcrumbs`, `Menu`, `Table`, `PropertyList`, `TreeView` (+ flattenTree), `ParamRow`, `BarChart`, `Segmented`, `Meter`, `PortMap`, `StepList`, `KeyValueRows`, `Freshness`, `CurveEditor`, `WeightsEditor`, `Timeline`, `NodeGraph` (+ `nudgePoint`, `curveNudge`, `shares`, `rulerStep`, `portPoint`, `wirePath` in `game.ts`) (Table cells may be atoms) | `MakeButtonGroup`, `MakeTitle`, `MakeCard`, `MakeStat`, `MakeMeter`, `MakePropertyList`, `MakeWeightsEditor` (curves / timelines / graphs: the engine's editors, §9) |
| **Elements (shared)** | `elements::{AppHeader, StatusBar, EmptyState, ErrorState, ConfirmDialog, Toast, CommandPalette, HelpOverlay, LogViewer, TypeToConfirm, DiffView, RunProgress}` (AppHeader has `env`) | `AppHeader`, `StatusBar`, `EmptyState`, `ErrorState`, `ConfirmDialog`, `Toast`, `CommandPalette`, `HelpOverlay`, `LogViewer`, `TypeToConfirm`, `DiffView`, `RunProgress` (AppHeader has `env`) | `MakeEmptyState`, `MakeErrorState`, `MakeLoading` |
| **UI** | `ui::{shell, Zones, SizeClass, columns, form_column, focus_column, centered, workspace, split, …}` | `AppShell`, `PageHead`, `Layout` (form, focus, dashboard, stream, workspace), `Split` | your tab spawner |
| **Keymap** (not a level: data the levels read) | `keymap::{Binding, Keymap}` | `Keymap`, `useKeymap`, `createPageKeys` (adds Ctrl K · ? · Esc and renders palette + help) | |
| **Constants** (not a level: numbers the levels read) | `consts::{layout, rules, format, glyph, input}` (+ `fmt` uses them) | `constants.ts` (as `consts`) | `QuagConsts` |

Tool-domain examples in the explorers (app elements built from the parts above): `FrameBudget`, `GpuMemory` (optimisation),
`MaterialParams` (tech art), `ServerLoad` + `LogViewer` (servers), `ProjectTree` (any hierarchy),
`FindingsList` + `FindingDetail` + `FirewallRules` on `AuditPage` (server security),
and the **dock** demo app (server / Docker admin): `ContainerTable`, `ContainerInspector`, `StackTree`, `HostCard`,
`NeedsAttention`, `Release`, `ImageTable`, `DiskUse` on `ContainersPage`, `HostsPage`, `DeployPage`, `ImagesPage`,
and the **forge** demo app (game design data): `AssetList`, `CurvePanel`, `KeyInspector`, `LootPanel`, `PerHundred`,
`BalanceTable`, `SequencePanel`, `ClipInspector`, `GraphPanel`, `NodeInspector` on `CurvesPage` (Workspace),
`LootPage` (List → Detail), `BalancePage` (Dashboard, with loading / can't-read states), `SequencePage` (Timeline),
`GraphPage` (Canvas).
Number formatting for all of them: `quag::fmt` in Rust / `fmt` in Solid (`bytes`, `bytesOf`, `age`, `uptime`).

Demo pages in both explorers, one per layout: `TasksPage` (List → Detail, + command palette, help, toast), `DashboardPage`,
`WorkspacePage`, `SettingsPage` (Form), `LogPage` (Stream), `WelcomePage` (Focus).

When you add a part, add it to this table **and to both explorers' catalogs** (`examples/explorer/catalog.rs`, `solid/explorer/catalog.tsx`).

### Folder layout in an app

```
src/quag/         the Quag parts this app copied from APP-UI (`quag add`), listed in quag.json. The app owns them.
src/ui/
  elements/      task_actions.rs  task_list.rs  task_inspector.rs  project_nav.rs
  pages/         tasks.rs  settings.rs
(atoms + molecules are copied from APP-UI with `node <APP-UI>/tools/quag.mjs add . <Part>`, never imported from it.
A new GENERIC atom/molecule goes INTO APP-UI first (both stacks), then gets copied; app-specific parts stay in the app.)
```
- Desktop (Solid): the same, with `src/quag/` copied from `quag-theme/solid/src` (+ `quag.css`), imported from `./quag`.
- One part = one file in APP-UI (`src/atoms/button.rs`, `solid/src/atoms/Button.tsx`), so `add` can copy just it. A part
  reaches other parts by name (Rust) or by a relative import of the exact file (Solid, never the `index.ts`), so the tool
  can follow what it needs. `node tools/quag.mjs check` proves that every part builds on its own.
- Unreal: `Private/Widgets/Elements/S<Name>.cpp`, `Private/Widgets/S<Tool>Tab.cpp`.
- Example: `quag-theme/examples/explorer/` (`data.rs`, `elements.rs`, `pages.rs`) and `quag-theme/solid/explorer/pond/`.

---

## 7. Page composition (the UI level)

### 7.1 Zones
Every page uses the same five zones; only **Header** and **Content** are required.

```
┌──────────────────────────────────────────────────────────┐
│ HEADER     ◆ app · location                tabs · search │  AppHeader
├──────────┬────────────────────────────┬──────────────────┤
│ NAV      │ CONTENT                    │ INSPECTOR        │
│ places   │ the thing being worked on  │ the selection    │
├──────────┴────────────────────────────┴──────────────────┤
│ STATUS     key hints                            ✓ Saved  │  StatusBar
└──────────────────────────────────────────────────────────┘
```

### 7.2 Fixed places
| Thing | Always goes |
|---|---|
| App name | Header, far left |
| Tabs (2–5 views of the same data) | Header, right |
| Search | Desktop: header right. Terminal: `/` line at top of the content |
| Page-level primary action (New, Scan) | Top-right of the content |
| Actions on the selection (`TaskActions`) | Bottom of the inspector |
| Destructive action | Last in its group, and it always opens `ConfirmDialog` |
| Validation error | Directly under its field (`Field` does this) |
| Result of an action | StatusBar right (or a toast on desktop) |
| Key hints | StatusBar left (terminal) |
| Empty content | `EmptyState` centered in the content zone (+ mascot if private) |
| A zone that failed | `ErrorState` in that zone only (what + why + Retry) |
| Every action by name | `CommandPalette` on `ctrl+k`, top-center |
| All keys of the page | `HelpOverlay` on `?` |
| Where you are, deep in a tree | `Breadcrumbs`, first line of the content |
| Actions on one item (right-click / `.`) | `Menu`, next to the item |

Button order inside a group: **terminal = primary first**; **desktop & Unreal = primary last (rightmost)**.

### 7.2b Keys: one keymap per page
Every page declares its keys **once**, as a `Keymap`. The StatusBar hints (`keymap.hints()`), the `HelpOverlay` (`?`)
and the `CommandPalette` (`ctrl+k`, `keymap.commands()`) all read it, and the key handler dispatches through it
(`keymap.find(key)` in Rust, `useKeymap` in Solid). Never write a hint, a help line or a command by hand: if a key
isn't in the keymap, it doesn't exist.

**Standard keys (both stacks):**

| Key | Means |
|---|---|
| ↑ ↓ | move in the focused list |
| ← → | change the value / fold a tree (← on a file: its folder) / move the playhead |
| `[ ]` | previous / next sub-item (key, clip) · workspace: resize panes |
| PgUp PgDn | previous / next item of the nav (curve, table, node, project) |
| Tab | terminal: next zone or field · desktop: never bound, the browser moves focus |
| Enter / Esc | open or confirm / close or back. Esc closes what's open first, the next Esc goes back |
| Del | delete, always asks first (danger dialogs focus "Keep"; ← → move, Enter presses) |
| Space · Ctrl S · Ctrl K · ? | play / toggle · save · commands (type to filter, ↑ ↓, Enter runs) · help |

A part that uses a key (tree, tabs, menu, split, an overlay's Esc) marks it handled, so the page never acts on it twice.

**Buttons in the terminal** are drawings: a button has no behavior of its own. The element that draws a row of buttons
writes it **once** as a table of `molecules::ActionButton` (label, keymap action id, look):
`ActionButton::new("Delete", "delete").danger()`. The row is drawn from the table (`ActionButton::group`) and pressed
from the same table (`keymap::ButtonRow`): when the zone with the buttons has focus (Tab), ← → move between them and
Enter runs the focused button's action through the same code as its key. A label can't land on another button's
action, and a button and its key can't do different things. Labels that change with state (Mark done / Reopen)
are two tables, picked by the same function for drawing and pressing. Pages with a
row of buttons add `keymap::BUTTONS` (or `BUTTON` for a single one) to their keymap, so the help lists it.
On the desktop, buttons are real `<button>`s: Tab reaches them and Enter / Space press them.
Toasts never take a key. Regression tests: `cargo test --examples` (Rust explorer), `test/pages.test.tsx` (Solid).

### 7.3 Page layouts (pick one per page)
| Layout | Zones used | For |
|---|---|---|
| **List → Detail** (default) | nav · content(list) · inspector | browsing/editing collections |
| **Dashboard** | content = KPI row + ≤ 6 cards | overviews |
| **Workspace** | tools strip · canvas · inspector · output | editors, tools |
| **Form** | content = one column ≤ 72 cols / 640px | settings, create/edit |
| **Focus** | content = centered column ≤ 50 cols / 480px | empty, splash, wizard, too-small |
| **Stream** | content = full-width log | logs, feeds, build output |
| **Canvas** | content = a pannable canvas · inspector | node graphs, maps, boards |
| **Timeline** | content = ruler + tracks · inspector | sequences, animations, schedules |

### 7.4 States: every element has five
| State | Shows |
|---|---|
| Ideal | the content |
| Empty | `EmptyState`: message + one hint (+ mascot if private) |
| Loading | the same layout with `Skeleton` bars (or a `spinner` + "Loading…"). Never a blank zone |
| Error | `ErrorState`: `✗` + what failed + why + "Retry"; the rest of the page stays usable |
| Overflow | truncate with `…`, wrap or compact button groups, scroll long lists |

### 7.5 Breakpoints (done by `ui::shell` / the CSS)
| Terminal | Desktop | Result |
|---|---|---|
| ≥ 120 cols | ≥ 1280px | nav · content · inspector |
| 80–119 | 960–1279px | nav · content, inspector under (terminal) / drawer (desktop) |
| 60–79 | < 960px | content only; nav and inspector open on demand |
| < 60×20 | < 800×560 | "Window too small" (Focus layout) |

Screenshots: `preview/pond-wide-140.png`, `pond-medium-100.png`, `pond-narrow-70.png`.

---

## 8. Checklist

**Starting an app:** copy `templates/tui-app` (Rust) or `templates/desktop-app` (Solid). They already have the shell,
one page with its keymap, one element with its empty state, and a consts file (see `templates/README.md`).

**Making a part**
- [ ] Is it at the lowest level that makes sense? (Could it be an atom? A molecule?)
- [ ] Is its name right for its level (generic below 3, app words at 3)?
- [ ] Does it only use levels below it?
- [ ] Does only an atom set colors?
- [ ] Atoms: are all states designed? Molecules: does it handle tight space without dropping parts?
- [ ] Elements: does it have its five states?
- [ ] Is it added to the inventory (§6) and to both explorers?

**Making a page**
- [ ] Is it built only from elements?
- [ ] Are all its keys in one `Keymap` (hints, help and palette read from it)?
- [ ] No magic numbers: is every size, threshold, format or timing a named constant
      (`quag::consts` / `quag/constants.ts` for the system, the app's own consts file for the app)?
- [ ] Is it one layout (§7.3), with everything in its fixed place (§7.2)?
- [ ] Squint test: is there exactly one bright thing, and is it the right one?
- [ ] Does it look right at all three widths?
- [ ] Does keyboard focus follow reading order, and does Esc go back?
