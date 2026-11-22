import { For, type JSX } from "solid-js";

/** A cell: text, or an atom (state, sparkline, meter) for elements that need one. */
export type Cell = string | JSX.Element;

export interface Column {
  header: string;
  /** numbers: right-aligned, tabular */
  num?: boolean;
  /** CSS width, e.g. "80px". Leave one column without it to take the rest. */
  width?: string;
}

/** MOLECULE · rows and columns with a header line. Cells end with … when too long. */
export function Table(props: { columns: Column[]; rows: Cell[][]; selected?: number; onSelect?: (i: number) => void; label?: string }) {
  return (
    <table class="q-table" aria-label={props.label} style={{ "table-layout": "fixed" }}>
      <thead>
        <tr><For each={props.columns}>{(c) => <th class={c.num ? "num" : undefined} style={{ width: c.width }}>{c.header}</th>}</For></tr>
      </thead>
      <tbody>
        <For each={props.rows}>
          {(r, i) => (
            <tr class={i() === props.selected ? "is-selected" : undefined} aria-selected={i() === props.selected} onClick={() => props.onSelect?.(i())}>
              <For each={r}>{(cell, c) => <td class={props.columns[c()]?.num ? "num" : undefined} title={typeof cell === "string" ? cell : undefined}>{cell}</td>}</For>
            </tr>
          )}
        </For>
      </tbody>
    </table>
  );
}
