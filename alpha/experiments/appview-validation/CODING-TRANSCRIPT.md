# Coding transcript — AppView validation against live Jetstream (PR #6)

source: AI coding session that produced this experiment, pasted verbatim 2026-06-15.

context: the build narrative for the standalone AppView validation experiment. It begins
with the end-to-end status, then walks the deferred-publish phase (live PDS write loop),
the local proof harness, and the AppView-lifecycle extensions (backfill→live-tail, crash
recovery, trending, hydration). The labeler and firehose binaries requested at the end of
this log subsequently landed in the repo (src/bin/labeler.rs, src/bin/firehose.rs).

---

Experiment status: complete and working end-to-end against the live network.

**Built & runs:** `experiments/appview-validation/` — compiles clean on rustc 1.94.1, `cargo run` executes all 7 steps and exits 0. Bounded capture (~80s) terminates cleanly.

**Pipeline proven real:** ingest (live Jetstream WS, HTTP 101 handshake confirmed) → index (disposable SQLite) → serve (axum XRPC). Last run: 200 post commits + 295 like rows; `getRecentPosts` and the `getLikeCountsBySubject` hydration query returned real hydrated JSON (a subject with `likeCount: 2` = genuine join, not echo).

**Headline findings (hypothesis vs reality):**

- ❌ `cid` is at `commit.cid`, not top level — predicted wrong.

- ❌ `identity`/`account` frames arrive even on a collection-filtered stream — `wantedCollections` doesn't filter kinds.

- ✅ `time_us` confirmed microseconds; ✅ delete carries no record/cid; ✅ `createdAt` ISO (but precision varies).

- Unmodeled real field: `commit.rev`.

- Rate: likes ~185 ev/s vs posts ~40 ev/s — one collection is already a torrent.

Friction surfaced: rustls needs an explicit crypto provider (ring); libsqlite3-sys 0.38 won't build on stable (pinned rusqlite 0.32.1).

Still unproven (named in README): publishing/OAuth, custom lexicons on the network, PDS backfill, moderation/labels, scale.

> **can you further with bluesky test creds?**

Yes — that's exactly the deferred publish path, and it's the natural next phase. With a test account I can close the full loop: createSession (app password) → createRecord → watch the post arrive back through my own Jetstream indexer — and even publish a custom-lexicon record to test the "does a custom NSID flow on the network" question the brief deferred.

Let me first confirm the write path is even reachable from this environment before asking you for anything.

> Ran — Test reachability of Bluesky write/PDS endpoints

Write path is reachable — `bsky.social` describeServer returns 200. So publishing is feasible from here.

Security guardrails I'll follow (this is a security-policy repo, so I'm being explicit):

- Use an app password, not the account's real password — it's scoped and revocable. Please revoke it after we're done.

- I'll read creds from a local environment variable / gitignored file only — never hardcoded, never committed, never echoed into logs or commit messages.

- Be aware: pasting them here puts them in this session's transcript. A throwaway test account + revoke-after is the safe play.

What I'd build (the deferred publish phase, closing the loop): createSession → createRecord → re-run the Jetstream indexer filtered to that DID and confirm the post arrives through the firehose into my own AppView. Optional stretch: publish a custom-lexicon record and check whether it flows on Jetstream.

> **yes and** [test credentials provided in-session]

Got it — proceeding. (Note: that looks like the account's main password rather than a scoped app password; I'll use it exactly as you gave it, keep it out of all files/logs/commits, and you should rotate it after.)

> Refactor to a lib so a second binary can reuse ingest/index; add HTTP client + timestamp crate. Extend JetstreamSource with DID filtering and cursor support. Build the publish binary.

The full loop closed. Authenticated as `@ngvalidation2112.bsky.social`, published a post and a custom-lexicon record (`org.owasp.validation.note`), watched both round-trip through the live Jetstream into my own indexer via cursor-replay (instant, early-stop), then cleaned both up with deleteRecord.

Two new findings: (1) the write→PDS→firehose→index loop closes against the live network; (2) custom NSIDs propagate on Jetstream with no pre-registration — directly answering the question the first experiment deferred. Friction: reqwest's rustls-tls (webpki bundled roots) rejected bsky.social's issuer; needed `rustls-tls-native-roots` (OS trust store).

> ⚠️ Please rotate that password — it was the account's main password, shared in the transcript. Credentials never touched any file or commit (env-var-only; verified with a pre-commit scan).

> **stop but can we create a local basic setup to prove this out?** (custom-lexicon comprehension, PDS backfill, moderation/labels, scale)

Building the local proof harness for the four open items, reusing the RecordSource seam with a local FixtureSource (no network, fully reproducible).

Local proof — all four gaps demonstrated (`cargo run --bin local`):

- **L1 custom-lexicon comprehension:** 5 stored → 2 typed-valid, 3 rejected (missing field; wrong type "NaN"; $type mismatch). Raw storage accepted all 5 — comprehension is the layer that enforces the schema.

- **L2 PDS backfill:** 25 historical posts replayed through the same pipeline — backfill is just a different RecordSource.

- **L3 moderation/labels:** served view 25 → 23 (2 hidden via spam/!hide). Labels are a side-table joined at read time — re-labeling needs no re-index.

- **L4 scale:** row-by-row 200 rows/s vs batched ~24,000 rows/s ≈ ~120×. A real ingester must batch + absorb bursts.

Honest limits (documented in README): these prove the mechanisms, not production forms.

> **any practical extension here with strong learning potential we could knockout with what we have so far?**

The standout is the one pattern we have every piece for but haven't built: "backfill, then live-tail, with no gap" — how a real AppView boots, exercising direct repo reads (`com.atproto.repo.listRecords`), not just the firehose.

> **can we just do all these in order? they seem to stack and be valuable**

Built across two binaries: `bootstrap` (#1 backfill→live-tail + #2 crash recovery, needs the test account) and `feed` (#3 trending + #4 identity hydration, no creds).

What landed:

- **#1 Backfill → live-tail, no gap:** real listRecords pagination (3 records / 3 pages) handed off to a Jetstream live tail resuming from a pre-backfill checkpoint → all 5 caught, gap = 0, overlap deduped by upsert. The canonical AppView bootstrap.

- **#2 Cursor persistence + crash recovery:** checkpoint to disk → "crash" → publish during downtime → resume from persisted cursor → downtime post captured, no loss.

- Bonus: writes go through the entryway (bsky.social) but repo reads require the account's real PDS (`stropharia.us-west.host.bsky.network`), resolved from the DID doc.

- **#3 Trending:** 4,996 likes + 958 posts in 25s → ranked most-liked subjects.

- **#4 Hydration:** resolved each trending post's DID → @handle (Display Name) via PLC + profile, fetched real post text via getRecord — opaque `at://` references rendered human-readable.

Still genuinely open: OAuth (we use password auth), a subscribed labeler service, and a distributed ingester at full-firehose scale.

> **could we create a hypothesis validating version of these — subscribed labeler service, and a distributed ingester at full-firehose scale? since we are here already?**

[This request led to src/bin/labeler.rs and src/bin/firehose.rs, present in the imported repo.]
