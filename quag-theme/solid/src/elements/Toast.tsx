import { TOAST_MS } from "../constants";
import { onCleanup, onMount, Show } from "solid-js";
import { Portal } from "solid-js/web";
import { Status, type StatusKind } from "../atoms";

/** ELEMENT (shared) · a result to notice, bottom-right. Goes away by itself after `ms` (default 4 s). */
export function Toast(props: { open: boolean; kind: StatusKind; message: string; ms?: number; onClose: () => void }) {
  return (
    <Show when={props.open}>
      <Portal><ToastBody {...props} /></Portal>
    </Show>
  );
}

function ToastBody(props: { kind: StatusKind; message: string; ms?: number; onClose: () => void }) {
  let t: number | undefined;
  onMount(() => (t = window.setTimeout(props.onClose, props.ms ?? TOAST_MS)));
  onCleanup(() => clearTimeout(t));
  return <div class="q-toast" role="status"><Status kind={props.kind}>{props.message}</Status></div>;
}
