# RUN-BUNDLE-PRECACHE

Ship a build-time snapshot of the mostly-static cook list and recipe index as part of the
release bundle, render from it instantly on first load, and revalidate against each PDS by
repo revision so that unchanged repos cost one tiny request and zero record fetches.

Repo: `CroftCommunity/arecipe`. Static bundle on GitHub Pages, deployed by Actions.

---

## 0. Standing directives

**TDD, red first.** Failing tests before implementation, fixtures before features,
red-to-green evidenced in the run summary.

**No browser storage APIs beyond Cache API and IndexedDB.** No localStorage.

**The snapshot is a cache, never an authority.** Everything it holds can be stale. Nothing
in the app may treat a snapshot value as true after live data has arrived.

---

## 1. The shape of the problem

The cook list seeded from the arecipe Bluesky account is long and nearly static. Today
every cold load pays full dynamic cost to reassemble content that has not moved in weeks.
The Wikibooks corpus from RUN-WIKIBOOKS-CORPUS makes this worse and better at once: it adds
thousands of records, and it changes twice a year.

The protocol already gives us the cheap primitive:

- `com.atproto.sync.getLatestCommit` returns the current commit CID and revision of a repo.
- `com.atproto.sync.getRepo` takes a `since` revision and returns only a diff.

So the revalidation question per cook is one small request that returns a string. Compare
it to the revision recorded at build time. Equal means nothing to do.

---

## 2. Build-time snapshot (D1)

A generator script, `scripts/snapshot.mjs`, run in CI before the bundle build.

For each DID in the seed list:

1. Resolve the DID to its PDS endpoint.
2. `com.atproto.sync.getLatestCommit` → record `rev` and `cid`.
3. `com.atproto.repo.listRecords` over the recipe collection, paginating by cursor, into a
   per-cook shard.
4. Re-check `getLatestCommit` after the listing completes. **If the rev moved during
   pagination, redo that cook.** A shard captured across a commit boundary is torn, and a
   torn shard paired with the newer rev means the app will never notice it is wrong. This
   is the subtle correctness bug in the whole design; write the test first.

Emit under `assets/`:

```
assets/snapshot/<buildId>/manifest.json
assets/snapshot/<buildId>/cooks/<did>.json
assets/snapshot/<buildId>/index.json
```

`assets/` is required, not stylistic: `scripts/build.mjs` is an allowlist copy into `dist/`
and only `assets/` is copied recursively. A snapshot written anywhere else silently fails
to deploy. Add a build test that asserts the snapshot files are present in `dist/`.

`manifest.json` per cook: `{ did, handle, displayName, rev, cid, recordCount, sha256,
capturedAt }`. `index.json` is the minimum needed for first paint: cook identity plus
recipe titles and rkeys, nothing else. Full recipe bodies live in the per-cook shards and
are loaded on demand.

**Size budget, enforced by a failing build:** `index.json` gzipped must stay under a
declared ceiling. Pick the number from measurement in D6, write it into the build, and fail
CI when it is exceeded. Without a hard gate this file grows until it defeats its own
purpose.

**Owner decision O1:** the seed list source. The starter pack is partly seeded from the
arecipe Bluesky account; the snapshot generator needs one canonical input. A committed
`snapshot-seed.json` is the simplest answer and keeps CI hermetic.

**D1 tests:** a fake PDS serves a repo; the generator produces a manifest whose sha256
matches the shard; a rev that changes mid-pagination causes exactly one retry of that cook
and no torn shard is ever written; a cook whose PDS is unreachable is omitted with a
recorded reason rather than failing the whole build.

---

## 3. Boot path (D2)

1. Load `index.json` from the bundle. It is same-origin, versioned by build id, and
   immutable, so it is cacheable forever.
2. **Render.** No spinner, no network on the critical path.
3. Only then start revalidation, off the critical path.

The measurable claim: a cold load renders the full cook list and recipe index with zero
network requests beyond the bundle itself. Prove it with a request-count assertion, not a
description.

**D2 tests:** boot with the network transport throwing on every call still renders the full
index; boot with a corrupt or truncated snapshot falls back to live loading and logs once,
without a blank screen and without an error dialog.

---

## 4. Revalidation (D3)

Per cook: one `getLatestCommit`. Compare to the manifest rev.

- **Unchanged** → done. Nothing else is fetched for that cook, ever, in that session.
- **Changed** → refetch that cook's records with `listRecords` and replace the shard in
  IndexedDB.

**Use `listRecords`, not the `getRepo?since=` diff, in this run.** The diff path returns a
CAR of MST blocks, which means shipping CAR and DAG-CBOR decoding into a bundle that
currently has none. That is real weight for a corpus whose whole premise is that it rarely
changes. Record the tradeoff in the run summary and leave the diff path as a documented
follow-on with a measured trigger: revisit it if refetch volume on a typical session
exceeds a stated threshold.

Scheduling, because N cooks means N rev checks:

- Cap concurrency at 4.
- Viewport first. Revalidate the cooks whose content is currently on screen, then the rest
  during idle time via `requestIdleCallback` with a `setTimeout` fallback.
- **Debounce across sessions.** Persist `lastRevalidatedAt` per cook. Skip revalidation
  entirely if it succeeded within the last N minutes. **Owner decision O2:** N. Suggested
  default 60 minutes, with the app's own explicit refresh control always bypassing it. The
  refresh control is the clockwise arrow; the counter-clockwise arrow remains reset and is
  untouched by this run.
- On failure, keep serving the snapshot. A failed revalidation is not an error state.

**D3 tests:** unchanged rev produces exactly one request per cook and zero record fetches;
one changed rev out of twenty produces exactly one refetch; concurrency never exceeds 4,
proven by instrumenting the fake transport; the debounce skips within the window and fires
outside it; the explicit refresh control bypasses the debounce; a rev check that rejects
leaves the snapshot rendered and the app usable.

---

## 5. Staleness honesty (D4)

The snapshot holds identity data that goes stale in ways recipe text does not. Handles get
changed. Display names get changed. Accounts get deactivated.

Rules:

- Recipe content from the snapshot may be rendered as-is.
- **Identity fields are provisional until revalidated.** When live data disagrees with the
  snapshot, live wins immediately and the UI updates in place without a reload.
- A cook whose repo is gone or deactivated is removed from the live view for that session
  and reported in the build's next snapshot. Do not render a dead cook because the bundle
  says they exist.

**D4 tests:** a handle change between build and runtime results in the live handle being
displayed after revalidation; a deactivated repo disappears from the list; a stale display
name never persists after a successful revalidation.

---

## 6. Service worker and release interactions (D5)

- Precache the snapshot at service worker install. Activate atomically. On activate, purge
  snapshot directories for build ids other than the active one, so old snapshots do not
  accumulate in the Cache API indefinitely.
- **Version pin interaction, load-bearing.** The pin is device-local, pins the current
  version only, and while active refuses all upgrades and shows no upgrade anywhere. A
  pinned install must therefore keep its own snapshot: the service worker must not swap in a
  newer build's snapshot, and the purge step must not delete the pinned build's directory.
  Write this test before writing the purge code, because the failure mode is a pinned
  install silently losing its cache.
- **Signed release interaction.** The snapshot ships inside the release bundle and is
  therefore covered by whatever the release signing process signs. Confirm that the snapshot
  files are inside the signed artifact and not added afterwards, and state the finding
  explicitly in the run summary.
- Live deltas go in IndexedDB, keyed by build id plus did plus rev, so a build change never
  mixes deltas across snapshot generations.

**D5 tests:** activate purges other build ids; activate with a pin active purges nothing;
IndexedDB keys are build-scoped; a snapshot present in Cache API is served without a network
request.

---

## 7. The Wikibooks corpus as first tenant (D6)

Once RUN-WIKIBOOKS-CORPUS publishes, its repo is the ideal case: one DID, one rev, thousands
of records, changing roughly twice a year. It should be snapshot-bundled and should
essentially never trigger a refetch.

- The corpus repo's rev is emitted in that run's `summary.json`. The snapshot generator can
  take it directly rather than re-deriving it, though it must still verify with
  `getLatestCommit` rather than trusting the file.
- Because the corpus is large, shard it further than one file per cook: split the corpus
  shard by first letter or by fixed record count, so a first paint never loads the whole
  thing. `index.json` still carries only titles and rkeys.

**D6 tests:** the corpus repo revalidates in exactly one request; corpus shards load
lazily and a recipe opened from the index fetches exactly one shard.

---

## 8. Measurement (D7) — required, not optional

The run summary must contain measured before-and-after numbers, not estimates:

| metric | before | after |
|---|---|---|
| network requests, cold load to full list | | |
| bytes transferred, cold load | | |
| time to first rendered list, throttled to Slow 4G | | |
| PDS requests per warm session, nothing changed upstream | | |
| PDS requests per warm session, one cook changed | | |
| snapshot size, raw and gzipped | | |

The fourth row is the headline. If it is not close to one request per cook, the design did
not land. Use the measured snapshot size to set the D1 build gate.

---

## 9. Acceptance

1. Every deliverable has a failing test in its history and a passing test now.
2. Cold load renders the full index with zero requests beyond the bundle.
3. A warm session with nothing changed upstream makes one request per cook and fetches no
   records.
4. The torn-shard case is proven impossible by test.
5. The pinned-install purge case is proven safe by test.
6. Snapshot files are verified present in `dist/` by a build test.
7. The measurement table is filled with real numbers.
8. O1 and O2 are answered at the top of the summary, or flagged as blocking.

## 10. Out of scope

The `getRepo?since=` CAR diff path. Offline authoring. Precaching images or blobs.
Server-side rendering of any kind. Changing the reset or refresh iconography.
