# discovery / beta — open threads (the staging queue at the beta gate)

date: 2026-06-25 · last reviewed: 2026-06-26 (author read-through →
`../alpha/plans/2026-06-26-open-threads-review.md`)

## What this is

A holding ledger for threads that are **being pulled toward beta but are not yet settled enough to
become resolved beta narrative**. It exists so a live need is never lost, while the resolved layer docs
and the Drystone protocol spec (`drystone-spec/`) stay a clean, settled synthesis. A thread
lives here — referenced, not asserted
— until its gates clear; only then does it graduate into a layer doc (and earn a row in
`../alpha/LAYER-ROLLUP.md`).

> **Theme retirement (2026-07-07).** The narrative themes `02`–`08` were re-filed onto the layer-cake and
> discarded. Any `0N` reference below (promotion targets, `beta/0N §x` provenance) is historical and now
> resolves to a layer: **02** to `history/` + `philosophy/`; **03** to `cairn/` + `fenced/`; **04 / 05 / 06**
> to `drystone-spec/` + `impl/`; **07** to `governance/` + `philosophy/`; **08** to `croft/` +
> `socialization/` (the former **01** is the spec's Part 1). The full trace is `../alpha/LAYER-ROLLUP.md`.
> Threads are not individually rewritten; read any `0N` through this map.

It is a **process artifact**, peer to `../alpha/BETA-ROLLUP.md` and `../alpha/COHESION.md` — **not** a
theme doc and **not** part of beta's forward narrative. Unlike a theme doc, it may point down into
`alpha/` freely (that is its job). It deliberately holds DRAFT / decision-gated / fact-unconfirmed
material **out** of the settled themes.

## Why it lives here (and not in alpha)

The settling happens **at the beta gate**, so the queue lives at the beta layer. But alpha must not
lose the need, so the alpha indexes reference it (`ROADMAP_TODO.md`, `COHESION.md`, `BETA-ROLLUP.md`
point here). Division of labor:

- `../alpha/ROADMAP_TODO.md` — the alpha **backlog** (everything up for consideration, any stage).
- `../alpha/BETA-ROLLUP.md` — the trace of what **landed** in beta (treatment → section), plus a
  coverage view of alpha sources **not yet** pulled up.
- **this file** — the subset of not-yet-pulled-up material that is **actively queued for beta and
  blocked on specific settling work**, with that work named per thread. A thread here is a `deferred`
  rollup item with its gates made explicit and a promotion target attached.
- `README.md` → **"Standing decisions surfaced, not resolved"** — the **decision-gate register** (the
  user's calls): MPL/AGPL license, total-device-loss recovery anchor, cooperative legal-review, Noria name,
  CroftC Phase-0 IP, genome-vs-strategy. These are **not** duplicated as threads here (they live in the
  README list + the relevant theme banners); a thread may *reference* a gate, but the gate itself is tracked
  there. (Capability Track A/B, key-custody, geer-name are tracked in `ROADMAP_TODO` A11/A12/A13 + T1/T24.)

## Entry schema

Each thread carries:

- **Layer**: which layer(s) it belongs to, for grep and grouping (a cross-cutting thread lists several):
  `history` · `philosophy` · `cairn` · `fenced` · `drystone-spec` · `impl` · `croft` · `governance` ·
  `socialization` · `activism`, plus `cross-cutting` for project-level decision gates. This tag is
  authoritative; the group headings below are a rendered view of it.
- **Status** — the lifecycle state (2026-06-26 model):
  - `open` — live and unsettled. Two sub-states: `surfaced` (logged, gates not yet worked) ·
    `gated` (blocked on named decisions/work).
  - `in-progress` — being actively worked (built/proved/researched) right now.
  - `promoted` — its content was **integrated** into a matured doc (a theme or the spec). Kept for
    provenance, out of the live queue (see "Promoted & closed" below).
  - `closed` — **resolved without promotion**: settled-and-incorporated elsewhere, or decided
    not to pursue. Kept for provenance, out of the live queue.
- **Type** — the *kind of work* the thread needs, so threads can be grouped and run out together:
  `needs-content` (write/explain it) · `needs-research` (external/primary-source dig) ·
  `needs-experimentation` (spike/measure) · `needs-proving` (build a proof/test harness) ·
  `legal-review` (counsel) · `publish` (ship/mint). A thread may carry a couples-with note.
- **What it is** — one or two lines. *(Migration target, per the 2026-06-26 review: expand the
  meatier threads into **Problem statement / Proposed directions / What's indeterminate** as they get
  worked — see structural decision S2 below. Not yet done across the board.)*
- **Promotion target** — which layer(s)/section it would land in.
- **Gates — must settle before it becomes resolved beta narrative** — the explicit blockers
  (decisions, `ENABLING` spec work, fact-confirmation).
- **Alpha provenance** — where the material lives now.

## Structural changes proposed by the 2026-06-26 review (surfaced — the user's call)

The author's read-through proposed reshaping the ledger itself. The state/type model above and moving
promoted/closed out of the live queue (below) are **applied**. The rest are **registered here, not
executed** — they are larger reorgs or net-new authored docs awaiting a greenlight. Full classification:
`../alpha/plans/2026-06-26-open-threads-review.md`.

- **S1 — per-thread directory split.** As threads grow into bigger contextual decisions, give each its
  own file under a directory (e.g. an `open/` dir and a `closed/` dir) so the queue is scannable. *Large
  reorg — not executed; awaiting greenlight.*
- **S2 — meat-on-the-bone expansion.** Expand each worked thread to **Problem statement / Proposed
  directions / What's indeterminate** (the current one-liners are too terse to reason from). *Applied only
  where the review supplied material; the rest carry a review note pending expansion.*
- **S3 — brand/voice + adoption-enablement twin docs, start now.** Stand up a brand/voice/messaging
  working doc **and** an adoption-enablement doc as **twins** that accrete ideas/links/"ammo" over time so
  nothing is forgotten. Maps to **T4** (brand) + **T11** (adoption). **Started 2026-06-26 (scaffolds):**
  `../alpha/narrative/brand-comms-workbook.md` + `../alpha/narrative/adoption-enablement.md` (cross-linked
  twins; index existing reservoirs, accrete from here).
- **S4 — per-platform design files.** One design file each for **Linux / macOS / Android / iOS**, walking
  out the common-core-vs-platform-difference trades; every not-yet-implemented platform needs its own.
  Maps to **T6 / T14**. **Started 2026-06-26 (scaffolds):** `../alpha/thinking/app/platforms/` (README +
  `linux/macos/android/ios.md`, anchored on the client-architecture ADR).
- **S5 — per-app PRDs.** One PRD each for **chat** and the **games pond**, plus the new modest starter
  use case — a peer-to-peer **"thinking of you"** signal (touch your phone to reach out to another
  person). Maps to **T15**. **Started 2026-06-26 (scaffolds):** `../alpha/thinking/app/prds/` (README +
  `chat/games-pond/thinking-of-you.md`).
- **S6 — by-type grouped index.** Once threads carry a Type (above), add a grouped view so a batch of one
  type can be run out together. *Follow-up to the Type field.*

## Promotion rule

A thread leaves the live queue **only when its gates are clear**. On promotion: write the settled
synthesis into the theme doc (quotes whole, verification flags inline), add the `../alpha/BETA-ROLLUP.md`
trace row, set the thread's status to `promoted`, and **move it down into "Promoted & closed (provenance
retained)"** (2026-06-26 convention — keep the open list scannable). A thread that is resolved without
promotion (settled elsewhere, or decided not to pursue) is set to `closed` and moved the same way. Until
a thread is promoted, beta theme docs may **not** assert its content as resolved.

---

## Open threads

> **Structure (2026-07-07).** Open-only. Threads are grouped by the layer they most belong to, plus a
> cross-cutting group for project-level decision gates. Closed and promoted threads live in
> `CLOSED-THREADS.md`; the spec keeps its own deeper open-items in `drystone-spec/open-threads.md`. When a
> thread closes or promotes, move its block to `CLOSED-THREADS.md` (do not delete it).

### Cross-cutting: decision gates and publication

#### T9 — Publication-readiness verification pass (01 Ostrom + 02 Clearances colour quotes)

- **Layer:** cross-cutting, philosophy, activism
- **Status:** `open · gated`.
- **Type:** `needs-research` (primary-source pass).
- **Review (2026-06-26):** still need to get the raw/primary-source material to clear each cited quote.
- **What it is:** a hard external-publication gate currently scattered as inline `[UNVERIFIED]` flags with
  no aggregating thread: **01**'s Ostrom subsidiarity passage is from the 2013 generalization, not
  *Governing the Commons* ("confirm against the primary text before direct citation"); **02**'s Clearances
  colour quotes (Chambers, "four-footed clansmen," the Shetland curse, the "Magna Carta of the Highlands"
  attribution, the "law locks up the man or woman" verse, the 1772 OED sentence) are tertiary-source
  `[UNVERIFIED]` and "must stay flagged until a primary-source pass."
- **Promotion target:** clears external publication of **01** and **02** (does not change their narrative,
  removes their publication blockers).
- **Gates:** a primary-source verification pass clearing each cited quote/attribution.
- **Alpha provenance:** `drystone-spec` Part 1 §3; `beta/02` §1/§4/§5. (Pass-2 fact-check left Ostrom as the one
  remaining 01 confirm.)

#### T27 — Promote "evidentiary, not operational" to a canonical principle?

- **Layer:** cross-cutting, philosophy
- **Status:** `open · gated` (a user curation decision).
- **Type:** `needs-content` (curation call).
- **What it is:** the rights-floor is **evidentiary, not operational** — it records and proves standing; it
  does not operate things (stated clean in `beta/05` §5 and its charter). This is **settled as a conclusion**;
  the open question is purely whether it should be **elevated to a named, canonical Tier-1 principle** in the
  alpha principles set (`crystallized/principles.md`) — a curation call, not a design question.
- **Promotion target:** `crystallized/principles.md` (alpha) if adopted; no beta-doc change needed (the
  conclusion already reads clean in `05`).
- **Gates:** the user's decision to name it as a principle (or leave it as a per-theme conclusion).
- **Alpha provenance:** `beta/05` §5; `../alpha/BETA-ROLLUP.md` 05 §Deferred-decision note.

> **T28 added 2026-06-26** by the alpha left-behind audit — extracted from an inline "later call" that the
> 2026-06-26 Hush-A-Phone relocation left at the bottom of `../alpha/thinking/historical-peer-rights.md`,
> rather than leaving it as an inline deferral.

#### T30 — Mature the Drystone spec to publication-final (the path to the defensive-publication DOI)

- **Layer:** cross-cutting, drystone-spec
- **Status:** `open · gated` (spec is beta-maturity; publication-final is the next stage up).
- **Type:** `publish`.
- **Review (2026-06-26):** reframe concretely — T30 is the **publish** thread: mature the spec, publish
  the final spec, mint the DOI. The **attorney legal-review** piece was **split out into its own thread,
  T32** (type `legal-review`), so the two kinds of work track separately.
- **What it is:** two closures take `drystone-spec` from beta to a mintable defensive-publication record:
  1. **Pin the `ENABLING` wire encodings, in sequence** (Part 2 App-B): the **canonical governance-fact byte
     encoding** (the base all others extend) → **frontier-closure-before-sort** (the highest-risk divergence
     point) → **frontier-commitment + acceptance-record format** → **§7.2 message formats field-by-field** →
     the **capability wire format** (gated on the Track A/B decision). Per the spec README sequencing note,
     **do not mint the v0.1 Zenodo DOI until §7.2 is buildable from the text alone** — that is when the
     disclosure becomes *enabling* (protective as prior art).
  2. **Confirm the `[confirm before publish]` external facts** against primary sources (currently
     web-verified-in-dialogue only): Matrix State Resolution / room v12 / CVE-2025-49090, Willow, Meadowcap,
     Keyhive (Part 2 §7 / App-A); the Beer quotes + Cybersyn/OGAS dates/figures (Part 1 §3); iroh cites the
     FACTCHECK SoT. Also resolve the spec's own reconciliations: the **`croft-*` → `drystone-*` tag rename**
     (re-prove, since the tag is signed over) and the **SHA-256 (§4) vs BLAKE3 (§7) hash-function** choice.
- **Promotion target:** `drystone-spec` → an `rc`/publish-stage spec + a Zenodo DOI + OpenTimestamps + a
  public Git release (the vehicle settled in **07 Pillar C / K9**; spec-text **CC0 1.0**, code **Apache-2.0**).
- **Gates:** the Track A/B capability decision (couples T1/T24) blocks the capability wire format; the rest
  is concrete spec-writing + a fact-confirmation sweep. Attorney review is **tracked separately in T32**
  (it gates the publish, but it is legal-review work, not spec-writing).
- **Provenance:** `drystone-spec` Part 2 Appendix A/B; `thinking/drystone-publication-and-defensive-disclosure.md`;
  couples T1 (promoted), T22 (tenure/re-key), T24 (Track A/B), T29 (MLS↔log binding), T32 (legal review).

> **T31–T32 added 2026-06-26** from the open-threads review read-through
> (`../alpha/plans/2026-06-26-open-threads-review.md`). T31 captures the recurring rights/role/capability
> disentanglement the review asked to clear up; T32 is the legal-review half split out of T30.

#### T32 — Attorney legal-review of the Drystone defensive-publication (patent-non-assertion)

- **Layer:** cross-cutting, governance
- **Status:** `open · gated` (split out of T30, 2026-06-26).
- **Type:** `legal-review`.
- **What it is:** the **NOT-LEGAL-ADVICE** attorney review the defensive-publication path needs before it
  ships — review of the **patent-non-assertion paragraph** (07 C3) and the disclosure framing — tracked as
  its own typed thread so legal work and spec-writing don't blur together. Distinct from (but adjacent to)
  the bannered cooperative legal-review gate in `README.md`, which it does not duplicate.
- **Promotion target:** clears a gate on **T30** (publish) and **07 Pillar C**; no beta-doc content change.
- **Gates:** engage counsel; the cooperative/foundation legal-review gate (README) is the broader call this
  sits under.
- **Provenance:** `beta/07` Pillar C (C3); `thinking/drystone-publication-and-defensive-disclosure.md`;
  split from T30.

#### T34 — Project Mercury: re-check the litigation docket before any external publication (time-sensitive)

- **Layer:** cross-cutting, activism
- **Status:** `open · gated` (surfaced 2026-07-06 by the activism-layer research set).
- **Type:** `needs-research` (couples-with `publish`).
- **What it is:** the strongest causal evidence in the `activism/` harm case — **Project Mercury**, a
  2019–2020 internal Meta/Nielsen deactivation study alleged (in a Nov 2025 unredacted Motley Rice filing,
  N.D. Cal. social-media MDL) to show users who stopped Facebook for a week "reported lower feelings of
  depression, anxiety, loneliness and social comparison," with an internal staffer reportedly writing "the
  Nielsen study does show causal impact on social comparison" and another reaching for a tobacco-companies
  analogy. **It is live litigation at the knowledge edge:** the underlying documents are sealed, Meta filed
  a motion to strike with a real methodological rebuttal (unblinded deactivation cannot separate the effect
  of leaving from the belief that leaving helps), and a hearing was set for **Jan 26 2026**. A re-check of
  public reporting as of late June 2026 surfaced no post-hearing ruling.
- **Promotion target:** none — this is a **publication gate** on the `activism/` set (and, transitively, on
  anything public-facing that leans on it), not beta-theme content. It does **not** gate the Drystone spec
  (open-items records this explicitly).
- **Gates:** pull the **PACER docket directly** (not aggregators) before external publication; retrieve the
  unsealed exhibits if/when unsealed rather than relying on the plaintiffs' framing as relayed by Reuters /
  CNBC / UPI; represent Meta's rebuttal at full strength every time. Same discipline applies to the
  X-platform study (Nature 2026) if it becomes load-bearing.
- **Provenance:** `activism/relational-field-research-brief.md` gap 14 + `structural-argument-narrative.md`
  Plank 2; `drystone-spec/open-items.md` (companion-tracked-separately note). Non-load-bearing follow-ups
  also flagged in `activism/README.md`: Tristan Harris to primary, the "63 break-glass" count to a Meta
  document, and the ~35-study internal corpus traced from the aggregator to individual exhibits.

### Philosophy (Layer 2)

#### T24 — What grounds a peer's authority, and what makes a right cost something to violate?

- **Layer:** philosophy, drystone-spec
- **Status:** `open · gated`.
- **Type:** `needs-research` (expand per S2; couples T31, T1 Track A/B).
- **Review (2026-06-26):** needs a plain-language problem statement plus the directions already
  considered. The user's framing to capture: a peer's authority is **local by nature** (it holds local
  state no one else has, meant to be corroborated by other peers); and *what makes a right cost something
  to violate* is that the **integrity of the whole system depends on each peer holding its rights to
  preserve variety**, which preserves the longevity/homeostasis of the broader social graph — strip peers
  of rights and that graph collapses. May need more research to find what's missing.
- **What it is:** the Drystone spec now defines a **peer as a locus of adjudication** (Part 2 §3.1/§5.2)
  and rights as standing whose removal cancels its own contestation (§5.3). But in a system with no
  central allocator, *peerhood-as-authority must bottom out in something*, and the grounding choice
  changes the enforcement primitive. Three candidates: (1) **cryptographic-fact** authority (self-enforcing
  but only key-shaped domains); (2) **consensus-conferred** (circular — a peer can be demoted to a sensor
  by collective non-recognition with no topological change, the silent peer→sensor rot, and it needs the
  very enforcement the design avoids); (3) **exit-backed** (a peer holds authority insofar as its absence
  costs the system something it can't replace — ties authority to variety; needs *legibility of exit*).
  Working definition to pressure-test: *a peer is a locus whose adjudication others must respect because
  the cost of not respecting it is borne by them, not only by the peer.* The spec needs a **companion
  question to "where do decision rights sit": "what makes those rights cost something to violate?"** —
  without it there is no early detector of the rot.
- **Promotion target:** `drystone-spec` Part 2 §5 (the peer/rights definition) + Part 1 §2.3
  (P-Peer-Equality). Currently held as an open question in **Part 2 Appendix B**.
- **Gates:** couples the **wolf test** and the §5.8 **exitability** backstop; and the grounding choice
  interacts with the **capability-mechanism Track A/B decision** (T1 / `ROADMAP_TODO` A11) — the
  enforcement primitive each grounding implies differs. Decide the grounding, then the peerhood/rights
  definitions can harden.
- **Alpha provenance:** `../alpha/thinking/algedonic-and-peerhood-as-adjudication.md` §5; raw
  `../alpha/seeds/transcripts/raw/beer-algedonic-cybersyn-ogas-dialogue-2026-06-25.md` (Turn 6).

> **T25–T26 added 2026-06-26** from the social-graph-as-substrate / storage-architecture dialogue. T25 is a
> *local-implementation* build (not protocol); T26 was an *app/product* reframe (theme 08) and is now
> **PROMOTED → 08** (see "Promoted & closed" below). The protocol-level conclusions already landed in the
> Drystone spec (Part 1 §2.0/§2.3, Part 2 §4.5.1/§7.3.3).

#### T28 — Maturity home for the historical peer-rights material (Hush-A-Phone lineage)

- **Layer:** philosophy, history
- **Status:** `open · surfaced` (a placement decision).
- **Type:** `needs-content`.
- **Review (2026-06-26):** the **history of peer rights and distributed social systems** is itself a
  topic, and there is enough material to possibly make it **its own theme** — leans toward option (a) /
  (b) over staying alpha-only.
- **What it is:** when the Hush-A-Phone / Bazelon "private benefit, not public detriment" legal ancestor was
  relocated out of the reasoning core (it is **not** spec material — vendor-neutral, historical), the new
  doc `../alpha/thinking/historical-peer-rights.md` was left with its eventual home undecided: (a) mature
  into its own beta theme, (b) fold into the history theme **`02`** (enclosure / commons — closely adjacent),
  or (c) stay alpha-only by design. It is also extensible (common-carrier, right-to-repair, interop
  mandates) which weighs toward (a)/(b).
- **Promotion target:** most likely **`02`** (historical-alignment is its register) or a small standalone
  historical theme; or stays alpha.
- **Gates:** the user's placement call; whether the lineage gets developed enough to warrant its own theme.
- **Alpha provenance:** `../alpha/thinking/historical-peer-rights.md`; the "no right to remove" legal-ancestor
  also lives in `crystallized/principles.md` (frozen) and was relocated out of the beta reasoning doc.

> **T29 added 2026-06-26** by the alpha left-behind audit — a research-surfaced open question
> (`research/messaging-solutions-landscape.md` §"top unresolved questions" #3) that was never surfaced as a
> thread or a spec open item.

### Cairn / fenced (Layer 3, 3')

#### T7 — atproto Permissioned/Private-Data watch-item (external dependency, gates 03 + 05)

- **Layer:** cairn, fenced, drystone-spec
- **Status:** `open · gated` (gate is external, not Croft-internal work).
- **Type:** `needs-research` (external watch-item).
- **Review (2026-06-26):** still deferred forward — external dependency, nothing to action yet.
- **What it is:** 03 calls atproto's Permissioned Data work "**the single most important external
  development to track** — it could narrow or complement Croft's private path." The real ATProto Private
  Data WG defers true E2EE / zero-knowledge; Croft sits on the harder ZK side. Couples to 05's `did:webvh`
  native-support `[UNVERIFIED]` gate.
- **Promotion target:** updates **03** (the field positioning) and **05** (preferred-DID-method choice)
  when it lands.
- **Gates:** the atproto WG reaches a settled E2EE/ZK posture; `did:webvh` native atproto support
  confirmed against the FACTCHECK SoT.
- **Alpha provenance:** `beta/03` §6; `beta/05` §3; FACTCHECK as SoT for the confirm.

#### Phase-1 coverage-recovery cohort — ECOSYSTEM.md → cairn (2026-07-08)

These threads stage the `../alpha/ECOSYSTEM.md` material that the coverage audit
(`../alpha/plans/2026-07-08-beta-coverage-gap-ledger.md`) found was never carried into beta — the single
largest hole, since `LAYER-ROLLUP.md:43–44` wrongly counted the whole register as "covered by existing
cairn docs." Each thread is a proposed cairn doc (a projects-register grain, per backlog C9); §8 (co-op
prior art) is governance-bound and staged as T48. `ECOSYSTEM.md` §9 (Zenodo/OpenTimestamps/IETF/Malleable)
is already carried in `governance/open-publication-and-ip-stewardship.md` and needs no thread; §7
(NLnet-NGI funding, C2PA media-provenance, DFRLab middleware) is partially in `socialization/brand-and-voice`
and rides T42/T48. **Refresh discipline applies to all:** volatile version/price/date facts must be
re-confirmed before external use, and iroh version facts cite the FACTCHECK SoT (do not re-verify).

> **PROMOTED 2026-07-08 (beta maturity; verification flags travel inline in the docs).** All eight threads
> were promoted into real layer docs. They stay listed here for one review cycle, then move to
> `CLOSED-THREADS.md` once the user has eyeballed the docs. Residual per-claim verification flags and the
> coupled user-decisions (A9, A10, T7, T33, the legal-review gate) are carried in the docs / their own
> trackers, not re-litigated here. Promotion map:
>
> | Thread | Promoted into |
> |---|---|
> | T41 | `cairn/substrate-prior-art.md` |
> | T42 | `cairn/identity-and-data-ownership-poles.md` |
> | T43 | `cairn/cross-platform-identity-provenance.md` |
> | T44 | `cairn/atproto-selfhosting-appviews-and-bridges.md` |
> | T45 | `cairn/atmospheric-web-and-aggregators.md` |
> | T46 | `cairn/iroh-app-pond-building-blocks.md` |
> | T47 | `cairn/object-capability-and-decentralized-mls-prior-art.md` |
> | T48 | `governance/cooperative-and-governance-prior-art.md` |
>
> One residual to reconcile (from T47): `cairn/willow-meadowcap.md` documents Meadowcap but not Keyhive by
> name, so T47's "Keyhive → willow-meadowcap" pointer runs slightly ahead of that file — add Keyhive there
> or soften the pointer in a later pass.

#### T41 — Substrate prior-art register (Peat + recursive-federation routing)

- **Layer:** cairn, impl
- **Status:** `open · surfaced`
- **Type:** `needs-content` (+ `needs-research` to refresh volatile facts)
- **What it is:** ECOSYSTEM §1 substrate rows, un-carried. Headline: **Peat** (Defense Unicorns) — the
  strongest existing prior-art for Croft's *exact* bet (Rust + iroh + Automerge CRDT + MLS, proven in
  denied/degraded; also §6). Plus recursive-federation routing prior-art (**RINA**; Named Data Networking /
  **Yggdrasil** / cjdns) and the homage tier (libp2p rejected-as-primary; Veilid/Holochain demoted;
  **p2panda**, **iroh-rings** as the closest peer-equality/capability neighbors). cairn credits weaker
  neighbors but omits the closest.
- **Promotion target:** new `cairn/substrate-prior-art.md` (or extend `adjacent-systems.md`); Peat also
  cross-referenced from `impl/` where the substrate bet is argued.
- **Gates:** confirm Peat facts (github.com/defenseunicorns/peat) and refresh iroh/companion-crate versions
  against the FACTCHECK SoT before external use.
- **Alpha provenance:** `../alpha/ECOSYSTEM.md` §1, §6 (Peat), §2 (deployment status).

#### T42 — The two identity/data-ownership poles: Solid & DSNP

- **Layer:** cairn
- **Status:** `open · surfaced`
- **Type:** `needs-content`
- **What it is:** ECOSYSTEM §5. **Solid + WebID / Solid-OIDC / DPoP (RFC 9449)** — the private-by-default,
  direct-access pole; and **DSNP + Frequency** — the social-graph-as-public-utility, on-chain pole
  (no built-in token in the core; delegation to user-agents without surrendering master keys). The
  load-bearing positioning that Croft sits *between* the poles with an E2EE private layer that is neither,
  sharing DSNP's unbundle-the-social-web + delegation goals while rejecting the chain, never reached beta.
- **Promotion target:** new `cairn/identity-and-data-ownership-poles.md`.
- **Gates:** web-verified 2026-06-22; refresh volatile facts before external use.
- **Alpha provenance:** `../alpha/ECOSYSTEM.md` §5 (Solid, DSNP rows).

#### T43 — Cross-platform identity provenance & the did:webvh↔did:plc chain

- **Layer:** cairn, drystone-spec (couples T7, A9, A10)
- **Status:** `open · gated`
- **Type:** `needs-content` (+ `needs-research` for the atproto-native-support confirm)
- **What it is:** the audit's top single gap (ledger J8 / B14). ECOSYSTEM §4: **did:webvh** (fka did:tdw;
  SCID-anchored append-only log, pre-rotation, `portable:true` genesis-only for "credible exit"),
  `didwebvh-rs`/`didtoolbox`, **plc.directory as a transparency-log-not-CA** and its centralization soft
  spot, `goat` PLC-op flow, the DIDComm hold-and-forward **delegate** prior art, and CT/CONIKS
  equivocation-*detection*. The load-bearing thesis: OOB mutually-anchored / root-signed provenance
  attestation is the *only* real cross-platform linkage; **hub-and-spoke** (did:webvh SCID root, did:plc
  spoke); plus the **negative result** that atproto resolves only did:plc/web/key, so did:webvh is not
  natively usable → the bidirectional `alsoKnownAs` workaround. Beta carries did:webvh only as an
  unexplained gate (T7) and the A9/A10 open decisions, not the conclusion they rest on.
- **Promotion target:** new `cairn/cross-platform-identity-provenance.md` (or `philosophy/`); feeds
  `drystone-spec/` §5 identity.
- **Gates:** `did:webvh` native atproto support confirmed vs FACTCHECK SoT (couples T7); the anchor-URI
  stability contract (**A9**) and PDS-held-vs-self-controlled rotation key (**A10**) are the user's calls.
- **Alpha provenance:** `../alpha/ECOSYSTEM.md` §4; `../alpha/thinking/cross-platform-identity-provenance.md`,
  `plc-identity-resilience.md`.

#### T44 — atproto self-hosting, AppViews & bridges (incl. Groundmist)

- **Layer:** cairn, governance (hosting economics)
- **Status:** `open · surfaced`
- **Type:** `needs-content` (+ `needs-research` to refresh volatile pricing)
- **What it is:** ECOSYSTEM §5e (PDS impls: Cocoon/Go+PG, **rsky-pds**/Rust, ElfHosted/DigitalOcean/Hostinger
  managed hosts; blob backends B2/R2/Hetzner/Wasabi with the Wasabi 90-day-retention trap and the
  cold-tier retrieval-penalty trap; the MinIO-archived and CAR-export corrections) + §5f (**Groundmist** —
  closest local-first-private atproto relative, headline; AppViewLite; **rsky-wintermute** private-community
  scaffolding; zeppelin full reference stack; **Jetstream** ingestion; Ouranos/Heron/atcute client bases;
  goat / PDS MOOver migration; **Bridgy Fed** CC0 + **Bounce** credible-exit; #IndieSky / Eurosky /
  Free Our Feeds infra movements). Together these demonstrate a self-run AppView inverting atproto
  public-by-default (the sovereign-AppView private-blocking / asymmetric-federation conclusions, ledger
  A14/A15).
- **Promotion target:** extend `cairn/blacksky-and-atproto-community.md` or new
  `cairn/atproto-selfhosting-appviews-and-bridges.md`; hosting-cost economics feed
  `governance/foundation-cooperative-and-sustainability`.
- **Gates:** prices are point-in-time (mark illustrative, refresh); preserve Groundmist's "private-by-default
  is intent, not yet security (ships auth-disabled)" caveat.
- **Alpha provenance:** `../alpha/ECOSYSTEM.md` §5e, §5f; `../alpha/research/atproto-sovereign-appview-club.md`.

#### T45 — Atmospheric-web apps & the aggregator license-map

- **Layer:** cairn, croft, socialization
- **Status:** `open · surfaced`
- **Type:** `needs-content` (+ `needs-research`: §5c is dialogue-sourced `[UNVERIFIED]`)
- **What it is:** ECOSYSTEM §5b (atmospheric-web apps: Tangled, WhiteWind, Leaflet/Standard.site, Semble,
  Streamplace, Flashes, ATmosphere-WordPress, **Graysky** custom-namespace exemplar, **Tap** backfill) +
  §5c (Rust/client tooling behind ports: Jacquard, megalodon-rs, lemmy-client-rs, Crux, Tauri, Leptos,
  Phanpy, deck.blue, **Openvibe** fused-timeline *anti-pattern*, Fedilab, webxdc). Load-bearing pieces: the
  aggregator/fork **license-map** + the **"AP/atproto have no per-activity gas → build a TweetDeck for the
  open web, write your own adapters"** finding (feeds the garden-of-ponds aggregator strategy), and the
  demand-side "atmospheric web / Neo-GeoCities / open-LinkedIn" adoption argument.
- **Promotion target:** new `cairn/atmospheric-web-and-aggregators.md`; feeds
  `croft/product-the-garden-of-ponds` (aggregator ponds) and `socialization/adoption-strategy` (demand side).
- **Gates:** §5c dialogue-sourced — verify before reliance; §5b web-verified, refresh volatile facts.
- **Alpha provenance:** `../alpha/ECOSYSTEM.md` §5b, §5c; `../alpha/thinking/atproto-atmospheric-web.md`.

#### T46 — iroh app-pond building blocks: games, realtime media, on-device AI

- **Layer:** cairn, croft
- **Status:** `open · surfaced`
- **Type:** `needs-content` (+ `needs-experimentation` for the media floor)
- **What it is:** ECOSYSTEM §5d (sendme; **libmarathon** Bevy+iroh+gossip+CRDT; ascii-royale; iroh-lan;
  godot-iroh; DataBeam; webxdc game catalog; netplayjs; **GGRS+matchbox** rollback; Curvytron; boardgame.io;
  **Cure53 webxdc audit** → disable webview WebRTC; on-device AI Foundation Models / Gemini Nano; the
  **Bond Touch** "thinking-of-you" anti-pattern) + the in-the-wild iroh realtime-media proof
  (callme / iroh-roq "proven Opus floor", ledger F19/B24), which beta spec §6.12 currently asserts with
  no wild reference. These are the games-pond and calls-pond building blocks; games is the named cold-start
  hook.
- **Promotion target:** new `cairn/iroh-app-pond-building-blocks.md`; feeds
  `croft/product-the-garden-of-ponds` (ponds) and a reference note in `impl/transport-iroh-gossip-and-quic.md`
  or spec §6.12 for the media floor.
- **Gates:** license-at-bundle checks (several GPL-3.0 / CC-BY-SA-3.0 traps flagged); the media-floor rows
  are Gemini-sourced (flag suspect; the callme/iroh-roq floor is corroborated).
- **Alpha provenance:** `../alpha/ECOSYSTEM.md` §5d; `../alpha/thinking/realtime-media-over-iroh.md`.

#### T47 — Object-capability & decentralized-MLS prior art

- **Layer:** cairn, drystone-spec (couples A11, T22, T29)
- **Status:** `open · surfaced`
- **Type:** `needs-content`
- **What it is:** two smaller ECOSYSTEM strands not fully carried. **Object-capability:** Spritely
  **Goblins / OCapN / CapTP** (§4 — "designation is authorization," POLA, petnames; also a no-VC/no-token
  NLnet-NGI-funded governance model worth crediting). **Decentralized-MLS siblings:** **DMLS / FREEK**
  and **`draft-xue-distributed-mls` ("TwoMLS")** (§2) — the closest serverless-MLS relatives, which
  quantify the fork→forward-secrecy cost the spec's §7 ordering incurs. (Keyhive / Meadowcap in §4 are
  already the A11 Track-A/B capability decision — cross-reference, do not re-file.)
- **Promotion target:** extend `cairn/mls-and-mimi.md` (decentralized-MLS siblings) + a note in
  `cairn/adjacent-systems.md` (Spritely object-capability); feeds `drystone-spec/` §7 and the A11 decision.
- **Gates:** DMLS/FREEK + draft-xue carry a "confirm before publish" flag in the source; refresh Spritely
  facts.
- **Alpha provenance:** `../alpha/ECOSYSTEM.md` §2 (DMLS/FREEK, draft-xue), §4 (Spritely, Keyhive, Meadowcap).

### Drystone-spec / impl (Layers 4, 5)

**Rights, roles, and capabilities.**

#### T31 — Disentangle rights / roles / capabilities / delegation (+ PeerSet, restricted combinations)

- **Layer:** drystone-spec, philosophy
- **Status:** `open · gated`.
- **Type:** `needs-content` (absorbs the former T21; informs T20, T3).
- **What it is:** the review found these four conflated in places and asked to clear them up. The author's
  working definitions, to be reconciled with the spec (they are mostly settled in the user's mind, not yet
  uniformly worded in `drystone-spec` Part 2 §5):
  - **Rights — inherent.** The combination of factors that makes a valid peer; remove them and the system
    itself degrades. Never delegated, never conferred.
  - **Capabilities — intrinsic.** What a peer *can do* by virtue of what it is (a radio, a push-notification
    token, 16 cores / 10 TB RAM). They differ peer-to-peer and make a peer more/less suited to a role, but
    **never** confer governance dominance — a high-capability peer has the same rights as any other.
  - **Roles — delegated.** Assigned, de-facto or by governance (e.g. the Creator role, reassignable by
    vote). Can be **mutually exclusive**; a role may be single-instance.
  - **PeerSet — a named bundle of roles + capabilities with a functional expectation** (e.g. the meer /
    blind-broker), packaged so governance can reason about and assign it as one named thing.
  - **Restricted combinations — fail noisy, not silently forbid.** Rather than hard-forbidding a role
    combination (the "must-never" case), if mutually-exclusive roles are voted together the group should
    **fail loudly** (lock / notify everyone / pause), surface "this combination puts your communication at
    risk," and offer human adjudication — e.g. fork the group excluding the peers who made that vote. (The
    motivating case: a group voting to make a known high-availability meer *no longer blind* changes the
    trust dynamics of the whole group — that is a human-trust situation, handled gracefully, not a silent
    capability flip.) The meer's decrypt-vs-blind line should be settled in terms of **MLS capabilities**.
  - **The `share` right (absorbs the former T21).** Of the four rights (Part 2 §5.3: tenure / exit / voice /
    share), `share` is the least-settled: if governance or a membership class can dilute it, part of it
    behaves like a *capability*, not a right. The sharper reframe from the review: a peer has no right to
    another peer's resources, so the real question is whether a peer has a **right to communicate** and what
    boundary still respects the right to exit and the right to fork. Settle `share` against this line.
- **Promotion target:** `drystone-spec` Part 2 §5 (sharpen rights/role/capability/PeerSet wording + add the
  restricted-combination / fail-noisy handling, and settle the `share` right/capability question absorbed
  from T21); informs **T20** (C10 ban-evasion / moderation) and **T3** (the meer's moderation boundary).
- **Gates:** reconcile the definitions against the current spec §5 wording (avoid duplication); decide
  whether restricted-combination fail-noisy handling is substrate or app layer; settle the meer decrypt
  capability in MLS terms.
- **Alpha provenance:** `beta/thinking/raw/open threads review Jun 26 at 8-17 PM.txt`;
  `../alpha/thinking/rights-vs-capabilities-definitions.md`; `drystone-spec` Part 2 §5.

#### T3 — Moderation & abuse under a blind broker (the constructive design body)

- **Layer:** drystone-spec, impl
- **Status:** `open · gated`.
- **Type:** `needs-content` (couples T31).
- **Review (2026-06-26):** the meer PeerSet holds the **same rights as a full peer** but has no local
  history and **never sees content** — so you **cannot moderate on content** through it. Moderation is
  therefore **separate from the meer PeerSet**, scoped to abuse / side-channel signals (a distinct
  delegation stack). Pin the meer's decrypt-vs-blind line in terms of **MLS capabilities** (see T31).
- **What it is:** the operational complement to 06's "safe by structure, not inspection" thesis — what a
  content-blind broker actually *does* about spam / CSAM / coordinated harm: client-side
  report-with-reveal, metadata-based rate-limiting, reputation, and the `{pending,released,rejected}`
  predicate-gated **hold/release plane** + **crypto-shred** — "must be designed in, not bolted on." Plus
  the **kid-friendly-vs-uninspectable** product tension.
- **Promotion target:** **06** (the design body it currently only gestures at). The CSAM/jurisdiction
  *legal* posture is already surfaced (06→07); this is the distinct *engineering/design* thread.
- **Gates:** decide the abuse-handling toolkit; reconcile with the geer's consented-visibility role;
  decide whether crypto-shred + hold/release ship in the substrate or app layer; the legal/CSAM piece
  (the user's) gates the rest.
- **Alpha provenance:** `../alpha/thinking/open-considerations.md` §5 + §9; `../alpha/ROADMAP_TODO.md`
  **D3 / D6 / E18**; `../alpha/COHESION.md` §18.

#### T20 — Conflict-reason corpus gaps (C4 / C7 / C8 / C9 / C10)

- **Layer:** drystone-spec
- **Status:** `open · gated`.
- **Type:** `needs-content` (expand per S2 — too terse to reason from).
- **Review (2026-06-26):** too terse — needs a plain-language **problem statement** per gap plus the
  **solutions we already preferred** when we discussed them, not just the C-codes.
- **What it is:** `merge-split-corpus.md` §4 enumerates the full conflict-reason space and surfaces five
  real, **unmodeled/partial reconcile-semantics gaps**: **C4** add-vs-add of the same person on different
  device keys across a partition (must fold by lineage, not double-count — interacts with multi-device
  E2.10); **C7** dissolve-vs-continue (hard-stop or resting-state, *undefined*); **C8** diamond-recombine
  conflict over a multi-parent DAG (topology proven, conflict-detection untested); **C9** equivocation
  hardening (A2.2 partial); **C10** ban-evasion re-add via a new device leaf (must not silently re-confer
  standing — the moderation surface).
- **Promotion target:** **04** (widens "what was proved" toward the full conflict space). **Overlaps T5**
  (scale/churn) but is a distinct surface (reconcile *semantics*, not scale) — confirm and fold where
  subsumed by T5.
- **Gates:** define C7's intended resolution; extend `detect` to multi-parent ancestry (C8); harden
  equivocation attribution (C9); model the ban-evasion re-add (C10).
- **Alpha provenance:** `../alpha/thinking/merge-split-corpus.md` §4 + §6 ("Tier 1b — reconcile-case corpus").

> **T21–T22 added 2026-06-26** from the rights-vs-capabilities grounding (was folded into `01` §5, K17;
> now in the Drystone spec — `drystone-spec` Part 1 §2.3 + Part 2 §5.3). The discriminating test and the
> four-rights cut are settled and in the spec; these are the two **verify-before-hardening** checks
> deliberately kept out of the spec's normative rights set — they gate hardening the four-rights *closed
> set*. (Were ROADMAP_TODO E32 b/c.)

**Scale validation.** T2, T5, and T38 share one test harness; T5's descriptive failure-modes now live in `../fenced/group-chat-failure-modes.md`.

#### T2 — Governance at scale (subsidiarity + liquid delegation; the concentration default)

- **Layer:** drystone-spec, impl
- **Status:** `open · gated`.
- **Type:** `needs-experimentation` (couples T5 — the test harness).
- **Review (2026-06-26):** likely **more than one large-scale governance model**, to hypothesize then
  play out on the test harness: (a) revocable/movable **delegate-vote** (liquid democracy); (b)
  **elected-admin / Reddit-style moderation** ("all-or-nothing" participation where peers still hold
  rights and vote with their feet); (c) **broadcast-only groups** (a different rights model entirely).
  Needs the harness to test scale across variety + quantity + peer count.
- **What it is:** how a centerless federation governs at scale (the ~200k breakpoint) without the
  cheap-fork Sybil defense getting expensive and without quietly growing a center — likely subsidiarity +
  instantly-revocable **liquid delegation**, with **concentration as the default failure** (the
  Pirate-Party lesson) resisted by decay/caps/bounded-chains/expiry/visibility, and **member ≠
  governance-constituent** modeled explicitly. Includes the honest admission that the membership
  sequencer / superpeer is a **load-bearing centralization point** whose funding/uptime/governance must
  be named as core, and the federation/inter-collective peering design surface.
- **Promotion target:** completes the federation handoff that the spec's peer-equality principle opens (`drystone-spec` Part 1 §2.3, Part 2 §5) and **07 B5** gives a legal
  shape to; touches **06** (Sybil softening). Likely a dedicated governance theme alongside T1.
- **Gates:** decide the delegation model; pick concentration-resistance levers; model
  member-vs-constituent; spec federation/peering; name the center's funding/governance honestly.
- **Alpha provenance:** `../alpha/ROADMAP_TODO.md` **D9** (+ D8 residual, E16 design surface);
  `../alpha/COHESION.md` **§22**; `../alpha/thinking/open-considerations.md` §4 (load-bearing superpeer);
  `../alpha/thinking/local-first-as-design-imperative.md` (open frontiers).

#### T5 — Protocol behavior at scale / group-chat failure modes

- **Layer:** drystone-spec, impl, fenced
- **Status:** `open · gated`.
- **Type:** `needs-experimentation` (couples T2 — same test harness).
- **Review (2026-06-26):** flows together with T2 — the gating work is to **build the test harness** and
  stand up conformance + performance + quality test cases (scale across variety, quantity, and peer
  count).
- **What it is:** the honest gap in 04's "we proved it" — 04 explicitly does **not** establish large-scale
  behavior or real-world fold/unfold UX. The open design questions: does survivor-selection need the
  superpeer to be deterministic (the project's honesty hinges on it); the superpeer-as-covert-ordering
  risk (is the pure-P2P tier "a demo"); immutable genesis-threshold amendability vs regress-grounding; and
  the **churn-fold Achilles heel** (governance-log noise from device churn making the member-list fold
  unmaintainable) with its concrete, unactioned recommendation to add a **synthetic high-churn /
  multi-partition test now**.
- **Promotion target:** **04** (widens it from "proved at human scale" toward production-shaped claims).
  **Overlaps T1's §X** conflict model — several questions may be T1's validation surface; confirm and fold
  where subsumed.
- **Gates:** survivor-selection determinism decision; pure-P2P-vs-superpeer ordering honesty; genesis
  amendability; write + run the churn/partition test (the test itself is alpha validation).
- **Alpha provenance:** `../alpha/research/group-chat-failure-modes.md` (+ `-plain.md`);
  `../alpha/crystallized/conclusions.md`; the test-plan backlog.

#### T38 - The two unearned measurements and the §11.10.1 experiment matrix (turn the scaling envelope into a sized one)

- **Layer:** drystone-spec, impl
- **Status:** `open · gated` (specified in the Part 2 §11 large-group-scaling section, not yet run).
- **Type:** `needs-experimentation` (couples-with `needs-proving`).
- **What it is:** the Part 2 §11 large-group-scaling section tags two measurements `Load-bearing, unearned` (§11.11):
  (1) per-commit and fan-out cost at hot-N = 500 / 1000 / 2000 on representative hardware (sets the real
  hot-N comfort ceiling, provisionally ~1500, and whether the 3–7k / 7–10k hot trees need sharding; extends
  to attesting-N 5,000 / 10,000 / 20,000 for the experimental public regime, to place the single-tree
  attesting-core ceiling in the band between the measured ~5,000 (Soler 2025) and RFC 9750's
  tens-of-thousands design target); and (2) return-backfill time as a function of dormancy-gap size (sets
  whether the §11.6 liveness windows are right). §11.10.1 states the full buildable experiment matrix
  (Experiments A–G, symbols, fixed policy, sweep points, pass/fail thresholds) against an OpenMLS-on-aarch64
  harness plus a gossip testbed.
- **Promotion target:** the section's tier numbers and the hot-N comfort ceiling move from reasoned envelope
  to measured once the matrix runs; the `[gates-release]` marker clears for any figure that becomes a
  stated SLA.
- **Gates:** build/instrument the OpenMLS harness and the gossip testbed; run Experiments A–G; pin and
  record ciphersuite, credential type, library version, and device SoC with every result set.
- **Provenance:** Part 2 §11 (large-group scaling), §11.10.1 + §11.11; document-pass-9, renumbered to §11 at document-pass-10 (2026-07-07).

#### T36 — Verify the re-plant instantiation mechanism folded into Drystone spec §7.6.4 (run the E12 set)

- **Layer:** drystone-spec, impl
- **Status:** `open · gated` (folded into the spec 2026-07-07 with a `needs verification` tag, on the user's
  instruction to fold in-context now rather than hold it in `impl/`).
- **Type:** `needs-proving` (couples-with `needs-experimentation`).
- **What it is:** the detailed MLS re-plant / atomic-swap mechanism (from `impl/delivery-layer/` and
  `impl/mls/`) was folded into Drystone spec **Part 2 §7.6.4** as design-in-context: unilateral O(N)
  instantiation, KeyPackage-per-member seating with the last-resort availability floor, fresh-stamp
  group-wide key refresh (PCS) with the last-resort exception, blank-node cost reset, planter
  byte-nondeterminism as dedup-not-fork, stale-`GroupInfo` external-commit PCS integrity, and the
  `epoch_authenticator` fold-not-parallel candidate. Every mechanism claim is grounded against a named RFC
  section, but the **composed operation is not yet exercised end-to-end on a real stack**, so §7.6.4 carries
  `[confirm before publish]` throughout.
- **Promotion target:** Drystone spec Part 2 §7.6.4 (already folded in-context); this thread tracks moving
  its status from `design; needs verification` to `green-real` once validated.
- **Gates:** run the **E12 experiment set** (E12.1–E12.7) against `mls-rs 0.55.2` (Rung A for MLS mechanics;
  Rung B for Drystone's own governance-chain and dataplane hash structures, which are not yet built, so
  E12.7 is modeled). Resolve the two library questions: whether `mls-rs` exposes ReInit as a first-class op
  emitting the resumption PSK (vs. fresh-create + manual PSK), and whether it surfaces resolution/blank
  counts directly (vs. a byte-size proxy). Resolve the spec's Appendix B re-plant items (intent ordering
  before the ReInit freeze; seating default Welcome vs external-commit; PSK cross-group linking; epoch-number
  metadata vs re-plant frequency; `epoch_authenticator` overlap).
- **Provenance:** folded from `impl/delivery-layer/12-replant-experiments.md` + `01-delivery-architecture.md`
  and `impl/mls/mls-hardcases-and-posture.md` (batches 7–8); the fold is Drystone spec document-pass-7
  (2026-07-07). The impl/ design corpus is retained as the derivation + experiment plan.

#### T22 — Does the `04` survivor re-key strand a peer's `tenure`?

- **Layer:** drystone-spec, impl
- **Status:** `open · gated`.
- **Type:** `needs-proving` (run the test).
- **Review (2026-06-26):** **tenure** may be the user's version of the "right to share" (T21) — *tenure =
  the ability to functionally be a peer to other peers*. This needs **testing** against the survivor /
  re-key path — "we should just do that."
- **What it is:** `tenure` (standing to remain a peer) is stated in `drystone-spec` (Part 2 §5.3) as an
  absolute right. But the `04` survivor / re-key mechanism could, in implementation, **strand a peer** (leave
  it unable to rejoin a re-keyed group). If so, `tenure` has an implementation-level exception that must be
  **named explicitly** rather than left absolute — otherwise the boundary over-claims.
- **Promotion target:** **04** (the survivor/re-key mechanism — does it preserve tenure, and under what
  bound) + a precise caveat back into `drystone-spec` Part 2 §5.3 if an exception is real.
- **Gates:** a protocol-level check of the re-key/survivor path against the tenure claim; if an exception
  exists, specify its bound; then the four-rights closed set can harden.
- **Alpha provenance:** `../alpha/thinking/rights-vs-capabilities-definitions.md` (the two open checks);
  `../alpha/ROADMAP_TODO.md` **E32 (c)**; `drystone-spec` Part 2 §5.3; `beta/04` (survivor re-key) / `beta/05` §7.

> **Folded into existing, not new threads:** the inter-collective peering *settled shape* (BGP-autonomy +
> postal-hierarchy + signed routing) → add to **T2**'s provenance so T2 doesn't re-derive it. **Borderline
> (engineering, likely ROADMAP not a beta thread):** the Automerge-over-application audit, and the Wire
> `core-crypto` (GPL-3.0) vs `openmls`/`mls-rs` engine+license decision (the latter couples to 07's
> flagged MPL-vs-AGPL substrate item).

> **T24 added 2026-06-26** from the Beer/OGAS intake — the unsettled design question that fell out of
> peerhood-as-adjudication. (Distinct from the now-closed T23, which was just the verbatim-Beer sourcing.)

**The public-by-default regime.** T39 is the cryptographic gate the T40 regime rests on.

#### T39 - Non-member-verifiable membership attestation (the mechanical core of the experimental public regime)

- **Layer:** drystone-spec
- **Status:** `open · gated` (`Load-bearing, unearned`; the whole §11.9.3 public regime is gated on it).
- **Type:** `needs-proving` (a real cryptographic design problem, sketched not solved).
- **What it is:** the experimental public-by-default regime (§11.9.3) routes a group's public message stream
  through an MLS-aware relay bridge into an AppView-shaped read view (§11.9.3.1). The open problem: let a
  *non-member* reader verify "attested member at standing X authored this item" from a forwarded artifact,
  without trusting the bridge and without the reader being an MLS member (which would defeat the
  read-outside-the-tree model). It may reduce to signing authored items with a credential chain verifiable
  against the group's published membership and governance state, but that is a sketch, and it composes with
  the lineage-attestation questions already open (the two-part credential and its ban-lineage interlock,
  the resume-vs-fresh identity fork, single-member time-delayed resumption-PSK presentation).
- **Promotion target:** on a proven mechanism, §11.9.3 moves from Design-experimental toward Design; until
  then the entire public regime is a candidate direction to prototype, not a committed part of the spec.
- **Gates:** solve and adversarially analyze the attestation-extraction; prototype the bridge; confirm the
  read path stays in the "helper cannot lie, only stall" box (content-addressed, governance-positioned
  items, gap-detectable omission).
- **Provenance:** Part 2 §11 (large-group scaling), §11.9.3.1 + §11.11 item 7; document-pass-9, renumbered to §11 at document-pass-10 (2026-07-07).

#### T40 - The public-by-default regime as a whole: status and the confidentiality-inversion posture

- **Layer:** drystone-spec
- **Status:** `open · experimental` (more speculative than the rest of the section; a candidate direction).
- **Type:** `design-experimental`.
- **What it is:** above roughly 7k members the section permits inverting the confidentiality model
  (§11.9.3): messages public by default (the AppView read view is the primary surface), MLS retained not to
  encrypt messages but for attestation and membership. The regime concedes Force 2 on purpose while keeping
  Force 1 (cryptographic membership), which is the honest inversion of the incumbent large-platform posture
  (payloads-encrypted-yet-server-can-inject-a-member). It carries a heavier honest-residual burden than the
  tiers below it: forward/post-compromise security relocate from message content to the attestation layer;
  a member MUST be able to tell unmistakably that the space is public; governance is more exposed against a
  public backdrop; the performance win is real on fan-out (indexed reads) and on the rate of expensive
  operations, but NOT on MLS's per-operation cost. It rests on a chain of plausible-but-unvalidated
  propositions and is gated by T39.
- **Promotion target:** prototype and stress the regime; if it holds, it graduates from candidate direction
  to a specified optional tier. It interacts with the T37 placement decision (a whole experimental
  subsection).
- **Gates:** T39 (the bridge attestation); a UX/consent design for the public-space clarity requirement; a
  decision on whether governance events are public at this tier; the §11.9.3.3 attesting-core ceiling
  measurements (part of T38).
- **Provenance:** Part 2 §11 (large-group scaling), §11.9.3 (and §11.9.3.1–§11.9.3.3); document-pass-9, renumbered to §11 at document-pass-10 (2026-07-07).

**Real-time media.**

#### T10 — Real-time media-layer hardening (finishes 04's media leg)

- **Layer:** drystone-spec, impl
- **Status:** `open · gated` (largely de-risked — a "close the last decisions" thread).
- **Type:** `needs-experimentation`.
- **Review (2026-06-26):** still deferred for now.
- **What it is:** 04 carries media only as *characterized* (E12 green-real on synthetic frames). str0m is
  production-grade server-side (weak exactly on P2P ICE, which Croft routes around) and the RoQ/MoQ split
  is adopted; the residual `[OPEN]` is whether str0m's strong/weak boundary is precisely tested, which
  sets the browser-facing SFU-meer exposure — feeding the pending TC-ENG0 (engine API audit) and TC-INT3
  (A1-vs-A2 engine/transport decision).
- **Promotion target:** **04** (hardens the media leg from "characterized" toward production-shaped).
- **Gates:** TC-ENG0 done; TC-INT3 decided; the str0m P2P-ICE boundary `[OPEN]` closed.
- **Alpha provenance:** `../alpha/research/str0m-production-readiness.md`,
  `../alpha/research/iroh-realtime-media-references.md`; `../alpha/thinking/realtime-media-over-iroh.md`.

> **T11–T17 added 2026-06-25** from a content-level completeness audit (four readers across
> `crystallized/`, `thinking/`, `narrative/`+dossier, `research/`+index) hunting alpha material walked
> out but not manifested in beta. These are the **unsettled** finds; the audit's **settled-but-unfolded
> conclusions** (the bigger bucket) are tracked separately in `../alpha/BETA-ROLLUP.md` → "Settled
> conclusions not yet folded," because they belong *in* the themes, not here.

#### T13 — Encrypt-then-content-address kills cross-user dedup (media storage economics)

- **Layer:** drystone-spec, impl
- **Status:** `open · gated`.
- **Type:** `needs-research` (needs a lot of thinking + comparative research; won't fit one file).
- **Review (2026-06-26):** possible directions to think through (none decided): treat a **group as a
  principal** and encrypt the asset once as a single **group blob** → dedup at group scale; or a
  **key-envelope** scheme (encrypt the content once, wrap the content-key per recipient). Needs
  comparative research — how do **Proton** (end-to-end encrypted file store) and **peergos** handle this.
  Flagged as one of the threads that clearly needs its own file.
- **What it is:** same media + different nonces ⇒ different ciphertext hashes ⇒ **no cross-user dedup**; for media-heavy use this breaks the storage math the survivability fund was costed on. A genuine seam between the media layer and the funding model (distinct from T10's media *transport* hardening).
- **Promotion target:** the **04/08 (media) ↔ 07 (survivability-fund costing)** seam.
- **Gates:** decide the storage/dedup posture and re-cost the fund accordingly.
- **Alpha provenance:** `../alpha/thinking/open-considerations.md` (the dedup item); `../alpha/experiments/encrypted-blob-share/`.

**Substrate and other.**

#### T6 — The per-platform trust-model doc (05's "highest-leverage next artifact")

- **Layer:** drystone-spec, impl
- **Status:** `open · gated`.
- **Type:** `needs-content` (couples T14; structural directive S4).
- **Review (2026-06-26):** still deferred; subsumed by the **per-platform design-files** directive (S4) —
  the per-network trust write-up is one slice of the per-platform design thinking.
- **What it is:** the per-network (Bluesky/AP/Mastodon/GoToSocial/Threads/Hive) write-up — the field used,
  what Croft claims / doesn't claim, the backlink mechanism, exact verifier steps + pseudocode. 05 *names*
  it as the highest-leverage next artifact but cannot assert its content because it does not exist.
- **Promotion target:** **05** (completes the identity theme).
- **Gates:** write it; confirm `alsoKnownAs` extra-entry persistence (`[UNVERIFIED]`, E14); resolve the
  anchor-URI stability contract (A9) and the PDS-vs-self-controlled rotation key (A10), which determine
  what each spoke can claim; depends partly on T7.
- **Alpha provenance:** `../alpha/ROADMAP_TODO.md` **E13** (+ A9/A10/E14);
  `../alpha/thinking/cross-platform-identity-provenance.md:222`; `beta/05` boundary. **Per-platform home
  (S4, started 2026-06-26):** `../alpha/thinking/app/platforms/`.

#### T8 — Forward-only revocation under irreversible commitments

- **Layer:** drystone-spec
- **Status:** `open · gated`.
- **Type:** `needs-content` (likely co-promotes with T1).
- **Review (2026-06-26):** the work, restated plainly — define the **reversible-vs-committing decision
  tag**, spec the **permanent attribution record**, and reconcile it with T1's append-only fold.
- **What it is:** revoking consent cannot rewind a spent action; decisions must be tagged
  reversible-or-committing **at decision time**, and the record must permanently, honestly attribute which
  consent supported which irreversible consequence. The governance-plane face of the recovery/consent
  problem; `drystone-spec` (Part 2 §5.6) states the *principle* (irreversible → maximal protection of rights where exit cannot help) but never
  names the *mechanism*.
- **Promotion target:** **04 / 06** (the governance log + revocation ladder); **01** (the
  protection-rigidity principle). **Likely co-promotes with T1.**
- **Gates:** define the reversible-vs-committing decision tag; spec the permanent attribution record;
  reconcile with T1's append-only fold.
- **Alpha provenance:** `../alpha/ROADMAP_TODO.md` **D10**; `../alpha/COHESION.md` **§22**; `drystone-spec` Part 2 §5.6.

#### T18 — LTS-for-interfaces / shapeability-paired-with-stability

- **Layer:** drystone-spec, impl
- **Status:** `open · surfaced`.
- **Type:** `needs-content` (impacts all platform implementations).
- **Review (2026-06-26):** considered **pretty much settled** as a design ethos that impacts every
  platform implementation; the open part is "what does it look like," not whether to adopt it.
- **What it is:** `principles.md` Tier 3 carries a settled-*stance* principle absent from beta:
  "**shapeability is only valuable paired with stability; constant UI change is quietly extractive**" —
  with a concrete **LTS-for-interfaces** mechanism (alpha/beta/stable channels, ~3yr stable window, opt-in
  change "trains" on a ~6mo cadence, the *learned surface* held stable, security changes the
  over-communicated exception, multiple live UI generations carrying an honest documentation/support cost).
  The product-layer twin of the non-extraction thesis: composability without stability *is* the extraction
  lever ("change-it-back friction becomes an engagement lever").
- **Promotion target:** **08** (a stability/shapeability product principle, paired with the composability
  stance it already carries); seam to **07** (anti-extraction). **Distinct from T17** — T17 scopes the
  three-audiences settings model + composable-interface ramp; this is the separate stability principle.
- **Gates:** decide the LTS channel/cadence model as a product *commitment* vs aspiration ("name the
  documentation/support cost or the principle dies in year two").
- **Alpha provenance:** `../alpha/crystallized/principles.md` Tier 3.

#### T19 — Blind-peer encrypted-search / coverage-attestation substrate

- **Layer:** drystone-spec, impl
- **Status:** `open · gated`.
- **Type:** `needs-experimentation` (optional / low appetite).
- **Review (2026-06-26):** **low appetite** — the user doesn't want to get into this; "seems like a
  losing game." Maybe some light experimentation to see if there's any utility, otherwise leave deferred.
- **What it is:** a substantial *unbuilt* design surface — blind peers expose the **hash-tree skeleton**
  (not payload); search is a bounded subtree scan where **the hash tree is the shard map** and the gather is
  **cryptographically attestable** (each worker returns matches + subtree root hash; coverage is a checkable
  set-cover over hashes); the two offload "animals" (HA-search-member with own copy = safe vs pure
  search-mediator that must be enrolled with decryption = crown-jewel target); and encrypted-search **leakage
  profiles** (deterministic leaks equality / SSE leaks access patterns — "you pick a leakage profile, not
  avoid one"). The author flags **content-predicate search-coverage attestation** as a genuinely-new seam
  wanting its own threat model before code.
- **Promotion target:** **04** (a substrate capability) or a dedicated search/discovery theme; couples to
  the meer roles (06).
- **Gates:** write the threat model; the honest-plaintext-evaluation half ("didn't skip matches after
  decrypting") is the hard, possibly-defer-able piece.
- **Alpha provenance:** `../alpha/thinking/local-first-as-design-imperative.md` (storage-substrate /
  discovery-fulfillment / "what's new" sections).

#### T25 — The Drystone redb storage-and-projection layer (vetted, adaptable local component)

- **Layer:** impl
- **Status:** **`in-progress`** (being built externally, 2026-06-26) — the build spec
  (`../alpha/seeds/generated-prompts/redb-social-graph-layer-build-prompt.md`) is being implemented in a
  separate build environment by the user. Was `surfaced`.
- **Type:** `needs-proving` (the build is the proof).
- **What it is:** the local **derived-state engine** — authoritative signed-assertion store + governance log
  + a rebuildable redb projection (graph adjacency index + declarative snapshot), behind a typed
  query/command/notification surface, with crypto/MLS/credentials/blob I/O as **injected traits** so it slots
  into the existing stack and is testable in isolation. The whole point is a **well-proven, adaptable**
  surface (property tests for order-insensitive convergence / rebuildability / authoritative-vs-derived
  consistency; mutation testing on fold+validation; adversarial, fork, partial-knowledge, compaction, scale).
  Local-implementation, **not** the protocol.
- **Promotion target:** an `experiments/` spike → eventually an implementation; not a beta theme. Couples
  T1 (the protocol the fold validates against) and the redb local-storage choice.
- **Gates:** build it from the prompt; review the property-test **generators** (diverse/forked/partial) and
  the **mutation-survivor list** (where "vetted" is won/lost); the **edge-table representation** (composite-key
  vs multimap) is an explicit build-time measurement.
- **Alpha provenance:** `../alpha/seeds/generated-prompts/redb-social-graph-layer-build-prompt.md`;
  `../alpha/thinking/social-graph-as-substrate.md` §4–5; raw
  `../alpha/seeds/transcripts/raw/social-graph-substrate-redb-storage-dialogue-2026-06-26.md`.

> **T27 added 2026-06-26** — extracted from an inline prior-tier pointer that had been sitting in `beta/05`
> (a `crystallized/principles.md` "flagged for insertion" note). It was cleaned out of the beta doc for tier
> discipline and promoted to a tracked decision here. Logged to correct a pattern of leaving real decisions
> as inline notes rather than surfacing them.

#### T29 — MLS group state ↔ governance-log / Automerge state consistency

- **Layer:** drystone-spec, impl
- **Status:** `open · gated` (an open design binding, spec-relevant).
- **Type:** `needs-experimentation` (scope unclear — experiments vs nail the spec/data shape).
- **Review (2026-06-26):** "a lot going on; unsure what we need" — whether the next step is **more
  experiments** or just **nailing down the spec** (the shape of the data, the typing). Needs scoping
  before it can be worked.
- **What it is:** the design makes the **governance log** authoritative for membership (the append-only fold,
  `drystone-spec` Part 2 §7) and **MLS** the key/epoch layer; "membership is bound to the ratchet" means they
  fork together (Part 2 §4.5.1 / the social-graph synthesis). But the **exact binding** — how MLS epoch
  transitions are driven by, and kept consistent with, the folded governance state, especially under
  concurrent commits / partition / survivor re-key — is **not specified**. The research named this "the exact
  problem Matrix is still solving for MLS-in-federation." It is an `ENABLING`-level integration, distinct from
  the §7 governance-conflict resolution (which orders *facts*) and from the borderline "Automerge-over-
  application audit" engineering note.
- **Promotion target:** `drystone-spec` Part 2 (a new §, or an Appendix B `ENABLING` item) once specified;
  couples T1 (governance), T29's sibling §7.5 frontier-closure, and the survivor/re-key path (T22).
- **Gates:** specify the membership-fact → MLS-commit binding and its behavior at fork/partition/re-key;
  confirm the Matrix-in-federation comparison **[confirm before publish]**.
- **Prior art (added 2026-06-26, [confirm before publish]):** this is the **decentralized-MLS frontier** —
  **DMLS/FREEK** (Phoenix R&D; FREEK = Alwen/Mularczyk/Tselekounis) and **`draft-xue-distributed-mls`** are
  sibling approaches to serverless ordering, drafts/PoC only (no production deployment). **FREEK quantifies
  the exact cost** Drystone's fork model incurs: processing out-of-order commits degrades forward secrecy,
  recovered via a **puncturable PRF** at a **storage cost** that scales with the retention window, group
  size, and **fork frequency**. So Drystone's "forks self-heal by deterministic tie-break in the retention
  window" is **not free** — the FS price is retain-and-puncture key material; this couples **T22**
  (survivor/re-key vs tenure). Design against, or adopt, the FREEK mechanism.
- **Alpha provenance:** `research/messaging-solutions-landscape.md` §top-unresolved #3;
  `thinking/social-graph-as-substrate.md` §7; `thinking/multi-device.md`;
  `thinking/field-trades-and-the-ordering-tension.md` §3–4.
- **Implementation finding (2026-06-26):** the storage-layer twin of this ordering tension was found and
  fixed in the redb projection during the PR #11 → `experiments` integration. A node card's identity fields
  (`created_at`/`created_by`) were **first-touch-wins** → ingest-order-sensitive → two peers with the same
  governance facts but different arrival orders derived different cards (an I2/I5 convergence violation,
  caught by the I3 rebuild check). Fixed by making them the **canonical `(created_at, created_by)` MIN**
  (commutative, order-independent) — the derived-layer analogue of the §7 content-hash tie-break. Writeup:
  `experiments/alpha/local_storage_projection/CONVERGENCE_FINDING.md`. Takeaway folded into the spec note
  below: every derived field must be a commutative canonical reduction of the facts, never an artifact of
  fold order ("last/first-writer-wins is a clock in disguise, and Drystone has no clock").

> **T30 added 2026-06-26** — consolidates the scattered spec-maturation work (spec App-B `ENABLING` items +
> `[confirm before publish]` flags + T1/T23/T29 residuals) into one tracked path-to-publication thread, so
> it is flagged here rather than only living inside the spec.

### Croft (Layer 6)

#### T12 — Consumer-pull economic inversion (M3) + the M0–M4 product-track sequencing

- **Layer:** croft
- **Status:** `open · gated` (settled-as-direction; under-designed).
- **Type:** `needs-content` (walk-through requested).
- **Review (2026-06-26):** the user wants a **plain-language walk-through** of this thread — they thought
  it was resolved and want to re-understand what is settled (the M0–M4 track) vs still open (M3, the
  consumer-pull/demand-side broker, which is named but not designed).
- **What it is:** the **fifth rung of the "recurring inversion"** — invert the ad model into a **consumer-side / demand-side broker** (the one economic pillar of the thesis with no home in 07 or 08). Plus the **M0–M4 product track** (M0 single-user vault → M1 secure group chat → M2 social graph you hold → M3 consumer-pull inversion → M4 the cooperative) — the staged delivery spine no theme carries.
- **Promotion target:** **07** (a third economic mechanism) and **08** (the product-track roadmap).
- **Gates:** M3 is named but not designed; the per-milestone shape needs work before it's resolved-beta.
- **Alpha provenance:** `../alpha/crystallized/conclusions.md` (M0–M4); `../alpha/crystallized/principles.md` (the five-scale inversion list).

#### T14 — iOS opportunistic-only P2P as a named product limitation

- **Layer:** croft, impl
- **Status:** `open · gated`.
- **Type:** `needs-content` (folds into the per-platform design files, S4; couples T6).
- **Review (2026-06-26):** doesn't need to stay a one-off thread — the iOS app must **play to iOS
  strengths** (event-driven from the system, push-triggered, sync) and that thinking belongs in a
  dedicated **iOS design file**, one of the per-platform set (S4). Common core where possible, but
  platforms differ; every not-yet-implemented platform needs its own version of this thinking.
- **What it is:** on iOS you cannot hold a background socket, so device-to-device P2P is **opportunistic, not deterministic**, and spontaneous off-grid meshing is aspirational/unproven — which structurally argues the meer is the dependable backbone, not a bonus. The four-property impossibility is already in 03; the **iOS-background constraint as a stated limitation on the product's connectivity promise** is not.
- **Promotion target:** **08 §9** (a peer asterisk to the "serverless"/relay-dependency one) and **03**.
- **Gates:** decide what Croft promises about off-grid/background sync (the product consequence is undecided).
- **Alpha provenance:** `../alpha/thinking/ios-opportunistic-p2p.md`. **Per-platform design file (S4,
  started 2026-06-26):** `../alpha/thinking/app/platforms/ios.md`.

#### T15 — P2P-games data layer (ephemeral-live / durable-outcome) + open attestation

- **Layer:** croft
- **Status:** `open · gated`.
- **Type:** `needs-content` (per-app PRDs, S5).
- **Review (2026-06-26):** game still called a **pond** (candidate). Wants per-app **PRDs/design docs** —
  chat gets its own, games gets its own. Plus a new modest **starter use case** worth its own design
  doc: a peer-to-peer **"thinking of you"** signal (instead of buying a connected bracelet, touch a spot
  on your phone and it reaches out / signals the other person) — small but a good first concrete use case.
- **What it is:** a settled-as-shape decision — **live play is always over iroh and always ephemeral; only the settled outcome is durable, by the players' choice** (one durable record per completed game). But the **outcome-attestation mechanism is explicitly open** and the games pond is "candidate, not committed."
- **Promotion target:** **08 §7**.
- **Gates:** the mutual-signed outcome-attestation mechanism; games-pond commitment.
- **Alpha provenance:** `../alpha/thinking/app/design-philosophy.md` (data-layer shape); `../alpha/thinking/app/ponds/` (attestation set aside). **PRD stubs (S5, started 2026-06-26):** `../alpha/thinking/app/prds/games-pond.md` + `../alpha/thinking/app/prds/thinking-of-you.md` (+ `chat.md`).

#### T17 — Three-audiences settings model + the composable-interface ramp

- **Layer:** croft
- **Status:** `open · gated`.
- **Type:** `needs-content` (lands in the per-platform design specs).
- **Review (2026-06-26):** considered **fairly settled** as a design principle that impacts the
  per-platform design specs (S4); the open part is "what does it look like" in each implementation, not
  whether to adopt it.
- **What it is:** settings serve **three audiences by relationship to the system** (never-touch / tune-a-few / full-surface), named by intent not depth, realized via a composable-interface ramp of self-authorship. Underpins 08's "composability is the user-respecting value" stance, which 08 currently asserts without the audience model beneath it. The composable-interface realization is explicitly a forward note (unproven).
- **Promotion target:** **08**.
- **Gates:** the composable-interface ramp has no proof/spec yet (a product-track concern).
- **Alpha provenance:** `../alpha/crystallized/principles.md` (three-audiences + composable-interface note).

> **T18–T20 added 2026-06-25** from the per-file alpha→beta coverage audit
> (`../alpha/plans/2026-06-25-beta-coverage-per-file-audit.md`) — the long-tail unsettled finds a
> grouped sweep missed: a settled-stance principle and two unbuilt design surfaces that beta correctly
> dropped (because unsettled/unbuilt) and that had no home in T1–T17.

### Governance (Layer 7)

#### T48 — Cooperative & governance prior-art register (ECOSYSTEM.md §8)

- **Layer:** governance, philosophy
- **Status:** `open · surfaced`
- **Type:** `needs-content` (+ `needs-research`: several rows dialogue-sourced) · couples `legal-review`
- **What it is:** the Phase-1 recovery counterpart to the cairn cohort (T41–T47), for the co-op lineage
  that was largely anonymized out of beta (ledger G11 / H9). Headline gaps: **Platform Cooperativism
  Consortium (Trebor Scholz; the 2014 "Platform Cooperativism vs. the Sharing Economy" founding essay)**
  as a *named movement*, plus the working existence-proofs (**Stocksy United, The Drivers Cooperative,
  Resonate, Social.coop, Mondragon, Green Bay Packers, the credit-union lineage**); the **fiscal-sponsor
  analysis** (**SPI/Debian** as the exact trademark-holding proof-of-concept, **SFC** as the permanent
  neutral home, **Aspiration** as the recommended interim foundation); Ostrom's commons work, Liquid
  Feedback, and the Purpose Foundation / steward-ownership seed-capital models; and the verified
  **anti-pattern failure lineage** (Ello / Ampled / Steemit / Diaspora / Coomappa). Beta argues the co-op
  form is *necessary* and even flags this literature unsourced in `peer-standing-and-the-cooperative-form.md`,
  yet omits the movement name and the proofs it is *possible*.
- **Promotion target:** extend `governance/foundation-cooperative-and-sustainability.md` and a
  `governance/reference-index.md` (Phase-2 C9); the Platform-Cooperativism movement also → `philosophy/`
  peer-standing.
- **Gates:** several rows dialogue-sourced — verify before reliance; the MO Chapter 351 / cooperative
  legal-review gate remains the user's (NOT-LEGAL-ADVICE — carry the reasoning, not the citations).
- **Alpha provenance:** `../alpha/ECOSYSTEM.md` §8; `../alpha/thinking/cooperative-social-union-model.md`,
  `foundation-and-ip-stewardship.md`.

#### T33 — Edge-preserving capital formation (funding the co-op without reinstalling the extractive edge)

- **Layer:** governance, philosophy
- **Status:** `open · surfaced` (surfaced 2026-07-06 by the peer-standing argument).
- **Type:** `needs-research` (couples-with `legal-review`).
- **What it is:** the one genuinely open engineering problem the peer-standing → cooperative-form
  argument *generates* (not a flaw in it). A social-graph platform has real infrastructure costs
  historically funded by exactly the securitized venture capital the argument rules out. Whether a
  cooperative can raise sufficient capital **without issuing any claim** (non-voting preferred, outside
  investor classes) that smuggles the asymmetric owner/user edge back in is contested and unresolved. If
  edge-dissolution is the point, the funding instrument must itself be edge-preserving — which sharply
  constrains the financing design space. Couples with the two grounding gaps the argument flags: the
  Rochdale / ICA cooperative legal mechanics (non-transferable member shares, patronage returns) and the
  platform-cooperativism capital-formation literature.
- **Promotion target:** `governance/` (Layer 6, the manifestation) — extends
  `philosophy/peer-standing-and-the-cooperative-form.md` §6 and theme `07`; sits under the bannered
  cooperative legal-review gate in `README.md`.
- **Gates:** ground the platform-cooperativism capital-formation literature; ground the ICA/Rochdale
  member-share mechanics; the broader cooperative/foundation legal-review gate (README) is the call this
  sits under.
- **Provenance:** `philosophy/peer-standing-and-the-cooperative-form.md` §6 + §8 (the `[tension]` on
  capital formation) and `philosophy/structural-argument-principles.md` §IX(32); assembled from conversation,
  delivered 2026-07-06 (via `dropoff/third.zip`, since removed — the committed governance docs are the record).

### Socialization (Layer 8)

#### T4 — Brand / voice / messaging (chapter landed; brand-direction decision open)

- **Layer:** socialization
- **Status:** `open · partially resolved 2026-07-07`. The brand/voice chapter landed as
  `../socialization/brand-and-voice.md` (it folded the reservoir plus the app-side notes). What stays open
  is only the **brand-name / direction DRIFT decision** and the clearance/verification gates below.
- **Type:** `needs-content` (structural directive S3; twinned with T11).
- **Review (2026-06-26):** **start the doc now** — a brand/voice/messaging working folder that accretes
  taglines, ideas, links, and "ammo" over time (twin to the adoption-enablement doc, T11), so nothing is
  forgotten and we can say "here's what we've been looking at" when it's time. (Brand-name DRIFT gate
  still applies before any of it hardens into a chapter.)
- **What it is:** `narrative/messaging-and-quotes.md` is a mature, provenance-tagged (OURS / CITE /
  CLEARANCE / UNVERIFIED) reservoir — taglines, the corporation-vs-person crowding-out framing (Gneezy &
  Rustichini, Ostrom, Ariely), the digital-living-room / IYKYK positioning, the Euphoria tie-in with a
  fair-use/trademark analysis. Now consolidated into `../socialization/brand-and-voice.md`; the open work
  is the brand-direction decision, not the chapter.
- **Promotion target:** landed at `../socialization/brand-and-voice.md`; this thread now tracks only the
  residual brand-direction DRIFT decision and the clearances.
- **Gates:** brand/product-name **DRIFT reconciled vs `NAMING.md`** (the 08/07 dependency — 08 says "must
  be reconciled before any brand chapter"); CLEARANCE items (Euphoria line) cleared with counsel;
  `[UNVERIFIED]` anecdotes confirmed or dropped.
- **Alpha provenance:** `../alpha/narrative/messaging-and-quotes.md`; the **app-side half of the same brand
  DRIFT** — `../alpha/thinking/app/brand-and-voice-notes.md` (taglines, two-speed answer, "Grow your own",
  message funnel) and `../alpha/assets/README.md` (draft wordmarks, license-gated) [added 2026-06-25
  per-file audit]; `../alpha/BETA-ROLLUP.md` coverage view ("likely feeds a future brand chapter");
  `../alpha/ROADMAP_TODO.md` C6 / A7. **Accreting home (S3, started 2026-06-26):**
  `../alpha/narrative/brand-comms-workbook.md`.

#### T11 — Adoption-chasm thesis + the institutional-mandate "fourth bridge"

- **Layer:** socialization, fenced
- **Status:** `open · partially resolved 2026-07-07`. The adoption strategy landed as
  `../socialization/adoption-strategy.md`. What stays open is the survey's **primary-source verification**
  and the undone **design-for-institutional-mandate** directive.
- **Type:** `needs-content` (structural directive S3; twin doc to T4).
- **Review (2026-06-26):** **start the doc now** — adoption-enablement is the **twin** of the brand/voice
  doc (T4); both accrete thinking, references, and "ammo" over time so we don't forget and can pull the
  thread when the moment comes.
- **What it is:** a survey of ~16 P2P/local-first projects concluding **only Signal crossed the chasm**, and that crossing needs three conditions — product parity, a non-extractive sustaining org, an inciting event (which produces *spikes*, not sustained migration). Plus a discovered **fourth bridge: institutional mandate** (Matrix's 25+ government adoptions were top-down) "worth designing for explicitly," and the **embedded-trust** corollary (P2P tools must embed in *existing* trust networks, not expect trust to form around the tool).
- **Promotion target:** landed at `../socialization/adoption-strategy.md`; the descriptive why-incumbents-win map is at `../fenced/platform-dominance-and-adoption.md`. This thread now tracks the survey verification and the institutional-mandate design directive.
- **Gates:** the survey carries a `[needs primary-source verification]` caveat (confirm before asserting); "design for institutional mandate" is an undone directive.
- **Alpha provenance:** `../alpha/research/p2p-founder-motivations-adoption.md` (RQ2 synthesis); `../alpha/SOVEREIGN-COMMONS-DOSSIER.md` §7; `../alpha/narrative/long-form.md` (adoption-curve risk, named not analyzed). **Accreting home (S3, started 2026-06-26):** `../alpha/narrative/adoption-enablement.md`.

## How to use this file

When a beta theme doc is tempted to assert something that is actually still in flight, park it here
instead with its gates named, and reference it. When the gates clear, promote it per the rule above and
record the trace in `../alpha/LAYER-ROLLUP.md`. On promotion (or closure), move the thread block to `CLOSED-THREADS.md` — keep the open list short and scannable (2026-06-26 review).
