import { BREADCRUMBS_MAX } from "../constants";
import { For } from "solid-js";

/** MOLECULE · where you are. Over `max` crumbs, the middle collapses to … (first and last always stay). */
export function Breadcrumbs(props: { path: string[]; max?: number; onSelect?: (i: number) => void }) {
  const shown = () => {
    const p = props.path.map((label, i) => ({ label, i }));
    const max = props.max ?? BREADCRUMBS_MAX;
    return p.length <= max ? p : [p[0], { label: "…", i: -1 }, ...p.slice(p.length - (max - 2))];
  };
  return (
    <nav aria-label="Breadcrumbs">
      <ol class="q-crumbs" style={{ margin: 0, padding: 0 }}>
        <For each={shown()}>
          {(c) => (
            <li class={c.i < 0 ? "is-collapsed" : undefined} aria-current={c.i === props.path.length - 1 ? "page" : undefined}
              onClick={() => c.i >= 0 && props.onSelect?.(c.i)}>{c.label}</li>
          )}
        </For>
      </ol>
    </nav>
  );
}
