# Transcript intake — ponds roadmap, the account kernel, ENS framework, client-side-search research (plan, Pass 1)

Intake of a batch of design-dialogue + research transcripts (delivered 2026-07-22) into the CroftC
`discovery` corpus, following `PLAYBOOK.md` (§2b non-PR transcripts, §3 corpus coherence, §4 provenance)
and the beta staging discipline (`beta/README.md`, `beta/LAYERS.md`, `beta/OPEN-THREADS.md`). The user's
framing: *"take these transcripts and fold them into our beta level content, and much of this lends
itself to the roadmapping or it's not tethered or ordered in a way that makes sense, we are at that
point."*

This doc is the durable handoff artifact so the work survives a context clear. It plans the filing; it
does **not** commit anything. Commit only on request (PLAYBOOK §3b).

## Status

**Pass 1 — plan approved; open decisions resolved (2026-07-22); executing Phase 1.** User resolutions
below. Phase 2 (fold into beta + author roadmap) is gated on a between-phases discussion. No commits
until asked.

### Decision resolutions (2026-07-22)
1. **Phased** — do Phase 1 now, discuss before Phase 2.
2. **ENS = candidate thinking, not settled law** — stage in `beta/OPEN-THREADS.md`; do **not** write to
   `NAMING.md`. The whole framework (multi-vantage concept *and* the specific expansions) is candidate.
3. **Research fact-check = full, at outcome time** — Phase 1 files T4b `[UNVERIFIED]`; the *full*
   fact-check runs in Phase 2 when the research is processed into its outcome (not deferred to a
   backlog-only item).
4. **Card-renderer domain separation = tentative conclusion** — record the separate-registrable-domain
   answer as tentatively concluded (revisitable), not an open question.
5. **Music-game licensing = backlog + candidate thinking only** — ROADMAP_TODO backlog + OPEN-THREADS
   candidate; not a resolved gate.

## The material (five raw files, from a batch of four pasted sessions)

All are claude.ai / Gemini pastes with no canonical export → preservation status
`preserved-condensed (cleaned-paste)`, content-faithful not byte-pristine (PLAYBOOK §4 caveat in each
header). Land in `alpha/seeds/transcripts/raw/` (beta is not an intake stage — new material lands in
alpha; only settled synthesis folds into beta).

| # | Source session | Proposed raw filename | Voice | Notes |
|---|----------------|-----------------------|-------|-------|
| T1 | card-maker packaging + push | `croft-card-maker-webxdc-packaging-and-push-2026-07-22.md` | Claude | design dialogue |
| T2 | encrypted prefs + repo-mirror + kernel origin | `croft-encrypted-prefs-repo-mirror-and-account-kernel-2026-07-22.md` | Claude | design dialogue; `io`/`omg` in source are typos for `croft.ing` (normalize in distill, note in raw) |
| T3 | games-pond roadmap + browser P2P + phased build | `croft-games-pond-roadmap-browser-p2p-phased-build-2026-07-22.md` | Claude | design dialogue; the browser-native-P2P *answer* is not in the paste — flag as truncated/answered-elsewhere (matchbox/WebRTC is the known answer, already in `beta/cairn/iroh-app-pond-building-blocks.md`) |
| T4a | ENS backronym naming | `ens-backronym-multivantage-framework-2026-07-22.md` | non-Claude | naming session |
| T4b | Gemini: atproto client-side search + Heardle pond + Kudoboard | `gemini-atproto-clientside-search-heardle-pond-kudoboard-2026-07-22.md` | Gemini | header carries prominent **`[UNVERIFIED]` — Gemini output, user flagged "needs more fact-checking"** |

**Why split T4 into two raw files:** the ENS naming chat and the Gemini research/game/cards chat are
distinct source conversations that distill to different homes (NAMING vs research/ponds). Splitting keeps
each raw file coherent with its provenance. Reversible if the user prefers one file.

## Problem Statement

A batch of four transcripts arrived that (a) have no verbatim home yet — provenance risk — and (b)
weave in and out of the same handful of topics without an order that a future reader could follow. The
user wants them folded into **beta-level** content and, where the material supports it, turned into a
**ponds roadmap** ("what to build and how it stacks", `fun.croft.ing`). The threads:

1. **The `account.croft.ing` kernel** (T1+T2) — the largest net-new body. A shared client-side origin
   (session broker + encrypted-prefs cache + `repo-mirror` + write outbox) reached by every `*.croft.ing`
   subdomain via one embedded iframe; subdomains become stateless skins. The map shows this is the
   *thinnest-covered* area in the corpus today (nearest homes: `alpha/thinking/app/client-architecture-adr.md`,
   `alpha/research/atproto-private-data-architecture.md`).
2. **The ponds/games roadmap** (T3+T4-Heardle) — a phased game build plan (P1–P10, determinism-first,
   verifiable clean-clear, follow-chain leaderboard), browser-native P2P for games, and a concrete
   music-guessing pond (Heardle-shaped) with a licensing model and a 6-channel "sounds-like" starter set.
3. **The ENS framework** (T4a) — a polite multi-vantage-point re-expansion of "enshittification".
4. **Client-side-search research** (T4b) — a Gemini report on serverless atproto SPA search, flagged for
   fact-checking.
5. **Kudoboard** (T4b) — a real group-card incumbent; loops back to T1's collaborative-card thread.

## Approach

Follow the fixed beta discipline for every thread:

```
   RAW (verbatim floor)          STAGING                    FOLDED (settled)
┌────────────────────────┐  ┌────────────────────┐  ┌────────────────────────────┐
│ alpha/seeds/transcripts │─▶│ beta/OPEN-THREADS.md│─▶│ beta/<layer>/ docs          │
│ /raw/ + MANIFEST row    │  │ (per-thread gates + │  │ (consolidate-to-one-whole,  │
│ + connective tissue     │  │  promotion target)  │  │  quotes stand alone) +      │
│ (COHESION/ECOSYSTEM/    │  │                     │  │  layer reference-index.md + │
│  ROADMAP_TODO/NAMING/   │  │                     │  │  alpha/BETA-ROLLUP trace    │
│  kindred-work)          │  │                     │  │                             │
└────────────────────────┘  └────────────────────┘  └────────────────────────────┘
        Phase 1 (safe)          Phase 1 (safe)             Phase 2 (judgment)
```

### Thread → destination map

| Thread (source) | OPEN-THREADS gate(s) | Promotion target (beta) | alpha homes it extends |
|---|---|---|---|
| **account kernel** (T1+T2) | domain-separation ⚑; Web-Push-vs-Pushover-ops ⚑; single-origin quota/eviction posture | new `beta/impl/` (or `beta/croft/`) kernel doc | `thinking/app/client-architecture-adr.md`, `research/atproto-private-data-architecture.md`, `thinking/app/pwa-spa-best-practices.md`, `thinking/local-first-as-design-imperative.md` |
| **card-maker packaging** (T1) | (shares the domain-separation gate) | `beta/croft/product-the-garden-of-ponds.md` (three-rung output ladder; collaborative card = host+`.xdc`) | `thinking/app/ponds/virtual-cards-and-guestbooks.md`, greetings MVP plan (E43) |
| **ponds/games roadmap** (T3) | games-pond "candidate-not-committed" gate (already OPEN-THREADS T≈873); solitaire-first sequencing | consolidated roadmap: extend `thinking/app/ponds/build-order.md` → fold into `beta/croft/product-the-garden-of-ponds.md`; P2P block into `beta/cairn/iroh-app-pond-building-blocks.md` | `ponds/build-order.md`, `ponds/build-shape-pass.md`, `prds/games-pond.md`, `prds/chat.md`, `ponds/fair-reveal-primitive-spec.md` |
| **Heardle music pond** (T4b) | music-licensing gate ⚑ (pastiche closeness; Nintendo/SEGA/JASRAC); anti-cheat = crypto monotonic/CID ordering (decided) | a per-pond doc under `thinking/app/ponds/` → `prds/`; Heardle-as-ENS-case into `beta/fenced/` or `beta/activism/` | `ponds/` set; `ponds/on-device-llm-feasibility.md` (adjacent) |
| **ENS framework** (T4a) | settled-vs-exploratory ⚑ | reasoning → `beta/fenced/aggregation-theory-and-the-enshittification-shield.md`; decision → `alpha/NAMING.md` new dated section | `NAMING.md`; `beta/socialization/coop-messaging-research.md` already flags "the acronym-re-expansion move ENS makes" |
| **client-side-search research** (T4b) | fact-check depth ⚑ | net-new `alpha/research/atproto-clientside-search.md` (research stays in research; not a beta layer) | `research/atproto-private-data-architecture.md`, `beta/cairn/atproto-*` mechanics docs |
| **Kudoboard** (T4b) | — | `alpha/ECOSYSTEM.md` row (group-card incumbent) + note in `beta/croft/product-the-garden-of-ponds.md` collaborative-card thread | — |

### Phase 1 — provenance + staging (safe floor)
1. Write the five raw files with correct headers + `preserved-condensed (cleaned-paste)` status + §4
   caveat; redact any creds (none seen); mark volatile facts `[UNVERIFIED]`; T4b header flags the whole
   doc unverified.
2. Add five rows to `alpha/seeds/transcripts/RAW-ARTIFACTS-MANIFEST.md`.
3. Connective tissue: `COHESION.md` (new §53+ mapping the kernel/ponds/ENS threads to their homes and
   any duplication/drift); `ROADMAP_TODO.md` section E new items (kernel build, ponds roadmap, Heardle
   pond, research fact-check, Web-Push onboarding); `ECOSYSTEM.md` (Kudoboard; Pushover; Web Push as a
   standard; the client-side search engines FlexSearch/MiniSearch/Orama/uFuzzy/SQLite-FTS5 as tools);
   `NAMING.md` (ENS — pending decision); `kindred-work.md` (Cory Doctorow already present under ENS —
   check for update only).
4. Stage every thread in `beta/OPEN-THREADS.md` with named gates + promotion-target layer per the map.

### Phase 2 — fold into beta + author the roadmap (judgment; gated on Open decisions)
5. Consolidate each settled thread into its beta layer doc (one-home-per-claim; quotes whole with a
   per-quote verification flag; reasoning travels with the decision).
6. Author the consolidated **ponds roadmap** (scope TBD — see Open decision 1).
7. Update the relevant `beta/<layer>/reference-index.md` files and record traces in
   `alpha/BETA-ROLLUP.md` / `alpha/LAYER-ROLLUP.md`.

## Reasoning

**Why raw lands in alpha even though the goal is beta.** `beta/README.md` is explicit that beta is not
an intake stage; folding is a promotion from a settled OPEN-THREADS entry, with the raw preserved in
alpha as the provenance floor. Writing distilled claims straight into a beta doc would skip the
one-home-per-claim consolidation and the reference-index/BETA-ROLLUP trace the tier requires.

**Why Phase 1 is separable and safe.** Raw preservation + manifest + connective tissue + OPEN-THREADS
staging lock provenance and make the material navigable without committing to any contested synthesis.
The judgment-heavy folding (Phase 2) depends on the open decisions and can happen after review.

**Why the kernel is treated as net-new, not an edit to an existing doc.** The map found no doc for
`account.croft.ing`, "the kernel", the auth broker, `repo-mirror`, or encrypted-prefs-as-repo-record;
the nearest home (`client-architecture-adr.md`) is a *shared-core-per-platform-shell* ADR, a different
axis. The kernel body earns its own consolidated doc once its gates are settled.

**FACTCHECK reconciliation (no drift).** T2 and T4b use "Merkle Search Tree" for atproto's repo
structure. The source-of-truth FACTCHECK
(`alpha/seeds/transcripts/raw/atproto-atmospheric-web-iroh-mobile-FACTCHECK.md`) *confirms* MST is
atproto's structure; the only refuted claim is describing **iroh-docs** as MST-based (it uses
range-based set reconciliation) — which none of these transcripts do. Flag cleared clean; cite the
FACTCHECK, do not re-verify. Sync v1.1 (relay non-archival; log streamed in causal order) is likewise
FACTCHECK-aligned. DRISL/DAG-CBOR treated in `beta/drystone-spec/dag-cbor-and-content-addressing.md`.

**Why the Gemini research files as research, not a beta layer.** It is a field survey (search-engine
comparison, VFS matrix, BM25) — `research/` per PLAYBOOK §1, distinct from `ECOSYSTEM.md`. It is
flagged `[UNVERIFIED]` throughout until a fact-check pass runs (Open decision 3); a `green`-looking
research doc built on an unverified model output would violate PLAYBOOK §4 "don't over-claim."

## Open decisions (surface, do not resolve — user's calls)

1. **Scope/phasing.** Phase 1 only (file + stage), then reassess — *recommended* — vs go all the way to
   beta folding + author the ponds roadmap this session. And if full: is the ponds roadmap a new beta
   doc, or an extension of `thinking/app/ponds/build-order.md` promoted into `beta/croft/`?
2. **ENS status.** Settled (record the multi-vantage framework in `NAMING.md` now) vs still exploratory
   (park in OPEN-THREADS). Sub-question: are the *specific expansions* locked (Business = Extractive
   Neglect Syndrome looked firmest; systems = Eventual Non-utility State; user = Erosive Nudge Syndrome;
   recursive = "ENS is Not Service"), or is only the multi-vantage *concept* settled?
3. **Research fact-check depth.** Defer (file `[UNVERIFIED]` + a ROADMAP_TODO validation item) —
   *recommended* — vs run a fact-check pass now against primary sources.
4. **Untrusted card-renderer domain separation** (architectural, T1). Separate registrable domain vs
   host-only session + kernel brokering. "Cheap config now / migration once 3 apps depend on it." →
   recorded as an OPEN-THREADS gate; the user decides before the kernel doc is folded.
5. **Music-game licensing gate** (compliance, T4b). The hybrid shape is decided (CC0/public-domain +
   original pastiche + auth-gated integrations, guest-first). The residual — "is pastiche too close?"
   and Nintendo/SEGA/JASRAC rights — is a legal call this plan will not auto-resolve (PLAYBOOK §5).

## Provenance checklist (per PLAYBOOK §6) — Phase 1 COMPLETE (2026-07-22)

```
[x] 5 raw files written; headers + cleaned-paste status + §4 caveat; io/omg typo noted; T4b [UNVERIFIED]
[x] creds redacted (none observed); volatile facts marked [UNVERIFIED]
[x] RAW-ARTIFACTS-MANIFEST.md — 5 rows ("2026-07-22 intake" section; closes the arecipe.zip truncated-report gap)
[x] COHESION.md — §53 (kernel/cards/ponds/ENS/research seam map + duplication-by-design notes)
[x] ROADMAP_TODO.md §E — E44 kernel, E45 card packaging, E46 ponds roadmap, E47 Heardle pond, E48 research FC, E49 ENS, E50 licensing
[x] ECOSYSTEM.md — §5g (Kudoboard, Pushover, Web Push, client-side search engines, coi-serviceworker); Jetstream/atcute already at §5f
[x] NAMING.md — deliberately NOT touched (ENS = candidate thinking, Open decision 2)
[x] kindred-work.md — no change (Doctorow already present under ENS; Kudoboard/Pushover are incumbents → ECOSYSTEM)
[x] beta/OPEN-THREADS.md — T55 kernel, T56 card packaging, T57 ponds roadmap, T58 Heardle pond, T59 ENS, T60 client-side search
[x] FACTCHECK cited for MST/Sync-v1.1; no re-verification (MST usage confirmed correct)
```

**Phase 1 is filed but NOT committed** (commit only on request, PLAYBOOK §3b).

## Phase 2 — progress (2026-07-22, per the between-phases discussion)

User directions: (1) promote `build-order.md` into `beta/croft/` + start the roadmap; (2) the account
kernel — **spike it, don't fold** (a strong proposition; prove it suits the need first); (3) run the
client-side-search fact-check fully; (4) ENS + Heardle stay in OPEN-THREADS.

- **[DONE] Ponds roadmap Pass 1.** `beta/croft/build-order-and-ponds-roadmap.md` (NEW) — `build-order.md`
  promoted in-tier + a first roadmap (build sequence settled; per-pond discipline + candidate ponds
  flagged). Added to `beta/croft/README.md` + `reference-index.md`; trace in `LAYER-ROLLUP.md`; `OPEN-THREADS`
  T57 → `in-progress`. Not committed.
- **[DONE] Client-side-search fact-check.** `alpha/research/atproto-clientside-search.md` (NEW, verified,
  em-dash-free, not committed). ~30 claims: 20 CONFIRMED / 3 PARTLY / 7 UNVERIFIED / 0 REFUTED. Corrections:
  **"SearchFn" likely invented** (substitute FlexSearch IndexedDB persistence), Jetstream filtered bandwidth
  overstated ~6–15×, uFuzzy ~7.5 KB not ~3 KB, MiniSearch is BM25+. Benchmark table + "Collection Signal
  Mode" left UNVERIFIED. E48 marked done; T60 fact-check resolved (optional cairn cross-ref remains).
- **[DONE] The account kernel → experiment series scoped (corpus pass done).** T55 reclassified to
  `needs-experimentation`. Plan: `plans/2026-07-22-account-kernel-spike.md`, expanded after a 4-way
  corpus fit/impact pass into an ordered series (gates G0 topology + G1 sealed-tier; then K1 storage
  make-or-break → K2 domain-sep → K3 single-writer → K4 browser-mirror → K5 session/prefs → K6 outbox).
  Findings: the data algorithm is already PROVEN in Rust (`hist_live` + encrypted-local-first Proofs) so the
  series targets browser realization + topology + concurrency + security, not the data logic; session-broker
  unification is the clearest win (built 3x); arecipe is off-site (topology gate G0); kernel is web-only +
  utility-plane. Not started (no code).
- **[DONE] Roadmap Pass 2.** Reconciled `build-order-and-ponds-roadmap.md` with the actual build state
  (`BUILD-INVENTORY.md`): a **two-track** structure — Track A (atproto pads live/shipping; aggregator pond
  the chosen next build; no resolver) vs Track B (iroh-native spine, all gated on the unbuilt tier-zero
  resolver; the candy-crush games live here). Account kernel is the A→B bridge. T57 note updated.
- **[PARKED] ENS (T59) + Heardle (T58)** stay as candidate OPEN-THREADS threads; no fold this session.

**Still open / next:** run the kernel spike (per its plan) when ready; the aggregator-vs-games near-term
build choice (Track A is the live direction); the games-pond commit decision; whether to fold the verified
client-side-search mechanics into a cairn doc. Nothing committed to git this session.
