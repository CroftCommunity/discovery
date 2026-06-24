# Croft conformance suite — definition

date: 2026-06-16
status: DESIGN (defines what to build, not the suite itself). Companion to `CROFT-PROTOCOL.md`;
measures a second implementation against that spec.

## Purpose

A protocol is only real when a *second*, independent implementation can interoperate. Today the
behavior lives in Rust/TS spikes + a TEST-PLAN; there is no black-box suite an alternate
implementation must pass. This document defines that suite: the **vectors** (deterministic
input→expected-output pairs) and **must-reject cases** a conformant Croft implementation has to
satisfy, plus the **threat model** the adversarial cases are measured against. It is the contract;
the spec (`CROFT-PROTOCOL.md`) is the prose.

## Threat model it is measured against

A conformant implementation is tested against an adversary who can: (T1) forge or tamper messages in
transit; (T2) replay/reorder/drop; (T3) present a valid signature from a non-member (no standing);
(T4) manufacture a quorum from many of its own devices (one lineage); (T5) inject a Sybil fresh
lineage; (T6) act as a malicious blind sequencer (reorder/drop/inject at the broker); (T7) attempt
membership change against a stale view; (T8) probe a group and observe failures; (T9) deanonymize via
graph topology. The suite does **not** assume a trusted server, a trusted relay, or a trusted clock.

## Vector categories (each: deterministic input → expected output)

1. **Derivations (interop anchor).** Given `lineage_id` / `group_id`, the suite fixes the expected
   `lineage_genesis`, `group_genesis`, and gossip `TopicId` (the §2 pre-images). A conformant impl
   MUST reproduce the exact 32-byte values.
2. **Signed pre-images.** Given `(branch, seq, author, payload)`, the expected `signing_bytes`
   (`"msg-v1" ‖ …`) and a known-good Ed25519 signature + verifying key. MUST verify the good vector and
   MUST reject a one-bit-flipped variant.
3. **Fold-by-lineage.** Given a set of device branches over distinct `device_did`s sharing
   `lineage_id`s, the expected actor view (`fold_by_lineage`). MUST collapse one actor's devices to one
   actor and count lineages, not leaves.
4. **Thresholds count lineages, not leaves (E2.10).** A quorum manufactured from N devices of ONE
   lineage MUST be rejected; the by-DID count is shown unsafe.
5. **Revocation.** Post-revocation, the revoked party's subsequent branches MUST be rejected;
   pre-revocation history MUST be retained. Threshold-authority vectors: a removal op MUST carry
   signatures meeting the stated policy or be rejected.
6. **Reconcile corpus C1–C10.** Each merge scenario → the expected verdict (converge | hard-stop
   contradiction | re-formation fork). A conformant impl MUST hard-stop on contradiction and MUST NOT
   auto-resolve.
7. **Adversarial AR-1…AR-6 (must-reject / must-bound).** Sybil fresh-lineage (AR-1), malicious
   sequencer reorder/drop/inject (AR-2), backfill DoS bounded-rejection-cost (AR-3), metadata-leak
   bound (AR-4), MLS-tree scaling bound (AR-5), replay/double-count (AR-6).
8. **Visibility V1–V9 + S2.** Regime immutable & in-content; no silent crossing; republish distinct;
   depth enforced by verifier; openness caps depth; inward/outward independent; public-membership
   bounded; freeze-by-default. **S2:** a structure-only share MUST be unrepresentable; only
   consented-distance scoped shares are constructible.
9. **Freshness no-false-current (E2.16).** Given (heard-recently?, caught-up?) the expected view-state;
   "current" MUST require both; silence MUST yield "unverified", never "current".

## Must-reject cases (the suite's teeth)

A conformant impl MUST reject (not silently accept or coerce): a broken hash chain; a non-contiguous
branch; a bad signature; **a valid signature from an author with no standing** (the load-bearing
case, T3); a foreign-genesis donor; a removed party's post-revocation branch; a membership op below
threshold; a contradiction (hard-stop, no auto-resolve); an over-depth propagation share; a
structure-only visibility share; an unknown version tag.

## Proposed vector-file layout

```
conformance/
  derivations.json        # id -> expected genesis/topic (cat 1)
  signing.json            # (branch,seq,author,payload) -> signing_bytes + sig + key (cat 2)
  fold.json               # branch sets -> expected actor view (cat 3,4)
  revocation.json         # op sequences -> accepted/rejected + retained history (cat 5)
  reconcile/C1..C10.json  # merge inputs -> verdict (cat 6)
  adversarial/AR1..AR6.json
  visibility/V1..V9.json + S2.json
  freshness.json          # (heard?,caughtUp?,tier) -> view-state (cat 9)
  MANIFEST.json           # suite version, spec version, vector hashes
```

Vectors are language-neutral JSON with byte-arrays hex-encoded. Each file carries the spec section it
exercises and the expected status (`accept`/`reject:<reason>`). A runner harness (per implementation)
feeds inputs through the public API and diffs against expected.

## Honesty boundaries a conformant implementation MUST NOT paper over

These are declared, not hidden: (1) MLS key-distribution must actually run (the faithful path models
the key registry as agreed state — a conformant impl MUST source keys from real MLS, not a fixture);
(2) revoke-**authority** must be a real signature meeting threshold, not a MAC; (3) freshness must run
over real transport with signed beacons; (4) the failed-op response dial must be an explicit
configured choice, not an accidental default. A suite that green-lights an impl while these are
stubbed is mis-scoped.

## Status of each category today (what the suite would be derived from)

| category | source proof | status |
|---|---|---|
| derivations, signing | faithful path, TEST-LOG | green-real |
| fold, thresholds | E2.9/E2.10, MD-G4 | green-real |
| revocation (mechanics) | E2.11, MD-G5 | green-real |
| revoke-authority threshold | T3/F2 + design | design |
| reconcile C1–C10 | corpus | green-real + green-model |
| AR-1…AR-6 | adversarial set | green-real / characterized |
| V1–V9, S2 | lineage-group-model | green-model |
| freshness | E2.16 | green-model |

The suite is buildable today for the green-real/green-model rows; the `design` rows wait on the
honesty-boundary work above.
