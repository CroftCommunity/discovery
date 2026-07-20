# RUN-05 — Full consistency, clarity, and correctness pass (specs at the center)

`Branch: fresh off main, e.g. run-05-consistency-pass. Phases 0–3 are markdown; Phase 4 is
comment-only code (no behavior change). Assumes RUN-04 is merged.`

## Posture

The two specs (`beta/drystone-spec/part-1-*.md`, `part-2-certifiable-design.md`) are the center of
gravity: where documents disagree, the spec's latest text wins unless a register or RUN summary
proves the spec stale, in which case the spec is corrected *to the evidence*. Thinking docs
(`alpha/thinking/*.md`) are exploratory by their own banners and yield to spec; registers and RUN
summaries are evidence, not prose to polish.

Two classes of action, kept strictly apart:
- **FIX**: mechanical, provable, meaning-preserving. Apply directly.
- **FINDING**: anything requiring a judgment call or changing meaning. Record in the findings file
  (T-OUT below), do not edit.

Never rewrite prose for style. Never reflow untouched paragraphs. Never delete a document: stale
operational docs get a superseded banner, nothing more.

## Active-document set

Specs: `part-1-reasoning-underpinnings.md`, `part-2-certifiable-design.md`,
`conventions-and-decisions.md`, `open-threads.md`, `part-2-changelog.md`,
`proposed-changes-2026-07-experiment-reconciliation.md`, the dag-cbor companion.
Registers: `alpha/experiments/{MASTER-INDEX,EXPERIMENT-BACKLOG,SPEC-DIVERGENCE-REGISTER,SPEC-ALIGNMENT-AND-ACTION-PLAN}.md`.
Runs: `alpha/experiments/RUN-0*-SUMMARY.md`, `NEXT-RUN-INSTRUCTIONS.md`, root `RUN-SUMMARY-*.md`.
Thinking: `alpha/thinking/{the-shape-of-disagreement,reconciliation-horizon,corroboration-and-quantified-trust}.md`.
Log: `beta/impl/experiments/drystone-reviews-and-experiments-log.md`. Design briefs:
`beta/impl/drystone-design/*.md` (check references only; do not edit content).

---

## Phase 0 — verify the RUN-04 landing (FINDINGS only, severity HIGH on any miss)

Confirm, against `RUN-04-INSTRUCTIONS`' verbatim texts:
1. `alpha/thinking/corroboration-and-quantified-trust.md` exists with banner + sections 0–6.
2. The **corroboration-dials** paragraph sits immediately after the Part 2 paragraph beginning
   "The load-bearing caveat, stated rather than claimed away", verbatim, tagged `Design`.
3. The **formula-valued threshold** paragraph closes §7.4.1 (immediately before the §7.4.2
   heading), verbatim, tagged `Design.`
4. **EXP-C1** sits beside EXP-H1 in the backlog with its four assertions and the §8.2(e) note.
5. Map, `part-2-changelog.md`, and reviews-log entries for the RUN-04 edits exist.

## Phase 1 — mechanical integrity (FIX)

1.1 **Cross-reference resolution.** Every `§N`, `§N.N`, `§N.N.N`, `R1`–`R7`, and `Appendix A–F`
    reference inside Part 2 resolves to an existing heading/bullet; every `Part 1 §…` citation from
    Part 2 resolves to an existing Part 1 heading. Fix numbering drift; FINDING if a reference's
    *target content* doesn't support the citing sentence (that's Phase 3.4).
1.2 **§0 Map ↔ structure.** Every Part 2 section appears in the back Map with an accurate line,
    including the RUN-02/03/04 additions (§7.2 R7; §7.3.2 F8 note; §7.6.2 continuity; §7.6.9
    horizon example; §7.4.1 formula paragraph; the corroboration-dials paragraph's section; the
    Appendix B manifest clause). Fix.
1.3 **Path references.** Every repo path cited anywhere in the active set exists
    (`alpha/thinking/*.md` names, test filenames, register names). Fix or FINDING if intent unclear.
1.4 **Changelog completeness.** `part-2-changelog.md` has an entry for every Part 2 edit in the
    RUN-02/03/04 summaries' file lists. Add missing entries in house style.
1.5 **Log completeness.** The reviews-and-experiments log has entries for: the 2026-07
    reconciliation landing (RUN-02), continuity + horizon (RUN-03), corroboration dials (RUN-04).
    Add any missing.
1.6 **Typos.** Spelling and obvious grammatical slips across the active set; meaning-preserving
    only; list every one in the summary.
1.7 **Part 1 internal integrity.** Every internal cross-reference within Part 1 resolves to an
    existing Part 1 heading; if Part 1 carries its own map or section index, it matches the actual
    structure (Part 1 was edited by the designated-expert pass, so do not assume it is pristine).
    FIX for resolution/structure; any Part 1 *content* issue is a FINDING — the no-edit rule on
    Part 1 body holds.

## Phase 2 — status and evidence coherence (FIX only where a register/summary proves it)

2.1 **Two named staleness fixes (apply):** RUN-03 Phase B ran the two-competing-quorums experiment
    in the same merge that landed the sentences claiming it hadn't run.
    a. §7.3.2, the F8 paragraph's closing "…`Design`, decided; the fold's behavior carries no
       evidence tag until the two-competing-quorums experiment runs." → replace the clause after
       "decided" with: "and now test-run: the fold hard-stops with the order-independent
       `contradiction:{byte-head}` and the rule retains its pre-conflict value (RED→GREEN,
       `two_competing_rulechange_quorums`, RUN-03). `Modeled.`"
    b. The §7.2 R7 residuals paragraph's second residual, same claim → same upgrade, keeping the
       residual's first half (the role-authorship gate) untouched.
2.2 **Tag audit.** Every status tag in §7.2's R-series, §7.3, §7.6, §8.2, §10 is consistent with
    `SPEC-DIVERGENCE-REGISTER` and the RUN summaries: F1 stays `Modeled` (X3 automated harness
    still pending) everywhere it's tagged; no passage claims relay/real-NAT evidence (§8.2(a)
    loopback caveat intact wherever live-transport convergence is cited); the fan-out claim carries
    loopback grade only. FINDING for any tag whose correct value is arguable.
2.3 **Register ↔ spec.** `hermetic-gossip` is the only Active row; every Reconciled row's
    "spec landing" pointer resolves. Fix pointers; FINDING for substance.
2.4 **Alignment doc.** §7 decisions annotations still match reality post RUN-03/04 (decision 2
    executed; 5 unchanged). Fix stale annotations.
2.5 **Living-document currency (metadata and roadmap).** Governing distinction: a **living**
    document (MASTER-INDEX, EXPERIMENT-BACKLOG, SPEC-DIVERGENCE-REGISTER, open-threads, any
    README/layout index) is brought current as a FIX, provable against the RUN summaries and
    registers; a **frozen** record (RUN-0N-SUMMARY, X3-CROSS-PACKAGE-SWEEP, FANOUT-M1, captured
    thinking-doc dialogues) is never rewritten — at most a one-line banner if its claims now
    mislead. Each living fact should have exactly one home; duplicates elsewhere become pointers.
    a. **MASTER-INDEX** reflects the state through RUN-04: current file inventory including the
       three thinking notes and all RUN summaries; current evidence picture (hermetic-gossip the
       only Active divergence row; F8 landed and test-run; R7 `Modeled` pending the automated X3
       harness; fan-out and X3 at their loopback/substrate grades); any "pressing edges" narrative
       matches the register, not an earlier snapshot.
    b. **EXPERIMENT-BACKLOG is the single roadmap.** Every item carries a current status
       (open / resolved-with-landing-run / superseded-with-pointer). Mark done: EXP-1 (fan-out M1,
       RUN-01), EXP-4 (probe + RUN-03 Phase B fix). Update the recommended execution order to the
       current queue: X3 automated cross-package harness; EXP-H1; EXP-C1; freshness/quiescence over
       live transport (§7.4.2, loopback grade); MLS welcome-over-iroh wired into conformance
       emission (cats 7/8/9 half one); BIP39 paper-recovery round-trip; B1 then A5 message
       continuity; meer P2–P6; X1 parked pending hardware. FINDING if any existing order text
       encodes a rationale this replacement would lose.
    c. **SPEC-ALIGNMENT-AND-ACTION-PLAN** is a point-in-time reconciliation, not a second roadmap.
       First migrate (FIX, listed) any still-open item whose *only* home is that doc into the
       backlog; then banner it: "Point-in-time reconciliation (2026-07-13); decisions recorded in
       §7 remain the record of those calls; current roadmap: EXPERIMENT-BACKLOG.md; current
       evidence: SPEC-DIVERGENCE-REGISTER.md." Do not maintain it forward.
    d. Any repo/README layout description matches the actual tree (paths, folder purposes, the
       RUN-instruction workflow). FIX.

## Phase 3 — semantic consistency (FINDINGS unless trivially safe)

3.1 **New-paragraph coherence.** Read each RUN-02/03/04 insert against its immediate neighbors and
    the sections it cites: corroboration-dials ¶ vs the beam-caveat ¶ (must sharpen, never read as
    closing the beam); formula ¶ vs §7.4's threshold prose and §5.7's dial; horizon ¶ vs §7.3.3
    checkpoint semantics and §7.4.3's stamp; continuity ¶ vs §7.6.2's arity bullets and §5.10/§5.11.
    FINDING for any tension, with both sentences quoted.
3.2 **Terminology uniformity.** One term per concept on the spec side: "contradiction byte-head",
    "horizon checkpoint", "horizon-checkpoint manifest", "approval subject", "corroboration dials",
    "quantified trust". Thinking-doc vocabulary (`ForkDescriptor`, "Layer 1"/"Layer 2",
    projection-verb names) must not appear in Part 2 — FINDING for any leak. FIX pure synonym drift
    (same concept, two spellings) toward the spec's first-introduced form.
3.3 **DR-block compliance.** Every passage added since RUN-02 checked against the DR language rules
    in `conventions-and-decisions.md` (continuity-framed, non-moral). FINDING per violation.
3.4 **Citation support.** For each Part 1 citation in the new passages (§2.0, §2.0.1, §2.5), read
    the cited Part 1 text and confirm it supports the claim as used. FINDING per mismatch.
3.5 **Proposed-changes doc closure.** Every F1–F8 in
    `proposed-changes-2026-07-experiment-reconciliation.md` is marked called/applied and nothing in
    it contradicts what actually landed; if it lacks one, add a top banner: "Historical record —
    all items landed (RUN-02, RUN-03). The authoritative text is Part 2." FINDING for any
    landed-vs-staged text divergence.
3.6 **Stale operational docs.** `NEXT-RUN-INSTRUCTIONS.md` and any root `RUN-SUMMARY-*.md`
    superseded by the numbered runs get a one-line superseded banner pointing at the current
    sequence. No deletions.
3.7 **Part 1 → Part 2 direction.** For each Part 1 passage that characterizes a mechanism Part 2
    has since refined — policy/rule change and quorum, contradiction handling and escalation,
    recovery, freshness/currency — confirm Part 1's characterization remains compatible with the
    landed Part 2 text (compatible means: nothing in Part 1 asserts a behavior Part 2 now
    contradicts; abstraction-level differences are fine). FINDING per tension, both passages
    quoted; never edit Part 1 body.
3.8 **Conventions and open-threads coherence.** Every DR-block term and defined convention in
    `conventions-and-decisions.md` is consistent with current spec usage, and spec-side terms
    introduced since RUN-02 ("approval subject", "contradiction byte-head", "horizon checkpoint",
    "horizon-checkpoint manifest", "corroboration dials") that plausibly belong in the shared
    vocabulary get a FINDING proposing (not making) the addition. `open-threads.md` entries are
    consistent with the landed state — in particular the bannered recovery item versus the
    corrected alignment-doc framing, and any Stage/experiment references versus the RUN summaries.
    FIX for dead pointers; FINDING for substance.

## Phase 4 — queued code comments (comment-only; separate commit)

At `local_storage_projection/src/fold_derived.rs::detect_competing_rulechange`, extend the guard
comment; mirror the addition at the shared positively-established-concurrency guard used by the
mutual-expulsion path. Text (adapt line-wrapping only):

    // Concurrency must be positively established: a RuleChange with empty antecedents makes no
    // causal claim, so bare re-sets never contradict and fold as sequential amendments in
    // canonical (merge_cmp) order. Consequence, deliberate: a threshold-1 rule can flap between
    // concurrent setters deterministically but without a contradiction banner. Every quorum-met
    // change carries its approvals as antecedents (Part 2 §7.2 R7), so the F8 marquee case always
    // trips this predicate. If the silent flap ever proves socially wrong for a Group, the
    // remedies are a Part 2 note or raising that rule's threshold. Decided knowingly (RUN-03
    // audit, 2026-07-14).

No behavior change; both suites and clippy must stay green with zero new warnings vs baseline.

---

## Guardrails

- Verbatim-anchor rule throughout: anchor not found exactly → skip, record, continue.
- FIX vs FINDING discipline is the run's core contract; when in doubt, FINDING.
- No edits to Part 1 body, `conventions-and-decisions.md`, or thinking-doc content (banners and
  path fixes excepted); no code beyond Phase 4's comments.

## Output

1. `alpha/experiments/RUN-05-SUMMARY.md`: per-phase status, every FIX applied (file, line,
   before→after for one-liners), Phase 4 test/clippy result.
2. `alpha/experiments/CONSISTENCY-FINDINGS-2026-07.md`: three sections — **Fixed mechanically**
   (the full FIX list), **Needs an owner call** (each FINDING with severity HIGH/MED/LOW, the
   quoted text, and a proposed resolution), **Verified clean** (each check that passed, stated, so
   silence is never ambiguous).
