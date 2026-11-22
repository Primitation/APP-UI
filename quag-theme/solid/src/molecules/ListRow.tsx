import { For, type JSX, Show } from "solid-js";
import { Status, type StatusKind, Tag } from "../atoms/small";

export interface ListRowProps {
  title: JSX.Element;
  /** undefined = no checkbox column; true/false = done/open circle */
  check?: boolean;
  tags?: string[];
  status?: StatusKind;
  meta?: string;
  metaAccent?: boolean;
  selected?: boolean;
  onClick?: () => void;
}

/** MOLECULE · one list row: check + title on the left; tags, status, meta right-aligned. */
export function ListRow(props: ListRowProps) {
  const cls = () => `q-row${props.selected ? " is-selected" : ""}${props.check ? " is-done" : ""}`;
  return (
    <li class={cls()} onClick={props.onClick} aria-selected={props.selected}>
      <Show when={props.check !== undefined}>
        <span class={`q-row-check${props.check ? " on" : ""}`} aria-hidden="true" />
      </Show>
      <span class="q-row-title">{props.title}</span>
      <For each={props.tags ?? []}>{(t) => <Tag>{t}</Tag>}</For>
      <span class="q-row-status"><Show when={props.status}>{(s) => <Status kind={s()} />}</Show></span>
      <Show when={props.meta}>
        <span class={`q-row-meta${props.metaAccent ? " accent" : ""}`}>{props.meta}</span>
      </Show>
    </li>
  );
}

/** MOLECULE container for ListRows. */
export function List(props: { children: JSX.Element; label?: string }) {
  return <ul class="q-list" role="listbox" aria-label={props.label}>{props.children}</ul>;
}
