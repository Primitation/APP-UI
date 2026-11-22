import { onCleanup, onMount, Show } from "solid-js";
import { Portal } from "solid-js/web";
import { ButtonGroup } from "../molecules/ButtonGroup";

export interface ConfirmDialogProps {
  open: boolean;
  title: string;
  message: string;
  confirmLabel: string;
  cancelLabel?: string;
  danger?: boolean;
  onConfirm: () => void;
  onCancel: () => void;
}

/** ELEMENT (shared) · "are you sure?". Esc and backdrop click cancel; focus starts on the safe button. */
export function ConfirmDialog(props: ConfirmDialogProps) {
  return (
    <Show when={props.open}>
      <Portal>
        <DialogBody {...props} />
      </Portal>
    </Show>
  );
}

function DialogBody(props: ConfirmDialogProps) {
  let root!: HTMLDivElement;
  const onKey = (e: KeyboardEvent) => { if (e.key === "Escape") { e.preventDefault(); props.onCancel(); } };
  onMount(() => {
    document.addEventListener("keydown", onKey);
    root.querySelector<HTMLButtonElement>("button")?.focus(); // cancel comes first = safe default
  });
  onCleanup(() => document.removeEventListener("keydown", onKey));
  return (
    <div class="q-backdrop" onClick={(e) => e.target === e.currentTarget && props.onCancel()}>
      <div class="q-dialog" role="alertdialog" aria-modal="true" aria-label={props.title} ref={root}>
        <h2>{props.title}</h2>
        <p class="q-t-muted" style={{ margin: 0 }}>{props.message}</p>
        <div style={{ "margin-top": "24px" }}>
          <ButtonGroup
            align="end"
            buttons={[
              { label: props.cancelLabel ?? "Cancel", kind: "ghost", onClick: props.onCancel },
              { label: props.confirmLabel, kind: props.danger ? "danger" : "primary", onClick: props.onConfirm },
            ]}
          />
        </div>
      </div>
    </div>
  );
}
