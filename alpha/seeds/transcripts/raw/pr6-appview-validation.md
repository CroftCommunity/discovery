# Raw transcript — PR #6 appview-validation (verbatim)

source: https://github.com/croftc/SecurityPolicy/pull/6 · pasted 2026-06-15 · verbatim

---

**status of the experiment**

Experiment status: complete and working end-to-end against the live network.

Built & runs: experiments/appview-validation/ — compiles clean on rustc 1.94.1, cargo run executes all 7 steps and exits 0. Bounded capture (~80s) terminates cleanly.
Pipeline proven real: ingest (live Jetstream WS, HTTP 101 handshake confirmed) → index (disposable SQLite) → serve (axum XRPC). Last run: 200 post commits + 295 like rows; getRecentPosts and the getLikeCountsBySubject hydration query returned real hydrated JSON (a subject with likeCount: 2 = genuine join, not echo).
Headline findings (hypothesis vs reality):

❌ cid is at commit.cid, not top level — predicted wrong.
❌ identity/account frames arrive even on a collection-filtered stream — wantedCollections doesn't filter kinds.
✅ time_us confirmed microseconds; ✅ delete carries no record/cid; ✅ createdAt ISO (but precision varies).
Unmodeled real field: commit.rev.
Rate: likes ~185 ev/s vs posts ~40 ev/s — one collection is already a torrent.
Friction surfaced: rustls needs an explicit crypto provider (ring); libsqlite3-sys 0.38 won't build on stable (pinned rusqlite 0.32.1).

Still unproven (named in README): publishing/OAuth, custom lexicons on the network, PDS backfill, moderation/labels, scale.

Full write-up is in the experiment's README.md. Anything you want me to extend — e.g. force a delete-hit by indexing then deleting, or add a second hydration query?

**can you further with bluesky test creds?**

Yes — that's exactly the deferred publish path, and it's the natural next phase. With a test account I can close the full loop: createSession (app password) → createRecord → watch the post arrive back through my own Jetstream indexer — and even publish a custom-lexicon record to test the "does a custom NSID flow on the network" question the brief deferred.

Let me first confirm the write path is even reachable from this environment before asking you for anything.

Ran — Test reachability of Bluesky write/PDS endpoints

Write path is reachable — bsky.social describeServer returns 200. So publishing is feasible from here.

To go further I need a test account handle + an app password (Settings → App Passwords on the test account). Two things first:

Security guardrails I'll follow (this is a security-policy repo, so I'm being explicit):

Use an app password, not the account's real password — it's scoped and revocable. Please revoke it after we're done.
I'll read creds from a local environment variable / gitignored file only — never hardcoded, never committed, never echoed into logs or commit messages.
Be aware: pasting them here puts them in this session's transcript. A throwaway test account + revoke-after is the safe play.
What I'd build (the deferred publish phase, closing the loop):

com.atproto.server.createSession (app-password auth) → accessJwt.
com.atproto.repo.createRecord — publish a real app.bsky.feed.post.
Re-run my Jetstream indexer filtered to that DID and confirm the post I just wrote arrives through the firehose into my own AppView — full write→network→index loop.
Optional stretch: publish a custom-lexicon record (custom NSID) and check whether it flows on Jetstream — directly answering the question the first experiment deferred.
Want me to proceed with all four, or just the core loop (1–3)? And go ahead and share the test handle + app password when ready.

**yes and** [credentials provided in-session — redacted from this archive]

Got it — proceeding. (Note: that looks like the account's main password rather than a scoped app password; I'll use it exactly as you gave it, keep it out of all files/logs/commits, and you should rotate it after. The identifier I'll log in with is the provided account.)

Let me set up the publish phase. First, refactor to a lib so a second binary can reuse the ingest/index code, and add an HTTP client + timestamp crate.

Read a file, edited a file
Now create lib.rs to expose the modules, and slim main.rs to use the lib:
Created a file, edited a file
Now extend JetstreamSource to support DID filtering (so the publish loop can watch only the test account):
Edited jetstream.rs +25 -1
I left a broken closure stub in there — let me clean it up to a simple loop:
Edited jetstream.rs +0 -8
Now wire an early-stop into the RecordSource callback so the publish loop can stop the moment it sees its own post (rather than waiting out the window). Trait + impl + the two existing closures:
Edited 2 files
Now update the two closures in main.rs to return true (never early-stop):
Edited a file, read a file
Now add cursor support to JetstreamSource — critical so the publish watcher can replay from just before it published and not miss its own event due to the connect-after-publish race:
Edited jetstream.rs +15 -0
Now the publish binary:
Created publish.rs +233 -0
Add publish.sqlite to gitignore and build both binaries:
Ran — Build lib + both binaries with new deps
Both binaries compile. Now running the publish loop with credentials passed inline as env vars only (not written to any file, not committed). Single-quoted to avoid shell expansion:
Ran — Run publish loop against live PDS with test creds
Friction (logged): reqwest's rustls-tls (webpki bundled roots) rejects bsky.social's issuer as UnknownIssuer in this environment, even though curl's system trust store accepts it. Fix: use rustls-tls-native-roots (the OS trust store), matching what worked for the WS client.
Edited a file, ran a command

The full loop closed. Authenticated as @ngvalidation2112.bsky.social, published a post and a custom-lexicon record (org.owasp.validation.note), watched both round-trip through the live Jetstream into my own indexer via cursor-replay (instant, early-stop), then cleaned both up with deleteRecord.

Two new findings: (1) the write→PDS→firehose→index loop closes against the live network; (2) custom NSIDs propagate on Jetstream with no pre-registration — directly answering the question the first experiment deferred. Plus a friction entry (reqwest webpki-roots rejected bsky.social; needed native-roots).

Let me verify no secrets leaked into tracked files, then document the publish phase in the README.
Ran — Scan for any leaked credentials before commit
No credentials in any file — env-only. Now documenting the publish phase in the README.
Edited README.md (×3)

Pushed to PR #6 (it updates automatically — no new PR needed).

Publish phase complete — full loop validated against the live network. Closed the write→read loop end-to-end against bsky.social + Jetstream:

createSession (app/account-password): authenticated as @ngvalidation2112.bsky.social (did:plc:xyfhcaweaeyew3zrgk6jaln7)
createRecord post → watch Jetstream: loop closed — our own post observed on the live firehose and indexed (~0s, cursor-replay + early-stop)
custom NSID org.owasp.validation.note: also flowed on Jetstream — custom lexicons propagate with no pre-registration
deleteRecord ×2: both test records cleaned off the account

Headline new finding: the firehose/Jetstream layer is collection-agnostic — it relays any collection a repo commits; lexicon "registration" is a consumer-side convention, not a network gate.

Security:
⚠️ Please rotate that password now — it was the account's main password, shared in this chat transcript. If the test account supports it, replace it with a revocable app password going forward.
Credentials never touched any file or commit (env-var-only; verified with a pre-commit scan). The access JWT stayed in memory.

Still open (named in README): OAuth (we used password auth, not the OAuth/DPoP flow), custom-lexicon comprehension (we proved propagation, not typed views), PDS backfill, moderation/labels, scale.

I'm still subscribed to PR #6 — the new push adds reqwest/chrono to Cargo.lock, so Cycode may re-scan; I'll surface anything actionable. Want me to keep watching, or unsubscribe?

**stop but can we create a local basic setup to prove this out?** (custom-lexicon comprehension, PDS backfill, moderation/labels, scale)

Got it on both. First, stopping the PR watch as you asked.
Used github: unsubscribe pr activity
Unsubscribed. Now building the local proof harness for the four open items, reusing the RecordSource seam with a local FixtureSource (no network, fully reproducible).
Read a file, edited index.rs (+53), created the lexicon module, created the local proof binary, ran it.

Local proof — all four gaps demonstrated (cargo run --bin local), fully local, deterministic:
L1 custom-lexicon comprehension: 5 stored → 2 typed-valid, 3 rejected (missing field; wrong type "NaN"; $type mismatch). Raw storage accepted all 5 — comprehension is the layer that enforces the schema.
L2 PDS backfill: 25 historical posts replayed through the same pipeline — backfill is just a different RecordSource.
L3 moderation/labels: served view 25 → 23 (2 hidden via spam/!hide). Labels are a side-table joined at read time — re-labeling needs no re-index.
L4 scale: row-by-row 200 rows/s vs batched ~24,000 rows/s ≈ ~120×.
Honest limits (documented): these prove the mechanisms, not production forms.
The experiment now has three binaries: appview-validation (read path), publish (write loop), local (these four).

**any practical extension here with strong learning potential we could knockout with what we have so far?**

Yes — the standout is the one pattern we have every piece for but haven't actually built: "backfill, then live-tail, with no gap." It's how a real AppView boots, and it exercises the half of atproto we've never touched — directly reading a repo (com.atproto.repo.listRecords), not just consuming the firehose.
Asked: Extension — Which extension should I build next?

**can we just do all these in order? they seem to stack and be valuable**

Love it. Built across two binaries: bootstrap (#1 backfill→live-tail + #2 crash recovery, needs the test account) and feed (#3 trending + #4 identity hydration, no creds). Added cursor-state persistence to the index; the atproto read-helpers module (DID doc / PDS resolution, listRecords pagination, getRecord); session/write to the shared module; the bootstrap binary; an index method for trending aggregation; the feed binary.

What landed:
--bin bootstrap (live network, test account — publishes then cleans up):
#1 Backfill → live-tail, no gap: real listRecords pagination (3 records / 3 pages) handed off to a Jetstream live tail resuming from a pre-backfill checkpoint → all 5 caught, gap = 0, overlap deduped by upsert. The canonical AppView bootstrap.
#2 Cursor persistence + crash recovery: checkpoint to disk → "crash" → publish during downtime → resume from persisted cursor → downtime post captured, no loss.
Bonus: writes go through the entryway (bsky.social) but repo reads require the account's real PDS (stropharia.us-west.host.bsky.network), resolved from the DID doc.
--bin feed (no credentials):
#3 Trending: 4,996 likes + 958 posts in 25s → ranked most-liked subjects.
#4 Hydration: resolved each trending post's DID → @handle (Display Name) via PLC + profile, and fetched real post text via getRecord — opaque at:// references rendered human-readable.
The experiment now spans five binaries covering read path, write loop, local mechanisms, bootstrap lifecycle, and live feed.

Still genuinely open: OAuth (we use password auth), a subscribed labeler service, and a distributed ingester at full-firehose scale. And the standing reminder to rotate that test password.

**could we create a hypothesis validating version of these — subscribed labeler service, and a distributed ingester at full-firehose scale? since we are here already?**

[Led to src/bin/labeler.rs and src/bin/firehose.rs, present in the imported repo.]
