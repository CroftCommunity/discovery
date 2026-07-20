# RUN-APPVIEW-INFRA-02: Croft AppView hosting kit, complete brief (supersedes 01)

`Status: runnable brief, 2026-07-17. Supersedes RUN-APPVIEW-INFRA-01 entirely; do not execute 01.
Scope: the generic multi-tenant AppView hosting kit (OVH VPS + Litestream + R2 + Porkbun DNS),
PLUS the per-user own-data API, PLUS discovery and fork-agnostic build for the access-gated
large-group tier, staged to maximize what is built and proven BEFORE any purchase. Three phases:
Phase 1 unattended (no credentials, no money), Phase 1.5 owner-credentialed but FREE (real R2,
no VPS), Phase 2 owner-gated purchase. TDD/checks-first per the standing directive (2026-07-15):
every deliverable's executable check is committed red before the deliverable; the run summary
shows red-to-green commit order per deliverable.`

---

## 0. Owner pre-steps

1. Create an empty repo `CroftCommunity/appview-infra` (or name an alternative at session start).
   Not the discovery repo; deployment IaC and service kits are not corpus material. Design
   documents produced here that carry SPEC-STATUS implications for Drystone docs are routed as
   notes for the discovery repo's staging process, never edited into it from this repo.

2. Decide (before Phase 2 only) the OVH endpoint: `ovh-eu`, `ovh-ca`, or `ovh-us` (US is a
   separate subsidiary with separate accounts).

3. No domains needed. All fqdns are manifest fields with croft.ing staging defaults.

---

## 1. Context (read once)

### 1.1 The portfolio and the abstraction

Several small always-on services share one shape: a single static binary, SQLite storage,
possibly a long-lived ingest connection, HTTPS serving. Tenants known and candidate:

- **stellin-appview** (marquee first tenant): filtered Jetstream ingest + XRPC serving, per the
  RUN-14 spike line in the discovery repo.

- **croft-groups service** (near-term, see 1.4): the access-gated large-group serving tier,
  shared mechanism with Stellin's groups.

- **arecipe feed service** (candidate, not built now); future pdsview/skylite adjacencies.

Each tenant is one manifest, `services/<name>.toml`:

```toml
name        = "stellin-appview"
fqdn        = "stellin-staging.croft.ing"
port        = 8101                       # unique per service
artifact    = "stub"                     # later: github:CroftCommunity/<repo>/releases
serve_api   = true                       # generates the own-data API sidecar (1.3)
api_mode    = "shared-wal"               # or "snapshot"
gated_groups = true                      # enables roster-gated group serving (1.4)
data_profile.canonical  = ["state.db"]   # Litestream targets (sqlite only)
data_profile.disposable = ["index.db"]   # rebuilt from source; NEVER backed up
data_profile.blobs      = ["blobs/"]     # rclone-mirrored dirs
```

Everything downstream is GENERATED from manifests: systemd units, Caddy vhosts, litestream.yml,
rclone timers, drill assertions. Hand-editing generated files is forbidden; generator + manifests
are the source of truth.

### 1.2 State taxonomy (kit invariant, applies to every tenant)

Canonical = observation-born or grant-born state nothing on the network can rebuild (cursors,
telemetry, grants, group rosters, any server-held keys). Disposable = projections rebuildable
from firehose/backfill/sealed history; NEVER backed up, and a check enforces it (D5). Blobs =
data the service stores and distributes without needing to read it. Off-provider backup because
provider backups share the provider's failure domain (OVH's own docs: backups replicate within
the same datacentre). R2 because zero-egress restores are free at the moment they matter and the
free tier (10 GB storage, 1M writes/mo) covers this scale.

### 1.3 The per-user own-data API (decided 2026-07-17)

Users reach their canonical records at their PDS; what only the AppView holds about a user is
observation-born and grant-born state (their telemetry, grants, roster memberships, entitled
sealed blobs) plus derived views. The API is therefore self-scoping: verified caller DID, every
query implicitly filtered to `subject_did = caller`, plus a paginated bulk-export route.

Serving is CONTAINED in a separate process per tenant, `<name>-api`:

- SQLite WAL mode permits many read-only connections in separate processes concurrent with the
  single writer on the same files; that is the free "replica" for `api_mode = "shared-wal"`.

- Containment is mechanical: dedicated user, `ReadOnlyPaths=` on the data dir (the process is
  OS-incapable of writing), own port and `api.<fqdn>` vhost, `CPUQuota=` and IO scheduling so
  export queries cannot starve the writer's cursor advancement, statement timeouts and paginated
  short transactions because long read transactions pin the WAL.

- `api_mode = "snapshot"`: a timer runs `VACUUM INTO .../serve/state.db` every N minutes and
  atomically swaps; the API serves the snapshot (zero writer interaction, N minutes staleness).
  The `serve/` dir is derived state, classed disposable.

- Escalation to a second box (continuous restore-from-R2 on another VPS serving the API) is a
  runbook paragraph, not built.

Auth is the atproto service-auth JWT verifier pattern being proven in the discovery repo's
RUN-14 EXP-A; this run uses a stubbed verifier behind the same interface (D12 note).

### 1.4 The access-gated large-group tier (stated 2026-07-17; discovery + fork-agnostic build)

Design stance on record: past a scale boundary (working number ~5,000 members; the boundary is a
parameter, not a decision this run makes), group confidentiality via E2EE is a mirage (any member
leaks; sociological privacy is gone), so large groups are served FROM the AppView as **private
but not E2EE**: roster-gated serving to verified member DIDs, trusted-gatekeeper posture stated
honestly, with search, helpers, and moderation native because the server can read. The same
mechanism is intended to serve both Stellin groups and Croft Groups. Below the boundary, groups
remain MLS-sealed (croft-group crates); the AppView remains a content-blind distributor for that
tier per the §H hybrid topology.

Two things follow for THIS run:

- **A write-path fork exists and is an OWNER DECISION, not the session's** (guardrail 5): where
  large-group content canonically lives. Variant A: authors publish ciphertext records to their
  own repos sealed to a server-held group scope key; the AppView decrypts to serve; canonical
  server state = the keys + roster (small, in state.db); content remains repo-canonical and the
  index stays disposable. Variant B: members write directly to the AppView API; the AppView is
  the system of record; content is server-canonical (blobs/ + state.db) with the heavier backup
  obligation that implies. D11 delivers the comparison brief; D12 builds only the fork-agnostic
  parts behind a storage trait.

- **Spec-status implications are routed, not decided.** The tier deliberately accepts the
  "degrades toward trusted-gatekeeper" arm of social-mapping's open AppView-scope-key item FOR
  THE LARGE TIER ONLY. D11 includes a short note formatted for the discovery repo's staging
  process (proposed-changes doc); this repo does not touch Drystone docs.

### 1.5 Service contract

`CONTRACT.md` (D2): flags `--data-dir` and `--listen`, `/healthz` returning 200 when ready, all
state under the data dir, no root, no ports below 1024. An API addendum for `serve_api` tenants:
read-only opens, statement timeout, pagination limits. A gated-groups addendum for
`gated_groups` tenants: the verifier interface and the roster-gate semantics (1.4). A shared
stub satisfies contract + addenda so the whole kit, the local rehearsal, and the drill run before
any real binary exists.

---

## 2. Guardrails

1. **Branch/commit discipline.** `main` of the fresh repo (empty, unshared) or `claude/infra-02`
   if the owner pushed anything. One commit per deliverable; `test(infra):` red before
   `feat(infra):` green; summary table shows the order.

2. **No secrets in the repo, ever.** Credentials only via environment (OVH_* trio; R2 as
   AWS_*/LITESTREAM_*). `.gitignore`: `*.tfstate*`, `.terraform/`, `*.env`. Terraform state
   local. Secret-shaped-string grep in `make check` (D1).

3. **Money gate.** Anything ordering an OVH service is Phase 2 with explicit owner go; OVH VPS is
   a MONTHLY purchase; show live catalog prices first. Phase 1.5 must be genuinely free: R2 free
   tier only, assert usage stays within it.

4. **Checks-first, honestly scoped.** Component checks: `terraform fmt -check` + `validate`,
   shellcheck, `caddy validate`, litestream config parse, `systemd-analyze verify`, bats for
   scripts/generator/stub behavior. Tools install via proxy; a tool that will not install marks
   its check BLOCKED in the summary, never silently skipped. Whole-system acceptance = the
   Phase 2 fire drill; the local rehearsal (D13) and Phase 1.5 are its staged approximations and
   the summary must distinguish the three grades.

5. **Owner decisions surfaced, never made.** Endpoint/region, plan code, drill variant, bucket
   topology, Premium backup, the group-tier write-path fork (1.4), and the scale boundary number.
   Write options up, pause or proceed to the next deliverable.

6. **Stop rules.** ~30 min time-box on stuck installs/builds; checkpoint, mark PARTIAL, move on.
   No scope beyond the deliverables list.

---

## 3. Phase 1 deliverables (unattended, free, in order)

**D1. Skeleton and hygiene.** `Makefile` (`check` target), `.gitignore`,
`scripts/no-secrets-check.sh` wired in. Red state: `make check` exists and fails.

**D2. Contract and stub.** `CONTRACT.md` + addenda (1.5); `stub/` implementing all of it
(healthz, data-dir files per a passed profile, a trivial authed echo route behind the verifier
interface). Check: bats starts the stub, asserts healthz, files, and that unauthenticated calls
to authed routes get 401.

**D3. Manifests and generator.** `services/stellin-appview.toml` (fields as in 1.1, artifact
"stub"), `services/croft-groups.toml` (gated_groups tenant, artifact "stub"),
`services/_example.toml` documented. `scripts/render.sh` (or small Rust/Go tool) emitting into
`generated/`: per-service systemd unit (dedicated user, StateDirectory, Restart=always,
NoNewPrivileges, ProtectSystem=strict, ReadWritePaths on its data dir, PrivateTmp); per-fqdn
Caddy vhosts (HTTP-01; Porkbun holds only A/AAAA); litestream.yml section per canonical sqlite
(1s sync, 24h snapshots, 168h retention); rclone service+timer per blob dir (5 min,
`--immutable` where content-addressed, flag where not); AND per `serve_api` tenant the api unit
(ReadOnlyPaths, CPUQuota, IOSchedulingClass, own port, `api.<fqdn>` vhost) plus, in snapshot
mode, the VACUUM INTO timer. Checks first: bats fixtures with fake manifests asserting the exact
generated set, port-collision detection, rejection of canonical/disposable overlap, and assertion
that api units carry ReadOnlyPaths and never a ReadWritePaths on the data dir.

**D4. Terraform.** `terraform/`: ovh provider pinned; variables (endpoint, datacenter, plan_code
REQUIRED with no guessed default, ssh key); `ovh_vps` + Debian 12; outputs IPs/service name.
`scripts/catalog-vps.sh` querying `/order/catalog/public/vps` for live plan codes and prices
(codes are undocumented and generation-specific; this is how the owner picks in Phase 2).
Standard daily backup is included free; order no backup option; comment that Premium 7-day is
one plan block later. Checks: fmt + validate (credential-free).

**D5. Backup-audit invariant.** `scripts/backup-audit.sh` in `make check`: parses manifests +
generated backup configs; FAIL if any disposable path (including api `serve/` snapshot dirs) is
a backup target, or any canonical path lacks one. Red first against a broken fixture. This is
the state-taxonomy rule promoted to executable kit law.

**D6. Bucket topology.** Default ONE R2 bucket, per-service prefixes
(`<bucket>/<service>/{state,blobs}`), one scoped token. `docs/BUCKETS.md` states the tradeoff
(one leaked token exposes all tenants' canonical state; per-service buckets is the isolation
upgrade) as an owner call. Generator emits prefix-correct paths (D3 fixtures extended).

**D7. Bootstrap.** `bootstrap/bootstrap.sh`, idempotent on fresh Debian 12: per-service and
per-api users from manifests; nftables 22/80/443; unattended-upgrades; SSH hardening (keys only,
no root password auth); Caddy from official apt repo; Litestream + rclone pinned in
`versions.env`; install `generated/`; enable units. Second run is a no-op exiting 0. Checks:
shellcheck; bats double-run in container/chroot if the environment allows, else `--plan` dry-run
assertions with the limitation recorded.

**D8. Deploy pipeline.** Reusable `deploy-service.yml` (workflow_call: service name + artifact);
server-side `deploy` user with forced-command rsync into `/opt/<service>/`, atomic move, restart
that unit only. Checks: actionlint if installable; bats on the receive script.

**D9. DNS doc (Porkbun, manual v1).** `docs/DNS.md`: per-service and per-api A/AAAA records from
Terraform outputs; ACME is HTTP-01 so no DNS API needed; future `_lexicon` TXTs listed as
pending per-app namespace decisions, not created; one line that Porkbun has an API if automation
is ever wanted. Check: link check in `make check`.

**D10. Own-data API behavior in the stub.** Extend the stub with the self-scoping semantics:
`getMyRows` returns only rows whose subject matches the verified caller; `export` streams
paginated; a deliberately slow query hits the statement timeout. Red-first matrix: caller A
never sees caller B's rows; unauthenticated 401; timeout fires; and a process-level test that
the api process, run under the generated unit's constraints (systemd-run with the same
sandboxing where the environment allows), CANNOT write into the data dir. Where systemd
sandboxing cannot be exercised in-environment, assert the unit file fields instead and record
the limitation.

**D11. Group-tier design brief (discovery deliverable, owner-facing).** `docs/GROUPS.md`:

- The tier definition per 1.4, with the honest-posture language drafted (private, roster-gated,
  NOT cryptographically confidential; member-leak equivalence argument; boundary as parameter
  `group_scale_boundary`, working number 5,000, decision deferred).

- The write-path fork analyzed: Variant A (repo-ciphertext + server-held scope key; content
  repo-canonical, server canonical state = keys + roster; index stays disposable; per-member
  writes ride atproto; key custody and rotation obligations land in state.db) vs Variant B
  (direct API writes; server-canonical content; simplest immediate build; heaviest backup and
  export obligations; weakest data-portability story). Each scored against: state taxonomy
  (1.2), backup cost, restore drill shape, moderation/helper mechanics, migration path between
  variants, and the social-mapping §H posture including the deliberate trusted-gatekeeper
  acceptance for this tier only.

- A short routing note formatted for the discovery repo's proposed-changes staging doc (the
  large-tier stance and its relation to the open AppView-scope-key item), to be carried over by
  the owner or a discovery-repo session; NOT applied from here.

- Ends with the decision request: variant, boundary number, and whether croft-groups launches
  before or with stellin. Check: link check; brief reviewed against 1.4 for internal
  consistency (a bats-greppable claims list is acceptable as the mechanical check, recorded as
  such).

**D12. Roster-gated serving, fork-agnostic build in the stub.** Behind a `GroupStore` trait that
both variants can implement later: `roster(group)`, `content(group, cursor)`. Stub implements a
fixture store. Serving semantics red-first, mirroring the RUN-14 EXP-A matrix: member DID gets
content; non-member 403; anonymous 401; removed member 403 on next call; and the offering-vs-
reading distinction documented as NOT applying in this tier (the server reads by design; the
honest posture is the feature). The verifier is the same stubbed interface as D2/D10; a comment
marks real service-auth verification as proven in discovery RUN-14 EXP-A and swapped in with the
real binary. Roster and grants are classed canonical in the manifests (D3 fixtures assert it and
D5 enforces the backup).

**D13. Local full-stack rehearsal (the pre-purchase capstone).** `make local-up`: run every
tenant's stub plus api process on localhost ports using the generated configuration adapted to
user-mode execution (systemd user units where available, plain supervised processes otherwise,
with the adaptation recorded); Caddy on high ports with internal TLS or plain HTTP locally;
litestream replicating to a `file://` replica and rclone to a local directory standing in for
R2. Then `drill/fire-drill.sh --variant local`: destroy the local state dirs, restore from the
local replicas, restart, and run the FULL per-tenant assertion loop (healthz, canonical marker,
blob marker, api self-scoping spot-check, gated-group member/non-member spot-check). This proves
the drill logic, the generator output, and the restore choreography end to end with zero
credentials and zero spend. Checks: the drill script's assertions ARE the check; red first by
running it before restore logic exists.

**D14. Runbook.** `docs/RUNBOOK.md`: box loss (re-apply, bootstrap, per-manifest restore +
copy-back, start, disposables rebuild; recovery points stated: seconds for canonical sqlite,
5 min for blobs, zero for disposables by construction); cursor loss; Litestream failure alerting
via journald; credential rotation (OVH, R2, deploy key); ADD-A-TENANT in ~5 steps; the api
escalation ladder (shared-wal → snapshot → second box) with its trigger; WHEN-TO-SPLIT a tenant
to its own VPS; the "why not" list (section 6). Checks: shellcheck on referenced scripts; link
check.

**D15. Phase 1 summary.** `RUN-SUMMARY.md`: red/green commit table per deliverable, tool
versions, BLOCKED items with one-line unblocks, the D11 decision request restated, and the exact
Phase 1.5 and Phase 2 sequences.

---

## 4. Phase 1.5 (owner-credentialed, FREE, no purchase)

Owner supplies only: an R2 bucket + bucket-scoped token. No OVH account action.

- **P15-1.** From the dev environment, point litestream and rclone at real R2 (same generated
  configs, endpoint swapped): plant canonical + blob markers per tenant, sync, `litestream
  restore` to temp paths, rclone ls; assert. The backup plane is now proven against the real
  store pre-purchase.

- **P15-2.** Re-run the local drill (D13) with the real-R2 replicas as the restore source. The
  only untested seams remaining are OVH provisioning, real DNS/ACME, and systemd-on-the-box.

- **P15-3.** Record R2 op counts vs the free tier in the summary (guardrail 3).

---

## 5. Phase 2 (owner-gated purchase)

- **P2-1.** `catalog-vps.sh` → owner picks plan_code/datacenter from live prices; record them.

- **P2-2.** `terraform plan` shown → owner go → `apply`.

- **P2-3.** Bootstrap twice; second run no-op (idempotence against reality).

- **P2-4.** Owner creates Porkbun A/AAAA records per DNS.md (service and api fqdns); verify ACME
  issuance and `/healthz` per tenant.

- **P2-5.** Backup plane on the box: markers, sync, restore-to-temp, rclone ls, assert (the
  P15-1 procedure, now from production).

- **P2-6.** Fire drill, owner-chosen variant (second-box: true provider-loss rehearsal, one more
  monthly purchase, then destroy; reinstall: free, same-box). PASS = the full D13 assertion loop
  green against the restored box; any restore failure halts go-live until understood.

- **P2-7.** Summary update: prices paid, drill variant/result, observed recovery points, and the
  follow-up list (swap stubs for real binaries as the RUN-14 line and the groups decision
  produce them; `_lexicon` TXTs on namespace decisions; bucket topology confirmation; Premium
  backup yes/no; D11 decisions if still open).

---

## 6. Explicit non-goals

No HA, no load balancer, no containers, no orchestrator, no monitoring beyond `/healthz` + an
owner-configured external pinger, no DNS automation, no remote Terraform state, no backup of any
disposable path ever, no MLS/small-group machinery in this repo (that is croft-group territory in
the discovery repo), no deciding the group write-path fork or the scale boundary (D11 asks; the
owner answers), no per-tenant VPSes until the runbook's split triggers fire. Each is one sentence
in the runbook's "why not" list so future sessions do not add them helpfully.
