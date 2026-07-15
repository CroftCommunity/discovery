# T1 findings — lineage-proving credential on the MLS leaf

date: 2026-06-16

claim under test: real-library dependency #2 (proof-ledger) / COHESION #7 — can a *signed,
unforgeable* lineage claim ride on the openmls 0.8.1 leaf, readable and verifiable by a second
member, so the presentation fold (E2.9) and lineage-counted thresholds (E2.10) rest on real
library behaviour rather than the TS model's assumption?

decision in force: logical binding (multi-device.md §10.1) — the device key is independent; the
lineage-root key signs `(lineage_id, device_did)`; the optional HD recovery seed is backup-only
and does not enter this format.

test: `crates/lineage-mls/tests/t1_lineage_credential.rs` (4 tests, all green; E1.1–E1.4 still
green — no regression).

## Result: GO

A `LineageClaim { lineage_id, device_did, lineage_sig }` rides on the leaf, is read off another
member's leaf, and verifies against the lineage root's public key from signed data alone. A claim
naming a lineage it was not signed by is rejected. Two devices of one lineage carry the same
`lineage_id` (the fold/threshold key). This discharges dependency #2 for the structured path and
unblocks T2 (multi-device E2.9–E2.16) and T1b/C4.

| test | asserts | result |
|---|---|---|
| `t1_lineage_rides_on_leaf_and_verifies` | claim rides on founder's own leaf; verifies vs root vk | ✅ |
| `t1_other_member_reads_and_verifies_lineage` | a member reads + verifies *another* device's leaf claim; same lineage_id folds | ✅ |
| `t1_forged_lineage_claim_is_rejected` | a claim not signed by the named root fails verification | ✅ |
| `t1_probe_custom_credential_type_wall` | does openmls accept a real custom credential type? (characterization) | see below |

## The spike-both wall probe (the deliverable)

The T1 decision was "spike both, document the wall": try a real custom `Credential` /
`CredentialType::Other` first to find where openmls 0.8.1 stops, then fall back to a structured
`BasicCredential` identity.

**Finding: there is no wall at group founding.** openmls 0.8.1 **accepted**
`Credential::new(CredentialType::Other(0xCA11), bytes)` to build a group
(`MlsGroup::builder().build(...)` returned `Ok`). The crate's documented limitation ("currently
only supports the BasicCredential") and the reinit wall recorded in `lineage-mls/lib.rs:19-26`
do **not** extend to constructing/founding with a custom credential type.

Implication: the **structured-`BasicCredential` identity is a choice, not a forced fallback.** We
shipped the structured path because it needs no custom-type interop assumptions across clients and
keeps the wire format under our control, but a first-class custom credential type remains available
if a future need (e.g. a distinct on-wire `CredentialType` for lineage) justifies it.

## Honesty boundary (what this does NOT yet establish)

- The probe founded a group with a custom credential; it did **not** exercise whether a *second
  member's* client accepts a custom-type leaf through add/commit **validation**. That deeper
  custom-type interop check is deferred — it is not needed for the chosen structured path.
- The structured claim is verified at the application layer (our `LineageClaim::verify`), not by
  MLS itself. MLS treats the identity bytes as opaque; the lineage signature is our governance
  layer's check, consistent with "compose MLS, own governance."
- Real-multimachine replay (verifying the claim crosses the wire and verifies identically on a
  separate box) is a small follow-on, mirroring the A1b / local-first-history pattern.

## Ledger effect

- proof-ledger dependency #2: **CLOSED for the structured path** (leaf carries a verifiable,
  unforgeable lineage claim on real openmls 0.8.1).
- Unblocks T2 (E2.9–E2.16) and T1b/C4 (add-vs-add fold) to move from `spec` toward `green-real`.
