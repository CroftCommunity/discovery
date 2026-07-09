# discovery / beta: closed and promoted threads (archive)

date: 2026-07-07

The archive of threads that have left the live queue in `OPEN-THREADS.md`: `promoted` (integrated into a
matured layer doc) or `closed` (resolved without promotion). Kept for provenance and back-reference, other
threads still cite these T-numbers. `OPEN-THREADS.md` is the current open-only list; the theme->layer map
and the entry schema live there. Do not delete a thread; move its block here when it closes or promotes.

The theme numbers (`0N`) some older entries reference resolve to layers per `../alpha/LAYER-ROLLUP.md`.

---

### T37 - Placement and numbering of the large-group-scaling section (it collides with Part 2 §7)

- **Layer:** drystone-spec
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

- **Layer:** drystone-spec
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

- **Layer:** croft
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

- **Layer:** drystone-spec, philosophy
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

- **Layer:** fenced
- **Status:** `open · gated` (lessons settled; design responses unbuilt).
- **Type:** `needs-content` (expand per S2).
- **What it is:** Matrix's production-paid E2EE lessons as direct design commitments — treat "every current member can decrypt every current-epoch message" as a **continuously-tested invariant** with a friendly key-request/healing path (never a dead "Unable to decrypt" tile); make **recovery setup near-mandatory in onboarding** with a blocking warning before any single-device-no-recovery state; and the **expectation-gap list** (instant full-history search, link previews, read receipts, "all my history on a new phone") Croft makes Hard — "planned, not discovered late." Beyond T5 (scale) and T1 (governance).
- **Promotion target:** **04** (the decrypt-invariant + healing path) and **08** (the onboarding-flow + expectation-setting UX). The recovery-onboarding *flow* is distinct from the bannered recovery-*anchor* gate.
- **Gates:** decide the invariant/healing mechanism and the onboarding gate UX.
- **Alpha provenance:** `../alpha/research/discord-matrix-groupchat.md` (Matrix lessons + expectation gaps).

### T35 — The uncompensated community labor + data-opacity indictment (a distinct activism thread)  (promoted 2026-07-07 → activism/platform-extraction-and-captured-labor.md)

- **Layer:** activism
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

### T21 — Is `share` fully a right, or partly a membership-class capability?  (folded into T31, 2026-07-07)

- **Layer:** drystone-spec
- **Status:** `open · gated`.
- **Type:** `needs-content` (couples T31 — rights/role/capability disentanglement).
- **Review (2026-06-26):** part of this is the capability-vs-role confusion (see T31). Also question the
  **word "share"** itself — peers have a right, but **you cannot have a right to someone else's
  resources** in this model, so "share" may misframe it. The real question is whether peers have a
  **right to communicate** with others and **what the boundary is** that still respects the right to exit
  and the right to fork. "There's something there; needs more thinking." (See also T22 — tenure may be
  the user's version of this.)
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

---

## Promoted in the 2026-07-08/09 comprehensive pass

These threads left the open queue when their content was integrated into layer docs during the comprehensive pass (plan: `../alpha/plans/2026-07-08-beta-comprehensive-pass.md`). Status: **promoted**. Kept here for T-number back-reference; the promotion map is in each block and in the pass plan.

### T28 — promoted → `philosophy/the-peer-rights-razor-and-its-lineage.md`

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


### Phase-1 recovery cohort T41–T47 (cairn) + T48 (governance) — promoted

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

