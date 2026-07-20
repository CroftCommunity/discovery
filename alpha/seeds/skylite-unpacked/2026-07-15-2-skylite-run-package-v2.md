# Skylite run package v2 — RUN-STRUCT (structure & switches) + RUN-SOCIAL (social layer)

`Status: instruction set for Claude Code. Supersedes 2026-07-15-1 (tier-ladder
version). RUN-SOCIAL is gated on RUN-STRUCT merged. Grounded 2026-07-15 against:
live skylite.croft.ing, bsky.app mobile-web reference screenshots
(sponsor-provided), atproto.com/guides/scopes, docs.bsky.app OAuth docs,
bsky.social Terms of Service (13+ age floor). All prior v1-plan decisions stand
where not amended here.`

Repo: `CroftCommunity/skylite`. All prior run conventions apply (fresh branch per
run, hermetic gate green at merge, RUN summary, no new deps unlisted, idea-capture
files read-only, verbatim anchors, stop rules).

---

## Terminology ruling (applies everywhere)

**Sponsor** and **explorer**. Rename sweep covers UI copy, page names/paths
(`/sponsor`), code identifiers, comments, and repo docs from prior runs. Lexicon
NSIDs do NOT change. Framing is role-based, not age-based: it must read correctly
for a tween, a mature teen, or a tech-averse elder.

## The two-switch model (replaces the tier ladder — tiers are dead)

Per-explorer config carries two independent switches:

**Switch 1 — `localOnly` (boolean, DEFAULT TRUE).**
While true: the explorer has no account and no credentials; the device makes
only anonymous public GETs (feeds, images); saves and notes live locally with
backup/restore; the share button works (needs no credentials). Copy claim,
exactly scoped: "nothing about you leaves this device" — true because reads
are anonymous and saves generate zero traffic. Do NOT claim "talks to no one":
public reads still reveal ordinary request metadata (IP) to the hosts fetched,
like any website.
While false ("sharing on"): the explorer has an account; likes exist and are
visible to friends. Account creation and OAuth happen LAZILY at the moment
this switch is first turned off — never at provisioning.

**Switch 2 — `skin` ("simple" | "full").** Purely cosmetic. Simple: large type,
high contrast, get-help button. Full: bsky-shaped (RUN-SOCIAL B4). No code may
gate any capability on skin — capabilities key on `localOnly` alone.

**Saves are local in EVERY mode.** There is no save record type, no
`ing.croft.skylite.save` lexicon, no public reading list, ever. Durability and
device moves = backup/restore (S5). Likes are the only social object.

No mode has any PDS-persisted messaging; discussion is always the native share
sheet (off-network).

Config record v2 (one record per explorer, rkey per explorer): displayName,
localOnly, skin, inclusion list, channel groupings, friends[] (DIDs),
showFriendsHearts (boolean, default false — see B2 note), pause flag,
staleness window, schema version. Migration path from the v1 record shape in
the sponsor page.

---

# RUN-STRUCT — structure, switches, sponsor dashboard

## S1 Landing page and de-confusion
`/` becomes the product explainer: what Skylite is, sponsor and explorer roles,
the one meaningful choice (on-this-device-only vs sharing), how a device gets
set up; door to `/sponsor`, door to explorer-device setup. Project/provenance
docs (CONCEPT, IDEAS, PROVENANCE, seeds) demoted to one small "about the
project" footer link.
**Accept:** a stranger landing on `/` understands two roles and one switch and
finds the right door without seeing lexicon talk.

## S2 Sponsor multi-explorer dashboard
`/sponsor`: OAuth sign-in (granular scope limited to the config collection;
exact multi-action scope-string syntax = verify in-run against
atproto.com/guides/scopes). After sign-in, listRecords over
`ing.croft.skylite.config` → one card per explorer. Create / edit / remove
configs. Per-explorer provisioning: QR + copyable link carrying
sponsorDid + rkey.
**Accept:** two explorer configs manageable side by side; a fresh device
provisioned from each QR shows that explorer's garden and switches.

## S3 Rename sweep
As ruled. Grep-clean for guardian/custodian/viewer/child in UI strings, paths,
identifiers (idea-capture files exempt — historical record).

## S4 Switch plumbing + skin architecture
One shared core (feed merge, config poll, filtering, scrapbook, pause and
staleness) behind two skins; build/refine SIMPLE now, FULL ships in RUN-SOCIAL.
`localOnly=true` end-to-end complete after this run. Flipping either switch in
the sponsor dashboard changes the explorer device on next poll.
**Accept:** capabilities provably keyed on localOnly, never on skin (test
asserts it).

## S5 Backup and restore (all explorers, all modes)
Export: one file (versioned JSON) containing saves, notes, and local explorer
settings, produced via download/share sheet (AirDrop-able). Import on a fresh
device restores it. Plain copy about what the file contains and that it is the
only copy in localOnly mode.
**Accept:** export on device A, import on device B, scrapbook identical;
works installed-to-homescreen on iOS.

## S6 Refresh that works
Explicit refresh control always visible in the feed header, plus a custom pull
gesture on the feed container (native pull-to-refresh is unreliable in
standalone PWAs). Both re-fetch feeds and re-poll config.
**Accept:** works installed on iOS Safari; airplane mode shows the offline
banner, not a dead spinner.

## S7 Sponsor label-audit view
(a) Meanings: fetch label definitions published by the labelers in play
(Bluesky's default moderation service at minimum; exact definition lexicon =
verify in-run) and render name, description, and what Skylite does with each.
(b) Effectiveness: replay the exact garden fetch+filter client-side on the
sponsor's device over the fetched window; show counts of posts hidden per
label per account, with expandable examples. Copy states plainly: this audits
the filter using public data; nothing is collected from the explorer's device.
**Accept:** sponsor can answer "what did the garden hide this week and why"
with numbers, per explorer.

---

# RUN-SOCIAL — sharing mode, likes, share button, full skin
`Gate: RUN-STRUCT merged. Age gate documented, not solved: bsky.social ToS
requires 13+; younger explorers stay localOnly until the family-PDS route
exists (explicitly deferred, again).`

## B1 Lazy explorer identity
Triggered the first time a sponsor flips localOnly off: account
linking/creation flow, then explorer-device OAuth using the arecipe-proven
client wiring, granular scope ONLY for `ing.croft.skylite.like`
(create/delete; verify syntax in-run). Verify `sub` against the provisioned
expectation. Session lapse/eviction degrades to localOnly behavior with a
gentle "sign back in to like" state — never a lockout, never a blocking modal.
**Accept:** kill the session; garden still works; hearts gray out with honest
copy; re-auth restores.

## B2 Likes — the parallel catalogue
Lexicon `ing.croft.skylite.like`: subject strongRef + createdAt (mirror
app.bsky.feed.like's shape; document future conversion as a replay — spec
note, not built). Like/unlike on posts. Friends' hearts: pull each friend
DID's skylite.like collection via public listRecords on their PDS; surface
"liked by <friend>". Friends come only from the explorer's config.
**Honesty copy (required):** friends see these here; like everything on this
network, the records themselves are public.
**Lurk view `[confirm]`:** `showFriendsHearts` lets a localOnly explorer see
friends' hearts via anonymous public reads (see-but-not-be-seen). Build it,
default false, sponsor-controlled; flipping it on requires no account. Copy
notes the scoped metadata reality (requests go to friends' PDS hosts).
**CSP note:** friend PDSes are arbitrary origins; widen connect-src as
narrowly as feasible and document the tradeoff in the run summary; script-src
stays locked.
**Accept:** two test accounts like posts; each sees the other's hearts; a
non-friend's likes never surface; localOnly device with the flag on sees
hearts without any credential existing.

## B3 Post-view page + native share
One page rendering a single post inside garden chrome (page-per-destination).
Share button on every post uses the Web Share API → OS share sheet (iMessage /
Android Messages land free); payload = the skylite.croft.ing post link + text.
Works in ALL modes including localOnly — sharing a link needs no credentials.
This is the off-network discussion path; keep it one tap.
**Accept:** share from an installed PWA opens the OS sheet; the shared link
opens inside Skylite, not bsky.app.

## B4 Full skin
Clone the bsky.app mobile-web layout (logged-out reference screenshots,
2026-07-15): feed card anatomy, nav placement, hamburger menu — Skylite-branded
(night-sky palette), menu carries Skylite ecosystem links (croft.ing,
arecipe.app). Consult the frontend-design skill during implementation.
Graduation continuity is the point: at 18 the walls come down but the room
looks the same. No engagement counts anywhere, same as always.
**Accept:** side-by-side with bsky.app mobile web the layout reads as the same
family; capabilities identical across skins.

## Deferred (filed, not forgotten)
Family PDS (prerequisite for under-13 sharing); skylite.like →
app.bsky.feed.like conversion tool; co-sponsor (second DID); Sky-Shield;
feed generators.

## Verify-in-run ledger (both runs)
Granular multi-action scope syntax; labeler definition lexicon shape; public
listRecords behavior against non-bsky PDS hosts; CSP connect-src strategy for
arbitrary friend PDS origins.
