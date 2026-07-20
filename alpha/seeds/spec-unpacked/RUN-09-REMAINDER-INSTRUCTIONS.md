# RUN-09 — The runnable remainder: settle, cover, continue, reconcile, measure

`Branch: fresh off main (RUN-08 merged), e.g. run-09-remainder. Five parts in dependency-and-risk
order; EVERY part ships independently (commit + summary section per part), so the run may stop
cleanly after any part and the branch still merges. Supersedes the retired standalone
RUN-09-INSTRUCTIONS (its content landed in RUN-08 Part 2).`

## Run-wide rules

- **The I9 firewall** holds: no trust-tier work (release predicates, threshold shares,
  revoke-authority ordering, co-sign-vs-vote). Parked with X1/hot-N-500, the `[gates-release]` +
  BLAKE3 pass, and the frozen-emitter integration — none of these are in this run.
- **TDD, every code part:** acceptance criteria land as failing tests first (fixtures before
  features), and each part's summary section evidences the red → green order.
- FIX/FINDING discipline for any doc surprise; frozen-record rule with its ratified normalization
  exception; verbatim anchors; minimal diffs; conditional spec/register edits only on stated
  evidence; both suites + clippy green at every commit boundary, zero new warnings.
- **Pre-compliance:** every new report/test carries the `Serves:` header / `Earns/bounds:`
  doc-comment from birth, and EVIDENCE-MAP.md gains a row for anything that earns or bounds a
  tagged claim (the map stays index-not-source).

---

## Part 1 — riders: traceability settlement + the Proofs confirmation (markdown)

Apply the five rulings (owner-confirmed 2026-07-15) and record them in
CONSISTENCY-FINDINGS-2026-07.md's settlement section:

1. **FND-T1 — per-band target form.** The forward-link target is redefined per evidence band:
   experiment-earned tags resolve to test/report + RUN; `Verified-RFC` / literature-anchored tags
   resolve to their primary-source anchor (RFC/section), which is already their correct evidence.
   Add one sentence to conventions A.9 stating the two forms. Re-audit the ~40-tag substrate band
   under the right form: any *experiment-earned* tag still lacking a pointer stays a FINDING;
   the rest close as satisfied.
2. **FND-T4 — no retrofit.** The standardized `(evidence: …, RUN-NN[, grade])` parenthetical is
   NOT retrofitted; adopt it via one A.9 note as the recommended form for evidence sentences
   written from now on.
3. **FND-T5.** De-backtick `Reviewer-judgment` in §10.4 into plain prose (no eleventh rung).
4. **FND-T6.** One A.9 note: the legacy `green-real`/`green-model`/`not_yet_emitted` vocabulary is
   alpha-tier only and never appears in Part 1/Part 2.
5. **FND-T7.** Canonicalize the hyphenated `real-NAT` everywhere in living docs.
6. The reviews-log RUN-09 entry records: the Proofs fold-in authorization confirmed by the owner
   2026-07-15.

## Part 2 — Vouch payload-validation coverage (backlog §2d)

1. RED first: consumer-path tests (croft-chat side, mirroring the existing harness shapes) that
   exercise the fold's I5 Vouch payload gate (`fold_derived.rs:447–472`) — well-formed accepted,
   malformed/oversized/truncated rejected, and the acceptance/rejection visible through folded
   state, not internals.
2. GREEN, then prove: re-run the scoped mutation sweep (`--file fold_derived.rs`, the RUN-07
   command from `X3-AUTOMATED-SWEEP.md`) and show the 10 previously-justified Vouch survivors are
   now killed. Append the result to `X3-AUTOMATED-SWEEP.md` as a dated addendum section (living
   report gains an addendum; the original verdict text is not rewritten).
3. Retire backlog §2d (landing run RUN-09). No spec tag moves — this closes a coverage residual,
   not a claim.

## Part 3 — B1 → A5: the message-continuity half of re-plant (the last major build)

Goal: §7.6.2's other half — an in-flight conversation survives the repoint with **no loss and no
duplication**. The membership half is already `Verified`; the message half is unbuilt.

1. Read first: the `replant-continuity` crate (it exists and carries a `Serves:` header — build on
   it, do not start parallel); Appendix B's B1 dataplane hash-structure items; §7.6.2's continuity
   claims; the E12.2/E12.7 backlog rows.
2. RED: the continuity assertions as failing tests over two members and a re-plant —
   (a) every data-plane entry authored before the repoint is present after it, exactly once;
   (b) entries authored *during* the repoint window (in flight) land exactly once, on the
   post-repoint group, in causal order;
   (c) both members' post-repoint folds and content sets are byte-identical across arrival orders;
   (d) negative: a duplicated or dropped frame injected by the harness is detected, not absorbed.
3. GREEN: implement the minimal B1 hash structures the assertions require (experiment-grade,
   documented; no `[gates-release]` byte pinning — test-only serialization where comparison needs
   bytes).
4. Conditional edits, only on all-green: §7.6.2's continuity sentence upgrades to name both halves
   with the message half at **loopback grade** (`Modeled` unless the evidence genuinely meets a
   higher rung under A.9 — when in doubt, `Modeled` and say why in the summary); EVIDENCE-MAP row;
   backlog E12.2/E12.7 → done; changelog/MASTER-INDEX.
5. Stop rules: MLS-internals changes; anything requiring the trust tier; wire-format pinning. If
   the in-flight (b) case needs transport machinery that doesn't exist at loopback, land (a)/(c)/
   (d), split (b) out as a precisely-shaped backlog row, and say so.

## Part 4 — RBSR slice: steady-state anti-entropy at loopback (§6.8.1's open half)

Connect-time catch-up is proven; recovering a frame dropped **between already-connected peers** is
not, and FANOUT-M1's super-linear connect-time-resync flag points here.

1. RED: two connected members, one gossip frame suppressed by the harness mid-session; assert the
   gap is detected and repaired without a reconnect, and the folds re-converge byte-identically.
2. GREEN: the minimal range-reconciliation (or digest-compare) pass that repairs it —
   experiment-grade, loopback, no wire pinning.
3. Conditional edit on green: one §6.8.1 sentence recording steady-state repair at loopback grade
   (tag per A.9, same when-in-doubt rule); EVIDENCE-MAP row; backlog row updated.
4. Ship-without rule: this part is the first to drop if the run runs long — record the RED tests
   as the precisely-shaped backlog item and stop.

## Part 5 — fan-out repeated-run (measurement, cheapest last)

The `fanout-single-run` register row retires on "repeated-run or hot-N measurement." The
repeated-run arm is loopback-cheap:

1. Re-run the FANOUT-M1 measurement K times (K ≥ 5) at the existing N = 2/4/8/16, same harness;
   report per-N spread (min/median/max) as a dated addendum to `FANOUT-M1.md`.
2. Conditional, only if the spread supports the recorded magnitudes: retire `fanout-single-run`
   per its own stated condition (Reconciled, landing run RUN-09) and fix the one §11.11 caveat
   clause that names it. If the spread is wide: the row stays, the addendum records it, and that
   honesty is the deliverable.

---

## Output

`alpha/experiments/RUN-09-SUMMARY.md`: one section per part — status (done / shipped-partial /
dropped-at-ship-rule), red → green evidence for Parts 2–4, every conditional edit applied or
withheld and why, and the parked list restated (I9; X1; hot-N 500+; `[gates-release]` + BLAKE3;
frozen-emitter integration; Layer-2 design frontier) so the board state is readable from the
summary alone. Findings, if any, to CONSISTENCY-FINDINGS-2026-07.md for the usual walk.
