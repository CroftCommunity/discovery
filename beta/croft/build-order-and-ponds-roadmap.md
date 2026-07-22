# The ponds roadmap: build order, per-pond discipline, and the pond catalog

date: 2026-07-22 (Pass 2 — reconciled with the actual build state: two tracks, the iroh-native spine
resolver-gated, the atproto pads live. The Track-B sequence is settled; candidate ponds and the per-pond
discipline are recent and flagged)

**What this is.** The roadmap for the garden: what to build, in what order, and how each pond earns its
place. It answers the standing question "what do we build and how does it stack" by laying the pond and
pad catalog onto a sequenced build plan, foundation first, breadth last. It sits beside
`product-the-garden-of-ponds.md` (the product *shape*) and `presence-ritual-and-composed-ponds.md` (the
product *conclusions*); this doc is the *sequence and the catalog*. Everything here rides the neutral core
(`../drystone-spec/`, `../impl/`) and the pad building blocks credited in `../cairn/iroh-app-pond-building-blocks.md`.

## The two tracks (how it actually stacks)

The garden is being built along **two tracks at different maturity**, and reading the sequence below
without that frame is misleading. The distinction is which substrate a pond rides.

**Track A - atproto / PDS pads (the shipping lineage).** These ride a user's own atproto identity and PDS
records. They are **proven and live today** and need **no deep-link resolver**: `arecipe` (recipe box +
meal planner), `skylite` (a tended Bluesky window), and `pdsview` (a PDS content browser) are live, and the
1:1 greeting card (`greetings.croft.ing`) shipped as a static PWA. The **currently chosen next build is an
aggregator pond** (Bluesky / Mastodon / Lemmy) on this same path, because it inherits a population instead
of facing the empty-room problem and needs no new navigation primitive. The music-guessing pond is another
Track A candidate. This is where near-term shipping happens.

**Track B - iroh-native ponds (the composable local-first garden).** These are the peer-to-peer,
no-account, no-server ponds: the games pond (including the candy-crush-style match-3 and solitaire), the
native Croft Group pond, and the presence/ritual heart. They are the deeper thesis, and they are **all
gated on the tier-zero deep-link resolver** (Section 0.2 below), which is **not built**; iroh transport is
still spike-level. The **Phase 0-5 build sequence below is the plan for Track B** once the resolver exists.

**How the two tracks relate.** Track A ships value now on a proven substrate and builds the audience and
the operating muscle; Track B is the differentiated end-state the resolver unlocks. The client-side
**account kernel** (`../OPEN-THREADS.md` T55) is the bridge across the live Track A pads. **Update (K1
spike, 2026-07-22):** the make-or-break test found same-site subdomain storage is *shared* on Chromium but
*partitioned* on WebKit/Safari (hence iOS) on real `croft.ing` domains, so the original **one-shared-store**
form is not viable cross-browser. The kernel pivots to a **per-app storage + postMessage sync-coordinator**
form (a message broker / single-writer coordinator, not a shared blockstore); the session-broker
unification (OAuth/DPoP built 3x across the pads) may still stand on its own. Details + evidence in T55,
`../../alpha/spike/account-kernel/K1-SPIKE-RESULTS.md` / `KC0-RESULTS.md`, and the settled record
`../../alpha/thinking/app/estate-architecture-and-browser-constraints.md` (single shared origin for
co-located pads + isolated subdomains for untrusted-input apps + off-domain sandbox for untrusted code;
per-app login the serverless default, a small BFF the optional cross-browser-SSO add-on).

## The ordering logic (Track B, the iroh-native spine)

Three rules decide the sequence.

**First, the growth path and the core UX are the same component**, so it comes before content. The
deep-link resolver is both the only navigation into a pond and the entire acquisition model (there is no
public discovery by design), so nothing spreads without it. It is the root dependency. First-among-equals
with it are the **substrate adapters** (the iroh integration and the webxdc-compat shim), because every
pond and pad sits on them. Resolver and substrate are the foundation; they unlock everything else.

**Second, prove one pond fully before building three.** The integration surface, where the resolver, the
substrate, the pond context boundaries, and a UI shell cohere into something a non-technical person finds
simple, is where local-first projects actually die. One complete vertical slice flushes out the seam-work
cheaply.

**Third, sequence by leverage, not by excitement.** The fair-reveal primitive unlocks a whole column of
games and governance-grade voting at once, so it ranks above any single app. Shared engines (the
event-sourced store behind a ledger and a guestbook) get built once, early, and reused.

## The build sequence (Track B — gated on the resolver)

This is the plan for the iroh-native spine. None of it is built; Phase 0.2 (the resolver) is the tier-zero
blocker for everything else here. Track A (the live atproto pads and the aggregator-pond next build) does
not wait on this sequence.

### Phase 0 — Foundation (the root dependencies)

Nothing user-facing ships here. This is the platform everything else assumes.

- **0.1 The iroh integration layer.** Wrap iroh's three protocols into one clean internal API: a broadcast
  channel (gossip), a synced key-value store (docs), blob transfer (blobs), plus a direct point-to-point
  QUIC stream per peer pair. The direct channel is what makes Croft a better host than a broadcast-only
  webxdc runtime. Decide and honestly document the relay posture (default relays now, self-host later).
- **0.2 The deep-link resolver and the catalog manifest.** The tier-zero component (see the parallel gates
  below). Build the URL grammar `{pond identity + capability} / {app id + version} / {instance + entry
  context}`, shareable at three depths (pond, activity, instance); the single catalog manifest the resolver
  reads (and the optional assistant later reads too); and the three intake cases (in-app routing;
  join-from-link via a ticket, where the link is a credential so capability is decided per channel; and
  cold-install via the claim-code / one-more-tap flow, since seamless deferred deep-linking is not privately
  achievable, tracked as a constraint in `product-the-garden-of-ponds.md`).
- **0.3 The pond context boundary.** Each pond is its own security context. The Cure53 webxdc audit lesson
  (one-homed in `../cairn/iroh-app-pond-building-blocks.md`) is that CSP alone does not contain a webview,
  so establish the hard process boundary between ponds now, before apps exist, and disable WebRTC in any
  pad webview per platform (pure upside, since iroh is the transport).
- **0.4 The shared event-sourced store.** A small reusable library on iroh-docs: append-only immutable
  authored events folded into a view client-side, never a shared mutable total. The ledger and the
  guestbook both use it; build it once. This is the same social-graph-as-substrate log shape described in
  `social-graph-as-substrate.md`.

**Gate out of Phase 0:** a developer can mint a deep-link, have it resolve and join a pond on another
device, and read/write events that sync. No apps yet, just the plumbing proven.

### Phase 1 — The first pond, end-to-end (the vertical slice)

Pick one pond and make it genuinely complete and good, including the unglamorous parts (onboarding, the
empty state, the share flow, graceful absence of the assistant). The goal is to flush out the integration
surface, not to ship breadth. Recommended first pond: the **games pond**, because games exercise the most
transport variety (turn-based and real-time) and the cold-open hook is what gets anyone in the door.

- **1.1 A build-fresh reference game** (Four in a Row): the cold-open hook and the reference implementation
  of the whole challenge-to-outcome loop. Perfect-information, trivial logic, turn-based, clean license, so
  it proves the pattern with the least game logic.
- **1.2 The fair-reveal (commit-reveal) primitive:** one module (commit, lock, reveal, verify) built now,
  because it is the dependency for governance-grade voting and several games.
- **1.3 A second game using fair-reveal** (a dice or hidden-info game) to prove the primitive generalizes
  beyond voting.
- **1.4 The pond/verb/pin UI shell:** the three-layer model (ponds, verb-grouped activities, a pinned top
  layer), with deep-links landing inside the navigation with orientation chrome, not in a void; adaptive
  orientation (heavier for first-from-link arrivals, lighter for returning members).

**Gate out of Phase 1:** a non-technical person receives a shared link, lands on a specific game ready to
play, plays a friend peer-to-peer, the result records, and they can navigate the rest of the pond, all
without the assistant, on both first-build targets.

### Phase 2 — The killer utility (prove the second register)

One pond proved the fun register. Now prove the useful, keep-installed register with the highest-value
utility, reusing Phase 0-1.

- **2.1 Split-the-check** (build-fresh on the shared event-sourced store): the keep-installed reason, mostly
  UI and balance-folding on a substrate that already exists (integer minor units, append-only events,
  per-member roster keys).
- **2.2 Local real-time voting** (build-fresh): a live-tally mode on gossip plus a secret-ballot mode
  reusing the fair-reveal primitive. This is the bridge artifact, a fun group toy that is also cooperative
  governance infrastructure.

**Gate out of Phase 2:** a group is splitting real expenses and making real decisions in the app, with no
account and no server, and the balance and tally are correct across peers.

### Phase 3 — The heart (presence and ritual)

With both registers proven, build the cluster that makes a group feel ongoing, the emotional core and the
clearest values showcase (detailed in `presence-ritual-and-composed-ponds.md`).

- **3.1 The thinking-of-you ping** (build-fresh on gossip): the purest expression of the thesis and nearly
  the smallest app, roughly fifty bytes on a broadcast channel, the direct rebuke to the Bond Touch
  anti-pattern.
- **3.2 The guestbook plus question-of-the-day** (build-fresh on the shared store): the one app that
  accumulates. Append-only with redaction, no streaks (the no-Skinner-box discipline is the whole ethic);
  the daily prompt is a deterministic date-seed, no server picks it.
- **3.3 Lightweight invites with local reminders** (build-fresh on docs): closes the loop from find-a-time
  to gathering to guestbook; the real constraint is mobile background-notification limits per platform.
- **3.4 The cheap ritual additions** (Would You Rather, This-or-That, Two Truths and a Lie, rose-thorn-bud,
  status-glance): near-free once voting and fair-reveal exist.

**Gate out of Phase 3:** the three ponds (play, useful, together) all work, share one substrate and one
resolver, and a group has a reason to stay beyond any single task.

### Phase 4 — Breadth (now it is cheap)

Only with the foundation proven does going wide make sense.

- **4.1 The webxdc API shim**, implemented once, which makes the entire wrappable catalog (credited in
  `../cairn/iroh-app-pond-building-blocks.md`) available. A dozen games for one compatibility layer;
  inherit-and-flag licenses per app.
- **4.2 The remaining build-fresh classics** (Reversi, Dots and Boxes, Checkers, Go, a daily word puzzle, a
  drawing game).
- **4.3 The port-tier real-time games** (Curve Fever via Curvytron; GGRS + matchbox rollback), gated behind
  a direct iroh connection.
- **4.4 The remaining utilities** (shared album, checklist, notes, watch-together, shared map with offline
  tiles).
- **4.5 Pond presets** (a "Trip" preset first) that recombine existing apps into one-tap curated bundles and
  land the offline story hard.

### Phase 5 — The great-to-have (explicitly last, explicitly optional)

- **5.1 The on-device assistant** as a natural-language front-end to the resolver, never a dependency for
  any navigation below it. Detect-and-degrade across engine tiers (platform model, bundled WASM fallback,
  no-assistant), with strict catalog-constraint validation so the model only ranks intent and can never
  emit a link that does not resolve. This phase can slip indefinitely without harming anything, which is
  why it is last.

## The two parallel gates (conditions on the project, not phases)

- **The deep-link resolver (0.2) is tier-zero**, because it is both the core UX and the entire growth
  model. If it is not excellent, nothing spreads.
- **The sustainability model must be at least sketched before Phase 3 ships**, because the guestbook
  creates a custodial obligation the moment a family trusts it with years of memory. The
  cooperative-or-foundation answer (Layer 7, `../governance/`) turns "no one makes a buck" from a
  vulnerability into a credible forever-promise. Do not invite that trust before you can keep it.

## Per-pond build discipline (the pattern each game pond follows)

*(A recent distillation, 2026-07-22; the leaderboard scope is a settled owner decision, the rest is the
build-discipline pattern extracted from a game-design pass.)*

A game pond is built determinism-first, so the artifact survives and its outcomes are verifiable:

- **Determinism before engine.** The deliverable that matters first is the fixture corpus and the tie-break
  tables, not the code: golden vectors and a rules document, then the engine, red-first throughout. A
  Rust-to-wasm core buys the native-plus-wasm cross-build determinism test essentially free, and that test
  protects every phase downstream. A throwaway feel-spike (one afternoon, no tests, deleted after) can
  precede it to check the mechanics feel good.
- **Verifiable outcomes, not trusted arithmetic.** A per-game result is individually verifiable by replaying
  its move list against the engine's state hash. This is the same property the game-outcome-as-custom-
  lexicon conclusion needs (`presence-ritual-and-composed-ponds.md`): the settled outcome is a durable,
  self-checking record, so a comparison is a shared fact rather than a client's claim. Prefer a count of
  clean clears ("142 cleared clean") over a ratio, because a count is additive, forgiving, and individually
  verifiable, where an aggregate ratio is not verifiable at all.
- **Compare within the follow-chain, not globally.** The leaderboard shows a number per level next to the
  people you follow (and, if wanted, those who follow them), not a global ranking. The stranger-scale
  problems that justify anti-farming machinery do not exist among the twenty people you actually know;
  keeping comparison inside the social graph keeps it conversation, not competition. This aligns with
  social-graph-as-substrate: the graph you hold scopes who you measure against.
- **Sustainment is a standing drill, not a phase.** The ten-year promise means, per game document type
  (levels, saves, share codes), keeping a fixture per version forever and running an annual byte-identical
  regeneration from the master seed on a clean machine; dependency minimalism (few, pinned, boring);
  periodic checks on real devices for browser drift (storage eviction, service-worker behaviour, wasm
  feature availability); and exit artefacts (export works, source is licensed, the static bundle is
  self-hostable, and a plain statement of what happens if the project stops). This is the same survivability
  posture the sustainability gate above demands, applied to an artifact a person keeps.

Sequencing note: a game with no level-generation step (for example solitaire) is a shorter build than one
that needs generated content, and it is the same determinism work, so a determinism-first pond can ship a
real artifact soonest by starting with the simpler game and letting a richer one inherit the engine
discipline.

## The pond and pad catalog (what stacks where)

Activities by track and build state. **State** is the honest current status (live / shipped / built /
candidate / gated / not-built); **Phase** applies only to Track B's sequence. Candidate and gated items
point to `../OPEN-THREADS.md`.

| Activity | Track | Kind | State | Where in the sequence |
|---|---|---|---|---|
| `arecipe` (recipe box + meal planner) | A | pad | **live** (arecipe.app) | shipping lineage, no resolver |
| `skylite` (tended Bluesky window) | A | pad | **live** (skylite.croft.ing) | shipping lineage, no resolver |
| `pdsview` (PDS content browser) | A | tool-pad | **live** (pdsview.croft.ing) | shipping lineage; natural first home for `repo-mirror` |
| Greeting card, 1:1 link-delivered | A | pad | **shipped** (greetings.croft.ing, static PWA) | shipping lineage |
| Aggregator pond (Bluesky / Mastodon / Lemmy) | A | pond | **candidate — the chosen next build** | inherits a population, needs no resolver (the current direction) |
| Music-guessing pond (Heardle-shaped) | A | pond | candidate | local-first PWA, outcome as custom lexicon, crypto-monotonic anti-cheat; blocked on a licensing question (`../OPEN-THREADS.md` T58) |
| Collaborative group card (multi-writer) + card packaging ladder | A | pad | candidate | the collaborative arm needs the anon-multi-write shim; packaging model image / single-file HTML / `.xdc` (`../OPEN-THREADS.md` T56) |
| **Account kernel** (`account.croft.ing`) | A→B bridge | substrate | **K1 done: shared-store form FAILS on WebKit/iOS → pivot to postMessage sync-coordinator** (`../OPEN-THREADS.md` T55) | one-shared-store refuted on Safari; per-app storage + coordinator is the live form |
| Deep-link resolver + catalog manifest | B | substrate | **not built (tier-zero blocker)** | Phase 0.2 |
| iroh integration layer | B | substrate | spike-level | Phase 0.1 |
| Games pond (Four in a Row, fair-reveal, second game) | B | pond | designed, gated | Phase 1 (recommended first Track-B pond; commit open, T57) |
| Match-3 (candy-crush-style) + solitaire | B | pads (games) | designed, gated | Phase 1 / 4; **the worked examples behind the per-pond discipline above** (golden-vectors-first, verifiable clean-clear by replay, follow-chain leaderboard, P10 sustainment). Solitaire ships soonest (no level generation); match-3 inherits its engine. P1-P10 narrative (tail) in the raw (T57) |
| Split-the-check | B | pad | designed | Phase 2 |
| Local real-time voting | B | pad | designed (also governance infra) | Phase 2 |
| Presence & Ritual (ping, guestbook + QOTD, invites, rituals) | B | pond | designed | Phase 3 (`presence-ritual-and-composed-ponds.md`) |
| Webxdc-wrapped catalog + build-fresh classics | B | pads | designed | Phase 4 |
| Port-tier real-time games (Curve Fever, rollback) | B | pads | designed, gated on a direct iroh connection | Phase 4 |
| On-device assistant | B | overlay | designed | Phase 5 (explicitly optional, explicitly last) |

Reading the catalog: **Track A is where the next real build happens** (the aggregator pond), on a substrate
that already carries four live pads. **Track B is entirely designed-not-built and blocked at Phase 0.2**
(the resolver); the games pond, including the candy-crush match-3, lives there and cannot ship until the
resolver does. The **account kernel** is the one piece being actively spiked, and its near-term
justification is Track A (unify the live pads into one signed-in, one-mirror estate), which is also why it
is worth proving now rather than deferring to the Track B spine.

## One-screen summary

Two tracks. **Track A (atproto pads)** is live and shipping: arecipe, skylite, and pdsview are live and the
1:1 greeting card shipped; the **aggregator pond is the chosen next build**, and it needs no resolver.
**Track B (iroh-native ponds)** is designed but blocked at the tier-zero deep-link resolver: foundation
(iroh layer, resolver + manifest, pond context boundary, shared store), then one pond end-to-end (games),
then the killer utility (split-the-check, voting), then the heart (ping, guestbook, invites, rituals), then
breadth (webxdc shim, classics, port-tier games, utilities, presets), then the optional assistant. Each
Track-B game pond is built determinism-first with verifiable outcomes, compared within the follow-chain,
and kept alive by a standing sustainment drill; the candy-crush match-3 and solitaire live here. The
**account kernel** bridges the two (it unifies the live Track A pads first) and is being proved by spike.
Resolver is tier-zero for Track B; sustainability is sketched before the heart ships.

## Provenance & status

Pass 2, 2026-07-22. **Pass 2 reconciled the roadmap with the actual build state** (`../../alpha/BUILD-INVENTORY.md`,
2026-07-20): the two-track split, the live Track A pads (arecipe / skylite / pdsview live; greetings shipped
2026-07-21), the aggregator pond as the chosen next build (ROADMAP_TODO E39), and the fact that the whole
Track B spine is gated on the unbuilt tier-zero resolver (E11). Pass 1 had presented the Track B "games
pond first" sequence as the roadmap and mislabeled the atproto ponds as merely parallel.

The Track B build sequence and the two parallel gates consolidate the settled build-order thinking
(`../../alpha/thinking/app/ponds/build-order.md`, promoted in-tier here) with the companion `fair-reveal`
and pond-catalog work in the same alpha corpus. The per-pond build discipline and the pond candidates
(music-guessing, collaborative card, account kernel) are recent and flagged; their raw sources are the
2026-07-22 batch (`../../alpha/seeds/transcripts/raw/croft-games-pond-roadmap-browser-p2p-phased-build-2026-07-22.md`,
`...card-maker-webxdc-packaging-and-push-2026-07-22.md`,
`...gemini-atproto-clientside-search-heardle-pond-kudoboard-2026-07-22.md`), staged in `../OPEN-THREADS.md`
T55-T58 and tracked in `../../alpha/ROADMAP_TODO.md` E44-E47 (aggregator direction E39). The promotion trace
is recorded in `../../alpha/LAYER-ROLLUP.md`. Open decisions stay surfaced, not resolved: the games-pond
commit, the account-kernel domain-separation gate (T55), and the music-game licensing question (T58, the
owner's call, not-legal-advice). iroh/atproto facts cite the FACTCHECK source of truth; they are not
re-verified here.
