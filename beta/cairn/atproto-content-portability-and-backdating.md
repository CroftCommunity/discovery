# atproto Content Portability and Backdating

`Status: survey, current as of mid-2026. The atmosphere moves fast; treat tool availability, hard limits, and roadmap claims as a snapshot. Dated-source and not-re-verified flags below are carried deliberately, not smoothed into settled fact.`

`Scope: the atproto write path for one-time content backfill (backdating semantics, the two-step image path, hard limits, reconciliation), a concrete cross-protocol migration case study (Pixelfed export), the tooling landscape and the specific gap in it, and the backdated-post labeler as a moderation primitive. Two of the sections below are preserved as epistemic-discipline case studies, not just as facts.`

This doc catalogues a building block Drystone cares about twice over: as the mechanics of moving durable content into a per-author repository on original dates, and as a worked example of the network's stance on time-honesty (backdating is allowed but detectable, not blocked). Both bear directly on any design that wants to carry a real history into a portable identity without either faking a track record or being unable to import at all.

## The atproto write path for content backfill

The write path is well documented and simpler than most export sides it has to consume. It is the load-bearing half of any backfill.

- A post is `app.bsky.feed.post`, requiring `text` and `createdAt`.

- `createdAt` is a plain client-supplied timestamp, settable to any past date. This is the backdating capability: nothing in the write path forces `createdAt` to "now."

- Images take a two-step path. Upload each image via `com.atproto.repo.uploadBlob`, which returns a blob ref (CID, mimeType, size); then reference the blob(s) in an `app.bsky.embed.images` embed, each image carrying its own alt text.

- Hard limits: four images per post, and roughly 1MB per image. Docs give conflicting figures (1MB vs 2MB), so targeting about 1MB is the safe read. EXIF should be stripped before upload (strongly recommended, and may be enforced more strictly later).

- `com.atproto.repo.listMissingBlobs` is the reconciliation call: after writing records, use it so nothing a record references gets garbage-collected.

The detail that makes backdating actually work for a profile history is `sortAt`, defined as the earlier of `createdAt` and `indexedAt`. Because a backdated post's `createdAt` is older than its `indexedAt` (when the network actually saw it), `sortAt` takes the old `createdAt`, and the imported post sorts correctly into the profile's chronological history rather than piling up at the top on import day.

No dedicated photo-native lexicon was found that beats `app.bsky.feed.post` for timeline rendering, so that is the lexicon to target for a photo backfill.

End to end, the grounded build order is: pull the export while the source is still up; download every image locally now (irreplaceable, see below); transform each post's caption to text and its original date to `createdAt`; preprocess images (strip EXIF, recompress under about 1MB, split or drop extras past four); upload blobs and capture refs; write each record via `createRecord`/`applyWrites` as `app.bsky.feed.post` with the embed and the backdated `createdAt`; then reconcile with `listMissingBlobs`. The result is a profile carrying its photo history on original dates, sorted chronologically, honestly marked as archived, entirely within supported mechanisms.

## The Pixelfed export gotcha (a migration case study)

Pixelfed is worth cataloguing not because it is special but because its export shape is a concrete instance of everything that makes atproto content migration harder on the source side than the write side.

- The "export" is multiple separate JSON files (`account.json`, `outbox.json`, likes, bookmarks), not one archive.

- Critically, there is no image data in the export, only URLs to where assets are currently hosted. If the source instance goes down, the archive is useless because no image bytes are included. This is why "download every image locally now" is an irreplaceable step: the URLs die with the instance.

- `outbox.json` is the source of truth, holding posts as ActivityStreams objects (an ActivityStreams OrderedCollection). [Dated-source flag: the exact `outbox.json` structure comes from community writeups dated late 2024/2023, corroborated on "URLs not bytes" but not re-verified against the current Pixelfed version. Pull your own export and inspect one post object before writing a parser against a year-old description.]

- Posts arrive as single items with thread relationships lost, so multi-post threads flatten. Fine for a photo diary, not for reply chains.

- A community script exists to download photos from a Pixelfed export JSON, but its author notes it fails if `outbox.json` cannot be generated, which happened historically for accounts over 500 statuses. [Dated-source flag: the 500-status export ceiling is dated late 2024 and may since be fixed; check against the live instance rather than trusting the figure.]

Do not confuse any of this with Instagram-to-Pixelfed import guides, which are the opposite direction and irrelevant here.

## The tooling landscape and the gap

Surveying published tools, each individual constraint has a tool, but the combination needed for a dated one-time photo backfill does not exist as a shipped product.

- `mastodon-to-bluesky` (a maintained CLI) is architecturally the closest: it reads a Mastodon outbox, downloads media, splits long posts into threads, posts to Bluesky, with incremental transfer, dry-run, and retry. But its README states plainly that Bluesky "doesn't support backdating posts," so it stamps the current date/time and appends `[Originally posted: ...]` as text. That directly defeats date preservation. It is also tuned to Mastodon's shape, not Pixelfed's photo-centric objects.

- Crossposting and continuous-sync tools (`mastodon-bluesky-sync`, MicroPoster) are ongoing mirrors, the wrong mode entirely for a one-time backfill, and none backdate.

- Bounce (from A New Social, the Bridgy Fed team) is cross-protocol account migration between ActivityPub (Mastodon, Pixelfed named explicitly) and atproto. Two disqualifiers: at launch it only supported Bluesky-to-Mastodon/Pixelfed (the reverse of the needed direction), and even the later Mastodon-to-Bluesky path is explicit that "your content will not come with you." It moves the social graph, not posts.

- Bridgy Fed bridges live traffic between AP and atproto; it does not backfill history. Bridging Pixelfed specifically was reported as finicky by at least one user.

The gap, stated plainly: nobody has published a tool that is source=AP-photo-app AND mode=one-time-backfill AND dates=preserved, together. Each constraint alone has a tool; not the combination. The two realistic routes are to fork `mastodon-to-bluesky` (pointing it at the Pixelfed export shape and setting `createdAt` to the original date instead of appending text, which is enabling a capability the tool's author chose not to use, not defeating a block) or to write a fresh script given the Pixelfed export is idiosyncratic anyway. Idempotency (tracking already-written post IDs to survive reruns) and PDS write rate-limit pacing matter either way.

## The self-correction discipline example

This section is preserved as a grounding-discipline case study, not just as a set of facts. It is a worked example of the retract-unsourced-speculation move.

The session initially speculated that the Mastodon-importer author declined to backdate because he "believed backdating wasn't supported, or wanted to avoid the archived-post treatment." On being challenged to confirm the claim, the session separated the two halves and retracted the unsourced one:

- Claim 1 (the platform honors a past `createdAt`): confirmed, triple-sourced. The docs state clients may insert any value for `createdAt`, enabling import/migration, with `sortAt` defined as the earlier of `createdAt`/`indexedAt`; a real user reported bulk-loading old Twitter/Facebook history into Bluesky with original dates, producing a profile diary back to 2003; and a dedicated "port tweets to Bluesky maintaining original date" tool exists (a Show HN posting).

- Claim 2 (the Mastodon tool opts out by choice): confirmed from its own README, whose stated (and factually incorrect) reason is that "Bluesky doesn't support backdating posts."

- The retracted part: the extra speculative motive ("wanted to avoid archived-post treatment") had no source and was withdrawn. The corrected, sourced statement is narrower: the tool does not backdate because its author stated (incorrectly) that the platform does not support it.

The discipline point is that the capability existing and the tool declining to use it are both true and are not in tension; the only thing that had to go was the unsourced guess about why. Keeping the two confirmed claims and dropping the speculative third is the shape of the move worth preserving.

## The backdated-post labeler (@backdate.mozzius.dev)

A labeler running on the atproto network flags posts where the record's self-reported `createdAt` does not match when the post first appeared on the firehose. Mechanically it operationalizes the one check that catches backdating (declared time vs observed arrival time) and publishes the result as a subscribable label, using atproto's user-side moderation primitive (labelers).

The category-level reasons this kind of tool exists are groundable without asserting the specific operator's motive:

- Backdating is invisible by default in normal UI, so there is no built-in signal that a post's declared date differs from when it actually appeared.

- It enables a named scam pattern: create an account whose posts appear to predict past events (sports scores, stock prices) using timestamps from before those events, then monetize the fabricated track record. This was raised explicitly in the comments under the tweet-porting tool's Show HN. Bluesky's own docs flag the far-future-timestamp variant as a possibility for "shenanigans," so a labeler is a natural fit.

The session explicitly declined to assert why this specific labeler operator built it (serious anti-scam tool vs demonstration vs a bit), citing no source, while standing behind the category-level reason. That refusal is preserved here as modeled epistemic discipline, not as a gap someone should later fill in with a guess.

The design-relevant framing: an honest archival backfill would likely get flagged as backdated by such a labeler, and that is a feature, not a defect. The flag is the truthful "this was imported, not organic" signal for exactly this use case. It also confirms the network's stance is "backdating allowed, but detectable," not "blocked."

## What this establishes (and does not)

Establishes that atproto's write path makes durable, dated content backfill mechanically straightforward (client-supplied `createdAt`, the `uploadBlob`-then-`embed.images` two-step, `sortAt` taking the earlier timestamp so imports sort into history, `listMissingBlobs` for reconciliation), and that the hard limits (four images, about 1MB each, EXIF stripped) are the real constraints. Establishes that the network's posture on time-honesty is allowed-but-detectable: backdating is a supported capability, and a labeler makes it visible, which is the honest signal an archival import should welcome rather than evade.

Establishes, via the Pixelfed case, that the source side is where migration is hard (multi-file exports, URLs not bytes, flattened threads), and that the specific tool combining an AP photo source, one-time-backfill mode, and preserved dates does not yet exist, though the pieces do.

Preserves two discipline examples for reuse: the self-correction that retracted an unsourced motive while keeping the sourced claims, and the labeler section's refusal to assert a specific operator's intent without a source.

Does not settle the dated-source facts it carries: the 500-status export ceiling and the exact `outbox.json` shape are late-2024/2023 community claims flagged for re-verification against a live instance, not confirmed current behavior. Does not specify Drystone's own import behavior or governance; it catalogues the field's mechanics and stance that any such design would have to sit alongside.
