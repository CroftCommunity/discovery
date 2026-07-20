# F1 amended — R7 with the three riders folded in

Drop-in replacement for the `§7.2, add one realization bullet after R6` diff block in
`beta/drystone-spec/proposed-changes-2026-07-experiment-reconciliation.md`. The §8.2(e) diff in the
staged doc stands unchanged. Grounding: fold code (`fold_derived.rs` Step 5.6, `threshold_for`,
`rule_change_approval_subject`, `act_subject`, `gather_approvers`), Part 2 §4.6 / §5.7 / §7.3.1.

```diff
  - **R6, Attributable acceptance.** A participant accepting a write under a capability **MUST**
    record the causal frontier of governance facts it had synced at acceptance …

+ - **R7, Content-bound quorum for policy changes.** A change to replicated policy (a threshold, a
+   rule) **MUST** be admitted only when k distinct personae — counted by lineage (§4.5), never by
+   client or device — have each authored an approval fact that references the **content hash of the
+   exact change** (its "approval subject"), so an approval cannot be replayed onto a different
+   change and the quorum is *enforced at fold time*, not merely recorded. Three semantics are
+   normative, not incidental:
+
+   1. **The prior rule governs.** The threshold consulted is the one in force at the act's position
+      in the causal order — the rules *before* the change applies — never the rules the change would
+      install; otherwise a proposer could lower the gate with the very act being gated (propose
+      quorum-of-one, approve it alone under the proposed rule). A change to the rule-change
+      threshold itself is the marquee case and is gated identically.
+
+   2. **Under-quorum is pending, never partial.** Approvals are ordinary governance facts authored
+      *before* the enacting act and referenced by it as antecedents (the §5.7 co-signed-op shape). A
+      proposal below threshold is simply a proposal whose enacting act does not yet exist; an
+      enacting act that arrives without sufficient matching antecedent approvals is **rejected
+      deterministically** by every honest node — it never applies partially and never waits in fold
+      state. Enactment is always a new act referencing the accumulated approvals.
+
+   3. **The subject preimage is the canonical payload encoding.** The approval subject is the digest
+      of the change's canonical payload bytes (§4.6), **not** of the enclosing fact envelope — it
+      must be computable before the enacting act exists, and an envelope digest would be circular
+      against the antecedent references in (2). This introduces no canonicalization surface beyond
+      the one §4.6 already makes load-bearing; the subject's byte-level encoding is pinned with the
+      others (`[gates-release]`, Appendix B).
+
+   This is R3 applied to policy changes: the approvals are governance facts that fold
+   deterministically to the same admit/reject on every honest node. `Modeled` (RED→GREEN:
+   `rulechange_threshold_enforced.rs`; end-to-end via the session API:
+   `rulechange_quorum_via_api.rs`; manual mutation gate passed. Formal cross-package
+   `cargo-mutants` sweep pending — X3).
```

## Residuals to carry with R7 (one paragraph, same landing spot)

```diff
+ Two residuals bound R7's current evidence. First, the reference fold retains a hard-coded
+ **role-authorship gate** alongside the quorum (a RuleChange author must hold Owner; an Approval
+ author must hold Owner or Admin): a spike simplification standing in for per-act approver-role
+ granularity (§5.5 dial), which remains open — R7 does not retire it, and "enforced" here means the
+ count, not the role model. Second, **two concurrent RuleChanges to the same rule that both meet
+ quorum** are untested: whether they resolve as same-type concurrents by the §7.3.1 content-address
+ tiebreak (the mutual-expulsion analogy, §7.3.2) or cross the §7.6 hard-stop boundary is exactly the
+ "two-competing-quorums" backlog item, and the fold's behavior there carries no evidence tag until
+ that experiment runs.
```

## The two calls the staged doc asks (recommendations)

1. **Home: §7.2 as R7**, with a one-line cross-reference from §7.3's fold-step text ("threshold
   enforcement at position: see R7"). Rationale: R7 is an interface guarantee of the
   grant-and-revocation surface, and the R-series is where a reader audits what the interface
   promises; §7.3 is how the fold delivers it.

2. **Status: `Modeled`, not `Verified`**, matching the ladder: reference-implementation evidence
   plus a manual mutation gate is exactly the `Modeled` bar, and the formal cross-package
   `cargo-mutants` sweep (X3) is the named event that would justify the upgrade.

## Session-emit residual (unchanged from the alignment doc, restated for the log entry)

`MembershipRemove`, `RoleGrant`, `RoleRevoke` still have no `Session` surface command (fold logic
exists); any near-term demo of removals reverts to hand-crafted facts. Client backlog, not spec.
