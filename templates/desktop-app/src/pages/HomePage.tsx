/** PAGE: places elements in the AppShell zones. Keys declared ONCE with createPageKeys
 *  (status bar hints, ? help and Ctrl K palette all read them). */
import { createSignal } from "solid-js";
import { AppHeader, AppShell, Button, createPageKeys, PageHead, StatusBar, Toast } from "../quag";
import { DEMO_ITEMS } from "../consts";
import { type Item, ItemList } from "../elements";

export function HomePage() {
  const [items, setItems] = createSignal<Item[]>(
    Array.from({ length: DEMO_ITEMS }, (_, i) => ({ id: i + 1, title: `Item ${i + 1}`, meta: "today", done: false })),
  );
  const [sel, setSel] = createSignal(1);
  const [toast, setToast] = createSignal<string>();
  const move = (d: number) => {
    const list = items(), i = list.findIndex((x) => x.id === sel());
    const next = list[Math.max(0, Math.min(list.length - 1, i + d))];
    if (next) setSel(next.id);
  };
  const toggle = () => {
    setItems((all) => all.map((x) => (x.id === sel() ? { ...x, done: !x.done } : x)));
    setToast(items().find((x) => x.id === sel())?.done ? "Marked done" : "Reopened");
  };
  const add = () => {
    const id = Math.max(0, ...items().map((x) => x.id)) + 1;
    setItems((all) => [...all, { id, title: `Item ${id}`, meta: "today", done: false }]);
    setSel(id);
  };
  const keys = createPageKeys([
    { key: "↑ ↓", what: "previous / next", group: "Move", hint: "move", match: ["ArrowUp", "ArrowDown"], run: (k) => move(k === "ArrowDown" ? 1 : -1) },
    { key: "N", what: "new item", group: "Act", hint: "new", command: "New item", match: ["n"], run: add },
    { key: "X", what: "mark done / reopen", group: "Act", hint: "done", command: "Mark done", match: ["x"], run: toggle },
  ]);
  return (
    <AppShell header={<AppHeader app="quag-app" location="home" />} status={<StatusBar hints={keys.hints()} />}>
      <PageHead title="Items"><Button kind="primary" onClick={add}>＋ New item</Button></PageHead>
      <ItemList items={items()} selected={sel()} onSelect={setSel} />
      {keys.overlays}
      <Toast open={!!toast()} kind="ok" message={toast() ?? ""} onClose={() => setToast()} />
    </AppShell>
  );
}
