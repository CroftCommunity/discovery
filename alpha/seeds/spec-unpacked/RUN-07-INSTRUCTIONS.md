# RUN-07 — The code run: X3 automated harness, EXP-H1, EXP-C1 (+ two doc riders)

`Branch: fresh off main (RUN-06 merged), e.g. run-07-code-run. Phase A is markdown; Phases B–D are
code, one phase per commit (or commit series). Long-running phase: B (mutation sweep) — budget
accordingly and use the file scoping below.`

## Phase A — riders and settlement recording (markdown, first commit)

A1. **T11 double-tag polish.** Part 2 §7.3.2, the F8 paragraph's closing sentence: change
    "`Design`, decided and now test-run:" to "Decided by design and now test-run:" so the sentence
    carries exactly one status tag (the terminal `Modeled.`).

A2. **FND-1 range cite (the ruled cheap middle).** In Part 2 §8.2(e), change "the freshness
    precondition on originating such an op (§7.4.2)" to "(§7.4–§7.4.2)". Same change at the one
    EXPERIMENT-BACKLOG.md site quoting that clause. The two §7.4.2 cites in the §7.4.2-hazards
    table rows are correct on their own terms — do not touch.

A3. **Settlement addendum.** In CONSISTENCY-FINDINGS-2026-07.md's Settlement section, append:
    FND-1 ruling refined 2026-07-14 to the range cite (§7.4–§7.4.2) in living docs; FND-5/FND-6
    dispositions ratified post hoc by the owner (mechanical normalization to current conventions is
    accepted on preserved/provenance blocks as a narrow exception to the frozen-record rule);
    FND-8's original finding recorded as mistaken (site was already clean).

A4. Changelog entry for A1/A2; reviews-log gets one RUN-07 entry at the end of the run covering
    all phases.

## Phase B — X3: the automated cross-package mutation harness

Goal (the named `Verified` bar from `X3-CROSS-PACKAGE-SWEEP.md`): an automated cargo-mutants
configuration in which mutants in `local_storage_projection` are exercised by the croft-chat
integration suite, killing the 61 in-substrate survivors or justifying each individually.

1. Read `X3-CROSS-PACKAGE-SWEEP.md` first; the survivor set is confined to `fold_auth.rs`,
   `fold_derived.rs`, and `governance.rs` (`governance.rs` counting arithmetic is already
   mutation-clean; its survivors are elsewhere in the file).
2. Determine the tool configuration empirically (`cargo mutants --help`; the tool has evolved —
   do not trust remembered flags): the requirement is mutate-package = `local_storage_projection`,
   test scope includes the croft-chat tests that exercise it. If no single-invocation config
   exists, a thin harness script that copies/patches and drives the croft-chat suite per mutant is
   acceptable — document whichever shape lands as the repeatable command in the report.
3. Bound runtime with `--file` scoping to the three survivor files; a full-package sweep is a
   stretch goal, not required for the bar.
4. Output: `alpha/experiments/local_storage_projection/X3-AUTOMATED-SWEEP.md` — the exact
   repeatable command/config, per-mutant disposition for all 61 (killed / justified-with-reason /
   still-surviving), runtime, and the verdict.
5. **Status upgrades are conditional.** Only if every previously-surviving mutant is killed or
   individually justified: upgrade §7.2 R7's evidence tag `Modeled` → `Verified` (updating its
   parenthetical evidence line to cite the automated sweep), make the matching §8.2 and
   register/MASTER-INDEX updates, and changelog it. If any unjustified survivor remains: no spec
   edits; record the residual in the report and the register row stays as-is.
6. Stop rule: if the harness requires changing production code (not test/dev-dependencies or
   harness scripts), stop and report.

## Phase C — EXP-H1: horizon-manifest determinism (backlog §2b)

Implement the minimal horizon machinery as pure fold-side functions plus tests; no wire formats,
no persistence, no networking.

1. In `local_storage_projection`, add (experiment-grade, documented as such):
   - `horizon_manifest(state) -> (frontier_head, sorted Vec<contradiction byte-heads currently open>)`
     — pure function over folded state; ordering fully deterministic.
   - a trigger predicate for the two cadence terms: fires on an epoch-roll event, and on N facts
     accumulated since the last horizon (counter resets at each boundary). Constants seeded in the
     test, standing in for the genesis rule.
2. Tests (croft-chat, mirroring the competing-quorums harness shape):
   - Two members, one contradiction each way it can now arise (mutual expulsion; competing
     RuleChange), both arrival orders: `horizon_manifest` output **byte-identical** across members
     and orders (comparison via a deterministic test-only serialization, explicitly commented as
     NOT the `[gates-release]` manifest encoding).
   - Both trigger modes produce a boundary at the same fact position on both members.
   - Negative: a resolved/absent contradiction does not appear; an open one never ages out of the
     manifest (decay is presentation, not truth — assert the descriptor persists across horizons).
3. On green: mark backlog §2b done (landing run RUN-07); reviews-log coverage in the RUN-07 entry;
   no spec status changes (the §7.6.9 paragraph stays `Design` — this earns the *manifest
   determinism* claim only, and the report should say exactly that).

## Phase D — EXP-C1: the completeness-ahead contract (backlog §2c)

Four separately RED-able assertions, loopback/fold-level only. Where a needed mechanism does not
yet exist in the crates, implement the minimal experiment-grade version (documented as such); if
any assertion would require MLS internals or network-transport changes, split that assertion out,
report, and land the rest.

1. **Stall-at-threshold.** Withhold one governance fact from node X; X's attempted enforcement of
   a dependent irreversible act stalls below freshness threshold k while X continues serving reads
   on best-known state. Assert: no breach, and reads unaffected.
2. **Stamp detection.** X receives a data-plane entry whose generation stamp is ahead of X's
   governance frontier; assert the gap is detected and sized, and X fills it before acting.
3. **Solicitation reach.** The withheld fact is referenced by nothing (unreferenced tail); X's
   frontier ask to a peer holding it surfaces it; assert the fold then admits it identically to
   normal arrival (same fingerprint as a node that received it live).
4. **Formula-valued k.** With k = ceil(n/2) over the folded member set, assert every node computes
   the identical k at the same act position across arrival orders.

On green: mark backlog §2c done (landing run RUN-07); update the §8.2(e) clause edited in A2 to
record loopback-grade exercise of the origination precondition ("exercised at loopback grade
(EXP-C1, RUN-07); real-NAT path remains X1") — this is the one Phase D spec touch, conditional on
all four assertions passing; changelog + register/MASTER-INDEX accordingly.

## Guardrails (all phases)

- No wire/byte encodings: anything serialized for comparison is test-only and commented as such;
  the `[gates-release]` items stay untouched.
- TDD where the shape allows: land the failing assertion first, then the machinery.
- Both suites + clippy green at every commit boundary; zero new warnings vs baseline.
- Verbatim-anchor rule for every doc edit; minimal diffs.
- Conditional spec edits (B5, D-close) apply only on the stated evidence; when in doubt, report
  instead of upgrading.

## Output

`alpha/experiments/RUN-07-SUMMARY.md`: per-phase status; the X3 verdict and repeatable command;
EXP-H1/EXP-C1 test names and both-order outputs; every conditional spec edit applied or withheld
and why; files changed.
