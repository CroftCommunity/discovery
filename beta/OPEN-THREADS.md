# discovery / beta — open threads (the staging queue at the beta gate)

date: 2026-06-25

## What this is

A holding ledger for threads that are **being pulled toward beta but are not yet settled enough to
become resolved beta narrative**. It exists so a live need is never lost, while the eight resolved
theme docs (`01`–`08`) stay a clean, settled synthesis. A thread lives here — referenced, not asserted
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

### T1 — Drystone governance & peer model (§2 peers/rights/capabilities + §X governance conflicts)

- **Status:** `gated`.
- **What it is:** the governance-layer design distilled 2026-06-24 — one kind of peer; **rights**
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
- **Promotion target:** completes the federation handoff that **01 §6** opens and **07 B5** gives a legal
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
- **Alpha provenance:** `../alpha/narrative/messaging-and-quotes.md`; `../alpha/BETA-ROLLUP.md`
  coverage view ("likely feeds a future brand chapter"); `../alpha/ROADMAP_TODO.md` C6 / A7.

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
  problem; 01 §5 states the *principle* (irreversible → constitutional rigidity bites hardest) but never
  names the *mechanism*.
- **Promotion target:** **04 / 06** (the governance log + revocation ladder); **01** (the
  protection-rigidity principle). **Likely co-promotes with T1.**
- **Gates:** define the reversible-vs-committing decision tag; spec the permanent attribution record;
  reconcile with T1's append-only fold.
- **Alpha provenance:** `../alpha/ROADMAP_TODO.md` **D10**; `../alpha/COHESION.md` **§22**; `beta/01` §5.

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
- **Alpha provenance:** `beta/01` §2.4; `beta/02` §1/§4/§5. (Pass-2 fact-check left Ostrom as the one
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

---

## How to use this file

When a beta theme doc is tempted to assert something that is actually still in flight, park it here
instead with its gates named, and reference it. When the gates clear, promote it per the rule above and
record the trace in `../alpha/BETA-ROLLUP.md`. Keep this list short — a thread that has been
`ready-to-promote` for a while should be promoted, not parked.
