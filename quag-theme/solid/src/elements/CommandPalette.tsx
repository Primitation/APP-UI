import { createSignal, onCleanup, onMount, Show } from "solid-js";
import { Portal } from "solid-js/web";
import { Menu, type MenuItem, SearchBar } from "../molecules";

/** ELEMENT (shared) · Ctrl+K: every action of the app by name. ↑↓ move, Enter runs, Esc closes. */
export function CommandPalette(props: { open: boolean; items: MenuItem[]; onClose: () => void }) {
  return (
    <Show when={props.open}>
      <Portal><PaletteBody {...props} /></Portal>
    </Show>
  );
}

function PaletteBody(props: { items: MenuItem[]; onClose: () => void }) {
  const [q, setQ] = createSignal("");
  const [active, setActive] = createSignal(0);
  let root!: HTMLDivElement;
  const shown = () => props.items.filter((i) => i.label.toLowerCase().includes(q().toLowerCase()));
  const run = (it?: MenuItem) => { it?.onSelect?.(); props.onClose(); };
  const onKey = (e: KeyboardEvent) => {
    if (e.key === "Escape") { e.preventDefault(); props.onClose(); } // marked: Esc only closes the palette
    else if (e.key === "ArrowDown") { e.preventDefault(); setActive((a) => Math.min(a + 1, shown().length - 1)); }
    else if (e.key === "ArrowUp") { e.preventDefault(); setActive((a) => Math.max(a - 1, 0)); }
    else if (e.key === "Enter") run(shown()[active()]);
  };
  onMount(() => { document.addEventListener("keydown", onKey); root.querySelector("input")?.focus(); });
  onCleanup(() => document.removeEventListener("keydown", onKey));
  return (
    <div class="q-backdrop is-top" onClick={(e) => e.target === e.currentTarget && props.onClose()}>
      <div class="q-palette" role="dialog" aria-modal="true" aria-label="Commands" ref={root}>
        <SearchBar value={q()} onValue={(v) => { setQ(v); setActive(0); }} placeholder="Type a command…" shortcut="Ctrl K" />
        <Show when={shown().length} fallback={<div class="q-empty-line">No command matches.</div>}>
          <Menu bare items={shown().map((i) => ({ ...i, onSelect: () => run(i) }))} active={active()} label="Commands" />
        </Show>
      </div>
    </div>
  );
}
