# AP-AMBASSADOR — the ActivityPub-ambassador receipt lane

`Role charter, RUN-AP-01 (2026-07-20). Standing header — the graduation trigger
is a NAMED condition (following the RUN-ATTEST-04 V10 convention): the
ap-ambassador lane's release pass when it (a) opens a socket toward a real
fediverse host, (b) resolves an actor public key over the wire, and (c) mints a
receipt whose stored key material was fetched, not fixtured. This document
stays in alpha beside the crate until that trigger fires.`

## 0. Governing principle (canonical)

The ambassador respects the customs of the protocol federated with. It is a
delivery-plane role in the A.7 sense: it holds **no ordering authority, no
membership authority**, and — pinned this run — **no governance conductivity**.
Where the two cultures' semantics differ, the remedy is clear provenance: every
fact the ambassador mints states exactly *what was received, from whom,
verified how*, so intent and history can be contextualized safely on both
sides. (evidence: owner verdicts AP-V1..V5, RUN-AP-01 brief)

## 1. Settled inputs — the five walked verdicts (DECIDED, not relitigated)

### AP-V1 (register) — DECIDED (RUN-AP-01, owner-confirmed in brief §1)

No pre-registration of `ap_signed_follow` in the qualifying-antecedent
register. Register rows are minted when a persona chooses to bind; the entry
then binds what's given — **a one-sided, observer-attested reception
relationship over an interval, nothing more**. This run touches NO register
file (P7 non-touch check, tests + git-diff assertion). (evidence: AP-V1,
RUN-AP-01, `AntecedentRegister` in `../attest-family/src/fold.rs` unchanged)

### AP-V2 (record composition) — DECIDED (RUN-AP-01)

**Evidence-complete.** The receipt record carries:

1. the full ActivityPub activity JSON (as received bytes),
2. the HTTP-signature headers as received (name/value pairs, canonical order),
3. the actor public key pinned at verification time (SPKI DER bytes + keyId).

**Posture-conditional blinded form.** The record carries `commitment` +
`H(evidence body)`; the body itself sits in the ambassador store, produced at
the tier the roster is visible at (RUN-19 blinded-tier machinery, adapted).

**Lean-projection rider.** Outbound projection is lean-by-default
(vanilla-Mastodon-shaped activities); full-discourse federation is a
per-persona opt-in dial (~the 20%). **Outbound delivery is OUT OF SCOPE this
run** — the rider lands as posture language only. (evidence: AP-V2,
RUN-AP-01, AP-OC-6 deferred to the outbound-delivery run)

### AP-V3 (undo/delete) — DECIDED (RUN-AP-01)

**Undo Follow = a second receipt record.** Nothing is deleted; the fold
derives follower intervals from Follow/Undo pairs (§A.3 machinery).

**Custom-respect rider.** ActivityPub asserts removal semantics for `Delete`
(receiver SHOULD remove, MAY Tombstone) and none for `Undo`; on
`Delete(actor|object)` **the ambassador redacts the evidence body and keeps
the fact skeleton** — commitment + interval boundaries. Masked never-was-world
equality (RUN-ATTEST-03 pattern). Post-redaction the record's state marker
degrades from `evidence-complete` to `attested-redacted`.

A re-verification attempt on a redacted record fails with a distinct
`EvidenceRedacted` variant — never `SignatureMismatch` (the degradation is
honest, not an error masquerade). **Undo does not trigger redaction** (only
`Delete` does). (evidence: AP-V3, RUN-AP-01, `redact.rs` + P4 tests)

### AP-V4 (governance) — DECIDED (RUN-AP-01)

**Hard exclusion as a ROLE boundary, not a per-kind rule.** Governance stays
on Croft groups via the same association machinery as every other atproto
case; **no code path exists** from ambassador-attested facts into R7 co-signs,
vouch antecedents, or quorum counting.

Structural + behavioral tests, **permanent-red** (P5). Structural: no
ambassador crate path is imported by any R7/governance crate, and no
ambassador fact type implements the co-sign/vouch antecedent traits (closed
`AntecedentKind` enum in attest-family admits no ambassador variant by
construction). Behavioral: attest-family's fold, given an ambassador receipt
id as a vouch antecedent, refuses to promote the vouch to standing
(no-qualifying-antecedent), AND the ambassador's own
`reject_governance_use()` returns a distinct typed error. (evidence: AP-V4,
RUN-AP-01, P5 tests permanent per this section)

### AP-V5 (identity upgrade) — DECIDED (RUN-AP-01)

**Fresh-start default.** An AP actor acquiring an atproto identity opens a
new interval with **no linkage**. Inference is not consent — the ambassador
NEVER links unilaterally, even where the correlation is obvious.

The **ONLY upgrade path** is a **subject-initiated dual-proof binding**: a
record signed with the DID repo key over
`{DID, AP actor id, AP-side origin proof, antecedent = H(old gateway receipt)}`,
minted by the subject, carrying the old gateway fact as antecedent. The
upgraded fact **inherits grade from the binding, not from the gateway's
observation**.

- A binding missing either proof leg is rejected.
- A binding whose signer is the gateway (not the subject) is rejected.
- The AP-side origin proof is **fixture-level this run**; the live
  `rel="me"` / actor-document leg is gated with the live leg (see the run
  brief §6).

(evidence: AP-V5, RUN-AP-01, `binding.rs` + P6 tests)

## 2. Evidence-grade ladder for ambassador facts

Ambassador facts sit **below native two-sided facts** on the ladder (they are
observer-attested, not two-sided). Within the ambassador tier:

- **`evidence-complete`** — the full activity JSON + HTTP-signature headers
  + pinned actor key are held in the store; the receipt is byte-verifiable
  from those bytes alone. (A.9 grade: `Modeled` this run — fixture keys, no
  live fetch.)
- **`attested-redacted`** — the evidence body has been redacted (AP-V3
  custom rider on `Delete`). The commitment survives; the receipt is no
  longer byte-verifiable. Re-verification returns the distinct
  `EvidenceRedacted` variant. Masked-equal to a never-was world except for
  the commitment and marker (P4).

Neither grade contributes to a co-sign or vouch antecedent (P5).

## 3. Lean-projection posture rider (posture language, no delivery this run)

An ambassador receipt is **fact-of-reception**, not a native
Croft-two-sided fact. Its **outbound projection back to the fediverse**, when
a delivery run wires it up, is:

- **Lean-by-default:** the ambassador projects only vanilla-Mastodon-shaped
  activities (`Follow`, `Undo`, `Accept`, `Reject`, `Note`, `Like`,
  `Announce`) with only the fields those activities carry in the wild — no
  Croft-native discourse structure attached.
- **Full-discourse federation is a per-persona opt-in dial (~the 20%).**
  When a persona opts in, richer Croft context rides along as content the
  federated side is free to ignore. Opt-in is a fact on the persona's own
  lineage, not a global switch.

The outbound-delivery mechanics themselves (queue shape, retry span,
sharedInbox strategy) are **AP-OC-6**, next run.

## 4. Open considerations surfaced, NOT decided this run

| tag | topic |
|---|---|
| **AP-OC-6** | Outbound-delivery mechanics (queue, retry span, sharedInbox strategy) — next run. |
| **AP-OC-7** | Lexicon drafts (`ing.croft.ap.*`) — deferred until the delivery run; sketch nothing now. `lexicons/` intentionally empty this run. |
| **AP-OC-8** | The blinded-tier follower-count disclosure dial — does a blinded roster publish cardinality? |
| **AP-OC-9** | Inbound non-follow activities (replies, likes) — a different fact family, untouched here. |

## 5. Declared stand-ins

- **Fixture RSA keypairs** — deterministic-seed generation (rand_chacha,
  fixed seed) stands in for real actor keys. The runtime verify path is
  the real path; only key material is fixtured. Live upgrade rides the
  gated live leg (brief §6).
- **In-test key resolver** — a fixture map keyed by `keyId` stands in for
  actor-document fetch over HTTPS. Same shape as the runtime resolver
  trait; the fetch impl is out of scope this run.
- **AP-side origin proof (P6)** — a signed AP activity from the actor's
  key stands in for `rel="me"` / actor-document link. Live leg = the
  gated live leg (brief §6).
- **Ambassador store** — an in-memory `BTreeMap<ObjectId, StoredBody>`;
  the persistent-store shape is out of scope this run.
- **No `AntecedentKind` ambassador variant exists** — by AP-V1 no such
  variant should exist; the P5 structural + closed-enum evidence pin it.

## 6. Kinship to other lanes

- **attest-family** — canonical dag-cbor path REUSED unchanged (§4.6). The
  R7-governed `AntecedentRegister` is UNTOUCHED (P7 non-touch); the closed
  `AntecedentKind` enum is the compile boundary that makes the P5 role
  boundary structural.
- **tier-proof / RUN-19** — the blinded-body-with-commitment machinery is
  patterned after the RUN-19 sealed-tier shape (commitment publishes,
  body sits behind decrypt). Adapted here with fixture salt; no
  wire-encoding pinning.
- **RUN-ATTEST-03 (AP-V3 custom rider)** — the masked never-was-world
  equality pattern for `Delete` mirrors RUN-ATTEST-03's edge-supersede
  pattern (redact the body, keep the skeleton, mark the state).
- **RUN-AP-01 sibling xmtp-ambassador** — the same delivery-role
  boundary is intended for the XMTP direction (RUN-FS-01, out of scope
  here). This lane's role charter (§0) applies verbatim in that lane.
