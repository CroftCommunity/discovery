# Resume prompt — cooperative metered-storage build (item-store), Phase 7 (S3 metering boundary)

Copy everything below the line into a fresh session to continue the build. Self-contained and compact.

---

You're in the CroftC workspace (`/Users/cpettet/git/chasemp/CroftC`). We are executing a phase-plan build
of an **experimental product**: a network-accessible, custom PDS-like **cooperative metered-storage
service** in Rust — the E0–E9 item-storage ledger ported to Rust, gaining an S3-compatible interface + a
thin atproto PDS blob layer, destined for VPS deploy via croft-stack. **The E0–E9 ledger core (Phases 0–6)
is DONE, mutation-gated, and MERGED to `main`** (PR #39). Continue from **Phase 7**.

## Orient (sources of truth — read in this order)
1. **The build plan (the single handoff artifact):**
   `discovery/alpha/plans/2026-07-31-1-plan-coop-metered-storage-service.md`. Read the **Outcome Summary**,
   the **Reasoning** ("Architecture layers" 5-axis picture + "Access model" + "meter the boundary"),
   **Phase 7** (+ its Verified Assumptions **D1 / D3 / D6 / D7**), **Open Questions**, and the **Review Log**
   (incl. the **Decision 2026-07-31 — zero-downtime + Caddy / E87** and the mutation Test-run entries).
2. `discovery/AGENTS.md` + `discovery/PLAYBOOK.md` — workspace orientation + filing.
3. Lane: `discovery/alpha/plans/2026-07-31-coop-storage-metered-hosting-lane.md`; `ROADMAP_TODO.md` **E82**
   (status) + **E83–E87** (tracked threads that attach in Phase 7: E83 op-dispatch seam, E84 reflink,
   E87 graceful-shutdown seam).

## Where things stand
- **Branch:** `claude/item-store-phase7`, cut fresh off `origin/main` (which contains the merged core). Git
  identity: **chasemp** (`chase@owasp.org`, remote `git@github-personal:CroftCommunity/discovery`); before
  `gh`, run `gh auth switch --user chasemp`. **Pull `main` + rebase before starting** (main moves fast).
- **The crate:** `discovery/alpha/experiments/item-store/` — a **standalone** Rust crate (its own
  `Cargo.toml`; NOT a workspace member — run cargo from the crate dir, never `-p`). Placeholder name
  `item-store` (real name is a Phase-9 decision, A21).
- **Shipped E0–E9** (each TDD RED→GREEN, clippy pedantic + fmt clean, mutation-gated, committed + plan-synced):
  P1 E0 crypto/identity `8389a0e` · P2 E1–E2 items+manifest `9476b97` · P3 E3 receipts+ledger+canonical
  `b31be48` · P4 E4 statements + per-user SQLite `18fd199`+`0b0464c` · P5 E5–E6 audit+dial+seeded-RNG
  `1640b17` · P6 E7–E9 seal/tombstone+grace `e9b06bc`. **88 tests; E0–E9 mutation gate green (zero real
  survivors).** Modules present: `crypto, identity, item, manifest, canonical, ledger, receipts, clock,
  pricing, statements, persist, rng, audit, dial, seal, grace`.

## How to work (conventions — do not deviate)
- **TDD RED→GREEN per phase:** write the wiring test + empty module(s) + `lib.rs` decls → `cargo test` and
  confirm it FAILS (RED) → implement → GREEN. Watch it fail first.
- **Gates (from the crate dir):** `cargo test` · `cargo clippy --all-targets -- -W clippy::pedantic -D
  warnings` · `cargo fmt --check`. All clean before commit. `.cargo/mutants.toml` + `clippy.toml` exist.
- **rust-enforcer:** no `unwrap`/`expect` in prod paths (only impossible-error `expect("…because…")`);
  `Result`/`thiserror`; **secret key material** in newtypes with **`Zeroize` + manual redacting `Debug`**
  (never printed/serialized — see `seal.rs` `CollectionWriter`/`UnsealAuthority`); doc comments + `#[must_use]`
  on public items; integer cents; **no test-only methods on prod types**.
- **E86 mutation discipline** (run `cargo mutants --file src/<new>.rs …` after the phase; kill real
  survivors): for a **non-behavioral internal** (hashing/mixing/encoding), pin the **exact** output with a
  **golden-vector test** (see `rng.rs` — golden vectors from the TS oracle) rather than only property
  asserts. For a **predicate**, assert **both** directions (a mutant `-> true`/`-> false` must fail); an
  **always-true-by-construction** predicate is a test-only smell — remove it and assert the arithmetic. Only
  `exclude_re` genuinely equivalent mutants, with a rationale comment.
- **Commit per phase, then plan-sync (separate commit):** after GREEN + gates + mutants, commit the code;
  then edit the plan — header `— SHIPPED (\`<sha>\`)`, a **Delivered** note (record any write-set expansion),
  the **Outcome Summary** row, a Review-Log mutation Test-run entry; update `ROADMAP_TODO` E82 status + the
  crate `README.md` Status. **No emojis in docs** (`DONE`/`SHIPPED`/`pending`). Commit trailer:
  `Co-Authored-By: Claude Opus 4.8 <noreply@anthropic.com>`. **Push after each phase** (user-authorized).
- **Gotchas:** (a) the RTK git wrapper returns **stale** `rev-parse`/`status` — verify real state with
  **`rtk proxy git …`**. (b) cargo's cwd persists across Bash calls — a stray `cd` breaks later relative
  paths; prefer absolute paths or re-`cd` to the crate dir. (c) `cargo mutants` writes `mutants.out*/`
  (gitignored) — don't commit it.

## Phase 7 spec — S3-compatible interface + the metering boundary (SEAM #1 + #2)
**No port oracle** — this is the **novel** part (a **case-b S3-compatible *client* interface** has **no PDS
prior art**; both rsky-pds and the official PDS use S3 only as an internal *backend*, D6). Build from the S3
API + the plan; **learn from `rsky-pds`** for the `BlobStore` trait shape + handler→backend mapping (D1),
don't fork it. **Goal:** a real HTTP server where **the network boundary IS the metering boundary** — every
transfer emits a signed receipt (postage) and rent derives from the customer's signed manifest.

- `src/server.rs` — HTTP server (**axum** + tokio; axum is the settled choice — our interface is novel, no
  need to mirror rsky's Rocket) with **S3-compatible PUT/GET** object routes.
- `src/blobstore.rs` — the pluggable `BlobStore` trait (dumb Layer-1 backend; **FS first**, then
  Garage/SeaweedFS/R2) + the boundary byte-count → receipt hook. **[E84]** the temp→permanent move +
  content-addressed dedup may use `copy_file_range`/`FICLONE` reflink (the one cheap v0 kernel edge; no
  hardening cost). This trait is the E84 attach point for later io_uring/zero-copy backends.
- **op-dispatch `SEAM:`** — route handling through a small `Op` enum / dispatch fn (not inlined in the HTTP
  handler) so a later per-DID compute-observability wrapper (**E83**) can scope a *heavy* op into a per-DID
  cgroup without a rewrite. **Not a v0 feature — just the seam.** Cheap ops (blob PUT/GET) are never scoped.
- `src/main.rs` — the **runnable binary** (the lib can't be curl'd/deployed). **[E87] build-from-the-start
  seam:** optionally inherit a listening fd (systemd socket-activation via `sd_listen_fds`/`LISTEN_FDS`) +
  a SIGTERM **graceful-shutdown** path (stop-accept → drain in-flight → SQLite `wal_checkpoint(TRUNCATE)` →
  exit 0). Cheap seam so the zero-downtime spike can measure options later; not a v0 feature.
- Persistence note: `persist.rs`'s `rusqlite::Connection` is `!Sync` (a Phase-4b `SEAM:`) — resolve pooling
  here (e.g. a per-DID connection pool / `r2d2_sqlite`, or a single-writer actor task).
- **Wiring test `tests/wiring_s3_metered.rs`** (the anti-dead-code gate; RED→GREEN): HTTP **PUT** bytes →
  a signed receipt is recorded + postage tallied; **GET** returns the exact bytes; rent recomputable from
  the manifest. Bind an **ephemeral port**, assert no port leaks. (Drive via `reqwest` or
  `tower::ServiceExt::oneshot` against the axum `Router`.)
- **Observability:** structured `tracing` → stdout/journald **for our own debugging** (**[D7]** the
  croft-stack poller reads **cgroup v2**, NOT tracing — this is not a poller contract). Per HTTP request
  (method/key/status/bytes-in/out, INFO); per receipt (id/mode/running-total/ledger-index, INFO); backend
  (impl/CID/bytes, DEBUG); **fail-loud WARN/ERROR on ANY byte-count mismatch between the HTTP boundary and
  the receipt** (the metering-integrity invariant). Never log secret material.
- **Validation: Broad.** Out-of-harness: `curl` PUT then GET a real object against a locally-run instance;
  inspect the per-user SQLite for the receipt + running total; **assert HTTP-boundary byte count == receipt
  byte count**; recompute rent independently; confirm the ephemeral port is released. **Run the E86
  end-to-end abuse suite here** — drive the live engine and actively try to break it (forge/replay receipts,
  inflate the manifest, tamper at rest across the boundary, walkaway, double-count audit, malformed input).
- **New deps:** `axum`, `tokio` (rt-multi-thread, macros, signal), a test client (`reqwest` or `tower`),
  `tracing` + `tracing-subscriber`. Keep the hot path simple (tokio epoll + std I/O + SHA-NI); adopt only
  `copy_file_range`/reflink early (E84).

## Settled — do NOT re-litigate
Boundary = **both** (S3 metering plane + atproto PDS blob layer, Phase 8 in v0). Two-layer split: dumb
pluggable `BlobStore` (Layer 1) under boundary metering/provenance (Layer 2); the backend never meters.
axum is fine (learn from rsky, don't fork). 5 orthogonal axes (interface / index-addressing (flat v0) /
physical backend / confidentiality / access) compose freely; metering is content-agnostic. Access model:
public-read default, gateable per object, writes = owner DID + delegated caps — lands on the Phase-8
`uploadBlob` auth `SEAM:`, not the E0–E9 core. Metering records co-locate in per-user SQLite. E83/E84/E85/
E86/E87 are tracked post-v0 threads — land only their **seams** in v0.

## Immediate next step
Execute **Phase 7**: read the plan's Phase-7 spec + D1/D3/D6/D7 and the `rsky-pds` `BlobStore`/handler notes;
invoke the `phase-plan` skill's `execute.md` discipline; write the RED `tests/wiring_s3_metered.rs` + empty
`server.rs`/`blobstore.rs`/`main.rs` + `lib.rs` decls; port to GREEN (minimal S3 PUT/GET subset, mark the
rest `SEAM:`); run the gates + a scoped `cargo mutants`; do the Broad out-of-harness checks + the E86 e2e
abuse suite; commit + plan-sync + push. Then **Phase 8** (atproto blob API `uploadBlob`/`getBlob`/`listBlobs`
over the same metered path — exact D2-confirmed shapes, no guessing). **Phase 9 (deploy) is gated on the
user's decisions** (service name A21 / repo-IP home / VPS-kernel probe) — do not start it unprompted.

**Plan file:** `discovery/alpha/plans/2026-07-31-1-plan-coop-metered-storage-service.md`
