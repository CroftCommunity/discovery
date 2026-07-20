# RUN-ATTEST-04 — Settlement Riders (V4–V10): Closing the Walk

Status: INSTRUCTION — for execution via Claude Code in `CroftCommunity/discovery`

Gate: RUN-ATTEST-03 merged (audited 2026-07-18 against snapshot `17discovery-main`). The 2026-07-18 owner-call walk is COMPLETE, ten of ten. This run carries verdicts 4–10 into `attest-family` and its docs. After this run, zero `pending` OWNER-CALL tags remain in the attest lane.

---

## 1. The verdicts being settled (owner decisions, confirmed in chat 2026-07-18; do not relitigate)

- **V4 (resolvability stand-in default).** Graded-by-default on the scoped tier: counterparts of a standing co-signed edge can resolve you; strangers get cardinality only. OPEN is a deliberate per-persona posture (motivating case: workplace personas for reading structure). Reviews assert experience, not relationship; public-tier discoverability is untouched by this default.
- **V5 (PA OC-1, issuer public lineage).** Per-epoch **signed tree heads over keyed commitments** replace the per-credential public receipt pile (CT/RFC-9162 shape). Confirmation = **holder-stapled inclusion proofs**; the verifier never contacts the co-op. Revocation = per-epoch superseded set. Publication's value claim is re-graded: consistency + holder self-verifiability + epoch-grain volume, **never** noncoercion or process truth.
- **V6 (PA OC-2, sibling batching).** Ledger posture ratified as inherited from V5 (canonical-order leaves, epoch grain, nothing finer published). Ceremony spacing is **user guidance**: the co-op informs ("minting siblings in one sitting clusters them in the PLC log") and honors either choice without friction. Plus the **era-anchoring move**: issuer operational time is governance time — key rotations, tree-head epochs, and register changes are era'd governance-lineage facts; the co-op is literally a Drystone group.
- **Era-reissue (owner refinement extending V5–V6).** On governance changes, a holder may request reissue under the new era with ONLY the holder signing the request. Old-era credentials never expire; silence carries no penalty. Membership becomes era-graded, "meaningful but factual."
- **V7 (sole_anchor(context)).** REJECTED, not deferred. Uniqueness is group-local membership vetting (a co-op's quorum counts member handles, one per vetted ID at its own chosen vetting level; personas are signing instruments), never a portable credential. Portable proof-of-personhood is the eating-the-spider escalation; the valuable, less invasive question is already `vetted_holder`.
- **V8 (PA OC-4, fee semantics).** No protocol surface; co-op policy. Fee attaches to the vetting event and the mint act. **Era-reissues are free** — structurally pinned: reissue requires no new vetting antecedent. Guidance recorded: publish the fee schedule in the co-op's own governance lineage; bootstrap with a simple generous offering (third-party or light-tier vetting is honest because process provenance names the mechanism, and re-vet + reissue upgrades later).
- **V9 (F-PA-3 graduation).** PRIMITIVES gains the generalized rule: *any monotonic, published, infrastructure-side ordering is a correlator; infrastructure publishes at era grain or not at all*, with a pointer to F-PA-3. The FINDINGS entry stays untouched.
- **V10 (AT OC-1, PRIMITIVES home).** Stays in alpha beside the crate. The graduation trigger is written into the doc header as a NAMED condition: graduates to the beta spec tree at the attest lane's release pass, defined as (a) this settlement rider landed, (b) grades re-evaluated, (c) lexicon drafts promoted or explicitly versioned. No thin beta summary is ever created (one design, one document — the fold_auth lesson).

## 2. Standing directives

All RUN-ATTEST-01 §2 directives carry over binding: TDD red-first with red-to-green evidence per part (staged-violation reds, refutation-pin style, for invariants green at birth); cryptographic ordering, never wall-clock (the tree-head cadence is the seed-rule shape, epoch roll OR N facts — no timer anywhere); canonical dag-cbor via existing machinery; supersede-never-revoke on Drystone-tier objects; reuse over reimplementation (R7 machinery via the substrate, never shadowed); A.9 evidence discipline with `(evidence: …, RUN-ATTEST-04[, grade])`; frozen record; site gate if the docs tree is touched; when-in-doubt-Modeled.

**Named conditional edits (the ONLY tag moves authorized):** PA OC-1, PA OC-2, PA OC-3, PA OC-4 `pending` → `DECIDED (V-n, 2026-07-18, owner-confirmed in chat)`; the resolvable-to-all stand-in flag in the RUN-ATTEST-01 deviation note gains a pointer to V4 (the note itself is frozen — the pointer lands in PRIMITIVES/FINDINGS, not by editing the frozen summary); AT OC-1 in PRIMITIVES → `DECIDED (V10, …)`. Nothing else moves.

## 3. Part A — V4: the graded resolvability default (red-first)

Replace the resolvable-to-all stand-in with the graded default. Semantics: a viewer resolves a persona named at the far end of an edge iff (a) the viewer holds a standing co-signed edge with that persona, or (b) that persona's policy explicitly opens further. Strangers receive cardinality only. Silence (no policy act ever taken) IS the graded posture — zero configuration required.

Tests:

- **T-A4.1 `fresh_persona_graded_by_default`** — THE red for this part, captured against the old default: a fresh persona with zero policy acts is, to a stranger's full sweep, present only as cardinality; every resolution attempt is absent-not-redacted (T-AT3.3 pattern). Under the pre-change default this fails loudly; capture it.
- **T-A4.2 `counterpart_resolves`** — a standing co-signed edge makes each side resolvable to the other, and ONLY to each other, with no policy act.
- **T-A4.3 `open_is_a_posture_supersede`** — opting a persona open is a policy supersede with lineage: deliberate, visible in that persona's own record, reversible by further supersede. The workplace-persona fixture exercises it end-to-end.
- **T-A4.4 `cardinality_still_leakless`** — re-run T-AT3.5's serialization fuzz under the new default: the mutual count is now the stranger-facing tier, so the no-identity-leak property is re-proven where it now matters most.
- **T-A4.5 `sweeps_remeasured`** — re-run the T-AT4.3 and T-PA2.1 correlation-resistance property sweeps under the graded default (they were measured under resolvable-to-all). Expected: equal or stronger; any weaker case is a FINDING, not silently absorbed.
- PRIMITIVES gains the default's definition with the kinship sentence: this default and the CONTACT.md contact-policy family are siblings — per-scope consent dials where silence renders pending, never assent; a stranger who wants more than cardinality has a lawful path, and it is the knock.

## 4. Part B — V5 + V6 + era-reissue: the issuer transparency rework (red-first)

This part replaces machinery, so it is the run's center of mass.

### B.1 Tree heads over keyed commitments

- Per governance era, the issuer holds a confidential commitment key. **Candidate derivation implemented as the modeled choice** (flagged in-code as owner-revisitable, not an OC): `key_e = KDF(coop_secret, era_anchor_e)` where `era_anchor_e` is the governance-lineage fact opening era *e*. Rotation therefore rides era rolls by construction; a leaked key exposes exactly one era.
- Each issuance mints `commitment = HMAC(key_e, credential_id)`. Leaves sort in canonical order **by commitment value** (mint order structurally absent, inheriting T-PA1.4's intent). Per epoch, the issuer publishes ONE signed tree head: Merkle root + leaf count + superseded-set root + era anchor reference. Cadence is the seed-rule shape (epoch roll OR N facts); no wall-clock input exists anywhere in the pipeline (extend T-AT0.4's scan over the new module).
- The per-credential public receipt pile and its unordered-set fold are REMOVED (supersede-never-revoke governs Drystone-tier *objects*; test scaffolding and issuer-internal structures are code, and code changes by commit — state this in the summary so nobody reads the removal as a lineage violation).

Tests: **T-A4.6 `tree_head_is_the_only_publication`** (public issuer surface = signed heads, nothing per-credential; staged-violation red: a per-credential receipt on the public surface); **T-A4.7 `leaves_canonical_by_commitment`** (permuted mint orders → identical tree; the F-PA-3/V9 rule as behavior); **T-A4.8 `keyed_commitments_resist_dictionary`** (an outsider holding a guessed credential_id cannot confirm membership without the era key); **T-A4.9 `head_cadence_is_governed`** (cadence parameters live on the reused R7 machinery; under-quorum change refused whole; hard-stop inherited).

### B.2 Stapling replaces the status check

- At mint, the holder receives their inclusion proof. Verification is a pure function: `verify(head, proof, credential) -> bool`. The T-PA6.3 XRPC-shaped status-check endpoint is REWORKED into (a) head publication and (b) that pure verify — the issuer-facing verifier query surface is deleted, and with it the (verifier, subject) capture leak.
- Supersession: a superseded credential's commitment enters the epoch's superseded set; a holder cannot staple a *fresh-head* proof for it (**T-A4.10 `superseded_cannot_staple_fresh`**). Freshness requirements remain app policy, fail-closed as always — restate, don't re-derive.
- **T-A4.11 `verifier_never_contacts_issuer`** — API-surface negative test in the T-AT5.4 style: no public operation accepts a verifier query naming a subject; the F-AT-4 allowlist pin will fire on the new operations — review and allowlist with notes, the established flow.
- **T-A4.12 `holder_verifies_own_inclusion`** — the promised pin, now in its final form: holder + proof + published head verifies; tampered head or proof fails.
- **T-A4.13 `audit_over_heads`** — T-PA6.2's covenant audit reworked to the tree model: per-epoch leaf counts sum, head signatures verify, superseded sets well-formed, contiguous era references — zero identities resolved; tampered fixture caught.

### B.3 Era-reissue

- A holder-signed reissue request (unilateral — only their signature) cites their existing credential + the new era anchor as antecedents. The issuer's reissue supersedes the old credential, **chains the original vetting event** (no new vetting antecedent — this is also V8's free-reissue structural pin, one test serving two verdicts: **T-A4.14 `reissue_chains_vetting_no_new_antecedent`**), and mints a fresh commitment under the new era key.
- **T-A4.15 `cross_era_commitments_unlinkable`** — old and new commitments for the same holder share no derivable correlator in the full public sweep (extends T-PA2.1 across eras).
- **T-A4.16 `no_standing_computation`** — the guard with teeth: the fold exposes era facts only (issued-under, last-reissued-under, holder-requested). No type, field, or derivable value expresses active/lapsed/current/in-good-standing (compile-boundary + serialization scan, the T-AT0.2 pattern extended with the standing vocabulary). Red by staging a `current_member: bool`; delete at green.
- PRIMITIVES: the era-reissue definition, the silence sentence (old-era credentials are valid facts forever; re-affirmation is voluntary; silence carries no penalty), and the meaningful-but-factual guard (the protocol supplies the factual half; meaning is human).

### B.4 Docs for Part B

- PRIMITIVES: the tree-head model; the value-claim sentence verbatim in spirit — *publication proves consistency, holder self-verifiability, and epoch-grain volume; it cannot and does not prove noncoercion or that vetting truly happened; process honesty lives in governance and the covenant*; the co-op-as-Drystone-group sentence (its governance lineage is the era spine; issuer epochs, key rotations, and registers are era'd governance facts; its co-signed tree heads are horizon checkpoints of its own group); the V6 user-guidance sentence (ceremony spacing is the user's informed choice; the co-op informs and honors either).
- ATTEST-ATPROTO-MATCHUP.md Row 7 updated to the stapling model, dated as a V5 revision (briefs are living docs; the prior text stays visible in git history, no frozen-record issue).
- Lexicon draft `ing.croft.attest.commitmentEpoch` superseded by a `treeHead` draft shape (DRAFT, non-normative); `atproto_map` round-trip (T-A3.7 family) extended to it.

## 5. Part C — V7: the recorded rejection

- PRIMITIVES `sole_anchor(context)` entry converts from "defined, not built" to a **recorded rejection** carrying the rationale: only one-credential-per-ID yields real uniqueness and anything short is gameable; uniqueness is therefore group-local membership vetting under local authority, never a portable credential; portable proof-of-personhood is the escalation the design refuses ("eating the spider to eat the fly"); the valuable, less invasive question — one real vetted human behind this persona — is `vetted_holder`, already built. Same treatment full-issuance-facts received: the rejected pole stays findable with its reasoning.
- One sentence lands beside the quorum machinery docs: **governance counts member handles (one per vetted ID at the group's own chosen vetting level); personas sign.**
- T-PA5.3's unaskable pin: remove any carve-out phrasing referencing a future sole_anchor; the pin now stands unqualified. Tag resolution per §2.

## 6. Part D — V8 + V9 + V10: PRIMITIVES settlement

- **V8**: the fee-posture paragraph — no protocol surface; fee attaches to vetting event + mint act; era-reissues free (structural pin = T-A4.14); guidance: schedule in the co-op's governance lineage; bootstrap posture (light tiers are honest facts via process provenance; re-vet + reissue upgrades). Tag resolution.
- **V9**: the graduated rule, generalized wording exactly as decided, with the F-PA-3 pointer. FINDINGS untouched.
- **V10**: the PRIMITIVES header gains the graduation trigger as a named condition (§1 wording). AT OC-1 tag resolution.
- The owed sentence from the RUN-ATTEST-03 audit lands in the matchup brief or PRIMITIVES (executor's choice, cross-referenced): *the crate's canonical dag-cbor form is the source of truth for core hashes; lexicon records embed core content rather than crate-computed hashes, so no cross-encoder hash equality is ever required; atproto CIDs are locators, never joins.*
- MASTER-INDEX row updated: walk complete, V1–V10 settled, zero pending OCs in the lane.

## 7. Definition of green

Parts A–D tests green red-first (T-A4.1's and T-A4.6's reds captured against pre-change behavior; staged reds deleted at green); the six named tag moves made and no others (grep audit in the summary); prior suite green — expected churn is confined to: resolvability-default-dependent tests (updated to graded, each change named), the receipt-pile/status-check tests (replaced by B.1/B.2 successors, each replacement named old → new), and the sole_anchor carve-out phrasing; T-AT0.* floor invariants re-green including the wall-clock scan over the new issuer module; PRIMITIVES carries every §3–§6 addition with is/is-NOT pairs maintained and the V10 header; matchup brief Row 7 revised and dated; lexicon treeHead draft present with round-trip; pure workspace clean, clippy clean on touched code; site gate green if docs tree touched.

## 8. Run summary requirements

Red-to-green evidence per part; the removed-machinery statement (§4.B.1) verbatim; T-A4.5's remeasured sweep results stated explicitly (equal/stronger, or the FINDING); the F-AT-4 allowlist extensions with notes; the tag-audit grep output; deviations with reasons; confirmation that the walk's ledger reads ten of ten with zero pending.

## 9. Drop order (if constrained)

T-A4.15 cross-era breadth (keep the test, narrow the property sweep) → B.4 lexicon treeHead round-trip (keep the draft file) → T-A4.5 breadth (keep T-AT4.3's re-run, defer T-PA2.1's). Parts A core, B.1–B.3, and C are the run's reason to exist; do not drop.
