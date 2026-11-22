/**
 * UI: pages of the demo app. Each page only places ELEMENTS in the AppShell zones,
 * and declares its keys ONCE with createPageKeys (status bar hints, ? help and Ctrl K palette read it).
 * One page per layout of STRUCTURE.md §7.3.
 */
import * as C from "../consts";
import { createSignal } from "solid-js";
import {
  AppHeader, AppShell, Button, ConfirmDialog, consts, createPageKeys, EmptyState, Field, Input, Kbd, Layout, LogViewer, PageHead, Split,
  StatusBar, Tabs, Toast, TypeToConfirm,
} from "../../src";
import { bigLog, FINDINGS, inProject, inTab, NOTE, PROJECT_ROWS, PROJECTS, projectOfRow, RANGE_HOURS, TAB_EMPTY, TABS, TASKS } from "./data";

/** The project the tasks page opens on (the dashboard's Enter sets it). */
const [openProject, setOpenProject] = createSignal(0);
import {
  Activity, Attention, EditorTools, FindingDetail, FindingsList, FirewallRules, Kpis, NoteEditor, NotificationSettings,
  OutputLog, ProjectNav, ProjectTable, ProjectTree, ServerList, SyncSettings, TaskInspector, TaskList, TaskProperties,
} from "./elements";
import mascot from "../../../web/quagsire.gif";

/** List → Detail */
export function TasksPage() {
  const [tab, setTab] = createSignal(0);
  const [tasks, setTasks] = createSignal(TASKS);
  const [sel, setSel] = createSignal(2);
  const [toast, setToast] = createSignal<string>();
  const [project, setProject] = createSignal(openProject());
  const [del, setDel] = createSignal(false);
  const visible = () => tasks().filter((t) => inTab(t, tab()) && inProject(t, project()));
  const current = () => visible().find((t) => t.id === sel()) ?? visible()[0];
  const switchTab = (i: number) => { setTab(i); const first = visible()[0]; if (first) setSel(first.id); };
  // picking a project selects its first task
  const pick = (p: number) => { setProject(p); const first = visible()[0]; if (first) setSel(first.id); };
  const toggle = () => {
    const t = current();
    if (!t) return;
    setTasks((ts) => ts.map((x) => (x.id === t.id ? { ...x, done: !x.done } : x)));
    setToast(t.done ? "Task reopened" : "Task marked done");
  };
  const remove = () => {
    const t = current();
    setDel(false);
    if (!t) return;
    setTasks((ts) => ts.filter((x) => x.id !== t.id));
    setToast("Task deleted");
  };
  const move = (d: number) => {
    const v = visible(), i = v.findIndex((t) => t.id === sel());
    const t = v[Math.max(0, Math.min(v.length - 1, i + d))];
    if (t) setSel(t.id);
  };
  const keys = createPageKeys([
    { key: "↑ ↓", what: "previous / next task", group: "Move", hint: "move", match: ["ArrowUp", "ArrowDown"], run: (k) => move(k === "ArrowDown" ? 1 : -1) },
    { key: "PgUp PgDn", what: "previous / next project", group: "Move", hint: "project", match: ["PageUp", "PageDown"], run: (k) => pick(Math.max(0, Math.min(PROJECTS.length - 1, project() + (k === "PageDown" ? 1 : -1)))) },
    { key: "1 2 3", what: "switch tab: today, upcoming, done", group: "Move", match: ["1", "2", "3"], command: "Go to tab: Today", run: (k) => switchTab(Number(k || 1) - 1) },
    { key: "N", what: "new task", group: "Act", hint: "new", command: "New task", match: ["n"], run: () => setToast("New task (demo)") },
    { key: "X", what: "mark done / reopen", group: "Act", command: "Mark task done", match: ["x"], run: toggle },
    { key: "Del", what: "delete (asks first)", group: "Act", hint: "delete", command: "Delete task", danger: true, match: ["Delete"], run: () => { if (current()) setDel(true); } },
  ]);
  return (
    <AppShell
      header={<AppHeader app="pond" location="tasks"><Tabs tabs={TABS} active={tab()} onSelect={switchTab} /></AppHeader>}
      nav={<ProjectNav active={project()} onSelect={pick} tasks={tasks()} />}
      inspector={<TaskInspector task={current()} onToggle={toggle} />}
      status={<StatusBar hints={keys.hints()} status={{ kind: "ok", text: "Saved 2s ago" }} />}
    >
      <PageHead title={`${TABS[tab()]}${project() ? ` · ${PROJECTS[project()]}` : ""}`}><Button kind="primary">＋ New task</Button></PageHead>
      <TaskList tasks={visible()} selected={current()?.id} onSelect={setSel} mascot={mascot} empty={TAB_EMPTY[tab()]} />
      <ConfirmDialog open={del()} title="Delete task" message={`“${current()?.title ?? ""}” can't be restored.`} confirmLabel="Delete" cancelLabel="Keep" danger
        onConfirm={remove} onCancel={() => setDel(false)} />
      {keys.overlays}
      <Toast open={!!toast()} kind="ok" message={toast() ?? ""} onClose={() => setToast()} />
    </AppShell>
  );
}

/** Dashboard */
export function DashboardPage() {
  const [toast, setToast] = createSignal<string>();
  const [row, setRow] = createSignal(1);
  const open = () => { setOpenProject(projectOfRow(row())); location.hash = "/page/tasks"; };
  const keys = createPageKeys([
    { key: "↑ ↓", what: "previous / next project", group: "Move", hint: "move", match: ["ArrowUp", "ArrowDown"],
      run: (k) => setRow((i) => Math.max(0, Math.min(PROJECT_ROWS.length - 1, i + (k === "ArrowDown" ? 1 : -1)))) },
    { key: "Enter", what: "open the project's tasks", group: "Act", hint: "open", command: "Open project", match: ["Enter"], run: open },
    { key: "R", what: "refresh", group: "Act", hint: "refresh", command: "Refresh", match: ["r"], run: () => setToast("Refreshed") },
  ]);
  return (
    <AppShell
      header={<AppHeader app="pond" location="overview" />}
      nav={<ProjectNav active={projectOfRow(row())} />}
      status={<StatusBar hints={keys.hints()} status={{ kind: "ok", text: "Synced" }} />}
    >
      <PageHead title="Overview" />
      <Layout kind="dashboard">
        <Kpis />
        <Attention />
        <Activity />
        <div style={{ "grid-column": "1 / -1" }}><ProjectTable selected={row()} onSelect={setRow} /></div>
      </Layout>
      {keys.overlays}
      <Toast open={!!toast()} kind="ok" message={toast() ?? ""} onClose={() => setToast()} />
    </AppShell>
  );
}

/** Form */
export function SettingsPage() {
  const [name, setName] = createSignal("My pond");
  const [toast, setToast] = createSignal<string>();
  const keys = createPageKeys([
    { key: "Ctrl S", what: "save", group: "Act", hint: "save", command: "Save settings", match: ["ctrl+s"], run: () => setToast("Settings saved") },
  ]);
  return (
    <AppShell
      header={<AppHeader app="pond" location="settings" />}
      nav={<ProjectNav active={-1} />}
      status={<StatusBar hints={keys.hints()} />}
    >
      <PageHead title="Settings" />
      <Layout kind="form">
        <h3 style={{ margin: "8px 0 12px" }}>General</h3>
        <Field label="Workspace name" help="Shown in the window title.">{(id) => <Input id={id} value={name()} onValue={setName} />}</Field>
        <Field label="Week starts on" error="Pick Monday or Sunday.">{(id, invalid) => <Input id={id} value="Friday" invalid={invalid} />}</Field>
        <NotificationSettings />
        <SyncSettings />
        <div class="q-btn-group end" style={{ "border-top": "1px solid var(--q-border)", "padding-top": "16px" }}>
          <Button kind="ghost">Cancel</Button>
          <Button kind="primary" onClick={() => setToast("Settings saved")}>Save</Button>
        </div>
      </Layout>
      {keys.overlays}
      <Toast open={!!toast()} kind="ok" message={toast() ?? ""} onClose={() => setToast()} />
    </AppShell>
  );
}

/** Stream */
const SERVER_LOG = bigLog(C.LOG_LINES);

export function LogPage() {
  const keys = createPageKeys([
    { key: "type", what: "filter the lines (the first letter jumps into the filter)", group: "Filter", hint: "filter", command: "Filter lines",
      match: [..."abcdefghijklmnopqrstuvwxyz0123456789/"], run: (k) => {
        const input = document.querySelector<HTMLInputElement>(".q-log input");
        if (!input) return;
        input.focus();
        if (k && k !== "/") { input.value += k; input.dispatchEvent(new Event("input", { bubbles: true })); }
      } },
    { key: "End", what: "follow the newest line", group: "Move", hint: "follow", command: "Follow newest", match: ["End"],
      run: () => document.querySelector<HTMLButtonElement>(".q-log button[aria-pressed]")?.click() },
  ]);
  return (
    <AppShell
      header={<AppHeader app="pond" location="server log · api-1" />}
      status={<StatusBar hints={keys.hints()} status={{ kind: "info", text: "api-1 · live" }} />}
    >
      <div style={{ height: "calc(100vh - 130px)" }}><LogViewer lines={SERVER_LOG} /></div>
      {keys.overlays}
    </AppShell>
  );
}

/** Focus */
export function WelcomePage() {
  const [toast, setToast] = createSignal<string>();
  const create = () => setToast("New task (demo)");
  const keys = createPageKeys([
    { key: "N", what: "create a task", group: "Act", hint: "new task", command: "New task", match: ["n"], run: create },
  ]);
  return (
    <AppShell header={<AppHeader app="pond" />} status={<StatusBar hints={keys.hints()} />}>
      <Layout kind="focus">
        <EmptyState
          mascot={mascot}
          message="Welcome to pond."
          hint={<>Your tasks, calm and in one place. Press <Kbd>N</Kbd> to add the first one.</>}
          action={<Button kind="primary" onClick={create}>Create a task</Button>}
        />
      </Layout>
      {keys.overlays}
      <Toast open={!!toast()} kind="ok" message={toast() ?? ""} onClose={() => setToast()} />
    </AppShell>
  );
}

/** Workspace */
/** Hand a key to the file tree, as if it had focus. */
const treeKey = (key = "") => {
  const tree = document.querySelector<HTMLElement>(".q-tree");
  tree?.focus();
  tree?.dispatchEvent(new KeyboardEvent("keydown", { key, bubbles: true, cancelable: true }));
};

export function WorkspacePage() {
  const [tool, setTool] = createSignal(1);
  const [note, setNote] = createSignal(NOTE);
  const [split, setSplit] = createSignal(0.72);
  const [toast, setToast] = createSignal<string>();
  const keys = createPageKeys([
    // the file tree's own keys, from anywhere on the page (not while typing in the editor)
    { key: "↑ ↓", what: "move in the file tree", group: "Move", match: ["ArrowUp", "ArrowDown"], run: (k) => treeKey(k) },
    { key: "← →", what: "close / open a folder (← on a file: its folder)", group: "Move", hint: "fold", match: ["ArrowLeft", "ArrowRight"], run: (k) => treeKey(k) },
    { key: "Ctrl S", what: "save", group: "Act", hint: "save", command: "Save note", match: ["ctrl+s"], run: () => setToast("Note saved") },
    { key: "[ ]", what: "smaller / bigger editor", group: "View", hint: "resize", command: "Bigger editor", match: ["[", "]"],
      run: (k) => setSplit((r) => Math.min(C.SPLIT_MAX, Math.max(C.SPLIT_MIN, r + (k === "[" ? -consts.SPLIT_STEP : consts.SPLIT_STEP)))) },
  ]);
  return (
    <AppShell
      header={<AppHeader app="pond" location="editor" />}
      nav={<ProjectTree />}
      inspector={<TaskProperties task={TASKS[1]} note={note()} />}
      status={<StatusBar hints={keys.hints()} status={{ kind: "info", text: "Unsaved changes" }} />}
    >
      <Layout kind="workspace">
        <EditorTools active={tool()} onSelect={setTool} />
        <div style={{ height: "calc(100vh - 140px)" }}>
          <Split ratio={split()} onRatio={setSplit} label="Resize editor and output" first={<NoteEditor value={note()} onValue={setNote} />} second={<OutputLog />} />
        </div>
      </Layout>
      {keys.overlays}
      <Toast open={!!toast()} kind="ok" message={toast() ?? ""} onClose={() => setToast()} />
    </AppShell>
  );
}

/** List → Detail, security flavour: servers · findings · detail. */
export function AuditPage() {
  const [range, setRange] = createSignal(2);
  const [sel, setSel] = createSignal(0);
  const [revoke, setRevoke] = createSignal(false);
  const [revealed, setRevealed] = createSignal(false);
  const [toast, setToast] = createSignal<string>();
  const findings = () => FINDINGS.filter((f) => f.hours <= RANGE_HOURS[range()]);
  const current = () => findings()[Math.min(sel(), findings().length - 1)];
  const keys = createPageKeys([
    { key: "↑ ↓", what: "previous / next finding", group: "Move", hint: "move", match: ["ArrowUp", "ArrowDown"],
      run: (k) => setSel((i) => Math.max(0, Math.min(findings().length - 1, i + (k === "ArrowDown" ? 1 : -1)))) },
    { key: "1 – 5", what: "time range 15m … 30d", group: "Move", hint: "range", match: ["1", "2", "3", "4", "5"], run: (k) => { setRange(Number(k || 3) - 1); setSel(0); } },
    { key: "S", what: "show / hide the secret", group: "Act", hint: "show key", command: "Show / hide secret", match: ["s"], run: () => setRevealed((v) => !v) },
    { key: "F", what: "fix", group: "Act", command: "Fix finding", match: ["f"], run: () => setToast("Fix scheduled") },
    { key: "I", what: "ignore", group: "Act", command: "Ignore finding", match: ["i"], run: () => setToast("Ignored") },
    { key: "X", what: "revoke the key (type to confirm)", group: "Act", hint: "revoke", command: "Revoke key", danger: true, match: ["x"], run: () => setRevoke(true) },
  ]);
  return (
    <AppShell
      header={<AppHeader app="pond" location="security audit" />}
      nav={<ServerList />}
      inspector={<FindingDetail finding={current()} revealed={revealed()} onRevoke={() => setRevoke(true)} onIgnore={() => setToast("Ignored")} onFix={() => setToast("Fix scheduled")} />}
      status={<StatusBar hints={keys.hints()} status={{ kind: "warn", text: "1 critical" }} />}
    >
      <PageHead title="Findings" />
      <div style={{ display: "grid", gap: "16px" }}>
        <FindingsList findings={findings()} selected={sel()} onSelect={setSel} range={range()} onRange={(i) => { setRange(i); setSel(0); }} />
        <FirewallRules />
      </div>
      <TypeToConfirm open={revoke()} title="Revoke key" message="Every client using this API key stops working." name="api-1" confirmLabel="Revoke"
        onCancel={() => setRevoke(false)} onConfirm={() => { setRevoke(false); setToast("Key revoked"); }} />
      {keys.overlays}
      <Toast open={!!toast()} kind="ok" message={toast() ?? ""} onClose={() => setToast()} />
    </AppShell>
  );
}

import { DOCK_PAGES } from "../dock/pages";
import { FORGE_PAGES } from "../forge/pages";

export const PAGES = [
  { id: "tasks", name: "TasksPage", layout: "List → Detail", page: TasksPage },
  { id: "dashboard", name: "DashboardPage", layout: "Dashboard", page: DashboardPage },
  { id: "workspace", name: "WorkspacePage", layout: "Workspace", page: WorkspacePage },
  { id: "settings", name: "SettingsPage", layout: "Form", page: SettingsPage },
  { id: "log", name: "LogPage", layout: "Stream", page: LogPage },
  { id: "audit", name: "AuditPage", layout: "List → Detail", page: AuditPage },
  ...DOCK_PAGES,
  ...FORGE_PAGES,
  { id: "welcome", name: "WelcomePage", layout: "Focus", page: WelcomePage },
] as const;
