# X3 — the automated cross-package mutation sweep (RUN-07)

Serves: Part 2 §7.2 R7 (the content-bound-quorum *count* is cross-package mutation-clean) — earns/bounds: `Verified` (count only; the role-authorship gate stays R7's open residual) — register: `fold-auth-duplicate` (Reconciled) + the mutation-gate note — landed: RUN-07.

`RUN-07, 2026-07-15. cargo-mutants 27.1.0, Rust 1.94.1. Closes the X3 residual left open by
X3-CROSS-PACKAGE-SWEEP.md: the *automated* harness that mutates the substrate while running the
croft-chat consumer suite, resolving every in-substrate survivor mechanically rather than by
category argument plus one hand-kill.`

## The bar

From `X3-CROSS-PACKAGE-SWEEP.md`: an automated configuration in which mutants in
`local_storage_projection` are exercised by the croft-chat integration suite, **killing the 61
in-substrate survivors or justifying each individually**. This run reaches that bar: **7 killed, 54
individually justified, 0 unjustified survivors.**

## Tool configuration (determined empirically)

`cargo mutants --help` (27.1.0) exposes `-p/--package` (which package to *mutate*),
`--test-package`/`--test-workspace` (which tests to *run*), and `--in-place`. The clean single-
invocation shape — mutate `local_storage_projection`, test `croft-chat` — is **not reachable**,
because the two crates are separate workspaces: cargo rejects adding `local_storage_projection` to
the croft-chat workspace (`workspace member ... is not hierarchically below the workspace root`),
and cargo-mutants' copy step cannot carry a path dependency that lives outside the workspace root.

So the repeatable shape is the **thin harness the X3 report anticipated** (`x3_cross_package_harness.py`,
committed alongside this report): for each in-substrate survivor it applies the exact cargo-mutants
diff to the real substrate source, runs the croft-chat suite (which links `local_storage_projection`
as a path dependency and drives it through `surface::LocalStore` → `fold_derived::DerivedFold`),
records killed (a consumer test fails) vs surviving, and reverts. No production code and no manifest
is changed; the harness patches and reverts in place.

### Repeatable command

```sh
cd alpha/experiments/local_storage_projection

# 1. Current substrate survivor set (the 61), same --re function scope as the original X3 sweep:
PROPTEST_CASES=8 cargo mutants -j2 --timeout 120 \
  --file src/fold_auth.rs --file src/governance.rs --file src/fold_derived.rs \
  --re 'required_threshold_for_rule_change|count_personae_by_lineage|threshold_met|is_under_determined|check_authorization|role_ge_admin|role_ge_owner|role_ge_member|fn author_role|rule_change_approval_subject|act_subject|decode_rule_key|tiebreak|detect_fork' \
  -o /tmp/substrate.out
cp /tmp/substrate.out/mutants.out/missed.txt x3-sweep-data/missed-run07.txt   # 61 survivors

# 2. The automated cross-package sweep over those survivors (the residual X3 harness):
python3 x3_cross_package_harness.py x3-sweep-data/missed-run07.txt x3-sweep-data/cross-package-run07.json
```

Raw artifacts: `x3-sweep-data/missed-run07.txt` (the 61), `x3-sweep-data/cross-package-run07.json`
(per-mutant disposition + timings). Substrate sweep: **53 caught, 61 missed, 5 unviable** (119
viable). Cross-package harness wall time: **~57 min** (61 × ~56 s per rebuild-and-test cycle,
`-j1` in place).

## The load-bearing finding — the two `check_authorization` copies are NOT interchangeable

The original X3 report read the 61 survivors as a single population "all in the cross-package-covered
decision path." The automated sweep refutes that framing precisely:

**`fold_auth.rs` is a parallel copy that the consumer suite never reaches.** `surface::LocalStore`
(the public API croft-chat drives) constructs `fold_derived::DerivedFold`; `fold_auth::AuthFold` is
instantiated *only* inside `fold_auth.rs`'s own `#[cfg(test)]` module. `fold_derived.rs` even labels
its own authorization helpers "duplicated from fold_auth for independence." So **every one of the 31
`fold_auth.rs` survivors survives the croft-chat suite too** — not because the trust logic is weak,
but because that copy is off the consumer path. The live authorization path is `fold_derived.rs`, and
that is where the cross-package suite does its work.

## Result — per-population

| Population | count | cross-package disposition |
|---|---:|---|
| **`fold_auth.rs`** (off-consumer-path duplicate) | 31 | **survive, justified** — never linked by `surface`/croft-chat; `AuthFold` runs only in its own unit tests |
| **`fold_derived.rs` — R7 count-enforcement** | 7 | **KILLED cross-package** |
| **`fold_derived.rs` — role-authorship gate** | 7 | survive, justified — R7 explicitly excludes the role model |
| **`fold_derived.rs` — Vouch payload validation** | 10 | survive, justified — no consumer Vouch test (uncovered residual) |
| **`fold_derived.rs` — node-card provenance** | 3 | survive, justified — social-graph plumbing, not the auth/threshold claim |
| **`fold_derived.rs` — boundary / adjacent-rule** | 3 | survive, justified — equivalent or unreachable on real payloads |
| **`governance.rs`** — threshold counting/quorum arithmetic | 0 | mutation-clean in-substrate (no survivors) |

### The 7 killed (the R7 content-bound-quorum enforcement path)

| Mutant | Killed by |
|---|---|
| `fold_derived.rs:517` `rule_change_approval_subject → [0;32]` | `rulechange_threshold_enforced::approval_for_a_different_change_does_not_count` (a constant subject makes an approval for change→5 satisfy change→9's quorum) — **verified by hand-apply**: `assertion left == right failed: amendment must be REJECTED` |
| `fold_derived.rs:517` `rule_change_approval_subject → [1;32]` | same test (any constant subject collapses the approval binding) |
| `fold_derived.rs:526` delete `act_subject` membership/role arm | the approval-subject resolution for membership/role acts → `None`, breaking quorum matching (`threshold_enforced` / `rulechange_threshold_enforced`) |
| `fold_derived.rs:343` delete `decode_rule_key` arm 1 (RemoveMember) | a RuleChange on `rule_key 1` is rejected — exercised by `completeness` / `competing_quorums` disjoint-rulekey tests |
| `fold_derived.rs:362` `role_ge_admin → true` (helper) | a non-admin's membership act is admitted where a test requires rejection |
| `fold_derived.rs:382` `role_ge_admin` guard (MembershipAdd) `→ true` | same, on the MembershipAdd branch |
| `fold_derived.rs:441` `role_ge_admin` guard (Approval) `→ false` | a valid Approval is rejected → the quorum never assembles → threshold tests fail |

These are exactly the **content-bound quorum count**: the approval-subject binding (517), the
approval-subject resolution (526), the rule-key decode (343), and the membership-admin gate that
carries the count. The croft-chat suite kills all of them; the substrate suite could not (its own
tests do not exercise the positive path). **This is the X3 result R7 was waiting on.**

### The 54 justified survivors (per-mutant, all recorded)

**`fold_auth.rs` (31) — off-consumer-path duplicate.** `237,241` role helpers; `260,271,282,319,332,384`
`check_authorization` role guards; `298,347,355,361` comparison boundaries; `360,367` offset
arithmetic; `370` match-arm delete; `395,396` `decode_rule_key` arms. Every one is a mutation of a
copy the consumer never links. Justification is structural and grep-provable, not statistical.

**`fold_derived.rs` role-authorship gate (7).** `366` `role_ge_owner→true`, `370` `role_ge_member→true`;
guards `390` (MembershipRemove), `399` (RoleGrant/RoleRevoke), `420` (RuleChange), `431` (Message/
content), `441→true` (Approval). R7's own text (§7.2, the "two residuals" paragraph) states the
role-authorship gate is "a spike simplification ... which remains open — R7 does not retire it, and
'enforced' here means the count, not the role model." So a surviving role-gate mutant is **outside
R7's claim**, not a hole in it. The sweep *quantifies* that documented residual: the entire role
gate is unpinned by the consumer suite.

**`fold_derived.rs` Vouch payload validation (10).** `449` (×2, min-length), `461` (×4), `462`, `469`
(×2), `470` — the I5 Vouch payload-length/strength checks. No croft-chat test authors a `Vouch`
act, and the substrate suite does not exercise the payload boundaries either. Unrelated to R7;
recorded as a genuine **uncovered residual** (a future Vouch-payload proptest, not a threshold hole).

**`fold_derived.rs` node-card provenance (3).** `2178` (`created_by` in `upsert_node_stub`), `2225`
(×2, `created_at`/`created_by` in `upsert_node_full`). Node-card provenance fields for the
social-graph projection, converged by a canonical MIN rule — not the authority/threshold trust
claim. A node-card convergence test would pin them; out of X3's auth scope.

**`fold_derived.rs` boundary / adjacent-rule (3).** `344` delete `decode_rule_key` arm 2 (RoleChange
rule_key) — no consumer test performs a RuleChange on `rule_key 2`; adjacent to, but not, the
RuleChange-threshold marquee. `408` RuleChange payload-length `< 5 → > 5` — equivalent for the
exactly-5-byte RuleChange payload (`rule_key`(1) ‖ `new_value`(4)), neither bound trips. `530`
`act_subject` `< 32 → <= 32` — unreachable: membership payloads are 33 bytes (`principal`(32) ‖
`role`(1)), so the 32-byte boundary is never hit.

## Verdict

**PASS — the X3 automated cross-package harness bar is met.** Every one of the 61 in-substrate
survivors is mechanically resolved: **7 killed** by the croft-chat suite (the R7 content-bound-quorum
enforcement path), **54 individually justified** (31 off-path duplicate, 7 role-gate that R7
explicitly excludes, 10 uncovered Vouch payload checks, 3 node-card provenance, 3
boundary/adjacent-rule). Zero unjustified survivors.

**R7's count-enforcement is cross-package mutation-clean.** The load-bearing mutants — the approval
subject (`rule_change_approval_subject`), the approval-subject resolution (`act_subject`), the
rule-key decode, and the membership-admin gate that carries the count, on top of the already
mutation-clean `governance.rs` threshold counting — all die to the consumer suite. This earns the
`Verified` upgrade for R7's *count* claim (applied this run; see the changelog and register).

**Residuals recorded (not blockers for the R7 count claim):**
- The **role-authorship gate** in `fold_derived::check_authorization` is entirely unpinned by the
  consumer suite (7 survivors). This is the same residual R7 already flags as open ("the role model,
  not the count"); the sweep quantifies it. Closing it is the per-act approver-role work (backlog §2a).
- **Vouch payload validation** (10 survivors) is uncovered by both suites — a future Vouch-payload
  proptest, unrelated to the R7 trust claim.
- **`fold_auth.rs`** was a dead parallel copy of the authorization logic, not on any consumer path.
  **Retired in the RUN-07 follow-up (owner-authorized):** `fold_auth.rs` deleted and `pub mod fold_auth;`
  removed, so `fold_derived` is now the single authorization path — the one the consumer suite pins.
  Both crates + clippy green after removal; register row `fold-auth-duplicate` moved to Reconciled.

**Stop rule (Phase B.6):** not triggered. The harness required no production-code change — only a
test-driver script and in-place patch/revert.

---

## Addendum — Vouch payload validation now covered cross-package (RUN-09, 2026-07-15)

`RUN-09 Part 2. cargo-mutants 27.1.0, Rust 1.94.1. Closes backlog §2d — the Vouch
payload-validation residual this report recorded as "uncovered by both suites" (10 justified
survivors, the "10 uncovered Vouch payload checks" line of the verdict above). The original
verdict text is unchanged; this addendum records the follow-on coverage.`

**What changed.** A consumer-path test — `croft-chat/croft-chat/tests/vouch_payload.rs` (9 tests) —
now exercises the fold's I5 Vouch payload gate (`fold_derived.rs:447-477`) end-to-end: a hand-crafted
`Vouch` envelope is driven through the real `DerivedFold` (the path `surface::LocalStore` builds),
well-formed accepted / malformed / empty-context / truncated / bad-strength rejected, with the
accept/reject decision read back through **folded state** — the derived VOUCHES edge surfaced by
`LocalStore::get_trust_signals` — plus the fold's public accept/reject outcome. No production code
changed; this is coverage only.

**The re-run.** The same cross-package harness (`x3_cross_package_harness.py`, restricted to
`--file src/fold_derived.rs` since `fold_auth.rs` was retired in the RUN-07 follow-up) applied each
Vouch-region mutant to the real substrate and ran the croft-chat consumer suite. The scope is the
whole Vouch payload region (lines 449/461/462/469/470) — the **10 previously-justified survivors**
of the RUN-07 sweep plus the **9 additional** operator-substitution mutants the current tool lists
there.

```sh
cd alpha/experiments/local_storage_projection
# the 10 previously-justified survivors:
sed -n '45,54p' x3-sweep-data/missed-run07.txt > x3-sweep-data/vouch-survivors-run09.txt
# the full current Vouch-region set (10 + 9):
cargo mutants --list --file src/fold_derived.rs | grep -E ':(449|461|462|469|470):' \
  > x3-sweep-data/vouch-region-all-run09.txt
python3 x3_vouch_harness.py x3-sweep-data/vouch-region-all-run09.txt \
  x3-sweep-data/vouch-cross-package-run09.json   # fold_derived.rs-only SCOPE_FILES
```

**Result — 19 killed, 0 survived** (wall ~1244 s, 19 × ~65 s per rebuild-and-test cycle). Every
Vouch-region mutant now dies to the croft-chat suite, including all 10 RUN-07 justified survivors:

| Mutant (`fold_derived.rs:` line:col) | RUN-07 | RUN-09 cross-package |
|---|---|---|
| `449:34 < → ==` , `< → <=` | justified survivor | **KILLED** |
| `449:34 < → >` | (caught in substrate) | **KILLED** |
| `461:31/35/45 + → -` | justified survivor (`-`) | **KILLED** |
| `461:31/35/45 + → *` | (caught / new) | **KILLED** |
| `462:34 < → >` | justified survivor | **KILLED** |
| `462:34 < → ==` , `< → <=` | (new) | **KILLED** |
| `469:48/52 + → -` | justified survivor | **KILLED** |
| `469:48/52 + → *` | (new) | **KILLED** |
| `470:30 > → <` | justified survivor | **KILLED** |
| `470:30 > → ==` , `> → >=` | (new) | **KILLED** |

How the kills land, by check: the **min-length** mutants (449) flip the rejection *reason* at the
37-byte boundary — the empty-context test pins "reject as AuthorizationFailed, not too-short". The
**`required`-arithmetic** mutants (461) shrink `required` below the actual length, so the truncation
check passes and the strength read runs off the end — the one-byte-short test's clean-gate rejection
is the divergence. The **truncation-comparison** mutant (462 `< → >`) rejects a valid-with-trailing
payload — the oversized-trailing test pins that trailing bytes are accepted. The **strength-index**
mutants (469) misread the strength byte — the offset tests place a `>2` byte (or an underflowing
index) where the mis-index lands, so a valid vouch is wrongly rejected (or panics). The
**strength-bound** mutant (470 `> → <`) rejects the valid low strengths — the well-formed
(strength 1) test's acceptance is the divergence.

Raw artifacts: `x3-sweep-data/vouch-survivors-run09.txt` (the 10), `x3-sweep-data/vouch-region-all-run09.txt`
(the 19), `x3-sweep-data/vouch-cross-package-run09.json` (per-mutant disposition + timings).

**Scope note (unchanged claims).** This closes a **coverage residual**, not a spec claim: no status
tag moves. R7's count-enforcement verdict above stands as written; the role-authorship gate residual
is untouched. Backlog §2d is retired (landing run RUN-09).
