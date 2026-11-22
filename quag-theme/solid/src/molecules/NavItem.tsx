import { Show } from "solid-js";

/** MOLECULE · sidebar item: icon + label + count. */
export function NavItem(props: { label: string; icon?: string; count?: number; active?: boolean; onClick?: () => void }) {
  return (
    <button class="q-nav" aria-current={props.active ? "page" : undefined} onClick={props.onClick}>
      <Show when={props.icon}><span aria-hidden="true">{props.icon}</span></Show>
      {props.label}
      <Show when={props.count !== undefined}><span class="q-count">{props.count}</span></Show>
    </button>
  );
}
