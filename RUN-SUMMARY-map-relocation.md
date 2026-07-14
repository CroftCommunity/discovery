# Run summary: codify and land the map-at-the-back convention

`Date: 2026-07-14. Branch: claude/map-at-back-convention-vztuuc (off main).`

One purpose: the convention now says document-scale maps live at the **back**
with a one-line head pointer for agent notice, the doc method guide states that
rule, and every document-scale map in the live spec set (Part 1, Part 2) actually
does it. Other document-scale head maps elsewhere in the repo are listed below
for a later run, not touched.

## What was found

- **Method guide (sole definer of the convention):** `beta/impl/doc-writing-method.md`.
  It defines the map convention as **Rule 15** ("Carry an annotated section map...").
  Part 2 §11's Rule 15 note and the fold checklist item (iv) both cite it as
  "Rule 15". Exactly one live doc defines it, so the first two stop rules did not fire.
  The method guide has **no** document-scale `## 0. Map` at its own head, so Task 1
  step 4 (apply Task 2 to the guide's own map) was a no-op.

- **Live spec parts:** `beta/drystone-spec/part-1-reasoning-underpinnings.md` and
  `beta/drystone-spec/part-2-certifiable-design.md`, each carrying a document-scale
  `## 0. Map` at the head (Part 1 line 16, Part 2 line 7) and **no** `Map:` pointer,
  so neither was already conformant; both needed relocation.

- **Section-internal map:** Part 2 §11 carries a `### 0. Map` (line ~1924). Left in
  place per the invariant; only document-scale maps relocate.

### Current Rule 15, quoted verbatim before editing (Task 1 step 2)

> ## 15. Carry an annotated section map at the top of each part
>
> Each spec part opens with a **section map**: one line per top-level section and per major subsection, each snippet stating what the section covers, what it depends on, and what it is orthogonal to. The map is maintained as sections are added, so it always reflects the current structure.
>
> **Why.** A part large enough to be useful is too large to hold in the head, and a reader, a later fold, or a discussion returning to the doc needs to find the one section that bears on a question without reading the whole part. A bare table of contents gives the titles; the annotated map gives the *shape*: which sections are load-bearing for which, and, crucially, which concerns are orthogonal. Naming orthogonality is the point a plain contents list misses: a section can belong in a part yet be independent of most of it (scaling analysis and conflict-handling both live under governance, yet a change to one rarely touches the other), and stating that lets a reader or a fold pull the relevant section without dragging in the unrelated ones. The map is therefore both an index and a dependency sketch, and it makes the part's internal seams legible.
>
> **Form.** A `## 0. Map` block near the top, after the header and legend and before the first section. One entry per top-level section, with major subsections nested, each entry a phrase for scope plus, where it matters, a `depends on:` note and an `orthogonal to:` note pointing by section number. Keep each entry to a line or two; the body carries the detail, the map carries the shape. The map is a maintained artifact, not a one-time index: any fold or edit that adds, moves, removes, or repurposes a section, or that changes a section's scope, dependencies, or orthogonality, **MUST** update the affected map entry in the same pass, and a fold or edit that leaves the map stale has not completed. This duty is binding on folds and editors specifically, because a stale map is worse than no map: it silently misdirects the next reader or fold to the wrong section, or hides a dependency it claims to index, and a wrong index is trusted where an absent one at least prompts a search. This is the maintained-in-place discipline Rule 13 draws for the conventions reference and the changelog, applied to structure.
>
> **Why it earns its place.** This is the navigational analog of the conventions reference (Rule 13): where that makes vocabulary and decisions checkable across the suite, the section map makes *structure* checkable within a part, and a gap or a stale entry in the map is a gap or a drift in the part's organization, visible at a glance. It is also what keeps interlinking honest: every mechanism naming the section it depends on produces a web of cross-references, and the map is the index over that web, so the two together let a reader traverse the part by dependency rather than by page order.

The drop-in was **merged, not replaced blind**: the existing Why paragraph, the
one-line-per-section phrasing, the maintenance MUST-duty, and the "Why it earns its
place" paragraph were all preserved. Only the placement guidance changed (head -> back
plus head pointer), and the three drop-in paragraphs (back placement, name stability,
section-internal carve-out) were added. No obligations were dropped, so the wording
stop rule did not fire.

## What was edited

### Task 1 (commit: "Task 1: amend Rule 15...")

`beta/impl/doc-writing-method.md`, Rule 15:

- Title: "...at the **top** of each part" -> "...at the **back** of each part".
- Opening: now states the map is `## 0. Map`, lives at the back with the index and
  reference matter, and the head carries a one-line `` `Map: ...` `` pointer for agent
  notice; adds the name-stability and section-internal-map paragraphs from the drop-in.
- Form: the placement sentence now reads "A `## 0. Map` block at the back of the
  document, ... after the last appendix or reference section, and a one-line
  `` `Map: ...` `` pointer in the front-matter meta block naming where it sits." The
  maintenance duty and everything else in Form are unchanged.
- Rule number (15) and (no) status tag preserved. No em-dashes introduced.

### Task 2 (commit: "Task 2: relocate the document-scale map to the back...")

Per file: cut the `## 0. Map` block from the head, added a `Map:` pointer to the
front-matter meta block, appended the map (with one amended preamble clause) at the
very end after `---`, leaving exactly one `---` between front matter and `## 1.`.

- **Part 1** (`part-1-reasoning-underpinnings.md`): pointer added after the `Defines:`
  meta line, text ending "...lives at the end of this part, **after the upstream
  reference links**; maintained as sections change." Map appended after the
  "Upstream reference links" section. Part 1 had a `---` both before and after the
  head map, so the two were collapsed to one.
- **Part 2** (`part-2-certifiable-design.md`): pointer added after the `Resolution:`
  front-matter meta line (the first occurrence; a second `` `Resolution: `` meta line
  exists deeper in the body at line ~1892 and was deliberately not targeted), text
  ending "...lives at the end of this part, **after Appendix F**; maintained as
  sections change." Map appended after Appendix F. Part 2 had no `---` before the
  head map and one after; the existing one was preserved, so no separator was added.

Preamble clause amended in both maps: the first `, maintained as sections change.`
became `, maintained as sections change; kept here at the back and pointed to from the
front matter, so the opening page stays prose.` Nothing else in either map body changed.

Untouched, as required: Part 2's status line (its "§0 map" mentions remain true), Part 2
§11's internal `### 0. Map` and its Rule 15 note, and all section/normative content.

## What was skipped and why

- **Method guide's own head map:** none exists, so Task 1 step 4 was a no-op.

- **Other document-scale head maps (stop rule: list, do not edit this run).** Three
  frozen seed-corpus copies carry a document-scale `## 0. Map` at the head:
  - `alpha/seeds/p10-p11-corpus/p10-full-part1-principles.md` (line 16)
  - `alpha/seeds/p10-p11-corpus/p10-full-part2-mechanics.md` (line 51)
  - `alpha/seeds/p10-p11-corpus/p11-full-part2-mechanics.md` (line 7)

  These are historical seed snapshots (the p10/p11 corpus), out of scope for this
  documentation change, and are left as-is. They are recorded here for a later run.

- No stop rule halted either task. All anchors (`## 0. Map`, the `---` before `## 1.`,
  the meta block) matched the shapes described in the brief.

## Verification checklist (per edited file)

### Part 1 (`part-1-reasoning-underpinnings.md`)

- `grep -c '^## 0. Map'` = **1**, at line 1222 of 1252 (tail, after the references). PASS
- Exactly **one** `` `Map: `` pointer line, in the front matter before §1. PASS
- Relocated map body **byte-identical** to the cut block except the one preamble
  clause (unified diff of cut vs appended shows only that one line changed). PASS
- Per-file diff shows only the cut (head), the pointer, and the appended block; two
  head `---` collapsed to one. PASS
- No em-dashes introduced (0 on added lines). PASS

### Part 2 (`part-2-certifiable-design.md`)

- `grep -c '^## 0. Map'` = **1**, at line 2987 of 3035 (tail, after Appendix F). PASS
- Internal `### 0. Map` still present (§11), count 1. PASS
- Exactly **one** `` `Map: `` pointer line, in the front matter before §1. PASS
- Relocated map body **byte-identical** to the cut block except the one preamble
  clause (unified diff shows only that one line changed). PASS
- Per-file diff shows only the cut (head), the pointer, and the appended block; the
  pre-existing `---` before `## 1.` preserved, none added. PASS
- No em-dashes introduced: net em-dash change = 0 (the one `—` in the Appendix E map
  entry is pre-existing content relocated verbatim, removed at head and re-added at
  tail). PASS
- Status line's "the §0 map matches the section structure" mention intact. PASS

### Method guide (`beta/impl/doc-writing-method.md`)

- Rule 15 title now "at the back of each part"; number and structure preserved. PASS
- No em-dashes introduced (0 on added lines). PASS

### Repo-wide

- `git diff --stat` (working tree, before commits) showed only the three intended
  files. PASS
