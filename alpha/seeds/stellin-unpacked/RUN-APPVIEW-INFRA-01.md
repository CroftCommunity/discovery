# RUN-APPVIEW-INFRA-01: generic Croft AppView hosting kit (OVH + Litestream + R2, multi-tenant)

`Status: runnable brief, 2026-07-17. Supersedes RUN-STELLIN-INFRA-01 (same architecture,
generalized): the kit hosts N small app services on one VPS, driven by per-service manifests.
Stellin is the marquee first tenant, not the design ceiling. Two phases: Phase 1 unattended
(build + validate, no money, no credentials), Phase 2 owner-gated (provision, bootstrap, fire
drill). DNS is at Porkbun. TDD/checks-first per the standing directive (2026-07-15): every
deliverable's executable check is committed red before the deliverable, red-to-green order shown
in the run summary.`

---

## 0. Owner pre-steps

1. Create an empty repo `CroftCommunity/appview-infra` (or name the session an alternative at
   start). Not the discovery repo; deployment IaC is not corpus material.

2. Decide the OVH endpoint before Phase 2: `ovh-eu`, `ovh-ca`, or `ovh-us` (US is a separate
   subsidiary with separate accounts). Phase 1 parameterizes it.

3. No domains required yet. Every service's fqdn is a manifest field; staging defaults live under
   croft.ing subdomains until per-app domain decisions land.

---

## 1. Context and the core abstraction (read once)

The Croft portfolio will run several small always-on services with the same shape: a single
static binary, SQLite storage, maybe a long-lived ingest connection, HTTPS serving. Known and
candidate tenants:

- **stellin-appview** (first tenant, the marquee case): filtered Jetstream ingest + XRPC serving,
  per the RUN-14 spike line in the discovery repo.

- **arecipe feed service** (candidate, not built now): the D4 edge-function alternative for
  on-demand .ics rendering, if arecipe ever wants a server-side option.

- Future pdsview/skylite adjacent services, labelers, or helpers as they appear.

**The core abstraction is the service manifest.** Each tenant is one file,
`services/<name>.toml`, declaring everything the kit needs:

```toml
name        = "stellin-appview"
fqdn        = "stellin-staging.croft.ing"
port        = 8101                      # unique per service
artifact    = "github:CroftCommunity/<repo>/releases"   # or "stub"
data_profile.canonical  = ["state.db"]  # Litestream targets (sqlite only)
data_profile.disposable = ["index.db"]  # rebuilt from source; NEVER backed up
data_profile.blobs      = ["blobs/"]    # rclone-mirrored dirs (ciphertext-safe)
```

Everything downstream is GENERATED from manifests: systemd units, the Caddyfile vhosts, the
litestream.yml, the rclone timer set, the drill's assertion list. Hand-editing generated files is
forbidden; the generator plus manifests are the source of truth. The load-bearing rule inherited
from the Stellin design: disposable paths appear in NO backup configuration, and a check enforces
it (D5) so no future session helpfully backs up a rebuildable index.

State taxonomy rationale (decided 2026-07-17, applies to every tenant): canonical =
observation-born or grant-born state nothing on the network can rebuild (cursors, telemetry,
grants); disposable = projections rebuildable from firehose/backfill/sealed history; blobs =
sealed ciphertext the service distributes content-blind. Off-provider backup because provider
backups share the provider's failure domain (OVH's own docs: backups replicate within the same
datacentre); R2 because zero-egress restores are free at the moment they matter and the free tier
(10 GB, 1M writes/mo) covers this scale.

**Service contract** (all tenants honor it; `CONTRACT.md` is deliverable D2): flags `--data-dir`
and `--listen`, endpoint `/healthz` returning 200 when ready, all state under its data dir, no
root, no ports below 1024. A shared ~40-line stub satisfies the contract so the entire kit and
the fire drill run before any real binary exists; a real tenant swaps in by changing `artifact`.

---

## 2. Guardrails

1. **Branch/commit discipline.** `main` of the fresh repo (empty, unshared) or `claude/infra-01`
   if the owner pushed anything. One commit per deliverable; `test(infra):` red before
   `feat(infra):` green.

2. **No secrets in the repo, ever.** Credentials only via environment (OVH_* trio, R2 keys as
   AWS_*/LITESTREAM_*). `.gitignore`: `*.tfstate*`, `.terraform/`, `*.env`. Terraform state local
   in Phase 2. Secret-shaped-string grep check is part of D1 and `make check`.

3. **Money gate.** Anything ordering an OVH service is Phase 2 with explicit owner go. OVH VPS is
   a MONTHLY purchase; show live catalog prices before apply. The multi-tenant point is exactly
   that N services share ONE ~€4-5 box; the drill's optional second box is a separate owner call.

4. **Checks-first, honestly scoped.** Component checks: `terraform fmt -check` + `validate`,
   shellcheck, `caddy validate`, litestream config parse, `systemd-analyze verify`, bats tests
   for scripts and the generator. Install tools via proxy; a tool that will not install marks its
   check BLOCKED in the summary, never silently skipped. The whole-system acceptance test is the
   Phase 2 fire drill; Phase 1 greens are component-level and the summary says so.

5. **Stop rules.** ~30 min time-box on stuck installs. Owner decisions surfaced, not made:
   endpoint/region, plan code, drill variant, bucket topology (see D6), Premium backup. No scope
   beyond the deliverables.

---

## 3. Phase 1 deliverables (unattended, in order)

**D1. Skeleton and hygiene.** `Makefile` with `check`; `.gitignore`; `scripts/no-secrets-check.sh`
wired in. Red state: `make check` exists and fails.

**D2. Contract and stub.** `CONTRACT.md` as in section 1; `stub/` implementing it. Check: bats
starts the stub with a temp data dir, asserts `/healthz` 200 and the declared files, kills it.

**D3. Manifests and the generator.** `services/stellin-appview.toml` (real first tenant, artifact
= "stub" for now) and `services/_example.toml` (documented template). `scripts/render.sh` (or a
small Rust/Go tool if cleaner) reading all manifests and emitting into `generated/`: one systemd
unit per service (User=<name> dedicated user, StateDirectory, Restart=always, NoNewPrivileges,
ProtectSystem=strict, ReadWritePaths on its data dir, PrivateTmp), one Caddyfile with a vhost per
fqdn reverse_proxying its port (HTTP-01 ACME; Porkbun holds only A/AAAA records), one
litestream.yml section per canonical sqlite file (1s sync, 24h snapshots, 168h retention), one
rclone sync service+timer per blob dir (5 min, `--immutable` where blob naming is
content-addressed; flag any tenant where it is not). Checks first: bats fixtures with two fake
manifests asserting the exact generated set, port-collision detection, and rejection of a
manifest whose canonical and disposable lists overlap.

**D4. Terraform.** `terraform/`: ovh provider pinned; variables (endpoint, datacenter, plan_code
required with no guessed default, ssh key); `ovh_vps` resource with plan structure + Debian 12;
outputs IPs and service name. `scripts/catalog-vps.sh` querying `/order/catalog/public/vps` for
live plan codes/prices (codes are undocumented and generation-specific; this script is how the
owner picks in Phase 2). Standard daily backup is included free on current VPS; order no backup
option; comment that Premium 7-day is one plan block if ever wanted. Checks: fmt + validate
(credential-free).

**D5. The no-backup-of-disposables enforcement.** `scripts/backup-audit.sh`: parses every
manifest and the generated litestream.yml + rclone units, fails if any disposable path appears in
any backup target, and fails if any canonical path is missing one. In `make check`. This is the
Stellin design rule promoted to kit invariant; check written red first against a deliberately
broken fixture.

**D6. Bucket topology note + config.** Default: ONE R2 bucket, per-service prefixes
(`<bucket>/<service>/{state,blobs}`), one scoped token. Tradeoff documented in
`docs/BUCKETS.md`: single token means one leaked credential exposes all tenants' canonical state
(blobs are ciphertext regardless); per-service buckets + tokens is the isolation upgrade, listed
as an owner call with the config change required. Check: link check + the generator emitting
prefix-correct paths (covered by D3 fixtures, extended here).

**D7. Bootstrap.** `bootstrap/bootstrap.sh`, idempotent on fresh Debian 12: per-service system
users from manifests; nftables 22/80/443; unattended-upgrades; SSH hardening; Caddy from official
apt repo; Litestream + rclone pinned in `versions.env`; install `generated/`; enable units.
Second run is a no-op exiting 0. Checks: shellcheck; bats double-run in container/chroot if the
environment allows, else `--plan` dry-run assertions with the limitation recorded.

**D8. Deploy pipeline.** One reusable workflow (`.github/workflows/deploy-service.yml`,
workflow_call) taking service name + artifact; per-tenant app repos call it on release. Server
side: `deploy` user with forced-command rsync into `/opt/<service>/`, atomic move, restart that
unit only. Checks: actionlint if installable; bats on the receive script.

**D9. DNS doc (Porkbun, manual v1).** `docs/DNS.md`: per-service A/AAAA records from Terraform
outputs; ACME needs no DNS API; future `_lexicon` TXTs listed as pending per-app namespace
decisions, not created; one line that Porkbun has an API if automation is ever wanted.

**D10. Runbook + drill.** `docs/RUNBOOK.md`: box loss (re-apply, bootstrap, per-service
`litestream restore` + rclone copy-back driven by the manifests, start, disposables rebuild from
their sources; recovery points stated: seconds/5min/zero); cursor loss; Litestream failure;
credential rotation; ADD-A-TENANT procedure (write manifest, render, re-run bootstrap, add DNS
record) as the kit's whole point, kept to ~5 steps. `drill/fire-drill.sh --variant
second-box|reinstall`: restores EVERY manifest's services onto the target and asserts, per
service, healthz 200 + a pre-planted canonical marker present + blob marker present. Checks:
shellcheck; bats on the drill's assertion loop against two stub tenants.

**D11. Phase 1 summary.** `RUN-SUMMARY.md`: red/green commit table, tool versions, BLOCKED items
with one-line unblocks, exact Phase 2 sequence.

---

## 4. Phase 2 (owner-gated)

- **P2-1.** catalog-vps.sh → owner picks plan_code/datacenter from live prices; record them.

- **P2-2.** `terraform plan` shown → owner go → `apply`.

- **P2-3.** bootstrap twice; second run no-op (idempotence against reality).

- **P2-4.** Owner creates Porkbun records per DNS.md; verify ACME + `/healthz` per tenant (stub).

- **P2-5.** Backup plane per tenant: plant a marker row in each canonical db and a marker blob;
  after one sync interval, `litestream restore` to a temp path and rclone ls; assert both.

- **P2-6.** Fire drill, owner-chosen variant (second-box: true provider-loss rehearsal, one more
  monthly purchase, then destroy; reinstall: free, same-box). PASS = all per-service assertions
  green; any restore failure halts go-live until understood.

- **P2-7.** Summary update: prices, drill result, observed recovery points, follow-ups (swap
  stellin stub for the real binary when RUN-14's line produces it; `_lexicon` TXTs on namespace
  decisions; bucket topology confirmation; Premium backup yes/no).

---

## 5. Explicit non-goals

No HA, no load balancer, no containers, no orchestrator, no monitoring stack beyond `/healthz` +
an external pinger (owner-configured; URL noted in runbook), no DNS automation, no remote
Terraform state, no backup of any disposable path ever, no per-tenant VPSes until a tenant
outgrows the shared box (the runbook's "when to split" note: sustained CPU pressure or a tenant
needing its own failure domain). Each lives as one sentence in the runbook's "why not" list so
future sessions do not add them helpfully.
