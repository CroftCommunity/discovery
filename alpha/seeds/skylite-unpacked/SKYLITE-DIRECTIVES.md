# SKYLITE-DIRECTIVES.md — consolidated Claude Code instruction set

`Status: EXECUTABLE. Supersedes: 2026-07-14-1 build plan (baseline, kept for
provenance), 2026-07-15-1 (tier ladder, dead), 2026-07-15-2 (package v2),
2026-07-15-3 (RUN-DISCOVER), 2026-07-15-4 (addendum A1). This file is the
single source of truth for the next three runs. Grounded 2026-07-15 against:
docs.bsky.app (public AppView, OAuth), atproto.com/guides/scopes,
bsky.social ToS (13+), bsky.social email-2FA support docs, live
skylite.croft.ing, sponsor-provided bsky.app mobile-web reference screenshots.`

Repo: `CroftCommunity/skylite`. Domain: skylite.croft.ing (live, Pages via
GitHub Actions, CNAME carried in the built artifact).

---

## 0. Standing conventions (every run, non-negotiable)

- **TDD, always.** Every phase begins by encoding its acceptance bullets as
  FAILING tests before any implementation: unit tests against checked-in
  fixtures for core logic; hermetic Playwright specs (mocked responses) for
  behavior. Fixtures are built before features. Run summaries must evidence
  red-to-green order per phase (test commit precedes or accompanies the first
  implementation commit). Named invariant tests written before the code they
  guard: `capabilities-key-on-localOnly-never-skin`, `label-floor-excludes`,
  `labeled-embed-never-renders`, `navigation-wall-blocks-embed-browsing`.
  Honest carve-out: visual/skin polish is covered by behavior specs plus
  owner review; do not write pixel-assertion theater.

- Fresh branch per run (`run-struct`, `run-social`, `run-discover`); no pushes
  to main; PR + merge after owner audit. One RUN-SUMMARY.md per run, listing
  any new dependencies with reasons and all verify-in-run findings.

- Hermetic tier only in CI (`npm test` = lint + typecheck + unit + build +
  hermetic e2e). `@live` smoke tests exist but never run in push CI. No
  credentials in-repo, ever.

- Runtime origins: exactly public.api.bsky.app, PDS hosts, and the Bluesky
  CDN. CSP written to that allowlist; where friend/feed PDS hosts are
  arbitrary, widen connect-src as narrowly as feasible and document the
  tradeoff in the run summary; script-src stays locked. Zero third-party
  origins otherwise.

- Idea-capture files (README.md, CONCEPT.md, IDEAS.md, PROVENANCE.md, seeds/)
  are read-only. Copy marked `[confirm before publish]` is carried verbatim;
  lay it out, never rewrite it. Stop rules: ambiguity on any [confirm] item
  halts that item only, recorded in the summary; the run proceeds.

- Toolchain (established): strict TypeScript + esbuild, no framework,
  page-per-destination, Vitest + Playwright, GitHub Actions Pages deploy.

## 1. Vocabulary and naming (all UI copy, retroactive sweep)

- Roles: **sponsor** (was guardian/custodian) and **explorer** (was
  viewer/child/kid). Role-based, never age-based; must read correctly for a
  tween, a teen, or a tech-averse elder. Grep-clean old terms from UI
  strings, paths, identifiers (idea-capture files exempt).

- Features: **Garden** (sponsor-curated feed), **My Sky** (explorer's own
  follows, its own tab), **Telescope** (discovery), **Saves** (plain word;
  sweep "scrapbook" from UI copy).

- Flavor rule: caught-stars language ("catch this star") sparingly, SIMPLE
  skin copy only, never a feature name, never in full skin.

- Lexicon NSIDs never rename: `ing.croft.skylite.config`,
  `ing.croft.skylite.like`, `ing.croft.skylite.follow`.

## 2. The two-switch model (tiers are dead)

Per-explorer, sponsor-set:

- **`localOnly` (boolean, DEFAULT TRUE).** True: no account, no credentials;
  device makes only anonymous public GETs; saves/notes/follows live locally
  with backup/restore; share button works. Privacy copy exactly scoped:
  "nothing about you leaves this device" (true: reads are anonymous, saves
  generate zero traffic). Never claim "talks to no one" (requests reveal
  ordinary metadata to fetched hosts). False ("sharing on"): explorer has an
  account; likes and PDS follows exist. Account creation is LAZY, at the
  moment this first flips off, never at provisioning.

- **`skin` ("simple" | "full").** Purely cosmetic. Simple: large type, high
  contrast, get-help button. Full: bsky.app-mobile-web-shaped, Skylite
  night-sky branding, ecosystem links (croft.ing, arecipe.app) in menu, no
  engagement counts anywhere. NO capability may key on skin.

- **Saves are local in EVERY mode.** No save record type exists, ever.
  Durability = backup/restore. Likes and follows are the only social objects.
  No mode has PDS-persisted messaging; discussion is the native share sheet.

Config record (one per explorer, RANDOM rkey, never a child's name):
displayName (nickname guidance enforced inline), localOnly, skin,
inclusion list, channel groupings, friends[] (DIDs, reciprocal,
sponsor-curated), showFriendsHearts (default false `[confirm]`),
approvedFeeds[] (feed URIs + cached names), telescope (default false),
showReposts (default true), pause, staleness window (default 72h), schema
version. Migration from the current deployed record shape ships in the
sponsor page. Schema carries NO age, birthday, school, or location fields,
ever. Follows are NOT in this record; they belong to the explorer.

## 3. Garden rendering principles (apply wherever posts render)

- Inclusion ceiling: garden = merged getAuthorFeed of the inclusion list,
  newest first, cursor-aware, no algorithm, no counts.

- Label floor: label-bearing posts are EXCLUDED (not blurred) everywhere:
  garden, My Sky, Telescope, embeds, post-view page.

- Quote/repost principle: outside content is eligible only via a garden
  author's act, label-floored identically (labels attach to the embedded
  quoted view; verify shape in-run). Quotes always render inline. The wall
  is navigational: an embed never opens casual browsing of the outside
  author's feed; the deliberate path in is follow-to-My-Sky. `showReposts`
  switch exists because reposts inject whole outside posts at volume; its UI
  explains that reasoning in plain words. Honest residual in sponsor copy:
  for outside authors, labels are the only safety layer.

- External links: interstitial ("this leaves Skylite"); link cards render
  domain + title only.

- Garden-change transparency: the explorer device diffs config polls locally
  and shows plain notices ("3 accounts were added to your garden"). Always
  on, not switchable. This is honesty toward the explorer and the
  sponsor-account-compromise tripwire.

- Pause: enforced on every successful config poll; if config unreachable
  beyond the staleness window, the garden locks until it is. Offline shows
  cached content with a banner.

---

# RUN-STRUCT

## S1 Landing content and role funnel
`/` = hero + one-switch explainer + two doors. Door A "I look after someone"
→ /sponsor (sign-in → dashboard → first-garden wizard → provisioning QR).
Door B "I was given a link or code" → explorer provisioning (scan/paste →
garden); a device arriving via provisioning link skips the landing. Footer:
"about the project" (demoted idea-capture docs), license. Product surface and
project docs never share navigation.

Copy v1, carry verbatim `[confirm before publish — every line]`:

> **Skylite**
> A window to the stars.
>
> A calm, read-first window into Bluesky, grown for you by someone who cares
> about you. No algorithm, no ads, no counts, no strangers.
>
> **How it works.** A sponsor tends a garden: the set of voices an explorer
> sees. Explorers read, save, and share what they find. One switch matters:
> "on this device only." While it is on, nothing about the explorer ever
> leaves the device. Turning it off, together, when the time is right, adds
> hearts that friends can see.
>
> [ I look after someone ]   [ I was given a link or code ]
>
> **Honesty, up front.** Gardens are public records, like everything on this
> network. Saves and notes never leave the device, ever. Hearts, when
> enabled, are public records shown among friends.

**Accept:** funnel walkable cold for both roles; copy byte-verbatim; a
stranger can state the two roles and the one switch after one screen.

## S2 Sponsor multi-explorer dashboard
/sponsor: atproto OAuth (arecipe-proven wiring; granular scope limited to the
config collection; multi-action scope syntax = verify in-run; app passwords
rejected everywhere). listRecords over the config collection → one card per
explorer; create/edit/remove; per-explorer provisioning QR + copyable link
(sponsorDid + rkey; carries no secrets by design). Public-record hygiene
(nicknames enforced with inline why, random rkeys, "this record is public"
note). Onboarding checklist (required step, plain words): enable bsky.social
email 2FA (what bsky.social offers today; TOTP/passkeys not shipped
upstream), harden the email inbox behind it, know the PDS OAuth-session
revocation page.
**Accept:** two explorers side by side; fresh devices provisioned from each
QR show the right garden and switches.

## S3 Rename and naming sweep
Per §1. Includes sweeping "scrapbook" to Saves in UI copy.

## S4 Switch plumbing + skins + rendering principles
Shared core (merge, config poll, filtering incl. §3 quote/repost and label
rules, saves, pause/staleness, transparency notices) behind two skins; build
SIMPLE now (full ships in RUN-SOCIAL). localOnly=true path complete
end-to-end. Invariant tests per §0 written first.
**Accept:** flipping either switch in the dashboard changes the device on
next poll; capability-vs-skin invariant proven by named test; labeled-embed
and navigation-wall fixtures pass.

## S5 Backup and restore (all explorers, all modes)
Export one versioned JSON (saves, notes, local follows, local settings) via
download/share sheet; import restores on a fresh device. Plain copy: what the
file contains; in localOnly mode it is the only copy.
**Accept:** export device A → import device B, Saves identical; works
installed on iOS.

## S6 Refresh that works
Always-visible refresh control in the feed header plus a custom pull gesture
on the feed container (native pull-to-refresh is unreliable in standalone
PWAs). Both re-fetch feeds and re-poll config.
**Accept:** works installed on iOS Safari; airplane mode shows the offline
banner, not a dead spinner.

## S7 Sponsor label-audit view
(a) Meanings: fetch label definitions from the labelers in play (Bluesky's
default moderation service at minimum; definition lexicon = verify in-run);
render name, description, and what Skylite does with each. (b) Effectiveness:
replay the exact garden fetch+filter on the sponsor's device over the fetched
window; counts of posts hidden per label per account, PLUS label-excluded
embeds as their own count, with expandable examples. Copy: this audits the
filter using public data; nothing is collected from the explorer's device.
**Accept:** "what did the garden hide this week and why" answered with
numbers, per explorer.

---

# RUN-SOCIAL (gate: RUN-STRUCT merged)

`Age gate documented, not solved: bsky.social ToS requires 13+; younger
explorers stay localOnly until the family-PDS route exists (deferred).`

## B1 Lazy explorer identity — BLOCKED on custody ruling
`STOP RULE: halt this phase until the owner confirms account custody.
Proposed defaults awaiting confirmation: sponsor holds the account email and
password (password manager); recovery/rotation key generated at creation and
stored OFFLINE by the sponsor (per the graduation story), handed over at
graduation; the explorer device holds ONLY the scoped OAuth session, never
the password.`
Triggered the first time localOnly flips off: account linking/creation, then
explorer-device OAuth, granular scope ONLY `ing.croft.skylite.like` +
`ing.croft.skylite.follow` (create/delete; verify syntax in-run). Verify
`sub` against the provisioned expectation. Session lapse/eviction degrades to
localOnly behavior with a gentle "sign back in to like" state; never a
lockout, never a blocking modal.
**Accept:** kill the session; garden still works; hearts/follows gray out
with honest copy; re-auth restores.

## B2 Likes
`ing.croft.skylite.like`: subject strongRef + createdAt, mirroring
app.bsky.feed.like (document 18-conversion as a replay; spec note, not
built). Like/unlike; explorer one-tap delete of her own records. Friends'
hearts via public listRecords on friend PDSes; friends only from config.
Honesty copy: friends see these here; the records themselves are public.
Lurk view: `showFriendsHearts` lets a localOnly explorer see friends' hearts
via anonymous reads (see-but-not-be-seen); default false `[confirm]`;
progressive disclosure (the sponsor toggle surfaces only when a friend DID
actually has a like collection), explanation ships with the toggle.
Documented stance in sponsor copy: the sponsor cannot delete the explorer's
records; remedies are conversation and pause; her repo is hers, that is the
point.
**Accept:** two test accounts exchange hearts; a non-friend's likes never
surface; localOnly device with the flag on sees hearts with no credential
existing.

## B3 Post-view page + native share
One page rendering a single post in garden chrome. Share button = Web Share
API → OS share sheet; payload = the skylite.croft.ing post link + text; works
in ALL modes. Shared links open inside Skylite, not bsky.app.
**Accept:** installed-PWA share opens the OS sheet; link round-trips into
Skylite.

## B4 Full skin
Clone the bsky.app mobile-web layout from the owner's reference screenshots
(feed card anatomy, nav placement, hamburger menu), Skylite night-sky
branding, ecosystem links in menu. Consult the frontend-design skill.
Graduation continuity is the point. No engagement counts anywhere.
**Accept:** side-by-side, the layout reads as the same family; capabilities
identical across skins (invariant test already guards this).

---

# RUN-DISCOVER (gate: RUN-SOCIAL merged)

## D1 My Sky
Follow/unfollow on any account encountered (garden, Telescope, post-view).
One-way; no notifications; never blended with friends. localOnly: local list,
in the S5 backup, visible to the sponsor only by holding the device. Sharing:
`ing.croft.skylite.follow` records (subject DID + createdAt, mirroring
app.bsky.graph.follow; 18-conversion spec note). My Sky tab uses the garden's
merge/render engine and the full §3 rules; visually distinct from the Garden
so curation and her choices stay legible as different things.
**Accept:** follow from Telescope appears in My Sky; unfollow removes;
localOnly follows appear in backup export; sharing follows appear in the repo.

## D2 Telescope rung 1 — approved windows
Discovery tab of the sponsor's approvedFeeds; each opens via
app.bsky.feed.getFeed with garden-grade rendering and the label floor.
Follow-from-here works. Credential-free, available regardless of localOnly.
**Accept:** an approved topic feed browses, filters labels, and a found
account lands in My Sky.

## D3 Telescope rung 2 — open search (sponsor-switched)
Visible only when telescope=true. Post + account search, label floor as
exclusion. Honest copy: the Telescope points at the whole open sky; garden
rules don't reach here beyond the label floor.
**EMPIRICAL FIRST, before any UI:** probe app.bsky.feed.searchPosts and
app.bsky.actor.searchActors unauthenticated against public.api.bsky.app,
including cursor pagination; record results in the summary. (Docs list
searchPosts as no-auth; a 403 has been reported unauthenticated; cursor
reportedly restricted.) If blocked or crippled: ship rung 1 only, leave rung
2 behind a build flag, file the finding; authenticated-search via PDS
proxying + rpc-type scopes is a FUTURE investigation, not this run.
**Accept (if unauth search works):** search returns label-floored results; a
known labeled fixture provably never renders.

## D4 Sponsor side
Feed picker (paste URL/URI with preview: name, author, sample posts; browse
only if an unauthenticated popular-feeds endpoint exists, verify in-run,
don't force it). Telescope switch per explorer, default off, plain copy.
My Sky visibility panel for sharing-mode explorers only (public listRecords),
framed as awareness-not-surveillance; localOnly explorers' follows are
explicitly NOT visible remotely and the panel says so.
**Accept:** feed added with preview; telescope on for one explorer, off for
the other; each device reflects its switches on next poll.

---

## Deferred ledger (filed, not forgotten)
Family PDS (prerequisite for under-13 sharing); like/follow → mainline
conversion tool; co-sponsor (second DID); Sky-Shield; Skylite-authored feed
generators (needs a server, conflicts with the serverless spine); passkey-on-
PDS question; authenticated-search fallback; algorithmic suggestions are
DELIBERATELY ABSENT forever, do not add them in any run.

## Verify-in-run ledger (consolidated)
Granular multi-action OAuth scope syntax (config; like+follow); labeler
definition lexicon shape; embedded-quote view label shape; public listRecords
against non-bsky PDS hosts; CSP connect-src strategy for arbitrary PDS
origins; unauthenticated searchPosts/searchActors incl. cursor;
unauthenticated popular-feeds endpoint; getFeed behavior when a generator is
offline (graceful copy); getAuthorFeed filter params for reply/repost
handling server-side vs client-side; public.api.bsky.app rate-limit posture
(polite backoff + SW caching); video/media playback from the Bluesky CDN in
the PWA (CSP, HLS support).

## Open [confirm] items (stop rules attached where noted)
1. S1 landing copy, line by line, before publish.
2. B1 account custody (BLOCKS B1; defaults proposed above).
3. showFriendsHearts default-false lurk view.
