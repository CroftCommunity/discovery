# atproto NSID and Lexicon mechanics: the Smoke Signal worked example

`Status: deep dive, current as of mid-2026. NSID mechanics and Smoke Signal schema/tech-stack facts are drawn from primary-ish sources (repo READMEs, schema JSON, official docs) per their confidence below. Two items are explicitly flagged unresolved and are not settled: the h3/hthree naming tension and the "self-limiting AppView" definition. This is the deep dive behind the one-line Smoke Signal row in atproto-ecosystem.md, not a duplicate of that landscape table.`

## Overview

atproto's Lexicon system needs a global naming scheme so that any schema (a record type, an XRPC endpoint, and more) can be referred to unambiguously across the network. That naming scheme is the NSID, the Namespaced Identifier. It is a small mechanism with a large amount of load-bearing behaviour riding on it, and it carries a real gap between what the name looks like and what it actually guarantees.

This doc does two things. First, it walks the NSID mechanics: structure, why the reverse-DNS binding matters, a naming-rule tension the source flags as unresolved, and the limit of the metaphor (owning an NSID does not make a schema fetchable). Second, it uses Smoke Signal, an atproto events and RSVP app, as the worked example: a real product whose two-generation schema migration and Rust tech stack make the abstract NSID rules concrete.

## NSID mechanics

**What it is.** An NSID is atproto's global naming scheme for Lexicon schemas: record types, XRPC endpoints, and more. Examples: `com.atproto.sync.getRecord`, `app.bsky.feed.post`, `community.lexicon.calendar.event`.

**Structure.** An NSID is a fully-qualified hostname written in reverse domain-name order, plus one additional name segment on the end. Take `community.lexicon.calendar.event`: the domain authority is `community.lexicon.calendar` (that is `calendar.lexicon.community` reversed), and the name is `event`. The domain-authority segments must form a valid hostname when reversed; the final name segment is an ASCII camel-case string (hence `getRecord` and `fooBarV2`, never hyphenated forms). A valid NSID needs at least three segments; `com.example` alone is invalid.

**The AT-URI shape and record keys.** Records themselves are addressed as AT-URIs of the form `at://<did>/<collection-nsid>/<rkey>`. The collection segment of that URI is exactly an NSID (for example `events.smokesignal.calendar.event`), which is how a naming scheme for schemas becomes the addressing scheme for concrete records in a repository. The `<rkey>` (record key) in the Smoke Signal schemas is a TID: a timestamp-based key, auto-generated rather than semantic. A live example cited from the source: `at://did:plc:cbkjy5n7bk3ax2wplmtjofq2/events.smokesignal.calendar.event/3l5movzhkwk2w`.

**Why reverse-DNS is load-bearing, not cosmetic.** The reverse-DNS form ties schema-naming authority to domain ownership. Only whoever controls `calendar.lexicon.community` can legitimately author schemas under that reversed name. Using a subdomain in the authority requires controlling the full domain: `com.atproto.sync.getRecord` requires control of `sync.atproto.com`, not merely `atproto.com`. The stakes of this binding are concrete. If the `smokesignal.events` domain were lost, a bad actor who acquired it could break every record validating against `events.smokesignal.*`. That risk is exactly why a community-governed namespace exists, and why there is pressure toward DID- or record-based resolution as a hedge against domain loss.

**The naming-rule tension (flagged unresolved).** The name segment's prose rule allows ASCII letters and digits, forbids hyphens, and forbids a digit as the first character. On its face that prose appears to permit a letter-first name that contains a digit; the spec's own examples include `fooBarV2` and `cn.8.lex.stuff`. Yet `community.lexicon.location.h3` was reportedly rejected by a validator and renamed to `hthree`. The likely reconciliation offered is that the reference regex is stricter than the prose describes, but this was not authoritatively resolved. Treat it as an open tension between the spec's prose and its reference implementation, to be settled by direct testing against a validator (for example `goat lex`) rather than trusted from either the prose or the reasoning above.

**The metaphor's limit (also load-bearing).** Owning an NSID does not require the schema to be fetchable at that domain. Per the spec as quoted in the source: there is currently no automated mechanism for verifying control of a domain authority, none for fetching a Lexicon schema given an NSID, and none for enumerating all NSIDs under a base domain. So an NSID asserts authority by convention; historically nothing enforced or resolved it. That is the gap a separate Lexicon-resolution effort exists to close. The name looks like a URL but behaves more like a claimed label than a working link.

## Smoke Signal, the worked example

**What it is.** Smoke Signal (`smokesignal.events`) is an events and RSVP platform built on atproto: users create events, and others RSVP going, interested, or not-going. It was built by Nick Gerakines (Dayton, Ohio), launched around September 2024, then relaunched and open-sourced around its one-year anniversary (about July 2025).

**Founding motivation.** In Gerakines' words: "I started Smoke Signal because users should have ownership and control of the content that they create." He built del.icio.us at Yahoo and contributed to early Web 2.0 sites (upcoming.org, Flickr), and watched Upcoming get shut down after Yahoo's acquisition. That is the stated origin of his focus on data portability; he credits atproto with getting data portability right where ActivityPub and ActivityStreams did not.

**Architecture.** Events and RSVPs live in the user's own PDS as atproto records, not in a Smoke-Signal-owned database. The platform runs its own AppView to aggregate and display them, but if Smoke Signal disappeared the records would remain in each user's repository. The platform indexes a record only by its AT-URI; it does not store the record content itself. A "self-limiting AppView" concept was mentioned in a July 2025 talk, but its precise definition and enforcement mechanism were not retrieved; treat it as undefined, an open item pending a direct look at the talk.

**The Lexicon Community governance effort.** Gerakines drove a broader Lexicon initiative beyond this single app: bookmarks, events, locations, and reactions lexicons, developed through a self-governed Lexicon Community that is independent from Bluesky Social PBC, with a volunteer technical steering committee holding final authority on technical direction, governance, and infrastructure. The event and RSVP schemas are meant to be a shared, community-governed vocabulary another app could read, not Smoke-Signal-proprietary.

**The two-generation storage-path migration.** The schemas moved from an app-owned namespace to the community-governed one, which is the practical demonstration of why domain-bound naming authority matters. All record-type lexicons here use a TID record key (auto-generated, not semantic).

Original namespace:

- `events.smokesignal.calendar.event` (the app-owned first generation)

Community namespace (what the ecosystem is converging on), verified from the raw schema JSON in the `lexicon-community/lexicon` repo:

- `community.lexicon.calendar.event` (record, key tid)

- `community.lexicon.calendar.rsvp` (record, key tid; required fields `subject` and `status`, where `subject` is the AT-URI of the event it points at, the cross-PDS graph link)

- `community.lexicon.bookmarks.bookmark` (record, key tid)

- `community.lexicon.location.address` (object, not a record, so no own AT-URI or collection; embedded as a field value inside an event)

- `community.lexicon.location.geo` (object)

- `community.lexicon.location.hthree` (object; renamed from `h3`, per the naming tension flagged above)

- `community.lexicon.location.fsq` (object, Foursquare Places)

**The tech stack (with a correction preserved).** Verified from the repo README and BUILD config (hosted on Tangled, atproto's git-hosting project):

- **Language:** Rust, edition 2024, minimum version 1.90.

- **Database:** Postgres via the `sqlx` toolkit (`DATABASE_URL=postgres://...`, `sqlx migrate run`). This corrects an earlier same-session claim of SQLite, which was wrong; the actual config shows Postgres. The correction is preserved here as a grounding-discipline example: a stack detail asserted from memory was overturned by reading the config, and the record keeps the correction rather than smoothing it over.

- **Cache/session:** Redis; session encryption via a 64-character hex cookie key.

- **Templating:** development mode with template reloading; production mode with embedded templates behind an `embed` feature flag.

- **Blob storage:** pluggable, a filesystem path or an S3 URL via `CONTENT_STORAGE`.

- **Auth:** full atproto OAuth with two backends, a direct PDS backend or AIP (ATProtocol Identity Provider, also Gerakines' own project), an OAuth 2.0 / OpenID Connect service for atproto identities, described in the July 2025 talk as letting you "set up an OIDC provider that authenticates via PDS OAuth."

- **Identity:** service identity via a `did:web` service key; configurable PLC hostname (default `plc.directory`); an admin-DID allowlist.

- **Hosting/source:** open source on Tangled; a Discourse forum linked via AIP for atproto login.

**Two honest gaps left open.** The HTTP framework was not confirmed (axum is likely given the Rust ecosystem and a `:3000` default port, but this was not asserted without seeing Cargo.toml). The "self-limiting AppView" concept from the July 2025 talk was not defined. Both remain open, not settled.

## Relevance to Drystone/cairn

NSID is the direct atproto analogue of the question Drystone's own naming and identity discipline has to answer: how do you name a schema and bind naming authority to something verifiable. Drystone's vocabulary/naming discipline and its DID-adjacent identity model both touch that seam, so how atproto resolved it (reverse-DNS authority) and where it left a gap are both directly instructive.

The reverse-DNS-authority-without-fetchability gap is the concrete cautionary data point. atproto ties naming authority to domain ownership but has no automated mechanism to verify that control, fetch a schema from an NSID, or enumerate a domain's schemas. Any scheme that wants naming authority to also imply resolvability should treat that gap as a warning: authority by convention is not the same as a working, resolvable link, and the two can drift apart until a separate resolution effort closes the distance. The Smoke Signal migration from `events.smokesignal.*` to `community.lexicon.*` is the same lesson from the other side, moving authority off a single owned domain and onto a governed namespace precisely to de-risk the domain-loss failure mode.

## What this establishes (and does not)

Establishes a grounded reference for atproto's NSID mechanics (structure, the reverse-DNS authority binding, the AT-URI shape, TID record keys) and a worked example (Smoke Signal) that makes those rules concrete through a real two-generation schema migration and a verified Rust/Postgres/Redis stack. Establishes the direct relevance of the reverse-DNS-authority-without-fetchability gap to Drystone's naming and identity discipline.

Does not resolve the two flagged-open items: the h3/hthree naming tension (prose appears to permit a letter-first name containing a digit, yet `h3` was rejected and renamed) is left open pending a validator test, and the "self-limiting AppView" concept is left undefined pending a direct look at the July 2025 talk. Does not confirm Smoke Signal's HTTP framework (axum is a guess, not asserted). Does not duplicate the ecosystem landscape table in atproto-ecosystem.md; it is the deep dive behind that one-line row.
