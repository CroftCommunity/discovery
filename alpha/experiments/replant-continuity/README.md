# replant-continuity — E12.7 (the re-plant's Rung-B governance continuity)

Serves: Part 2 §7.6.2 (the MLS re-plant stamps exactly the governance-fold-derived member set — the membership-continuity keystone, E12.7) — earns/bounds: §7.6.2 `Verified` (membership half) — register: none — landed: 2026-07-13 reconciliation (F2).

The bridge that closes the re-plant loop. Batteries 5–6 characterized the **governance fold**
(`local_storage_projection`); Battery 7 Rung-A (`mls-replant`) characterized the **MLS stamp**.
E12.7 is the join: it proves the member set the fold *derives* is exactly the member set the
re-plant *stamps*, and that this correspondence survives change.

This is the **only** place the two crates meet. The substrate stays openmls-free; `mls-replant`
stays fold-free; this crate depends on both by path (own lockfile). A member has two faces here —
a **governance identity** (a device signer + a principal the fold reasons over) and an **MLS
identity** (a `Persona`) — and the experiments assert the two never drift.

The fold is driven through its **real `DerivedFold::ingest` path** (signatures verified,
credentials resolved, authorization and thresholds enforced) over an in-memory store — not a
short-circuit. The derived member set is read back from the persisted `GroupState`, translated to
principals, and compared against the principals recovered from a fresh MLS stamp's crypto
membership. The two are computed independently: redb state vs. an actual openmls group.

## Experiments (`cargo test -- --nocapture`)

- **E12.7 keystone** (`e12_7_1_stamp_tracks_derivation`) — across genesis + four authorized adds,
  the stamp seats *exactly* the fold's derived set at every step. No stray seat (over-broad
  group), no missing seat (a governed member locked out of the keys).
- **E12.7 removal** (`e12_7_2_removal_propagates`) — an authorized removal drops the member from
  the derived set *and* the fresh stamp's membership. "Removal is real": the re-plant re-keys the
  departed member out, it is not governance theatre.
- **E12.7 unauthorized** (`e12_7_3_unauthorized_no_drift`) — an add authored by a non-member is
  *rejected at ingest*; the derived set does not move and the stamp seats no unauthorized
  principal. The fold — not the MLS layer — is the sole authority on membership.

## Message continuity (E12.2 / E12.7 message facet — RUN-09 Part 3)

The other half of §7.6.2: an in-flight conversation surviving the repoint with **no loss and no
duplication**. The membership half above says *whom* the new group seats; this says the *content*
survives the switch. It lives in `src/dataplane.rs` — the **B1 dataplane hash structure**: a
content-addressed, causally-linked record set whose digest is a pure function of the causally-ordered
content set (byte-identical across arrival orders). Records carry the governance-generation stamp
(§7.6) and their causal antecedent, so a duplicate (a repeated content id) and a drop (a referenced
antecedent that never arrives) are *detected*, not absorbed.

`tests/e12_2_message_continuity.rs` drives it over the real re-plant membership: (a) every
pre-repoint entry is present after, exactly once; (b) in-flight entries land once, in causal order,
on the post-repoint group; (c) both members converge byte-identically across arrival orders; (d) an
injected dup/drop is detected. **`Modeled` at loopback grade** (§7.6.2): delivery is harness-driven,
not real transport, and the record encoding is deliberately *not* `[gates-release]` wire-pinned
(test-only serialization). Real over-the-wire delivery and the pinned encoding remain open.

## Scope

Rung B for *both* continuity halves. Membership is `Verified` end-to-end against a real MLS library
and the real fold; message continuity is `Modeled` at loopback over the B1 hash structure. What is
closed: the crypto membership *and* the conversation content are each a **function of the governance
chain and the dataplane history**, not independent sources of truth — the re-plant loses neither
across a repoint.
