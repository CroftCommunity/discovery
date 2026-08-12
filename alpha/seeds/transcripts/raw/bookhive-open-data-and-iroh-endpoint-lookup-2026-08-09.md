# BookWyrm/BookHive open-data ladder, then the PDS→iroh endpoint lookup and croftcall (Claude, 2026-08-09)

- **Source:** a claude.ai dialogue the owner pasted into a Claude Code session in `CroftC/` on
  2026-08-09. Two distinct bodies in one paste: (A) an open-data comparison of book-review sites
  (BookWyrm / BookHive / Open Library), and (B) the origin dialogue for **croft-relay + the croftcall
  exchange page + the Android client**.
- **Preservation status:** **preserved-condensed (cleaned-paste, content-faithful — §4).** Owner
  turns near-verbatim; assistant turns content-faithful with UI chrome stripped (collapsed
  thinking-summary headers, "Ran a command", citation chips `BookHive` / `GitHub`, and the trailing
  artifact cards `Croft relay plan · Document · MD`, `Croft exchange · Code · HTML`, `Croftcall
  android · ZIP`, `Croft call build instructions · Document · MD`).
- **DUPLICATION:** body (B) — from *"I want to use a PDS lexicon for an Iroh account lookup"* through
  the build-instructions turn — appeared **twice, verbatim** in the paste. Reproduced **once**, per
  the no-triplication rule. This is the same paste-duplication pattern seen in the 2026-07-27
  croft-landscape intake.
- **Credentials:** none present; none redacted.
- **The four deliverables named here are filed separately and verbatim** as
  `seeds/callpds-unpacked/` (from `callpds.zip`) — referenced, not re-embedded, per §4.
- **Verification:** body (A) claims are `[UNVERIFIED]` unless noted (see the per-claim notes).
  Body (B)'s mechanism claims are **superseded in part** — see the supersession note at the end,
  which is the most important thing in this file.

---

## Part A — the open-data ladder for book reviews

### A1. The question

The owner pasted a description of **BookWyrm** ("the most popular direct, open-source replacement
for Goodreads… ad-free, decentralized book tracking built on ActivityPub, part of the Fediverse")
and asked:

> Explain how this site and federation works

**Assistant.** Yes — and the AT Protocol one exists: **BookHive**. Worth explaining, because
atproto's architecture answers almost every complaint raised about BookWyrm.

BookHive is a Goodreads alternative on the AT Protocol. You sign in with an existing Bluesky
account; every review, tracked book, and shelf is stored on atproto in an open portable format
accessible to other apps; it imports from Goodreads and StoryGraph exports; and it is fully open
source, including the book dataset it uses and publishes. **Skylights** is a smaller atproto
book-review app, and both read the same underlying records because of how the protocol works.

**Why atproto changes the answers.** ActivityPub is **message-passing**: your data lives on your
instance and copies are pushed to followers' inboxes — which is why new BookWyrm instances start
blind. Atproto splits it differently: your data lives in a **personal data server (PDS)**, a
repository you control, keyed to your identity rather than to any app. Apps are **AppViews** —
indexers that consume a firehose of everyone's public records and build a queryable view.
Consequences:

- **Cold start disappears.** A new book app does not wait for follows to trickle reviews in; it
  indexes the whole network's existing public book records from day one.
- **No instance-picking, no per-server signup walls**, since identity is portable and separate from
  the app.
- Reviews are public, structured, schema-defined records (**lexicons**), so "open Goodreads review
  library" is closer to true: the corpus is openly harvestable by design. The copyright ambiguity
  still technically applies, but the **data-availability** question is settled.

**The trade-off is the mirror image:** atproto is public-by-default and currently depends on relay
infrastructure Bluesky the company mostly runs, so you give up BookWyrm's small-community privacy
model and self-determination for global reach. And on "an open Goodreads" in the sense of the
accumulated 15-year review archive: **that does not exist anywhere open.** Amazon owns it, closed
the public Goodreads API years ago, and no open project has an equivalent corpus. BookHive is the
bet that one can be accumulated in the open this time.

### A2. BookHive's architecture and lexicon space

> This is another big one on the list of companies owning culture and for losing it for profit,
> spite or indifference
>
> What is bookhives design and arch? What is the lexicon space?

**Assistant.** BookHive's design is deliberately thin: the protocol carries the weight and the app is
close to a stateless view over it. From the README: backend is **Hono** (lightweight TypeScript web
framework) using AT Proto for OAuth, frontend is mostly static HTML with some Hono JSX for dynamic
content, database is **SQLite with Kysely** as the ORM. Three tiers:

1. **Source of truth: your PDS.** All data is stored in your PDS and can be used by other apps.
   Writing a review means BookHive, authorized via Bluesky OAuth, writes a **signed record into your
   own repository**, not its database.
2. **Ingestion: the firehose.** BookHive displays a feed of new books and reviews from the
   network-wide stream of record events relayed from everyone's PDSes.
3. **Index: the SQLite database is just the AppView cache** — an aggregation of firehose records so
   pages render fast. **Rebuildable from the network**, which is why something as modest as SQLite
   suffices.

The lexicon space is **`buzz.bookhive.*`**, reverse-DNS of the domain. From a live record the
assistant verified two: a tracked book is a **`buzz.bookhive.book`** record containing `title`,
`authors`, a **`hiveId`** (BookHive's internal book identifier linking your record to a canonical
book entry), a cover image blob, `createdAt`, and a reading status expressed as a reference into
**`buzz.bookhive.defs`**, e.g. `buzz.bookhive.defs#reading`. Its address is an AT URI of the form
`at://did:plc:…/buzz.bookhive.book/<rkey>`. The repo's defined lexicons also cover reviews and
ratings in the same namespace, but **GitHub blocked listing the lexicons directory, so those exact
record type names could not be given** (assistant's own caveat).

> That AT URI shape is the point: the record is addressed by your identity plus schema plus key,
> with no server name in it, so any app can resolve and reuse it.

On the "companies losing culture" concern, two concrete mitigations: the code is **MIT licensed**,
and the repo ships a workflow that pulls a **sanitized SQLite export of the whole instance weekly**
as a GitHub Actions artifact. So the aggregate index is periodically snapshotted in the open, and the
underlying records outlive the app by construction. **Honest caveat:** `hiveId` and the canonical
book dataset are BookHive's own layer, so a successor app inherits the reviews for free but has to
**re-resolve which book each one points at**.

### A3. The correction — two different meanings of "open"

> So for book review sites with unambiguously open data it's literally just bookhive?

**Assistant.** Almost, but the precise answer depends on which "open" you mean, and splitting that
resolves it. Two different properties get called open data: **openly licensed** (anyone may legally
reuse it) and **openly available** (anyone can technically get all of it). **No book review site
fully has both.**

- **Unambiguously open *license*: Open Library, not BookHive.** Open Library requires that all user
  contributions be given under **CC0 1.0** (public-domain dedication, waives all rights) and
  publishes **bulk dumps** of all records in JSON. That is the only place in this space where reuse
  rights are legally settled. The catch is what their "reviews" are: **Community Reviews** is a
  crowd-sourced tagging system — aspect tags (pace, mood, difficulty), **not prose**. Fully open
  data, but thin as reviews.
- **Unambiguously open *availability*: BookHive and its atproto neighbours.** Every review is a
  public, schema'd record in the author's own repository, streamed on the firehose; BookHive also
  publishes its book dataset and weekly sanitized DB exports. Skylights sits on the same records.
  **But nothing in atproto or BookHive attaches a CC0-style license to your review text.** The data
  is *architecturally unencloseable*, while the legal reuse right is still the implied-license fog.
- **Neither: the ActivityPub side.** BookWyrm's reviews are partially available (only what federated
  to you) and unlicensed. **NeoDB**, an ActivityPub review site covering books and other media, sits
  in the same bucket — *assistant flagged it pulled no source on NeoDB, so treat that placement as
  unverified.*

**The corrected claim:** BookHive is the only prose-review social site whose full review corpus is
open **by construction**, and Open Library is the only one whose contribution data is open **by
license**. An "open Goodreads" in the full sense would need both at once, and the closest move
toward it would be BookHive adopting a CC0 or CC-BY grant in its lexicon or terms — which, as far as
the assistant could find, **no one has done yet**.

---

## Part B — PDS lexicon for iroh endpoint lookup, and croftcall

> *(This body appeared twice in the paste, verbatim. Reproduced once.)*

### B1. The ask

> I want to use a PDS lexicon for an Iroh account lookup so I can have a link for users that choose
> to publish their cryptographic association so that one iroh user could call another. I have a
> relay of my own at relay.croft.ing
>
> Can you talk me through this setup in detail and what's possible

**Assistant.** *(Delivered as the artifact `croft-relay-plan.md` — filed verbatim at
`seeds/callpds-unpacked/croft-relay-plan.md`.)* The choices flagged as load-bearing:

- **Embed-before-fork is the load-bearing decision.** Phase 0 forces the ADR: since the `Bucket`
  rate-limit primitive is public specifically for embedders mounting the relay protocol via their own
  HTTP server, there is a real chance most of this lives in a **croft-relay binary that wraps
  iroh-relay**, with the fork shrinking to one or two "expose this seam" patches. Cheapest to
  maintain, easiest to upstream — so the plan tries it first and falls back per-capability.
- **Phases map to the dials, one story each.** Phase 1 registered-only using the stock HTTP hook
  (zero fork, ships alone). Phase 2 the stateless signed token, mirroring n0's own short-lived token
  scoped to the endpoint's identity. Phase 3 turns the tier claim into a per-connection bucket,
  including a **calibration step that measures holepunch coordination bytes in the harness rather
  than guessing** the coordination-tier cap. Phases 4–5 hardening and PR packaging. Two features
  explicitly tagged upstream candidates and kept policy-free.
- **Two guardrails baked in:** Phase 0's first job is **re-verifying every claim the plan makes
  against the pinned commit**, since APIs like `set_client_rate_limit` are weeks old and moving; and
  **mutation testing (`cargo-mutants`) is scoped per-phase** with a hard rule of no unexplained
  survivors in admission or token-verification paths, while mutation configs stay **out of anything
  destined upstream** ("not their convention, not in the contribution").
- **Section 7 has five open questions, each with a default** so the run does not block: token format
  (defaulted to JWT/EdDSA), repo shape, whether coordination tier hard-fails relayed media, deploy
  timing for Phase 1, and metrics cardinality.

### B2. The exchange page and the Android client

> Ok the relay testing landed and everything is green
>
> Can we design a page that allows now looking up the endpointid by bluesky handle and then has a
> connect button that would open on a local app
>
> And let's build a basic android app that will open with it that is ready to iroh connect two
> endpoints, so we'll need a home user and a way for it to load the called user from the deep link
> from the webpage

**Assistant.** *(Delivered as `croft-exchange.html` and `croftcall-android.zip`.)*

The lookup page works as-is against live infrastructure: **handle → DID** via the public AppView
XRPC, **DID → PDS** via `plc.directory` (`did:web` handled too), then **`com.atproto.repo.getRecord`**
on their PDS for **`ing.croft.iroh.endpoint/self`**. Resolution renders as a live ledger trace so
users see each hop stamp in, and the Connect button emits
**`croftcall://call?endpoint=…&relay=…&handle=…&did=…`**. The full pipeline is testable today:
publish a record to your own repo via `putRecord` and look yourself up. A missing record shows a
"not listed" state rather than an error, since that is the normal case.

The Android project is a complete scaffold on the official **Kotlin bindings from Maven Central**:
persistent identity (secret key in `EncryptedSharedPreferences` so the published `EndpointId` stays
stable), `singleTask` activity with the `croftcall://` intent filter feeding `onNewIntent`, a callee
card populated from the link, dial-by-`EndpointId`, and a trivial **`croft-call/0` hello exchange**
as the v0 "call" so media can slot in later. Lifecycle follows iroh's Android guidance: shutdown on
background, re-bind with the persisted key on foreground.

**Honesty ledger** (also written into the README): the **bind/identity/lifecycle** API calls are
verified against the retrieved Kotlin docs, but the **accept/connect/stream method names are marked
`VERIFY`** in `CallPeer.kt`. The docs only promise the API maps 1:1 to Rust, so those names should be
checked against the Dokka reference and n0's `hello-iroh-ffi` Android sample (whose `IrohPeer.kt` has
exactly this shape) **before first compile**. Two more deliberate choices: the app uses `presetN0()`
relays for day one, with the `relay.croft.ing` + token swap isolated to one function; and it is
callable **only while foregrounded**, since staying reachable in the background needs a foreground
service and push-to-wake, which is its own phase. An `adb` one-liner in the README tests the deep
link without the web page.

### B3. The build instructions

> I need comprehensive Claude code instructions to build both of these in a new repo that deploys to
> GitHub pages

**Assistant.** *(Delivered as `croft-call-build-instructions.md`.)* Load-bearing choices:

- **Monorepo with one contract file.** The page and the app share exactly one interface — the
  `croftcall://call` deep link plus the record shape — so **`docs/contract.md` is the source of
  truth** and both halves' test suites assert against it: vitest on an extracted `resolver.js` (with
  injectable `fetch`, so no live network in tests) mirrors JVM unit tests on `DeepLink.parse` and
  `WireFormat`. Changes flow **contract → tests → implementations**.
- **Pages deploys the web half; CI builds the Android half.** GitHub Pages cannot host an app, so
  `android.yml` produces the debug APK as a downloadable workflow artifact (releases on tags), while
  `web.yml` runs tests then ships `web/` through the configure/upload/deploy-pages chain. The Pages
  fit is clean because the page is pure client-side and **atproto's APIs are CORS-open by design**,
  so no backend is needed.
- **Android order is verify-then-compile.** The first Android task is resolving the `VERIFY` markers
  in `CallPeer.kt` against the Dokka reference and n0's sample `IrohPeer.kt` — **names only,
  architecture untouched**, diff recorded in an ADR. Known packaging landmines (JNA `@aar`, Kotlin
  2.2, ABI fallback path) are called out so Claude Code does not rediscover them, including which
  source to trust if the Maven-artifact-on-Android conflict bites.
- **Gated Phase 4 synergy:** if `call.croft.ing` later points at Pages, the same origin serves both
  the exchange page and `.well-known/assetlinks.json`, upgrading the deep link from the
  scheme-chooser prompt to **verified App Links**. Gated on the owner's go-ahead since it touches DNS
  and needs the signing-cert fingerprint, which the instructions **explicitly forbid inventing**.
  Defaults set on the four open questions; scheme links work everywhere meanwhile.

---

## Supersession note — read before using Part B as design

Part B is the **origin** dialogue. Since it was written, the design moved:

1. **`CroftCommunity/connect` was built from these artifacts** and is now the corpus's ground truth
   (`plans/2026-08-07-1-plan-croft-relay-tiered-admission.md` cites its `docs/contract.md` §1–§2).
2. **The single-record contract here is superseded.** Part B uses rkey `self` read via `getRecord`.
   The tiered-admission plan's **Phase 10** moves to **per-device records via `listRecords`** plus a
   request-policy record. As of `main` @ `cc94b26`, `listRecords` is verified from lexicon source and
   Phase 10 is green-lit. **Do not implement from Part B.**
3. **The croft-relay design moved further still** — see
   `raw/croft-relay-tiered-admission-fork-vs-embed-2026-08-07.md`, which walks the fork instinct to a
   no-fork design and replaces rate-limiting with a byte budget + clean disconnect.
4. **The Android client crashes immediately on launch** (owner-reported 2026-08-09). The unresolved
   `VERIFY` markers in `CallPeer.kt` are the leading suspicion but **were not confirmed against the
   real crash** at filing time. Tracked as its own open item; do not record a cause that has not been
   observed.

## Distilled to

- `ROADMAP_TODO.md` **E98** (Croft's own content-licensing posture — the CC0-vs-availability gap).
- `ECOSYSTEM.md` **§5k** (book-review/open-data register).
- `COHESION.md` **§70** (the origin-vs-current contract seam).
