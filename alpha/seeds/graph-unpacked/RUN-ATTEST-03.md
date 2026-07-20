# RUN-ATTEST-03 — Settlement Riders (Verdicts 1–3) + ATProto Abilities Matchup

Status: INSTRUCTION — for execution via Claude Code in `CroftCommunity/discovery`

Gate: RUN-ATTEST-01 and RUN-ATTEST-02 merged (confirmed by owner audit 2026-07-18).

---

## 1. Context and intent

The owner-call walk on the attest board reached three verdicts on 2026-07-18. This run has two jobs:

**Part A** — integrate the three verdicts into `attest-family` and its docs as settlement riders (the established ritual: verdicts confirmed in conversation ride the next run).

**Part B** — deliver `ATTEST-ATPROTO-MATCHUP.md`: the design described in protocol-abilities terms ("what do we need from the substrate to realize this design") mapped against what ATProto provides **today**, in the shape of the prior seam/readiness briefs. The design has reached effectively first-class ATProto PDS integration for its public tier; this brief makes that claim precise, grounded, and honest about gaps.

**The verdicts being integrated (settled; do not relitigate):**

- **V1 (AT OC-2 = option B).** A vouch requires a qualifying antecedent from a **closed class**: co-signed edge, transaction attestation, or ceremony fact — no longer edge-only. Rationale of record: these are shapes of one provenance mechanism; what varies is the kind of trust bound — bidirectional (co-signed edge) or unidirectional (transaction, ceremony) — and both are valid.
- **V2 (AT OC-3 ratified + public-tier retraction semantics).** Persist-with-marker stands for vouches whose base edge is superseded. Retraction on the public tier means the author removing the record from their own PDS — the canonical copy — which is native ATProto deletion; it was never a promise to pull content back from the network. Amend = whole-record replace. No wall-clock anywhere. Private review/remediation is a **different mechanism with its own logic** — parked, out of scope.
- **V3 (AT OC-4 = defer).** `unilateral_private` stays vocabulary-only in v1. When it ships, it ships as a **private-substrate artifact** (an MLS-group-of-one object on the private tier), not as a fourth public consent mode.

**Still OPEN — this run must not decide or disturb:** AT OC-1 (PRIMITIVES graduation), PA OC-1 (issuer lineage content), PA OC-2 (sibling batching), PA OC-3 (`sole_anchor`), PA OC-4 (fee semantics), the resolvable-to-all stand-in default, and F-PA-3's graduation. Their `pending` tags stay exactly as they are.

## 2. Standing directives (all binding, carried from RUN-ATTEST-01 §2)

TDD red-first with red-to-green evidence per part (staged-violation reds for negative invariants, refutation-pin style); cryptographic ordering, never wall-clock; canonical dag-cbor via the existing machinery; supersede-never-revoke on all Drystone-side objects; reuse over reimplementation; A.9 evidence discipline — and note Part B is a **research brief**, so its factual claims about ATProto take the Verified-RFC band form: primary-source anchor (atproto.com specification pages, the lexicon/repo/identity specs, official Bluesky engineering posts) per claim, fetched during the session, with retrieval date. Blogs are corroboration only and say so. Frozen record; site gate (`site/build.py` incl. Mermaid) part of green if the docs tree is touched; owner calls beyond V1–V3 are flagged, never made.

**One explicitly authorized edit class:** the `// OWNER-CALL: OC-2 pending` and `OC-3 pending` comments in `t_at2_vouch.rs`/`src` and the `OC-4` entries in PRIMITIVES change from `pending` to `decided 2026-07-18` with a one-line pointer to this run's summary. Only those three. This is a named conditional edit in the sense of the evidence discipline, authorized here by the owner's verdicts.

## 3. Part A — settlement riders (TDD)

### A.1 Antecedent class widening (V1)

- **Types**: `AntecedentKind` closed enum {`co_signed_edge`, `transaction`, `ceremony`}; the vouch's grade vocabulary becomes kind-derived: `edge_backed` | `transaction_backed` | `ceremony_backed` (T-AT2.4's `transaction_backed` is already correct; edge-backed vouches gain their explicit grade rather than an implicit one). Grade remains presentation metadata only — the T-AT1.7 rule re-asserted here.
- **Fold**: a vouch citing ANY standing member of the class folds to standing; a vouch with no qualifying antecedent still folds to pending forever (the T-PA3.1 discipline, unchanged in spirit).
- **The class itself is governed.** Model the antecedent-class register on the reused R7 machinery exactly as the covenant and the anchor-count dial were (content-bound quorum, under-quorum refused whole, contradiction hard-stop inherited unchanged — one inheritance test, the `dial_inherits` pattern). Widening the class later (e.g. a future antecedent kind) is a quorum act with visible lineage, not a code edit.

Red-first tests:

- **T-A3.1 `plumber_case`** — transaction-backed vouch with NO edge between the parties folds to standing, grade `transaction_backed`. (This was pending-forever under RUN-ATTEST-01; the red is the old behavior, captured before the fold change.)
- **T-A3.2 `ceremony_backed_vouch`** — same shape via a co-presence ceremony fact.
- **T-A3.3 `free_floating_still_pending`** — no antecedent → pending forever; unchanged, re-asserted against the widened fold.
- **T-A3.4 `edge_backed_grade_explicit`** — existing edge-backed vouches carry `edge_backed`; RUN-ATTEST-01's Part 2 tests updated only where grade names appear, behavior otherwise byte-identical (assert the T-AT2.2 edge-fold-unchanged invariant still holds post-widening).
- **T-A3.5 `antecedent_class_is_governed`** — under-quorum widening refused whole; quorum-met widening visible in lineage; hard-stop inherited. Staged-ungoverned red per the refutation-pin style.

### A.2 Marker refinement (V2)

- Rename the presentation marker to name its antecedent kind: `edge_superseded`. 
- **T-A3.6 `only_edges_supersede`** — transaction- and ceremony-backed vouches can never acquire an antecedent-superseded marker (their antecedent kinds have no "ended" state — a completed transaction or held ceremony cannot be unhappened). Negative test over the fold surface.
- **T-A3.6b `withdrawn_is_absent_not_tombstoned`** — the Drystone-side half of V2's no-active-trace guarantee: an author-superseded (withdrawn) vouch or review is ABSENT from every corroboration structure — no tombstone field, no count, no "something was here" — while lineage retains the objects per T-AT0.3. Red by staging a `withdrawn` tombstone field on the response type (refutation-pin style); deleted at green. Absence-not-redaction is the T-AT3.3 pattern applied to withdrawal.
- PRIMITIVES gains the tier sentence pair (see A.4); no other code behavior changes.

### A.3 `unilateral_private` deferral note (V3)

No code. PRIMITIVES-ATTEST.md's `unilateral_private` entry gains: "Deferred from v1 (decided 2026-07-18). When it ships, it ships as a private-substrate artifact — an MLS-group-of-one object on the private tier — not as a fourth public consent mode. It precedes no private-tier logic; it inherits it." Zero tests remains the deliberate statement.

### A.4 PRIMITIVES + FINDINGS updates

- Antecedent class definition (is/is-NOT pair; the V1 rationale sentence about bidirectional vs unidirectional trust binding, attributed to the owner verdict of 2026-07-18).
- Retraction semantics per tier: Drystone-side = author supersede, lineage intact; public ATProto tier = author deletes/replaces the record in their own PDS, the canonical copy — cross-reference the Part B brief for the substrate grounding. Private remediation: one line, "different mechanism with its own logic, parked."
- FINDINGS: F-AT-5 cross-reference extended to the new antecedent-class register (amendments must causally chain — same rule, third register).
- MASTER-INDEX row updated to reflect V1–V3 settled and the brief landed.

## 4. Part B — `ATTEST-ATPROTO-MATCHUP.md` (research brief)

Lands in `alpha/experiments/` beside the prior briefs. Structure it as: required abilities → current ATProto matchup → placement decision per ability → gaps and correlators → lexicon sketch appendix. Every ATProto factual claim carries a primary-source anchor fetched in-session.

### B.1 Required abilities (enumerate from the design; each names the primitive/test it serves)

1. **Author-sovereign publish / delete / whole-replace of signed records at stable keys** — serves V2 retraction/amend on the public review tier.
2. **DID-anchored authorship with verifiable signatures** — serves every attestation's provenance floor.
3. **Cross-record reference by (DID, collection, rkey) and by CID** — serves the co-signed edge (two halves in two repos citing one canonical core) and antecedent citation (V1's class). The brief must state the join precisely: a TRUE bidirectional CID cross-reference is impossible (each CID would need to contain the other — circular), so the edge realizes as half A published first, half B citing A's CID, and BOTH halves carrying the shared core hash; the core-hash equality is the real join (T-AT1.2's rule), the CID citation is one-directional convenience. Naive "two records point at each other" is not implementable and must not be sketched.
4. **Custom record schemas under an owned namespace** — serves the closed vocabularies (consent modes, scopes, antecedent kinds, predicates). Sketch NSIDs under `ing.croft.attest.*` (reversed `croft.ing`).
5. **Aggregation/serving layer with viewer-aware responses** — serves the corroboration-structure query (no-scalar, viewer-relative). Map to AppView + the Stellin EXP-A viewer-aware serving machinery and appview-infra hosting.
6. **Subject notice delivery** — serves T-AT5.2's notice fact. Investigate honestly what exists at protocol level today vs app layer.
7. **Multiple independent accounts as personas** — serves anchor personas. One persona = one DID.
8. **Deletion semantics visible to the network** — what a record delete emits (firehose/relay events, tombstone behavior), what "no residue" means at each hop; serves the honest statement of V2.
9. **Private / permissioned data** — serves the persona-scoped edge tier, resolvability policies, and (eventually) `unilateral_private`. Establish current shipped state vs roadmap (the Permissioned Data direction and the community private-data work); this tier stays Drystone-side until the substrate ships it — state that as the placement.
10. **Account portability and key rotation** — serves persona root-keypair custody; note interaction with recovery (I9) without reopening it.

### B.2 Matchup table

One row per ability: **native today / partial / roadmap / absent**, with the anchor, and the placement decision: `atproto-native now` | `AppView-layer (Croft)` | `Drystone-side until substrate ships` | `product-layer`. The two-tier architecture statement follows the table: public tier (reviews, open vouches, referrals) native now; persona-scoped tier (edges, resolvability, scoped disclosure) Drystone-side; bridge points named.

### B.3 Gaps and correlators (the honest section)

- **The PLC directory correlator**: the identity directory's public operation log and what it timestamps — analyze whether sibling anchor-persona accounts created near-simultaneously correlate through it, in the same spirit as F-PA-3's mint-lamport finding. Analyze, do NOT decide mitigations (candidate mitigations listed for the owner: creation spacing as product guidance, alternative DID methods, issuance decoupled from account creation). This is expected to become **F-AT-6**.
- **Delete visibility**: if deletes are broadcast events, third parties that logged the firehose retain both the record and the fact of its deletion — reconcile this honestly with V2's "effectively no residue at the authoritative layer" (the claim survives; the brief bounds it).
- **Notice delivery gap** (if B.1.6 finds no protocol-level path): notice facts serve at the AppView/product layer; stated plainly.
- **Private-data gap**: whatever B.1.9 finds, the persona-scoped tier's Drystone-side placement is restated with the substrate condition under which it could migrate.

### B.4 Lexicon sketch appendix (DRAFT, non-normative)

Sketch `ing.croft.attest.review`, `.vouch`, `.edgeHalf`, `.transactionFact`, and `.credentialPresentation` as draft lexicon shapes — enough to show the mapping is real, marked DRAFT-PENDING, no claim of finality, and explicitly NOT registered or published by this run.

## 5. Definition of green

Part A tests green red-first (T-A3.1 red captured against pre-change fold behavior); full crate suite green including the updated Part 2 tests; T-AT0.* floor invariants re-green across the widened types; pure workspace clean, clippy clean on touched code; the three authorized tag edits made and no others; PRIMITIVES + FINDINGS + MASTER-INDEX updated; Part B brief present with every B.1 ability rowed in B.2, every ATProto claim anchored with retrieval date, the B.3 correlator analysis present (F-AT-6 filed if confirmed), and the appendix marked DRAFT; site gate green if docs tree touched.

## 6. Run summary requirements

Red-to-green evidence per part; the T-A3.1 old-behavior red called out explicitly (it documents the semantic change V1 authorizes); the B.2 table reproduced in the summary; any ability whose matchup status surprised expectations flagged for the owner; FIX vs FINDING; deviations with reasons. Status: Part A `Modeled` (reused R7 legs cite existing grades for the reused portion only); Part B claims graded per source band.

## 7. Drop order (if constrained)

B.4 lexicon appendix → A.1's governed-register leg T-A3.5 (keep the widening itself + T-A3.1–3.4) → B.3 depth (keep the PLC correlator analysis; it is the highest-value unknown). Part A.2/A.3 and the B.2 table are the spine; do not drop.

## 8. Owner items this run will surface (not decide)

- The PLC correlator finding (expected F-AT-6) and its mitigation menu.
- Any B.2 row that lands "absent" where the design assumed "native" — each becomes a named owner item.
- Whether the lexicon sketches proceed toward registration (a Croft product-layer decision, adjacent to the Stellin work, not a discovery decision).

## 9. Addendum (same-day reconciliation — two additions, both binding)

- **B.1.3 gains a load-bearing sub-check: canonical-form compatibility.** The "first-class integration" claim quietly assumes the crate's §4.6 canonical dag-cbor form and atproto's deterministic CBOR rules agree — if they diverge, core hashes computed by `attest-family` will not match atproto CIDs for the same payload, and every cross-tier reference silently breaks. Verify against the anchored data-model spec text (map key ordering, integer/float encoding, CID form), state the verdict explicitly in B.2, and if they diverge, name the translation boundary rather than papering over it. If Part C-style offline verification is feasible (encode one fixture payload both ways, compare), do it and cite it; otherwise the spec-text comparison stands alone, graded accordingly.
- **Evidence archiving (audit follow-up from the 01/02 pass).** Commit the captured RED and GREEN outputs under `attest-family/evidence/` (`attest3-red-*.txt`, `attest3-green-*.txt`) rather than digests alone — the 01/02 digests were attested but not independently re-verifiable. Forward-only; do not retro-manufacture 01/02 artifacts. Summary cites file paths.
