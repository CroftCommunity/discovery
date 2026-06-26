# Build prompt: the Drystone redb storage-and-projection layer (vetted, adaptable)

> Authored 2026-06-26 from the social-graph-substrate / storage-architecture dialogue
> (`../transcripts/raw/social-graph-substrate-redb-storage-dialogue-2026-06-26.md`). This is a
> **local-implementation** build spec — the redb layer of a Drystone implementation — **not** the
> vendor-neutral Drystone *protocol* (that is `../../../beta/drystone-spec/`). Hand this to a build
> environment with a Rust toolchain. It is self-contained on purpose.

## 0. What you are building, and what you are not

A **Rust crate**: the redb storage-and-projection layer plus its query/command/notification surface, as a
**slot-in component** for an existing stack. It owns durable local truth, the derived projections, the fold
engine, and the public surface. It **does not own** cryptography, MLS, credential issuance/validation,
lamport issuance, blob storage, networking, or UI — those are **injected traits** the component calls but
does not implement. The component is built and proven **in isolation against mock implementations** of those
traits.

**The deliverable is that a set of invariants provably hold.** The code is what makes them hold; the tests
are what prove it. A complete-looking crate whose tests only exercise the happy path is a failure of this
prompt. Lead with the invariants (§2); treat the test architecture (§9) as a first-class, per-stage
deliverable, not a final afterthought.

## 1. Layering context (where this sits)

```
  peer protocol            ← Drystone wire spec (NOT this crate)
  ecosystem tech stack     ← iroh, automerge, mls-rs, redb, BLAKE3 (deps; redb is bound here)
  local tech stack         ← THIS CRATE: assertion store + fold + projection + surface
  platform presentation    ← UI/UX (NOT this crate; builds against the surface)
```

The crate's value is the two interfaces it exposes: the **injected-trait boundary** below/beside it (so it
slots in) and the **query/command/notification surface** above it (so the UI builds on it). Get both right
and the component drops in and the UI iterates freely.

## 2. Invariants (the specification — the build serves these)

- **I1 — Single writer.** The fold engine is the *only* writer. Every mutation flows through one validated
  path. Queries are read-only.
- **I2 — Atomic apply.** Appending an authoritative assertion and applying its effects to the derived tables
  happen in **one redb write transaction**. The index can never partially reflect an assertion.
- **I3 — Never blind trust.** An assertion (local or peer-sourced) earns a write only after local validation:
  signature verified (injected `Verifier`), device→principal credential validated (injected
  `CredentialResolver`), and **authorization checked against the rules in force at the assertion's position
  in governance order**. Validation failure → rejected, no write.
- **I4 — Authoritative vs derived.** `auth_*` tables are the source of truth (durable, append-mostly, pruned
  only by compaction). `idx_*`/`state_*` tables are a **pure, rebuildable function** of `auth_*`. No derived
  state without authoritative justification; no governance assertion without derived reflection.
- **I5 — Order-insensitive convergence.** Applying a causally-consistent set of assertions in any valid order
  yields **byte-identical** derived tables. (Edges may reference not-yet-known nodes; cards fill lazily.)
- **I6 — Rebuildability.** Dropping all `idx_*`/`state_*` tables and re-folding `auth_*` in causal order
  reproduces them exactly. This is the corruption-recovery path and the iteration superpower; it must be a
  fast, tested, first-class operation.
- **I7 — Snapshot is a cache.** `state_group` (the declarative snapshot) is valid **iff** its
  `computed_at_gov_head` equals the group's current `auth_gov_log` head; it is never synced and never trusted
  from a peer. On mismatch, re-fold the tail.
- **I8 — Fork is legitimate.** Disagreement with a validly-ratified governance change produces a divergent
  `state_group` lineage from a common ancestor — both internally valid. Concurrent same-epoch governance is a
  detectable fork. Forks are first-class outcomes, not errors.
- **I9 — Partial knowledge is first-class.** A reference to a node whose content you don't hold is a normal
  state (held / known-unheld-wanted / known-unheld-uninterested / unavailable), never corruption. Stub cards
  carry the kind (from the typed id).
- **I10 — Provenance, not verdicts.** Trust signals (vouches) are graded, contextual, surfaced as signals;
  the surface exposes **no computed trust verdict**. Authorization is a deterministic local check; trust is a
  signal for a human/policy.

## 3. The frozen core (changing these breaks compatibility or forces migration)

### 3.1 Typed ids and the hash/id split

- A **node id** is `kind_tag (fixed width, spec-defined) ‖ hash (32 bytes, BLAKE3)`. Kind is self-describing
  in every reference.
- The **raw 32-byte hash** is the *content-address* (keys `auth_assertions`, blob refs). The **typed id** is
  the *graph reference* (keys `idx_edges`/`idx_nodes`). `typed_id = tag ++ hash`; `hash = typed_id[tag_len..]`.
  Hold this split consistently or you get lookup misses.
- Kind tag is assigned **deterministically from the creating assertion's semantics** (so peers agree). The
  **kind-tag table is frozen-but-additive** (reserve headroom; new kinds = new tag values). Initial table:
  `0x01 Group · 0x02 Principal · 0x03 Device · 0x04 ArtifactChat · 0x05 ArtifactNote · 0x06 ArtifactLink ·
  0x07 ArtifactGame · …`.

### 3.2 The assertion envelope (frozen; wire-and-storage)

`{ version: u8, assertion_type: u16, author_device: DeviceId, author_principal: PrincipalId,
group: GroupId, antecedents: Vec<Hash>, lamport: u64, timestamp: u64, payload: Vec<u8>, signature: Vec<u8> }`

- `version` leads (evolve encoding additively). `assertion_type` is `u16` (additive headroom).
- **Per-device authorship:** `author_device` is the signing identity (signature verifies against it);
  `author_principal` is the user-principal it is credentialed to. Signature is by the device key; the
  principal binding is established by the credential chain.
- `timestamp` is **advisory only** — never used for ordering or authorization (wall clocks lie). `lamport` is
  **per-device** and is the ordering input.
- The row hash = BLAKE3 over the canonical serialization including the signature. Storing is **idempotent**
  by hash.
- `payload` is type-specific and **internally version-tagged**.

### 3.3 Other frozen items

- **Crypto/serialization:** BLAKE3 hashing; one canonical serialization for hashing/signing. Fixed.
- **Value-version tag:** every authoritative value carries a leading version byte (additive payload
  evolution, no hard migration).
- **Authoritative table key shapes** (§4) are frozen; values tolerate versioning.
- The **fold knows the concrete assertion types** and their semantics (concrete, not generic over an injected
  semantics trait). Adding a type later is a localized, test-guarded change.

## 4. redb tables — one file, two families

> redb keys are ordered byte strings; range scans walk lexicographic byte order. Every composite key is a
> concatenation of **fixed-width big-endian** fields; the field you range-scan over goes **last**.
> **Edge-table representation is an explicit build-time exploration:** default to **composite-key plain
> tables** (below); measure against **multimap tables** (value must be orderable, which fights structured
> `EdgeMeta`); switch only if measurement justifies it — edge tables are derived/rebuildable, so switching
> later is cheap and safe.

**Authoritative (`auth_`, durable, append-mostly):**

- `auth_assertions` — key `hash(32)` → value `versioned envelope`. The flattened Merkle DAG; idempotent.
- `auth_assertions_by_device` — key `DeviceId(32) ‖ lamport(8 BE)` → `hash(32)`. Per-device causal stream for
  range-sync. (Classed `auth_` for durable sync use though technically rebuildable — a stated judgment call.)
- `auth_gov_log` — key `GroupId(32) ‖ gov_seq(8 BE)` → `hash(32)`. The **never-compacted governance spine**.
- `auth_artifacts` — key `hash(32)` → `versioned { kind, inline_bytes | blob_hash, metadata }`. Small content
  inline; large content as a blob hash (bytes live in iroh-blobs, never in redb).
- `auth_genesis` — key `GroupId(32)` → `versioned { policy_version, initial_rules (incl. initial amendment
  threshold + operational thresholds), governance_retention = Permanent, content_compaction { trigger_depth,
  trigger_age_secs, cadence }, retention_courtesy, trust_character, founding_assertion: Hash }`. Genesis fixes
  the cryptographic root + **initial** covenant values only.

**Derived (`idx_`/`state_`, rebuildable; shapes free to change):**

- `idx_edges_out` — key `source_typed_id ‖ edge_type(2 BE) ‖ target_typed_id` → `versioned EdgeMeta
  { since_lamport, since_assertion: Hash, present: bool, attributes }`.
- `idx_edges_in` — key `target_typed_id ‖ edge_type(2 BE) ‖ source_typed_id` → same `EdgeMeta`. **Written in
  the same transaction as the forward edge** (I2).
- `idx_nodes` — key `typed_id` → `versioned NodeCard { kind, present: bool, title, created_by, created_at,
  summary_fields, blob_hash: Option<Hash> }`. Stub cards (`present:false`, kind from the typed id) for
  not-yet-known targets.
- `state_group` — key `GroupId(32)` → `versioned GroupState { computed_at_gov_head: Hash, computed_at_gov_seq,
  members: [(PrincipalId, Role, since)], rules (ALL covenants incl. the mutable amendment threshold),
  epoch_ref, fork_status: Clean | ForkedFrom(Hash) }`. Cache; valid per I7.
- `state_checkpoints` — key `GroupId(32) ‖ ckpt_seq(8 BE)` → `versioned Checkpoint { covers_up_to_gov_head:
  Hash, content_merkle_root: Hash, created_at, signers, signatures }`. **Built-but-off by default.**

Edge types (initial, additive): `MEMBER_OF`, `ATTACHED_TO`, `SPLIT_FROM`, `RECOMBINED_FROM`, `REPLIES_TO`,
`VOUCHES_FOR`, plus the **composition** vs **valuation** distinction carried as an edge-type attribute or
distinct types (composition = shared MLS lineage; valuation = weighted trust, no shared keys).

## 5. Governance model (mutable covenants, fork-on-disagreement)

- **All covenants live in folded `state_group.rules` and are mutable** — including the **amendment threshold
  itself**, which is just the strictest covenant. A rule-change assertion is validated against **that rule's
  value at its position in governance order** (the fold applies governance in order, checking each change
  against the rules as they stood just before it). There is **no immutable amendment field**; genesis fixes
  only the cryptographic root + initial covenant values.
- **Disagreement with a validly-ratified change is a fork**, never a veto. A rogue change (not meeting the
  current threshold) is invalid and rejected. "Identical compaction/state across peers" holds **per-lineage**;
  cross-lineage divergence is definitionally a fork (I8).

## 6. The fold engine (sole writer)

For each assertion, in **one write transaction**: (1) verify signature (`Verifier`) + validate device→
principal credential (`CredentialResolver`); (2) validate authorization against current `state_group.rules`
(rules-at-position during replay) — reject with no write on failure; (3) append to `auth_assertions` +
`auth_assertions_by_device`, and if governance, to `auth_gov_log` advancing `gov_seq`; (4) apply effects —
both edge directions, upsert cards (stub `present:false` for unknown typed-ids, kind from the id; validate
kind-expectations from the id, recording expected-kind on stubs for deferred checking when the target
arrives), re-fold the affected `state_group`; (5) commit atomically. The fold knows the concrete types
(§3.3). Guarantees: I5 (order-insensitive convergence), I6 (rebuildable), I4 (authoritative-justified).

## 7. Injected traits (the slot-in boundary — define at Stage 0, before storage code)

- `Verifier` / `Signer` — verify a signature against a device key; sign bytes. The component holds no keys.
- `CredentialResolver` — does this device credential validly bind to this user-principal (the
  user-principal-as-self-AS scheme lives in the host stack's MLS/credential layer).
- `LamportSource` / device-identity — issues per-device lamport values and the device identity; the component
  validates monotonicity within a device stream but does not mint values.
- `BlobPresence` — "do we hold blob X"; the component stores blob hashes + a `present` flag and **emits
  fetch-requests as outputs**, never doing network I/O.

The crate must compile and pass its full suite against **mocks** of these, with **no concrete crypto/MLS/
network dependency** — that is the mechanical proof it will drop into the host stack.

## 8. Public surface (the proven, adaptable contract — UI never touches redb)

- **Queries** (read-only, sync, return **view-models** not storage types): `get_group_summary`,
  `list_my_groups`, `list_group_attachments` (cards incl. `present:bool`), `get_timeline(group, window)`
  (`window = LastN | Since | Range | Around`; widening = backfill request), `get_trust_signals(subject,
  context)` (**graded signals, never a verdict** — no `trusted:bool` field, I10), `get_principal`,
  `get_node_card`, `get_fetch_state` (`NotRequested | Requested | Unavailable | Available`).
- **Commands** (produce assertions via the fold; async where they touch injected network/MLS-adjacent traits;
  return **honest `CommandResult` enums** incl. `Applied | PendingSignatures { have, need } | Rejected
  { reason }` and fork outcomes): `create_group`, `add_member`, `remove_member` (result communicates
  "removed going forward" honestly — does **not** imply retroactive erasure), `add_device` (returns the
  scan-to-add enrollment payload), `send_message`, `attach(group, kind, params)` (siblings of chat, symmetric
  across kinds), `vouch(subject, context, strength)` (**context + strength mandatory** — the data model
  enforces the trust philosophy), `request_fetch`, `request_rejoin(group, evidence) -> RejoinOutcome
  (RecognizedReturner | TreatedAsNew | PendingSocialDecision | ReadOnlyGranted)`.
- **Change-notification stream** — emits on derived-state changes (timeline changed, group state changed,
  fetch state changed) for UI reactivity. Without it the UI polls or goes stale.

View-models are **purpose-built for display**, decoupled from storage types (absorb change from both sides).
The interface depends on the storage layer only via: queries read `state_`/`idx_`/`auth_artifacts` and
assemble view-models; commands construct envelopes and hand them to the fold (the sole writer) and translate
its validation result into a `CommandResult`. Three independent axes of change: derived-table shape (re-fold),
assertion types (additive + a command), view-models (query assembly + UI). Decisions to make at build:
sync-query / async-command concurrency model; where view-model assembly lives (the interface layer, fluid).

## 9. Build stages (one delivery, sequenced so proof accumulates)

- **Stage 0 — trait boundary + mocks.** Define §7 traits and mock impls. Nothing below depends on concrete
  crypto/MLS/network. *Proof: crate compiles against mocks.*
- **Stage 1 — frozen core types.** Envelope, typed ids + kind-tag table, canonical serialization + hashing,
  value-version discipline. *Tests: round-trip serialization, hash idempotence, typed-id/hash split.*
- **Stage 2 — authoritative tables + the fold's write path.** `auth_*` tables, signature/credential/auth
  validation, atomic append. *Tests: I2 atomicity, I3 reject-on-invalid (bad sig, failed authz, wrong-kind
  ref, dangling antecedent accepted-as-deferred, lamport collision, duplicate idempotent).*
- **Stage 3 — derived projection + the fold's effect path.** `idx_*`/`state_*`, both edge directions, cards,
  declarative snapshot. *Tests: I5 order-insensitive convergence (property), I6 rebuildability (property),
  I4 authoritative-vs-derived consistency (property), I9 partial-knowledge four states.*
- **Stage 4 — governance + forks + compaction.** Rules-at-position validation, mutable amendment threshold,
  fork lineages, content compaction + checkpoints + rebuild-after-compaction. *Tests: I8 fork (contested
  covenant → divergent valid lineages; concurrent same-epoch detectable; deterministic tiebreak identical
  across instances), compaction prunes content-not-governance + correct Merkle root + dormant-catch-up rebuild,
  per-lineage-identical pruning.*
- **Stage 5 — public surface.** Queries, commands, notification stream. *Tests: interface-contract (queries
  consistent with state; commands → right assertions + right `CommandResult` incl. non-happy variants;
  notifications fire correctly), I10 (get_trust_signals exposes no verdict; vouch requires context+strength).*
- **Stage 6 — scale/diversity + mutation pass.** *Tests: large diverse logs (many groups/principals/devices/
  kinds, deep lineages, forks, compaction) hold all invariants + bounded performance (fold throughput, query
  latency, rebuild time); **cargo-mutants with a high kill rate on the fold + validation modules** (a silent
  bug there corrupts every projection or admits unauthorized assertions).*

## 10. Test architecture (first-class; what makes the surface proven)

- **Property-based (proptest):** I5, I6, I4 over arbitrary valid logs. **Generators must produce genuinely
  diverse, forked, partially-known DAGs — trivial generators prove nothing** and are the most common way a
  property suite gives false confidence.
- **Mutation testing (cargo-mutants):** systematically introduce bugs; confirm tests catch them. High kill
  rate **required** on fold + validation. This tests the tests; it is what justifies trust in a foundation
  others adapt.
- **Adversarial/malformed-input:** every rejection path (I3) explicitly tested.
- **Fork / partial-knowledge / compaction / interface-contract / scale** as per the stages.

## 11. Documentation deliverable

The **frozen-vs-fluid boundary** explicitly documented (safe-to-change-and-re-fold vs breaks-compatibility),
the **injected-trait contracts**, and the **key-encoding byte layouts**, so future adaptation is guided.

## Definition of done

The crate compiles and passes the full suite against mocks with **no concrete crypto/MLS/network dependency**;
every invariant I1–I10 has a passing property or contract test; cargo-mutants reports a high kill rate on
fold + validation with the survivor list reviewed; the edge-table representation decision is **measured**, not
assumed; and the frozen/fluid boundary + trait contracts + key layouts are documented.

> **Reviewer note (carry to the build's return):** "vetted" is won or lost in the property-test **generators**
> (are they diverse/forked/partial enough) and the **mutation-survivor list** (what bugs the suite fails to
> catch). Review those specifically.
