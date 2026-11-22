/** ELEMENTS of the forge demo (game design data): molecules wired to its data. They never set colors. */
import { For, type JSX, Match, Show, Switch } from "solid-js";
import {
  BarChart, ButtonGroup, Card, CurveEditor, DeltaBadge, EmptyState, ErrorState, fmtNum, List, ListRow, NodeGraph, PropertyList, type Pt, shares,
  Skeleton, Status, Table, Timeline, WeightsEditor,
} from "../../src";
import { OUTLIER_RATIO } from "../consts";
import { type CurveAsset, GRAPH, type Load, median, NODE_PARAMS, SEQ_SPAN, SEQUENCE, UNITS, value, WIRES } from "./data";

/** Loading → skeleton rows; failed → ErrorState (what + why + Retry); ready → the content. Same for every forge panel. */
function Loaded(props: { load?: Load; file: string; onRetry?: () => void; children: JSX.Element }) {
  return (
    <Switch fallback={props.children}>
      <Match when={props.load === "loading"}><div style={{ display: "grid", gap: "14px", padding: "6px 0" }}><For each={[0.9, 0.7, 0.8, 0.6]}>{(w) => <Skeleton width={`${w * 100}%`} />}</For></div></Match>
      <Match when={props.load === "failed"}><ErrorState what={`Can't read ${props.file}.`} why="line 14: expected 4 columns, found 3" onRetry={props.onRetry} /></Match>
    </Switch>
  );
}

/** AssetList: the curves / tables of the project. */
export function AssetList(props: { title: string; names: string[]; selected: number; onSelect: (i: number) => void }) {
  return (
    <div style={{ padding: "4px" }}>
      <h3 style={{ margin: "0 0 10px" }}>{props.title}</h3>
      <List label={props.title}><For each={props.names}>{(n, i) => <ListRow title={n} selected={i() === props.selected} onClick={() => props.onSelect(i())} />}</For></List>
    </div>
  );
}

/** CurvePanel: the curve being edited. */
export function CurvePanel(props: { load?: Load; asset: CurveAsset; points: Pt[]; onPoints: (p: Pt[]) => void; selected: number; onSelect: (i: number) => void }) {
  return (
    <Card style={{ display: "grid", "grid-template-rows": "auto minmax(0, 1fr)", gap: "12px", height: "100%" }}>
      <div><h3 style={{ margin: 0 }}>Curve · {props.asset.name}</h3><span style={{ color: "var(--q-muted)" }}>{props.asset.what}</span></div>
      <Loaded load={props.load} file="data/curves.csv">
        <CurveEditor points={props.points} onPoints={props.onPoints} x={props.asset.x} y={props.asset.y} xUnit={props.asset.xUnit} yUnit={props.asset.yUnit}
          selected={props.selected} onSelect={props.onSelect} label={props.asset.name} />
      </Loaded>
    </Card>
  );
}

/** KeyInspector: the selected key, its slope to the next, add / delete. */
export function KeyInspector(props: { asset: CurveAsset; points: Pt[]; selected: number; onAdd: () => void; onDelete: () => void }) {
  const p = () => props.points[props.selected];
  const slope = () => {
    const a = props.points[props.selected], b = props.points[props.selected + 1];
    if (!a || !b) return "last key";
    const r = (b[1] - a[1]) / Math.max(b[0] - a[0], Number.EPSILON);
    return `${r >= 0 ? "▲" : "▼"} ${fmtNum(Math.abs(r), props.asset.yUnit)} per ${props.asset.xUnit || "step"}`;
  };
  const edge = () => props.selected === 0 || props.selected === props.points.length - 1;
  return (
    <Show when={p()} fallback={<EmptyState message="No key selected." />}>
      {(k) => (
        <div style={{ display: "grid", gap: "16px", padding: "4px" }}>
          <h3 style={{ margin: 0 }}>Key</h3>
          <PropertyList items={[["Key", `${props.selected + 1} of ${props.points.length}`], ["x", fmtNum(k()[0], props.asset.xUnit)], ["y", fmtNum(k()[1], props.asset.yUnit)], ["Slope to next", slope()]]} />
          <ButtonGroup buttons={[
            { label: "Delete key", kind: "danger", onClick: props.onDelete, disabled: edge(), title: edge() ? "The first and last keys hold the range ends" : undefined },
            { label: "Add key", onClick: props.onAdd, disabled: props.selected === props.points.length - 1 },
          ]} />
        </div>
      )}
    </Show>
  );
}

/** LootPanel: one drop table's weights. Empty → says so. */
export function LootPanel(props: { load?: Load; name: string; items: [string, number][]; selected: number; onSelect: (i: number) => void; onWeight: (i: number, w: number) => void }) {
  return (
    <Card>
      <h3 style={{ margin: "0 0 14px" }}>Drops · {props.name}</h3>
      <Loaded load={props.load} file="data/loot.csv">
        <Show when={props.items.length} fallback={<EmptyState message="This table drops nothing." />}>
          <WeightsEditor items={props.items} selected={props.selected} onSelect={props.onSelect} onWeight={props.onWeight} />
        </Show>
      </Loaded>
    </Card>
  );
}

/** PerHundred: what 100 kills give, on average. */
export function PerHundred(props: { items: [string, number][] }) {
  const rows = () => { const s = shares(props.items.map((i) => i[1])); return props.items.map(([n], i) => [n, Math.round(s[i] * 1000) / 10] as [string, number]); };
  return <div style={{ padding: "4px" }}><h3 style={{ margin: "0 0 12px" }}>Per 100 kills</h3><BarChart items={rows()} /></div>;
}

/** BalanceKpis: the roster's numbers. */
export function BalanceKpis(props: { load?: Load }) {
  const [strong] = outliers();
  // no numbers until the data is there
  const kpi = (v: string, l: string) => <Card class="q-kpi"><div class="q-kpi-value">{(props.load ?? "ready") === "ready" ? v : "–"}</div><div class="q-kpi-label">{l}</div></Card>;
  return <div class="q-kpis">{kpi(String(UNITS.length), "units")}{kpi(String(UNITS.filter((u) => u.now.join() !== u.prev.join()).length), "changed this patch")}{kpi(median().toFixed(1), "median DPS / 100 gold")}{kpi(String(strong), "strong outliers")}</div>;
}

/** (strong, weak) outliers by DPS per gold. */
export const outliers = (): [number, number] => {
  const m = median(), v = UNITS.map((u) => value(u.now));
  return [v.filter((x) => x > m * OUTLIER_RATIO).length, v.filter((x) => x < m / OUTLIER_RATIO).length];
};

/** BalanceTable: units vs last patch; deltas know which way is good; outliers marked (warn / info, not errors). */
export function BalanceTable(props: { selected: number; onSelect: (i: number) => void; load?: Load; onRetry?: () => void }) {
  const m = median();
  const stat = (now: number, prev: number, higher: boolean) => (
    <span style={{ display: "inline-flex", gap: "10px", "align-items": "baseline" }}>{fmtNum(now)}<Show when={Math.abs(now / prev - 1) >= 0.0005}><DeltaBadge change={now / prev - 1} higherIsBetter={higher} /></Show></span>
  );
  return (
    <Card>
      <h3 style={{ margin: "0 0 12px" }}>Units · vs last patch</h3>
      <Loaded load={props.load} file="data/units.csv" onRetry={props.onRetry}><Table label="Units" selected={props.selected} onSelect={props.onSelect}
        columns={[{ header: "Unit" }, { header: "Class", width: "100px" }, { header: "HP", width: "130px" }, { header: "DPS", width: "130px" }, { header: "Cost", width: "130px" }, { header: "DPS/100g", num: true, width: "90px" }, { header: "", width: "90px" }]}
        rows={UNITS.map((u) => {
          const v = value(u.now);
          return [u.name, u.cls, stat(u.now[0], u.prev[0], true), stat(u.now[1], u.prev[1], true), stat(u.now[2], u.prev[2], false), v.toFixed(1),
            v > m * OUTLIER_RATIO ? <Status kind="warn">strong</Status> : v < m / OUTLIER_RATIO ? <Status kind="info">weak</Status> : ""];
        })} /></Loaded>
    </Card>
  );
}

/** SequencePanel: the cutscene's tracks. */
export function SequencePanel(props: { playhead: number; onPlayhead: (t: number) => void; playing: boolean; selected: [number, number]; onSelect: (t: number, c: number) => void }) {
  return (
    <Card>
      <h3 style={{ margin: "0 0 14px" }}>Sequence · boss_intro <span style={{ color: "var(--q-muted)", "font-weight": 400, "margin-left": "12px" }}>{props.playing ? "▶" : "‖"} {fmtNum(props.playhead, "s")}</span></h3>
      <Timeline tracks={SEQUENCE} span={SEQ_SPAN} playhead={props.playhead} onPlayhead={props.onPlayhead} unit="s" selected={props.selected} onSelect={props.onSelect} />
    </Card>
  );
}

/** ClipInspector: the selected clip. */
export function ClipInspector(props: { selected: [number, number] }) {
  const tr = () => SEQUENCE[props.selected[0]], c = () => tr()?.clips[props.selected[1]];
  return (
    <Show when={c()} fallback={<EmptyState message="Select a clip." />}>
      {(c) => (
        <div style={{ display: "grid", gap: "12px", padding: "4px" }}>
          <h3 style={{ margin: 0 }}>Clip</h3>
          <PropertyList items={[["Track", tr().name], ["Clip", c().label], ["Start", fmtNum(c().start, "s")], ["Length", fmtNum(c().len, "s")], ["End", fmtNum(c().start + c().len, "s")]]} />
        </div>
      )}
    </Show>
  );
}

/** GraphPanel: the ability's nodes and wires. */
export function GraphPanel(props: { positions: Pt[]; onMove: (i: number, at: Pt) => void; selected: number; onSelect: (i: number) => void; pan: Pt; onPan: (p: Pt) => void; load?: Load }) {
  return (
    <Card style={{ display: "grid", "grid-template-rows": "auto minmax(0, 1fr)", gap: "12px", height: "100%" }}>
      <h3 style={{ margin: 0 }}>Ability · fireball</h3>
      <Loaded load={props.load} file="abilities/fireball.graph">
        <NodeGraph nodes={GRAPH} positions={props.positions} wires={WIRES} selected={props.selected} onSelect={props.onSelect} onMove={props.onMove} pan={props.pan} onPan={props.onPan} label="fireball" />
      </Loaded>
    </Card>
  );
}

/** NodeInspector: the selected node's settings and what it is wired to. */
export function NodeInspector(props: { selected: number }) {
  const n = () => GRAPH[props.selected];
  const links = () => WIRES.flatMap((w) => [
    ...(w.to[0] === props.selected ? [`${n().inputs[w.to[1]]} ← ${GRAPH[w.from[0]].title}`] : []),
    ...(w.from[0] === props.selected ? [`${n().outputs[w.from[1]]} → ${GRAPH[w.to[0]].title}`] : []),
  ]);
  return (
    <Show when={n()} fallback={<EmptyState message="Select a node." />}>
      {(node) => (
        <div style={{ display: "grid", gap: "14px", padding: "4px" }}>
          <h3 style={{ margin: 0 }}>{node().title}</h3>
          <PropertyList items={NODE_PARAMS[props.selected]} />
          <div><div style={{ color: "var(--q-muted)", "margin-bottom": "6px" }}>Wired to</div><For each={links()}>{(l) => <div>{l}</div>}</For></div>
        </div>
      )}
    </Show>
  );
}
