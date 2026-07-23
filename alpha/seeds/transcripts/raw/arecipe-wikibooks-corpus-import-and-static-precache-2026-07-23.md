# Raw: arecipe — Wikibooks recipe-corpus import & static-list precache (2026-07-23)

**Preservation status: preserved-condensed (cleaned-paste, content-faithful — NOT byte-pristine) —
PLAYBOOK §4.** Source: a pasted Claude chat session, 2026-07-23. UI render chrome stripped (thinking-summary
headers, "Document · MD" markers, inline citation chips folded into the Sources note, "Show more"). No
secrets. Part of the 2026-07-23 batch (plan:
`alpha/plans/2026-07-23-transcript-intake-socialization-arecipe-measurement.md`).

**Flags — technical/quantitative claims `[UNVERIFIED]`** (Phase-2 fact-check): the `{{Recipe summary}}`
parameter set; Wikibooks page counts (~3,600 recipes; 704/1,557/1,273/93 by difficulty; 802 with images);
CC BY-SA 4.0 "since June 2023" / older 3.0; MediaWiki Action API limits (50 pageids/query regular, 500
results/list query); `$wgRCMaxAge = 30 days`; the recentchanges 30-day horizon; the atproto sync endpoints
(`com.atproto.sync.getLatestCommit`, `getRepo since rev`, CAR/MST). Uses "Merkle Search Tree" for atproto's
repo — **FACTCHECK-confirmed correct** (the refuted MST claim is iroh-docs=MST, which this does not make).
arecipe-repo design thinking (`CroftCommunity/arecipe`). **Three RUN files produced, delivered as `wiki.zip`
(NOT pasted here)** — referenced-but-not-captured; capture verbatim under `seeds/*-unpacked/` at Phase 2.

**Fact-checked 2026-07-23** → `alpha/research/2026-07-23-batch-factcheck.md`. Mostly CONFIRMED; **two
corrections:** (1) `{{Recipe summary}}` has **no `category` parameter** — categorization is automatic via
`difficulty` (`noincludecat` suppresses); the importer maps difficulty→category. (2) The **pipe-trick is NOT
mandated** by policy — ingredients must be in procedure order (✓) and "linked to the most appropriate
ingredient page," but not necessarily `[[Cookbook:carrot|]]` syntax (the link-parse premise still holds).
Counts refreshed: Category:Recipes **3,824** (Very Easy 723 / Easy 1,627 / Medium 1,327 / Difficult 102 /
Very Difficult 2; **832** with images). CC-BY-SA (7 June 2023), Action API limits, `$wgRCMaxAge` 30d on
Wikimedia (software default 90d), dumps→Content-File-Exports, and `getLatestCommit`/`getRepo since`→CAR all
CONFIRMED.

---

**User:** I'm thinking we can write a utility to run locally against
`https://en.wikibooks.org/wiki/Category:Recipes` and `https://en.wikibooks.org/wiki/Cookbook:Table_of_Contents`
to pull those recipes into the arecipe world — a wiki-looks-recipe to a recipe.app pipeline that can be run
periodically to update. Thoughts? I'm also seeing our list from the arecipe Bluesky account is getting quite
long and will be mostly static; considering precaching by distributing assets with the site and syncing
latest text from the PDS or something — lots of dynamic load for mostly static content is a bummer.

**Assistant:** Both are worth doing, and they land on the same mechanism.

**Wikibooks import — the source is more structured than a generic scrape target.**
- Every recipe carries `{{Recipe summary}}`, whose parameters are `category`, `servings`, `time`,
  `difficulty` (1–5), `image`, `energy`, `note`, plus `yield`, `cuisine`, `origin` in the documented
  examples. All parameters are optional — treat them as a **sparse map, not a schema.**
- Membership is machine-defined: the `{{Recipe}}` template is what adds a page to `Category:Recipes`, so
  `list=categorymembers` in the Cookbook namespace is the **authoritative enumeration.** The Table of
  Contents page is a human index and will drift from it.
- The real gift is ingredients. Cookbook policy says ingredients must be listed in procedure order and
  linked with the pipe trick, `[[Cookbook:carrot|]]`. That makes ingredient extraction a **link parse rather
  than an NLP problem**, and it feeds the conservative parser already built for RUN-SHOPPING-LIST.
- Scale is small: difficulty subcategories sum to ~3,600 pages (704 very easy, 1,557 easy, 1,273 medium, 93
  difficult); 802 recipes carry images. A few thousand records, not a corpus problem.

**License is the design constraint, not the transport.** New edits are CC BY-SA 4.0 since June 2023; older
revisions stay under 3.0; requirements are appropriate credit plus distributing remixes under the same
license. ShareAlike propagates, so an imported record needs **source URL, revision id, and license stamped
on it**, and any user edit-and-republish is an adaptation. That is a lexicon-field decision before it is a
pipeline decision, and it sits alongside the describe-don't-license posture already ruled for user recipes.

**Transport: do not build on dumps.** `dumps.wikimedia.org` marks the XML dumps deprecated (points to
MediaWiki Content File Exports), though dumps are still produced monthly. At 3,600 pages the **Action API** is
the right size anyway, provided the job behaves: a meaningful User-Agent with contact info, ≤3 concurrent
requests, respect for `Retry-After` on 429 (robot policy asks for concurrency of 1 and a one-second delay).
Store last revid per page and refetch only what changed → the periodic run is nearly free.

**Where the records live is the interesting question.** The natural home is a dedicated **arecipe-owned repo
published as a followable cook** — reuses the cook-follows mechanism already shipped, keeps the import out of
user repos where ShareAlike would tangle with their own content, and gives the sync job exactly one write
target.

**On the static list — you already have the cheap primitive.** `com.atproto.sync.getLatestCommit` returns
the current commit CID and rev; `getRepo` takes a `since` rev and returns only a diff. So: bundle a
build-time snapshot plus the rev it was taken at, render from it instantly, then one tiny rev check per
followed repo in the background, and only pull when rev moved. The seam: the diff comes back as a CAR of MST
blocks, so the fast path costs a CAR/CBOR decoder in the bundle, and it is **per-repo, not per-collection** —
for a long follow list that is N rev checks (cheap ones, but N). Falling back to `listRecords` on rev change
keeps the bundle thin at the cost of refetching a whole collection when anything in it moves. **Treat the
bundle as a cold-start cache, never as authority** — display names and handles go stale in ways recipe text
does not. The Wikibooks repo is the easy case: one repo, one rev, changes rarely.

**User:** Create 3 comprehensive Claude Code instruction sets: (1) a CLI tool to run locally to pull the
recipe corpus from Wikibooks into a structured local representation with intelligent delta handling (rerun in
6 months, smart about updates flowing wiki → tool → PDS); (2) most have a serves, time, and difficulty
rating — we only have prep time represented; how they do it hanging off the bottom of the image in 3 rows
seems good, let's do that; and another for the release-bundling process to speed up first load and PDS load.

**Assistant:** Three run files (delivered as `wiki.zip` — not reproduced here), all TDD red-first,
parallel-safe except where noted. Four findings that shaped them, one load-bearing:
- **Recent changes will not work for a six-month rerun.** `$wgRCMaxAge` is 30 days on Wikimedia wikis, and
  the recentchanges API cannot list further back than that — a semiannual job built on it would silently miss
  five months. The delta path is instead **full category enumeration plus a batched revision sweep** (50
  pageids per query for regular clients, 500 results per list query → a full discovery pass at under 100
  requests).
- **Infobox values are free text, not numbers** (the template docs show `servings = 1-2`, `yield = 4
  burgers`, `time = 30 minutes`). Both files carry a display-plus-hint model so sorting never rewrites the
  source string.
- **Two independent change axes** — upstream revid moved, or our parser improved. Only the first costs a wiki
  request; `--reparse` walks the second with the network off (the mode you'll live in during parser work).
- **The meta-strip run opens with a discovery gate.** Recipes are `exchange.recipe.recipe`, owned by
  `recipe.exchange`, so where `serves` and `difficulty` live is not arecipe's unilateral call. D0 reports what
  exists before anything is written, then branches. Recommendation: open-world fields on records arecipe
  authors, with an upstream proposal running in parallel.

**Open owner decisions, collected (surfaced, not resolved):** separate repo vs `tools/` for the sync CLI; the
licence string and attribution text for imported records; ShareAlike handling when a user edits an import;
the PDS account that hosts the corpus; the storage path for serves and difficulty; the snapshot seed-list
source; the revalidation debounce window.

(Artifacts delivered: **Run wikibooks corpus**, **Run recipe meta strip**, **Run bundle precache** — as MD
documents, plus `wiki.zip`. Not pasted into this session; capture at Phase 2 if the archive is provided.)

---

**Sources cited in-session (dialogue-sourced, `[UNVERIFIED]`):** Wikibooks (`{{Recipe summary}}` docs,
Cookbook ingredient policy, `Category:Recipes` counts); Creative Commons / Wikimedia (CC BY-SA 4.0 since June
2023, ShareAlike); MediaWiki (Action API limits, `$wgRCMaxAge`, dumps deprecation → Content File Exports);
Bluesky / GitHub (`getLatestCommit`, `getRepo since rev`).
