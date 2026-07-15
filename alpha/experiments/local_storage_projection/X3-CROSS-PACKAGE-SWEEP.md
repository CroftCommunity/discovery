# X3 — cross-package mutation sweep on the auth/governance path (EXP-3)

`RUN-01 EXP-3. Branch claude/experiments-run-01, 2026-07-14. cargo-mutants 27.1.0, Rust 1.94.1.`

> **Partially refuted (RUN-07 · rider RUN-08).** This report's central conclusion — that the 61 survivors are cross-package-covered instrument artifacts — was partially refuted by `X3-AUTOMATED-SWEEP.md` (RUN-07): 31 of the 61 were dead-duplicate `fold_auth` survivors, never linked into the consumer path, so the automated sweep could not kill them by that route. See the `fold-auth-duplicate` register row (`SPEC-DIVERGENCE-REGISTER.md`). Record preserved verbatim below.

## What this tests

§7.3 / §8.2(g) — the authority/threshold trust claim: a surviving mutant in `fold_auth` /
`governance` (a silently-weakened threshold or role check that no test notices) is a real hole in the
trust claim. Also raises **F1**'s RuleChange-enforcement status from `Modeled` toward
mutation-`Verified`. The known hard part (register, `MUTATION_TESTING.md`): the **positive-path
coverage lives cross-package in `croft-chat`**, so a substrate-only sweep *under-counts* — many
authorization-decision mutants survive in-package because the tests that pin them are in a different
crate.

## Method

`cargo mutants` scoped to the authority/threshold functions across three files
(`--re` on function names, `PROPTEST_CASES=8`, the repo's `.cargo/mutants.toml` skips of the heavy
`tests_stage7` property tests, `-j 2`, `--timeout 120`):

- `governance.rs`: `required_threshold_for_rule_change`, `count_personae_by_lineage`, `threshold_met`,
  `is_under_determined`, `tiebreak`, `detect_fork`, `is_ancestor`/`are_concurrent`.
- `fold_auth.rs`: `check_authorization`, `role_ge_admin/owner/member`, `author_role`, `decode_rule_key`.
- `fold_derived.rs`: `check_authorization` (its second copy), `act_subject`,
  `rule_change_approval_subject`, `decode_rule_key`.

Raw artifacts in `x3-sweep-data/` (`missed.txt`, `caught.txt`, `unviable.txt`, `outcomes.json`). The
multi-MB per-mutant logs (`mutants.out/`) are not committed (per `MUTATION_TESTING.md`).

## Result — substrate-only sweep

**120 mutants: 54 caught, 61 missed, 5 unviable** (18 min). Substrate-only score = 54 / 115 viable =
**47%**. But the two populations must be read separately:

| File / population | caught | missed | reading |
|---|---|---|---|
| **`governance.rs`** — threshold *counting* & quorum arithmetic | 13 | **0** | **mutation-clean in-substrate** — `threshold_met` (`>=→<`, `→true/false`), `count_personae_by_lineage` (`→0/1`), `required_threshold_for_rule_change` (`→0/1`) **all caught**. Zero survivors. |
| `fold_auth.rs` — authorization *decision* (`check_authorization` + `role_ge_*`) | 13 | 31 | survives in-substrate — positive-path coverage is cross-package |
| `fold_derived.rs` — auth decision + `act_subject` / `rule_change_approval_subject` | 28 | 30 | same; incl. the F1 approval-subject function |

**The survivors are not spread across the trust path — they cluster entirely in the authorization
_decision_ (the `role_ge_*` guards in the two `check_authorization` copies) and the approval-subject
helpers (`act_subject`, `rule_change_approval_subject`). None are in threshold _counting_.**

## Cross-package reconciliation (the reason the survivors survive)

A substrate-only sweep runs only `local_storage_projection`'s **own** test suite. The authorization
_decision_ path is exercised by behavior — a non-owner's act being rejected, an approval binding to its
exact change — and that behavior is asserted **cross-package in `croft-chat`** (`mutual_expulsion.rs`,
`removed_then_included.rs`, `role_thrash.rs`, `rulechange_threshold_enforced.rs`,
`rulechange_quorum_via_api.rs`, `convergence.rs`, `contradiction.rs`). So those mutants survive the
substrate suite and would be killed by the consumer suite. This is the register's stated X3 reason,
now **quantified** (61 survivors, all in the cross-package-covered decision path).

### Demonstration — a substrate-survivor is killed cross-package (F1 approval-subject)

Highest-value survivor for **F1**: `rule_change_approval_subject -> [u8;32]` mutated to a **constant**
(`[0;32]`) — MISSED in-substrate. Hand-applied it and ran the `croft-chat` cross-package test
`rulechange_threshold_enforced::approval_for_a_different_change_does_not_count`:

```
test approval_for_a_different_change_does_not_count ... FAILED
assertion `left == right` failed: amendment must be REJECTED — A2's approval named a
different change (→5), not this one (→9)
```

**Killed.** A constant approval-subject makes every change hash to the same subject, so an approval for
change→5 would wrongly satisfy change→9's quorum; the cross-package test catches exactly that. Mutation
reverted; the substrate suite is green again (`rulechange_threshold_enforced` 4/4). This is the
`act_subject→None` / `rule_change_approval_subject→const` manual gate the register already claimed for
`rulechange-quorum` — now shown by hand-kill against the real cross-package test.

## Verdict

**PASS on the trust question; PARTIAL on X3 mechanization.**

- **FALSIFY not met.** The FALSIFY target was "surviving mutants in threshold-counting or
  approval-subject logic." Threshold-counting: **0 survivors** in-substrate. Approval-subject: survives
  in-substrate but is **demonstrably cross-package-killed** (above). **No real hole** in the
  authority/threshold trust path was found — the survivors are an *instrument artifact* (substrate-only
  cannot see the consumer's tests), not weakened trust logic.
- **F1:** the RuleChange approval-subject path is mutation-clean cross-package (demonstrated for the
  load-bearing survivor). This supports F1 staying `Modeled` and edging toward `Verified` — but the
  *formal, automated* cross-package sweep is the bar for `Verified`, and that is the residual below.
- **Residual X3 (still open):** the *automated* cross-package harness — a sweep that mutates the
  substrate files while running `croft-chat`'s suite so all 61 survivors resolve mechanically, rather
  than by category argument + one hand-kill. `local_storage_projection` and `croft-chat` are separate
  crates (separate `Cargo.lock`), so this needs a workspace/`--test-package` configuration and budgets
  the slower consumer suite; not built in this run (time-box). This run **quantified and demonstrated**
  the under-count, which is the input that harness needs.

## Reproduce

```sh
cd alpha/experiments/local_storage_projection
PROPTEST_CASES=8 cargo mutants -j 2 --timeout 120 \
  --file src/fold_auth.rs --file src/governance.rs --file src/fold_derived.rs \
  --re 'required_threshold_for_rule_change|count_personae_by_lineage|threshold_met|is_under_determined|check_authorization|role_ge_admin|role_ge_owner|role_ge_member|fn author_role|rule_change_approval_subject|act_subject|decode_rule_key|tiebreak|detect_fork'
```
