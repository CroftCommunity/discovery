# RUN-ATTEST-02 — Anchor Personas: No Default, Hard Splits, Reality Anchors

Status: INSTRUCTION — for execution via Claude Code in `CroftCommunity/discovery`

Gate: requires the `attest-family` crate from RUN-ATTEST-01 merged (Parts 0–3 minimum; Part 6 issuer machinery strongly preferred — if Part 6 was dropped, execute it first as a pre-part here).

---

## 1. Context and intent

RUN-ATTEST-01 proved the attestation family (edges, vouches, reviews, issuer predicates). This run proves the **anchor-persona model** layered on it, designed 2026-07-17:

- A member can hold several credentialed persona root keypairs ("anchor personas"), minted by the co-op for a fee.
- **No default exists.** Nothing in any public object ranks, orders, or distinguishes a "primary" persona. An observer's total knowledge is "this persona carries the predicate; that one does not."
- Each anchor signals a **reality anchor** (a vetted human stands behind this persona) while the graphs stay hard-split.
- The claim under test, stated as the property pair: **accountability attaches per-persona; unity of the human stays private.** Privacy and meaningful provenance simultaneously, not traded off.

The experiments below either prove a property, demonstrate a bounded residue honestly (FINDINGS), or measure a quantity (anonymity sets). All three outcomes are deliverables; a clean FINDINGS entry is success, not failure.

## 2. Standing directives

All RUN-ATTEST-01 §2 directives carry over unchanged and binding: TDD red-first with red-to-green evidence per part; cryptographic ordering, never wall-clock; canonical dag-cbor via existing machinery; supersede-never-revoke; A.9 evidence discipline with `(evidence: …, RUN-ATTEST-02[, grade])`; frozen record; site gate if docs touched; owner calls flagged (§8), narrowest option implemented and tagged.

One addition: **measurement experiments (EXP-PA4) are exempt from pass/fail framing but not from TDD** — the measurement harness itself gets red-first tests (known small fixture → hand-computed expected set sizes) before it runs on the full fixture populations.

## 3. Fixtures (extend RUN-ATTEST-01 Part 0; fixtures before features)

- Holders H1..H5. H1 holds anchor personas P1a, P1b, P1c (the "3 legit anchors" case). H2 holds P2a, P2b. H3–H5 hold one each. Holder↔persona linkage lives ONLY in fixture bookkeeping, never in any payload or issuer public object.
- Two issuer fixtures: COOP-S (small, 12 member-holders, populated with generated filler personas) and COOP-L (large, 400 member-holders, generated). Same code paths, different populations — for EXP-PA4.
- Mint-ceremony harness: `mint(issuer, holder_fixture, predicates) -> credential` where the harness performs the vetting-event stand-in, derives the credential **independently per persona** (fresh nonces, no shared state across calls), and returns what the *persona* publishes. What the *issuer* retains/publishes is exactly the subject of EXP-PA6 — the harness must route it through the issuer-state type under test, not around it.
- Single-predicate credentials are the unit (`over_18`, `phone_verified`, `payment_verified`, `vetted_holder`). Bundles exist only as presentation-side composition (see T-PA4.4).
- Adversary harness: `sweep_public(viewer) -> Corpus` gathering every byte a third-party viewer can obtain (persona folds, issuer public lineage, query responses), for the unlinkability property tests. Reuse RUN-ATTEST-01's T-AT4.3 sweep; extend it, do not fork it.

## 4. The experiments

### EXP-PA1 — The no-default invariant

Claim: no public object, field, or derivable value designates any persona as primary, first, or ranked.

- **T-PA1.1 `no_rank_representable`** — compile-boundary assertion (EXP-B style): credential and persona types contain no ordinal, `primary`, rank, or sequence field. Extends RUN-ATTEST-01 T-AT0.2.
- **T-PA1.2 `sibling_indistinguishability`** — shuffle the serialized public objects of P1a/P1b/P1c among ten single-anchor strangers' objects; property test asserts no deterministic function over public bytes selects H1's first-minted persona better than chance. Document the test's power honestly (what adversary class it models: public-data-only, no issuer insider, no traffic analysis).
- **T-PA1.3 `issuer_lineage_carries_commitments_not_identities`** — the issuer's public lineage contains blinded commitments (hash of credential + fresh salt), never subject persona identifiers. Verification of a credential needs only the issuer's signature on the credential itself; no public registry lookup exists. (Narrowest option per OC-1; tag it.)
- **T-PA1.4 `commitment_fold_is_unordered_per_epoch`** — commitments fold as an unordered set within an epoch, so adjacency in the issuer lineage cannot correlate siblings minted in one ceremony session. (Mitigation for the batching correlator; OC-2 covers the alternative of ceremony-spacing policy.)

### EXP-PA2 — Sibling unlinkability

Claim: credentials issued to sibling personas share no correlator in public data.

- **T-PA2.1 `sibling_credentials_unlinkable`** — the rider promised in conversation: property test over `sweep_public` output; credentials of P1a/P1b/P1c share no serial, batch id, key material, salt reuse, or derivable value. Fuzz serializations for leakage (the T-AT3.5 pattern).
- **T-PA2.2 `independent_derivation_enforced`** — the mint path cannot reuse nonces/salts across calls: attempt to mint two credentials with shared derivation state fails at the type boundary or is rejected deterministically.
- **T-PA2.3 `status_check_no_cross_leak`** — the credential status-check protocol (T-PA6.3) for P1a's credential returns nothing enabling inference about P1b/P1c or about any other member (response fields enumerated and asserted exact).
- **T-PA2.4 (FINDINGS, not code)** — enumerate residual correlators outside the model's control: shared counterpart personas across sibling graphs (already recorded in RUN-ATTEST-01 T-AT4.3), behavioral/stylometric linkage, network-layer metadata. State plainly that these are out of protocol scope and where each would be mitigated (client hygiene, transport). Grade: Modeled by design.

### EXP-PA3 — Fee as sybil friction (modeled)

Claim: credentialed personas are cheap for a human, expensive to farm. No payment rail is built; the *structure* that makes the fee meaningful is.

- **T-PA3.1 `no_credential_without_vetting_antecedent`** — fold-level: a credential lacking a vetting-event antecedent (fixture fact) folds to pending, never standing. There is no code path to a standing credential that bypasses the antecedent (negative API-surface test, T-AT5.4 pattern).
- **T-PA3.2 `anchor_count_is_a_governed_dial`** — max-anchors-per-member is a rule under the existing R7 machinery (reuse, don't reimplement), changeable only by quorum, visible in lineage. Test quorum-gated change and under-quorum pending, inheriting the concurrent-quorum hard-stop unchanged.
- **T-PA3.3 `member_anchor_count_not_public`** — property test over `sweep_public`: no public object or aggregate reveals any member's anchor count, including "some member holds 3" (commitment counts are total-only, never per-member partitioned).

### EXP-PA4 — Anonymity-set measurement (instrumented, not pass/fail)

Claim to quantify, not assert: a credential partitions personas into the pool of same-(issuer, predicate) holders; small co-ops give thinner cover; rare predicate bundles shrink it further.

- **T-PA4.1 `harness_correct_on_known_fixture`** — red-first: hand-computed set sizes on a 6-persona fixture; harness must reproduce them exactly before touching COOP-S/COOP-L.
- **M-PA4.2** — measure anonymity-set size per (issuer, predicate) for COOP-S (12) and COOP-L (400); tabulate.
- **M-PA4.3** — measure the shrink from presentation-side bundle composition: `over_18` alone vs `over_18 + phone_verified + payment_verified` shown together, both co-ops.
- **T-PA4.4 `presentation_is_subset_capable`** — because single-predicate credentials are the unit (§3), a persona can present any subset without revealing unpresented predicates it holds; test that a presentation of `over_18` alone contains no trace of the persona's other credentials.
- **Deliverable**: `FINDINGS-ANONYMITY-SETS.md` — the tables from M-PA4.2/3 plus plain-language guidance (coarse predicates by default; small co-ops should say so to members; federation across issuers as the eventual widener, noted as direction only). Grade: measurement, Modeled fixtures.

### EXP-PA5 — Dual attachment: accountability per persona, unity private

Claim: a persona carries its own record fully; siblings carry none of it.

- **T-PA5.1 `record_stays_with_persona`** — give P1a a history: superseded vouches, a review dispute (review + signed reply), a superseded credential. Assert P1a's fold carries all of it, supersede-never-erase.
- **T-PA5.2 `siblings_unaffected`** — byte-level: P1b's and P1c's folds are identical before and after P1a's history accrues, from every third-party viewer's sweep.
- **T-PA5.3 `anchor_is_not_uniqueness`** — vocabulary + negative test: the predicate is `vetted_holder` ("a vetted human stands behind this persona"), and no operation, query, or derivable value answers "do personas X and Y share a holder?" — assert the question is unaskable through the public API surface. `PRIMITIVES-ATTEST.md` gains the explicit sentence: an anchor credential is NOT proof of unique personhood; one human may hold several; contexts requiring one-persona-per-human (voting, one-account promotions) need a different, scope-bound predicate that deliberately reintroduces linkage within that context — defined in vocabulary as `sole_anchor(context)`, NOT built (OC-3).

### EXP-PA6 — Issuer covenant and no-record under multi-persona

Claim: the issuer's retained and published state stays minimal even when members hold multiple anchors, and covenant compliance is auditable without unmasking anyone.

- **T-PA6.1 `issuer_state_is_assertions_plus_process_only`** — compile-boundary + runtime: the issuer-state type can hold its own signed assertions, process-provenance metadata, blinded commitments, and the payment-bookkeeping stand-in explicitly typed as `SeamBoundary` (the known linkage point, named in the type system so it cannot silently spread). Substrate remains unrepresentable (inherits T-AT6.1).
- **T-PA6.2 `covenant_audit_without_unmasking`** — an audit query over the issuer's public lineage verifies: total credentials issued matches total commitments, every commitment has a well-formed shape, the covenant rule's lineage is intact — while resolving zero persona identities. Test the audit passes on honest state and fails on a tampered fixture.
- **T-PA6.3 `status_check_protocol`** — OCSP-shaped, read-side solicitation (matches the completeness-ahead posture: solicitation reaches the tail at quantified trust): verifier submits a credential hash; issuer answers current/superseded, signed, deterministically, from its own assertion lineage. Staleness of an unanswered check is presentation, never verdict (no fail-open badge, no verdict-by-timeout — align with the fail-closed discipline: an app choosing to require a fresh answer fails closed, and that choice is app policy, not protocol).
- **T-PA6.4 `supersede_reaches_verifier_without_registry`** — end-to-end: issuer supersedes P1a's `phone_verified`; a verifier running the status check gets `superseded`; P1a cannot present the old credential as current against a checking verifier, while the old object itself remains intact in lineage (T-AT0.3 invariant).

## 5. Definition of green

All T-* tests green red-first; M-* measurements produced with T-PA4.1 proving the harness; `FINDINGS-ANONYMITY-SETS.md` and the T-PA2.4 FINDINGS entry present; `PRIMITIVES-ATTEST.md` updated with: anchor persona, reality anchor, `vetted_holder` vs `sole_anchor(context)`, commitment, status check — each with the one-sentence is/is-not pair; pure workspace clean; site gate green if docs touched.

## 6. Run summary requirements

Per standing directive: red-to-green evidence per experiment; the adversary-class statement from T-PA1.2 reproduced verbatim; OWNER-CALL tags emitted; FIX vs FINDING classifications; measured tables inlined or linked. Status: Modeled throughout except reused R7 machinery, which cites its existing grade for the reused portion only.

## 7. Drop order (if constrained)

EXP-PA4 measurement breadth (keep T-PA4.1 and T-PA4.4) → EXP-PA3.2 (keep PA3.1, PA3.3) → EXP-PA6.4 end-to-end (keep PA6.1–6.3). EXP-PA1, PA2, PA5 are the model's spine; do not drop.

## 8. Owner calls surfaced (do not decide)

- **OC-1**: issuer public-lineage content — blinded commitments (implemented, narrow) vs publishing nothing (verification by signature alone, but T-PA6.2's audit becomes attestation-based) vs full issuance facts (rejected in design conversation but recorded as the rejected pole).
- **OC-2**: sibling-batching mitigation — unordered per-epoch commitment folds (implemented) vs ceremony-spacing policy vs both.
- **OC-3**: whether `sole_anchor(context)` ever ships, and if so which contexts justify deliberate intra-context linkage.
- **OC-4**: fee semantics — flat per-anchor vs vetting-tier pricing; pure policy, no protocol impact, listed because T-PA3.2 makes the count dial governable and the fee will be asked about in the same breath.

## 9. Non-goals (explicit)

No payment rails, no real vetting, no BBS/unlinkable-presentation cryptography (T-PA2.x proves unlinkability of what v1 publishes; presentation-unlinkability across repeated shows of the SAME credential to different verifiers is the deferred cryptographic layer — state this distinction in FINDINGS so the two claims are never conflated). No traffic-analysis or stylometric defenses (named residue, T-PA2.4). No ATProto integration. No `sole_anchor` implementation.
