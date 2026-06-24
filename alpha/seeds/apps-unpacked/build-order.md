# Build Order

author: research session

scope: a sequenced build plan for the P2P local-first ecosystem, from foundation to breadth, with the rationale for the ordering and the gates between phases

date: 2026-06-21

principle: build the load-bearing things first, prove one vertical slice end-to-end before going wide, and never let a great-to-have (the assistant) become a dependency for anything below it.

---

## The ordering logic

Three rules decide the sequence.

First, **the growth path and the core UX are the same component** (the deep-link resolver), so it comes before content. Nothing spreads without it, and almost every app needs it to be shareable, so it is the root dependency.

First-among-equals with it, **the substrate adapters** (your iroh integration and the webxdc shim), because every app sits on them. Resolver and substrate are the foundation phase; they unlock everything else.

Second, **prove one pond fully before building three**, because the integration surface (resolver + substrate + context boundaries + UI shell cohering into something a non-technical person finds simple) is where local-first projects actually die. One complete vertical slice flushes out the seam-work cheaply.

Third, **sequence by leverage, not by excitement**. The fair-reveal primitive unlocks a whole column of games and the governance-grade voting at once, so it ranks above any single app. Shared engines (the event-sourced store behind ledger and guestbook) get built once, early, and reused.

---

## Phase 0: Foundation (the root dependencies)

Nothing user-facing ships here. This is the platform everything else assumes.

**0.1 — The iroh integration layer.**

Wrap iroh's three protocols into a clean internal API your apps call: a broadcast channel (gossip), a synced key-value store (docs), and blob transfer (blobs), plus a direct point-to-point QUIC stream per peer pair. This is the thing that makes you a better host than webxdc (direct channels, not broadcast-only).

Verify the relay-fallback story and decide the relay posture (n0 default relays for now, self-host later), and write it honestly into the charter.

**0.2 — The deep-link resolver and the catalog manifest.**

The single most strategically important component, because it is both the core navigation unlock and the entire acquisition model (no public discovery by design).

Build the URL grammar: `{pond identity + capability} / {app id + version} / {instance + entry context}`, shareable at three depths (pond, activity, instance).

Build the one catalog manifest (ponds, apps, capabilities) that the resolver reads, and that the assistant will later read too. One source of truth.

Handle the three intake cases: in-app routing (easy), join-from-link via ticket (the link is a credential, so decide capability-per-channel), and cold-install via the claim-code / one-more-tap flow (warm opens through Universal/App Links, App Clips for proximity, claim-code for cold).

**0.3 — The pond context boundary.**

Each pond is its own security context (the lesson from the webxdc Cure53 audit: CSP alone does not contain a webview). Establish the hard process/context boundary between ponds now, before apps exist, so a compromised app in one pond cannot reach another pond's capabilities or keys.

If any third-party/wrapped web app code will run, disable WebRTC in the webview per platform here (it is pure upside since iroh is your transport, and it closes the exfiltration hole that cost Delta Chat months).

**0.4 — The shared event-sourced store.**

A small reusable library implementing the recurring data pattern on iroh-docs: append-only immutable authored events, folded into a view client-side, never a shared mutable total (last-writer-clobbers). The ledger and the guestbook both use this; build it once.

**Gate out of Phase 0:** a developer can mint a deep-link, have it resolve and join a pond on another device, and read/write events that sync. No apps yet, just the plumbing proven.

---

## Phase 1: The first pond, end-to-end (the vertical slice)

Pick **one** pond and make it genuinely complete and good, including the unglamorous parts (onboarding, the empty state, the share flow, graceful absence of the assistant). The goal is to flush out the integration surface, not to ship breadth.

Recommended first pond: **the games pond**, anchored by the cold-open hook, because games exercise the most transport variety (turn-based and real-time) and the hook is the thing that gets anyone in the door.

**1.1 — Four in a Row (build-fresh).**

The cold-open hook and your reference implementation of the entire challenge-to-outcome loop. Perfect-information, trivial logic, turn-based, clean license. Getting this one working over the resolver + iroh + a Bluesky-or-equivalent outcome record proves the whole pattern with the least game logic.

**1.2 — The fair-reveal primitive** (see the companion spec).

Build it now, because it is the dependency for the governance-grade voting and several games, and Phase 1 is where shared primitives belong. One module: commit, lock, reveal, verify.

**1.3 — A second game that uses fair-reveal.**

Backgammon (dice fairness) or a quick hidden-info game, to exercise the primitive in anger and prove it generalizes beyond voting.

**1.4 — The pond/verb/pin UI shell.**

The three-layer model (ponds → verb-grouped activities → pinned top layer), with deep-links landing *inside* the navigation (focused content plus orientation chrome), not in a void. Adaptive orientation: heavier for first-from-link arrivals, lighter for returning members.

**Gate out of Phase 1:** a non-technical person receives a shared link, lands on a specific game ready to play, plays a friend peer-to-peer, the result records, and they can look up and navigate the rest of the pond, all without the assistant, on both first-build targets. If this is smooth, the integration surface is proven.

---

## Phase 2: The killer utility (prove the second register)

One pond proved the "fun" register. Now prove the "useful, keep-installed" register with the single highest-value utility, reusing everything from Phase 0-1.

**2.1 — Split-the-check (build-fresh on the shared event-sourced store).**

The killer utility and the keep-installed reason. It reuses the Phase 0.4 store directly (it is the worked example of the event-sourced ledger), so it is mostly UI and balance-folding on a substrate that already exists. Integer minor units, append-only events, per-member roster keys.

**2.2 — Local real-time voting (build-fresh).**

Promote toward flagship. Open live-tally mode on gossip (the theatrical one) plus secret-ballot mode reusing the Phase 1.2 fair-reveal primitive. This is the bridge artifact: a fun group toy that is also cooperative-governance infrastructure for Croft.

**Gate out of Phase 2:** a group is splitting real expenses and making real decisions in the app, with no account and no server, and the balance/tally is correct across peers. The two registers (fun, useful) are both proven on the same substrate.

---

## Phase 3: The heart (presence and ritual)

With both registers proven, build the cluster that makes a group *feel* ongoing, the project's emotional core and its clearest values showcase.

**3.1 — The thinking-of-you ping (build-fresh on gossip).**

The purest expression of the thesis and nearly the smallest app. Non-verbal, pressure-free, real-time-first. Ships fast because it is fifty bytes on a broadcast channel.

**3.2 — The guestbook + question-of-the-day (build-fresh on the shared event-sourced store).**

The connective tissue, the one app that accumulates. Reuses the Phase 0.4 store (same shape as the ledger), adds blobs for photos. Append-only with redaction, no streaks (the no-Skinner-box discipline is the whole ethic). The daily prompt is a deterministic date-seed, no server picks it.

**3.3 — Lightweight invites with local reminders (build-fresh on docs).**

Closes the loop (find-a-time → invite → gathering → guestbook). Per-member RSVP keys, absolute-time storage, local notifications (verify mobile background-notification limits per platform, this is the real constraint).

**3.4 — The cheap ritual additions.**

Would You Rather, This-or-That, Two Truths and a Lie, rose-thorn-bud, status-glance, mostly reusing voting or fair-reveal, so they are near-free once Phase 1-2 exist.

**Gate out of Phase 3:** the three ponds (play, useful, together) all work, share one substrate and one resolver, and a group has a reason to stay beyond any single task.

---

## Phase 4: Breadth (now it is cheap)

Only here, with the foundation proven and the patterns established, does going wide make sense.

**4.1 — The webxdc API shim**, implemented once, which makes the entire ArcaneCircle catalog wrappable. This is the big leverage moment for breadth: a dozen games for one compatibility layer. Inherit-and-flag licenses per app.

**4.2 — The remaining build-fresh classics** (Reversi, Dots and Boxes, Checkers, Go, the daily word puzzle, the drawing game), each cheap on the proven substrate.

**4.3 — The port-tier real-time games** (Curve Fever via Curvytron, netplayjs/GGRS for rollback), gated behind a direct iroh connection, framed as "look what's possible."

**4.4 — The remaining utilities** (shared album, checklist, notes, watch-together with its media-locality discipline, shared map with offline tiles).

**4.5 — Pond presets** (the "Trip" preset as the first), which recombine existing apps into one-tap curated bundles and land the offline story hard.

---

## Phase 5: The great-to-have (explicitly last, explicitly optional)

**5.1 — The on-device assistant**, as a natural-language front-end to the Phase 0.2 resolver, never a dependency for any navigation below it.

Detect-and-degrade across three engine tiers: platform model (macOS Foundation Models with guided generation; Android ML Kit where present), bundled WASM fallback (for the no-platform-model Android majority, if you decide the conversational layer is worth a model download there), and no-assistant (the complete menu navigation everywhere else).

Strict catalog-constraint validation, engine-agnostic, so the model only ranks intent and the resolver maps to real links. The model can never emit a link that does not resolve.

This phase can slip indefinitely without harming anything, which is exactly why it is last.

---

## The two things that gate the whole thing (run in parallel, not a phase)

These are not build steps so much as conditions on the project, and they should be addressed alongside the phases above.

**The deep-link resolver (Phase 0.2) is tier-zero** because it is both the core UX and the entire growth model. If it is not excellent, nothing spreads. It earns being built first and built carefully.

**The sustainability model** needs to be at least sketched before Phase 3 ships, because the guestbook creates a custodial obligation the moment a family trusts it with years of memory. The cooperative-dues-or-foundation answer turns "no one makes a buck" from a vulnerability into a credible forever-promise, and it is the thing a skeptic pokes first. Do not invite that trust before you can keep the promise.

---

## One-screen summary

Phase 0, foundation: iroh layer, deep-link resolver + manifest, pond context boundary, shared event-sourced store.

Phase 1, first pond end-to-end: Four in a Row, fair-reveal primitive, a second game, the verb/pin UI shell.

Phase 2, killer utility: split-the-check, local real-time voting.

Phase 3, the heart: thinking-of-you ping, guestbook + daily prompt, invites, cheap ritual games.

Phase 4, breadth: webxdc shim, remaining classics, port-tier real-time games, remaining utilities, pond presets.

Phase 5, great-to-have: the optional on-device assistant.

Parallel gates: resolver is tier-zero; sustainability model sketched before Phase 3.
