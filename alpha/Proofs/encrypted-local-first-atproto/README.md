# Encrypted Local-First × AT Protocol — A Hypothesis, Its Experiments, and Results

This directory is a research log. It exists to test one architectural bet with
working code on the real Rust crate stack, to record what held and what didn't,
and to leave a full accounting that a decision can be made from.

Each subdirectory is a standalone `cargo run` with a labeled lifecycle, a
pass/fail summary, a version report, and a README of findings. All twelve passed;
the value is in *what passing required* and *what surprised us*.

---

## 1. Central hypothesis

> A private, end-to-end-encrypted, **local-first** group can be built on MLS
> (group keys) + a CRDT (content), and its data can **interoperate with the
> public AT Protocol network** such that the public/private split is a *policy
> and transport boundary, not a data-model fork* — and this composes on the
> *current, real* Rust crate stack, not in principle.

If true: the private group and the public network share one data model; "going
public" is selective publication + auth, not a migration; and the hard problems
(identity, concurrency, deletion) have tractable, demonstrable answers.

## 2. Decomposed hypotheses → verdicts

| # | Claim under test | Phase(s) | Verdict |
|---|------------------|----------|---------|
| H1 | MLS exporter secrets can key durable content (AEAD) and rotate correctly across membership change. | 1 | **Confirmed** |
| H2 | Locally-authored, encrypted, CRDT-synced records are valid atproto lexicon records with no reshaping. | 2, 6 | **Confirmed** |
| H3 | An atproto AppView can be source-agnostic: identical index/serve code for our stack and the public firehose. | 3a, 3b | **Confirmed** (byte-identical modules) |
| H4 | A public/private split can publish selected records while provably never leaking private ones. | 4, 6 | **Confirmed** |
| H5 | The live publish/ingest path (PDS + firehose + AppView) works over real sockets with consistent content IDs. | 5, 6 | **Confirmed** (locally) |
| H6 | Concurrent membership changes can be made safe; concurrent content edits need no coordination. | 7, 10 | **Confirmed — but required a sequencer** |
| H7 | Group identity and public DID can be cryptographically, verifiably bound (with lifecycle). | 8, 11 | **Confirmed** |
| H8 | A record's identity can be stable across the private→public boundary. | 9 | **Confirmed — required pinning rkeys** |
| H9 | Removing a member revokes their access to group content. | 12 | **Refuted as stated → re-scoped** (forward secrecy only; redaction needs re-encryption) |

## 3. Method & discipline

- **Real stack, not mocks.** MLS (`openmls`), CRDT (`automerge`), AEAD
  (`chacha20poly1305`), lexicons + CIDv1 (`cid`/`serde_ipld_dagcbor`), HTTP/WS
  (`axum`/`reqwest`/`tokio-tungstenite`), SQLite (`rusqlite`), signatures
  (`ed25519-dalek`) are all exercised for real.
- **Prove the novel seam; stub the well-understood one.** `iroh` transport is
  stubbed throughout (content addressing is real); it is a swappable seam, not a
  research question.
- **Offline by construction.** The environment's egress allowlist blocks the live
  atproto network and package registries, so the live flip is proven on a local
  PDS. This is a fidelity limit, stated honestly, not a gap in the model.
- **Falsifiable framing.** Each phase prints explicit PASS/FAIL assertions and an
  "issues surfaced" section; a green run is necessary but the findings are the
  output.

## 4. The architecture under test

```
   PRIVATE (encrypted, local-first)                         PUBLIC (atproto)
   ┌───────────────────────────────┐                  ┌───────────────────────┐
   │ MLS group (openmls)           │   mirror +       │ PDS repo (did:plc)     │
   │  ├ per-epoch key (exporter)   │   redaction +    │  ├ createRecord/put    │
   │  ├ AEAD content (chacha20)    │──▶ rkey-pin +  ──▶│  ├ firehose (WS)       │──▶ AppView
   │  ├ Automerge CRDT docs        │   identity       │  └ lexicon records     │   (SQLite index +
   │  └ Willow 4-tuple addressing  │   binding        │                        │    XRPC, hydrated)
   │ transport: iroh (stubbed)     │                  │ identity: did:plc ⇄ did:key (signed binding)
   └───────────────────────────────┘                  └───────────────────────┘
     sequencer totally-orders membership   ·   content stays unordered/P2P (CRDT)
```

Key design decision validated repeatedly: MLS is used **only** for the rotating
group key; durable content is encrypted by us with an AEAD keyed by the MLS
**exporter secret**, and the content itself is a CRDT. This cleanly separates
"who is in the group" (ordered, MLS) from "what the group said" (unordered, CRDT).

## 5. Experiments & results

Each entry: **Hypothesis** tested · **Experiment** run · **Result** · **Finding**.

### Phase 1 — `encrypted-sync-slice` · 5/5
- **Hypothesis (H1):** exporter-derived per-epoch keys match across members and rotate on membership change; CRDT snapshots/deltas survive under AEAD.
- **Experiment:** Alice creates an MLS group, adds Bob, both derive the epoch key independently; Alice writes an Automerge doc, encrypts a snapshot, Bob bootstraps; incremental change syncs back; Carol is added and the key rotates.
- **Result:** independently-derived keys are byte-identical and rotate; snapshot bootstrap + incremental sync + epoch-boundary join all work.
- **Finding:** the exporter-keys-content design is sound. *Surprise:* it forced closing a toolchain gap — see §7.

### Phase 2 — `local-first-lexicon-app` · 6/6
- **Hypothesis (H2):** records authored privately are already valid atproto lexicon data.
- **Experiment:** define real `feed.post`/`feed.reaction` lexicons; author, validate, encrypt, CRDT-sync, reconstruct across an epoch rotation; serialize one record to the exact `com.atproto.repo.createRecord` payload and re-validate.
- **Result:** the round-tripped record is valid lexicon JSON, ready to POST unchanged.
- **Finding:** the public publish path is transport + auth, not a data migration. The atproto repo model (`collection`/`rkey`) maps naturally onto the CRDT doc.

### Phase 3a — `local-appview` · 5/5
- **Hypothesis (H3):** the AppView (ingest/index/serve) need not know its source.
- **Experiment:** define a `RecordSource` trait + Jetstream-shaped `RecordEvent`; feed a `LocalStackSource` over the decrypted doc into a SQLite indexer + axum XRPC server; query a **hydrated** timeline (posts joined with reactions); validate output against a read lexicon; rebuild the index from source.
- **Result:** hydration works, output validates, the index is a disposable projection.
- **Finding:** the indexer/server hold zero references to MLS/CRDT/crypto — the source boundary is clean.

### Phase 3b — `jetstream-appview` · 5/5
- **Hypothesis (H3, sharper):** the *same* AppView consumes the real public firehose format.
- **Experiment:** a `JetstreamSource` parses real Jetstream commit JSON (kind/collection filtering, create/update/delete, cursor/resume); `indexer.rs`/`server.rs`/`views.rs` are copied **byte-identical** from 3a (asserted at runtime via `include_str!`). Content IDs are real CIDv1 (dag-cbor/sha-256).
- **Result:** the swap required zero downstream changes; the gap fields vs. real Jetstream (cid/rev/cursor/kind) are mapped or documented.
- **Finding:** source-agnosticism is real, not aspirational — proven by identical bytes.

### Phase 4 — `public-private-split` · 5/5
- **Hypothesis (H4):** selected records publish; private records never leak.
- **Experiment:** default-deny visibility (out-of-band metadata, since lexicon schemas are closed); a mirror that publishes public records and **redacts** a public reaction whose subject is a *private* post (which would leak the private post's existence); a substring scan asserts no private text/URI/rkey in the projection.
- **Result:** only intended records cross; non-leakage holds; the redaction (referential-integrity) rule fires.
- **Finding:** the boundary is a single auditable choke point. *Surprise:* a public reference to a private record is itself a leak — a real, non-obvious rule.

### Phase 5 — `local-pds-bridge` · 4/4
- **Hypothesis (H5):** the live publish/ingest mechanics work over real sockets.
- **Experiment:** a minimal **real atproto PDS in Rust** (`createSession`, `createRecord`, a WebSocket firehose); a real `reqwest` client publishes; a real `tokio-tungstenite` consumer feeds the byte-identical AppView; CID parity is checked (PDS-assigned CID == locally recomputed).
- **Result:** end-to-end over loopback; CIDs match.
- **Finding:** only the live network (egress-blocked) is stubbed; the data path is real. The official PDS couldn't be used (registry egress blocked), which is *why* a minimal real PDS was built.

### Phase 6 — `end-to-end-slice` · 6/6
- **Hypothesis (composite H2+H4+H5):** the whole chain composes.
- **Experiment:** one program: encrypted MLS group → mirror (with redaction) → **publish to the PDS** → firehose → AppView hydrated timeline; assert end-to-end non-leakage and identity continuity.
- **Result:** all stages compose; private data never reaches the public read path.
- **Finding:** wiring it together surfaced the integration issues the isolated phases hid (identity mapping, AT-URI rewrite, rkey instability) — addressed in 8/9.

### Phase 7 — `concurrent-membership` · 3/3
- **Hypothesis (H6):** concurrency is manageable.
- **Experiment:** two members commit membership changes against the same epoch; observe the fork; resolve via a deterministic tiebreak (loser aborts, applies the winner, re-proposes). Separately, two members make concurrent content edits and exchange changes.
- **Result:** MLS **forks** and openmls rejects a commit from a superseded epoch — a total order is *required*. Automerge content auto-merges with no fork, no lost writes.
- **Finding:** the design rule is *order membership, merge content.* This **refuted the earlier "out of scope" hand-wave**: a sequencer is load-bearing (built in Phase 10).

### Phase 8 — `identity-binding` · 5/5
- **Hypothesis (H7):** group identity and public DID can be verifiably bound.
- **Experiment:** a statement naming both `did:plc` and `did:key` is signed by *both* the account key and the real MLS signing key (raw Ed25519); verification uses only the account's DID-doc key (the group key is embedded in the `did:key`). Forge + tamper attempts are run.
- **Result:** valid binding verifies; forged claim of another's group key is rejected (no MLS private key); tamper rejected.
- **Finding:** bidirectional signing defeats forgery in either direction.

### Phase 9 — `stable-record-identity` · 5/5
- **Hypothesis (H8):** a record's identity can be stable across the boundary.
- **Experiment:** pin the rkey at creation and publish with that exact rkey; verify the public URI is a *pure authority rewrite* of the group URI; test idempotent re-create, conflict rejection, and `putRecord` edits.
- **Result:** rkey honored; mapping is a pure function (no lookup table); idempotent create; edits keep the URI stable while the CID changes.
- **Finding:** **revised an assumption** — Phase 6 needed a lookup table because rkeys were reassigned; pinning rkeys retires it and makes strongRef rewriting deterministic + verifiable (with Phase 8).

### Phase 10 — `membership-sequencer` · 3/3
- **Hypothesis (H6, realized):** an explicit sequencer provides the required total order.
- **Experiment:** a delivery-service `Sequencer` accepts the first commit per epoch and rejects the rest; three members concurrently propose adds; rejected proposers catch up and re-submit; drain the queue.
- **Result:** three concurrent proposals drain in three rounds, all six members converge to one epoch/membership/key — no fork, no starvation; the sequencer holds the canonical order.
- **Finding:** membership ordering needs an owned role; content stays P2P, preserving local-first.

### Phase 11 — `binding-lifecycle` · 6/6
- **Hypothesis (H7, hardened):** bindings can expire, be revoked, and rotate.
- **Experiment:** add a *signed* validity window; an account-signed revocation that supersedes a binding from a date; a key-rotation flow (revoke old, issue new).
- **Result:** in-window valid; expired/not-yet-valid rejected; extending the window breaks the signature; revocation invalidates; rotation works.
- **Finding:** closes Phase 8's gap. *Open:* revocation **discovery** and using a dedicated rotation key (not the everyday account key) are the real-world hard parts.

### Phase 12 — `removal-redaction` · 5/5
- **Hypothesis (H9):** removing a member revokes their access to content.
- **Experiment:** remove a member; test whether they can read post-removal content (forward secrecy) and whether they can still read pre-removal content; then re-encrypt the old content under the new key and re-test.
- **Result:** **H9 is false as stated.** Removal gives forward secrecy (no *future* access) but the ex-member retains old-epoch keys and can still read *old* content. Re-encryption under the new key revokes access to the *stored* copy.
- **Finding (re-scoped):** "remove member" ≠ "redact history." Redaction needs re-encryption, and even that only controls the stored copy — it cannot retract what an ex-member already saw. A "delete from history" feature must say so honestly.

## 6. Confirmed conclusions

1. The encrypted local-first core works on the real stack (H1).
2. The data model is atproto-native; publishing is transport + auth (H2).
3. The AppView is genuinely source-agnostic — byte-identical across our stack and the public firehose (H3).
4. The public/private boundary holds with provable non-leakage and a single auditable choke point (H4).
5. The live publish/ingest path is real over sockets, with CID parity (H5).
6. Concurrency splits cleanly: order membership (sequencer), merge content (CRDT) (H6).
7. Cross-network identity is cryptographically bound with a full lifecycle (H7).
8. Record identity is stable across the boundary once rkeys are pinned (H8).
9. Deletion semantics are understood and correctly bounded (H9, re-scoped).

## 7. Assumptions revised or refuted (the surprises worth recording)

- **"Removing a member revokes access."** Refuted (H9). Forward secrecy ≠ redaction; needs re-encryption, with hard limits. *Product impact.*
- **"MLS handles concurrent membership."** Refuted (H6). MLS forks on concurrent commits; an external total order (sequencer) is mandatory. *Architecture impact: a sequencer role is required, not optional.*
- **"Published records keep their identity."** Revised (H8). PDSes reassign rkeys unless you pin them; pinning is required for stable identity and pure ref-rewriting.
- **"A public reaction to a private post is fine."** Refuted (Phase 4). It leaks the private post's existence; must be redacted.
- **Toolchain reality (vs. the original briefs):**
  - `automerge` 0.7's `AutoCommit::get_missing_deps/get_changes/get_heads` take `&mut self` (the briefs assumed `&self` for one); `get_changes` does return owned `Vec<Change>`.
  - `openmls` 0.8.1 requires its `0.5.x` companion crates; `cargo add` picked stale `0.4.x`, causing a duplicate-`openmls_traits` trait mismatch.
  - `MlsMessageOut/In::into_welcome` are `#[cfg(test-utils)]`-gated; production uses `extract()` → `MlsMessageBodyIn::Welcome`.
  - `rusqlite` 0.40 pulls `libsqlite3-sys` 0.38, whose build script needs the unstable `cfg_select`; pinned to 0.32 (libsqlite3-sys 0.30).
  - `reqwest` with `default-features=false` lacks `RequestBuilder::query`; built query strings manually.
  These are the difference between "works in a tutorial" and "compiles today"; closing the `automerge 0.7`/Rust-1.80 gap was Phase 1's explicit purpose.

## 8. Open design decisions (characterized, not unknowns)

- **Sequencer ownership & trust** (P10): who runs it (superpeer vs. designated/
  rotating member); it can censor/reorder *membership* (never read content);
  needs an explicit, auditable ordering rule.
- **Revocation distribution** (P11): a canonical publish location + freshness
  policy; sign revocations with a dedicated rotation key, as `did:plc` does.
- **"Delete from history" semantics** (P12): re-encryption bounds *future* access
  to the stored copy and cannot undo prior exposure — decide and state the promise.
- **Real-network deployment**: a base-URL + credentials change (the code path is
  unchanged); blocked here only by the environment's egress allowlist. A
  ready-to-run validator (`live-bsky-validate/`) and a resume checklist
  (`live-bsky-validate/RESUME.md`) are committed so this can be picked up in an
  egress-enabled environment.
- **Transport** (`iroh`): a swappable seam, deliberately stubbed; real QUIC/blobs
  over loopback is a focused follow-up touching only the storage layer.

## 9. Threats to validity — what these experiments do NOT prove

- **No live atproto interop.** Everything runs offline against a *minimal* Rust
  PDS, not bsky's PDS. Not proven: real `did:plc` registration/resolution, repo
  MST/CAR construction + commit signing, OAuth, and the production Jetstream
  service. The local PDS proves *mechanics and data shape*, not wire-level
  interop with the live network.
- **No real transport.** `iroh` QUIC/blobs are stubbed; networking-specific
  behavior (NAT traversal, relays, partition/latency) is untested.
- **Targeted, not exhaustive, validation.** The lexicon validator covers the
  constructs our schemas use, not all of Lexicon; the sequencer uses a simple
  first-wins rule; `maxGraphemes` is approximated by scalar count. Scope is noted
  per crate.
- **Single-process simulation.** Members are objects in one process; no Byzantine
  faults, clock skew adversaries, or storage-failure modes are modeled.
- **CID/identity simplifications.** `did:key` uses a reversible hex form (real
  uses multibase/multicodec); `did:plc` is derived from a key rather than a PLC
  genesis op. The *shapes* match; the trust roots are stubbed.

## 10. Reproduce & versions

Each crate: `cd experiments/<crate> && cargo run` (and `cargo test` where
present). Pinned across the suite:

`rustc` 1.94.1 · `automerge` 0.7.4 · `openmls` 0.8.1 (+ rust_crypto 0.5.1) ·
`chacha20poly1305` 0.10.1 · `ed25519-dalek` 2.2.0 · `rusqlite` 0.32 ·
`axum` 0.8.9 · `tokio` 1.52.3 · `reqwest` 0.13.4 · `tokio-tungstenite` 0.29.0 ·
`cid` 0.11.3 · `serde_ipld_dagcbor` 0.6.4 · `serde_json` 1.0.150.
`iroh` 0.98.2 / `iroh-blobs` 0.102.0 resolvable but not linked.

---

### Bottom line

The central hypothesis holds on the real stack: a private, encrypted, local-first
group and the public atproto network can share one data model, with the
public/private split as a policy + transport boundary. The hard problems have
demonstrated answers — identity binding (with lifecycle), a membership sequencer,
stable record identity — and the one refuted assumption (removal = redaction) is
now correctly scoped. What remains is deployment configuration and a set of
named design decisions, not open research risk.
