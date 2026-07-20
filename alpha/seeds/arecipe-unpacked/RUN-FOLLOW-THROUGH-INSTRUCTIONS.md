# RUN-FOLLOW-THROUGH — cook-follows hardening + live proof + loopback refresh fix (+ pwa-check brief)

Self-contained instruction file for Claude Code, repo `CroftCommunity/arecipe`.
Four parts; each ships independently and the gate (`npm test`) is green at the
end of every part. [verify-in-run] items are probed empirically and recorded in
the run summary. Surprises that contradict this file's grounding are FINDINGS,
reported not silently absorbed.

## 0. Mission

The cook-follows run shipped clean (reviewed 2026-07-15). This run closes its
two audited residuals, fixes the one open bug in TODO.md that keeps two live
specs `fixme`d, and clears a TODO evaluation item:

- **Part 1** — teach the local store which follows are published, so the
  mirror can prune remote unfollows and the publish offer can never resurrect
  an unfollow made on another device.
- **Part 2** — the `@live` signed-in cookFollow round-trip, clearing the
  recorded assumption that a novel `app.arecipe.*` collection behaves like
  `mealPlan` on the live PDS.
- **Part 3** — stable loopback OAuth `client_id` across pages, un-`fixme`ing
  the `two-tab-live` / `two-device-read` forceRefresh specs.
- **Part 4** — read-and-report: evaluate `pwa-check` for the gate.

NOT in this run (named so it isn't scope-crept in): the Browse cache-first SWR
paint (TODO.md "Ideas"), which is the natural next run now that the default
feed merges starter + followed cooks and a cold load waits on many repos.

## 1. Standing conventions (non-negotiable)

1. **TDD, red first.** Failing tests before implementation in every part;
   fixtures before features; the run summary evidences red → green order per
   part. Implementation without a preceding failing test is a run defect.
2. **Gate green at the end of every part**, not just the run.
3. **Style**: strict vanilla TS, pure cores with injectable deps, DOM wiring
   guarded by e2e, module comments explain why. Browse bundle ships zero auth
   code (existing e2e guard) — nothing in this run may weaken it.
4. **Plan file** `plans/2026-07-XX-N-plan-follow-through.md` before Part 1;
   Status updated at completion, house format.
5. **Live-spec discipline** (Parts 2–3): follow the existing `@live` helper
   conventions in `tests/e2e/helpers/live.ts`; anything a live spec creates it
   deletes, including on failure (teardown in `finally`/afterEach).

## 2. Grounded context (verified in the post-cook-follows snapshot, 2026-07-15)

- `src/social/cook-follows-local.ts`: `LocalCookFollow = { did, handle }`,
  localStorage key `cook-follows`, ops `list/has/add/remove`; `add` is
  idempotent by DID and NEVER overwrites an existing row (first handle wins) —
  so a published-marker needs a new op, not a widened `add`. The read filter
  keeps only rows with string `did` + `handle`; extra fields survive
  round-trip untyped.
- `src/social/cook-follows-pds.ts`: `COOK_FOLLOW_COLLECTION =
  'app.arecipe.cookFollow'`; `followCook(agent, subject)` → createRecord,
  returns `{uri, cid}`; `unfollowCook` resolves rkey by subject then
  deleteRecord; `listCookFollows(target)` → public listRecords (one page,
  `limit=100`); `mirrorCookFollowsDown` currently ADD-ONLY (stores the bare
  DID as the handle placeholder).
- `src/social/cookbook-members-view.ts`: mounts add panel + members list +
  the D6 publish offer (`testid publish-offer`), imports the pds module —
  the only importer, keeping the browse split intact.
- Loopback OAuth: `src/auth/oauth-client.ts` `buildLoopbackMetadata` derives
  `redirect_uri` from `location.pathname`, baking the INITIATING PAGE into the
  loopback `client_id` (`http://localhost?redirect_uri=…&scope=…`). Hence a
  token minted on signin.html cannot refresh on any other page ("Token was not
  issued to this client"); production uses one fixed hosted
  `client-metadata.json` and is unaffected. `two-tab-live.spec.ts:28` and
  `two-device-read.spec.ts:38` carry `test.fixme` pointing at TODO.md.
- TODO.md carries the bug entry (fix directions sketched there) and the
  `pwa-check` evaluation item (<https://github.com/pwa-today/pwa-check>).

Phase 0 of the run: re-ground this section against main (runs may have landed
since); drift = FINDING + adapt details, never the locked decisions.

## 3. Locked design decisions

- **D1 Published marker.** `LocalCookFollow` gains OPTIONAL
  `publishedRkey?: string`. New store op `markPublished(did, rkey)` (upsert on
  the existing row; no-op if the DID is absent); `add` stays first-write-wins.
  A row WITH `publishedRkey` means "this device believes a PDS record exists";
  WITHOUT means "local-only, pending publish".
- **D2 Mirror becomes reconciling, not add-only.** `mirrorCookFollowsDown`:
  (a) upsert every PDS record into the store and `markPublished` it with its
  rkey; (b) PRUNE every local row whose `publishedRkey` is set but whose rkey
  is absent from the PDS list — that follow was deleted remotely (another
  device's unfollow) and must not survive or be re-offered; (c) rows without
  `publishedRkey` are untouched — they are the D6 publish offer's material.
- **D3 Publish is adopt-first, never duplicating.** Publishing a local-only
  row first checks the fresh PDS list for the subject: if a record already
  exists, ADOPT it (`markPublished` with the found rkey, no write); else
  createRecord then `markPublished` with the new rkey (parse from the returned
  `uri`). This makes publish idempotent under double-tap AND migrates
  pre-marker rows (existing stores have no markers; their published rows get
  adopted on first offer or first mirror, not re-created).
- **D4 Follow/unfollow keep the marker true.** Signed-in follow stamps the
  rkey on the local row immediately after createRecord; signed-in unfollow
  removes the local row (as today) — no tombstones.
- **D5 Live spec scope (Part 2).** One `@live` spec file
  (`cook-follows-live.spec.ts`): signed-in follow of a fixture-known DID →
  public `listRecords` on the account's own PDS shows exactly one cookFollow
  record with that subject → unfollow → listRecords shows none. This clears
  the plan-file assumption ("novel app.arecipe.* collection behaves like
  mealPlan") with evidence; note the clearance in the run summary AND flip the
  assumption wording in `docs/LEXICONS.md` if it is recorded there.
- **D6 Loopback fix (Part 3).** Goal: one stable loopback `client_id` for all
  pages so refresh works everywhere in local dev. [verify-in-run] first: can a
  loopback `client_id` carry MULTIPLE `redirect_uri` params (atproto spec
  permits repeated redirect_uri query params for loopback clients — confirm
  against the current spec text and @atproto/oauth-client-browser behavior)?
  If yes: enumerate the app's page redirect_uris in one stable client_id.
  If no: pin the loopback redirect_uri to ONE canonical page (signin.html) and
  preserve return-to-initiating-page through the existing post-auth flow.
  Either way: hosted mode byte-identical (unit-assert it); acceptance = remove
  both `test.fixme` lines and the forceRefresh specs pass `@live`; update the
  TODO.md bug entry to resolved with the chosen direction.
- **D7 pwa-check brief (Part 4).** Read-and-report ONLY, no gate adoption in
  this run. Run it against the built app; report to
  `docs/sources/PWA-CHECK-EVALUATION.md` (or the repo's convention for such
  notes — follow LEXICONS/PROBE-NOTES precedent): what it checks, what it
  caught or would catch that Playwright doesn't, false-positive noise, hermetic
  suitability (offline? deterministic?), and a recommendation (gate / periodic
  / skip) with reasoning. Tick or annotate the TODO.md item.
- **D8 Deferred**: Browse SWR paint (next run); handle resolution for
  mirrored-down rows showing bare DIDs; listRecords pagination past 100
  follows (note as a known bound in code comment if not already).

## 4. Parts

### Part 1 — published marker + reconciling mirror + adopt-first publish
RED `tests/unit/social/cook-follows-local.spec.ts` (extend): `markPublished`
upserts on existing row, no-op on absent DID; marker survives round-trip;
pre-marker stored rows (fixture JSON without the field) still parse.
RED `tests/unit/social/cook-follows-pds.spec.ts` (extend, fetch-fake +
agent-fake per existing patterns): mirror stamps rkeys; mirror prunes a
marked row absent from PDS; mirror leaves unmarked rows alone; publish adopts
an existing subject without createRecord; publish creates + stamps otherwise;
double publish = one record.
RED `tests/unit/social/cookbook-members-view.spec.ts` (extend): the offer
lists ONLY unmarked rows; a pruned row disappears from list and offer.
GREEN: implement D1–D4. Then e2e (extend `cook-follows.spec.ts`, routed
fixtures): a locally-marked follow whose record is absent from the routed
listRecords response is pruned on Account load and never offered.

### Part 2 — @live cookFollow round-trip
RED: write the live spec per D5 (it will fail before Part 1's marker changes
only if they broke something — it is primarily NEW coverage; its red state is
the unimplemented spec). Follow live.ts conventions; guaranteed cleanup; skip
cleanly when live creds are absent (match existing @live gating).
GREEN: passing run recorded in the summary with the listRecords evidence.

### Part 3 — loopback client_id
Probe D6's [verify-in-run] FIRST and record the answer. RED: unit tests on
`buildLoopbackMetadata` — client_id identical across two pathnames; hosted
metadata unchanged; (chosen-direction specifics: multi-redirect enumeration
XOR canonical-page pinning + return-to preserved). GREEN: implement; remove
both `test.fixme`s; run the two live specs; update TODO.md.

### Part 4 — pwa-check brief
No RED/GREEN (read-and-report); the report itself carries the evidence: exact
command, version pinned, raw findings, then the assessment per D7.

## 5. Run summary

`RUN-FOLLOW-THROUGH-SUMMARY.md` (repo root, house style): per part — red →
green evidence, the D6 [verify-in-run] answer and chosen direction, the live
listRecords evidence clearing the lexicon assumption, pwa-check verdict
paragraph, gate results per part, FINDINGS if any, files touched.

## 6. Acceptance criteria (each maps to a named test or artifact)

1. An unfollow made on another device disappears locally after one signed-in
   load and is never re-offered for publish.
2. Publishing is idempotent: double-tap or pre-marker migration yields exactly
   one PDS record per cook.
3. A live signed-in follow/unfollow round-trip is proven against the real PDS
   and the recorded assumption is cleared.
4. Local-dev token refresh works on every authed page; the two live specs run
   un-fixme'd; hosted auth metadata is byte-identical.
5. A pwa-check recommendation exists in docs with evidence.
6. Browse bundle still ships zero auth code; gate green at every part
   boundary.
