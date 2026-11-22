import { For } from "solid-js";
import { Bar, fmtNum } from "../atoms/charts";

/** MOLECULE · compare a few amounts: label · bar · value. Over `budget` = red. */
export function BarChart(props: { items: [string, number][]; unit?: string; max?: number; budget?: number }) {
  const max = () => props.max ?? Math.max(...props.items.map((i) => i[1]), Number.EPSILON);
  return (
    <div class="q-barchart">
      <For each={props.items}>
        {([label, v]) => (
          <>
            <span class="label">{label}</span>
            <Bar fraction={v / max()} over={props.budget !== undefined && v > props.budget} />
            <span class="value">{fmtNum(v, props.unit)}</span>
          </>
        )}
      </For>
    </div>
  );
}
