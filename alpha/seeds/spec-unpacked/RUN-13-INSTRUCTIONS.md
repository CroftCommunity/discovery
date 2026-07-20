# RUN-13 — The gradients on Pages: five tiers, the classroom scaffold, Mermaid, review packaging

`Branch: fresh off main, GATED ON RUN-12 MERGED (this run edits the site build and the
socialization doc, both of which RUN-12 touches or neighbors). Markdown + site tooling only — no
spec edits, no crate changes. Site gate green at every commit; TDD on the build additions. Each
part ships independently.`

## Part 1 — the five-tier gradients model (the doc)

Extend `beta/socialization/visual-identity-and-the-progressive-depth-website.md` from three tiers
to five, shallowest to deepest: **one-liner → elevator → over tea (one-pager) → classroom →
library**, preserving its existing whole-truth constraint and generalizing it as the invariant:
*the tiers differ in order, energy, and altitude — never in truth.*

Per-tier character block (question answered / perspective / energy / depth / failure modes):
1. **One-liner — the inscription.** "Should I turn my head?" The stranger in motion, zero granted
   seconds. Still, carved, one breath. Zero mechanism, pure identity. The only tier that travels
   without its author, so it must survive secondhand repetition by a non-understander; it may
   narrow the truth arbitrarily, never bend it. Failure modes: the lie of compression, the empty
   vessel, jargon. Test battery: the secondhand test, the hostile reading, the library check.
2. **Elevator — the witness.** "What is this?" Plain declarative whole-truth fact, 2–3 sentences,
   no mechanism, no hype. (Note that `elevator-pitch.md`'s existing one-liner register is hereby
   promoted to tier 1; cross-reference, don't duplicate.)
3. **Over tea — the friend.** "What would this be like for me?" The listener's own social life;
   one sustained metaphor per concept, seam marked; warm, unhurried, dialogic.
4. **Classroom — the guide.** "How does it work, and why must it?" Learner order
   (need-before-mechanism), patient and cumulative, full mechanism truth at library precision in
   narrative order; every claim ends in something you can run. A different *axis* from the
   library: a path, not a reference.
5. **Library — the reviewer.** "Is it true?" Normative order, the evidence ladder, primary
   sources; the reader drives; the text withstands. (This tier already exists: the specs, the
   site, EVIDENCE-MAP.)

Close the section with the **loop principle**: the tagline and the classroom's closing refrain are
designed as siblings — the shallowest and deepest tiers touch, so the one-liner is the sentence
the classroom earns.

## Part 2 — the tagline workshop (for review, NOT decided)

Add a `## The one-liner: candidates under test` subsection (same doc or a sibling page): the test
battery stated, then the candidate table — each row: candidate / source truth it compresses /
secondhand survival / hostile reading / library check. Seed rows (owner will pick; the run
declares no winner):
- "It proves what was said. Never who was right." (the razor)
- "Groups no one owns." (center-freedom)
- "Everyone keeps their own memory. The group still agrees." (the refrain)
- "Memory without a master." (center-freedom, mood-forward)
- "Disagreement is a fact, not a failure." (fork-not-verdict)
Mark the section `FOR REVIEW — owner selects; the elevator doc's one-liner register updates only
after selection.`

## Part 3 — the classroom scaffold

1. Create `alpha/classroom/` with `00-arc.md` (the arc file provided alongside these
   instructions: three acts, ten chapters, beat structure, refrain, escalation logic, production
   notes) and chapter skeletons `01-two-people.md` … `10-the-planet.md`. Each skeleton: the
   chapter's beats from the arc (NEED / STORY / DIAGRAM / PRECISE STATEMENT / PROVE-IT / REFRAIN)
   as headed placeholders; the PROVE-IT box pre-wired to real test names and EVIDENCE-MAP rows
   (verify each pointer resolves — dead pointer fails the run); prose bodies marked
   `DRAFT-PENDING (written in conversation, not by runs)`.
2. Chapters 01 and 05 carry the two seed Mermaid diagrams as fenced ```mermaid blocks (sources
   provided alongside).
3. Backlog: a classroom row (chapters drafted-in-conversation; scaffold landed RUN-13).

## Part 4 — Mermaid in the site + nav

1. The site build renders fenced ```mermaid blocks (pinned version; build-time pre-render or
   pinned client-side script — choose empirically, prefer whichever keeps the no-network-at-read
   property if feasible, and document the choice).
2. **The gate extends to diagrams:** a mermaid block that fails to parse fails the build. TDD:
   resolver-test additions first (valid block renders; invalid block fails naming the file; a
   mermaid block inside a code example is not double-processed).
3. Nav: a **Gradients** page (Parts 1–2's content) and a **Classroom** section (the arc + chapter
   skeletons, labeled draft), plus the landing page updated to present the five tiers as reading
   paths into the same truth.
4. Allowlist/anchor audit re-run; PR gate unchanged.

## Part 5 — review packaging

A short `REVIEW-2026-07.md` on the Gradients page (or beside it) listing exactly what's under
review and how to respond: the tagline candidate table; the tier characters; the arc's act
structure and chapter order; the refrain-tagline coupling. One line per question, no essays — the
review is read on Pages after deployment.

## Guardrails

- No spec, register, or crate edits; the socialization doc, classroom tree, and site tooling only.
- Frozen-record rule; verbatim anchors; FIX/FINDING for surprises; both the site gate and (for the
  build changes) the resolver test suite green at every commit.
- Deployment reminder recorded in the summary: if the one-time Pages setup (Settings → Pages →
  Source: GitHub Actions) hasn't been flipped, the review URL waits on it.

## Output

`alpha/experiments/RUN-13-SUMMARY.md`: per-part status; the Mermaid tooling choice and its
red → green; the PROVE-IT pointer verification list; the live review URL (or the pending-setup
note); files added.
