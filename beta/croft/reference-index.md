# Reference index: "a composable garden of ponds and pads"

Every external reference the croft layer leans on, grouped by type, with a resolvable
locator where known, a relationship-plus-verification marker, and which croft doc relies
on it. Croft is product thinking, so the sources here are external products, tech, and
design references named in prose rather than academic papers. Much of croft's tech
grounding is one-homed in the sibling `cairn/`, `impl/`, and `drystone-spec/` layers; for
those the entry points at the sibling's source of record by name rather than
re-cataloguing it. This index captures what croft ITSELF asserts, plus those pointers.

Marker key:

Relationship (how croft treats the reference):

BUILD-ON = croft builds directly on the thing.

POND = an external social ecosystem croft connects to as a pond (native shape kept end to end).

WRAP / PORT = the inclusion-pathway relationship for a pad (webxdc-compat shim, or WebRTC-onto-iroh polyfill).

ADOPT-PATTERN = croft adopts the pattern or discipline, not necessarily the artifact or framework.

CONSTRAINT = a platform limit croft designs around rather than a thing it uses.

ANTI-PATTERN = a negative example croft's stance refuses.

Verification:

PRIMARY = the artifact itself (project repo, spec, vendor documentation, published audit).

VENDOR-DOCUMENTED = a vendor's own platform docs; volatile, re-checked at build time.

FACTCHECK = an iroh / atproto fact whose source of truth is the FACTCHECK; not re-verified here.

DIALOGUE-SOURCED = originated in design dialogue; carries a `[confirm]` against a primary before external use.

ONE-HOMED-IN-SIBLING = the source of record lives in a named sibling-layer doc; pointed to here, not re-catalogued.

NOT-YET-LOCATED = named in croft prose (or in the task framing behind it) but no resolvable locator captured this session.

---

## Substrate & tech it builds on

**iroh** (QUIC transport, relay, iroh-gossip, browser-peer relaying, hole-punching). Locator: FACTCHECK source of truth (iroh core 1.0.0); one-homed in cairn `iroh-app-pond-building-blocks.md` and impl `transport-iroh-gossip-and-quic.md`. BUILD-ON. FACTCHECK / ONE-HOMED-IN-SIBLING. The transport under every pond and pad, the tiered web-exposure model, and the pad security bar's WebRTC-containment win. Relied on by all three docs (`product-the-garden-of-ponds.md` §6–§8, `presence-ritual-and-composed-ponds.md` tiered-exposure, `social-graph-as-substrate.md` implicitly via the shared log).

**atproto / `at://` references / custom lexicons**. Locator: atproto facts cite the FACTCHECK source of truth; ecosystem one-homed in cairn `atproto-ecosystem.md` and `atproto-nsid-and-lexicon-mechanics.md`. BUILD-ON. FACTCHECK / ONE-HOMED-IN-SIBLING. Grounds the Bluesky pond, the cross-pond awareness that resolves an `at://` reference to a renderable card, and game-outcome-as-custom-lexicon (its own record type, public and resolvable but rendering only where a client knows the lexicon). `product-the-garden-of-ponds.md` §4 (awareness), `presence-ritual-and-composed-ponds.md` (outcome record).

**Bluesky**. Locator: bsky.app / atproto; one-homed in cairn atproto docs. POND. ONE-HOMED-IN-SIBLING. Named pond example. `product-the-garden-of-ponds.md` §Overview.

**Fediverse via the Mastodon client API**. Locator: Mastodon client API (docs.joinmastodon.org/client). POND. PRIMARY (locator inferred from the named API; not explicitly cited in prose). Named pond example, connected through the Mastodon client API rather than a fused model. `product-the-garden-of-ponds.md` §Overview, §2.

**Lemmy** (forum over ActivityPub). Locator: join-lemmy.org; one-homed in cairn (ActivityPub field). POND. ONE-HOMED-IN-SIBLING. Named pond example; kept a separate pond from the microblog fediverse despite shared ActivityPub substrate ("shared substrate underneath is an implementation detail beneath the seam"). `product-the-garden-of-ponds.md` §Overview, §2.

**ActivityPub**. Locator: W3C ActivityPub Recommendation (w3.org/TR/activitypub); one-homed in cairn. BUILD-ON (honest-seams rationale). ONE-HOMED-IN-SIBLING. The load-bearing assertion croft makes on it: "ActivityPub deliberately has no canonical data shapes, so there is nothing to normalize against even if fusion were wanted." `product-the-garden-of-ponds.md` §2.

**webxdc** (the wrap pathway: a webxdc-compatible API surface — `sendUpdate`, `setUpdateListener`, `joinRealtimeChannel` — plus the webxdc broadcast layer). Locator: webxdc.org; the wrappable game catalog (adbenitez / ArcaneCircle) is one-homed in cairn `iroh-app-pond-building-blocks.md`. WRAP (via a webxdc-compat shim backed by the iroh transport). PRIMARY / ONE-HOMED-IN-SIBLING. `product-the-garden-of-ponds.md` §6.

**Browser WebRTC API / raw WebRTC / `RTCPeerConnection`**. Locator: W3C WebRTC (w3.org/TR/webrtc). PORT (polyfill the browser WebRTC API onto a direct iroh QUIC stream, mock the signaling handshake) and CONSTRAINT (disabled inside the pad webview, since the transport is iroh QUIC, not browser WebRTC). PRIMARY. `product-the-garden-of-ponds.md` §6 (port pathway), §8 (security bar).

**Cure53 webxdc security audit** ("CSP alone does not contain a webview"; the WebRTC / DNS-prefetch exfiltration finding). Locator: Cure53 / OpenTechFund report; one-homed in cairn `iroh-app-pond-building-blocks.md` §Security. ADOPT-PATTERN (the security lesson) / ONE-HOMED-IN-SIBLING. The governing lesson behind the whole pad security bar. `product-the-garden-of-ponds.md` §8.

**Content-Security-Policy (CSP)**. Locator: W3C CSP Level 3 / MDN. CONSTRAINT (the mechanism the Cure53 audit shows is insufficient on its own). PRIMARY. `product-the-garden-of-ponds.md` §8.

**commit-reveal** (the fair-reveal leverage primitive). Locator: standard cryptographic commitment scheme (generic primitive; no single canonical spec). BUILD-ON / ADOPT-PATTERN (built once as a reusable module powering governance-grade voting, dice fairness, and hidden-info games). DIALOGUE-SOURCED (design intent, not yet proven in code). `product-the-garden-of-ponds.md` §6.

**MLS** (the composition edge — "shared MLS lineage"). Locator: RFC 9420 (Messaging Layer Security), RFC 9750; one-homed in cairn `mls-and-mimi.md` and `drystone-spec/`. BUILD-ON (named here only to locate the composition/valuation boundary). ONE-HOMED-IN-SIBLING. Croft carries only the *valuation* edge itself (directional, weighted, keyless); the *composition* edge (MLS) is deferred to the substrate doc and the spec. `presence-ritual-and-composed-ponds.md` §edges.

**Games & media pad building blocks** (libmarathon, ascii-royale, iroh-lan, godot-iroh, GGRS + matchbox, netplayjs, Curvytron, boardgame.io, sendme / DataBeam, callme / iroh-roq, iroh-live / MoQ, str0m). Locator: all one-homed in cairn `iroh-app-pond-building-blocks.md`, each carried there with its reuse / port / wrap / reject call and license flags. ONE-HOMED-IN-SIBLING. Croft points at this set through the three inclusion pathways and the calls pond; it does not re-catalogue the parts. `product-the-garden-of-ponds.md` §6, `presence-ritual-and-composed-ponds.md` (calls / game outcome).

**Bond Touch (and kin)** (consumer "thinking-of-you" bracelets). Locator: bondtouch.com; one-homed in cairn `iroh-app-pond-building-blocks.md` §anti-pattern. ANTI-PATTERN / ONE-HOMED-IN-SIBLING. The negative example the free, no-account, undiscontinuable thinking-of-you ping directly rebukes ("~fifty bytes over the wire" wrapped in an account, a business, and a cloud relay). DIALOGUE-SOURCED `[confirm]`. `presence-ritual-and-composed-ponds.md` §Presence & Ritual.

**Delta Chat** ("the Delta-Chat shape" — the thread-bottomed product). Locator: delta.chat. ANTI-PATTERN (negative structural example). PRIMARY (product named in prose). The thread-at-the-bottom category error the social-graph-as-substrate reframe corrects structurally. `social-graph-as-substrate.md` §What this dissolves.

**Google+ Circles** (the extractor-built social graph that died). Locator: the discontinued Google+ product (shut down 2019). ANTI-PATTERN / prior-art (the motivating illustration). PRIMARY (product named in prose). The canonical example that every real-life social-graph tool built by an extractor felt invasive and died — Croft's evidence that a graph-you-hold is genuinely novel, not a feature gap. Companion prior-art in the same argument: the *shadow profile* (the graph built about a person without consent), a general concept, not a single source. `social-graph-as-substrate.md` §Why a graph you hold does not already exist.

## Platform constraints

**Cold-install deferred-deeplink is not privately achievable**. Croft asserts this abstractly: "the seamless-deferred mechanisms that would enable it are gone or require fingerprinting," forcing the claim-code "one-more-tap" flow. The specific named mechanisms behind that assertion (Android Instant Apps ended; Firebase Dynamic Links shut down; iOS App Clips iOS-only) are the underlying facts and are NOT named in croft's own prose. CONSTRAINT. DIALOGUE-SOURCED / NOT-YET-LOCATED — the specific mechanism names, shutdown dates, and platform scope want a resolvable locator before external use; croft ITSELF asserts only the abstract version. `product-the-garden-of-ponds.md` §6 (the tier-zero deep-link resolver).

**On-device model coverage is a patchwork** (the optional assistant's substrate). Croft asserts four tiers: strong on capable Apple devices with token-level guided generation; steeply device-gated on Android flagships; real but desktop-only and flag-gated in Chrome; effectively absent in Firefox and mobile web. Locators split:
- **Apple Foundation Models** (~3B on-device, `@Generable` guided generation) and **Google Gemini Nano** (AICore + ML Kit GenAI) are one-homed in cairn `iroh-app-pond-building-blocks.md` §on-device-AI. BUILD-ON (optional). VENDOR-DOCUMENTED / ONE-HOMED-IN-SIBLING.
- **Chrome built-in AI** (desktop-only, flag-gated) is croft-asserted and NOT catalogued in cairn. Locator: Chrome built-in AI / Prompt API (developer.chrome.com/docs/ai). CONSTRAINT. VENDOR-DOCUMENTED / NOT-YET-LOCATED (wants a confirmed locator).
- **Firefox / mobile web absence** ("bundled-WASM-or-nothing") is a croft-asserted platform fact. CONSTRAINT. DIALOGUE-SOURCED.
All are volatile ("specific model sizes and device lists move monthly and are re-checked at build time"). `product-the-garden-of-ponds.md` §7.

**Browsers cannot hole-punch** → a browser peer's traffic is relayed for the entire session ("a complete relay broker, not setup-and-handoff"), reaching browsers via WebTransport for broadcast media. Locator: iroh browser-peer facts cite the FACTCHECK source of truth; WebTransport is W3C (w3.org/TR/webtransport). CONSTRAINT. FACTCHECK / DIALOGUE-SOURCED `[confirm]`. Drives the Tier-2 logged-in-browser-peer design and the run-your-own-relay operational consequence. `presence-ritual-and-composed-ponds.md` §iroh tiered-exposure.

## Design & architecture references

**Functional-core / imperative-shell**. Locator: the named pattern (Gary Bernhardt, "Boundaries," 2012). ADOPT-PATTERN. PRIMARY (established pattern). The single most important structural decision — a pure synchronous `(state, intent) -> (state, effects)` core emitting effects as data, the shell performing I/O. Green-real in Phase 0. `product-the-garden-of-ponds.md` §3, §4.

**Crux**. Locator: github.com/redbadger/crux (pre-1.0). ADOPT-PATTERN (adopt the shape slim, not the framework — "adopting a pre-1.0 framework whole would inherit churn behind a port anyway"). PRIMARY. "This is the shape Crux ships in production." `product-the-garden-of-ponds.md` §3.

**Design tokens** (the quality-bar discipline: "a design token is a named design decision"; token-contract tests plus per-state snapshots). Locator: the design-token concept (W3C Design Tokens Community Group; general practice). ADOPT-PATTERN (design discipline: "nothing ships placed by eye"). PRIMARY (established discipline). `product-the-garden-of-ponds.md` §5.

**Mature super-app design guidance** (the shared pond/pad criteria set, "with the host-as-gatekeeper mechanics stripped out"). Locator: super-app design guidance (e.g. WeChat mini-program design guidelines) — the specific source is not named in croft prose. ADOPT-PATTERN (adapt the criteria, remove the gatekeeper mechanics). DIALOGUE-SOURCED / NOT-YET-LOCATED — the guidance source is unnamed and wants a locator before external use. The granular design bar now drawn from it lives in `product-the-garden-of-ponds.md` §5: the five-principle checklist, the depth-of-three exit-trap ("the documented super-app failure case"), the roughly 7-9mm touch-target floor, the feedback taxonomy (auto-dismiss for success, persistent-never-flashed for errors, result view for completion), one-animation-per-view, and accessibility-as-first-class ("the world's largest app" treats elderly-friendly design as a named, mandatory, separate concern). `product-the-garden-of-ponds.md` §5.

---

## Empirical proofs (sibling Proofs/ repo)

**Proof: social-layer visibility regimes (S2a structure-leaks-identity / S2b scoped-consent)**, `Proofs/alpha/lineage-group-model/SOCIAL_LAYER_FINDINGS.md`. PRIMARY (empirical model; all nine visibility invariants pass). The source of record for the substrate-safety result in `social-graph-as-substrate.md` §Structure leaks identity: S2a models the canonical town-of-4,000 attack and shows the target's anonymity set collapses to one, making "structure without names" unrepresentable (a `never`-returning constructor); S2b shows the only constructible share is a per-distance consent map that carries no topology field. Cited by name here, not by cross-repo path.

---

## Coverage note: where croft's dependencies' sources of record live

Croft is the product layer; its tech grounding is deliberately thin on primary catalogues because those are one-homed in sibling layers. Read this index together with:

- **cairn `iroh-app-pond-building-blocks.md`** — the source of record for the games/media pad blocks, the wrappable webxdc catalog, the Cure53 security audit, the two on-device-AI models (Apple Foundation Models, Gemini Nano), and the Bond Touch anti-pattern. Croft consumes these; cairn credits and verifies them.
- **cairn atproto docs (`atproto-ecosystem.md`, `atproto-nsid-and-lexicon-mechanics.md`)** — the atproto / lexicon grounding under the Bluesky pond and the custom-lexicon outcome record.
- **cairn `mls-and-mimi.md` and `drystone-spec/`** — the MLS composition edge that sits beside croft's valuation edge.
- **impl `transport-iroh-gossip-and-quic.md`** — iroh's transport mechanics (RoQ/MoQ, relay, gossip) under the tiered-exposure model.
- **the FACTCHECK source of truth** — all iroh / atproto version and capability facts (iroh core 1.0.0); not re-verified here.

`the-helper-tier-and-the-baseline-floor.md` (added 2026-07-22) introduces **no new external references**:
it is a product-principle synthesis grounding in the neutral-spec principles (peer equality, durable
enablement, exitability — one-homed in `../drystone-spec/`) and in the account-kernel spike (K1/KC0,
provenance in `../../alpha/spike/account-kernel/`). Its named helpers (AppView, login broker, relay,
delivery service, etc.) are each catalogued at their own homes (ECOSYSTEM, cairn, the appview-infra
experiments); it points at them rather than re-cataloguing.

`build-order-and-ponds-roadmap.md` (added 2026-07-22) introduces **no new external references**: it
sequences and catalogues activities over the blocks already credited above (iroh, webxdc + the wrappable
catalog, the Cure53 audit, GGRS + matchbox and the other pad blocks, commit-reveal / fair-reveal, the
event-sourced store shape, atproto custom lexicons, functional-core / imperative-shell). Its candidate
ponds (music-guessing, collaborative card) and the account-kernel substrate are dialogue-sourced 2026-07-22
and carry `[confirm]` / gate flags via `../OPEN-THREADS.md` T55-T58; they are not asserted as settled here.

What this index captures that is croft's OWN: the pond selections (Bluesky, Mastodon-client-API fediverse, Lemmy) and the honest-seams reading of ActivityPub; the inclusion-pathway relationships (webxdc wrap, WebRTC-onto-iroh port); the architecture and design references it adopts (functional-core / imperative-shell, Crux, design tokens, super-app criteria); the social-graph prior-art it argues from (Google+ Circles as the canonical corpse, the shadow profile); the empirical social-layer visibility proof that grounds the structure-leaks-identity result; and the platform constraints it designs around (cold-install deferred-deeplink loss, the on-device-model patchwork, the browser hole-punch limit).

Most-flagged items to resolve before external use: the deferred-deeplink mechanism names and dates (Instant Apps / Firebase Dynamic Links / App Clips) are NOT-YET-LOCATED and asserted only abstractly by croft; the super-app design-guidance source is unnamed (NOT-YET-LOCATED); the Chrome built-in AI locator wants confirmation; and the dialogue-sourced Presence & Ritual product calls (the thinking-of-you ping, the browser-peer relay model) carry `[confirm]` against a primary.
