# Raw transcript: Pixelfed-to-atproto backfill, backdating semantics, Smoke Signal, and NSID mechanics, 2026-07-08

`Provenance caveat (PLAYBOOK §4): content-faithful cleaned paste, not a byte-pristine export. A Gemini/web
research session grounding several atproto mechanics: cross-protocol content backfill (Pixelfed export
format, the atproto write path, createdAt backdating), the tooling landscape for that migration, the
backdated-post labeler as a moderation-primitive example, and a deep dive on Smoke Signal (an atproto
events app) and NSID (Namespaced Identifiers). Multiple claims in the source are self-flagged [UNVERIFIED]
or corrected mid-session (a self-correction on speculating an author's motive, a stack-detail correction
from SQLite to Postgres); those flags are preserved below, not smoothed over. One gap: the session opened
by asking about stream.place (https://stream.place/) but the transcript excerpt available jumps directly
into the Pixelfed-backfill answer without ever circling back to stream.place, so nothing about that site is
captured here; do not infer content for it from this transcript.`

## Part A: Pixelfed-to-atproto one-time backfill

**The goal:** copy an existing Pixelfed post history into an atproto (Bluesky) profile once, statically, no
ongoing link, dated to the originals.

**The Pixelfed export gotcha (the load-bearing finding).** Pixelfed's "export" is multiple separate JSON
files (`account.json`, `outbox.json`, likes, bookmarks), not one archive. Critically, there is no image
data in the export, only URLs to where assets are currently hosted; if the source instance goes down, the
archive is useless because no image bytes are included. `outbox.json` is your source of truth (posts as
ActivityStreams objects) but posts arrive as single items with thread relationships lost, so multi-post
threads flatten (fine for a photo diary, not for reply chains). A community script exists to download
photos from a Pixelfed export JSON, but its author notes it fails if `outbox.json` can't be generated,
which happened historically for accounts over 500 statuses (dated late 2024, may since be fixed, check
against the live instance). Do not confuse this with Instagram-to-Pixelfed import guides, which are the
opposite direction and irrelevant.

**The atproto write path (grounded, well-documented, simpler than the export side).** A post is
`app.bsky.feed.post`, requiring `text` and `createdAt`; `createdAt` is a plain client-supplied timestamp
settable to any past date (this is the backdating capability). Images: upload each via
`com.atproto.repo.uploadBlob` (returns a blob ref with CID, mimeType, size), then reference the blob(s) in
an `app.bsky.embed.images` embed, each image carrying its own alt text. Hard limits: four images per post,
~1MB per image (docs give conflicting numbers, 1MB vs 2MB, so target ~1MB safe); EXIF should be stripped
before upload (strongly recommended, may be enforced more strictly later). No dedicated photo-native
lexicon was found that beats `app.bsky.feed.post` for timeline rendering; target that lexicon.

**The build order (grounded end to end):**

1. While the source instance is up, pull `outbox.json` and `account.json`; confirm no status-count ceiling
   truncated it.
2. Download every image from the URLs in `outbox.json` locally now (irreplaceable step, URLs die with the
   instance).
3. Per post, transform: caption to post text, `published` date to `createdAt` (the original date), prepare
   image(s).
4. Preprocess images: strip EXIF, resize/recompress under ~1MB, split or drop extras if a post has more
   than four images.
5. Upload blobs via `uploadBlob`, capture blob refs.
6. Write the record via `createRecord`/`applyWrites` as `app.bsky.feed.post` with the embed and backdated
   `createdAt`.
7. Reconcile via `com.atproto.repo.listMissingBlobs` so nothing referenced gets garbage-collected.

Result: the profile carries the photo history on original dates, sorted chronologically in the author feed,
honestly marked as archived (see the labeler in Part B), fully within supported mechanisms, no
system-fooling.

**Honesty flags carried in the source:** the 500-status ceiling and exact `outbox.json` structure come from
community writeups dated late 2024/2023, corroborated on "URLs not bytes" but not re-verified against the
current Pixelfed version; pull your own export and inspect one post object before writing a parser against
a year-old description.

## Part B: does tooling exist to do this? (the gap, mapped)

**What exists, and why none of it is a direct fit:**

- **`mastodon-to-bluesky`** (a maintained CLI): reads a Mastodon outbox, downloads media, splits long posts
  into threads, posts to Bluesky, with incremental transfer/dry-run/retry. Architecturally close to the
  needed pipeline, but its README states plainly that Bluesky "doesn't support backdating posts," so it
  stamps the current date/time and appends `[Originally posted: ...]` as text. That directly defeats
  date-preservation. It is also tuned to Mastodon's shape, not Pixelfed's photo-centric objects.

- **Crossposting/sync tools** (`mastodon-bluesky-sync`, MicroPoster): continuous ongoing mirrors, the wrong
  mode entirely for a one-time backfill, and none backdate.

- **Bounce** (from A New Social / the Bridgy Fed team): cross-protocol *account migration* between
  ActivityPub (Mastodon, Pixelfed named explicitly) and atproto. Two disqualifiers: at launch it only
  supported Bluesky-to-Mastodon/Pixelfed (the reverse of the needed direction), and even the later
  Mastodon-to-Bluesky path is explicit that "your content will not come with you," it moves the social
  graph, not posts.

- **Bridgy Fed:** bridges live traffic between AP and atproto; does not backfill history. Bridging Pixelfed
  specifically was reported as finicky by at least one user.

**The gap, stated plainly:** nobody has published a tool that is source=Pixelfed AND mode=one-time-backfill
AND dates=preserved, together. Each constraint individually has a tool; not the combination.

**Two realistic routes:** (1) fork `mastodon-to-bluesky` (already does outbox-reading, media download,
four-image handling, thread-splitting; the key edit is pointing it at Pixelfed's export shape and setting
`createdAt` to the original date instead of appending text, which is enabling a capability the tool's
author chose not to use, not hacking around a block); or (2) write a fresh script given the Pixelfed export
is idiosyncratic anyway. Idempotency (track already-written post IDs to survive reruns) and PDS write
rate-limit pacing both matter in either route.

## Part C: the self-correction on tool-author motive (a grounding discipline example)

The session initially speculated the Mastodon-importer author declined to backdate because he "believed
backdating wasn't supported, or wanted to avoid the archived-post treatment." On being challenged
("confirm we are right... and both"), the session separated the two claims and retracted the unsupported
half:

- **Claim 1 (the protocol/platform honors a past `createdAt`): CONFIRMED**, triple-sourced: the docs state
  clients may insert any value for `createdAt`, enabling import/migration, with `sortAt` defined as the
  earlier of `createdAt`/`indexedAt`; a real user reported bulk-loading old Twitter/Facebook history into
  Bluesky with original dates, producing a profile diary back to 2003; a dedicated "port tweets to Bluesky
  maintaining original date" tool exists (a Show HN posting).

- **Claim 2 (the Mastodon tool opts out by choice): CONFIRMED** from its own README ("Bluesky doesn't
  support backdating posts" is the stated, and factually incorrect, reason given).

- **The retracted part:** the extra speculative motive ("wanted to avoid archived-post treatment") had no
  source and was withdrawn. The corrected, sourced statement is narrower: the tool doesn't backdate because
  its author stated (incorrectly) that the platform doesn't support it. The capability existing and the
  tool declining to use it are both true and are not in tension.

## Part D: the backdated-post labeler, `@backdate.mozzius.dev`

A labeler running on the atproto network flags posts where the record's self-reported `createdAt` doesn't
match when the post first appeared on the firehose, i.e. it operationalizes the one check that catches
backdating and publishes the result as a subscribable label. Grounded reasons this kind of tool exists
(category-level, not the specific operator's stated motive, which the session declined to assert without a
source): backdating is invisible by default in the UI, and it enables a named scam pattern (create an
account whose posts *appear* to predict past events like sports scores or stock prices, using timestamps
from before those events, then monetize the fabricated track record; this was raised explicitly in the
comments under the tweet-porting-tool's Show HN). Bluesky's own docs flag the far-future-timestamp variant
as a possibility for "shenanigans," so a labeler is a natural fit for atproto's user-side moderation
primitive (labelers). The session explicitly declined to assert why this *specific* labeler operator built
it (serious anti-scam tool vs demonstration vs a bit), citing no source, while standing behind the
category-level reason. Design-relevant framing for an honest-archival goal: a Pixelfed backfill would
likely get flagged as backdated by such a labeler, which is the truthful "this was imported, not organic"
signal, a feature for this use case, not a defect; it further confirms the network's stance is "backdating
allowed, but detectable," not "blocked."

## Part E: Smoke Signal (smokesignal.events), an atproto events/RSVP app

**What it is.** An events and RSVP platform built on atproto: users create events, others RSVP
going/interested/not-going. Built by Nick Gerakines (Dayton, Ohio), launched ~September 2024, relaunched
and open-sourced around its one-year anniversary (~July 2025).

**Founding motivation.** Gerakines: "I started Smoke Signal because users should have ownership and control
of the content that they create." He built del.icio.us at Yahoo and contributed to early Web 2.0 sites
(upcoming.org, Flickr), and watched Upcoming get shut down after Yahoo's acquisition, the stated origin of
his focus on data portability; he credits atproto with getting data portability right where
ActivityPub/ActivityStreams did not.

**Architecture.** Events and RSVPs live in the user's own PDS as atproto records, not in a Smoke-Signal-
owned database; the platform runs its own AppView to aggregate and display them, but if Smoke Signal
disappeared the records remain in each user's repository. A "self-limiting AppView" concept was mentioned
in a July 2025 talk but its precise definition/enforcement mechanism was not retrieved in this session;
treat as undefined pending a direct look at the talk.

**The Lexicon Community effort.** Gerakines drove a broader lexicon initiative beyond just this app:
bookmarks, events, locations, and reactions lexicons, developed through a self-governed Lexicon Community
independent from Bluesky Social PBC, with a volunteer technical steering committee holding final authority
on technical direction, governance, and infrastructure. The event/RSVP schemas are meant to be a shared,
community-governed vocabulary another app could read, not Smoke-Signal-proprietary.

**Governance/land acknowledgment note.** The name draws from Indigenous language/concepts; the developers
and Gerakines' conference talks acknowledge the app was built in the traditional homelands of the Myaamia
and Shawnee people, forcibly removed.

### Smoke Signal's storage paths (two generations, verified from the schema files)

Records live as atproto records in the user's PDS, indexed by the platform only as an AT-URI (the platform
does not store the record content itself). An AT-URI has the shape `at://<did>/<collection-nsid>/<rkey>`.
All record-type lexicons here use `"key": "tid"` (a timestamp-based record key, auto-generated, not
semantic).

- **Original namespace:** `events.smokesignal.calendar.event` (a live example cited:
  `at://did:plc:cbkjy5n7bk3ax2wplmtjofq2/events.smokesignal.calendar.event/3l5movzhkwk2w`).

- **Community namespace** (what the ecosystem is converging on), verified from the raw schema JSON in the
  `lexicon-community/lexicon` repo:
  - `community/lexicon/calendar/event.json` to `community.lexicon.calendar.event` (record, key tid)
  - `community/lexicon/calendar/rsvp.json` to `community.lexicon.calendar.rsvp` (record, key tid, required
    fields `subject` + `status`; `subject` is the AT-URI of the event it points at, the cross-PDS graph link)
  - `community/lexicon/bookmarks/bookmark.json` to `community.lexicon.bookmarks.bookmark` (record, key tid)
  - `community/lexicon/location/address.json` to `community.lexicon.location.address` (object, not a
    record, so no own AT-URI/collection; embedded as a field value inside an event)
  - `community/lexicon/location/geo.json` to `community.lexicon.location.geo` (object)
  - `community/lexicon/location/hthree.json` to `community.lexicon.location.hthree` (object; renamed from
    `h3` because a validator rejected the digit-only-looking segment even though the NSID prose rules
    appear to permit a letter-first name containing a digit, an unresolved tension between the spec's prose
    and its reference regex, flagged not authoritatively resolved in-session)
  - `community/lexicon/location/fsq.json` to `community.lexicon.location.fsq` (object, Foursquare Places)

### Smoke Signal's tech stack (corrected mid-session; the correction is preserved)

Verified from the repo README and BUILD config (hosted on Tangled, atproto's git-hosting project):

- **Language:** Rust, edition 2024, minimum version 1.90.
- **Database:** Postgres via the `sqlx` toolkit (`DATABASE_URL=postgres://...`, `sqlx migrate run`). **This
  corrects an earlier same-session claim of SQLite**, which was wrong; the actual config shows Postgres.
- **Cache/session:** Redis; session encryption via a 64-character hex cookie key.
- **Templating:** development mode with template reloading; production mode with embedded templates behind
  an `embed` feature flag.
- **Blob storage:** pluggable, filesystem path or S3 URL via `CONTENT_STORAGE`.
- **Auth:** full atproto OAuth, two backends: a direct PDS backend, or **AIP** (ATProtocol Identity
  Provider, also Gerakines' own project), an OAuth 2.0 / OpenID Connect service for atproto identities,
  described in the July 2025 talk as letting you "set up an OIDC provider that authenticates via PDS OAuth."
- **Identity:** service identity via a `did:web` service key; configurable PLC hostname (default
  `plc.directory`); admin-DID allowlist.
- **Hosting/source:** open source on Tangled; a Discourse forum linked via AIP for atproto login.
- **Feature set claimed:** event management with timezone/location support, RSVP with status/validation, a
  ticket-based RSVP acceptance system, private event content gated on RSVP status, email notifications,
  profile caching.

**Two honest gaps left open in-session:** the HTTP framework was not confirmed (axum is likely given the
Rust ecosystem and a `:3000` default port, but this was explicitly not asserted without seeing Cargo.toml);
the "self-limiting AppView" concept from the July 2025 talk was not defined.

## Part F: NSID (Namespaced Identifier)

**What it is.** atproto's global naming scheme for Lexicon schemas (records, XRPC endpoints, and more),
e.g. `com.atproto.sync.getRecord`, `app.bsky.feed.post`, `community.lexicon.calendar.event`.

**Structure.** A fully-qualified hostname in reverse domain-name order, plus one additional name segment.
For `community.lexicon.calendar.event`: the domain authority is `community.lexicon.calendar` (i.e.
`calendar.lexicon.community` reversed), and the name is `event`. The domain-authority segments must form a
valid hostname reversed; the final name segment is an ASCII camel-case string (hence `getRecord`,
`fooBarV2`, never hyphenated forms).

**Why reverse-DNS (load-bearing, not cosmetic).** It ties schema ownership to domain ownership: only
whoever controls `calendar.lexicon.community` can legitimately author schemas under that reversed name. A
concern surfaced in-session: if the smokesignal.events domain were lost, a bad actor gaining it could break
every record validating against `events.smokesignal.*`, which is exactly why a community namespace (and
pressure toward DID/record-based resolution as a hedge) exists. Using a subdomain in the authority requires
controlling the *full* domain (e.g. `com.atproto.sync.getRecord` requires controlling `sync.atproto.com`,
not merely `atproto.com`).

**The naming rules, and where the source flags a genuine tension.** The name segment's prose rule allows
ASCII letters and digits, forbids hyphens, and forbids a digit as the first character; an NSID needs at
least three segments minimum (`com.example` alone is invalid). The session flagged an unresolved seam: the
prose appears to permit a letter-first name containing a digit (the spec's own examples include
`fooBarV2` and `cn.8.lex.stuff`), yet `community.lexicon.location.h3` was reportedly rejected by a
validator and renamed to `hthree`; the likely reconciliation offered is that the reference regex is
stricter than the prose describes, but this was not authoritatively resolved and is flagged for direct
testing against a validator (e.g. `goat lex`) rather than trusted from either the prose or the session's
reasoning.

**The metaphor's limit (also load-bearing).** Owning an NSID does not require the schema to be fetchable at
that domain. Per the spec as quoted in-session: no automated mechanism for verifying control of a domain
authority currently exists, nor for fetching a lexicon schema for a given NSID, nor for enumerating all
NSIDs for a base domain. So the NSID asserts authority by convention; historically nothing enforced or
resolved it, which is the gap a separate lexicon-resolution effort exists to close. The name looks like a
URL but behaves more like a claimed label than a working link.

Design relevance for Drystone/cairn: NSID is the direct atproto analogue of "how do you name a schema and
bind naming authority to something verifiable," a question Drystone's own vocabulary/naming discipline and
its DID-adjacent identity model both touch; the reverse-DNS-authority-without-fetchability gap is a
concrete cautionary data point for any scheme that wants naming authority to also imply resolvability.
