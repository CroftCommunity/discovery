# Proposed Part 2 changes — experiment reconciliation (2026-07, for review)

`Status: PROPOSED, not applied. This is a reviewable diff, not a spec edit. Part 2 is reviewed
material; nothing here touches part-2-certifiable-design.md until you approve it line by line. On
approval, each accepted item lands as a part-2-changelog.md entry (house style: "§X, done. <what>
(primer §Y).") and the corresponding edit is made to Part 2.`

`Source of every claim below: the code spikes now in alpha/experiments/, cross-referenced in
alpha/experiments/SPEC-ALIGNMENT-AND-ACTION-PLAN.md §3–4 and SPEC2-OVERLAY.md. Each item cites the
experiment that earns it.`

## How to read this

Each proposed change carries:
- **Target** — the Part 2 section and the current line.
- **Class** — `status-move` (a maturity tag changes, no mechanism change), `new-mechanism` (spec text
  gains a mechanism the experiments built), or `caveat` (an honesty boundary is tightened/annotated).
- **Diff** — current text (`-`) → proposed text (`+`), quoted verbatim from Part 2.
- **Why + evidence** — the experiment result that earns it.
- **Decision** — `ready` (mechanical, low-risk) or `needs-call` (a judgment is yours).

The changes are independent; approve any subset.

---

## F1 — RuleChange/policy-change quorum is *enforced*, not merely stored  ·  `new-mechanism`  ·  called

**Target:** §7.2 (the grant-and-revocation interface, R-series) and §8.2(e). This is the one item
that adds a mechanism Part 2 does not currently carry — a grep of the spec for
`rule_change_approval` / `rulechange` returns nothing.

**Why + evidence.** During experimentation the RuleChange quorum was found **not enforced** — an
Owner-role proxy stood in and the substrate test checked only that a threshold was *stored*, not that
it *gated* the act (delta `rulechange-quorum`, `SPEC-DIVERGENCE-REGISTER.md`). The fix built the real
mechanism: a RuleChange now carries a **content-hash approval subject** so a policy change is admitted
only once k **distinct personae by lineage** have each signed an approval that references the change's
exact content hash — the same distinct-personae path membership already uses. Proven RED→GREEN
(disabling the arm fails 2 cases; `rulechange_threshold_enforced.rs`, 4 cases; and end-to-end through
the real `Session` API in `rulechange_quorum_via_api.rs`), with a manual mutation gate
(`act_subject→None` and `rule_change_approval_subject→const` both killed by named tests).

This is the concrete realization of R3 (a revocation/governance change "folds deterministically, so
all synced honest member nodes agree") **generalized from membership/role revocation to rule
changes**. The R-series is deliberately mechanism-neutral, so the cleanest fold is a short realization
note, not a rewrite of R3.

**Diff — §7.2, add one realization bullet after R6** (the R-list currently ends at R6, then
"R3 and R6 are what defeat a silent state-reset…"):

```diff
  - **R6, Attributable acceptance.** A participant accepting a write under a capability **MUST**
    record the causal frontier of governance facts it had synced at acceptance …

+ - **R7, Content-bound quorum for policy changes.** A change to replicated policy (a threshold, a
+   rule) **MUST** be admitted only when k distinct personae — counted by lineage (§4.5), never by
+   client or device — have each authored an approval fact that references the **content hash of the
+   exact change** (its "approval subject"), so an approval cannot be replayed onto a different
+   change and the quorum is *enforced at fold time*, not merely recorded. Three semantics are
+   normative, not incidental. (1) **The prior rule governs.** The threshold consulted is the one in
+   force at the act's position in the causal order — the rules *before* the change applies — never
+   the rules the change would install; otherwise a proposer could lower the gate with the very act
+   being gated. A change to the rule-change threshold itself is the marquee case and is gated
+   identically. (2) **Under-quorum is pending, never partial.** Approvals are ordinary governance
+   facts authored before the enacting act and referenced by it as antecedents (the §5.7 co-signed-op
+   shape); a proposal below threshold is simply a proposal whose enacting act does not yet exist,
+   and an enacting act arriving without sufficient matching antecedent approvals is rejected
+   deterministically by every honest node — it never applies partially and never waits in fold
+   state; enactment is always a new act referencing the accumulated approvals, and concurrent
+   enactors are benign per §7.3's decision-folds/enforcement-commits rule. (3) **The subject
+   preimage is the canonical payload encoding.** The approval subject is the digest of the change's
+   canonical payload bytes (§4.6), not of the enclosing fact envelope — it must be computable before
+   the enacting act exists, and an envelope digest would be circular against the antecedent
+   references in (2). This introduces no canonicalization surface beyond the one §4.6 already makes
+   load-bearing; the subject's byte-level encoding is pinned with the others (`[gates-release]`,
+   Appendix B). This is R3 applied to policy changes: the approvals are governance facts that fold
+   deterministically to the same admit/reject on every honest node. `Modeled` (RED→GREEN:
+   `rulechange_threshold_enforced.rs`; end-to-end via the session API: `rulechange_quorum_via_api.rs`;
+   manual mutation gate passed. Formal cross-package `cargo-mutants` sweep pending — X3).
```

Residuals to carry with R7 (same landing spot, one paragraph after the R-list closing remark):

```diff
+ Two residuals bound R7's current evidence. First, the reference fold retains a hard-coded
+ **role-authorship gate** alongside the quorum (a RuleChange author must hold Owner; an Approval
+ author must hold Owner or Admin): a spike simplification standing in for per-act approver-role
+ granularity, which remains open — R7 does not retire it, and "enforced" here means the count, not
+ the role model. Second, **two concurrent RuleChanges to the same rule that both meet quorum** are a
+ genuine contradiction under §7.6, decided as such (see the §7.3.2 boundary note), and the fold's
+ behavior there carries no evidence tag until the two-competing-quorums experiment runs.
```

**Diff — §8.2(e), tighten the honesty boundary** (currently a flat "not test-run"):

```diff
- (e) the membership-op freshness threshold and the admin-floor rule are decided but not yet test-run;
+ (e) the membership-op freshness threshold is decided but not yet test-run, and the admin-floor rule
+ is decided; policy-change quorum **enforcement** is now test-run (RED→GREEN, §7.2 R7), though the
+ freshness precondition on originating such an op (§7.4.2) is not yet exercised over live transport;
```

**The call for you.** (1) Is `R7` the right home, or would you rather this sit in §7.3 (governance
facts) as a worked realization? (2) Status tag: I've proposed `Modeled` (reference-implementation +
manual mutation gate), *not* `Verified`, because the formal cross-package mutation sweep (X3) hasn't
run — confirm that's the bar you want. Called 2026-07-13: (1) R7 in §7.2 with a §7.3 cross-reference;
(2) `Modeled` until X3.

**Mutation evidence added by RUN-01 EXP-3.** The formal scoped `cargo-mutants` sweep now backs the R7
enforcement: **threshold-counting has 0 survivors** in-substrate (`governance.rs` `threshold_met` /
`count_personae_by_lineage` / `required_threshold_for_rule_change` all caught), and the F1
approval-subject function `rule_change_approval_subject→const` was **hand-killed against the
cross-package test** `approval_for_a_different_change_does_not_count`. This strengthens the case for
`Modeled`; the bar for `Verified` remains the *automated* cross-package sweep (61 authorization-decision
survivors resolve only when the consumer suite runs against substrate mutants — residual X3). See
`local_storage_projection/X3-CROSS-PACKAGE-SWEEP.md`.

**Boundary sharpened by RUN-01 EXP-4 (competing quorums).** R7 quorum enforcement is real but **not
concurrent-conflict-aware**: a RuleChange is admitted once its own quorum is met, but the fold does
**not** escalate when *two competing quorums* concurrently admit conflicting changes to the same rule —
they auto-resolve order-dependently (`fork="clean"`, last-folded wins), a silent I5 violation on a
§7.6.1 shape (refutation `two_competing_rulechange_quorums`, register `competing-quorum-autoresolve`).
So R7 should read "enforced per-act" and must **not** be over-read as "the concurrent case is handled."
The competing-quorum contradiction predicate is a separate, design-gated item (backlog §2a) — mirrors
the mutual-expulsion predicate but for RuleChange. Recommend R7 land with an explicit caveat that
concurrent competing quorums are an open §7.6.1 escalation shape, not yet built.

---

## F2 — Re-plant membership continuity: `Design` → corroborated  ·  `status-move`  ·  ready

**Target:** §7.6.2 (re-plant, three arities) / §7.6.11 (the instantiation mechanism). §7.6.4 closes
`Design.`; the specific property "the MLS-stamped member set equals the fold-derived member set"
carries no evidence tag today.

**Why + evidence.** `mls-replant` (E12.1, E12.3–E12.6; suite 7/0 on **real openmls 0.8.1**) proved the
re-plant mechanics (dedup-not-fork keystone, drift reset, leaf rotation, last-resort availability), and
`replant-continuity` (3/0) proved the load-bearing invariant directly: driving the real
`DerivedFold::ingest` path, the MLS-stamped member set is **exactly** the set the governance fold
derives — across genesis, authorized adds, real removals, and *rejected* unauthorized changes.

**Diff — §7.6.2, append a corroboration sentence** to the paragraph that defines re-plant ("read
current membership from the governance chain, instantiate a fresh MLS group over it, and atomically
repoint"):

```diff
  A fork is not a distinct mechanism from a heal or a routine re-key; all three are the **same
  operation at different arity**, which the delivery layer calls **re-plant** (§6): read current
  membership from the governance chain, instantiate a fresh MLS group over it, and atomically repoint
- the conversation to it.
+ the conversation to it. `Verified` (the membership half: an MLS group stamped over the fold-derived
+ set has exactly that set as its cryptographic membership, across genesis, authorized adds, real
+ removals, and rejected unauthorized changes — driven through the real fold-ingest path, on real
+ openmls 0.8.1). The **message**-continuity half (an in-flight conversation surviving the repoint
+ with no loss or dup) is not yet built — it needs the dataplane hash structures (Appendix B / B1).
```

**Decision:** `ready`. Scope is deliberately narrow — only the membership property upgrades; message
continuity stays explicitly open so the tag can't be over-read.

---

## F3 — Connect-time catch-up demonstrated; RBSR still unimplemented; steady-state open  ·  `caveat`  ·  ready

**Target:** §6.8.1 (gap-aware history convergence). Currently the gap-fill is `Verified`
source-agnostically, and "the RBSR production construction … is a §5 decision; the scaling shape is
`Verified`, the specific construction is not yet chosen."

**Why + evidence.** Delta `x2-backfill` was reconciled: the late-joiner catch-up no longer rests on a
per-tick whole-log **re-flood** (the original stand-in). But note the *shape* of what was built —
`iroh_bus.rs` re-broadcasts the **whole retained log once on `NeighborUp`** (`TAG_RESYNC`). That is
spec-*shaped* (out-of-band, connect-triggered, O(log) per join) but it is **not RBSR**: it ships the
whole log, not a range-reconciled *diff*. So §6.8.1's efficient range-based mechanism is corroborated
in *direction* only, and two sub-properties remain unproven: (1) RBSR's diff-only efficiency, and (2)
**steady-state anti-entropy** — recovering a live frame dropped to an *existing* neighbor with no new
join to trigger a resync.

**Diff — §6.8.1, annotate the RBSR-construction line:**

```diff
- The RBSR production construction (Willow 3d-range versus Negentropy) is a §5 decision; the scaling
- shape is `Verified`, the specific construction is not yet chosen (Appendix B).
+ The RBSR production construction (Willow 3d-range versus Negentropy) is a §5 decision; the scaling
+ shape is `Verified`, the specific construction is not yet chosen (Appendix B). Connect-time
+ catch-up was additionally demonstrated over **real iroh-gossip** (a late joiner reaches an identical
+ head on `NeighborUp`), but via a whole-retained-log re-broadcast, a **coarser push cousin of RBSR**,
+ not the diff-only range reconciliation itself; and **steady-state anti-entropy** (a live frame lost
+ to an existing neighbor, no new join) is not yet exercised. Both remain open (Appendix B).
```

**Decision:** `ready`. This *removes* a possible over-read (that live-gossip green means RBSR works)
rather than adding a claim.

---

## F4 — §11.11 measurement #1: both halves now measured (fan-out on loopback)  ·  `status-move`  ·  ready

**Target:** §11.11, measurement #1 ("Per-commit and fan-out cost at hot-N = 500 / 1000 / 2000 …",
tagged `Load-bearing, unearned`).

**Why + evidence.** `mls-replant`'s M1 study measured the per-commit re-key cost band on real openmls
(an O(N) floor ↔ O(log N) ceiling per commit). The **fan-out** half is now measured too (RUN-01 EXP-1,
`alpha/experiments/croft-chat/FANOUT-M1.md`): N local `serve` processes converging over real
iroh-gossip at N = 2/4/8/16 on the loopback testbed. Findings: **per-node gossip cost is linear in the
live set** (`live_sent = 2N + 1`), aggregate O(N²) (inherent to flood gossip), and **head convergence
holds at every N** (identical fingerprints — I5 scales across the fan-out). One honest flag, carried
into the annotation: the **connect-time resync** path is super-linear on the bootstrap hub and
full-settle (`pending == 0`) does not complete past N ≈ 8 in the measured window — which is the same
open gap F3 records (RBSR / steady-state anti-entropy), now with a concrete cost signal. Scope note:
in this testbed a membership boundary is a **governance fact over gossip**, not an openmls commit, so
this is the fan-out *volume/latency* curve; the cryptographic per-commit band stays with M1.

**Diff — §11.11, annotate measurement #1** (append to its paragraph):

```diff
  1. **Per-commit and fan-out cost at hot-N = 500 / 1000 / 2000, on representative hardware.** …
     plus a gossip testbed measuring fan-out latency and total message count versus live-N.
+    *Measured (2026-07):* the **per-commit** band is measured on real openmls 0.8.1 (an O(N) floor ↔
+    O(log N) ceiling per commit; `alpha/experiments/mls-replant`, M1). The **fan-out** half is now
+    measured over real iroh-gossip on a loopback testbed at N = 2/4/8/16 (`croft-chat/FANOUT-M1.md`):
+    per-node gossip cost is **linear in the live set** (`2N+1`), aggregate O(N²), and head convergence
+    holds at every N (identical fingerprints). Two boundaries remain: the fan-out figures are on
+    **loopback, not representative hardware at hot-N = 500+** (magnitude indicative, register
+    `fanout-single-run`), and the **connect-time resync** cost is super-linear on the bootstrap hub —
+    full-settle does not complete past N ≈ 8 in-window, the RBSR / steady-state gap of §6.8.1. The
+    measurement moves from *unearned* to *earned in shape* (both halves), *magnitude-open at scale*.
```

**Decision:** `ready`. Does not touch the §11.12 posture table (no decided posture changes). The
super-linear-resync flag is consistent with — not a new claim beyond — F3's RBSR annotation.

**Landing state (post RUN-02).** RUN-02 already landed the *half-earned* wording into Part 2 §11.11
("fan-out half stays unearned though runnable… moves from unearned to half-earned"). RUN-01 EXP-1 then
*measured* the fan-out, so this F4 revision **supersedes** that landed wording: it proposes a follow-on
§11.11 edit from *half-earned* → *earned in shape* (loopback), magnitude-open at hot-N = 500+. Not yet
applied to Part 2 — this is the next §11.11 touch, pending review.

---

## F5 — §8.2(a) freshness: loopback live transport earned, relay path still open  ·  `caveat`  ·  ready

**Target:** §8.2(a). Currently: "freshness (§7.4) is proven in the model, not yet over live
transport."

**Why + evidence.** `croft-chat` + `iroh` converge over **real iroh-gossip** — 2- and 4-node `serve`
processes reach identical fingerprints — but on **loopback, no relay** (delta `hermetic-gossip` is
still active). The relay + holepunch path (real NAT) is **X1**, unreproducible where Internet UDP is
blocked. So "live transport" is now *partly* earned and the boundary should say exactly which part.

**Diff — §8.2(a):**

```diff
- (a) freshness (§7.4) is proven in the model, not yet over live transport;
+ (a) freshness (§7.4) is proven in the model and demonstrated over **loopback** live transport
+ (2- and 4-node real iroh-gossip convergence to identical fingerprints); it is **not** yet proven
+ over the **relay + holepunch (real-NAT)** path, which needs live NAT traversal (X1);
```

**Decision:** `ready`. Keeps the honesty boundary open where it must be (the relay path) while
recording the real loopback evidence.

---

## F6 — §7.6.3 ReInit stranding window: leave `[confirm]` as-is  ·  `caveat`  ·  ready (no spec edit)

**Target:** §7.6.3 (ReInit-not-atomic; "the governance chain records the re-plant intent … before the
freeze … Whether the ordering is genuinely intent-recorded-before-freeze is the open question …
`[confirm]`").

**Why + evidence.** `mls-replant`'s last-resort-availability result (E12.6) shows a re-plant can be
*completed* from the chain, which touches **availability** — but it does **not** establish the
**intent-recorded-before-freeze ordering** the discharge actually requires. So the honest move is *no
change*: the `[confirm]` stands. Recorded here only so the next reader doesn't mistake E12.6 for the
discharge.

**Decision:** `ready` — nothing to apply to Part 2. This is a register note, included so the diff set
is complete and the non-change is deliberate, not an omission.

---

## F7 — §10 / §9 conformance gaps: annotate cats 7/8/9 as not-yet-emitted  ·  `caveat`  ·  called

**Target:** §10.2 / §10.3 realization ledger and §9 (conformance). Tier-0 meer is `Verified` (zero
payload keys); the conformance-vector work is partial.

**Why + evidence.** The `iroh` conformance-core records categories **7/8/9** (AR / visibility /
freshness) as `not_yet_emitted`, and the revoke-authority-threshold vector as a `PLACEHOLDER`. These
are gated on two unbuilt pieces: MLS key-distribution **over the wire** (today a modeled registry) and
threshold-revoke as real k-of-n **over the wire** (today an MD-G5 sha-256 MAC stand-in).

**Proposed:** a one-line status annotation in the §10 realization ledger (and/or §9) that conformance
categories 7/8/9 and the revoke-authority vector are **specified but not yet emitted**, gated on
MLS-key-distribution-over-wire and threshold-revoke-over-wire.

**The call for you.** I've left this as prose rather than a quoted diff because the §10 ledger is a
table and I'd rather you point me at the exact row to touch than guess the cell format. If you want it
in, say where and I'll render the precise diff. Called 2026-07-13: annotate now, as a footnote line
immediately beneath the §10.5 realization-ledger table (not a cell edit), naming cats 7/8/9 and the
revoke-authority vector as specified-but-not-yet-emitted, gated on MLS-key-distribution-over-wire and
threshold-revoke-over-wire.

---

## F8 — Two concurrent quorum-met RuleChanges to the same rule are a genuine contradiction, not a tiebreak  ·  `caveat`  ·  called

**Target:** §7.3.2 (the "Boundary with the §7.6 hard-stop" paragraph) and §7.6.1 (the two-member
escalation set). New item this run, surfaced by R7: a content-address tiebreak *is* available for two
concurrent quorum-met RuleChanges to the same rule, so the design has to say, on the record, why it is
refused.

**Why + evidence.** R7 makes policy changes quorum-gated (F1), which raises the concurrency question it
does not itself answer: what happens when two RuleChanges to the same rule each meet quorum concurrently.
A deterministic content-address tiebreak would resolve it silently, but a silently-losing *rule* rewrites
the Group's constitution downstream with the disagreement never surfaced, manufacturing a utility verdict
(Part 1 §2.5) in a way a superseded-but-visible membership removal does not. The owner's call this run is
that this collision is a **§7.6-class genuine contradiction**, hard-stopped and human-adjudicated, never
tiebroken. The protocol's obligation is a transparent, unambiguous, grounded statement of the two
conflicting facts in governance language, with no editorializing. No experiment has exercised the fold's
behavior in this case yet (the two-competing-quorums experiment is backlogged), so it carries no evidence
tag.

**Diff — §7.3.2, append to the "Boundary with the §7.6 hard-stop" paragraph:**

```diff
+ One case is decided on the escalation side of this line by design rather than by tiebreak
+ availability: two concurrent policy changes (§7.2 R7) to the same rule that each meet quorum. A
+ content-address tiebreak is available and would be deterministic, but it is refused: a losing
+ membership removal remains a visible, auditable superseded entry, while a silently losing *rule*
+ rewrites what the Group's constitution does downstream without the disagreement ever being seen,
+ which manufactures a utility verdict in exactly the sense Part 1 §2.5 forbids. Such a collision is
+ most often a misunderstanding or a legitimate grievance, and the protocol's obligation is the §7.6
+ posture: hard-stop and present the contradiction as an unambiguous, grounded statement of the two
+ conflicting facts in governance language, with no editorial resolution. `Design`, decided; the
+ fold's behavior carries no evidence tag until the two-competing-quorums experiment runs.
```

**Diff — §7.6.1, extend the "Contradiction: too many valid claims" enumeration:**

```diff
- ... each committing the other's removal against the same epoch; or a removed-then-included merge.
- Detected as concurrent commits that will not linearize.
+ ... each committing the other's removal against the same epoch; or a removed-then-included merge;
+ or two concurrent quorum-met policy changes to the same rule (§7.2 R7), which escalate here by
+ design rather than by tiebreak availability (§7.3.2). Detected as concurrent commits that will not
+ linearize.
```

**Decision:** `called` (owner's decision this run). Recorded so R7's concurrency boundary is explicit
rather than left as an unstated tiebreak. `Design`, decided; no evidence tag until the experiment runs.

**RUN-01 EXP-5 assessment (no cats moved).** EXP-5 targeted exactly these gaps. Finding: the
**MLS-key-distribution-over-wire** half is **already realized in a spike** (`iroh/crates/mls-welcome-over-iroh`
— a real openmls Welcome over a real iroh connection; the joiner derives the identical exporter secret +
lineage fold from the wire), so "the modeled registry" is made real there, though not yet wired into
conformance emission. The **threshold-revoke-over-wire** half is **design-gated** (the revocation-authority
model — who-may-revoke / the k-of-n dial / key discovery; MASTER-INDEX I9) and was **stopped, not
improvised**, per the brief's EXP-5 stop rule (options A/B/C in backlog §6d-i). So conformance cats 7/8/9
and the revoke-authority vector **remain not-yet-emitted** — the F7 annotation stands unchanged, now with
a named blocker (the revocation-authority decision) rather than an open "TBD".

---

## Summary table

| # | Target § | Class | Decision | One-line |
|---|---|---|---|---|
| **F1** | §7.2 R7 + §8.2(e) | new-mechanism | called | RuleChange quorum *enforced* via content-hash approval subject |
| **F2** | §7.6.2 | status-move | ready | Re-plant membership continuity → `Verified` (membership half only) |
| **F3** | §6.8.1 | caveat | ready | Connect-time catch-up shown; RBSR + steady-state still open |
| **F4** | §11.11 #1 | status-move | ready | Per-commit **and** fan-out measured (fan-out: linear per-node `2N+1`, O(N²) aggregate, heads converge; resync super-linear past N≈8) |
| **F5** | §8.2(a) | caveat | ready | Freshness earned on loopback; relay path (X1) open |
| **F6** | §7.6.3 | caveat | ready | No change — `[confirm]` stands; E12.6 ≠ discharge |
| **F7** | §10 / §9 | caveat | called | Conformance cats 7/8/9 not-yet-emitted |
| **F8** | §7.3.2 + §7.6.1 | caveat | called | Two quorum-met RuleChanges to one rule → §7.6 contradiction, not tiebreak |

**Landing plan.** Landed by RUN-02 (2026-07-13) as one dated `part-2-changelog.md` entry
("Pass: 2026-07 real-substrate experiment reconciliation") with the edits applied to Part 2. The
`needs-call` items (F1, F7) were answered by the owner (see each item's call), F8 was added as a new
decision this run, and all items landed together.
