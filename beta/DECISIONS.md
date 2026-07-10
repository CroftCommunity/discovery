# discovery / beta — decisions register (pitch-resolution index)

date: 2026-07-09

**Status / Register.** This is a **pitch-resolution index over the reasoning, never a replacement for
it.** Every row states a decision at its tersest resolution and links *down* to the beta layer doc (and
section) where the argument is carried whole, at library resolution. It obeys the anti-rollup rule in
`LAYERS.md` → "Reasoning travels with the decision (the anti-rollup rule)": a settled decision recorded at
a terse resolution must link to its full reasoning, and that reasoning must live in a beta doc — recording
a decision without its grounding is the failure this register exists to prevent. Nothing here re-argues a
decision or resolves an open gate; both would break the index-not-replacement discipline.

## How to read

- **Settled vs walked.** Section 1 is decisions already made (with the date decided). Section 2 is the
  decision gates the user **walked in the 2026-07-09 open-gate walkthrough**: each now carries the call
  made plus its **residual** (an empirical param, a prototype mechanism, a trademark/legal clearance, or a
  product-layer control). A residual is *surfaced, not resolved* here — the reasoning home carries it.
- **Every row links to its reasoning home.** The "Reasoning home" column names a beta layer doc and the
  section that carries the full argument (the library resolution). To act on or re-derive a decision, read
  the home, not this row.
- **A decision you cannot re-derive from its linked home is a defect, not an entry.** Where a decision's
  reasoning is not yet carried in any beta doc, it is named in the closing "Reasoning-gaps" note as a
  recovery residual rather than given a settled-decision row it cannot support.
- **Markers travel.** `NOT-LEGAL-ADVICE` and `[UNVERIFIED]` flags are preserved on the rows they qualify;
  the linked home carries them inline too.
- The `A#` / `C#` labels are stable cross-reference identifiers for each decision, not file paths.

## Decisions taken 2026-07-09 (open-gate walkthrough)

The user walked the open gates and made these calls. Reasoning homes are the docs named; several
deliberately lock *principles* and send the *mechanism* to a prototype (per the anti-rollup rule, the
open mechanism keeps a reasoned home in OPEN-THREADS / the spec open-items).

- **A1 — ACCEPTED.** The MPL-2.0 `hpke-rs` dependency is accepted (mandatory for RFC 9420 HPKE; weak
  file-level copyleft, compatible with the AGPL reference code). Attorney confirmation folded into the
  legal-review gate. → `governance/open-publication-and-ip-stewardship.md` (license posture).
- **A2 + A12 — DECIDED (principles); mechanism → prototype.** The **meer is always blind** (invariant, never
  holds usable keys). Recovery is a **separate custodial role** that holds the needed key material accessed
  **only under defined conditions** (conditional/break-glass, not standing read). It **composes** — a
  social-recovery **quorum and/or a designated custodial delegate** — with a **group-level default** plus an
  optional **per-user** designation, over a self-custody floor. The exact conditions, the quorum/delegate
  composition, and the group-default shape are worked out in the **recovery-anchor prototype**. The A10
  rotation key folds into this custody. → `drystone-spec` §7.3.9 + the recovery workstream.
  - **Refined 2026-07-09 (three-case model).** Recovery decomposes into three cases with different mechanisms:
    **Case 1** (lose a device, others remain) = *not recovery* — the two-phase BAN of the lost client.
    **Case 2** (lose all devices, backup exists) = recover the backup → **rotate the lineage forward** →
    rejoin as the same principal; the backup lives in a **pluggable target — QR/printed-sheet (air-gapped),
    a file export, or the PDS encrypted-blob-vault (the provider-recovery analog, blind: ciphertext-only)** —
    unlocked by passphrase and/or quorum. **Case 3** (parent key gone, no backup) = irreducibly **social**:
    a group quorum **vouches** a new lineage is persona X; feasibility is **tiered by social closeness**
    (close groups can, large/anonymous cannot → new persona). This **resolves the VSS-vs-FROST question as a
    per-case split, not a choice: `vsss-rs`-style verifiable secret-reconstruction for the Case-2 unlock;
    `frost-ed25519`-style threshold-authorization for the Case-3 vouch** (`[verify before committing]`).
    Insight behind it: the persona's parent keypair (lineage) is the anchor, so recovery is *rotate the
    lineage forward*, not reconstruct an old key — which is why Case 2/3 never re-materialize the "one key =
    the whole person" single point of compromise. Full designs: `../alpha/plans/2026-07-09-proof-experiments-a11-and-recovery.md` (E-REC.0–.5).
  - **Refined further 2026-07-09 (recovery group + the safety ladder).** The Case-2 quorum is a **recovery
    group**: the KEK is split across **n share-holders** recovered by **threshold-decryption** (k-of-n unlock
    without reassembling the secret; verifiable). The share-holders are **pluggable — people, the user's own
    home devices (scatter-stash), or a mix, k-of-n over the *union*** (a house fire is survivable via friends;
    friend-collusion is insufficient without the home devices). A **second factor** (k guardian shares **and**
    the user's passphrase) is the **collusion defense** — the group is necessary but not sufficient. Recovery
    paths form a **difficulty = safety ladder** (device → home-scatter → kept-artifact → social-group →
    +second-factor → Case-3 vouch); the user/group picks a rung. The **irreducible law:** you cannot maximize
    both "get back with nothing" and "no group can impersonate me" — the second factor trades one for the
    other. **Status: this is now design-complete enough to move to experiments** (E-REC.0/.1/.5 on the resumed
    `croft-chat` core); the remaining params (k, n, the rung default, the second-factor composition) are
    empirical, to be settled from the prototype, not by more design.
- **A11 — SPIKE THEN DECIDE.** Run a **revocation-immediacy spike** (Track-A epoch window vs the
  moderation/ban-evasion threat model) and pick **Track A (Meadowcap)** vs **Track B (Keyhive)** from data.
  Critical path to the v0.1 DOI. → engineering-validation plan.
- **A9 — DECIDED.** Stable **logical URI** (mutable content, committed-resolvable), **plus a portable,
  self-held offline proof-of-prior-identity** so a location change / domain fold preserves the provable link
  (buildable on the did:webvh SCID + pre-rotation chain + the `alsoKnownAs` ladder). →
  `cairn/cross-platform-identity-provenance.md`.
- **A10 — DECIDED.** **Pluggable** rotation-key custody with a **safe default**; **PDS-held-as-default is an
  acceptable governance decision**; **self-controlled** is the available sovereignty option; both persist
  long-term. A9's portable proof preserves credible exit regardless. → `cairn/cross-platform-identity-provenance.md`.
- **A13 — DEFERRED.** Retire "geer" eventually; placeholder **"gating role"** (name by capability). Not blocking.
- **A5 — KEPT as product-design item (T51).** App-layer, bounded by voice = assert-not-compel; not blocking.
- **Noria — ADOPTED as working foundation name, pending trademark clearance.** → `governance/foundation-cooperative-and-sustainability.md`.
- **Cooperative legal-review — DEFERRED to Phase-1 entity formation.** NOT-LEGAL-ADVICE; carry reasoning, not citations.
- **T4 — PARTIALLY DECIDED.** **Croft = the app/product/consumer brand** (Croft-forward); the **reference
  implementation is neutrally branded** (distro-style separation), NOT Croft — confirms the vendor-neutral
  `drystone-*` tag direction (spec Part 2 Appendix B). Open: the neutral ref-impl name + taglines/voice. →
  `governance/` + `socialization/brand-and-voice.md`.
- **Badge-teeth (load-bearing-few) — DECIDED.** The "Speaks Drystone" badge asserts **technical
  conformance**; the load-bearing principles get teeth by being **encoded as conformance requirements**
  (no subjective adherence-gatekeeping); non-encodable principles stay aspirational. Mandate for the
  conformance harness. → `governance/foundation-cooperative-and-sustainability.md` (mark) + engineering plan.
- **C15 — DECIDED (closed).** Leave "evidentiary, not operational" as a **per-conclusion**; do **not** elevate
  it to a named Tier-1 principle (keep the principle set lean).
- **Revocation is two-phase — DECIDED (design shape; 2026-07-09).** Because all state/representation is local,
  revocation splits into **phase 1 — experiential** (the ban fact crossing the governance threshold makes every
  client auto-ignore the party immediately, free, ATProto-`block`-shaped: experience-shaping, not crypto
  exclusion) and **phase 2 — cryptographic** (key loss at an epoch roll). An **always-available
  force-immediate-roll lever** collapses the two for the severe case (paying rekey cost); the **default phase-2
  timing tiers on the force-roll cost = f(group size, connectivity)** — small/well-connected ≈ immediate,
  hundreds+/disconnected = slow natural roll by default. **Honesty caveat:** between phases the party can still
  *decrypt* new state, so UX/governance must be explicit and severe cases use force-roll. This carries the
  E-A11.0 immediacy requirement at phase 1, so it **pushes A11 → Track A near-certain** (Track B not required).
  Reasoning home: `drystone-spec/part-2-certifiable-design.md` §7.6.12 (two-phase revocation: experiential
  immediacy, cryptographic exclusion, the force-roll lever, cost-tiered default, and the honesty caveat);
  the E-A11.0 threat-model / spike methodology is in
  `../alpha/plans/2026-07-09-proof-experiments-a11-and-recovery.md`. Couples T3 moderation and
  `fenced/app-store-survivability-and-abuse-posture.md`. Mechanism numbers
  (the ~50 boundary, the loose-tier bound) confirmed in E-A11.A.

*(Propagation into the individual docs/spec sections/plan follows this record; until propagated, this block
is the authoritative statement of the calls.)*

## Section 1 — Settled decisions

| Decision (pitch resolution) | Status (decided) | ID | Reasoning home (beta doc → section) |
|---|---|---|---|
| Neutral **reference implementation + product code = AGPL-3.0-or-later + DCO** (network-copyleft: forkable but never closeable; AGPLv3 §11 preserves the patent grant Apache-2.0 had offered). | decided 2026-07-09 | C13 | `governance/foundation-cooperative-and-sustainability.md` → "The two-body structure and its three decoupled layers" + "AGPL-3.0-or-later plus DCO: the structural-openness lock"; also `governance/open-publication-and-ip-stewardship.md` → "License posture (current project choices)" |
| **Spec text = CC0 1.0 Universal** (maximal "no one can claim or restrict the idea"; CC-BY 4.0 was the earlier alternative, not chosen). | decided 2026-06-25 | A14 | `governance/open-publication-and-ip-stewardship.md` → "License posture (current project choices)" |
| **The cooperative is the operating form** (the peer-standing argument concludes that only a cooperative, edge-free ownership form can constitute peer standing — the form is required, not preferred). | decided (argument settled) | — | `philosophy/peer-standing-and-the-cooperative-form.md` → §6 "Why the cooperative is the compatible form"; its manifestation → `governance/foundation-cooperative-and-sustainability.md` → "The two-body structure and its three decoupled layers" |
| **Two tiers of mark:** the **house mark** (Croft, licensed only to the canonical instance; a fork must rename) and a **"Speaks Drystone" compatibility badge** (the honest place for forks; starts as an honor system, not a formal certification mark). | decided (design intent) | — | `governance/foundation-cooperative-and-sustainability.md` → "Two tiers of mark: the house mark and the compatibility badge" |
| **Protocol name = Drystone** (taken from dry-stone-wall construction: equal-but-different stones, no load-bearing keystone). | pinned 2026-06-22 | A7 | `history/crofting-dry-stone-and-the-enclosure-inversion.md` → the dry-stone-wall passage (the material lineage of the name); the protocol meaning of the wall is argued in `philosophy/` |
| **Croft app Phase-0 imported and its shape proven** — shared functional core + thin per-platform shells, the five binding client decisions green-real across CLI / web / desktop. | import done 2026-06-22; architecture decided 2026-06-22 | A8 · E19 | `croft/product-the-garden-of-ponds.md` → the shared-core / thin-shell spine and its five binding decisions (green-real, Phase 0) |
| **Defensive-publication posture:** prior-art-first (a complete, enabling, third-party-timestamped disclosure is the load-bearing instrument, not the license), with a **Zenodo DOI + OpenTimestamps** as the timestamp vehicle (chosen over an IETF Internet-Draft). | decided (posture + vehicle) | A14 (vehicle half) · E31 | `governance/open-publication-and-ip-stewardship.md` → "Defensive-publication strategy (prior-art first)" + "The venue map (per-layer, then sequence) / Publication sequence" |
| **Persona vocabulary** — the identity model is *principal · client · persona* (a persona is the human-layer identity rooted in one cryptographic lineage; client-count and device-count are not persona-count). | decided (spec vocabulary of record) | — | `drystone-spec/part-2-certifiable-design.md` → §5.2 "Principal, client, persona: the identity model" + Appendix D "Term lattice and invariants (the vocabulary of record)"; persona as principle in `drystone-spec/part-1-reasoning-underpinnings.md` → §2 |
| **Governance model = append-only monotonic fold** (governance facts are entries, not mutations), a **timestamp-free causal-and-cryptographic total order** (no wall-clock in the ordering spine), **no state reset**, a **capped / delegable / revocable-by-succession root** (never infinite), and a **regress-free fold with attributable acceptance**. | decided (Design-stage, complete) | — | `drystone-spec/part-2-certifiable-design.md` → §7.3 "Governance facts are entries, not mutations" (incl. "The unconflictable root"), §7.3.1 "The total order … causal and cryptographic only, never temporal", §7.5 "Attributable acceptance and the regress-free fold" |
| **Committed hash = BLAKE3** — §7 is designed on BLAKE3 and §4 (proven on SHA-256) is re-based onto it, making SHA-256 the legacy side. | decided (committed; end-to-end re-proof pending) | — | `drystone-spec/part-2-certifiable-design.md` → §6 introduction (maturity paragraph) + §4.1 "cryptographic foundation"; the end-to-end BLAKE3 re-proof of §4 is tracked in Appendix B |
| **Peer-rights floor = tenure / voice / exit**, each fixed by what its removal forecloses; the floor is **evidentiary, not operational**; a claim on a Group's commons ("share") is **not** a right. | decided (floor; two verify-before-harden checks open) | E32 | `drystone-spec/part-2-certifiable-design.md` → §5.3 "Rights: the inherent, equal floor …"; lineage in `philosophy/the-peer-rights-razor-and-its-lineage.md`. (Open residuals: tenure-under-re-key and the `share` boundary — Section 2 / reasoning-gaps.) |
| **Large-group-scaling section placement** — folded into Part 2 as **§11** (resolving the §7 numbering collision). | decided 2026-07-07 | T37 | `drystone-spec/part-2-certifiable-design.md` → §11 "Large-Group Scaling, Dormancy, and Re-entry" |
| **Badge-teeth (load-bearing-few — genome vs strategy):** the "Speaks Drystone" compatibility badge asserts **technical conformance**; the load-bearing principles get teeth by being **encoded as conformance requirements** (no subjective adherence-gatekeeping), and non-encodable principles stay aspirational. Mandate for the conformance harness. | decided 2026-07-09 | — | `governance/foundation-cooperative-and-sustainability.md` → "Two tiers of mark: the house mark and the compatibility badge" + the conformance-harness mandate |
| **Anchor-URI stability + portable proof-of-prior-identity** — a stable **logical URI** (mutable content, committed-resolvable) plus a portable, self-held **offline proof-of-prior-identity** so a location change / domain fold preserves the provable link (did:webvh SCID + pre-rotation chain + the `alsoKnownAs` ladder). | decided 2026-07-09 | A9 | `cairn/cross-platform-identity-provenance.md` → the A9 surfacing (the `alsoKnownAs` equivalence ladder is evidentiary, not authoritative) |
| **Rotation-key custody = pluggable with a safe default** — **PDS-held-as-default is an acceptable governance decision**, **self-controlled** is the available sovereignty option, both persist long-term; A9's portable proof preserves credible exit regardless. | decided 2026-07-09 | A10 | `cairn/cross-platform-identity-provenance.md` → the rotation-key / A10 surfacing |

## Section 2 — Decision gates walked 2026-07-09 (calls made; residuals tracked)

| Decision to make | Status | ID | Blocked on it | Reasoning home (beta doc → section) |
|---|---|---|---|---|
| **Substrate license acceptance — MPL-2.0** (`hpke-rs` is mandatory for RFC 9420 HPKE; no permissive substitute). A compliance call, not code. | ACCEPTED 2026-07-09 — sign-off folded into the legal-review gate | A1 | Residual: attorney confirmation, folded into the cooperative legal-review gate. | `governance/open-publication-and-ip-stewardship.md` → "License posture" (the `hpke-rs` MPL-2.0 note); `governance/foundation-cooperative-and-sustainability.md` → the "Code" row |
| **Total-device-loss recovery anchor** — the concrete recovery/break-glass mechanism (custodian ladder / k-of-n guardian / time-delayed break-glass / survivor fork). Largest residual protocol risk. | principles DECIDED 2026-07-09 (three-case model + recovery-group / safety-ladder); mechanism → recovery-anchor prototype (k, n, rung default, second-factor composition are empirical) | A2 | Residual (empirical, not design): the guardian-set / break-glass / recovery-secret default params, settled from the prototype. | `drystone-spec/part-2-certifiable-design.md` → §7.3.9 "Principal recovery and break-glass" (three-case model, the difficulty-is-safety ladder, the threshold-decryption recovery group over the people+home-devices union, the second factor + irreducible law, and the Case-2 storage invariants are all now carried; only the empirical params — k/n, delay, contest window, KDF/threshold-scheme choices — remain prototype-pending) |
| **Capability mechanism — Track A (Meadowcap-shaped, delegated tokens) vs Track B (Keyhive-shaped, convergent membership graph).** Decided on the revocation-immediacy criterion. | SPIKE-THEN-DECIDE 2026-07-09 — narrowed to Track A near-certain (the two-phase revocation decision carries the E-A11.0 immediacy at phase 1) | A11 | The capability wire format, which in turn gates minting the v0.1 archival DOI. | `drystone-spec/part-2-certifiable-design.md` → Appendix A "Alternatives Considered" (the Track A/B entry) + §5.5 (the Meadowcap grounding) + Appendix B |
| **Key-custody default — blind-relay (Option A) vs revocable trusted delegate (Option B).** | principle DECIDED 2026-07-09 (A2/A12: the meer is always blind; recovery is a separate custodial role); mechanism → prototype | A12 | Device re-provisioning UX; and what, if anything, structurally resists Option B quietly rebuilding a readable homeserver. | `drystone-spec/part-2-certifiable-design.md` → §6.5.2 / §6.6.2 (the blind-relay / blind-meer roles) + §7.3.9 (recovery) |
| **Rename the "geer" gating peer** — gating is the one capability that bumps the read right; role names decompose to `floor + [capabilities]`. | DEFERRED 2026-07-09 — retire "geer" eventually; placeholder "gating role" (name by capability); not blocking | A13 | The read-gating capability and its role name. | `drystone-spec/part-2-certifiable-design.md` → §5.8.1 "Open item: gating against the read right" |
| **Foundation name — candidate Noria** (must be independent of the flagship name so the foundation's neutrality holds). `[UNVERIFIED]` trademark clearance. | ADOPTED 2026-07-09 as working foundation name; residual: trademark clearance `[UNVERIFIED]` | — | Residual: trademark clearance before the name hardens. | `governance/foundation-cooperative-and-sustainability.md` → "Entity phasing …" + "Decision-gated: surfaced, not resolved"; also `governance/open-publication-and-ip-stewardship.md` (closing note) |
| **Cooperative legal-review gate** — Missouri Chapter 351 LCA + foundation legal structure; statute sections, tax codes, fees are dialogue-sourced and need counsel. **NOT-LEGAL-ADVICE.** | DEFERRED 2026-07-09 to Phase-1 entity formation | — | Any entity filing or reliance on the specific §/fee figures. | `governance/foundation-cooperative-and-sustainability.md` → "Decision-gated: surfaced, not resolved" + "The four-pillar Social Union" |
| **Republish / "can still quote" UX control** — structural V3 is done; the human-layer control that lets a peer still quote is unspecified. | KEPT as product-design item (T51) 2026-07-09 — app-layer, bounded by voice = assert-not-compel; not blocking | A5 | The republish/quote product control at the app layer. | Philosophical grounding: `drystone-spec/part-1-reasoning-underpinnings.md` → §2 (voice = assert into your own record + reach willing peers, **not** compel amplification) + `philosophy/the-peer-rights-razor-and-its-lineage.md`. The UX control itself is app-layer and not yet carried (see reasoning-gaps). |

## Reasoning-gaps (recovery residuals — decision surfaced, reasoning not yet in a beta doc)

Per the anti-rollup rule, these are decisions or open user-calls whose full reasoning is **not yet carried
in any beta layer doc** (it lives only in the prior tier). They are named here as recovery residuals, not
given settled rows they cannot support:

- **A6 — vote-accumulation under churn/partition** (vote expiry, retraction, stale-vote handling). No beta
  reasoning home found; the governance-at-scale voting model is not yet consolidated into a beta doc.
- **S3 — quiet membership** (be in a group without exposing other edges) and **S4 — multi-identity, no
  forced linkage** (distinct lineages, no provable correlation). The spec establishes that personae are
  always distinct lineages (`drystone-spec/part-2-certifiable-design.md` → §5.0/§5.2), but the S3/S4
  *privacy properties as decisions* are not carried into a beta doc — they remain prior-tier open edges.
- **A8, the app-body IP/ownership call** — distinct from the completed Phase-0 import (settled above). The
  code-license posture is in `governance/`, but the specific ownership/assignment call for the imported app
  body is surfaced only in `README.md` → "Standing decisions", with no library-resolution reasoning home
  yet.
- **A2 recovery design** — now **homed in the spec** (`drystone-spec/part-2-certifiable-design.md` §7.3.9:
  three-case model, difficulty-is-safety ladder, threshold-decryption recovery group over the people+home-
  devices union, second factor + irreducible law, Case-2 storage invariants). No longer a reasoning-gap; only
  the empirical params (k/n, delay/contest window, KDF/threshold-scheme) are prototype-pending. The
  **proof-experiment methodology** (E-REC.0–.5) legitimately remains in the alpha prototype plan.
- **A11 capability track** — the two-phase revocation reasoning it depends on is now homed in the spec
  (§7.6.12), and the Track-A/B tradeoffs are carried in spec Appendix A + §5.5; A11 itself remains an open
  gate (Section 2), narrowed to Track A near-certain, awaiting the revocation-immediacy spike. The E-A11.0 /
  E-REC proof-experiment methodology legitimately lives in
  `../alpha/plans/2026-07-09-proof-experiments-a11-and-recovery.md` (a proof plan, not decision reasoning).

**Status ambiguities noted across sources:**

- **A14** is settled for the *spec text* (CC0 1.0), but A14's original *reference-code* half (Apache-2.0)
  is **superseded by C13** (AGPL-3.0-or-later + DCO). The register carries the current state; A14 should be
  read as the spec-text decision only.
- **A1 (MPL-2.0)** was **ACCEPTED** in the 2026-07-09 walkthrough (the technical constraint — `hpke-rs` is
  mandatory for RFC 9420 HPKE — was never in dispute); the only residual is the attorney acceptance sign-off,
  which is folded into the cooperative legal-review gate. It sits in Section 2 as a walked gate with that
  residual, not as an unresolved open gate.
