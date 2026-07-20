# RUN-BRAND — Claude Code instructions (self-contained)

You are executing a brand-integration run on `CroftCommunity/skylite`, the
kid-safe read-only Bluesky PWA at skylite.croft.ing. The app already exists
and is deployed (static bundle, strict TypeScript + esbuild, no framework,
page-per-destination, Vitest + Playwright, GitHub Actions Pages deploy).
Your job in this run: take the five owner-provided brand renders already
committed under `icons/`, canonicalize them, build the design-token and
theming system, produce the full PWA icon/splash pipeline, and restyle the
existing surfaces so the app is Skylite-branded in light and dark themes.
Do not build new product features in this run.

## Conventions (non-negotiable)

1. **TDD, always.** Every phase starts by encoding its acceptance criteria
   as FAILING tests before implementation. Fixtures before features. The
   run summary must evidence red-to-green order per phase (test commit
   precedes or accompanies the first implementation commit). Visual polish
   is exempt from pixel assertions; behavior and invariants are not.
2. Work on a fresh branch `run-brand`. Never push to main. Finish with a
   PR and a `RUN-BRAND-SUMMARY.md`.
3. Hermetic test gate stays green at every commit that claims a phase:
   lint + typecheck + unit + build + hermetic e2e. Nothing in CI touches
   the network. No credentials anywhere in the repo.
4. No new dependencies without listing each in the summary with a reason.
   Prefer zero new runtime dependencies; build-time image tooling is
   acceptable if listed.
5. These files are READ-ONLY: README.md, CONCEPT.md, IDEAS.md,
   PROVENANCE.md, seeds/. Do not edit them.
6. Stop rule: if an instruction below is ambiguous in a way that changes
   the outcome, halt THAT item, record the question in the summary, and
   continue with the rest of the run.
7. The two palette-board renders contain a mock "New Post / Send Post"
   compose card. That is decorative concept art. Skylite has NO compose,
   posting, or reply surface, ever. Take colors from the boards; take no
   UI semantics from them.

## Phase 0 — Canonicalize the source assets

The owner committed five renders to `icons/` with camera-roll names.
Verify each file BY CONTENT (open and look) before renaming, then `git mv`:

- light palette board (silver header, blue-sky, "Concept Light mode")
  → `assets/brand/source/palette-light.png` (expected: icons/1784122685740.png)
- dark palette board (dark header, night clouds, "Concept Dark mode")
  → `assets/brand/source/palette-dark.png` (expected: icons/1784122882476.png)
- light logo (day-sky window, monarch butterfly + contrail loop, "Skylite")
  → `assets/brand/source/logo-light.png` (expected: icons/1784123056842.png)
- dark logo (butterfly-shaped star constellation in night-sky window)
  → `assets/brand/source/logo-dark.png` (expected: icons/1784123332879.png)
- sunset splash (leaded-glass butterfly window over mountain valley,
  capitalized "Skylite" on the wall)
  → `assets/brand/source/splash-sunset.png` (expected: icons/1784125793370~2.png)

Add `assets/brand/source/PROVENANCE.md`: owner-provided concept renders,
2026-07-15; canonical color hexes live in the tokens file, not read from
these images.

**Bundle hygiene (test-first):** write a hermetic test that FAILS if any
file from `assets/brand/source/` appears in the built `dist/`. The source
renders total several MB and must never ship in the PWA payload; only
derived, optimized assets enter dist/. Adjust build copy rules until green.

## Phase 1 — Design tokens (semantic, paired, contrast-tested)

Canonical palette (these hexes are authoritative):

Light: Skylite Blue #2D8BCC, Monarch Orange #FF8C00, Twilight Navy #1C335C,
Cloud White #FFFFFF, Aero Mist #F1F8FB, Sunbeam Yellow #FFDD00, Aero Grey
#A1BAC5, Sky Silver #C0CCD6.

Dark: Deep-Sky Blue #1976D2, Rich Monarch Orange #D35400, Dark Twilight
Navy #0E1A33, Charcoal Black #212121, Deep Aero Mist #2A2A2A, Deep Sunbeam
Yellow #F1C40F, Steel-Mist Grey #3A3A3A, Gunmetal Black #1C1C1C.

Create ONE tokens stylesheet defining CSS custom properties per theme with
semantic names; raw hexes may appear only in this file, never in component
CSS. Semantic roles (map per theme; adjust the grey assignments per the
contrast tests):

- `--bg` (Cloud White / Charcoal Black), `--bg-raised` (Aero Mist / Deep
  Aero Mist), `--bg-sunken` (Sky Silver / Gunmetal Black)
- `--ink` (Twilight Navy / Cloud White), `--ink-muted` (pick the grey that
  passes contrast on each theme's backgrounds)
- `--accent` (Skylite Blue / Deep-Sky Blue), `--cta` (Monarch Orange /
  Rich Monarch Orange), `--highlight` (Sunbeam Yellow / Deep Sunbeam
  Yellow), `--border` (Sky Silver / Steel-Mist Grey), `--navy` (Twilight
  Navy / Dark Twilight Navy)

Tokens are approved PAIRS (fill + ink), not lone colors. Hard rules:

- WHITE TEXT ON #FF8C00 IS FORBIDDEN (≈2.2:1; fails WCAG AA). CTA buttons
  in light mode use `--cta` fill with `--navy` or darker ink, OR darken the
  CTA fill until a chosen ink passes 4.5:1. Record the final CTA pair, with
  before/after ratios, in the summary.
- `--highlight` yellow always carries dark ink. Never white-on-yellow.
- Every pair used by any component must pass WCAG AA: 4.5:1 body text,
  3:1 large text and meaningful icons.

**Test-first:** before writing the tokens file, write (a) a unit test that
parses the declared pairs and computes WCAG contrast, failing on any
violation; (b) a lint-style test failing on raw hex literals in component
CSS outside the tokens file. Both start red, end green.

## Phase 2 — Theme mechanics

Theme (light/dark) is a device-local, explorer-owned setting; it never
syncs anywhere and is a third axis independent of any capability or skin
switch. Default follows `prefers-color-scheme`; a manual override control
persists locally (localStorage/IndexedDB). Update `theme-color` meta per
active theme. Both themes are served by the same pages via the token file;
no per-theme stylesheets forked by hand.

**Test-first:** behavior specs: (a) with no override, flipping the emulated
color-scheme flips the UI; (b) the manual override survives reload and wins
over the media query; (c) `theme-color` meta matches the active theme.

## Phase 3 — Wordmark, icons, splash

- **SVG wordmark "Skylite"** (capitalized; this is ruled, everywhere) in
  `--navy`, matching the boards' type treatment. Must render crisply at
  24px and at 200px. Used in headers and the landing/app hero.
- **Raster icon set** derived from `logo-light.png` (home screens don't
  theme, so the day-window logo is the app icon everywhere): maskable
  192 and 512 PNGs (pad so the window survives the safe zone without the
  frame clipping), apple-touch-icon 180, favicon set. Wire into the web
  manifest; keep the manifest valid.
- **In-app header logo swaps with theme:** day-window mark in light,
  constellation mark in dark (derive small crisp crops from the two logo
  renders; the wordmark next to them is the SVG, not baked pixels).
- **Splash (PWA):** derive from `splash-sunset.png` with these rules: lift
  the butterfly window as centered artwork on a token background with
  generous margins; the wordmark is rendered from the SVG, NEVER baked into
  the crop, so no device crop can cut it; generate the
  `apple-touch-startup-image` set portrait-first per current device
  classes, keeping the butterfly whole in every crop. Investigate whether
  startup images can vary by `prefers-color-scheme` media queries; if yes,
  offer a dark variant (constellation logo on `--bg` dark) and the sunset
  for light; if not, the sunset serves both themes. Record the finding.
  The full uncropped sunset render is reserved for the landing hero later;
  do not consume it destructively.
- The splash source is ~101 KB (smaller than the other renders and likely
  an edited crop). Derive what it supports cleanly; if the largest splash
  sizes upscale poorly, say so plainly in the summary so the owner can
  supply a higher-resolution original.
- Motif restraint: butterfly/contrail/constellation imagery may appear in
  splash and empty states only in this run. Never as decoration on feed
  chrome.
- Asset generation must be reproducible: record the exact commands (or
  script them under scripts/) in the summary.

**Test-first:** hermetic e2e asserting manifest validity, presence and
declared sizes of all icons, presence of the startup-image link tags, and
the per-theme header-logo swap.

## Phase 4 — Apply to the live surfaces

Restyle the EXISTING pages with the tokens and assets (this run builds no
new pages): backgrounds, text, buttons, borders, header with wordmark +
theme-swapped mark, theme toggle exposed somewhere sensible and low-key.
The visible version stamp stays. Both themes must show zero contrast-test
violations on real components (extend the Phase 1 test to sweep rendered
computed styles in e2e if feasible; otherwise assert the pair discipline
statically and note the limit).

## Summary requirements (RUN-BRAND-SUMMARY.md)

Red-to-green evidence per phase; the final CTA pair ruling with ratios;
any hex adjustments made to pass AA (before/after); which grey serves as
`--ink-muted` per theme and its measured ratios; asset generation commands;
the splash `prefers-color-scheme` finding; the splash-resolution adequacy
flag; new dependencies with reasons; any halted items with the open
question stated.
