# hist-atproto-spike — the PDS-backend mechanical seam (RUN-HIST-01 Part B)

`Serves: ../../beta/impl/drystone-design/history-durability.md (§G envelope,
§I/§J convergence, §L pruning/checkpoints) and
../../beta/impl/drystone-design/rbsr-construction.md (req. 3 stateless
responder, req. 5 omission resistance), via the Part A brief
../HIST-ATPROTO-MATCHUP.md. Own lane (HIST), renumberable; attest-lane
precedent. Grade: everything Modeled; no wire encoding pinned.`

Seven assertion sets, each landed red-first (digest-attested scratchpad
outputs; see RUN-HIST-01-SUMMARY.md):

| set | assertion | test |
|---|---|---|
| B1 | §G envelope ↔ `ing.croft.hist.entry` record, lossless bytewise | `tests/b1_roundtrip.rs` |
| B2 | rkey lexicographic order ≡ (subspace, counter) order, incl. the padding boundary | `tests/b2_rkey_order.rs` |
| B3 | chain gap over cursored listRecords-shaped pages named by §I bounding digests | `tests/b3_gap_pages.rs` |
| B4 | delivery-order permutations fold identically; cursor structurally unreadable | `tests/b4_fold_order.rs` + `src/delivery.rs` compile_fail |
| B5 | CAR re-hydration converges; missing block NAMED, never silent | `tests/b5_car_rehydrate.rs` |
| B6 | omission detected regardless of responder honesty; admission = reused A2.3 machinery | `tests/b6_omission.rs` |
| B7 | prune only above a present, verifiable checkpoint marker (§L construction stays OPEN) | `tests/b7_prune_guard.rs` |

Live legs (`tests/live_legs.rs`) are attended-optional (RUN-14 creds-supplied
precedent): env-gated (`HIST_SPIKE_LIVE=1`, `PDS_URL`, `PDS_IDENTIFIER`,
`PDS_PASSWORD`), skipping with a named reason absent creds — never a silent
pass. They observe the blob-lifecycle claims of matchup row 1 (temporary
storage on upload, re-upload no-op before and after referencing, the
mandatory referencing record, last-reference cleanup). Service-auth is NOT a
live leg (RUN-14 EXP-A, cited by reference).

Reused machinery (path deps, never shadowed): `lineage-history` /
`lineage-core` (the A2.3 backfill admission — standing plus contiguity),
`serde_ipld_dagcbor` + `ipld-core` (the proven canonical dag-cbor path). No
openmls (pure-workspace, L2a precedent). The CARv1 codec is an in-crate
fixture-grade construction (named dependency choice, `src/car.rs`).

All five `OWNER-CALL: HS OC-n pending` tags are carried in `src/lib.rs` (the
register), with in-place tags at their seams (`src/record.rs` OC-2,
`tests/live_legs.rs` OC-3, `lexicons/README.md` OC-5). Decisions belong to
the owner walk, not this crate.

FINDINGS: `FINDINGS.md` (F-HIST-1 — per-group DID enumerability).
