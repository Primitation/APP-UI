/** UI: the forge demo pages (game design data). Pages only place elements; keys once with createPageKeys. */
import * as C from "../consts";
import { createSignal, onCleanup } from "solid-js";
import { AppHeader, AppShell, consts, createPageKeys, curveNudge, Layout, PageHead, type Pt, StatusBar, Toast } from "../../src";
import { CURVES, GRAPH, GRAPH_AT, type Load, LOOT, SEQ_SPAN, SEQUENCE, UNITS } from "./data";
import { AssetList, BalanceKpis, BalanceTable, ClipInspector, CurvePanel, GraphPanel, KeyInspector, LootPanel, NodeInspector, outliers, PerHundred, SequencePanel } from "./elements";

const header = (location: string) => <AppHeader app="forge" location={location} />;
const C_NODE = consts.NODE_STEP_PX, C_NODE_FAST = consts.NODE_STEP_FAST_PX;
const ARROWS = ["ArrowLeft", "ArrowRight", "ArrowUp", "ArrowDown"];
const clampI = (i: number, n: number) => Math.max(0, Math.min(n - 1, i));

/** Workspace: curves · the curve · the selected key. */
export function CurvesPage() {
  const [curve, setCurve] = createSignal(0);
  const [points, setPoints] = createSignal<Pt[]>(CURVES[0].points);
  const [key, setKey] = createSignal(3);
  const [toast, setToast] = createSignal<string>();
  const a = () => CURVES[curve()];
  const pick = (i: number) => { setCurve(i); setPoints(CURVES[i].points); setKey(0); };
  const add = () => {
    const p = points(), i = key();
    if (i >= p.length - 1) return;
    setPoints([...p.slice(0, i + 1), [(p[i][0] + p[i + 1][0]) / 2, (p[i][1] + p[i + 1][1]) / 2], ...p.slice(i + 1)]);
    setKey(i + 1);
  };
  // the first and last keys hold the range ends: they stay
  const del = () => { const i = key(); if (i > 0 && i < points().length - 1) { setPoints(points().filter((_, j) => j !== i)); setKey(i - 1); setToast("Key deleted"); } };
  const keys = createPageKeys([
    { key: "PgUp PgDn", what: "previous / next curve", group: "Move", hint: "curve", match: ["PageUp", "PageDown"], run: (k) => pick((curve() + (k === "PageUp" ? CURVES.length - 1 : 1)) % CURVES.length) },
    { key: "[ ]", what: "previous / next key", group: "Move", hint: "key", match: ["[", "]"], run: (k) => setKey((i) => clampI(i + (k === "]" ? 1 : -1), points().length)) },
    { key: "← → ↑ ↓", what: "move the key", group: "Edit", hint: "move key", match: ARROWS, run: (k) => setPoints(curveNudge(points(), key(), k, a().x, a().y)) },
    { key: "Shift ← → ↑ ↓", what: "move the key a little", group: "Edit", match: ARROWS.map((k) => `shift+${k}`), run: (k) => setPoints(curveNudge(points(), key(), k, a().x, a().y)) },
    { key: "A", what: "add a key after this one", group: "Edit", command: "Add key", match: ["a"], run: add },
    { key: "Del", what: "delete the key", group: "Edit", command: "Delete key", danger: true, match: ["Delete"], run: del },
  ]);
  return (
    <AppShell header={header("curves")} nav={<AssetList title="Curves" names={CURVES.map((c) => c.name)} selected={curve()} onSelect={pick} />}
      inspector={<KeyInspector asset={a()} points={points()} selected={key()} onAdd={add} onDelete={del} />}
      status={<StatusBar hints={keys.hints()} status={{ kind: "ok", text: "saved" }} />}>
      <CurvePanel asset={a()} points={points()} onPoints={setPoints} selected={key()} onSelect={setKey} />
      {keys.overlays}
      <Toast open={!!toast()} kind="ok" message={toast() ?? ""} onClose={() => setToast()} />
    </AppShell>
  );
}

/** List → Detail: drop tables · weights · what 100 kills give. */
export function LootPage() {
  const [table, setTable] = createSignal(0);
  const [weights, setWeights] = createSignal(LOOT.map((t) => t[1].map((i) => i[1])));
  const [sel, setSel] = createSignal(3);
  const items = () => LOOT[table()][1].map(([n], i) => [n, weights()[table()][i]] as [string, number]);
  const setW = (i: number, w: number) => setWeights((all) => all.map((t, ti) => (ti !== table() ? t : t.map((x, j) => (j === i ? Math.max(0, w) : x)))));
  const pick = (t: number) => { setTable(t); setSel(0); };
  const keys = createPageKeys([
    { key: "PgUp PgDn", what: "previous / next table", group: "Move", hint: "table", match: ["PageUp", "PageDown"], run: (k) => pick((table() + (k === "PageUp" ? LOOT.length - 1 : 1)) % LOOT.length) },
    { key: "↑ ↓", what: "previous / next item", group: "Move", hint: "move", match: ["ArrowUp", "ArrowDown"], run: (k) => setSel((i) => clampI(i + (k === "ArrowDown" ? 1 : -1), items().length)) },
    { key: "← →", what: "less / more weight", group: "Edit", hint: "weight", match: ["ArrowLeft", "ArrowRight"], run: (k) => setW(sel(), items()[sel()][1] + (k === "ArrowRight" ? 1 : -1)) },
  ]);
  return (
    <AppShell header={header("loot tables")} nav={<AssetList title="Tables" names={LOOT.map((t) => t[0])} selected={table()} onSelect={pick} />}
      inspector={<PerHundred items={items()} />} status={<StatusBar hints={keys.hints()} status={{ kind: "ok", text: "saved" }} />}>
      <LootPanel name={LOOT[table()][0]} items={items()} selected={sel()} onSelect={setSel} onWeight={setW} />
      {keys.overlays}
    </AppShell>
  );
}

/** Dashboard: the roster's numbers, then every unit against last patch. */
export function BalancePage() {
  const [sel, setSel] = createSignal(3);
  const [load, setLoad] = createSignal<Load>("ready");
  const [strong, weak] = outliers();
  // reloading shows the skeleton until the data is back
  const reload = () => { setLoad("loading"); const t = setTimeout(() => setLoad("ready"), C.SETTLE_MS); onCleanup(() => clearTimeout(t)); };
  const status = () => load() === "ready" ? { kind: (strong ? "warn" : "ok") as "warn" | "ok", text: `${strong} strong · ${weak} weak` } : load() === "loading" ? { kind: "info" as const, text: "loading…" } : { kind: "err" as const, text: "data/units.csv unreadable" };
  const keys = createPageKeys([
    { key: "R", what: "reload the data", group: "Act", hint: "reload", command: "Reload data", match: ["r"], run: reload },
    { key: "↑ ↓", what: "previous / next unit", group: "Move", hint: "move", match: ["ArrowUp", "ArrowDown"], run: (k) => setSel((i) => clampI(i + (k === "ArrowDown" ? 1 : -1), UNITS.length)) },
  ]);
  return (
    <AppShell header={header("balance · patch 0.9")} status={<StatusBar hints={keys.hints()} status={status()} />}>
      <PageHead title="Balance" />
      <Layout kind="dashboard">
        <BalanceKpis load={load()} />
        <div style={{ "grid-column": "1 / -1" }}><BalanceTable selected={sel()} onSelect={setSel} load={load()} onRetry={reload} /></div>
      </Layout>
      {keys.overlays}
    </AppShell>
  );
}

/** Timeline: the sequence · the selected clip. */
export function SequencePage() {
  const [playhead, setPlayhead] = createSignal(12.5);
  const [playing, setPlaying] = createSignal(false);
  const [clip, setClip] = createSignal<[number, number]>([0, 2]);
  const timer = setInterval(() => { if (playing()) setPlayhead((t) => (t + C.PLAY_STEP_SECS > SEQ_SPAN[1] ? SEQ_SPAN[0] : t + C.PLAY_STEP_SECS)); }, C.PLAY_FRAME_MS);
  onCleanup(() => clearInterval(timer));
  const scrub = (t: number) => setPlayhead(Math.max(SEQ_SPAN[0], Math.min(SEQ_SPAN[1], t)));
  const keys = createPageKeys([
    { key: "Space", what: "play / pause", group: "Play", hint: "play", command: "Play / pause", match: [" "], run: () => setPlaying((p) => !p) },
    { key: "← →", what: "move the playhead", group: "Play", hint: "scrub", match: ["ArrowLeft", "ArrowRight"], run: (k) => scrub(playhead() + (k === "ArrowRight" ? C.SCRUB_SECS : -C.SCRUB_SECS)) },
    { key: "↑ ↓", what: "previous / next track", group: "Move", hint: "track", match: ["ArrowUp", "ArrowDown"], run: (k) => { const t = clampI(clip()[0] + (k === "ArrowDown" ? 1 : -1), SEQUENCE.length); setClip([t, Math.min(clip()[1], SEQUENCE[t].clips.length - 1)]); } },
    { key: "[ ]", what: "previous / next clip", group: "Move", hint: "clip", match: ["[", "]"], run: (k) => setClip(([t, c]) => [t, clampI(c + (k === "]" ? 1 : -1), SEQUENCE[t].clips.length)]) },
  ]);
  return (
    <AppShell header={header("sequences")} inspector={<ClipInspector selected={clip()} />} status={<StatusBar hints={keys.hints()} status={{ kind: "ok", text: "saved" }} />}>
      <SequencePanel playhead={playhead()} onPlayhead={scrub} playing={playing()} selected={clip()} onSelect={(t, c) => setClip([t, c])} />
      {keys.overlays}
    </AppShell>
  );
}

/** Canvas: the graph · the selected node. */
export function GraphPage() {
  const [at, setAt] = createSignal<Pt[]>(GRAPH_AT);
  const [node, setNode] = createSignal(1);
  const [pan, setPan] = createSignal<Pt>([0, 0]);
  const moveTo = (i: number, p: Pt) => setAt((all) => all.map((x, j) => (j === i ? [Math.max(0, p[0]), Math.max(0, p[1])] : x)));
  const step = (k: string) => {
    const s = k.startsWith("shift+") ? C_NODE_FAST : C_NODE;
    const d: Record<string, Pt> = { ArrowLeft: [-s, 0], ArrowRight: [s, 0], ArrowUp: [0, -s], ArrowDown: [0, s] };
    const [dx, dy] = d[k.replace("shift+", "")] ?? [0, 0];
    const [x, y] = at()[node()];
    moveTo(node(), [x + dx, y + dy]);
  };
  const keys = createPageKeys([
    { key: "PgUp PgDn", what: "previous / next node", group: "Move", hint: "node", match: ["PageUp", "PageDown"], run: (k) => setNode((i) => (i + (k === "PageUp" ? GRAPH.length - 1 : 1)) % GRAPH.length) },
    { key: "← → ↑ ↓", what: "move the node", group: "Edit", hint: "move node", match: ARROWS, run: step },
    { key: "Shift ← → ↑ ↓", what: "move the node faster", group: "Edit", match: ARROWS.map((k) => `shift+${k}`), run: step },
    { key: "drag", what: "drag a title to move a node, the background to pan", group: "View" },
  ]);
  return (
    <AppShell header={header("abilities")} inspector={<NodeInspector selected={node()} />} status={<StatusBar hints={keys.hints()} status={{ kind: "ok", text: "saved" }} />}>
      <GraphPanel positions={at()} onMove={moveTo} selected={node()} onSelect={setNode} pan={pan()} onPan={(p) => setPan([Math.max(0, p[0]), Math.max(0, p[1])])} />
      {keys.overlays}
    </AppShell>
  );
}

export const FORGE_PAGES = [
  { id: "curves", name: "CurvesPage (forge)", layout: "Workspace", page: CurvesPage },
  { id: "loot", name: "LootPage (forge)", layout: "List → Detail", page: LootPage },
  { id: "balance", name: "BalancePage (forge)", layout: "Dashboard", page: BalancePage },
  { id: "sequence", name: "SequencePage (forge)", layout: "Timeline", page: SequencePage },
  { id: "graph", name: "GraphPage (forge)", layout: "Canvas", page: GraphPage },
] as const;
