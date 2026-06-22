# The Croft app — design thinking (a new body of work)

date: 2026-06-22

status: intake in progress. This is the **application/client layer** ("Croft" the product) — a
distinct new body of thinking that *rides on* the proven lineage-groups substrate. Everything in
`discovery/` until now has been the **protocol/substrate** (lineage-groups) under the umbrella
vision; the app was only ever mentioned incidentally ("a likely eventual client"). This directory
is where the app's design lives, kept separate from the protocol thinking.

Source: a long design dialogue (filed verbatim-cleaned at
`seeds/transcripts/raw/croft-app-design-dialogue-2026-06-20-to-22.md`) plus a 6-file artifact zip
(frozen at `seeds/multiecosystemapp-unpacked/`). This README is the untangle: what is discovery
(thinking) here, and what is experiment (code) that lives elsewhere.

## What Croft-the-app is

A **composable, user-respecting "utility garden"**: one warm, consistent shell that hosts many
**ponds** (connections to existing social ecosystems — Bluesky, Mastodon/fediverse, Lemmy — each
kept native, honest seams, no fused model) and **pads** (small self-contained apps that float in
the garden — games, tools). The one pond Croft fully owns is **Croft Group**: private group chat
(and later P2P social games) on the **iroh** substrate — i.e. the lineage-groups work, surfaced.

Architecture spine: a **pure Rust functional core** (`(state, intent) -> (state, effects)`, no
I/O, WASM-clean) behind a thin **imperative shell** per platform (CLI, web/Leptos, desktop/Tauri,
mobile), with effects performed by the host. Hexagonal, effects-as-data — the Crux *pattern*
adopted slim, not the framework. Web-first; desktop is the wrapped web app; the heavy P2P work is
isolated in the one pond Croft owns. **This is now a settled decision** (2026-06-22):
`client-architecture-adr.md` — one shared core + thin per-platform shells (each supplying its own
`effects.rs` callout), two callout axes (platform + implementation-behind-a-port). Prior client work
(`../../../experiments/iroh/crates/croft-chat-cli`) adapts to it (ROADMAP_TODO E19).

## The discovery / experiment split (the untangle)

**Discovery (thinking) — here:**

- `design-philosophy.md` — the why: values, the functional-core/imperative-shell spine, honest
  seams, the garden thesis, credit-and-traceability, decision rules, the stack, and a decisions log
  (incl. candidate ponds: OpenMeet read-only, P2P games).

- `client-architecture-adr.md` — the **settled client-architecture decision** (2026-06-22): one
  shared functional core + thin per-platform shells, two callout axes (platform `effects.rs` +
  implementation adapters behind a port). The model all client work adapts to. Proven in the Phase-0
  import.

- `design-criteria.md` — the bar: shared + pond-specific + pad-specific criteria (on a
  WeChat-derived four-principle skeleton, gatekeeper mechanics stripped), and the visual system
  (cream / deep-green / ink palette with **semantic-meaning** accents, not per-pond).

- `brand-and-voice-notes.md` — **working draft, unsettled.** Name, tagline ("Grow your own"),
  two-speed answer to "own what", "homegrown" as identity register, the message funnel. Reconcile
  with `../../NAMING.md` (umbrella Croft) when it settles — currently flagged in ROADMAP.

- `../../seeds/generated-prompts/games-pond-research-prompt.md` — the prompt that forked the games
  hunt into its own deep session. **Now run** — it produced the `ponds/` cluster below.

- **`ponds/` — the ponds & pads catalog + build mechanics (2026-06-20→22 games/apps dialogue).** The
  deep dive into what actually fills the garden and how to build it. Raw dialogue at
  `../../seeds/transcripts/raw/croft-app-ponds-games-dialogue-2026-06-20-to-22.md`; 8 artifacts
  frozen at `../../seeds/apps-unpacked/`. Contents:
  - *Catalog* — `games-pond-authoritative-list.md` (games ranked by fun, by inclusion pathway:
    build-fresh / wrap-via-webxdc-shim / port-via-WebRTC-swap), `apps-pond-utility-list.md`
    (utilities by "have you seen this?" energy), `p2p-games-pond-launch-set.md` (the original hunt),
    `webxdc-security-and-competitive-games.md` (the Cure53-audited webxdc security model + the
    competitive 2-4p widening).
  - *Build mechanics* — `build-order.md` (six-phase sequencing; the deep-link resolver is tier-zero
    because it is both the core UX unlock and the only acquisition path), `fair-reveal-primitive-spec.md`
    (one shared commit-reveal module powering voting + dice + hidden-info games),
    `build-shape-pass.md` (license verification, the iroh-docs event-sourced ledger schema, local
    real-time voting, the ponds/verbs/pinning UX model).
  - *Feasibility* — `on-device-llm-feasibility.md` (per-platform on-device-LLM assessment for an
    optional, **never-required**, easily-disabled navigation assistant; Android+macOS are the
    first-build targets, iOS deferred).

  Substrate note: this all rides **iroh 1.0** — `iroh-gossip` (ephemeral/live), `iroh-docs`
  (accreting state, LWW per (author,key) → event-sourcing), `iroh-blobs` (content-addressed files).
  Honest asterisk: browser peers are permanently relayed, and direct hole-punch falls back to relays
  (n0's by default) — "no application server" is true; connection-bootstrap leans on relays.

**Experiment (code) — lives elsewhere, import deferred:**

- `build-specs/BUILD-SPEC.md` (Phase 0) and `build-specs/BUILD-SPEC-PHASE-1-2.md` are the
  build *intent* (same role as `../experiment-suite.md` for the protocol). They describe the code,
  not the code.

- **Phase 0 was actually built** (functional core + CLI/web/desktop shells, 20/20 acceptance tests
  green) in a **CroftC repo (PR #10)**. **IMPORTED 2026-06-22** to
  `../../../experiments/croft-app-phase0/` (byte-identical), at the user's direction — the A8
  IP/ownership decision exercised (paper trail in that dir's `PR-CONVERSATION.md`). It is the
  executable Phase 0 + the proof of the client-architecture ADR. Deferred within it: M6 (live
  Jacquard adapter). Residual: the CroftC-side PR is still OPEN (A8). See ROADMAP_TODO A8/B11/C7 and
  COHESION §23.

## Open risks the dialogue surfaced (not yet resolved)

These are captured in `../open-considerations.md`; the highest-stakes ones:

- **Infrastructure sustainability ↔ the cooperative model.** Relays (browser peers are permanently
  relayed), the public bridge node, the scoped appview, and push origination are ongoing metered
  cost. "Cooperative" is so far a value, not yet a funding/governance *mechanism* — the most
  important unthought thing.

- **Moderation / safety vs. the kid-friendly goal.** E2EE-P2P-relay architecture is in genuine
  tension with courting "gen alpha / kid-friendly" use (CSAM flowing through blind relays;
  stranger-connection in chat/games). Must be squared before launch, not after.

- **Cold-start for the owned pond.** Aggregator ponds inherit other networks' populations; Croft
  Group has the empty-room problem. The games hook is currently the only answer, and games is a
  *candidate* pond, not committed.

- **CroftC entanglement** (above) — most time-sensitive.

## Follow-ons (not done this pass)

The dialogue contains substantial industry research (iroh-in-browser maturity, webxdc/Delta Chat
games + the WebRTC-transport-swap porting recipe, super-apps / W3C MiniApp, atproto appview routing
via service-proxy/service-auth, Rust client libs ATrium/Jacquard/megalodon, Crux/FCIS) and many
new ecosystem entries. Distilling that into `../../research/` and `../../ECOSYSTEM.md`, and
reconciling the brand naming into `../../NAMING.md`, are the next intake steps — flagged in ROADMAP
and COHESION so they are not lost.
