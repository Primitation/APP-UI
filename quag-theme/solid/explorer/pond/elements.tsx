/**
 * ELEMENTS of the demo app "pond": molecules put to use, wired to pond's data.
 * Each one has a job, reads data, wires actions, and owns its states. No colors here.
 */
import { createSignal, For, Show } from "solid-js";
import {
  ListRow as Row, PropertyList as Props, Segmented, SecretField, SeverityBadge, Status as StatusIcon,
  BarChart, Breadcrumbs, Button, DeltaBadge, Histogram, ParamRow, Select, Sparkline, TreeView, ButtonGroup, Card, Checkbox, ConfirmDialog, EmptyState, ErrorState, Field, Input, Kbd, List, ListRow,
  NavItem, Progress, PropertyList, RadioGroup, SearchBar, Spinner, Table, Text, Textarea,
} from "../../src";
import { DONE_PER_DAY, type Finding, FRAME_MS, LOG, MEMORY_MB, openIn, PROJECT_ROWS, PROJECTS, RANGES, RULES, SERVERS, type Task, TASKS, TREE } from "./data";

/** TaskActions = ButtonGroup + the selected task. The user's own example. */
export function TaskActions(props: { task: Task; onToggle?: () => void; onDelete?: () => void }) {
  const [confirm, setConfirm] = createSignal(false);
  return (
    <>
      <ButtonGroup
        align="end"
        buttons={[
          { label: "Delete", kind: "danger", onClick: () => setConfirm(true) },
          { label: "Edit" },
          props.task.done
            ? { label: "Reopen", onClick: props.onToggle }
            : { label: "Mark done", kind: "primary", onClick: props.onToggle },
        ]}
      />
      <ConfirmDialog
        open={confirm()}
        title="Delete task?"
        message={`“${props.task.title}” can't be restored.`}
        confirmLabel="Delete"
        cancelLabel="Keep"
        danger
        onConfirm={() => { setConfirm(false); props.onDelete?.(); }}
        onCancel={() => setConfirm(false)}
      />
    </>
  );
}

/** TaskList: search + rows. States: loading, empty, ideal. */
export function TaskList(props: { tasks: Task[]; selected?: number; onSelect?: (id: number) => void; loading?: boolean; mascot?: string;
  /** what the empty list says: message, key, what the key does */
  empty?: [string, string, string] }) {
  const [q, setQ] = createSignal("");
  const shown = () => props.tasks.filter((t) => t.title.toLowerCase().includes(q().toLowerCase()));
  return (
    <div class="q-card" style={{ padding: "8px" }}>
      <div style={{ padding: "4px 4px 8px", "max-width": "320px" }}><SearchBar value={q()} onValue={setQ} /></div>
      <Show when={!props.loading} fallback={<div style={{ padding: "24px" }}><Spinner /> <Text tone="muted">Loading tasks…</Text></div>}>
        <Show
          when={shown().length > 0}
          fallback={<EmptyState message={props.empty?.[0] ?? "Nothing here yet."} hint={<>Press <Kbd>{props.empty?.[1] ?? "N"}</Kbd> {props.empty?.[2] ?? "to add a task"}.</>} mascot={props.mascot} />}
        >
          <List label="Tasks">
            <For each={shown()}>
              {(t) => (
                <ListRow
                  title={t.title}
                  check={t.done}
                  tags={[t.tag]}
                  status={t.state}
                  meta={t.due}
                  metaAccent={t.due === "today"}
                  selected={t.id === props.selected}
                  onClick={() => props.onSelect?.(t.id)}
                />
              )}
            </For>
          </List>
        </Show>
      </Show>
    </div>
  );
}

/** TaskInspector: fields of the selected task + TaskActions. State: nothing selected. */
export function TaskInspector(props: { task?: Task; onToggle?: () => void }) {
  return (
    <Show when={props.task} fallback={<Text tone="muted">Select a task to see its details.</Text>}>
      {(task) => (
        <>
          <h2 style={{ "margin-bottom": "16px" }}>{task().title}</h2>
          <Field label="Notes" help="Markdown is fine.">
            {(id) => <Textarea id={id} value={task().notes} placeholder="Add notes…" />}
          </Field>
          <Field label="Due" error={task().due === "tomorrow" ? "Can't read that date." : undefined}>
            {(id, invalid) => <Input id={id} value={task().due === "tomorrow" ? "tomorow" : task().due} invalid={invalid} />}
          </Field>
          <TaskActions task={task()} onToggle={props.onToggle} />
        </>
      )}
    </Show>
  );
}

/** ProjectNav: brand + projects + this week's progress. */
export function ProjectNav(props: { active: number; onSelect?: (i: number) => void; tasks?: Task[] }) {
  return (
    <>
      <div class="q-brand">pond</div>
      <For each={PROJECTS}>
        {(name, i) => <NavItem label={name} icon="◇" count={openIn(props.tasks ?? TASKS, i())} active={i() === props.active} onClick={() => props.onSelect?.(i())} />}
      </For>
      <div style={{ "margin-top": "auto", padding: "8px" }}>
        <h3 style={{ display: "flex", "justify-content": "space-between", "margin-bottom": "8px" }}>
          This week <Text tone="muted">7 / 12</Text>
        </h3>
        <Progress value={7 / 12} label="Tasks done this week" />
      </div>
    </>
  );
}

/** NotificationSettings: a settings group (Form layout). */
export function NotificationSettings() {
  const [due, setDue] = createSignal(true);
  const [daily, setDaily] = createSignal(false);
  const [sound, setSound] = createSignal(true);
  return (
    <>
      <h3 style={{ margin: "8px 0 12px" }}>Notifications</h3>
      <div style={{ display: "grid", gap: "10px", "margin-bottom": "24px" }}>
        <Checkbox checked={due()} onChange={setDue}>Remind me when a task is due</Checkbox>
        <Checkbox checked={daily()} onChange={setDaily}>Daily summary at 9:00</Checkbox>
        <Checkbox toggle checked={sound()} onChange={setSound}>Sounds</Checkbox>
      </div>
    </>
  );
}

// ── Workspace elements (the note editor) ──

const TOOLS = ["¶", "B", "I", "•", "#"];

/** EditorTools: the tools strip. One button per tool; the active one is primary. */
export function EditorTools(props: { active: number; onSelect?: (i: number) => void }) {
  return (
    <div class="q-tools" role="toolbar" aria-label="Tools">
      <For each={TOOLS}>{(t, i) => <Button kind={i() === props.active ? "primary" : "ghost"} aria-pressed={i() === props.active} onClick={() => props.onSelect?.(i())}>{t}</Button>}</For>
    </div>
  );
}

/** NoteEditor: where you are (Breadcrumbs) + the note text. */
export function NoteEditor(props: { value: string; onValue: (v: string) => void }) {
  return (
    <Card style={{ display: "grid", gap: "12px", "grid-template-rows": "auto 1fr", height: "100%" }}>
      <Breadcrumbs path={["pond", "quag-theme", "Write README", "README.md"]} />
      <Textarea value={props.value} onValue={props.onValue} aria-label="Note" style={{ height: "100%", "font-family": "var(--q-mono)", "font-size": "13px" }} />
    </Card>
  );
}

/** TaskProperties: the workspace inspector. Facts (PropertyList) + one choice (RadioGroup). */
export function TaskProperties(props: { task: Task; note: string }) {
  const [prio, setPrio] = createSignal(1);
  const words = () => `${props.note.split(/\s+/).filter(Boolean).length}`;
  return (
    <div style={{ display: "grid", gap: "20px", padding: "4px" }}>
      <h3>Properties</h3>
      <PropertyList items={[["Task", props.task.title], ["Tag", props.task.tag], ["Due", props.task.due], ["Lines", `${props.note.split("\n").length}`], ["Words", words()]]} />
      <Field label="Priority">{() => <RadioGroup label="Priority" options={["Low", "Normal", "High"]} value={prio()} onChange={setPrio} />}</Field>
    </div>
  );
}

/** ProjectTable: every project with its counts. States: ideal, error (Retry). */
export function ProjectTable(props: { error?: boolean; onRetry?: () => void; selected?: number; onSelect?: (i: number) => void }) {
  const [own, setOwn] = createSignal(1);
  const sel = () => props.selected ?? own();
  const setSel = (i: number) => (props.onSelect ? props.onSelect(i) : setOwn(i));
  return (
    <Card>
      <h3 style={{ "margin-bottom": "12px" }}>Projects</h3>
      <Show when={!props.error} fallback={<ErrorState what="Couldn't load projects" why="The sync server didn't answer (timeout)." onRetry={props.onRetry} />}>
        <Table label="Projects" selected={sel()} onSelect={setSel}
          columns={[{ header: "Project" }, { header: "Open", num: true, width: "70px" }, { header: "Done", num: true, width: "70px" }, { header: "Changed", num: true, width: "120px" }]}
          rows={PROJECT_ROWS} />
      </Show>
    </Card>
  );
}

// ── Dashboard: activity ──

/** Activity: tasks done per day (Histogram) and the change vs the week before (DeltaBadge). */
export function Activity() {
  const prev = DONE_PER_DAY.slice(0, 7).reduce((a, b) => a + b, 0);
  const now = DONE_PER_DAY.slice(7).reduce((a, b) => a + b, 0);
  return (
    <Card>
      <h3 style={{ "margin-bottom": "12px" }}>Activity</h3>
      <Text tone="muted">done per day · 14 days</Text>
      <div style={{ margin: "10px 0" }}><Histogram values={DONE_PER_DAY} width={220} height={48} label="Tasks done per day" /></div>
      <div style={{ display: "flex", gap: "10px", "align-items": "baseline" }}><Text>{now} this week</Text><DeltaBadge change={now / prev - 1} higherIsBetter /><Text tone="faint">vs last week</Text></div>
    </Card>
  );
}

// ── Settings: sync ──

/** SyncSettings: how often to sync (Select) and how early to remind (ParamRow). */
export function SyncSettings() {
  const [every, setEvery] = createSignal("5 minutes");
  const [lead, setLead] = createSignal(15);
  return (
    <div style={{ display: "grid", gap: "14px", "margin-top": "8px" }}>
      <h3>Sync</h3>
      <Field label="Sync every">{(id) => <Select id={id} value={every()} onValue={setEvery} options={["1 minute", "5 minutes", "Hourly", "Manually"]} />}</Field>
      <ParamRow label="Remind before" value={lead()} onValue={setLead} min={0} max={120} step={5} unit="min" />
    </div>
  );
}

// ── Workspace: project tree ──

/** ProjectTree: projects and files. Keeps which folders are open. */
export function ProjectTree() {
  const [open, setOpen] = createSignal(new Set(["qt", "qt/docs"]));
  const [sel, setSel] = createSignal("qt/docs/structure");
  return (
    <div style={{ display: "grid", gap: "8px" }}>
      <h3 style={{ padding: "0 8px" }}>Files</h3>
      <TreeView label="Files" items={TREE} open={open()} selected={sel()} onSelect={setSel}
        onToggle={(id, o) => setOpen((s) => { const n = new Set(s); if (o) n.add(id); else n.delete(id); return n; })} />
    </div>
  );
}

// ── Domain demos (catalog) ──

/** FrameBudget: frame times against 16.6 ms; spikes red; change vs last build. */
export function FrameBudget() {
  const avg = FRAME_MS.reduce((a, b) => a + b, 0) / FRAME_MS.length;
  const over = FRAME_MS.filter((v) => v > 16.6).length;
  return (
    <Card>
      <h3 style={{ "margin-bottom": "8px" }}>Frame time</h3>
      <div style={{ display: "flex", gap: "16px", "align-items": "baseline", "margin-bottom": "12px" }}>
        <Text>avg {avg.toFixed(1)} ms</Text><Text tone="err">{over} frames over 16.6 ms</Text><DeltaBadge change={-0.083} higherIsBetter={false} /><Text tone="faint">vs last build</Text>
      </div>
      <Histogram values={FRAME_MS} max={25} budget={16.6} width={480} height={80} label="Frame times" />
    </Card>
  );
}

/** GpuMemory: memory per asset type, 800 MB budget each. */
export function GpuMemory() {
  return <Card><h3 style={{ "margin-bottom": "12px" }}>GPU memory</h3><BarChart items={MEMORY_MB} unit="MB" max={1000} budget={800} /></Card>;
}

/** MaterialParams: a tech-art tuning panel. */
export function MaterialParams() {
  const [r, setR] = createSignal(0.45), [m, setM] = createSignal(0), [n, setN] = createSignal(1.2), [t, setT] = createSignal(4);
  return (
    <Card style={{ display: "grid", gap: "10px" }}>
      <h3>Material</h3>
      <ParamRow label="Roughness" value={r()} onValue={setR} min={0} max={1} step={0.01} decimals={2} />
      <ParamRow label="Metallic" value={m()} onValue={setM} min={0} max={1} step={0.01} decimals={2} />
      <ParamRow label="Normal strength" value={n()} onValue={setN} min={0} max={4} step={0.1} decimals={1} />
      <ParamRow label="Tiling" value={t()} onValue={setT} min={1} max={16} unit="×" />
    </Card>
  );
}

/** ServerLoad: CPU trend of one server, 80 % budget. */
export function ServerLoad(props: { name: string; cpu: number[]; compact?: boolean }) {
  const w = () => (props.compact ? 56 : 140);
  return (
    <div style={{ display: "grid", "grid-template-columns": `minmax(0, 1fr) ${w()}px 36px`, gap: props.compact ? "8px" : "12px", "align-items": "center", "max-width": props.compact ? undefined : "260px" }}>
      <Text tone="muted">{props.name}</Text>
      <Sparkline values={props.cpu} max={100} budget={80} width={w()} label={`${props.name} CPU`} />
      <Text>{props.cpu[props.cpu.length - 1]} %</Text>
    </div>
  );
}


// ── Moved off the pages (pages only place elements) ──

/** Kpis: the four dashboard numbers. */
export function Kpis() {
  const kpi = (value: string, label: string) => <Card class="q-kpi"><div class="q-kpi-value">{value}</div><div class="q-kpi-label">{label}</div></Card>;
  return <div class="q-kpis">{kpi("12", "open tasks")}{kpi("7/12", "done this week")}{kpi("2", "overdue")}{kpi("4", "projects")}</div>;
}

/** Attention: what needs looking at now. */
export function Attention() {
  return (
    <Card>
      <h3 style={{ "margin-bottom": "12px" }}>Needs attention</h3>
      <div style={{ display: "grid", gap: "8px" }}>
        <StatusIcon kind="err">Fix save crash on empty file</StatusIcon>
        <StatusIcon kind="warn">Port settings screen: date unreadable</StatusIcon>
      </div>
    </Card>
  );
}

/** OutputLog: the workspace's output pane. */
export function OutputLog() {
  return (
    <Card style={{ height: "100%" }}>
      <div class="q-layout-stream">
        <For each={LOG}>{([time, kind, msg]) => <div><Text tone="faint">{time}</Text><StatusIcon kind={kind} /><Text>{msg}</Text></div>}</For>
      </div>
    </Card>
  );
}

// ── Security (audit page) ──

/** ServerList: servers with their CPU trend (nav of the audit page). */
export function ServerList() {
  return (
    <div style={{ display: "grid", gap: "10px", padding: "4px 8px" }}>
      <h3>Servers</h3>
      <For each={SERVERS}>{([n, cpu]) => <ServerLoad compact name={n} cpu={cpu} />}</For>
    </div>
  );
}

/** FindingsList: findings worst first, filtered by the time range (Segmented). States: ideal, empty. */
export function FindingsList(props: { findings: Finding[]; selected: number; onSelect: (i: number) => void; range: number; onRange: (i: number) => void }) {
  const bad = () => props.findings.filter((f) => f.sev === "critical" || f.sev === "high").length;
  return (
    <Card style={{ display: "grid", gap: "12px" }}>
      <div style={{ display: "flex", "align-items": "center", gap: "12px" }}>
        <Text>{props.findings.length} findings</Text><Text tone="err">{bad()} high or worse</Text>
        <span style={{ "margin-left": "auto" }}><Segmented label="Time range" options={RANGES} value={props.range} onChange={props.onRange} /></span>
      </div>
      <Show when={props.findings.length} fallback={<EmptyState message="No findings in this range." hint={<>Press <Kbd>5</Kbd> for the last 30 days.</>} />}>
        <List label="Findings">
          <For each={props.findings}>
            {(f, i) => <Row title={<><SeverityBadge level={f.sev} /> {f.title}</>} meta={`${f.host} · ${f.age}`} selected={i() === props.selected} onClick={() => props.onSelect(i())} />}
          </For>
        </List>
      </Show>
    </Card>
  );
}

/** FindingDetail: the selected finding, the leaked secret (masked) and the actions. */
export function FindingDetail(props: { finding?: Finding; revealed?: boolean; onRevoke: () => void; onIgnore: () => void; onFix?: () => void }) {
  return (
    <Show when={props.finding} fallback={<EmptyState message="Select a finding." />}>
      {(f) => (
        <div style={{ display: "grid", gap: "16px", padding: "4px" }}>
          <div><SeverityBadge level={f().sev} /></div>
          <h2 style={{ margin: 0 }}>{f().title}</h2>
          <Props items={[["Host", f().host], ["Found", `${f().age} ago`], ["Status", "open"]]} />
          <Text tone="muted">{f().detail}</Text>
          <Show when={f().secret}>{(sec) => <Field label="Leaked key">{() => <SecretField value={sec()} label="Leaked key" shown={props.revealed} />}</Field>}</Show>
          <ButtonGroup align="end" buttons={f().secret
            ? [{ label: "Ignore", kind: "ghost", onClick: props.onIgnore }, { label: "Revoke key", kind: "danger", onClick: props.onRevoke }]
            : [{ label: "Ignore", kind: "ghost", onClick: props.onIgnore }, { label: "Fix", kind: "primary", onClick: props.onFix }]} />
        </div>
      )}
    </Show>
  );
}

/** FirewallRules: rules in order, with hits. */
export function FirewallRules() {
  return (
    <Card>
      <h3 style={{ "margin-bottom": "12px" }}>Firewall · cache-1</h3>
      <Table label="Firewall rules" selected={3} rows={RULES}
        columns={[{ header: "#", num: true, width: "40px" }, { header: "Action", width: "80px" }, { header: "Port", width: "100px" }, { header: "Source" }, { header: "Hits", num: true, width: "90px" }]} />
    </Card>
  );
}
