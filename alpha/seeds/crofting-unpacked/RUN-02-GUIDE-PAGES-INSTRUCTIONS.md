# RUN-02: crofting_site — the arecipe and Skylite guide pages

Target repo: `CroftCommunity/crofting_site` (main, currently the RUN-01 site:
index + plot/wall/valley/library + styles.css + fonts, live at croft.ing).
Execute on a fresh branch `run-02-guide-pages` off `main`. Write
`RUN-02-SUMMARY.md`. Do not merge; leave for review.

## Guardrails (unchanged from RUN-01, restated)

- Plain HTML + the existing single styles.css. No frameworks, no JS, no build
  step, no new dependencies. Zero loaded external requests; outbound clickable
  links only.
- Reuse the existing header, footer, palette tokens, type, and the `.course`
  divider. Extend styles.css only where the new pages genuinely need it; do
  not restyle existing pages.
- Copy below is FINAL and reviewed; place verbatim. No embellishment.
- TDD-first, per standing convention: Phase 1 creates the checks harness with
  assertions for the NEW pages included, runs it, records the red; pages are
  built to green. Red-to-green order evidenced in the summary.

## Phase 1 (RED): checks/check_site.py

Create `checks/check_site.py`, Python 3 stdlib only, exit nonzero on failure,
covering the WHOLE site (this becomes the standing regression net RUN-01
lacked):

- No `<script>` in any HTML file; `CNAME` is exactly `croft.ing`.
- Every internal href/asset reference resolves to a file in the repo
  (directory hrefs resolve to `<dir>/index.html`).
- External URLs limited to an allowlist: github.com/CroftCommunity,
  arecipe.app, arecipe.croft.ing, skylite.croft.ing, recipe.exchange; plus the
  SVG namespace inside CSS data URIs.
- Pillar pages each show the four tier labels in order; their Signpost and
  Surface text matches index.html exactly.
- NEW: `arecipe/index.html` and `skylite/index.html` exist; each contains the
  tier labels THE SIGNPOST, THE SURFACE, THE BEDROCK in order; the arecipe
  page contains "A shared community recipe box." and the Amanda tagline; the
  skylite page contains "A tended window into the butterfly garden of
  Bluesky."; index.html's growing section contains "Skylite".

Run it now: the NEW assertions must fail (the rest must pass, proving the
harness is honest). Record the red output in the summary.

## Phase 2: the two pages

Create `arecipe/index.html` and `skylite/index.html` (directory style, so
croft.ing/arecipe and croft.ing/skylite resolve). Because these live one level
deep, reference shared assets by absolute root paths (`/styles.css`,
`/assets/favicon.svg`); the site is served at the domain root. Same header and
footer as every page. Tier labels styled exactly like the pillar pages'
(small-caps, `--granite`).

### arecipe/index.html — copy verbatim

Title: `arecipe — croft.ing`

THE SIGNPOST

A shared community recipe box.

THE SURFACE

arecipe is a recipe box and meal planner for communities: families, supper
clubs, neighborhoods. Your recipes live as records in your own account on an
open network (a Bluesky account works as-is), not controlled and served back
to you by somebody else's platform. arecipe is just a good kitchen window
onto them.

[The one button on the page, ruddy style: **Open arecipe** →
https://arecipe.app ]

THE FIELD GUIDE

**Start without an account.** Open Alchemy, the drafting workspace, and start
writing. Drafts save to your device, and no sign-up wall stands anywhere in
the way. Publishing is the one act that asks you to sign in.

**Sign in to share and to keep.** Signing in with your Bluesky account (any
AT Protocol account works) is what publishes a recipe into your own data
store. That is what makes it shareable with your community, and what lets you
open it from your other devices.

**Browse and cook.** Browse shows recipes from the network. Your Cookbook
gathers your own recipes plus those of the cooks you follow, and every
cookbook has a public view that opens from a plain link.

**Compare versions of a dish.** Every dish can hold alternative versions side
by side: your grandmother's and the internet's, on one screen.

**Plan the week.** Meals is a weekly planner: assign recipes to days, then
repeat planned weeks onto the calendar for a full menu. Meal plans get public
share links too, so the household can just look.

An advanced, opt-in step publishes your plan as an ongoing calendar feed you
can subscribe to from Google Calendar.
[Link the words "calendar feed" → https://arecipe.app/calendar-setup.html ]

BUILT TO BE YOURS

There is no arecipe server. The app is a static bundle in your browser; your
recipes are records in your own data store; the same records already render on
recipe.exchange, a separate project, with no coordination between the two.
If arecipe disappeared tomorrow, your recipes would not. That is not just a
promise; it is an architectural guarantee, and the full reasoning has its own
page: **The treatise** at arecipe.croft.ing.
[Link "The treatise" → https://arecipe.croft.ing ]

ITS PLACE IN THE VALLEY

arecipe is the first working crop on croft.ing: proof that the plot model
holds, with something as unglamorous and beloved as the family recipe box.

THE BEDROCK

The app, the reasoning, and the source, in that order.

- Use it: https://arecipe.app
- Why it is built this way: https://arecipe.croft.ing
- Source: https://github.com/CroftCommunity/arecipe

Very bottom of the page, right-aligned, italic, small, `--granite`:

*arecipe: the a is for Amanda*

### skylite/index.html — copy verbatim

Title: `Skylite — croft.ing`

THE SIGNPOST

A tended window into the butterfly garden of Bluesky.

THE SURFACE

Skylite is a window into Bluesky for anyone who'd rather the view were
tended: no algorithm, no ads, no posting, no strangers. A sponsor tends a
garden of accounts and feeds worth seeing; the explorer just enjoys the sky.

WHAT IT IS TODAY

Skylite is under active construction, and the scaffolding is up and live at
skylite.croft.ing. A sponsor (whoever tends the garden) curates what appears;
an explorer (whoever the garden is for) gets a calm, chronological view of
just that, honoring the network's own content labels. No account is needed to
look, and in its default mode nothing about the explorer leaves the device.
[Link "skylite.croft.ing" → https://skylite.croft.ing ]

ITS PLACE IN THE VALLEY

Some explorers are children and their sponsor is a parent. Some are
grandparents who want exactly their grandkids' posts and not one thing more.
Some just want a quieter sky than the feeds will sell them. Skylite is the
gate in the wall, built at whatever height it is needed: the same Croft
conviction, that the person is not the product, extended to the people the
feeds treat as one most freely.

THE BEDROCK

- The window itself: https://skylite.croft.ing
- Source: https://github.com/CroftCommunity/skylite

## Phase 3: index.html growing-section update

In "What's growing now", edit in place:

- The arecipe entry: keep its existing description sentence; make the name
  "arecipe" link to `/arecipe/` and keep the https://arecipe.app link as its
  own "Use it" link.
- Add a Skylite entry between arecipe and The Plot, same markup shape (bold
  term, description sentences, no dashes):

  **Skylite** — name links to `/skylite/`. Description: A tended window into
  Bluesky, for kids, grandparents, and anyone who'd rather the view were
  curated by someone who cares. Under active construction, and live.
  "See it" link: https://skylite.croft.ing

- The Plot entry: unchanged.

Do not touch the hero, pillars, closing, or footer.

## Phase 4 (GREEN), README, summary

Run `python3 checks/check_site.py` to green. Update README with one line about
the guide pages and the checks harness. RUN-02-SUMMARY.md records: red output,
green output, files created/edited, any styles.css additions, and confirmation
that no existing page's copy changed except the growing section.

## Acceptance

Checks green; pages readable at 360px and desktop; zero scripts; zero loaded
external resources; existing pillar/tier sync untouched.
