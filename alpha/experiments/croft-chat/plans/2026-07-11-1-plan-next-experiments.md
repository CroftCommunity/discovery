# Drystone: Next Experiments to Further Demonstrate Viability

`Status: plan. Extends the existing Provable-Experiments Plan (Batteries 1–4). Written against
the actual P20 code state in this repo (croft-chat integrated CLI, local_storage_projection
substrate, iroh spikes), so every item names the code it starts from and the exact claim it
tries to break. Same inherited rule: each item is a refutation attempt with a falsification
condition, not a demo. Rung A = measured on real hardware/library; Rung B = modeled against
Drystone's own structures.`

## Where the code already is (so we don't re-run what's proven)

P1–P20 shipped an integrated vertical slice. What is **already corroborated in code**:

- **Order-insensitive convergence (I5)** — proven, but as a *single hand-built two-node
  scenario* (`croft-chat/croft-chat/tests/convergence.rs`) plus a 4-node `serve` run over
  real iroh-gossip (P19, identical `fingerprint`). It is a demonstration, not yet a property.
- **§7.6 hard-stop on contradiction** — proven for exactly *one* shape: two genesis
  assertions for the same group → `ForkStatus::ForkedFrom` → blocking banner (P20,
  `tests/contradiction.rs`). No silent winner. Good.
- **I6 amendment-threshold self-amendment**, compaction/merkle-root, the content-hash
  `tiebreak` (`governance.rs`), lamport restart-resume, cross-process persistence.
- **MLS Welcome over real iroh** — a joiner derives both the MLS group secret *and* the
  lineage-folded standing from a wire-delivered Welcome (`iroh/crates/mls-welcome-over-iroh`).

Three facts in that code decide the priorities below:

1. `ForkStatus` is `{ Clean, ForkedFrom(hash) }` — it can represent **too-many valid claims**
   (contradiction) but has **no representation for too-few** (§7.6.1's under-determination:
   a required role vacant with no admissible successor). Half the escalation set is unbuilt.
2. `convergence.rs` is a fixed scenario; the `tiebreak()` path is not property-exercised, and
   the `Replicator` is a **per-device contiguous-chain applier** — i.e. convergence is proven
   *under an assumption of a complete causal set*, which is precisely the open keystone
   (Battery 4, gap-completeness).
3. The Welcome spike stops at *join*; the **re-plant** (stamp-fresh-group-over-member-set,
   atomic repoint, last-resort seating) — the whole of Battery 1 — is not yet code.

The experiments below are ordered by *how much a failure would cost the design*, not by ease.

---

## Battery 5. The completeness assumption, made testable now (the keystone, brought forward)

`The existing plan (Battery 4) parks gap-completeness as "nothing to test against until
detection exists." That is too pessimistic. The dangerous failure — two nodes agreeing on a
head that omits a fact admitted elsewhere — is observable now, against the P20 fold and real
gossip, without building the detector first. This battery tries to break the convergence claim
by attacking its unstated precondition. Rung A (real iroh), highest value.`

**G1. Gap injection: an incomplete set must not produce a confidently-wrong agreed head.**

- Claim under test: convergence (I5) is claimed only over a *complete* causal set. The
  `Replicator`'s contiguous-chain applier should *stall at the gap*, never fold past it.
- Procedure: two `serve` nodes over iroh-gossip; drop or indefinitely delay one middle
  governance fact to node B (induced loss in the `Transport` adapter). Let both quiesce.
  Compare heads and `fingerprint`.
- Falsifies if: B reaches a *stable, self-consistent* head that A also holds a different
  version of, with neither node signalling incompleteness — i.e. a silent divergence dressed
  as convergence. That would show the fingerprint equality in P18/P19 is an artifact of
  lossless testbed delivery, not a property.
- Passes (weakly) if: B visibly under-authorizes / stalls at the gap and A's head is a strict
  superset — the monotonic-fold posture holding under real loss.
- Output: the first real evidence for or against the keystone. Rung A.

**G2. Late-heal is monotonic, never a reversion (the CVE-2025-49090 guard, live).**

- Claim under test: when the withheld fact finally arrives, B advances from under-authorized
  to authorized and **never reverts** an already-admitted decision (Part 1 §2.2; the Matrix
  state-reset class is the named counterexample).
- Procedure: continue G1 — deliver the withheld fact late; assert B's post-heal state is A's
  head and that no previously-admitted assertion changed its admitted/derived value.
- Falsifies if: any admitted fact flips, or the derived projection resets to an earlier value.
  Rung A against the real fold, not just the `fold_derived` unit tests.

**G3. Duplicate/replay under gossip resolves by content-hash, at scale.**

- Claim under test: the §6 content-hash dedup (already in `SharedDirBus` seen-set + the
  governance `tiebreak`) holds under a real broadcast medium with induced duplicates and
  reorder, not only the shared-dir scramble.
- Procedure: inject duplicates and out-of-order redelivery on the iroh path; assert exactly-once
  application and identical head.
- Falsifies if: a duplicate applies twice, or reorder changes the head. Rung A.

---

## Battery 6. Governance-fold conformance, hardened from scenario to property (sharpens Battery 3)

`Battery 3 asks for V1–V5 as property/vector tests. The code has the *scenarios* (convergence,
contradiction) but not the *properties*. This battery lifts each to the rigor the plan wants,
against the real fold in local_storage_projection. Rung B (Drystone's own fold), runnable now.`

**V1′. Promote convergence to a property test (generated DAGs, randomized order).**

- Start from: `convergence.rs` (one scenario) + `governance.rs::tiebreak`.
- Procedure: `proptest` generating well-formed governance-fact DAGs (adds, removes, role
  changes, rule amendments) with random *admissible* arrival orders into two independent folds;
  assert identical head. Explicitly generate the causal-tie case so the **cryptographic
  tiebreak path is exercised**, not just the causally-determined one.
- Falsifies if: any complete-set input yields divergent heads across orderings, or the tiebreak
  branch is never hit (proving the current scenario under-tests it). Rung B.

**V4′. Build and refute the *second* escalation shape — under-determination.**

- The gap: `ForkStatus` cannot express "required role vacant, no admissible successor." A
  contradiction-only watcher misses it entirely (§7.6.1). This is the single most important
  *missing* mechanism the code review surfaced.
- Procedure: (a) extend the substrate to represent under-determination as a distinct hard-stop
  state (not `ForkedFrom`); (b) construct a sole-admin-pruned group whose only admin is removed
  with no successor meeting `role_change_threshold`; assert the fold **hard-stops and surfaces
  the legible picture**, does not silently continue on a headless group.
- Falsifies if: the pruned group folds onward as if healthy, or the vacancy is representable
  only as a fork. Rung B. *(This one requires a small substrate change before it can be run —
  flagged in the ledger as "needs build.")*

**V3′. Regress-free adversarial vectors against `check_authorization`.**

- Procedure: inject a fact whose authorizing grant is (a) absent, (b) present but itself never
  admitted, (c) admitted only *after* the fact in causal order. Assert rejection in all three
  from the unconflictable base.
- Falsifies if: any fact is admitted on the strength of a grant not itself admitted-earlier.
  Rung B against the real `fold_auth`.

**V5′. Threshold counts personae by lineage, never clients — in the fold path.**

- Procedure: a multi-signer threshold satisfied only by multiple *clients of one principal*;
  assert rejection in the governance-fold path (extends the 66/0 suite's persona-level check
  down into `required_threshold_for_rule_change` evaluation).
- Falsifies if: co-signatures from one principal's devices ever satisfy a k-of-n. Rung B.

---

## Battery 7. Re-plant, built on the Welcome spike and refuted (executes Battery 1)

`Battery 1 (E12.1–E12.7) is fully specified but unbuilt. The mls-welcome-over-iroh spike is the
foundation: it already stamps a real openmls group and folds standing from group state. The
next code step is stamp-over-member-set + atomic repoint. Rung A for MLS mechanics; E12.7 Rung
B. Run E12.3 first — it is the correctness keystone of the fork story.`

- **E12.3 first (byte-nondeterminism is a dedup, not a fork).** Two planters stamp
  concurrently from an identical member set read from the governance chain; confirm the
  divergent tree bytes resolve by the *same* content-hash `tiebreak` the substrate already uses
  for governance, and that nothing downstream reads tree shape. **Cross-check worth having:**
  the MLS-layer dedup and the governance-layer `tiebreak` should agree by construction — a
  disagreement is a real fork and falsifies the claim. Rung A / Rung B split as in E12.3.
- **E12.1** baseline stamp cost O(N) at N = 100…2000 → feeds Battery 2's M1.
- **E12.2 / E12.4 / E12.5 / E12.6** per the existing spec, once the repoint exists.
- Toolchain decision the plan flags: the spike uses **openmls** (`lineage-mls`), while Battery 1
  specs **mls-rs 0.55.2**. Recommendation: keep openmls for the whole Rung A battery to reuse the
  spike and its aarch64 build, and note the switch — do **not** run both and compare absolute
  costs (the plan's own caveat).

---

## Battery 8. Operational viability: cross-host, faults, and the mutation gate (closes deferred P18/P20)

`Two items are explicitly deferred in the P20 close-out; both are "demonstrate viability" in the
operational sense and are runnable as soon as the boxes are up. Rung A.`

- **X1. Live cross-host over real NAT.** Run the `RUN.md` recipe on the secroute boxes
  (`secroute-testing-one` bootstrap + creator, NAT workstation joins via n0 relay). Convert the
  in-process gossip fingerprint-equality into a real-network one. Falsifies if NAT/relay traversal
  changes the converged head or fails to quiesce within the window.
- **X2. Fault injection during convergence.** Kill a node mid-converge; partition then heal;
  assert convergence resumes to the same head (monotonic catch-up, no reversion). Pairs with G2.
- **X3. cargo-mutants re-sweep on `fold_auth` / `governance`.** The user's standing trust gate,
  deferred at P20. A surviving mutant in the authority-check or threshold path is a real hole in
  the conformance claim. Falsifies the "mutation-vetted substrate" status if mutants survive.

---

## Battery 2 linkage (sizing) — what unblocks when

M1 (per-commit + fan-out) needs Battery 7's re-plant for a real per-*boundary* number, but the
per-*commit* half can run against the openmls spike now. M2 (return-backfill vs dormancy) can be
approximated now against the redb history layer over synthetic gap sizes (1/7/30/90 days),
before the full history-convergence node exists — a modeled lower bound that later goes Rung A.

---

## Recommended sequence and status ledger

| # | Experiment | Runnable now? | Rung | Moves |
|---|---|---|---|---|
| 1 | **G1/G2/G3** completeness under lossy gossip | **Yes** (P20 code + loss adapter) | A | Attacks the keystone convergence precondition — highest value |
| 2 | **V1′** convergence → property test | **Yes** | B | I5 scenario → property; exercises `tiebreak` |
| 3 | **V3′/V5′** regress-free + lineage vectors | **Yes** | B | Extends 66/0 suite into the fold path |
| 4 | **V4′** under-determination hard-stop | Needs small substrate build | B | Builds the *missing* second escalation shape |
| 5 | **X1/X2/X3** cross-host, faults, mutants | Yes (boxes/CI) | A | Closes deferred P20 operational items |
| 6 | **E12.3 → E12.1/2/4/5/6** re-plant | Needs build on Welcome spike | A/B | Executes Battery 1; feeds M1 |
| 7 | **M1/M2** sizing | Partial now, full after #6 | A | Sizes §5.9 exit-affordability |

`Through-line: #1–#3 run today against code that exists and attack the two claims the code
currently only demonstrates (complete-set convergence, single-shape hard-stop). #4 fills the one
concrete mechanism gap the code review found. #5 converts an in-process proof into a networked,
fault-tested, mutation-vetted one. #6–#7 are the build-then-measure frontier the existing plan
already scoped. Nothing here claims "proven"; each item names what a failure would refute.`

---

## Execution log (2026-07-11 → 07-12)

All items below are implemented and green, driven through the **real** `Replicator` +
`Session` + fold (not a bypassed fold). Tests live in `croft-chat/croft-chat/tests/`
with shared scaffolding in `tests/common/mod.rs` that hand-crafts signed assertions the
`Session` API cannot emit (RuleChange, cross-device chains) and makes a dropped or
duplicated frame a first-class knob. Each test's header is its result record.

The log has two phases: first the refutation battery that found the completeness gap,
then the fix and its hardening.

### Phase 1 — the battery (find and bound the gap)

| Item | Test | Verdict | What it established |
|---|---|---|---|
| **G1** | `completeness.rs` | **Refutation** | The completeness precondition of I5 was *unenforced*. Neither the Replicator (per-device contiguity) nor the fold (never read `.antecedents`) covered a **cross-device** antecedent gap: node B admitted a fact whose antecedent was dropped and diverged silently on governance state (threshold 7 vs 1), both `clean`. Localized the Battery 4 keystone to one absent guard. |
| **V3′** | `regress_free.rs` | Corroboration | An unadmitted grant authorizes nothing: a plain Member's add is rejected even against the complete set, and a dropped authority-conferring fact makes the dependent add fail (strict subset) rather than diverge. The fold was complete-safe for **authority**, gap-blind only for **state** — the contrast that made G1 precise. |
| **G2** | `heal.rs` | Corroboration | Heal is monotonic — under-authorize, never revert (the structural defense against the Matrix state-reset class, CVE-2025-49090). |
| **G3** | `dedup.rs` | Corroboration | Duplication + reorder is idempotent: a node fed every frame 3× and scrambled reaches the identical fingerprint to clean single delivery. |
| **V1′** | `convergence_property.rs` | Corroboration | Convergence lifted from one scenario to a property: 64 seeded-random orders of a concurrent 2-device DAG all converge to one head (§7.3.1 canonical order). |
| **V4′** | `under_determination.rs` | **Build + corroboration** | The *second* §7.6.1 escalation shape. Added `ForkStatus::UnderDetermined` + `governance::is_under_determined`; removing the sole Owner now hard-stops with a distinct signal + app banner instead of silently running a headless group. |

### Phase 2 — the fix (close the gap) and its hardening

| Item | Test | Verdict | What it establishes |
|---|---|---|---|
| **G1 fix** | (`fold_derived::ingest` Step 5.5) + `fold_derived::tests::missing_antecedent…` | **Fix** | A **governance** fact is held back (`FoldError::MissingAntecedents`) until every declared antecedent is present; the Replicator's retry heals it. Govern-scoped by the razor (§2.0.1): data-plane messages keep optimistic acceptance. G1 and G2 flip to *verify* the closed behaviour — B now holds the fact back and its head is a strict **prefix** of a complete peer's, not a divergent head. |
| **guard-property** | `guard_property.rs` | Corroboration | Prefix-closure as a property: across 56 cases (random cross-device drops + random orders of a causal chain), a node's admitted membership is always a contiguous prefix — it never folds a fact whose antecedent chain reaches a dropped one, and complete deliveries admit all. |
| **pending-signal** | `pending_signal.rs` + `Replicator::pending_len`/`is_settled` + `serve` output | Corroboration | Incompleteness is observable: a held-back node reports `pending_len > 0` (not settled), so it can show it is catching up rather than present a stale prefix as current; it settles once the antecedent arrives. |

### Phase 3 — concurrent contradiction (§7.6.1, the §2.5 residue): find and close

Under-determination (V4′) covered the "too few" escalation shape. Phase 3 attacks the
"too many valid claims" shape for *concurrent non-genesis* governance — which P20's
genesis-only collision detector missed. Design + primitive in
`2026-07-12-1-design-concurrent-contradiction.md`.

| Item | Test | Verdict | What it establishes |
|---|---|---|---|
| **mutual expulsion** | `mutual_expulsion.rs` | **Refutation → Fix** | A⊗B (A expels B while B expels A) silently auto-resolved to an *order-dependent* survivor ({O,A} vs {O,B}, both clean) — a silent I5 violation on the canonical §2.5 residue. The fold now detects it and hard-stops: both orders → `{O,A,B}` + identical `Contradiction`, no verdict. |
| **removed-then-included** | `removed_then_included.rs` | **Refutation → Fix** | A concurrent add/remove race on one subject resolved last-writer-wins, order-dependently. Now hard-stops inclusively (X retained) with identical `Contradiction` in both orders. |
| **role thrash** | `role_thrash.rs` | **Refutation → Fix** | A concurrent grant/revoke race on one subject resolved last-writer-wins (Member vs Admin), order-dependently. Now hard-stops with the subject reverted to its base role, identical `Contradiction` in both orders. |
| **benign concurrent removes** | `benign_concurrent_removes.rs` | Corroboration (boundary) | Two admins removing *different* members converge and stay clean — the false-trip guard. Must stay green through any reconcile change. |
| **concurrency primitive** | `governance::are_concurrent` unit | Corroboration | Causal-concurrency over the antecedent DAG; concurrency is *necessary but not sufficient* for a contradiction. |

Mechanism of the fix: `detect_mutual_expulsion` (unauthorized path) and
`detect_removed_then_included` (authorized path) → `resolve_contradiction`, which replays
the log in canonical `merge_cmp` order excluding the conflicting remove(s), retaining the
contested parties and flagging `ForkStatus::Contradiction` with the canonical pair hash.
**Soundness guard** (the full suite caught a regression without it): concurrency must be
positively established, so antecedent-free (bare/legacy) facts are treated as sequential,
never a contradiction — the conservative side of the false-trip line.

**Net.** The governance fold is now characterized and hardened end-to-end under both lossy
delivery *and* concurrent conflict: authority gaps reject, state gaps hold and heal,
duplicates/reorders are no-ops, convergence is order-independent over a complete set, a
lagging node can tell it is behind, and **all** §7.6.1 residue shapes hard-stop —
under-determination plus all three concurrent-contradiction shapes (mutual expulsion,
removed-then-included, role thrash) — without false-tripping on benign concurrency.
Substrate lib suite 96/0; croft-chat suite green.

### Phase 4 — V5′: k-of-n threshold enforcement (approval facts)

Thresholds were decorative (a single Owner satisfied any k-of-n — the Stage-3 Owner-proxy).
Now enforced. Design in `2026-07-12-2-design-threshold-enforcement.md` (Option A chosen).

| Item | Test | Verdict | What it establishes |
|---|---|---|---|
| **threshold enforcement** | `threshold_enforced.rs` | **Refutation → Fix** | A single Owner no longer meets a 2-of-n add (X absent); a second admin's `Approval` meets it (X present). A threshold-k act references k `Approval` facts as antecedents; the antecedent guard holds it until they arrive; Step 5.6 counts distinct approver personae and rejects below quorum (`ThresholdNotMet`). |
| **lineage / no client-stuffing** | `governance::thresholds_count_personae_by_lineage…` (unit) + self-approval case | Corroboration | Counting is by principal, so a persona's clients — or a persona approving its own act — never inflate the quorum (§5.7). |

`ForkStatus`/contradiction and thresholds compose: existing threshold-1 groups are
unaffected (Step 5.6 skips required ≤ 1). Four completeness tests that used
`add_member_threshold=7` as an arbitrary divergence value were migrated to
`remove_member_threshold` now that adds are gated. Substrate lib 97/0; croft-chat green.

**Net so far (governance fold, Phases 1–4).** The fold is characterized and hardened
end-to-end: completeness under lossy delivery (held/heal/dedup/observable), order-
independent convergence, all §7.6.1 residue shapes hard-stopped without false-tripping,
and k-of-n thresholds enforced over distinct personae by lineage.

**Open on the fold.** Byte-level head of a *contradicted* group still names the triggering
fact (pre-existing; not a membership divergence). RuleChange thresholds (no principal
subject); per-act approver-role granularity; the concurrency interaction (two competing
quorums → §7.6.1 contradiction). Cross-host + `cargo-mutants` re-sweep (Battery 8 — tool
not installed here); a live "catching up…" TUI indicator (the App does not hold a
Replicator; only `serve` does).

### Phase 5 — Battery 7: the MLS re-plant against a real library (Rung A)

The Welcome spike stopped at *join*; the re-plant — stamp-fresh-group-over-member-set (§7.6.2)
— was specified but unbuilt, and it is the mechanical keystone of the fork/heal story. Built as
a standalone crate `alpha/mls-replant/` on **openmls 0.8.1** (the spike's pin). `Proofs/lineage-mls`
is absent in this workspace, so the E12 mechanics are exercised against openmls directly. The
member set is modelled as a list of personae (each with its own crypto provider — separate
devices). Governance-chain continuity (E12.7) is Rung B and stays with the fold.

| Item | Test | Verdict | What it establishes |
|---|---|---|---|
| **E12.3** (keystone) | `e12_3_dedup_not_fork` | **Corroboration** | Two personae stamping independently over the same member set produce *different* tree bytes but *identical* membership — a dedup the content-hash tiebreak resolves, never a fork. The load-bearing correctness claim of the whole re-plant story. |
| **E12.1** | `e12_1_baseline_cost` | Corroboration | Baseline stamp cost is O(N): per-member Welcome bytes stay ~flat (≈152–155 B/mbr) across N = 25…500. Feeds M1. |
| **E12.4** | `e12_4_drift_reset` | Corroboration (bounded) | A fresh stamp resets the re-key drift a tree accumulates through interior removals. Byte-size proxy; direction corroborates, magnitude understates (openmls serializes blanks compactly). |
| **E12.5** | `e12_5_leaf_rotation` | Corroboration | A fresh stamp rotates every member's leaf encryption key at once while preserving each persona's signature identity — a group-wide re-key. |
| **E12.6** | `e12_6_last_resort` | Corroboration | Availability is bounded by the last-resort package: the swap never blocks. A member seated via a *reused* last-resort package does not rotate (the E12.5 exception); a fresh-package member does. |

**M1 — the per-commit cost band (Battery 2, Rung A).** The optimistic reading assumed commits
are O(log N). Measured, it is a **band**, not a point, and the pessimistic end is real:

- **Floor** (`m1_per_commit_cost`, *refutation*): a **lone** committer's self-update on a
  bulk-add-stamped tree is **O(N)** (~80–130 B/mbr, flat across N and across repeated commits) —
  the co-path resolves over the blank sibling subtrees the stamp leaves. A re-key/membership
  cost, not per-message (messages are O(1)).
- **Ceiling** (`m1_populated_tree`, *honest close*): once every member has committed once (a full
  round-robin populates the interior), a commit is **O(log N)** — per-member bytes *fall* as N
  grows (90→52→30 B/mbr at N=8/16/32 vs sparse 131→109→97; ~3× cheaper per member at N=32).

Which regime a hot group sits in is decided by how many *distinct* members commit. A lone active
re-keyer pays the O(N) floor — raising the §5.9 exit-affordability floor and shortening the
§11.11 liveness window more than the optimistic reading assumed. Tune the window for the floor.
mls-replant suite 7/0, clippy clean.

### Phase 6 — E12.7: Rung-B governance continuity (the bridge)

The re-plant's keystone: the crypto membership must be a *function of* the governance chain,
never an independent authority. Built as a new standalone crate `alpha/replant-continuity/` —
the **only** place the substrate and openmls meet (the fold stays openmls-free; `mls-replant`
stays fold-free; the bridge depends on both by path). The fold is driven through its **real
`DerivedFold::ingest` path** (signatures, credentials, authorization, thresholds all enforced)
over an in-memory store; the derived member set is read back from the persisted `GroupState`
and compared — as an independent computation — against the principals recovered from a fresh
MLS stamp's actual crypto membership.

| Item | Test | Verdict | What it establishes |
|---|---|---|---|
| **keystone** | `e12_7_1_stamp_tracks_derivation` | **Corroboration** | Across genesis + four authorized adds, the stamp seats *exactly* the fold's derived set at every step — no stray seat (over-broad group), no missing seat (a governed member locked out of the keys). |
| **removal** | `e12_7_2_removal_propagates` | Corroboration | An authorized removal drops the member from the derived set *and* the fresh stamp — "removal is real", the re-plant re-keys the departed member out, not governance theatre. |
| **unauthorized** | `e12_7_3_unauthorized_no_drift` | Corroboration (adversarial) | An add authored by a non-member is *rejected at ingest*; the derived set does not move and the stamp seats no unauthorized principal. The fold — not MLS — is the sole membership authority. |

replant-continuity suite 3/0, clippy clean (no substrate change — the mutation-vetted crate is
untouched; the bridge implements its own signer/verifier/cred-resolver).

**Still open in Battery 7.** E12.2 and E12.7's *message*-continuity facet (the §7.6.2 atomic
repoint of an in-flight conversation) are Rung B over Drystone's dataplane hash structures, not
these crates. M1's fan-out half and M2 (return-backfill vs dormancy) need the iroh gossip
testbed. Done: five of seven E12 Rung-A MLS mechanics, the full M1 per-commit cost band, and
E12.7's **membership**-continuity bridge (fold ⇔ stamp, end-to-end against real openmls).

### Phase 7 — Battery 8 / X1-local + X2: a relay-less loopback iroh testbed, and fault injection

Battery 8's cross-host/fault items were parked on "needs the iroh gossip testbed." The dependency
was softened: added a **direct-only iroh mode** (`RelayChoice::LocalDirect` — `presets::Minimal` +
`RelayMode::Disabled`, wait for a local direct addr instead of a relay home) selected by a topology
`relay_mode = "disabled"`. Two `serve` processes now converge over **real iroh-gossip on 127.0.0.1
with no relay and no Internet** (`croft-chat/localhost.toml`; `RUN.md` "Same-host recipe";
fingerprint `503af2f0895c9b2d`). The two convergence tests moved to `LocalDirect` and are now
hermetic. This localizes X1 to *real-NAT-only* (which genuinely needs the boxes) and brings X2, M1's
fan-out half, and croft-group L5 in-reach locally.

**X2 — fault injection during convergence** (`scripts/x2-fault-injection.sh`). Two real `serve`
processes; A creates + enrolls B + sends `a1`; B joins + sends `b1`; both converge to `{a1,b1}`. B
is **SIGKILLed** mid-run; `a2` is admitted by A while B is down; B is **restarted on the same
store** and rejoins.

| Sub-claim | Verdict | What it established |
|---|---|---|
| crash-consistency | **PASS** | B's redb store survives the SIGKILL with `{a1,b1}` committed and readable. |
| no-reversion (monotonic) | **PASS** | After rejoin, B still admits `a1,b1` — an already-admitted fact is never lost/reverted (the operational form of G2 / the CVE-2025-49090 guard). |
| catch-up-after-absence | **GAP → FIX** | *Refuted first:* B did **not** fold `a2` on rejoin; it stalled at its pre-crash prefix while A held `{a1,b1,a2}` (heads differed). **Cause:** `Replicator::publish_group` re-broadcasts identical frames, but **iroh-gossip ids messages by `blake3(content)`** and caches received ids (90 s), so a node that returns *after* a frame's first broadcast never receives it. Measured contrast: a node present for the first broadcast received all 4 frames and converged; one rejoining ~5 s later received **1 of 815** re-broadcasts and stalled. *Then fixed* (see below): B rejoins and folds `a2`, `A head == B head == 21abb458e940a723`. |

**The fix — sync-on-connect (spec mechanism).** First landed as a prototype (a per-tick nonce
re-flood — registered spec-delta `x2-backfill`), then **reconciled to the spec mechanism** and the
delta retired. `IrohGossipBus` now broadcasts each distinct frame **once** in steady state
(`TAG_LIVE`), and on `Event::NeighborUp` re-broadcasts the retained log **once** as `TAG_RESYNC`
with fresh ids — so a joining/returning node catches up at connect time, at O(log) cost per join
rather than a per-tick re-flood of the whole log. X2 stays all-green across repeated runs on this
mechanism (`A head == B head`). **Not** closed by this: M2's *sizing* study (push-resync vs
pull-on-connect; cost at 1/7/30/90-day gaps) and steady-state anti-entropy (a *live* frame dropped
to an already-connected neighbor, with no new join to trigger a resync). See
`alpha/experiments/SPEC-DIVERGENCE-REGISTER.md` → Reconciled.

**Net.** All three X2 sub-claims now hold on the loopback testbed: crash-consistency, monotonic
no-reversion, and catch-up-after-absence (a returning node re-converges to the exact same head).
The refutation localized the missing mechanism to **sync-on-connect backfill** (the operational
core of **M2**), and the nonce prototype closes it; M2 remains as the *sizing/production* study
(NeighborUp-gated vs pull, cost at 1/7/30/90-day gaps). Reproduce: `scripts/x2-fault-injection.sh`.

### Phase 8 — RuleChange k-of-n quorum enforced (reconcile the `rulechange-quorum` spec-delta)

V5′ enforced membership/role thresholds but **deferred RuleChange** — a RuleChange has no principal
subject, so Step 5.6's `act_subject` returned `None` and the fold fell back to an Owner-role proxy.
The substrate test verified only that a threshold is *stored* at position, explicitly punting on
enforcement ("the check that it FAILS requires a full quorum evaluator"). That was registered as
the `rulechange-quorum` **weakened-assertion** spec-delta.

Closed by giving a RuleChange a **content-hash approval subject**: `rule_change_approval_subject(payload)
= blake3(payload)`. Approvers now name `(RuleChange, H(payload))` — pre-computable from the proposed
change — and Step 5.6 counts distinct approver personae by lineage, the identical path as membership.
The `(type, subject)` pair keeps it distinct from a principal subject. Epoch/replay and
two-competing-quorum concurrency stay at **parity with membership** (the concurrency interaction is a
separate, still-open §7.6.1 item).

| Item | Test | Verdict | What it establishes |
|---|---|---|---|
| single signer | `rulechange_threshold_enforced::single_signer_fails_rulechange_two_of_n` | **Refutation → Fix** | At rule_change_threshold=2, a lone Owner's amendment is **rejected**; the target rule is unchanged. (Was applied before the fix — RED proven by disabling the arm.) |
| two personae | `…::two_distinct_personae_meet_rulechange_two_of_n` | Corroboration | Owner + a second admin's `Approval` naming the change's hash → admitted (2 distinct). |
| self-approval | `…::self_approval_does_not_reach_rulechange_quorum` | Corroboration | A persona approving its own RuleChange is still one persona → rejected. |
| mismatched approval | `…::approval_for_a_different_change_does_not_count` | Corroboration (binding) | An approval for change →5 does **not** satisfy a different change →9 — pins the subject to be injective across payloads. |
| substrate reject | `governance::test_i6_amendment_threshold_self_amend` (strengthened) | **Fix** | The substrate's own I6 test now *ingests* the second single-signer amendment and asserts `Err` + rule unchanged, instead of punting. |

**Mutation evidence (targeted, manual).** `act_subject`'s RuleChange arm → `None` is killed by the
two reject cases; `rule_change_approval_subject` → a constant is killed by the mismatched-approval
case (the first three cases can't — approver and act use the helper symmetrically). Both temporary
mutants reverted clean. A formal `cargo-mutants` sweep is now mechanically unblocked (tool
installed) but needs a cross-package harness (V5′ positive coverage is in `croft-chat`) — that is
**X3**. Substrate `test_i6` green; croft-chat suite green incl. the 4 new cases; clippy clean.

### Phase 9 — `Session` governance emit API (reconcile the `handcrafted-assertions` spec-delta)

The threshold work exposed an API-surface gap: `Session` could emit membership + messages but
**not** RuleChange or Approval, so governance-fact tests hand-built signed assertions. Closed by
adding the emit primitives:

- Substrate surface (`LocalStore`): `rule_change(group, rule_key, new_value, approvals, signer)` and
  `approve(group, act_type, subject, signer)` — same build→sign→ingest→notify shape as `add_member`,
  each returning the fact's hash so a proposer can reference an approval.
- `Session`: `propose_rule_change` + `approve_rule_change` (the latter derives the change's
  content-hash subject so the approval names exactly what the fold enforces).

Proven end-to-end by `rulechange_quorum_via_api.rs`: two live `Session`s replicate while O proposes,
A2 approves the specific change, O references the approval, and the quorum of two distinct personae
admits the amendment (add_member_threshold 1→5) which then converges on A2 — a lone amendment is
refused. No hand-crafted envelopes. croft-chat suite 22/0; clippy clean.

**Scope note.** This closes the *capability* gap. The `tests/common` scaffolding stays — its
remaining use (adversarial delivery: dropped/duplicated/reordered frames; withheld antecedents) is
legitimate refutation testing a well-behaved API must refuse to produce, not an API gap. Still not
emittable via `Session` (a lesser, separate gap): `MembershipRemove` and `RoleGrant`/`RoleRevoke`.
See `alpha/experiments/SPEC-DIVERGENCE-REGISTER.md` → Reconciled.

---

## Net across the whole program

Two load-bearing subsystems moved from *Design* toward *Verified*, each under a Popperian
refutation discipline (every test names what a failure would refute, and several **found**
real defects before closing them):

1. **The governance fold** (`local_storage_projection`, Phases 1–4) — complete under lossy
   delivery, order-independent in convergence, hard-stopping on **every** §7.6.1 residue shape
   (under-determination + all three concurrent-contradiction shapes) without false-tripping on
   benign concurrency, and enforcing k-of-n thresholds by lineage. Substrate lib 97/0.
2. **The MLS re-plant** (`mls-replant` + `replant-continuity`, Phases 5–6) — the fork/heal/
   re-key story against real openmls 0.8.1: byte-nondeterminism is a dedup not a fork (E12.3),
   the O(N) stamp and the full sparse→populated per-commit cost band (M1), leaf rotation and
   last-resort availability (E12.5/6), and — the keystone — the crypto membership is a
   *function of* the governance chain end-to-end (E12.7: adds track, removals re-key out,
   unauthorized changes never seat). mls-replant 7/0, replant-continuity 3/0.

**Refutations that changed the design, not just confirmed it:** the cross-device completeness
gap (G1), the three order-dependent silent divergences on the §2.5 residue (mutual expulsion,
removed-then-included, role thrash), decorative thresholds (V5′), and the "commits are
log-cheap" assumption (M1 floor is O(N)). Each is now a guard with a test that would catch its
return.

**What remains genuinely blocked** (needs infrastructure, not more of this discipline): the
Rung-B *dataplane* continuity (E12.2 + E12.7's message facet) over Drystone's hash structures,
and the iroh gossip testbed for M1's fan-out half and M2. Battery 8's `cargo-mutants` re-sweep
awaits the tool. These are the honest edges of what this run could refute.
