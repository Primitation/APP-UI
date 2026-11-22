import { Show } from "solid-js";
import { Spinner, Status, Text } from "../atoms/small";
import { ButtonGroup } from "../molecules/ButtonGroup";
import { type StepSpec, StepList } from "../molecules/servers";
import { type LogEntry, LogViewer } from "./LogViewer";

/**
 * ELEMENT (shared) · a running job (deploy, backup, migration): steps, its live output, then Cancel or Retry.
 * A zone of the page, not a modal: the app stays usable while it runs.
 */
export function RunProgress(props: { steps: StepSpec[]; output: LogEntry[]; onCancel?: () => void; onRetry?: () => void; onRollback?: () => void; onClose?: () => void }) {
  const failed = () => props.steps.some((s) => s.state === "failed");
  const finished = () => props.steps.every((s) => s.state === "done" || s.state === "skipped");
  return (
    <div class="q-runprog">
      <StepList steps={props.steps} inline />
      <LogViewer lines={props.output} />
      <footer>
        <Show when={failed()} fallback={<Show when={finished()} fallback={<><Spinner /><Text tone="muted">running</Text></>}><Status kind="ok">done</Status></Show>}>
          <Status kind="err">failed</Status>
        </Show>
        <span style={{ "margin-left": "auto" }}>
          <ButtonGroup align="end" buttons={failed()
            ? [{ label: "Roll back", onClick: props.onRollback }, { label: "Retry", kind: "primary", onClick: props.onRetry }]
            : finished() ? [{ label: "Close", kind: "primary", onClick: props.onClose }] : [{ label: "Cancel", onClick: props.onCancel }]} />
        </span>
      </footer>
    </div>
  );
}
