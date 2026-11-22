/** The two progress bars, live, on the animation tick (the same clock and timings as the TUI demo). */
const DEMO_STEPS_NAMES = ["Pull", "Build image", "Migrate", "Health check", "Switch traffic"];
const PER_STEP_TICKS = 15, SMOOTH_TICKS = 100;
function ProgressDemo() {
  const [tick, setTick] = createSignal(37);
  const timer = setInterval(() => setTick((t) => t + 1), consts.TICK_MS);
  onCleanup(() => clearInterval(timer));
  const frac = () => (tick() % SMOOTH_TICKS) / SMOOTH_TICKS;
  const n = DEMO_STEPS_NAMES.length;
  const t = () => tick() % (PER_STEP_TICKS * (n + 1));
  const done = () => Math.min(n, Math.floor(t() / PER_STEP_TICKS));
  const label = () => (done() === n ? `Done · ${n} of ${n}` : `Step ${done() + 1} of ${n} · ${DEMO_STEPS_NAMES[done()]}`);
  const line = (bar: JSX.Element, num?: string) => <div style={{ display: "flex", gap: "12px", "align-items": "center" }}><div style={{ flex: 1 }}>{bar}</div>{num && <Text tone="muted">{num}</Text>}</div>;
  return (
    <div style={{ display: "grid", gap: "8px", "max-width": "480px" }}>
      <Text>Downloading model.bin</Text>
      {line(<SmoothProgress value={frac()} label="Download" />, `${Math.round(frac() * 100)} %`)}
      <div style={{ "margin-top": "12px" }}><Text>{label()}</Text></div>
      <StepProgress total={n} done={done()} current={(t() % PER_STEP_TICKS) / PER_STEP_TICKS} label="Deploy" />
      <div style={{ "margin-top": "12px" }}><Text>Step 4 of 5 · Health check failed</Text></div>
      <StepProgress total={n} done={3} current={0.6} failed label="Deploy (failed)" />
      <div style={{ "margin-top": "12px", display: "flex", gap: "8px", "align-items": "center" }}><Spinner /><Text tone="muted">Loading…</Text></div>
    </div>
  );
}
function CurveDemo() {
  const a = CURVES[0];
  const [p, setP] = createSignal<Pt[]>(a.points);
  const [k, setK] = createSignal(3);
  return <div style={{ height: "280px", "max-width": "720px" }}><CurveEditor points={p()} onPoints={setP} x={a.x} y={a.y} xUnit={a.xUnit} yUnit={a.yUnit} selected={k()} onSelect={setK} /></div>;
}
function WeightsDemo() {
  const [w, setW] = createSignal(LOOT[0][1]);
  const [sel, setSel] = createSignal(3);
  return <div style={{ "max-width": "720px" }}><WeightsEditor items={w()} selected={sel()} onSelect={setSel} onWeight={(i, v) => setW((all) => all.map((x, j) => (j === i ? [x[0], v] : x)))} /></div>;
}
function GraphDemo() {
  const [at, setAt] = createSignal<Pt[]>(GRAPH_AT);
  const [sel, setSel] = createSignal(1);
  const [pan, setPan] = createSignal<Pt>([0, 0]);
  return <div style={{ height: "440px" }}><NodeGraph nodes={GRAPH} positions={at()} wires={WIRES} selected={sel()} onSelect={setSel}
    onMove={(i, p) => setAt((all) => all.map((x, j) => (j === i ? p : x)))} pan={pan()} onPan={(p) => setPan([Math.max(0, p[0]), Math.max(0, p[1])])} /></div>;
}
function TimelineDemo() {
  const [t, setT] = createSignal(12.5);
  const [c, setC] = createSignal<[number, number]>([0, 2]);
  return <div style={{ "max-width": "900px" }}><Timeline tracks={SEQUENCE} span={SEQ_SPAN} playhead={t()} onPlayhead={setT} unit="s" selected={c()} onSelect={(a, b) => setC([a, b])} /></div>;
}

/** The explorer's catalog: every part, by level, with a live preview. Add new parts here. */
import * as C from "./consts";
import { createSignal, onCleanup, type JSX } from "solid-js";
import {
  AppHeader, Badge, Breadcrumbs, Button, ButtonGroup, Card, Checkbox, CommandPalette, ConfirmDialog, EmptyState, ErrorState,
  Field, HelpOverlay, Input, Kbd, Link, List, ListRow, Menu, NavItem, Progress, PropertyList, Radio, RadioGroup, SearchBar,
  Skeleton, SmoothProgress, Spinner, Status, StatusBar, StepProgress, Table, Tabs, Tag, Text, Toast, consts,
  SecretField, Segmented, SEVERITIES, SeverityBadge, TypeToConfirm,
  Bar, BarChart, DeltaBadge, Histogram, LogViewer, NumberField, ParamRow, Select, Slider, Sparkline, Split, TreeView,
} from "../src";
import { CurveEditor, NodeGraph, type Pt, Timeline, WeightsEditor } from "../src";
import { CURVES, GRAPH, GRAPH_AT, LOOT, SEQ_SPAN, SEQUENCE, WIRES } from "./forge/data";
import { BalanceTable } from "./forge/elements";
import { bigLog, FINDINGS, FRAME_MS, MEMORY_MB, NOTE, RANGES, TASKS, TREE } from "./pond/data";
import {
  FindingDetail, FindingsList, FirewallRules, ServerList,
  Activity, EditorTools, FrameBudget, GpuMemory, MaterialParams, NoteEditor, ProjectNav, ProjectTable, ProjectTree, ServerLoad,
  SyncSettings, TaskActions, TaskInspector, TaskList, TaskProperties,
} from "./pond/elements";
import { PAGES } from "./pond/pages";
import {
  DiffView, EnvTag, Freshness, KeyValueRows, Meter, PortMap, RunProgress, RunStateLabel, ShortId, StepList, type StepSpec,
} from "../src";
import { CONTAINERS, DEPLOY_LOG, DIFF, HOSTS } from "./dock/data";
import { ContainerInspector, ContainerTable, HostCard, NeedsAttention } from "./dock/elements";

const DEMO_STEPS: StepSpec[] = [{ name: "pull", state: "done", took: "8s" }, { name: "migrate", state: "done", took: "3s" }, { name: "health check", state: "running" }, { name: "switch", state: "waiting" }];

export type Level = "Atoms" | "Molecules" | "Elements" | "UI";
export interface Entry {
  id: string;
  level: Level;
  name: string;
  what: string;
  from?: string;
  preview?: () => JSX.Element;
  /** UI level: id of the full page (route #/page/<id>) */
  page?: string;
}

const row = (...c: JSX.Element[]) => <div style={{ display: "flex", gap: "12px", "align-items": "center", "flex-wrap": "wrap" }}>{c}</div>;
const col = (...c: JSX.Element[]) => <div style={{ display: "grid", gap: "12px", "max-width": "420px" }}>{c}</div>;

function CheckboxDemo() {
  const [a, setA] = createSignal(true);
  const [b, setB] = createSignal(false);
  const [c, setC] = createSignal(true);
  return col(
    <Checkbox checked={a()} onChange={setA}>Checked</Checkbox>,
    <Checkbox checked={b()} onChange={setB}>Unchecked</Checkbox>,
    <Checkbox checked={false} disabled>Disabled</Checkbox>,
    <Checkbox toggle checked={c()} onChange={setC}>Toggle variant</Checkbox>,
  );
}
function TabsDemo() {
  const [t, setT] = createSignal(0);
  return <Tabs tabs={["Today", "Upcoming", "Done"]} active={t()} onSelect={setT} />;
}
function SearchDemo() {
  const [v, setV] = createSignal("");
  return <div style={{ "max-width": "360px" }}><SearchBar value={v()} onValue={setV} /></div>;
}
function DialogDemo() {
  const [open, setOpen] = createSignal(false);
  return (
    <>
      <Button kind="danger" onClick={() => setOpen(true)}>Open the dialog</Button>
      <ConfirmDialog open={open()} title="Delete task?" message="“Write README” can't be restored." confirmLabel="Delete" cancelLabel="Keep" danger onConfirm={() => setOpen(false)} onCancel={() => setOpen(false)} />
    </>
  );
}

function RadioDemo() {
  const [v, setV] = createSignal(1);
  return col(
    <Radio name="demo" checked={v() === 0} onSelect={() => setV(0)}>Option A</Radio>,
    <Radio name="demo" checked={v() === 1} onSelect={() => setV(1)}>Option B</Radio>,
    <Radio name="demo-off" checked={false} disabled>Disabled</Radio>,
  );
}
function RadioGroupDemo() {
  const [v, setV] = createSignal(1);
  return col(<RadioGroup label="Priority" options={["Low", "Normal", "High"]} value={v()} onChange={setV} />,
    <RadioGroup column label="Priority" options={["Low", "Normal", "High"]} value={v()} onChange={setV} />);
}
function OverlayDemo(props: { kind: "palette" | "help" | "toast" }) {
  const [open, setOpen] = createSignal(false);
  return (
    <>
      <Button onClick={() => setOpen(true)}>{props.kind === "palette" ? "Open the palette (or Ctrl K on TasksPage)" : props.kind === "help" ? "Show the keys (or ? on TasksPage)" : "Show a toast"}</Button>
      {props.kind === "palette" && <CommandPalette open={open()} onClose={() => setOpen(false)} items={[{ label: "New task", key: "N" }, { label: "Go to tab: Today", key: "1" }, { label: "Delete task", key: "Del", danger: true }]} />}
      {props.kind === "help" && <HelpOverlay open={open()} onClose={() => setOpen(false)} groups={[["Move", [["↑ ↓", "previous / next"], ["Tab", "next zone"]]], ["App", [["Ctrl K", "commands"], ["Esc", "close / back"]]]]} />}
      {props.kind === "toast" && <Toast open={open()} kind="ok" message="Task marked done" onClose={() => setOpen(false)} />}
    </>
  );
}
function EditorDemo() {
  const [note, setNote] = createSignal(NOTE);
  return (
    <div style={{ display: "grid", "grid-template-columns": "44px 1fr 260px", gap: "12px", height: "340px" }}>
      <EditorTools active={1} /><NoteEditor value={note()} onValue={setNote} /><TaskProperties task={TASKS[1]} note={note()} />
    </div>
  );
}

function NumberDemo() {
  const [a, setA] = createSignal(16.6), [b, setB] = createSignal(0.45);
  return col(
    row(<NumberField value={a()} onValue={setA} unit="ms" step={0.1} decimals={1} />, <Text tone="faint">drag sideways · ↑↓ (Shift ×10) · wheel · type</Text>),
    row(<NumberField value={b()} onValue={setB} min={0} max={1} step={0.01} decimals={2} />, <NumberField value={-3} unit="°C" invalid />),
    <Slider value={b()} onValue={setB} min={0} max={1} aria-label="Value" />,
  );
}
function SelectDemo() {
  const [v, setV] = createSignal("5 minutes");
  return <div style={{ width: "240px" }}><Select value={v()} onValue={setV} options={["1 minute", "5 minutes", "Hourly"]} aria-label="Sync every" /></div>;
}
function SplitDemo() {
  const [r, setR] = createSignal(0.4);
  return <div style={{ height: "220px", "max-width": "640px" }}><Split vertical ratio={r()} onRatio={setR} first={<Card style={{ height: "100%" }}><Text tone="muted">left · drag the line, or focus it and use ←→</Text></Card>} second={<Card style={{ height: "100%" }}><Text tone="muted">right</Text></Card>} /></div>;
}
function TreeDemo() {
  const [open, setOpen] = createSignal(new Set(["qt", "qt/docs"]));
  const [sel, setSel] = createSignal("qt/docs/structure");
  return <div style={{ width: "300px" }}><TreeView label="Files" items={TREE} open={open()} selected={sel()} onSelect={setSel} onToggle={(id, o) => setOpen((s) => { const n = new Set(s); o ? n.add(id) : n.delete(id); return n; })} /></div>;
}
function ParamDemo() {
  const [v, setV] = createSignal(1.5);
  return <div style={{ "max-width": "520px" }}><ParamRow label="Exposure" value={v()} onValue={setV} min={-4} max={4} step={0.1} decimals={1} unit="EV" /></div>;
}
const LOG_DEMO = bigLog(C.LOG_LINES_PREVIEW);

function SegmentedDemo() {
  const [r, setR] = createSignal(2), [v, setV] = createSignal(0);
  return col(<Segmented label="Time range" options={RANGES} value={r()} onChange={setR} />, <Segmented label="View" options={["List", "Grid"]} value={v()} onChange={setV} />);
}
function TypeConfirmDemo() {
  const [open, setOpen] = createSignal(false);
  return (<><Button kind="danger" onClick={() => setOpen(true)}>Delete server…</Button>
    <TypeToConfirm open={open()} title="Delete server" message="api-1 and all its data will be deleted." name="api-1" confirmLabel="Delete" onCancel={() => setOpen(false)} onConfirm={() => setOpen(false)} /></>);
}
function FindingsDemo() {
  const [sel, setSel] = createSignal(0), [r, setR] = createSignal(3);
  return (
    <div style={{ display: "grid", "grid-template-columns": "1fr 300px", gap: "16px" }}>
      <FindingsList findings={FINDINGS} selected={sel()} onSelect={setSel} range={r()} onRange={setR} />
      <FindingDetail finding={FINDINGS[sel()]} onRevoke={() => {}} onIgnore={() => {}} />
    </div>
  );
}

export const CATALOG: Entry[] = [
  // ── 1 ATOMS
  { id: "button", level: "Atoms", name: "Button", what: "One button. Kinds: primary (max one per screen), secondary, danger, ghost.",
    preview: () => col(row(<Button kind="primary">Primary</Button>, <Button>Secondary</Button>, <Button kind="danger">Danger</Button>, <Button kind="ghost">Ghost</Button>),
      row(<Button disabled>Disabled</Button>, <Text tone="faint">hover, press and Tab-focus them</Text>)) },
  { id: "checkbox", level: "Atoms", name: "Checkbox", what: "A box or a toggle. Keeps a real input for keyboard and screen readers.", preview: () => <CheckboxDemo /> },
  { id: "input", level: "Atoms", name: "Input", what: "One-line text input. States: empty, filled, focused, invalid, disabled.",
    preview: () => col(<Input placeholder="Placeholder…" />, <Input value="Filled value" />, <Input value="tomorow" invalid />, <Input value="Disabled" disabled />) },
  { id: "card", level: "Atoms", name: "Card", what: "The box: an empty container. Focused = Quag outline.",
    preview: () => row(<Card style={{ width: "200px", height: "90px" }}><Text tone="muted">Card</Text></Card>, <Card focused style={{ width: "200px", height: "90px" }}><Text tone="muted">Focused card</Text></Card>) },
  { id: "text", level: "Atoms", name: "Text", what: "Text in one of six tones. Use it instead of choosing a color.",
    preview: () => col(<Text tone="title">◆ Title</Text>, <Text tone="heading">Heading</Text>, <Text>Body text</Text>, <Text tone="muted">Muted: secondary info</Text>, <Text tone="faint">Faint: placeholders, disabled</Text>, <Text tone="accent">Accent: the one thing</Text>) },
  { id: "small", level: "Atoms", name: "Tag · Kbd · Status", what: "Small atoms: tag (Gill), keyboard key, status icon + text.",
    preview: () => col(row(<Tag>docs</Tag>, <Tag>bug</Tag>, <Kbd>/</Kbd>, <Kbd>Ctrl</Kbd>),
      row(<Status kind="ok">Saved</Status>, <Status kind="warn">Low disk</Status>, <Status kind="err">Sync failed</Status>, <Status kind="info">Update</Status>)) },
  { id: "progress", level: "Atoms", name: "Progress · Spinner", what: "Progress bar (numbers go next to it) and a spinner (always with a label).",
    preview: () => col(<Progress value={0.58} label="58%" />, row(<Spinner />, <Text tone="muted">Loading…</Text>)) },

  { id: "radio", level: "Atoms", name: "Radio", what: "One option of a single choice. Real radio input: arrow keys move inside a group.", preview: () => <RadioDemo /> },
  { id: "badge", level: "Atoms", name: "Badge · Link · status Text", what: "A count (0 hides, 99+), a link, text in a status tone.",
    preview: () => col(row(<Text>Inbox</Text>, <Badge count={3} />, <Text>Mentions</Text>, <Badge count={128} />),
      row(<Text tone="muted">See</Text>, <Link>STRUCTURE.md</Link>), row(<Text tone="err">Upload failed</Text>, <Text tone="ok">Saved</Text>, <Text tone="warn">Slow</Text>)) },
  { id: "progressbars", level: "Atoms", name: "SmoothProgress · StepProgress", what: "Progress that glides (downloads), and progress by steps (deploys). Numbers next to the bar; the spinner turns once per 0.8 s.",
    preview: () => <ProgressDemo /> },
  { id: "skeleton", level: "Atoms", name: "Skeleton", what: "Loading placeholder where text will be. Keeps the layout while loading.",
    preview: () => col(<Skeleton width="60%" />, <Skeleton width="90%" />, <Skeleton width="75%" />, <Skeleton width="40%" />) },

  { id: "runstate", level: "Atoms", name: "RunStateLabel · EnvTag · ShortId", what: "Servers: container state (stopped is grey, not red), environment, ids.",
    preview: () => col(
      row(<RunStateLabel state={{ kind: "running" }} />, <RunStateLabel state={{ kind: "restarting" }} />, <RunStateLabel state={{ kind: "stopped" }} />, <RunStateLabel state={{ kind: "paused" }} />),
      row(<RunStateLabel state={{ kind: "exited", code: 0 }} />, <RunStateLabel state={{ kind: "exited", code: 137 }} />, <RunStateLabel state={{ kind: "flapping", restarts: 5 }} />),
      row(<EnvTag env="prod" />, <EnvTag env="staging" />, <EnvTag env="dev" />, <ShortId id="sha256:3f9a1c2b7e41aa00ff" />)) },
  { id: "severity", level: "Atoms", name: "SeverityBadge", what: "Five levels on four colors: CRIT filled, HIGH outlined, same width.",
    preview: () => row(...SEVERITIES.map((l) => <SeverityBadge level={l} />)) },
  { id: "secret", level: "Atoms", name: "SecretField", what: "A key or password: masked (last 4 visible), Show and Copy. Shown = yellow border.",
    preview: () => <div style={{ "max-width": "420px" }}><SecretField value="sk_live_51HxQ2bK9vTz7m3f9a" /></div> },
  { id: "number", level: "Atoms", name: "NumberField · Slider", what: "A number with its unit; drag, arrows, wheel or type. And its slider.", preview: () => <NumberDemo /> },
  { id: "select", level: "Atoms", name: "Select", what: "A dropdown. A real <select>: keyboard and the OS list for free.", preview: () => <SelectDemo /> },
  { id: "charts", level: "Atoms", name: "Sparkline · Histogram · Bar · Delta", what: "Chart atoms. Budgets turn values red; deltas know which way is good.",
    preview: () => col(<Sparkline values={FRAME_MS} max={25} budget={16.6} width={240} />, <Histogram values={FRAME_MS} max={25} budget={16.6} width={420} height={70} />,
      <div style={{ width: "260px", display: "grid", gap: "8px" }}><Bar fraction={0.62} /><Bar fraction={0.91} over /></div>,
      row(<DeltaBadge change={-0.12} higherIsBetter={false} />, <DeltaBadge change={0.08} higherIsBetter={false} />, <DeltaBadge change={0.15} higherIsBetter />, <DeltaBadge change={0} higherIsBetter />)) },

  // ── 2 MOLECULES
  { id: "buttongroup", level: "Molecules", name: "ButtonGroup", from: "Button × n in a box",
    what: "An array of buttons in a row, optionally boxed. Generic: it doesn't know what the buttons do.",
    preview: () => col(<ButtonGroup buttons={[{ label: "Button A" }, { label: "Button B" }, { label: "Button C", kind: "primary" }]} />,
      <ButtonGroup boxed align="end" buttons={[{ label: "Cancel", kind: "ghost" }, { label: "Confirm", kind: "primary" }]} />) },
  { id: "field", level: "Molecules", name: "Field", from: "Label + Input + Help", what: "Label above an input, with help or error text under it.",
    preview: () => col(<Field label="Name" help="Shown in the title.">{(id) => <Input id={id} value="pond" />}</Field>,
      <Field label="Due" error="Can't read that date.">{(id, bad) => <Input id={id} value="tomorow" invalid={bad} />}</Field>) },
  { id: "searchbar", level: "Molecules", name: "SearchBar", from: "icon + Input + Kbd", what: "Search input with its shortcut.", preview: () => <SearchDemo /> },
  { id: "tabs", level: "Molecules", name: "Tabs", from: "tab buttons", what: "2–5 views of the same data.", preview: () => <TabsDemo /> },
  { id: "navitem", level: "Molecules", name: "NavItem", from: "icon + label + count", what: "One place in the sidebar.",
    preview: () => <div style={{ width: "232px", display: "grid", gap: "4px" }}><NavItem label="Inbox" icon="◇" count={4} active /><NavItem label="quag-theme" icon="◇" count={2} /></div> },
  { id: "listrow", level: "Molecules", name: "ListRow", from: "check + title + Tag + Status + meta", what: "One row of a list. Long titles end with …",
    preview: () => <div style={{ "max-width": "560px" }}><List><ListRow title="Open row" check={false} tags={["docs"]} meta="today" metaAccent /><ListRow title="Selected row" check={false} tags={["ui"]} status="warn" meta="fri" selected /><ListRow title="Done row" check tags={["ui"]} status="ok" meta="done" /><ListRow title="A very long title that will not fit in the space it has and gets cut" check={false} tags={["bug"]} status="err" meta="sat" /></List></div> },

  { id: "radiogroup", level: "Molecules", name: "RadioGroup", from: "Radio × n", what: "One choice among a few. A row, or a column.", preview: () => <RadioGroupDemo /> },
  { id: "breadcrumbs", level: "Molecules", name: "Breadcrumbs", from: "Text × n", what: "Where you are. Over `max`, the middle collapses to …",
    preview: () => col(<Breadcrumbs path={["pond", "quag-theme", "Write README", "README.md"]} />, <Breadcrumbs max={3} path={["pond", "quag-theme", "Write README", "README.md"]} />) },
  { id: "menu", level: "Molecules", name: "Menu", from: "Card + item buttons + Kbd", what: "Actions with their keys. Danger last, after a separator, in red.",
    preview: () => <div style={{ width: "240px" }}><Menu label="Task" active={1} items={[{ label: "Open", key: "Enter" }, { label: "Rename", key: "R" }, { label: "Duplicate", key: "Ctrl D" }, { label: "Delete", key: "Del", danger: true }]} /></div> },
  { id: "table", level: "Molecules", name: "Table", from: "header + rows of Text", what: "Rows and columns. Numbers right-aligned; long cells end with …",
    preview: () => <div style={{ "max-width": "560px" }}><Table selected={1} columns={[{ header: "Name" }, { header: "Size", num: true, width: "80px" }, { header: "Changed", num: true, width: "110px" }]}
      rows={[["README.md", "4 KB", "today"], ["Cargo.toml", "1 KB", "today"], ["a_very_long_file_name_that_will_be_cut.rs", "12 KB", "yesterday"]]} /></div> },
  { id: "proplist", level: "Molecules", name: "PropertyList", from: "Text × 2 per row", what: "Read-only key / value facts.",
    preview: () => <div style={{ width: "320px" }}><PropertyList items={[["Name", "pond"], ["Version", "0.4.2"], ["Path", "~/apps/pond"], ["Size", "1.2 MB"]]} /></div> },

  { id: "tree", level: "Molecules", name: "TreeView", from: "rows + twisty + Status", what: "Any hierarchy. ↑↓ move, → opens, ← closes or goes to the parent.", preview: () => <TreeDemo /> },
  { id: "paramrow", level: "Molecules", name: "ParamRow", from: "Label + Slider + NumberField", what: "A tunable number. Narrow: the slider hides.", preview: () => <ParamDemo /> },
  { id: "barchart", level: "Molecules", name: "BarChart", from: "label + Bar + value", what: "Compare a few amounts. Over budget = red.",
    preview: () => <div style={{ "max-width": "520px" }}><BarChart items={MEMORY_MB} unit="MB" max={1000} budget={800} /></div> },
  { id: "meter", level: "Molecules", name: "Meter · PortMap · Freshness", from: "UsageBar + Status · text", what: "Usage of a limit (80 % yellow, 100 % red); a published port; how old live data is.",
    preview: () => <div style={{ display: "grid", gap: "12px", "max-width": "420px" }}>
      <Meter label="api-1" fraction={210 / 512} value="210 / 512 MiB" /><Meter label="api-2" fraction={430 / 512} value="430 / 512 MiB" /><Meter label="worker" fraction={530 / 512} value="530 / 512 MiB" />
      {row(<PortMap host={8080} container={80} proto="tcp" />, <PortMap host={6379} container={6379} proto="tcp" public />, <PortMap container={5432} proto="tcp" />)}
      {row(<Freshness age={3} />, <Freshness age={190} staleAfter={600} />, <Freshness age={45} />)}
    </div> },
  { id: "steps", level: "Molecules", name: "StepList · KeyValueRows", from: "StepIcon · SecretField", what: "Steps of a job (vertical or inline); env vars with masked secrets.",
    preview: () => <div style={{ display: "grid", gap: "20px", "max-width": "460px" }}>
      <StepList steps={DEMO_STEPS} inline /><StepList steps={DEMO_STEPS} />
      <KeyValueRows rows={[["RUST_LOG", "info"], ["DATABASE_URL", "postgres://shop:hunter2@db/shop", true], ["WORKERS", "4"]]} />
    </div> },
  { id: "segmented", level: "Molecules", name: "Segmented", from: "joined buttons", what: "One of a few short options: time range, view mode. ←→ move.", preview: () => <SegmentedDemo /> },
  { id: "split", level: "UI", name: "Split", what: "Resizable panes: drag, or focus the line and use the arrows.", preview: () => <SplitDemo /> },

  // ── 3 ELEMENTS
  { id: "taskactions", level: "Elements", name: "TaskActions", from: "ButtonGroup + the selected task",
    what: "Your example: the ButtonGroup put to use. It picks the buttons from the data (done → Reopen) and wires them (Delete → ConfirmDialog).",
    preview: () => col(<Text tone="muted">Open task:</Text>, <TaskActions task={TASKS[1]} />, <Text tone="muted">Done task:</Text>, <TaskActions task={TASKS[3]} />) },
  { id: "tasklist", level: "Elements", name: "TaskList", from: "SearchBar + List + ListRow, EmptyState, Spinner",
    what: "The tasks, searchable. Owns its states: ideal, loading, empty.",
    preview: () => col(<TaskList tasks={TASKS.slice(0, 4)} selected={2} />, <TaskList tasks={[]} loading />, <TaskList tasks={[]} />) },
  { id: "taskinspector", level: "Elements", name: "TaskInspector", from: "Field × 2 + TaskActions", what: "Details of the selected task. State: nothing selected.",
    preview: () => row(<div style={{ width: "320px" }}><TaskInspector task={TASKS[2]} /></div>, <div style={{ width: "240px" }}><TaskInspector /></div>) },
  { id: "projectnav", level: "Elements", name: "ProjectNav", from: "NavItem × n + Progress", what: "Pond's sidebar.",
    preview: () => <div style={{ width: "232px", height: "300px", display: "flex", "flex-direction": "column", background: "var(--q-surface)", padding: "12px", "border-radius": "10px" }}><ProjectNav active={0} /></div> },
  { id: "editor", level: "Elements", name: "EditorTools · NoteEditor · TaskProperties", from: "Button × n · Breadcrumbs + Textarea · PropertyList + RadioGroup",
    what: "The workspace elements.", preview: () => <EditorDemo /> },
  { id: "projecttable", level: "Elements", name: "ProjectTable", from: "Card + Table, ErrorState", what: "Every project with its counts. States: ideal, error.",
    preview: () => col(<ProjectTable />, <ProjectTable error />) },
  { id: "projecttree", level: "Elements", name: "ProjectTree", from: "TreeView", what: "The workspace's files.", preview: () => <div style={{ width: "300px" }}><ProjectTree /></div> },
  { id: "activity", level: "Elements", name: "Activity · SyncSettings", from: "Histogram + DeltaBadge · Select + ParamRow", what: "Dashboard trend; a settings group.",
    preview: () => row(<div style={{ width: "300px" }}><Activity /></div>, <div style={{ width: "360px" }}><SyncSettings /></div>) },
  { id: "perf", level: "Elements", name: "FrameBudget · GpuMemory", from: "Histogram + DeltaBadge · BarChart", what: "Optimisation: frame times vs 16.6 ms, memory per asset type.",
    preview: () => <div style={{ display: "grid", gap: "12px", "max-width": "560px" }}><FrameBudget /><GpuMemory /></div> },
  { id: "material", level: "Elements", name: "MaterialParams · ServerLoad", from: "ParamRow × n · Sparkline", what: "Tech art tuning; server CPU trend.",
    preview: () => <div style={{ display: "grid", gap: "16px", "max-width": "520px" }}><MaterialParams /><ServerLoad name="api-1" cpu={[22, 30, 28, 35, 41, 38, 44, 52, 47, 42]} /><ServerLoad name="db-1" cpu={[55, 62, 70, 78, 84, 91, 88, 86, 82, 87]} /></div> },
  { id: "typeconfirm", level: "Elements", name: "TypeToConfirm (shared)", from: "dialog + Kbd + Input + ButtonGroup", what: "Type the name before an action that can't be undone.", preview: () => <TypeConfirmDemo /> },
  { id: "findings", level: "Elements", name: "FindingsList · FindingDetail", from: "Segmented + ListRow + SeverityBadge · PropertyList + SecretField + ButtonGroup",
    what: "Security: findings worst first, and the selected one.", preview: () => <FindingsDemo /> },
  { id: "firewall", level: "Elements", name: "FirewallRules · ServerList", from: "Table · Sparkline", what: "Security: rules with hits; servers with CPU trend.",
    preview: () => <div style={{ display: "grid", "grid-template-columns": "1fr 240px", gap: "16px", "max-width": "860px" }}><FirewallRules /><ServerList /></div> },
  { id: "diff", level: "Elements", name: "DiffView (shared)", from: "DiffLine + Text + EmptyState", what: "Review a change before applying it.",
    preview: () => <div style={{ height: "380px", display: "grid", "max-width": "640px" }}><DiffView title="Changes · compose.yaml" lines={DIFF} /></div> },
  { id: "curve", level: "Molecules", name: "Curve · CurveEditor", from: "Curve (atom) + labels + readout", what: "Game design: a curve to shape (difficulty, XP, falloff). Drag keys; the page binds the arrows.",
    preview: () => <CurveDemo /> },
  { id: "weights", level: "Molecules", name: "WeightsEditor", from: "Bar + NumberField + labels", what: "Weighted choices (loot, spawns): %, odds, always 100 %.",
    preview: () => <WeightsDemo /> },
  { id: "timeline", level: "Molecules", name: "Timeline", from: "Clip + Playhead (atoms) + ruler", what: "Tracks of clips over time (cutscenes, events, waves). Click the ruler to move the playhead.",
    preview: () => <TimelineDemo /> },
  { id: "nodegraph", level: "Molecules", name: "NodeGraph", from: "NodeFrame + NodePort + Wire (atoms)", what: "Nodes and wires (abilities, dialogue, shaders, AI). Drag titles to move nodes, the background to pan.",
    preview: () => <GraphDemo /> },
  { id: "balance-states", level: "Elements", name: "BalanceTable · states", from: "Skeleton · ErrorState", what: "forge: every panel shows loading and can't-read the same way; KPIs show – until the data is there.",
    preview: () => <div style={{ display: "grid", "grid-template-columns": "1fr 1fr", gap: "16px" }}><BalanceTable selected={0} onSelect={() => {}} load="loading" /><BalanceTable selected={0} onSelect={() => {}} load="failed" onRetry={() => {}} /></div> },
  { id: "balance", level: "Elements", name: "BalanceTable", from: "Table (atom cells) + DeltaBadge + Status", what: "forge: units vs last patch, value for cost, outliers (warn / info, not errors).",
    preview: () => <BalanceTable selected={3} onSelect={() => {}} /> },
  { id: "runprogress", level: "Elements", name: "RunProgress (shared)", from: "StepList + LogViewer + ButtonGroup", what: "A running job, its output, Cancel / Retry. A zone, not a modal.",
    preview: () => <div style={{ height: "340px", "max-width": "720px" }}><RunProgress steps={DEMO_STEPS} output={DEPLOY_LOG.slice(0, 5)} /></div> },
  { id: "containers", level: "Elements", name: "ContainerTable · ContainerInspector", from: "Table (atom cells) + RunStateLabel + Sparkline + Meter · PropertyList + PortMap + KeyValueRows",
    what: "dock: containers of a host, and the selected one.",
    preview: () => <div style={{ display: "grid", "grid-template-columns": "1fr 320px", gap: "16px" }}>
      <ContainerTable states={CONTAINERS.map((c) => c.state)} selected={1} onSelect={() => {}} />
      <ContainerInspector container={CONTAINERS[1]} state={CONTAINERS[1].state} onRestart={() => {}} onStartStop={() => {}} onRemove={() => {}} />
    </div> },
  { id: "hostcards", level: "Elements", name: "HostCard · NeedsAttention", from: "Meter + Sparkline / ErrorState · ListRow", what: "dock: one host (or why it can't be reached); what's wrong.",
    preview: () => <div style={{ display: "grid", "grid-template-columns": "1fr 1fr", gap: "16px", "max-width": "900px" }}><HostCard host={HOSTS[2]} /><HostCard host={HOSTS[3]} /><div style={{ "grid-column": "1 / -1" }}><NeedsAttention /></div></div> },
  { id: "logviewer", level: "Elements", name: "LogViewer (shared)", from: "SearchBar + virtual rows + Mark", what: "Big logs (this one: 100,000 lines). Filter, highlight, follow.",
    preview: () => <div style={{ height: "380px" }}><LogViewer lines={LOG_DEMO} /></div> },
  { id: "appheader", level: "Elements", name: "AppHeader (shared)", from: "brand + Tabs/actions", what: "Every app's header.",
    preview: () => <div style={{ border: "1px solid var(--q-border)", "border-radius": "10px" }}><AppHeader app="pond" location="tasks"><Tabs tabs={["Today", "Done"]} active={0} onSelect={() => {}} /></AppHeader></div> },
  { id: "statusbar", level: "Elements", name: "StatusBar (shared)", from: "Kbd hints + Status", what: "Key hints left, last result right.",
    preview: () => <StatusBar hints={[["/", "search"], ["N", "new"]]} status={{ kind: "ok", text: "Saved 2s ago" }} /> },
  { id: "emptystate", level: "Elements", name: "EmptyState (shared)", from: "Text + Kbd (+ mascot)", what: "The empty state of any zone. Mascot only in private apps.",
    preview: () => <Card><EmptyState message="Nothing here yet." hint={<>Press <Kbd>N</Kbd> to add a task.</>} /></Card> },
  { id: "confirm", level: "Elements", name: "ConfirmDialog (shared)", from: "Card + Text + ButtonGroup", what: "“Are you sure?”. Esc cancels; focus starts on the safe button.", preview: () => <DialogDemo /> },

  { id: "errorstate", level: "Elements", name: "ErrorState (shared)", from: "Status + Text + Button", what: "A zone failed: what + why + Retry. The rest of the page stays usable.",
    preview: () => <Card style={{ "max-width": "480px" }}><ErrorState what="Couldn't load tasks" why="The sync server didn't answer (timeout)." /></Card> },
  { id: "toast", level: "Elements", name: "Toast (shared)", from: "Status in a raised box", what: "A result to notice. Bottom-right; goes away by itself.", preview: () => <OverlayDemo kind="toast" /> },
  { id: "palette", level: "Elements", name: "CommandPalette (shared)", from: "SearchBar + Menu", what: "Ctrl K: every action by name. ↑↓, Enter, Esc.", preview: () => <OverlayDemo kind="palette" /> },
  { id: "help", level: "Elements", name: "HelpOverlay (shared)", from: "Kbd + Text in a dialog", what: "?: every key of the page, grouped.", preview: () => <OverlayDemo kind="help" /> },

  // ── 4 UI
  ...PAGES.map((p) => ({
    id: `ui-${p.id}`, level: "UI" as Level, name: p.name, page: p.id,
    what: `A page in the ${p.layout} layout. It only places elements in the AppShell zones.`,
  })),
];

export const LEVELS: { level: Level; n: number; rule: string }[] = [
  { level: "Atoms", n: 1, rule: "one control; only atoms set colors" },
  { level: "Molecules", n: 2, rule: "atoms arranged in a box; generic" },
  { level: "Elements", n: 3, rule: "a molecule put to use, wired to data" },
  { level: "UI", n: 4, rule: "a page placing elements" },
];
