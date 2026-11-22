import { Button, Status, Text } from "../atoms";

/** ELEMENT (shared) · a zone failed: what + why + Retry. Only that zone shows it; the rest stays usable. */
export function ErrorState(props: { what: string; why: string; onRetry?: () => void }) {
  return (
    <div class="q-error-state" role="alert">
      <span class="q-error-title"><Status kind="err">{props.what}</Status></span>
      <Text tone="muted">{props.why}</Text>
      <Button onClick={props.onRetry} style={{ "margin-top": "8px" }}>Retry</Button>
    </div>
  );
}
