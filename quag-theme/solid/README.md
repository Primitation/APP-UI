# Quag for Solid

The Quag house style as **Solid** components, organised by the four levels of `../../STRUCTURE.md`.
The look lives in `../web/quag.css`; the structure lives here.

```
src/
  atoms/      Button Checkbox Radio Input Textarea NumberField Slider Select Card Text Tag Kbd Badge Link Mark Status
              Progress Spinner Skeleton Sparkline Histogram Bar DeltaBadge SeverityBadge SecretField
              RunStateLabel EnvTag ShortId UsageBar DiffLine StepIcon
              Divider Label Help
  molecules/  ButtonGroup Field SearchBar Tabs NavItem ListRow List RadioGroup Breadcrumbs Menu Table PropertyList
              TreeView ParamRow BarChart Segmented Meter PortMap StepList KeyValueRows Freshness
  elements/   AppHeader StatusBar EmptyState ErrorState ConfirmDialog Toast CommandPalette HelpOverlay LogViewer
              TypeToConfirm DiffView RunProgress
              (shared; app-specific elements live in the app)
  ui/         AppShell PageHead Layout (form · focus · dashboard · stream · workspace) Split
  constants.ts every shared number: rules (80 % usage, 30 s stale), formats, timing, sizes (= Rust `consts`)
  fmt.ts      bytes bytesOf age uptime (same rules as the Rust `fmt`)
  game.ts     nudgePoint curveNudge shares rulerStep portPoint wirePath (same rules as the Rust molecules)
  keymap.tsx  createPageKeys([...]): a page's keys once → StatusBar hints, ? help, Ctrl K palette
explorer/     the parts explorer: list on the left, live preview on the right, one route per UI page
  dock/       demo app for server / Docker admin: containers, hosts, deploy, images
  forge/      demo app for game design data: curves, loot tables, unit balance, sequences, ability graph
  pond/       demo app: data.ts, elements.tsx (TaskActions, TaskList…), pages.tsx (one page per layout)
```

## Run the explorer

```sh
npm install
npm run dev        # http://localhost:5173 · ↑↓ browse · Enter opens a UI page · Esc goes back
npm run typecheck
npm run build      # → dist-explorer/
```

Routes: `#/<part-id>` selects a part; `#/page/<id>` shows one UI page full window.

## Use it in an app

Apps copy the parts they use (they never import from APP-UI):

```sh
node <APP-UI>/tools/quag.mjs new desktop ~/code/my-app        # template + its parts
node <APP-UI>/tools/quag.mjs add ~/code/my-app ButtonGroup    # more parts, with the files they import
```

The parts land in the app's `src/quag/` (same folders as `src/` here, plus `quag.css` and a generated `index.ts`).

```tsx
import "./quag/quag.css"; // once, at the root
import { AppShell, AppHeader, ButtonGroup, type ButtonSpec } from "./quag";

// ELEMENT (in your app): the molecule put to use
export function TaskActions(props: { task: Task }) {
  const buttons = (): ButtonSpec[] => [
    { label: "Delete", kind: "danger", onClick: () => askDelete(props.task) },
    { label: "Edit" },
    props.task.done ? { label: "Reopen" } : { label: "Mark done", kind: "primary" },
  ];
  return <ButtonGroup align="end" buttons={buttons()} />;
}

// UI (in your app): the page only places elements
export function TasksPage() {
  return (
    <AppShell header={<AppHeader app="pond" />} nav={<ProjectNav />} inspector={<TaskInspector />}>
      <TaskList />
    </AppShell>
  );
}
```

Rules: only atoms set colors (through `quag.css` classes); molecules arrange; elements wire data;
pages place elements. Desktop button order: primary **last**. The mascot (`EmptyState mascot=…`) is for **private apps only**.
