import { STALE_AFTER_SECS } from "../constants";
import { For, Show } from "solid-js";
import { type StepState, StepIcon, SecretField, Status, UsageBar, usageTone } from "../atoms";
import { age } from "../fmt";

/** MOLECULE · `Memory ━━━━━── 412 / 512 MiB`. Past 80 % the value gets the status icon too. */
export function Meter(props: { label?: string; fraction: number; value: string; labelWidth?: string }) {
  const tone = () => usageTone(props.fraction);
  return (
    <div class={`q-meter${props.label ? "" : " no-label"}`} style={{ "--label-w": props.labelWidth ?? "64px" }}>
      <Show when={props.label}><span class="label">{props.label}</span></Show>
      <UsageBar fraction={props.fraction} label={props.label} />
      <span class="value">
        <Show when={tone()} fallback={props.value}>{(t) => <Status kind={t()}>{props.value}</Status>}</Show>
      </span>
    </div>
  );
}

export interface PortSpec { host?: number; container: number; proto: string; public?: boolean }
/** MOLECULE · `8080 → 80/tcp`; published on all interfaces gets `!` (reachable from outside). */
export function PortMap(props: PortSpec) {
  return (
    <span class="q-port" title={props.public ? "Published on all interfaces" : undefined}>
      <Show when={props.public}><Status kind="warn" /></Show>
      <Show when={props.host !== undefined}><span>{props.host}</span><span class="arrow">→</span></Show>
      <span class="proto">{props.container}/{props.proto}</span>
    </span>
  );
}

export interface StepSpec { name: string; state: StepState; took?: string }
/** MOLECULE · steps of a job: vertical with times, or inline `✓ pull › ⠋ deploy › ○ check`. */
export function StepList(props: { steps: StepSpec[]; inline?: boolean }) {
  return (
    <ol class={`q-steps${props.inline ? " inline" : ""}`} aria-label="Steps">
      <For each={props.steps}>
        {(s) => <li class={s.state}><StepIcon state={s.state} /><span>{s.name}</span><Show when={!props.inline && s.took}><span class="took">{s.took}</span></Show></li>}
      </For>
    </ol>
  );
}

/** MOLECULE · key = value rows (env vars, labels). Secret values are SecretFields (masked, Show, Copy).
 *  `stacked`: key above value, for narrow places (inspector). `reveal`: show / hide every secret (a page key). */
export function KeyValueRows(props: { rows: [string, string, boolean?][]; stacked?: boolean; reveal?: boolean }) {
  return (
    <dl class={`q-kv${props.stacked ? " stacked" : ""}`}>
      <For each={props.rows}>
        {([k, v, secret]) => <><dt>{k}</dt><dd>{secret ? <SecretField value={v} label={k} shown={props.reveal} /> : <span title={v}>{v}</span>}</dd></>}
      </For>
    </dl>
  );
}

/** MOLECULE · how fresh live data is: `updated 3s ago`, or `! stale 45s` (yellow) after `staleAfter` seconds. */
export function Freshness(props: { age: number; staleAfter?: number }) {
  const stale = () => props.age > (props.staleAfter ?? STALE_AFTER_SECS);
  return (
    <span class={`q-fresh${stale() ? " stale" : ""}`} aria-live="polite">
      {stale() ? <Status kind="warn">stale {props.age}s</Status> : `updated ${age(props.age)}`}
    </span>
  );
}
