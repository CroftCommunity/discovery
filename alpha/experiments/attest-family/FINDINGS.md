# FINDINGS — attest-family (RUN-ATTEST-01)

Findings ledger for the attestation-family experiments. FINDING = something
learned that the design must carry; FIX = a defect corrected during the run.
Everything here is grade **Modeled** (fixture keypairs, in-memory fold) unless
stated otherwise.

## F-AT-1 — Persona correlation residue (T-AT4.3; FINDING, expected, recorded not solved)

The property test proves the *identifier* floor: across the public folded
surface reachable by a third-party viewer, no shared identifier, key material,
or derivable value links P1a and P1b (same holder H1) other than the shared
counterpart P2 itself — the holder linkage is not even representable in a
payload type. What remains possible is **behavioral/metadata correlation**, and
it is expected, not solved:

- **Shared counterpart structure.** Anyone who can resolve P2's edge list sees
  that both P1a and P1b connect to P2. With more shared counterparts the
  intersection fingerprint sharpens (the classic co-link attack).
- **Timing-shaped graph structure.** Lamport-adjacent activity bursts, similar
  edge-formation cadence, correlated supersede patterns, and similar scope
  vocabularies can all suggest common authorship. Nothing in the model hides
  activity shape.
- **Ceremony geography.** Co-presence ceremony facts assert shared sessions;
  two personas repeatedly co-present with the same third parties leak social
  proximity as a claim pattern.

This residue is the documented cost of the v1 posture (plain per-persona
keypairs, no unlinkable credentials). It is recorded here so nobody later
mistakes the identifier floor for full unlinkability. (evidence:
`tests/t_at4_resolvability.rs` `persona_correlation_resistance`, RUN-ATTEST-01,
Modeled)

## F-AT-2 — The co-op issuer linkage seam (T-AT4.4; FINDING, Modeled by design)

The **issuer linkage seam**: an issuer that credentials multiple personas of
one holder can link them. The COOP issuer verifies a holder's substrate
(document, phone, card) and then issues predicates to personas; if one holder
brings P1a and P1b to the same issuer, the issuer's own records — not any
protocol object — connect them. Nothing in the corroboration/resolvability
machinery exposes that link (T-AT6.1 keeps substrate unrepresentable in
payloads; T-AT6.2 keeps predicates process-bound), but the seam exists at the
issuer.

**v1 posture (bounds the blast radius):**
- **per-persona optional issuance** — a holder chooses per persona whether to
  request issuance at all; an uncredentialed persona never touches the issuer;
- **no-record covenant** — the co-op issuer's governed covenant (the T-AT6.4
  rule) commits it to retaining no substrate and no cross-persona issuance
  ledger beyond what auditing its own process requires; weakening that covenant
  requires a content-bound quorum and is contradiction-hard-stopped.

**Deferred cryptographic direction (out of scope, §9):** "unlinkable presentations"
— BBS-style signatures / anonymous credentials, where a persona
proves "some trusted issuer asserted over_18" without revealing which issuance
event (or issuer-side correlatable identifier) backs it. Documented as the
direction only; no BBS et al. in this run.

Grade: **Modeled, by design** — this entry is the deliverable; there is no code
test for an organizational seam. (evidence: RUN-ATTEST-01 §4 Part 4,
T-AT4.4)

## F-AT-3 — Fold-order vs hash-order under wall-clock shifts (T-AT0.4; FINDING)

Payload date claims participate in object bytes, so shifting a date changes
every object id downstream of it. T-AT0.4 therefore compares corpora under
**hash erasure** (object ids mapped to fixture indices): fold order — (lamport,
author bytes, object id) — and folded state are identical because the id
tiebreak is only reached for same-(lamport, author) collisions, which the
per-author logical clock rules out. The invariant the family actually carries:
wall-clock claims may change *identities* (they are signed content) but can
never change *outcomes* (ordering, conflict results, standing). (evidence:
`tests/t_at0_floor.rs` `ordering_ignores_wallclock`, RUN-ATTEST-01, Modeled)

## F-AT-4 — What the no-suppression surface actually is (T-AT5.4; FINDING)

The negative API-surface test pins the public operation list of the fold/query
modules to an exact allowlist and behaviorally confirms the subject's only
powers over a review are (a) the signed reply peer object and (b) their own
persona's resolvability policy — which filters by *attester*, so it cannot
remove the review from any third viewer's structure. The load-bearing part is
the **allowlist pin**: any future public operation fails the test until it is
reviewed against the suppression invariant. (evidence:
`tests/t_at5_review.rs` `no_suppression_path_exists`, RUN-ATTEST-01, Modeled)

## F-AT-5 — Covenant amendments must be causally chained (T-AT6.4; FINDING)

First green attempt at T-AT6.4 exposed a real property of the reused R7
machinery: a quorum-met covenant weakening that does NOT cite the covenant's
establishing RuleChange as antecedent is causally CONCURRENT with it, and the
substrate's §7.6.1 competing-RuleChange predicate correctly hard-stops the pair
(`contradiction:{min-hash}`) instead of applying the amendment. The corrected
test cites the establishment (`set_full`) from every later attempt. Carried
forward as covenant practice: **an amendment is a supersede in lineage, so it
must name the register state it amends; an unchained amendment is a competitor,
not an amendment.** Not a defect — the machinery refusing a verdict on an
ambiguous history is the designed behavior. (evidence:
`tests/t_at6_covenant.rs`, RUN-ATTEST-01; machinery grade per the substrate's
existing §7.2/§7.6.1 status, this modeling Modeled)
