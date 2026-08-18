# Handoff prompt: build and run the C-series + amended S-series (head-currency, admission fact, token re-entry)

`Written 2026-08-17, at the close of the independent-review session (review delivered; the`
`admission-fact design and the C-series agreed with the owner in talk-through). Copy everything`
`below the line into a fresh session.`

---

You are the **build-and-run session** for the experiment set that gates decision-2's merge into
canonical Part 2. Two prior arcs feed you: the S23–S26 token-re-entry plan (2026-08-16) and the
independent review + owner talk-through (2026-08-17) that amended it and added the C-series. Your
job: build and run the experiments RED-first, record results with honest fidelity rungs, and amend
the handoff state. **You do not merge anything into canonical Part 2, and you do not edit the
WORKING copy's decision REV blocks — ratification wording is the owner's.**

## Orient (read in this order)

1. `alpha/plans/2026-08-17-1-plan-head-currency-and-admission-fact.md` — **your primary plan**:
   C2 (behind-detection), C3 (HeadAck), C4 (the Bob/Dana stale-admission end-to-end, incl. the
   arrival-order permutation arm 1a and the two-sided boundary control arm 3), C5 (ack cost,
   informative), the admission-fact design amendment (typed as an acceptance/event record opening
   a membership span — never a slot-competing addition; §11.11 item 3's mapping obligation
   discharged there), and the S24/S25 amendments (S24 refusal arm (d) forged-ledger/severed-fact;
   S25 banned-holder population arm; S25 consumes HeadAck as the corroboration source).
2. `alpha/plans/2026-08-16-1-plan-token-reentry-proveout.md` — the S23–S26 base plan you amend.
3. `beta/drystone-spec/REVIEW-decision-talkthrough-2026-08-17.md` — the review these amendments
   answer. Minimum: the verdict summary (worst-three), section H's coverage table (what is
   COVERED / DEFERRED-stated / UNCOVERED), and the missed-issues register.
4. `beta/drystone-spec/part-2-certifiable-design-WORKING-2026-08-16.md` — the decision REV blocks
   (`grep -n 'REV 2026-08-1'` — note the one `2026-08-17` tag). Canonical part-2 stays untouched.
5. `alpha/experiments/meer-queue/STATE-AND-NEXT.md` and `TEST-LOG.md` — harness state, S1–S22
   evidence, the "easy to get wrong later" list (read it; several entries bite these builds).
6. ROADMAP rows **E105–E112** (`alpha/ROADMAP_TODO.md`) — E112 is this work's row.

**Homes:** C4 and S23–S26 in `alpha/experiments/meer-queue` (real openmls 0.8.1, Rung A for the
MLS half); C2/C3/C5 in the croft-chat / `local_storage_projection` side (the fold in
`fold_derived.rs`; real iroh-gossip at loopback per the FANOUT-M1 harness). The fold and gossip
work is `Modeled`/loopback rung — label it so.

## Standing rules

- **Worktree first.** Concurrent sessions in this workspace `git add -A`; never work in the
  shared tree. `git -C discovery worktree add ../worktrees/discovery/c-series <branch>` (worktrees
  live under `CroftC/worktrees/<parent_repo>/<name>`).
- **TDD, RED-first, no exceptions.** Every arm starts failing. S23 arm 1 and C2 arm 1 are
  explicitly RED-first in their plans — watch them fail for the stated reason before wiring the
  mechanism. Commit the green state before any hand-mutation (the restore path is
  `git checkout HEAD -- <path>`, never stash-based). Wait for owner approval before commits per
  house rules.
- **Method guards** (standing findings, both plans): surface every ingest/processing result — no
  `let _ =` (G1's lesson); compare branches at equal epoch numbers for AEAD-grade checks (S19's
  lesson); scenario realism (the adversary is a former member, not a stranger — the banned-holder
  arm exists for exactly this).
- **Honest rungs.** A green C-series does **not** discharge the Appendix B completeness beam —
  the plan maps each arm to the beam's four discharge obligations; report against that mapping,
  never as "beam earned." Loopback is `Modeled`, not `Verified`.
- **Ground-truth ordering** stands: test code > TEST-LOG > dossier/walk > WORKING REV prose >
  ROADMAP rows. If a run contradicts a REV claim, the run wins and the disagreement is a finding
  to report, not to smooth over.
- **The gate** for decision-2's graduation is **S23–S26 + C2–C4** (C5 informative). The
  S23–S25-vs-S23–S26 wording inconsistency across the six artifacts is known (review item 25);
  align them only if the owner asks.

## Suggested order of work

1. **S23** (ledger, RED negative arm first) — cheapest, and its failure-mode answer (clean error
   vs silent drop) is load-bearing for the strict-merge premise (review, group D missed-issue 4).
2. **C2 + C3** in parallel with S23 (different harness side): behind-detection, then HeadAck with
   its union/adversarial arms.
3. **S24 as amended** (graceful + admission-fact assertions; refusal arms a–d incl. the new
   severed-fact arm; s-i/s-ii; perishability; artifact isolation).
4. **C4** (same-branch, arrival-order permutation 1a, diverged-branch heal, boundary control) —
   consumes S24's machinery and the admission fact.
5. **S25 as amended** (four arms + banned-holder population arm, HeadAck as corroboration
   source) — produces the first propagation-window number.
6. **S26** (admission-at-position; the characterization arm 2 is the mutation-killer).
7. **C5** (ack cost curve) last; informative.

## Exit

Per both plans' exit criteria: arms green or failing-with-named-understood-modes; TEST-LOG.md
rows with fidelity rungs; STATE-AND-NEXT amended; report which review coverage gaps
(section H, item 24) each run closed. Then stop — the WORKING REV graduation and the step-5 merge
are the owner's next conversation, not this session's.
