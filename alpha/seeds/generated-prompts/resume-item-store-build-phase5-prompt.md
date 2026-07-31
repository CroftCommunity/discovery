# Resume prompt — cooperative metered-storage build (item-store), Phase 5 onward

Copy everything below the line into a fresh session to continue the build. It is self-contained.

---

You're in the CroftC workspace (`/Users/cpettet/git/chasemp/CroftC`). We are executing a phase-plan
build of an **experimental product**: a network-accessible, custom PDS-like **cooperative
metered-storage service** in Rust — the E0–E9 item-storage ledger ported to Rust, with an
S3-compatible interface + a thin atproto PDS blob layer, destined for VPS deploy via croft-stack.
**Phases 0–4 are DONE, committed, and pushed.** Continue from **Phase 5**.

## Orient first (sources of truth — read in this order)
1. **The build plan (single handoff artifact):**
   `discovery/alpha/plans/2026-07-31-1-plan-coop-metered-storage-service.md`. Read the **Outcome
   Summary** (top), the **Reasoning** ("Architecture layers" 5-axis picture + "Access model"), the
   **Phases**, **Open Questions**, and the **Review Log** end to end. This carries all the *why*.
2. `discovery/AGENTS.md` + `discovery/PLAYBOOK.md` — workspace orientation + filing.
3. The lane doc: `discovery/alpha/plans/2026-07-31-coop-storage-metered-hosting-lane.md`.
4. `discovery/alpha/ROADMAP_TODO.md` entries **E82** (the lane) and **E83–E86** (tracked design threads).

## Where things stand
- **Branch:** `claude/amble-coop-filing` (HEAD `9d1502b`, pushed to `origin` =
  `github-personal:CroftCommunity/discovery`). Git identity: chasemp (`chase@owasp.org`).
- **The crate:** `discovery/alpha/experiments/item-store/` — a **standalone** Rust crate (its own
  `Cargo.toml`; NOT a workspace member — run cargo from the crate dir, never `-p`). Placeholder name
  `item-store` (real name is a Phase-9 decision, A21).
- **Shipped (each TDD RED→GREEN, clippy pedantic + fmt clean, committed + plan-synced):**
  - Phase 0 discovery (`e5f0004`): rsky-pds / official-PDS / atproto surface confirmed firsthand.
  - Phase 1 (`8389a0e`) — E0 crypto/identity (`crypto.rs`, `identity.rs`).
  - Phase 2 (`9476b97`) — E1–E2 content-addressed items + signed Merkle manifest (`item.rs`, `manifest.rs`).
  - Phase 3 (`b31be48`) — E3 two-mode receipts + append-only signed ledger + canonical serialization
    (`receipts.rs`, `ledger.rs`, `canonical.rs`).
  - Phase 4 (`18fd199` + `0b0464c`) — E4 balance-forward statements + byte-day rent + rollup/purge
    (`clock.rs`, `pricing.rs`, `statements.rs`) **+ per-user SQLite persistence** (`persist.rs`,
    `rusqlite` bundled; `:memory:` mode in tests = real persistence path, no mocking).
  - **48 tests. Mutation gate green E0–E4** (`cargo-mutants`: 172 → 145 caught / 0 real survivors;
    1 excluded equivalent mutant). E86 test-hardening layer 1 (paired rejection tests) applied.

## Port oracle (the spec)
`discovery/alpha/experiments/item-storage-protocol-standalone/` — the dependency-free TypeScript E0–E14
suite (runs 81/81). Port **module-by-module, porting its assertions as the RED tests first**. Module map:
`crypto.ts`, `actor.ts` (deriveId), `item.ts`, `manifest.ts`, `canonical.ts`, `ledger.ts`, `receipt.ts`,
`statement.ts`, `clock.ts`, `rng.ts`, `pricing.ts`, `audit.ts`, `seal.ts`, `erasure.ts`, `financing.ts`,
plus `exp/e0_identity.ts … e11_financing.ts`. Cross-check the full `item-storage-protocol/` (has `SPEC.md`)
for detail. Preserve the `SEAM:` grep discipline (every mock stands in for real infra with a `SEAM:` note).

## How to work (conventions — do not deviate)
- **TDD RED→GREEN, per phase:** write the wiring test + empty module(s) + `lib.rs` decls → run
  `cargo test` and confirm it FAILS (RED) → implement → GREEN. Watch it fail before you implement.
- **Gates (run from the crate dir):** `cargo test` · `cargo clippy --all-targets -- -W clippy::pedantic
  -D warnings` · `cargo fmt --check`. All must be clean before commit. `.cargo/mutants.toml` (equivalent-
  mutant + trivial-accessor exclusions) and `clippy.toml` (`doc-valid-idents`) already exist.
- **rust-enforcer discipline:** no `unwrap`/`expect` in prod paths (only impossible-error `expect("…")`),
  `Result`/`thiserror`, `Zeroize` on secret key material, doc comments on all public items,
  `#[must_use]`, integer cents for money. No test-only methods on production types (model adversarial
  cases via legitimate constructors like `from_parts`).
- **E86 test-hardening:** for every verify/predicate path add a **paired should-fail-for-sure negative**
  (not just the happy path). Run `cargo mutants` periodically (after each phase or two) and kill real
  survivors; equivalent mutants get an `exclude_re` in `.cargo/mutants.toml` with a rationale comment.
  **Still owed (E86 layers 2–3):** `proptest` property tests (canonical round-trip, Merkle
  order-independence, sign/verify round-trips) and a **Phase-7 end-to-end abuse suite** (drive the live
  engine and try to break it: forge/replay receipts, inflate manifest, tamper at rest, walkaway, malformed
  input).
- **Commit per phase, then plan-sync:** after GREEN, commit the code; then edit the plan — mark the phase
  header `— SHIPPED (\`<sha>\`)`, add a **Delivered** note (and record any write-set expansion vs the
  Pass-2 spec), update the **Outcome Summary** row, and update the crate `README.md` Status. Commit that
  separately. **No emojis in docs** — use plain tokens (`DONE`/`SHIPPED`/`pending`), per AGENTS.md.
- **Commit message trailer:** `Co-Authored-By: Claude Opus 4.8 <noreply@anthropic.com>`.
- **Push:** the branch is already pushed and tracking; push after each phase (the user authorized pushes
  on this branch this session). If unsure, ask.
- **Gotchas:** (a) the RTK git wrapper returns **stale** `git rev-parse`/status — verify real git state
  with `dangerouslyDisableSandbox: true`. (b) cargo's cwd persists across Bash calls — a stray `cd` will
  break later relative paths; prefer absolute paths or re-`cd` to the crate dir. (c) `cargo mutants`
  writes `mutants.out*/` (gitignored) — don't commit it.

## Remaining phases (from the plan)
- **Phase 5 — spot-checks + the audit dial (port E5–E6).** `audit.rs` (random k-sample retrieve +
  re-fingerprint; detection math `1−(1−f)^k`; public-randomness challenge seeding — port `rng.ts` for a
  deterministic seeded RNG) + `dial.rs` (audit tiers, cost linear in audit count, chosen tier a signed
  declaration; audit pricing = `auditCents` in `pricing.ts`, add it now). Wiring test `e5_audit.rs`:
  measured detection ≈ `1−(1−f)^k` within tolerance (seed the RNG, log the seed), honest provider passes,
  cost scales with k not corpus size; `e6_dial.rs`: linear cost + pro-rate-on-change + tier-edge boundary
  cases. Validation: Moderate (eyeball the measured-vs-predicted table). Read oracle: `src/audit.ts`,
  `exp/e5_audits.ts`, `exp/e6_dial.ts`, `pricing.ts`.
- **Phase 6 — seal + tombstone + grace (port E7–E9).** `seal.rs` (pin root + key-ceremony mock =
  destroy write-cred → write fails closed, `SEAM:`; rotation watch; tombstone destroys rotation too) +
  `grace.rs` (grace events as signed ledger entries netting to zero). Wiring test `e7_seal.rs`
  (no write path succeeds post-ceremony; direct mutation caught vs pinned root) + `e8_tombstone.rs` +
  `e9_grace.rs`. Fail-closed paths must log LOUD. Completes the **E0–E9 ledger core**; compare pass-count
  to the TS oracle.
- **Phase 7 — S3-compatible interface + metering boundary (SEAM #1+#2).** `server.rs` (HTTP, e.g. axum —
  rsky uses Rocket but axum is fine; our S3 interface is novel, no PDS prior art) with S3-compatible
  PUT/GET; `blobstore.rs` (pluggable `BlobStore` trait; FS first, then Garage/SeaweedFS; the temp→permanent
  move may use `copy_file_range`/reflink — E84's one cheap v0 kernel edge); `main.rs` (**the runnable
  binary** — the lib can't be curl'd/deployed); an **op-dispatch `SEAM:`** (forward-compat for E83 per-DID
  cgroup scopes). Wiring test `wiring_s3_metered.rs` (HTTP PUT → signed receipt + postage tallied; GET
  returns bytes; rent recomputable) — the anti-dead-code gate. Observability = `tracing`→journald for OUR
  debugging (NOT the telemetry poller — D7: the poller reads cgroup v2, not tracing); fail-loud on any
  HTTP-boundary-vs-receipt byte-count mismatch. Validation: **Broad** — curl PUT/GET, inspect SQLite,
  assert byte-count integrity, port-leak check, **run the E86 e2e abuse suite here**.
- **Phase 8 — minimal atproto PDS blob API (in v0).** `pds_api.rs` — the D2-confirmed floor:
  `uploadBlob` (POST, **auth required** → v0 mocks auth behind a `SEAM:`), `getBlob` (GET, public),
  `listBlobs` (GET, public), mapped onto `blobstore` (mapping modeled on rsky's handlers). `uploadBlob`
  must return the exact shape `{"blob":{"$type":"blob","ref":{"$link":"<CIDv1>"},"mimeType":..,"size":..}}`;
  `listBlobs` → `{"cids":[..],"cursor":..}`. Wiring test `wiring_pds_blob.rs`. Validation: Broad — diff
  responses against the D2 shapes (no guessing).
- **Phase 9 — croft-stack deploy + VPS smoke test.** `services/<name>.toml` + role/wiring in the
  **croft-stack repo** (separate repo — two commits, two repos: croft-stack deploy + discovery docs).
  Telemetry = **systemd cgroup accounting** (`*Accounting=yes` + `MemoryHigh`/`MemoryMax`/`CPUQuota`/
  `TasksMax`), NOT app metrics; hardening baseline (`systemd-analyze security` ≈ 1.2–1.5; Rust can take
  full `MemoryDenyWriteExecute`); **never add a cgroup namespace** (breaks the poller). Docs go stale here:
  ECOSYSTEM §5c-3, COHESION §65, ROADMAP_TODO E82 status, lane "Next step", croft-stack README. **Decisions
  due at Phase 9 (the user's calls): the real service NAME (A21) + the repo/IP home + a Phase-9 probe of
  the VPS kernel version (io_uring/PSI/reflink need 5.x+, E84).**
- **Phase 10 — history-convergence consumer (gated, later).** Content-blind meer over the same store;
  RBSR set-reconciliation over envelope hashes; open `HS OC-2` (CID-vs-envelope-hash reconciliation — keep
  v0 addressing pluggable). Gated on drystone MLS being real; relay-lab E8/E9 not started.

## Settled — do NOT re-litigate (in the plan's Open Questions / Reasoning)
- Boundary = **both** (S3 metering plane + atproto PDS blob layer); Phase 8 in v0.
- Two-layer split: dumb pluggable `BlobStore` (Layer 1) under boundary metering/provenance (Layer 2);
  the backend never meters.
- Two-mode receipts (Unilateral | Bilateral, social-trust-selected). Per-user SQLite co-location (built).
- **5 orthogonal axes:** interface / index-addressing (flat now; MST/RBSR later, E85) / physical backend /
  **confidentiality** (plaintext↔encrypted; "private" = encryption, not a server read-bit) / **access**
  (public-read default / gated per object — server-enforceable even when blind; writes = owner DID +
  delegated caps). Metering is content-agnostic (blind hosting bills natively). Public relay replication
  (`subscribeRepos`) is opt-in, out of v0.
- E11–E14 (funder-diligence) deferred; E83 (compute observability) / E84 (kernel-perf) / E85 (index
  structures) tracked, post-v0.

## Immediate next step
Execute **Phase 5** (E5–E6): read the oracle (`audit.ts`, `exp/e5_audits.ts`, `exp/e6_dial.ts`,
`pricing.ts`), invoke the `phase-plan` skill's `execute.md` discipline, write the RED wiring tests, port to
GREEN, run the gates + a `cargo mutants` pass, commit + plan-sync + push. Optionally fold in E86 layer 2
(`proptest`) opportunistically. Then continue 6 → 7 → 8 → 9.

**Plan file:** `discovery/alpha/plans/2026-07-31-1-plan-coop-metered-storage-service.md`
</content>
