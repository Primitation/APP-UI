import { createSignal, onCleanup, onMount, Show } from "solid-js";
import { Portal } from "solid-js/web";
import { Input } from "../atoms/Input";
import { Kbd } from "../atoms/small";
import { ButtonGroup } from "../molecules/ButtonGroup";

/**
 * ELEMENT (shared) · for actions that can't be undone on something important (delete a server, revoke a key):
 * the user types its name; the danger button stays disabled until it matches. Esc cancels.
 */
export function TypeToConfirm(props: { open: boolean; title: string; message: string; name: string; confirmLabel: string; onConfirm: () => void; onCancel: () => void }) {
  return (
    <Show when={props.open}>
      <Portal><Body {...props} /></Portal>
    </Show>
  );
}

function Body(props: { title: string; message: string; name: string; confirmLabel: string; onConfirm: () => void; onCancel: () => void }) {
  const [typed, setTyped] = createSignal("");
  let root!: HTMLDivElement;
  const ok = () => typed() === props.name;
  const onKey = (e: KeyboardEvent) => {
    if (e.key === "Escape") { e.preventDefault(); props.onCancel(); }
    if (e.key === "Enter" && ok()) { e.preventDefault(); props.onConfirm(); }
  };
  onMount(() => { document.addEventListener("keydown", onKey); root.querySelector("input")?.focus(); });
  onCleanup(() => document.removeEventListener("keydown", onKey));
  return (
    <div class="q-backdrop" onClick={(e) => e.target === e.currentTarget && props.onCancel()}>
      <div class="q-dialog" role="alertdialog" aria-modal="true" aria-label={props.title} ref={root}>
        <h2>{props.title}</h2>
        <p class="q-t-muted" style={{ margin: "0 0 16px" }}>{props.message}</p>
        <label class="q-label">Type <Kbd>{props.name}</Kbd> to confirm</label>
        <Input value={typed()} onValue={setTyped} invalid={typed() !== "" && !props.name.startsWith(typed())} aria-label={`Type ${props.name} to confirm`} />
        <div style={{ "margin-top": "24px" }}>
          <ButtonGroup align="end" buttons={[
            { label: "Cancel", kind: "ghost", onClick: props.onCancel },
            { label: props.confirmLabel, kind: "danger", onClick: props.onConfirm, disabled: !ok() },
          ]} />
        </div>
      </div>
    </div>
  );
}
