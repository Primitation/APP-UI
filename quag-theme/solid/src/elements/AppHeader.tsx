import { type JSX, Show } from "solid-js";
import { type Env, EnvTag } from "../atoms";

/** ELEMENT (shared) · the header: ◆ app name left, tabs/search/actions right. */
/** `env`: server tools show the environment on every page (PROD in red). */
export function AppHeader(props: { app: string; location?: string; env?: Env; children?: JSX.Element }) {
  return (
    <header class="q-topbar">
      <span class="q-brand" style={{ padding: 0 }}>{props.app}</span>
      <Show when={props.env}>{(e) => <EnvTag env={e()} />}</Show>
      <Show when={props.location}><span class="q-t-muted">{props.location}</span></Show>
      <div style={{ "margin-left": "auto", display: "flex", "align-items": "center", gap: "16px" }}>{props.children}</div>
    </header>
  );
}
