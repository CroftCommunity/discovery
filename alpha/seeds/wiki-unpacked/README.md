# seeds/wiki-unpacked — frozen verbatim RUN briefs (from `wiki.zip`, 2026-07-23)

**preserved-verbatim** (byte-verified against `wiki.zip` on extraction, 2026-07-23). The three Claude Code
instruction files produced in the arecipe Wikibooks-import / precache dialogue
(`../transcripts/raw/arecipe-wikibooks-corpus-import-and-static-precache-2026-07-23.md`). Arecipe-repo
build/experiment briefs; executions live in `CroftCommunity/arecipe`.

| File | What |
|---|---|
| `RUN-WIKIBOOKS-CORPUS.md` | Local CLI importing Wikibooks Cookbook recipes with intelligent delta handling (full enumeration + revid sweep, NOT recentchanges); CC-BY-SA stamping. |
| `RUN-RECIPE-META-STRIP.md` | serves/time/difficulty as 3 rows under the image; D0 lexicon-ownership gate (`exchange.recipe.recipe`); display-plus-hint, never rewrite source strings. |
| `RUN-BUNDLE-PRECACHE.md` | Static-list release process: build-time snapshot + rev, background `getLatestCommit` checks, bundle-as-cold-start-cache-never-authority. |

**Fact-check corrections that fold into `RUN-WIKIBOOKS-CORPUS`/`RUN-RECIPE-META-STRIP`**
(`../../research/2026-07-23-batch-factcheck.md`): `{{Recipe summary}}` has **no `category` param** (auto via
`difficulty`); the **pipe-trick is not policy-mandated** (ingredients are wikilinked + procedure-ordered, but
not necessarily `[[Cookbook:carrot|]]`). Backlog: ROADMAP_TODO E56–E58. The `wiki.zip` archive can be retired
at the user's discretion (contents byte-verified here).
