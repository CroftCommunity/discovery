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

**F-AP-2 — the AP JSON parser refused arrays.** Surfaced by the shim-leg
upgrade (`tests/shim_leg.rs::t_ap1s_3_shim_delete_actor_redacts_held_receipts`)
against the Mastodon Delete(Actor) specimen shape.

The hand-rolled `parse_json_object` in `src/verify.rs` handled top-level
strings and nested objects, but refused JSON arrays — a fine tradeoff
against the RUN-AP-01 fixture bodies (which had no arrays), but wrong
against real Mastodon: the Delete(Actor) specimen carries
`"to":["https://www.w3.org/ns/activitystreams#Public"]` per
`fed-shim/tests/specimens/mastodon-delete-actor-observed-shape.md`.

The parser returned `VerifyError::MalformedActivity("body is not a JSON
object")` when it hit `[`, so a real-Mastodon Delete never verified.

Corrected in `src/verify.rs::parse_json_object`: extended to skip past
balanced `[…]` arrays (mirroring the existing balanced `{…}`-object
skip). The array's contents are still not exposed — the ambassador only
uses top-level `type`, `actor`, `object`, `id`. This is a fidelity fix,
not a feature addition.

RED evidence: `t_ap1s_3` failed with `MalformedActivity("body is not a
JSON object")` before the fix. GREEN evidence: same test passes; the
Delete redaction fires as designed and the ambassador store moves
matching receipts to `AttestedRedacted`.

Filed: RUN-AP-01 shim-leg upgrade (post-merge). Serves: AP-V3 (Delete
custom rider) — proves the ambassador handles real-Mastodon Delete
shapes. No status tag moves — a bug-fix on the parser, no spec-facing
change. Rationale for the RUN-AP-01 P1 grade UPDATED: bytes now
specimen-anchored (fed-shim). Grade tag stays `Modeled` because no
live Mastodon has been round-tripped yet (still gated per RUN-AP-01
§6 / FED-SHIM.md §4).
