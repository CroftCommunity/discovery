# RUN-15 — AppView hosting kit: designed and built in discovery, staged for production

`Run summary, 2026-07-17. Branch claude/appview-infra-run-15-bgi0bd, from main at the RUN-14 merge
(e3ad447). A generic multi-tenant hosting kit built and fully validated in
alpha/experiments/appview-infra/, TDD red-first per deliverable, zero spend, no OVH account. The
kit/ subtree is written as the future root of CroftCommunity/appview-infra (D15 extracts it);
design corpus (GROUPS.md, this summary, the staged spec note) stays in discovery. Supersedes the
standalone RUN-APPVIEW-INFRA-01/-02 briefs.`

## Environment preflight

| Tool | Version | Note |
|---|---|---|
| bats | 1.10.0 | test harness |
| shellcheck | 0.9.0 | all shell clean at default severity |
| terraform | 1.9.8 | fmt clean; **validate BLOCKED** (provider registry egress) |
| sqlite3 | 3.45.1 | CLI + python `sqlite3` module |
| rclone | 1.60.1 | blob mirror (apt) |
| litestream | 0.3.13 | canonical replica (built via Go module proxy; `version` prints "development build") |
| caddy | 2.8.4 | vhosts + local drill (built via Go module proxy; `version` prints "unknown") |
| actionlint | 1.7.12 | workflow lint |
| go | 1.24.7 | used only to install litestream/caddy/actionlint through the module proxy (github release assets are 403 via the egress proxy) |
| python3 | 3.11.15 | stub + generator (stdlib only: tomllib, sqlite3, http.server) |

**One BLOCKED check:** `terraform validate` — the ovh provider download redirects to github.com,
which the egress proxy denies (403). `scripts/terraform-check.sh` runs `fmt -check` (green) and
reports validate BLOCKED rather than silently skipping it. **Unblock:** run in Phase 2 where
`registry.terraform.io` + github release assets are reachable, then `terraform validate`.

## Deliverables — red → green commit table

Every deliverable is one red (`test(infra):`) then one green (`feat(infra):`) commit; the red
precedes the green in history. Grades: **component** (unit/behaviour proven in isolation),
**local-rehearsal** (proven end-to-end in the local full-stack drill), **corpus** (a design/doc
deliverable with a mechanical consistency check).

| D | What | RED | GREEN | Grade |
|---|---|---|---|---|
| D1 | skeleton + no-secrets hygiene | `5e0b05f` | `fc7010f` | component |
| D2 | contract + stub (healthz, data profile, 401) | `3b6b800` | `3119e80` | component |
| D3 | manifests + generator (units/vhosts/litestream/rclone) | `dcc7192` | `8f3fe4c` | component |
| D4 | OVH terraform + live-catalog helper | `649b1bc` | `f2bc3b2` | component (fmt); validate **BLOCKED** |
| D5 | backup-audit invariant (taxonomy as law) | `f1a02f3` | `a0b7dd4` | component |
| D6 | one bucket, per-service prefixes + BUCKETS.md | `777e95e` | `795cb83` | component |
| D7 | idempotent bootstrap | `570ae65` | `f2a0795` | component (plan-grade; apply is Phase 2) |
| D8 | deploy workflow + forced-command receiver | `36d5d92` | `fd114b4` | component |
| D9 | DNS doc + markdown link-check | `1321dd7` | `ee2ad61` | component |
| D10 | own-data API: self-scoping, export, timeout, containment | `7afe0c7` | `3632ff5` | component (+ OS write-block via unshare) |
| D11 | group-tier design brief + staged spec note | `606a119` | `3edcbc9` | corpus |
| D12 | roster-gated serving behind GroupStore | `ceaab54` | `4c565aa` | component |
| D13 | local full-stack rehearsal + fire drill | `364275c` | `bb90137` | **local-rehearsal** |
| D14 | operator runbook | `68170ff` | `b741373` | corpus |
| D15 | extraction to standalone repo | `20b6f5c` | `174c2f9` | component (+ standalone `make check`) |
| D16 | this summary + registers | (this section) | (this commit) | corpus |

`make check` aggregates every component/local-rehearsal check (D16's own consistency check and the
extraction meta-check run explicitly, not in the aggregate — a repo re-extracting itself recurses).

## Grades — what each green actually earned

- **The fire drill (D13) is the load-bearing green.** Both tenants go through
  bring-up → plant canonical + blob markers → back up (litestream file:// replica + local rclone
  dir) → **destroy all local state** → **restore** → restart → full assertion loop. The canonical
  marker only returns if litestream restored it; the blob marker only if rclone did. This is
  **local-rehearsal grade**: real litestream, real rclone, real caddy, real sqlite — only the
  execution substrate (user-mode vs systemd) and the replica endpoints (file/local vs R2) are
  adapted. Phase 1.5 swaps the endpoints to real R2; Phase 2 swaps the substrate to the box.
- **D10 containment** is proven at OS level with a mount-namespace read-only bind (`unshare`)
  standing in for systemd `ReadOnlyPaths` (no PID-1 systemd here): a write into the data dir is
  blocked. Self-scoping/export/timeout are process-grade against live stub processes.
- **D7 bootstrap** is plan-grade: `--plan` idempotence + content verified; real double-apply is
  Phase 2 (this is not a fresh Debian box and must not be mutated).
- **D4 terraform** is fmt-grade; validate is BLOCKED (above).

## Recorded seam (found by building it)

**shared-wal live reads vs strict `ReadOnlyPaths`.** A read-only SQLite connection to a WAL
database wants a writable `-shm`; a strict read-only mount blocks it. So `shared-wal` api reads
under systemd `ReadOnlyPaths` are in tension — **snapshot mode is containment-clean** (static
`serve/state.db`, no WAL). The drill produces the snapshot the VACUUM-INTO timer would. Confirm the
shared-wal-under-ReadOnlyPaths behaviour on-box in Phase 2 before defaulting a high-read tenant to
shared-wal. (RUNBOOK escalation ladder + own_data.bats record this.)

## SPEC-DELTA register (six declared stand-ins — none weakens a proven mechanism)

All six are rows in `SPEC-DIVERGENCE-REGISTER.md` (Active table), tagged at the site.

| ID | Stands in for | Path back |
|---|---|---|
| `run15-stub-verifier` | the atproto service-auth JWT verifier (RUN-14 EXP-A) behind the same interface | swap `StubVerifier` for the real verifier with the real binary |
| `run15-local-root` | systemd `User=` non-root guarantee; the rehearsal container is root-only | on the box, `User=` provides non-root; `STUB_ALLOW_ROOT` is test-only |
| `run15-tf-validate` | `terraform validate` (provider registry BLOCKED via egress) | run validate in Phase 2 where the registry is reachable |
| `run15-bootstrap-dryrun` | real bootstrap apply / double-apply on a fresh Debian box | Phase 2 P2-3 runs `--apply` twice on the box |
| `run15-sandbox-unshare` | systemd `ReadOnlyPaths` (no PID-1 systemd here) | the box's api unit carries `ReadOnlyPaths`; asserted in generated units |
| `run15-usermode` | systemd-supervised units + R2 endpoints (drill uses user-mode processes + file/local replicas) | Phase 1.5 swaps to real R2; Phase 2 swaps to systemd on the box |

## Owner decisions surfaced (never made)

- **Group write-path fork** — Variant A (repo-canonical ciphertext, AppView decrypts) vs Variant B
  (server-canonical content). GROUPS.md §2 scores both; D12 built the fork-agnostic serving.
- **`group_scale_boundary`** — confirm 5000 or set another (a parameter, not a mechanism).
- **croft-groups launch order** — before or with stellin-appview.
- **OVH endpoint/region, plan_code, drill variant, bucket topology, Premium backup, extraction
  timing** — surfaced in terraform vars (no defaults), BUCKETS.md, EXTRACTION.md, and below.

### The D11 decision request (restated)

1. Which write-path variant for the large-group tier — **A** or **B**?
2. The `group_scale_boundary` number — confirm **5000** or set another?
3. Does **croft-groups** launch **before** or **with** stellin-appview?

## Exact Phase 1.5 sequence (owner supplies an R2 bucket + scoped token; FREE, no purchase)

- **P15-1.** Point litestream + rclone at real R2 (same generated configs, endpoint swapped): plant
  canonical + blob markers per tenant, sync, `litestream restore` to temp paths, `rclone ls`,
  assert. The backup plane proven against the real store pre-purchase.
- **P15-2.** Re-run the local drill (D13) with **real-R2 replicas** as the restore source.
- **P15-3.** Record R2 op counts vs the free tier (10 GB, 1M writes/mo) in this summary.
- Remaining untested seams after 1.5: OVH provisioning, real DNS/ACME, systemd-on-the-box.

## Exact Phase 2 sequence (from the extracted repo, owner-gated purchase)

- **P2-0.** Owner creates empty `CroftCommunity/appview-infra`; run `scripts/extract-to-repo.sh`;
  push. All further work in the new repo; discovery keeps the corpus + this summary; the new repo's
  `PROVENANCE.md` points back.
- **P2-1.** `catalog-vps.sh` → owner picks `plan_code` + `datacenter` from live prices.
- **P2-2.** `terraform plan` shown → owner go → `apply` (also confirms `terraform validate`).
- **P2-3.** `bootstrap.sh --apply` twice; second run a no-op (real idempotence).
- **P2-4.** Owner creates Porkbun A/AAAA (service + api fqdns); verify ACME + `/healthz` per tenant.
- **P2-5.** Backup plane on the box (the P15-1 marker procedure from production).
- **P2-6.** Fire drill, owner-chosen variant (reinstall = free/same-box; second-box = true
  provider-loss rehearsal, one more monthly purchase then destroy). PASS = the full D13 assertion
  loop green against the restored box; any restore failure halts go-live.
- **P2-7.** Summary update in the new repo (mirrored here): prices paid, drill variant/result,
  observed recovery points, follow-ups (swap stubs for real binaries as RUN-14's line + the D11
  decision produce them; `_lexicon` TXTs on namespace decisions; bucket topology confirmation;
  Premium backup yes/no).

## Reproduce

```bash
cd alpha/experiments/appview-infra/kit
make check           # the full gate (component + local-rehearsal)
make local-up        # bring the stack up on localhost (no credentials)
make local-drill     # destroy → restore → assert, end to end
make check-extraction  # extract to a temp dir; make check passes standalone
# corpus consistency (outside the kit gate):
bats ../corpus-tests/*.bats
```
