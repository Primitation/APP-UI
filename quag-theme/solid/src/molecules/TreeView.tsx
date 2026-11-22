import { For, Show } from "solid-js";
import { Status, type StatusKind } from "../atoms";

export interface TreeItem {
  id: string;
  label: string;
  meta?: string;
  status?: StatusKind;
  children?: TreeItem[];
}

export interface TreeRow { item: TreeItem; depth: number; open?: boolean }

/** The visible rows, given which ids are open. */
export function flattenTree(items: TreeItem[], isOpen: (id: string) => boolean, depth = 0): TreeRow[] {
  return items.flatMap((item) => {
    const open = item.children?.length ? isOpen(item.id) : undefined;
    const row: TreeRow = { item, depth, open };
    return open ? [row, ...flattenTree(item.children!, isOpen, depth + 1)] : [row];
  });
}

/**
 * MOLECULE · any hierarchy. ↑↓ move, → opens (or goes to the first child), ← closes (or goes to the parent),
 * Enter activates. The app owns which ids are open and which is selected.
 */
export function TreeView(props: {
  items: TreeItem[];
  open: Set<string>;
  onToggle: (id: string, open: boolean) => void;
  selected?: string;
  onSelect?: (id: string) => void;
  onActivate?: (id: string) => void;
  label?: string;
}) {
  const rows = () => flattenTree(props.items, (id) => props.open.has(id));
  const idx = () => rows().findIndex((r) => r.item.id === props.selected);
  const onKey = (e: KeyboardEvent) => {
    const r = rows(), i = Math.max(idx(), 0), cur = r[i];
    const go = (j: number) => { const t = r[Math.max(0, Math.min(r.length - 1, j))]; if (t) props.onSelect?.(t.item.id); };
    if (e.key === "ArrowDown") go(i + 1);
    else if (e.key === "ArrowUp") go(i - 1);
    else if (e.key === "ArrowRight" && cur) { if (cur.open === false) props.onToggle(cur.item.id, true); else if (cur.open) go(i + 1); }
    else if (e.key === "ArrowLeft" && cur) {
      if (cur.open) props.onToggle(cur.item.id, false);
      else { for (let j = i - 1; j >= 0; j--) if (r[j].depth < cur.depth) { go(j); break; } }
    } else if (e.key === "Enter" && cur) props.onActivate?.(cur.item.id);
    else return;
    e.preventDefault();
  };
  return (
    <ul class="q-tree" role="tree" aria-label={props.label} tabIndex={0} onKeyDown={onKey}>
      <For each={rows()}>
        {(r) => (
          <li role="treeitem" aria-expanded={r.open} aria-selected={r.item.id === props.selected} aria-level={r.depth + 1}
            class={`q-tree-row${r.item.id === props.selected ? " is-selected" : ""}`} style={{ "padding-left": `${8 + r.depth * 16}px` }}
            onClick={() => { props.onSelect?.(r.item.id); if (r.open !== undefined) props.onToggle(r.item.id, !r.open); }}
            onDblClick={() => props.onActivate?.(r.item.id)}>
            <span class="q-twisty">{r.open === undefined ? "" : r.open ? "▾" : "▸"}</span>
            <span class="q-tree-label">{r.item.label}</span>
            <Show when={r.item.status}>{(s) => <Status kind={s()} />}</Show>
            <Show when={r.item.meta}><span class="q-row-meta">{r.item.meta}</span></Show>
          </li>
        )}
      </For>
    </ul>
  );
}
