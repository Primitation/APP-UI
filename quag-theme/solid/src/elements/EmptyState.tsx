import { type JSX, Show } from "solid-js";

export interface EmptyStateProps {
  message: string;
  hint?: JSX.Element;
  /** URL of the mascot GIF. PRIVATE apps only (DESIGN.md §6); leave unset in public apps. */
  mascot?: string;
  action?: JSX.Element;
}

/** ELEMENT (shared) · the "nothing here" state of any content zone. */
export function EmptyState(props: EmptyStateProps) {
  return (
    <div class="q-empty">
      <Show when={props.mascot}><img class="q-mascot" src={props.mascot} alt="" /></Show>
      <div class="q-t-text" style={{ "font-weight": 600 }}>{props.message}</div>
      <Show when={props.hint}><div>{props.hint}</div></Show>
      <Show when={props.action}><div style={{ "margin-top": "8px" }}>{props.action}</div></Show>
    </div>
  );
}
