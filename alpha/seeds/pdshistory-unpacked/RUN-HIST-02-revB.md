# RUN-HIST-02 rev B - hist-atproto live legs, gentle profile (bsky.social)

Lane: hist-atproto. Supersedes rev A of this brief. Same purpose: turn the lane's named seams from mechanically-proven to live-proven and produce evidence rows for OC-1..5. This revision fixes the host as a **throwaway account on bsky.social** and adds a gentleness contract: we are a guest on shared infrastructure, and the run must be indistinguishable from a small well-behaved client. This run produces evidence, not decisions.

## Environment and credentials

One tier only. The destructive tier from rev A is out of this run entirely.

- Host: bsky.social (Bluesky-hosted PDS). Account: freshly created throwaway, used for nothing else. Auth: app password → `com.atproto.server.createSession`. Do not use OAuth client registration for a spike; do not use the account owner's main password.
- Creds via environment only: `HIST_PDS_HOST`, `HIST_PDS_HANDLE`, `HIST_PDS_APP_PASSWORD`. Never committed, never echoed into logs, fixtures, or the results doc.
- Abort any step whose setup would touch a repo other than the throwaway's own.

## Gentleness contract (MUST, checked in the run summary)

1. **Write budget:** ≤100 record writes and ≤3 blob uploads (each ≤64 KB) for the entire run, including retries. The budget is a counter in the harness; exceeding it is a run failure, not a warning.
2. **Pacing:** ≤1 request/second sustained, single-flight (no concurrent requests). Batch with `applyWrites` where it reduces request count.
3. **Rate limits are data, not obstacles:** on any 429 or rate-limit signal, halt the experiment, record the response (including any rate-limit headers the host returns), and resume after the indicated window or not at all. Never retry-hammer. E8 consumes these observations passively; nothing in this run probes for limits by inducing failures, with the single exception noted in E8.
4. **Public by default, so synthetic by default:** everything written to a bsky.social repo is public and syndicated to relays/firehose consumers. All envelope content MUST be synthetic fixture data, clearly namespaced (`ing.croft.hist.entry`), containing nothing personal, nothing operational, and no strings that would read as spam to a human who stumbles on the record.
5. **Teardown:** at end of run, export the final CAR as the archive of record, then delete all records and created content, leaving the repo empty. Keep the account (reruns), keep the CAR (evidence).
6. **No account-level operations:** no PLC operations, no handle changes, no email/identity churn, no deactivation dance. The account is created once, used quietly, tidied.

## TDD mandate (standing directive)

Unchanged from rev A: acceptance criteria as failing tests before implementation, fixtures before features, red-to-green evidenced per experiment in the run summary. An experiment whose test never demonstrably failed is UNPROVEN regardless of final color. Live calls sit behind the `LiveLeg` trait; the stub impl runs in CI, the live impl is cred-gated; record live responses as fixtures on first green so the suite replays offline.

## Standing MUST-NOTs

- Fold-from-repo-order is a MUST-NOT everywhere except E6's labeled negative control. Firehose seq, commit rev, and rkey enumeration are delivery cursors, never order (GROUPS v2 row 11).
- No PDS-returned state is fold input without local verification (envelope hash chain first, repo commit signature second).
- No spec numbers from memory in the summary: cite HIST-ATPROTO-MATCHUP.md anchors or a measurement made here, and mark which is which.

## Experiments

### E1 - Canonical form and CID identity
**Question:** are Drystone canonical dag-cbor bytes and the stored record bytes identical, i.e. can the record CID serve directly as the reconciliation byte-head (OC-2)?
**Fixtures first:** three canned synthetic envelopes with locally computed canonical encodings and CIDs.
**Failing test:** `local_cid == pds_record_cid` and `local_bytes == sync_block_bytes` per fixture; red against stub.
**Procedure:** write via `com.atproto.repo.applyWrites` (one batch, three creates); read back via `com.atproto.repo.getRecord` (JSON path) and `com.atproto.sync.getRecord` (proof path, raw blocks); compare.
**Pass:** identity holds, OR the divergence is captured precisely (which fields, what re-encoding) so OC-2 gets a re-hash-at-boundary cost instead of a guess. Unexplained mismatch is a fail.
**Budget:** ~3 writes, ~6 reads.

### E2 - rkey order is the reconciliation index
**Question:** does (hashed subspace, zero-padded counter) as rkey make `listRecords` enumeration the sorted reconciliation index, stable across pagination?
**Failing test:** for randomized write order of **N=36 envelopes across 2 subspaces**, cursored `listRecords` pages (page size 7) concatenate to exact bytewise-sorted rkey order; chain-gap detector over pages finds zero gaps on the complete set.
**Procedure:** shuffle write order; write in `applyWrites` batches of 6 to stay inside pacing; enumerate with small pages to force many cursor seams. Two shuffles maximum live (property-test the shuffle space exhaustively against recorded fixtures offline instead of re-writing live).
**Pass:** property holds on both live shuffles and the offline replay set; page boundaries show no duplication or omission.
**Budget:** ~72 writes worst case across both shuffles - this is the budget's main consumer; if the first shuffle is green and fixtures are recorded, the second live shuffle MAY be skipped in favor of offline permutation testing, reported as such.

### E3 - CAR export as store re-hydration
**Question:** is `com.atproto.sync.getRepo` (CAR) sufficient to rebuild the helper store from nothing?
**Failing test:** (a) commit signature verifies against the signing key resolved from the account's DID document; (b) a fresh store re-hydrated from CAR yields byte-identical envelopes and identical fold state to the store that did the writing.
**Procedure:** export after E2's writes; verify; re-hydrate into a second store instance; diff structurally.
**Pass:** both green; fold-state equality asserted structurally, not by log inspection.
**Budget:** ~2 reads (CAR + DID doc).

### E4 - Blob tier and the publicity seam (GC portion observational)
**Question:** does the sealed-blob path behave as mapped - upload, referencing public record, and does a dereferenced blob become unfetchable (the GC fact that forces the public-marker tier)?
**Failing test:** upload one small blob (≤64 KB) via `uploadBlob`; assert fetch behavior via `com.atproto.sync.getBlob` before a record references it and after the referencing record lands.
**Procedure:** then delete the referencing record and probe `getBlob` at 0, +10m, +1h, and at teardown. Four probes total; no tighter polling.
**Pass:** pre/post-reference behavior asserted; GC-after-deref reported as timestamped observation only, tagged with host and date - implementation-defined behavior, not generalized. Feeds OC-4/OC-5.
**Budget:** 1 blob, ~2 writes, ~6 reads.

### E5 - Omission and deletion visibility
**Question:** when a mid-chain record is deleted, does completeness degrade while correctness holds?
**Failing test:** after deleting the record at counter k (one of E2's, chosen mid-chain): (a) chain-gap detector fires at k from cursored enumeration AND from a fresh CAR export; (b) fold over remaining facts is unchanged for all conclusions not depending on k; (c) nothing interpolates or silently skips the gap.
**Pass:** all three; the gap surfaces as a completeness-ahead signal, never patched.
**Budget:** 1 delete, ~3 reads.

### E6 - Order-divergence negative control
**Question:** live proof that repo/commit order and reconciliation order genuinely diverge, and that our fold is immune.
**Failing test:** write counters {5,3,4} of a fresh subspace in that wall-clock order (three separate single-record commits, deliberately not batched, so commit order is observable); assert commit order differs from rkey order; assert fold-by-antecedent-hashes yields correct state; assert a quarantined canary fold-by-commit-order yields observably different state.
**Pass:** divergence demonstrated and the canary is wrong in the expected way. Canary lives in the test module only, marked `NEGATIVE-CONTROL`, never in a shipping path.
**Budget:** 3 writes, ~4 reads.

### E7 - Scribe removal via PLC rotation: **DEFERRED**
Not runnable gently on a hosted account and not attempted here. No PLC operations on bsky.social in this run. Deferred to a future self-hosted run (rev A's T2 design carries forward unchanged). Reported as SKIPPED(deferred-self-host) in the results table so OC-1/OC-3 show an honest evidence gap rather than silence.

### E8 - Limits, passively
**Question:** what limits does the host document and what does normal traffic reveal, versus the anchors in HIST-ATPROTO-MATCHUP.md Part A?
**Method:** documentation-cited numbers (with sources) side by side with passively observed data: any rate-limit headers on ordinary responses, any 429s encountered under the gentleness contract, and record sizes actually accepted during E1–E6. **One** deliberately oversize record write is permitted as the sole induced failure of the run, to confirm it fails cleanly and does not corrupt subsequent chain writes (post-failure chain-gap check green); it counts against the write budget.
**Pass:** table complete with documented / observed / source columns; no probe-to-failure beyond the single permitted write.

## Deliverables

1. `spike/hist_live/`: `LiveLeg` trait, stub + live impls, fixtures, all acceptance tests, budget-and-pacing harness with its counters asserted in tests.
2. `HIST-LIVE-RESULTS.md`: per-experiment verdicts (GREEN / RED / SKIPPED(deferred-self-host) / UNPROVEN / OBSERVATIONAL), red-to-green evidence, the E8 limits table, budget ledger (writes/reads/blobs consumed vs cap), teardown confirmation, and the OC-1..5 evidence map (one line each, no recommendations).
3. Recorded fixtures for offline replay; the teardown CAR as archive of record.

## Out of scope

Everything rev A excluded, plus: PLC operations of any kind, OAuth client work, firehose subscription (public relay load for a spike is not gentle and RUN-14 machinery is reused by reference), and any write outside the throwaway repo.
