import { createMemo, createSignal, For, onCleanup, onMount } from "solid-js";
import { Bar, Clip, Curve, fmtNum, NodeFrame, NodePort, NumberField, Playhead, Status, Wire } from "../atoms";
import { CLICK_SLOP_PX, NODE_HEAD_H, NODE_PAD_Y, NODE_ROW_H, NODE_W, RULER_TICK_PX, WEIGHT_STEP } from "../constants";
import { type NodeSpec, nodeHeight, nudgePoint, portPoint, type Pt, type Range, rulerStep, shares, type WireSpec, wirePath } from "../game";

/**
 * MOLECULE · a curve to edit. y labels, the curve, x labels, a readout of the selected key. Drag keys with the mouse.
 * Keys belong to the page's keymap (one per page): bind them to `curveNudge` / `onSelect`.
 */
export function CurveEditor(props: { points: Pt[]; onPoints: (p: Pt[]) => void; x: Range; y: Range; xUnit?: string; yUnit?: string; selected: number; onSelect: (i: number) => void; label?: string }) {
  const sel = () => props.points[props.selected];
  return (
    <div class="q-curve-editor" aria-label={props.label ?? "Curve editor"}>
      <div class="y"><span>{fmtNum(props.y[1], props.yUnit)}</span><span>{fmtNum((props.y[0] + props.y[1]) / 2, props.yUnit)}</span><span>{fmtNum(props.y[0], props.yUnit)}</span></div>
      <Curve points={props.points} x={props.x} y={props.y} selected={props.selected} onSelect={props.onSelect} label={props.label}
        onDrag={(i, x, y) => { const [px, py] = props.points[i]; props.onPoints(nudgePoint(props.points, i, x - px, y - py, props.x, props.y)); }} />
      <div class="x"><span>{fmtNum(props.x[0], props.xUnit)}</span><span>{fmtNum(props.x[1], props.xUnit)}</span></div>
      <div class="readout">
        <b>◆ key {props.selected + 1}/{props.points.length}</b>
        x {fmtNum(sel()?.[0] ?? 0, props.xUnit)} · y {fmtNum(sel()?.[1] ?? 0, props.yUnit)}
      </div>
    </div>
  );
}

/** MOLECULE · weighted choices: raw weight (editable), bar, % of total (always 100 %), odds `1 in N`. */
export function WeightsEditor(props: { items: [string, number][]; onWeight: (i: number, w: number) => void; selected?: number; onSelect?: (i: number) => void }) {
  const sh = createMemo(() => shares(props.items.map((i) => i[1])));
  const top = () => Math.max(...sh(), Number.EPSILON);
  const sum = () => props.items.reduce((a, i) => a + i[1], 0);
  return (
    <div class="q-weights" role="table" aria-label="Weights">
      <For each={props.items}>
        {([name, w], i) => (
          <div class={`row${i() === props.selected ? " is-selected" : ""}`} role="row" onClick={() => props.onSelect?.(i())}>
            <span class="name">{name}</span>
            <Bar fraction={sh()[i()] / top()} />
            <NumberField value={w} min={0} step={WEIGHT_STEP} onValue={(v) => props.onWeight(i(), v)} aria-label={`${name} weight`} />
            <span class="pct">{(sh()[i()] * 100).toFixed(1)} %</span>
            <span class="odds">{sh()[i()] > 0 ? `1 in ${fmtNum(1 / sh()[i()])}` : "never"}</span>
          </div>
        )}
      </For>
      <div class="total">{props.items.length} entries · weights {fmtNum(sum())} · <Status kind={sum() > 0 ? "ok" : "warn"}>{sum() > 0 ? "100 %" : "nothing can drop"}</Status></div>
    </div>
  );
}

export interface ClipSpec { start: number; len: number; label: string }
export interface TrackSpec { name: string; clips: ClipSpec[] }

/** MOLECULE · tracks of clips over time, a ruler (click to move the playhead) and the playhead. */
export function Timeline(props: { tracks: TrackSpec[]; span: Range; playhead: number; onPlayhead?: (t: number) => void; unit?: string; selected?: [number, number]; onSelect?: (t: number, c: number) => void }) {
  const [width, setWidth] = createSignal(800);
  let lanes!: HTMLDivElement;
  onMount(() => { const ro = new ResizeObserver(() => setWidth(lanes.clientWidth)); ro.observe(lanes); onCleanup(() => ro.disconnect()); });
  const len = () => props.span[1] - props.span[0];
  const pct = (t: number) => `${((t - props.span[0]) / len()) * 100}%`;
  const ticks = createMemo(() => {
    const step = rulerStep(len(), width() / RULER_TICK_PX), out: number[] = [];
    for (let t = Math.ceil(props.span[0] / step) * step; t <= props.span[1]; t += step) out.push(t);
    return out;
  });
  const scrub = (e: MouseEvent) => { const r = (e.currentTarget as HTMLElement).getBoundingClientRect(); props.onPlayhead?.(props.span[0] + ((e.clientX - r.left) / r.width) * len()); };
  return (
    <div class="q-timeline">
      <span />
      <div class="ruler" onClick={scrub} aria-label="Ruler: click to move the playhead">
        <For each={ticks()}>{(t) => <span class={t >= props.span[1] ? "end" : undefined} style={{ left: pct(t) }}>{fmtNum(t, props.unit)}</span>}</For>
        <Playhead left={pct(props.playhead)} />
      </div>
      <div style={{ display: "grid", gap: "8px" }}><For each={props.tracks}>{(t) => <span class="name">{t.name}</span>}</For></div>
      <div class="lanes" ref={lanes}>
        <For each={props.tracks}>
          {(tr, ti) => (
            <div class="lane">
              <For each={tr.clips}>
                {(c, ci) => <Clip label={c.label} left={pct(c.start)} width={`${(c.len / len()) * 100}%`} selected={props.selected?.[0] === ti() && props.selected?.[1] === ci()} onClick={() => props.onSelect?.(ti(), ci())} />}
              </For>
            </div>
          )}
        </For>
        <Playhead left={pct(props.playhead)} />
      </div>
    </div>
  );
}

/**
 * MOLECULE · nodes and the wires between them (abilities, dialogue, shaders, AI).
 * Drag a node by its title; drag the background to pan. Wires are drawn under the nodes;
 * the selected node and its wires are Quag. Keys belong to the page (move with arrows: see GraphPage).
 */
export function NodeGraph(props: {
  nodes: NodeSpec[]; positions: Pt[]; wires: WireSpec[]; selected?: number;
  onSelect?: (i: number) => void; onMove?: (i: number, at: Pt) => void; pan?: Pt; onPan?: (p: Pt) => void; label?: string;
}) {
  const pan = () => props.pan ?? [0, 0];
  const [panning, setPanning] = createSignal(false);
  let drag: { kind: "node" | "pan"; i: number; x: number; y: number; from: Pt } | null = null;
  const down = (e: PointerEvent, kind: "node" | "pan", i = -1) => {
    if (e.button !== 0) return;
    e.stopPropagation();
    drag = { kind, i, x: e.clientX, y: e.clientY, from: kind === "node" ? props.positions[i] : pan() };
    if (kind === "node") props.onSelect?.(i);
  };
  const move = (e: PointerEvent) => {
    if (!drag) return;
    const dx = e.clientX - drag.x, dy = e.clientY - drag.y;
    if (drag.kind === "pan") {
      if (Math.abs(dx) + Math.abs(dy) > CLICK_SLOP_PX) setPanning(true);
      props.onPan?.([drag.from[0] - dx, drag.from[1] - dy]);
    } else props.onMove?.(drag.i, [Math.max(0, drag.from[0] + dx), Math.max(0, drag.from[1] + dy)]);
  };
  const up = () => { drag = null; setPanning(false); };
  const wired = (i: number, p: number, out: boolean) => props.wires.some((w) => (out ? w.from[0] === i && w.from[1] === p : w.to[0] === i && w.to[1] === p));
  return (
    <div class={`q-graph${panning() ? " is-panning" : ""}`} aria-label={props.label ?? "Node graph"}
      onPointerDown={(e) => down(e, "pan")} onPointerMove={move} onPointerUp={up} onPointerLeave={up}>
      <div class="world" style={{ transform: `translate(${-pan()[0]}px, ${-pan()[1]}px)` }}>
        <svg width="1" height="1">
          <For each={props.wires}>
            {(w) => <Wire active={props.selected === w.from[0] || props.selected === w.to[0]}
              d={wirePath(portPoint(props.positions[w.from[0]], w.from[1], true), portPoint(props.positions[w.to[0]], w.to[1], false))} />}
          </For>
        </svg>
        <For each={props.nodes}>
          {(n, i) => (
            <NodeFrame title={n.title} selected={props.selected === i()} onHeadDown={(e) => down(e, "node", i())}
              style={{ left: `${props.positions[i()][0]}px`, top: `${props.positions[i()][1]}px`, width: `${NODE_W}px`, height: `${nodeHeight(n)}px` }}
              headStyle={{ height: `${NODE_HEAD_H}px` }}>
              <div class="ports" style={{ padding: `${NODE_PAD_Y}px 0` }} onPointerDown={(e) => { e.stopPropagation(); props.onSelect?.(i()); }}>
                <div><For each={n.inputs}>{(p, j) => <div class="in" style={{ height: `${NODE_ROW_H}px` }}><NodePort wired={wired(i(), j(), false)} />{p}</div>}</For></div>
                <div><For each={n.outputs}>{(p, j) => <div class="out" style={{ height: `${NODE_ROW_H}px` }}>{p}<NodePort wired={wired(i(), j(), true)} /></div>}</For></div>
              </div>
            </NodeFrame>
          )}
        </For>
      </div>
    </div>
  );
}
