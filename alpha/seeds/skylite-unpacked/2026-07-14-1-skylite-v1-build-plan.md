# Skylite v1 — build plan (for Claude Code execution)

`Status: proposed plan, pre-RUN-01. Grounded 2026-07-14 against: skylite repo (IDEAS.md,
CONCEPT.md, PROVENANCE.md), CroftCommunity/arecipe README (live), docs.bsky.app HTTP
reference + the getAuthorFeed lexicon (bluesky-social/atproto). Decision defaults are
flagged [confirm]; rule on those before handing RUN-01 to Claude Code.`

Repo: `CroftCommunity/skylite`. Domain: `skylite.croft.ing` (explicit DNS record —
the no-wildcard `*.croft.ing` policy stands). License: AGPL-3.0 (already in repo).

---

## 0. What v1 is, in one paragraph

A read-only, non-algorithmic, installable PWA window into Bluesky for one child.
The feed is a made thing: a guardian-curated inclusion list of accounts, each pulled
with `app.bsky.feed.getAuthorFeed`, merged newest-first, client-side, no server, no
account, no login. Guardian controls (pause switch, list edits, channel toggles) are a
record in the *guardian's* PDS, read publicly by the kid's device. Scrapbook is local.
No posting, no replying, no DMs, no metrics — by construction.

## 1. Grounded facts the plan stands on

- **Unauthenticated reads are supported by design.** The `app.bsky.feed.getAuthorFeed`
  lexicon states "Does not require auth" (bluesky-social/atproto, lexicon JSON), and
  docs.bsky.app directs unauthenticated `app.bsky.*` reads to
  `https://public.api.bsky.app`. Verified 2026-07-14.

- **Consequence:** v1 needs zero OAuth. Every hazard in IDEAS.md §5 (eviction shreds
  the badge, passkey ≠ re-login, public-client session ceilings) attaches to gated
  reads or to writes. v1 has neither. §5 is *deferred intact*, not resolved — it
  returns the day Skylite writes to a PDS or reads from a closed one.

- **arecipe is the proven template.** Backendless static PWA, vanilla strict
  TypeScript + esbuild, no framework, page-per-destination (no router), Vitest unit +
  Playwright e2e, a hermetic `npm test` gate in push CI and a separate `@live` tier
  run locally, GitHub Pages deploy via Actions, CNAME. Skylite copies this shape
  wholesale (arecipe README, live, verified 2026-07-14).

- **PDS `getRecord` is a stable public unauthenticated read** with a JSON envelope
  (established in the arecipe .ics work). JSON is exactly what a config poll wants,
  so the guardian-config-as-PDS-record design has its transport already verified.
  Browser CORS against PDS read endpoints is proven daily by arecipe itself.

- **Not grounded yet, carried as Phase-1 checks:** rate-limit posture of
  `public.api.bsky.app` for a polite polling client; exact CORS headers on
  third-party PDS `getRecord` (arecipe proves bsky-network hosts; others vary).

## 2. Decisions (defaults taken; rule before RUN-01)

**D1 — No kid account in v1.** The child has no DID, no login, nothing. The
did:plc / graduation / rotation-key endgame from IDEAS.md §1 is the *destination*
and shapes nothing in v1 except that we never build anything that would fight it.
`[confirm]`

**D2 — Guardian config lives in the guardian's PDS** as a custom record (lexicon
`ing.croft.skylite.config` under the guardian's repo), containing: pause flag,
inclusion list (DIDs + display names), channel groupings, settings version. Kid's
device is provisioned once with `guardianDid + rkey` (QR code or setup link), then
polls via public `getRecord`. Local cache of last-good config; see D5 for offline.
Fallback mode for guardians without a Bluesky account: local-only config with
export/import. `[confirm — this is the one genuinely new design in the plan]`

**D3 — Sky-Shield is out of scope for v1.** The CONCEPT.md §3 tension (AI
moderation vs. no-server spine) stands unresolved, as filed. v1 safety = the
inclusion-list ceiling + honoring ATProto moderation labels already present on
post views + no interaction surface. Label-honoring is cheap and real; do it.
`[confirm]`

**D4 — Scrapbook is local-only** (IndexedDB), per CONCEPT.md's own cross-check.
Clip post + private note. Survives offline; lost if the PWA is deleted — say so
plainly in the UI (anti-decoy stance). `[confirm]`

**D5 — Offline fails open for cached content, closed for staleness.** Everything
cached already passed the inclusion ceiling, so showing it offline is safe; show
the "saved posts, you're offline" banner. The pause flag is enforced on every
successful config poll, and if the config hasn't been reachable for N hours
(default 72) the garden locks until it is. This is the intentional wiring
IDEAS.md §4 demanded. `[confirm N]`

**D6 — Background lock is a local gate, not a credential.** `visibilitychange`/
`blur` locks the UI; unlock is a local check (PIN in v1; platform-authenticator
WebAuthn as a later nicety). Per IDEAS.md §5 this gate is a lock on Skylite's
door and nothing more — with no session behind it, that's now literally true.
`[confirm PIN-first]`

**D7 — External links are gated.** Tapping a link in a post shows a "this leaves
Skylite" interstitial; embedded link cards render domain + title only. The walled
garden shouldn't have unmarked doors. `[confirm]`

**D8 — Same-repo build.** Code lands in `CroftCommunity/skylite` alongside the
concept docs (arecipe pattern: docs + src in one repo). Idea-capture files stay
untouched at root; build adds `src/`, `plans/`, `tests/`, page HTML at root.
`[confirm]`

## 3. Phases

Each phase = one or more Claude Code runs; hermetic gate green at every merge;
phase acceptance is demonstrable in a browser against the built bundle.

### Phase 0 — Scaffold and pipeline
Toolchain mirroring arecipe: strict TS + esbuild, ESLint, Vitest, Playwright,
`npm test` hermetic gate (lint + typecheck + unit + build + e2e), build script
producing `dist/`, GitHub Actions push CI + Pages deploy, `CNAME skylite.croft.ing`,
web manifest (name Skylite, night-sky palette from CONCEPT.md §4), placeholder
icon set, an index page that renders a version stamp.
**Accept:** CI green; Pages serves the stamp at the custom domain after DNS lands.
**Manual follow-ups:** DNS record for `skylite`, Pages enablement, HTTPS enforce.

### Phase 1 — The garden (read path)
`getAuthorFeed` client over `public.api.bsky.app`; merge N author feeds
newest-first with cursor-aware pagination; post rendering (text, images with alt
text, video posters; label-bearing posts filtered/blurred per D3; links per D7);
inclusion list from a checked-in dev fixture for now; big bright Lite UI
(high-contrast, large text, no counts anywhere).
**Tests:** merge logic unit-tested against fixtures (fixture discipline per
arecipe PRACTICES); e2e renders a garden from mocked responses hermetically; one
`@live` smoke against the real AppView run locally only.
**Verify-in-phase:** rate-limit behavior; backoff + SW cache to stay polite.
**Accept:** a real three-account garden renders on an iPad from the deployed URL.

### Phase 2 — Guardian controls
The `ing.croft.skylite.config` record shape (versioned, documented in-repo);
guardian setup page (`guardian.html`) that reads/writes the record — this page,
used by the *guardian on their own device*, is where OAuth enters, scoped to the
guardian only and reusing arecipe's proven atproto OAuth client wiring; kid-device
provisioning (QR/link carrying `guardianDid + rkey`); config poll + pause
enforcement + D5 staleness lock; channel toggles as list groupings.
**Accept:** flipping pause in the guardian's PDS locks the kid's device on next
poll; toggling a channel changes the garden.

### Phase 3 — PWA hardening (the IDEAS.md §4 list, minus auth)
SW with `skipWaiting` + `clients.claim`, cache-busted assets, visible version
stamp (exists since Phase 0; now asserted in e2e); offline cache of posts +
blob images with the D5 banner; background lock (D6); `apple-touch-startup-image`
splash set, maskable icons, `theme-color`, status-bar style.
**Accept:** airplane-mode iPad shows the cached garden with banner; backgrounding
locks; a deployed update is picked up on next open with the new stamp visible.

### Phase 4 — Care features
Scrapbook (D4); the out-of-band "something's wrong" button (one tap → prefilled
`mailto:`/messaging handoff to the trusted adult — no platform reporting, no
telemetry); the "how Skylite works" honest view (v1 form of "your stuff": what
she sees and why, what the guardian can do, what is public — no repo of her own
exists yet, so it teaches the network's shape rather than listing records).
**Accept:** all three usable by a kid without help.

### Explicitly deferred (filed, not forgotten)
Kid's own DID + graduation ceremony (IDEAS.md §1, with its _verify_ list);
Sky-Shield (D3); co-viewing/cast; server-side Sky-Channels feed generators;
private family federation (IDEAS.md §6); HEIC canvas pipeline (only matters when
the kid can upload, i.e. post-v1); passkey-on-PDS question (§5 _verify_).

## 4. Claude Code run conventions (arecipe/discovery house rules)

- Fresh branch per run (`run-01-scaffold`, ...); no pushes to main; PR + merge
  after audit.

- Instruction file per run under `plans/`, dated arecipe-style; run writes
  `RUN-NN-SUMMARY.md`.

- Hermetic tier only in CI; `@live` never in push CI; no credentials in-repo.

- No new runtime dependencies without listing them in the summary with a reason;
  zero third-party origins at runtime except `public.api.bsky.app`, PDS hosts,
  and the Bluesky CDN for blobs — CSP written to exactly that allowlist.

- Verbatim-anchor guardrails when editing existing docs; stop rules: any
  ambiguity about a [confirm] item halts that item, not the run.

- Idea-capture files (README, CONCEPT.md, IDEAS.md, PROVENANCE.md, seeds/) are
  read-only for runs until a docs-restructure run says otherwise.

## 5. Open questions carried forward (unchanged from the repo)

Age band (6–9 vs 10–12) — affects UI sizing and copy, not architecture; the
boredom risk — v1 is the experiment that answers it; monetization — moot while
serverless; the IDEAS.md raw-transcript gap in PROVENANCE.md still wants the
arecipe session export checked.
