import { type JSX, splitProps } from "solid-js";

export interface InputProps extends Omit<JSX.InputHTMLAttributes<HTMLInputElement>, "class" | "onInput"> {
  invalid?: boolean;
  onValue?: (value: string) => void;
  class?: string;
}

/** ATOM · one-line text input. */
export function Input(props: InputProps) {
  const [own, rest] = splitProps(props, ["invalid", "onValue", "class"]);
  return (
    <input
      class={`q-input${own.class ? " " + own.class : ""}`}
      aria-invalid={own.invalid ? "true" : undefined}
      onInput={(e) => own.onValue?.(e.currentTarget.value)}
      {...rest}
    />
  );
}

export interface TextareaProps extends Omit<JSX.TextareaHTMLAttributes<HTMLTextAreaElement>, "class" | "onInput"> {
  onValue?: (value: string) => void;
}

/** ATOM · multi-line text input. */
export function Textarea(props: TextareaProps) {
  const [own, rest] = splitProps(props, ["onValue"]);
  return <textarea class="q-textarea" onInput={(e) => own.onValue?.(e.currentTarget.value)} {...rest} />;
}
