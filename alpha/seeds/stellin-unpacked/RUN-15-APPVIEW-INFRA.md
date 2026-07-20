# RUN-15: AppView hosting kit, designed and built in discovery, staged out for production

`Status: runnable brief, 2026-07-17. Supersedes the standalone RUN-APPVIEW-INFRA-01/-02 briefs;
do not execute those. HOMING: Phase 1 and Phase 1.5 execute INSIDE the discovery repo under its
house guardrails, keeping the design work tight to the corpus; Phase 2 (real provisioning)
executes from a new standalone repo populated by an explicit extraction deliverable. Numbering
assumes the RUN-14 brief (Stellin AppView spikes) holds its number whether or not it has
executed; confirm against alpha/experiments/MASTER-INDEX.md and renumber consistently if taken.
This run does NOT depend on RUN-14 having run (the auth verifier is stubbed behind an interface
here); the two are parallel-safe.`

You are continuing the Croft/Drystone program. Orientation: `discovery/AGENTS.md` and the
registers in `alpha/experiments/` (`MASTER-INDEX.md`, `EXPERIMENT-BACKLOG.md`,
`SPEC-DIVERGENCE-REGISTER.md`). Read those first; do not re-derive what they establish.

---

## 0. Owner pre-steps

1. None for Phase 1 (it lands entirely in discovery). Before Phase 2: create the empty repo
   `CroftCommunity/appview-infra` for the extraction (D16) to target, and decide the OVH
   endpoint (`ovh-eu`, `ovh-ca`, `ovh-us`; US is a separate subsidiary with separate accounts).

2. No domains needed. All fqdns are manifest fields with croft.ing staging defaults.

---

## 1. Context (read once)

### 1.1 What is being built and where it lives

A generic multi-tenant hosting kit for the Croft portfolio's small always-on services (single
static binary, SQLite, optional long-lived ingest, HTTPS serving), targeting one OVH VPS with
Litestream-to-R2 backup and Porkbun DNS. Phase 1 builds and validates the ENTIRE kit, including
a full local rehearsal, at `alpha/experiments/appview-infra/` in this repo:

```
alpha/experiments/appview-infra/
  README.md            # experiment-style summary (goal/approach/effort/result)
  GROUPS.md            # D11 design brief (corpus material; stays in discovery)
  kit/                 # everything that will graduate to the standalone repo
    Makefile  services/  scripts/  generated/  terraform/  bootstrap/
    config-templates/  stub/  drill/  docs/  .github/
```

The `kit/` subtree is written from day one as the future root of `CroftCommunity/appview-infra`;
D16 extracts it. Design documents (GROUPS.md, this run's summary, spec-facing notes) are corpus
and STAY in discovery.

Tenants known and candidate: **stellin-appview** (marquee first tenant; filtered Jetstream
ingest + XRPC serving, per the RUN-14 spike line), **croft-groups** (near-term; the access-gated
large-group tier, shared mechanism with Stellin groups, see 1.4), arecipe feed service
(candidate, not built), future pdsview/skylite adjacencies.

Each tenant is one manifest, `kit/services/<name>.toml`:

```toml
name        = "stellin-appview"
fqdn        = "stellin-staging.croft.ing"
port        = 8101                       # unique per service
artifact    = "stub"                     # later: github:CroftCommunity/<repo>/releases
serve_api   = true                       # generates the own-data API sidecar (1.3)
api_mode    = "shared-wal"               # or "snapshot"
gated_groups = true                      # roster-gated group serving (1.4)
data_profile.canonical  = ["state.db"]   # Litestream targets (sqlite only)
data_profile.disposable = ["index.db"]   # rebuilt from source; NEVER backed up
data_profile.blobs      = ["blobs/"]     # rclone-mirrored dirs
```

Everything downstream is GENERATED from manifests: systemd units, Caddy vhosts, litestream.yml,
rclone timers, drill assertions. Hand-editing generated files is forbidden; generator +
manifests are the source of truth.

### 1.2 State taxonomy (kit invariant, every tenant)

Canonical = observation-born or grant-born state nothing on the network can rebuild (cursors,
telemetry, grants, group rosters, any server-held keys). Disposable = projections rebuildable
from firehose/backfill/sealed history; NEVER backed up; enforced by check (D5). Blobs = data the
service stores and distributes without needing to read it. Off-provider backup because provider
backups share the provider's failure domain (OVH's own docs: backups replicate within the same
datacentre). R2 because zero-egress restores are free when they matter and the free tier
(10 GB, 1M writes/mo) covers this scale.

### 1.3 The per-user own-data API (decided 2026-07-17)

Users reach canonical records at their PDS; the AppView uniquely holds their observation-born
and grant-born state (telemetry, grants, roster memberships, entitled sealed blobs) plus derived
views. The API is self-scoping: verified caller DID; every query implicitly
`subject_did = caller`; plus paginated bulk export. Served by a separate contained process per
tenant, `<name>-api`:

- `shared-wal` mode: SQLite WAL permits many read-only connections in separate processes
  concurrent with the single writer on the same files; the "replica" is free.

- Containment is mechanical: dedicated user; `ReadOnlyPaths=` on the data dir (OS-incapable of
  writing); own port and `api.<fqdn>` vhost; `CPUQuota=` + IO scheduling so exports cannot
  starve the writer's cursor; statement timeouts and paginated short transactions because long
  reads pin the WAL.

- `snapshot` mode: a timer runs `VACUUM INTO .../serve/state.db` and atomically swaps; the API
  serves the snapshot (zero writer interaction, minutes of staleness). `serve/` is derived,
  classed disposable.

- Second-box escalation (continuous restore-from-R2 serving the API) is a runbook paragraph,
  not built.

Auth is the atproto service-auth JWT pattern being proven in RUN-14 EXP-A; here it is a stubbed
verifier behind the same interface.

### 1.4 The access-gated large-group tier (stated 2026-07-17)

Design stance on record: past a scale boundary (working number ~5,000 members; a parameter, not
this run's decision), E2EE group confidentiality is a mirage (any member leaks), so large groups
serve FROM the AppView as **private but not E2EE**: roster-gated serving to verified member
DIDs, trusted-gatekeeper posture stated honestly, with search, helpers, and moderation native.
The same mechanism is intended for Stellin groups and Croft Groups. Below the boundary, groups
remain MLS-sealed (croft-group crates) with the AppView as content-blind distributor per the §H
hybrid topology.

Two consequences for this run:

- **A write-path fork exists and is an OWNER DECISION** (guardrail 5): where large-group content
  canonically lives. Variant A: authors publish ciphertext records to their own repos sealed to
  a server-held group scope key; the AppView decrypts to serve; server canonical state = keys +
  roster (small, in state.db); content stays repo-canonical; index stays disposable. Variant B:
  members write directly to the AppView API; server-canonical content (blobs/ + state.db);
  simplest immediate build; heaviest backup and weakest portability story. D11 delivers the
  comparison; D12 builds only the fork-agnostic parts behind a storage trait.

- **Spec-status handling is native here.** Because this run executes in discovery, D11's
  spec-facing note (the large-tier stance deliberately taking the trusted-gatekeeper arm of
  social-mapping's open AppView-scope-key item, for the large tier only) is APPENDED to
  `beta/drystone-spec/proposed-changes-2026-07-experiment-reconciliation.md` with a row in
  `beta/impl/experiments/drystone-reviews-and-experiments-log.md`, same commit, per house
  guardrails. The reviewed spec files themselves are never edited.

### 1.5 Service contract

`kit/CONTRACT.md` (D2): flags `--data-dir` and `--listen`; `/healthz` 200 when ready; all state
under the data dir; no root; no ports below 1024. API addendum for `serve_api` tenants:
read-only opens, statement timeout, pagination limits. Gated-groups addendum: the verifier
interface and roster-gate semantics. A shared stub satisfies contract + addenda so the kit, the
local rehearsal, and the drill run before any real binary exists.

---

## 2. Guardrails

1. **Branch discipline (discovery house rules).** Do not commit on `main` or any branch under
   review. `git checkout -B claude/appview-infra-run-15 origin/main`. One commit per
   deliverable; `test(infra):` red before `feat(infra):` green; the summary shows the order.

2. **Never edit the reviewed spec.** `beta/drystone-spec/part-*` and conventions are
   review-frozen; spec-facing output goes to the proposed-changes staging doc + the reviews log,
   same commit (see 1.4).

3. **The honesty contract.** Any green resting on a stand-in (stub verifier, fixture roster,
   user-mode systemd adaptation, dry-run bootstrap check) carries a
   `SPEC-DELTA[<id> | stand-in]` tag at the site and a register row in the same commit.
   Distinguish component-grade, local-rehearsal-grade, and (later) production-grade greens; the
   summary must say which each deliverable earned.

4. **No secrets in the repo, ever.** Credentials only via environment (OVH_* trio; R2 as
   AWS_*/LITESTREAM_*). `.gitignore` within `kit/`: `*.tfstate*`, `.terraform/`, `*.env`.
   Secret-shaped-string grep in `make check` (D1).

5. **Owner decisions surfaced, never made.** OVH endpoint/region, plan code, drill variant,
   bucket topology, Premium backup, the group write-path fork, the scale boundary number, and
   extraction timing. Write the options up; proceed to the next deliverable.

6. **Money gate.** Nothing in Phase 1 or 1.5 spends money or requires an OVH account. Phase 1.5
   uses only the R2 free tier and asserts usage stays within it. All OVH ordering is Phase 2,
   from the extracted repo, with explicit owner go and live catalog prices shown first.

7. **Stop rules.** ~30 min time-box on stuck installs/builds; checkpoint, PARTIAL, move on.
   Tools that will not install via the proxy mark their checks BLOCKED, never silently skipped.
   No scope beyond the deliverables.

8. **Registers.** New rows in `EXPERIMENT-BACKLOG.md` and `MASTER-INDEX.md` for this run; the
   run ends with `alpha/experiments/RUN-15-SUMMARY.md` in the established summary format.

---

## 3. Phase 1 deliverables (in discovery, unattended, free, in order)

**D1. Skeleton and hygiene.** `kit/Makefile` (`check` target), `kit/.gitignore`,
`kit/scripts/no-secrets-check.sh` wired in. Red state: `make check` exists and fails.

**D2. Contract and stub.** `kit/CONTRACT.md` + addenda (1.5); `kit/stub/` implementing all of
it (healthz; data-dir files per a passed profile; a trivial authed echo route behind the
verifier interface). Check: bats starts the stub, asserts healthz, files, and 401 on
unauthenticated authed routes.

**D3. Manifests and generator.** `kit/services/stellin-appview.toml` and
`kit/services/croft-groups.toml` (both artifact "stub"), `kit/services/_example.toml`
documented. `kit/scripts/render.sh` (or a small Rust tool if cleaner) emitting into
`kit/generated/`: per-service systemd unit (dedicated user, StateDirectory, Restart=always,
NoNewPrivileges, ProtectSystem=strict, ReadWritePaths on its data dir, PrivateTmp); per-fqdn
Caddy vhosts (HTTP-01; Porkbun holds only A/AAAA records); litestream.yml section per canonical
sqlite (1s sync, 24h snapshots, 168h retention); rclone service+timer per blob dir (5 min,
`--immutable` where content-addressed, flagged where not); per `serve_api` tenant the api unit
(ReadOnlyPaths, CPUQuota, IOSchedulingClass, own port, `api.<fqdn>` vhost) plus the VACUUM INTO
timer in snapshot mode. Checks first: bats fixtures with fake manifests asserting the exact
generated set; port-collision detection; rejection of canonical/disposable overlap; assertion
that api units carry ReadOnlyPaths and never ReadWritePaths on the data dir.

**D4. Terraform.** `kit/terraform/`: ovh provider pinned; variables (endpoint, datacenter,
plan_code REQUIRED with no guessed default, ssh key); `ovh_vps` + Debian 12; outputs
IPs/service name. `kit/scripts/catalog-vps.sh` querying `/order/catalog/public/vps` for live
plan codes and prices (codes are undocumented and generation-specific; this is how the owner
picks in Phase 2). Standard daily backup is included free on current VPS; order no backup
option; comment that Premium 7-day is one plan block later. Checks: `terraform fmt -check` +
`terraform validate` (credential-free).

**D5. Backup-audit invariant.** `kit/scripts/backup-audit.sh` in `make check`: parses manifests
+ generated backup configs; FAIL if any disposable path (including api `serve/` dirs) is a
backup target, or any canonical path lacks one. Red first against a broken fixture. The state
taxonomy promoted to executable kit law.

**D6. Bucket topology.** Default ONE R2 bucket, per-service prefixes
(`<bucket>/<service>/{state,blobs}`), one scoped token. `kit/docs/BUCKETS.md` states the
tradeoff (one leaked token exposes all tenants' canonical state; per-service buckets is the
isolation upgrade) as an owner call. Generator emits prefix-correct paths (D3 fixtures
extended).

**D7. Bootstrap.** `kit/bootstrap/bootstrap.sh`, idempotent on fresh Debian 12: per-service and
per-api users from manifests; nftables 22/80/443; unattended-upgrades; SSH hardening; Caddy
from official apt repo; Litestream + rclone pinned in `versions.env`; install `generated/`;
enable units. Second run is a no-op exiting 0. Checks: shellcheck; bats double-run in
container/chroot if the environment allows, else `--plan` dry-run assertions with the
limitation recorded (SPEC-DELTA per guardrail 3).

**D8. Deploy pipeline.** `kit/.github/workflows/deploy-service.yml` (workflow_call: service
name + artifact); server-side `deploy` user with forced-command rsync into `/opt/<service>/`,
atomic move, restart that unit only. Checks: actionlint if installable; bats on the receive
script.

**D9. DNS doc (Porkbun, manual v1).** `kit/docs/DNS.md`: per-service and per-api A/AAAA records
from Terraform outputs; ACME is HTTP-01 so no DNS API needed; future `_lexicon` TXTs listed as
pending per-app namespace decisions, not created; one line that Porkbun has an API if
automation is ever wanted. Check: link check in `make check`.

**D10. Own-data API behavior in the stub.** Extend the stub with self-scoping semantics:
`getMyRows` returns only rows whose subject matches the verified caller; `export` streams
paginated; a deliberately slow query hits the statement timeout. Red-first matrix: caller A
never sees caller B's rows; unauthenticated 401; timeout fires; and a process-level test that
the api process, run under the generated unit's constraints (systemd-run with the same
sandboxing where the environment allows), CANNOT write into the data dir; where sandboxing
cannot be exercised, assert the unit fields and record the limitation.

**D11. Group-tier design brief (corpus deliverable; stays in discovery).**
`alpha/experiments/appview-infra/GROUPS.md`:

- The tier definition per 1.4 with honest-posture language drafted (private, roster-gated, NOT
  cryptographically confidential; the member-leak equivalence argument; boundary as parameter
  `group_scale_boundary`, working number 5,000, decision deferred).

- The write-path fork analyzed: Variant A vs Variant B (1.4), each scored against the state
  taxonomy, backup cost, restore-drill shape, moderation and helper mechanics, migration path
  between variants, and the §H posture including the deliberate trusted-gatekeeper acceptance
  for this tier only.

- The spec-facing note APPENDED to the proposed-changes staging doc + reviews-log row, same
  commit (1.4). The reviewed spec is untouched.

- Ends with the decision request: variant, boundary number, and whether croft-groups launches
  before or with stellin. Check: link check plus a bats-greppable claims list verifying the
  brief's stated invariants match 1.2/1.4 (mechanical consistency check, recorded as such).

**D12. Roster-gated serving, fork-agnostic, in the stub.** Behind a `GroupStore` trait both
variants can implement: `roster(group)`, `content(group, cursor)`. Stub implements a fixture
store. Red-first matrix mirroring RUN-14 EXP-A: member DID gets content; non-member 403;
anonymous 401; removed member 403 on next call; and the offering-vs-reading distinction
documented as NOT applying in this tier (the server reads by design; the honest posture is the
feature). Verifier is the stubbed interface; comment marks real service-auth verification as
RUN-14 EXP-A territory, swapped in with the real binary. Roster and grants are classed
canonical in the manifests (D3 fixtures assert; D5 enforces).

**D13. Local full-stack rehearsal (the pre-purchase capstone).** `make local-up`: every
tenant's stub + api process on localhost ports using the generated configuration adapted to
user-mode execution (systemd user units where available, plain supervised processes otherwise;
adaptation recorded as SPEC-DELTA); Caddy on high ports, plain HTTP locally; litestream to a
`file://` replica; rclone to a local directory standing in for R2. Then
`kit/drill/fire-drill.sh --variant local`: destroy local state dirs, restore from local
replicas, restart, run the FULL per-tenant assertion loop (healthz; canonical marker; blob
marker; api self-scoping spot-check; gated-group member/non-member spot-check). Proves drill
logic, generator output, and restore choreography end to end with zero credentials and zero
spend. The drill's assertions ARE the check; red first by running it before restore logic
exists.

**D14. Runbook.** `kit/docs/RUNBOOK.md`: box loss (re-apply, bootstrap, per-manifest restore +
copy-back, start, disposables rebuild; recovery points stated: seconds canonical sqlite, 5 min
blobs, zero disposables by construction); cursor loss; Litestream failure alerting via
journald; credential rotation; ADD-A-TENANT in ~5 steps; the api escalation ladder
(shared-wal → snapshot → second box) with triggers; WHEN-TO-SPLIT a tenant to its own VPS; the
"why not" list (section 6). Checks: shellcheck on referenced scripts; link check.

**D15. Extraction plan for stage 2 (built now, executed at P2-0).**
`kit/scripts/extract-to-repo.sh` + `kit/docs/EXTRACTION.md`: produce the standalone
`CroftCommunity/appview-infra` content from the `kit/` subtree (git subtree split or a clean
copy with history noted), root = kit contents; include a generated `PROVENANCE.md` in the new
repo ("extracted from CroftCommunity/discovery @ <commit>, design corpus remains there:
GROUPS.md, RUN-15 summary"); corpus files (GROUPS.md, summaries, spec notes) are EXCLUDED and a
pointer README section links back. Check: bats runs the extraction into a temp dir and asserts
the exact expected tree, presence of PROVENANCE.md, absence of corpus files, and that
`make check` passes inside the extracted tree standalone.

**D16. Run summary + registers.** `alpha/experiments/RUN-15-SUMMARY.md`: red/green commit table
per deliverable; grade per green (component vs local-rehearsal); tool versions; BLOCKED items
with one-line unblocks; SPEC-DELTA register rows; the D11 decision request restated; exact
Phase 1.5 and Phase 2 sequences. Backlog and MASTER-INDEX rows updated.

---

## 4. Phase 1.5 (in discovery, owner-credentialed, FREE, no purchase)

Owner supplies only an R2 bucket + bucket-scoped token. No OVH account action.

- **P15-1.** From the dev environment, point litestream and rclone at real R2 (same generated
  configs, endpoint swapped): plant canonical + blob markers per tenant; sync;
  `litestream restore` to temp paths; rclone ls; assert. The backup plane is proven against the
  real store pre-purchase.

- **P15-2.** Re-run the local drill (D13) with real-R2 replicas as the restore source. The only
  untested seams remaining are OVH provisioning, real DNS/ACME, and systemd-on-the-box.

- **P15-3.** Record R2 op counts vs the free tier in the summary (guardrail 6).

---

## 5. Phase 2 (from the extracted repo, owner-gated purchase)

- **P2-0.** Owner creates the empty `CroftCommunity/appview-infra`; run the D15 extraction; push;
  from here on, work in the new repo. Discovery keeps the design corpus and the run summary; the
  new repo's PROVENANCE.md points back.

- **P2-1.** `catalog-vps.sh` → owner picks plan_code/datacenter from live prices; record them.

- **P2-2.** `terraform plan` shown → owner go → `apply`.

- **P2-3.** Bootstrap twice; second run no-op (idempotence against reality).

- **P2-4.** Owner creates Porkbun A/AAAA records per DNS.md (service and api fqdns); verify ACME
  issuance and `/healthz` per tenant (stubs).

- **P2-5.** Backup plane on the box: the P15-1 marker procedure, now from production.

- **P2-6.** Fire drill, owner-chosen variant (second-box: true provider-loss rehearsal, one more
  monthly purchase, then destroy; reinstall: free, same-box). PASS = the full D13 assertion loop
  green against the restored box; any restore failure halts go-live until understood.

- **P2-7.** Summary update (in the new repo, mirrored as a closing note to the discovery
  summary): prices paid, drill variant/result, observed recovery points, follow-ups (swap stubs
  for real binaries as RUN-14's line and the D11 decision produce them; `_lexicon` TXTs on
  namespace decisions; bucket topology confirmation; Premium backup yes/no).

---

## 6. Explicit non-goals

No HA, no load balancer, no containers, no orchestrator, no monitoring beyond `/healthz` + an
owner-configured external pinger, no DNS automation, no remote Terraform state, no backup of any
disposable path ever, no MLS/small-group machinery in the kit (croft-group territory), no
deciding the group write-path fork or the scale boundary (D11 asks; the owner answers), no
per-tenant VPSes until the runbook's split triggers fire, and no production work from the
discovery repo (Phase 2 runs only from the extracted repo). Each is one sentence in the
runbook's "why not" list so future sessions do not add them helpfully.
