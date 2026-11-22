import { SPLIT_MIN, SPLIT_STEP } from "../constants";
import { createSignal, type JSX } from "solid-js";

/**
 * UI · two resizable panes. Drag the handle, or focus it and use ←/→ (↑/↓ when stacked), 5 % per step.
 * `ratio` is the first pane's share; keep it in the page state to remember it.
 */
export function Split(props: {
  first: JSX.Element;
  second: JSX.Element;
  ratio: number;
  onRatio: (r: number) => void;
  /** true = side by side (vertical handle) */
  vertical?: boolean;
  min?: number;
  label?: string;
}) {
  const [dragging, setDragging] = createSignal(false);
  let root!: HTMLDivElement;
  const clamp = (r: number) => Math.min(1 - (props.min ?? SPLIT_MIN), Math.max(props.min ?? SPLIT_MIN, r));
  const move = (e: PointerEvent) => {
    if (!dragging()) return;
    const b = root.getBoundingClientRect();
    props.onRatio(clamp(props.vertical ? (e.clientX - b.left) / b.width : (e.clientY - b.top) / b.height));
  };
  const key = (e: KeyboardEvent) => {
    const back = props.vertical ? "ArrowLeft" : "ArrowUp", fwd = props.vertical ? "ArrowRight" : "ArrowDown";
    if (e.key === back || e.key === fwd) { e.preventDefault(); props.onRatio(clamp(props.ratio + (e.key === fwd ? SPLIT_STEP : -SPLIT_STEP))); }
  };
  return (
    <div class={`q-split ${props.vertical ? "is-vertical" : "is-horizontal"}`} ref={root} onPointerMove={move} onPointerUp={() => setDragging(false)}>
      <div class="q-pane" style={{ flex: `${props.ratio} 1 0` }}>{props.first}</div>
      <div class={`q-split-handle ${props.vertical ? "is-vertical" : "is-horizontal"}${dragging() ? " is-dragging" : ""}`}
        role="separator" aria-orientation={props.vertical ? "vertical" : "horizontal"} aria-label={props.label ?? "Resize"}
        aria-valuenow={Math.round(props.ratio * 100)} aria-valuemin={0} aria-valuemax={100} tabIndex={0}
        onPointerDown={(e) => { setDragging(true); e.currentTarget.setPointerCapture(e.pointerId); }}
        onPointerUp={() => setDragging(false)} onKeyDown={key} />
      <div class="q-pane" style={{ flex: `${1 - props.ratio} 1 0` }}>{props.second}</div>
    </div>
  );
}
