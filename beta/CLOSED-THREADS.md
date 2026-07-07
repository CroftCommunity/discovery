# discovery / beta: closed and promoted threads (archive)

date: 2026-07-07

The archive of threads that have left the live queue in `OPEN-THREADS.md`: `promoted` (integrated into a
matured layer doc) or `closed` (resolved without promotion). Kept for provenance and back-reference, other
threads still cite these T-numbers. `OPEN-THREADS.md` is the current open-only list; the theme->layer map
and the entry schema live there. Do not delete a thread; move its block here when it closes or promotes.

The theme numbers (`0N`) some older entries reference resolve to layers per `../alpha/LAYER-ROLLUP.md`.

---

### T37 - Placement and numbering of the large-group-scaling section (it collides with Part 2 §7)

- **Status:** `promoted · resolved 2026-07-07`. The user chose to fold it into Part 2 as **§11**. Done at spec document-pass-10: the section moved into `../drystone-spec/part-2-certifiable-design.md` as §11 (after §10, before Appendix A), its internal §7.N renumbered to §11.N, external real-§7 refs preserved, the §0 Map given a §11 line, and the standalone `large-group-scaling.md` removed (byte-identical original frozen in the batch-11 seed zip). Downstream refs updated to §11.
- **Type:** `structure` (a spec-organization decision, not a proving or experimentation gate).
- **What it is:** the batch-11 spec deliverable (`drystone-spec/large-group-scaling.md`) is titled "Part 2
  §7, Large-Group Scaling, Dormancy, and Re-entry" and numbers itself §7.1–§7.14, but Part 2 already has a
  §7 (Synchronization and governance-conflict resolution) that this section **depends on and cites** (its
  "Part 2 §7.3.1 / §7.3.2 / §7.3.5 / §7.3.6 / §7.4.2 / §7.6.4 / §7.6.7 / §7.8 / §7.9 / §7.9.2 / §7.9.3"
  references all resolve against the existing §7). Both cannot be §7. It was filed byte-identical as a
  standalone section companion (keeping its authored numbering, nothing overwritten, nothing renumbered
  unilaterally) rather than merged, precisely so the numbering choice stays the user's.
- **The decision to make:** (a) fold it into `part-2-certifiable-design.md` as a new later section (Part 2
  currently ends at §10 + Appendices A–D, so §11 is the natural slot), renumbering its internal §7.x to
  §11.x and leaving the "Part 2 §7.x" external citations as-is; or (b) keep it as a standalone module and
  give it a non-colliding module name/number; or (c) some other placement. Option (a) is the larger edit
  (every internal §7.N → §11.N) but yields one coherent Part 2; option (b) is zero-edit but leaves a
  section titled "§7" alongside the real §7.
- **Promotion target:** whichever placement is chosen; on resolution the `drystone-spec/README.md`
  numbering caveat and the `CHANGELOG.md` document-pass-9 note are updated to match.
- **Gates:** the user's placement call. No proving or experimentation needed.
- **Provenance:** filed from `../alpha/seeds/large-group-scaling-batch11/eleven.zip`
  (`drystone-large-group-scaling.md`); document-pass-9 (2026-07-07).

### T1 — Drystone governance & peer model (§2 peers/rights/capabilities + §X governance conflicts) — PROMOTED → drystone-spec

- **Status:** `promoted` **2026-06-26 → `drystone-spec`** (built this session; residual gates carried into the
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

### T26 — Social-graph-as-substrate: the product reframe (chat as a tenant) — PROMOTED → 08

- **Status:** `promoted` **2026-06-26 → `08`** (user-approved "yes we should reframe this way"). The reframe
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
- **Gates (residual, kept open):** the group's-face UX
  (testable/iterative, built on the T25 local framework that "guides rather than binds"); reconcile the
  sticky-group lifecycle with `06`/membership-vs-access.
- **Alpha provenance:** `../alpha/thinking/social-graph-as-substrate.md` §1–3; raw
  `../alpha/seeds/transcripts/raw/social-graph-substrate-redb-storage-dialogue-2026-06-26.md`.

### T23 — Verbatim grounding quotes for the systems-science section (Ashby gloss, Beer paraphrase) — CLOSED

- **Status:** `closed` **2026-06-26** (source supplied + incorporated). One verification flag carries
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

### T16 — Matrix close-cousin E2EE operational lessons (UTD invariant, mandatory-recovery onboarding, expectation-gap)  (promoted 2026-07-07 → fenced/group-chat-failure-modes.md)

- **Status:** `open · gated` (lessons settled; design responses unbuilt).
- **Type:** `needs-content` (expand per S2).
- **What it is:** Matrix's production-paid E2EE lessons as direct design commitments — treat "every current member can decrypt every current-epoch message" as a **continuously-tested invariant** with a friendly key-request/healing path (never a dead "Unable to decrypt" tile); make **recovery setup near-mandatory in onboarding** with a blocking warning before any single-device-no-recovery state; and the **expectation-gap list** (instant full-history search, link previews, read receipts, "all my history on a new phone") Croft makes Hard — "planned, not discovered late." Beyond T5 (scale) and T1 (governance).
- **Promotion target:** **04** (the decrypt-invariant + healing path) and **08** (the onboarding-flow + expectation-setting UX). The recovery-onboarding *flow* is distinct from the bannered recovery-*anchor* gate.
- **Gates:** decide the invariant/healing mechanism and the onboarding gate UX.
- **Alpha provenance:** `../alpha/research/discord-matrix-groupchat.md` (Matrix lessons + expectation gaps).

### T35 — The uncompensated community labor + data-opacity indictment (a distinct activism thread)  (promoted 2026-07-07 → activism/platform-extraction-and-captured-labor.md)

- **Status:** `open · surfaced` (surfaced 2026-07-06).
- **Type:** `needs-research` (couples-with `needs-content`).
- **What it is:** a *second*, distinct activism indictment, separate from the "platforms author the
  relational field" harm case (Layer 8, already filed). The charge: closed platforms run on near-universal
  **community moderation and good-faith community interaction that is never published anywhere**, so the
  platforms **absorb the labor and hide the scale** at once. You cannot even see how much moderation effort
  and healthy interaction you are producing for them, because the metrics (average group sizes, activity
  and moderation volumes on closed platforms) are not disclosed. Two harms compounded: **uncompensated
  community labor** (extraction of unpaid moderation/curation) and **epistemic opacity** (the scale of that
  labor, and of the interaction it sustains, is structurally invisible). A cooperative/edge-free form makes
  both legible and returns the value to the members who produce it, which is the constructive counterpoint.
- **Promotion target:** `activism/` (Layer 8), as a companion indictment to
  `structural-argument-narrative.md`; likely its own brief once sourced.
- **Gates:** find sourceable figures on closed-platform group sizes and activity/moderation volumes (or
  document that they are undisclosed, which is itself part of the claim); establish what *is* publishable
  vs. structurally hidden. Hold the same source discipline as the existing activism set (primary or
  clearly-flagged, conflicts shown).
- **Provenance:** user articulation 2026-07-06 (the layering discussion); seed examples: Facebook average
  group size, activity metrics on closed platforms, unpublished community moderation.

