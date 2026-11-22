import "../../web/quag.css";
import "./explorer.css";
import { createSignal, For, Match, onCleanup, onMount, Show, Switch } from "solid-js";
import { render } from "solid-js/web";
import { AppHeader, AppShell, Button, Card, NavItem, PageHead, SearchBar, StatusBar, Tag, Text } from "../src";
import { CATALOG, LEVELS } from "./catalog";
import { PAGES } from "./pond/pages";

// Routes:  #/<entry-id>  = explorer with that part selected   ·   #/page/<id> = one UI page, full window
const route = () => location.hash.replace(/^#\/?/, "");
const [path, setPath] = createSignal(route());
addEventListener("hashchange", () => setPath(route()));
const go = (p: string) => (location.hash = `/${p}`);

function Explorer() {
  const [filter, setFilter] = createSignal("");
  const selected = () => CATALOG.find((e) => e.id === path()) ?? CATALOG[0];
  // the parts in list order (grouped by level), only those matching the filter
  const order = () => LEVELS.flatMap((lv) => CATALOG.filter((e) => e.level === lv.level && e.name.toLowerCase().includes(filter().toLowerCase())));
  const at = () => Math.max(0, order().indexOf(selected()));
  const to = (i: number) => { const e = order()[i]; if (e) go(e.id); };
  const move = (d: number) => { const n = order().length; if (n) to((at() + d + n) % n); };
  // first part of the next / previous level
  const level = (d: number) => {
    const o = order(), lv = selected().level;
    if (d > 0) return to(o.findIndex((e, i) => i > at() && e.level !== lv));
    const prev = o.slice(0, at()).reverse().find((e) => e.level !== lv)?.level;
    if (prev) to(o.findIndex((e) => e.level === prev));
  };
  const onFilter = (v: string) => { setFilter(v); if (!order().includes(selected())) to(0); };
  const searchKey = (e: KeyboardEvent) => {
    if (e.key === "Escape") { e.preventDefault(); onFilter(""); (e.target as HTMLElement).blur(); }
    if (e.key === "Enter" || e.key === "ArrowDown") { e.preventDefault(); (e.target as HTMLElement).blur(); }
  };
  const onKey = (e: KeyboardEvent) => {
    if (e.defaultPrevented) return; // a part in the preview used the key (tree, tabs, menu, split…)
    if ((e.target as HTMLElement).closest("input, textarea")) return;
    if (e.key === "Enter" && (e.target as HTMLElement).closest("button, a[href]")) return; // the button presses itself
    const k = e.key;
    const act = k === "ArrowDown" || k === "j" ? () => move(1) : k === "ArrowUp" || k === "k" ? () => move(-1)
      : k === "Home" ? () => to(0) : k === "End" ? () => to(order().length - 1)
      : k === "PageDown" ? () => level(1) : k === "PageUp" ? () => level(-1)
      : k === "/" ? () => document.querySelector<HTMLInputElement>(".x-filter input")?.focus()
      : k === "Enter" && selected().page ? () => go(`page/${selected().page}`) : undefined;
    if (act) { e.preventDefault(); act(); }
  };
  onMount(() => addEventListener("keydown", onKey));
  onCleanup(() => removeEventListener("keydown", onKey));

  return (
    <AppShell
      header={<AppHeader app="Quag" location="parts explorer"><Text tone="muted">{CATALOG.length} parts</Text></AppHeader>}
      nav={
        <>
          <div class="x-filter" onKeyDown={searchKey}><SearchBar value={filter()} onValue={onFilter} placeholder="Filter parts…" shortcut="/" /></div>
          <Show when={order().length} fallback={<Text tone="faint">No part matches.</Text>}>
          <For each={LEVELS.filter((lv) => order().some((e) => e.level === lv.level))}>
            {(lv) => (
              <>
                <h3 class="x-level">{lv.n} · {lv.level}</h3>
                <For each={order().filter((e) => e.level === lv.level)}>
                  {(e) => <NavItem label={e.name} active={e === selected()} onClick={() => go(e.id)} />}
                </For>
              </>
            )}
          </For>
          </Show>
        </>
      }
      status={<StatusBar hints={[["↑↓", "browse"], ["PgUp PgDn", "level"], ["/", "filter"], ...(selected().page ? [["Enter", "open page"] as [string, string]] : [])]} />}
    >
      <PageHead title={selected().name}><Tag>{selected().level}</Tag></PageHead>
      <p class="q-t-muted x-what">{selected().what}</p>
      <Show when={selected().from}><p class="x-from"><Text tone="faint">built from</Text> <Text tone="heading">{selected().from}</Text></p></Show>
      <Show
        when={selected().page}
        fallback={<Card class="x-stage">{selected().preview?.()}</Card>}
      >
        {(id) => (
          <>
            <div class="x-thumb">
              <iframe src={`#/page/${id()}`} title={`${selected().name} preview`} tabindex="-1" />
            </div>
            <div class="q-btn-group" style={{ "margin-top": "16px" }}>
              <Button kind="primary" onClick={() => go(`page/${id()}`)}>Open page ↗</Button>
            </div>
          </>
        )}
      </Show>
    </AppShell>
  );
}

function App() {
  // Esc on a full page goes back to its entry in the explorer
  const onEsc = (e: KeyboardEvent) => {
    if (e.key !== "Escape" || e.defaultPrevented || !path().startsWith("page/") || window.self !== window.top) return;
    if (document.querySelector(".q-backdrop")) return; // a dialog handles its own Esc
    go(`ui-${path().slice(5)}`);
  };
  onMount(() => addEventListener("keydown", onEsc));
  onCleanup(() => removeEventListener("keydown", onEsc));
  return (
    <Switch fallback={<Explorer />}>
      <Match when={path().startsWith("page/") && PAGES.find((p) => `page/${p.id}` === path())}>
        {(p) => {
          const Page = p().page;
          // framed: the status bar leaves room for the "← Explorer" button
          onMount(() => document.body.classList.toggle("x-framed", window.self === window.top));
          onCleanup(() => document.body.classList.remove("x-framed"));
          return (
            <>
              <Page />
              <Show when={window.self === window.top}>
                <button class="x-back q-btn q-ghost" onClick={() => go(`ui-${p().id}`)}>← Explorer</button>
              </Show>
            </>
          );
        }}
      </Match>
    </Switch>
  );
}

render(() => <App />, document.getElementById("root")!);
