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

## Scope

Rung B for *membership* continuity. E12.7's remaining facet — dataplane *message* continuity
across a boundary (the §7.6.2 atomic repoint of an in-flight conversation) — belongs with E12.2
and Drystone's dataplane hash structures, not this crate. What is closed here: the crypto
membership is a **function of the governance chain**, verified end-to-end against a real MLS
library and the real fold.
