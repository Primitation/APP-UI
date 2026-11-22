import { CLICK_SLOP_PX, DRAG_PX_PER_STEP, SHIFT_STEP_MULT } from "../constants";
import { createSignal } from "solid-js";

export interface NumberFieldProps {
  value: number;
  onValue?: (v: number) => void;
  unit?: string;
  step?: number;
  min?: number;
  max?: number;
  decimals?: number;
  invalid?: boolean;
  disabled?: boolean;
  id?: string;
  "aria-label"?: string;
}

/**
 * ATOM · a number with its unit. ◂ ▸ buttons, ↑/↓ keys (Shift = ×10), mouse wheel,
 * and drag left/right on the field (the tech-art way). Typing also works.
 */
export function NumberField(props: NumberFieldProps) {
  const step = () => props.step ?? 1;
  const clamp = (v: number) => Math.min(props.max ?? Infinity, Math.max(props.min ?? -Infinity, v));
  const round = (v: number) => Number(v.toFixed(props.decimals ?? 0));
  const set = (v: number) => props.onValue?.(round(clamp(v)));
  const [drag, setDrag] = createSignal<{ x: number; v: number } | null>(null);
  const text = () => props.value.toFixed(props.decimals ?? 0);
  return (
    <span class={`q-number${props.invalid ? " is-invalid" : ""}`}>
      <button type="button" tabIndex={-1} aria-label="Decrease" disabled={props.disabled || props.value <= (props.min ?? -Infinity)} onClick={() => set(props.value - step())}>◂</button>
      <input
        id={props.id}
        inputMode="decimal"
        aria-label={props["aria-label"]}
        aria-invalid={props.invalid}
        disabled={props.disabled}
        value={text()}
        onChange={(e) => { const v = parseFloat(e.currentTarget.value); if (!Number.isNaN(v)) set(v); else e.currentTarget.value = text(); }}
        onKeyDown={(e) => {
          if (e.key === "ArrowUp" || e.key === "ArrowDown") {
            e.preventDefault();
            set(props.value + (e.key === "ArrowUp" ? 1 : -1) * step() * (e.shiftKey ? SHIFT_STEP_MULT : 1));
          }
        }}
        onWheel={(e) => { if (document.activeElement === e.currentTarget) { e.preventDefault(); set(props.value + (e.deltaY < 0 ? step() : -step())); } }}
        onPointerDown={(e) => { if (document.activeElement !== e.currentTarget) { setDrag({ x: e.clientX, v: props.value }); e.currentTarget.setPointerCapture(e.pointerId); } }}
        onPointerMove={(e) => { const d = drag(); if (d) set(d.v + Math.round((e.clientX - d.x) / DRAG_PX_PER_STEP) * step()); }}
        onPointerUp={(e) => { const d = drag(); setDrag(null); if (d && Math.abs(e.clientX - d.x) < CLICK_SLOP_PX) e.currentTarget.focus(); }}
      />
      {props.unit && <span class="q-unit">{props.unit}</span>}
      <button type="button" tabIndex={-1} aria-label="Increase" disabled={props.disabled || props.value >= (props.max ?? Infinity)} onClick={() => set(props.value + step())}>▸</button>
    </span>
  );
}

/** ATOM · a value between min and max. Put the number next to it (NumberField). */
export function Slider(props: { value: number; onValue?: (v: number) => void; min: number; max: number; step?: number; disabled?: boolean; "aria-label"?: string }) {
  const fill = () => `${((props.value - props.min) / (props.max - props.min || 1)) * 100}%`;
  return (
    <input type="range" class="q-slider" style={{ "--fill": fill() }} min={props.min} max={props.max} step={props.step ?? "any"}
      value={props.value} disabled={props.disabled} aria-label={props["aria-label"]}
      onInput={(e) => props.onValue?.(parseFloat(e.currentTarget.value))} />
  );
}

/** ATOM · a dropdown. A real <select>: keyboard, screen readers and the OS list for free. */
export function Select(props: { value: string; options: string[]; onValue?: (v: string) => void; id?: string; disabled?: boolean; "aria-label"?: string }) {
  return (
    <select class="q-select" id={props.id} disabled={props.disabled} aria-label={props["aria-label"]} onChange={(e) => props.onValue?.(e.currentTarget.value)}>
      {props.options.map((o) => <option value={o} selected={o === props.value}>{o}</option>)}
    </select>
  );
}
