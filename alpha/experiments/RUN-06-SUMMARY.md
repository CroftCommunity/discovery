# RUN-06 — Findings settlement (the RUN-05 register, closed)

`Branch: claude/run-06-findings-settlement-f63jxn (off the RUN-05 merge). Markdown + comment-only code.
Captured 2026-07-14.`

RUN-05 ran the full consistency pass and recorded 11 FINDINGS in `CONSISTENCY-FINDINGS-2026-07.md` —
items requiring a judgment call or a meaning change, deliberately left unedited. RUN-06 **is** the
owner-call pass: it settles all 11. The load-bearing judgment calls (FND-1, FND-2, FND-9, and the FND-5/10
scope) were owner-decided 2026-07-14; the rest carried a single clear resolution. The per-finding
disposition table with quoted basis is the new **Settlement (RUN-06)** section of
`CONSISTENCY-FINDINGS-2026-07.md`; this file is the run's index and rationale.

---

## Owner calls (decided 2026-07-14)

- **FND-1 — accept §7.4.2 as shorthand.** The `§7.4.2` citation for the "must be caught-up +
  corroboration-fresh to originate a membership/governance op" precondition is confirmed as the accepted
  doc-wide shorthand for the §7.4 freshness + §7.4.2 recovery cluster. No edit. (The other §7.4.2
  citations — GroupInfo-as-claim corroboration — are correct on their own terms.) Recorded so later passes
  do not re-flag it.
- **FND-2 — land F4.** §11.11 measurement #1 regraded from *half-earned* to *earned in shape* (both
  halves), *magnitude-open at scale*, on the strength of RUN-01 EXP-1's real iroh-gossip loopback
  measurement, carrying the `fanout-single-run` magnitude caveat and the super-linear connect-time-resync
  flag (§6.8.1).
- **FND-9 — `fanout-single-run` stays Active.** A live proxy-measurement gap (shape holds, magnitude
  indicative). No edit needed: the register and `MASTER-INDEX.md` already list both Active rows, and no
  live doc asserted a single active row. Consistent with landing F4.
- **FND-5 / FND-10 — settle both.** §7.6.11's preserved banner normalized to the A.9 ladder; the RUN-02..04
  governance/reconcile terms added to the conventions A.11 shared surface.

## The applied fixes (one line each)

- **FND-2** — `part-2-certifiable-design.md` §11.11 measurement #1 regraded; `proposed-changes-…F4` marked
  landed; `part-2-changelog.md` RUN-06 entry.
- **FND-3** — the 7 short-form `alpha/SPEC-DIVERGENCE-REGISTER.md` uses repointed to
  `alpha/experiments/SPEC-DIVERGENCE-REGISTER.md` (register SPEC-DELTA template + two live code tags,
  comment-only, + `REPO-README.md` + three `croft-chat/plans/*`). Template and code tags moved together so
  the greppable format stays in sync.
- **FND-4** — the 5 `11-doc-method.md` cites in `conventions-and-decisions.md` repointed to
  `doc-writing-method.md` (canonical `beta/impl/doc-writing-method.md`). The `alpha/seeds/p10-*` raw-seed
  copy left verbatim.
- **FND-5** — §7.6.11 banner: `design → Design`, `[confirm before publish] → [confirm]`; the two cited
  source paths gained the leading `../`. Preserved block otherwise untouched.
- **FND-6** — `part-2-changelog.md` Pass-2 entry `12-side-histories-and-threading.md` →
  `side-histories-and-threading.md`. `07-history-modes.md` left with its prefix (the file keeps it).
- **FND-7** — the in-repo `alpha/thinking/revocation-authority.md` confirmed authoritative; the
  "out of this workspace" reference in `EXPERIMENT-BACKLOG.md` §6d dropped and repointed;
  `iroh/TEST-LOG.md` repointed.
- **FND-8** — verified already clean (no doubled "Part 2" at Part 1 §2.5); recorded, no edit.
- **FND-10** — conventions A.11 extended with *approval subject*, *contradiction byte-head*,
  *horizon checkpoint* / *horizon-checkpoint manifest*, *corroboration dials*, *quantified trust*, each
  anchored to its Part 2 definition.
- **FND-11** — Part 1 `## 0. Map` gained the missing `## Upstream reference links (versioned)` entry.

---

## Guardrail compliance

- RUN-06 inverts RUN-05's FIX/FINDING deferral by design: this is the settlement pass, so FINDINGS are
  resolved, not re-deferred. Every meaning-changing or judgment call was owner-decided; the mechanical
  repoints are provable and meaning-preserving.
- Three normally-protected surfaces were touched, each within the finding's stated permission: Part 1
  back-matter (FND-11, an additive Map line only — no body prose), `conventions-and-decisions.md` (FND-4
  path-fix + FND-10 vocabulary addition), and the code SPEC-DELTA tags (FND-3, comment-only).
- No mechanism and no code behavior changed. The `.rs` diff is comment-only by inspection (`git diff`
  shows only `//` lines). No document deleted; no prose reflowed beyond the anchored edits. Part 2 body
  inserts use house em-dash style; changelog prose stays hyphen / `RED to GREEN`-only.

## Files changed

Markdown:
`beta/drystone-spec/part-2-certifiable-design.md` (§11.11, §7.6.11),
`beta/drystone-spec/part-1-reasoning-underpinnings.md` (Map),
`beta/drystone-spec/conventions-and-decisions.md` (A.11, doc-method paths),
`beta/drystone-spec/part-2-changelog.md`,
`beta/drystone-spec/proposed-changes-2026-07-experiment-reconciliation.md`,
`alpha/experiments/SPEC-DIVERGENCE-REGISTER.md`, `alpha/experiments/REPO-README.md`,
`alpha/experiments/EXPERIMENT-BACKLOG.md`, `alpha/experiments/MASTER-INDEX.md`,
`alpha/experiments/iroh/TEST-LOG.md`,
`alpha/experiments/croft-chat/plans/2026-07-11-1-plan-next-experiments.md`,
`alpha/experiments/croft-chat/plans/2026-07-12-2-design-threshold-enforcement.md`,
`alpha/experiments/CONSISTENCY-FINDINGS-2026-07.md` (settlement ledger),
`alpha/experiments/RUN-06-SUMMARY.md` (this file).

Code (comment-only):
`alpha/experiments/croft-chat/croft-chat/src/iroh_bus.rs`,
`alpha/experiments/croft-chat/croft-chat/tests/iroh_convergence.rs`.
