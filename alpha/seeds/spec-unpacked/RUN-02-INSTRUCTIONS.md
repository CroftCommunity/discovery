# RUN-02 — Land the 2026-07 experiment reconciliation into the spec set

`Scope: markdown surgery only. No code changes, no test changes, no register-row deletions.`

## Context (read first)

- Authoritative specs: `beta/drystone-spec/part-2-certifiable-design.md` (Part 2),
  `part-1-reasoning-underpinnings.md` (Part 1, expected untouched this run),
  `conventions-and-decisions.md` (vocabulary and the Rule 15 map convention).
- The staged diff set this run applies: `beta/drystone-spec/proposed-changes-2026-07-experiment-reconciliation.md`
  (items F1–F7; F1 and F7 were `needs-call` and the calls are answered below).
- Bookkeeping docs touched: `beta/impl/experiments/drystone-reviews-and-experiments-log.md`,
  `alpha/experiments/SPEC-DIVERGENCE-REGISTER.md`, `alpha/experiments/SPEC-ALIGNMENT-AND-ACTION-PLAN.md`,
  `alpha/experiments/EXPERIMENT-BACKLOG.md`.
- Rule 15: every Part 2 section edit must be reflected in the back `## 0. Map` (it lives after
  Appendix F). Also append entries to `beta/drystone-spec/part-2-changelog.md` per its existing format.

## Branch and sequencing

Fresh branch off `main`, e.g. `run-02-spec-reconciliation`. If the RUN-01 branch is unmerged, do
NOT rebase onto it or touch its files; if a file this run edits was also changed by RUN-01 on main,
stop and report rather than resolving silently.

## Decisions this run encodes (the owner's calls — do not re-open)

1. F1 lands now, as **R7 in §7.2** (not §7.3), status **`Modeled`** pending the X3 mutation sweep.
2. Two concurrent quorum-met RuleChanges to the same rule are a **genuine contradiction**: §7.6-class
   hard-stop and human adjudication, never a tiebreak. The protocol's obligation is a transparent,
   unambiguous, grounded statement of the conflicting facts in governance language — no editorializing.
3. croft-group L2–L5 **reuse** the proven Drystone crates; reuse is a **condition of considered
   compatibility** (a re-implementation does not count as compatible).
4. The recovery "largest open design problem" framing is stale: `open-threads.md` records a direction
   **confirmed 2026-07-07** (build the lock now: threshold across independent trust domains; trust
   predicate is per-deployment/per-persona policy). Only the trust-predicate design remains open.
5. `alpha/experiments/` is confirmed as the experiments home; the standalone `experiments/` repo is
   to be frozen/retired (note the decision; the actual freeze happens outside this run).
6. X1 (real-NAT relay path) stays an open honesty boundary; no hardware provisioning implied.

## Tasks, in order

### T1 — Amend F1 in the staged-diff doc

In `proposed-changes-2026-07-experiment-reconciliation.md`, section `## F1`:

a. Replace the entire ```diff block that adds the R7 bullet with the amended block below (verbatim).
b. Beneath it, add the residuals block below (verbatim) as a second diff block with a one-line lead-in
   "Residuals to carry with R7 (same landing spot, one paragraph after the R-list closing remark):".
c. Under "**The call for you.**", append: "Called 2026-07-13: (1) R7 in §7.2 with a §7.3 cross-reference;
   (2) `Modeled` until X3." and change the header's `**needs-call**` to `called`.
d. Update the F1 row of the summary table (line ~237) status accordingly.

Amended R7 block:

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

Residuals block:

```diff
+ Two residuals bound R7's current evidence. First, the reference fold retains a hard-coded
+ **role-authorship gate** alongside the quorum (a RuleChange author must hold Owner; an Approval
+ author must hold Owner or Admin): a spike simplification standing in for per-act approver-role
+ granularity, which remains open — R7 does not retire it, and "enforced" here means the count, not
+ the role model. Second, **two concurrent RuleChanges to the same rule that both meet quorum** are a
+ genuine contradiction under §7.6, decided as such (see the §7.3.2 boundary note), and the fold's
+ behavior there carries no evidence tag until the two-competing-quorums experiment runs.
```

### T2 — Apply F1 to Part 2

a. In §7.2, insert the R7 bullet (the `+` lines above, rendered as normal spec text, matching the
   R1–R6 bullet style exactly) immediately after R6 and before the paragraph beginning "R3 and R6 are
   what defeat a silent state-reset". Then insert the residuals paragraph after that closing paragraph.
b. Apply the staged §8.2(e) diff exactly as written in the F1 section.
c. In §7.3, at the text corresponding to fold-time threshold enforcement (the rules-at-position /
   Step-5.6 semantics; search near the §7.3.1 authorization precondition), add one cross-reference
   sentence: "Threshold enforcement for policy changes, including the approval subject and the
   prior-rule-governs semantics, is specified at §7.2 R7." Place it where the local prose style allows.

### T3 — Record the two-competing-quorums decision (new item; log it as F8)

a. In Part 2, at the §7.3.2 "Boundary with the §7.6 hard-stop" paragraph, append:

   "One case is decided on the escalation side of this line by design rather than by tiebreak
   availability: two concurrent policy changes (§7.2 R7) to the same rule that each meet quorum. A
   content-address tiebreak is available and would be deterministic, but it is refused: a losing
   membership removal remains a visible, auditable superseded entry, while a silently losing *rule*
   rewrites what the Group's constitution does downstream without the disagreement ever being seen,
   which manufactures a utility verdict in exactly the sense Part 1 §2.5 forbids. Such a collision is
   most often a misunderstanding or a legitimate grievance, and the protocol's obligation is the §7.6
   posture: hard-stop and present the contradiction as an unambiguous, grounded statement of the two
   conflicting facts in governance language, with no editorial resolution. `Design`, decided; the
   fold's behavior carries no evidence tag until the two-competing-quorums experiment runs."

b. Add a matching one-line note in §7.6 (or §7.6.1) if its enumeration of contradiction shapes has a
   natural insertion point; if none exists cleanly, skip and record the skip in the summary.
c. Add an F8 entry to the staged-diff doc (same format as F1–F7: target, why, diff, decision `called`)
   and an F8 row in its summary table.

### T4 — Apply F2–F6 as staged

Apply each staged diff verbatim to its target section. F6 requires no Part 2 edit (verify only).

### T5 — Apply F7 (call answered: annotate now, footnote placement)

Add the annotation as a footnote line immediately beneath the §10 realization-ledger table (not a
cell edit): conformance categories 7/8/9 (AR / visibility / freshness) and the revoke-authority
vector are specified but not yet emitted, gated on MLS-key-distribution-over-wire and
threshold-revoke-over-wire. If a ledger row unambiguously covers conformance vectors, additionally
annotate that row. Record the placement chosen in the summary. Mark F7 `called` in the staged doc.

### T6 — Rule 15 and changelogs

Update the back `## 0. Map` of Part 2 for every touched section (§7.2, §7.3, §7.6 if touched,
§8.2, §6.8.1, §7.6.2, §11.11, §10). Append `part-2-changelog.md` entries in its existing style.

### T7 — Reviews-and-experiments log entry

Append to `beta/impl/experiments/drystone-reviews-and-experiments-log.md` a new section
`## 2026-07-13, Real-substrate spikes, reconciliation landing` mirroring the v2 entry's structure:
what ran (the imported spike corpus), the three reconciled deltas and the one active
(`hermetic-gossip`), and the spec effects (F1/R7 new mechanism at `Modeled`; F2 membership-continuity
upgrade; F3–F5, F7 caveats; F8 decision; §8.2(e) tightened).

### T8 — Registers and the alignment doc

a. `SPEC-DIVERGENCE-REGISTER.md`: rows are already correct (three reconciled, hermetic-gossip
   active) — verify only; add to the rulechange-quorum reconciled row's evidence cell: "Spec landing:
   §7.2 R7 (RUN-02)."
b. `SPEC-ALIGNMENT-AND-ACTION-PLAN.md` §7 open decisions — annotate each in place (strikethrough or a
   bold "Decided:" lead per the doc's style):
   1 → home confirmed, standalone repo to be frozen (freeze itself out of scope).
   2 → executed by RUN-02.
   3 → reuse; reuse is a condition of considered compatibility.
   4 → correct the stale framing: cite `open-threads.md`'s 2026-07-07 confirmed direction; remaining
       open item is the trust-predicate design; the BIP39 paper-recovery spike is the Tier-1 first step.
   5 → boundary held open; no provisioning now.
c. `EXPERIMENT-BACKLOG.md`: update the two-competing-quorums item to carry the decided expected
   behavior (hard-stop + transparent grounded contradiction statement, per T3); update the
   croft-group L2–L5 item with the reuse decision and the compatibility condition.

## Guardrails

- Match house style exactly: status-tag placement, em-dash usage, bullet shapes, MUST/MAY casing.
- If any anchor text in these instructions is not found verbatim, do not guess a nearby location:
  stop that task, record the miss in the summary, continue with the rest.
- Do not modify Part 1, any Rust code, any test, or any file under `alpha/experiments/*/src`.
- Minimal diffs: no reflowing of untouched paragraphs.

## Output

Write `alpha/experiments/RUN-02-SUMMARY.md`: per-task status (done / skipped+why / anchor-miss),
every placement judgment made (T3b, T5), and the full list of files changed.
