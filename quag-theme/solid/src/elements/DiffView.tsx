import { For, Show } from "solid-js";
import { type DiffKind, DiffLine, Text } from "../atoms";
import { EmptyState } from "./EmptyState";

/** ELEMENT (shared) · review a change before applying it: `+4 −3` then the lines. No changes → empty state. */
export function DiffView(props: { title: string; lines: [DiffKind, string][] }) {
  const add = () => props.lines.filter((l) => l[0] === "add").length;
  const del = () => props.lines.filter((l) => l[0] === "del").length;
  return (
    <section class="q-diff" aria-label={props.title}>
      <header><h3>{props.title}</h3><Text tone="ok">+{add()}</Text><Text tone="err">−{del()}</Text></header>
      <Show when={add() + del() > 0} fallback={<EmptyState message="Nothing to apply: no changes." />}>
        <div class="lines"><For each={props.lines}>{([k, t]) => <DiffLine kind={k} text={t} />}</For></div>
      </Show>
    </section>
  );
}
