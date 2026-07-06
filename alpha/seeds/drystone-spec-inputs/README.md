# Drystone spec — frozen raw inputs (dropoff batches 1 & 2)

date: 2026-07-06 (frozen when the `beta/drystone-spec/dropoff/` scratch area was cleaned up)

**What this is.** The **pre-edit raw** of the Drystone spec as it first arrived, preserved here before
the transient `dropoff/` staging area was removed. These files are the inputs *before* the in-tree
editing passes (document-pass-1, -2, -3), so they differ from the committed
`beta/drystone-spec/part-1…/part-2…` — that is exactly why they are kept: the committed tree carries the
edited spec, and git history carries the per-pass diffs, but the pristine as-received inputs live only
here.

- `batch1-files.zip/` — first drop (`files.zip`): `drystone-part1.md`, `drystone-part2.md`,
  `drystone-changelog.md`, `drystone-review-handoff.md`.
- `batch2-second.zip/` — second drop (`second.zip`): `part1-drystone.md`, `part2-drystone.md`,
  `open-items-drystone.md`, `session-summary-drystone.md`.

**Preservation status: preserved-verbatim** (byte-identical to the dropoff originals; `diff -rq`
confirmed at freeze time). Do not edit — this is a frozen provenance floor (PLAYBOOK §4).

**Not preserved here:** the third and fourth dropoff batches. Batch three (peer-standing set) and batch
four (activism set + spec pass-3) were filed **byte-identical** into the tree, so their committed copies
*are* the verbatim record and their zips were removed without a separate frozen seed (see the
2026-07-06 manifest entries). The only batch-four file not carried into the tree,
`drystone-part1-voice-bridge.md`, was the superseded standalone draft of Part 1 §2.6 and was
intentionally not preserved.
