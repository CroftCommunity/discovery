# RUN-WIKIBOOKS-CORPUS

Build a local CLI that pulls the en.wikibooks Cookbook recipe corpus into a structured
local representation, computes intelligent deltas across long gaps, and publishes the
result into a PDS as arecipe-consumable recipe records.

Target cadence: run now, run again in six months, and have the second run do the minimum
correct work.

---

## 0. Standing directives (non-negotiable)

**TDD, red first.** Every deliverable below is encoded as failing tests before any
implementation. Fixtures land before features. The run summary must show red-to-green
order per deliverable, with the failing output quoted. A deliverable with no failing
test in its history is not done.

**No network in the test suite** except the single live-grade smoke test in D12, which is
gated behind an environment variable and skipped by default.

**Declared stand-ins register.** Anything you fake, stub, or approximate goes in a
`STAND-INS.md` register with what it stands in for and what would make it real.

**Never silently drop content.** Any wikitext the transform does not understand is either
preserved verbatim or recorded in a per-recipe `parseFlags` array. Silent loss is a bug.

---

## 1. Homing and shape

**Owner decision O1 (answer before scaffolding):** new repo
`CroftCommunity/wikibooks-cookbook-sync`, or `tools/wikibooks/` inside
`CroftCommunity/arecipe`?

Recommendation: **new repo**. arecipe is a vanilla strict TypeScript, no-framework,
zero-runtime-dependency static bundle built by an allowlist copy in `scripts/build.mjs`.
This tool needs an atproto client, a wikitext parser, SQLite, and it holds publish
credentials. None of that belongs in the bundle's dependency graph. Default to the new
repo unless the owner says otherwise; if the answer is `tools/`, add a hard test that no
tool dependency can reach `src/` and that `build.mjs` output is byte-identical before and
after the tool lands.

Language and tooling: strict TypeScript, Node 20+, no framework. Test runner: `node:test`
plus `node:assert/strict` unless the repo already standardises elsewhere.

### Three stages, hard-separated

```
fetch  ──► raw/            (wikitext + metadata, exactly as retrieved)
           │
transform ──► ir/          (normalized recipe IR, pure function of raw/)
           │
publish  ──► PDS           (records + delta application)
```

The separation is the point. A parser improvement must be re-runnable against `raw/`
with zero wiki traffic. A publish retry must not re-transform. Enforce it: `transform`
has no network imports, and there is a test that asserts this by inspecting the module
graph.

---

## 2. What we know about the source (grounded, do not re-derive)

These are established facts. Use them; verify the ones marked VERIFY at runtime rather
than trusting them.

- **Membership is template-driven.** The `{{Recipe}}` template is what places a page into
  `Category:Recipes`. That category is the authoritative enumeration; the
  `Cookbook:Table_of_Contents` page is a hand-maintained index and will drift. Enumerate
  the category, not the ToC.

- **Recipes carry an infobox.** `{{Recipe summary}}` parameters, all optional:
  `category`, `servings`, `time`, `difficulty`, `image`, `energy`, `note`, plus `yield`,
  `cuisine`, `origin` in the documented examples. `difficulty` is documented as a number
  1 to 5. **Values are free text**: the template's own documentation shows
  `servings = 1-2`, `yield = 4 burgers`, `time = 30 minutes`,
  `energy = 100 Cal (400 kJ)`. Do not type these as numbers.

- **Ingredients are semi-structured.** Cookbook policy requires ingredients listed in the
  order the procedure calls for them, linked with the pipe trick: `[[Cookbook:carrot|]]`.
  Ingredient extraction is therefore a link parse, not an NLP problem. Treat the link
  target as the canonical ingredient token and the rendered line as the display string.

- **Scale.** The difficulty subcategories sum to roughly 3,600 pages (704 very easy,
  1,557 easy, 1,273 medium, 93 difficult). 802 recipes carry images. This is a small
  corpus; design for correctness, not throughput.

- **VERIFY: namespace id.** The Cookbook namespace is believed to be 102 on Wikibooks
  wikis. Resolve it at runtime via `action=query&meta=siteinfo&siprop=namespaces` and
  fail loudly if the resolved id differs from the cached one. Never hardcode.

- **VERIFY: category flatness.** `Category:Recipes` is described as automatically
  generated and complete. Confirm by comparing the enumerated page count against the sum
  of the `Category:Recipes by difficulty` subcategories. If the flat enumeration is
  materially smaller, fall back to recursive subcategory walk with a depth cap of 3 and a
  visited-set cycle guard, and report the discrepancy prominently in the run summary.

- **Do not build on XML dumps.** `dumps.wikimedia.org` now marks the XML dumps deprecated
  and points to MediaWiki Content File Exports; dumps are monthly. At this corpus size
  the Action API is the right instrument anyway.

---

## 3. Wiki etiquette (D1)

These are hard requirements, and they are testable against a fake transport. Do not test
them by hitting the wiki.

- **User-Agent:** `arecipe-wikibooks-sync/<version> (https://arecipe.app; <contact>)`.
  The contact string is configuration with no default. The tool refuses to start if it is
  unset. Wikimedia policy requires a meaningful, contactable User-Agent, and generic or
  absent agents are subject to blocking.
- **Concurrency 1.** The robot policy asks for total concurrency of at most 1 with at
  least a one second delay between requests. Honour that; it costs us minutes on a corpus
  this size.
- **`maxlag=5`** on every request. On a maxlag error, sleep and retry with exponential
  backoff.
- **Honour `Retry-After`** on HTTP 429. Rate limits are being deployed across Wikimedia
  APIs in 2026 and target unauthenticated automated traffic specifically.
- **`Accept-Encoding: gzip`.**
- **Pause on 5xx** for at least 15 minutes, per the robot policy.
- `format=json&formatversion=2` everywhere.

**D1 tests (red first):** a fake HTTP transport records every request. Assert the UA
string shape, the refusal to start without contact config, serial ordering with a
measured minimum gap, `maxlag` presence, backoff schedule on a synthetic 429 with
`Retry-After: 30`, and the 5xx pause. Assert the process exits non-zero and writes no
partial state when the UA contact is missing.

---

## 4. The ledger (D2)

SQLite at `state/corpus.db`. One row per recipe, **keyed by `pageid`, never by title.**
Page moves keep the pageid and change the title; keying by title creates phantom
delete-plus-create pairs on every rename.

Columns, minimum:

```
pageid            INTEGER PRIMARY KEY
title             TEXT      -- current wiki title
revid             INTEGER   -- last revision we fetched
rev_timestamp     TEXT
raw_sha256        TEXT      -- hash of stored wikitext
ir_sha256         TEXT      -- hash of canonicalised IR JSON
transform_version INTEGER   -- bumped when the parser changes semantics
status            TEXT      -- active | decategorised | deleted | skipped
skip_reason       TEXT
record_rkey       TEXT
record_cid        TEXT
published_at      TEXT
published_repo_rev TEXT     -- PDS commit rev at time of publish
first_seen        TEXT
last_seen         TEXT
```

Plus a `runs` table: run id, started, finished, mode, counts by outcome, wiki requests
made, PDS writes made.

**Two independent change axes.** This is the intelligence the brief asks for:

1. **Upstream change** — `revid` differs from the ledger. The wiki content moved.
2. **Local change** — `transform_version` differs, or re-transforming stored raw content
   produces a different `ir_sha256`. Our parser got better; the wiki did nothing.

Either axis triggers a republish. Only axis 1 triggers a wiki fetch. `--reparse` walks
axis 2 alone with the network disabled entirely, which is the mode you will use most
during parser work.

**D2 tests:** ledger round-trips; rename (same pageid, new title) produces an update and
zero deletes; a parser-version bump with unchanged wiki content produces republishes and
zero wiki requests; a re-run with nothing changed produces zero of everything.

---

## 5. Delta discovery across a six-month gap (D3)

**Do not use `list=recentchanges`.** `$wgRCMaxAge` is 30 days on Wikimedia wikis, so a
semiannual run would silently miss five months of edits. This is the single most
important design constraint in the run and it deserves a test that documents it: a test
named for the constraint, asserting the discovery path never calls the recentchanges
module.

The correct approach at this corpus size is full enumeration plus a batched revision
sweep.

**Step 1, enumerate.**
`action=query&list=categorymembers&cmtitle=Category:Recipes&cmtype=page&cmprop=ids|title&cmlimit=max`
with continuation. Regular clients get up to 500 results per list query; 5000 requires
the `apihighlimits` right, which we do not have. Expect roughly 8 requests.

**Step 2, revision sweep.**
`action=query&pageids=<up to 50>&prop=revisions&rvprop=ids|timestamp&rvslots=main`.
The multivalue cap is 50 titles or pageids per query for regular clients. Roughly 73
requests for the full corpus. Total discovery cost: under 100 requests, a couple of
minutes at one request per second.

**Step 3, classify** each pageid:

| Ledger | Enumeration | Classification | Action |
|---|---|---|---|
| absent | present | `new` | fetch, transform, create |
| present, revid differs | present | `changed` | fetch, transform, update |
| present, revid same | present | `unchanged` | nothing (unless `--reparse`) |
| present | absent | `vanished` | resolve, see below |

**Resolving `vanished`.** Absence from the category is ambiguous and the two causes need
different handling. Issue one follow-up `action=query&pageids=<id>&prop=info`:

- Page still exists → `decategorised`. The `{{Recipe}}` template was removed or the page
  was restructured. Default action: **retract the published record and mark the ledger
  row `decategorised`**, because the wiki no longer considers it a recipe. Flag every one
  of these individually in the run summary with its title, since a bulk decategorisation
  upstream would otherwise silently gut our corpus.
- Page is gone → `deleted`. Retract the record.

**Blast-radius guard (mandatory).** If `vanished` exceeds 5% of the ledger in a single
run, abort before any PDS write, write the plan file, and exit non-zero with a message
naming the count. A category rename or a template edit upstream must not be able to
delete thousands of our records unattended.

**D3 tests:** fixture enumerations covering each row of the table; the rename case; the
decategorised-versus-deleted split; the 5% guard tripping at 5.1% and not at 4.9%;
continuation handling across a synthetic three-page enumeration; a test asserting request
count stays under a declared budget for a 3,600-page corpus.

---

## 6. Fetch stage (D4)

For `new` and `changed` pageids only:
`action=query&pageids=<batch>&prop=revisions&rvprop=ids|timestamp|content&rvslots=main`.

Write to `raw/<pageid>.json`: pageid, title, revid, timestamp, wikitext, fetched-at,
and the exact request URL. `raw/` is content-addressable by revid and is never mutated,
only added to. Keep the previous revision's raw file; disk is free and diffing raw
wikitext across a six-month gap is the fastest way to debug a transform regression.

Resumability: the fetch stage writes a `runs/<runid>/progress.json` after every batch and
resumes from it. A killed run costs one batch, not the corpus.

**D4 tests:** batching respects the 50 cap; resume after a simulated crash mid-batch
re-fetches exactly one batch; raw files are never overwritten in place.

---

## 7. Transform stage (D5 to D8)

Pure. No network. No clock, except through an injected clock. Deterministic: same input
bytes produce byte-identical IR JSON, with sorted keys and a canonical serializer.

### D5 — infobox

Parse `{{Recipe summary}}` and its `{{recipesummary}}` alias, case-insensitively, with
whitespace-tolerant parameter names. Handle nested braces and pipes inside `[[...]]`
links, which a naive split on `|` will mangle. Emit:

```ts
type Summary = {
  category?: string;
  servings?: string;      // free text, e.g. "1-2"
  servingsHint?: { min: number; max?: number };   // parsed when unambiguous
  yield?: string;
  time?: string;          // free text, e.g. "30 minutes"
  timeMinutesHint?: number;
  difficulty?: 1|2|3|4|5;
  cuisine?: string;
  origin?: string;
  energy?: string;
  note?: string;
};
```

The `Hint` fields exist so the UI can sort and filter without ever losing the source
string. When a hint cannot be derived confidently, omit it. Never guess.

Difficulty out of range, non-numeric, or empty → omit the field and add a parseFlag.

### D6 — ingredients

Locate the ingredients section by heading, tolerating `== Ingredients ==`,
`===Ingredients===`, and plural or possessive variants. For each list item emit:

```ts
type IngredientLine = {
  raw: string;            // wikitext, untouched
  display: string;        // markup stripped, links rendered to their display text
  refs: string[];         // resolved Cookbook: link targets, e.g. "carrot"
  optional: boolean;      // detected from a leading "optional" marker only
};
```

Resolve the pipe trick: `[[Cookbook:Carrot|]]` renders as `Carrot` and refs to `Carrot`.
Handle `[[Cookbook:Carrot|carrots]]`, bare `[[Cookbook:Carrot]]`, and non-Cookbook links
(keep display, no ref).

**Conservative posture, matching RUN-SHOPPING-LIST:** no quantity parsing, no descriptor
folding, no unit normalisation in this run. Lines flow through as text plus refs. The
shopping-list parser already in arecipe is the consumer and it already handles
"as listed" attribution for unparsed lines.

### D7 — procedure and remaining sections

Procedure steps from the numbered list under the procedure heading. Preserve any
sub-lists as nested steps rather than flattening. Capture Notes, Tips, Variations, and
Warnings sections as named prose blocks. Strip: category links, ref tags and their
contents, navigation and stub templates, image syntax. Anything else unrecognised stays
in the text with a parseFlag naming it.

### D8 — completeness gate

A page is publishable only if it has at least one ingredient line **and** at least one
procedure step. Everything else is `skipped` with a reason. Half-recipes must never reach
a PDS. Count them, list them in the run summary, and expect the count to be non-trivial
on a wiki with stub recipes.

**D5 to D8 tests:** capture at least 25 real pages' wikitext as committed fixtures,
chosen deliberately to span the awkward cases: minimal stub, no infobox, infobox with
every parameter, nested templates in a parameter, ingredients with sub-lists, unusual
heading spellings, non-Latin characters in titles, an ingredient line with three links,
a page with a table in the procedure. Snapshot the IR for each. The snapshots are the
regression suite. Fixtures land and fail before any parser code exists.

---

## 8. Provenance and licence (D9)

Every published record carries provenance. This is not optional and it is not a footer.

```
sourceUrl        https://en.wikibooks.org/wiki/Cookbook:<Title>
sourcePermalink  https://en.wikibooks.org/w/index.php?oldid=<revid>
sourceRevId      <revid>
sourceHistoryUrl https://en.wikibooks.org/w/index.php?title=Cookbook:<Title>&action=history
retrievedAt      <ISO 8601>
license          <see O2>
```

`sourceUrl` already exists in the arecipe lexicon as provenance from RUN-RECIPE-IMPORT.
Reuse it rather than inventing a parallel field, and rebase against `LEXICONS.md` before
adding anything.

**Grounded facts for the owner, stated without legal conclusion**, consistent with the
agents-page posture of citing sources and making no legal claims in arecipe's voice:

- Wikimedia projects moved their default text licence to CC BY-SA 4.0 in June 2023;
  revisions predating that remain under CC BY-SA 3.0 (Creative Commons announcement,
  29 June 2023; Wikimedia Meta, Terms of use/Creative Commons 4.0).
- The Meta FAQ describes the requirements as unchanged in kind: appropriate credit, and
  distribution of remixes under the same licence.

**Owner decision O2:** what exact licence identifier and attribution string goes on
imported records, given that a current revision of a long-lived page is a composite of
edits from both licence eras. Do not resolve this in code. The run implements whatever
string the owner supplies, as configuration, and tests that it is present on every
record and that a missing configuration value blocks publish.

**Owner decision O3:** ShareAlike propagation. If a user edits an imported recipe and
republishes it into their own PDS, that derived record needs the same treatment. Options:
(a) block editing of imported records in the UI, (b) allow it and carry the provenance
plus licence fields forward onto the derivative, (c) fork-on-edit with an explicit
"based on" reference. This run only needs to know which, so the record shape supports it.
Recommendation: (c), because it keeps the user's own cookbook clean and the lineage
explicit, but it is the owner's call.

**Images: out of scope.** Commons files carry per-file licences that vary and must be
checked individually. Import no images in this run. Record `image` from the infobox as a
filename string only, with a flag noting it is unresolved. This matches the deferral
already taken in RUN-RECIPE-IMPORT.

---

## 9. Publish stage (D10, D11)

**Target:** a dedicated arecipe-owned PDS account that presents as a followable cook.
This reuses the cook-follows mechanism already shipped, keeps ShareAlike content out of
users' own repos, and gives the sync job exactly one write target.
**Owner decision O4:** the handle and hosting for that account.

### D10 — record mapping and idempotency

**rkey is deterministic:** `wb-<pageid>`. Not a TID. Determinism is what makes the
six-month rerun idempotent and what makes a rename an update instead of an orphan plus a
create. Add a test that asserts rkey stability across a title change.

Read `LEXICONS.md` first and map onto the existing recipe record shape. Recipes are
consumed from `exchange.recipe.recipe`, owned by recipe.exchange rather than arecipe, so
**do not extend that lexicon in this run.** Fields with no home go into the open-world
provenance area or wait for RUN-RECIPE-META-STRIP to resolve where servings and
difficulty live. Report the gap; do not paper over it.

### D11 — delta application

Batch writes through `com.atproto.repo.applyWrites` if the pinned SDK exposes it, and
verify that against the lexicon at implementation time rather than assuming; otherwise
sequential `putRecord` with backoff. Order: creates and updates first, retractions last,
so a mid-run failure never leaves the corpus with holes.

**`--dry-run` is the default.** Publishing requires an explicit `--publish` flag. Dry run
writes `runs/<runid>/plan.json` with every create, update and retraction, plus a
human-readable summary with counts and three sampled diffs. The owner reads the plan
before anything is written.

After a successful publish, record the resulting repo commit rev in `published_repo_rev`
and emit it to `runs/<runid>/summary.json`. RUN-BUNDLE-PRECACHE consumes that rev.

**D10 and D11 tests:** a fake PDS records writes; rkey determinism across renames;
retraction ordering; dry run performs zero writes and produces a plan whose contents
match the ledger diff exactly; a mid-run failure resumes without duplicating creates;
`--publish` without `--dry-run` having been run at least once in the same run id is
refused.

---

## 10. Live smoke test (D12)

One test, gated behind `WIKIBOOKS_LIVE=1`, skipped by default, that fetches exactly three
named pages from the real API with the real etiquette layer and asserts the transform
produces publishable IR. This is the only thing separating a stand-in-grade suite from
live grade. Register it in `STAND-INS.md` as the boundary.

Do not point the live test at a PDS. Publish stays dry.

---

## 11. Operator surface (D13)

```
wbsync discover                  # enumerate + revision sweep, write the delta plan
wbsync fetch                     # fetch new and changed only
wbsync transform [--reparse]     # network-free
wbsync plan                      # ledger diff -> plan.json, no writes
wbsync publish --publish         # apply the plan
wbsync run                       # all of the above, dry by default
wbsync status                    # ledger counts, last run, drift summary
```

Every command is resumable and safe to re-run. `wbsync run` with nothing changed upstream
must make roughly 80 wiki requests, zero fetches, zero transforms, zero PDS writes, and
say so. That outcome is the acceptance test for "smart about updates."

---

## 12. Acceptance

The run is done when:

1. Every deliverable D1 to D13 has a failing test in its history and a passing test now.
2. `wbsync run` twice in a row, with no upstream change, makes zero PDS writes on the
   second pass, and the run summary states the request count.
3. A simulated rename, a simulated decategorisation, a simulated deletion, and a
   simulated parser-version bump each produce the correct and distinct behaviour, proven
   by test.
4. The 5% blast-radius guard is proven to abort before any write.
5. `STAND-INS.md` is complete.
6. The run summary reports: corpus size, publishable count, skipped count with reasons
   grouped, parseFlag frequency table sorted by count, request counts for wiki and PDS,
   wall time, and the resulting repo rev.
7. Owner decisions O1 to O4 are restated at the top of the summary with their answers, or
   flagged as blocking.

## 13. Explicitly out of scope

Image import. Quantity and unit parsing. Translation or non-English Wikibooks. Writing
back to the wiki, ever. Any UI. The bundling and precache work, which is
RUN-BUNDLE-PRECACHE.
