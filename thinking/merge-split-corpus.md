# Merge / Split Case Corpus — the three trees and their reconciliation

date: 2026-06-16

status: design + case-enumeration doc, feeding a proofs corpus. Doubles as the technical
explanation layer: a reader should be able to learn *how Croft handles every way a group can
split and rejoin* from this one document, and a builder should be able to read off the proof
targets (proven / gap / spec) from the same tables.

source machinery: `Proofs/lineage-groups/crates/lineage-core/{dag,conflict,survivor}.rs`,
`crates/lineage-mls/lib.rs`. Status tags mirror `crystallized/proof-ledger.md`.

---

## 1. Why "the group hash tree" is actually three trees

Croft binds three tree-shaped structures. Most confusion about "merging groups" comes from
conflating them; the design's safety comes from keeping them distinct and binding them by one
rule.

```
┌─ MLS ratchet tree (cryptographic) ───────────────────────────────┐
│  openmls 0.8.1. A SINGLE LINEAR EPOCH CHAIN. By MLS rule it       │
│  CANNOT merge: two epochs are never reconciled into one.          │
│  Split = fresh genesis (new tree). Heal = external-commit re-key  │
│  of the loser into the survivor's epoch.                          │
└───────────────────────────────────────────────────────────────────┘
            ▲ bound by: "a governance op is enacted only once it is an MLS commit"
┌─ Governance DAG (lineage forest) ─────────────────────────────────┐
│  dag.rs. Nodes = branches; edges = parents. 0 parents = root,     │
│  1 = fork, 2 = recombine/fresh-genesis. This tree DOES fork and   │
│  recombine; it is the navigable lineage. Standing & shares_lineage│
│  are decided here, from signed data alone.                        │
└───────────────────────────────────────────────────────────────────┘
┌─ History CRDT (per-branch) ───────────────────────────────────────┐
│  Per-branch Automerge. NEVER merges into one transcript (the      │
│  anti-"six tapes" rule, E2.8). Branches stay distinct & navigable;│
│  reconciliation is consensual backfill, not interleave.           │
└───────────────────────────────────────────────────────────────────┘
```

The binding rule is the whole design: **a governance decision (DAG) is only real once it is
an MLS commit (ratchet tree); history (CRDT) records but never adjudicates.** A split or merge
event therefore has a coordinated effect across all three rows — the corpus below always reads
the event in all three columns.

## 2. Split cases (how a single group becomes two)

| ID | Trigger | MLS ratchet tree | Governance DAG | History CRDT | Handling mechanism | Status |
|---|---|---|---|---|---|---|
| **S1** | Network partition — both sides keep operating | two epoch chains diverge | one branch → fork (1 parent each side) | each side appends locally | survivor selection on reconnect | green-real (E2.3) |
| **S2** | Re-formation / trapdoor — ejected member re-forms minus removers | fresh genesis epoch | fork; removers excluded from membership but retain lineage standing (history not erased) | new branch; prior readable | A1 re-formation; `shares_lineage_with_original` true | green-real-multimachine |
| **S3** | Multi-device divergence — a user's own devices edit offline | each device is its own member/leaf | one lineage, per-device branches | each device its own Automerge doc | self-sync = backfill (§4) | spec → T2 / T-gossip |
| **S4** | Deliberate sub-group spawn — a group births a child topic | new MLS group | fork | new branch | designed, not built | spec |

## 3. Merge / heal cases (how two branches rejoin, or deliberately don't)

| ID | Trigger | MLS ratchet tree | Governance DAG | History CRDT | Handling | Status |
|---|---|---|---|---|---|---|
| **M1** | Reconnect, no contradiction | external-commit re-key of loser into survivor epoch | both ops union; identical DAG | branches stay distinct + navigable | `detect → Heal`; `select_survivor` (deterministic, symmetric) | green-real (E2.3/I10) |
| **M2** | Contradictory membership | NO merge; epochs stay separate | two branches persist, attributed | both readable | `detect → HardStop`, escalate to human | green-real (E2.4/I6) |
| **M3** | Rejected merge (resting state) | two valid MLS groups coexist | two branches | both | E2.5 — conflict is a feature, not an error | green-real |
| **M4** | Human forces a clean merge (mint-a-third) | fresh MLS genesis, re-add all | recombine (2 parents); both prior logs read-only ancestry | both inherited read-only; nothing reordered | E2.6 / E1.3 | green-real |
| **M5** | Quorum overrides a hard-stop | apply governed op, then heal | explicit signed Readmit or ConfirmRemoval | per decision | `quorum_override` — threshold-meeting, all-or-nothing, never auto-resolves (A2.5) | green-real |
| **M6** | Self-sync heal (own devices) | same lineage; trivially shares genesis | fold leaves per lineage | backfill import | E2.12 = the existing backfill path, no special case | spec → T-gossip |

## 4. Conflict-reason taxonomy — the array to widen

`conflict.rs::detect` currently classifies exactly one reason (`RemovedThenIncluded`). That is
the canonical hard case, but a corpus meant as a technical explanation layer must enumerate the
whole space — and doing so surfaces real proof gaps (C4, C7, C8, C10 unmodeled; C9 partial).

| ID | Conflict | Expected handling | Status |
|---|---|---|---|
| **C1** | Removed-then-included (boot on one side, keep on other) | hard-stop, escalate | green-real (E2.4) |
| **C2** | Included-then-removed (the symmetric direction) | hard-stop (bidirectional check covers it) | green-real |
| **C3** | Concurrent identical remove (both sides boot the same DID) | **heal** — agreement, not conflict; assert no false hard-stop | likely covered — **verify** |
| **C4** | Add-vs-add of the *same person* on *different device keys* across the partition | should fold by lineage, not double-count; assert one actor, not two | **GAP — unmodeled** (directly multi-device) |
| **C5** | Threshold-rule divergence (two sides claim different genesis thresholds) | structurally impossible — genesis is immutable (I1); assert unrepresentable | assert-absent (I1 green-real) |
| **C6** | Stale-admin authority (a departed/removed genesis admin still governs) | rejected — authority is per-epoch | green-real (A2.4 closed) |
| **C7** | Concurrent dissolve-vs-continue (one side dissolved the group, other kept operating) | hard-stop or resting-state? — **undefined** | **GAP — unmodeled** |
| **C8** | Diamond recombine (A↔B then B↔C then re-merge) — conflict semantics over a multi-parent DAG | ancestor walk handles topology; conflict detection over a diamond untested | **GAP — topology proven, conflict untested** |
| **C9** | Governance equivocation / fork attack (same author signs two contradictory ops) | detect + attribute the fork | **PARTIAL** — A2.2 found "fork-detecting/attributable" needs hardening |
| **C10** | Re-add after a cross-partition removal (ban-evasion via a new device leaf) | the new leaf must not silently re-confer standing | **GAP — unmodeled** (achilles FM8: moderation) |
| **C11** | leave-all-under-lineage vs group-remove-one-leaf race | both ops expressible and distinct; deterministic outcome | spec (E2.13/E2.14) |

## 5. Survivor-selection determinism (the heal pivot)

When M1 heals, both disconnected sides must independently compute the *same* survivor with no
negotiation round, or the heal forks again. `survivor.rs` does this: `select_survivor` is a
total, symmetric order (`select(a,b) == select(b,a)`), default = more-members-then-smaller-
genesis-hash. Proven order-independent under 300×4 fuzzed delivery orders (A2.1, green) and
byte-identical across 3 machines (cross-machine A1/A1b, `5d82a5df…`). The rule is a *parameter*;
the *requirement* is determinism — any variant a proof introduces must remain a deterministic
total order.

## 6. Proof targets this corpus generates

New rows to add to the ledger and TEST-PLAN, in leverage order:

1. **C4 — multi-device add-vs-add fold** (gap). Highest priority: it is the multi-device case
   and it interacts with lineage-counted thresholds (E2.10). Belongs with T2.
2. **C7 — dissolve-vs-continue** (gap). Define the intended resolution (likely resting-state +
   escalate), then prove it.
3. **C8 — diamond-recombine conflict** (gap). Extend `detect` to multi-parent ancestry.
4. **C9 — equivocation hardening** (partial). Close the A2.2 gap: make a forked author's two
   ops attributable, not just detectable.
5. **C10 — ban-evasion re-add** (gap). A new device leaf under a previously-removed lineage
   must not silently re-confer standing (moderation surface; pairs with achilles T4 FM8).
6. **C3 verify** — assert concurrent-identical-remove heals (no false hard-stop). Cheap.

These slot into TEST-PLAN as a new **Tier 1b — reconcile-case corpus**, executed alongside the
multi-device tier (they share the `lineage-core` reconcile machinery).
