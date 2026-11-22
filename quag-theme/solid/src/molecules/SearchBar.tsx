import { Input, Kbd } from "../atoms";

/** MOLECULE · search icon + input + key hint. */
export function SearchBar(props: { value: string; onValue: (v: string) => void; placeholder?: string; shortcut?: string }) {
  return (
    <div class="q-search">
      <Input value={props.value} onValue={props.onValue} placeholder={props.placeholder ?? "Search…"} aria-label="Search" />
      <Kbd>{props.shortcut ?? "/"}</Kbd>
    </div>
  );
}
