/** ELEMENTS of the dock demo: molecules wired to its data. They never set colors. */
import { createSignal, For, Show } from "solid-js";
import {
  BarChart, ButtonGroup, Card, EmptyState, ErrorState, fmt, KeyValueRows, List, ListRow, Meter, PortMap, PropertyList, RunStateLabel,
  type RunState, ShortId, Skeleton, Sparkline, type StepSpec, StepList, Status, Table, Text, TreeView, isUp,
} from "../../src";
import { ATTENTION, type Container, CONTAINERS, GIB_, type Host, IMAGES } from "./data";

/** ContainerTable: state, name, image, CPU trend, memory of its limit, uptime. States: unreachable, loading, empty. */
export function ContainerTable(props: { states: RunState[]; selected: number; onSelect: (i: number) => void; reachable?: boolean; loading?: boolean }) {
  const up = () => props.states.filter(isUp).length;
  return (
    <Card>
      <h3 style={{ "margin-bottom": "12px" }}>Containers · {up()}/{props.states.length} up</h3>
      <Show when={props.reachable !== false} fallback={<ErrorState what="Can't reach the Docker daemon on prod-1." why="ssh: connect to prod-1:22 timed out after 10s" onRetry={() => {}} />}>
        <Show when={!props.loading} fallback={<div style={{ display: "grid", gap: "14px" }}><For each={[0.9, 0.7, 0.8, 0.6]}>{(w) => <Skeleton width={`${w * 100}%`} />}</For></div>}>
          <Show when={props.states.length} fallback={<EmptyState message="No containers on this host." />}>
            <Table label="Containers" selected={props.selected} onSelect={props.onSelect}
              columns={[{ header: "State", width: "140px" }, { header: "Name" }, { header: "Image", width: "150px" }, { header: "CPU", width: "90px" }, { header: "Memory", width: "170px" }, { header: "Up", num: true, width: "70px" }]}
              rows={CONTAINERS.map((c, i) => [
                <RunStateLabel state={props.states[i]} />, c.name, c.image,
                isUp(props.states[i]) ? <Sparkline values={c.cpu} max={100} budget={80} width={72} height={16} label={`${c.name} CPU`} /> : "–",
                isUp(props.states[i]) ? <Meter fraction={c.mem / c.limit} value={fmt.bytes(c.mem)} /> : "–",
                c.up && isUp(props.states[i]) ? fmt.uptime(c.up) : "–",
              ])} />
          </Show>
        </Show>
      </Show>
    </Card>
  );
}

/** ContainerInspector: facts, usage, ports, environment (secrets masked), actions. */
export function ContainerInspector(props: { container?: Container; state: RunState; reveal?: boolean; onRestart: () => void; onStartStop: () => void; onRemove: () => void }) {
  return (
    <Show when={props.container} fallback={<EmptyState message="Select a container." />}>
      {(c) => (
        <div style={{ display: "grid", gap: "16px", padding: "4px" }}>
          <div><h2 style={{ margin: "0 0 6px" }}>{c().stack} / {c().name}</h2><RunStateLabel state={props.state} /></div>
          <PropertyList items={[["Image", c().image], ["Uptime", isUp(props.state) ? fmt.uptime(c().up) : "–"], ["Host", "prod-1"]]} />
          <div style={{ display: "flex", gap: "8px", "align-items": "center" }}><Text tone="muted">Id</Text><ShortId id={c().id} /></div>
          <div style={{ display: "grid", gap: "8px" }}>
            <Meter label="CPU" fraction={(c().cpu.at(-1) ?? 0) / 100} value={`${c().cpu.at(-1) ?? 0} %`} />
            <Meter label="Memory" fraction={c().mem / c().limit} value={fmt.bytesOf(c().mem, c().limit)} />
          </div>
          <div><h3 style={{ "margin-bottom": "8px" }}>Ports</h3>
            <Show when={c().ports.length} fallback={<Text tone="faint">none published</Text>}>
              <div style={{ display: "grid", gap: "4px" }}><For each={c().ports}>{(p) => <PortMap {...p} />}</For></div>
            </Show>
          </div>
          <div><h3 style={{ "margin-bottom": "8px" }}>Environment</h3>
            <Show when={c().env.length} fallback={<Text tone="faint">none</Text>}><KeyValueRows rows={c().env} stacked reveal={props.reveal} /></Show>
          </div>
          <ButtonGroup align="end" buttons={[
            { label: "Remove", kind: "danger", onClick: props.onRemove }, { label: isUp(props.state) ? "Stop" : "Start", onClick: props.onStartStop },
            { label: "Restart", kind: "primary", onClick: props.onRestart },
          ]} />
        </div>
      )}
    </Show>
  );
}

/** StackTree: stacks, containers up, worst state as status. */
export function StackTree() {
  const [sel, setSel] = createSignal("shop");
  return (
    <div style={{ padding: "4px 8px" }}>
      <h3 style={{ margin: "0 0 8px 8px" }}>Stacks</h3>
      <TreeView label="Stacks" items={[
        { id: "shop", label: "shop", meta: "6/7", status: "warn" }, { id: "monitoring", label: "monitoring", meta: "1/2", status: "err" }, { id: "backup", label: "backup", meta: "0/1" },
      ]} open={new Set()} onToggle={() => {}} selected={sel()} onSelect={setSel} />
    </div>
  );
}

/** HostCard: one host; unreachable → ErrorState with Retry. */
export function HostCard(props: { host: Host; selected?: boolean }) {
  const h = () => props.host;
  return (
    <Card focused={props.selected}>
      <h3 style={{ "margin-bottom": "10px" }}>{h().name}</h3>
      <Show when={h().reachable} fallback={<ErrorState what={`Can't reach ${h().name}.`} why="no answer for 4 min · last seen 13:58" onRetry={() => {}} />}>
        <div style={{ display: "flex", gap: "10px", "margin-bottom": "14px", "font-size": "13px" }}>
          <Text tone="muted">{h().role}</Text><Status kind={h().up[0] === h().up[1] ? "ok" : "warn"}>{h().up[0]}/{h().up[1]} up</Status><Text tone="faint">{fmt.uptime(h().uptime)}</Text>
        </div>
        <div style={{ display: "grid", gap: "8px" }}>
          <Meter label="CPU" fraction={(h().cpu.at(-1) ?? 0) / 100} value={`${h().cpu.at(-1)} %`} />
          <Meter label="Memory" fraction={h().mem[0] / h().mem[1]} value={fmt.bytesOf(...h().mem)} />
          <Meter label="Disk" fraction={h().disk[0] / h().disk[1]} value={fmt.bytesOf(...h().disk)} />
        </div>
        <div style={{ display: "flex", gap: "12px", "align-items": "center", "margin-top": "12px" }}><Text tone="faint">CPU 1h</Text><Sparkline values={h().cpu} max={100} budget={80} width={200} /></div>
      </Show>
    </Card>
  );
}

/** NeedsAttention: what's wrong across hosts, worst first. */
export function NeedsAttention() {
  return (
    <Card>
      <h3 style={{ "margin-bottom": "8px" }}>Needs attention</h3>
      <List label="Needs attention"><For each={ATTENTION}>{([k, t, m]) => <ListRow title={t} status={k} meta={m} />}</For></List>
    </Card>
  );
}

/** Release: what's deployed where, and the steps. */
export function Release(props: { steps: StepSpec[] }) {
  return (
    <div style={{ display: "grid", gap: "16px", padding: "4px" }}>
      <h3>Release</h3>
      <PropertyList items={[["Stack", "shop"], ["From", "1.8.2"], ["To", "1.9.0"], ["Hosts", "prod-1, prod-2"], ["Strategy", "start new, then stop old"]]} />
      <h3>Steps</h3>
      <StepList steps={props.steps} />
    </div>
  );
}

/** ImageTable: images, size, age, used by; unused ones say so. */
export function ImageTable(props: { selected: number; onSelect: (i: number) => void }) {
  const unused = IMAGES.filter((i) => i[4] === 0);
  return (
    <Card>
      <h3 style={{ "margin-bottom": "12px" }}>Images · {unused.length} unused, {fmt.bytes(unused.reduce((a, i) => a + i[2], 0))}</h3>
      <Table label="Images" selected={props.selected} onSelect={props.onSelect}
        columns={[{ header: "Image" }, { header: "Id", width: "140px" }, { header: "Size", num: true, width: "90px" }, { header: "Created", num: true, width: "90px" }, { header: "Used by", width: "90px" }]}
        rows={IMAGES.map(([n, id, size, a, used]) => [n, <ShortId id={id} />, fmt.bytes(size), fmt.age(a), used ? `${used} ctr` : <Text tone="faint">unused</Text>])} />
    </Card>
  );
}

/** DiskUse: where the disk goes, and Prune. */
export function DiskUse(props: { onPrune: () => void }) {
  return (
    <div style={{ display: "grid", gap: "16px", padding: "4px" }}>
      <h3>Disk · prod-1</h3>
      <Meter label="Used" fraction={41 / 80} value={fmt.bytesOf(41 * GIB_, 80 * GIB_)} />
      <BarChart unit="GiB" items={[["volumes", 18.2], ["images", 6.1], ["build cache", 2.3], ["containers", 0.4]]} />
      <Text tone="muted">Prune removes 4 unused images and the build cache: 4.6 GiB.</Text>
      <ButtonGroup align="end" buttons={[{ label: "Prune…", kind: "danger", onClick: props.onPrune }]} />
    </div>
  );
}
