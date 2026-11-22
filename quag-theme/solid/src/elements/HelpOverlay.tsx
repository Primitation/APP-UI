import { For, onCleanup, onMount, Show } from "solid-js";
import { Portal } from "solid-js/web";
import { Kbd } from "../atoms/small";

export type KeyGroup = [title: string, keys: [key: string, what: string][]];

/** ELEMENT (shared) · `?`: every key of the current page, grouped. Esc or a click outside closes it. */
export function HelpOverlay(props: { open: boolean; groups: KeyGroup[]; onClose: () => void }) {
  return (
    <Show when={props.open}>
      <Portal><HelpBody {...props} /></Portal>
    </Show>
  );
}

function HelpBody(props: { groups: KeyGroup[]; onClose: () => void }) {
  // Esc is handled here: marked, so nothing else (e.g. "Esc = back") acts on it too
  const onKey = (e: KeyboardEvent) => { if (e.key === "Escape") { e.preventDefault(); props.onClose(); } };
  onMount(() => document.addEventListener("keydown", onKey));
  onCleanup(() => document.removeEventListener("keydown", onKey));
  return (
    <div class="q-backdrop" onClick={(e) => e.target === e.currentTarget && props.onClose()}>
      <div class="q-dialog" role="dialog" aria-modal="true" aria-label="Keyboard shortcuts" style={{ width: "min(640px, 92vw)" }}>
        <h2 style={{ "margin-bottom": "16px" }}>Keys</h2>
        <div class="q-help-grid">
          <For each={props.groups}>
            {([title, keys]) => (
              <section>
                <h3>{title}</h3>
                <dl><For each={keys}>{([k, what]) => <><dt><Kbd>{k}</Kbd></dt><dd>{what}</dd></>}</For></dl>
              </section>
            )}
          </For>
        </div>
      </div>
    </div>
  );
}
