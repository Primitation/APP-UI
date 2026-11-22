import { LOG_INITIAL_HEIGHT, LOG_OVERSCAN_ABOVE, LOG_OVERSCAN_BELOW, LOG_ROW_PX } from "../constants";
import { createEffect, createMemo, createSignal, For, on } from "solid-js";
import { Mark, Status, type StatusKind, Text } from "../atoms";
import { SearchBar } from "../molecules";

export interface LogEntry { time: string; kind: StatusKind; message: string }

const ROW = LOG_ROW_PX;

/**
 * ELEMENT (shared) · a big log. Filter (highlighted), follows the newest line until you scroll up,
 * and only the visible lines are in the DOM (100k+ lines are fine).
 */
export function LogViewer(props: { lines: LogEntry[]; only?: StatusKind[] }) {
  const [filter, setFilter] = createSignal("");
  const [top, setTop] = createSignal(0);
  const [height, setHeight] = createSignal(LOG_INITIAL_HEIGHT);
  const [follow, setFollow] = createSignal(true);
  let body!: HTMLDivElement;
  const shown = createMemo(() => {
    const f = filter().toLowerCase();
    return props.lines.filter((l) => (!props.only || props.only.includes(l.kind)) && (!f || l.message.toLowerCase().includes(f)));
  });
  const first = () => Math.max(0, Math.floor(top() / ROW) - LOG_OVERSCAN_ABOVE);
  const visible = () => shown().slice(first(), first() + Math.ceil(height() / ROW) + LOG_OVERSCAN_BELOW);
  createEffect(on(shown, () => { if (follow() && body) queueMicrotask(() => (body.scrollTop = body.scrollHeight)); }));
  const onScroll = () => {
    setTop(body.scrollTop);
    setHeight(body.clientHeight);
    setFollow(body.scrollTop + body.clientHeight >= body.scrollHeight - ROW);
  };
  return (
    <div class="q-log">
      <div class="q-log-head">
        <SearchBar value={filter()} onValue={setFilter} placeholder="Filter…" />
        <span class="count">{shown().length.toLocaleString()} / {props.lines.length.toLocaleString()} lines</span>
        <button type="button" class="q-btn q-ghost" aria-pressed={follow()} onClick={() => { setFollow(true); body.scrollTop = body.scrollHeight; }}>
          {follow() ? "● following" : "↓ follow"}
        </button>
      </div>
      <div class="q-log-body" ref={body} onScroll={onScroll} role="log" aria-live="off">
        <div style={{ height: `${shown().length * ROW}px` }} />
        <For each={visible()}>
          {(l, i) => (
            <div class="q-log-line" style={{ top: `${(first() + i()) * ROW}px` }}>
              <Text tone="faint">{l.time}</Text><Status kind={l.kind} /><span><Mark text={l.message} query={filter()} /></span>
            </div>
          )}
        </For>
      </div>
    </div>
  );
}
