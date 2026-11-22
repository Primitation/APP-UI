import { For } from "solid-js";
import { CURVE_KEY_R, CURVE_KEY_R_SELECTED, CURVE_VIEW_H, CURVE_VIEW_W } from "../constants";
import type { JSX } from "solid-js";
import type { Pt, Range } from "../game";

/**
 * ATOM · a curve through key points: Quag line, keys, the selected key in Gill. Drag a key to move it
 * (the app decides where it may go: pass `onDrag` the clamped result of `nudgePoint`).
 */
export function Curve(props: { points: Pt[]; x: Range; y: Range; selected?: number; onSelect?: (i: number) => void; onDrag?: (i: number, x: number, y: number) => void; label?: string }) {
  const sx = (v: number) => ((v - props.x[0]) / (props.x[1] - props.x[0])) * CURVE_VIEW_W;
  const sy = (v: number) => CURVE_VIEW_H - ((v - props.y[0]) / (props.y[1] - props.y[0])) * CURVE_VIEW_H;
  let svg!: SVGSVGElement;
  const toValue = (e: PointerEvent): Pt => {
    const r = svg.getBoundingClientRect();
    return [props.x[0] + ((e.clientX - r.left) / r.width) * (props.x[1] - props.x[0]), props.y[1] - ((e.clientY - r.top) / r.height) * (props.y[1] - props.y[0])];
  };
  let dragging = -1;
  // the line is an SVG stretched to the box; the keys are HTML on top, so they stay round at any size
  const size = (sel: boolean) => `${2 * (sel ? CURVE_KEY_R_SELECTED : CURVE_KEY_R)}px`;
  return (
    <div class="q-curve" role="img" aria-label={props.label ?? "curve"}
      onPointerMove={(e) => { if (dragging >= 0) { const [x, y] = toValue(e); props.onDrag?.(dragging, x, y); } }}
      onPointerUp={() => (dragging = -1)} onPointerLeave={() => (dragging = -1)}>
      <svg ref={svg} viewBox={`0 0 ${CURVE_VIEW_W} ${CURVE_VIEW_H}`} preserveAspectRatio="none">
        <line class="grid" x1={0} x2={CURVE_VIEW_W} y1={CURVE_VIEW_H / 2} y2={CURVE_VIEW_H / 2} vector-effect="non-scaling-stroke" />
        <line class="base" x1={0} x2={CURVE_VIEW_W} y1={CURVE_VIEW_H} y2={CURVE_VIEW_H} vector-effect="non-scaling-stroke" />
        <polyline class="line" points={props.points.map(([x, y]) => `${sx(x)},${sy(y)}`).join(" ")} />
      </svg>
      <For each={props.points}>
        {([x, y], i) => (
          <span class={`key${i() === props.selected ? " is-selected" : ""}`}
            style={{ left: `${(sx(x) / CURVE_VIEW_W) * 100}%`, top: `${(sy(y) / CURVE_VIEW_H) * 100}%`, width: size(i() === props.selected), height: size(i() === props.selected) }}
            onPointerDown={(e) => { e.preventDefault(); dragging = i(); props.onSelect?.(i()); }} />
        )}
      </For>
    </div>
  );
}

/** ATOM · one clip on a timeline (its position is set by the Timeline). Selected = Quag fill. */
export function Clip(props: { label: string; selected?: boolean; left: string; width: string; onClick?: () => void }) {
  return <div class={`q-clip${props.selected ? " is-selected" : ""}`} style={{ left: props.left, width: props.width }} title={props.label} onClick={props.onClick} role="button" aria-pressed={props.selected}>{props.label}</div>;
}

/** ATOM · the playhead: a Gill line with a head, over clips. */
export function Playhead(props: { left: string }) {
  return <div class="q-playhead" style={{ left: props.left }} aria-hidden="true" />;
}

/** ATOM · a node's frame: raised card, title row (drag handle); Quag border when selected. Size comes from the graph. */
export function NodeFrame(props: { title: string; selected?: boolean; style: JSX.CSSProperties; headStyle: JSX.CSSProperties; onHeadDown?: (e: PointerEvent) => void; onClick?: () => void; children?: JSX.Element }) {
  return (
    <div class={`q-node${props.selected ? " is-selected" : ""}`} style={props.style} onClick={props.onClick} role="group" aria-label={props.title}>
      <div class="head" style={props.headStyle} onPointerDown={(e) => props.onHeadDown?.(e)}>{props.title}</div>
      {props.children}
    </div>
  );
}

/** ATOM · a port on a node's edge: filled when wired. */
export function NodePort(props: { wired?: boolean }) {
  return <span class={`q-node-port${props.wired ? " is-wired" : ""}`} aria-hidden="true" />;
}

/** ATOM · a wire (SVG path): faint, Quag when it touches the selected node. */
export function Wire(props: { d: string; active?: boolean }) {
  return <path class={`q-wire${props.active ? " is-active" : ""}`} d={props.d} />;
}
