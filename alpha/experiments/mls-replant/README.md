# mls-replant — Battery 7 (the re-plant, Rung A)

The **re-plant** primitive against a real MLS library (openmls 0.8.1, the version the
`mls-welcome-over-iroh` spike pinned). The whole fork/heal/re-key story reduces to one
operation: read the member set from the governance chain, **stamp a fresh MLS group over
it**, atomically repoint the conversation (§7.6.2). This crate implements the stamp and the
E12 experiments that probe it.

Standalone crate (own lockfile). It does **not** depend on the `Proofs/lineage-groups`
`lineage-mls` wrapper, which is absent in this workspace — the E12 mechanics are pure
openmls. The governance chain is out of scope here (that is Rung B, the
`local_storage_projection` fold); the member set is modelled as a list of personae.

## Experiments (`cargo test -- --nocapture`)

- **E12.1** (`e12_1_baseline_cost`) — baseline stamp cost is O(N). Measures Commit/Welcome
  bytes and wall time at N = 25…500; per-member Welcome bytes stays ~flat (≈152–155 B/mbr),
  confirming linearity. Feeds Battery 2 / M1.
- **E12.3** (`e12_3_dedup_not_fork`) — the core correctness claim: two members stamping
  independently from the same member set produce different tree bytes but identical
  membership — a dedup resolved by the content-hash tiebreak, never a fork.
- **E12.4** (`e12_4_drift_reset`) — a fresh stamp resets the re-key drift a tree accumulates
  through (interior) removals. Byte-size proxy; the direction (fresh < evolved) corroborates,
  the magnitude understates (openmls serializes blanks compactly; no resolution/blank count).
- **E12.5** (`e12_5_leaf_rotation`) — a fresh stamp rotates every member's leaf encryption
  key at once while preserving each persona's identity (signature key) — a group-wide re-key.
- **E12.6** (`e12_6_last_resort`) — availability is bounded by the last-resort package: the
  swap never blocks. A member seated via a reused last-resort package does not rotate (the
  E12.5 exception); a fresh-package member does.

## Battery 2 / M1 — per-commit cost band (Rung A)

The per-commit cost is a **band**, not a point. Which end a hot group sits at is decided by how
many *distinct* members commit — concentrate re-keying on one member and you pay the O(N) floor;
spread it and you settle toward the O(log N) ceiling. Tune the §11.11 liveness window for the floor.

- **M1 floor** (`m1_per_commit_cost`) — **refutation of the log-cheap assumption:** a lone
  member's self-update commit is **O(N)** (~80–130 B/member, flat across N and across repeated
  commits) — the co-path resolves over the blank sibling subtrees a bulk-add stamp leaves. It
  is a re-key/membership cost, not per-message (messages are O(1)).
- **M1 ceiling** (`m1_populated_tree`) — the honest close: once every member has committed once
  (a full round-robin populates the tree interior), a subsequent commit is **O(log N)** —
  per-member bytes *fall* as N grows (measured 90→52→30 B/mbr at N=8/16/32, vs sparse
  131→109→97). At N=32 a populated commit is ~3× cheaper per member than a sparse one.

So a lone active member re-keying a large hot tree pays the O(N) floor, raising the §5.9
exit-affordability floor more than the optimistic reading assumed; a group with activity spread
across members approaches the O(log N) ceiling. The fan-out half needs the iroh gossip testbed.

## E12.7 — done in the bridge crate

Rung-B governance continuity (the crypto membership is a *function of* the governance chain)
lives in the sibling crate `../replant-continuity/`, the only place this crate and the
`local_storage_projection` fold meet. It drives the real fold ingest path and asserts a fresh
stamp seats exactly the fold's derived member set across adds, removals, and rejected
unauthorized changes.

## Not yet done (E12.2, M1 fan-out, M2)

Atomic-swap *message* continuity (E12.2 — its continuity half is Rung B, Drystone's dataplane
hash structures, not this crate), M1's fan-out half, and M2 (return-backfill vs dormancy) need
the iroh gossip testbed. Five of the seven E12 items (the Rung-A MLS mechanics) plus the full
M1 per-commit cost band are done here; E12.7 is done next door.
