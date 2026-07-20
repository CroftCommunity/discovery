# Skylite RUN-DISCOVER — Telescope, My Sky, and naming

`Status: instruction set for Claude Code. Gated on RUN-STRUCT and RUN-SOCIAL
(package v2, 2026-07-15-2) merged. Amends v2 copy where noted; all v2 decisions
otherwise stand. Grounded 2026-07-15: docs.bsky.app HTTP reference (searchPosts
listed as not requiring auth) AGAINST a reported unauthenticated 403
(bsky-docs issue #332) and a report of unauthenticated queries working except
cursor pagination — unauthenticated search is UNCERTAIN and is resolved
empirically in D4, not assumed.`

Repo: `CroftCommunity/skylite`. All standing run conventions apply.

---

## Naming ruling (applies to all UI copy, this run and retroactively)

- **Garden** — the sponsor-curated feed (unchanged).
- **My Sky** — the explorer's own followed accounts, rendered as its own tab.
- **Telescope** — discovery (both rungs below).
- **Saves** — the save feature, plain word, everywhere. "Scrapbook" was a
  working name; sweep it from UI copy (repo docs/history exempt).
- Flavor rule: caught-stars language ("catch this star") may appear sparingly
  in the SIMPLE skin's copy only, never as a feature name, never in full skin.

## Config record v3 (amends v2; migration in sponsor page)

Adds per explorer: `approvedFeeds[]` (feed-generator URIs + cached display
names), `telescope` (boolean, DEFAULT FALSE — enables open search),
`schema version` bump. Follows are NOT in the config record — they belong to
the explorer (local list, or her own repo in sharing mode).

---

## D1 My Sky — explorer follows
Follow/unfollow action on any account encountered (garden posts, Telescope
results, post-view page). One-way; no notification to anyone; friends remain
a separate, sponsor-curated, reciprocal concept — do not blend them.

- localOnly: follows are a local list (included in the S5 backup file). The
  sponsor sees them only by looking at the device; no remote copy exists.

- sharing mode: follows are `ing.croft.skylite.follow` records (subject DID +
  createdAt, mirroring app.bsky.graph.follow's shape; document 18-conversion
  as a replay — spec note, not built). Honest copy: these are public records,
  shown here as yours. Scope addition: the explorer OAuth scope grows to
  include this collection (create/delete; verify syntax in-run).

My Sky tab: same merge/render engine as the garden, sourced from the follow
list, same label floor. The garden and My Sky are visually distinct tabs, not
blended, so the sponsor's curation and the explorer's own choices stay legible
as different things.
**Accept:** follow from a Telescope result appears in My Sky; unfollow removes
it; localOnly device shows follows in backup export; sharing device shows the
record in the repo.

## D2 Telescope rung 1 — approved windows
Discovery tab listing the sponsor's `approvedFeeds`. Each opens as a feed view
via `app.bsky.feed.getFeed` (public AppView), rendered with garden-grade post
rendering and the label floor (exclusion, not blur). Follow-from-here works.
Credential-free; available regardless of localOnly.
**Accept:** an approved crochet-type feed browses, filters labels, and an
account found there lands in My Sky.

## D3 Telescope rung 2 — open search (sponsor-switched)
Visible only when `telescope=true` for this explorer. Post search and account
search. Label floor as exclusion. Honest copy: the Telescope points at the
whole open sky; the sponsor's garden rules don't reach here beyond the label
floor.
**Empirical first step (before building UI):** probe
`app.bsky.feed.searchPosts` and `app.bsky.actor.searchActors` unauthenticated
against public.api.bsky.app, including cursor pagination; record results in
the run summary. If unauthenticated search is blocked or crippled: ship rung 1
only, leave rung 2 behind a build flag, and file the finding (a possible
authenticated-search path via PDS proxying + rpc-type scopes is a FUTURE
investigation, not this run).
**Accept (if unauth search works):** search "crochet" returns label-floored
posts; a label-bearing post provably never renders (test with a known labeled
fixture).

## D4 Sponsor side
- Feed picker: add approved feeds by pasting a feed URL/URI, with a preview
  before saving (feed name, author, sample posts). If a public
  popular/suggested-feeds endpoint is available unauthenticated, offer browse;
  otherwise paste-only is fine (verify in-run, don't force it).

- Telescope switch per explorer, default off, with plain copy about what it
  opens.

- My Sky visibility panel: for sharing-mode explorers only, show their follow
  records (public listRecords). Framing copy matters: visible because these
  records are public, shown for awareness, not surveillance; localOnly
  explorers' follows are explicitly NOT visible remotely and the panel says so.
**Accept:** sponsor adds a feed, flips telescope for one explorer and not the
other; each device reflects its own switches on next poll.

## Deferred (filed, not forgotten)
Authenticated-search fallback investigation (rpc scopes / PDS proxying);
feed-generator authorship (Skylite publishing its own curated topic feeds —
would need a server component, conflicts with the serverless spine, park it);
follow-suggestion surfaces (deliberately absent: no algorithmic suggestions
anywhere in Skylite).

## Verify-in-run ledger (this run)
Unauthenticated searchPosts/searchActors incl. cursor; unauthenticated
popular-feeds endpoint availability; granular scope syntax for the follow
collection; getFeed behavior for feeds whose generator is offline (graceful
failure copy).
