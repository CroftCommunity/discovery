# RUN-10 — Publish the specs (Pages, followable cross-references) + three briefs off the pile

`Branch: fresh off main, e.g. run-10-publish-and-briefs. Parallel-safe with RUN-09 by design: this
run adds ONLY new files (site generator, workflow, three briefs) and edits no spec, register, or
crate — rebase on main after RUN-09 merges, and the build's own ref-check verifies nothing moved.
Each part ships independently.`

## Part 1 — the spec site (GitHub Pages)

Goal: a reader can open the published specs in a browser and **follow every cross-reference as a
link** — within a document (§7.4.1), across the set ("Part 1 §2.5"), and into the companions.

### Hard requirements

1. **Sources stay canonical.** Zero edits to the markdown sources. All anchor generation and
   reference linking happens at build time. If a source seems to need an edit to render, that is a
   FINDING, not an edit.
2. **Stable anchors from section numbers.** Every heading gets an id derived from its section
   number (e.g. `#s7-6-2`), not its title text, so links survive title rewording. Appendices get
   `#appendix-b`-style ids; R-bullets (§7.2 R1–R7) get `#s7-2-r7`-style ids if the renderer can
   anchor list items, else they resolve to §7.2.
3. **Reference autolinking.** Every inline `§N[.N[.N]]` becomes a link to the anchor in the same
   document; every `Part 1 §…` / `Part 2 §…` links across documents; repo-path citations to
   published companions (EVIDENCE-MAP, the thinking notes) become relative links.
4. **The broken-ref gate.** A §-reference that resolves to no heading FAILS the build, listing the
   offenders. This makes the site build a permanent, free consistency guard on the corpus — every
   future run that breaks a cross-reference gets caught at push. (Known-good baseline: RUN-08
   Part 2 verified all Part 2 refs resolve, so the gate should pass on day one; if it doesn't,
   the discrepancy list is a FINDING.)
5. **Scope.** Published set: `beta/drystone-spec/` (Part 1, Part 2, conventions-and-decisions,
   open-threads, EVIDENCE-MAP, part-2-changelog, the dag-cbor companion, the bannered
   proposed-changes as historical) plus `alpha/thinking/` (labeled "Exploratory — design
   dialogues" in the nav, since Part 2 cites them by path). A landing index page names the two
   tiers and links the reading order (Part 1 → Part 2 → EVIDENCE-MAP). Nothing else is published.
6. **Tags render literally.** Status tags (`Verified`, `Modeled`, …) and `[gates-release]` markers
   are content, not chrome — render them as-is (monospace is fine; no reinterpretation, no icons
   that editorialize).
7. **Deploy.** GitHub Actions → Pages, on push to main; the same job runs the broken-ref gate on
   pull requests without deploying.

### Approach

- Choose tooling empirically and prefer boring: a single build script (any language already in the
  repo's toolchain) plus `actions/deploy-pages` beats a heavy framework unless the framework meets
  requirement 2–4 out of the box (mkdocs-material with a hook is acceptable if it demonstrably
  does). Pin versions.
- **TDD applies to the resolver:** unit tests first — a doc with in-doc refs, a cross-doc ref, a
  ref to a missing section (must fail with the offender named), an appendix ref, a ref inside a
  code span (must NOT be linkified). Red → green evidenced in the summary.
- In-run verification: build locally; report the stats (documents built, headings anchored,
  references found / resolved / skipped-in-code-spans, unresolved = 0), and spot-list five sample
  links (section → target) for the audit.
- README gains one line pointing at the published URL; MASTER-INDEX notes the site and the gate.

## Part 2 — brief: the §5.2 group-principal seam (Meadowcap/Willow × MLS)

Read-and-report only; no spec edits; no code.

1. Read Part 2 §5.2 (the group-principal open seam) and §7.1's Willow-shaped commitment; read the
   current Meadowcap and Willow specifications (fetch the current published versions; cite
   version/date).
2. Report to `beta/impl/drystone-design/group-principal-seam.md` (with a `Serves:` header):
   whether Meadowcap's communal-namespace construction can carry the Group principal as §5.2
   sketches it; how capability rotation composes with MLS epoch rotation under churn; the exact
   points of impedance (identifier formats, delegation depth, revocation semantics); and a
   recommended construction with its costs. Anything contradicting current Part 2 text is a
   FINDING, quoted both ways.
3. Backlog: add or update the §5.2 row with the brief as its reference and a next-experiment shape
   if the brief identifies one.

## Part 3 — brief: frozen-emitter integration (decision brief for the owner)

Read-and-report only. The RUN-08 residual: the welcome-over-iroh key-distribution sourcing is
proven but not wired into the frozen Proofs conformance emitter.

Report to `alpha/experiments/EMITTER-INTEGRATION-BRIEF.md`: the options with costs and risks —
(A) fork-and-extend the frozen emitter inside `alpha/Proofs/` (breaks the frozen notice; keeps one
suite); (B) a thin adapter crate outside Proofs that drives the frozen emitter and adds the
sourcing evidence alongside (preserves frozen; two artifacts to keep in sync); (C) defer to the
`[gates-release]` pass where the governance-resolution vectors join the suite anyway (zero work
now; the residual stands). Recommend one with reasoning; the decision is the owner's; no
implementation in this run.

## Part 4 — brief: croft-group L2 readiness

Read-and-report only. The reuse decision (croft-group L2–L5 build on the proven crates; reuse is a
condition of considered compatibility) is on record; the Layer-2 resolution-ACL design is parked.

Report to `alpha/experiments/CROFT-GROUP-L2-READINESS.md`: what L2 concretely requires (from the
croft-group plans and the L2–L5 definitions); which requirements the proven crates satisfy today
(with the EVIDENCE-MAP rows that say so); which genuinely wait on the parked Layer-2 design or the
I9 call; and the largest L2 slice buildable now without touching either. If that slice is
non-trivial, shape it as a backlog row (RED-able assertions included) — do not build it.

## Run-wide rules

- I9 firewall holds (Parts 2–4 are reads; nothing decides the trust tier).
- No edits to specs, registers, crates, or frozen records anywhere in this run; briefs and site
  files only. FIX/FINDING discipline for anything surprising, findings to
  CONSISTENCY-FINDINGS-2026-07.md.
- Pre-compliance: the three briefs carry `Serves:` headers; the site's build script carries its
  unit tests.
- Rebase on main after RUN-09 merges before this branch merges; re-run the build gate post-rebase.

## Output

`alpha/experiments/RUN-10-SUMMARY.md`: Part 1 — tooling chosen and why, resolver red → green, the
build stats and five sample links, the published URL once live; Parts 2–4 — one-paragraph verdicts
each with a pointer to the brief; any findings; files added.
