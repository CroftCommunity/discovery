# RUN-GUIDE-HELPER — ask a question, get sent to the exact place

Self-contained instruction file for Claude Code, repo `CroftCommunity/arecipe`.

`[verify-in-run]` items are probed in Phase 0 and recorded in the run summary
BEFORE code depends on them. Contradictions with stated context are FINDINGS.

**TDD is mandatory.** Every acceptance criterion is encoded as a failing test
before the implementation exists. Red output then green output recorded per
phase in the summary.

## 0. Mission

A question box on the user guide. The user types "can other people see my
recipes" and lands on the exact section that answers it, with the anchor already
scrolled into view.

**The deep link is the product. Any prose is decoration.**

That ordering decides every design question below. A helper that writes a
paragraph and does not tell you where to go has failed even if the paragraph is
correct, because the user cannot act on it or verify it. A helper that returns
the right anchor with no prose at all has succeeded.

## 1. The architecture, in three layers

Each layer ships independently and improves the one below it. **Layer A ships
alone and is the deliverable of this run.** B and C are specified here so they
cannot be bolted on wrongly later, and are gated on Layer A's measured
performance.

- **Layer A — deterministic retrieval.** Build-time section index plus lexical
  search. No model, no download, works on every device including the ones that
  will never run a model.
- **Layer B — curated question phrasings.** Hand-written alternate phrasings per
  section, folded into the same lexical index. Nearly free, and on a bounded
  help corpus this routinely beats semantic search.
- **Layer C — model assist.** Query embedding for semantic recall, and
  optionally a generative summary. Constrained hard. Never required.

## 2. Locked design decisions

**D1 — The index is generated from the guide, never hand-maintained.** A build
step walks the user guide source, and for each section emits:

```ts
interface GuideSection {
  anchor: string;      // stable id, must exist in the rendered guide
  title: string;
  breadcrumb: string[];   // parent headings, for display
  text: string;           // section body, plain text
  phrasings: string[];    // Layer B, empty in Layer A
}
```

If the index is authored separately from the guide it will drift, and a help
system that confidently points at a section that no longer exists is worse than
no help system. `[verify-in-run]` the guide's source format and heading anchor
scheme.

**D2 — Anchor validity is enforced at build time.** The build fails if any
emitted `anchor` is not present in the rendered guide HTML. This is a hard gate,
not a warning.

**D3 — Results are links, always.** Each result renders as: breadcrumb, section
title, a one-line excerpt, and a link to `guide.html#anchor`. Clicking scrolls
the target into view and visibly highlights it, so the user can see they landed
in the right place rather than guessing.

**D4 — No confident answer without a link.** If nothing scores above the
threshold, the helper says no section covers the question and offers the guide's
table of contents. It does **not** improvise. This is a testable invariant, not
a tone preference.

**D5 — Layer C may only cite anchors that exist.** Every anchor a model returns
is validated against the known anchor set. An answer containing an unknown
anchor is rejected wholesale and the helper falls back to Layer A results.

Same shape as the verbatim invariant in EXP-IMPORT-EXTRACTION, deliberately. The
model is allowed to rank and to summarize retrieved text. It is not allowed to
invent a destination.

**D6 — Layer C is additive and optional.** With no model available, the helper
is Layer A plus B and says nothing about a missing model. No "AI unavailable"
copy. Users on unsupported devices should not learn that they are missing
something; they should just get working help.

**D7 — Ordering in the UI is link first.** If a generative summary is present it
renders **below** the ranked links, never above, and is visibly marked as a
summary of the linked sections. This survives the summary being wrong: the user
has already seen where to go.

**D8 — No telemetry.** Do not log queries anywhere, do not persist them beyond
the session, do not send them off device under any circumstance. Help queries
are a confession of what someone does not understand, and the posture here is
that nothing leaves the device.

## 3. Phases

### Phase 0 — Re-ground (no code)

Record: the user guide's source and rendered locations, its heading and anchor
convention, whether anchors are currently stable across rebuilds, how MiniSearch
is currently configured on Browse and Cookbook and whether that config is
reusable, how `scripts/build.mjs` registers generated assets, and where the
question box should sit on the guide page.

**If guide anchors are not currently stable across rebuilds, fixing that is the
first task of Phase 2**, because every link this feature produces depends on it.

### Phase 1 (RED) — tests first

Index build:

1. A fixture guide with three sections produces three index entries with the
   correct anchors and breadcrumbs.
2. Nested headings produce correct breadcrumb chains.
3. Section text excludes child section text (no double counting).
4. The build fails when an emitted anchor is absent from the rendered HTML.
5. Building twice from identical input produces byte-identical output.

Retrieval:

6. A fixture question set of 25 questions, each with a hand-marked correct
   section, yields the correct section at rank 1 for a recorded baseline count,
   and within the top 3 for a higher recorded count. These two numbers are the
   feature's measured quality and go in the summary.
7. A question with no relevant section returns no results above threshold.
8. An empty query returns no results and does not throw.
9. Results are stable for the same query and index (no nondeterministic tie
   ordering).

Invariants:

10. Every returned result carries a non-empty `anchor` present in the index.
11. Below threshold, the helper returns the no-match state and the no-match
    state contains a link to the table of contents.
12. Given a simulated Layer C response citing an unknown anchor, the response is
    rejected and Layer A results are returned instead.

E2E:

13. Typing a fixture question and submitting shows ranked results with visible
    breadcrumbs and links.
14. Clicking a result navigates to `guide.html#anchor` and the target section is
    scrolled into view and highlighted.
15. With no model available, the helper works and no copy mentions a model.

### Phase 2 (GREEN) — implement Layer A

Order: anchor stability if needed, then the build-time index generator, then the
retrieval module, then the question box UI, then the highlight-on-arrival
behavior.

### Phase 3 — Layer B

Add `phrasings` to the index for every section: three to six real questions a
person would actually type, written by a person, not generated. Re-run the
fixture question set and record the delta against Phase 2's numbers.

**Gate:** if Layer B moves top-1 accuracy to a level the owner considers
sufficient, **stop here and do not build Layer C.** Record the decision. A help
system that needs no model on any device is the better outcome, not a lesser
one.

### Phase 4 — Layer C (only if gated in)

Precompute section embeddings at build time so the shipped artifact is a small
vector file and only the query needs a model at runtime. Fuse with the lexical
score; the fusion rule is derived from the fixture set, not guessed. Generative
summary is a separate sub-phase and is the last thing built, if at all, under
D5 and D7.

### Phase 5 — Gate and summary

`npm test` fully green. `RUN-GUIDE-HELPER-SUMMARY.md` records Phase 0 findings,
red then green output per phase, the top-1 and top-3 accuracy numbers after
Phase 2 and again after Phase 3, the Layer C gate decision with reasoning, files
touched, and a grep confirming no query is logged or transmitted.

## 4. Acceptance criteria

1. The index is generated from the guide and the build fails on an invalid
   anchor (1 to 4).
2. The index build is deterministic (5).
3. Retrieval quality is measured, not asserted, against a committed 25-question
   fixture set (6).
4. Every result is a working deep link that lands and highlights (10, 13, 14).
5. No-match is an explicit state with a route onward, never an improvised answer
   (7, 11).
6. A model that cites an unknown anchor is rejected wholesale (12).
7. Everything works with no model and says nothing about it (15).
8. No query leaves the device or is persisted (grep in the summary).

## 5. Deliberately out of scope

- Answering questions about the user's own data ("when did I last plan X"). That
  is a different feature with a different data path and different privacy
  properties. The guide helper answers questions about the app.
- Conversational follow-up or session memory.
- Feedback widgets, thumbs up/down, or anything that would require logging
  queries.
- Editing the guide's content. This run indexes what exists; gaps it exposes are
  recorded as findings for a separate writing pass.
