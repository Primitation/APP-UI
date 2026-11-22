import { type JSX, Show, createUniqueId } from "solid-js";
import { Help, Label } from "../atoms/small";

export interface FieldProps {
  label: string;
  help?: string;
  /** When set, replaces the help text and marks the field invalid. */
  error?: string;
  /** The input atom. Receives the id so the label points at it. */
  children: (id: string, invalid: boolean) => JSX.Element;
}

/** MOLECULE · label + input + help/error. The error always sits directly under its input. */
export function Field(props: FieldProps) {
  const id = createUniqueId();
  return (
    <div class="q-field">
      <Label for={id}>{props.label}</Label>
      {props.children(id, !!props.error)}
      <Show when={props.error} fallback={<Show when={props.help}><Help>{props.help}</Help></Show>}>
        <Help error>{props.error}</Help>
      </Show>
    </div>
  );
}
