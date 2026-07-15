# RUN-07 — the code run: X3 automated harness, EXP-H1, EXP-C1 (+ two doc riders)

`Branch: claude/run-07-code-run-35zmwx, off main (RUN-06 merged). 2026-07-14 / 2026-07-15.
cargo-mutants 27.1.0, Rust 1.94.1. One phase per commit.`

## Per-phase status

| Phase | What | Status |
|---|---|---|
| **A** | Riders + settlement recording (markdown) | ✅ done (commit `c9a7ea4`) |
| **B** | X3 automated cross-package mutation harness | ✅ done, bar met (commit `61300cc`) |
| **C** | EXP-H1 horizon-manifest determinism | ✅ done, green (commit `e721489`) |
| **D** | EXP-C1 completeness-ahead contract (4 assertions) | ✅ done, all four green (commit `1f7b572`) |

Both suites (`local_storage_projection` 97 tests; `croft-chat` 25 test binaries) + clippy green at every
commit boundary; no new warnings vs baseline.

---

## Phase A — riders and settlement recording

- **A1 (T11).** Part 2 §7.3.2 F8 paragraph's closing sentence: `` `Design`, decided and now test-run: ``
  → `Decided by design and now test-run:`, so it carries exactly one status tag (the terminal `` `Modeled.` ``).
- **A2 (FND-1).** Part 2 §8.2(e) originating-op freshness-precondition cite widened `(§7.4.2)` →
  `(§7.4–§7.4.2)`; same one-site widening in `EXPERIMENT-BACKLOG.md`. The two §7.4.2-hazards table cites
  left untouched.
- **A3.** `CONSISTENCY-FINDINGS-2026-07.md` settlement addendum: FND-1 refined to the range cite;
  FND-5/FND-6 dispositions ratified post hoc (narrow frozen-record exception); FND-8 recorded as mistaken.
- **A4.** `part-2-changelog.md` entry for A1/A2.

---

## Phase B — X3 automated cross-package mutation harness

**Verdict: PASS — the bar is met.** The current-code substrate sweep found 61 survivors; the automated
cross-package harness resolves all 61 mechanically: **7 killed, 54 individually justified, 0 unjustified.**
Full detail and per-mutant table in `local_storage_projection/X3-AUTOMATED-SWEEP.md`.

**Tool shape (determined empirically).** The clean single-invocation config (mutate
`local_storage_projection`, test `croft-chat`) is unreachable — the two crates are separate workspaces,
cargo rejects an out-of-tree member, and cargo-mutants cannot copy a path dep outside the workspace root.
So the repeatable shape is the thin harness the X3 report anticipated: patch the substrate with the exact
cargo-mutants diff, run the consumer suite, revert. No production code or manifest changed.

### Repeatable command

```sh
cd alpha/experiments/local_storage_projection
# 1. current substrate survivor set (61), same --re function scope as the original X3 sweep:
PROPTEST_CASES=8 cargo mutants -j2 --timeout 120 \
  --file src/fold_auth.rs --file src/governance.rs --file src/fold_derived.rs \
  --re 'required_threshold_for_rule_change|count_personae_by_lineage|threshold_met|is_under_determined|check_authorization|role_ge_admin|role_ge_owner|role_ge_member|fn author_role|rule_change_approval_subject|act_subject|decode_rule_key|tiebreak|detect_fork' \
  -o /tmp/substrate.out
cp /tmp/substrate.out/mutants.out/missed.txt x3-sweep-data/missed-run07.txt
# 2. automated cross-package sweep over those survivors:
python3 x3_cross_package_harness.py x3-sweep-data/missed-run07.txt x3-sweep-data/cross-package-run07.json
```

Substrate sweep: 53 caught, 61 missed, 5 unviable. Cross-package harness: ~57 min (61 × ~56 s/cycle, in place).

### The 7 killed (R7 content-bound-quorum count path)

`fold_derived.rs`: `rule_change_approval_subject → [0;32]` and `→ [1;32]` (verified by hand-apply:
`approval_for_a_different_change_does_not_count` FAILED — "amendment must be REJECTED"); `act_subject`
membership/role arm deletion; `decode_rule_key` arm 1; `role_ge_admin → true` (helper); `role_ge_admin`
MembershipAdd guard `→ true`; `role_ge_admin` Approval guard `→ false`.

### The 54 justified survivors (all recorded, per-mutant, in X3-AUTOMATED-SWEEP.md)

- **31 in `fold_auth.rs` — off-consumer-path duplicate.** The load-bearing finding: `fold_auth::AuthFold`
  is instantiated only in its own `#[cfg(test)]`; the live path is `surface::LocalStore` →
  `fold_derived::DerivedFold`. Every fold_auth survivor survives the consumer suite because the copy is
  never linked. This refutes the original X3 reading ("all survivors cross-package-covered"). New register
  row `fold-auth-duplicate` (Active, flagged not fixed — retiring the copy is a production change).
- **7 role-authorship gate** (`role_ge_owner/member`, various `check_authorization` guards) — R7's own text
  excludes the role model ("'enforced' here means the count, not the role model"). Outside the R7 claim; the
  sweep quantifies that documented-open residual.
- **10 Vouch payload validation** — no consumer Vouch test; unrelated to R7; a genuine uncovered residual.
- **3 node-card provenance** (`created_by`/`created_at`) — social-graph plumbing, not the auth/threshold claim.
- **3 boundary/adjacent-rule** — `decode_rule_key` arm 2 (role_change rule_key, untested); RuleChange
  payload-len boundary (equivalent for 5-byte payloads); `act_subject` len boundary (unreachable, payloads
  are 33 B).

**Stop rule (B.6): not triggered** — the harness required no production-code change.

---

## Phase C — EXP-H1 horizon-manifest determinism

`local_storage_projection::horizon` (pure fold-side, experiment-grade — no wire format, no persistence, no
networking; the `[gates-release]` §7.6.9 encoding untouched) + `DerivedFold::read_group_state` +
`croft-chat/tests/horizon_manifest.rs` (4 tests, green).

`horizon_manifest(state) -> (frontier_head, sorted open-contradiction byte-heads)`. `frontier_head` is an
order-independent digest of the folded state's converging content (gov_seq + rules + sorted members); the
raw `computed_at_gov_head` is the last-INGESTED hash and is arrival-order dependent, so it is excluded.

### Test names and both-order outputs

- `horizon_manifest_identical_across_orders_mutual_expulsion` — order1 and order2:
  `open=[d4196bb5f96678d1…c80398ff]` (byte-identical), full manifest byte-identical.
- `horizon_manifest_identical_across_orders_competing_rulechange` — order1 and order2:
  `open=[62584e17c767bcd7…6798fd71]` (byte-identical), full manifest byte-identical.
- `horizon_trigger_both_modes_fire_at_same_position` — fact-count boundaries at `[N, 2N, 3N]` on both
  members; epoch roll fires immediately and resets the counter.
- `horizon_manifest_negatives` — a clean group has empty open-contradictions; an open contradiction persists
  across horizon boundaries (decay is presentation, not truth).

Byte-identity compared via a test-only serialization explicitly commented as NOT the `[gates-release]`
encoding. Earns the *manifest determinism* claim only; §7.6.9 stays `Design`.

---

## Phase D — EXP-C1 completeness-ahead contract

`local_storage_projection::completeness_ahead` (pure helpers `quorum_k`, `detect_stamp_gap`,
`admits_irreversible`; 3 inline unit tests) + `croft-chat/tests/completeness_ahead.rs` (4 tests, green).
No assertion needed MLS internals or network transport, so none was split out; all four landed at loopback
/ fold grade. Freshness / stamp values are integers the test seeds; the `[gates-release]` stamp and `(G,D)`
cursor encodings are untouched.

### Test names

- `stall_at_threshold_no_breach_reads_unaffected` — a node denied a governance fact stalls the dependent
  irreversible act below `k = ceil(n/2)` (fail-closed, no breach) while reads on best-known prefix state
  continue (n=3, k=2, seeded freshness 1 < 2 → stall; a current node at k acts).
- `stamp_ahead_detected_sized_and_filled_before_acting` — `detect_stamp_gap(4, 7) = Some(3)`; after fill,
  `detect_stamp_gap(7, 7) = None`; X acts only once the gap is filled.
- `solicitation_surfaces_unreferenced_tail_folds_identically` — an unreferenced-tail MembershipAdd absent on
  X is surfaced by a frontier ask (re-delivered into X's persistent live pipeline) and folds to the
  byte-identical fingerprint of a node that received it live.
- `formula_valued_k_identical_across_orders` — folded member count n1 = n2 = 3 across both arrival orders;
  `quorum_k(3) = 2` identical at the same act position.

---

## Conditional spec edits — applied vs withheld

| Edit | Condition | Decision | Why |
|---|---|---|---|
| **B5: §7.2 R7 `Modeled` → `Verified`** | every survivor killed or justified | **APPLIED** (scoped to the count) | All 61 resolved (7 killed, 54 justified). The 7 killed are exactly R7's content-bound-quorum count path; on top of mutation-clean `governance.rs` counting, the count claim is cross-package mutation-clean. Tag scoped to the *count*; the role-authorship gate stays the open residual R7 already names. |
| **B5: §8.2(e) enforcement note** | same | **APPLIED** | Records the enforcement as cross-package mutation-`Verified` for the count. |
| **B5: register + MASTER-INDEX + changelog** | same | **APPLIED** | Mutation-gate note updated; `fold-auth-duplicate` row added; A2 marked done; changelog entry. |
| **D-close: §8.2(e) loopback note** | all four EXP-C1 assertions pass | **APPLIED** | "exercised at loopback grade (EXP-C1, RUN-07) ... real-NAT path remains X1". |

Nothing was withheld: both conditional gates were met on the stated evidence. The `Verified` tag is
deliberately narrowed to the *count* (not the role model), and every non-killed survivor is recorded with
its individual justification rather than swept under the tag.

---

## Files changed (16 files, +1672 / -14)

**New:** `local_storage_projection/X3-AUTOMATED-SWEEP.md`, `x3_cross_package_harness.py`,
`x3-sweep-data/{missed-run07.txt, cross-package-run07.json}`, `src/horizon.rs`, `src/completeness_ahead.rs`;
`croft-chat/tests/{horizon_manifest.rs, completeness_ahead.rs}`; this summary.

**Modified:** `local_storage_projection/src/{lib.rs, fold_derived.rs}` (module registration +
`read_group_state`); `beta/drystone-spec/part-2-certifiable-design.md` (§7.3.2 T11, §8.2(e) FND-1 + R7 +
EXP-C1, §7.2 R7 `Verified`); `part-2-changelog.md` (three RUN-07 entries);
`EXPERIMENT-BACKLOG.md` (FND-1 cite, §2b + §2c done); `SPEC-DIVERGENCE-REGISTER.md` (mutation-gate note +
`fold-auth-duplicate` row); `MASTER-INDEX.md` (A2 done, EXP-H1/EXP-C1 done);
`CONSISTENCY-FINDINGS-2026-07.md` (settlement addendum).
