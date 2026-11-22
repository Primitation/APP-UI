/** Pure helpers for the game design parts (same rules as the Rust molecules: nudge_point, shares, ruler_step). */
import { NODE_HEAD_H, NODE_PAD_Y, NODE_ROW_H, NODE_W, NUDGE, NUDGE_FINE, WIRE_BEND_MIN } from "./constants";

export type Pt = [number, number];
export type Range = [number, number];

/** Moves key i by (dx, dy): stays between its neighbours on x and inside the ranges; first and last keep their x. */
export function nudgePoint(points: Pt[], i: number, dx: number, dy: number, x: Range, y: Range): Pt[] {
  if (i < 0 || i >= points.length) return points;
  const lo = i === 0 ? x[0] : points[i - 1][0];
  const hi = i === points.length - 1 ? x[1] : points[i + 1][0];
  const edge = i === 0 || i === points.length - 1;
  return points.map((p, j) => (j !== i ? p : [edge ? p[0] : Math.min(hi, Math.max(lo, p[0] + dx)), Math.min(y[1], Math.max(y[0], p[1] + dy))]));
}

/** What an arrow key (key name from Keymap.nameOf) does to the selected key: NUDGE of the range, Shift = NUDGE_FINE. */
export function curveNudge(points: Pt[], i: number, key: string, x: Range, y: Range): Pt[] {
  const share = key.startsWith("shift+") ? NUDGE_FINE : NUDGE;
  const w = (x[1] - x[0]) * share, h = (y[1] - y[0]) * share;
  const d: Record<string, [number, number]> = { ArrowLeft: [-w, 0], ArrowRight: [w, 0], ArrowUp: [0, h], ArrowDown: [0, -h] };
  const [dx, dy] = d[key.replace("shift+", "")] ?? [0, 0];
  return nudgePoint(points, i, dx, dy, x, y);
}

/** Share of each weight in the total (0..1). All zero → all zero. */
export function shares(weights: number[]): number[] {
  const t = weights.reduce((a, b) => a + b, 0);
  return weights.map((w) => (t > 0 ? w / t : 0));
}

/** A "nice" ruler step (1, 2, 5 × 10ⁿ) for `span` over `ticks` labels. */
export function rulerStep(span: number, ticks: number): number {
  const raw = span / Math.max(ticks, 1);
  const p = 10 ** Math.floor(Math.log10(raw));
  return [1, 2, 5, 10].map((m) => m * p).find((s) => s >= raw) ?? 10 * p;
}

/** Node graphs: a node, a wire (node, output port) → (node, input port), and where a port sits. */
export interface NodeSpec { title: string; inputs: string[]; outputs: string[] }
export interface WireSpec { from: [number, number]; to: [number, number] }

/** Outer height of a node (px). */
export const nodeHeight = (n: NodeSpec) => NODE_HEAD_H + 2 * NODE_PAD_Y + Math.max(n.inputs.length, n.outputs.length, 1) * NODE_ROW_H;

/** The centre of port `p` on a node at `at` (px): left edge for inputs, right edge for outputs. */
export function portPoint(at: Pt, p: number, output: boolean): Pt {
  return [at[0] + (output ? NODE_W : 0), at[1] + NODE_HEAD_H + NODE_PAD_Y + p * NODE_ROW_H + NODE_ROW_H / 2];
}

/** An SVG path for a wire: leaves the output going right, enters the input from the left. */
export function wirePath(a: Pt, b: Pt): string {
  const d = Math.max(WIRE_BEND_MIN, Math.abs(b[0] - a[0]) / 2);
  return `M ${a[0]} ${a[1]} C ${a[0] + d} ${a[1]}, ${b[0] - d} ${b[1]}, ${b[0]} ${b[1]}`;
}
