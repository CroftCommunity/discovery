# Part A — fork + deterministic reconcile, demonstrated on three real partitioned peers

date: 2026-06-15

**Claim under test** (META-TRANSCRIPT §Part A): membership changes never require a superpeer; two
(or three) fully-disconnected peers can independently compute the *same* surviving state from only
the histories they hold — complementary divergence converges, contradictory divergence is detected,
preserved, and attributed but **never auto-resolved**. If they can, the decentralization claim is
real.

**Verdict: demonstrated.** Moved from argued-in-simulation to demonstrated across three genuinely
separate machines (AWS us-east-1a/1b/1c, different instance types), with no superpeer and no orderer.

## Method (history-exchange across real machines)

The reconcile logic lives in the (TDD-tested) `lineage-core` crate (`gov`, `conflict`, `survivor`,
`detect_equivocation`). A throwaway, TDD-exempt driver — `crates/reconcile-harness` — wires it to a
CLI: `apply` builds a fixed, deterministic shared world (same admins/founders/key-seeds → identical
`GenesisId` and reproducible Ed25519 signatures on every box, nothing secret exchanged), applies one
signed membership op, and emits the op-log as JSON. `reconcile` re-loads N exchanged logs, replays
each locally, and runs `detect` / `select_survivor` / `detect_equivocation`.

- **Partition** = each box applies its op in isolation (it never sees the others' ops).
- **Reconnect** = the op-logs are exchanged (collected to the Mac, redistributed to every box).
- **Determinism proof** = each box runs `reconcile` *locally* and the emitted verdict JSON is
  compared byte-for-byte across machines. Identical verdict ⇒ each peer computed the same outcome
  from only the histories it holds.

Shared world: admins `{alice,bob,carol,dave}` (so an honest partition uses **disjoint** signer sets
per branch — a real contradiction is not mislabelled as one admin equivocating), founders
`{alice,bob,carol,dave,erin}`, `Remove` threshold 2, `Add` threshold 1.

Scenario (3-way fork, each a different op, mutually partitioned):
- **node-1** `add frank` (signer carol) — keeps erin
- **node-2** `remove erin` (signers alice,bob) — boots erin
- **node-3** `add grace` (signer dave) — keeps erin

## Results

**Foundation — shared genesis identical on all three machines:**
`4f8c1f28c517171172ed85d195b4aeaf0e03166f54775e780479e6712c4162bc` (INV-ANCESTRY: the anchor is
stable across peers, computed independently on each box).

**A1b — 3-way verdict byte-identical across all three independent machines** (canonical order):
`sha256 = 5d82a5df4890c7aaa2811bf866bae6b43188d3c1323e168c7c457861e819bf37` on node-1, node-2, AND
node-3 (`part-a-evidence/verdict-sha-per-box.txt`). The verdict (`part-a-evidence/verdict-canonical.json`):
- `fork_detected: true`, `distinct_heads: 3` — all three branches diverged.
- `contradiction: true`, `contested: ["erin"]` — `RemovedThenIncluded(erin)` on the node-1↔node-2
  and node-2↔node-3 pairs; node-1↔node-3 is `Heal` (both kept erin — complementary).
- `survivor_order_independent: true`; `equivocations: []` (honest disjoint-signer partition).
- The losing branch is **preserved + attributable**: every branch's ops (kind, subject, signers)
  are retained in the verdict; the detector changed no one's membership.

**A1 — 2-way:**
- contradiction (node-1 keeps erin | node-2 boots erin): `HardStop` · `RemovedThenIncluded(erin)`.
- complementary (node-1 adds frank | node-3 adds grace, both keep erin): `Heal`, deterministic survivor.

**Reconnect/merge-order independence** (the semilattice check): survivor `241d2718…` and contested
`erin` were invariant across all four merge orders tested — `[1,2,3]`, `[3,2,1]`, `[2,3,1]`,
`[2,1,3]`. A 2-way tiebreak can look deterministic by luck; a 3-way fork that converges regardless
of order is real evidence the join is order-independent.

## What this proves — and what it does not

**Proves:** the reconcile *computation* is deterministic and identical across genuinely separate
machines; contradictions hard-stop with both branches preserved and attributed and are never
auto-resolved; complementary divergence converges; convergence is merge-order-independent — all with
**no superpeer present**. This is the decentralization claim, demonstrated.

**Does not prove (honesty boundary):** the "partition/reconnect" is modelled as op-log file exchange,
not a real network partition over a live transport — the *computation* is on real separate machines,
the *transport* is not exercised here (that is Part B's iroh work and the lineage-iroh Phase 3
caveat). MLS epoch key-schedule timing is still modelled (see the existing PHASE_1_FINDINGS honesty
note). The survivor rule used is `MemberCountThenGenesis` (a deterministic total order); the thesis
allows the rule to vary — only determinism is required.

**Note surfaced by the run:** the genesis rules are immutable (no op kind can change a
threshold/role), so the plan's "node-3 changes a role/threshold" is not expressible — node-3 instead
applied another membership op. That immutability is itself invariant I1, not a gap.

## Reproduce

```sh
# on each box, from the synced workspace:
./target/debug/reconcile-harness apply add frank carol   > /tmp/n_one.json     # node-1
./target/debug/reconcile-harness apply remove erin alice,bob > /tmp/n_two.json # node-2
./target/debug/reconcile-harness apply add grace dave    > /tmp/n_three.json   # node-3
# exchange all three logs to every box, then on EACH box:
./target/debug/reconcile-harness reconcile /tmp/n_one.json /tmp/n_two.json /tmp/n_three.json | sha256sum
# all boxes must print the same sha.
```

## A3 — two-mode superpeer test (capability vs right): superpeer is a CAPABILITY

node-3 designated always-on broker; file movement relayed via the Mac (boxes lack inter-box keys).
Evidence: `part-a-evidence/a3-*.json`, `a3-results-summary.txt`.

- **A3.1 durable queue** — node-1's commit (`add frank`) held by the broker while node-2 is offline,
  then synced. Broker preserved exact bytes (origin = broker-held = delivered sha `7a945964…`).
  node-2's end-state is **identical** whether the commit arrived via broker (Mode 1) or directly
  (Mode 2): head `241d2718…`, members incl. erin+frank → broker = availability, a capability.
- **A3.2 tamper-evidence** — broker altered one signature char in the held log; node-2 **rejected**
  it on replay (`signature … failed verification`) → broker cannot alter held state undetectably.
- **A3.3 capability-vs-right crux** — a contradiction routed through the broker vs reconciled by a
  peer produced the **identical verdict** (`sha256 = 5f79e0736fb2a0e53c5c52a65a9882c93740df02838dc18912c381bc171c7db7`,
  both `HardStop` `RemovedThenIncluded(erin)`) → broker has no resolving power a peer lacks.

**Verdict:** no outcome reachable in Mode 1 is unreachable in Mode 2. The superpeer is a
**capability, not a right** — it adds availability/convenience only. No leak.

## A2 — conformance (settled by A3)

Every broker shortcut has a no-broker equivalent reaching the same end: durable-queue delivery ≡
direct delivery (A3.1); complementary concurrent commits are commutative (A1b order-independence); a
contradiction hard-stops identically with or without the broker (A3.3). No reachable-only-with-broker
outcome found.

## A1 criterion 4 — the re-formation backstop (clean exit / anti-capture), demonstrated cross-machine

`reconcile-harness reform <ejected> <removers> <followers>` builds the lineage DAG and re-forms a
group off the shared ancestor. Scenario: erin (ejected by alice+bob) re-forms with carol+dave. All
three boxes produced the **identical** reformed genesis `338d8cc8fd7f8390dba6d941d89eb11e52510e31c9743f7a9baec7ef5a9f53e4` and the same verdict:
- reformed members `{carol, dave, erin}`; removers `{alice, bob}` **excluded from membership**;
- `shares_lineage_with_original: true` — the re-formation **provably descends** from the original
  (no false ancestry, legible exit);
- `ejected_has_standing` on both original and reformed; `removers_retain_lineage_standing: true` —
  history is **not erased** (alice/bob remain in the shared lineage; standing ≠ current membership).

This is the honest anti-capture guarantee: crypto does not prevent a social bad outcome (a majority
can eject a minority), but it guarantees the minority a **clean, legible exit** — they re-form with
whoever follows, provably descended from the shared root, and no history is rewritten.

**Part A is COMPLETE** (A0, A1, A1b, A3, A2, + the re-formation backstop). Remaining hardening
follow-on: re-running the whole exchange over the live iroh transport (vs file-exchange) — the
transport itself is proven separately in Part B (B1/B-gossip).
