# EXP-IMPORT-EXTRACTION — how much better can import get, and what actually gets it there

An experiment. Branch only. No CSP change, no dependency added to the shipped
bundle, nothing merged to `main` except a findings document and, if Arm 1 wins,
a follow-up run file.

## 0. Verified starting state

Read from the deployed build on 2026-07-22, not from memory:

The manifest declares `share_target` with `action: "./mine.html"`, `method:
"GET"`, `enctype: "application/x-www-form-urlencoded"`, and params `title`,
`text`, `url`. `mine.html` reads those from `location.search`, normalizes them,
calls `history.replaceState` to strip the query, then mounts the importer with
`acquireFromUrl` and `acquireFromPaste` handlers. On success it saves a draft
and redirects to `editor.html?draft=`.

So the share path is built and the human confirmation surface already exists.
The open question is purely about extraction quality.

## 1. The question, in the right order

Not "should we add a model". The order is:

1. What fraction of real import attempts fail today, and **why** does each one
   fail?
2. How much of that is closed by deterministic parsing the app does not yet do?
3. Is there a residual worth a desktop-only model?

Skipping straight to 3 means sizing a feature against an imagined gap.

## 2. Phase 0 — Instrument before improving

Assemble a corpus of **40 real recipe sources**, deliberately spread across:

- large recipe sites with clean JSON-LD
- personal food blogs, with and without JSON-LD
- sites using microdata or RDFa rather than JSON-LD
- sites using the h-recipe microformat
- a JS-rendered page whose recipe is not in the initial HTML
- a page behind a consent wall
- a non-English page
- plain pasted text from a printed cookbook
- plain pasted text from an email or message

Run each through the current `acquireFromUrl` and `acquireFromPaste` and record
a per-source row:

- Did the client-side fetch succeed, or fail, and on what: **CORS**, network,
  status code, or content type.
- Was structured data present, and in which format.
- Did the parser produce a draft.
- Per field (name, ingredients, instructions, yield, times, image, source),
  correct / partial / missing / wrong.

**The CORS count is the headline number of this phase.** With no backend, an
arbitrary third-party fetch from the browser is subject to that origin's CORS
policy, and most recipe sites do not opt in. Whatever fraction of the corpus
cannot be fetched at all is a permanent ceiling on the URL rung, unrelated to
parsing quality and unfixable without a proxy, which is a backend. If that
number is high, then the shared `text` param and the paste path are the real
import surface and everything downstream should be aimed there.

Record this before touching any parser.

## 3. Arm 1 — Harden the deterministic path

Zero bytes, works on every device, no desktop constraint. Candidates, each
measured for how many corpus rows it converts:

- JSON-LD nested inside `@graph`, or delivered as a top-level array, or with
  `@type` as an array containing `Recipe`.
- `recipeInstructions` arriving as `HowToStep` / `HowToSection` objects rather
  than strings, including nested sections.
- `recipeIngredient` vs the legacy `ingredients` key.
- ISO 8601 durations for `prepTime`, `cookTime`, `totalTime`.
- HTML entity decoding and stray markup inside extracted strings.
- Microdata and RDFa extraction.
- h-recipe microformat.
- `yield` in its several spellings and shapes.

Report the corpus conversion delta per item so the follow-up run file can be
ordered by value rather than by tidiness.

## 4. Arm 2 — Constrained model extraction on the residual

Only the rows Arm 1 still fails.

**Model.** Chrome's Prompt API, with `expectedInputs` text and a
`responseConstraint` JSON Schema pinning output to the recipe field shape.
Desktop-only by construction: Chrome's docs state Chrome for Android and iOS are
not supported. That is acceptable here, unlike the fridge-photo case, because
import is a sit-down task and the ladder degrades honestly. It is not acceptable
to describe it as anything other than a desktop assist.

**The invariant that makes this safe to ship at all:**

> Every extracted ingredient string and every extracted instruction string MUST
> appear verbatim in the source text. An extraction containing any string that
> does not is rejected **wholesale**, not partially accepted.

The model selects spans. It never writes prose. This is not a quality
preference, it is a provenance requirement: the agents-page posture is to cite
sources and make no claims over them, and a model that rewrites instruction text
has manufactured a derivative work and blurred exactly the line that posture
depends on. A parser that extracts has not.

Measure the **rejection rate**. A high rejection rate is not a bug to tune
around; it is the finding. It means the model is composing rather than
extracting, and the arm fails.

**Deterministic always wins.** If structured data parsed, the model never runs
and never overwrites a field that was found. It fills gaps only.

**The editor is the confirmation surface.** Model output arrives as a draft in
`editor.html` like any other import. Nothing is ever written to a record without
a human looking at it. No new UX is needed, which is unusual and worth noting in
the findings.

## 5. Metrics

Per arm, per field: precision, recall.

Per arm, overall: **usable draft rate**, defined as the fraction of sources
where a person accepts the draft with no edits or only trivial ones. This is the
number that decides anything. Field-level scores that do not move the usable
draft rate are noise.

Also for Arm 2: rejection rate under the verbatim invariant, cold model
availability check latency, per-source extraction latency, and how many machines
in the test set even satisfy the Prompt API's hardware requirements.

## 6. Kill criteria, fixed before results exist

- If Arm 1 closes most of the gap left after the CORS ceiling, **Arm 2 does not
  ship.** A desktop-only code path plus a model dependency is a real ongoing
  cost and needs a real remaining problem to justify it.
- If Arm 2's verbatim rejection rate exceeds 20%, Arm 2 fails. The model is
  writing, not selecting.
- If Arm 2's usable draft rate on the residual is below 50%, Arm 2 fails.
  A coin flip that still needs full editor review is not an assist.
- No result from Arm 2 may be used to argue for relaxing the verbatim
  invariant. If the invariant is what is holding quality down, the answer is
  that this approach does not work here.

## 7. Deliverables

`docs/EXP-IMPORT-EXTRACTION.md`: the 40-row corpus table with per-source
outcomes, the CORS ceiling number stated prominently, the per-item conversion
delta for Arm 1, Arm 2's full metrics including rejection rate, machine and
browser versions for every Arm 2 measurement, and a go/no-go per arm against
section 6.

If Arm 1 wins, also produce `RUN-IMPORT-HARDENING.md` in the house style,
ordered by measured conversion value, with the corpus committed as fixtures.

## 8. TDD note

Experiments are exempt from the shipping gate, with one exception, and here it
is the most important code in the file. Written test-first, all passing before a
single source is scored:

- **The verbatim validator.** Tests for exact match, whitespace-normalized
  match, a string absent from the source, a string that is a substring of a
  different field, empty extraction, and empty source. This function is the
  safety mechanism for the entire arm; if it is wrong, every Arm 2 number is
  meaningless.
- The per-field scorer: exact, partial, missing, wrong, and the normalization
  rules for each.
- The usable-draft-rate aggregator, including how a trivial edit is defined and
  counted.
