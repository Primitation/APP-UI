import { For } from "solid-js";

/** MOLECULE · a row of tabs. 2–5 views of the same data. ← → (when a tab has focus) switch tab. */
export function Tabs(props: { tabs: string[]; active: number; onSelect: (i: number) => void }) {
  let nav!: HTMLElement;
  const key = (e: KeyboardEvent) => {
    const d = e.key === "ArrowRight" ? 1 : e.key === "ArrowLeft" ? -1 : 0;
    if (!d) return;
    e.preventDefault(); // handled: the page's keys and the explorer leave it alone
    const i = (props.active + d + props.tabs.length) % props.tabs.length;
    props.onSelect(i);
    (nav.querySelectorAll<HTMLElement>("[role=tab]")[i])?.focus();
  };
  return (
    <nav class="q-tabs" role="tablist" ref={nav} onKeyDown={key}>
      <For each={props.tabs}>
        {(t, i) => (
          <button class="q-tab" role="tab" aria-selected={i() === props.active} tabIndex={i() === props.active ? 0 : -1} onClick={() => props.onSelect(i())}>
            {t}
          </button>
        )}
      </For>
    </nav>
  );
}
