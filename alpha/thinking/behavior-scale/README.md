# Behavior-scale — index, build pipeline, and registry

This cluster holds the **reusable method** for building Croft's persona-switch pads: a fully
functional in-browser model of a product, used as the permanent instrument for UX work,
function-level testing, and agent-driven development — and **scaled across a continuum of backends
(mock → small → medium → large)** without forking the front end. It is a cross-cutting method (it
applies to any Croft pad), homed here at top level rather than under `thinking/app/`.

Naming: the founding dialogue called this "the behavioral twin"; **renamed to *behavior-scale* (user,
2026-07-31)** — "twin" implied exactly two deployments, but there is one behavioral model deployed at
a point on a scale axis. Provenance: authored in claude.ai design dialogues (2026-07); raw session
filed content-faithful at `../seeds/transcripts/raw/stellin-graze-behavior-scale-sessions-2026-07.md`
(PLAYBOOK §4).

## The continuum (by backend)

One behavioral model, one contract, deployed at a point on a scale axis set by how much real backend
sits under it:

- **Mock** — no real backend or data; the starting UI (persona dropdown, in-memory event log). "Mock"
  = *no backend*, not *faked* — the logic is real. What ships first; what Stellin and Graze are today.
- **Small** — a small real backend for a high-trust group (honor-system identity, event-log `sync`
  across a few devices). Honest small-scale software, not a hardened multi-tenant service.
- **Medium** — a mid-scale real backend (some capabilities on `api`, real accounts/concurrency).
- **Large** — the full scaled platform the mock was always a model of (e.g. an atproto AppView, or a
  Next.js + Postgres service).

The front end and the contract do not change along the continuum — only the substrate under each
capability does.

## The two method docs

| Doc | What it is |
|---|---|
| [`behavior-scale-methodology.md`](behavior-scale-methodology.md) | The full-lifecycle discipline: one contract across the mock→small→medium→large continuum. Vocabulary, the three load-bearing artifacts (shared contract, conformance harness, agent-legible law), four substrates (`memory`/`api`/`hybrid`/`sync`), eleven invariants, decision procedures, the small-tier requirements, a seven-layer roadmap, appendix skeletons. |
| [`persona-switch-prototype.md`](persona-switch-prototype.md) | The **prototype layer** (the mock tier) on its own: how to build the local-only static SPA/PWA — five load-bearing ideas, GitHub Pages constraints, repo shape, event-log data architecture, persona-roster design, seed authoring, the dev bar, client-side engines, the four screen states, the acceptance-checklist method, and the actions.js migration seam. |

Relationship: the persona-switch prototype builds the **mock** (the first, independently-valuable
tier). The later layers (contract → adapter → conformance → scaling → agent → small-tier) are how the
same model scales up the continuum into a provable model of a real backend and, eventually, a
publishable small-scale edition.

## The build pipeline (how each build was made, reusable)

Both current builds followed the same lineage. To spin up the next one, repeat it:

```
1. Pick a genre exemplar        (LinkedIn → Stellin; Reddit → Graze)
2. Clean-room UX research        → research/<exemplar>-ux-*.md
   (what the exemplar does, what any product could choose, what the prototype should do;
    ground ranking/graph math in primary sources; label secondary/reconstructed facts)
3. Extend to a build-ready spec  → thinking/app/build-specs/<name>-*-build-spec.md
   (routes, screens, data model, API surface, algorithms, milestone plan)
4. Re-substrate onto the method  → thinking/app/build-specs/<name>-persona-switch-spec.md
   (data model → event vocabulary; API → selector/action contract; math → pure engines;
    add brand, persona roster, seed scenario, frontier markers, acceptance checklist)
5. Build the mock in milestones  (PWA layer last; dev bar reads as scaffolding, not product)
```

Clean-room discipline throughout: read as the *genre*, never the brand (no cloned logos, copy, or
exact brand colors); spend distinctiveness on one signature element (Stellin: the degree badge;
Graze: the comment collapse gutter).

## Registry of builds

| Build | Genre | Current tier (ships now) | Large-tier target (the real backend) |
|---|---|---|---|
| **Stellin** (was "Meridian") | Professional networking (LinkedIn-shape) | **mock** — `stellin.app` · repo `CroftCommunity/stellin` · persona-switch PWA | atproto AppView "Stellin by Croft" — `seeds/stellin-unpacked/` RUN-14/15 appview-infra; `research/stellin-name-clearance-2026-07.md`; `NAMING.md` → "App-layer naming" |
| **Graze** | Topic aggregation / forum (Reddit-shape) | **mock** — `graze.ing` · repo `CroftCommunity/graze` · persona-switch PWA | **contested — see below.** The build spec assumes Next.js + Postgres; the corpus's active forum plan (`plans/2026-07-27-read-first-forum-mvp.md`) chose a read-first lens over the public Bluesky AppView |

Per-build filed material:

- **Stellin:** build prompt `../thinking/app/build-specs/stellin-meridian-build-prompt.md` · research
  `../research/linkedin-ux-architecture-2026-07.md`.
- **Graze:** build spec `../thinking/app/build-specs/graze-persona-switch-spec.md` · topic-aggregation
  build spec `../thinking/app/build-specs/graze-topic-aggregation-build-spec.md` · research
  `../research/reddit-aggregation-ux-slashdot-baseline-2026-07.md`.

## The mock ↔ large-tier relationship (why this matters to the corpus)

Behavior-scale is the corpus's "one contract, scaled across a continuum" made concrete. Each Croft pad
can start as a **mock** (the truthful UX instrument that ships immediately) and scale, capability by
capability, toward a **large** real backend, joined by one shared contract and proven identical by the
conformance harness. Same seam whether the pad is Stellin, Graze, or the next one.

**Open reconciliation (tracked in `COHESION.md` §63):** for Graze, the build spec's assumed large-tier
backend (Next.js + Postgres) and the corpus's active forum plan (read-first lens over the public
Bluesky AppView, `plans/2026-07-27-read-first-forum-mvp.md`, on the Social Tree backbone) are **not the
same architecture**. Under this methodology that is a fork to surface, not resolve — the mock answers
"does it behave correctly and feel right"; which backend the large tier uses is the user's design
decision. Do not silently merge the two visions.

## Boundary (state everywhere a build reports results)

A behavior-scale build answers "does it behave correctly and does it feel right." The mock **never**
answers "does it hold up." In-memory correctness implies nothing about concurrency, conflicts, latency
under load, abuse handling, or capacity. Any question with a number attached routes to the tier that
runs at that scale.
