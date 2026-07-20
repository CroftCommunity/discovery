# SKYLITE-DIRECTIVES amendment — RUN-BRAND (logos, palettes, theming)

`Status: EXECUTABLE amendment to SKYLITE-DIRECTIVES.md. Insert RUN-BRAND as the
first run; RUN-STRUCT gains the gate "RUN-BRAND merged" and consumes its tokens
and assets (S1 landing, S4 skins). All §0 standing conventions apply, TDD
first. Source assets: four owner-provided renders (light palette board, dark
palette board, light logo, dark logo), 2026-07-15.`

## Ground rules for the source assets

- The four renders are CONCEPT ART. Colors and marks are canonical; the mock
  UI inside the palette boards is NOT. The boards show a "New Post / Send
  Post" compose card: ignore it entirely. Skylite has no compose surface,
  ever; take the hexes, nothing else.

- The photoreal window icons are usable as RASTER assets (app icons, splash,
  favicon) via crop/resize. In-UI branding uses a clean SVG wordmark and
  simplified marks built in this run. Do not attempt to recreate the
  photoreal windows in SVG.

- Motif restraint: the monarch butterfly and contrail/constellation motifs
  may appear in splash, empty states, and the landing hero only. Never as
  chrome decoration on feed surfaces. Same restraint rule as caught-stars
  copy.

## Canonical palette (from the boards, verbatim hexes)

Light: Skylite Blue #2D8BCC, Monarch Orange #FF8C00, Twilight Navy #1C335C,
Cloud White #FFFFFF; supporting Aero Mist #F1F8FB, Sunbeam Yellow #FFDD00,
Aero Grey #A1BAC5, Sky Silver #C0CCD6.

Dark: Deep-Sky Blue #1976D2, Rich Monarch Orange #D35400, Dark Twilight Navy
#0E1A33, Charcoal Black #212121; supporting Deep Aero Mist #2A2A2A, Deep
Sunbeam Yellow #F1C40F, Steel-Mist Grey #3A3A3A, Gunmetal Black #1C1C1C.

## BR0 Relocate and canonicalize the source assets

The owner committed the five source renders to `icons/` with camera-roll
names. First task, before any derivation: `git mv` them to canonical paths
(verify each by CONTENT before renaming; the mapping below was made by eye
from the owner's screenshots):

- icons/1784122685740.png (light palette board) → assets/brand/source/palette-light.png
- icons/1784122882476.png (dark palette board) → assets/brand/source/palette-dark.png
- icons/1784123056842.png (light logo, day-window) → assets/brand/source/logo-light.png
- icons/1784123332879.png (dark logo, constellation) → assets/brand/source/logo-dark.png
- icons/1784125793370~2.png (sunset splash, capitalized wordmark) → assets/brand/source/splash-sunset.png

Write the PROVENANCE note (owner-provided, 2026-07-15) alongside. Then ensure
source art is EXCLUDED from the deployed bundle: the boards and logos total
several MB of concept art that must never ship in the PWA payload; only
derived, optimized assets enter dist/. Verify the build's copy rules
accordingly and assert it in a hermetic test (dist contains no file from
assets/brand/source/).

Flag for the summary: the splash source is 101 KB versus ~1 MB for the
others, and its "~2" name suggests an edited crop; derive what it supports
cleanly and note in the summary if the landing hero or large splash sizes
want a higher-resolution original from the owner.

## BR1 Design tokens (semantic, paired, tested)

CSS custom properties in one tokens file, semantic names mapped per theme
(raw hexes appear only in the token definitions, never in component CSS):

- `--bg` (Cloud White / Charcoal Black), `--bg-raised` (Aero Mist / Deep Aero
  Mist), `--bg-sunken` (Sky Silver / Gunmetal Black)

- `--ink` (Twilight Navy / Cloud White), `--ink-muted` (Aero Grey /
  Steel-Mist Grey text usage = verify contrast, adjust which grey serves as
  muted ink per theme)

- `--accent` (Skylite Blue / Deep-Sky Blue), `--cta` (Monarch Orange / Rich
  Monarch Orange), `--highlight` (Sunbeam Yellow / Deep Sunbeam Yellow),
  `--border` (Sky Silver / Steel-Mist Grey), `--navy` (Twilight Navy / Dark
  Twilight Navy, for wordmark and headers)

Tokens are defined as APPROVED PAIRS (fill + ink), not lone colors:

- CTA buttons: `--cta` fill with `--navy`-or-darker ink in light mode. WHITE
  TEXT ON #FF8C00 IS FORBIDDEN (≈2.2:1, fails WCAG AA 4.5:1). If white-on-
  orange is wanted aesthetically, the fill must darken until the pair passes;
  record the final pair in the summary.

- `--highlight` always carries dark ink (yellow never carries white text).

- Every pair used in components must pass AA: 4.5:1 body, 3:1 large
  text/icons.

TDD: contrast tests FIRST, red before the tokens file exists: a unit test
computes WCAG contrast for every declared pair and fails on any violation;
a lint-style test fails on raw hex literals in component CSS.
**Accept:** all pairs pass AA by test; no raw hexes outside tokens file.

## BR2 Theme mechanics (third axis, explorer-owned)

Axes stay orthogonal: `localOnly` (capability, sponsor), `skin` (simple/full,
sponsor), `theme` (light/dark, EXPLORER'S, device-local). Theme defaults to
`prefers-color-scheme`, manual override persisted locally (in the S5 backup
file), never in the config record. Both skins consume the same tokens; skins
differ in scale, spacing, and affordances, never in palette. `theme-color`
meta and manifest colors update per theme where supported.
TDD: behavior spec first: override sticks across reload; media-query default
honored when no override.
**Accept:** flipping OS dark mode flips an un-overridden install; the
override survives reload and appears in the backup export.

## BR3 Logo and icon pipeline

- SVG wordmark "Skylite" (navy `--navy`, matching the boards' type
  treatment) for headers and the landing hero; must render crisply at 24px
  and 200px.

- Splash source (owner-provided, 2026-07-15): the sunset butterfly-window
  render. Derivation rule: the butterfly window is lifted as centered artwork
  on a token background with generous margins; the wordmark is rendered
  separately from the SVG (never baked into the crop, so no device crop can
  cut it); portrait-safe crops per device class keep the butterfly whole.
  The full uncropped render is reserved for the landing-page hero. Whether
  startup images can theme via prefers-color-scheme media queries = verify
  in-run; if not, the sunset splash serves both themes (it bridges them).

- Wordmark case RULED (2026-07-15): "Skylite", capitalized, everywhere — the
  SVG wordmark and all UI copy. A revised splash render with the capitalized
  wordmark supersedes the lowercase one; use the revised render as the sole
  splash source (the lowercase version is not committed).

- Raster set from the provided renders: maskable icons 192/512 (safe-zone
  padded), apple-touch-icon 180, favicon set, iOS splash set
  (apple-touch-startup-image). LIGHT logo (daytime window) is the app icon
  everywhere (home screens don't theme); DARK logo (butterfly constellation)
  is used in-app in dark theme and on the dark landing variant. In-app header
  logo swaps with theme: day window in light, constellation in dark.

- Source renders stored under assets/brand/source/ with a PROVENANCE note
  (owner-provided, 2026-07-15); derived assets under assets/brand/ with the
  generation commands recorded so they are reproducible.

TDD: hermetic e2e asserts manifest validity, icon presence at declared sizes,
and per-theme header-logo swap.
**Accept:** installed PWA shows the window icon and a correct splash;
in-app logo follows theme.

## BR4 Apply to the live surface

Restyle the existing deployed pages with the tokens (this run does NOT build
RUN-STRUCT's pages; it makes what exists brand-correct so RUN-STRUCT starts
from a themed base). Version stamp stays visible.
**Accept:** live site reads as Skylite-branded in both themes with zero
contrast-test violations.

## Run mechanics

Branch `run-brand`. Gate for RUN-STRUCT becomes "RUN-BRAND merged". Summary
must include: final CTA pair ruling, any hex adjustments made to pass AA
(with before/after), and the asset generation commands.

## Verify-in-run additions
Muted-ink grey selection per theme (which supporting grey passes AA on which
background); maskable-icon safe-zone crop of the window renders (the frame
must not clip); iOS splash-set behavior with the dark render on OLED.
