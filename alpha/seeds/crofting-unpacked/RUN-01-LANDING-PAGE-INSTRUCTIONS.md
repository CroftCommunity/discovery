# RUN-01: Initial croft.ing landing page

Target repo: `CroftCommunity/crofting_site` (main). Execute on a fresh branch off `main`
named `run-01-landing-page`. Work the phases in order. Write `RUN-01-SUMMARY.md` at repo
root when done. Do not merge; leave the branch for review.

Design sources of record (do not re-derive; this file carries everything you need):
`discovery/beta/socialization/visual-identity-and-the-progressive-depth-website.md`,
`brand-and-voice.md`, and `discovery/beta/croft/croft-ing-the-website-and-the-plot.md`.
All copy below is final text to place verbatim unless a note says otherwise.

## Guardrails

- STOP if the repo contains anything beyond `LICENSE` (AGPL-3.0) and `README.md`. That
  means the snapshot this run was written against is stale. Report and halt.
- No frameworks, no package.json, no build step, no JavaScript on any page. Plain HTML +
  one CSS file. The pages must render correctly from `file://` and from a static server.
- Zero external requests: no CDN fonts, no analytics, no external images, no hotlinks.
  Fonts are self-hosted (Phase 2). If font files cannot be fetched, fall back to the
  system stacks defined below, note it in the summary, and continue. Never link Google
  Fonts.
- Do not add tracking of any kind. Do not add cookie banners or tracking notices.
- Do not enable GitHub Pages or touch repo settings; that is a manual step recorded in
  the summary for the maintainer.
- Copy discipline: place the copy in this file verbatim. Do not embellish, do not add
  exclamation points, do not add marketing language. Where a section says DRAFT, light
  editorial smoothing for flow is allowed but no new claims.

## Phase 1: structure

Create:

```
index.html          the landing page
plot.html           pillar page: The Plot
wall.html           pillar page: The Wall
valley.html         pillar page: The Valley
library.html        the Bedrock index (links out to the discovery repo)
styles.css
CNAME               containing exactly: croft.ing
assets/fonts/       woff2 files + OFL license texts (Phase 2)
assets/favicon.svg  (Phase 3)
```

Every page shares the same header and footer.

Header: left, the wordmark "CROFT.ing" set in the heading serif, linking to `/`
(style note: "CROFT" in Iron Ore Black, ".ing" in Ruddy Orange, no space). Right, two
quiet text links: "Library" → `library.html`, "What's growing" → `index.html#growing`.

Footer, small and quiet, one line each:

> Croft is built in the open. [Site source](https://github.com/CroftCommunity/crofting_site) · [The discovery repo](https://github.com/CroftCommunity/discovery)
>
> AGPL-3.0. No cookies, no scripts, no tracking.

## Phase 2: type and palette

Self-host two families, subset to latin, weights only as listed:

- Lora (headings): 500, 600. Source: the `google/fonts` GitHub repo or fontsource
  artifacts. License: SIL OFL 1.1; include `OFL-Lora.txt` beside the files.
- Inter (body/UI): 400, 600. Source: the `rsms/inter` release zip or fontsource.
  License: SIL OFL 1.1; include `OFL-Inter.txt`.

`@font-face` with `font-display: swap`. Fallback stacks:
headings `Lora, Georgia, 'Times New Roman', serif`;
body `Inter, system-ui, -apple-system, 'Segoe UI', sans-serif`.

CSS custom properties, the tectonic palette (working values, from the design board):

```css
:root {
  --schist:   #2F3539;  /* Deep Schist: structure, dark blocks */
  --granite:  #A2A9AB;  /* Light Granite: rules, subtle text */
  --ruddy:    #B75C34;  /* Ruddy Orange: primary action, heading accents */
  --moss:     #3D6546;  /* Dark Moss: links, growth, active states */
  --ink:      #1C1E20;  /* Iron Ore Black: primary text */
  --canvas:   #E9E1D6;  /* Oatmeal Canvas: page background */
}
```

Layout rules (these carry the craft stance; treat as requirements):

- Background `--canvas`, text `--ink`. Never pure white, never pure black.
- Single centered column, `max-width: 42rem` for prose; the pillar row on the landing
  page may widen to `56rem`. Generous space: at least `5rem` vertical between sections
  on desktop, scaled down on mobile.
- Body 1.0625rem/1.7. Headings in Lora, tight and calm; h1 is the only large element.
- Links in `--moss`, underlined, no hover tricks beyond a color deepen.
- One button style only: `--ruddy` background, `--canvas` text, small radius, no shadow,
  no gradient.
- Section divider: an inline SVG "drystone course" pattern in place of an `<hr>`. A
  single row of 7 to 9 interlocking rectangles of slightly varying widths, 1.5px
  strokes in `--granite`, no fill, height about 14px, centered, width about 220px.
  Define it once and reuse. Crisp and geometric, a masonry blueprint, not a texture.
- Responsive: pillars are a 3-column grid at ≥48rem, single column below. No other
  breakpoint complexity.
- No animation anywhere except `prefers-reduced-motion`-safe link color transitions.

## Phase 3: favicon

`assets/favicon.svg`: three stacked rectangles of unequal widths (a tiny drystone
cairn), stroked in `--schist` on transparent, offset like a real stack. Reference it
from every page. Keep it under 1 KB.

## Phase 4: the landing page (index.html)

Order of sections, copy verbatim.

### Hero

# GROW YOUR OWN.

(styling: h1 in Lora 600, letterspaced slightly, `--ink`, with the final period in
`--ruddy`.)

> A quiet, personal space on the modern web. A plot of your own, built to last.
> No profit motive.

Two quiet links beneath, not buttons: "Understand the landscape" → `plot.html`, and
"What's growing now" → `#growing`.

### The three pillars

Each pillar is a card with: the name (h2), the Signpost line (one sentence, set as a
lede), the Surface pitch (2 to 3 sentences), and one link "Understand the landscape" →
its pillar page. Label the tiers only on the pillar pages, not here; here they just
read as good writing.

**The Plot**

Signpost: A small place on the web that is actually yours.

Surface: It belongs to you. You tend the content, and it is yours to keep: not a
profile rented from a platform, but a plot that stays standing no matter who hosts the
road past it.

**The Wall**

Signpost: Structure without mortar: pieces that hold because they fit.

Surface: Built on Drystone, a protocol with no central authority acting as the glue.
Like a drystone wall, every piece is placed to fit its neighbors, and nothing holds it
together but good alignment.

**The Valley**

Signpost: Neighbors by choice, joined by cooperation rather than profit.

Surface: A plot alone is a homestead; plots together are a valley. The road between
them is an open network, and the valley runs on cooperation, not capital.

### What's growing now  (`id="growing"`)

Heading: **What's growing now**

Intro line: Croft grows a crop at a time. These are the working pieces today.

- **arecipe** — A recipe box and meal planner that keeps your recipes and meal plans in
  your own data store, with shareable public views. The first working crop.
  Link: https://arecipe.app
- **The Plot** — The personal homestead page: your own corner of the valley, rendered
  straight from your own records. Under cultivation.
  (No link. Do not invent one.)

(Note: the em dashes above are markdown list formatting in this instruction file; in
the HTML, set the name as a bold term and the description as its own sentence, no
dashes.)

### Closing

Set as a centered, quiet blockquote above the footer:

> A collaborative effort to reclaim a kinder, more human web.
> You are entirely welcome here.

## Phase 5: the pillar pages

Each pillar page stacks the four depth tiers in fixed order with small, visible tier
labels (small-caps, `--granite`, above each tier). This explicit tiering is deliberate:
the maintainer wants the layers side by side so they can be reflected on and kept in
sync. The labels, in order:

1. `THE SIGNPOST` — the one-liner (same text as the landing page).
2. `THE SURFACE` — the elevator pitch (same text as the landing page).
3. `THE SOIL` — the one-pager essay (DRAFT copy below; smoothing allowed).
4. `THE BEDROCK` — 1 to 3 links into the discovery repo, plus one line of framing.

Identical text at tiers 1 and 2 across index and pillar pages is a feature, not
duplication to fix.

### plot.html — The Soil (DRAFT)

Most of what people call a home on the web is a rental. The address, the contents, and
the guest list belong to a platform, and the platform's needs come first. When its
model shifts, the home goes with it. The history of the personal web is a history of
bulldozed neighborhoods: beloved page-hosting communities bought and deleted, media
libraries lost in migrations, well-meaning ad-free networks that burned out and shut
down. The pattern is not malice. It is what happens when a home depends on a landlord.

A Croft plot is built the other way around. Your identity and your records live on an
open protocol, in a data store that answers to you. The plot page is just a viewer:
its only job is to find your records and paint them. If this viewer ever disappears,
you point another one at the same records and everything renders unchanged. Nothing to
export, nothing to migrate, nothing lost.

That also changes what an "empty" page means. A plot is not a feed to perform on; it
is soil to tend at your own pace. Untended soil is not failure. It is fallow, which is
a healthy phase of preparation, and it will still be yours when you come back to it.

Bedrock framing line: The full product thinking, including the plot's architecture and
its design history, lives in the open discovery repo.
Links: the discovery repo root; `beta/croft/croft-ing-the-website-and-the-plot.md`.

### wall.html — The Soil (DRAFT)

A drystone wall has no mortar. Every stone is chosen and placed to fit the stones
around it, and the wall stands because the fit is good. Walls built this way outlast
the people who built them.

Drystone, the protocol underneath Croft, is built on the same discipline. There is no
central authority holding the structure together and no privileged node whose copy of
events counts as the real one. Every participant can check, locally, that the record
in front of them is consistent and corroborated.

The protocol also knows what is not its job. Machines are good at provenance: what was
said, by whom, and what corroborates it. Machines have no access to utility: what is
true, fair, or right. Drystone computes the first and never pretends to the second;
judgment stays with people. And when people genuinely disagree, the protocol does not
manufacture a winner. Disagreement is recorded plainly, and communities part or
reconcile on human terms. A fork, not a verdict.

Bedrock framing line: The full specification, its reasoning, and the running
experiments live in the open discovery repo.
Links: the discovery repo root; `beta/drystone-spec/`.

### valley.html — The Soil (DRAFT)

A plot on its own is a homestead. Plots within reach of each other are a valley, and
the valley is the point: visiting a neighbor, leaving a note at their gate, walking
from plot to plot along paths people chose to lay, rather than scrolling a feed a
machine composed for you.

The road through the valley is an open network, not a private one. Croft builds on
existing open social protocol rails, so a plot is not an island: notes and visits
thread into the wider network without a bespoke platform in the middle.

What holds the valley together is the same thing that holds the wall together:
alignment rather than extraction. There is no engagement machinery, no ranking that
rewards heat, and no investor who needs the neighborhood to grow forever. Cooperation
is the whole business model, which is to say there is no business model. There are
neighbors.

Bedrock framing line: The community and governance thinking behind the valley lives in
the open discovery repo.
Links: the discovery repo root; `beta/socialization/`; `beta/governance/`.

## Phase 6: library.html

Heading: **The Library**

Intro (verbatim):

> Everything above is the short version on purpose. The long version exists, in full,
> in the open. The discovery repo holds the philosophy, the Drystone protocol
> specifications, the product design history, and the running experiments, exactly as
> they were worked out.

Then a short link list (plain, no cards):

- The discovery repo → https://github.com/CroftCommunity/discovery
- The Drystone specification → https://github.com/CroftCommunity/discovery/tree/main/beta/drystone-spec
- Philosophy → https://github.com/CroftCommunity/discovery/tree/main/beta/philosophy
- The Croft product surface → https://github.com/CroftCommunity/discovery/tree/main/beta/croft

Verify each path exists in the public discovery repo before linking; if a path differs,
link the nearest real directory and note the substitution in the summary.

## Phase 7: README and summary

Replace README.md with: what this repo is (the croft.ing landing page), the
no-dependency stance (plain HTML/CSS, no JS, no external requests), how to preview
(`python3 -m http.server` or open index.html), the depth-tier convention
(Signpost / Surface / Soil / Bedrock and the keep-in-sync rule), and the palette/type
tokens' source of record (the discovery repo's visual-identity doc).

`RUN-01-SUMMARY.md` must record: files created; font sourcing outcome (versions and
where fetched, or the fallback taken); any link substitutions from Phase 6; any copy
smoothing done in DRAFT sections (quote before/after); and the two manual follow-ups
for the maintainer: enable GitHub Pages (deploy from branch, root) and point croft.ing
DNS at Pages (apex A/AAAA records plus the CNAME file already committed).

## Acceptance checks (run before writing the summary)

- Zero network requests: grep all HTML/CSS for `http` and confirm every hit is an
  intentional outbound link (arecipe.app, github.com), never a loaded resource.
- No `<script>` tags anywhere.
- Each pillar page shows all four tier labels in order, and tiers 1 and 2 match the
  landing page text exactly.
- Pages are readable at 360px wide and at desktop width.
- `CNAME` contains exactly `croft.ing`.
