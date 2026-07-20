# RUN-ATTEST-01 — Attestation Family: Proving the Model in Code

Status: INSTRUCTION — for execution via Claude Code in `CroftCommunity/discovery`

Lane: own lane (like RUN-STELLIN-INFRA-01). If the owner prefers mainline numbering, retitle to the next free RUN-NN; content is unchanged.

---

## 1. Context and intent

This run proves out, in code, the attestation family designed in conversation on 2026-07-17: attested "I know you" edges, scoped vouches, reviews, and co-op issuer predicates — all as consequences of the founding razor (the protocol computes provenance: consistent + corroborated; it never computes utility: true/right, which is left to humans at the edges).

The model under test, in one paragraph: there is **one attestation family** with two axes. **Subject type**: persona, or thing (business, product, work). **Consent mode**: `mutual` (co-signed edge, the friend case), `unilateral_notice` (subject notified, signed reply allowed, no countersign required — the review case), `unilateral_private` (note to self). Everything else — scope labels, supersede-never-revoke, per-viewer resolvability, corroboration structure, provenance grades — is shared machinery. No trust score exists anywhere. Queries return corroboration structure, viewer-relative; clients do the weighting.

The deliverable of this run is twofold: (a) experiment-grade evidence that each claimed property holds or fails, and (b) `PRIMITIVES-ATTEST.md`, the usable primitive language — because "seeing the principle through to usable primitive language is the scope."

## 2. Standing directives (all binding)

- **TDD, red-first, always** (permanent directive 2026-07-15). For every part: encode acceptance criteria as failing tests before any implementation; fixtures before features; the run summary must evidence red-to-green order per test group (paste the initial red output or its digest).
- **Ordering is cryptographic, never timestamps.** Wall-clock values may appear *inside* payloads as asserted claims (e.g. an issuer's "sighted on 2026-07-17") but must never influence fold ordering or conflict outcomes. This is tested explicitly (T-AT0.4).
- **Canonical encoding.** All payloads use the existing canonical dag-cbor path (§4.6 machinery). Reuse the proven crates; reuse is a condition of considered compatibility. Do not re-implement canonicalization.
- **Supersede, never revoke-in-place.** No operation in this run mutates or deletes a prior object. Ever.
- **Evidence discipline.** Any sentence added to spec/doc trees carries the standard parenthetical `(evidence: …, RUN-ATTEST-01[, grade])` per the A.9 ladder. When in doubt, grade Modeled. No retrofits to existing text.
- **Frozen record.** Do not edit frozen corpora. New docs only, in the locations specified below.
- **Site gate.** If anything lands under the docs tree, `site/build.py` (broken-ref + anchor audit) is part of the definition of green.
- **Owner calls are flagged, not made.** §8 lists them. If execution hits one, implement the narrowest option, tag the test `// OWNER-CALL: OC-n pending`, and record it in the run summary.

## 3. Layout and stand-ins

- New experiment crate under the existing experiments area, following current experiment-crate layout and naming conventions: working name `attest-family`. Pure workspace; no network deps, no crypto beyond what the workspace already provides.
- `PRIMITIVES-ATTEST.md` lands next to the crate (alpha side) as a living doc, DRAFT-PENDING. Whether/where it graduates into beta spec is OC-1 (owner call), not this run.
- **Declared stand-ins:** personas are fixture keypairs (no DID/atproto integration — explicit non-goal, see §9); "resolvability policy" is an in-memory table; the co-op issuer is a fixture persona with an `issuer` role marker; the transaction attestation (verified-purchase analog) is a fixture fact, no payment rail.
- **Fixtures before features (Part 0 builds all of these):**
  - Holders H1, H2, H3; personas P1a/P1b (both H1 — the linkage exists ONLY in fixture bookkeeping, never in any payload), P2 (H2), P3 (H3).
  - Issuer persona COOP with process-provenance metadata builders.
  - Thing subjects: BIZ1 (a plumber), WORK1 (a product).
  - Canonical payload builder for each attestation kind; permutation harness for order-independence tests (reuse the existing one if present).

## 4. The parts

Execute in order. Each part: red tests → implement → green → summary entry.

### Part 0 — Fixtures, vocabulary, and floor invariants

Deliver fixtures per §3 and the first cut of `PRIMITIVES-ATTEST.md` (terms: attestation, edge, half, vouch, review, predicate, scope, consent mode, ceremony fact, grade, corroboration structure, resolvability). Every term gets one sentence of definition and one sentence of what it is NOT.

Floor tests (red first):

- **T-AT0.1 `canonical_roundtrip`** — every attestation kind round-trips through canonical encoding byte-identically.
- **T-AT0.2 `no_score_field_exists`** — compile-boundary assertion: no public type in the crate contains any numeric trust/score/rating/rank field. (Pattern: type-level test in the style of EXP-B's content-blind compile-boundary assertion.)
- **T-AT0.3 `supersede_preserves_prior`** — after any supersede, the prior object's bytes are retrievable unchanged.
- **T-AT0.4 `ordering_ignores_wallclock`** — two corpora identical except for permuted/shifted wall-clock payload fields fold to identical order and identical state.

### Part 1 — EXP-AT1: The co-signed edge (mutual mode)

An edge is a co-signed op (R7 shape: approvals are antecedents). Alice's half is an antecedent of Bob's; the edge exists iff both halves reference the same canonical **shared core** (both persona ids, edge id, consent mode = mutual, ceremony facts). The per-side **label** ("friend from school" vs "roommate's sister") is side-local and may differ.

Red tests:

- **T-AT1.1 `half_is_not_an_edge`** — a single half folds to pending; no query surfaces it as an edge; pending is never partial.
- **T-AT1.2 `edge_iff_matching_core_hash`** — both halves referencing the same core hash → edge exists.
- **T-AT1.3 `core_mismatch_never_edges`** — tamper one side's core (e.g. consent mode) → two pendings forever, no edge, no error-verdict; fold states the facts.
- **T-AT1.4 `labels_are_side_local`** — differing labels do not affect core hash or edge existence; each side's fold shows its own label.
- **T-AT1.5 `edge_supersede_lineage`** — superseding edge (ending the relationship) leaves the old edge in lineage; current view shows superseded status; T-AT0.3 invariant holds.
- **T-AT1.6 `order_independent_fold`** — property test: all permutations of {half A, half B, supersede} arrivals reach identical fold state.
- **T-AT1.7 `ceremony_grade_in_person`** — a co-presence session fact signed by both in the same session yields grade `in_person`; without it, grade `remote`. Grade is provenance metadata on the edge, never an input to any comparison or ordering (assert no code path consumes grade except serialization/display).

### Part 2 — EXP-AT2: Scoped vouches layered on the edge

A vouch is a separate, later, unilateral claim by one edge participant about the other, in a named scope ("would hire as contractor"), citing the base edge as antecedent. Vouches supersede independently of the edge.

Red tests:

- **T-AT2.1 `vouch_cites_edge_antecedent`** — a vouch without a resolvable base-edge antecedent folds to pending, not to a standing vouch. (Narrowest option per OC-2; tag it.)
- **T-AT2.2 `vouch_supersede_independent`** — narrowing or withdrawing a vouch changes only the vouch lineage; the edge fold is byte-identical before/after.
- **T-AT2.3 `edge_supersede_marks_vouches`** — when the base edge is superseded, dependent vouches remain intact objects; their fold gains `antecedent_superseded` presentation metadata; they are never auto-withdrawn (no verdict by side effect). (Narrowest option per OC-3; tag it.)
- **T-AT2.4 `transaction_antecedent_raises_grade`** — a vouch/review citing a transaction attestation as antecedent carries grade `transaction_backed`; same rule as T-AT1.7: grade is metadata only.

### Part 3 — EXP-AT3: Scoped corroboration, no scalar, viewer-relative

The only query in the model: given (viewer, subject, scope), return the **corroboration structure** — the set of standing vouches/reviews whose scope matches and whose attester persona is *resolvable to this viewer* — with grades and lineage pointers. Never an aggregate.

Red tests:

- **T-AT3.1 `returns_structure_not_scalar`** — response type contains no aggregate numeric field and no computed ordering (results in canonical-hash order only). Ties to T-AT0.2.
- **T-AT3.2 `scope_filter_exact`** — only scope-matching attestations traverse; adjacent scopes ("contractor" vs "babysitter") never bleed.
- **T-AT3.3 `resolvability_filters_traversal`** — an attestation whose attester is not resolvable to the viewer is absent from the response entirely (not redacted-but-counted — absent).
- **T-AT3.4 `viewer_relativity`** — same (subject, scope), two viewers with different resolvability policies → provably different corroboration structures; each internally consistent.
- **T-AT3.5 `mutual_count_without_identity`** — the "N connections in common" disclosure returns cardinality only; property test asserts the serialized response contains no identifier, hash, or derivable value of the N counterpart personas beyond the count. Fuzz the serialization for leakage.

### Part 4 — EXP-AT4: Resolvability governed by the named party + correlation resistance

Resolvability of a persona named at the far end of an edge is governed by that party's policy, never by the edge holder's disclosure choices.

Red tests:

- **T-AT4.1 `edge_holder_cannot_grant_far_end`** — holder discloses their edge list to viewer V; a far-end persona whose policy excludes V remains unresolvable in V's view of that list (edge visible as existing, far end opaque).
- **T-AT4.2 `policy_change_is_supersede`** — resolvability policy changes are superseding facts with lineage, not mutations.
- **T-AT4.3 `persona_correlation_resistance`** — P1a and P1b (same holder H1) each mint edges to P2. Property test over all public folded data reachable by any third-party viewer: no shared identifier, key material, or derivable value links P1a and P1b other than the shared counterpart P2 itself. Document in FINDINGS what *behavioral/metadata* correlation remains possible (shared counterpart, timing-shaped graph structure) — that residue is expected and is recorded, not solved.
- **T-AT4.4 `issuer_linkage_seam_documented`** — not a code test: a FINDINGS entry stating the co-op-as-issuer linkage seam (an issuer that credentials multiple personas of one holder can link them), the v1 posture (per-persona optional issuance + no-record covenant bounds blast radius), and the deferred cryptographic direction (unlinkable presentations, BBS-style — out of scope, see §9). Grade: Modeled, by design.

### Part 5 — EXP-AT5: Unilateral-with-notice mode (reviews)

Reviews must not require subject countersignature (a business would countersign only praise — that failure is the point). Integrity comes from provenance structure: signed authorship, scope, antecedent grades, subject notice, and signed reply as a peer object.

Red tests:

- **T-AT5.1 `review_stands_without_countersign`** — a `unilateral_notice` attestation folds to standing with only the author's signature.
- **T-AT5.2 `subject_notice_fact_emitted`** — folding a review deterministically produces a notice fact addressed to the subject (delivery is out of scope; the fact's existence and determinism are in scope).
- **T-AT5.3 `signed_reply_is_peer_object`** — the subject's reply attaches referencing the review; the review's bytes are unchanged; the corroboration structure returns both.
- **T-AT5.4 `no_suppression_path_exists`** — negative API-surface test: enumerate the crate's public operations; assert none accepts (subject, review) in a way that removes, hides, or demotes the review from any third viewer's corroboration structure. The subject's only powers: reply (T-AT5.3) and their own resolvability policy over *their own* persona.
- **T-AT5.5 `freshness_is_presentation`** — an old review gains staleness presentation metadata under a governed freshness dial; it is never dropped or down-ranked by the protocol. No verdict-by-timeout (fail-closed discipline).

### Part 6 — EXP-AT6: Issuer predicates + covenant as governed rule

The co-op issuer asserts predicates ("over_18", "phone_verified", "payment_verified") about a persona. The payload is: predicate, subject persona, issuer, process-provenance metadata (method, date-as-claim, role). The substrate (ID number, card number) must be *unrepresentable*.

Red tests:

- **T-AT6.1 `substrate_unrepresentable`** — compile-boundary assertion in the EXP-B style: predicate payload types have no field capable of carrying substrate; a doc-tested example showing that attempting it does not compile.
- **T-AT6.2 `predicate_inseparable_from_issuer_and_process`** — every serialization of a predicate carries issuer + process metadata; no code path yields a bare "over_18: true" detached from who asserted it and how.
- **T-AT6.3 `expiring_predicate_via_supersede`** — phone verification is refreshed by supersede; the stale predicate persists in lineage with staleness presentation per T-AT5.5's dial; never expiry-by-timeout.
- **T-AT6.4 `covenant_is_r7_rule`** — reuse the existing R7 content-bound quorum machinery (do not reimplement): model the no-monetization covenant as a rule; assert (a) weakening it requires quorum met on the canonical payload hash, (b) under-quorum change is pending, never partial, (c) the change and its approval antecedents are visible in lineage. If two concurrent quorum-met changes collide, the existing contradiction hard-stop applies unchanged — add one test confirming the attest crate inherits that behavior rather than shadowing it.

## 5. Definition of green

All parts' tests green; T-AT0.* invariants green across the whole crate; `cargo test` clean in the pure workspace; site gate green if docs tree touched; `PRIMITIVES-ATTEST.md` present with every §4 term defined; FINDINGS entries for T-AT4.3 residue and T-AT4.4 seam present.

## 6. Run summary requirements

Per standing directive: red-to-green evidence per part (initial failing output digest + final green), test counts, any OWNER-CALL tags emitted, any FIX vs FINDING classifications, deviations from this file with reasons. Status tags: everything lands `Modeled` unless it directly reuses `Verified` machinery unmodified (T-AT6.4 may cite the reused machinery's existing grade for the reused portion only).

## 7. Drop order (if constrained)

Drop from the tail first: Part 5 → Part 6 (except T-AT6.1, keep the compile-boundary pattern) → Part 4 property-test breadth (keep T-AT4.1). Parts 0–3 are the model's spine; do not drop.

## 8. Owner calls surfaced (do not decide)

- **OC-1**: where `PRIMITIVES-ATTEST.md` graduates (alpha living doc vs beta spec section) and its frozen-record implications.
- **OC-2**: may a vouch stand alone without a base edge (e.g. vouching for a professional you transacted with but never "friended")? This run implements edge-antecedent-required as the narrow option; the transaction-antecedent path (T-AT2.4) hints at the alternative.
- **OC-3**: long-term semantics of vouches whose base edge is superseded (persist-with-marker, as implemented, vs require re-affirmation window).
- **OC-4**: whether `unilateral_private` mode ships at all in v1 (it is defined in vocabulary but has no experiments here; zero tests is a deliberate statement, not an oversight).

## 9. Non-goals (explicit)

No ATProto/lexicon/DID integration (separate later run; the open-vouch-on-PDS tier is a different substrate). No unlinkable-presentation cryptography (BBS et al.) — documented as the deferred direction only. No UI, no network, no delivery of notice facts, no payment rails, no real identity checking. No trust scores — not as a non-goal but as a tested impossibility (T-AT0.2, T-AT3.1).
