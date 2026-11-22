import { HISTOGRAM_H, HISTOGRAM_W, NUM_NO_DECIMALS_FROM, NUM_ONE_DECIMAL_FROM, SPARK_DOT_R, SPARK_PAD, SPARKLINE_H, SPARKLINE_W } from "../constants";
import { For, Show } from "solid-js";

export interface ChartProps {
  values: number[];
  /** Top of the scale (default: the biggest value). */
  max?: number;
  /** Values above it are red; a dashed line marks it. */
  budget?: number;
  width?: number;
  height?: number;
  label?: string;
}

/** ATOM · vertical bars: frame times, distributions, activity. */
export function Histogram(props: ChartProps) {
  const w = () => props.width ?? HISTOGRAM_W;
  const h = () => props.height ?? HISTOGRAM_H;
  const max = () => props.max ?? Math.max(...props.values, Number.EPSILON);
  const bw = () => w() / Math.max(props.values.length, 1);
  const y = (v: number) => h() - (Math.min(v, max()) / max()) * h();
  return (
    <svg class="q-chart" width={w()} height={h()} role="img" aria-label={props.label ?? "chart"}>
      <For each={props.values}>
        {(v, i) => <rect class={`bar${props.budget !== undefined && v > props.budget ? " over" : ""}`} x={i() * bw() + 0.5} width={Math.max(bw() - 1, 1)} y={y(v)} height={h() - y(v)} rx={1} />}
      </For>
      <Show when={props.budget !== undefined && props.budget < max()}>
        <line class="budget" x1={0} x2={w()} y1={y(props.budget!)} y2={y(props.budget!)} />
      </Show>
    </svg>
  );
}

/** ATOM · a small trend line (one row of text tall). Over budget = red dot at the end. */
export function Sparkline(props: ChartProps) {
  const w = () => props.width ?? SPARKLINE_W;
  const h = () => props.height ?? SPARKLINE_H;
  const max = () => props.max ?? Math.max(...props.values, Number.EPSILON);
  /** y of a value: inside the box, SPARK_PAD from the top and bottom edges. */
  const yOf = (v: number) => h() - (Math.min(v, max()) / max()) * (h() - 2 * SPARK_PAD) - SPARK_PAD;
  const pts = () => props.values.map((v, i) => `${(i / Math.max(props.values.length - 1, 1)) * w()},${yOf(v)}`).join(" ");
  const last = () => props.values[props.values.length - 1] ?? 0;
  const over = () => props.budget !== undefined && last() > props.budget;
  return (
    <svg class="q-chart" width={w()} height={h()} role="img" aria-label={props.label ?? `trend, last ${last()}`}>
      <Show when={props.budget !== undefined && props.budget < max()}>
        <line class="budget" x1={0} x2={w()} y1={yOf(props.budget!)} y2={yOf(props.budget!)} />
      </Show>
      <polyline class="line" points={pts()} />
      <circle class={`bar${over() ? " over" : ""}`} cx={w()} cy={yOf(last())} r={SPARK_DOT_R} />
    </svg>
  );
}

/** ATOM · one horizontal bar on a track (bar charts, budgets). */
export function Bar(props: { fraction: number; over?: boolean }) {
  return <div class={`q-bar${props.over ? " over" : ""}`}><span style={{ width: `${Math.max(0, Math.min(1, props.fraction)) * 100}%` }} /></div>;
}

/** ATOM · change between two measures. Green when it went the good way. */
export function DeltaBadge(props: { change: number; higherIsBetter: boolean }) {
  const kind = () => (Math.abs(props.change) < 0.0005 ? "same" : (props.change > 0) === props.higherIsBetter ? "good" : "bad");
  const text = () => (kind() === "same" ? "= 0%" : `${props.change > 0 ? "▲" : "▼"} ${props.change > 0 ? "+" : ""}${(props.change * 100).toFixed(1)}%`);
  return <span class={`q-delta ${kind()}`}>{text()}</span>;
}

/** Number + unit the house way: ≤ 3 significant digits, trailing zeros dropped. */
export function fmtNum(v: number, unit = ""): string {
  const n = Math.abs(v) >= NUM_NO_DECIMALS_FROM ? v.toFixed(0) : Math.abs(v) >= NUM_ONE_DECIMAL_FROM ? v.toFixed(1) : v.toFixed(2);
  const t = n.includes(".") ? n.replace(/0+$/, "").replace(/\.$/, "") : n;
  return unit ? `${t} ${unit}` : t;
}

/** ATOM · a search match inside text. */
export function Mark(props: { text: string; query: string }) {
  const i = () => (props.query ? props.text.toLowerCase().indexOf(props.query.toLowerCase()) : -1);
  return (
    <Show when={i() >= 0} fallback={props.text}>
      {props.text.slice(0, i())}<mark>{props.text.slice(i(), i() + props.query.length)}</mark>{props.text.slice(i() + props.query.length)}
    </Show>
  );
}
