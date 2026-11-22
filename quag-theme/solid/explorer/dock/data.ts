/** "dock" demo data: a small Docker setup on two prod hosts. Same data as the terminal demo. */
import type { DiffKind, LogEntry, PortSpec, RunState } from "../../src";

const MIB = 1024 * 1024, GIB = 1024 * MIB;

export interface Container {
  name: string; stack: string; image: string; id: string; state: RunState; cpu: number[];
  mem: number; limit: number; ports: PortSpec[]; up: number; env: [string, string, boolean?][]; volumes: string[];
}
const API_ENV: [string, string, boolean?][] = [
  ["RUST_LOG", "info"], ["DATABASE_URL", "postgres://shop:hunter2@db:5432/shop", true], ["STRIPE_KEY", "sk_live_51HxQ2bK9vTz7m3f9a", true], ["WORKERS", "4"],
];
const flat = Array(10).fill(0);
export const CONTAINERS: Container[] = [
  { name: "proxy", stack: "shop", image: "traefik:3.1", id: "sha256:7be41aa03f9a", state: { kind: "running" }, cpu: [3, 4, 3, 5, 6, 4, 3, 4, 5, 4], mem: 61 * MIB, limit: 256 * MIB, ports: [{ host: 443, container: 443, proto: "tcp", public: true }, { host: 80, container: 80, proto: "tcp", public: true }], up: 3_542_400, env: [["TZ", "Europe/Paris"]], volumes: ["certs:/letsencrypt"] },
  { name: "api-1", stack: "shop", image: "shop/api:1.8.2", id: "sha256:3f9a1c2b7e41", state: { kind: "running" }, cpu: [22, 30, 28, 35, 41, 38, 44, 52, 47, 42], mem: 412 * MIB, limit: 512 * MIB, ports: [{ container: 8080, proto: "tcp" }], up: 187_200, env: API_ENV, volumes: ["uploads:/srv/uploads"] },
  { name: "api-2", stack: "shop", image: "shop/api:1.8.2", id: "sha256:91c2b7e413f9", state: { kind: "flapping", restarts: 5 }, cpu: [60, 5, 70, 2, 80, 4, 75, 3, 88, 6], mem: 498 * MIB, limit: 512 * MIB, ports: [{ container: 8080, proto: "tcp" }], up: 140, env: API_ENV, volumes: ["uploads:/srv/uploads"] },
  { name: "worker", stack: "shop", image: "shop/worker:1.8.2", id: "sha256:b7e413f9a1c2", state: { kind: "running" }, cpu: [12, 14, 13, 15, 18, 16, 14, 13, 15, 14], mem: 210 * MIB, limit: GIB, ports: [], up: 187_200, env: API_ENV, volumes: [] },
  { name: "db", stack: "shop", image: "postgres:16.4", id: "sha256:e413f9a1c2b7", state: { kind: "running" }, cpu: [8, 9, 8, 12, 10, 9, 11, 10, 9, 8], mem: 1_503_238_553, limit: 2 * GIB, ports: [{ container: 5432, proto: "tcp" }], up: 3_542_400, env: [["POSTGRES_PASSWORD", "hunter2-but-longer", true]], volumes: ["pgdata:/var/lib/postgresql/data"] },
  { name: "cache", stack: "shop", image: "redis:7.4", id: "sha256:13f9a1c2b7e4", state: { kind: "running" }, cpu: [2, 2, 3, 2, 2, 3, 2, 2, 2, 3], mem: 88 * MIB, limit: 256 * MIB, ports: [{ host: 6379, container: 6379, proto: "tcp", public: true }], up: 3_542_400, env: [], volumes: [] },
  { name: "mailer", stack: "shop", image: "shop/mailer:0.4.0", id: "sha256:c2b7e413f9a1", state: { kind: "paused" }, cpu: flat, mem: 34 * MIB, limit: 128 * MIB, ports: [], up: 86_400, env: [], volumes: [] },
  { name: "prometheus", stack: "monitoring", image: "prom/prometheus:2.54", id: "sha256:a1c2b7e413f9", state: { kind: "exited", code: 137 }, cpu: flat, mem: 0, limit: GIB, ports: [], up: 0, env: [], volumes: ["prom:/prometheus"] },
  { name: "grafana", stack: "monitoring", image: "grafana/grafana:11.2", id: "sha256:f9a1c2b7e413", state: { kind: "running" }, cpu: [4, 5, 4, 6, 5, 4, 5, 6, 4, 5], mem: 140 * MIB, limit: 512 * MIB, ports: [{ host: 3000, container: 3000, proto: "tcp" }], up: 3_542_400, env: [["GF_SECURITY_ADMIN_PASSWORD", "grafana-admin-pw", true]], volumes: ["grafana:/var/lib/grafana"] },
  { name: "restic", stack: "backup", image: "restic/restic:0.17", id: "sha256:2b7e413f9a1c", state: { kind: "exited", code: 0 }, cpu: flat, mem: 0, limit: 512 * MIB, ports: [], up: 0, env: [["RESTIC_PASSWORD", "correct-horse-battery", true]], volumes: ["pgdata:/data:ro"] },
];

export interface Host { name: string; role: string; reachable: boolean; cpu: number[]; mem: [number, number]; disk: [number, number]; up: [number, number]; uptime: number }
export const HOSTS: Host[] = [
  { name: "prod-1", role: "shop · monitoring", reachable: true, cpu: [35, 42, 38, 51, 47, 55, 61, 58, 52, 49, 54, 57], mem: [5100 * MIB, 8 * GIB], disk: [41 * GIB, 80 * GIB], up: [8, 10], uptime: 3_542_400 },
  { name: "prod-2", role: "shop (replica)", reachable: true, cpu: [20, 22, 25, 21, 24, 26, 23, 22, 25, 24, 23, 21], mem: [3300 * MIB, 8 * GIB], disk: [33 * GIB, 80 * GIB], up: [5, 5], uptime: 3_542_400 },
  { name: "backup-1", role: "backup", reachable: true, cpu: [5, 4, 6, 90, 95, 8, 5, 4, 5, 4, 6, 5], mem: [900 * MIB, 2 * GIB], disk: [455 * GIB, 500 * GIB], up: [1, 2], uptime: 1_209_600 },
  { name: "db-2", role: "postgres replica", reachable: false, cpu: [], mem: [0, 0], disk: [0, 0], up: [0, 0], uptime: 0 },
];

export const ATTENTION: ["err" | "warn", string, string][] = [
  ["err", "db-2 unreachable", "4 min"], ["err", "prometheus crashed: out of memory (137)", "prod-1"],
  ["warn", "api-2 restarted 5× in 10 min", "prod-1"], ["warn", "backup-1 disk at 91 %", "backup-1"], ["warn", "port 6379 (redis) public", "prod-1"],
];

export const DIFF: [DiffKind, string][] = [
  ["hunk", "@@ services.api @@"], ["same", "api:"], ["del", "  image: shop/api:1.8.2"], ["add", "  image: shop/api:1.9.0"],
  ["same", "  deploy:"], ["same", "    replicas: 2"], ["del", "    resources: { limits: { memory: 512M } }"], ["add", "    resources: { limits: { memory: 768M } }"],
  ["same", "  environment:"], ["add", '    FEATURE_INVOICES: "on"'], ["hunk", "@@ services.worker @@"], ["del", "  image: shop/worker:1.8.2"], ["add", "  image: shop/worker:1.9.0"],
];
export const STEP_NAMES = ["pull", "migrate", "start new", "health check", "switch", "stop old"];
export const STEP_TOOK = ["8s", "3s", "6s", "12s", "1s", "4s"];
export const DEPLOY_LOG: LogEntry[] = [
  { time: "14:02:01", kind: "info", message: "pull shop/api:1.9.0 · 212 MiB" }, { time: "14:02:09", kind: "ok", message: "pulled shop/api:1.9.0, shop/worker:1.9.0" },
  { time: "14:02:10", kind: "info", message: "migrate: 2 pending (0042_invoices, 0043_invoice_index)" }, { time: "14:02:13", kind: "ok", message: "migrations applied" },
  { time: "14:02:14", kind: "info", message: "start api-3, api-4 (1.9.0)" }, { time: "14:02:20", kind: "info", message: "health check GET /healthz on api-3, api-4" },
  { time: "14:02:32", kind: "ok", message: "api-3 healthy · api-4 healthy" }, { time: "14:02:33", kind: "info", message: "switch traffic → api-3, api-4" },
  { time: "14:02:34", kind: "info", message: "stop api-1, api-2 (1.8.2)" }, { time: "14:02:38", kind: "ok", message: "deployed shop 1.9.0 in 37s" },
];
export const FAIL_LOG: LogEntry = { time: "14:02:32", kind: "err", message: "api-3 unhealthy: GET /healthz → 503 (3 tries) · old version still serving" };

/** image, id, size, age (s), used by */
export const IMAGES: [string, string, number, number, number][] = [
  ["shop/api:1.8.2", "sha256:3f9a1c2b7e41", 212 * MIB, 864_000, 2], ["shop/worker:1.8.2", "sha256:b7e413f9a1c2", 198 * MIB, 864_000, 1],
  ["postgres:16.4", "sha256:e413f9a1c2b7", 432 * MIB, 3_542_400, 1], ["shop/api:1.7.9", "sha256:77e41aa03f9a", 209 * MIB, 2_592_000, 0],
  ["shop/api:1.7.8", "sha256:aa03f9a77e41", 207 * MIB, 3_196_800, 0], ["node:20-alpine", "sha256:41aa03f9a77e", 1288 * MIB, 5_184_000, 0],
  ["redis:7.4", "sha256:13f9a1c2b7e4", 117 * MIB, 3_542_400, 1], ["grafana/grafana:11.2", "sha256:f9a1c2b7e413", 455 * MIB, 3_542_400, 1],
  ["<none>", "sha256:0bad1dea5e11", 640 * MIB, 6_048_000, 0],
];
export const GIB_ = GIB;
