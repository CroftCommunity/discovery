# discovery / beta — open threads (the staging queue at the beta gate)

date: 2026-06-25

## What this is

A holding ledger for threads that are **being pulled toward beta but are not yet settled enough to
become resolved beta narrative**. It exists so a live need is never lost, while the resolved theme docs
(`02`–`08`) and the Drystone protocol spec (`drystone-spec/`) stay a clean, settled synthesis. A thread
lives here — referenced, not asserted
— until its gates clear; only then does it graduate into a theme doc (and earn a row in
`../alpha/BETA-ROLLUP.md`).

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

- **Status** — `surfaced` (logged, gates not yet worked) · `gated` (blocked on named decisions/work) ·
  `ready-to-promote` (gates clear; next pass folds it into a theme).
- **What it is** — one or two lines.
- **Promotion target** — which beta theme(s)/section it would land in (or a proposed new theme).
- **Gates — must settle before it becomes resolved beta narrative** — the explicit blockers
  (decisions, `ENABLING` spec work, fact-confirmation).
- **Alpha provenance** — where the material lives now.

## Promotion rule

A thread leaves this file and enters a theme doc **only when its gates are clear**. On promotion:
write the settled synthesis into the theme doc (quotes whole, verification flags inline), add the
`../alpha/BETA-ROLLUP.md` trace row, and strike the thread here with a one-line "promoted → NN §X
(date)" note. Until then, beta theme docs may **not** assert the thread's content as resolved.

---

## Open threads

### T1 — Drystone governance & peer model (§2 peers/rights/capabilities + §X governance conflicts) — PROMOTED → drystone-spec

- **Status:** **PROMOTED 2026-06-26 → `drystone-spec`** (built this session; residual gates carried into the
  spec, see below). The 2026-06-25 "design we have NOT built / `P-*` unwritten" framing below is **superseded**.
- **Resolution:** the §2/§X drafts were matured into the **beta Drystone protocol spec** — the `P-*`
  principles are now **defined** in `drystone-spec` Part 1 §2 (`P-Local-Truth`, `P-Knowable-Truth`,
  `P-Peer-Equality`, `P-Durable-Enablement`); §2 peers/rights/capabilities/PeerSets/meer/exitability →
  Part 2 §5; §X governance log / timestamp-free order / R1–R6 / attributable acceptance / regress-free fold
  → Part 2 §7. **Residual gates (now carried in the spec, not here):** the `ENABLING` wire formats
  (canonical fact encoding, frontier-closure, frontier-commitment, capability wire format) → spec Part 2
  **Appendix B**; **Track A (Meadowcap) vs Track B (Keyhive)** → spec **Appendix A** + couples T24;
  **key-custody default (A12)** and **geer-name (A13)** → ROADMAP_TODO; the Matrix/Willow/Meadowcap/Keyhive
  facts carry **[confirm before publish]** in the spec. Optional curation: whether to back-port the named
  `P-*` principles into alpha `crystallized/principles.md` (they are canonical in the spec) — a curation
  call, low-priority.
- **What it was (pre-spec staging):** the governance-layer design distilled 2026-06-24 — one kind of peer;
  **rights**
  (universal, never delegated) vs **capabilities** (additive, delegated, revocable); the
  **capability / role / PeerSet** layers (the meer recast as a PeerSet, satisfying read-your-own-history
  vacuously); the **exitability** backstop + **asymmetry-of-expressible-range** framing; revocation as an
  epoch-rotating expulsion-shaped fact; and the §X conflict-resolution model (**append-only fold →
  no-state-reset**, a **timestamp-free causal order**, an unconflictable capped root, the R1–R6
  capability interface, attributable-acceptance, and the regress-breaking termination construction). Its
  spine is a deep **Matrix close-cousin contrast** (where Matrix's choices cost a CVE + a multi-week
  outage; Croft's eventual-consistency bet dodges that class).
- **Promotion target:** primarily **04 (the protocol we proved)** and **06 (safety without
  surveillance)**; possibly a dedicated governance theme if it grows. The new principles
  (`P-Local-Truth`, `P-Peer-Equality`, `P-Knowable-Truth`, **`P-Durable-Enablement`**, the
  **peer-capability-floor**) would land in **01 / the principle set** once written.
- **Gates — must settle before resolved-beta:**
  1. **Status is DRAFT / ENABLING.** The wire formats are unspecified; the hardest piece
     (**frontier-closure before sort**, §X.8.5) is open. Beta 04 is about what we *proved*; this is
     design we have *not built*.
  2. **Two core mechanisms undecided** — capability mechanism **Track A (Meadowcap) vs Track B
     (Keyhive)** (`ROADMAP_TODO` **A11**); key-custody default **blind-relay vs trusted-delegate**
     (**A12**, incl. the "does Option-B-as-default rebuild a readable homeserver?" question). The "geer"
     name is also open (**A13**).
  3. **Cited facts not yet SoT-confirmed.** The Matrix / Willow / Meadowcap / Keyhive claims were
     web-verified *in the source dialogue only* — confirm (CVE-2025-49090; room v12 / MSC4289;
     Megolm/UTD; Seshat desktop-only search; Meadowcap "no native revocation"; Willow unenforceable
     timestamp + `is_authorised_write`; matrix.org Postgres postmortem; Karlsruhe SACMAT 2020; Element X
     vs Classic) against a source of truth before any of it hardens into beta. **Do not** re-introduce
     the dialogue's self-corrected false claim ("Matrix E2EE is bilaterally disable-able" — it is a
     one-way latch).
  4. **The `P-*` principles do not yet exist** in `../alpha/crystallized/principles.md` by these names;
     §2/§X `Realizes` them but they are unwritten.
- **Alpha provenance:** `../alpha/thinking/drystone-spec/section-2-peers-rights-capabilities.md`,
  `…/section-x-governance-conflicts.md`, README; raw dialogue
  `../alpha/seeds/transcripts/raw/drystone-peers-rights-governance-matrix-dialogue-2026-06-24.md`.
  Backlog: `../alpha/ROADMAP_TODO.md` **E30** (+ A11/A12/A13); seam: `../alpha/COHESION.md` **§37**.

> **T2–T10 added 2026-06-25** from a sweep of the alpha backlog (`ROADMAP_TODO` A/D/E), the seam/edge
> trackers (`COHESION` OPEN/DRIFT, `open-edges.md`, `open-considerations.md`), the beta themes'
> "establishes / does not" boundaries, and the rollup coverage view — the under-staged, beta-bound
> threads that the eight resolved themes correctly dropped (because unsettled) and that had no home
> before this ledger existed. Ranked by alpha→beta maturation impact. Already-bannered gates (recovery
> anchor, MPL, cooperative legal review, Noria, CroftC IP, genome-vs-strategy, V3 republish-UX,
> cold-start, brand-name DRIFT) are deliberately **not** duplicated here — they are already visible in
> their themes.

### T2 — Governance at scale (subsidiarity + liquid delegation; the concentration default)

- **Status:** `gated`.
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

### T3 — Moderation & abuse under a blind broker (the constructive design body)

- **Status:** `gated`.
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

### T4 — A brand / voice / messaging chapter (a missing theme)

- **Status:** `gated`.
- **What it is:** `narrative/messaging-and-quotes.md` is a mature, provenance-tagged (OURS / CITE /
  CLEARANCE / UNVERIFIED) reservoir — taglines, the corporation-vs-person crowding-out framing (Gneezy &
  Rustichini, Ostrom, Ariely), the digital-living-room / IYKYK positioning, the Euphoria tie-in with a
  fair-use/trademark analysis. A chapter's worth of brand voice that no beta theme absorbs.
- **Promotion target:** a **new brand/voice theme** (none of the eight is one); the rollup itself
  anticipates it.
- **Gates:** brand/product-name **DRIFT reconciled vs `NAMING.md`** (the 08/07 dependency — 08 says "must
  be reconciled before any brand chapter"); CLEARANCE items (Euphoria line) cleared with counsel;
  `[UNVERIFIED]` anecdotes confirmed or dropped.
- **Alpha provenance:** `../alpha/narrative/messaging-and-quotes.md`; the **app-side half of the same brand
  DRIFT** — `../alpha/thinking/app/brand-and-voice-notes.md` (taglines, two-speed answer, "Grow your own",
  message funnel) and `../alpha/assets/README.md` (draft wordmarks, license-gated) [added 2026-06-25
  per-file audit]; `../alpha/BETA-ROLLUP.md` coverage view ("likely feeds a future brand chapter");
  `../alpha/ROADMAP_TODO.md` C6 / A7.

### T5 — Protocol behavior at scale / group-chat failure modes

- **Status:** `gated`.
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

### T6 — The per-platform trust-model doc (05's "highest-leverage next artifact")

- **Status:** `gated`.
- **What it is:** the per-network (Bluesky/AP/Mastodon/GoToSocial/Threads/Hive) write-up — the field used,
  what Croft claims / doesn't claim, the backlink mechanism, exact verifier steps + pseudocode. 05 *names*
  it as the highest-leverage next artifact but cannot assert its content because it does not exist.
- **Promotion target:** **05** (completes the identity theme).
- **Gates:** write it; confirm `alsoKnownAs` extra-entry persistence (`[UNVERIFIED]`, E14); resolve the
  anchor-URI stability contract (A9) and the PDS-vs-self-controlled rotation key (A10), which determine
  what each spoke can claim; depends partly on T7.
- **Alpha provenance:** `../alpha/ROADMAP_TODO.md` **E13** (+ A9/A10/E14);
  `../alpha/thinking/cross-platform-identity-provenance.md:222`; `beta/05` boundary.

### T7 — atproto Permissioned/Private-Data watch-item (external dependency, gates 03 + 05)

- **Status:** `gated` (gate is external, not Croft-internal work).
- **What it is:** 03 calls atproto's Permissioned Data work "**the single most important external
  development to track** — it could narrow or complement Croft's private path." The real ATProto Private
  Data WG defers true E2EE / zero-knowledge; Croft sits on the harder ZK side. Couples to 05's `did:webvh`
  native-support `[UNVERIFIED]` gate.
- **Promotion target:** updates **03** (the field positioning) and **05** (preferred-DID-method choice)
  when it lands.
- **Gates:** the atproto WG reaches a settled E2EE/ZK posture; `did:webvh` native atproto support
  confirmed against the FACTCHECK SoT.
- **Alpha provenance:** `beta/03` §6; `beta/05` §3; FACTCHECK as SoT for the confirm.

### T8 — Forward-only revocation under irreversible commitments

- **Status:** `gated`.
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

### T9 — Publication-readiness verification pass (01 Ostrom + 02 Clearances colour quotes)

- **Status:** `gated`.
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

### T10 — Real-time media-layer hardening (finishes 04's media leg)

- **Status:** `gated` (largely de-risked — a "close the last decisions" thread).
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

### T11 — Adoption-chasm thesis + the institutional-mandate "fourth bridge"

- **Status:** `gated` (provenance-gated finding + an undone design directive).
- **What it is:** a survey of ~16 P2P/local-first projects concluding **only Signal crossed the chasm**, and that crossing needs three conditions — product parity, a non-extractive sustaining org, an inciting event (which produces *spikes*, not sustained migration). Plus a discovered **fourth bridge: institutional mandate** (Matrix's 25+ government adoptions were top-down) "worth designing for explicitly," and the **embedded-trust** corollary (P2P tools must embed in *existing* trust networks, not expect trust to form around the tool).
- **Promotion target:** **07** (the institutional-adoption path as a sustainability lever) and **03** (close the field map with the "only Signal crossed, and why" verdict).
- **Gates:** the survey carries a `[needs primary-source verification]` caveat (confirm before asserting); "design for institutional mandate" is an undone directive.
- **Alpha provenance:** `../alpha/research/p2p-founder-motivations-adoption.md` (RQ2 synthesis); `../alpha/SOVEREIGN-COMMONS-DOSSIER.md` §7; `../alpha/narrative/long-form.md` (adoption-curve risk, named not analyzed).

### T12 — Consumer-pull economic inversion (M3) + the M0–M4 product-track sequencing

- **Status:** `gated` (settled-as-direction; under-designed).
- **What it is:** the **fifth rung of the "recurring inversion"** — invert the ad model into a **consumer-side / demand-side broker** (the one economic pillar of the thesis with no home in 07 or 08). Plus the **M0–M4 product track** (M0 single-user vault → M1 secure group chat → M2 social graph you hold → M3 consumer-pull inversion → M4 the cooperative) — the staged delivery spine no theme carries.
- **Promotion target:** **07** (a third economic mechanism) and **08** (the product-track roadmap).
- **Gates:** M3 is named but not designed; the per-milestone shape needs work before it's resolved-beta.
- **Alpha provenance:** `../alpha/crystallized/conclusions.md` (M0–M4); `../alpha/crystallized/principles.md` (the five-scale inversion list).

### T13 — Encrypt-then-content-address kills cross-user dedup (media storage economics)

- **Status:** `gated`.
- **What it is:** same media + different nonces ⇒ different ciphertext hashes ⇒ **no cross-user dedup**; for media-heavy use this breaks the storage math the survivability fund was costed on. A genuine seam between the media layer and the funding model (distinct from T10's media *transport* hardening).
- **Promotion target:** the **04/08 (media) ↔ 07 (survivability-fund costing)** seam.
- **Gates:** decide the storage/dedup posture and re-cost the fund accordingly.
- **Alpha provenance:** `../alpha/thinking/open-considerations.md` (the dedup item); `../alpha/experiments/encrypted-blob-share/`.

### T14 — iOS opportunistic-only P2P as a named product limitation

- **Status:** `gated`.
- **What it is:** on iOS you cannot hold a background socket, so device-to-device P2P is **opportunistic, not deterministic**, and spontaneous off-grid meshing is aspirational/unproven — which structurally argues the meer is the dependable backbone, not a bonus. The four-property impossibility is already in 03; the **iOS-background constraint as a stated limitation on the product's connectivity promise** is not.
- **Promotion target:** **08 §9** (a peer asterisk to the "serverless"/relay-dependency one) and **03**.
- **Gates:** decide what Croft promises about off-grid/background sync (the product consequence is undecided).
- **Alpha provenance:** `../alpha/thinking/ios-opportunistic-p2p.md`.

### T15 — P2P-games data layer (ephemeral-live / durable-outcome) + open attestation

- **Status:** `gated`.
- **What it is:** a settled-as-shape decision — **live play is always over iroh and always ephemeral; only the settled outcome is durable, by the players' choice** (one durable record per completed game). But the **outcome-attestation mechanism is explicitly open** and the games pond is "candidate, not committed."
- **Promotion target:** **08 §7**.
- **Gates:** the mutual-signed outcome-attestation mechanism; games-pond commitment.
- **Alpha provenance:** `../alpha/thinking/app/design-philosophy.md` (data-layer shape); `../alpha/thinking/app/ponds/` (attestation set aside).

### T16 — Matrix close-cousin E2EE operational lessons (UTD invariant, mandatory-recovery onboarding, expectation-gap)

- **Status:** `gated` (lessons settled; design responses unbuilt).
- **What it is:** Matrix's production-paid E2EE lessons as direct design commitments — treat "every current member can decrypt every current-epoch message" as a **continuously-tested invariant** with a friendly key-request/healing path (never a dead "Unable to decrypt" tile); make **recovery setup near-mandatory in onboarding** with a blocking warning before any single-device-no-recovery state; and the **expectation-gap list** (instant full-history search, link previews, read receipts, "all my history on a new phone") Croft makes Hard — "planned, not discovered late." Beyond T5 (scale) and T1 (governance).
- **Promotion target:** **04** (the decrypt-invariant + healing path) and **08** (the onboarding-flow + expectation-setting UX). The recovery-onboarding *flow* is distinct from the bannered recovery-*anchor* gate.
- **Gates:** decide the invariant/healing mechanism and the onboarding gate UX.
- **Alpha provenance:** `../alpha/research/discord-matrix-groupchat.md` (Matrix lessons + expectation gaps).

### T17 — Three-audiences settings model + the composable-interface ramp

- **Status:** `gated`.
- **What it is:** settings serve **three audiences by relationship to the system** (never-touch / tune-a-few / full-surface), named by intent not depth, realized via a composable-interface ramp of self-authorship. Underpins 08's "composability is the user-respecting value" stance, which 08 currently asserts without the audience model beneath it. The composable-interface realization is explicitly a forward note (unproven).
- **Promotion target:** **08**.
- **Gates:** the composable-interface ramp has no proof/spec yet (a product-track concern).
- **Alpha provenance:** `../alpha/crystallized/principles.md` (three-audiences + composable-interface note).

> **T18–T20 added 2026-06-25** from the per-file alpha→beta coverage audit
> (`../alpha/plans/2026-06-25-beta-coverage-per-file-audit.md`) — the long-tail unsettled finds a
> grouped sweep missed: a settled-stance principle and two unbuilt design surfaces that beta correctly
> dropped (because unsettled/unbuilt) and that had no home in T1–T17.

### T18 — LTS-for-interfaces / shapeability-paired-with-stability

- **Status:** `surfaced`.
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

### T19 — Blind-peer encrypted-search / coverage-attestation substrate

- **Status:** `gated`.
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

### T20 — Conflict-reason corpus gaps (C4 / C7 / C8 / C9 / C10)

- **Status:** `gated`.
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

### T21 — Is `share` fully a right, or partly a membership-class capability?

- **Status:** `gated`.
- **What it is:** of the four rights named in `drystone-spec` (Part 2 §5.3 — tenure / exit / voice / share),
  **`share`** — a claim on the collective's commons — is the least-settled. If `share` can be legitimately
  diluted by governance or membership class (a real possibility under the cooperative model), then part of
  it behaves like a *capability*, not a right. The boundary "no right to remove the rights of others" needs
  to know **which portion of `share` is the inviolable floor** and which portion is a class-varying
  entitlement.
- **Promotion target:** `drystone-spec` Part 2 §5.3 (sharpen the `share` definition; it is already flagged
  open there and in Part 2 Appendix B) + **07** (the cooperative membership / patronage model decides the
  dilutable portion).
- **Gates:** decide, in the cooperative model, the inviolable-floor vs class-varying split of `share`; then
  the four-rights closed set can harden into the spec.
- **Alpha provenance:** `../alpha/thinking/rights-vs-capabilities-definitions.md` (the two open checks);
  `../alpha/ROADMAP_TODO.md` **E32 (b)**; `drystone-spec` Part 2 §5.3; `beta/07` Pillar A.

### T22 — Does the `04` survivor re-key strand a peer's `tenure`?

- **Status:** `gated`.
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

> **T23 added 2026-06-26** from the beta-01 → Drystone-spec build (Part 1 §3, the cross-field grounding).

### T23 — Verbatim grounding quotes for the systems-science section (Ashby gloss, Beer paraphrase) — CLOSED

- **Status:** **CLOSED 2026-06-26** (source supplied + incorporated). One verification flag carries
  forward (below).
- **What it was:** the systems-science grounding carried two non-verbatim blocks — an Ashby
  survival-condition **gloss** and a **Beer** paraphrase. The spec build dropped both as quotations
  (kept Ashby's real line "Only variety can destroy variety," *Intro to Cybernetics* p. 207) and held the
  Beer grounding as prose pending a real source.
- **Resolution:** the user supplied the Beer source (the Beer / algedonic / Cybersyn / OGAS dialogue,
  `../alpha/seeds/transcripts/raw/beer-algedonic-cybersyn-ogas-dialogue-2026-06-25.md`). Incorporated into
  **Drystone spec Part 1 §3** with two defensible-verbatim Beer quotes ("ride the dynamics," *Brain of the
  Firm*; "their only hope," *Designing Freedom* 1974), the plain-language **algedonic** explanation, and
  the **Cybersyn/OGAS** natural experiment. The "aids to human viability…" phrasing is confirmed a
  synthesis gloss (not attributed to Beer). The richer thread — the **adjudication-locus axis**,
  **peerhood-as-adjudication**, and **exit-backed authority** — landed in Part 2 §3/§5.2/§8/App-B and the
  alpha synthesis `../alpha/thinking/algedonic-and-peerhood-as-adjudication.md`.
- **Carried flag:** the two Beer quotes and the Cybersyn/OGAS dates/figures are web-verified in the
  dialogue only and **[confirm before publish]** against primary editions before a publication-final
  release. (Tracked in the spec's Part 2 Appendix B external-fact-confirmation item.)

> **T24 added 2026-06-26** from the Beer/OGAS intake — the unsettled design question that fell out of
> peerhood-as-adjudication. (Distinct from the now-closed T23, which was just the verbatim-Beer sourcing.)

### T24 — What grounds a peer's authority, and what makes a right cost something to violate?

- **Status:** `gated`.
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
> *local-implementation* build (not protocol); T26 is an *app/product* reframe (theme 08). The protocol-level
> conclusions already landed in the Drystone spec (Part 1 §2.0/§2.3, Part 2 §4.5.1/§7.3.3).

### T25 — The Drystone redb storage-and-projection layer (vetted, adaptable local component)

- **Status:** **in progress (being built externally, 2026-06-26)** — the build spec
  (`../alpha/seeds/generated-prompts/redb-social-graph-layer-build-prompt.md`) is being implemented in a
  separate build environment by the user. Was `surfaced`.
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

### T26 — Social-graph-as-substrate: the product reframe (chat as a tenant) — PROMOTED → 08

- **Status:** **PROMOTED 2026-06-26 → `08`** (user-approved "yes we should reframe this way"). The reframe
  is folded into `08`: theme narrative + charter re-anchored on the social graph as substrate; new **§1**
  (the social graph is the substrate; the garden grows from it) with **§1.1** (the durable group made
  invisible — group≠member-set, implicit/sticky/pruned lifecycle, the group's-face UX, local-vs-shared-anchor)
  and **§1.2** (ponds/pads as a group's siblings); §4 re-pointed (`group-core` is the substrate core);
  establishes-rewritten. **Residual gates kept open** (below) — the group's-face UX iteration (on the T25
  local framework) and reconciling the sticky-group lifecycle with `06`/membership-vs-access remain design
  work, but the *shape* is now settled in 08.
- **What it was (the reframe, now in 08 §1):** invert the app pyramid — the **social graph is the substrate;
  chat is one tenant**, peer to games/calls/photos hung off a durable **group** (the group is the index; a
  chat can end while the group persists). **group identity ≠ member set**; **implicit/sticky group
  lifecycle**; the **local-projection vs shared-anchor** seam; the **load-bearing-but-invisible graph** UX.
  Dissolves the Delta-Chat "games pollute a thread / chats couple membership+governance" pain.
- **What it is:** invert the app pyramid — the **social graph is the substrate; chat is one tenant**, peer
  to games/calls/photos hung off a durable **group** (the group is the index; a chat can end while the group
  persists; spin up a fresh chat with the same group, attachments intact). With it: **group identity ≠ member
  set** (stable ID + locally-overridable presentation name); **implicit/sticky group lifecycle** (sticky =
  matchable for reconciliation · live-non-sticky · pruned-never-resurrected; reconcile-vs-fresh-vs-prune is a
  per-formation human choice); the **local-projection vs shared-anchor** seam (naming/stickiness local;
  membership/cross-participant-identity/new-attachments need a shared anchor); and the **load-bearing-but-
  invisible graph** UX (the group's "home/face," many-doors-one-room) — the hardest UX problem. Dissolves the
  Delta-Chat "games pollute a thread / un-pinnable / chats live forever and couple membership+governance" pain.
- **Promotion target:** **`08` (Croft the product)** — a significant reframe of its app shape; surfaced, not a
  unilateral rewrite. The substrate claim is core Drystone (in the spec); the *product surfacing* is 08.
- **Gates:** the user's direction on restructuring `08` around the substrate model; the group's-face UX
  (testable/iterative, built on the T25 local framework that "guides rather than binds"); reconcile the
  sticky-group lifecycle with `06`/membership-vs-access.
- **Alpha provenance:** `../alpha/thinking/social-graph-as-substrate.md` §1–3; raw
  `../alpha/seeds/transcripts/raw/social-graph-substrate-redb-storage-dialogue-2026-06-26.md`.

> **T27 added 2026-06-26** — extracted from an inline prior-tier pointer that had been sitting in `beta/05`
> (a `crystallized/principles.md` "flagged for insertion" note). It was cleaned out of the beta doc for tier
> discipline and promoted to a tracked decision here. Logged to correct a pattern of leaving real decisions
> as inline notes rather than surfacing them.

### T27 — Promote "evidentiary, not operational" to a canonical principle?

- **Status:** `gated` (a user curation decision).
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

### T28 — Maturity home for the historical peer-rights material (Hush-A-Phone lineage)

- **Status:** `surfaced` (a placement decision).
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

### T29 — MLS group state ↔ governance-log / Automerge state consistency

- **Status:** `gated` (an open design binding, spec-relevant).
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
- **Alpha provenance:** `research/messaging-solutions-landscape.md` §top-unresolved #3;
  `thinking/social-graph-as-substrate.md` §7; `thinking/multi-device.md`.

> **T30 added 2026-06-26** — consolidates the scattered spec-maturation work (spec App-B `ENABLING` items +
> `[confirm before publish]` flags + T1/T23/T29 residuals) into one tracked path-to-publication thread, so
> it is flagged here rather than only living inside the spec.

### T30 — Mature the Drystone spec to publication-final (the path to the defensive-publication DOI)

- **Status:** `gated` (spec is beta-maturity; publication-final is the next stage up).
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
  is concrete spec-writing + a fact-confirmation sweep. **NOT-LEGAL-ADVICE:** attorney review of the
  patent-non-assertion paragraph still advised (07 C3).
- **Provenance:** `drystone-spec` Part 2 Appendix A/B; `thinking/drystone-publication-and-defensive-disclosure.md`;
  couples T1 (PROMOTED), T22 (tenure/re-key), T24 (Track A/B), T29 (MLS↔log binding).

## How to use this file

When a beta theme doc is tempted to assert something that is actually still in flight, park it here
instead with its gates named, and reference it. When the gates clear, promote it per the rule above and
record the trace in `../alpha/BETA-ROLLUP.md`. Keep this list short — a thread that has been
`ready-to-promote` for a while should be promoted, not parked.
