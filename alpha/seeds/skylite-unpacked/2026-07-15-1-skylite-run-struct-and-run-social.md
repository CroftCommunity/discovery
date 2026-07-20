# Skylite run package — RUN-STRUCT (structure & tiers) + RUN-SOCIAL (social layer)

`Status: instruction set for Claude Code. RUN-SOCIAL is gated on RUN-STRUCT merged.
Grounded 2026-07-15 against: live skylite.croft.ing, bsky.app mobile-web reference
screenshots (sponsor-provided), atproto.com/guides/scopes, docs.bsky.app OAuth docs,
bsky.social Terms of Service (13+ age floor). Supersedes the Phase 1/2 UI direction
in the v1 build plan where they conflict; all other v1 plan decisions stand.`

Repo: `CroftCommunity/skylite`. All prior run conventions apply (fresh branch per
run, hermetic gate green at merge, RUN summary, no new deps unlisted, idea-capture
files read-only, verbatim anchors, stop rules).

---

## Terminology ruling (applies everywhere)

**Sponsor** replaces guardian/custodian. **Explorer** replaces viewer/child/kid.
Rename sweep covers UI copy, page names/paths (`/sponsor`), code identifiers,
comments, and repo docs created by prior runs. Lexicon NSIDs do NOT change
(`ing.croft.skylite.config` etc. stay; renames live above the record layer).
The framing is deliberately role-based, not age-based: it must read correctly
for a tween, a mature teen, or a tech-averse elder.

## Tier model (per-explorer, set by sponsor in the config record)

`tier: "read" | "simple" | "full"`

- **read** — no account, no credentials on device. Discover (inclusion-list
  garden), scrapbook saves LOCAL-ONLY (mortal; UI says plainly they live on
  this device), share via native share sheet, get-help button. Simple skin
  forced. This is v1-current behavior, formalized.

- **simple** — explorer HAS an account (DID + scoped OAuth). Adds: likes
  (PDS records), saves as PDS records (persistent) with notes staying local,
  friends' likes visible. Tween-oriented skin: large type, get-help button.

- **full** — identical capabilities to simple. Presentation changes only:
  bsky.app-mobile-web-shaped skin, Skylite-branded, ecosystem links in menu.

Invariant: read→simple is a capability boundary; simple→full is skin only.
No code may gate a capability on the skin. No tier has any PDS-persisted
messaging; discussion is always the native share sheet (off-network).

Config record v2 (one record per explorer, rkey per explorer): displayName,
tier, inclusion list, channel groupings, friends[] (DIDs; used by simple/full),
pause flag, staleness window, schema version. Write a v1→v2 migration path in
the sponsor page (read old shape, offer upgrade write).

---

# RUN-STRUCT — structure, tiers, sponsor dashboard

## S1 Landing page and de-confusion
`/` becomes the product explainer: what Skylite is, the sponsor and explorer
roles, how a device gets set up; door to `/sponsor`, door to explorer-device
setup. Project/provenance docs (CONCEPT, IDEAS, PROVENANCE, seeds) demoted to
one small "about the project" footer link. Product surface and inside-baseball
must not share navigation.
**Accept:** a stranger landing on `/` understands the two roles and finds the
right door without seeing lexicon talk.

## S2 Sponsor multi-explorer dashboard
`/sponsor`: OAuth sign-in (granular scope limited to the config collection;
exact multi-action scope-string syntax = verify in-run against
atproto.com/guides/scopes). After sign-in, list all records in
`ing.croft.skylite.config` via listRecords → one card per explorer. Create /
edit / remove explorer configs. Per-explorer provisioning: QR + copyable link
carrying sponsorDid + rkey.
**Accept:** two explorer configs manageable side by side; a fresh device
provisioned from each QR shows that explorer's garden and tier.

## S3 Rename sweep
As ruled above. Grep-clean for guardian/custodian/viewer/child in UI strings,
paths, and identifiers (idea-capture files exempt, they are historical record).

## S4 Tier lever + skin architecture
One shared core (feed merge, config poll, filtering, scrapbook, pause/staleness)
behind two skins. Build the SIMPLE skin now (refine current UI: large type,
high contrast, get-help). FULL skin ships in RUN-SOCIAL. Tier read from config;
read tier fully functional end-to-end after this run.
**Accept:** flipping tier read↔simple in the sponsor dashboard changes explorer
behavior on next poll; capabilities provably keyed on tier, not skin.

## S5 Refresh that works
Native pull-to-refresh is unreliable in standalone PWAs and scroll containers,
so: explicit refresh control always visible in the feed header, plus a custom
pull gesture on the feed container. Both re-fetch feeds and re-poll config.
**Accept:** works installed-to-homescreen on iOS Safari, airplane-mode shows
the offline banner instead of a dead spinner.

## S6 Sponsor label-audit view
Two parts. (a) Meanings: fetch the label definitions published by the labelers
in play (Bluesky's default moderation service at minimum; exact
declaration/definition lexicon = verify in-run) and render name, description,
and what Skylite does with each. (b) Effectiveness: replay the exact garden
fetch+filter client-side on the sponsor's device over the fetched window and
show counts: posts hidden, per label, per account, with expandable examples.
Copy states plainly: this audits the filter using public data; nothing is
collected from the explorer's device.
**Accept:** sponsor can answer "what did the garden hide this week and why"
with numbers, per explorer config.

---

# RUN-SOCIAL — explorer accounts, likes, persistent saves, full skin
`Gate: RUN-STRUCT merged. Age gate documented, not solved: bsky.social ToS
requires 13+; under-age explorers stay tier=read until the family-PDS route
exists (explicitly deferred, again).`

## B1 Explorer identity (simple/full tiers)
Explorer-device OAuth using the arecipe-proven client wiring, granular scopes
ONLY for `ing.croft.skylite.like` and `ing.croft.skylite.save` (create/delete;
verify scope syntax in-run). Verify `sub` against the provisioned expectation.
Eviction/lapse behavior: degrade to read-tier behavior with a gentle "sign back
in to like and save" state — never a lockout, never a blocking modal for a
child.
**Accept:** kill the session artificially; garden still works; likes/saves
gray out with honest copy; re-auth restores.

## B2 Likes — the parallel catalogue
Lexicon `ing.croft.skylite.like`: subject strongRef + createdAt (mirror the
shape of app.bsky.feed.like so future conversion is a replay; document the
conversion as a spec note, do not build it). Like/unlike on posts. Friends'
likes: pull each friend DID's skylite.like collection via public listRecords
on their PDS and surface "liked by <friend>" in the garden. Friends come only
from the explorer's config (sponsor-curated).
**Honesty copy (required, verbatim intent not verbatim words):** friends see
these here; like everything on this network the records themselves are public.
**CSP note:** friend PDSes are arbitrary origins; widen connect-src as narrowly
as feasible (document the choice and tradeoff in the run summary) while
script-src stays locked.
**Accept:** two test accounts like posts; each sees the other's hearts; a
non-friend account's likes never surface.

## B3 Saves — persistent for simple/full
Lexicon `ing.croft.skylite.save`: subject strongRef + createdAt. Notes remain
LOCAL, keyed by the save's record URI (atproto has no private data; say so in
the UI: "your saves list is public like everything here; your notes never
leave this device"). Read tier keeps local-mortal saves unchanged. Offer
one-way import of existing local saves into PDS saves when a device upgrades
read→simple.
**Accept:** save on device A, appears on device B (same explorer); notes do
not cross devices; read-tier device unchanged.

## B4 Post-view page + share
One page rendering a single post inside the garden chrome (page-per-destination
model). Share button uses the Web Share API → native share sheet (iMessage /
Android Messages land free); payload = the skylite.croft.ing post link + text.
Works on ALL tiers (read included) since sharing needs no credentials.
**Accept:** share from an installed PWA opens the OS sheet; the shared link
opens inside Skylite, not bsky.app.

## B5 Full skin
Clone the bsky.app mobile-web layout (logged-out reference screenshots from
sponsor, 2026-07-15): feed card anatomy, bottom/side nav placement, hamburger
menu — Skylite-branded (night-sky palette), menu carries Skylite ecosystem
links (croft.ing, arecipe.app). Consult the frontend-design skill during
implementation. The point is graduation continuity: at 18 the walls come down
but the room looks the same. No engagement counts anywhere, same as always.
**Accept:** side-by-side with bsky.app mobile web, the layout reads as the
same family; all capabilities identical to simple skin.

## Deferred (filed, not forgotten)
Family PDS (now the explicit prerequisite for under-13 writes); the
skylite.like→app.bsky.feed.like conversion tool; co-sponsor (second DID);
Sky-Shield; feed generators.

## Verify-in-run ledger (both runs)
Granular multi-action scope syntax; labeler definition lexicon shape;
public listRecords behavior against non-bsky PDS hosts; CSP connect-src
strategy for arbitrary friend PDS origins.
