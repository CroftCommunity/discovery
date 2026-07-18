# Experiment backlog — defined but not-yet-run

A snapshot review (2026-07-13) of every experiment/spike that is **defined** (has a plan, spec,
or procedure written) but **not yet run or not yet complete** across `alpha/`. The point is
legibility: each item names what it is *for* and what it is *blocked on*, so the remaining work
can be picked up and pushed into the corpus.

Maturity legend:
- **Specified** — has a written method + a pass/falsification condition; runnable as soon as its
  blocker clears.
- **Sketched** — named with intent, no procedure yet; needs a plan pass before it runs.
- **Parked/gated** — deliberately deferred pending a hardware resource, a tool, or a design/legal
  decision; do not start blind.

Sources are the plan/roadmap/spec/next-session/TEST-LOG docs inside each experiment.

---

## 0. Doc hygiene

- ✅ **Done (2026-07-13).** Both manifests reconciled to all 12 experiments: the root
  `README.md` listed only 8 and `alpha/README.md` only 7 (it also omitted `iroh` and still used
  the pre-staging `experiments/` tree root). The four re-plant-line experiments
  (`local_storage_projection`, `mls-replant`, `replant-continuity`, `croft-chat`) plus `iroh` were
  added with summaries; both now cross-link this backlog and `MASTER-INDEX.md`.
- **Still open:** `iroh/` carries the **Alt.Drive → Croft.Drive rename** chore (TEST-LOG B5) and
  unconverted `[HYPOTHESIS]` → `MEASURED` tags in `RELAY-PLACEMENT-LAB-SPEC.md` for runs already
  done. Both are larger sweeps within `iroh/` — sequenced in `MASTER-INDEX.md`, not done here.

---

## 1. Cross-cutting blockers (the keys that unlock the most work)

Most unrun items are gated on one of five things. Clearing a key unblocks a whole column below.

| Key | What it unblocks |
|---|---|
| ✅ **Local iroh gossip testbed (loopback, no relay)** — *done 2026-07-13* | X2 fault injection, M1 fan-out, croft-chat multi-process convergence, croft-group L5. Runs on one host via `croft-chat/localhost.toml` (`relay_mode = "disabled"`); see `croft-chat/RUN.md` + `MASTER-INDEX.md §5` |
| **Reachable secroute boxes + public UDP ingress** (real NAT) | Only X1 (real-NAT traversal) and iroh E0-NAT/E4 — the items that *require* a relay-holepunch path, which can't exist where Internet UDP is blocked |
| **`cargo-mutants` installed** ✅ (installed 2026-07-13) | Battery 8 X3 (the auth/governance trust gate). Remaining X3 work is the automated cross-package sweep, not the tool |
| **macOS + iOS hardware** | iroh Spike 3 (macFUSE / B4), Spike 7 (iOS-iroh-blob feasibility — the iroh→Veilid breakpoint) |
| **`ipvsadm` on the boxes** | iroh E4 (LVS frontend HA) |
| **A design/legal decision** | identity & key-recovery model (E3.3), geer (compellability / legal review) |

---

## 2. Drystone governance fold + MLS re-plant
`local_storage_projection` · `mls-replant` · `replant-continuity` · `croft-chat`
Master ledger: `croft-chat/plans/2026-07-11-1-plan-next-experiments.md` (Batteries 5–8).

Batteries 5–6 (completeness + fold conformance) and the Rung-A MLS mechanics (E12.1, E12.3–E12.7)
are **done and green**. What remains:

| Item | For (claim under test) | Maturity | Blocked on |
|---|---|---|---|
| **E12.2** — atomic-swap *message* continuity | ✅ **DONE (RUN-09 Part 3).** In-flight conversation survives the §7.6.2 re-plant repoint with no loss/dup, `Modeled` at loopback grade. B1 dataplane hash structure built (`replant-continuity/src/dataplane.rs`); `e12_2_message_continuity.rs` (5 tests) proves pre-repoint exactly-once, in-flight causal-order, cross-order digest equality, and dup/drop detection. §7.6.2 message half → `Modeled`; EVIDENCE-MAP row added. | ✅ Complete (loopback) | — (real transport / wire pinning open) |
| **E12.7 message facet** | ✅ **DONE (RUN-09 Part 3).** The message-continuity half of the bridge (membership half already `Verified`) — landed with E12.2 in `replant-continuity`, driven over the real re-plant membership. | ✅ Complete (loopback) | — |
| **M1 fan-out half** | ✅ **DONE (RUN-01 EXP-1, 2026-07-14).** Fan-out curve captured over real iroh-gossip at N=2/4/8/16 (`croft-chat/FANOUT-M1.md`): per-node gossip cost **linear** (`live_sent=2N+1`), aggregate O(N²), head-convergence holds at all N (fingerprints match). **Flag:** connect-time resync is super-linear on the bootstrap hub and full-settle (`pending==0`) doesn't complete past N≈8 in-window — corroborates the open RBSR/steady-state gap. Register: `fanout-single-run` (proxy-measurement, magnitude indicative). | ✅ Complete (curve + shape) | — (loopback testbed) |
| **M2** — return-backfill vs dormancy | Cost of a returning member catching up vs staying dormant, at 1/7/30/90-day gaps | Specified (modeled lower-bound runnable now against redb history) | **Mechanism now built** — sync-on-connect resync (`iroh_bus`, `Event::NeighborUp` → re-broadcast retained log); **steady-state anti-entropy** now demonstrated at loopback (RUN-09 Part 4: `croft-chat` `anti_entropy` range-summary/diff + `steady_state_anti_entropy.rs`; §6.8.1 → `Modeled`). M2 is the *sizing* study that remains (push-resync vs pull-on-connect, cost at 1/7/30/90-day gaps); the range-partitioned production construction is now landed at loopback (RUN-12 Part 3b, `partitioned_anti_entropy.rs`, `Modeled`); real-transport loss (X1) and the `[gates-release]` wire fingerprint stay open |
| **X1** — live cross-host over real NAT | Convert in-process fingerprint-equality into a real-network one | Specified (`RUN.md` cross-host recipe) | secroute boxes + NAT workstation (genuinely needs real NAT) |
| **X2** — fault injection during convergence | Kill/crash/heal mid-converge → same head, no reversion, catch-up | ✅ **DONE — all green (loopback testbed, 2026-07-13)** — `scripts/x2-fault-injection.sh` | crash-consistency + monotonic no-reversion + **catch-up** all PASS (`A head == B head`). Catch-up was first *refuted* (gossip dedups re-broadcasts) then *fixed* with a prototype nonce backfill in `iroh_bus`. See ledger Phase 7 |
| **X3** — `cargo-mutants` re-sweep on `fold_auth`/`governance` | A surviving mutant in the authority/threshold path = a real hole in the trust claim | Substrate sweep DONE; cross-package harness open | ✅ **Substrate sweep run (RUN-01 EXP-3, 2026-07-14):** 120 mutants → 54 caught, **0 survivors in threshold-counting** (`governance.rs` 13/13), 61 survivors **all** in the cross-package-covered authorization-*decision* path (`check_authorization`/`role_ge_*`/`act_subject`/`rule_change_approval_subject`). Demonstrated a survivor (`rule_change_approval_subject→const`) is killed by `croft-chat`'s `approval_for_a_different_change_does_not_count`. No real hole found. See `local_storage_projection/X3-CROSS-PACKAGE-SWEEP.md`. **Remaining:** the *automated* cross-package harness (mutate substrate while running `croft-chat`'s suite so all 61 survivors resolve mechanically) — separate crates/`Cargo.lock`, budgets the slow consumer suite. |
| **Fold open items** | ~~RuleChange thresholds~~ (✅ done); ~~contradicted-group byte-head naming~~ (✅ **done, RUN-01 EXP-4** — `competing_quorums.rs::contradicted_group_byte_head_is_min_hash_order_independent`: the byte-head is exactly `min(H(F),H(G))`, order-independent); **two-competing-quorums** → **decided (RUN-02 F8): §7.6-class genuine contradiction, hard-stop + grounded contradiction statement in governance language, never a content-address tiebreak (§7.3.2 / §7.6.1)**; the experiment F8 said would earn the evidence tag **has now run — RUN-01 EXP-4: ⚠️ FALSIFIED** (impl currently auto-resolves order-dependently; a confirmed impl gap vs. the decided spec — see §2a); per-act approver-role granularity (**undecided design** — see §2a); live "catching up…" TUI indicator (App holds no Replicator; UX, skipped unattended); the competing-RuleChange **impl gap is now closed — RUN-03 Phase B** (`detect_competing_rulechange`; register row Reconciled; see §2a) | Byte-head done; competing-quorum decided + impl gap **closed (RUN-03)**; approver-role open | — |

### 2a. Fold findings from RUN-01 EXP-4

**FINDING — competing RuleChange quorums auto-resolve (§7.6.1). Design decided (RUN-02 F8); this is now
an implementation gap.** RUN-01 EXP-4 is exactly the experiment F8 said would earn the fold's evidence
tag, and it **refuted the current implementation**: two concurrent conflicting RuleChanges on the same
rule, each carrying a valid k-of-n quorum, **silently auto-resolve order-dependently** (last-folded
wins; `fork="clean"`, no hard-stop) — an I5 violation on the shape §7.6 says must escalate. Pinned by
`competing_quorums.rs::two_competing_rulechange_quorums` (refutation) and register row
`competing-quorum-autoresolve`. **The design is not open — RUN-02 F8 decided it** (a §7.6-class genuine
contradiction, hard-stopped, never content-address-tiebroken; §7.3.2 / §7.6.1). What remains is
**implementation**: extend the fold's contradiction predicate set (mutual-expulsion, removed-then-included,
role-thrash — `2026-07-12-1-design-concurrent-contradiction.md`) to cover RuleChange. The remaining choice
is *which predicate shape* (an implementation call, not a design one): (A) **same-rule-different-value** —
two concurrent RuleChanges whose `rule_key` matches and `new_value` differs → `Contradiction`, byte-head
`min(H(F),H(G))`, retain the pre-change value (no verdict); narrowest, mirrors mutual-expulsion, and matches
F8's "hard-stop, never tiebreak". (B) a broader **same-subject** predicate keyed on the
`rule_change_approval_subject` content hash. *(An earlier "causal-order-only / reject-as-incomplete"
option is now ruled out — F8 requires hard-stop-and-adjudicate, not silent rejection.)* Recommend (A):
narrowest escalation surface, and it is the direct realization of F8.

> **RESOLVED — impl gap closed (RUN-03 Phase B).** Option (A) landed:
> `fold_derived::detect_competing_rulechange` extends the concurrent-contradiction predicate family
> (mutual-expulsion / removed-then-included / role-thrash) with the narrowest F8 form — two concurrent
> **admitted** RuleChanges on the **same `rule_key`** with **differing `new_value`** hard-stop, surfaced
> identically as `contradiction:{min(H(F),H(G))}`, with the rule left at its pre-conflict value (no
> verdict). Same rule_key + same value is concordant; different rule_keys never conflict. The refutation
> pin `two_competing_rulechange_quorums` flipped RED→GREEN and now asserts both fold orders are identical
> (byte-identical contradiction status; `add_member_threshold` unchanged at the pre-conflict `1`); two
> negative cases (`concurrent_same_value_rulechanges_are_concordant`,
> `concurrent_disjoint_rulekey_rulechanges_do_not_conflict`) added and green. Register row
> `competing-quorum-autoresolve` moved Active → Reconciled (§7.3.2 / §7.6.1, F8; RUN-03). **Still open:**
> per-act approver-role granularity (below) stays a design call, untouched by this run. **EXP-H1** (§2b)
> can now include a competing-RuleChange entry in its horizon manifest alongside the mutual-expulsion one.

**FINDING — approver-role granularity is role-agnostic (still undecided design — do not decide
autonomously).** Step 5.6 counts distinct approver personae **by lineage regardless of role** — a
Member's `Approval` currently counts toward a RuleChange or RoleGrant quorum the same as an Admin's.
Whether an act's quorum should require approvers holding a minimum role *for that act* is an **undecided
design question** (the spec's R-series is mechanism-neutral; nothing decides approver role-gating; RUN-02
R7 explicitly lists it as an open residual). Not tested/implemented here — deciding it is a trust-model
call. *Options for the human:* (A) **role-agnostic** (status quo — any member's approval counts;
simplest, but a low-privilege member can help meet a high-privilege quorum). (B) **per-act role floor** —
each act type carries a minimum approver role (e.g. RuleChange/RoleGrant need Admin+ approvers), enforced
in `gather_approvers`/Step 5.6 by filtering approvers below the floor before counting. (C)
**weight-by-role** — richer, likely over-engineered for now. No recommendation without the trust-model
owner; flagged so the next session decides deliberately rather than by omission.

### 2b. EXP-H1 — horizon-manifest determinism ✅ DONE (RUN-07)

**✅ DONE (RUN-07, landing run RUN-07).** Landed as pure fold-side functions in
`local_storage_projection::horizon` (experiment-grade: no wire format, no persistence, no networking;
the `[gates-release]` manifest encoding stays `Design`) plus `read_group_state`, exercised cross-package
by `croft-chat/tests/horizon_manifest.rs` (4 tests, green). `horizon_manifest(state) -> (frontier_head,
sorted open-contradiction byte-heads)` is **byte-identical across members and arrival orders** for both
ways a contradiction now arises — **mutual expulsion** and **competing quorum-met RuleChange** — compared
via a test-only serialization explicitly not the `[gates-release]` encoding. The `frontier_head` is an
order-independent digest of the folded state's converging content (members + rules + gov_seq), because the
raw `computed_at_gov_head` is the last-INGESTED hash and is arrival-order dependent. The `HorizonCadence`
trigger fires on an epoch roll and on N facts since the last boundary (counter reset each boundary), and
both members locate the boundary at the same fact position. Negatives pinned: a resolved/absent
contradiction is absent from the manifest, and an open one persists across horizon boundaries (decay is
presentation, not truth). Earns the **manifest-determinism** claim only; §7.6.9 stays `Design`.

### 2b′. EXP-H2 — the horizon checkpoint as a foldable fact ✅ DONE (RUN-12 Part 6)

**✅ DONE (RUN-12 Part 6).** EXP-H1 proved the manifest computes identically; the checkpoint was still
only a pure function. EXP-H2 lands the **fact form** as pure fold-side shapes in
`local_storage_projection::horizon_checkpoint` (`CheckpointFact { signer, manifest }`,
`corroboration_count`), exercised cross-package by `croft-chat/croft-chat/tests/horizon_checkpoint.rs`
(3 tests, green; RED→GREEN evidenced by a probe that drops the manifest-match guard and makes a
non-matching member falsely corroborate). A member records a horizon-checkpoint fact carrying its OWN
folded `(frontier digest, manifest)`; a second member that independently folded the identical set
records a co-signing fact naming the same digests. Assertions: (1) the corroboration count for a
`(frontier, manifest)` pair folds deterministically and identically across members and arrival orders
(and a signer's clients collapse to one lineage, §7.3.4); (2) a member whose fold does NOT match records
nothing toward another's manifest — no false corroboration; (3) an open contradiction persists in the
manifest across successive checkpoints (the H1 decay-is-presentation assertion, at the fact layer).
Scope wall held: experiment-grade fact shapes, test-only serialization, no wire pinning (the manifest
encoding stays `[gates-release]`); §7.3.3 semantics unchanged (a co-signature corroborates an independent
identical fold, never a substitute). **Grade judgment (A.9):** the §7.6.9 worked example **stays
`Design`** — EXP-H2 earns the fact-form corroboration-determinism claim (a `Modeled`-grade sub-result,
carried in its own EVIDENCE-MAP row), but the §7.6.9 composition's `[gates-release]` manifest encoding is
unpinned and the full cadence composition is not proven end-to-end, so no tag moved. EVIDENCE-MAP row
added.

**EXP-H1 — horizon-manifest determinism (runnable today).** The objective, no-policy half of the
reconciliation-horizon design (`alpha/thinking/reconciliation-horizon.md`; spec landing: Part 2 §7.6.9
horizon-cadence worked example). Two members, one contradiction; drive a **horizon boundary in both
trigger modes** (an epoch roll, and N-facts accumulating with no epoch roll), and assert **byte-identity
of each member's horizon manifest `(frontier head, sorted set of open contradiction byte-heads)` across
members and across arrival orders**. The manifest carries no resolution policy and no rendered view, so
it is the half that can be pinned as a determinism test before any Layer-2 projection machinery exists.
**Runnable now against the mutual-expulsion contradiction** (its byte-head naming is already
order-independent, RUN-01 EXP-4); **extends to competing-RuleChange after Phase B** lands the predicate,
at which point the manifest simply grows one contradiction entry rather than changing shape. Cross-refs:
`reconciliation-horizon.md` §7 (first spike), Part 2 §7.6.9 (the cadence and the manifest), Appendix B
(`[gates-release]` horizon-checkpoint manifest encoding).

### 2c. EXP-C1 — the completeness-ahead contract ✅ DONE (RUN-07)

**✅ DONE (RUN-07, landing run RUN-07).** All four assertions landed and green at loopback / fold grade
(`croft-chat/tests/completeness_ahead.rs`, 4 tests + `local_storage_projection::completeness_ahead` pure
helpers, 3 inline unit tests). None required MLS internals or network transport, so nothing was split
out. (1) **Stall-at-threshold**: a node denied a governance fact stalls the dependent irreversible act
below freshness `k = ceil(n/2)` (fail-closed, no breach) while still serving reads on its best-known
prefix state. (2) **Stamp detection**: `detect_stamp_gap(local, entry_stamp)` detects and sizes a
data-plane entry stamped ahead of the governance frontier, and returns `None` once the node fills the gap
before acting. (3) **Solicitation reach**: an unreferenced-tail fact absent on X is surfaced by a frontier
ask (re-delivered into X's live pipeline) and folds to the **byte-identical fingerprint** of a node that
received it live. (4) **Formula-valued k**: `quorum_k(n) = ceil(n/2)` over the folded member count is
identical at the same act position across arrival orders (member count converges by construction). The
freshness / generation-stamp values are integers the test seeds, standing in for the attested values; the
`[gates-release]` stamp and `(G, D)` cursor encodings are untouched. Spec touch: §8.2(e) records the
origination precondition exercised at loopback grade (real-NAT path remains X1). Experiment-grade,
loopback only.

**EXP-C1 — the completeness-ahead contract (loopback, runnable now, no new infra).** The demonstration
side of the corroboration-dials framing (`alpha/thinking/corroboration-and-quantified-trust.md`; spec
landing: Part 2 §7.3.3 corroboration-dials paragraph and §7.4.1 formula-valued freshness threshold).
Earning the beam means demonstrating the *contract* completeness-ahead already carries, not eliminating
the intrinsic isolated-node limit. Four assertions, each RED-able before the behavior exists:

1. **Stall-at-threshold (delay over breach).** Withhold one governance fact from node X; X's attempted
   enforcement of a dependent irreversible act stalls below freshness threshold k while X continues
   reads on best-known state. No breach, no stall of reads.
2. **Stamp detection.** X receives a data-plane entry whose §7.4.3 generation stamp is ahead of X's
   frontier; the gap is detected, sized, and named, and X fills it before acting (the behind-via-traffic
   case, demonstrated end to end).
3. **Solicitation reach.** The withheld fact is stamped by nothing (the unreferenced tail); X's frontier
   ask to any peer holding it surfaces it; the fold then admits it identically to normal arrival.
4. **Formula-valued k.** With k = ceil(n/2) over the folded member set, every node computes the identical
   k at the same act position across arrival orders.

Shares boundary machinery with EXP-H1 (both drive the §7.3 read/enforce line and the §7.4 freshness
cursor against a withheld frontier). Discharges, at loopback grade, part of §8.2(e)'s residual that "the
freshness precondition on originating such an op (§7.4–§7.4.2) is not yet exercised over live transport" (the
precondition is exercised over loopback here; the relay/real-NAT path stays X1). Cross-refs:
`corroboration-and-quantified-trust.md` §6 (the contract), Part 2 §7.3.3 (the dials and the fail-closed
gate), §7.4 (the k-distinct-lineages threshold), §7.4.3 (the generation stamp).

### 2d. Vouch payload-validation is an uncovered residual (from RUN-07 X3 automated sweep) — RETIRED (RUN-09)

**RETIRED (RUN-09, 2026-07-15).** The retirement condition is met: `croft-chat/croft-chat/tests/vouch_payload.rs`
(9 tests) drives the fold's I5 Vouch gate end-to-end through `surface::LocalStore`'s
`DerivedFold` path and reads the accept/reject decision back through folded state
(`get_trust_signals`). The RUN-09 cross-package re-run of the sweep (addendum in
`X3-AUTOMATED-SWEEP.md`) shows **19/19 Vouch-region mutants killed, 0 survived** — the 10 RUN-07
justified survivors plus the 9 additional operator mutants. Coverage residual closed; no status tag
moved (this was never a claim). Original finding preserved below for provenance.

**FINDING — the fold's I5 Vouch payload gate is uncovered by both suites.** The RUN-07 automated
cross-package sweep (`local_storage_projection/X3-AUTOMATED-SWEEP.md`) recorded **10 justified
survivors** in the `fold_derived::check_authorization` Vouch arm — the I5 Vouch payload-length /
non-empty-context / strength-byte checks (`fold_derived.rs:447–472`, mutants at `449` ×2, `461` ×4,
`462`, `469` ×2, `470`). These are **genuinely uncovered**: no `croft-chat` consumer test authors a
`Vouch` act, and the substrate suite does not exercise the payload boundaries either, so the mutants
survive both suites. This is unrelated to the R7 threshold/count trust claim (§7.2) — it is a
distinct, honestly-recorded coverage hole in the Vouch act's payload validation, not a weakened
threshold. **Serves:** the fold's I5 Vouch gate (`fold_derived.rs` `AssertionType::Vouch`), the
social-recovery / re-attestation vouch primitive (Part 2 §7.6.4 lineage-divergence vouch, Case-3
recovery §7.3.9). **Register / spec tag:** none — recorded as a sweep residual against
`X3-AUTOMATED-SWEEP.md` (not a divergence stand-in, so no `SPEC-DIVERGENCE-REGISTER.md` row and no
status tag moves). **Retirement condition:** consumer-path `Vouch` tests that kill the 10 survivors
(a Vouch-payload proptest driven through `surface::LocalStore`), **or** an explicit experiment-grade
justification recorded against the sweep for each survivor. Discovered RUN-07; filed RUN-08.

### 2e. Group-principal identity seam (§5.2 / §5.10) — narrowed by RUN-10 Part 2

| Seam | Status | Reference | Next-experiment shape |
|---|---|---|---|
| **§5.2 / §5.10 group-principal identity construction** — whether a Meadowcap communal namespace can carry the Group principal, and how it rotates under churn | **Narrowed (RUN-10 Part 2).** Construction recommended: **primary** communal namespace (the Group-principal *is* a communal namespace at all times); namespace = genesis hash `H(tag ‖ group_id)`; each persona = a self-authorizing subspace; read confidentiality = the fold-gated asset key (already resolved in `asset-keying.md`); authority = a folded, revocable Group Role. §5.10's "how does the communal-namespace key rotate under churn" question **dissolves** — a communal namespace has no shared secret to rotate (write authority is per-subspace, read is the asset key). The Meadowcap composition check (asset-keying.md's "decisive check" Open item) is answered **affirmatively**. All Drystone bindings `Design`. | `beta/impl/drystone-design/group-principal-seam.md` (RUN-10 Part 2) | **Next experiment:** exercise the client→subspace lineage fold end-to-end on the real openmls-Welcome-over-iroh harness — derive a per-persona `SubspaceId` by folding a persona's multiple MLS leaves to one lineage identity; author a Meadowcap-shaped write authorized per-subspace; a governance-fold removal voids the capability by **re-issue, not overwrite** and re-wraps the asset key. Closes the identifier-mapping `[gates-release]` (E.1) against real crypto; moves the subspace-mapping half `Design → green-real`. **✅ EXECUTED (RUN-11 Part 3):** built as the standalone `Design`-grade crate `alpha/experiments/group-principal-seam` (blake3, test-only serialization; RED→GREEN evidenced). Five assertions green (`tests/seam.rs`): (1) the Group principal *is* the genesis hash `H(tag ‖ group_id)`, stable across churn; (2) a persona = a self-authorizing subspace; (3) capability issuance is downstream of the folded Group Role; (4) **re-issue, never revoke-in-place** across a fold-driven authority change — the revoked persona's stale capability fails, the surviving member's pre-change capability is superseded (not overwritten), and the re-issued one succeeds; (5) deterministic on both members, order-independent. **FINDING (scope wall):** the row's aspiration to move the subspace-mapping half `Design → green-real` *against real crypto* and to close the `SubspaceId` byte encoding (E.1) exceeds Part 3's `Design`-grade / no-wire-pinning wall — FINDING-stopped in the Part 3 pass. **Partly resolved (RUN-11 follow-on):** the **subspace-derivation half is now `green-real`** — `tests/subspace_fold_green_real.rs` folds a persona's several **real openmls 0.8.1** device leaves to one subspace identity via the `Verified` `lineage-mls::Device::fold_by_lineage` primitive (RUN-08 `fold_matches`), deterministically. What remains genuinely open (parked, not a defect): the `SubspaceId` **byte encoding** stays `[gates-release]` (Appendix B / E.1), and the revocation-authority **trust tier** stays **I9**. |

**Owner-decides (I9 / not this run):** whether to freeze the primary-namespace stance now vs carry to Appendix B; whether a Group **MAY** permit owned sub-namespaces for single-author content inside its communal Group-principal.

### 2f. Message continuity over real transport (§7.6.2 message half) — ✅ DONE (RUN-12 Part 2)

| Seam | Status | Reference | Shape as built |
|---|---|---|---|
| **§7.6.2 message-continuity half over real transport** — the RUN-09 records were harness-delivered; re-drive the same E12.2 assertions with the B1 continuity records carried over **real iroh-gossip at loopback**, the harness injecting only the duplicate/drop faults | **✅ DONE (RUN-12 Part 2).** Built as `croft-chat/croft-chat/tests/iroh_message_continuity.rs` (2 tests, green; RED→GREEN evidenced by a no-bootstrap probe — with no swarm the records never traverse the transport and the convergence assertion fails). Two `IrohGossipBus` nodes at `LocalDirect` loopback; node A publishes the B1 `Record`s (test-only serialization riding inside the gossip `Frame` payloads — no B1 wire pinning) and node B drains-and-folds them into a `History`; the four continuity claims re-asserted over real gossip delivery, the harness injecting only the dup/drop fault. `replant-continuity`'s pure dataplane is reused behind the `iroh-it` feature so the default build stays openmls-free. **Grade decision:** §7.6.2 message half **stays `Modeled`** (A.9 when-in-doubt) — the records now ride real transport (rationale drops "delivered by the harness"), but the record serialization is test-only so the `[gates-release]` B1 record encoding is unpinned, and delivery is loopback (real-NAT = X1). | `replant-continuity/src/dataplane.rs` (the B1 `Record`/`History` reused); `croft-chat/src/iroh_bus.rs` (`IrohGossipBus`, `RelayChoice::LocalDirect`); `croft-chat/croft-chat/tests/iroh_message_continuity.rs` (the build) | **Landed.** (a) every pre-repoint entry present after, exactly once; (b) in-flight entries land once in causal order post-repoint; (c) both members converge byte-identically across arrival orders; (d) an injected duplicate or dropped frame is *detected*, not absorbed. §7.6.2 EVIDENCE-MAP row + body updated; `part-2-changelog.md` entry. Open: the `[gates-release]` B1 record encoding; real-NAT = X1. |

---

## 3. croft-group (shared-core / per-shell)
`croft-group/plans/2026-06-22-1-plan-croft-chat-cli-group-core.md` — happy-path slice CLOSED;
L1–L6 sequenced, not built. Each gets its own plan.

| Item | For | Maturity |
|---|---|---|
| **L1 — Real identity** | hardcoded handle → real DID/lineage identity in `ChatMessage` + versioned wire (may fold into L2) | Sketched |
| **L2 — MLS / encryption** | `Frame` payload becomes MLS-ciphertext; key/epoch state enters the core; `Zeroize` applies | Sketched |
| **L2a — MLS-sealed happy-path frame** (mechanism half of L2; shaped RUN-10 Part 4) | Seal a `Frame` payload as real MLS ciphertext reusing the proven crates (`Proofs/lineage-groups/crates/lineage-mls`, `mls-welcome-over-iroh`, `mls-replant`/`replant-continuity`, the `bip39-recovery-roundtrip` Zeroize pattern); firewalled from the parked resolution-ACL (croft-group L3) and the I9 call. **RED-able assertions:** (1) seal ≠ plaintext and round-trips through real ciphertext; (2) a no-key peer ⇒ observable `FrameDropped`, model byte-identical, no panic; (3) a real Welcome yields `epoch_secret_match: true` then decrypt; (4) a governed removal re-keys the departed reader out (PCS) via the existing `Verified` k-of-n path, no new authority knob; (5) secrets are Zeroize/no-`Debug` in a sibling crate, none leak into `Effect`/`WireError`, `group-core` stays crypto-free; (6) a firewall-guard test asserting no who-may-revoke / co-sign-ordering / recovery-tier selector exists. Ref: `CROFT-GROUP-L2-READINESS.md` (RUN-10 Part 4). **✅ DONE (RUN-11 Part 2):** built as the standalone crate `croft-group/crates/group-seal` (own lockfile, empty `[workspace]`), reusing `lineage-mls` (the openmls `Device`) and depending ON `group-core` so the pure core stays openmls-free; all six assertions green (`group-seal/tests/l2a_sealed_frame.rs`, RED→GREEN evidenced by an identity-seal stub for assertion 1), clippy-pedantic + fmt clean. `Verified` at loopback grade (real openmls 0.8.1 seal/Welcome/PCS re-key; in-process harness delivery, no wire pinning; real-NAT = X1). The authority/projection halves (R8-tier, R9, R10) stay firewalled. | ✅ **Done (RUN-11 Part 2)** |
| **L2b — (no discrete mechanism-half slice inside the wall) — FINDING (RUN-12 Part 4→5)** | RUN-12 Part 5 attempted the next croft-group slice "as shaped in `CROFT-GROUP-L2-READINESS.md` beyond L2a." **FINDING: the brief defines no discrete L2b.** L2a is explicitly "the whole of L2's *mechanism* half" (readiness brief §4, line 152); the entire remainder — R8-tier (recovery trust predicate), R9 (resolution-ACL / read-scope under fork = croft-group **L3**, F-L2-NAME), R10 (authorized revocation over the wire) — is the **authority/projection half**, firewalled behind I9 and each carrying an explicit **OWNER-DECISION** gate. Part 5's scope wall (mechanism-half-only; FINDING-stop anything touching the authority half or the resolution-ACL) therefore leaves nothing autonomously buildable. **Smallest shapeable slice proposed:** **R10 — authorized revocation over the wire**, the authority half of the exact re-key L2a already built the mechanism for (`group-seal::Sealer::remove_member` is the mechanical re-key; R10 adds *who may revoke, the k-of-n dial, co-sign-vs-vote ordering*). It is the smallest next slice but sits **outside** the mechanism-half wall — it requires first clearing the I9 / OWNER-DECISION gate (the revocation-authority model, `alpha/thinking/revocation-authority.md`), which is a human call, not an autonomous build. **Part 5 dropped cleanly** (no code); this row records the FINDING and the proposed slice for a future run that clears the gate. Ref: `CROFT-GROUP-L2-READINESS.md` §3b/§4, F-L2-NAME. | **FINDING — dropped (RUN-12 Part 5); R10 proposed, I9/OWNER-DECISION-gated** |
| **L3 — Fork/merge + reconvergence-per-plane** | multi-head DAG + per-plane reconvergence policy bound into the asset hash | Sketched |
| **L4 — Governance / delegate planes** | threshold group-principal, capability-vs-authority delegates, the rights-floor | Sketched |
| **L5 — Real-iroh Transport adapter** | second `Transport` impl over iroh-gossip; real async runtime; same scenario tests. **Goes live P2P here.** | Sketched (blocked on iroh testbed) |
| **L6 — Shared-shell composition** | shared shell crate hosting feed + group ponds (Tauri/web); cross-pond read-only awareness (broker deferred) | Sketched |

> Note the overlap with §2: croft-group L2–L5 re-derive, in the shared-shell architecture, mechanics
> the Drystone line has already proven in `local_storage_projection`/`mls-replant`. Worth deciding
> whether L-series builds on those crates rather than re-implementing.
>
> **Decided (RUN-02, 2026-07-13):** croft-group L2–L5 **reuse** the proven Drystone crates. Reuse is a
> **condition of considered compatibility**: a re-implementation of the same mechanics does not count as
> compatible, so L2–L5 build on `local_storage_projection`/`mls-replant` rather than proving the same
> mechanics twice.

---

## 4. automerge-partial-reconstruction

| Item | For | Maturity | Blocked on |
|---|---|---|---|
| **0.7 confirmation** | ✅ **DONE (RUN-01 EXP-2, 2026-07-14).** Re-ran `src/main.rs` against `automerge = "0.7"` (→ 0.7.4) on Rust 1.94.1: all 4 partial-reconstruction invariants hold on the ship target (only change-hash values differ). Register row `automerge-0.6.1` → Reconciled. See `automerge-partial-reconstruction/REPORT.md`. | ✅ Complete | — (Rust 1.94 present) |

---

## 5. croft-app-phase0
Phase 0 (M1–M6) is **complete**. One design deferral remains, not a runnable experiment:
the moderation/labeling question is deferred with M6's live-adapter reversibility (see `BUILD-SPEC.md`).

---

## 6. iroh substrate
Largest surface. Done items (E1/E3/E5/E6/E7/E10/E11-logic/E12-local, meer P0/P1, conformance-core,
Part A, B-series, T-series) excluded. Sources: `RELAY-PLACEMENT-LAB-SPEC.md`, `TEST-LOG.md`,
`RESUME-NEXT-SESSION.md`, `NEXT-SESSION-2026-06-16.md`, `docs/roadmap.md`, `docs/phase-0-spikes.md`.

### 6a. Relay & placement lab (SPEC §4)
| Item | For | Maturity | Blocked on |
|---|---|---|---|
| **E4** — LVS frontend HA + within-shard persistence | IPVS director gives HA + intra-shard balancing without breaking DNS co-location; source-hash pins a peer across reconnects | Specified | `ipvsadm` + public ingress |
| **E8** — multi-namespace peers vs superpeer bridge | crossover density where a straddling superpeer beats N per-peer relay connections | Specified | meer P2 (bridge mode) |
| **E9** — meer confidentiality tiers (T1/T2/no-mirror) | each tier's convergence cost/capability curve (Tier 0 done) | Specified | meer P3/P6 |
| **E0** — hole-punch FAILS (CGNAT/mobile) | worst-case relay cost when the pair stays relay-bound | Specified | NAT'd Mac + UDP 3478 / TCP 3343 ingress |
| **E0** — reconnect-storm CPU driver + sustained-transfer | find NIC/handshake walls under a reconnect storm + long-lived transfer | Sketched | — |
| **E2** — DNS-origin + pkarr integration | replace `MemoryLookup` with the real DNS/pkarr-published placement record | Sketched | — |

### 6b. Media-round follow-ons (SPEC §4a)
| Item | For | Maturity |
|---|---|---|
| **E11 full form** | real moq-rs Tracks + GStreamer/Opus + WebTransport browser reach (only relay logic is green) | Parked (= meer P5) |
| **E11 — TC-META1/TC-META2** | media metadata-leak bound + CBR-padding measurements (AR-4 for media) | Sketched |
| **E12 transport-carried** | SFrame-over-MLS keying over the live E10 datagram rig, real Opus/RTP + RFC 9605 wire (green only with synthetic frames) | Sketched |
| **E10 — mesh-vs-meer comparison** | remaining E10 follow-on (raw-UDP baseline done) | Sketched |

### 6c. Meer build (production, TDD-gated — `meer-superpeer-design.md` P2–P6)
The *running form* of E8/E9/E11/E12. P0/P1 (Tier-0 blind mirror) done.
| Item | For | Maturity |
|---|---|---|
| **Meer P2 — bridge mode** | straddle 2 namespaces/relays (runs E8) | Specified |
| **Meer P3 — Tier-1 + no-mirror + reliability/overlap curve** | runs the rest of E9 | Specified |
| **Meer P4 — RoQ SFU role** | transport form of E12 | Specified |
| **Meer P5 — MoQ relay role** | transport form of E11 | Specified |
| **Meer P6 — Tier-2 (policy-gated) meer** | key-holding, policy-gated mirror | Specified |

### 6d. Faithful follow-ons + conformance gaps (production, TDD)
| Item | For | Maturity | Blocked on |
|---|---|---|---|
| **MLS key-distribution over the wire** | make the modeled verifying-key registry a real over-iroh distribution (standing FAITHFUL honesty boundary) | **Green-real, reproduced RUN-08** — `iroh/crates/mls-welcome-over-iroh` distributes a REAL openmls Welcome over a real iroh connection; joiner derives the identical MLS exporter secret + identical lineage fold from the wire-delivered Welcome. Reproduced in-environment RUN-08 (`relay-lab-runs/C-mls-welcome-2026-07-15-run08`), made buildable here by the Proofs fold-in (`alpha/Proofs/`). Still not wired into the conformance *emitter* (today in-process) | Residual is emitter-integration, not the mechanism; the real-NAT path stays X1 |
| **Threshold revoke-authority as real k-of-n over the wire** | replace the MD-G5 sha-256 MAC stand-in with a genuine k-of-n authority signature | **DESIGN-GATED — RUN-01 EXP-5 stop (see finding row §6d-i)** | **A design decision** (the revocation-authority model: who-may-revoke, the k-of-n dial, key discovery / trust root) — `iroh/TEST-LOG.md` MD-G5 note: "the next layer up, NOT in this spike"; design lives in `alpha/thinking/revocation-authority.md`. Not improvised. |
| **Conformance vectors cats 7/8/9** (AR / visibility / freshness) | were recorded `not_yet_emitted` at the §10.5-footnote layer; ground truth after the Proofs fold-in | **Emitted — 66/0 re-proven in-environment (RUN-08).** The folded reference conformance-core (`alpha/Proofs/lineage-groups/crates/conformance`, `run-vectors`) re-proves cats 1–9 at 66/0: cat 7 adversarial AR-1…AR-6 (real Rust, 8/0), cat 8 visibility (TS-authoritative, 11/0), cat 9 freshness (TS-authoritative, 3/0), plus the cat-5b revoke-authority *mechanism* (4/0). The "not_yet_emitted" reading is superseded for the vectors themselves | Residual is the over-the-wire *authority distribution* (b) — gated on I9 (firewall). See the Part 2 traceability FINDING (RUN-08) on the stale footnote |
| **Domain-tagged pre-image reconciliation** | decide whether `lineage-core` (plain sha256) + the iroh spike adopt CROFT-PROTOCOL §2 domain-tagged genesis/topic pre-images | Sketched | — |

**§6d-i — FINDING / DESIGN GATE (RUN-01 EXP-5, do not decide autonomously).** EXP-5 asked to replace the
modeled verifying-key registry with real over-iroh key distribution **and** the sha-256 MAC revoke
stand-in with a real k-of-n threshold signature, then emit conformance cats 7/8/9 + the revoke-authority
vector. Assessment: **half 1 is already realized in a spike** (`mls-welcome-over-iroh` — real Welcome
over real iroh, matching exporter secret + fold); **half 2 hits a design gate** and was **stopped, not
improvised** (brief's explicit EXP-5 stop rule). The gate is the **revocation-authority model**, which
is the identity/key-recovery problem (§6g / MASTER-INDEX I9) applied to revoke: *who may revoke, at what
k-of-n, and how the threshold signers' keys are discovered/trusted over the wire.* *Options for the
human:* (A) **quorum-of-Ed25519** — a revoke is authorized by k independent Ed25519 signatures over the
revoke fact, counted as distinct personae by lineage exactly like the fold's k-of-n governance path
(reuses the proven `count_personae_by_lineage`/approval-subject mechanism; no new crypto; the signer set
is the current admin/owner set derived from the fold — no separate trust root). (B) **true threshold
signature** (e.g. FROST) — one aggregate signature, needs a distributed key-gen ceremony and a threshold
group whose membership + key distribution is itself a design (heavier, and the DKG trust root is the
open identity problem). (C) **defer** — keep the MAC stand-in tagged until the identity/key-recovery
decision (I9) lands, then build revoke on top. Recommend **(A)** — it composes on the already-verified
governance k-of-n and needs no new trust root, so it is the one that is *not* blocked on I9; but it is
still a spec/trust decision (does a revoke reuse the governance quorum, or is revoke-authority a
separate role?) and so is left for the human. Once decided, emitting cats 7/8/9 + the revoke-authority
vector is downstream integration, not a new gate.

### 6e. Smaller open-edges (spike-class, RESUME §8 E)
`metadata-leak-under-failed-ops` · E6 steady-state goodput + bandwidth-cap · E7 churn storm ·
`fold_by_lineage` cost under churn · AR-4 relay-side timing+volume packet capture (quantify the
leak the bound only characterized). All **Sketched**.

### 6f. Phase-0 de-risking spikes (`docs/phase-0-spikes.md`)
| Item | For | Maturity | Blocked on |
|---|---|---|---|
| **Spike 1** — iroh-docs 10K-entry manifest sync + load | sync timing/memory + deterministic-nonce key behavior (B2 did 10 entries same-process) | Specified | — |
| **Spike 3** — macFUSE hello-world mount (= B4) | encrypt-on-write FUSE mount, throughput + app-compat | Specified | macOS hardware |
| **Spike 4** — full dumbpipe-shape ticket pairing | ticket encoder/QR + 6-digit code + 4-word BIP39 confirm + MitM negative test (B3 did the bootstrap seam) | Specified | — |
| **Spike 5** — iroh-docs vs version-vector ADR | the go/no-go decision doc | Specified | Spike 1 results |
| **Spike 6** — DESIGN.md update + retro | close-out chore | Specified | — |
| **Spike 7** — iOS-iroh-blob runtime feasibility | on-device build + fg/cellular fetch + wifi↔cellular handoff + APNs background sync + battery — the iroh→Veilid breakpoint | Parked | physical iOS device |

### 6g. Identity / key-recovery (NEXT-SESSION EXPLICIT TODOs — the largest open problem)
| Item | For | Maturity |
|---|---|---|
| **Recovery model — quorum social recovery vs minimal-central-authority VC issuer** | spec the two candidate recovery models together (proof-ledger E3.3) | Parked (kept out of autonomous work by design) |
| **BIP39 paper-recovery round-trip spike** | recoveryKey ↔ 24-word mnemonic (KAT-verified) then secretbox-wrap the masterKey — cheapest first step for the above | **✅ done (RUN-08)** — `alpha/experiments/bip39-recovery-roundtrip` (11 tests, clippy clean): recoveryKey ⇄ 24-word BIP39 English mnemonic round-trips bit-exact; the standard English KATs pass both directions incl. checksum-failure negatives (corrupted word, transposed pair, out-of-wordlist, wrong count); masterKey secretbox-wrapped (dryoc XSalsa20-Poly1305) under the recoveryKey unwraps bit-exact, wrong-key/tamper fails cleanly. This is the Tier-1 **lock** only; the trust tier stays I9 (firewall). Crate choice experiment-grade, not `[gates-release]` |

### 6h. Standalone / after-layer spikes (NEXT-SESSION · roadmap · DESIGN §14)
| Item | For | Maturity | Blocked on |
|---|---|---|---|
| **Track 2** — cross-host iroh-docs 3-replica | 3-machine convergence + reconnect-after-partition + durable-queue + LWW-clobber evidence (B2 same-process only) | Specified | boxes (largely subsumed by E3) |
| **HashSeq single-file striping** | stripe one large blob across providers without OOM (currently OOM-kills) | Sketched | — |
| **Automerge-over-iroh interactive-artifact spike** | CRDT sync for edits/reactions/read-receipts + "declare consistency model per artifact type" | Sketched | — |
| **DESIGN §14 open-question spikes** | idle-node memory footprint; NodeId-change-on-reinstall re-pair flow; SQLCipher-value threat-model walkthrough | Sketched | — |

### 6i. Gated — do not start without the resource/decision
- **geer (gating-peer)** — design only; blocked on legal review (compellability).
- **S3/S4** (design gate G5), **T8** (UX decision), **T10** bsky / **T13** iOS host — named as gated;
  the T/S items point into the sibling `discovery`/`Proofs` repos, not this one.

---

## 6b. Stellin AppView — caller-identity (RUN-14)

The private-AppView pipeline is proven (`appview-validation`: ingest/index/serve/backfill/scale). RUN-14
earned the one missing capability — **the AppView learning who its caller is** — and the two design
claims that depend on it. Remaining rows are the named non-goals and the untouched design decision.

| Item | Status | What it is / blocked on |
|---|---|---|
| **Viewer-aware serving via atproto service auth (EXP-A)** | ✅ **Done (RUN-14)** | `appview-validation` phase 8 `authserve` + `serviceauth`/`viewserve`: real service-auth JWT verify against DID-doc keys (secp256k1/p256), viewer-gated `getProfileView`, verified-read telemetry. Live P-A3 confirmed; P-A1/A2 blocked on creds. Declared stand-ins `run14-A2` (recruiter roster), `run14-A4` (service-DID `aud`, dormant). |
| **Sealed offer-gating — §H hybrid serve half (EXP-B)** | ✅ **Done (RUN-14)** | `appview-validation` phase 9 `sealed`: content-blind store, verified-member offer-gating, compilation-boundary blindness, offering-vs-reading. |
| **The helper seam — content helper indexes by grant (EXP-C)** | ✅ **Done (RUN-14)** | New crate `helper-seam` on real openmls (`group-seal`): admit-by-grant → index → serve; forward-blind on revocation; no authority. |
| **Interactive OAuth/DPoP — the PWA client-login leg** | **Parked (attended-run territory)** | atproto interactive OAuth + DPoP needs a browser hop the unattended env lacks (RUN-14 named non-goal). Distinct from service auth (server-to-server, proven EXP-A). Needs an attended session with a browser; also the live service-auth token leg (EXP-A P-A1/A2) needs `ATP_TEST_HANDLE`/`PASSWORD`. |
| **The AppView-provisioned scope key** | **Parked (design decision — untouched by RUN-14 per stop rule 5b; RUN-15 D11 stages the trusted-gatekeeper arm for the large tier only, `needs-call`)** | social-mapping Open items: what the audience scope key protecting an AppView-served audience *is*, and how it is granted and rotated among authorized readers. Determines whether the AppView path matches MLS-group confidentiality or approximates it. Not improvised in a run; needs the owner's design call. |

---

## 6c. AppView hosting kit (RUN-15)

Phase 1 built and validated entirely in `appview-infra/` (`RUN-15-SUMMARY.md`). Phases 1.5 and 2 are staged and owner-gated.

| Item | Status | What it is / blocked on |
|---|---|---|
| **The hosting kit (D1–D16)** | ✅ **Done (RUN-15)** | Manifests+generator (systemd/Caddy/litestream/rclone), backup-audit invariant, own-data API sidecar (self-scoping/export/timeout/OS containment), roster-gated large-group tier behind a `GroupStore` trait, terraform+bootstrap+deploy, and a local destroy→restore fire drill (local-rehearsal grade). `make check` green; extracted tree passes standalone. |
| **Phase 1.5 — backup plane against real R2** | **Ready — code path PROVEN locally; only endpoint+creds remain (owner supplies an R2 bucket + scoped token; FREE)** | The `s3://` code path (litestream `type: s3` + rclone s3 remote) is proven end-to-end by `make local-drill-s3` against a local **MinIO** standing in for R2 — full destroy→restore→assert green (`SPEC-DELTA[run15-s3-local]`). **Next action:** create the R2 token (R2 → Manage R2 API Tokens → Object Read & Write on `croft-appview-staging`), export endpoint + keys, point the same configs at real R2, restore to temp paths, re-run the drill with real-R2 replicas, and record exact op counts vs the free tier. No OVH account, no purchase. |
| **R2 free-tier fit (write-rate lever)** | **Finding (RUN-15 P15-3-local) — owner call at go-live** | The drill's op report + rate model: litestream at `sync-interval=1s` emits ~1 PUT/s per continuously-writing canonical db ≈ **2.6M PUT/mo**, OVER R2's 1M/mo free Class-A tier. Free-tier fit is governed by **write frequency, not tenant count**; the lever is the sync-interval (raise it for high-write tenants) or a paid tier. Confirm exact counts against real R2 in Phase 1.5. |
| **Phase 2 — provision + go-live** | **Owner-gated purchase, from the extracted repo** | Extract (P2-0) → `catalog-vps.sh` pick (P2-1) → `terraform plan`/`apply` (P2-2, also unblocks `terraform validate`) → `bootstrap --apply` twice (P2-3) → Porkbun records + ACME (P2-4) → backup plane on box (P2-5) → fire drill, owner variant (P2-6) → summary (P2-7). |
| **Group write-path fork + scale boundary + launch order** | **Owner decision (RUN-15 D11 asks; GROUPS.md §5)** | Variant A (repo-canonical) vs B (server-canonical); confirm `group_scale_boundary` (working 5000); croft-groups before/with stellin. D12 built the fork-agnostic serving; the kit does not decide. |
| **`terraform validate` (BLOCKED)** | **BLOCKED (egress) — reported, not skipped** | ovh provider download → github 403 via the egress proxy. Unblocks in Phase 2 where the registry is reachable. `SPEC-DELTA[run15-tf-validate]`. |

---

## 6d. The group tier model (RUN-16)

The canonical group tier model (`alpha/experiments/appview-infra/GROUPS.md` v2, Section A) restates the
group design as **one lineage / one envelope / one delivery plane / one catalogue**, with the tier
expressed as two independent policy axes (membership `open`|`gated`|`sealed`; write
`open`|`members`|`named-set`|`single`). It is a docs-and-registers landing; the rows below carry what
it opens as forward work or records as landed design. Spec-facing implications are staged, not landed
(`proposed-changes` RUN-16 update, `needs-call`); the reviewed spec is untouched.

| Item | Status | What it is / blocked on |
|---|---|---|
| **(a) Sealed-tier ceiling — churn-simulation experiment** | **Runnable candidate (RUN-16 shaped)** | Measure the sealed tier's practical ceiling (GROUPS.md A.9: "low hundreds — churn + no-arbiter adjudication") as an **evidence-graded curve** rather than ratifying a number by assertion. Method: a churn simulation over the **croft-group loopback harness** (no boxes) — concurrent-commit / detected-contradiction rate and human-adjudication load as functions of membership N and churn rate — producing the curve A.10(2) says should be measured before the sealed ceiling / `group_scale_boundary` is ratified. Runnable now on the loopback testbed. Blocked on: nothing external; a shaped experiment for a future run (RUN-17+). |
| **(b) The write-policy axis** | **Landed design (RUN-16, GROUPS.md A.2)** | The second, independent policy axis — who may author into a scope (`open`|`members`|`named-set`|`single`) — orthogonal to the membership axis. The catalogue displays both fields; the same gate enforces both at serve time and relay time (A.8). No code: a design record. Enforcement realization reuses the backplane roster-lookup-at-causal-position check and the R7 admission machinery (§2 R7); nothing new is proven here. |
| **(c) Sealed-scope helper-index rows are observation-born (taxonomy correction)** | **Landed correction (RUN-16, GROUPS.md A.7) — flagged for the kit's backup-audit** | State-taxonomy correction: index rows a **content helper** derives from **sealed** scopes are **observation-born, not projections** — forward secrecy ages the ciphertext out of decryptability, so they cannot be re-derived from any surviving canonical source the way a backplane projection can. They must therefore be classed **canonical** (a small sidecar in `state.db`) or **knowingly accepted as losable** in the scope's posture language — never silently classed `disposable` like the backplane index. Flagged for the AppView hosting kit's **backup-audit invariant** (RUN-15 D5): a sealed-helper index target must be declared canonical-with-backup or explicitly accepted-losable at audit time. Blocked on: the kit's owner picking canonical-sidecar vs accepted-losable per tenant. |
| **(d) Self-host an iroh relay vs public relays (A.10 item 4)** | **Owner call (RUN-16)** | Whether to **self-host an iroh relay** — a service-manifest question for the AppView hosting kit (a relay is another always-on service the kit's manifests+generator could provision alongside the AppView/api units; see `appview-infra/kit` and RUN-15's service-manifest model) — or **use the public n0 relays initially**. Loads only for sealed scopes and steward governance (A.8: the overlay is loaded only by sealed scopes), so relay load scales with sealed-group count, not user count. Owner decision, not made here. |
| **(e) History-convergence role — membership-interval backfill** | **Landed design (RUN-16, GROUPS.md A.7); mechanism pending RUN-17 proof** | The history-convergence node converges envelope sets across transports (set reconciliation over envelope hashes — the RBSR machinery, `Modeled` at loopback from RUN-12 Part 3b) and serves **backfill scoped by membership interval**: a requester who proves membership receives history from any causal point forward at which the proof holds (their intervals per A.3), and nothing earlier; in sealed scopes the node serves ciphertext only and decryption stays bounded by held keys (forward secrecy not overridden by delivery). Interval-forward is the default; widening it is a per-scope governance dial. Landed as design in A.7; the **interval-scoping-of-offering** mechanism is the new unproven piece — a shaped **RUN-17** proof (drive interval-scoped backfill over the loopback convergence harness, asserting a requester receives exactly their intervals and nothing earlier). |
## 6e. The tier proof (RUN-17)

The RUN-16 group model built and run end to end in `tier-proof/` (`RUN-17-SUMMARY.md`). P1–P9 green, red-first, 52 tests; component/loopback grade; the two live atproto legs are BLOCKED on credentials, reported not pretended. This run **proves the pieces §6d shaped**: (a) the sealed-tier churn ceiling (P9b curve), (b) the write-policy axis (P3), and (e) the history-convergence interval-scoped backfill (P8) all land here as executable proofs.

| Item | Status | What it is / blocked on |
|---|---|---|
| **The tier proof (P1–P9)** | ✅ **Done (RUN-17)** | One signed envelope (`H(envelope)` identity, canonical dag-cbor) carrying every record; a fold to catalogue + interval roster; open-tier self-registration + backfill reconstructability; write-policy axis + validate-before-relay; gated two-sided facts (co-sign threshold, causal revocation cut, silence≠verdict, archive rebuild); device-key delegation (event-driven revocation); blinded roster; **real openmls** steward group (sealed reasoning → public verdict, `steward-seal/`); tier transition as re-plant; three co-hosted delivery-role processes + interval backfill; measured scale numbers. `cargo test` + clippy green. |
| **P2/P5 live legs (BLOCKED)** | **BLOCKED (creds) — reported, not skipped** | Genesis/self-registration to a real PDS + Jetstream ingest (P2) and `did:plc` DID-doc resolution (P5) need `ATP_TEST_*`. The `MemSource` / `DidKeyResolver` stand-ins run behind the same interfaces. `SPEC-DELTA[run17-live-source, run17-did-resolver]`. **Next:** set the env vars; the interfaces are unchanged. |
| **P4 multi-party accounts** | **Ready — local keypairs stand in** | The gated-tier two-sided facts run against local ed25519 keypairs; `ATP_TEST_*_2/_3` upgrade steward/member/outsider to distinct live accounts. `SPEC-DELTA[run17-multiparty]`. |
| **P8 real iroh overlay** | **Optional upgrade — local sockets stand in** | The swarm-peer runs over local TCP behind a socket seam; a clean iroh build via the proxy upgrades to a real two-peer exchange. `SPEC-DELTA[run17-swarm-local]`. |
| **P9(b) real churn at scale** | **Optional upgrade — local epoch model** | Drive `group-seal` epoch rolls at increasing N to replace the model curve; epoch-roll cost is already a measured O(N) quantity. `SPEC-DELTA[run17-churn-model]`. |
| **Variant A/B · boundary number · interval-widening** | **Owner decisions (not taken)** | P9 informs the boundary; the gated tier stays fork-agnostic; the interval-widening dial is asserted default. None decided here (brief §4). |

## 6f. Reception completeness + the publications positioning (RUN-18)

A delta run on the RUN-16/17 surfaces (`RUN-18-SUMMARY.md`): the corpus gains the reception-completeness
paragraph (GROUPS.md A.2) and the standalone publications positioning (`appview-infra/PUBLICATIONS.md`);
`tier-proof/` gains the executable proofs B1–B6.

| Item | Status | What it is / blocked on |
|---|---|---|
| **Reception completeness for write-restricted scopes** | ✅ **Done (RUN-18; proven by Part B)** | Per-author envelope chaining (first anchors to genesis); completeness as DETECTION up to the newest held envelope, never delivery; withheld-tail limit per the completeness-ahead doctrine with multimodal delivery as mitigation; open enrollment never weakens verification. Canonical text GROUPS.md A.2; proven executable by B1 (chaining at the relay), B2 (gap detection + repair, no oracle), B3 (the honest tail claim + swarm closure), B4 (chaining × interval composition). |
| **The publications positioning doc** | ✅ **Landed (RUN-18, `appview-infra/PUBLICATIONS.md`)** | The vanilla-atproto comparison: native proofs (authorship, integrity, current-state completeness), the degeneration principle as binding, the single-agent limit + the provable multi-party fact as the one added atom, the delta table (tamper-evident history vs tamper-free current state; the three-way retraction distinction), the subscriber reframe (two rosters one lineage; guarantee beneficiary; structural consent; paid tier as a policy value; the honest scope of "managing"). Joins the corpus link + anchor audit. |
| **The auditable-count claim** | ✅ **Proven (RUN-18 B6, component vs the landed harness)** | On the landed P2 open-tier machinery: an independent second fold re-derives exactly the roster count the DS serves; an authenticated unsubscribe moves both counts identically; a count asserted without folded records behind it is detectable as unsupported. Live upgrade rides the P2 live leg (`ATP_TEST_*`). |
| **B5 live retraction leg** | **BLOCKED (creds) — simulated, tagged** | The middle-issue deletion runs against the landed harness's delete event, `SPEC-DELTA[run18-retraction-local | stand-in]`; with `ATP_TEST_*` set it deletes a real PDS record and upgrades to live grade. |

---

## 7. The classroom tier (docs track — not an experiment)

| Item | Status | What it is / blocked on |
|---|---|---|
| **Classroom chapters (tier 4 of the gradient)** | Scaffold landed **RUN-13**; bodies **drafted in conversation, not by runs** | `alpha/classroom/`: the arc (`00-arc.md`, three acts / ten chapters) + chapter skeletons `01`–`10` with headed beats, PROVE-IT boxes wired to real tests and EVIDENCE-MAP rows, and the two seed Mermaid diagrams (Ch. 1, Ch. 5). Each chapter's prose body is `DRAFT-PENDING` and is written with the owner in conversation; runs only maintain the scaffold, the pointers, and the diagrams' validity under the site gate |

---

## Recommended execution order

**Done since this snapshot (2026-07-13 → RUN-11, 2026-07-16):** the local iroh gossip testbed
(loopback, no relay); **X2** fault injection (crash-consistency + no-reversion + catch-up via
sync-on-connect); `cargo-mutants` installed; **RuleChange thresholds** enforced; the `Session`
governance emit API. **RUN-01** — A4/M1 fan-out (EXP-1), automerge 0.7 (EXP-2), X3 substrate sweep
(EXP-3), contradicted-head byte naming (EXP-4). **RUN-03 Phase B** — the competing-RuleChange
contradiction predicate (the F8 impl gap, closed); **RUN-03/04** — the continuity-decoupling,
reconciliation-horizon, and corroboration-dials design passes. **RUN-05/06** — the full consistency
pass and the settlement of its 11 register findings. **RUN-07** — the X3 automated cross-package
harness (R7 count now cross-package `Verified`), **EXP-H1** horizon-manifest determinism (§2b), and
**EXP-C1** the completeness-ahead contract (§2c). **RUN-08** — MLS-welcome-over-iroh reproduced +
conformance 66/0 re-prove; the **BIP39** paper-recovery round-trip (the Tier-1 *lock*); the spec↔experiment
traceability pass. **RUN-09** — the **Vouch** payload-validation coverage (§2d retired); the **E12.2/E12.7
message facet** (B1 → A5, §7.6.2 message half → `Modeled` at loopback); the **M2 steady-state slice**
(steady-state anti-entropy, §6.8.1 → `Modeled` at loopback); the **fan-out repeated-run** (`fanout-single-run`
retired); the **traceability settlement** (FND-T1/T4/T5/T6/T7). **RUN-10** — the published spec site +
broken-ref gate; the three briefs (group-principal seam §2e shaped, emitter-integration decision,
croft-group L2 readiness with L2a shaped). **RUN-11** — the Part 1 riders settled (FND-T2/T3/T4, FND-R10-1/4/5,
the emitter decision, the Map fixes). Reconciled deltas: `x2-backfill`, `rulechange-quorum`,
`handcrafted-assertions`, `automerge-0.6.1`, `competing-quorum-autoresolve` (RUN-03); `fanout-single-run`
(RUN-09 Part 5 repeated-run). **Active:** `hermetic-gossip` (needs the boxes / X1) — the only remaining
Active row. See `SPEC-DIVERGENCE-REGISTER.md`.

Remaining, in leverage order (current queue):

1. **L2a — the MLS-sealed happy-path frame** (§3 L2a; shaped RUN-10 Part 4, built **RUN-11 Part 2**).
   The mechanism half of croft-group L2, seal/unseal over real MLS at loopback reusing the proven
   crates; firewalled from the parked resolution-ACL (croft-group L3) and I9.
2. **§2e — the group-principal seam spike** (§2e; shaped RUN-10 Part 2, built **RUN-11 Part 3**). Group
   as a Meadowcap communal namespace, personae as subspaces, capability re-issuance across a
   fold-driven authority change, all `Design`-grade Drystone bindings.
3. **Message continuity over real transport** — ✅ **DONE (RUN-12 Part 2, §2f):** the E12.2 assertions
   re-driven with the B1 continuity records carried over real iroh-gossip at loopback, the harness
   injecting only the duplicate/drop faults (`iroh_message_continuity.rs`, 2 tests green, RED→GREEN via
   a no-bootstrap probe). §7.6.2 message half **stays `Modeled`** (records now real-transport-delivered,
   rationale updated; the `[gates-release]` B1 record encoding unpinned and real-NAT = X1 still gate).
4. **Range-partitioned RBSR construction** — ✅ **DONE (RUN-12 Parts 3a/3b), a read-then-build.** Part
   3a's brief (`beta/impl/drystone-design/rbsr-construction.md`) measured **Willow 3d-range versus
   Negentropy** over §6.8.1's linear `(device, lamport)` key space and recommended the Negentropy-style
   one-dimensional recursive reconciler (the governance-fact surface, §6.8.5/§7.2, is a distinct
   Willow-shaped 3d space — a separate choice). Part 3b landed it at loopback (`partitioned_anti_entropy.rs`,
   3 tests: diff-only equivalence, fingerprint composition, O(log)-ish rounds at scale) — a large divergent
   range repaired in O(log) rounds shipping only the divergence, replacing the whole-set compare. `Modeled`
   at loopback. Open: the `[gates-release]` wire fingerprint (Appendix B) and real-transport loss (X1).
5. **croft-group L2b+** — the layers past L2a per `CROFT-GROUP-L2-READINESS.md`: the authority half
   (revocation-over-the-wire, the trust tier) waits on **I9**, and read-scope-under-fork waits on the
   parked resolution-ACL (croft-group L3).
6. **The parked list (verbatim):** **I9** (the identity/key-recovery trust tier, the largest open
   problem; the Tier-1 lock landed RUN-08, the Tier-2 trust predicate is the open call); **X1** (real-NAT,
   needs the boxes); **hot-N 500+** (fan-out magnitude at scale); **`[gates-release]` + BLAKE3** (the
   Appendix B wire/byte pinning); **emitter integration** (now formally deferred by decision — Option C,
   defer to the `[gates-release]` pass, owner 2026-07-15; Option B fallback); and the **resolution-ACL
   (croft-group L3)** design frontier.
