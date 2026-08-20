# Handoff prompt: graduate decision-2 and run the step-5 merge (the gate is now green)

`EXECUTED 2026-08-19, in-session with the owner (one clean merge + the L7 beat; diff reviewed by`
`the owner before commit). Kept for the record; see STATE-AND-NEXT §2026-08-19 for the outcome.`

`Written 2026-08-18, at the close of the E112 build-and-run session. The C-series + amended`
`S-series ran green on branch c-series-proveout (discovery worktree). Everything below the line`
`is for a fresh session; copy it in.`

---

You are the **graduation-and-merge session** for the readmission conversation (decisions 1–2 of the
five-decision talk-through). The thing that gated you is now discharged: the experiment gate for
decision-2 — **S23–S26 + C2–C4** (C5 informative) — ran **green**, RED-first where the plans
designate it. Your job is the work that was blocked on that outcome: **graduate decision-2's REV
blocks from PRELIMINARY, execute the step-5 merge into canonical Part 2, and align the six
gate-wording artifacts.** Ratification wording is the owner's — you prepare and propose; the owner
ratifies.

## What is now unblocked (and was not before)

Before this, decision-2's mechanism text sat at *Design, preliminary — gated on S23–S26*, and the
merge could not include it. The gate is met, so:

1. **Decision-2 graduates.** The WORKING §11.6/§11.7 DECISION-2 REV blocks — *and the admission-fact
   amendment* — can move from PRELIMINARY to ratified/evidence-complete, because the composition is
   now measured, not asserted.
2. **The step-5 merge can include decision-2.** The open question was scope; the gate being green
   removes the "carry decision-2 as preliminary" branch from the options.
3. **The six-artifact wording inconsistency is resolvable.** The gate is **S23–S26 + C2–C4** (review
   item 25); align the artifacts that still say "S23–S25" or bare "S23–S26."

## Orient (read in this order)

1. `alpha/experiments/meer-queue/STATE-AND-NEXT.md` → the **2026-08-17 (evening)** section — the
   one-screen outcome, the fidelity rungs, and the **review-coverage-gap mapping** (which run
   closed which gap). This is your "what happened" artifact.
2. `alpha/experiments/meer-queue/TEST-LOG.md` → the S23, S24, C4, S25, S26 sections (verdict lines
   with rungs). `alpha/experiments/local_storage_projection/C-SERIES-RESULTS.md` → C2, C3, C5.
   **Ground-truth ordering stands: test code > TEST-LOG > dossier/walk > WORKING REV prose >
   ROADMAP rows.** If a verdict contradicts a REV claim, the run wins and the disagreement is a
   finding to raise, not smooth over.
3. `alpha/plans/2026-08-17-1-plan-head-currency-and-admission-fact.md` → the admission-fact design
   amendment (the "What it buys" list is the merge-prose ammunition) and the **comparator placement**
   the owner settled (acceptance/event record opening a span, never slot-competing — §11.11 item 3).
4. `beta/drystone-spec/REVIEW-decision-talkthrough-2026-08-17.md` → the verdict summary, section H
   coverage table, and the missed-issues register (so the graduation prose cites what is now covered
   vs still DEFERRED/UNCOVERED).
5. `beta/drystone-spec/part-2-certifiable-design-WORKING-2026-08-16.md` → the DECISION-2 REV blocks
   you will graduate (`grep -n 'REV 2026-08-1'`; note the one `2026-08-17` tag). **Canonical part-2
   is still untouched — the merge is the act that changes it.**

The build lives on branch **`c-series-proveout`** (worktree `CroftC/worktrees/discovery/c-series`),
committed (spec/plan base + build). Note the worktree needs a symlink
`worktrees/discovery/CISS → ../../CISS` to build meer-queue (a cross-repo path dep); it is untracked.

## What to produce

- **Graduation edits (WORKING copy).** Move the §11.6/§11.7 DECISION-2 blocks and the admission-fact
  amendment from PRELIMINARY to graduated, each citing its evidence by verdict line (e.g. "S24 arm
  (d) MEASURED" for the severed-fact leg). Do **not** invent stronger claims than the rungs support —
  loopback is Modeled, not Verified; the Appendix-B beam is *not* discharged (these runs earn
  evidence against obligations 1–4, not the theorem — say so where the beam is referenced).
- **The step-5 merge into canonical Part 2.** Fold the ratified corrections + E106/E108/E96 +
  cold-is-a-state + the now-graduated decision-2 (incl. the admission fact) into canonical part-2.
  Carry the merge-prose properties the plan lists: fork detection from chain data (narrows E107's
  open signal), the restored §7.3.6 decision/enactment split on the external-commit path, the
  successor text for the orphaned §11.8 positioning MUST (item 23.3), and add-commit-as-mint-point
  now an asserted property (gap 6).
- **Six-artifact alignment to `S23–S26 + C2–C4`.** DECISION-2 header, ROADMAP E105/E107/E110/E111,
  the banner, STATE-AND-NEXT, and the plan exit criteria. One pass, one wording.
- **The Croft presentation obligations (note, do not build).** They ride E111 (product layer, not a
  protocol dial): the §7.6.6 factual statement, the §7.6.12 + corrected-§11.8 exposure disclosure,
  the three response registers reachable, and returner-side legibility (E108's no-lying-by-omission).
  Flag them as downstream product work so they are not lost.

## The reviewer's five-item merge checklist (added 2026-08-18 — corrections the merge must carry)

From `REVIEW-decision-talkthrough-2026-08-17.md`; each has its evidence pointer there. These are
merge-time prose obligations the gate's green runs did **not** retire:

1. **Family-dial qualifiers (review item 6).** When cold-is-a-state merges into §11.6, qualify the
   "effectively infinite liveness window" paragraph: §11.5's mandatory sub-250 heal cadence makes
   missed-epoch count grow with absence; no-meer **phase-1** catch-up has no measured
   commit-stream home (S22 measured GroupInfo serving, a phase-2 artifact).
2. **Wrap-once-at-origin + §6.6.2 rewording (review group F missed-issue).** E96's merge must add
   "an object is wrapped once, at origin; the wrapped bytes are the identity all paths carry"
   (random-nonce sealing otherwise breaks §6.6.4 cross-path dedup), and reword §6.6.2's "identical
   `PrivateMessage` bytes" → "the outermost sealed bytes."
3. **The §7.3.8 qualifier on "never admission" (review item 13).** S25 arm 4 measured the refusal
   half; "roster knowledge only" is structural. The corollary must read "…never admission, up to
   §7.3.8's corroborated-not-proven completeness residual (the Appendix B beam)."
4. **The at-position ↔ standing-at-head reconciling sentence (review group D missed-issue 1).**
   S26's at-position rule and §11.8's "standing resolves at head" merge into one document; write
   the sentence reconciling them (redemption-at-live-edge vs replay-evaluation) or they read as
   contradictory.
5. **E109/E108 citation fixes (review items 17, 22).** Attribute "confidentiality past a certain
   group size is an illusion" to the owner (it is not in §11.9.3); bridge §8's genesis-immutability
   to §11.9.3/§11.10's "enter the regime" wording (successor-Group form or cross-ref to E109); fix
   E108's §7.6 pointer (the presentation sentence lives in §7.3.2).

Also in scope from the review, smaller: evidence-grade tags on the untagged §11.7 talk-through
addenda (group C missed-issue); the two orphaned §11.8 paragraphs (already in "What to produce").

## Decisions that are the owner's (surface, do not choose)

- The **exact ratification wording** of each graduated REV (you draft; the owner ratifies).
- **Merge granularity** — one clean merge now, or ratified-corrections-first with decision-2 in a
  second pass. The gate being green makes "one clean merge" viable; confirm with the owner.
- Anything where a run **contradicts** a prior REV claim — raise it; do not reconcile silently.

## Stop

When the graduation edits + the step-5 merge + the six-artifact alignment are drafted and the
owner has ratified wording, canonical Part 2 carries decision-2. Then the readmission conversation's
step 5 is closed, and the next front is **Phase 11 (cap/admission)** on the client side
(`CroftCommunity/connect` `docs/PHASE11-HANDOFF.md`) — a separate track, not this session's.
