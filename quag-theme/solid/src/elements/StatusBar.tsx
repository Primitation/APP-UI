import { For, type JSX, Show } from "solid-js";
import { Kbd, Status, type StatusKind } from "../atoms/small";

/** ELEMENT (shared) · bottom bar: key hints left, last result right. */
export function StatusBar(props: { hints?: [string, string][]; status?: { kind: StatusKind; text: string }; children?: JSX.Element }) {
  return (
    <footer class="q-statusbar">
      <For each={props.hints ?? []}>{([k, label]) => <span><Kbd>{k}</Kbd> {label}</span>}</For>
      {props.children}
      <Show when={props.status}>
        {(s) => <span style={{ "margin-left": "auto" }}><Status kind={s().kind}>{s().text}</Status></span>}
      </Show>
    </footer>
  );
}
