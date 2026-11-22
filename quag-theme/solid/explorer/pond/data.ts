import type { StatusKind } from "../../src";

/** `project`: index in PROJECTS (never 0: "All" is not a project). */
export interface Task { id: number; done: boolean; title: string; tag: string; due: string; state?: StatusKind; notes: string; project: number }

export const TASKS: Task[] = [
  { id: 1, done: false, title: "Fix save crash on empty file", tag: "bug", due: "today", state: "err", notes: "Repro: open, clear, Ctrl+S.", project: 1 },
  { id: 2, done: false, title: "Write README for quag-theme", tag: "docs", due: "today", notes: "Keep the API tiny; link STRUCTURE.md.", project: 1 },
  { id: 3, done: false, title: "Port settings screen to theme", tag: "ui", due: "tomorrow", state: "warn", notes: "", project: 1 },
  { id: 4, done: true, title: "Pick a color palette", tag: "ui", due: "done", state: "ok", notes: "Quagsire.", project: 1 },
  { id: 5, done: false, title: "Backup tablet partitions", tag: "chore", due: "fri", notes: "", project: 3 },
  { id: 6, done: false, title: "Shader test in Godot", tag: "game", due: "sat", state: "info", notes: "", project: 2 },
];

/** Projects in the nav; the first entry shows every task. */
export const PROJECTS = ["All", "quag-theme", "web-gl-shooter", "Tablet"];
export const inProject = (t: Task, p: number) => p === 0 || t.project === p;
/** Open tasks in project p. */
export const openIn = (tasks: Task[], p: number) => tasks.filter((t) => !t.done && inProject(t, p)).length;

/** The tasks page's tabs: Today (open, due today), Upcoming (open, due later), Done. */
export const TABS = ["Today", "Upcoming", "Done"];
export const inTab = (t: Task, tab: number) => (tab === 0 ? !t.done && t.due === "today" : tab === 1 ? !t.done && t.due !== "today" : t.done);
/** What an empty tab says, and the key that helps. */
export const TAB_EMPTY: [string, string, string][] = [
  ["Nothing due today.", "2", "to see what's next"],
  ["Nothing coming up.", "N", "to add a task"],
  ["Nothing finished yet.", "1", "to see what's on"],
];

export const LOG: [string, StatusKind, string][] = [
  ["12:01:03", "info", "Connected to sync server"],
  ["12:01:09", "warn", "Slow response (812 ms)"],
  ["12:01:12", "err", "Upload failed: timeout after 30 s"],
  ["12:01:15", "info", "Retrying upload (1/3)"],
  ["12:01:17", "ok", "Upload complete: 4 tasks"],
  ["12:02:40", "info", "Project 'quag-theme' opened"],
  ["12:03:02", "ok", "Saved"],
];

/** The note open in the Workspace page. */
export const NOTE = `# README for quag-theme

Quag is the house style: calm, dark, Quagsire blue.

## Use it
- depend on quag-theme (path dependency)
- build pages from elements, never from atoms
- only atoms set colors

## Levels
atom → molecule → element → UI`;

export const PROJECT_ROWS: string[][] = [
  ["Inbox", "4", "12", "today"],
  ["quag-theme", "2", "31", "today"],
  ["web-gl-shooter", "7", "9", "3 days ago"],
  ["Tablet", "1", "4", "last week"],
];

export const DONE_PER_DAY = [3, 4, 2, 5, 3, 2, 5, 4, 5, 3, 6, 3, 4, 3];

export const FRAME_MS = [
  12.1, 12.4, 11.9, 13.0, 12.2, 12.8, 14.1, 12.0, 11.8, 12.5, 19.8, 12.3, 12.1, 12.6, 13.2, 12.9, 12.0, 11.7, 12.4, 12.2,
  12.8, 13.5, 12.1, 24.6, 13.0, 12.4, 12.2, 11.9, 12.7, 12.3, 12.5, 12.0, 13.1, 12.6, 12.2, 17.2, 12.4, 12.1, 11.8, 12.6,
  12.9, 12.3, 12.0, 12.5, 13.3, 12.7, 12.2, 12.4, 12.1, 12.8, 12.0, 12.6, 18.4, 12.3, 12.1, 12.5, 12.2, 12.9, 12.4, 12.0,
];

export const MEMORY_MB: [string, number][] = [["Textures", 812], ["Meshes", 236], ["Render targets", 410], ["Audio", 64], ["Other", 38]];

import type { LogEntry, TreeItem } from "../../src";

export const TREE: TreeItem[] = [
  { id: "inbox", label: "Inbox", children: [{ id: "inbox/crash", label: "Fix save crash", status: "err" }, { id: "inbox/backup", label: "Backup tablet" }] },
  { id: "qt", label: "quag-theme", children: [
    { id: "qt/docs", label: "docs", children: [{ id: "qt/docs/readme", label: "README.md", meta: "4 KB" }, { id: "qt/docs/structure", label: "STRUCTURE.md", meta: "9 KB" }] },
    { id: "qt/src", label: "src", children: [{ id: "qt/src/atoms", label: "atoms.rs" }, { id: "qt/src/molecules", label: "molecules.rs" }] },
  ] },
  { id: "gl", label: "web-gl-shooter", children: [{ id: "gl/shader", label: "Shader test" }] },
  { id: "tablet", label: "Tablet" },
];

/** A generated server log, `n` lines. */
export function bigLog(n: number): LogEntry[] {
  const MSG: [LogEntry["kind"], string][] = [
    ["info", "GET /api/tasks 200 in 12 ms"], ["info", "GET /api/projects 200 in 8 ms"], ["ok", "Sync complete: 4 tasks"],
    ["warn", "Slow response from db (812 ms)"], ["info", "POST /api/tasks 201 in 21 ms"], ["err", "Upload failed: timeout after 30 s"],
    ["warn", "Failed login for user 'admin' from 203.0.113.7"], ["info", "Cache hit ratio 94 %"],
  ];
  return Array.from({ length: n }, (_, i) => {
    const s = 8 * 3600 + i, [kind, message] = MSG[(i * 7 + Math.floor(i / 13)) % MSG.length];
    const p = (x: number) => String(x).padStart(2, "0");
    return { time: `${p(Math.floor(s / 3600) % 24)}:${p(Math.floor(s / 60) % 60)}:${p(s % 60)}`, kind, message };
  });
}

import type { Severity } from "../../src";
export interface Finding { sev: Severity; title: string; host: string; age: string; hours: number; detail: string; secret?: string }
export const FINDINGS: Finding[] = [
  { sev: "critical", title: "API key committed to repository", host: "api-1", age: "10m", hours: 10 / 60, detail: "Found in config/prod.env (commit 3f9a1c2).", secret: "sk_live_51HxQ2bK9vTz7m3f9a" },
  { sev: "high", title: "SSH allows password login", host: "db-1", age: "1d", hours: 24, detail: "PasswordAuthentication yes in sshd_config." },
  { sev: "high", title: "12 failed admin logins from one IP", host: "api-1", age: "40m", hours: 40 / 60, detail: "203.0.113.7, 12 attempts in 4 minutes." },
  { sev: "medium", title: "TLS certificate expires in 9 days", host: "web-1", age: "6h", hours: 6, detail: "pond.example.net, auto-renew failed." },
  { sev: "medium", title: "Port 6379 (Redis) open to the internet", host: "cache-1", age: "2d", hours: 48, detail: "Bound to 0.0.0.0, no auth." },
  { sev: "low", title: "Kernel update available", host: "db-1", age: "4d", hours: 96, detail: "6.8.0-45 → 6.8.0-48 (security fixes)." },
  { sev: "info", title: "New user 'deploy' added", host: "web-1", age: "12d", hours: 288, detail: "By root, via ansible." },
];
export const RULES: string[][] = [
  ["1", "allow", "443/tcp", "any", "1.2 M"], ["2", "allow", "22/tcp", "10.0.0.0/8", "3 412"], ["3", "deny", "22/tcp", "any", "18 905"],
  ["4", "allow", "6379/tcp", "any", "211"], ["5", "deny", "all", "any", "52 117"],
];
export const RANGES = ["15m", "1h", "24h", "7d", "30d"];
export const RANGE_HOURS = [0.25, 1, 24, 168, 720];
export const SERVERS: [string, number[]][] = [
  ["api-1", [22, 30, 28, 35, 41, 38, 44, 52, 47, 42]], ["db-1", [55, 62, 70, 78, 84, 91, 88, 86, 82, 87]],
  ["web-1", [12, 14, 13, 15, 18, 16, 14, 13, 15, 14]], ["cache-1", [8, 9, 8, 12, 10, 9, 11, 10, 9, 8]],
];

/** Dashboard: the project a row of PROJECT_ROWS opens (Inbox = every task). */
export const projectOfRow = (row: number) => Math.max(0, PROJECTS.indexOf(PROJECT_ROWS[row]?.[0] ?? ""));
