/** ELEMENTS of this app: molecules wired to its data. Each owns its states (empty, …). Never sets colors. */
import { For, Show } from "solid-js";
import { Card, EmptyState, Kbd, List, ListRow } from "./quag";

export interface Item { id: number; title: string; meta: string; done: boolean }

/** ItemList: the items, one ListRow each. Empty → EmptyState with the key to add one. */
export function ItemList(props: { items: Item[]; selected: number; onSelect: (id: number) => void }) {
  return (
    <Card>
      <Show when={props.items.length} fallback={<EmptyState message="Nothing here yet." hint={<>Press <Kbd>N</Kbd> to add one.</>} />}>
        <List label="Items">
          <For each={props.items}>
            {(it) => <ListRow title={it.title} meta={it.meta} check={it.done} selected={it.id === props.selected} onClick={() => props.onSelect(it.id)} />}
          </For>
        </List>
      </Show>
    </Card>
  );
}
