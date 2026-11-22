import { COPIED_MS, SHORT_ID_LEN, USAGE_ERR, USAGE_WARN } from "../constants";
import { createSignal, Show } from "solid-js";

/** State of a container / service / job. Stopped is NOT an error (grey); only crashes are red. */
export type RunState =
  | { kind: "running" | "starting" | "stopping" | "restarting" | "stopped" | "paused" }
  | { kind: "exited"; code: number }
  | { kind: "flapping"; restarts: number };

export const isBusy = (s: RunState) => s.kind === "starting" || s.kind === "stopping" || s.kind === "restarting";
export const isUp = (s: RunState) => s.kind === "running" || s.kind === "flapping";
export function runTone(s: RunState): "ok" | "warn" | "err" | "neutral" {
  if (s.kind === "running") return "ok";
  if (isBusy(s) || s.kind === "flapping") return "warn";
  if (s.kind === "exited" && s.code !== 0) return "err";
  return "neutral";
}
export function runLabel(s: RunState): string {
  if (s.kind === "exited") return s.code === 0 ? "exited 0" : `crashed ${s.code}`;
  if (s.kind === "flapping") return `restarted ${s.restarts}×`;
  return s.kind;
}

/** ATOM · `● running` · spinner `starting` · `○ stopped` · `‖ paused` · `✗ crashed 137` · `! restarted 5×`. */
export function RunStateLabel(props: { state: RunState }) {
  const icon = () => {
    const s = props.state;
    if (s.kind === "running") return "●";
    if (s.kind === "paused") return "‖";
    if (s.kind === "flapping") return "!";
    if (s.kind === "exited" && s.code !== 0) return "✗";
    return "○";
  };
  return (
    <span class={`q-run ${runTone(props.state)}`}>
      <Show when={isBusy(props.state)} fallback={<span aria-hidden="true">{icon()}</span>}><span class="q-spinner" aria-hidden="true" /></Show>
      {runLabel(props.state)}
    </span>
  );
}

export type Env = "prod" | "staging" | "dev";
/** ATOM · the environment, in the header of every page: PROD red outline, STAGING yellow, DEV grey. */
export function EnvTag(props: { env: Env }) {
  return <span class={`q-env ${props.env}`} aria-label={`environment ${props.env}`}>{props.env}</span>;
}

/** ATOM · an id or digest shortened to 12 characters (`sha256:` dropped), full value on hover, copy button. */
export function ShortId(props: { id: string }) {
  const [copied, setCopied] = createSignal(false);
  const short = () => props.id.replace(/^sha256:/, "").slice(0, SHORT_ID_LEN);
  const copy = () => { navigator.clipboard?.writeText(props.id).catch(() => {}); setCopied(true); setTimeout(() => setCopied(false), COPIED_MS); };
  return (
    <span class="q-shortid" title={props.id}>
      {short()}
      <button type="button" onClick={copy} aria-label={`Copy ${props.id}`}>{copied() ? "✓" : "⧉"}</button>
    </span>
  );
}

/** ATOM · usage of a limit: accent, yellow from 80 %, red at the limit. Thin, so many fit calmly on a page. */
export function UsageBar(props: { fraction: number; label?: string }) {
  const tone = () => { const t = usageTone(props.fraction); return t ? ` ${t}` : ""; };
  return (
    <div class={`q-usage${tone()}`} role="meter" aria-valuemin={0} aria-valuemax={100} aria-valuenow={Math.round(props.fraction * 100)} aria-label={props.label}>
      <span style={{ width: `${Math.max(0, Math.min(1, props.fraction)) * 100}%` }} />
    </div>
  );
}
export const usageTone = (f: number): "err" | "warn" | undefined => (f >= USAGE_ERR ? "err" : f >= USAGE_WARN ? "warn" : undefined);

export type DiffKind = "same" | "add" | "del" | "hunk";
/** ATOM · one line of a diff: `+` green, `−` red and struck, same faint, `@@` Gill. */
export function DiffLine(props: { kind: DiffKind; text: string }) {
  const mark = () => ({ add: "+", del: "−", same: "", hunk: "" })[props.kind];
  return <div class={`q-diff-line ${props.kind}`}><b aria-hidden="true">{mark()}</b><span>{props.text}</span></div>;
}

export type StepState = "waiting" | "running" | "done" | "failed" | "skipped";
/** ATOM · `✓` done · spinner running · `○` waiting · `✗` failed · `–` skipped. */
export function StepIcon(props: { state: StepState }) {
  const icon = () => ({ done: "✓", failed: "✗", waiting: "○", skipped: "–", running: "" })[props.state];
  return (
    <span class={`q-step-icon ${props.state}`} aria-label={props.state}>
      <Show when={props.state === "running"} fallback={icon()}><span class="q-spinner" /></Show>
    </span>
  );
}
