# Drystone spec site — build tooling

A static site that publishes the Drystone corpus so **every cross-reference is a
followable link** — within a document (`§7.6.2`), across the set (`Part 1 §2.5`),
into the appendices (`Appendix B`), and into the published companions. The build
doubles as a **broken-reference gate**: a `§`-reference in Part 1 or Part 2 that
resolves to no heading fails the build, making the site a permanent, free
consistency guard on the corpus.

## Design (boring on purpose)

A single Python build script plus one unit-tested resolver module — no framework.

| file | role |
|------|------|
| `resolver.py` | Pure, stdlib-only logic: section-number → stable anchor id, reference autolinking + the gate, and mermaid-block substitution. Unit-tested in isolation from rendering. |
| `test_resolver.py` | The resolver contract (29 tests). `python3 -m unittest test_resolver`. |
| `build.py` | Renders markdown → HTML (`markdown==3.7`), injects anchors, autolinks, pre-renders diagrams, runs the gates, emits the site + landing index. |
| `requirements.txt` | The one pinned Python dependency. |
| `package.json` | The one pinned Node dependency: `@mermaid-js/mermaid-cli` (the diagram pre-renderer; `package-lock.json` locks the tree). |
| `puppeteer-config.json` | Headless-browser flags for the renderer (`--no-sandbox`). |

Sources are **canonical**: the build makes zero edits to the markdown. Every anchor
and link is generated at build time. If a source looked like it needed an edit to
render, that would be a FINDING, not an edit.

## Anchors (stable, from section numbers)

Each heading's id is derived from its **number**, not its title, so links survive a
title rewording:

- `## 7.6.2. Foo` → `#s7-6-2`
- `## Appendix B. …` → `#appendix-b`; `### C.4 …` → `#c-4`
- §7.2 R-bullets (`- **R7, …`) get list-item ids `#s7-2-r7`; a `§7.2 R9` with no
  such bullet falls back to `#s7-2`.
- prose headings fall back to a title slug.

## Reference resolution

- `Part 1 §N…` / `Part 2 §N…` → the other document's anchor (cross-doc).
- `Appendix X` → the appendix anchor (appendices live in Part 2).
- bare `§N…` → this document if it has that section, else the document's fallbacks
  (companions annotate Part 2, so they fall back to Part 2 then Part 1).
- a backticked repo-path to a **published** file (`` `alpha/thinking/x.md` ``) →
  a relative link, keeping its monospace rendering. An unpublished path is left
  literal (intentionally not a link, not a broken ref).
- **RFC / BCP section citations also use `§`** (`RFC 9420 §16.4`). These are
  detected — by adjacency to an `RFC ####` / `BCP ##` token, or as a section number
  cited in an RFC-context block that no Drystone heading defines — and left literal.
  They are **not** Drystone references and do not trip the gate.
- References inside code spans and fenced blocks are never linkified (code examples
  stay literal). Status tags (`Verified`, `[gates-release]`, …) are content: they
  render as-is (monospace where the source backticks them), never reinterpreted.

## The gates

- **Broken references (RUN-10).** Part 1 and Part 2 are **hard-gated**: an unresolved
  `§`-reference in either fails the build (`exit 1`), naming the offenders.
- **Companion allowlist (RUN-12).** Outside the hard-gated spec, the set of unresolved
  companion refs must match `COMPANION_ALLOWLIST` in `build.py` **exactly** — a new
  unlisted unresolved ref or a stale allowlist entry each fails the build.
- **Diagrams (RUN-13).** A ```` ```mermaid ```` block that fails to parse/render fails
  the build, naming the source file and block. A mermaid example *quoted inside another
  fenced code block* is literal content and is never processed.

## Mermaid diagrams (build-time pre-render — the choice, documented)

Fenced ```` ```mermaid ```` blocks are rendered **at build time** to inline SVG by
`@mermaid-js/mermaid-cli`, pinned in `site/package.json` (mermaid 11.16.0). Decided
empirically (RUN-13 Part 4) over the alternative — a pinned client-side
`mermaid.min.js` — because pre-rendering:

1. keeps the strongest form of the **no-network-at-read** property: the published page
   embeds the SVG, needs **no JavaScript and no fetch of any kind** to display it
   (a vendored client script would also avoid third-party fetches, but still puts
   ~2.8 MB of JS between the reader and the figure);
2. gives the **diagram gate for free**: the renderer's parse failure *is* the build
   failure — no separate validation path that could drift from the display path;
3. costs network only at **build** time (CI lets puppeteer fetch its headless browser;
   locally, point `PUPPETEER_EXECUTABLE_PATH` at an existing chromium).

Diagrams are wrapped in `.mermaid-figure` (a light plate, so the fixed SVG palette
stays readable in the dark scheme; wide diagrams scroll horizontally).

## The published set (nothing else is published)

- **Specification tier** — the eight named `beta/drystone-spec/` documents: Part 1,
  Part 2, the Evidence Map, conventions-and-decisions, open-threads, the Part 2
  changelog, the DAG-CBOR primer, and the (historical) proposed-changes note.
- **Socialization tier (RUN-13)** — the named `beta/socialization/` gradients
  documents (the five-tier depth model + the one-liner candidates under review),
  published as the **Gradients** page(s).
- **Classroom tier (RUN-13, draft)** — all markdown under `alpha/classroom/` (the
  arc + chapter skeletons), labeled "Classroom · draft"; prose bodies are
  `DRAFT-PENDING`, written in conversation.
- **Exploratory tier** — all markdown under `alpha/thinking/`, labeled
  "Exploratory — design dialogue", published because Part 2 cites these notes by path.

## Run it

```sh
pip install -r site/requirements.txt       # markdown==3.7
npm ci --prefix site                       # pinned mermaid-cli (once; puppeteer fetches
                                           #   a browser, or set PUPPETEER_EXECUTABLE_PATH)
python3 -m unittest -v site/test_resolver  # resolver tests (run from repo root)
python3 site/build.py                      # build into site/_site/
python3 site/build.py --check              # run the gates only; write nothing (the PR check)
```

## Deploy

`.github/workflows/pages.yml` builds and deploys on push to `main` via
`actions/deploy-pages`, and runs the same build (hence the gate) on pull requests
without deploying. One-time owner setup: repo **Settings → Pages → Source:
"GitHub Actions"**.
