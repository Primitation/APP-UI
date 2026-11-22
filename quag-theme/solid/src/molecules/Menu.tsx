import { For, Show } from "solid-js";
import { Kbd } from "../atoms/small";

export interface MenuItem {
  label: string;
  /** shortcut shown on the right */
  key?: string;
  /** destroys something: red, and keep it last */
  danger?: boolean;
  onSelect?: () => void;
}

/** MOLECULE · a column of actions with their keys. Bare (no box) when `bare` (e.g. inside the CommandPalette).
 *  ↑ ↓ (when an item has focus) move between the items. */
export function Menu(props: { items: MenuItem[]; active?: number; label?: string; bare?: boolean }) {
  const key = (e: KeyboardEvent) => {
    const d = e.key === "ArrowDown" ? 1 : e.key === "ArrowUp" ? -1 : 0;
    if (!d) return;
    e.preventDefault(); // handled: the page's keys and the explorer leave it alone
    const items = [...(e.currentTarget as HTMLElement).querySelectorAll<HTMLElement>("[role=menuitem]")];
    const i = items.indexOf(document.activeElement as HTMLElement);
    items[Math.max(0, Math.min(items.length - 1, (i < 0 ? 0 : i + d)))]?.focus();
  };
  return (
    <div class={props.bare ? undefined : "q-menu"} role="menu" aria-label={props.label} onKeyDown={key}>
      <For each={props.items}>
        {(it, i) => (
          <>
            <Show when={it.danger && i() > 0 && !props.items[i() - 1].danger}><div class="q-menu-sep" role="separator" /></Show>
            <button type="button" role="menuitem"
              class={`q-menu-item${i() === props.active ? " is-active" : ""}${it.danger ? " is-danger" : ""}`}
              onClick={() => it.onSelect?.()}>
              {it.label}
              <Show when={it.key}><Kbd>{it.key}</Kbd></Show>
            </button>
          </>
        )}
      </For>
    </div>
  );
}
