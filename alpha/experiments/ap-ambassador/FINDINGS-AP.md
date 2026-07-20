# FINDINGS-AP — RUN-AP-01

`Findings from the ap-ambassador receipt lane run (2026-07-20). Numbered
F-AP-1..n. FIX = corrected in-run. FINDING = surfaced, tagged, recorded.
DECIDED = owner-settled input (tag from the RUN-AP-01 brief §1); not a
finding, listed for cross-reference only.`

## DECIDED (owner-settled inputs, cross-reference — no finding to file)

- **AP-V1 (register)** — DECIDED (RUN-AP-01). No pre-registration of
  `ap_signed_follow` in the qualifying-antecedent register. See
  `AP-AMBASSADOR.md §1`.
- **AP-V2 (record composition)** — DECIDED (RUN-AP-01). Evidence-complete
  with posture-conditional blinded form; lean-projection rider (posture
  language only this run). See `AP-AMBASSADOR.md §1`.
- **AP-V3 (undo/delete)** — DECIDED (RUN-AP-01). Undo = second receipt;
  Delete = redact body + keep skeleton (masked never-was-world equality).
  See `AP-AMBASSADOR.md §1`.
- **AP-V4 (governance)** — DECIDED (RUN-AP-01). Hard exclusion as role
  boundary, structural + behavioral permanent-red. See `AP-AMBASSADOR.md §1`.
- **AP-V5 (identity upgrade)** — DECIDED (RUN-AP-01). Fresh-start default;
  the only upgrade path is a subject-initiated dual-proof binding. See
  `AP-AMBASSADOR.md §1`.

## Findings from this run

**F-AP-1 — receipt `state` is DELIBERATELY excluded from the identity-forming
canonical encoding.** Surfaced by T-AP4.2 (masked never-was-world equality).

The initial encoding included the `state` field ("evidence-complete" |
"attested-redacted") in the receipt-id calculation, which made redaction
change the id and broke identity invariance — a caller pointing at an id
would see a different id after the same receipt was redacted, contradicting
"the receipt names the received observation, not the current store state".

Corrected in `src/canonical.rs::encode_receipt` (see doc-comment): the
`state` field is a mutable per-store marker, not part of the receipt's
identity. The id names the received observation, which does not change on
redaction; the state marker moves separately. The masked never-was-world
equality then holds tightly:

  - receipt_id unchanged (identity-invariant),
  - commitment unchanged,
  - body_hash unchanged (the ambassador HAD it — commitment survives so
    "we saw this" is auditable),
  - attestation_marker unchanged (P5 legibility),
  - state = AttestedRedacted (the ONE per-record public change),
  - body = None (the byte-verifiable evidence is gone).

RED evidence: T-AP4.2 `assert_eq!(after.receipt_id(), f.receipt_id())`
failed on first fold. GREEN evidence: same assertion passes after the
encoder change; T-AP2 receipt-identity tests remain green (identity across
construction paths still matches, salt-change still changes the id).

Filed: RUN-AP-01. Serves: AP-V2 (record composition) + AP-V3 (undo/delete
custom rider). No status tag moves — this is a design refinement inside
the ambassador crate's own semantics, not a spec-facing change.
