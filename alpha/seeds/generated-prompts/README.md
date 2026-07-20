# Generated prompts — index

Ready-made handoff prompts: copy one into a fresh session to run (or re-run) a discrete effort. Each is
comprehensive but still a pointer — the canonical rules live in `../../../AGENTS.md`, `../../../PLAYBOOK.md`,
and the standing indexes. Status notes whether the prompt has been run.

- [`beta-synthesis-discovery-prompt.md`](beta-synthesis-discovery-prompt.md) — kick off the `discovery/beta/`
  resolved synthesis from the alpha corpus. **Run (2026-06-24)** → produced `discovery/beta/` (8 themes +
  README), `alpha/BETA-ROLLUP.md`, `../../../MATURITY-ROLLUP.md`.
- [`beta-factcheck-prompt.md`](beta-factcheck-prompt.md) — adversarially fact-check the beta first pass
  (quote fidelity, verification flags, do-not-carry absence, ledger accuracy) on the highest-capability
  model. **Run (2026-06-24)** → `alpha/plans/2026-06-24-beta-factcheck-report.md` (0 blockers, 6 majors,
  16 minors); corrections applied + logged in `alpha/plans/2026-06-24-beta-factcheck-corrections-log.md`.
- [`beta-factcheck-pass2-prompt.md`](beta-factcheck-pass2-prompt.md) — second clean fact-check pass after
  the pass-1 corrections: re-verify each correction landed and is itself right (REGRESSION-first), plus a
  full fresh adversarial sweep. **Not yet run.**
- [`file-transcripts-prompt.md`](file-transcripts-prompt.md) — file incoming transcripts/dossiers per
  `PLAYBOOK.md` (classify → preserve raw → distill → update connective tissue). Reusable.
- [`process-claude-code-sessions-prompt.md`](process-claude-code-sessions-prompt.md) — process pasted
  Claude Code session logs: verify-against-main first, extract decisions/narrative, leave execution logs
  behind (they're already in RUN summaries + git + registers). Reusable.
- [`games-pond-research-prompt.md`](games-pond-research-prompt.md) — the ponds/pads + P2P-games research
  pass. **Run** → produced `thinking/app/ponds/*` (COHESION §19).
- [`structural-tests-visibility-regimes-prompt.md`](structural-tests-visibility-regimes-prompt.md) — the
  V1–V9 social-layer visibility-regime tests. **Run** → discharged in `lineage-group-model` (COHESION §2).
- [`achilles-heel-research-prompt.md`](achilles-heel-research-prompt.md) — adversarial pressure-test of the
  ordering/consensus "dirty secret" (is the blind superpeer secretly the MLS ordering authority). **Not yet
  run** — the adversarial complement to the F-group proofs (COHESION §4).
- [`beta-coverage-per-file-audit-prompt.md`](beta-coverage-per-file-audit-prompt.md) — per-file alpha→beta
  completeness audit (disposition every alpha file; close the coverage list to zero). **Run (2026-06-25)** →
  `alpha/plans/2026-06-25-beta-coverage-per-file-audit.md` (folded K13–K16; staged T18–T20).
- [`beta-01-review-refinement-prompt.md`](beta-01-review-refinement-prompt.md) — **STALE / SUPERSEDED
  (reference only).** Originally: turn the user's verbal read-through review of beta theme 01 into a
  refinement plan and edit `beta/01-epistemic-foundation.md`. What actually happened: the review was
  processed but the target was reframed — `beta/01` was **retired** and its reasoning became **Part 1 of
  the new Drystone protocol spec** (`beta/drystone-spec/`). The transcript moved to
  `beta/thinking/raw/01_beta_review.txt`. See the banner in the prompt and
  `../../plans/2026-06-26-beta-01-review-refinements.md`. Do not run as written.
