# RUN-13 — the gradients on Pages: five tiers, the classroom scaffold, Mermaid, review packaging

`Run summary, 2026-07-16. Branch claude/run-13-gradients-classroom-qg3jev, restarted from main at
the RUN-12 merge (7261b43) — the run's gate ("RUN-12 merged") was honored: execution began only
after PR #13 landed (the session stopped at the gate first, with RUN-12 still mid-flight, and
resumed on the owner's go). Markdown + site tooling only — no spec, register, or crate edits.
Site gate + resolver suite green at every commit; the diagram gate is new this run and green.`

## Per-part status

| Part | What | Status |
|---|---|---|
| 1 | Five-tier gradients model in `beta/socialization/visual-identity-and-the-progressive-depth-website.md` | ✅ done (`71f2844`) |
| 2 | The one-liner candidate table, `FOR REVIEW`, no winner declared | ✅ done (same commit) |
| 3 | `alpha/classroom/` scaffold: arc verbatim + ten chapter skeletons + backlog row | ✅ done (`ecfa30f`) |
| 4 | Mermaid in the site build (TDD, gated) + Gradients/Classroom nav + landing reading-paths | ✅ done (`d6b29ff`) |
| 5 | `REVIEW-2026-07.md` review packet, published beside Gradients | ✅ done (`480af62`) |

**Part 1.** The three-depth model generalized to five tiers (one-liner → elevator → over tea →
classroom → library), the whole-truth constraint preserved as the invariant (*order, energy,
altitude — never truth*), per-tier character blocks (question / perspective / energy / depth /
failure modes), the tier-1 test battery, and the loop principle (the tagline and the classroom
refrain as siblings). The existing Surface/Soil/Bedrock homepage architecture kept, mapped onto
tiers 2/3/5. Naming note: the run brief's "`elevator-pitch.md`'s one-liner register" resolves in
this tree to **`sixty-second-pitch.md`** (the elevator-pitch doc that carries `## The one-liner`);
the promotion cross-reference names the real file.

**Part 2.** `## The one-liner: candidates under test` — battery stated, five seed candidates each
scored (source truth / secondhand / hostile / library). Marked `FOR REVIEW — owner selects`; the
elevator doc's register is untouched pending selection.

**Part 3.** `00-arc.md` committed verbatim as provided; skeletons `01-two-people` …
`10-the-planet` carry the arc's beats as headed placeholders (NEED / STORY / DIAGRAM / PRECISE
STATEMENT / PROVE-IT / REFRAIN), bodies `DRAFT-PENDING (written in conversation, not by runs)`;
chapters 01 and 05 carry the two seed Mermaid diagrams verbatim (the pinned renderer accepted the
`<br/>`/`<i>`/`<b>` label markup as drawn — **no simplification was needed**). Backlog gains the
classroom docs-track row (§7).

**Part 4.** See "the Mermaid tooling choice" below. Nav: `gradients.html` + eleven
`classroom-*.html` pages join the published set (part-2/part-1 §-fallbacks; tier badges and draft
banners); the landing page presents the five tiers as reading paths into the same truth; the top
nav gains Gradients/Classroom; CI installs the pinned renderer before the same gate run.

**Part 5.** Seven numbered one-line review questions (one-liner slate; tier characters; invariant
wording; act structure; chapter order; refrain–tagline coupling; the dispatch-desk register) plus
a one-line respond-how.

## The Mermaid tooling choice, and its red → green

**Choice: build-time pre-render** via `@mermaid-js/mermaid-cli`, pinned at **11.16.0** (mermaid
11.16.0) in `site/package.json` + lockfile — decided empirically over a pinned client-side
`mermaid.min.js`:

- **No-network-at-read, strongest form.** Diagrams are inline SVG in the emitted HTML; the
  published page needs no JavaScript and makes no fetch of any kind. (A vendored client script
  would also avoid third-party fetches but interposes ~2.8 MB of JS between reader and figure.)
- **The gate for free.** The renderer's parse failure *is* the build failure — display path and
  validation path cannot drift. Verified empirically: an injected bad block fails the build with
  exit 1, naming the file (`alpha/classroom/99-temp-bad.md: mermaid block 1 failed to render:
  Parse error on line 4`).
- Network only at build time (CI: puppeteer fetches its headless browser during `npm ci`; local:
  `PUPPETEER_EXECUTABLE_PATH` points at an existing chromium).

**TDD red → green.** Three resolver-test additions written first — (a) a valid block renders to
SVG and the raw block is gone, the renderer receiving unescaped source; (b) an invalid block
raises naming the source file; (c) a ```mermaid fence quoted inside a fenced code example is not
double-processed (renderer never called, HTML untouched) — run RED (import error: the
`substitute_mermaid_blocks` contract did not exist), then implemented in `site/resolver.py`,
run GREEN: **29 resolver tests OK** (26 prior + 3). Full build: **2 diagrams rendered**, unique
SVG ids, 0 hard-gated refs, companion allowlist exact.

## PROVE-IT pointer verification (every pointer resolved against the tree; none invented)

| Chapter | Pointer | Resolves to |
|---|---|---|
| 01 | `dedup.rs` | `alpha/experiments/croft-chat/croft-chat/tests/dedup.rs`; EVIDENCE-MAP §6.6.4 |
| 01, 02 | `convergence.rs` | `…/croft-chat/tests/convergence.rs`; EVIDENCE-MAP §4.1–4.6, §7.3.2 |
| 02 | corroboration counting | `…/croft-chat/tests/horizon_checkpoint.rs` (RUN-12 EXP-H2); EVIDENCE-MAP §7.6.9 / §7.3.3 |
| 03, 07 | `rulechange_quorum_via_api.rs`, `competing_quorums.rs` | `…/croft-chat/tests/`; EVIDENCE-MAP §7.2 R7, §7.2 residual, §7.3.2 |
| 03, 06, 09 | `l2a_sealed_frame.rs` (assertions 1/4/6) | `alpha/experiments/croft-group/crates/group-seal/tests/`; EVIDENCE-MAP §10.2/§10.5/§7.6.2 L2a row |
| 04 | `FANOUT-M1` | `alpha/experiments/croft-chat/FANOUT-M1.md`; EVIDENCE-MAP §11.11 #1 / §11.4–11.5 |
| 04 | steady-state repair | `…/croft-chat/tests/steady_state_anti_entropy.rs` (RUN-09) + `partitioned_anti_entropy.rs` (RUN-12); EVIDENCE-MAP §6.8.1 |
| 04 | EXP-H1 | `…/croft-chat/tests/horizon_manifest.rs`; EVIDENCE-MAP §7.6.9 |
| 05 | `mls-welcome-over-iroh` | `alpha/experiments/iroh/crates/mls-welcome-over-iroh` + `relay-lab-runs/C-mls-welcome-2026-07-15-run08`; EVIDENCE-MAP §10.5 (a) |
| 05 | §7.6.2 continuity | `alpha/experiments/replant-continuity/tests/e12_7_{1,2,3}_*.rs`, `e12_2_message_continuity.rs`; `…/croft-chat/tests/iroh_message_continuity.rs`; EVIDENCE-MAP §7.6.2 (both halves) |
| 05, 10 | `hermetic-gossip` | `alpha/experiments/SPEC-DIVERGENCE-REGISTER.md` (the only Active row; real-NAT = X1) |
| 06 | §5.11, §7.2 R6 | Part 2 headings/R-bullet (anchors `s5-11`, `s7-2-r6` verified) |
| 08 | BIP39 lock | `alpha/experiments/bip39-recovery-roundtrip` (11 tests); EVIDENCE-MAP §7.3.9; trust predicate = I9, taught as open |
| 09 | EXP-C1 stall / formula-k | `…/croft-chat/tests/completeness_ahead.rs` (`stall_at_threshold_no_breach_reads_unaffected`, `formula_valued_k_identical_across_orders`); EVIDENCE-MAP §7.3.3 / §7.4.1 |
| 10 | the map itself | `beta/drystone-spec/EVIDENCE-MAP.md` |

Mapping notes (recorded, not silently normalized): the arc's "EVIDENCE-MAP §7.3" wires to the
actual §7.3.2 rows plus §6.6.4; the arc's "the M2 steady-state tests" wires to
`steady_state_anti_entropy.rs`/`partitioned_anti_entropy.rs` (the §6.8.1 rows — no test in the
tree is literally named "M2"); `FANOUT-M1.md` lives under `croft-chat/`, not `iroh/`. Every §-ref
in the published classroom/gradients pages resolves through the part-2/part-1 fallback chain
(machine-checked by the gate on every build).

## Audit re-run + finding

RUN-10-scope audit clean on the emitted site: **1233 cross-file anchor links, 0 broken; 1866
same-file anchor links, 0 dangling**; companion allowlist exact (7/7, unchanged). A stricter
extension (plain file-target hrefs) surfaced 5 pre-existing raw relative-`.md` links in three
`alpha/thinking` docs → recorded as **FND-R13-1** (LOW, `CONSISTENCY-FINDINGS-2026-07.md`), not
fixed — the sources sit outside this run's sanctioned edit surface.

## The review URL (deployment)

The one-time Pages setup is **already flipped**: the pages workflow run for the RUN-12 merge on
`main` completed **success including the deploy job** (run #7, 2026-07-16), so the site is live at
<https://croftcommunity.github.io/discovery/>. When this branch merges, the review packet
deploys automatically to:

- **<https://croftcommunity.github.io/discovery/REVIEW-2026-07.html>** (the packet)
- <https://croftcommunity.github.io/discovery/gradients.html> (the model + the candidate table)
- <https://croftcommunity.github.io/discovery/classroom-00-arc.html> (the arc; chapters linked)

## Files

**Added:** `alpha/classroom/{00-arc.md, 01-two-people.md, 02-the-witness.md, 03-the-club.md,
04-the-hall.md, 05-the-split-room.md, 06-the-helpers.md, 07-the-co-op.md, 08-the-nonprofit.md,
09-the-dispatch-desk.md, 10-the-planet.md}`; `beta/socialization/REVIEW-2026-07.md`;
`site/{package.json, package-lock.json, puppeteer-config.json}`;
`alpha/experiments/RUN-13-SUMMARY.md`.

**Edited:** `beta/socialization/visual-identity-and-the-progressive-depth-website.md` (Parts
1–2); `site/{build.py, resolver.py, test_resolver.py, README.md, .gitignore}`;
`.github/workflows/pages.yml`; `alpha/experiments/EXPERIMENT-BACKLOG.md` (§7 classroom row);
`alpha/experiments/CONSISTENCY-FINDINGS-2026-07.md` (FND-R13-1);
`beta/impl/experiments/drystone-reviews-and-experiments-log.md` (run entry).

**Not touched:** the specs, the registers' status tags, every crate — per the guardrails.
