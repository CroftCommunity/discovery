# Cooperative metered storage — lane kickoff + v0 sketch

date: 2026-07-31

status: **lane opened; build committed (user, 2026-07-31) as an experimental product** — a Rust
custom-PDS-like metered storage service with an S3-compatible interface, deployed on the VPS via
croft-stack. The v0 sketch below is the starting point; the grounding + phase-plan come next.

lane: cooperative layer (the D5 sustainability *mechanism*). Backlog: ROADMAP_TODO **E82**; ties **E25**/**D5**.

related: `thinking/cooperative-social-union-model.md` (the charging mechanism), the proven experiment
`experiments/item-storage-protocol/` (E0–E14) + `experiments/item-storage-protocol-standalone/` (E0–E11),
`crystallized/principles.md` ("meter the boundary, not the machine").

---

## Problem statement

The co-op's flagship non-extractive service is **PDS-shaped metered storage**. The *protocol* is already
proven in code — the item-storage suite is **green-real** (the standalone runs **81/81 assertions across
E0–E11**, each printing its plain-language sentence). But it is an **in-process, deterministic
experiment** with `SEAM:` markers everywhere real infrastructure would go (hex hashes for CIDs,
in-process actors for the network, deleted files for destroyed keys).

To (a) **see it in action for real** and (b) **use it for ourselves as a starting point** (dogfood), we
need a minimal usable **v0**: a real object store, metered at the boundary, with bilateral signed
receipts and a monthly balance-forward statement we can verify from our own signed manifest. This lane
carries the walk from experiment → usable service.

## Committed direction (2026-07-31, user)

Build this as an **experimental product**: a **network-accessible, custom PDS-like storage service with
metering built in and an S3-compatible interface**, in **Rust**, extrapolating from existing custom-PDS
implementations, deployed on the VPS via **croft-stack** as a service we can test against.

- **Prior art to extrapolate from** (ECOSYSTEM §5e; `research/atproto-private-data-architecture.md`
  already recommends this path): **`rsky-pds`** (Blacksky / Rudy Fraser) — an alternative PDS in **Rust**
  with **Postgres + S3 blobs** — is closest to Croft's stack (build-on). Secondary: **Cocoon** (Go +
  Postgres, for the Postgres lesson) and the **official `@atproto/pds`** (TS, single-tenant SQLite) for
  the reference API surface. Blob backend: R2 / B2 / **Garage or SeaweedFS** — **not** MinIO (community
  edition archived Feb 2026). atproto **decouples identity from host** (CAR repo export/import), so the
  store can front a repo without owning the identity.
- **It doubles as the history-convergence substrate (the tie the user flagged).** The corpus's **MLS
  history-convergence server** — a *content-blind meer* that converges append-only MLS history without
  reading content (`plans/croft-stack/10-drystone-layer.md`; relay-lab E8/E9) — **needs exactly a
  metered, network-accessible, content-blind PDS-like store.** So this one service is the shared
  substrate under **both** (a) PDS blob hosting and (b) the history-convergence / meer role — a real
  unification, not two builds. The content-blind posture and the two-plane split already fit the meer's
  requirement.
- **Open design question (genuine, the user's):** how the service interacts on the Bluesky network — an
  **S3-type put/get interface** for the blob plane vs. the **atproto PDS API surface** a PDS speaks
  (`com.atproto.sync.*`, `com.atproto.repo.*`, `getBlob`/`uploadBlob`, relay/firehose). Likely **both**:
  an S3-compatible interface as the storage + metering boundary, behind the atproto PDS API the network
  expects. **Resolve by studying `rsky-pds` + the atproto PDS spec — do not guess the API shapes**
  (global rule: verify external APIs against a source of truth, never stub "likely follows this pattern").
- **Deploy via croft-stack on the VPS.** A new croft-stack service/tenant — the hardening baseline +
  netns isolation + telemetry all apply — sitting alongside / feeding **Phase 10** (`10-drystone-layer.md`:
  croft-groups + the convergence server). Serverless-floor ethos honored (the store stays optional /
  self-hostable).

## Approach

### The lane

A tracked workstream under the cooperative layer, homed in this doc (ROADMAP_TODO **E82**). Naming: the
co-op and its storage service are **unnamed** (A21) — "Drystone" is the P2P protocol, not this.

### v0 — the smallest useful, dogfoodable slice

**Definition of done for v0:** *"I can point a real object store at it, it meters my bytes with bilateral
signed receipts, and it produces a monthly balance-forward statement I can verify from my own signed
manifest."* Keep to the **E0–E9 core**; defer E10 (erasure) and E11–E14 (financing / funder-diligence).

```
  [ me — client CLI ]                              [ co-op — provider service ]
    own Ed25519 key                                  own Ed25519 key
    signed manifest        ──── put / get bytes ───▶ thin HTTP boundary
    (repo + blob CIDs)     ◀─── signed receipt ─────  over a real S3-compatible
    local ledger (JSONL)                              blobstore (MinIO → real bucket)
          │                                                  │
          └────── monthly: co-sign statement ◀───────────────┘
                  opening root + Σ receipts + byte-days = closing root; both sign
    verify script: recompute rent from my own manifest; check the receipt + statement chain
```

**SEAMs to close, in order** (each a small spike with its own verify step — this is where modeled ≠ real):
1. **The network boundary** — replace in-process actors with a thin HTTP `put`/`get` + receipt exchange.
   The boundary *is* the whole point ("meter the boundary"); keep it minimal and boundary-observable.
2. **A real blobstore** — back the provider with S3-compatible storage (**Garage or SeaweedFS** locally
   — MinIO's community edition was archived Feb 2026 — then R2/B2). Bytes-transferred (postage) and
   byte-days-at-rest (rent) measured at that boundary.
3. **Real CIDs** — swap hex SHA-256 for CIDv1 / DAG-CBOR so it is atproto/PDS-shaped (the experiment's
   one deliberate `SEAM:` simplification).
4. **Everything else from E0–E9 stays verbatim** — identity, manifest, receipts, statements, the
   audit dial, the seal — those are already real crypto + exact arithmetic; no rewrite.

**Explicitly out of scope for v0:** the tombstone key-destruction ceremony beyond the mock; E10 erasure
coding; E11 royalty + E12–E14 funder-diligence (that is the *capital* layer — a later slice, and its
transparency commitment is an unresolved decision).

**Dogfood target:** meter our own PDS blob storage (or a scratch bucket) for a month, produce a
statement, verify it. That is the "starting point" we can then show and build on.

## Reasoning

- **Dogfood-first is the honest path for a non-extractive service.** If we won't run our own metered
  storage and verify our own bill, we shouldn't ask members to. It also stress-tests the "meter the
  boundary" claim against real bytes, not seeded ones.
- **The boundary is the only hard part — close it first.** The experiment already discharged the
  ledger/crypto; the residual risk is entirely in the SEAMs (network, blobstore, CIDs). Closing them in
  that order yields something usable at each step, not a big-bang.
- **Keep v0 to the E0–E9 core** so we watch the *service* work before layering the *capital* machinery
  (E11–E14), whose transparency commitment is still an open call.
- **Ties D5.** This is the charging layer of the cooperative mechanism, made real and self-used.
  Legal-review (receipts-as-contract) stays deferred on the coop-layer pile until there is a running
  thing to review.

## Open items

- **[naming]** the co-op + storage-service name (A21) — needed before any public/repo naming; not a v0
  blocker (v0 can run under a placeholder).
- **[decision, deferred]** the D5 legal-review gate; the E11–E14 funder-diligence *and* its
  publish-co-attested-ledgers transparency commitment (revisit "once we have it in hand").
- **[risk]** the SEAM closures (real network / blobstore / CIDs) are where modeled ≠ real; each gets its
  own verify step so v0 never over-claims.
- **[scope]** where the v0 code lives / IP ownership — the experiment sits in
  `discovery/alpha/experiments/`; v0 growing into a real service touches the same IP/ownership question
  the app Phase-0 raised (surface, don't resolve).

## Next step (build sequence — grounding before code)

> **Build plan (phase-plan, Pass 1 done):** `2026-07-31-1-plan-coop-metered-storage-service.md` —
> Phase 0 discovery (rsky-pds + atproto surface, network-fetched) → E0–E9 Rust port (Phases 1–6) →
> S3-compatible metered boundary (Phase 7) → atproto PDS surface (Phase 8, Phase-0-gated) → croft-stack
> VPS deploy (Phase 9) → history-convergence consumer (Phase 10, gated/later).

Because this extrapolates from a real implementation and speaks a real network API, we ground and plan
before writing service code (global rules: verify APIs against a source of truth; TDD; phase-plan for
complex changes; rust-enforcer discipline).

1. **Prior-art / design-grounding pass (no guessing).** Study `rsky-pds`'s crate structure (how it does
   repo + blob + S3 + Postgres) and the atproto **PDS network API surface** (`com.atproto.sync.*` /
   `repo.*` / `getBlob` / `uploadBlob`, relay/firehose), and **resolve the S3-interface ↔ atproto-API
   question.** Output: a short grounding note in `research/` with confirmed shapes (not "likely").
2. **Phase-plan the Rust service** (`phase-plan` skill): the E0–E9 metering ledger ported to Rust behind
   an S3-compatible blob boundary + the atproto PDS API surface; the content-blind posture for the meer
   role; storage for the ledger (redb/Postgres). Problem/Approach/Reasoning; TDD ordering.
3. **Build the first vertical slice TDD** (recommend the metered blob put/get + signed receipt — the
   boundary), then wire a **croft-stack** service (tenant manifest, hardening, netns, telemetry) for VPS
   deploy.

Confirm the sequence, and tell me whether web/repo access is available in this environment for the
grounding pass (the `rsky-pds` repo + atproto docs), or whether you'll drop those sources in locally.
