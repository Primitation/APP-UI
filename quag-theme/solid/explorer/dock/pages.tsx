/**
 * UI: the dock demo pages (server / Docker administration). Pages only place elements; keys once with createPageKeys.
 */
import * as C from "../consts";
import { createSignal, onCleanup } from "solid-js";
import {
  AppHeader, AppShell, ConfirmDialog, createPageKeys, DiffView, Freshness, isUp, Layout, PageHead, RunProgress, type RunState,
  type StepSpec, StatusBar, Toast, TypeToConfirm,
} from "../../src";
import { CONTAINERS, DEPLOY_LOG, DIFF, FAIL_LOG, HOSTS, STEP_NAMES, STEP_TOOK } from "./data";
import { ContainerInspector, ContainerTable, DiskUse, HostCard, ImageTable, NeedsAttention, Release, StackTree } from "./elements";

const header = (location: string) => <AppHeader app="dock" env="prod" location={location} />;

/** List → Detail: stacks · containers · the selected container. */
export function ContainersPage() {
  const [sel, setSel] = createSignal(1);
  const [states, setStates] = createSignal<RunState[]>(CONTAINERS.map((c) => c.state));
  const [remove, setRemove] = createSignal(false);
  const [reveal, setReveal] = createSignal(false);
  const [toast, setToast] = createSignal<string>();
  const set = (i: number, s: RunState) => setStates((all) => all.map((x, j) => (j === i ? s : x)));
  // busy states settle like the daemon would
  const later = (i: number, s: RunState) => { const t = setTimeout(() => set(i, s), C.SETTLE_MS); onCleanup(() => clearTimeout(t)); };
  const restart = () => { set(sel(), { kind: "restarting" }); later(sel(), { kind: "running" }); setToast(`Restarting ${CONTAINERS[sel()].name}`); };
  const startStop = () => { const up = isUp(states()[sel()]); set(sel(), { kind: up ? "stopping" : "starting" }); later(sel(), { kind: up ? "stopped" : "running" }); };
  const keys = createPageKeys([
    { key: "↑ ↓", what: "previous / next", group: "Move", hint: "move", match: ["ArrowUp", "ArrowDown"], run: (k) => setSel((i) => Math.max(0, Math.min(CONTAINERS.length - 1, i + (k === "ArrowDown" ? 1 : -1)))) },
    { key: "R", what: "restart", group: "Act", hint: "restart", command: "Restart container", match: ["r"], run: restart },
    { key: "S", what: "stop / start", group: "Act", hint: "stop/start", command: "Stop or start container", match: ["s"], run: startStop },
    { key: "L", what: "follow its logs", group: "Act", command: "Show logs", match: ["l"], run: () => setToast(`Following logs of ${CONTAINERS[sel()].name}`) },
    { key: "E", what: "show / hide secret env values", group: "Act", command: "Show secrets", match: ["e"], run: () => setReveal((v) => !v) },
    { key: "Del", what: "remove (asks first)", group: "Act", command: "Remove container", danger: true, match: ["Delete"], run: () => setRemove(true) },
  ]);
  return (
    <AppShell header={header("containers · prod-1")} nav={<StackTree />}
      inspector={<ContainerInspector container={CONTAINERS[sel()]} state={states()[sel()]} reveal={reveal()} onRestart={restart} onStartStop={startStop} onRemove={() => setRemove(true)} />}
      status={<StatusBar hints={keys.hints()} />}>
      <PageHead title="Containers"><Freshness age={C.AGE_FRESH_SECS} /></PageHead>
      <ContainerTable states={states()} selected={sel()} onSelect={setSel} />
      <ConfirmDialog open={remove()} title="Remove container" message={`“${CONTAINERS[sel()].name}” is removed; its volumes are kept.`} confirmLabel="Remove" danger
        onConfirm={() => { setRemove(false); setToast(`Removed ${CONTAINERS[sel()].name} (demo: it stays listed)`); }} onCancel={() => setRemove(false)} />
      {keys.overlays}
      <Toast open={!!toast()} kind="ok" message={toast() ?? ""} onClose={() => setToast()} />
    </AppShell>
  );
}

/** Dashboard: every host and what needs attention. */
export function HostsPage() {
  const [toast, setToast] = createSignal<string>();
  const [sel, setSel] = createSignal(0);
  const keys = createPageKeys([
    { key: "← →", what: "previous / next host", group: "Move", hint: "move", match: ["ArrowLeft", "ArrowRight"],
      run: (k) => setSel((i) => Math.max(0, Math.min(HOSTS.length - 1, i + (k === "ArrowRight" ? 1 : -1)))) },
    { key: "Enter", what: "open host", group: "Move", hint: "open", match: ["Enter"], run: () => setToast("Host page: not in the demo") },
    { key: "R", what: "refresh now", group: "Act", hint: "refresh", command: "Refresh", match: ["r"], run: () => setToast("Refreshed 4 hosts") },
  ]);
  return (
    <AppShell header={header("hosts")} status={<StatusBar hints={keys.hints()} />}>
      <PageHead title="Hosts"><Freshness age={C.AGE_STALE_SECS} /></PageHead>
      <Layout kind="dashboard">
        {HOSTS.map((h, i) => <HostCard host={h} selected={i === sel()} />)}
        <div style={{ "grid-column": "1 / -1" }}><NeedsAttention /></div>
      </Layout>
      {keys.overlays}
      <Toast open={!!toast()} kind="ok" message={toast() ?? ""} onClose={() => setToast()} />
    </AppShell>
  );
}

/** Workspace: review → run → result. The run is a zone of the page, not a modal. */
export function DeployPage() {
  const [step, setStep] = createSignal<number>(); // undefined = reviewing
  const [fail, setFail] = createSignal(false);
  const [confirm, setConfirm] = createSignal(false);
  const [rollback, setRollback] = createSignal(false);
  const [toast, setToast] = createSignal<string>();
  let timer: ReturnType<typeof setInterval> | undefined;
  onCleanup(() => clearInterval(timer));
  const start = () => {
    setConfirm(false); setStep(0); clearInterval(timer);
    timer = setInterval(() => setStep((s) => { const n = (s ?? 0) + 1; if (n > STEP_NAMES.length || (fail() && n > C.DEPLOY_FAIL_STEP)) clearInterval(timer); return n; }), C.DEPLOY_STEP_MS);
  };
  const steps = (): StepSpec[] => STEP_NAMES.map((name, i) => {
    const d = step();
    let state: StepSpec["state"] = "waiting";
    if (d !== undefined) {
      if (fail() && d > C.DEPLOY_FAIL_STEP && i === C.DEPLOY_FAIL_STEP) state = "failed";
      else if (fail() && d > C.DEPLOY_FAIL_STEP && i > C.DEPLOY_FAIL_STEP) state = "skipped";
      else if (i < d) state = "done";
      else if (i === d) state = "running";
    }
    return { name, state, took: state === "done" ? STEP_TOOK[i] : undefined };
  });
  const failed = () => steps().some((s) => s.state === "failed");
  const output = () => failed() ? [...DEPLOY_LOG.slice(0, 6), FAIL_LOG] : DEPLOY_LOG.slice(0, (step() ?? 0) * 2);
  const keys = createPageKeys([
    { key: "D", what: "deploy (type the stack name: prod)", group: "Act", hint: "deploy", command: "Deploy shop 1.9.0", danger: true, match: ["d"], run: () => setConfirm(true) },
    { key: "Z", what: "roll back to 1.8.2 (asks first)", group: "Act", hint: "roll back", command: "Roll back", match: ["z"], run: () => setRollback(true) },
    { key: "F", what: "demo: make the health check fail", group: "Demo", command: "Demo: fail next deploy", match: ["f"], run: () => { setFail(!fail()); setToast(fail() ? "Next deploy fails its health check" : "Next deploy succeeds"); } },
  ]);
  return (
    <AppShell header={header("deploy · shop")} inspector={<Release steps={steps()} />}
      status={<StatusBar hints={keys.hints()} status={failed() ? { kind: "err", text: "failed · 1.8.2 still live" } : step() === undefined ? undefined : { kind: "info", text: "deploying…" }} />}>
      <div style={{ height: "calc(100vh - 110px)", display: "grid" }}>
        {step() === undefined
          ? <DiffView title="Changes · compose.yaml" lines={DIFF} />
          : <RunProgress steps={steps()} output={output()} onCancel={() => { clearInterval(timer); setStep(); }} onRetry={start} onRollback={() => setRollback(true)} onClose={() => setStep()} />}
      </div>
      <TypeToConfirm open={confirm()} title="Deploy to prod" message="Deploy shop 1.9.0 to prod-1 and prod-2?" name="shop" confirmLabel="Deploy" onConfirm={start} onCancel={() => setConfirm(false)} />
      <ConfirmDialog open={rollback()} title="Roll back" message="Roll back shop to 1.8.2? Containers restart one by one; no downtime." confirmLabel="Roll back"
        onConfirm={() => { setRollback(false); clearInterval(timer); setStep(); setToast("Rolled back to 1.8.2"); }} onCancel={() => setRollback(false)} />
      {keys.overlays}
      <Toast open={!!toast()} kind="ok" message={toast() ?? ""} onClose={() => setToast()} />
    </AppShell>
  );
}

/** List → Detail: images · disk use and prune. */
export function ImagesPage() {
  const [sel, setSel] = createSignal(3);
  const [prune, setPrune] = createSignal(false);
  const [toast, setToast] = createSignal<string>();
  const keys = createPageKeys([
    { key: "↑ ↓", what: "previous / next", group: "Move", hint: "move", match: ["ArrowUp", "ArrowDown"], run: (k) => setSel((i) => Math.max(0, i + (k === "ArrowDown" ? 1 : -1))) },
    { key: "P", what: "prune unused (type to confirm)", group: "Act", hint: "prune", command: "Prune unused images", danger: true, match: ["p"], run: () => setPrune(true) },
  ]);
  return (
    <AppShell header={header("images · prod-1")} inspector={<DiskUse onPrune={() => setPrune(true)} />} status={<StatusBar hints={keys.hints()} />}>
      <PageHead title="Images"><Freshness age={C.AGE_FRESH_SECS} /></PageHead>
      <ImageTable selected={sel()} onSelect={setSel} />
      <TypeToConfirm open={prune()} title="Prune images" message="Delete 4 unused images and the build cache (4.6 GiB)?" name="prune" confirmLabel="Prune"
        onConfirm={() => { setPrune(false); setToast("Pruned 4.6 GiB"); }} onCancel={() => setPrune(false)} />
      {keys.overlays}
      <Toast open={!!toast()} kind="ok" message={toast() ?? ""} onClose={() => setToast()} />
    </AppShell>
  );
}

export const DOCK_PAGES = [
  { id: "containers", name: "ContainersPage (dock)", layout: "List → Detail", page: ContainersPage },
  { id: "hosts", name: "HostsPage (dock)", layout: "Dashboard", page: HostsPage },
  { id: "deploy", name: "DeployPage (dock)", layout: "Workspace", page: DeployPage },
  { id: "images", name: "ImagesPage (dock)", layout: "List → Detail", page: ImagesPage },
] as const;
