# Plan: transcript intake 2026-07-27 — the "Social Tree" + Croft-landscape batch

date: 2026-07-27
identity: chasemp (`chase@owasp.org`, `github-personal`); repo `discovery`
status: **EXECUTED this pass (retrospective coordinator).** All four transcripts preserved, fact-checked, and
routed into the standing indexes; committed to `main`. Written after routing (not plan-first) at the user's
request, to give the batch the same plan-doc coordinator the 2026-07-22 / 2026-07-23 intakes have.

## Problem statement

Four transcripts arrived across one session (some pasted mid-turn): (1) a long **"Social Tree"** dialogue —
a local-first PWA that reframes Bluesky/atproto as a Reddit/Discourse forum fused with a Google-Reader RSS
reader; (2) a large **compound Croft-landscape** session (small/medium/big-world positioning · tree-as-
relational-UI · did:webvh watchers · Bluesky blob-dedup↔E2EE · a co-op service menu · best Bluesky clients ·
auth/MFA · Lightning/Nostr payments · responsive filters · Rust); (3)+(4) a paired **atproto-mechanics
explainer + phone-hotspot** Q&A. The risks specific to this batch: (a) it is heavy AI product-thinking with
many volatile technical claims (browser APIs, atproto internals, RFC numbers, spec semantics) that must not
be carried forward as fact; (b) it *reads* as net-new "build a Reddit on Bluesky" territory when it is
largely **existing tracks walked into product depth**; (c) it restates a settled Drystone principle
("compute provenance, never utility") that must be linked, not re-proposed; (d) it landed during a live
history-rewrite/force-push of the `discovery` repo, so filing had to survive resets and avoid pre-rewrite
SHAs and purged employer strings.

## Approach

Per PLAYBOOK: classify → preserve each raw under `seeds/transcripts/raw/…-2026-07-27.md` (cleaned-paste,
content-faithful — the compound paste triplicated its opening third → reproduced once) → heavy **live
fact-check** (user asked for verification, not blanket `[UNVERIFIED]`) → distil/route into the *existing*
standing indexes, never a parallel list → surface decisions, don't resolve (PLAYBOOK §5).

- **Fact-check:** eight parallel subagents (opus, matching the session model) across four domains → two
  research docs, `research/2026-07-27-social-tree-factcheck.md` (part 1) and `-factcheck-2.md` (part 2),
  ~80 load-bearing claims verified against primary sources. Raws carry a whole-document caveat pointing at
  the research docs; atproto/iroh/iOS baseline defers to the FACTCHECK SoT.
- **Routing:** ROADMAP_TODO **E62–E71**; ECOSYSTEM **§5i/§5j**; COHESION **§59/§60**; RAW-ARTIFACTS-MANIFEST
  rows for all raws. Everything framed as extension of E39 (aggregator pond), E24/E42 (sovereign AppView),
  E44 (account kernel/PWA sync), E48 (client-side search), E30/E31 (Drystone/heartwood), D5 (cooperative),
  the arecipe/Skylite/auth-helper clusters.
- **Git safety:** re-fetch + confirm sync after each force-push note; reference commits by message not SHA;
  verify no pre-rewrite SHA and no purged employer string in any authored file (0 hits — the one grep hit
  was the variable name `isAttested`).

## Reasoning

- **Why "extension, not net-new":** the Social Tree's own thesis ("a board is a saved query, no cold-start —
  we're a lens onto existing activity") *is* E39's thesis; its engine is E48's WASM-search substrate; its
  radius/mutes/mod-list/RSS layer is the client face of E24/E42; its sync/vault/push layer is E44. Filing it
  as a parallel "new product" would have fractured four existing tracks. One new E-item that extends them
  (E62) + the landscape items (E63–E71) keeps the corpus coherent.
- **Why the provenance razor is *linked*, not lifted:** it is already the crystallized bedrock **"The razor:
  compute provenance, never utility"** (`crystallized/principles.md` §"The deeper foundation (2026-06-20)";
  `beta/philosophy/epistemics-provenance-and-verification.md`; `commensurability-and-the-two-ledgers.md`).
  The transcripts restate it; treating it as a new candidate under-credited the corpus (corrected in
  E62/§59/§60).
- **Why heavy live verification:** this is AI product-thinking with exactly the failure mode the corpus
  guards against — plausible-but-wrong API/spec/RFC claims. Verifying now (vs deferring to a blanket
  `[UNVERIFIED]`) means the roadmap items carry the *corrections*, not the errors — which is what makes them
  actionable later.
- **Why retrospective plan-doc:** the batch was already fully routed + committed by the time the coordinator
  was requested; the value now is a single reasoning artifact + E-item map + the open-decisions list, matching
  the 2026-07-22/23 intake pattern so a future reader finds this batch the same way.

## The E-item map (E62–E71)

| # | One line | Home / relates |
|---|---|---|
| E62 | "Social Tree" — Reddit/forum + RSS-reader lens PWA over atproto (the whole product) | extends E39; E24/E42, E44, E48; §5i, §59 |
| E63 | Small/medium/big-world positioning + tree-as-relational-UI design criteria | E62; `social-layer.md`, `NAMING.md`; §60 |
| E64 | did:webvh watchers / bounded-staleness / "federation of availability" + DHT axis | E30/E31, heartwood; sovereign-AppView |
| E65 | Bluesky blob per-DID scoping ↔ the client-side-E2EE storage contract | E24/E42; `atproto-private-data-architecture.md` |
| E66 | Co-op low-maintenance service menu + "one encrypted sync service" | D5, E24, account kernel; §5j |
| E67 | Lightning/Nostr zaps for community value transfer (candidate, custody/legal-gated) | D5; §5j |
| E68 | Flat-cost group auth — passkey/DPoP/DBSC legs + TOTP MFA + JWKS mass-invalidation | `spike/auth-helper/`, account kernel, E60 |
| E69 | arecipe responsive filter/search UI playbook (261-source research) | arecipe cluster (E53–E59) |
| E70 | Skylite — borrow PWA patterns from TOKIMEKI / SkyFeed | Skylite; §5j |
| E71 | OPFS + File-System-Access "continuous export" — the honest correction | E62, E44 |

## Fact-check outcome (what changed)

Unusually accurate overall. The corrections baked into the items: **no native atproto E2EE / private-data**
(private plane needs a sidecar — MLS/Matrix/"Dark PDS"/WebRTC); **no `app.bsky.feed.hideReply` method**
(threadgate `hiddenReplies`); **no native threadgate "mutuals" rule** (compose following∩follower / a
`listRule`); **continuous-export folder-mirror is desktop-Chromium-only** (no iOS/Android; SW wiring
impossible); Privacy Pass = **RFC 9576/9577/9578** (not 9152); `PublicKeyCredential.supportsPrf` isn't real;
**iOS-18 hotspot client-IPv4-isolation = UNVERIFIED/forum-only**; Riseup pads 60d; DBSC shipped (Chrome 146);
`popover` = Baseline *Newly*; Path-50 ≠ Dunbar-150; MIMI = WG draft; `site.standard.*` real-but-community.

## Open decisions (surfaced, the user's to make — PLAYBOOK §5)

- **Private-comms / "private room" — RESOLVED out-of-scope (user 2026-07-27):** the private/E2EE plane is
  **Drystone's messaging plane** (`beta/drystone-spec/part-2-certifiable-design.md` §6 encryption stack /
  §6.1.2 MLS plane / §6.2.2 Layer-B `PrivateMessage` / §7.6.3 / §10.2). A *separate project leverages
  Drystone*; the forum does not build a sidecar. (Ties T7 + the beta MLS work.)
- **Payments (E67) — RESOLVED not-a-goal (user 2026-07-27):** parked as stretch discovery only; the
  ethical-relay business/custody/legal question is explicitly deferred.
- **Mutuals-only (clarified, user 2026-07-27):** possible as a *feed/read-view* (follows∩followers, client
  or feed-generator — such feeds exist in the wild); only network-enforced *reply*-gating by mutuals is not
  native (approximate via following+follower / a `listRule`). Not a blocker.
- **Non-pejorative framing** for "small world" (E63).
- **Downvote lexicon** publish-vs-local (E62); **RSS-CORS relay** vs the "$0/no-backend" framing (E62).
- **r=2 graph-resolution** strategy at scale (E62/E63).

## Status / next

Filed + committed to `main` (the batch filing commit, the atproto-mechanics-raw commit, and the
provenance-razor framing correction — resolve by commit message, not SHA, given the active history rewrite).
Backlog lives in ROADMAP_TODO §E (E62–E71). Natural next build homes: **E68** → `croft-stack/07-auth-helper.md`
+ `2026-07-22-account-kernel-spike.md`; **E69** → the arecipe repo; the rest remain `[explore]`/`[decision]`.
One hygiene follow-up (COHESION §60): the atproto-mechanics explainer is a deeper superset of
`atproto-architecture-appview-relay-explainer-2026-06-22.md` — consolidate into one atproto reference.
