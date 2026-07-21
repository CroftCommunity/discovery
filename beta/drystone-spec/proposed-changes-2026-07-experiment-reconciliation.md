# Proposed Part 2 changes — experiment reconciliation (2026-07, for review)

`Status: PROPOSED, not applied. This is a reviewable diff, not a spec edit. Part 2 is reviewed
material; nothing here touches part-2-certifiable-design.md until you approve it line by line. On
approval, each accepted item lands as a part-2-changelog.md entry (house style: "§X, done. <what>
(primer §Y).") and the corresponding edit is made to Part 2.`

> **Historical record — all items landed (RUN-02, RUN-03, RUN-06). The authoritative text is Part 2.** F1–F8
> were applied to Part 2 by RUN-02 (2026-07-13), and F8's implementation gap was closed by RUN-03 Phase B
> (2026-07-14). F4's fan-out re-measurement follow-on also landed: RUN-06 (2026-07-14) regraded §11.11
> measurement #1 from *half-earned* to *earned in shape* (both halves), and RUN-09 (2026-07-15) sharpened it
> to *magnitude replicated at loopback, open at hardware scale* (K=5 replication; the `fanout-single-run`
> register retired). See RUN-02/RUN-03/RUN-06 summaries and the Part 2 changelog.

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
§11.11 edit from *half-earned* → *earned in shape* (loopback), magnitude-open at hot-N = 500+. **Landed
in Part 2 §11.11 (RUN-06, 2026-07-14)** — measurement #1's annotation now reads *earned in shape* (both
halves), magnitude-open at scale, carrying the `fanout-single-run` register caveat and the super-linear
connect-time-resync flag.

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

**RUN-08 update (2026-07-15).** Two things reconcile this annotation to ground truth. (1) The reference
conformance-core is now folded into `discovery/alpha/Proofs/lineage-groups/crates/conformance` and
re-proves **66/0** across cats 1–9 in-environment — cats **7** (adversarial AR-1…AR-6, real Rust), **8**
(visibility, TS-authoritative), **9** (freshness, TS-authoritative), and the cat-5b revoke-authority
*mechanism* (real Ed25519 bundle over a lineage-counted quorum) are all emitted — so "cats 7/8/9
not-yet-emitted" is **superseded for the vectors themselves**; the honesty boundary is the over-the-wire
*sourcing*, not the vectors' existence. (2) Part 1B lands the **MLS-key-distribution-over-wire** half
`Verified` green-real at **loopback grade**: `mls-welcome-over-iroh` reproduced in-environment
(`relay-lab-runs/C-mls-welcome-2026-07-15-run08` — a real 1466-byte openmls Welcome over real iroh;
identical exporter secret + identical lineage fold), so the verifying-key/standing registry is
demonstrably sourceable from real over-the-wire MLS. Wiring that source into the conformance *emitter*
itself (today in-process) is the residual, and the real-NAT path stays X1. The **threshold-revoke-over-wire**
half, together with the co-sign-vs-vote authority ordering, **stays gated** on the revocation-authority
trust model (MASTER-INDEX I9) — the RUN-08 firewall; untouched. Part 2 §10.5 footnote updated to match.

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
| **F7** | §10 / §9 | caveat | called (RUN-08 update) | Cats 7/8/9 + revoke-authority *mechanism* emitted, 66/0 (RUN-08); key-distribution-over-wire green-real at loopback; threshold-revoke-over-wire + authority ordering gated (I9) |
| **F8** | §7.3.2 + §7.6.1 | caveat | called | Two quorum-met RuleChanges to one rule → §7.6 contradiction, not tiebreak |

**Landing plan.** Landed by RUN-02 (2026-07-13) as one dated `part-2-changelog.md` entry
("Pass: 2026-07 real-substrate experiment reconciliation") with the edits applied to Part 2. The
`needs-call` items (F1, F7) were answered by the owner (see each item's call), F8 was added as a new
decision this run, and all items landed together.

---

## RUN-14 — Stellin AppView caller-identity (social-mapping §H serve half + helper delegation)

> **Historical record — LANDED 2026-07-17 (owner-approved).** H-A, H-B, and H-C were applied to
> `social-mapping.md`: the §H serve-half gained a "serve half is now demonstrated" paragraph tagged
> `green-real` (offer-gating against a verified identity, compilation-boundary content-blindness), the
> §L helper-delegation bullet gained the content-helper demonstration (index-by-grant + revocation
> forward-blindness) tagged `green-real`, and the H-C caveat (service auth ≠ interactive OAuth/DPoP)
> was folded into the §H paragraph. No grade was moved to `Verified` — the evidence is loopback /
> in-process. The AppView-provisioned scope key was left `Design; open.` (untouched). The authoritative
> text is now `social-mapping.md`; this section is the reviewable diff of record.

`Status: LANDED 2026-07-17 (was PROPOSED, staged same day). Target here is the design text
beta/impl/drystone-design/social-mapping.md (§H and the helper-delegation bullet, §L), NOT Part 2.
The AppView-provisioned scope key (social-mapping Open items) is deliberately untouched — RUN-14 stop
rule 5b. Each item cites the RUN-14 experiment that earns it (RUN-14-SUMMARY.md, branch
claude/experiments-run-14).`

### H-A — §H serve half: offer-gating demonstrated against a real verified identity  ·  `status-move`  ·  ready

**Target:** social-mapping §H, the "AppView gates *offering*, never *reading*" invariant (currently
`Synthesis`), and its recommended hybrid default (MLS seal, AppView serve).

**Why + evidence.** EXP-B built the §H hybrid **serve half** executably: a content-blind store offers
opaque ciphertext only to a **verified** roster member (service-auth identity from EXP-A), and refuses
non-member/anonymous/nonexistent-group with one flat 403 (no length/existence leak). The content-blind
property is a **compilation boundary**, not a convention — the seal/open AEAD crate is absent from the
`sealed` server binary's dependency graph (`cargo tree` shows it only under `--features client-seal`),
so the store cannot read what it offers. Roster removal stops future offering while already-fetched
ciphertext + a retained key still decrypts — the "offering-vs-reading" sentence made a passing test.
EXP-A separately earns the "verified caller" half: real atproto service-auth JWTs verified against
real DID-document keys (secp256k1/p256), live P-A3 confirmed against `@bsky.app`.

**Proposed move:** the §H serve-half invariant → **experiment-earned** (was `Synthesis`) for the
*serving/offer-gating* mechanism specifically. The confidentiality guarantee still rests on encryption
(unchanged); what is newly earned is that an AppView can gate *offering* by verified identity without
holding the key.

**Decision:** `ready` (mechanism demonstrated at experiment grade; loopback/in-memory, no wire pinning).

### H-B — helper delegation: content helper indexes by grant, forward-blind on revocation  ·  `status-move`  ·  ready

**Target:** social-mapping §L helper-delegation bullet ("a *content* helper … may hold clear text …
and it is revocable … In neither case does the helper gain authority"), currently `Design`.

**Why + evidence.** EXP-C (`helper-seam`) closed the grant→index→serve loop on the **real MLS mechanism**
(`group-seal`, croft-group L2a): a helper admitted by a real Welcome decrypts group messages as any
member does, normalizes them through the source-agnostic `NormalizedEvent` boundary (copied from
public-roundtrip), and feeds the **same** index/serve path a public source feeds — one search returns
both a public-source and a helper-fed hit. Revocation (`remove_member` + epoch roll) makes the helper
**forward-blind**: frames sealed after the roll do not decrypt (MLS forward secrecy) and produce no
rows, while pre-revocation rows remain — the honest asymmetry ("what the helper was shown, it was
shown") stated in the test. The helper exposes **no authority surface** (join + ingest only).

**Proposed move:** the helper-delegation claim → **experiment-earned** (was `Design`) for the content-
helper mechanism (admit-by-grant, index-by-grant, revoke-to-forward-blind, no-authority).

**Decision:** `ready` (loopback grade; real openmls 0.8.1 seal/Welcome/PCS-removal; in-process harness,
no wire pinning — same grade wall as croft-group L2a).

### H-C — the standing gap stays named: interactive OAuth/DPoP is unproven  ·  `caveat`  ·  ready

**Why + evidence.** The service-auth path (EXP-A) is NOT the PWA client-login leg. Interactive atproto
**OAuth + DPoP** requires a browser hop this environment lacks; it was explicitly not attempted (RUN-14
named non-goal). Note EXP-A's own token leg is now **fully confirmed live** (P-A1/P-A2/P-A3, owner-
supplied creds — a real getServiceAuth token verified end-to-end against a real DID-doc key); the
remaining gap is *only* the interactive OAuth/DPoP client login, a distinct mechanism.

**Proposed caveat:** wherever the AppView caller-identity mechanism is recorded as earned, annotate that
it covers **service auth (server-to-server), not the interactive client-login leg** — OAuth/DPoP remains
attended-run territory (backlog). Keeps the gap register honest.

**Decision:** `ready` (a boundary annotation, no mechanism claim).

### RUN-14 summary addendum

| # | Target | Class | Decision | One-line |
|---|---|---|---|---|
| **H-A** | social-mapping §H serve half | status-move | ✅ landed | Offer-gating demonstrated against a verified identity; content-blindness is a compilation boundary (`Synthesis` → `green-real`, loopback) |
| **H-B** | social-mapping §L helper delegation | status-move | ✅ landed | Content helper indexes by grant over real MLS; forward-blind on revocation; no authority (`Design` → `green-real`, loopback) |
| **H-C** | social-mapping §H (caveat) | caveat | ✅ landed | Service auth ≠ interactive OAuth/DPoP (the PWA login leg stays unproven) |

**Untouched (RUN-14 stop rule 5b):** the AppView-provisioned scope key (social-mapping Open items) —
how the audience scope key is provisioned, granted, and rotated — is a design decision, not earned here.

---

## RUN-15 — the access-gated large-group tier takes the trusted-gatekeeper arm (this tier only)  ·  `caveat`/`needs-call`

`Source: RUN-15 hosting-kit design brief, alpha/experiments/appview-infra/GROUPS.md (D11). This is a
design-stance reconciliation, not an experiment-earned status move — no code proves a stance. Staged
for review; the reviewed spec is untouched.`

**Target:** social-mapping Open items — the **AppView-provisioned scope key** (left `Design; open` by
RUN-14, stop rule 5b): "how the audience scope key is provisioned, granted, and rotated."

**The stance (needs-call).** For groups **past `group_scale_boundary`** (a policy parameter, working
number 5000, deferred to the owner), the design accepts that cryptographic group confidentiality is a
mirage at scale (the member-leak equivalence: in a group of thousands, no-outside-leak is unattainable
because any member can re-publish). Such groups therefore serve **private but not E2EE** — the AppView
reads content and gates *offering* it by verified roster membership, as a stated **trusted gatekeeper**.
This deliberately takes the **trusted-gatekeeper arm** of the open scope-key item, **for the large tier
only**. Below the boundary the content-blind §H stance (RUN-14 EXP-B, `green-real`) is unchanged.

**The write-path fork stays an owner decision** (GROUPS.md §2): Variant A (repo-canonical ciphertext,
AppView decrypts to serve — scope key + roster are the only server-canonical state) vs Variant B
(server-canonical content — heaviest backup, weakest portability). D12 builds only the fork-agnostic
roster-gated serving behind a `GroupStore` trait; neither variant is chosen here.

**Proposed caveat, if accepted:** annotate the scope-key open item to record that the large-group tier
(≥ `group_scale_boundary`) serves under an explicit trusted-gatekeeper posture — private, roster-gated,
not cryptographically confidential — and that the scope-key provisioning question is scoped by the A/B
write-path decision. The small-group content-blind mechanism is untouched.

**Decision:** `needs-call` — this is a values/posture decision (portability & custody vs immediacy &
simplicity, and the honesty of the trusted-gatekeeper stance for large groups), not a mechanical move.
The three owner questions are in GROUPS.md §5 (variant, boundary number, croft-groups launch order).

### RUN-16 update — the two-tier framing is superseded by the three-tier / two-axis model  ·  `caveat`/`needs-call`

`Source: RUN-16 canonical model text, alpha/experiments/appview-infra/GROUPS.md (Section A, v2). This
extends — does not rewrite — the RUN-15 stance above. Still a design-stance reconciliation, not an
experiment-earned status move; still staged for review; the reviewed spec (part-*, conventions) is
untouched. The RUN-15 note stays valid: the RUN-16 model is the same posture generalized, so what was
"the large tier only" is now named precisely as the backplane tier.`

**What changes in the framing.** RUN-15 posed two tiers (below/above `group_scale_boundary`). RUN-16
supersedes that with **one lineage, one envelope, one delivery plane, one catalogue**, and makes the
tier a pair of independent policy values on a scope (GROUPS.md A.2): a **membership policy**
(`open` | `gated` | `sealed`) and a **write policy** (`open` | `members` | `named-set` | `single`). The
scale boundary is no longer where a group *becomes* something else; it is where a **gated (backplane)**
scope's serving economics justify the trusted-gatekeeper offering. The RUN-15 write-path fork and its
owner decision are unchanged and preserved (GROUPS.md Section B; restated in A.10).

**The trusted-gatekeeper acceptance now attaches to the backplane tier, with membership as universally
verifiable public fact.** In the sealed tier, authorship and membership stay **fused** (the MLS
key-schedule membership MAC). In the backplane (gated/open) tiers there is no key schedule, so the
fused check **splits** into two independent, publicly-computable ones: a **signature verified against
the author's DID-document key** proves authorship, and a **roster lookup at the message's causal
position** proves membership (GROUPS.md A.5). Membership is therefore a universally verifiable public
computation, not a server assertion — which is what makes the trusted-gatekeeper posture honest for
these tiers: the gatekeeper serves readable content, but who is a member is checkable by anyone against
the public fold, and the server holds no ordering or membership authority. This refines, and does not
retract, the RUN-15 acceptance of `Design; open` on the AppView-provisioned scope key (§H); Variant A's
server-held scope key is still the write-path choice that the scope-key provisioning question is scoped
by.

**The delivery plane is authority-free roles realized as separate processes, with a transport split.**
GROUPS.md A.7/A.8 name the delivery plane as a *set of roles*, each its **own process** — the web-native
Delivery Service, the swarm peer, the history-convergence node, and helpers — never fused into one
primitive, none holding ordering or membership authority (any sequence numbers a role assigns are
**delivery cursors, never order**; the covert-clock / no-arbiter rules). History access is **backfill
scoped by membership interval**. Transports split: the **iroh overlay and relays are loaded only by
sealed scopes and steward governance**, while backplane scopes ride the plain web stack end to end
(browsers first-class, no overlay), with optional per-group swarm gossip whose one survival rule is
**validate-before-relay**. Deduplication across delivery modes is the **envelope hash** in every tier.
This is a delivery-layer design stance; it touches no Part 2 mechanism and moves no status tag.

**Decision:** `needs-call`, unchanged from RUN-15 — the four owner questions are in GROUPS.md A.10
(write-path variant; the `group_scale_boundary` number, reframed as measurable via churn simulation;
launch order; and the new relay-hosting question — self-host an iroh relay vs public relays initially).
Nothing here is an experiment-earned move; the stance awaits the owner's call.

### RUN-18 addendum — reception completeness for write-restricted scopes; the degeneration principle  ·  `caveat`/`needs-call`

`Source: RUN-18 (alpha/experiments/RUN-18-SUMMARY.md) — the reception-completeness paragraph landed in
GROUPS.md A.2 and the publications positioning landed as
alpha/experiments/appview-infra/PUBLICATIONS.md, with the mechanism proven executable in tier-proof/
(B1–B6). This extends the RUN-16 update above; still staged, the reviewed spec (part-*, conventions)
untouched.`

**The chaining requirement, and completeness as detection.** In write-restricted scopes (`single`,
`named-set`) authors MUST chain their envelopes — each envelope carries the author's previous envelope
in its antecedents, the first anchoring to scope genesis. Reception completeness is thereby a
subscriber-side guarantee framed as **DETECTION, never delivery**: any reader holding envelope N can
verify the stream N-1..1 exists, detect any gap as a known omission, and repair it via
membership-interval backfill; the guarantee runs on public data alone and open enrollment never
weakens it. The honest limit is named per the completeness-ahead doctrine: a withheld TAIL is
undetectable until anything newer arrives by any path; multimodal delivery (DS plus optional swarm) is
the mitigation, and freshness/solicitation posture stays a governed dial, never a mechanism that
closes the limit. (Proven executable: RUN-18 B1–B4.)

**The degeneration principle as a design constraint.** Where a scope's policy pair asks nothing the
substrate does not already prove, the machinery MUST degenerate to bare atproto records plus chaining,
and nothing else — the open/single (newsletter) and open/open shapes ride bare records; authorship,
integrity, and current-state completeness are the substrate's native proofs and are not rebuilt
(PUBLICATIONS.md §1). Machinery switches on only where the policy pair asks for proofs vanilla
cannot give.

**The tamper-evident-history delta, the publication-facing consequence of chaining.** Vanilla atproto
proves a **tamper-free current state** (a deleted record is simply absent — retraction and
never-existence collapse into the same absence); chaining upgrades the stream to **tamper-evident
history**, where absence is classifiable three ways — **never-existed** (no chain references it),
**retracted** (referenced; deletion verifiable at source), **withheld** (referenced; no source offers
it and deletion cannot be shown). Retraction stays possible and honest; it can no longer be silent.
(Proven executable: RUN-18 B5; the auditable-reach corollary — a roster count any second fold
re-derives, and an unsupported asserted count detectable — B6.)

**Decision:** `needs-call`, unchanged — a design-stance extension riding the RUN-16 note; no status
tag moves; the reviewed spec is untouched.

## RUN-19 — the sealed tier's browser story loses its deferral: no bridge, custody-shaped caveats  ·  `status-move`/`needs-call`

`Source: RUN-19 (alpha/experiments/wasm-seal/ + alpha/experiments/RUN-19-SUMMARY.md). This is an
experiment-earned revision of landed design language — GROUPS.md A.8's "Honest costs, named"
paragraph — staged here per guardrail 3 (the landed doc is never edited by the run). The custody
posture below is DRAFTED FOR REVIEW, not ratified (guardrail 6).`

**Target:** `alpha/experiments/appview-infra/GROUPS.md` A.8, the deferred-browser sentence: *"and
the sealed tier's BROWSER story is deliberately deferred (WASM MLS plus a relay bridge; native
apps are the sane vessel for that tier for now)."*

**What RUN-19 showed.** The deferral's premise — that a browser sealed client would need an
overlay/relay bridge — does not hold. The croft-group L2a seal stack (`group-seal` → `lineage-mls`
→ openmls 0.8.1 + the pure-Rust provider) compiles to `wasm32-unknown-unknown` and RUNS there:
real group create / Welcome / seal-unseal / epoch roll / forward-blindness inside the module
(upstream OpenMLS CI builds but does not test this target — RUN-19 P1 supplies that evidence for
our stack). Cross-build goldens hold with **zero asymmetry** (wasm-sealed ↔ native-unsealed and
the reverse; transcript state byte-identical). And the full loop runs over **actual QUIC**: wasm
member → WebTransport → content-blind DS (no unseal capability in its dependency graph) →
offer-gated fetch → wasm member, commits and removals riding the same path, the removed member
still *offered* ciphertext it provably cannot *read*. Grades: wasm-node (the module under the
Node host, not a browser page — one headless-chrome attempt failed environmentally) and
quic-native (a native WebTransport client speaking the browser-identical protocol).

**Proposed revised sentence, if accepted:** *"the sealed tier's BROWSER client needs NO overlay
bridge — MLS in wasm over WebTransport to the content-blind DS is the path (RUN-19); its caveats
are custody-shaped, not transport-shaped (see the custody posture); native apps remain
first-class, not the only sane vessel."*

**The transport leg is product-promiseable (Baseline).** WebTransport reached cross-browser
Baseline in March 2026: Safari 26.4 (2026-03) joining Chrome 97+ (2022-01), Edge 98+ (2022-02),
Firefox 114+ (2023-06). Dev trust for a self-hosted DS uses `serverCertificateHashes`, whose
≤2-week certificate cap the pinned server library satisfies by construction
(`Identity::self_signed` = 14 days, RUN-19 PRED-WT3).

**Custody posture (DRAFT — FOR OWNER REVIEW, not ratified).** The browser caveats are about *key
custody in a page*, and they are bounded, not fatal: (1) group keys live in wasm linear memory
while the page runs — **XSS is the threat model** (a scripted page can drive the module's own API;
wasm memory is not an enclave), so the sealed surface must carry the strictest CSP/no-third-party-
script posture; (2) the **blast radius is bounded by device-key delegation** (RUN-17 P5): a
browser member joins under a delegated device key, never the account root — compromise burns one
device key; (3) **revocation is attestation deletion** (event-driven, no TTL) plus the MLS
removal re-key, which RUN-19 P5 shows evicting a member across the wire; (4) **eviction is
honest**: destroying the at-rest blob leaves no self-restore path (forward secrecy is not
overridden) — re-entry is a fresh add via Welcome, blind to the gap (RUN-19 P3); (5) state at
rest is ciphertext (AES-GCM via the provider) with the browser mapping WebCrypto-wrapped-key over
IndexedDB/OPFS, and MLS state is **single-writer** — the product must elect a tab leader (Web
Locks) before a second tab may touch the ratchet; (6) **draft-status pinning**: the WebTransport
ecosystem is draft-tracking — server and client must ship from matching revisions (RUN-19 pins
wtransport =0.7.1 for both sides; browsers track Baseline).

**Decision:** `needs-call` — the revised sentence and the custody-posture paragraph are language
for the owner to accept, amend, or reject; the evidence rows land regardless. The literal
browser page (product-side manual verification) and the croft-group gaps RUN-19 surfaced
(Welcome-joined member cannot invite — MissingRatchetTree; no state-at-rest surface in
group-seal/lineage-mls) are backlog rows, not part of this call.
