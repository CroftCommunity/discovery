# RUN-AGENTS-PAGE — llms.txt + an agent-facing guide to extracting recipes properly

Self-contained instruction file for Claude Code, repo `CroftCommunity/arecipe`.
This run ships CONTENT as much as code, so it carries an explicit publication
gate: the complete final copy of every shipped document appears verbatim in
the run summary, and merging the PR is publishing — the owner reads before
merge. [verify-in-run] items are probed and recorded before code depends on
them. Contradictions with grounding are FINDINGS.

## 0. Mission

Publish an agent-consumable guide on arecipe.app: how AI agents can extract,
normalize, attribute, and share recipes properly — including what the primary
legal sources say about recipe copyright — plus how to read arecipe's data
the way the protocol intends (public atproto records, not scraped HTML).

THE GOVERNING POSTURE (owner ruling, verbatim intent): **we make no legal
claims and only cite sources, plain and simple.** Every legal statement on
the page is a quotation from or tight attributed paraphrase of a NAMED
source. arecipe's own voice appears only in technical best practices and in
describing its own protocol surface. arecipe never asserts a legal
conclusion, never advises, and never licenses anything on its users' behalf
(recipes belong to their authors; the page DESCRIBES what the protocol makes
public and what the sources say — it grants nothing).

Endpoints: `/llms.txt` (the discovery index, per the llms.txt convention),
`/agents.md` (the canonical guide, Markdown), and `/agents.html` (the
human-readable mirror, generated from the same source at build time).

## 1. Standing conventions (non-negotiable)

1. **TDD where testable, red first.** The endpoint plumbing, format lints,
   parity check, and the claim-phrase guard all get failing tests before
   implementation. Content quality is enforced by the mechanized guards plus
   the owner's pre-merge read — both, not either.
2. **Gate green** (`npm test`) at every phase boundary; browse bundle-split
   guard untouched (this run adds static content + one footer link).
3. **Style**: strict vanilla TS for build tooling; committed content is
   Markdown; no raw hex, no new runtime dependencies; module comments
   explain why.
4. **Plan file** `plans/2026-07-XX-N-plan-agents-page.md` before coding;
   Status updated at completion, house format.
5. **Quote discipline**: every direct quotation in the shipped content is
   verified VERBATIM against the fetched source during this run (Phase 2);
   a quote that cannot be verified against its source is not shipped as a
   quote — cite without quoting instead. Quotes stay short (a phrase or a
   sentence); the page links the full sources.

## 2. Grounded context (researched/verified 2026-07-17; Phase 0 re-grounds repo details)

- **The convention.** llms.txt: a Markdown file at the domain root —
  proposed by Jeremy Howard (Answer.AI), September 3, 2024; spec at
  llmstxt.org. Shape: H1 site name first, a 1-3 sentence blockquote summary,
  optional freeform guidance, then H2 sections of `- [Title](URL):
  description` links; served text/plain or text/markdown, absolute HTTPS
  URLs. Status honesty (goes in the plan file, not the page): community
  convention, no standards body; adopted by Anthropic, Stripe, Cloudflare,
  Vercel and ~10% of sites; major LLM crawlers largely do not fetch it —
  the real consumers are agentic browsers, IDE agents, and MCP
  integrations; Chrome Lighthouse's "Agentic Browsing" category audits for
  its presence. robots.txt is a different layer (access, not comprehension)
  and this run does not touch it.
- **The citation set** (the page's Part A raw material; URLs for Phase 2
  verification):
  - US Copyright Office, Circular 33, "Works Not Protected by Copyright" —
    https://www.copyright.gov/circs/circ33.pdf — states there is "no
    copyrightable authorship in a mere listing of ingredients" and works an
    example refusing registration for a dressing recipe consisting of an
    ingredient list plus brief numbered instructions; also shows the Office
    limiting a cookbook registration to "the text and photographs,"
    the anchor for the photos-are-protected point.
  - 17 U.S.C. §102(b) — the statutory exclusion of ideas, procedures,
    processes (link the uscode.house.gov section).
  - Publications Int'l, Ltd. v. Meredith Corp., 88 F.3d 473 (7th Cir.
    1996) — full opinion PDF:
    https://cyber.harvard.edu/people/tfisher/IP/1996Publications.pdf —
    recipes at issue held unprotectable under §102(b); the court
    acknowledged recipes conveying "more than simply the directions for
    producing a certain dish" may qualify and expressly declined a per-se
    rule.
  - Lambing v. Godiva Chocolatier, 142 F.3d 434 (6th Cir. 1998)
    (unpublished) — recipes as "functional directions for achieving a
    result," excluded under §102(b). [Phase 2: locate a linkable copy of
    the opinion text; if none is reachable, attribute via a citing court
    document rather than quoting.]
  - Feist Publications v. Rural Telephone (U.S. 1991) — compilation
    protection limited to selection and arrangement, never the facts.
  - Optional further-reading row: Harrell v. St. John, 792 F. Supp. 2d 933
    (S.D. Miss. 2011); Tomaydo-Tomahhdo v. Vozary, 629 F. App'x 658
    (6th Cir. 2015).
  All sources are US; the page says so plainly.
- **Repo mechanics.** `scripts/build.mjs` is an allowlist copy into dist/ —
  root statics (client-metadata.json, CNAME) are established precedent;
  llms.txt / agents.md / agents.html need allowlist rows. Pages serves
  root-level statics at stable URLs. [verify-in-run: the content-type Pages
  emits for a served `.md` file — text/markdown and text/plain are both
  acceptable per the convention, so this is record-the-answer, not a
  blocker.]
- **Practice-what-we-publish symmetry.** The best-practices sections
  describe exactly what the repo already implements: the importer's JSON-LD
  ladder (schema.org Recipe; `recipeIngredient`; `recipeInstructions` as
  string / HowToStep / HowToSection; legacy `ingredients` key) and the
  shopping-list normalization grammar (unit synonym table, unicode
  fractions, ranges, conservative name folding). Phase 0 confirms both
  shipped and mines their docs/specs for the published guidance.
- **arecipe's data surface.** Recipes are `exchange.recipe.recipe` records;
  arecipe's own records live under `app.arecipe.*`; everything is public
  atproto data readable via `com.atproto.repo.listRecords` / `getRecord`
  against the author's PDS (handle → DID → PDS resolution). This is the
  "read the records, don't scrape the HTML" section's substance.

## 3. Locked design decisions

- **D1 Endpoints.** `/llms.txt` spec-shaped: `# arecipe` H1, blockquote
  summary (family recipe box on atproto; recipes are public records owned
  by their authors), a short guidance paragraph pointing agents at
  agents.md, then H2 sections linking agents.md, the data-access section
  anchor, docs/LEXICONS.md, and the reading-order basics. `/agents.md` is
  CANONICAL; `/agents.html` is generated from it at build time by a small
  build-step converter (no runtime Markdown parser, no dependency — a
  minimal converter for the subset of Markdown the doc uses, unit-tested),
  wrapped in the site chrome with a footer link "For AI agents" site-wide.
  A parity test asserts every content sentence of agents.md appears in
  agents.html.
- **D2 Voice separation (the ruling, mechanized).** agents.md has four
  parts:
  **Part A — What the sources say.** Legal content ONLY as quotation or
  attributed paraphrase: every paragraph names its source inline and links
  it. No sentence in arecipe's voice draws a legal conclusion. The
  practical framing the sources support, stated as attribution: the
  Copyright Office and the cited courts treat ingredient lists and
  functional directions as unprotected facts and process, while
  substantial literary expression — narrative, headnotes, creative
  descriptions — and photographs can be protected, and compilations are
  protected only in selection and arrangement.
  **Part B — Best practices for extraction** (our technical voice): prefer
  structured data over rendered HTML — schema.org/Recipe JSON-LD, the
  tolerant ladder (string/HowToStep/HowToSection/legacy keys); extract the
  data (ingredients, quantities, times, temperatures, functional steps);
  RE-EXPRESS instructions in your own functional language; take no
  headnotes, stories, or photographs; attribute with the recipe name, the
  author, and a link; respect robots.txt and rate limits; cache
  courteously.
  **Part C — Reading arecipe the right way**: don't scrape our pages —
  the data is public atproto records; worked examples: resolve handle →
  DID → PDS, `listRecords?collection=exchange.recipe.recipe`, `getRecord`;
  lexicon pointers; the sentence that recipes belong to their authors and
  arecipe grants nothing on their behalf; a courtesy note on request
  volume.
  **Part D — Plain notice**: informational only, not legal advice; all
  cited sources are US; links are to the primary sources; verify anything
  that matters with counsel. Short, unhedged, once.
- **D3 The claim-phrase guard.** A committed banned-phrase lint test (house
  precedent: the banned-absolutes guard) greps agents.md + agents.html for
  assertive legal-claim phrasings in our voice — at minimum: "is legal",
  "it is legal", "you may legally", "legally safe", "we guarantee", "you
  cannot be sued", "no risk", "fair use allows" — with the list committed
  and extended during drafting as near-misses surface. Part A's structural
  rule is also linted: every Part A paragraph must contain at least one
  source link.
- **D4 Quote verification.** During Phase 2, fetch each cited source and
  verify every quotation verbatim (whitespace-tolerant); record each
  verification (source, quote, match) in the run summary. Unreachable
  source → cite without quoting (per convention 5). No secondary-source
  quotes presented as primary.
- **D5 Discoverability.** Footer "For AI agents" link → agents.html;
  README one-liner; llms.txt at root. No robots.txt changes, no sitemap
  changes, no meta-tag games.
- **D6 Deferred (verbatim in plan file):** agent-permissions.json (still a
  research proposal); skill.md; non-US legal sections; translations; an
  arecipe MCP server (the natural successor to Part C); marking individual
  recipe pages with machine-readable pointers back to agents.md.

## 4. Phases

### Phase 0 — ground against main
Confirm: build.mjs allowlist mechanics; importer + shopping-list modules
shipped and their exact tolerances (mine them for Part B specifics so the
page matches the code); footer component seam for the site-wide link;
LEXICONS.md rows to link. Drift = FINDING.

### Phase 1 — plumbing + guards (red first)
RED: dist tests — llms.txt, agents.md, agents.html present post-build;
llms.txt format lint (H1 first, blockquote present, absolute HTTPS links,
sections parse); the D3 claim-phrase guard (seed it with the committed list
against placeholder content); Part A citation-link lint; md→html converter
unit specs (the doc's Markdown subset, deterministic output); parity test.
GREEN: allowlist rows, converter, skeleton files that pass structure lints
with placeholder content clearly marked DRAFT.

### Phase 2 — content
Draft agents.md per D2 in full. Fetch and verify every quote per D4.
Replace placeholders; all lints green with REAL content. The complete
document goes verbatim into the run summary. Where Part B states a
tolerance or a normalization rule, cross-check it against the shipped
module and cite our own docs — the page must not promise practices the
code doesn't perform.

### Phase 3 — mirror + link
agents.html generation wired into the build; footer link with testid
`agents-link`; e2e: both endpoints reachable in the built dist with 200 +
sane content; the footer link navigates; mobile-fit on agents.html.

### Phase 4 — closeout
[verify-in-run] the served .md content-type (record; either acceptable
value passes). Plan Status; README line; run summary: red → green per
phase, the quote-verification table, the final banned-phrase list, the
full shipped copy of llms.txt AND agents.md verbatim, and the explicit
line that MERGE = PUBLICATION pending the owner's read.

## 5. Acceptance criteria (each maps to a named test or artifact)

1. `/llms.txt`, `/agents.md`, `/agents.html` ship in dist and are
   spec-shaped; the html mirror passes parity with the canonical md.
2. Every legal statement in the shipped content is a verified quotation or
   an attributed paraphrase with an inline primary-source link; the
   quote-verification table in the summary covers 100% of quotes.
3. The claim-phrase guard and citation-link lint are permanent gate tests
   and pass on the real content; arecipe's voice draws no legal
   conclusion anywhere in the shipped documents.
4. Part B's practices match what the repo's own importer and shopping-list
   modules actually do; Part C's worked atproto examples are correct
   against the live protocol surface.
5. The page describes and never licenses: user recipes are stated to
   belong to their authors, with no rights granted on their behalf.
6. Footer link + README line ship; robots.txt and sitemap untouched; gate
   green throughout; full copy present in the run summary for the owner's
   pre-merge read.
