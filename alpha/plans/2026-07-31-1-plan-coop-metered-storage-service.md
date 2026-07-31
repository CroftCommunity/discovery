# Cooperative metered-storage service — build plan (Rust custom-PDS-like store)

> Phase-plan (three-pass). This is the **build plan**; the lane overview + committed direction live in
> `2026-07-31-coop-storage-metered-hosting-lane.md` (E82). Passes 1–3 + Phase 0 discovery complete;
> execution underway.

## Outcome Summary

| Phase | Status | Commit | Note |
|---|---|---|---|
| 0 Discovery | DONE | `e5f0004` | 7 tasks resolved firsthand; D7 corrected telemetry to cgroup-accounting; no v0 blocker. |
| 1 crypto/identity (E0) | SHIPPED | `8389a0e` | Deterministic Ed25519 derive + id derivation + sign/verify/pin; 9 tests, clippy/fmt clean. |
| 2 items + manifest (E1–E2) | SHIPPED | `9476b97` | Content-addressed `Item`/`ContentStore` (tamper-evident, dedup) + canonical signed Merkle manifest; 19 tests. |
| 3 receipts (E3) | SHIPPED | `b31be48` | Two-mode receipts (Bilateral/Unilateral) + append-only signed ledger + canonical serialization; walkaway/forgery caught; 30 tests. |
| 4 statements (E4) | SHIPPED | `18fd199`+`0b0464c` | Statement chain + byte-day rent + rollup/purge (4a) + per-user SQLite persistence (4b, `:memory:` tests); 48 tests. |
| 5 audit + dial (E5–E6) | pending | — | |
| 6 seal + grace (E7–E9) | pending | — | |
| 7 S3 metered boundary | pending | — | |
| 8 atproto PDS blob surface | pending | — | |
| 9 croft-stack deploy | pending | — | |
| 10 convergence consumer | gated | — | Phase-10 gated (drystone/MLS not real yet). |

## Problem Statement

The co-op's flagship non-extractive service is **PDS-shaped metered storage**. The *protocol* is proven
in code — the `item-storage-protocol` suite is green-real (the standalone ran **81/81 assertions across
E0–E11** this session), but it is an in-process, deterministic **experiment** with `SEAM:` markers where
real infrastructure belongs. The user has committed (2026-07-31) to building it as an **experimental
product**: a **network-accessible, custom PDS-like storage service in Rust**, with **metering built in**
(the E0–E9 ledger) and an **S3-compatible interface**, deployed on the VPS via **croft-stack**, so we can
run and test against it and dogfood it ourselves.

It must double as the **substrate for the MLS history-convergence server** (a content-blind meer,
croft-stack Phase 10 `10-drystone-layer.md`): one metered, content-blind, network-accessible store under
**two** consumers — PDS blob hosting and history convergence.

Constraints: no browser extension, but **network is open** (WebFetch/WebSearch, Playwright available).
The co-op and its storage service are **unnamed** (ROADMAP_TODO A21) — build under a descriptive
placeholder (`item-store`), not a brand. Advances the existential **D5** gate; the legal-review gate
stays deferred (user).

## Reasoning

**Why port, not rewrite.** E0–E9 is a complete, adversarially-tested design with exact ledger arithmetic
and real crypto (Ed25519/SHA-256). The residual risk is entirely in the `SEAM:`s (network boundary, real
blob backend, real CIDs) and in the atproto/PDS network surface — not in the ledger logic. So the plan
**ports the proven core to Rust module-by-module (TDD, porting the assertions)**, then closes the SEAMs.
This keeps every phase small and lets the Rust suite be checked against the TS reference behavior.

**Why Rust + rsky-pds as prior art.** The user chose Rust; it matches the corpus's substrate stance and
the global Rust discipline (rust-enforcer). `rsky-pds` (Blacksky) is a real **Rust + (per-actor) SQLite +
S3-blobs** PDS (Phase 0 D1: the repo README's "Postgres" is stale vs `main`'s `rusqlite`) — ECOSYSTEM §5e
already tags it "closest to Croft's stack, build-on," and
`research/atproto-private-data-architecture.md` already recommends this path. We extrapolate from a real
implementation rather than guess the atproto surface (global rule: never guess external API shapes).

**Why "meter the boundary."** The whole economic model (`crystallized/principles.md` Tier 3;
`thinking/cooperative-social-union-model.md`) charges a boundary-observable unit (postage = bytes
transferred, rent = byte-days from the member's own signed manifest). So the **S3-compatible interface is
also the metering boundary** — receipts are signed on each byte transfer, rent is derived from the signed
manifest. This is why the S3 interface and the ledger are not two features but one seam.

**The two-layer split this implies (user framing, 2026-07-31).** Metering is **not** a property of the
storage backend — the backend never does metering at all, by design. The service is two layers:
- **Layer 1 — the shim + dumb backend:** the S3-compatible + atproto PDS interface over a **pluggable**
  backend (FS first, then Garage/SeaweedFS/R2). Plain bytes in/out; content-addressed by CID computed at
  the boundary. The backend is *deliberately dumb* — that is the feature (any S3-compatible store works;
  nothing in the storage layer must be trusted).
- **Layer 2 — metering / crypto provenance (the E0–E9 ledger):** bilateral **signed receipts** on each
  transfer (postage) + the customer's **own signed manifest** (rent) + statements/audit/seal — all at the
  **boundary**. The provenance comes from the **two parties' keys + the customer's manifest**, never from
  the backend.

This is precisely why "meter the boundary, not the machine" holds: the *machine* (backend) needs no
provenance. Phases 1–6 build Layer 2; Phase 7 wires it at the S3 boundary over Layer 1 (`blobstore.rs`
trait); the atproto PDS layer (Phase 8) is a thin surface over the same boundary.

**Architecture layers — the consolidated picture (user framing, 2026-07-31).** The design separates into
**five orthogonal axes**; conflating them is the trap. They compose freely. (**E-number namespaces:**
E0–E14 = item-storage experiments; E82–E86 = `ROADMAP_TODO` backlog; relay-lab E8/E9 = the blind-mirror
confidentiality spike — three distinct namespaces.)

```
 (1) INTERFACE      S3-compatible · atproto PDS blob API   (· full atproto repo API — records, later)
     front door
 (2) INDEX /        flat (DID,CID)  ·  MST (NSID collection/rkey, a flat keyspace as a tree)
     ADDRESSING       ·  RBSR (range set-reconciliation over peer nodes)
     — pick per surface/consumer; each has its own needs:
       flat = lookup + dedup (v0 blobs) · MST = bounded O(log n) add/remove + inclusion/sync proofs
       (only if we host records; spec-required there) · RBSR = cheap peer set-reconciliation (Phase 10)
        ───────────── boundary metering / provenance (Layer 2) ─────────────
 (3) PHYSICAL       local disk · proxied object store (S3/Garage/R2) · PDS-style objects-alongside-records
     BACKEND          — the dumb Layer-1 `BlobStore` trait; ORTHOGONAL to (2): any index on any backend
 (4) CONFIDENTIALITY  plaintext ·············· encrypted (keys held by the group; blind vs delegate tier)
     (who can DECRYPT)  — encryption governs plaintext; the server never holds the confidentiality trust
                          anchor. blind = ciphertext-only; delegate = keys delegated so the server/reader
                          can read (the meer confidentiality tiers, relay-lab E8/E9, unmeasured — D4).
 (5) ACCESS         public-read (default) ·············· gated per object
     (who can FETCH)   — object gating is SERVER-ENFORCEABLE even when blind (hand over bytes or not
                         without knowing contents); billing ⟂ access; public-relay replication
                         (`subscribeRepos` firehose) is opt-in, out of v0. Writes: owner DID + delegated caps.
```

Key invariants: **metering never needs plaintext** (blind hosting bills correctly, natively); the backend
is dumb (index ⟂ backend); **access (5) and confidentiality (4) are independent** — they compose four ways
(public/gated × plaintext/encrypted; gated+plaintext = the delegate tier); and (4) touches (2) in one
place — the CID is over the *stored* bytes (ciphertext when encrypted), so cross-user dedup needs
**convergent encryption** (which leaks plaintext-equality) and blind hosting therefore generally forgoes
cross-user dedup. Blind + plaintext search don't coexist without a delegate (or searchable-encryption) —
a Phase-10 tier decision, not v0. v0 realises: interface {S3, atproto-blob}, index {flat}, backend
{FS→pluggable}, confidentiality {payload-agnostic — we meter ciphertext or plaintext identically},
access {public-read default, gateable per object}. MST/RBSR/records + blind-vs-delegate tiers are tracked
(E85, D4, `HS OC-2` — the CID-vs-envelope-hash reconciliation), not v0.

**Access model — two independent dimensions: access + confidentiality (user framing, 2026-07-31,
corrected).** The PDS/unix-`774` default is **public-read, authorized-write** — writes authorized to the
owner DID + **delegated capabilities** (OAuth scopes / capability grants — "some can write"; ties E83 +
the social-trust layer); metering orthogonal (billing ≠ access control). But **access and confidentiality
are two separate axes**, not one:
- **Access (object gating): server-enforceable, even when blind.** The store authenticates the requester
  and decides whether to hand over the *object* — without knowing its contents. So a read-ACL on the
  *bytes* works fine; "public-read" is a **default**, not a limitation, and can be gated per object.
- **Confidentiality (plaintext): encryption, orthogonal.** Who can *decrypt* is governed by keys the group
  holds, never the server. The server gates *access to ciphertext* but cannot (and need not) control
  decryption — the local-first/atproto stance (server is not the confidentiality trust anchor).

They compose four ways: public+plaintext, public+encrypted (anyone fetches ciphertext, only key-holders
read), gated+encrypted (defense in depth), gated+plaintext (the delegate tier — server/delegate can read).
**Public replication (the PDS→relay firehose, `subscribeRepos`) is an opt-in feature, not a requirement:**
objects you replicate to a public relay become publicly fetchable (you chose to broadcast them); objects
you don't stay access-gated by the store. Relay replication is later (full-repo sync scope, out of v0) and
a toggle. All of this lands at the **interface/auth layer (Phase 7/8)** on the Phase 8 `uploadBlob` auth
`SEAM:`, not in the E0–E9 core; tracked in Open Questions.

**Where Layer 2's records live (user framing, 2026-07-31).** The metering records are themselves
**signed, append-only, per-DID records of the same shape as user content** — so they are stored the same
way and **co-located with the user's repo** (the official PDS's **per-user SQLite**), not in a separate
engine (this supersedes an earlier redb lean). One provenance + management machinery (signing, CAR
export, per-user isolation) serves both content and metering. Subtlety for Phase 0/3–4: the customer's
**manifest is single-author** and drops straight into the repo, but **receipts/statements are bilateral
(co-signed by both parties)** — not standard single-author atproto records — so they live as a co-signed
structure *alongside* (same SQLite, distinct shape). **Rollup + purge boundary:** the balance-forward
close (E4) *is* a rollup — once a period is co-signed, its granular receipts are **purgeable past the
boundary** because the signed statement chain carries the provenance (cumulative state = the signed
rollup, not the raw receipts). This bounds the store's growth (rhymes with the corpus's verifiable-roll-up
/ checkpoint-compaction thinking).

**Why the boundary shape is a Phase 0 unknown, not a guess.** "Custom PDS-like with an S3-compatible
interface" leaves genuinely open whether the network boundary is (a) an S3 put/get API, (b) the atproto
PDS API (`com.atproto.sync.*`, `getBlob`/`uploadBlob`), or (c) both (S3 as the storage/metering plane
behind the atproto API the network speaks). This determines the scope of the boundary phases, so it is
resolved firsthand in Phase 0 against rsky-pds + the atproto spec before those phases are sized.

**Why croft-stack.** The VPS estate is already declarative (OpenTofu + Ansible + `render.py`), governed
(limits/telemetry on every unit), hardened (a reusable least-privilege baseline), and isolatable (netns).
A new service is a `services/<name>.toml` manifest + role under the "optional-accelerator, serverless
floor" ethos — which fits a store that must stay optional/self-hostable.

**Alternatives considered / rejected.** (1) *Fork rsky-pds directly* — rejected for v0: it carries full
atproto PDS scope + Postgres we may not need, and the metering is the novel part we want to own; we
*learn from* it, not fork it. (2) *Extend the official TS PDS with a metering plugin* — rejected: wrong
language, and metering-at-the-boundary wants to own the byte path. (3) *Skip the atproto surface, ship
pure S3* — kept as a fallback (v0 could be S3-only), but Phase 0 decides whether the PDS surface is in v0.

## Verified Assumptions

- **The port target is complete and proven.** E0–E9 exist as discrete TS modules
  (`experiments/item-storage-protocol/src/{crypto,items,manifest,receipts,statements,audit,seal}.ts` +
  `experiments/e{0..9}-*.ts` + `test/e{0..9}.test.ts`); the standalone (E0–E11) ran **81/81 green** this
  session (`node src/run.ts`). The assertions are the port spec.
- **rsky-pds is Rust + (per-actor) SQLite + S3 blobs** (ECOSYSTEM §5e; Blacksky/Rudy Fraser).
  **[Superseded by Phase 0 D1]** the README's "Postgres" is stale — `main`'s `Cargo.toml` uses `rusqlite`;
  see the D1 RESOLVED bullet below for the verified crate structure.
- **croft-stack service model** (croft-stack `00-model-and-manifests.md`, `service-hardening-plan.md`,
  `netns-isolation-plan.md`, `telemetry-client-plan.md`): a service = a `services/<name>.toml` manifest
  consumed by `render.py` → Ansible converge; hardening baseline + netns isolation + telemetry apply;
  "optional-accelerator, serverless floor, governed by default."
- **Blob backend:** MinIO community edition was **archived Feb 2026** → use **Garage or SeaweedFS**
  (self-host) or R2/B2 (ECOSYSTEM §5e).
- **atproto decouples identity from host** (CAR repo export/import; ECOSYSTEM §5e) — a store can front a
  repo without owning the identity.
- **Node v25 runs the TS suites** (confirmed this session) — available as the reference oracle during the
  port.
- **[Phase 0 RESOLVED, 2026-07-31 — D1] rsky-pds structure.** Rust workspace; `rsky-pds` uses **Rocket
  0.5.1** (not axum/actix), **`aws-sdk-s3` 1.29** + `aws-config` for the S3 blob backend, **`rusqlite`
  (SQLite, per-actor)** — the repo README's "Postgres + S3" is **stale** vs `main`'s Cargo.toml — and
  **`atrium-api` 0.24.6 + `rsky-lexicon`** for the atproto surface. Blob layer = `pub trait BlobStore`
  (`actor_store/blobstore.rs`) with `put_temp`/`make_permanent`/`put_permanent`/`get_bytes`/`get_stream`/
  `quarantine`/`delete*`; S3 key layout `tmp/{did}/{key}` → `blocks/{did}/{cid}`; handlers at
  `apis/com/atproto/repo/upload_blob.rs` (POST, 100 MiB cap) + `apis/com/atproto/sync/get_blob.rs` (GET,
  stream). **Directly reusable prior art for our Layer-1 `BlobStore` trait + the atproto→backend mapping.**
- **[Phase 0 RESOLVED, 2026-07-31 — D2] atproto blob API shapes** (sourced from canonical lexicon JSON,
  not the SPA docs). `com.atproto.repo.uploadBlob` = **POST** `/xrpc/com.atproto.repo.uploadBlob`, body =
  raw bytes (`encoding "*/*"`), **auth REQUIRED**, response
  `{"blob":{"$type":"blob","ref":{"$link":"<CID>"},"mimeType":"<ct>","size":<int>}}` (CIDv1; legacy
  `{cid,mimeType}` is read-only). `com.atproto.sync.getBlob` = **GET** `/xrpc/...?did=<did>&cid=<cid>`,
  **public (no auth)**, returns raw bytes. `com.atproto.sync.listBlobs` = **GET**, public,
  `{"cids":["<cid>",...],"cursor":"..."}`. **Minimal blob-hosting floor = uploadBlob + getBlob +
  listBlobs**; everything else (getRepo/getRecord/getBlocks/subscribeRepos/…) is full-repo-PDS scope,
  out of v0. (getBlob's `Content-Type` echo is behavioral — UNCONFIRMED in the lexicon.)
- **[Phase 0 RESOLVED, 2026-07-31 — D6] official PDS S3 support + interface-vs-backend.** `@atproto/pds`
  supports an **S3 blob backend** via `PDS_BLOBSTORE_S3_{BUCKET,REGION,ENDPOINT,FORCE_PATH_STYLE,
  ACCESS_KEY_ID,SECRET_ACCESS_KEY,…}` **or** disk via `PDS_BLOBSTORE_DISK_LOCATION` (discriminated union,
  exactly one; `bluesky-social/pds` defaults to disk `/pds/blocks`). Storage = **per-actor SQLite** (one
  `.sqlite` per DID + a PDS-wide SQLite; SQLite-only, local FS). **Load-bearing finding:** both PDSes use
  S3 only as an **internal backend the server writes to (case a)** — *neither exposes an S3-compatible API
  to clients (case b)*. Our client-facing S3 metering boundary is **case b, which has no PDS prior art** —
  the novel part we own, built from the S3 API spec, not forked. (The "S3" appears twice in our stack:
  case-b exposed front door = the metering plane; case-a optional internal backend = the dumb Layer-1
  store, Garage/R2.)
- **[Phase 0 RESOLVED, 2026-07-31 — D1-internal] internal Rust prior art.** `hist-atproto-spike` +
  `lexicon-community` give a **proven CIDv1 (`raw` 0x55 + sha-256) + DAG-CBOR path** (`serde_ipld_dagcbor`
  + `ipld-core` + `sha2`) that is **byte/CID-identical to real PDS records**, a live-PDS XRPC client
  pattern (`reqwest`; uploadBlob/create/delete lifecycle, env-gated), and hand-written draft lexicons (no
  atproto lexicon crate). Spike crypto is **blake3/sha2** (hist) + **ECDSA k256/p256/p384** (lexicon) —
  **no internal Ed25519 precedent**, so the **TS item-storage oracle stays the Ed25519 parity target**
  (Phase 1 `ed25519-dalek`), while the spikes' DAG-CBOR/CIDv1 path is **reusable to close Phase 2's
  `item.rs` CID `SEAM:`**.
- **[Phase 0 RESOLVED, 2026-07-31 — D5] storage layout.** Official-PDS **ActorStore = per-user SQLite
  (WAL) + a PDS-wide SQLite**; blobs are **per-DID (`(DID,CID)` tuples**, atproto disc. #1756 / Newbold).
  So metering records co-locate in the per-user SQLite (manifest = single-author repo record;
  receipts/statements = co-signed structure alongside); blob bytes in the pluggable Layer-1 backend keyed
  `(DID,CID)`; **FS first** (matches the PDS disk default), **Garage/SeaweedFS/R2 later** (matches rsky's
  `aws-sdk-s3` pattern; not MinIO — archived).
- **[Phase 0 RESOLVED, 2026-07-31 — D3, synthesized from D1/D2/D6] boundary shape.** BOTH, as
  user-confirmed: **(b) an S3-compatible client interface = the storage+metering plane** (Phase 7; novel,
  no PDS prior art) **+ the atproto PDS blob API as a thin layer over the same metered byte-path** (Phase
  8; floor = uploadBlob/getBlob/listBlobs, mapping borrowed from rsky's handlers→`BlobStore`). HTTP crate:
  rsky uses Rocket; **axum remains a fine independent choice** for our custom interface (learn from rsky,
  don't fork) — final pick at Phase 7.
- **[Phase 0 RESOLVED, 2026-07-31 — D7] CORRECTION: the telemetry poller reads cgroup v2, not tracing.**
  The croft-stack telemetry poller does **NOT** scrape Prometheus/journald/stdout/`tracing` (Prometheus/
  cAdvisor are explicit non-goals). It reads the **cgroup v2 filesystem** per unit
  (`/sys/fs/cgroup/system.slice/<unit>.service/`): `memory.current`/`memory.peak` (bytes), `pids.current`,
  `cpu.stat` (`usage_usec …`), `io.stat`. **A new Rust service emits no app-level metrics for the poller**
  — its systemd unit only needs `MemoryAccounting=CPUAccounting=IOAccounting=TasksAccounting=yes` + limits
  (`MemoryHigh` soft + generous `MemoryMax`, `CPUQuota`, `TasksMax`, `IOWeight`). This **corrects the
  Pass-3 premise** that Phase 7 `tracing` must match a "poller contract": app-level `tracing`→journald is
  for **our** debugging (`journalctl`), independent of the poller. Hardening: a Rust service (no JIT) can
  take the full `MemoryDenyWriteExecute` set (like caddy/broker); target `systemd-analyze security` ≈
  1.2–1.5; never add a cgroup namespace (breaks cross-unit cgroup reads).
- **[Phase 0, 2026-07-31 — D4] "one store, two consumers" holds by design for v0; one tracked item.**
  Blob hosting is the real v0 consumer; the content-blind convergence/meer consumer is **gated to Phase
  10** (drystone fold/MLS not real yet; relay-lab E8/E9 not started). **Tracked (Phase 10, not a v0
  blocker):** the two consumers address differently — blob hosting by **CID** (CIDv1 raw+sha-256), the
  convergence node by **envelope hash** (blake3 in-house digest); unifying them is the still-open
  `HS OC-2` (hist-spike `record.rs:8-13`). v0 keeps addressing **pluggable** so Phase 10 reconciles
  without a rewrite.
- **[Pass 2 verified] The port oracle is the STANDALONE** (`item-storage-protocol-standalone/`, the
  dependency-free E0–E11 build that ran 81/81 this session), not the full `item-storage-protocol/`. Its
  module names differ from what the per-phase Read-sets below still cite: standalone uses **`item.ts`,
  `receipt.ts`, `statement.ts`, `clock.ts` (not `time.ts`), `rng.ts` (not `prng.ts`), `pricing.ts`** (the
  dial cost), plus `crypto/manifest/ledger/audit/seal/canonical.ts`, **`actor.ts`** (the `deriveId`
  source that `identity.rs` ports), and experiments `e0_identity.ts …
  e11_financing.ts`. Execution reads the standalone; treat the full-version filenames in the Read-sets as
  the standalone equivalents.
- **[Pass 2 verified] No experiments-wide Rust workspace** — existing crates (`ap-ambassador`,
  `attest-family`, `hist-atproto-spike`, `lexicon-community`, …) are each **standalone** (own `Cargo.toml`).
  So `item-store` is its **own standalone crate**, not a workspace member; `cargo test` runs in the crate
  dir. (Phase 1's "workspace member" language corrected below.)
- **[Pass 2 verified] Internal Rust prior art exists** — `experiments/hist-atproto-spike/` and
  `experiments/lexicon-community/` are Rust atproto/history spikes in-corpus; Phase 0 reads these first
  (may reduce external fetching and directly inform the atproto-in-Rust + history-convergence surface).
- **[Pass 2 verified]** `item-storage-protocol/README.md` (8.0K), `ECOSYSTEM.md §5c-3`,
  `experiments/appview-infra/GROUPS.md` (27.8K) all exist — the Documentation-Impact / D4 references resolve.

## Documentation Impact

- `plans/2026-07-31-coop-storage-metered-hosting-lane.md` — add a pointer to this build plan; update the
  "Next step" as phases land. (Phase 0 opens it; each phase updates status.)
- `ROADMAP_TODO.md` **E82** — status transitions as phases complete. (Every phase.)
- `ECOSYSTEM.md` §5c-3 (Croft-owned live properties) — add the deployed service row when it goes live on
  the VPS. (Phase 9.)
- `plans/croft-stack/10-drystone-layer.md` — the convergence-server section already points here; update
  when the meer-mode consumer is real. (Phase 10, later.)
- `plans/croft-stack/README.md` + the `services/*` set — register the new service. (Phase 9.)
- `experiments/item-storage-protocol/README.md` — cross-reference the Rust port as the productionization.
  (Phase 1.)
- `COHESION.md` §65 — status note when v0 runs. (Phase 9.)
- New: the service's own `README.md` in its crate/dir. (Phase 1.)
- Repo/IP home is an Open Question (below); if it graduates to its own `CroftCommunity/<repo>`, that repo
  gets its own docs (deferred, gated on the naming + IP decision).

## Concurrency Map

Sequential spine: **Phase 0 → 1 → 2 → 3 → 4 → 5 → 6 → 7 → 8 → 9** (→ 10 later).

**All implementation phases sequential.** Reason: Phases 1–6 each add ledger modules that later phases
import and build on (receipts need items+manifest; statements need receipts; seal/grace need statements);
Phases 7–8 add the HTTP/PDS surface over that core; Phase 9 deploys the built binary. Each phase reads
what the prior wrote. Phase 0 is discovery (throwaway probes). No parallel set is justified — the
dependency chain is linear and the write-sets would overlap on the shared crate (`lib.rs`, the server
module). If, after Phase 0, the S3 interface (Phase 7) and the atproto PDS surface (Phase 8) prove to
have fully disjoint modules, revisit parallelizing {7,8} in Pass 2 — but only with disjoint write-sets +
a re-entry check; default remains sequential.

**[Pass 2] Phase 0 overlaps Phases 1–3 (the one real concurrency).** Phase 0 is discovery — it produces
notes, **no code write-set** — and Phases 1–3 port pure ledger logic (crypto / items / manifest /
receipts) that does **not** depend on the atproto/rsky/storage findings. So Phase 0 need not block the
port start; they are disjoint (notes vs the crate). Phase 0's findings must land before **Phase 4's
persistence** (needs D5's per-user-SQLite layout) and **Phases 7–8** (need D1/D2/D3/D6/**D7**). **{7,8} audited:
NOT parallel** — Phase 8 depends on Phase 7's server and shares `server.rs` / `Cargo.toml` (overlapping
write-set). The code spine (1→…→9) stays sequential.

**[Pass 3] Isolation invariant if Phase 0 and Phases 1–3 are run by separate agents.** Invariant: the
Phase 0 agent writes **only discovery notes** (into this plan doc / a scratch notes path), **never** under
`experiments/item-store/`, and runs **no `git` mutation** in the crate worktree. Re-entry check: `git
status` in the crate shows only the Phase 1–3 agent's files; main-repo HEAD == the pre-dispatch SHA; no
orphaned fetch process. Default execution remains **sequential** — Phase 0 is a user-reviewed checkpoint,
so the overlap is a permitted optimization, not the planned path.

## Phases

### Phase 0: Discovery (network-enabled; Discovery Exemption applies)

**Goal:** Resolve the external unknowns firsthand so the boundary phases are sized on evidence, not
inference. Network is open — fetch real sources.

- [x] **D1: PDS-in-Rust prior art — internal first, then rsky-pds.** **Probe:** (a) read the in-corpus
  Rust spikes **`experiments/hist-atproto-spike/`** and **`experiments/lexicon-community/`** (atproto /
  history in Rust — internal prior art we already own); then (b) WebFetch `github.com/blacksky-algorithms/rsky`
  (rsky-pds) — how it structures repo storage, blob storage, the S3 backend, and the HTTP/PDS API layer;
  note crate deps (axum/actix? sqlx? aws-sdk-s3/rust-s3? atproto lexicon crate). **Success:** a written
  list of the crates for (HTTP server, S3, DB, atproto lexicon) + how blob put/get maps to S3, and what
  the internal spikes already give us. **Disposition:** throwaway (notes only).
- [x] **D2: atproto PDS network API surface.** **Probe:** WebFetch the atproto lexicon/spec for
  `com.atproto.sync.*`, `com.atproto.repo.uploadBlob`, `com.atproto.sync.getBlob`, and the "what a PDS
  serves" docs (atproto.com). **Success:** the minimal endpoint set a PDS-like store must serve, and the
  blob upload/get request/response shapes (recorded as confirmed, not "likely"). **Disposition:** throwaway.
- [x] **D3: resolve the boundary shape.** From D1+D2: is the network boundary S3 put/get, the atproto PDS
  API, or both? **Success:** a decision with rationale (recommend: S3 as the storage+metering plane;
  atproto PDS API as the network-facing layer if in v0 scope). **Disposition:** throwaway → feeds Phases
  7/8 sizing + the Open Question.
- [x] **D4: history-convergence store requirement.** **Probe:** read `experiments/appview-infra/GROUPS.md`
  (the convergence node), the relay-lab E8/E9 briefs, and `plans/croft-stack/10-drystone-layer.md`
  locally. **Success:** the concrete requirement the content-blind meer places on the store (append-only
  envelope sets, content-blind, addressed how). **Disposition:** throwaway → confirms the "one store, two
  consumers" claim or flags a mismatch.
- [x] **D5: storage layout (confirm the Q5 resolution) + blob backend.** Confirm the official-PDS
  **per-user SQLite** layout (from D1/D6) and how to co-locate the metering records there — the manifest
  as a single-author repo record, receipts/statements as a co-signed structure alongside — and that the
  **rollup/purge** boundary fits. Pick the local blob backend (Garage/SeaweedFS; FS first). **Success:**
  a confirmed per-user-SQLite storage layout + backend aligned to official-PDS/rsky-pds + croft-stack.
  **Disposition:** throwaway.
- [x] **D6: official-PDS S3 support + the interface-vs-backend distinction** (from the user's question).
  **Probe:** WebFetch the official `bluesky-social/pds` repo/docs — confirm whether/how it supports an
  **S3 blob backend** (env-var config) vs the corpus's "SQLite + local-FS" note, and separate two
  surfaces: (a) **S3 as a blob backend the PDS writes bytes to** (internal), (b) **an S3-compatible
  interface exposed to clients** (our metering boundary, v0). **Success:** a confirmed statement of what
  the official PDS does for blobs + how our S3-compatible client interface differs from a backend S3
  store. **Disposition:** throwaway → feeds Phase 7 (which "S3" is the boundary).
- [x] **D7: telemetry/observability contract + the governed envelope (local docs; Pass-3 addition).**
  **Probe:** read `plans/croft-stack/telemetry-client-plan.md` and `plans/croft-stack/service-hardening-plan.md`
  locally — confirm (a) the metrics/log **format the telemetry poller scrapes** from a unit (so Phase 7's
  `tracing` output is *consumable*, not invented) and (b) the resource **envelope** (limits) a new unit must
  stay within. **Success:** a written statement of the poller's expected metric/log shape + the envelope
  ceilings the metered store must respect. **Disposition:** throwaway → feeds **Phase 7 instrumentation
  design** (so we don't emit tracing the poller can't read) + **Phase 9 telemetry wiring**. Not blocking
  (internal docs, not an external API), but must land before Phase 7's observability is built.
  **[D7 FINDING OVERTURNED THIS PREMISE — see the D7 RESOLVED bullet + Phase 0 COMPLETE:** the poller reads
  **cgroup v2 files**, it does NOT scrape `tracing`/logs/Prometheus; a Rust service emits no app-level
  metrics for it. The real "contract" is systemd accounting (`*Accounting=yes` + limits), not a log format;
  app `tracing` is our own journald debugging trail, independent of the poller.**]**

**Done when:** all BLOCKING open questions below are resolved, Verified Assumptions updated with
firsthand evidence, and Phases 7/8 re-sized if D3 changes their scope (record in Review Log).

**COMPLETE (2026-07-31).** All seven tasks resolved firsthand (three parallel researchers: local corpus
reads + live WebFetch of the atproto lexicons / rsky-pds / official PDS). Findings folded into Verified
Assumptions above ([Phase 0 RESOLVED …] bullets) and the affected phase specs (7/8/9 + 1/2/10). One
material correction (D7: the telemetry poller reads cgroup v2, not `tracing`) and two scope additions
(D2: `listBlobs` in the Phase-8 floor + an auth `SEAM:`; D4: keep v0 addressing pluggable for the
Phase-10 CID-vs-envelope-hash reconciliation). No BLOCKING open question was reopened; no v0 blocker
surfaced. See the Review Log Phase-0 entry.

---

Implementation phases (1–9) all follow the same shape: **port the proven module TDD (port its assertions
as the RED tests first), rust-enforcer discipline (no `unwrap()` in prod, `Result`/`thiserror`, doc
comments, `clippy::pedantic`), commit at green.** Crate/dir placeholder: `item-store`, a **standalone
crate** (its own `Cargo.toml`; no experiments-wide Rust workspace) under `experiments/` for dev (see
Open Question on repo/IP home). **Port oracle = the dependency-free `item-storage-protocol-standalone/`**
(the 81/81 build); cross-check the full `item-storage-protocol/` for E4/E7–E9 detail if needed. **Preserve
the SPEC's `SEAM:` grep discipline** — every place a mock stands in for real infra gets a `SEAM:` comment
so production gaps stay enumerable by grep. **Test invocation (Pass 3):** because `item-store` is a
**standalone crate** (not a workspace member), run `cargo test` **from `experiments/item-store/`** — `-p
item-store` is a workspace-package selector and does **not** apply to a standalone crate; filter by name
(`cargo test e0`) or target a specific integration binary (`cargo test --test e0_identity`). The
per-phase Verification commands below use this form.

### Phase 1: Crate skeleton + crypto/identity (port E0) — SHIPPED (`8389a0e`)
**Goal:** A Rust crate that generates keypairs, derives stable ids, signs/verifies — E0's "we recognize
you the same way we count you."
**Delivered (2026-07-31):** `item-store` standalone crate (`experiments/item-store/`) with `crypto.rs`
(deterministic Ed25519 derive from a `sha256("{master}::keyseed::{label}")` seed matching the TS oracle;
Zeroize on the secret seed; `sign_message`/`verify_message`; `public_key_from_hex` with a `thiserror`
`CryptoError`) and `identity.rs` (`derive_id` = `id:` + `sha256(pubkey)[..16]`; a pinned `Identity`).
Wiring test `tests/e0_identity.rs` ports E0's 4 assertions + a tamper edge, RED→GREEN. 9 tests pass;
`clippy::pedantic -D warnings` + `cargo fmt` clean. Cross-ref added to `item-storage-protocol/README.md`.
**Changes:**
- [ ] `Cargo.toml` + `src/lib.rs` (crate scaffold; **standalone crate**, own `Cargo.toml` — no
  experiments-wide Rust workspace, Pass-2 verified).
- [ ] `src/crypto.rs` — Ed25519 sign/verify + SHA-256 fingerprint, newtype-wrapped keys (Zeroize on
  secret material per rust-enforcer).
- [ ] `src/identity.rs` — deterministic id derivation from pubkey; pin/verify peer keys.
- [ ] `README.md` — the crate's purpose + the cross-ref to `item-storage-protocol`.
- [ ] **[Pass 3, Doc-Impact] Update `experiments/item-storage-protocol/README.md`** — cross-reference the
  Rust port as the productionization. This is the scheduled Documentation-Impact edit landing in the phase
  that first makes it stale (Phase 1 creates the port it must point to), not a trailing docs phase.
**Call chain:** `tests/e0` → `identity::derive`/`crypto::{sign,verify}`.
**Wiring test:** `tests/e0_identity.rs` — a message signed by A verifies under A's pinned key and fails
under B's; id derivation deterministic (ports E0's 4 assertions). RED → GREEN.
**Depends on:** Phase 0 (D5 storage choice informs nothing here; independent).
**Read-set:** `experiments/item-storage-protocol/src/{crypto,actor}.ts`, `.../exp/e0_identity.ts` (reference;
`actor.ts` = the `deriveId` source `identity.rs` ports).
**Write-set:** `experiments/item-store/{Cargo.toml,src/lib.rs,src/crypto.rs,src/identity.rs,README.md,tests/e0_identity.rs}`
+ `experiments/item-storage-protocol/README.md` (the cross-ref edit above).
**Shared-state contract:** no shared mutable state beyond the file write-set; standalone crate, so no
workspace `Cargo.toml` to touch (Pass-2 verified: no experiments-wide Rust workspace).
**Risks:** Rust Ed25519 crate choice (`ed25519-dalek`) vs the TS lib — confirm the same curve/encoding so
signatures are comparable to the oracle. **[Phase 0 D1-internal]** the in-corpus spikes use blake3/ECDSA,
**not** Ed25519 — there is **no internal Ed25519 precedent**, so the **TS item-storage oracle is the parity
target** (not the spikes); match its curve/encoding.
**Observability:** Library crate — the primary observable is the **typed error surface** (`thiserror`;
fail-loud on verify/derive failure, no silent fallback per the global rule). Add `tracing` at the crate
root; emit a WARN event on a signature/verify failure carrying the key **fingerprint** (never the secret —
Zeroize-wrapped keys must not `Debug`-print or serialize). No logging in the pure hash/derive path.
**Done when:** (1) *Behavioral:* the crate signs/verifies and derives ids matching the TS E0 behavior;
(2) *Verification:* `cargo test e0` green (from `experiments/item-store/`; standalone crate — no `-p`).
**Validation:** Narrow — wiring + unit tests sufficient.

### Phase 2: Content-addressed items + signed manifest (port E1–E2) — SHIPPED (`9476b97`)
**Goal:** Items named by fingerprint (tamper-evident) + a customer-signed manifest (the bill's source of
truth). E1–E2.
**Delivered (2026-07-31):** `item.rs` — `Item` (cid = SHA-256 fingerprint, computed on construction;
`SEAM:` for CIDv1/DAG-CBOR) + `ContentStore` (dumb key→bytes Layer-1 backend; `retrieve_verified`
re-fingerprints → `RetrieveError::{Missing,Tampered}` naming the failing item; content-addressed dedup).
`manifest.rs` — canonical sorted-leaf Merkle root (`leaf:cid:size` / `node:l:r`, dup-last padding, empty
sentinel) matching the oracle; `build_manifest` signs the root; `Manifest::verify` checks root-recompute +
signature; `expected_bytes`. Wiring test `tests/e2_manifest.rs` ports E2 (provider root == customer root,
order-independent; inflated-total + missing-leaf + impostor-key adversarials); `tests/e1_items.rs` ports
E1. 19 tests; `clippy::pedantic` + `fmt` clean.
**Changes:**
- [ ] `src/item.rs` — content-addressed object (fingerprint = SHA-256; `SEAM:` note for CIDv1/DAG-CBOR).
  **[Phase 0 D1-internal]** the CIDv1/DAG-CBOR `SEAM:` can be closed with the **proven in-corpus path**
  (`serde_ipld_dagcbor` + `ipld-core` + `sha2`, CIDv1 `raw` 0x55 + sha-256) from `hist-atproto-spike` /
  `lexicon-community` — byte/CID-identical to real PDS records; reuse it rather than re-deriving.
- [ ] `src/manifest.rs` — sorted `(fingerprint,size)` list, Merkle root, signature over the root; expected
  bytes-at-rest as a pure function of the manifest.
- [ ] `tests/e1_items.rs`, `tests/e2_manifest.rs` — port the E1/E2 assertions incl. adversarial (byte-flip
  detected for that item only; larger-total claim rejected by arithmetic; root-mismatch on missing item).
**Call chain:** `tests/e2` → `manifest::{root,verify,expected_bytes}` → `item::fingerprint`.
**Wiring test:** `tests/e2_manifest.rs` — provider-computed root == customer-signed root; both adversarial
claims detected. RED → GREEN.
**Depends on:** Phase 1.
**Read-set:** `.../src/{items,manifest}.ts`, `.../e{1,2}-*.ts`.
**Write-set:** `experiments/item-store/src/{item.rs,manifest.rs}`, `.../tests/e{1,2}_*.rs`.
**Shared-state contract:** none beyond write-set.
**Risks:** Merkle domain-separation must match the TS root construction to stay oracle-comparable.
**Observability:** Typed errors (`thiserror`) for tamper / root-mismatch — fail-loud, never a silent
accept. `tracing` WARN on a manifest-root mismatch or a byte-flip detection, carrying the offending
fingerprint + expected-vs-got root; DEBUG on manifest build (item count, total bytes).
**Done when:** (1) items round-trip + tamper detected; manifest root + expected-bytes correct; (2)
`cargo test e1 e2` green.
**Validation:** Narrow.

### Phase 3: Transfer receipts — postage metering (port E3) — SHIPPED (`b31be48`)
**Delivered (2026-07-31):** `canonical.rs` (deterministic sorted-key/no-whitespace serialization, `SEAM:`
for DAG-CBOR) + `ledger.rs` (append-only hash-linked signed ledger + `verify_entries` chain/signature
re-verification) + `receipts.rs` (two-mode receipt: `Bilateral` co-signed | `Unilateral` provider-signed
our-side measurement + walkaway + `from_parts` reconstruction; mode-selection `SEAM:`). Wiring test
`e3_receipts.rs`: receipts flow into both ledgers, the chain re-verifies end-to-end, totals reconcile;
forged-byte-count / walkaway-bounded / unilateral-not-co-attested adversarials. +`serde`/`serde_json`.
30 tests; clippy pedantic + fmt clean. **Write-set expanded:** added `canonical.rs` (prerequisite for
hashing/signing) + `Cargo.toml` serde deps.
**Goal:** Transfer receipts per increment in **two modes** — **unilateral** (provider-signed "our-side
measurement," valid by the trust relationship) and **bilateral** (both parties co-sign — the co-attested,
third-party-verifiable form). The **social-trust layer selects the mode** per transfer (size / sensitivity
/ trust distance); both are handled the same way on our end. Ports E3 (bilateral) + adds the unilateral
mode. "Meter the boundary, not the machine."
**Changes:**
- [ ] `src/ledger.rs` — append-only signed-entry ledger, the substrate under receipts/statements.
- [ ] `src/receipts.rs` — per-increment record (direction, fingerprint, byte range, running total, ts)
  with a **mode**: `Unilateral` (provider-signed only) | `Bilateral` (countersigned). Both append to the
  ledger identically; bilateral reconciles across both parties' copies.
- [ ] a **mode-selection seam** (`SEAM:` social-trust policy hook) — the trust layer decides unilateral
  vs bilateral per transfer; default configurable, not hardcoded. Reuses the same trust-distance primitive
  as the forum's subjective consensus.
- [ ] `tests/e3_receipts.rs` — port E3 bilateral (forged count fails signature; walkaway exposure ==
  increment) **+ unilateral** (provider-signed measurement logs and validates as an our-side measurement;
  test asserts it is single-party — *not* third-party-co-attested — so its provenance is weaker, valid by
  trust).
**Forward-compat note:** the deferred funder-diligence layer (E11–E14) requires **co-attested (bilateral)**
records to be verifiable-from-files (the E14 attested-vs-verified line), so records destined for that layer
must use bilateral mode — the two-mode design stays forward-compatible.
**Call chain:** `tests/e3` → `receipts::{ack,countersign,reconcile}` → `ledger::append`.
**Wiring test:** `tests/e3_receipts.rs` — both ledgers reconcile to identical totals; forged entry fails;
walkaway exposure bounded. RED → GREEN.
**Depends on:** Phase 2.
**Read-set:** `.../src/{ledger,receipts}.ts`, `.../e3-*.ts`.
**Write-set:** `experiments/item-store/src/{ledger.rs,receipts.rs,canonical.rs}`, `.../tests/e3_receipts.rs`,
`Cargo.toml` (serde deps). ([Pass-3-shipped] `canonical.rs` added — prerequisite for deterministic hashing/signing.)
**Shared-state contract:** ledger files under the crate's tmp/test dir only.
**Risks:** canonical serialization for signing (must be deterministic) — port `canonical.ts` faithfully.
**Observability:** First half of the **metering byte-path trail** Phase 7 exposes at the HTTP boundary.
`tracing` INFO per receipt (mode `Unilateral`/`Bilateral`, direction, fingerprint, byte range, running
total, ts); WARN on a forged-count / signature failure carrying the failing entry; DEBUG on ledger append
(entry index, running total). The mode-selection `SEAM:` logs which mode the trust policy chose and why.
**Done when:** (1) postage metered by signed receipts, walkaway bounded; (2) `cargo test e3` green.
**Validation:** Narrow.

### Phase 4: Balance-forward statements — the monthly close (port E4) — SHIPPED (4a `18fd199`, 4b `0b0464c`)
**Goal:** Co-signed opening→closing statements chained by hash; rent = byte-day integral; disputes bound
to one period — and the **rollup + purge boundary**: a co-signed period lets its granular receipts be
purged while the signed chain preserves provenance. E4 + purge.
**Delivered (2026-07-31), split into two commits:**
- **4a (`18fd199`) — statement logic:** `clock.rs` (`SimClock` day counter) + `pricing.rs` (`rent_cents`
  = floor(byte-days/10k), `postage_cents` = floor(bytes/1k), integer cents) + `statements.rs` (`RentTimeline`
  bytes-at-rest step-function → byte-day integral; `StatementBody`/`build_statement` hash over canonical
  body; `verify_chain` prev-link/hash-recompute/period-sequence reporting exact `failed_at`;
  `purge_receipts_settled_through`). Wiring test `e4_statements.rs` (3-period chain, balance-forward, rent
  == recomputed integral, tamper-located, fabrication-rejected, purge-preserves-chain).
- **4b (`0b0464c`) — per-user SQLite persistence** (D5 co-location; user chose to build it now, not defer:
  `rusqlite` `:memory:` mode runs the *real* persistence path in tests). `persist.rs` — a per-DID `Store`
  co-locating manifest (single-author) + receipts + statements (co-signed alongside); records stored as
  canonical JSON, `load_*` reconstruct + callers re-verify. Added `Deserialize` to the persisted types;
  `SEAM:` for `Connection` `!Sync` → pooling at Phase 7. Wiring test `wiring_persist.rs` (round-trip +
  per-DID isolation).
- **Write-set expanded** beyond the Pass-2 plan: added `clock.rs`, `pricing.rs`, `persist.rs`,
  `tests/wiring_persist.rs`, `Cargo.toml` (`rusqlite` bundled), `clippy.toml` (`doc-valid-idents`), and
  `Deserialize` derives on `manifest.rs`/`receipts.rs`. 48 tests; clippy pedantic + fmt clean.
**Changes:**
- [ ] `src/statements.rs` — balance-forward commit (opening root, closing root, rent, postage, fees),
  hash-chained; rent recomputable independently.
- [ ] **rollup + purge boundary** — once a period statement is co-signed, that period's granular receipts
  become purgeable (the hash-chained statement carries the provenance); cumulative state = the signed
  rollup, so the store stays bounded.
- [ ] persistence (Q5 resolution): the **manifest** as a single-author repo record and
  **receipts/statements** as the co-signed structure alongside, co-located in the per-user SQLite
  (evolves `ledger.rs` from Phase 3; exact module split refined in Pass 2).
- [ ] `tests/e4_statements.rs` — port E4 incl. adversarial (historical edit located at the exact link;
  fabricated period rejected) + a **purge test** (granular receipts of a co-signed period are dropped,
  yet the chain still verifies genesis→head and rent stays recomputable).
**Call chain:** `tests/e4` → `statements::{close,verify_chain,rent}` → `ledger` + `manifest`.
**Wiring test:** `tests/e4_statements.rs` — chain verifies genesis→head; rent == independent byte-day
integral; any historical edit located. RED → GREEN.
**Depends on:** Phase 3; **Phase 0 D5** (the persistence sub-item needs the confirmed per-user-SQLite layout).
**Read-set:** `.../src/statements.ts`, `.../e4-*.ts`.
**Write-set:** `experiments/item-store/src/statements.rs`, `.../tests/e4_statements.rs`.
**Shared-state contract:** none beyond write-set.
**Risks:** byte-day integration over a period timeline — port the time model (`clock.ts` — standalone name; not `time.ts`) exactly.
**Observability:** `tracing` INFO per statement close (period, opening/closing root, rent, postage, fees);
WARN on a chain-verify failure carrying the exact broken link; INFO on **purge** (period, receipt count
dropped, chain-still-verifies confirmation). The purge log is the audit trail that granular receipts were
dropped legitimately (provenance preserved by the co-signed chain).
**Done when:** (1) monthly statements co-signed + chained, rent correct; (2) `cargo test e4` green.
**Validation:** Narrow.

### Phase 5: Spot-checks + the audit dial (port E5–E6)
**Goal:** Random-sample audit with detection math `1−(1−f)^k`; a member-chosen, cost-priced dial. E5–E6.
**Changes:**
- [ ] `src/audit.rs` — random k-sample retrieve+fingerprint+verify; the detection-math check; public-
  randomness challenge seeding.
- [ ] `src/dial.rs` — audit tiers, cost linear in audit count, chosen tier as a signed declaration.
- [ ] `tests/e5_audit.rs`, `tests/e6_dial.rs` — port incl. the measured-vs-predicted detection table +
  linear-cost + pro-rate-on-change. **[Pass 3, mutation-resistance]** assert the *relationship* across k,
  not a single happy-path point: boundary cases `f=0` (no faults → detection 0 for any k), `k=0` (no
  sample → no detection), `k=1` (detection ≈ f), and full-corpus k (detection → 1). Dial cost: assert no
  discount/step at the tier edges (the boundary sample points), not one interior value.
**Call chain:** `tests/e5/e6` → `audit::sample`/`dial::price` → `manifest` + `ledger`.
**Wiring test:** `tests/e5_audit.rs` — measured detection ≈ `1−(1−f)^k` within tolerance; honest provider
passes; cost scales with k, not corpus size. RED → GREEN.
**Depends on:** Phase 4.
**Read-set:** `.../src/audit.ts`, `.../exp/e5_audits.ts`, `.../exp/e6_dial.ts` (**[Pass 3 spot-check]** the
standalone has **no `dial.ts` src module** — the dial/pricing logic lives in `src/pricing.ts` + the
`exp/e6_dial.ts` experiment; `dial.rs` is a new consolidation, oracle = those assertions).
**Write-set:** `experiments/item-store/src/{audit.rs,dial.rs}`, `.../tests/e{5,6}_*.rs`.
**Shared-state contract:** seeded RNG only (port `rng.ts` for determinism — standalone name; not `prng.ts`).
**Risks:** Monte-Carlo tolerance flakiness — seed the RNG (deterministic, per the SPEC).
**Observability:** `tracing` INFO per spot-check (k, sampled fingerprints, pass/fail, measured detection);
WARN on a failed sample (which fingerprint, expected-vs-got); DEBUG on dial price (tier, cost). **Log the
deterministic RNG seed** so a flaky Monte-Carlo run is reproducible from the log alone.
**Done when:** (1) detection math holds + dial priced at cost; (2) `cargo test e5 e6` green.
**Validation:** Moderate — also eyeball the measured-vs-predicted table.

### Phase 6: Seal + tombstone + grace (port E7–E9)
**Goal:** Cold-storage tiers (sealed=revocable, tombstone=permanent) verified against a pinned root; the
grace ledger (waivers + deceased-member hold net to zero). E7–E9.
**Changes:**
- [ ] `src/seal.rs` — pin root + key-ceremony (mock: destroy write-cred → write fails closed); rotation
  watch classifies customer-signed unseal vs alarm; tombstone destroys rotation too.
- [ ] `src/grace.rs` — grace events as signed ledger entries netting to zero against a grace account.
- [ ] `tests/e7_seal.rs`, `tests/e8_tombstone.rs`, `tests/e9_grace.rs` — port incl. adversarial (post-seal
  write fails; direct mutation caught by audit; all unseal paths fail after tombstone; books still balance).
**Call chain:** `tests/e7-9` → `seal::{pin,ceremony,watch}`/`grace::event` → `audit` + `ledger` + `statements`.
**Wiring test:** `tests/e7_seal.rs` — no write path succeeds post-ceremony; direct mutation caught vs
pinned root; postage over sealed period == audit reads. RED → GREEN.
**Depends on:** Phase 5.
**Read-set:** `.../src/seal.ts`, `.../exp/{e7_seal,e8_tombstone,e9_grace}.ts` (**[Pass 3 spot-check]** no
`grace.ts` src module — grace logic lives in `src/ledger.ts` + `exp/e9_grace.ts`; `grace.rs` is new,
oracle = those assertions).
**Write-set:** `experiments/item-store/src/{seal.rs,grace.rs}`, `.../tests/e{7,8,9}_*.rs`.
**Shared-state contract:** none beyond write-set (key "destruction" is a mock file delete under tmp).
**Risks:** the seal's fail-closed write path is a `SEAM:` (mock key deletion) — mark it; real
key-destruction is a later spike.
**Observability:** The fail-closed path must be **loud** — a silent denied write is a bug. `tracing` WARN
on any post-seal write attempt (denied, with the pinned root); INFO on ceremony + tombstone (which cred
"destroyed" — the mock `SEAM:`); INFO on grace events (event kind, running grace balance netting to zero).
**Done when:** (1) sealed/tombstone verified, grace balances; (2) `cargo test e7 e8 e9` green. This
completes the **E0–E9 ledger core in Rust** (parity with the TS oracle).
**Validation:** Moderate — run the full crate suite; compare pass-count to the TS reference.

### Phase 7: S3-compatible interface + the metering boundary (SEAM #1 + #2)
**Goal:** A real HTTP server exposing an **S3-compatible put/get**, wired so every transfer produces a
signed receipt (postage) and rent derives from the signed manifest — the network boundary IS the metering
boundary. Backed by a real blob store (Garage/SeaweedFS local; local-FS fallback for the very first slice).
**Changes:**
- [ ] `src/server.rs` — HTTP server (crate per Phase 0 D1, e.g. axum) with S3-compatible `PUT`/`GET`
  object routes.
- [ ] `src/blobstore.rs` — the pluggable storage backend behind the interface (`BlobStore` trait; local
  FS first, then S3/Garage) + the boundary byte-count → receipt hook. **[E84]** the FS backend's
  temp→permanent move + content-addressed dedup may use `copy_file_range`/reflink (the one cheap
  kernel-perf edge worth adopting in v0; no hardening cost). The `BlobStore` trait is the attach point for
  the rest of the kernel-performance tier (io_uring/zero-copy backend impl) — deferred (E84).
- [ ] **op-dispatch seam (forward-compat, `SEAM:`)** — route request handling through a small op-dispatch
  boundary (an `Op` enum / dispatch fn) rather than inlining all work in the HTTP handler. v0 dispatches
  in-process; the seam exists so a **later** per-DID compute-observability wrapper (E83, watch-in-place)
  can route a *heavy* op (CAR export, MST rebuild, audit sampling, seal ceremony) into a per-DID cgroup
  scope without a rewrite. Mirror of the pluggable-backend / pluggable-addressing hooks. **Not a v0
  feature** — just the seam. Cheap ops (blob PUT/GET) never get scoped (spawn cost > their compute).
- [ ] `src/main.rs` (or `src/bin/item-store.rs`) — the **runnable service binary** entry point (the lib
  alone can't be `curl`'d or deployed; Phases 7/9 need a real binary).
- [ ] `tests/wiring_s3_metered.rs` — end-to-end: `PUT` bytes over HTTP → a signed receipt is recorded and
  postage tallied; `GET` returns bytes + receipt; rent recomputable from the manifest.
**Call chain:** HTTP `PUT /obj` → `server::put` → `blobstore::write` + `receipts::ack` → `ledger::append`.
**Wiring test:** `tests/wiring_s3_metered.rs` — the whole path from an HTTP request to a ledger receipt is
live (not just `blobstore` in isolation). RED at phase start, GREEN at end. **This is the anti-dead-code
gate.**
**Depends on:** Phases 2–4 (items/manifest for content-addressing + rent; receipts/statements),
Phase 0 (D1 HTTP/S3 crate choice, D3 boundary shape, D5/D6 backend + which "S3" is the boundary).
**Read-set:** `src/{item,receipts,ledger,manifest,statements}.rs`; Phase 0 D1/D3/D6 notes.
**Write-set:** `experiments/item-store/src/{server.rs,blobstore.rs,main.rs}`, `.../tests/wiring_s3_metered.rs`,
`Cargo.toml` (new deps).
**Shared-state contract:** binds a local TCP port in tests (use an ephemeral port; assert none leaks);
writes blobs under a tmp dir / a local Garage instance scoped to the test.
**Risks:** S3 API compatibility surface is large — implement the **minimal** put/get subset for v0, mark
the rest `SEAM:`. Port count / async runtime (tokio) discipline.
**Observability:** The metering byte-path must be **traceable end-to-end** (the phase where the receipt/
ledger writes, the HTTP boundary, and the byte-count meet). Structured `tracing` events **to stdout/journald
for our own debugging** (**[Phase 0 D7]** the croft-stack poller reads cgroup v2, **not** tracing — so this
is not a "poller contract," it is the `journalctl` trail we read when metering looks wrong): per HTTP
request at the boundary — method, object key, status, bytes-in/out (INFO); per receipt written — receipt
id, mode, running total, ledger index (INFO); backend write/read — `BlobStore` impl, CID, byte count
(DEBUG); **fail-loud WARN/ERROR on any byte-count mismatch between the HTTP boundary and the receipt** (the
metering-integrity invariant). Log the bound ephemeral port (DEBUG) so a leaked port is diagnosable.
Resource telemetry (memory/cpu/io) is a **cgroup-accounting concern on the systemd unit at Phase 9**, not
an app-level emission here.
**Done when:** (1) *Behavioral:* I can `PUT`/`GET` a real object over HTTP and the transfer is metered
with a signed receipt + a recomputable rent; (2) *Verification:* `cargo test --test wiring_s3_metered`
green (from the crate dir; standalone — no `-p`) + a manual `curl PUT/GET` against a locally-run instance.
**Validation:** Broad — first real integration (HTTP + a real blob backend + the metering wiring).
Out-of-harness checks: `curl` PUT then GET a real object against a locally-run instance; inspect the
per-user SQLite to confirm the receipt + running total; **assert the HTTP-boundary byte count == the
receipt byte count** (metering integrity); recompute rent from the manifest independently; confirm the
ephemeral test port is released afterward (no leak). **[E86] Run the end-to-end abuse suite here** —
drive the live engine and actively try to break it (forge/replay receipts, inflate manifest, tamper at
rest across the boundary, walkaway, double-count audit, malformed input).

### Phase 8: Minimal atproto PDS API surface — the thin blob-endpoint layer (in v0, confirmed 2026-07-31)
**Goal:** Serve the minimal atproto PDS endpoints (at least `getBlob`/`uploadBlob`, plus whatever D2/D3
deem the v0 floor) as a **thin blob-endpoint layer on top of the S3-metering plane**, so the store is a
**PDS-like node on the Bluesky network**. **In v0 (user-confirmed): both boundaries from the start** — the
S3-compatible interface (Phase 7) is the storage/metering plane; this phase is the atproto layer over it.
Phase 0 D2/D3/D6 set the exact endpoint set + shapes; the phase is **not gated out**.
**Changes:**
- [ ] `src/pds_api.rs` — the atproto endpoints **from D2's confirmed floor: `uploadBlob` (POST, auth
  required), `getBlob` (GET, public), `listBlobs` (GET, public)** — mapped onto `blobstore` (mapping
  modeled on rsky's `apis/com/atproto/{repo/upload_blob,sync/get_blob}.rs`); the metering hook stays on
  the byte path. **`uploadBlob` must return the exact confirmed shape**
  `{"blob":{"$type":"blob","ref":{"$link":"<CIDv1>"},"mimeType":"<ct>","size":<int>}}`; `listBlobs`
  returns `{"cids":[...],"cursor":"..."}`.
- [ ] **auth `SEAM:`** — `uploadBlob` requires a session/JWT (OAuth Bearer) per D2; v0 stands in a mock
  auth check behind a `SEAM:` (real DID-session/OAuth is a later spike), `getBlob`/`listBlobs` stay public.
- [ ] `tests/wiring_pds_blob.rs` — end-to-end: an `uploadBlob`/`getBlob` round-trip over the atproto API,
  metered, returning the exact D2-confirmed `blob` shape (`$type`/`ref.$link`/`mimeType`/`size`); a
  `listBlobs` returns the uploaded CID.
**Call chain:** atproto `uploadBlob` → `pds_api::upload` → `blobstore::write` + `receipts::ack`.
**Wiring test:** `tests/wiring_pds_blob.rs` — the atproto endpoint reaches the metered blob path. RED → GREEN.
**Depends on:** Phase 7, Phase 0 (D2/D3).
**Read-set:** `src/{server,blobstore,receipts}.rs`; Phase 0 D2/D3 notes.
**Write-set:** `experiments/item-store/src/pds_api.rs`, `.../tests/wiring_pds_blob.rs`.
**Shared-state contract:** same as Phase 7 (ephemeral port, tmp storage).
**Risks:** must match the **verified** atproto shapes from D2 exactly — no guessing (global rule). If the
full PDS surface is large, ship only the blob subset for v0 and mark the rest `SEAM:`/tracked.
**Observability:** `tracing` at the atproto boundary — per `uploadBlob`/`getBlob`: endpoint, blob CID,
byte count, and **the metered receipt id it produced** (INFO); WARN on an atproto-shape mismatch vs D2.
Reuses Phase 7's byte-path trail (the metering hook is shared); this phase adds only the atproto-surface
span so the two boundaries are distinguishable in the telemetry.
**Done when:** (1) an atproto `uploadBlob`/`getBlob` round-trip works and is metered; (2) `cargo test
--test wiring_pds_blob` green (crate dir; no `-p`) + a manual probe against a locally-run instance.
**Validation:** Broad — out-of-harness: probe `uploadBlob`/`getBlob` against a locally-run instance
(`curl` or a minimal atproto client) and **diff the response against the D2-confirmed atproto shape**
(no guessing — global rule); confirm the round-trip is metered (receipt present) via the SQLite + the
Phase 7 byte-path trace.

### Phase 9: croft-stack deploy + VPS smoke test
**Goal:** Deploy the built binary on the VPS via croft-stack as a governed, hardened, isolated service we
can test against.
**Changes:**
- [ ] `croft-stack: services/<name>.toml` — the service manifest (fqdn, port, mode, data profile, limits,
  hardening carve-outs, netns) under the placeholder name.
- [ ] croft-stack role/wiring for the binary (build/ship, systemd unit via the template, telemetry
  poller, hardening baseline, netns isolation).
- [ ] a smoke test / canary hitting the deployed put/get (and blob endpoint if Phase 8 shipped).
- [ ] **[Pass 3, Doc-Impact] the scheduled doc edits that go stale at deploy** — in `discovery`:
  `ECOSYSTEM.md` §5c-3 (add the deployed-service row), `COHESION.md` §65 (v0-runs status note),
  `ROADMAP_TODO.md` E82 (status → deployed/live), the lane doc's "Next step",
  `plans/croft-stack/README.md` (the plan index notes the live service); in the **croft-stack repo**:
  register the service in its `services/*` set + repo docs. These land in Phase 9 because deploy is what
  makes them stale — not a trailing docs phase.
**Call chain:** `render.py` → Ansible converge → systemd unit → the running service; canary → HTTP put/get.
**Wiring test:** a deploy smoke test (put/get against the live VPS instance) — the service is reachable,
metered, and governed (telemetry shows it).
**Depends on:** Phase 7 (and Phase 8 if in v0). **This phase writes in the `croft-stack` repo**, not
`discovery` — its own write-set, committed there.
**Read-set:** croft-stack `00-model-and-manifests.md`, `service-hardening-plan.md`, `netns-isolation-plan.md`,
`telemetry-client-plan.md` (**[D7]** the cgroup-accounting contract — `*Accounting=yes` + limits, not a log
format), the tenant template; **Phase 0 D7 notes**.
**Write-set:** **two repos** — (croft-stack) `services/<name>.toml`, the role files, the canary config +
repo docs; (discovery) `ECOSYSTEM.md`, `COHESION.md`, `ROADMAP_TODO.md`,
`plans/2026-07-31-coop-storage-metered-hosting-lane.md`, `plans/croft-stack/README.md`. **Two commits, two
repos** (croft-stack deploy; discovery doc/status) — do not cross-commit.
**Shared-state contract:** touches the VPS (a real deploy) — governed envelope + netns; this is the one
phase with real external side effects. Coordinate with the estate (ports, DNS/TLS, identity block).
**Risks:** the VPS is production estate — deploy behind the hardening baseline + netns from the first;
confirm no port/identity collision with existing services (relay/broker/telemetry/canary/caddy).
**Observability:** The **telemetry-poller integration** is a **systemd-unit / cgroup-accounting concern**,
not app wiring (**[Phase 0 D7]** the poller reads cgroup v2 files, not `tracing`). Concretely: set
`MemoryAccounting=CPUAccounting=IOAccounting=TasksAccounting=yes` on the unit and declare the governed
envelope — `MemoryHigh` (soft) + a generous `MemoryMax`, `CPUQuota`, `TasksMax`, `IOWeight` sized to the
role; confirm `/sys/fs/cgroup/system.slice/<unit>.service/{memory.current,cpu.stat,pids.current,io.stat}`
populate and the poller records them **within the envelope** (telemetry-within-envelope). Separately, the
Phase 7/8 `tracing` trail lands in **journald** (read via `journalctl -u <unit>`) — verify **no secret
material** (Zeroize keys) reaches it. Never add a cgroup namespace (breaks the poller's cross-unit reads).
**Done when:** (1) *Behavioral:* the metered store is reachable on the VPS and a put/get round-trip is
metered + telemetered; (2) *Verification:* the smoke test passes against the live instance + telemetry
shows the unit within its envelope.
**Validation:** Broad — real deploy; out-of-harness: `curl` PUT/GET against the live fqdn; verify logs,
telemetry-**within-envelope**, and hardening (`systemd-analyze security` on the unit); confirm no
port/identity collision with existing services; check the byte-path/receipt events surface in the estate
telemetry, end-to-end in a prod-like setting.

### Phase 10 (tracked, later — gated): history-convergence consumer/meer mode
**Goal:** Wire the content-blind meer mode so the MLS history-convergence server uses this store as its
substrate (one store, two consumers). **Gated on drystone's fold/MLS becoming real** (per
`10-drystone-layer.md`) — likely **out of v0**; tracked, not built now.
**Depends on:** Phases 7–9 + a real MLS group producing real history.
**[Phase 0 D4] Open reconciliation (Phase-10-gated, not a v0 blocker):** the two consumers address
differently — blob hosting by **CID** (CIDv1 raw+sha-256), the content-blind convergence node by
**envelope hash** (blake3 in-house digest, set-reconciliation/RBSR over envelope hashes) — the still-open
`HS OC-2` (`hist-atproto-spike/src/record.rs:8-13`). The convergence node also requires **content-blind**
storage (ciphertext only, blindness at a compile/dependency boundary) and membership-interval-scoped
backfill (GROUPS.md). v0 must therefore keep the store's **addressing pluggable** (don't hardcode CID as
the only key) so Phase 10 can add the envelope-hash addressing without a rewrite. relay-lab E8/E9 (the
blind-mirror confidentiality tiers) are **not started** — a precondition to measure before this phase.
**Done when:** (deferred) the convergence node converges append-only history through this store,
content-blind.

## Open Questions

- [CONFIRMED: BLOCKING] **The network boundary shape** — **RESOLVED (user, 2026-07-31): BOTH, from the
  start** — an **S3-compatible interface as the storage + metering plane** with the **atproto PDS API as
  a thin blob-endpoint layer on top**. **Phase 8 is in v0** (not deferred). Phase 0 still verifies the
  exact atproto shapes (D2/D3) and the official-PDS S3 support + the interface-vs-backend distinction
  (new **D6**). *Both boundaries are core; sets Phases 7–8 scope.*
- [CONFIRMED: PHASE-GATED (Phase 9)] **Repo / IP home** — **RESOLVED (user, 2026-07-31): start in
  `discovery/alpha/experiments/item-store/`** (beside the `item-storage-protocol` it ports, under the
  reviewed-before-commit flow); decide the graduation target (own `CroftCommunity/<repo>` vs into
  `croft-stack`) at **Phase 9**. Same IP/ownership class as the app Phase-0 (A8) / foundation IP (E28) —
  the user's call at deploy.
- [CONFIRMED: PHASE-GATED (Phase 9)] **The service NAME** (A21) — **placeholder set to `item-store`**
  (user, 2026-07-31: "metered is a capability," so name the *noun* — items — not the capability; ties to
  `item-storage-protocol`). The co-op + storage service remain formally **unnamed**; the real name is a
  deliberate naming pass (Amble/Noria-style) triggered at **Phase 9** before a public fqdn/repo. Not a
  v0-code blocker.
- [CONFIRMED: PHASE-GATED (Phase 7)] **Blob backend for the first slice** — **RESOLVED (user,
  2026-07-31): a pluggable backend behind a trait; FS first, Garage/SeaweedFS/R2 later.** Because metering
  lives at the boundary (Layer 2), the backend (Layer 1) is a deliberately-dumb, swappable implementation
  detail — FS gets to the first wiring test fastest; Garage/SeaweedFS before Phase 9 deploy (not MinIO —
  archived). Confirmed against rsky-pds in Phase 0 D5.
- [CONFIRMED: ADVISORY] **Metering-ledger storage** — **RESOLVED (user, 2026-07-31): store the metering
  records *alongside the user repo* in the same per-user SQLite** (official-PDS pattern), as same-shaped
  signed records managed the same way — not a separate engine (supersedes the redb lean). Blob *bytes*
  stay in the pluggable backend (Layer 1); cumulative state is a **signed rollup** (balance-forward E4)
  with granular receipts **purgeable past the co-signed period boundary**. Subtlety for Phase 0/3–4:
  manifest = single-author (fits the repo); receipts/statements = bilateral co-signed (a co-signed
  structure alongside). ADVISORY — swappable behind the ledger API.
- [CONFIRMED: ADVISORY] **Does v0 include the E11–E14 funder-diligence layer?** **RESOLVED (user,
  2026-07-31): no — deferred** (capital layer; the publish-co-attested-ledgers transparency commitment is
  an unresolved decision). v0 stays the storage service. The E3 **two-mode receipt** (unilateral |
  bilateral, social-trust-selected) keeps v0 forward-compatible — the deferred layer will require
  bilateral (co-attested) records.
- [CONFIRMED: ADVISORY — post-v0, tracked E83] **Per-DID compute/mem/io observability ("watch in
  place").** **DECIDED (user, 2026-07-31):** leverage the **cgroup v2 primitive** to attribute per-repo /
  per-DID CPU/memory/IO load — as **operational data, NOT a billing axis**. Billing stays **transfer
  (postage) + storage (rent)**; compute/mem/io is for (a) capacity/scaling signals ("where to scale, when
  to step in"), (b) per-user load **attribution**, and (c) an **anti-gamesmanship cross-dimension** (a
  second orthogonal observation so abuse that games the transfer model — e.g. tiny transfers driving heavy
  compute — still shows up). **Mechanism:** `Delegate=yes` on the unit → the service creates a per-DID
  child cgroup for a *heavy* op, moves the worker in, and reads `cpu.stat`/`memory.peak`/`io.stat` deltas
  **directly** (the service is its own consumer of the cgroup primitive — **independent of** the
  croft-stack admin poller; no cross-repo dependency for attribution). Cheap ops (blob PUT/GET) are never
  scoped (spawn cost > their compute). Provenance: machine-measured → **Unilateral only** (never
  co-attestable) — moot here since it is not a charged/attested unit. **v0:** only the **Phase-7
  op-dispatch `SEAM:`** (forward-compat); the capability itself is post-v0 (E83). *Latent decision kept
  closed: whether compute ever becomes a billed axis — no for now (economic-model / D5, the user's call).*
  **Data model (decided 2026-07-31):** a **separate** record **linked by receipt/op id**, co-located in
  the per-user SQLite (a third shape alongside manifest + receipts/statements), **unsigned to start**,
  separately rolled-up/purged. Resource data is **never** embedded in the signed receipt — it is
  non-deterministic + Unilateral and would break the receipt's deterministic co-attestation. **Where
  signing would attach if we decide later:** a provider-signed **Unilateral** signature over the sidecar
  at op close-out (never co-signed) — a hook that could underpin **delegating / authorizing burden-heavy
  operations** via an attested resource record.
- [CONFIRMED: ADVISORY — post-v0, tracked E84] **Kernel-performance tier (compliant kernels).** The
  metering logic is irreducibly **L7/userspace** (CID + signed receipt; unlike LVS/IPVS/XDP at L3/L4), but
  transport + storage offload to the kernel: `SO_REUSEPORT` (connection LB), `io_uring` (async I/O,
  zero-copy send), `sendfile`/`splice` + page cache (zero-copy reads / free hot cache),
  `copy_file_range`/reflink (temp→permanent move + content-addr dedup), SHA-NI/ARMv8 sha-256 (the one CPU
  pass), **PSI** (contention signal → E83 "when to step in"). **v0 stays simple** (`tokio` epoll + std I/O
  + SHA-NI sha-256); adopt only `copy_file_range`/reflink early (cheap, no hardening cost). **Tension:**
  `io_uring` vs the seccomp/hardening baseline (`io_uring_disabled=2` on hardened kernels) — hardening
  likely wins v0; io_uring is a measured later upgrade. The `BlobStore` + Phase-7 op-dispatch seams are the
  attach points. Gated on a **Phase-9 probe of the VPS kernel version** (needs 5.x+).
- [CONFIRMED: ADVISORY — post-v0, tracked E85] **Object grouping / index structure (flat vs MST vs RBSR).**
  Three distinct set-structures: **blobs** (v0) = flat `(DID,CID)` set (not MST-structured even in atproto);
  the **billing manifest (E2)** = flat sorted Merkle root (already deterministic-over-set + tamper/omission
  detection — adequate for v0, a periodic co-signed snapshot); atproto **repo records** = an **MST** (keyed
  `collection/rkey`; deterministic shape + bounded O(log n) add/remove + compact diffs/proofs — a spec
  requirement only if we host records, full-repo scope out of v0); the **convergence consumer** (Phase 10)
  = **RBSR** over envelope hashes (iroh-docs; per FACTCHECK not an MST). **v0 flat is correct**; adopt an
  MST-like content-addressed object index only if/when we host atproto records, the flat manifest's O(n)
  re-sign hurts at scale, or we need compact audit-inclusion / sync-diff proofs. Keep manifest/index
  addressing **pluggable** (same seam as D4 / `HS OC-2`). Not a v0 blocker.
- [CONFIRMED: PHASE-GATED (Phase 7/8)] **Access model — two axes: access + confidentiality.** DECIDED
  (user, 2026-07-31, corrected): default **public-read, authorized-write** (PDS/unix-`774`); writes
  authorized to the owner DID + delegated capabilities. Metering orthogonal. **Access (object gating) IS
  server-enforceable, even when blind** — the store authenticates the requester and decides whether to
  hand over the object without knowing its contents (public-read is a default, gateable per object).
  **Confidentiality (plaintext) is a separate axis** — encryption; keys held by the group; the server
  gates ciphertext access but never controls decryption. They compose (public/gated × plaintext/encrypted;
  gated+plaintext = the delegate tier). **Public relay replication (`subscribeRepos` firehose) is an opt-in
  feature, not a requirement** (out of v0). Lands on the Phase 8 `uploadBlob` auth `SEAM:` (v0 mocks auth);
  not an E0–E9-core concern. Delegation ties E83 + the social-trust layer.
- [CONFIRMED: ADVISORY — tracked E86] **Test-hardening for a sensitive/complex engine.** DECIDED (user,
  2026-07-31): beyond per-phase wiring tests, three layers — (1) **paired rejection tests** (every verify
  path gets a should-fail-for-sure negative, not just a happy path — apply now, per phase); (2) **mutation
  testing** (`cargo-mutants`) + **property tests** (`proptest`: canonical round-trip, Merkle order-
  independence over random sets, sign/verify round-trips) — periodic, operationalises the Pass-3 mutation-
  resistance gate; (3) an **end-to-end abuse suite** driving the whole engine and actively trying to break
  it (forge/replay receipts, inflate manifest, tamper at rest across the boundary, walkaway, malformed
  input) — real version at **Phase 7** (HTTP engine); an in-process E0–E9 abuse suite before that. Not a
  v0 blocker; strengthens validation throughout.

## Review Log

- **Pass 1 (2026-07-31):** Base plan drafted. Problem/Reasoning grounded in the session's filing + local
  reads of the item-storage core, croft-stack model, ECOSYSTEM §5e. Phase 0 added for the rsky-pds/atproto
  unknowns (network now open → fetch, not local-drop). E0–E9 port split across Phases 1–6 (≤4 files each,
  per the split rule); SEAM-closure in Phases 7–8; deploy in Phase 9; convergence consumer tracked as
  Phase 10 (gated). Concurrency: all sequential (linear ledger dependency; shared crate write-set).
- **Pass 1 walk-through (2026-07-31):** Q1 (boundary shape) CONFIRMED BLOCKING and **resolved
  substantively — both boundaries from the start** (S3-compatible metering plane + a thin atproto PDS
  blob-endpoint layer on top). Effect: **Phase 8 moved from Phase-0-gated/deferrable → in v0**; added
  **D6** (verify official-PDS S3 support + the S3-interface-vs-backend distinction, from the user's
  question).
- **Pass 1 walk-through, cont'd (2026-07-31):** Q2 (repo/IP home) → PHASE-GATED (Phase 9), start in
  `experiments/item-store/`. Q3 (name) → PHASE-GATED (Phase 9), placeholder `item-store` ("metered is a
  capability" — name the noun). Q4 (blob backend) → PHASE-GATED (Phase 7), pluggable/FS-first (added the
  **two-layer split**: dumb backend vs boundary-metering). Q5 (ledger storage) → ADVISORY, **resolved to
  store metering records alongside the user repo in per-user SQLite** (official-PDS pattern; supersedes
  redb) + **rollup/purge** made explicit in Phase 4 + the bilateral-vs-single-author subtlety noted +
  D5 revised. Q6 (E11–E14 in v0?) → ADVISORY, **deferred**. Plus a design refinement to **Phase 3**: the
  transfer receipt is now **two-mode** — `Unilateral` (provider-signed our-side measurement) |
  `Bilateral` (co-signed) — **social-trust-layer-selected**, both valid; bilateral is the co-attested
  form the deferred capital layer (E11–E14) will require (forward-compatible; ties E14). **All 6 open
  questions confirmed; Pass 1 complete.**

### Pass 2: Gap Analysis — 2026-07-31
**Found:**
- **Port-oracle mismatch (factual):** Read-sets cited the full-version module names; the dependency-free
  **standalone** (the 81/81 build) is the correct oracle, with different names (`item/receipt/statement/
  clock/rng/pricing.ts`). Verified against both `src/` trees.
- **"Workspace member" was wrong (factual):** no experiments-wide Rust workspace exists; the sibling
  crates are standalone → `item-store` is its **own crate**.
- **Internal Rust prior art unreferenced:** `experiments/hist-atproto-spike/` + `lexicon-community/`
  (atproto/history in Rust) — Phase 0 should read these before fetching rsky externally.
- **Preconditions:** Phase 4 persistence depends on **Phase 0 D5**; Phase 7 depends on Phases **2**–4
  (needs items/manifest for content-addressing + rent) and needs a **`main.rs`** entry point (a library
  alone can't be `curl`'d or deployed — Phases 7/9 need a runnable binary).
- **SEAM discipline** (SPEC Part 5: mocks marked `SEAM:` so production gaps grep-enumerable) wasn't stated
  as a port invariant.
**Concurrency:**
- Code spine confirmed **sequential** (shared crate `lib.rs`/`Cargo.toml`). **{7,8} audited → NOT
  parallel** (Phase 8 depends on Phase 7's server; overlapping `server.rs`/`Cargo.toml` write-set).
  Surfaced one real overlap: **Phase 0 (discovery — no code write-set) ∥ Phases 1–3 (pure port)**;
  findings must land before Phase 4 persistence + Phases 7–8. Recorded in the Concurrency Map.
**Changed:**
- Verified Assumptions: added the port-oracle/standalone module map, the no-workspace fact, the
  internal-prior-art crates, resolved doc references. Preamble: oracle = standalone; standalone crate;
  SEAM discipline. Phase 0 D1: internal spikes first. Phase 1: standalone-crate language. Phase 4:
  +Phase-0-D5 dependency. Phase 7: Depends-on 2–4, +`main.rs`, `BlobStore` trait. Concurrency Map:
  Phase-0∥1–3 note + {7,8}-not-parallel.
**Confirmed:**
- The E0–E9 port target is complete + proven (81/81). The two-layer split, per-user-SQLite co-location,
  rollup/purge, and the two-mode receipt hold up under review. **No new open questions; no Pass-1 severity
  revised.**

### Pass 3: Quality Gates — 2026-07-31
Spot-checked the codebase first: all touch points resolve — the standalone oracle
(`experiments/item-storage-protocol-standalone/src/` with `item/receipt/statement/clock/rng/pricing.ts` +
`crypto/manifest/ledger/audit/seal/canonical` + `exp/e0_identity.ts … e11_financing.ts`), the full
`item-storage-protocol/{README.md,SPEC.md}` (SEAM discipline at SPEC.md:225), the internal Rust spikes
(`hist-atproto-spike`, `lexicon-community`), `appview-infra/GROUPS.md`, all `plans/croft-stack/*` (incl.
`telemetry-client-plan.md`, `10-drystone-layer.md`), `ROADMAP_TODO` E82, `ECOSYSTEM` §5c-3/§5e, `COHESION`
§65, the lane doc. `item-store` not yet created (expected). Two spot-check facts folded into read-sets:
the standalone has **no `dial.ts`/`grace.ts` src modules** (logic in `pricing.ts`/`ledger.ts` + the
`exp/e6_dial.ts`/`e9_grace.ts` experiments), and the RNG module is `rng.ts` (not `prng.ts`).

**TDD ordering:**
- Every impl phase already RED-first with a named wiring test (`tests/e*_*.rs`, `wiring_s3_metered.rs`,
  `wiring_pds_blob.rs`, Phase 9 deploy smoke test) failing before code, green at end — confirmed, no
  change needed.
- **Verification-command fix (real defect):** Phases 1/2/3/7 used `cargo test -p item-store …`, but Pass 2
  established `item-store` is a **standalone crate, not a workspace member**, so `-p` (a workspace-package
  selector) is wrong. Added a preamble Test-invocation note and normalized every Verification command to
  `cargo test <filter>` / `cargo test --test <binary>` run from the crate dir (also de-ellipsized the
  `cargo test ...` shorthands in Phases 4/5/6/8).
- **Mutation-resistance (Phase 5):** its detection-math/threshold tests were single-table; added explicit
  boundary cases (`f=0`, `k=0`, `k=1`, full-corpus k) + tier-edge assertions for the dial, and required
  asserting the *relationship* across k, not one interior point.
- rust-enforcer discipline (no `unwrap` in prod, `Result`/`thiserror`, Zeroize on keys, doc comments,
  `clippy::pedantic`) confirmed in the preamble + Phase 1; Observability additions reinforce the
  fail-loud / no-`Debug`-print-of-secrets rules.

**Observability:**
- The plan had **no** logging/metrics declarations — the biggest gap. Added an **Observability** field to
  every phase, calibrated: Phases 1–6 (library) = typed-error surface + `tracing` WARN on verify/tamper/
  chain/seal failures, DEBUG on appends, seed logged for the Monte-Carlo phase, fail-closed writes logged
  loud, secrets never printed. Phases 7–9 (the user's focus) = the full **metering byte-path** trail (HTTP
  boundary method/key/status/bytes; receipt id/mode/running-total/ledger-index; backend CID/bytes;
  fail-loud on any boundary-vs-receipt byte-count mismatch = the metering-integrity invariant), the atproto
  boundary span (endpoint/CID/receipt-id), and the **telemetry-poller integration** at Phase 9.
- Added **Phase 0 D7** (read `telemetry-client-plan.md` + `service-hardening-plan.md`) so Phase 7's
  `tracing` output is built to the poller's confirmed contract + the governed envelope, not invented; D7
  disposition = throwaway; wired into the Concurrency Map ("before Phases 7–8") and Phase 9's read-set.
  **[Superseded — see Phase 0 D7 RESOLVED: the poller reads cgroup v2, not `tracing`; this "tracing→poller
  contract" premise was overturned.]**

**Debugging readiness:**
- Phase boundaries are already commit-at-green checkpoints; each wiring test is the health gate. The new
  byte-count-mismatch WARN/ERROR (Phase 7) and the seed-logging (Phase 5) make the two flakiest paths
  self-diagnosing. Phase 9 read-set now includes the telemetry plan so a deploy-time telemetry gap traces
  back to the D7 contract.

**Validation calibration:**
- **Phase 7 upgraded Moderate → Broad** (first real integration: HTTP + blob backend + metering) with
  named out-of-harness checks (curl PUT/GET, SQLite receipt inspection, boundary-vs-receipt byte-count
  assertion, rent recompute, port-leak check). Phase 8 (Broad) given a concrete probe (diff response vs
  the D2-confirmed atproto shape). Phase 9 (Broad) named `systemd-analyze security`, telemetry-within-
  envelope, live-fqdn curl, port/identity-collision check. Phases 1–6 Narrow/Moderate left as-is
  (pure-logic port, appropriate).
- Phase 0 discovery tasks all carry a concrete question/probe/success + a **disposition** (all `throwaway`);
  D7 added in the same shape. None could be resolved during planning (they need live fetches / local reads
  at execution).

**Concurrency honesty:**
- Concurrency Map accounts for every phase; code spine sequential (shared `lib.rs`/`Cargo.toml`), {7,8}
  audited not-parallel. Converted the one real overlap (Phase 0 ∥ Phases 1–3) from a mechanism note to an
  **invariant + re-entry check** (Phase 0 writes only notes, never the crate, no git mutation in the crate
  worktree; re-entry: crate `git status` shows only the port agent's files, main HEAD == pre-dispatch SHA).
  No new parallelism available — the ledger dependency chain is genuinely linear.

**Documentation impact:**
- The section already scheduled updates per-triggering-phase (no trailing docs phase). **Promoted the
  scheduled edits to first-class phase checklist items + write-set entries:** Phase 1 now edits the
  existing `item-storage-protocol/README.md` (cross-ref) — previously only listed in Doc-Impact; Phase 9
  now has an explicit checklist item + write-set for `ECOSYSTEM §5c-3`, `COHESION §65`, `ROADMAP_TODO`
  E82, the lane "Next step", `plans/croft-stack/README.md`, and the croft-stack `services/*` registration,
  with a **two-repo / two-commit** note (croft-stack deploy vs discovery docs).

**Confirmed ready:** yes — pending the two prior-confirmed PHASE-GATED items (repo/IP home + service name
at Phase 9) and BLOCKING-resolved boundary shape (already resolved: both). No new open questions.

### Phase 0: Discovery — COMPLETE 2026-07-31
Executed under the Discovery Exemption (no TDD/wiring/commit-per-item; all seven tasks `throwaway` notes).
Ran three parallel researchers: local corpus reads (D1-internal/D4/D7/D5-corpus) + live WebFetch of the
canonical atproto lexicon JSON, rsky-pds, and the official `@atproto/pds`/`bluesky-social/pds` (D1-ext/
D2/D6). All findings + evidence are in Verified Assumptions ([Phase 0 RESOLVED …] bullets); this entry is
the narrative.

**Confirmed (assumptions held):**
- rsky-pds is real Rust prior art (Rocket + `aws-sdk-s3` + `rusqlite` per-actor + `atrium-api`/
  `rsky-lexicon`); its `BlobStore` trait + `blocks/{did}/{cid}` layout + `upload_blob.rs`/`get_blob.rs`
  handlers are directly reusable for Layer 1 + the atproto→backend mapping (D1).
- Official PDS = per-actor SQLite (WAL) + a PDS-wide SQLite; S3 supported as a **backend** via
  `PDS_BLOBSTORE_S3_*` (or disk) (D5/D6). Confirms the per-user-SQLite co-location decision.
- atproto blob API shapes locked from the lexicon source — uploadBlob (POST, auth), getBlob (GET, public),
  listBlobs (GET, public), blob response `{$type,ref.$link,mimeType,size}` (D2).
- "One store, two consumers" holds by design for v0 (blob hosting real; convergence gated to Phase 10) (D4).

**Changed (discovery altered the plan — the point of Phase 0):**
- **D7 material correction:** the croft-stack telemetry poller reads **cgroup v2 files, not `tracing`/
  Prometheus**. Rewrote Phase 7 + Phase 9 Observability: app `tracing`→journald is our own debugging trail;
  the poller integration is a **systemd-unit accounting + envelope** concern (`*Accounting=yes`,
  `MemoryHigh`/`MemoryMax`/`CPUQuota`/`TasksMax`/`IOWeight`), no app-level metric emission. Corrected the
  Pass-3 "build tracing to the poller contract" premise (Phase 0 D7 task, Concurrency Map).
- **D6 load-bearing distinction:** no PDS exposes an S3-compatible **client** interface (case b) — both use
  S3 only as an internal **backend** (case a). Recorded that our case-b metering boundary is the novel,
  no-prior-art part built from the S3 spec; noted "S3 appears twice" (exposed front door vs optional dumb
  backend) in VA + Phase 7 framing.
- **D2 scope additions to Phase 8:** added `listBlobs` to the minimal floor; added an **auth `SEAM:`**
  (uploadBlob needs a session/JWT — mock in v0, getBlob/listBlobs public); pinned the exact response shape.
- **D1-internal reuse:** Phase 2's CIDv1/DAG-CBOR `SEAM:` can be closed with the in-corpus
  `serde_ipld_dagcbor`+`ipld-core`+`sha2` path (byte/CID-identical to real PDS); Phase 1 risk updated —
  no internal Ed25519 precedent, TS oracle is the parity target.
- **D4 Phase-10 tracked item:** blob hosting addresses by CID, convergence by envelope-hash (open
  `HS OC-2`); v0 must keep addressing **pluggable**. Recorded in Phase 10 + VA. relay-lab E8/E9 not started.

**No v0 blocker surfaced; no BLOCKING open question reopened.** Phases 1–6 (pure ledger port) are unaffected
by discovery and ready to start. Phases 7–9 are now sized on firsthand evidence.

### Decision 2026-07-31 — per-DID compute/mem/io observability ("watch in place")
Design thread raised by the user off the D7 cgroup finding. Recorded (not built): the cgroup v2
**primitive** (not the admin poller) can attribute per-repo/per-DID CPU/memory/IO load; this is
**operational data + attribution + an anti-gamesmanship cross-dimension, NOT a billing axis** — billing
stays transfer + storage. Mechanism: `Delegate=yes` → the service reads per-DID child-cgroup deltas
directly, independent of the croft-stack poller (no cross-repo dependency for attribution; the poller-glob
change is only for an optional admin dashboard). Machine-measured ⇒ Unilateral provenance, moot since not
charged. **Plan changes:** added the **Phase-7 op-dispatch `SEAM:`** (forward-compat only — route heavy
ops through a dispatch boundary so a later per-DID scope wrapper slots in; cheap ops never scoped), a
**CONFIRMED: ADVISORY (post-v0) Open Question**, and backlog **E83** in `ROADMAP_TODO.md`. Corrected a
Pass-3-era caveat: per-DID attribution is **not** a croft-stack poller change (the service is its own
independent reader of the cgroup primitive). v0 scope unchanged; only the seam lands in v0. **Data model
(decided 2026-07-31):** separate sidecar record linked by receipt/op id, co-located per-user SQLite,
unsigned to start, separately purged; never embedded in the signed receipt; signing hook = provider-signed
Unilateral at close-out if later needed (a possible basis for delegating burden-heavy ops).

### Decision 2026-07-31 — kernel-performance tier (compliant kernels)
Design thread raised by the user (LVS-analogy: reuse kernel primitives). Recorded (not built): the L7
metering logic stays userspace, but transport+storage offload to the kernel — `SO_REUSEPORT`, `io_uring`
(+ zero-copy send), `sendfile`/`splice` + page cache, `copy_file_range`/reflink (move + content-addr
dedup), SHA-NI sha-256, PSI (contention → E83). **v0 stays simple** (`tokio` epoll + std I/O + SHA-NI);
only `copy_file_range`/reflink adopted early. Flagged tension: `io_uring` vs the seccomp/hardening baseline
(hardened kernels disable io_uring) — hardening wins v0, io_uring is a measured later upgrade. The
`BlobStore` + Phase-7 op-dispatch seams are the attach points; gated on a Phase-9 VPS-kernel-version probe
(needs 5.x+). Recorded as backlog **E84** + a Phase-7 blobstore note + Open Question. v0 scope unchanged.

### Decision 2026-07-31 — access model (corrected) + test-hardening (E86)
Two records from the Phase-3 discussion. **(1) Access model — corrected.** Initial framing overstated that
read-ACLs aren't server-enforceable in a blind model. User correction: the store *can* gate access to the
**object** (hand over bytes or not) without knowing its contents — access control is server-enforceable
even when blind; what the server cannot control is **decryption** (confidentiality). So **access and
confidentiality are two independent axes**, not one; "public-read" is a default, gateable per object; and
public relay replication (`subscribeRepos` firehose) is an **opt-in feature, not a requirement** (out of
v0). Corrected the Reasoning "Access model" note + the Open Question; lands on the Phase-8 auth `SEAM:`.
**(2) Test-hardening (E86).** Beyond wiring tests: paired rejection tests (now, per phase), `cargo-mutants`
+ `proptest` (periodic — operationalises the Pass-3 mutation-resistance gate), and a Phase-7 end-to-end
abuse suite. Backlog E86 + Open Question + Phase-7 validation note. Neither is a v0 blocker.

### Test run 2026-07-31 — E0–E3 mutation gate
Ran `cargo-mutants` on the E0–E3 crate (`experiments/item-store/`). First pass: 130 mutants, 24 survived —
almost all in the boolean structure of the receipt verify predicates (`verify_bilateral`'s `&&`,
`verify_unilateral`'s guard, `is_acknowledged`/`is_co_attested`) plus a **security-relevant** one: a
`leaf_hash` that ignored its input survived (no test checked the manifest root actually *binds* each leaf's
`(cid, size)` — a size-forgery would go undetected). Added paired should-fail-for-sure negatives (E86
layer 1) covering each. Second pass: **117 mutants → 103 caught / 0 missed / 13 unviable / 1 benign
timeout** (the `merkle_root` `while len > 1` → `>= 1` non-terminating loop). Trivial field accessors are
excluded via `.cargo/mutants.toml` (documented there; logic is never excluded). 35 tests total, clippy
pedantic + fmt clean. `proptest` + the Phase-7 e2e abuse suite remain (E86 layers 2–3).

### Test run 2026-07-31 — E0–E4 mutation gate (full-crate re-run)
Re-ran `cargo-mutants` over the full crate after Phase 4 (statements + persistence): **172 mutants → 145
caught / 1 missed / 24 unviable / 2 timeouts**. The single "missed" is an **equivalent mutant** — `replace
* with / in rent_cents`: because `RENT_NUMERATOR == 1`, `byte_days * 1 / D` and `byte_days / 1 / D` are
identical for every input, so no test can distinguish them. Excluded in `.cargo/mutants.toml` with that
rationale (not a coverage gap). **Zero real survivors across E0–E4.** 48 tests; clippy pedantic + fmt clean.
