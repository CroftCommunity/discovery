# Cooperative metered-storage service — build plan (Rust custom-PDS-like store)

> Phase-plan (three-pass). This is the **build plan**; the lane overview + committed direction live in
> `2026-07-31-coop-storage-metered-hosting-lane.md` (E82). Pass 1 below.

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
the global Rust discipline (rust-enforcer). `rsky-pds` (Blacksky) is a real **Rust + Postgres + S3-blobs**
PDS — ECOSYSTEM §5e already tags it "closest to Croft's stack, build-on," and
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
- **rsky-pds is Rust + Postgres + S3 blobs** (ECOSYSTEM §5e, verified web 2026-06-22; Blacksky/Rudy
  Fraser). Exact crate structure = Phase 0.
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
- **Unverified → Phase 0:** rsky-pds crate structure; the exact atproto PDS endpoint set + blob API
  shapes (`getBlob`/`uploadBlob`, `com.atproto.sync.*`); the boundary shape (S3 vs PDS-API vs both); what
  the history-convergence server requires of the store.
- **[Pass 2 verified] The port oracle is the STANDALONE** (`item-storage-protocol-standalone/`, the
  dependency-free E0–E11 build that ran 81/81 this session), not the full `item-storage-protocol/`. Its
  module names differ from what the per-phase Read-sets below still cite: standalone uses **`item.ts`,
  `receipt.ts`, `statement.ts`, `clock.ts` (not `time.ts`), `rng.ts` (not `prng.ts`), `pricing.ts`** (the
  dial cost), plus `crypto/manifest/ledger/audit/seal/canonical.ts` and experiments `e0_identity.ts …
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
persistence** (needs D5's per-user-SQLite layout) and **Phases 7–8** (need D1/D2/D3/D6). **{7,8} audited:
NOT parallel** — Phase 8 depends on Phase 7's server and shares `server.rs` / `Cargo.toml` (overlapping
write-set). The code spine (1→…→9) stays sequential.

## Phases

### Phase 0: Discovery (network-enabled; Discovery Exemption applies)

**Goal:** Resolve the external unknowns firsthand so the boundary phases are sized on evidence, not
inference. Network is open — fetch real sources.

- [ ] **D1: PDS-in-Rust prior art — internal first, then rsky-pds.** **Probe:** (a) read the in-corpus
  Rust spikes **`experiments/hist-atproto-spike/`** and **`experiments/lexicon-community/`** (atproto /
  history in Rust — internal prior art we already own); then (b) WebFetch `github.com/blacksky-algorithms/rsky`
  (rsky-pds) — how it structures repo storage, blob storage, the S3 backend, and the HTTP/PDS API layer;
  note crate deps (axum/actix? sqlx? aws-sdk-s3/rust-s3? atproto lexicon crate). **Success:** a written
  list of the crates for (HTTP server, S3, DB, atproto lexicon) + how blob put/get maps to S3, and what
  the internal spikes already give us. **Disposition:** throwaway (notes only).
- [ ] **D2: atproto PDS network API surface.** **Probe:** WebFetch the atproto lexicon/spec for
  `com.atproto.sync.*`, `com.atproto.repo.uploadBlob`, `com.atproto.sync.getBlob`, and the "what a PDS
  serves" docs (atproto.com). **Success:** the minimal endpoint set a PDS-like store must serve, and the
  blob upload/get request/response shapes (recorded as confirmed, not "likely"). **Disposition:** throwaway.
- [ ] **D3: resolve the boundary shape.** From D1+D2: is the network boundary S3 put/get, the atproto PDS
  API, or both? **Success:** a decision with rationale (recommend: S3 as the storage+metering plane;
  atproto PDS API as the network-facing layer if in v0 scope). **Disposition:** throwaway → feeds Phases
  7/8 sizing + the Open Question.
- [ ] **D4: history-convergence store requirement.** **Probe:** read `experiments/appview-infra/GROUPS.md`
  (the convergence node), the relay-lab E8/E9 briefs, and `plans/croft-stack/10-drystone-layer.md`
  locally. **Success:** the concrete requirement the content-blind meer places on the store (append-only
  envelope sets, content-blind, addressed how). **Disposition:** throwaway → confirms the "one store, two
  consumers" claim or flags a mismatch.
- [ ] **D5: storage layout (confirm the Q5 resolution) + blob backend.** Confirm the official-PDS
  **per-user SQLite** layout (from D1/D6) and how to co-locate the metering records there — the manifest
  as a single-author repo record, receipts/statements as a co-signed structure alongside — and that the
  **rollup/purge** boundary fits. Pick the local blob backend (Garage/SeaweedFS; FS first). **Success:**
  a confirmed per-user-SQLite storage layout + backend aligned to official-PDS/rsky-pds + croft-stack.
  **Disposition:** throwaway.
- [ ] **D6: official-PDS S3 support + the interface-vs-backend distinction** (from the user's question).
  **Probe:** WebFetch the official `bluesky-social/pds` repo/docs — confirm whether/how it supports an
  **S3 blob backend** (env-var config) vs the corpus's "SQLite + local-FS" note, and separate two
  surfaces: (a) **S3 as a blob backend the PDS writes bytes to** (internal), (b) **an S3-compatible
  interface exposed to clients** (our metering boundary, v0). **Success:** a confirmed statement of what
  the official PDS does for blobs + how our S3-compatible client interface differs from a backend S3
  store. **Disposition:** throwaway → feeds Phase 7 (which "S3" is the boundary).

**Done when:** all BLOCKING open questions below are resolved, Verified Assumptions updated with
firsthand evidence, and Phases 7/8 re-sized if D3 changes their scope (record in Review Log).

---

Implementation phases (1–9) all follow the same shape: **port the proven module TDD (port its assertions
as the RED tests first), rust-enforcer discipline (no `unwrap()` in prod, `Result`/`thiserror`, doc
comments, `clippy::pedantic`), commit at green.** Crate/dir placeholder: `item-store`, a **standalone
crate** (its own `Cargo.toml`; no experiments-wide Rust workspace) under `experiments/` for dev (see
Open Question on repo/IP home). **Port oracle = the dependency-free `item-storage-protocol-standalone/`**
(the 81/81 build); cross-check the full `item-storage-protocol/` for E4/E7–E9 detail if needed. **Preserve
the SPEC's `SEAM:` grep discipline** — every place a mock stands in for real infra gets a `SEAM:` comment
so production gaps stay enumerable by grep.

### Phase 1: Crate skeleton + crypto/identity (port E0)
**Goal:** A Rust crate that generates keypairs, derives stable ids, signs/verifies — E0's "we recognize
you the same way we count you."
**Changes:**
- [ ] `Cargo.toml` + `src/lib.rs` (crate scaffold; **standalone crate**, own `Cargo.toml` — no
  experiments-wide Rust workspace, Pass-2 verified).
- [ ] `src/crypto.rs` — Ed25519 sign/verify + SHA-256 fingerprint, newtype-wrapped keys (Zeroize on
  secret material per rust-enforcer).
- [ ] `src/identity.rs` — deterministic id derivation from pubkey; pin/verify peer keys.
- [ ] `README.md` — the crate's purpose + the cross-ref to `item-storage-protocol`.
**Call chain:** `tests/e0` → `identity::derive`/`crypto::{sign,verify}`.
**Wiring test:** `tests/e0_identity.rs` — a message signed by A verifies under A's pinned key and fails
under B's; id derivation deterministic (ports E0's 4 assertions). RED → GREEN.
**Depends on:** Phase 0 (D5 storage choice informs nothing here; independent).
**Read-set:** `experiments/item-storage-protocol/src/crypto.ts`, `.../e0-identity.ts` (reference).
**Write-set:** `experiments/item-store/{Cargo.toml,src/lib.rs,src/crypto.rs,src/identity.rs,README.md,tests/e0_identity.rs}`.
**Shared-state contract:** no shared mutable state beyond the file write-set; standalone crate, so no
workspace `Cargo.toml` to touch (Pass-2 verified: no experiments-wide Rust workspace).
**Risks:** Rust Ed25519 crate choice (`ed25519-dalek`) vs the TS lib — confirm the same curve/encoding so
signatures are comparable to the oracle.
**Done when:** (1) *Behavioral:* the crate signs/verifies and derives ids matching the TS E0 behavior;
(2) *Verification:* `cargo test -p item-store e0` green.
**Validation:** Narrow — wiring + unit tests sufficient.

### Phase 2: Content-addressed items + signed manifest (port E1–E2)
**Goal:** Items named by fingerprint (tamper-evident) + a customer-signed manifest (the bill's source of
truth). E1–E2.
**Changes:**
- [ ] `src/item.rs` — content-addressed object (fingerprint = SHA-256; `SEAM:` note for CIDv1/DAG-CBOR).
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
**Done when:** (1) items round-trip + tamper detected; manifest root + expected-bytes correct; (2)
`cargo test -p item-store e1 e2` green.
**Validation:** Narrow.

### Phase 3: Transfer receipts — postage metering (port E3)
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
**Write-set:** `experiments/item-store/src/{ledger.rs,receipts.rs}`, `.../tests/e3_receipts.rs`.
**Shared-state contract:** ledger files under the crate's tmp/test dir only.
**Risks:** canonical serialization for signing (must be deterministic) — port `canonical.ts` faithfully.
**Done when:** (1) postage metered by signed receipts, walkaway bounded; (2) `cargo test -p item-store
e3` green.
**Validation:** Narrow.

### Phase 4: Balance-forward statements — the monthly close (port E4)
**Goal:** Co-signed opening→closing statements chained by hash; rent = byte-day integral; disputes bound
to one period — and the **rollup + purge boundary**: a co-signed period lets its granular receipts be
purged while the signed chain preserves provenance. E4 + purge.
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
**Risks:** byte-day integration over a period timeline — port the time model (`time.ts`) exactly.
**Done when:** (1) monthly statements co-signed + chained, rent correct; (2) `cargo test ... e4` green.
**Validation:** Narrow.

### Phase 5: Spot-checks + the audit dial (port E5–E6)
**Goal:** Random-sample audit with detection math `1−(1−f)^k`; a member-chosen, cost-priced dial. E5–E6.
**Changes:**
- [ ] `src/audit.rs` — random k-sample retrieve+fingerprint+verify; the detection-math check; public-
  randomness challenge seeding.
- [ ] `src/dial.rs` — audit tiers, cost linear in audit count, chosen tier as a signed declaration.
- [ ] `tests/e5_audit.rs`, `tests/e6_dial.rs` — port incl. the measured-vs-predicted detection table +
  linear-cost + pro-rate-on-change.
**Call chain:** `tests/e5/e6` → `audit::sample`/`dial::price` → `manifest` + `ledger`.
**Wiring test:** `tests/e5_audit.rs` — measured detection ≈ `1−(1−f)^k` within tolerance; honest provider
passes; cost scales with k, not corpus size. RED → GREEN.
**Depends on:** Phase 4.
**Read-set:** `.../src/audit.ts`, `.../e{5,6}-*.ts`.
**Write-set:** `experiments/item-store/src/{audit.rs,dial.rs}`, `.../tests/e{5,6}_*.rs`.
**Shared-state contract:** seeded RNG only (port `prng.ts` for determinism).
**Risks:** Monte-Carlo tolerance flakiness — seed the RNG (deterministic, per the SPEC).
**Done when:** (1) detection math holds + dial priced at cost; (2) `cargo test ... e5 e6` green.
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
**Read-set:** `.../src/seal.ts`, `.../e{7,8,9}-*.ts`.
**Write-set:** `experiments/item-store/src/{seal.rs,grace.rs}`, `.../tests/e{7,8,9}_*.rs`.
**Shared-state contract:** none beyond write-set (key "destruction" is a mock file delete under tmp).
**Risks:** the seal's fail-closed write path is a `SEAM:` (mock key deletion) — mark it; real
key-destruction is a later spike.
**Done when:** (1) sealed/tombstone verified, grace balances; (2) `cargo test ... e7 e8 e9` green. This
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
  FS first, then S3/Garage) + the boundary byte-count → receipt hook.
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
**Done when:** (1) *Behavioral:* I can `PUT`/`GET` a real object over HTTP and the transfer is metered
with a signed receipt + a recomputable rent; (2) *Verification:* `cargo test -p item-store
wiring_s3_metered` green + a manual `curl PUT/GET` against a locally-run instance.
**Validation:** Moderate — run the server, exercise put/get with `curl`, inspect the ledger.

### Phase 8: Minimal atproto PDS API surface — the thin blob-endpoint layer (in v0, confirmed 2026-07-31)
**Goal:** Serve the minimal atproto PDS endpoints (at least `getBlob`/`uploadBlob`, plus whatever D2/D3
deem the v0 floor) as a **thin blob-endpoint layer on top of the S3-metering plane**, so the store is a
**PDS-like node on the Bluesky network**. **In v0 (user-confirmed): both boundaries from the start** — the
S3-compatible interface (Phase 7) is the storage/metering plane; this phase is the atproto layer over it.
Phase 0 D2/D3/D6 set the exact endpoint set + shapes; the phase is **not gated out**.
**Changes:**
- [ ] `src/pds_api.rs` — the atproto endpoints from D2 (blob upload/get mapped onto `blobstore`; the
  metering hook stays on the byte path).
- [ ] `tests/wiring_pds_blob.rs` — end-to-end: an `uploadBlob`/`getBlob` round-trip over the atproto API,
  metered, returning the atproto-shaped response confirmed in D2.
**Call chain:** atproto `uploadBlob` → `pds_api::upload` → `blobstore::write` + `receipts::ack`.
**Wiring test:** `tests/wiring_pds_blob.rs` — the atproto endpoint reaches the metered blob path. RED → GREEN.
**Depends on:** Phase 7, Phase 0 (D2/D3).
**Read-set:** `src/{server,blobstore,receipts}.rs`; Phase 0 D2/D3 notes.
**Write-set:** `experiments/item-store/src/pds_api.rs`, `.../tests/wiring_pds_blob.rs`.
**Shared-state contract:** same as Phase 7 (ephemeral port, tmp storage).
**Risks:** must match the **verified** atproto shapes from D2 exactly — no guessing (global rule). If the
full PDS surface is large, ship only the blob subset for v0 and mark the rest `SEAM:`/tracked.
**Done when:** (1) an atproto `uploadBlob`/`getBlob` round-trip works and is metered; (2) `cargo test ...
wiring_pds_blob` green + a manual probe against a locally-run instance.
**Validation:** Broad — verify against the atproto shapes; check the response matches the spec.

### Phase 9: croft-stack deploy + VPS smoke test
**Goal:** Deploy the built binary on the VPS via croft-stack as a governed, hardened, isolated service we
can test against.
**Changes:**
- [ ] `croft-stack: services/<name>.toml` — the service manifest (fqdn, port, mode, data profile, limits,
  hardening carve-outs, netns) under the placeholder name.
- [ ] croft-stack role/wiring for the binary (build/ship, systemd unit via the template, telemetry
  poller, hardening baseline, netns isolation).
- [ ] a smoke test / canary hitting the deployed put/get (and blob endpoint if Phase 8 shipped).
**Call chain:** `render.py` → Ansible converge → systemd unit → the running service; canary → HTTP put/get.
**Wiring test:** a deploy smoke test (put/get against the live VPS instance) — the service is reachable,
metered, and governed (telemetry shows it).
**Depends on:** Phase 7 (and Phase 8 if in v0). **This phase writes in the `croft-stack` repo**, not
`discovery` — its own write-set, committed there.
**Read-set:** croft-stack `00-model-and-manifests.md`, `service-hardening-plan.md`, `netns-isolation-plan.md`,
the tenant template.
**Write-set:** `croft-stack/services/<name>.toml`, the role files, the canary config.
**Shared-state contract:** touches the VPS (a real deploy) — governed envelope + netns; this is the one
phase with real external side effects. Coordinate with the estate (ports, DNS/TLS, identity block).
**Risks:** the VPS is production estate — deploy behind the hardening baseline + netns from the first;
confirm no port/identity collision with existing services (relay/broker/telemetry/canary/caddy).
**Done when:** (1) *Behavioral:* the metered store is reachable on the VPS and a put/get round-trip is
metered + telemetered; (2) *Verification:* the smoke test passes against the live instance + telemetry
shows the unit within its envelope.
**Validation:** Broad — real deploy; verify logs, telemetry, hardening (`systemd-analyze security`), and
the put/get path end-to-end in a prod-like setting.

### Phase 10 (tracked, later — gated): history-convergence consumer/meer mode
**Goal:** Wire the content-blind meer mode so the MLS history-convergence server uses this store as its
substrate (one store, two consumers). **Gated on drystone's fold/MLS becoming real** (per
`10-drystone-layer.md`) — likely **out of v0**; tracked, not built now.
**Depends on:** Phases 7–9 + a real MLS group producing real history.
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
