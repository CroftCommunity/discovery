# TREATISE RUN-01: arecipe.croft.ing — the treatise site

Target repo: `CroftCommunity/arecipe_treatise`.
MANUAL PRE-STEP for the maintainer (before this run): create the repo, empty or
LICENSE-only (AGPL-3.0 to match the family), default branch `main`.

Execute on a fresh branch `run-01-treatise` off `main`. Write `RUN-01-SUMMARY.md`
at repo root when done. Do not merge; leave for review.

This is a one-page site in the croft.ing family. It inherits the crofting_site
stance wholesale, restated here so this file is self-contained:

## Guardrails

- STOP if the repo contains anything beyond a LICENSE and/or README.md.
- Plain HTML + one CSS file. No frameworks, no package.json, no build step, no
  JavaScript. Pages must render from `file://` and from a static server.
- Zero loaded external requests. Fonts self-hosted; no CDNs, no analytics, no
  external images. Outbound clickable links are fine and expected.
- TDD-first, per standing convention: Phase 1 writes the acceptance checks as a
  runnable harness BEFORE any page exists, runs it, and records the failure;
  pages are then built to green. Red-to-green order must be evidenced in the
  summary.
- Copy discipline: the treatise copy in this file is FINAL and reviewed. Place
  it verbatim. No embellishment, no exclamation points, no added claims.

## Phase 1 (RED): the checks harness

Create `checks/check_site.py`, Python 3 stdlib only, exit nonzero on any
failure. It must assert:

- `index.html` exists; `CNAME` exists containing exactly `arecipe.croft.ing`.
- No `<script>` tag in any HTML file.
- Every `http(s)` URL in HTML/CSS is either in the outbound-link allowlist
  (arecipe.app, croft.ing, arecipe.croft.ing, github.com/CroftCommunity,
  recipe.exchange, developer.mozilla.org) or is the SVG namespace inside a CSS
  data URI. Nothing else, and nothing is a loaded resource.
- The six act headings and the kicker text below appear, in order.
- `assets/fonts/` contains the four woff2 files and both OFL license texts.
- All internal hrefs and asset references resolve to files.

Run it now, before anything else exists. Record the failing output in the
summary. This file is committed and stays as the regression net.

## Phase 2: structure and design

```
index.html
styles.css
CNAME                 containing exactly: arecipe.croft.ing
assets/fonts/         Lora 500/600 + Inter 400/600 woff2 + OFL-Lora.txt + OFL-Inter.txt
assets/favicon.svg    three stacked stroked rectangles (drystone cairn), --schist, <1 KB
checks/check_site.py
README.md
RUN-01-SUMMARY.md
```

Fonts: latin-subset woff2 from fontsource (via jsDelivr npm artifacts) or
google/fonts; `@font-face` with `font-display: swap`; fallbacks
`Lora, Georgia, 'Times New Roman', serif` and
`Inter, system-ui, -apple-system, 'Segoe UI', sans-serif`. If fetching fails,
ship the fallback stacks, note it, continue.

CSS tokens (the tectonic palette, identical to croft.ing):

```css
:root {
  --schist:   #2F3539;
  --granite:  #A2A9AB;
  --ruddy:    #B75C34;
  --moss:     #3D6546;
  --ink:      #1C1E20;
  --canvas:   #E9E1D6;
}
```

Layout rules: background `--canvas`, text `--ink`, never pure white or black.
One centered prose column, `max-width: 42rem`, body 1.0625rem/1.7, headings in
Lora, links `--moss` underlined. Between acts, the drystone-course divider: an
inline-CSS SVG data URI, one row of 7–9 interlocking stroked rectangles of
varying widths, 1.5px strokes in `--granite`, ~220x14px, centered. No
animation. Readable at 360px.

Header: the wordmark "CROFT.ing" in Lora ("CROFT" in `--ink`, ".ing" in
`--ruddy`), linking to https://croft.ing/. One right-side link: "The user
guide" → https://croft.ing/arecipe/.

Footer, identical in spirit to croft.ing:

> Croft is built in the open. [Site source](https://github.com/CroftCommunity/arecipe_treatise) · [The app source](https://github.com/CroftCommunity/arecipe)
>
> AGPL-3.0. No cookies, no scripts, no tracking.

## Phase 3: the page (copy FINAL, place verbatim)

Kicker, small caps, `--granite`: A CROFT FIELD REPORT

# The recipe box that cannot turn on you

Lede paragraph, set slightly larger:

> arecipe is a small recipe app built as a working argument: that the slow
> decay of beloved software is, in large part, a design outcome, and that a
> different design makes software structurally resistant to it.

## Act I. A short story you already know

A recipe app launches. It is fast, kind, and free, and you pour a decade of
family recipes into it. It gets popular, then acquired. Ads arrive in the
search results, then between your own recipes. The features you rely on drift
one by one behind a subscription, or more likely only ever launch behind one.
The export button technically exists and half works. Eventually a sunset
notice thanks you for being part of the journey, and the recipes you
transcribed one winter to share with your family are now, and it becomes
clear always were, under the custody of a third party.

Nobody in that story need be a villain for it to be true. Cory Doctorow named
the pattern enshittification (ENS from here on, and across Croft's pages): a
platform is good to its users until they are locked in, then value is clawed
back, stage by stage, because the operator answers to someone other than the
users. The word is crude; the mechanism is not. It is structural, which is,
maybe unexpectedly, a reason for hope: structures can be refused.

## Act II. The four preconditions

arecipe's design docs distill ENS into four structural preconditions. Call
them unilateral control, lock-in, the data asset, and captive network
effects. The decay needs all four: an operator with unilateral control over
how the platform behaves; data lock-in that makes leaving expensive; the
users' aggregated data held as a proprietary asset; and network effects that
make individual defection irrational, because leaving means leaving everyone.

Remove any one and the pattern stalls. Remove all four and it cannot begin.
arecipe removes all four.

**No unilateral control.** An operator exists: someone owns the domain,
maintains the code, signs releases. But the levers are minimum-authority, and
anything the operator publishes, users can decline.

**No lock-in.** Recipes are records in your own data store on the AT Protocol,
in an open schema. Any conforming app renders them. The proof is live, not
promised: the same records already appear on recipe.exchange, a separate
project, with no coordination between the two. And the data store itself may
be hosted by Bluesky, or by you and your community; that is a personal choice.

**No data asset.** The application holds nothing. No database, no user table,
no analytics. There is nothing to monetize because there is nothing there.

**No captive network effects.** Your people are your Bluesky graph, not an
arecipe roster. Adopt a competing app and your recipes, your follows, and your
moderation choices can all come with you.

## Act III. The shape: a PWA with no back half

Two terms, then what they mean here. A single-page application (SPA) is an app
that runs entirely in your browser as scripts and pages, rather than asking a
server to think for it. A progressive web app (PWA) is a website that your
phone or desktop can install and treat like a native app: an icon, an offline
shell, a full window. arecipe is both, and the mix is the point: SPA means all
the logic ships to you and runs on your device; PWA means it feels like an app
worth living in.

What arecipe deliberately is not, is served. There is no application server
anywhere: the whole app is a static bundle of files, built in the open and
hosted as plain pages. Each destination is its own document rather than one
mega-app with a router, so the surface area stays small and legible. Your
browser talks directly to your own data store; nothing of yours passes through
an arecipe machine, because there isn't one.

## Act IV. Security, honestly

No server means no server to breach and no password database to leak, and it
also means every defense must ship inside the pages themselves. Sign-in is
the network's own OAuth, run entirely in the browser, and the resulting
credentials are sender-constrained: a stolen token is inert off the device it
belongs to, because using it requires a proof signed by a key that never
leaves that device.

That moves the realistic threat to one place: malicious script running inside
the app's own origin. So that is where the security budget goes: a strict
content-security policy carried in every document, integrity hashes on the
code the pages load, zero third-party scripts, and a build gate that loads
every page under the enforced policy and refuses to ship on any violation.

And the honest residuals, stated rather than hidden: a fully compromised
device is out of scope for any browser app, and script that does get inside
the origin could use keys in place even though it cannot steal them. The
controls shrink that surface; nothing reduces it to zero, and a page that
claimed zero would be exhibiting the exact confidence this project exists to
refuse.

## Act V. What you actually get

Practically, here is what the architecture gives you. Your recipes render in
other apps today, so exit is a live demonstration, not a clause in a promise.
The app installs like any other and works installed or not, public meal-plan
links included, and an opt-in step publishes your plan as a calendar feed your
household can subscribe to. There are no ads and there never can be, not as
policy but as plumbing: there is no data to target with and no chokepoint to
rent. And the project's ambition is deliberately countercultural: reach a good
interface, then let it be. Software that is not trying to extract value has no
motivation to churn the interface out from under you.

[Link "installs like any other" →
https://developer.mozilla.org/en-US/docs/Web/Progressive_web_apps/Guides/Installing ]

## Act VI. What this does not claim

Bounded claims are the whole posture, so here are the bounds. This design does
not resist an author walking away; abandoned software stays abandoned. It does
not resist a court order against the domain; the frontend would end, though
the records would survive. Public records can be indexed by anyone, which is
the price of credible exit. And the largest honest residual is governance:
one person still owns the domain, holds the signing key, and decides what
ships. That is not ENS, since there is no data, rent, or chokepoint to extract
through, but it is concentrated authority, and the design keeps the door open
to sharing it out over time.

## The gate

One framing line: The app, the guide, the ground, and the source.

- The app itself: https://arecipe.app
- The user guide: https://croft.ing/arecipe/
- The homestead this grows on: https://croft.ing
- Source, philosophy, security posture, in full: https://github.com/CroftCommunity/arecipe
- The same recipes, rendered elsewhere: https://recipe.exchange

Very bottom of the page, right-aligned, italic, small, in `--granite`:

*arecipe: the a is for Amanda*

## Phase 4: README and summary

README: what this repo is (the arecipe treatise at arecipe.croft.ing, part of
the croft.ing family), the no-dependency stance, how to preview, how to run
`checks/check_site.py`.

RUN-01-SUMMARY.md records: the Phase 1 red output and the final green run;
files created; font sourcing outcome; and the manual follow-ups: (1) Pages =
deploy from branch, main, root; (2) custom domain `arecipe.croft.ing` in Pages
settings; (3) DNS CNAME record `arecipe` → `croftcommunity.github.io` at the
registrar; (4) Enforce HTTPS once the certificate issues.

## Acceptance

`python3 checks/check_site.py` exits 0. Zero script tags, zero loaded external
resources, copy verbatim, readable at 360px and desktop.
