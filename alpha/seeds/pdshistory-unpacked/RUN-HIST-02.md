# RUN-HIST-02 — hist-atproto live legs (PDS credentials)

Lane: hist-atproto. Follows RUN-HIST-01 (Part A HIST-ATPROTO-MATCHUP.md, Part B mechanical-seam spike crate). This run executes the live legs that Part B left attended-optional, against a real PDS with real credentials. Renumberable.

Purpose: turn the six seams already named in the lane from mechanically-proven to live-proven, and produce evidence rows for open owner calls OC-1..5 (repo ownership, digest-vs-CID identity, scribe key custody, envelope posture default, sealed-posture backend). This run produces evidence, not decisions. OC-1..5 remain HS owner calls.

## Environment and credentials

Two tiers. Every experiment declares its tier.

- **T1 (hosted-safe):** a throwaway account on a hosted PDS. Non-destructive to the PDS itself; destructive only to the throwaway repo's own records. Auth via app password → `com.atproto.server.createSession`.

- **T2 (destructive):** a self-hosted PDS (appview-infra kit box acceptable) where we control the instance and, for E7, hold the did:plc rotation keys. PLC rotation, account-level operations, and GC probing live here only.

Credential rules (MUST):
- Creds arrive via environment only: `HIST_PDS_HOST`, `HIST_PDS_HANDLE`, `HIST_PDS_APP_PASSWORD` (T1); `HIST_PDS2_*` equivalents plus `HIST_PLC_ROTATION_KEY` (T2). Never committed, never echoed into logs or run summaries, never written to fixtures.
- Throwaway identities only. No personal or production account. Abort any experiment whose setup would touch a repo not created by this run.
- T2 experiments run only when `HIST_LIVE_DESTRUCTIVE=1` is set. Absent that flag, they are skipped and reported as SKIPPED(T2-gate), not silently green.
- Respect the host's rate limits; on 429, back off and record the observed limit as data (feeds E8), do not retry-hammer.

## TDD mandate (standing directive, applies to every part)

Acceptance criteria are encoded as failing tests before any implementation or live call. Fixtures (canned envelopes, expected CIDs, expected orderings) land before features. Each experiment's test is written to fail against a stub harness first; the live client is the implementation that turns it green. The run summary MUST evidence red-to-green order per experiment (failing-run output or commit sequence). An experiment whose test never demonstrably failed is reported as UNPROVEN regardless of final color.

Live calls are wrapped behind a `LiveLeg` trait so the same acceptance tests run against the recorded-fixture stub (CI-safe) and the live PDS (cred-gated). Record live responses as fixtures on first green so the suite replays offline afterward.

## Standing MUST-NOTs

- Fold-from-repo-order is a MUST-NOT everywhere except E6's explicitly labeled negative control. Firehose seq, commit rev, and rkey enumeration are delivery cursors, never order (GROUPS v2 row 11).
- No experiment may treat PDS-returned state as authoritative fold input without local verification (envelope hash chain first, repo commit signature second).
- Do not state spec numbers (size limits, rate limits) from memory in the summary; cite HIST-ATPROTO-MATCHUP.md anchors or the measurement made here, and mark which is which.

## Experiments

### E1 — Canonical form and CID identity (T1)
**Question:** are Drystone canonical dag-cbor bytes and the PDS's stored record bytes identical, i.e. can the record CID serve directly as the reconciliation byte-head (OC-2)?
**Fixtures first:** three canned envelopes with locally computed canonical encodings and CIDs.
**Failing test:** assert `local_cid == pds_record_cid` and `local_bytes == sync_block_bytes` for each fixture; red against stub.
**Procedure:** write each envelope as `ing.croft.hist.entry` via `com.atproto.repo.applyWrites`; read back via `com.atproto.repo.getRecord` (JSON path) and via `com.atproto.sync.getRecord` (proof path, raw blocks); compare bytes and CIDs.
**Pass:** byte-identity and CID-identity hold, OR the divergence is captured precisely (which fields, what re-encoding) so OC-2 gets a re-hash-at-boundary cost instead of a guess. Either outcome is a valid result; an unexplained mismatch is a fail.

### E2 — rkey order is the reconciliation index (T1)
**Question:** does (hashed subspace, zero-padded counter) as rkey make `listRecords` enumeration the sorted reconciliation index, stable across pagination?
**Failing test:** property test — for randomized write order of N≥200 fixture envelopes across ≥3 subspaces, cursored `com.atproto.repo.listRecords` pages concatenate to exact bytewise-sorted rkey order; chain-gap detector over pages finds zero gaps on the complete set.
**Procedure:** write out of order deliberately (shuffle), enumerate with small page sizes (e.g. 7) to force many cursors.
**Pass:** property holds across ≥5 shuffles; page-boundary seams show no duplication or omission.

### E3 — CAR export as store re-hydration (T1)
**Question:** is `com.atproto.sync.getRepo` (CAR) sufficient to rebuild the helper store from nothing — store as cattle, not pet?
**Failing test:** given the CAR bytes, (a) commit signature verifies against the signing key resolved from the account's DID document, (b) a fresh store re-hydrated from CAR yields byte-identical envelopes and identical fold state to the store that did the writing.
**Procedure:** export after E2's writes; verify; re-hydrate into a second store instance; diff.
**Pass:** both assertions green; fold state equality is asserted structurally, not by log inspection.

### E4 — Blob tier and the publicity seam (T1, GC portion observational)
**Question:** does the sealed-blob path behave as mapped — blob upload, referencing public record, and does an unreferenced blob become unfetchable (the GC fact that forces the public-marker tier)?
**Failing test:** upload blob via `com.atproto.repo.uploadBlob`; assert unfetchable-or-pending before a record references it, fetchable via `com.atproto.sync.getBlob` after the referencing record lands.
**Procedure:** then delete the referencing record and probe `getBlob` on a schedule (0, 10m, 1h, 24h if run window allows).
**Pass:** the pre/post-reference behavior is asserted; GC-after-deref is reported as an observation with timestamps (GC timing is implementation-defined — record the PDS implementation and version, do not generalize). Feeds OC-4/OC-5.

### E5 — Omission and deletion visibility (T1)
**Question:** when a mid-chain record is deleted, does completeness degrade while correctness holds?
**Failing test:** after deleting the record at counter k: (a) chain-gap detector fires at k from cursored enumeration AND from a fresh CAR export; (b) fold over the remaining facts is unchanged for all conclusions not depending on k; (c) nothing in the pipeline silently interpolates or skips the gap.
**Pass:** all three; the gap is surfaced as a completeness-ahead signal, never patched.

### E6 — Order-divergence negative control (T1)
**Question:** live proof that repo/commit order and reconciliation order genuinely diverge, and that our fold is immune.
**Failing test:** write counters {5,3,4} in that wall-clock order; assert commit order (from repo commit history / firehose if attended) differs from rkey order; assert fold-by-antecedent-hashes yields correct state; assert a quarantined canary fold-by-commit-order yields observably different state.
**Pass:** divergence demonstrated and the canary is wrong in the expected way. The canary lives in the test module only, marked `NEGATIVE-CONTROL`, and is deleted from no shipping path because it was never in one.

### E7 — Scribe removal via PLC rotation (T2, gated)
**Question:** does the scribe-removal story work end to end — rotate the repo signing key, successor re-hydrates and continues, history still verifies?
**Failing test:** (a) pre-rotation commits verify against old key; (b) post-rotation write by successor verifies against new key resolved from updated DID doc; (c) full CAR spanning the rotation re-hydrates into one continuous store; (d) old signing key can no longer produce an accepted commit.
**Procedure:** on the T2 PDS with held rotation keys, perform the did:plc operation, then successor writes counter n+1.
**Pass:** all four. Feeds OC-1 and OC-3 directly.

### E8 — Limits, empirically (T1, plus T2 for host comparison)
**Question:** what are the real record-size, blob-size, and write-rate ceilings on the target hosts, versus the numbers anchored in HIST-ATPROTO-MATCHUP.md Part A?
**Failing test:** a table-driven probe asserting each documented limit is reproduced within tolerance or the divergence is recorded; oversize write must fail cleanly and must not corrupt subsequent chain writes (post-failure chain-gap check green).
**Pass:** table complete with measured vs documented columns and source citation per documented number.

## Deliverables

1. `spike/hist_live/` crate or module: `LiveLeg` trait, stub + live impls, fixtures, all acceptance tests.
2. `HIST-LIVE-RESULTS.md`: per-experiment verdict table (GREEN / RED / SKIPPED(T2-gate) / UNPROVEN / OBSERVATIONAL), red-to-green evidence per experiment, measured-limits table, and an OC-1..5 evidence map (which experiment feeds which open call, one line each, no recommendations).
3. Recorded fixtures so the suite is replayable offline.

## Out of scope

Sealed-tier MLS integration, the three-rung fetch menu beyond what E1–E3 touch, firehose-based live tailing as a delivery path (RUN-14 service-auth machinery is reused by reference, not re-proven), and any OC-1..5 decision.
