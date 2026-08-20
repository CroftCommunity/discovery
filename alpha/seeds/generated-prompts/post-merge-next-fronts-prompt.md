# Handoff prompt: the post-merge queue (pick up the first open front)

`Written 2026-08-19, at the close of the graduation-and-merge session. The readmission arc is`
`done: review → C-series/S-series gate (green) → graduation → step-5 merge. Canonical part-2`
`carries everything; the WORKING copy is historical. Copy everything below the line into a fresh`
`session to pick up the queue.`

---

You are picking up the **post-merge queue** for the Drystone spec corpus. The readmission
conversation (five decisions, the independent review, the E112 experiment gate, and the step-5
merge of 2026-08-19) is **closed** — do not re-open it. Your job is the next front in the queue
below. Confirm with the owner which front to take if they haven't said; the default is front 1.

## Orient (read in this order)

1. `alpha/experiments/meer-queue/STATE-AND-NEXT.md` → the **§"2026-08-19 — GRADUATED AND MERGED"**
   closing section — one screen: what merged, what remains, where.
2. `alpha/ROADMAP_TODO.md` rows **E110, E111, E112, E108** (and E109 for front 4) — each row
   carries its residuals and pointers. The backlog is the single source of open items; add new
   items there, never in parallel lists.
3. `beta/drystone-spec/part-2-certifiable-design.md` — **canonical now governs**; the readmission
   machinery lives at §11.6–§11.8 (token, ledger, doors, admission fact, layered gates), §7.3.2
   (`CONTESTED`), §6.4 (sealing), §11.9.3 (E109 bridge), Appendix E (L-arc incl. L7). The WORKING
   copy and the 2026-08-16/17 plans are historical records — cite them for provenance only.
4. For build fronts: `alpha/experiments/meer-queue/TEST-LOG.md` (S23–S26, C4) and
   `alpha/experiments/local_storage_projection/C-SERIES-RESULTS.md` (C2/C3/C5) — the measured
   base, with rungs.

## The queue, in suggested order

1. **E110 — write the A-series admission-interface consolidation** (spec work, fully unblocked).
   A1–A7 are enumerated on the row; the merge added an owed **A8: every admission deposits its
   admission fact** (§11.7). Home: canonical §10.2.2 or a §11.7-adjacent consolidation —
   requirement-vs-realization form, MLS as the realization, so a replacement is a substitution.
   All shapes are measured; this is consolidation writing, not design.
2. **E111 — author the implementation-profile template** (spec work, unblocked except sizing
   dials). The dial list is on the row (doors, issuance timing, token lifetime, finality posture,
   liveness/retention, regime attribute, pins); the door/issuance/finality dials are settled by
   the green gate; the sizing dials still wait on §11.11's measurements. The sheet also carries
   the **Croft presentation obligations** (factual statement §7.6.6; exposure disclosure
   §7.6.12/§11.8; three response registers reachable; returner-side "admission voided"
   legibility) — flag them into the Croft product backlog when the template lands.
3. **E108 implementation — croft-chat** (build). `fold_derived` gains the `CONTESTED` member-view
   state per canonical §7.3.2, with the review's schema requirements: the contradiction artifact
   carries the conflicting **pair as data, in a set** (the current `ForkStatus` min-hash single
   slot cannot represent two open contradictions), plus a **resolution fact type** (the hard-stop
   replay is not resolution), pinned by the arrival-order test. TDD, RED-first; work in a
   worktree (`worktrees/discovery/<name>` — concurrent sessions `git add -A`).
4. **E112 residuals** (build, pick per owner priority): the serve-time challenge-response as a
   signature surface (lineage-root key signing a server-influenced value — needs adversarial
   analysis before the wire pin); post-ban **ledger hygiene** (does a ban force token
   re-issuance?) + ledger pricing at the §11.10 tiers; **door-A** standing-check serve end to
   end; token-lifetime lapse variant + invite-lifecycle unification tests; rung upgrades
   (HeadAck over real iroh-gossip; real signatures on the governance plane).
5. **Separate track, not this queue:** Phase 11 (cap/admission) on the client side —
   `CroftCommunity/connect` `docs/PHASE11-HANDOFF.md`.

## Standing rules

Worktree first (never the shared tree); TDD RED-first for any build front; commit green before
any hand-mutation (`git checkout HEAD -- <path>` is the restore, never stash); no `let _ =`;
honest rungs (loopback is Modeled, never Verified; the Appendix-B completeness beam remains
undischarged — C2/C3 are evidence toward its obligations, not the theorem); plain-English
decision summaries for the owner; quote sources verbatim with file:line in any prompt you write
(a synthesized quote in an earlier handoff became a false review finding); don't commit or push
without the owner's go-ahead.
