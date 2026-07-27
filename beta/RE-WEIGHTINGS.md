# discovery / beta — re-weightings register (prose-weight supersession index)

date: 2026-07-27

**What this is.** The index of places where an *early, strongly-weighted* narrative conclusion was
overtaken by later thinking — demoted, accepted as an inevitability, reframed, or re-ranked by a roadmap
that did not exist when the claim was written — but the **reading prose still led with the old weight**.
`DECISIONS.md`, `OPEN-THREADS.md`, and `CLOSED-THREADS.md` track *decisions and threads*; none of them
catches a mismatch between how heavily a claim *reads* and how heavily current best thinking actually
weights it. This register does exactly that, and only that.

**Why it exists.** A long discovery journey earns the right to demote its own early strong claims. Early
reasoning was prone to overstating, and it had a limited view of the eventual roadmap and priorities — so
some conclusions that were reasonable when written now read as over-weighted from the current-day view
looking down. The correct posture is not "these were wrong" but "**these were reasoned with less context;
current priorities re-weight them.**" This file keeps that re-weighting auditable without re-litigating it.

**Register discipline (like `DECISIONS.md`).** One row per re-weighting. Each states the early framing at
its tersest, the matured front-foot weight, the context that changed the weight, and where the reframe
landed (or is proposed). Nothing here re-argues a claim; the reasoning lives in the linked beta doc. When a
reframe is applied, the doc leads with the matured claim and keeps the de-weight to a clause — the audit
trail is this register, not a supersession narrative inside the doc (`../MEMORY`-style "log outcomes, not
the journey").

## The two flavors of over-weight

- **Overstatement** — a claim asserted more absolutely, or more "forced," than the matured evidence
  supports (the cooperative absolute; "the caution forces the design"; history read as proof; "large and
  encrypted" unqualified).
- **Strategic mis-ranking** — something ranked as *the* spine that a later roadmap re-ranks to "one of
  several / a helper" (the deep-link resolver; onboarding tactics as "what crosses the chasm"). This is the
  flavor that stems most directly from the early limited view of the roadmap.

## Section 1 — Re-weightings applied (2026-07-27)

| ID | Early framing (over-weighted) | Matured front-foot weight | Context that changed it | Reframed in |
|---|---|---|---|---|
| RW1 | **Deep-link resolver = "tier-zero, the single most strategically important component… the entire acquisition model (no public discovery by design)"**; cold-install-deferred-deeplink a "not privately achievable" loss/blocker to design around. | The resolver is the **highest-leverage *helper*** — the one-tap-join and growth accelerator for the iroh-native (Track B) ponds, over a manual join-code floor the product works without. Near-term growth rides the population-inheriting aggregator pond + the public Croft.ing surface; "no *algorithmic* public feed by design" ≠ no public discovery. Cold arrival is **install-then-one-tap, industry-standard**, an honest experience to name, not a Croft-specific loss. | The helper-tier floor (`croft/the-helper-tier-and-the-baseline-floor.md`, 2026-07-22) classifies the resolver as a route-around-able helper and names the "growth depends on it → smuggle it into the required set" error; the two-track roadmap made Track A population-inheritance the actual near-term growth. | `croft/product-the-garden-of-ponds.md` §6 + establishes; `croft/build-order-and-ponds-roadmap.md` (Track-B framing, ordering rule 1, Phase 0/0.2, parallel gates, catalog row, one-screen summary); `croft/reference-index.md` (platform-constraints). |
| RW2 | **"Hosting peer standing requires *the cooperative* form, adopted from inception"** (absolute, in the thesis and the layer summary). | Requires an **edge-free ownership form adopted from inception** — a form that *dissolves* rather than *fences* the owner/participant edge; the cooperative is its **cleanest instance**, not the only one, and even it is **necessary, not sufficient**. | The corpus's own `[tension]` §31 already states the absolute "overstates" (foundation-ownership, perpetual-purpose trusts, steward-ownership also protect non-extractive purpose); it was buried in a late tension while the front matter fronted the absolute. | `philosophy/structural-argument-principles.md` Thesis; `philosophy/README.md` ("Where the argument connects" + "What this layer establishes"). |
| RW3 | **"The BLE caution *forces* the design"** — an always-on meer anchor presented as non-optional ("durability *must* live on an always-on node… forces the anchor to exist"). | The BLE negative result **motivates the *default deployment's*** meer-anchored delivery. The meer is a **chosen, revocable helper above the D-self self-hosted floor** — removing it costs convenience, never function or standing; a user's own always-on node fills the same role. | `delivery-layer/01` establishes D-self as the floor ("every other source can be removed") and `the-four-property-tension.md` frames the superpeer as a deliberate, revocable choice; the ios/BLE doc kept the pre-floor "forced" framing. | `impl/ios-background-execution-and-the-ble-caution.md` (title, status block, overview, §5 header + body, establishes). |
| RW4 | **"iroh is the transport substrate Croft depends on"** stated as a universal, unqualified dependency. | Kept — the dependency is ~99% the case; **narrowed** only by naming the one real exception: an open/backplane tier that can ride the plain web stack without the overlay. | The two-regime (sealed vs open/backplane) reconciliation demoted the iroh overlay to the sealed tier; still in-flight (not yet in `DECISIONS.md`), so this is an added exception clause, **not** a demotion. | `cairn/substrate-prior-art.md` ("iroh itself"). |
| RW5 | **Onboarding "first-ten-minutes" tactics = "the load-bearing claim… what crosses the chasm"** (was RW-c1). | The tactics do not *cross* the chasm on their own; crossing needs the strategy's **three conditions at once** (product parity + non-extractive sustaining org + exogenous inciting event). What they do is let an inciting spike **convert into retained users rather than drain back out** — necessary at the crossing, not sufficient. | `socialization/adoption-strategy.md` names the three simultaneous conditions and casts onboarding readiness as the spike-conversion tactic; the onboarding-wall doc fronted it as *the* crossing move and diluted "load-bearing" across four parallel tactics. | `socialization/adoption-tactics-and-the-onboarding-wall.md` (headline claim; the four "Why load-bearing:" stamps → "Why it matters:"). |
| RW6 | **History read as constitutive proof** — "the name *is* the design… a cross-civilizational structural law… the evidence the metaphor needs" (was RW-c2). | History's register is **resonance / homage** (its own header; `LAYERS.md`: "why it *resonates*"). The cross-civilizational recurrence is a **resonant precedent** the name can lean on; it does **not** prove the design is *right* — that is philosophy's job (Layer 2). Read the croft and you have read the design's *shape*, a mnemonic for the commitments, not their proof. | The doc's own status header says "Register: reinforcement / homage" and `LAYERS.md` splits history (why it resonates) from philosophy (why it is right); the body had drifted into constitutive-proof language. | `history/the-enclosure-inversion-present-and-global.md` (§"Dry stone recurs…" header + intro, the "more than decorative" passage, the history↔philosophy bridge, the "name is the thesis" close, and the establishes clause). |

## Section 2 — Identified, not yet applied (audit complete; reframes proposed)

Surfaced by the 2026-07-27 six-layer re-weighting sweep. **RW-c1** (onboarding) and **RW-c2**
(history-as-proof) were applied the same day and promoted to Section 1 as **RW5** and **RW6**. The rest are
registered so they are not lost. The two **in-flight** rows (RW-c3, RW-c4) should be *qualified, not
rewritten*, until their underlying reconciliation lands.

| ID | Early framing (over-weighted) | Matured view | Recommended reframe | Status |
|---|---|---|---|---|
| RW-c3 | **"Large *and* encrypted"** presented as Drystone's resolved win (`fenced/group-scale-versus-e2ee.md` + `fenced/README.md`). | Tiered: holds for the sealed tier (≤~7k); above it Drystone deliberately concedes Force 2 — **private but not E2EE**, AppView reads content, MLS retained for attestation/membership (`OPEN-THREADS.md` T40; the experiment reconciliation). | **Qualify** "large and encrypted" to the sealed tier and name the >7k concession; do not rewrite while T40 is `open · experimental`. | in-flight — qualify only |
| RW-c4 | **Meadowcap presented as *the* ownership/read-enforcement mechanism** (`drystone-spec` §5.10/§5.11). | Appendix A: the Track A/B capability choice is **deferred**, "no normative text assumes a track," author leans Keyhive on revocation immediacy. | Extend the §5.5 "Meadowcap-*shaped*" hedge into §5.10/§5.11, or narrow Appendix A's disclaimer so the two stop disagreeing; do not commit a track. | in-flight — qualify only |
| RW-c5 | **Two-ledgers frame = "the load-bearing spine… commitments read off directly"** (`philosophy/commensurability-and-the-two-ledgers.md`). | `philosophy/reference-index.md` calls exactly that frame the layer's "defining weakness" — an unverified coined frame, no primary edition checked. | Downgrade "the load-bearing spine… read off directly" to "a proposed organizing frame the commitments are consistent with"; surface the sourcing flag near the top. | proposed (overstatement; softer) |
| RW-c6 | **Activism flagship still titles itself "the structural argument / the argument we have been building"** (`activism/structural-argument-narrative.md`). | The register split gave "structural" to philosophy and made activism the **empirical / motivation-only** register; the spec depends on it "only for motivation, never for a mechanism." | Retitle toward the empirical register ("the harm case" / "the empirical indictment"); soften "the argument we have been building"; cross-point philosophy for the structural spine. | proposed (naming/altitude; softer) |
| RW-c7 | **"Free, permanently, with no asterisk… structurally impossible"** moat (`socialization/adoption-tactics-and-the-onboarding-wall.md`). | The matured governance finding: marginal *server* cost is near-zero but **labor/stewardship is the real recurring cost and the single biggest long-term risk** (`socialization/coop-messaging-research.md`, `governance/foundation-cooperative-and-sustainability.md`; T33 open). | Scope the claim to marginal *serving* cost and add the honest labor asterisk ("the cost that remains is member-funded labor, which is what governance must protect"). | proposed (overstatement; softer) |
| RW-c8 | **Delivery-layer docs cite `mls-rs 0.55.2` as "the real libraries" validated against** (`impl/delivery-layer/*`). | The reference implementation and spec reconciliation standardized on **openmls 0.8.1**; mls-rs was not carried forward. | Add a one-line currency note: round-1/2 ran on mls-rs 0.55.2; the reference impl subsequently standardized on openmls 0.8.1; the mls-rs results stand as historical validation of the *design shapes*. | proposed (staleness, not weight) |
| RW-c9 | **README "open residue" still lists the iroh-gossip / iroh version skew** as a live "integration-residue item" (`impl/README.md`, `impl/delivery-layer/00-session-summary.md`). | Experiment **E0.1 cleared it** — builds against stable iroh 1.0.1. | Strike the item from the open-residue list or annotate it "resolved by E0.1." | proposed (staleness, not weight) |

## Provenance

Surfaced by a six-layer parallel re-weighting audit on 2026-07-27 (croft · philosophy+history · cairn+fenced
· impl · governance+socialization+activism · drystone-spec), each layer swept for early strong-weighted
conclusions the journey has since de-weighted, accepted-as-inevitable, or re-ranked while the prose still
led with the old weight. The spec and governance layers came back essentially clean — their version-tracking
and verification discipline already does this job. RW1–RW4 applied the same day; RW5–RW6 (onboarding,
history-as-proof) applied in a same-day second pass; Section 2 records the remaining candidates.
