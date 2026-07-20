# RUN-HIST-01 summary — PDS-backed history store: the HIST-ATPROTO matchup brief + the mechanical-seam spike

`Own-lane run (HIST, renumberable to mainline later; attest-lane precedent),
executed 2026-07-20 against the instruction file "RUN-HIST-01 — PDS-backed
history store". Serves beta/impl/drystone-design/history-durability.md (§E/§G/
§I/§J/§L) and rbsr-construction.md (reqs. 3, 5). Nothing dropped: Part A
(brief, 11 rows + lexicon sketches + ordering rider + OC register), Part B
(spike, B1–B7 red-first + attended live legs), FINDINGS (F-HIST-1), registers,
site gate all executed. I9 firewall held: NO trust tier decided, NOTHING
frozen, all five HS OC tags pending.`

## What landed

- **Part A** — `HIST-ATPROTO-MATCHUP.md` (alpha/experiments/, beside
  ATTEST-ATPROTO-MATCHUP.md): eleven-row required-abilities inventory, every
  current-ATProto claim anchored to a primary source **fetched in-session
  2026-07-20** (repository / record-key / blob / sync / lexicon / event-stream
  / data-model / xrpc specs, the **live listRecords lexicon JSON**, the did:plc
  spec, the account-migration guide; hosted-PDS operational limits from
  operator docs, `[confirm]`-graded — no import markers needed, network was
  available). Carries: the **row-1 publicity fact stated without softening**
  (blobs on the PDS require a referencing record, so some index record is
  public repo content — no PDS-resident-blob configuration publishes nothing);
  the **row-3 named gap** (the current lexicon has `cursor`/`limit`/`reverse`
  and NO `rkeyStart`/`rkeyEnd`-style bounds — verified against the lexicon
  JSON, not assumed); the **row-7 hist-specific delete bound** (retained
  residue is ciphertext plus envelope — metadata-shaped harm, not
  content-shaped, and permanent for whoever mirrored first); the **ordering
  rider** written once and referenced from rows 2/9; draft
  `ing.croft.hist.entry` / `ing.croft.hist.checkpoint` lexicon sketches
  (checkpoint SHAPE only — §L's construction stays open, said in the sketch);
  the mechanical no-lexicon-home list; and the five-entry **HS OC register**
  (§6), all pending.

- **Part B** — `hist-atproto-spike/` (new crate): B1–B7 green red-first (23
  integration tests + 2 compile_fail doc-tests, clippy-clean). Reused, never
  shadowed: `lineage-history`/`lineage-core` path deps (the **A2.3
  `backfill_import` admission — standing plus contiguity — used unmodified**
  by B6), `serde_ipld_dagcbor`+`ipld-core` (the proven canonical dag-cbor
  path). No openmls (pure workspace, L2a precedent). The CARv1 codec is an
  **in-crate fixture-grade construction** (the named dependency choice for
  B5: varint-framed CARv1 layout, dag-cbor header, root-index block; not a
  wire pin). Spike-local byte choices documented at their definition sites:
  20-digit zero-padded counter rkey + 8-byte hash prefix
  (`[gates-release]`-adjacent), single-char-keyed canonical envelope map,
  blessed-shaped blob CIDs (raw+sha-256) carried **beside** blake3
  `entry_digest` (HS OC-2 left un-fused).

- **Live legs (attended, creds-supplied precedent RUN-14)** — run against the
  **real bsky.social PDS** with user-supplied credentials (env-only, never in
  the tree): uploadBlob → temporary-storage entry state observed; re-upload
  of identical bytes → **identical blob CID asserted, before AND after a
  referencing record existed** (the no-op claim); the mandatory referencing
  record created in the `ing.croft.hist.entry` collection — the live PDS
  accepted the padded 37-char rkey
  (`…/ing.croft.hist.entry/8d4aca5ae305ad05-00000000000000000000`) — then
  deleted (cleanup; blob back to unreferenced, GC-eligible). Absent creds the
  leg **skips with a named reason** (`SKIP(live): …`), never a silent pass
  (skip run digest-attested below). Service-auth was NOT re-proven (row 5
  cites RUN-14 EXP-A by reference).

- **FINDINGS** — `hist-atproto-spike/FINDINGS.md`: **F-HIST-1** (per-group
  DID announces group existence to a public, permanent, enumerable directory
  — creation time, scribe-supersession timeline, hosting fingerprint; the
  F-AT-6 extension filed in this lane, not by editing the attest lane's
  file).

- **Registers** — MASTER-INDEX row for the lane; EVIDENCE-MAP **section E
  (HIST band)** with one row per B-assertion + the live leg;
  EXPERIMENT-BACKLOG **§6h** (full GC-window observation parked ≥1 h;
  thin-layer bounded-range lexicon; store-side RBSR build item; chunking
  still open with the 50 MB operator bound as its forcing number).

**Status tags:** everything lands `Modeled` except the live-leg row
(`Verified`, narrow live-observation bounds). B6 cites the reused A2.3
machinery at its own grade (Proofs Phase 2.5); the spike models on top.

## Red → green evidence (digest-attested scratchpad outputs, not committed)

One RED batch against the staged seams, then the green swap — every staged
seam marked `STAGED-RED` in source and deleted at green:

- **RED (full suite, `--no-fail-fast`, 2026-07-20)** — 15 failures across all
  seven assertion sets, each the designed failure mode: B1 round-trip caught
  the dropped `sizeHint`; B2 caught the unpadded encoding at the 9→10
  boundary; B3's gap test caught the naive last-page consumer calling a
  gapped chain complete; B4 all three caught the order-sensitive
  (append-in-arrival-order) fold; B5 caught the silent skip of a missing
  block; B6 caught the signature-deep admission accepting a withheld span, a
  re-signed payload-gap span, and a stranger's well-signed history; B7 caught
  the ungated prune. sha256 `585213d52911710f…` (`hist01-red-run-full.txt`).
- **GREEN (post-swap)** — sha256 `33d9197d4db78d44…`
  (`hist01-green-run-full.txt`); **final green after clippy cleanup** (0
  warnings): 23 integration + 2 compile_fail doc-tests, 0 failed. sha256
  `f11c2dc193d99d54…` (`hist01-green-final.txt`).
- **Live skip (creds absent)** — the named-reason skip captured. sha256
  `88b7475ac03a0524…` (`hist01-live-skip.txt`).
- **Live attended (creds supplied)** — the observations above. sha256
  `e14d3e9fa957b32c…` (`hist01-live-attended.txt`). No secret appears in the
  log (session response bodies are not printed).

**Red cascades (documented, not separate stagings):** staged B1 also redded
B5's convergence-equality assertion (re-hydrated envelopes had lost
`sizeHint`), and staged B2 also redded B3's complete-chain ordering (unpadded
rkeys page out of counter order) — each a real downstream consequence of the
named staged seam.

**Green at birth, stated honestly:** the permitted-path and fixture
assertions were green at the red stage by design — B1's malformed-rejection +
canonical round-trip, B3's stateless-responder pin, B5's codec round-trip,
B6's honest-span admission and `tampered_ordering` (the staged form did
verify signatures, so the signature-binding rejection already held), B7's
gated-prune-permitted path (the staged form pruned unconditionally). The
2 compile_fail doc-tests (the delivery-cursor type boundary) were green from
birth — the boundary is compile-level and cannot be red-staged without
removing the type.

## Test map

| Set | Tests | Result |
|---|---|---|
| B1 | `envelope_record_roundtrip_is_lossless_bytewise` (per-entry byte equality over canonical encoding; blob CID beside entry_digest, un-fused, HS OC-2) · `malformed_records_are_rejected_whole_never_repaired` · `envelope_canonical_bytes_roundtrip` | 3/3 green |
| B2 | `rkey_lexicographic_order_equals_subspace_counter_order` (all pairs over 4 subspaces × every decimal boundary 9/10 … 10¹⁸ + u64::MAX) · `rkey_stays_inside_the_record_key_contract` (alphabet, ≤80, deterministic) | 2/2 green |
| B3 | `gap_is_named_by_bounding_digests_across_pages` (entry 11 of 23 withheld, pages of 5 — the §I bounding digests exactly) · `complete_chain_across_pages_is_complete` · `responder_is_stateless_and_cursor_driven` (replay-identical, interleaved consumers, cursor-end convention) | 3/3 green |
| B4 | `delivery_permutations_fold_to_identical_state` (chain/reversed/strided → one digest) · `folded_chains_are_in_predecessor_order_not_arrival_order` (worst-case reverse arrival) · `missing_link_stays_pending_never_guessed_from_delivery_order` + 2 compile_fail doc-tests (cursor value unreadable; cursor unorderable) | 3/3 + 2 doc |
| B5 | `full_car_rehydrates_to_convergence_equality` (two subspaces, digest-identical state) · `missing_block_is_named_never_silent` (named by CID) · `car_roundtrip_preserves_blocks` | 3/3 green |
| B6 | `honest_span_is_admitted_and_yields_the_chain` · `withheld_entry_with_valid_signatures_is_rejected_noncontiguous` (located at index 4/seq 5) · `compliant_transport_omission_is_named_by_bounding_digests` (compromised-scribe worst case: re-signed contiguous transport, payload chain names the omission) · `strangers_wellsigned_history_is_rejected_unauthorized` · `tampered_ordering_fails_signature_binding` | 5/5 green |
| B7 | `prune_without_checkpoint_is_rejected` · `prune_with_verifiable_checkpoint_is_permitted_and_bounded` · `checkpoint_must_cover_the_requested_prune` (under-covering + cross-subspace) · `unverifiable_marker_gates_closed` (tampered commitment + wrong anchor) — gate only, §L construction OPEN, said in the test doc-comment | 4/4 green |
| Live | `live_blob_lifecycle_observation` (attended) / named skip otherwise | observed |

## The five HS OC tags — all pending, with locations

| Tag | Question | Locations (doc · code) |
|---|---|---|
| HS OC-1 | repo ownership: service DID vs per-group DID (per-group pays F-HIST-1) | matchup §6 · `src/lib.rs` register · FINDINGS.md |
| HS OC-2 | reconciliation identity: `entry_digest` ≡ blob CID vs separate | matchup §6 · `src/lib.rs` + `src/record.rs` + `tests/b1_roundtrip.rs` |
| HS OC-3 | scribe key custody + PLC rotation-key holders | matchup §6 · `src/lib.rs` + `tests/live_legs.rs` |
| HS OC-4 | envelope posture default (opt-in vs opt-out per scope) | matchup §6 · `src/lib.rs` register |
| HS OC-5 | sealed-posture backend shape (row 11's a/b/c) | matchup §6 (row 11) · `src/lib.rs` + `lexicons/README.md` |

## Deviations from the instruction file (with reasons)

1. **"GROUPS.md v2 row 11" resolves to no numbered table row** in the current
   GROUPS.md (the A.9 tier table has nine rows). The delivery-cursor rule the
   brief means is the A.7 sentence ("any sequence numbers a role assigns are
   delivery cursors, never order"); cited as **A.7** throughout, flagged here
   rather than silently absorbed.
2. **The live legs gained a record create/delete cycle** beyond the two named
   observations, because the anchored re-upload no-op claim is stated for a
   *referenced* blob — confirming it honestly required the mandatory
   referencing record (which is itself the row-1 fact made mechanical). Strict
   cleanup: the record was deleted in the same run. Full GC observation
   (deletion after the ≥1 h grace floor) is out of single-session scope by
   construction — parked in backlog §6h with the recipe.
3. **The lexicon sketches carry no string vocabularies**, so the
   enum-never-knownValues rule had nothing to bind to; it is recorded in
   `lexicons/README.md` so a later field addition inherits it.
4. **B4's structural impossibility is compile-level, so it was green at
   birth** (2 compile_fail doc-tests); the behavioral half (permutation
   equality) carried the red. The brief allowed "type-level or asserted" —
   both are present.
5. **Red cascades** (B1→B5, B2→B3) meant two green-path tests redded under
   another set's staging; documented above rather than staged separately.

## Definition of done — checklist

- Part A: 11 rows, all anchored in-session (2026-07-20) or `[confirm]`-marked
  with the question written out (row 10's operator values): **yes**.
- Row 5 cites RUN-14 EXP-A with no new proving: **yes** (no new anchor on
  that row).
- Row 1 states the publicity consequence without softening: **yes**.
- Ordering rider written once, referenced rows 2/9, B4 renders it: **yes**.
- Part B: B1–B7 green red-first, digests above; B4's staged order-sensitive
  fold captured failing in the scratchpad record: **yes**.
- Live legs skip named / attended-confirmed: **yes** (both digests above).
- F-HIST-1 filed in this lane, attest lane untouched: **yes**.
- All five HS OC tags pending, doc + code: **yes** (table above).
- Zero edits outside the lane + registers; frozen records byte-untouched:
  **yes** (new files only, plus MASTER-INDEX / EVIDENCE-MAP §E /
  EXPERIMENT-BACKLOG §6h rows).
- Crate clippy-clean: **yes** (0 warnings, all targets).
- Site gate `site/build.py`: **run and green** (86 pages, resolver suite OK;
  the new docs live outside the published spec set — run anyway per
  precedent).
