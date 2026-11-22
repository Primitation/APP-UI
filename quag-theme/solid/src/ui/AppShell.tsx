import { type JSX, Show } from "solid-js";

export interface AppShellProps {
  header: JSX.Element;
  nav?: JSX.Element;
  inspector?: JSX.Element;
  status?: JSX.Element;
  /** Below 1280px the inspector becomes a drawer: set this to show it. */
  inspectorOpen?: boolean;
  /** Below 960px the nav hides: set this to show it. */
  navOpen?: boolean;
  children: JSX.Element;
}

/** UI · the page skeleton (STRUCTURE.md §7). Pages fill the zones with ELEMENTS only. */
export function AppShell(props: AppShellProps) {
  const cls = () =>
    ["q-app", props.inspector && "has-inspector", !props.nav && "no-nav", props.inspectorOpen && "is-inspector-open", props.navOpen && "is-nav-open"]
      .filter(Boolean)
      .join(" ");
  return (
    <div class={cls()}>
      <Show when={props.nav}><aside class="q-sidebar">{props.nav}</aside></Show>
      {props.header}
      <main class="q-main">{props.children}</main>
      <Show when={props.inspector}><aside class="q-inspector">{props.inspector}</aside></Show>
      {props.status}
    </div>
  );
}

/** UI · page title row: h1 left, page-level actions right. */
export function PageHead(props: { title: string; children?: JSX.Element }) {
  return (
    <div class="q-page-head">
      <h1>{props.title}</h1>
      {props.children}
    </div>
  );
}

/** UI · page layouts (STRUCTURE.md §7.3). */
export function Layout(props: { kind: "form" | "focus" | "dashboard" | "stream" | "workspace"; children: JSX.Element }) {
  return <div class={`q-layout-${props.kind}`}>{props.children}</div>;
}
