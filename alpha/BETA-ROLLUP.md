# Alpha → beta rollup ledger (the maturity-transition trace)

date: 2026-06-24

purpose: the auditable record of how `alpha/` content and semantics were rolled up into the `beta/`
synthesis — what was lifted from each source, how it was treated, and where it landed (or why it was
not pulled up). It lives **at the prior level (alpha)** on purpose: the audit trail sits with the
corpus it traces, so `beta/` stays a clean forward narrative and we can double-check coverage from here
rather than guessing whether something was missed or included unexpectedly.

**This ledger is now the *sole* home of the alpha→beta map** (as of 2026-06-25): the beta theme docs were
stripped of all prior-tier references — no `Sources (alpha)` footers, no `Provenance trace` lines, no
inline `thinking/…`/`research/…` quote attributions, no `COHESION §` pointers — so a beta doc reads clean
at its maturity level and the full source→treatment→landing trace exists *only* here. Lay the two side by
side to see what was pulled and where it went. (The per-quote internal-source attributions that used to
sit inline in beta are captured at the source→section granularity in the per-theme tables below.)

This is a **process/reflection artifact** (peer to `COHESION.md`, `RAW-ARTIFACTS-MANIFEST.md`), not
corpus content. It is the only addition to `alpha/` for the alpha→beta transition; existing alpha
corpus content stays frozen (PLAYBOOK §4). The repeatable *method* behind this ledger lives at the
discovery root in `../MATURITY-ROLLUP.md` (cross-stage), to be reused at beta→rc.

**Companion at the beta gate:** material that is queued for beta but **not yet settled** (DRAFT /
decision-gated / fact-unconfirmed) is held in `../beta/OPEN-THREADS.md` — a `deferred` item with its
gates named and a promotion target — so it is tracked toward beta without polluting the resolved
themes. This ledger records what **landed**; that one records what is **waiting on settling work**.
First entry: T1 — the Drystone governance & peer model (`thinking/drystone-spec/`).

## How to read this

For each beta theme: the one-line thesis, then a table mapping each alpha source to its **treatment**
and the beta section it **landed in**.

Treatment codes:

- **synthesized** — lifted and recomposed into the resolved account.
- **collapsed** — a contradiction/seam (often a `COHESION.md` entry) resolved into a single statement.
- **harvested** — a conclusion that was buried in an alpha transcript/doc, surfaced into the synthesis
  layer.
- **carried-flag** — verification status (verbatim quote, `green-real`, `[UNVERIFIED]`,
  `NOT-LEGAL-ADVICE`) carried forward unchanged.
- **excluded** — deliberately not carried (a do-not-carry item); the reason is named.
- **deferred** — real alpha material not yet pulled up; tracked here so it is not silently dropped.

Status legend per theme: **drafted** · **planned** (intended sources listed for coverage; trace filled
when drafted).

---

## 01 — epistemic foundation (N1 / G1) — **drafted** → `beta/01-epistemic-foundation.md`

Thesis: a distributed system can only establish provenance, never truth → compute provenance, never
utility; local-first is the generative premise; rediscovered across 2,400 years.

| alpha source | tag | treatment | landed in |
|---|---|---|---|
| `narrative/lineage-of-a-design-imperative.md` | S | **synthesized** (the 2,400-yr arc) + **carried-flag** (every verbatim quote + its appendix verification status reproduced whole) | §2.1–2.6 |
| `thinking/local-first-as-design-imperative.md` | S | **synthesized** — generative premise, two-part theorem, friction-as-diagnostic, design-for-conditions, rights-floor, federation | §3, §4, §5, §6 |
| `crystallized/principles.md` ("The deeper foundation" + Tier 1) | S | **synthesized** (razor, plurality-as-survival, the one boundary, equal-rights-of-all-shapes) + **harvested** (the razor as an explicit stated imperative, not just a section) | §1, §5, §6 |
| `seeds/transcripts/raw/croft-architecture-design-dialogue-2026-06-20.md` | R | provenance-only (raw origin; cited refs verified in-dialogue) | footer |
| `COHESION.md` §25 + `seeds/transcripts/raw/crypto-wars-to-p2p-pds-economics-dialogue-2026-06-22.md` (+FACTCHECK) | R | **synthesized** (Hush-A-Phone/Carterfone legal ancestor, CONFIRMED) + **excluded** (REFUTED Zimmermann/Meyer/"Voskop" fabrications) | §5; do-not-carry |

**Deferred (tracked, not dropped):** the alpha essay's "objection-by-objection toolkit" and "Further
reinforcements to pursue" (von Foerster's order-through-noise; Jane Jacobs; ecology's
diversity-stability literature) — supporting material, candidates for a later expansion of 01.

**Carried verification flags:** Socrates/Mill/Hayek/Ashby/Beer/Scott = Verified; **Peirce / Popper /
Ostrom(subsidiarity) = confirm against primary edition before publish.**

---

## 04 — the protocol we proved (N5 / G4) — **drafted** → `beta/04-the-protocol-we-proved.md`

Thesis: model a group as a navigable lineage; bind MLS ratchet + governance log + history CRDT; fork
cleanly, escalate genuine conflict to a human; green-real on openmls 0.8.1.

| alpha source | tag | treatment | landed in |
|---|---|---|---|
| `thinking/thesis-lineage-groups.md` | S | **synthesized** — thesis, two-tree model, survivor reconnect, invariants I1–I10, honesty boundaries | §1, §2, §3, §5 |
| `thinking/merge-split-corpus.md` | U | **synthesized** — split/merge/conflict taxonomy feeding conformance | §3 |
| `crystallized/CROFT-PROTOCOL.md` | S | **synthesized** (the Drystone wire spec) + **carried-flag** (proof status inline) + **collapsed** (the §2 spec-vs-code pre-image discrepancy surfaced as a real reconciliation item) | §2, §5 |
| `crystallized/proof-ledger.md` | S | **synthesized** + **carried-flag** — Phase-1 GO, I1–I10 status, adversarial gaps closed, cross-machine, conformance 66/0, media round | §3, §4 |
| `crystallized/test-narrative.md` | S | **synthesized** — reasoning over proofs (what each test means/leaves open) | §3, §5 |
| `crystallized/conformance-suite.md`, `crystallized/TEST-CORPUS.md` | S/U | **synthesized** — what a conformant impl must pass | §3 |
| `thinking/realtime-media-over-iroh.md` | S | **synthesized** — media-layer design (str0m/RoQ/MoQ, blind SFU meer) | §4 |
| `ROUND-2026-06-17-media-meer-conformance.md` | U | **synthesized** — E10/E12/E11/meer plain-language results | §4 |
| `seeds/transcripts/design-dialogue-2026-06-13-to-14.md` | R | provenance-only (richest single seed, preserved-verbatim) | footer |
| `COHESION.md` §1 (crypto gate), §2 (V3 limit), §3–5 (roll-up/broker/public-path), §12 (recovery) | — | **collapsed** — seams resolved into the proof account + open-risks | §3, §5 |

**Carried verification flags:** Phase-1 gate GO + I1–I10 + multi-device + cross-machine + conformance =
green-real/green-model as in the ledger; media E12 = green-real (synthetic frames); E10/E11 =
characterized; total-device-loss recovery = **open (largest residual risk)**; V3 = green-model for
automatic crossing only; S3 quiet membership = spec/unsolved. iroh facts cite the SoT FACTCHECK.

---

## 02 — enclosure and its inversion (N2 / G3a+G1) — **drafted** → `beta/02-enclosure-and-its-inversion.md`

Thesis: every era's commons gets enclosed; the croft is the rare halt (plot + surviving common); the word
names both the trap and the balance.

| alpha source | tag | treatment | landed in |
|---|---|---|---|
| `narrative/verticals/croft-the-name-and-the-commons.md` | S | **synthesized** (etymology + inversion register; trap-and-balance reconciliation) | §1, §2, §7 |
| `NAMING.md` | S | **harvested** (Croft + Drystone SETTLED rationale; the Princeps Problem) | §3, §4, §7 |
| `seeds/.../croft-crofting-research.md` | S | **synthesized** (de-romanticized ground truth; canonical register) | §3, §4, §7 |
| `seeds/.../croft-crofting-narrative.md` | R | **carried-flag** (quotable colour; every quote `[UNVERIFIED]` tertiary) | §4, §5 |
| `seeds/.../croft-clare-enclosure-poems-2026-06-23.md` | R | **harvested** (public-domain primary texts, copied exact) | §3, §5 |
| `seeds/.../croft-etymology-enclosure-tradition-dialogue-2026-06-23.md` | S | **synthesized** (MED specifics, verb-1772, commons-rebellion tradition, global pattern) | §1, §2, §5 |
| `research/social-platform-cycle.md` | U | **harvested** (historical half: enclosure recurs into the digital era). Prescriptive half **deferred → 07** | §6 |
| `seeds/.../p2p-architecture-origin-dialogue.md` | R | **excluded** from body; one fragment (the maintenance-phase hinge → 07) | boundary call |
| `seeds/.../groundmist-hive-identity-chain-iroh-games-dialogue-2026-06-22.md` | R | **carried-flag / mostly excluded** (net-new = Hard Fork 23 ≈ $6.3M / 23.6M STEEM, NOT $5M) | §6 |
| `COHESION.md` §14, §16, §24, §30, §34 | — | **collapsed** (name↔commons; research-vs-narrative drift; Drystone naming; trap-and-balance) | §2, §4, §6, §7 |

**Excluded (do-not-carry):** the "ancient free clan" heroic myth (use research file as truth, narrative is
colour); the "noble chief betrayed by outsiders" myth; the genocide/ethnic-cleansing framing as *settled*
(it is a live scholarly dispute — surface as contested, do not adjudicate). **Surfaced `[UNVERIFIED]`:** the
1772 Manchester Directory verbatim sentence; all narrative-file colour quotes; the "Magna Carta of…"
phrasing/attribution variants; Winstanley primary-text vs corpus-variant wording.

---

## 03 — the living ecosystem (N3 / G3b) — **drafted** → `beta/03-the-living-ecosystem.md`

Thesis: against the live field, Croft's public-DID + private-MLS-blind-to-host bet is "different, not
weaker," and atproto's own deferral of E2EE sharpens it.

| alpha source | tag | treatment | landed in |
|---|---|---|---|
| `ECOSYSTEM.md` | S/INDEX | **synthesized** (field register); dialogue-sourced rows **carried-flag** | §3–§5, §7–§9 |
| `research/messaging-solutions-landscape.md` | S | **synthesized + harvested** (the universal trade; SSB cautionary tale; differentiators) | §1, §2, §5 |
| `research/discord-dominance.md` | S | **synthesized**; Discord financials **carried-flag** (`[UNVERIFIED]`) | §7 |
| `research/public-social-protocols.md` | S | **synthesized + harvested** (dual-use identity anchor; public-by-default forces split) | §3, §4 |
| `research/atproto-private-data-architecture.md` | S | **synthesized** (atproto direction sharpens differentiation); **collapses COHESION §26** | §6 |
| `research/atproto-sovereign-appview-club.md` | S | **synthesized** (read-side expression); **collapses §29**; harvests Twitter Circles → S5 | §8 |
| `research/germ-xchat-features.md` | U | **synthesized + harvested** (privacy-free/convenience-effortful inversion; closest cousin) | §5, §6 |
| `thinking/atproto-atmospheric-web.md` | U | **synthesized** (demand-side argument for Croft's crypto) | §6 (supporting) |
| `thinking/ios-opportunistic-p2p.md` | U | **harvested** (four-property impossibility; Peat prior art); **collapses §25** | §1, §9 |
| `seeds/.../atproto-atmospheric-web-iroh-mobile-FACTCHECK.md` | INDEX | **carried as source of truth** (cite, don't re-verify) | §3, §4, §6, §8 |
| `COHESION.md` §3, §8, §9, §11, §13, §17, §25, §26, §28, §29, §31, §32 | — | **collapsed/harvested** (messaging-landscape seams: SSB cautionary tale → bounded broadcast tier, media path; live dual-use identity evidence; atmospheric-web↔lineage-groups; three poles; sovereign-appview; iroh corroboration; open-social landscape) | §1, §2, §3, §4, §6, §8 |

**Excluded (do-not-carry):** iroh-docs "Merkle Search Trees" (REFUTED — range-set reconciliation; MST is
atproto's); the fictional "AT Messaging working group" (REFUTED); `did:key` atproto-resolvability (REFUTED);
`did:plc` = "Public Liaison Corporation" (fabrication); the false Vultr 1-Click PDS app (real = DigitalOcean);
"Voskop" (fabrication); refuted figures (Steem HF23 = $6.3M not $5M; Farcaster rent ~$7 not $5).

---

## 05 — identity you carry (N6 / G5) — **drafted** → `beta/05-identity-you-carry.md`

Thesis: keys≠identity; person=DID lineage; cross-platform continuity is attestation, never a master key;
total-device-loss recovery is the open problem.

| alpha source | tag | treatment | landed in |
|---|---|---|---|
| `thinking/multi-device.md` | S | **synthesized** (keys≠identity; device=MLS member; presentation fold; thresholds-count-lineages; §10.1 recovery DECISION) | §1, §2, §7 |
| `thinking/plc-identity-resilience.md` | S | **synthesized** (DID-method scorecard; the validating PLC read-replica; archive resizing) | §3, §4, §8 |
| `thinking/cross-platform-identity-provenance.md` | S | **synthesized** (hub-and-spoke attestation; no cross-network authority key; attestation-not-derivation; convergence hedge) | §5, §6, §8 |
| `seeds/.../croft-identity-provenance-dialogue-2026-06-20.md` | R | **carried-flag** (verbatim ASSISTANT formulations; corroborates §5–§7) | §5 (quotes) |
| `seeds/.../atproto-atmospheric-web-iroh-mobile-FACTCHECK.md` | INDEX | **cite, don't re-verify** (blessed methods; did:plc expansion; carried identity facts) | §8 |
| `COHESION.md` §7, §12, §21 | — | **collapsed** (INV-LINEAGE-NOT-LEAF logic CLOSED / library tracked; recovery = THE open problem; cross-platform CLOSED) | §1, §2, §5, §7 |

**Excluded (do-not-carry):** `did:key` resolvability (REFUTED); native atproto `did:webvh` support
(`[UNVERIFIED]`, gates §3's preferred option); PLC governance handoff *completed* (`[UNVERIFIED]`); the
"Equivalency Assertion" fabricated DID-Core label; "Public Liaison Corporation"; over-strong "cannot be
faked" (use "resistant to third-party spoofing"). **Surfaced:** the recovery-anchor decision (user's);
the anchor-URI stability contract; the unwritten per-platform trust-model doc (highest-leverage next artifact).

---

## 06 — safety without surveillance (N8 / G6, graduated) — **drafted** → `beta/06-safety-without-surveillance.md`

Thesis: a content-blind system stays safe by structure not inspection; membership ≠ access.

| alpha source | tag | treatment | landed in |
|---|---|---|---|
| `thinking/abuse-resistance-and-the-rave-trap.md` | S | **synthesized** (rave-trap; Signal-vs-Telegram; scale/peer levers; routing; fork blast-radius) | §1, §2, §10, §11 |
| `thinking/geer-gating-peer.md` | S | **synthesized** (consented content-visible role; rights-not-capability; label-not-enforce; compellability) | §3 |
| `thinking/failed-op-response.md` | U | **synthesized** (LOUD/SILENT/BLACKHOLE dial; immune signal; k-corroboration) | §4 |
| `thinking/freshness-signal.md` | S (shared 04) | **synthesized** — *reasoned socially* (no-false-current; freshness-gates-authority; MEMBERSHIP-FRESH). Crypto wire owned by 04 | §5 |
| `thinking/revocation-authority.md` | S (shared 04) | **synthesized** — *reasoned socially* (threshold dial; ADMIN FLOOR; never-irrevocable ladder; capture≠brick). Validator owned by 04 | §6 |
| `thinking/meer-superpeer-design.md` | U | **synthesized** (blind-by-construction thesis; anti-entrenchment; state portability) | §1, §7 |
| `thinking/membership-vs-access-the-public-door.md` | S | **synthesized** (membership≠access; public door; Sybil softening; D9) | §8 |
| `thinking/social-layer.md` | S | **synthesized** (S1–S5 visibility regimes; S5 Twitter-Circles structural-not-runtime) | §9 |
| `thinking/group-privacy-lanes-design-note.md` | S | **synthesized** (three-lane routing prevents the dangerous hybrid) | §10 |
| `COHESION.md` §2, §29, §36 | — | **collapsed** (V3 limitation; Twitter Circles → S5; membership-vs-access) | §8, §9 |

**Excluded (do-not-carry):** the "central hand on the wheel" / edge-AI-scan-then-snitch playbook (named as
the rejected anti-pattern only). **Over-claims avoided:** freshness "solves" partition (it narrows the
window; fresh-but-wrong is left to 04's hard-stop); the geer "solves" compellability; the floor "prevents
capture" (it prevents brick). **Surfaced:** V3 UX control before republish ships; S3 quiet membership
unsolved; E11 ten-second-door engineering; geer compellability (needs legal); CSAM posture + jurisdiction
(the user's / 07's). *No "Princeps Problem" formulation exists in these sources — not introduced (it lives
in NAMING / theme 02).*

---

## 07 — sustainability & stewardship (N4 / G2, decision-gated) — **drafted** → `beta/07-sustainability-and-stewardship.md`

Thesis: non-extraction is anti-fragile but survives only if TWO mechanisms carry it — a cooperative AND an
IP-stewardship structure.

| alpha source | tag | treatment | landed in |
|---|---|---|---|
| `research/social-platform-cycle.md` | S | **synthesized** (the rug-pull as a capital-structure phenomenon; survivor analysis). $ figures **excluded** | A1, A2 |
| `crystallized/principles.md` (Tier 1) | S | **synthesized + carried-flag** (settled values: anti-fragile, pay-the-keepers, credit-union, founder-sunset) | A2–A5 |
| `thinking/cooperative-social-union-model.md` | S | **synthesized** (four-pillar Social Union; failure lineage). §351 sections + fees + royalty figures **excluded** (NOT-LEGAL-ADVICE) | A6, A7, B6 |
| `thinking/foundation-and-ip-stewardship.md` | S | **synthesized** (three decoupled layers; AGPL+DCO lock; two-tier marks; entity phasing; assignment-with-goodwill). Fiscal-sponsor fees **excluded** | B1–B5 |
| `thinking/governance-and-survivability.md` | U | **synthesized** (anti-rug-pull: bankruptcy-remote steward + pre-funded archive). Technical archive mechanics **cross-referenced only** | A8 |
| `research/discord-dominance.md` | U | **harvested (narrow)** — only the moderator-labor-as-captured-value finding; figures **carried-flag** (`[UNVERIFIED]`) | B6 |
| `NAMING.md` | S | **harvested** — foundation-layer only: the **Noria** candidate (pending clearance) + the independence rationale. Croft/Drystone → theme 02 | B5; open items |
| `COHESION.md` §33, §35, §36 | — | **collapsed** (cooperative model; foundation/IP + Noria; Discord labor) | A6, B5, B6 |

**Excluded (do-not-carry — NOT-LEGAL-ADVICE):** all MO Chapter 351 statute sections; Articles/name-reservation/
sponsor fees; royalty %/multiple-cap/salary-runway/survivability-fund dollar figures; Discord revenue/ARR/MAU/
valuation; every acquisition price; state-comparison statute citations. **Carry the reasoning, not the
citations.** **Decision-gated surfaced:** legal-clearance + entity gate; Noria name clearance; fiscal-sponsor
selection; coop-vs-coop+501(c)(3) form; "is the badge where principles get teeth" (genome-vs-strategy, seam
with 01); MPL-vs-AGPL relationship to the substrate (flagged for confirmation, not asserted).

---

## 08 — Croft the product (N7 / G7, decision-gated) — **drafted** → `beta/08-croft-the-product.md`

Thesis: surface the proven substrate as a composable garden of ponds + pads on one core, thin shells.

| alpha source | tag | treatment | landed in |
|---|---|---|---|
| `thinking/app/README.md` | S/INDEX | **synthesized** (the untangle; open-risks register) | narrative; open items |
| `thinking/app/design-philosophy.md` | S | **synthesized** (garden thesis; FCIS spine; honest seams; craft rule) | §1, §2, §6 |
| `thinking/app/client-architecture-adr.md` | S | **synthesized** (one core + thin shells; two axes; option-C decomposition; awareness/interactivity) | §3, §4 |
| `thinking/app/design-criteria.md` | S | **synthesized** (the quality bar; pond/pad criteria; visual system) | §6 |
| `thinking/app/build-specs/BUILD-SPEC.md` | S/U | **carried-flag** (Phase-0 green-real; the 5 DECISIONS; cursor-invariant proof) | §3 (proof status) |
| `thinking/app/ponds/build-order.md` | U | **synthesized** (six-phase sequencing; resolver = tier-zero; fair-reveal leverage; sustainability gate) | §7, §9 |
| `thinking/app/ponds/games-pond-authoritative-list.md` | U | **synthesized** (catalog; three inclusion pathways) | §7 |
| `thinking/app/ponds/webxdc-security-and-competitive-games.md` | U | **synthesized** (the Cure53/CSP finding; disable-WebRTC; hard context boundary; per-match pseudonym) | §6 |
| `thinking/interaction-tiers.md` | S | **synthesized** ("three products, one send button"; visible-cost privacy) | §5 |
| `thinking/membership-vs-access-the-public-door.md` | S | **synthesized** (membership≠access at the product layer; ten-second door) | §8 |
| `seeds/.../atproto-atmospheric-web-iroh-mobile-FACTCHECK.md` | INDEX | **cite for iroh** (iroh 1.0; relays/hole-punch fallback; Tauri native WebView) | §6, §9 |
| `COHESION.md` §18, §19, §23 | — | **collapsed** (app body CLOSED-consumes-substrate; ponds/games + honest asterisk + security finding; Phase-0 DECIDED + option C) | §1, §3, §9 |

**Excluded (do-not-carry):** unsettled brand/product names ("Croft" as product, "Croft Group", "Grow your
own", "homegrown") — NOT propagated into structure (only umbrella-Croft and Drystone are settled, → theme
02); any over-claimed "serverless" (true only as "no application server," with the relay asterisk).
**Decision-gated surfaced:** CroftC Phase-0 IP/ownership (the user's; CroftC-side PR open); brand DRIFT
(brand-and-voice-notes vs NAMING.md) reconcile before any brand chapter; the E11 deep-link-resolver
engineering; cold-start for the owned pond; the cooperative-mechanism tie-in (shared with 07).

---

## Coverage view — what is dispositioned, and what is not

All eight themes are now drafted; every alpha **spine** source is dispositioned above. Remaining alpha
sources **not yet claimed by a beta theme** (their disposition — fold into a theme vs. retire as an
alpha-only working/index surface — is an open call recorded in `../MATURITY-ROLLUP.md`):

- **Index / working surfaces:** `ANALYSIS.md`, `TEST-PLAN.md`, `ROUND-2026-06-17-media-meer-conformance.md`
  (partly drawn into 04), `thinking/open-considerations.md`, `thinking/open-edges.md`.
- **Narrative skeletons:** `narrative/long-form.md`, `narrative/short.md`, `narrative/messaging-and-quotes.md`
  (the brand/quotes reservoir — likely feeds a future brand chapter once the brand DRIFT in 07/08 settles).
- **Research not yet surfaced as a beta spine:** `research/discord-matrix-groupchat.md`,
  `research/group-chat-failure-modes.md` + `-plain.md`, `research/iroh-realtime-media-references.md`,
  `research/str0m-production-readiness.md`, `research/open-publication-and-ip-protection.md`,
  `research/socialization-and-publication-venues.md` (supporting material for 03/04/07; pull up on demand).
- **Conformance/spec detail:** `crystallized/conclusions.md`, `crystallized/conformance-suite.md`,
  `crystallized/TEST-CORPUS.md`, `crystallized/CROFT-PROTOCOL.md` (cited by 04; full reconciliation of the
  §2 spec-vs-code pre-image discrepancy is a 04 follow-on).
- **The umbrella dossier:** `SOVEREIGN-COMMONS-DOSSIER.md` (pre-Croft naming; high provenance debt — its
  durable content is distributed across 01/02/07; retire-vs-keep is a later call).

Closing this list to zero (or to an explicit "retired as alpha-only") is the remaining coverage work.
