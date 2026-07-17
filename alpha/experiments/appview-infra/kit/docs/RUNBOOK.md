# Runbook

Operator procedures for the appview-infra box. Everything here assumes the
manifests (`services/*.toml`) + the generator are the source of truth; never
hand-edit `generated/`.

## Recovery points (what you get back, and how fresh)

| Class      | Mechanism            | Recovery point                         |
|------------|----------------------|----------------------------------------|
| canonical  | Litestream → R2      | **seconds** (1s sync interval)         |
| blobs      | rclone → R2          | **5 min** (timer interval)             |
| disposable | none, by design      | **zero** — rebuilt from source, never backed up |

## Box loss (full VPS gone)

The provider's own backups share the provider's failure domain, so recovery is
off-provider (R2). Steps:

1. `terraform apply` a fresh VPS (Phase 2 flow: catalog → plan → apply).
2. Run `bootstrap/bootstrap.sh --apply` on the new box (idempotent).
3. Create the Porkbun A/AAAA records (see `DNS.md`) if the IP changed.
4. Per manifest, **restore canonical** from R2:
   `litestream restore -o /var/lib/<svc>/state.db <replica-url>`, then
   **copy back blobs**: `rclone copy r2:<bucket>/<svc>/blobs /var/lib/<svc>/blobs`.
5. `systemctl start <svc>.service <svc>-api.service` and the timers.
6. Disposables rebuild themselves from firehose/backfill — **nothing to restore**
   (zero recovery point by construction; the backup-audit invariant guarantees
   no disposable path was ever a backup target).

Recovery points as above: seconds for canonical sqlite, 5 min for blobs, zero
for disposables.

## Cursor loss (canonical, but only the cursor)

A lost/rolled-back ingest cursor is **canonical** — it is observation-born and
nothing on the network rebuilds it. It rides inside `state.db`, so it is
restored by the same Litestream restore as box loss. If only the cursor is
suspect (not the whole db), restore `state.db` to a temp path, read the cursor
row, and reconcile. Never "rebuild" a cursor from the index — the index is
disposable and may be behind.

## Litestream failure alerting (via journald)

Litestream logs to the journal. Alert on it there:

- `journalctl -u litestream -p err --since -1h` — any replication error.
- A watchdog (owner-configured external pinger + `journalctl` grep) should page
  if `litestream` exits, or if "sync" stops appearing, for more than a few
  minutes. There is no metrics stack (non-goal); journald + the external pinger
  is the whole alerting surface.

## Credential rotation

Credentials live only in the environment (guardrail 4), never in the repo.

1. **R2 (Litestream + rclone).** Mint a new bucket-scoped token in Cloudflare,
   update the box's environment (the systemd drop-in / `EnvironmentFile`), then
   `systemctl restart litestream` and re-arm the rclone timers. Revoke the old
   token. (Per `BUCKETS.md`, one scoped token covers all tenants by default.)
2. **OVH API (Terraform).** Rotate the `OVH_*` trio in your operator env; no box
   change. Only needed for provisioning, not serving.
3. **deploy key.** Replace `deploy`'s `authorized_keys` entry (forced command
   unchanged) and update the CI secret `deploy_ssh_key`.

## ADD-A-TENANT (≈5 steps)

1. `cp services/_example.toml services/<name>.toml` and edit: `name`, `fqdn`,
   `port` (unique across ALL service+api ports), the `data_profile`, and the
   `serve_api`/`gated_groups` flags.
2. `make generate` — emits the tenant's units, vhosts, litestream section, and
   rclone timers. `make check` — the backup-audit + render checks must pass.
3. `bootstrap/bootstrap.sh --apply` (idempotent) — creates the tenant's users,
   installs the new generated files, enables the units.
4. Create the Porkbun A/AAAA records for `<fqdn>` and (if `serve_api`)
   `api.<fqdn>` (see `DNS.md`).
5. Deploy the artifact via the `deploy-service` workflow, verify `/healthz`.

## API escalation ladder (with triggers)

Start every `serve_api` tenant at the cheapest rung; climb only when a trigger
fires:

1. **shared-wal** (default). The api reads the live db read-only. Trigger to
   climb: read load starts pinning the WAL / the writer's cursor visibly stalls
   under export load, or the ReadOnlyPaths/-shm tension (see below) bites.
2. **snapshot.** A VACUUM-INTO timer produces `serve/state.db`; the api serves
   that (minutes of staleness, zero writer interaction). Trigger to climb:
   snapshot production or export volume competes with the writer for IO/CPU
   despite `CPUQuota`/`IOSchedulingClass`.
3. **second box** (runbook paragraph, not built). A second VPS does continuous
   `litestream restore` from R2 and serves the api only. Trigger: the api's read
   load needs its own failure domain / its own IO budget. This is one more
   monthly purchase — an owner call.

Note (recorded seam): `shared-wal` live reads want a writable `-shm`, which
strict `ReadOnlyPaths` blocks; snapshot mode is containment-clean. Confirm the
shared-wal-under-ReadOnlyPaths behaviour on-box in Phase 2 before defaulting a
high-read tenant to shared-wal.

## WHEN-TO-SPLIT a tenant to its own VPS

Keep everyone on one box until a concrete trigger fires; then split the noisy
tenant to its own VPS (its manifest is portable — same generator, new box):

- sustained CPU/IO contention that `CPUQuota`/`IOSchedulingClass` cannot fix;
- a tenant's canonical state approaching the shared R2 free-tier limits;
- a tenant needing its own failure domain (compliance, an SLA, or blast-radius
  isolation of its canonical state);
- the api escalation ladder reaching "second box" for that tenant anyway.

## Why not (the non-goals — one line each, so they are not added helpfully)

- **No HA / no failover pair** — single box; recovery is restore-from-R2, not
  hot standby.
- **No load balancer** — one box, Caddy terminates TLS directly.
- **No containers** — plain systemd units and a single binary per tenant.
- **No orchestrator** (k8s/nomad) — systemd is the whole control plane.
- **No monitoring stack** — `/healthz` plus an owner-configured external pinger
  and journald; nothing more.
- **No DNS automation** — Porkbun A/AAAA are created by hand (`DNS.md`).
- **No remote Terraform state** — state stays local and gitignored.
- **No backup of any disposable path, ever** — enforced by the backup-audit.
- **No MLS / small-group machinery in the kit** — that is croft-group territory;
  the kit only hosts and serves.
- **No deciding the group write-path fork or the scale boundary** — `GROUPS.md`
  asks; the owner answers.
- **No per-tenant VPSes** until a WHEN-TO-SPLIT trigger fires.
- **No production work from the discovery repo** — Phase 2 runs only from the
  extracted `appview-infra` repo.
