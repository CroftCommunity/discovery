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
**Deferred decision (2026-06-26, was an inline prior-tier pointer in `beta/05`, now cleaned out for tier
discipline):** whether the **"evidentiary, not operational"** framing of the rights-floor is elevated to a
named, canonical Tier-1 principle in `crystallized/principles.md`. Settled-as-conclusion (it is stated clean
in `beta/05`); the *promotion* into the principles set is the user's curation call. Tracked here and as
`../beta/OPEN-THREADS.md` **T27**, not carried inline in the beta doc.

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

**Coverage view CLOSED (2026-06-25 per-file audit).** The per-file audit
(`plans/2026-06-25-beta-coverage-per-file-audit.md`) dispositioned all 165 alpha files exactly once and
closed every source above to either **folded → beta §** or **alpha-only by design**:

- **Index / working surfaces** (`ANALYSIS.md`, `TEST-PLAN.md`, `open-considerations.md`, `open-edges.md`):
  alpha-only by design (PROCESS/INDEX or CITED). `ROUND-2026-06-17-…` → **folded to 04 §4** (E10/E12/E11/meer
  results); residual media-hardening → T10.
- **Narrative skeletons** (`narrative/long-form.md`, `short.md`): alpha-only by design (PROCESS/INDEX —
  drafting surfaces; conclusions folded across beta). `messaging-and-quotes.md` → STAGED T4 (K11 crossed to
  07 A2b).
- **Research** (`discord-matrix-groupchat.md` → T16; `group-chat-failure-modes(-plain).md` → T5;
  `iroh-realtime-media-references.md`, `str0m-production-readiness.md` → CITED + T10;
  `open-publication-and-ip-protection.md`, `socialization-and-publication-venues.md` → **folded to 07 Pillar
  C (K9, venue map intact)**): all closed.
- **Conformance/spec detail** (`crystallized/conclusions.md` → folded across 04/01/03 + M0–M4 to T12;
  `conformance-suite.md`, `TEST-CORPUS.md`, `CROFT-PROTOCOL.md` → CITED, alpha-only by design backing 04):
  closed.
- **The umbrella dossier** (`SOVEREIGN-COMMONS-DOSSIER.md`): PROVENANCE/RAW; durable content verified
  distributed across 01/02/07 (K2/K3/K5/K10) — **except two framings flagged below (CM-A1, CM-A4)**;
  retire-vs-keep remains a later call.

**Ledger-accuracy nit:** the 03 table tags `ECOSYSTEM.md → synthesized → 03`, but ECOSYSTEM §8's cooperative
prior-art register (Ostrom/Stocksy/Mondragon/credit-union/Drivers/Resonate/PCC/SPI/SFC) is cited by **07's**
reasoning, not 03's body. Treat ECOSYSTEM §8 as **CITED-NOT-FOLDED** (the register stays alpha by design; 07
carries the operative subset in B1/B5/B6). Not a coverage gap.

## New CONCLUSION-MISSING finds (2026-06-25 per-file audit) — K13–K16 FOLDED; partials unfolded

The per-file pass found settled conclusions still unfolded after K1–K12. Four were independently
grep-confirmed ABSENT from beta and, **on the user's per-theme approval, were folded 2026-06-25 as K13–K16**
(clean beta narrative, no prior-tier links — exactly as K1–K12). Three are partial folds
(headline/discipline already present, a sub-nuance flattened); they remain **unfolded** (optional, low-value).

| # | Settled conclusion | Folded into | Alpha provenance | Status |
|---|---|---|---|---|
| K13 | **"Connection itself is the newest enclosure — platforms rent our relationships back to us."** The relationship/third-place as the present enclosure target; bridges 02 §1–5 to *why a social product*. | **02 §6** (new closing ¶) | `SOVEREIGN-COMMONS-DOSSIER.md` §2/§12 | **folded 2026-06-25** |
| K14 | **Recovery is two tiers: the lock (buildable now) vs the trust (unsolved).** Mechanism shippable now (threshold shares across independent trust domains); only the trust predicate is undecided. | **05 §7** (new ¶) | `thinking/local-first-as-design-imperative.md` | **folded 2026-06-25** |
| K15 | **The "non-mimicry moat":** non-extraction unlocks a feature class extractive competitors structurally cannot ship (the affirmative competitive case). | **07 A9** (new subsection) | `thinking/cooperative-social-union-model.md` | **folded 2026-06-25** |
| K16 | **Linear/extractive vs cyclical/relational "operating systems"** — a third framing of the civic "why." | **07 A0** (new subsection, framing Pillar A) | `SOVEREIGN-COMMONS-DOSSIER.md` §3.1/§12 | **folded 2026-06-25** |
| CM-P1 | Roll-up trust **trilemma** + accumulator/MMR end-state (partial: 04 §3 already lands the answer + "never authority-signed" discipline; only the 3-way framing + MMR future-path absent). | — (unfolded) | `thinking/design-notes-addendum.md` §2 | partial; optional |
| CM-P2 | The positive **"guarantee actually made"** sentence — legibility+exit, not convergence (partial: 04 already carries the social-legibility invariant; only the "we don't promise convergence" formulation flattened). | — (unfolded) | `thinking/design-notes-addendum.md` §6/§8 | partial; optional |
| CM-P3 | Geer visibility-dial **3-rung enumeration** (partial: 06 §3 already says "least-invasive rung" + "other rungs remain compellable"; only the report/classifier/full-key enumeration absent). | — (unfolded) | `thinking/geer-gating-peer.md` | partial; optional |

K13–K16 each landed as clean beta narrative (no prior-tier links; verification flags only where the
source carries them); their source→landing trace is the table immediately above, exactly as K1–K12 are
recorded in their own table (folded conclusions are tracked in the K-table, not duplicated into the
per-theme tables).

The unsettled long-tail finds from the same audit are staged as **T18 (LTS-for-interfaces), T19 (blind-peer
encrypted-search substrate), T20 (conflict-reason gaps C4/C7/C8/C9/C10)** in `../beta/OPEN-THREADS.md`.

## Beta refinement (2026-06-24 publication dialogue): 07 Pillar C prior-art vehicle

Distinct from K13–K16 (newly-*found* missing conclusions), this is a **refinement of an already-settled and
already-folded** conclusion. New alpha intake — the 2026-06-24 publication/defensive-disclosure dialogue
(`seeds/transcripts/raw/drystone-publication-defensive-disclosure-dialogue-2026-06-24.md` →
`thinking/drystone-publication-and-defensive-disclosure.md`) — overturned the **prior-art *vehicle*** half of
**K9 / 07 Pillar C** (which had recorded "IETF Internet-Draft first then arXiv").

| ref | Was (settled) | Refined to (folded 2026-06-25, user-approved) | Folded into |
|---|---|---|---|
| K9 / 07 C3 | prior art via **IETF Internet-Draft first, then arXiv** | prior art via a **third-party-witnessed DOI'd archive (Zenodo) + an OpenTimestamps cryptographic timestamp + a public Git release** *first*; an IETF I-D is a later, *more-encumbered* destination (IETF Trust holds reuse rights; needs 2 interop impls); arXiv gatekept. Disclosure only *enabling* once the sync wire format is field-by-field — that gates minting the v0.1 DOI. | **07 C3** (rewritten) + narrative overview + charter + C4 *Grounds* + the NOT-LEGAL-ADVICE banner |

**License sub-decision — resolved 2026-06-25 (user-approved).** The spec-text license is **CC0 1.0** (over
attribution-only CC-BY 4.0): for a spec written to be a freely-buildable standard, maximal openness serves
the "no one can claim or restrict the idea" goal better than keeping authorship bound to the text. Folded
into **07 C2** + the narrative overview, charter, and C3 NOT-LEGAL-ADVICE banner; the Apache-2.0 reference-code
license is unchanged. (ROADMAP_TODO **A14** closed; intersects A1's MPL/AGPL substrate posture but does not
resolve it. NOT-LEGAL-ADVICE — attorney review of the patent-non-assertion paragraph still advised.) Seam:
COHESION §38.

## Beta grounding (2026-06-24 rights-vs-capabilities dialogue): 01 §5

| ref | What | Folded into |
|---|---|---|
| K17 | **Rights-vs-capability discriminating test** grounding 01's boundary — a *right* is standing that must survive for any dispute about it to stay contestable; a *capability* (role / delegation / moderation power / write-access / vote weight) has its removal leave standing-to-object intact. The four rights fixed by what removal forecloses (tenure/exit/voice/share); the voice-vs-amplification cut (assert + reach willing peers ≠ compel amplification → keeps 06's label-not-enforce legitimate). The standing-side face of 01 §6.1's data-plane/control-plane split (K6). | **01 §5** (new paragraph after the boundary bullets) |

Provenance: `thinking/rights-vs-capabilities-definitions.md` (the cite-able block) ←
`seeds/transcripts/raw/rights-vs-capabilities-definitions-dialogue-2026-06-24.md`. **Two checks left open**
(do NOT harden the four-rights *closed set* into the normative spec until clear): is `share` fully a right or
partly a membership-class capability (→ 07); does the 04 survivor re-key strand `tenure` (→ 04). **Staged
beta `../beta/OPEN-THREADS.md` T21 / T22** (2026-06-26); tracked ROADMAP_TODO **E32 (b)/(c)**; seam COHESION §39.

## Beta 01 review → Drystone protocol spec (2026-06-26)

A voice-transcribed read-through review of `beta/01-epistemic-foundation.md` (now filed
`beta/thinking/raw/01_beta_review.txt`, manifest row added) was processed into a classified refinement
list (`plans/2026-06-26-beta-01-review-refinements.md`). In dialogue the target was reframed: rather than
polish 01 in place, **01's content became Part 1 of a new beta Drystone protocol spec** — a build-against,
certify-against document in the format of the alpha `thinking/drystone-spec/` scaffold. The spec is
**vendor-neutral**: Drystone is the protocol; Croft is one ecosystem built on it (not the protocol).

**Landed:** `beta/drystone-spec/` (README + `part-1-reasoning-underpinnings.md` +
`part-2-certifiable-design.md`).

| source | treatment | landed in |
|---|---|---|
| `beta/01-epistemic-foundation.md` (the razor; local-first; rights-floor; equal-in-rights; design posture) | **synthesized** into named design principles | spec **Part 1** §2 (`P-Local-Truth`, `P-Knowable-Truth`, `P-Peer-Equality`, `P-Durable-Enablement`) + §2.0 razor + §3 convergence |
| `01` cross-field grounding (Mill, Hayek, Ashby, Popper, Ostrom) | **carried-flag** (verbatim quotes whole + per-quote verification) | spec Part 1 §3 |
| `crystallized/CROFT-PROTOCOL.md` (proven wire spec) | **synthesized** (data model, signed message, integrity-vs-authority, multi-device fold, transport/media, freshness, visibility, failed-op, conformance) + **carried-flag** (green-real / green-model / design proof status) | spec **Part 2** §4, §6, §7.4/§7.6, §8, §9 |
| `thinking/drystone-spec/section-2-peers-rights-capabilities.md` | **synthesized** (one peer; rights vs capabilities; capability/role/PeerSet; meer; revocation; exitability; asymmetry-of-expressible-range) | spec Part 2 §5 |
| `thinking/drystone-spec/section-x-governance-conflicts.md` | **synthesized + carried-flag (ENABLING)** (append-only governance log; timestamp-free total order; R1–R6 capability interface; attributable acceptance; regress-free fold; Track A/B) | spec Part 2 §7 |
| `thinking/drystone-spec/drystone-spec-v0.1-skeleton.md` | **synthesized** (RFC structure, defensive-publication notice, CC0 text license, sequencing caveat) | spec README + section frame |

**This filled a real gap:** the alpha section drafts `Realizes` `P-*` principles whose defining §1 was
**never written** (ROADMAP E30) — Part 1 writes them.

**Review decisions applied (content changes from the settled 01):**
- **Excluded:** the **Socrates** quote(s) (over-broad; ethics now leads with Mill) and the **Peirce** "do
  not block the way of inquiry" quote (too opaque in context). *Both were `Verified`; cut on the author's
  review direction, not a verification failure.*
- **De-emphasized:** the "2,400 years" framing (duration is not the argument).
- **No paraphrase-as-quote:** the Ashby survival-condition **gloss** and the **Beer** paraphrase were
  dropped *as quotations* — Part 1 keeps Ashby's real line and carries Beer as reasoning prose, with a
  verbatim Beer citation **pending a user-supplied transcript** (staged `../beta/OPEN-THREADS.md` **T23**).
- **Relocated:** the **Hush-A-Phone / Bazelon** confirmed legal ancestor moved *out* of the reasoning
  layer into a new historical-alignment doc `thinking/historical-peer-rights.md` (framed as historical
  *peer rights*, vendor-neutral; CONFIRMED Bazelon quote preserved). It is **not** carried into the spec.
- **Node-not-system, lead-with-grounds, define-terms, deflate-aphorisms** applied throughout Part 1.

**Reconciliation items surfaced (Part 2 Appendix B, not resolved):** vendor-neutral naming (the reference
impl's signed `croft-*` domain-separation tags must become `drystone-*` and be re-proven, since the tag is
signed over); hash-function reconciliation (§4 proven on SHA-256 vs §7 designed on BLAKE3); the `ENABLING`
wire encodings (canonical fact encoding, frontier-commitment, frontier-closure, capability wire format);
root-authority succession; the `share`/`tenure` open checks (T21/T22); and the Matrix/Willow/Meadowcap/
Keyhive comparisons flagged **[confirm before publish]**.

**Corpus disposition — EXECUTED 2026-06-26 (user-directed "01 goes away as superseded").**
`beta/01-epistemic-foundation.md` was **deleted**; its reasoning lives in `drystone-spec` Part 1. The
README theme table + reading-order diagram now show the spec as the front "why" (the `02`–`08` numbering
is retained for cross-ref stability; there is intentionally no `01` theme doc). All 26 `01` cross-refs
across `02`/`03`/`04`/`05`/`07`/`OPEN-THREADS` were rewired to `drystone-spec` (Part 1 for the why, Part 2
§5 for the rights set; T21/T22 repointed). Only deliberate supersession/history notes still name `01`. The
stale handoff prompt (`seeds/generated-prompts/beta-01-review-refinement-prompt.md`) was banner-retired as
reference-only.

### Beer / algedonic / Cybersyn-OGAS intake (2026-06-26) — closes T23, sharpens the spec

New alpha intake (`seeds/transcripts/raw/beer-algedonic-cybersyn-ogas-dialogue-2026-06-25.md`, the
"new Beer transcript" promised at T23) was synthesized to
`thinking/algedonic-and-peerhood-as-adjudication.md` and incorporated:

| what | treatment | landed in |
|---|---|---|
| Real Beer quotes ("ride the dynamics," *Brain of the Firm*; "their only hope," *Designing Freedom* 1974) + plain-language **algedonic** channel + the **Cybersyn/OGAS** anecdote | **synthesized + carried-flag [confirm before publish]** (replaces the dropped paraphrase) | `drystone-spec` Part 1 §3 |
| The **adjudication-locus axis** + **peerhood = where decision rights sit** ("a peer is a locus that can adjudicate; count the adjudication loci; the wires lie, the authority topology tells the truth") + the OGAS anti-pattern | **synthesized** | `drystone-spec` Part 2 §3.1 (new) + §5.2 (sharpened) |
| **Label-not-enforce as a peerhood-preserving primitive** (enforcement relocates adjudication → peers become sensors) + hard-stop-and-escalate as the algedonic channel | **synthesized** | `drystone-spec` Part 2 §7.6, §8 |
| **Exit-backed authority** + the companion question "what makes a right cost something to violate?" (3 groundings; couples the wolf test + exitability + the Track A/B decision) | **synthesized** (open design question) | `drystone-spec` Part 2 Appendix B; staged **OPEN-THREADS T24** |

"aids to human viability, not excuses for automatic command" is **confirmed a secondary gloss** — kept as
own synthesis, not attributed to Beer. The Beer quotes + Cybersyn/OGAS dates/figures are
**[confirm before publish]** against primary editions (carried in Part 2 Appendix B). **OPEN-THREADS T23
CLOSED.** Seam: COHESION §40.

## Social-graph-as-substrate / storage-architecture intake (2026-06-26)

New alpha intake (`seeds/transcripts/raw/social-graph-substrate-redb-storage-dialogue-2026-06-26.md`, the
second "benefit from current context" handoff) synthesized to `thinking/social-graph-as-substrate.md`, with
a buildable redb spec at `seeds/generated-prompts/redb-social-graph-layer-build-prompt.md`. Layered per the
user's Drystone(protocol) / Croft(app) / redb(local-impl) cut:

| what | treatment | landed in |
|---|---|---|
| **Trust-vs-provenance** (provenance certain; "is this key really Anna" is a utility call; all trust roots in social trust incl. root CAs; the binding act records, doesn't establish) + **vouch as graded, contextual, non-transitive signal, never a verdict** (PGP's lesson) | **synthesized** | Drystone spec **Part 1 §2.0** (under the razor) + Part 2 §5/§8 `vouch`/`get_trust_signals` |
| **Recursive principal** (a peer is a locus of adjudication; user = group of devices; community = group of users) + **composition vs valuation edges** (shared-MLS-lineage vs weighted-trust-no-shared-keys) + **per-edge adversarial posture** | **synthesized** | Drystone spec **Part 1 §2.3** + Part 2 §3.1/§5 |
| **Per-device authorship + lamport** + **user-principal-as-self-AS** credential model + **devices-as-MLS-leaves** (no nesting, no dup signature keys) | **synthesized + carried-flag [confirm before publish] (MLS RFC 9420/9750)** | Drystone spec **Part 2 §4.5.1** |
| **Declarative snapshot = cache** (valid iff head matches; never synced/trusted-from-peer; "values that passed authorization at each step") + **verifiable roll-up** (commits to gov-head hash; re-expandable; Option-A self-fold) + **two-tier compaction** (governance permanent, content compactable; off by default) | **synthesized** (sharpens §7) | Drystone spec **Part 2 §7.3.3** |
| **Social graph as the substrate, chat as a tenant** + group≠member-set + implicit/sticky group lifecycle + local-projection-vs-shared-anchor seam + invisible-graph UX | **synthesized → FOLDED into 08 (user-approved 2026-06-26)** | `beta/08` narrative + charter + new **§1/§1.1/§1.2** + §4 re-point + establishes; trace via **OPEN-THREADS T26 (PROMOTED → 08)**; `thinking/social-graph-as-substrate.md` §1–3 |
| **Authoritative assertion-DAG + derived redb projection** (local-first CQRS), governance log + roll-up, redb as derived-state engine, blobs in iroh-blobs | **synthesized** (local-implementation) | redb build prompt; **OPEN-THREADS T25**; `thinking/social-graph-as-substrate.md` §4–5 |
| Prior art (Keet/Holepunch chat-bottomed-vs-graph-bottomed; ATProto public-follow-graph; Gun/OrbitDB/redb/Automerge) | **harvested** (web-verified-in-dialogue) | candidate ECOSYSTEM.md additions (Keet/Holepunch + the contrast); flagged **[confirm before publish]** |

**Layering discipline held:** local-implementation (redb) and app-product (social-graph-substrate UX) were
**kept out of the vendor-neutral protocol spec**; only protocol-level conclusions were folded into
`drystone-spec`. Web-searched facts (Keet/ATProto/redb/Automerge/MLS) carry **[confirm before publish]**.
Seam: COHESION §41.

## Settled conclusions not yet folded (2026-06-25 completeness audit)

> **Landing-pointer redirect (2026-06-26):** the K-tables below (and the K13–K16 / K17 tables above) record
> several folds as landing in **`01 §X`**. Theme `01` was **retired/superseded 2026-06-26** — its reasoning
> is now **Part 1** of the Drystone protocol spec (`beta/drystone-spec/`) and the rights/peer material is in
> **Part 2 §5**. So read every "`01 §X`" landing in these tables as **`drystone-spec` Part 1 (reasoning) /
> Part 2 §5 (rights & capabilities)**. The folds themselves are intact; only the doc they live in moved.

A content-level audit (reading `crystallized/`, `thinking/`, `narrative/`+dossier, `research/` against
the eight themes) found **settled** conclusions walked out in alpha that had not landed in beta. Unlike the
`OPEN-THREADS` (which are *unsettled*), these were **promotion-ready coverage gaps** — they belonged *in* a
theme and simply hadn't been folded. **These were folded into the themes on 2026-06-25** (landings in the
"Folded into" column); the table is retained as the record of the gap and its closure. **Ledger correction,
now resolved:** the per-theme tables above had tagged `crystallized/principles.md` Tier 1 as fully
"synthesized" into 01/07, but it was **overstated** — several Tier-1 principles (and one green-real proof)
had not crossed. With K1–K12 folded, `principles.md` Tier 1 is now drained into beta.

| # | Settled conclusion (folded 2026-06-25) | Folded into | Alpha provenance |
|---|---|---|---|
| K1 | **Governance-log roll-up / threshold-signed checkpoint** — settled-history compaction so a client need not replay the whole log; threshold-signed (never authority/broker), can't span a fork. **green-real, CLOSED 2026-06-16** (`gov::Checkpoint`/`verify_checkpoint`). The direct answer to the SSB unbounded-log death. **Highest — it is proven and unrepresented.** | 04 §3/§5 | `thinking/design-notes-addendum.md`; `crystallized/proof-ledger.md`; `COHESION.md` §3 |
| K2 | **The "recurring inversion"** — the named five-scale generator (extractive stateful intermediary → stateless/content-blind/optional → wrapped in a reinforcing institution); ties the protocol moves (04/06) to the economic ones (07) as one idea. Each instance is in beta; the unifying pattern is nowhere. | 01 §3/§6 or 07 | `crystallized/principles.md` Tier 1; `SOVEREIGN-COMMONS-DOSSIER.md` |
| K3 | **"Centralization *capture*, not centralization itself"** + the **"credibly-decentralized-but-operationally-centralized" trap** — the permission slip for having a meer/relay/co-op at all, and the diagnostic criterion 03 judges the field by. Used everywhere in beta, stated nowhere. | 01/07 + 03 | `crystallized/principles.md` Tier 1; dossier §2/§7 |
| K4 | **Two convergent modes + the conformance test + keep-the-ceremony-warm** — superpeer-assisted and pure-P2P must reach the same governance state; the superpeer only *accelerates*; "for every superpeer feature, is there an outcome unreachable without it?"; run the pure-P2P checkpoint ceremony on a schedule so the no-superpeer path can't rot. The operational guard that keeps rights distributed. | 06 §7 / 04 | `thinking/design-notes-addendum.md` |
| K5 | **Per-tier security properties matrix** (the guardrail that keeps "different, not weaker" honest) + the two sharp claimable properties: **transparent offline** (two phones sync with zero internet) and **no central operator to compel**. | 03 §5 / 06 | `crystallized/principles.md` Tier 1; dossier §7/§13 |
| K6 | **Capabilities-not-rights as a named principle** — the **data-plane (capabilities) vs control-plane (rights)** grounding + the **availability-as-a-back-door** seam (a right exercisable only through one peer's presence is escrowed to it). The settled epistemic framing (distinct from T1's gated wire design — likely the `P-Peer-Equality` principle T1/E30 needs written). | 01 (principle set) | `thinking/design-notes-addendum.md` |
| K7 | **Capability-vs-authority delegate primitive + planes/blast-radius** — capability ("do the work") vs authority ("act as you") never conflated; courier-vs-agent (pre-seal payload, delegate only the trigger); liveness is the boundary; planes = blast-radius classes, reconvergence policy per-plane bound into the asset hash. The general delegation theory under meer/geer/recovery; only a parenthetical in 08. | 04 / 05 / 06 | `thinking/local-first-as-design-imperative.md` |
| K8 | **Meer confidentiality dial (Tier 0 / 1 / 2 / no-mirror)** — "blind" is a per-group governance dial, not binary; Tier-2 (key-holding) only where a co-op designates a trusted member; a Tier-0/1 meer must prove it holds no payload key. Beta flattens it to binary "blind." | 06 §1/§7 | `thinking/meer-superpeer-design.md` |
| K9 | **Defensive-publication / IP-and-venue strategy** — publish the spec defensively (no own patent), **doc CC-BY 4.0 + reference code Apache-2.0**, IETF Internet-Draft first (prior art) then arXiv, with a per-layer venue map. The *external* twin of 07's *internal* AGPL/trademark IP story; couples T9. (Not even in the coverage view above.) **[Prior-art *vehicle* refined 2026-06-24 → Zenodo DOI + OpenTimestamps first, IETF-I-D demoted; text license CC0-vs-CC-BY now open (A14). See "Beta refinement" section above.]** | 07 (a Pillar C) | `research/open-publication-and-ip-protection.md`; `research/socialization-and-publication-venues.md` |
| K10 | **Free-tier mandate** — participation must be possible on a bare phone (CPU/memory/storage/internet), no purchased infrastructure. A settled access *commitment*, not a feature. | 01 §4 / 08 | `crystallized/principles.md` Tier 1; dossier §7 |
| K11 | **Motivation-crowding-out evidence** (Gneezy & Rustichini, "A Fine is a Price") — the peer-reviewed backbone for *why* a fee-for-everything model is structurally corrosive (prices crowd out the intrinsic motivation that makes community real), distinct from the rug-pull/anti-fragile argument 07 already carries. | 07 | `narrative/messaging-and-quotes.md` |
| K12 | **On-device-LLM strictly-optional invariant** — no single on-device model can be assumed present; the assistant must detect-availability-first, accelerate-never-gate, and every path must be reachable without it (menus/verbs/deep-links a complete navigation system on their own). | 08 §6 | `thinking/app/on-device-llm-feasibility.md` |

Lower-priority / borderline (noted, not yet listed as gaps): the **user-need-first / Google+ lesson**
(largely implied by 08/01); the **"type at creation, not a runtime toggle"** MLS-coherence *rationale*
(the headline is already in 08 §5, only the reason was flattened).

**Disposition (done 2026-06-25):** K1–K12 were folded into the themes via a per-theme pass (clean beta
narrative, no prior-tier links). Exact landings: **K1** → 04 §3 (proof row) + §5; **K2** → 01 §6.2; **K3**
→ 01 §6.3 (+ the field-criterion in 03 §3); **K4** → 06 §7; **K5** → 03 §5; **K6** → 01 §6.1; **K7** → 04
(substrate-model subsection between §2 and §3); **K8** → 06 §1; **K9** → 07 Pillar C; **K10** → 08 Charter
(the access floor); **K11** → 07 A2b; **K12** → 08 §10. `crystallized/principles.md` Tier 1 is now drained
into beta. The two lower-priority items noted above (user-need-first / Google+; the "type at creation"
MLS-coherence rationale) remain optional and unfolded.
