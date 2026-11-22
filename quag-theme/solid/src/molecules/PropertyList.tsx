import { For } from "solid-js";

/** MOLECULE · read-only facts as key / value rows, keys aligned. */
export function PropertyList(props: { items: [string, string][] }) {
  return (
    <dl class="q-props">
      <For each={props.items}>{([k, v]) => <><dt>{k}</dt><dd title={v}>{v}</dd></>}</For>
    </dl>
  );
}
