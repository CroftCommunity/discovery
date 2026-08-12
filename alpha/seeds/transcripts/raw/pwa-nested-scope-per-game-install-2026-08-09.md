# PWA nested scope — per-game install vs whole-shelf install, and why Firefox lags (Claude, 2026-08-09)

- **Source:** a claude.ai dialogue the owner pasted into a Claude Code session in `CroftC/` on
  2026-08-09. Four turns: the nested-scope mechanics question, the per-browser outcomes, and a
  history of Mozilla's PWA position.
- **Preservation status:** **preserved-condensed (cleaned-paste, content-faithful — §4).** The
  owner's questions are near-verbatim; the assistant's turns are reproduced content-faithfully with
  UI render chrome stripped: the collapsed thinking-summary headers (which appeared **duplicated**
  in the paste — reproduced once, as they are render chrome, not content) and the trailing citation
  chips (`w3`, `web.dev`, `Brainhub + 2`, `alternativeto`, `OMG! Ubuntu`, `Testmuai`, `XenForo`,
  `Mozilla Bugzilla`). Citation *attributions* are retained inline in prose where the assistant
  named a source. No canonical export exists — this is the best-available raw.
- **Credentials:** none present; none redacted.
- **Verification status:** **every platform claim below is `[UNVERIFIED]`.** This is a
  model-generated dialogue whose citation chips could not be followed from here. The load-bearing
  ones are flagged inline. **FACTCHECK does not bear:** the standing fact-check file covers
  atproto / iroh / **iOS-P2P**; this transcript's iOS content is Safari PWA install / web-clip
  behaviour, a different subject. It was not consulted and must not be cited as though it settles
  these claims.
- **Why this file matters beyond the dialogue:** it prompted the discovery that **`fun.croft.ing`
  has no PWA wiring at all** (no manifest, no service worker, no registration anywhere in the
  repo), so the question as asked — "how should I arrange my manifests" — rests on a premise that
  does not hold. See "Grounding findings" at the end, which are session findings, **not** part of
  the pasted dialogue.

---

## Turn 1 — the question

> Is there a way to set it up so that each individual game is installable as a PWA on
>
> https://fun.croft.ing/solitaire/
>
> Is there a way to ALSO allow installing the full site then as well so it's an either or?

**Assistant.** Mechanically yes, and the "either/or" part comes free because **each HTML page links
exactly one manifest**: a page cannot offer multiple installable manifests, and there is no way for
site UI to trigger installation of a specific one. So the shelf homepage links a root manifest
(`id: "/"`, `scope: "/"`) and each game page links its own. Where the user hits install determines
which app they get, and **distinct `id` values keep the installs separate** even though scopes
overlap.

The catch is that this is exactly the nested-scope configuration Chrome's guidance rates worst:
separate origins recommended, same-origin non-overlapping paths not recommended, overlapping/nested
paths strongly not recommended. The concrete symptoms, per the same web.dev doc:

- With the outer app installed but a game **not** installed, game URLs opened from the outer app
  just stay inside the outer app, and links to the game will not link-capture into the game app
  since they are considered within the outer app's scope.
- On Android/ChromeOS, if the shelf ships as a **Trusted Web Activity** the outer app captures all
  links, and even with a game installed the OS still offers to open it in the outer app.
- A user who installs both gets ambiguity: tapping a `/solitaire/` link may open in the shelf app
  rather than the solitaire app, depending on platform.

For a games shelf, that failure mode is arguably tolerable. Solitaire opening inside the installed
shelf app is odd but not broken, since the shelf contains the games anyway. The clean escape hatch
if it annoys you later: **move the shelf app (or the games) to a subdomain** so no scope contains
the other.

One layer note: the manifest spec does not forbid any of this; the link-capturing behaviour above is
**Chromium's deployment choice**, and Safari/Firefox handle install surfaces differently again.

## Turn 2 — per-browser outcomes

> Ok so what's the result in those

**Assistant.** Short version: the nested-scope headaches are mostly a **Chromium** problem. Safari
and Firefox do not do install prompts or link capturing, so the either/or resolves itself by
"whichever page you were on when you installed."

**Safari, iOS.** No automatic install prompt; the user manually uses Add to Home Screen from the
Share sheet. The manifest linked from the current page is what gets installed, so the shelf page
installs the shelf and a game page installs the game, with nothing arbitrating between them.
`start_url`, `scope`, icons, and service-worker caching do work, and **scope behaves as presentation
only**: navigating out of scope renders the page in an in-app browser rather than your app window.
Each icon is effectively an independent web clip. *(Assistant noted it could not pull an Apple
primary source on link routing; the widely reported behaviour is that links tapped elsewhere on iOS
open in the browser rather than in an installed web app, so the outer-app-captures-inner-links
conflict does not exist there.)* Nested scopes cost nothing on iOS beyond the user needing to visit
each game to install it.

**Safari, macOS.** Add to Dock works the same way: per-page, manual, no scope arbitration.

**Firefox, desktop.** As of **Firefox 143** you can pin sites as web apps to the taskbar in
simplified windows, but Mozilla is explicit this is **not full PWA support**: the browser will not
implement the PWA spec's capabilities, and web-app windows keep the address bar and toolbar. It is
per-site pinning, so again no manifest-scope conflict, just a less app-like result.

**Firefox, Android.** Offers Add to Home Screen using the current page's manifest, same per-page
logic as Safari. *(Assistant flagged its sourcing here as blog-level corroboration only, not Mozilla
docs.)*

**Net effect:** ship both manifests. Chromium users get the messy-but-tolerable nesting behaviour;
everyone else gets clean per-page installs by construction.

## Turn 3 — why Firefox lags

> Why is Firefox behind on PWA?

**Assistant.** Two distinct things, often conflated: a **product decision** about install UI, and a
**standards stance** about app-like capabilities.

**The product decision.** Firefox had a desktop install feature (SSB, Site Specific Browser: run a
site in its own chromeless window) as an experiment, and killed it in **2021**. The stated reasons,
from the removal bug: developers found "little to no perceived user benefit" and multiple known bugs
blocked further development, and the feature was "costing us time in terms of bug triage" while its
presence "sent the wrong signal that this is a supported feature." A Mozilla developer stated flatly
that "there is currently no plan for PWA support in Firefox," and even the engineer who wrote most of
the code said he was frustrated to see it removed but that it was the decision that made sense at the
time. *(Assistant marked the following explicitly as its own inference:)* a resource-constrained org
triaging a half-finished feature against Chrome's mature one, in a period when Mozilla was doing
layoffs.

**The standards stance.** Separately, Mozilla has been skeptical of the capability APIs Chromium
ships under the PWA/Fugu umbrella. Concretely, Mozilla published a **"harmful"** standards position
on the File System Access API's local-disk pickers and has never shipped them, while still shipping
the sandboxed Origin Private File System. So Firefox lags on install UX **by choice** and on powerful
APIs **on security/privacy grounds** — different layers. Firefox's service worker, manifest parsing,
and offline support are fine.

**The 2025 reversal** is consistent with both: Firefox 143 shipped taskbar web apps, but Mozilla's
product manager said the goal is a more app-like experience for websites *without pretending you have
left Firefox* — explicitly not implementing the PWA spec's full feature set, keeping the toolbar and
address bar. That is a philosophical position: web apps are still websites, and hiding that is the
part Mozilla declines to do.

---

## Grounding findings (this session, NOT part of the pasted dialogue)

Checked against the real repos on 2026-08-09. These are the reason the dialogue's advice cannot be
applied as written.

1. **`fun.croft.ing` is not a PWA at all.** No `*.webmanifest`, no `manifest*.json`, no
   `rel="manifest"`, no `serviceWorker` registration anywhere in `fun/` (searched `src/`,
   `build.mjs`, `dist/`). The dialogue answers "how should I arrange my manifests" as though
   manifests exist. The real work is adopting a PWA chassis, which is materially larger.

2. **The dialogue's concrete recommendation violates a hard house standard.** It advises `id: "/"`,
   `scope: "/"` — absolute-root paths. `croft-pwa/docs/PRACTICES.md` §"Relative paths
   (subpath-portable)" names the manifest's `start_url`/`scope`/icons explicitly, and the croft-pwa
   **build fails** on an absolute-root path (`docs/PRACTICES.md:24-42`). This is a gate, not a style
   preference. Recorded as COHESION §69.

3. **Relative paths make the nested design cleaner, not harder.** A manifest at
   `/<game>/manifest.webmanifest` with `start_url: "./"` and `scope: "./"` yields scope `/<game>/`
   with no per-game absolute path baked in. And `fun/build.mjs` emits all 20 pages through a
   **single `page()` template function**, so the injection point is one function.

4. **`[UNVERIFIED]` and load-bearing: the `id` claim.** The whole either/or rests on "distinct `id`
   values keep the installs separate." Per the manifest spec, `id` is resolved against the **origin**,
   not against the manifest's own path — unlike `start_url` and `scope`. So a relative `id: "./"` may
   not track the directory the way the other two do, which is exactly where the house relative-path
   rule and this design could collide. **Not independently verified this session** (per the
   don't-guess-at-spec-behaviour rule) — it is cheap to settle empirically: build two manifests,
   install both, inspect the install list.

## Owner decision (2026-08-09)

Nested scope is the owner's preference — *"separate when installed isolated, together when installed
at the top level"* — **conditional on it working**. That condition maps exactly onto finding 4, so
the `id` check is a **gate** in the build plan, not a footnote: verify separation on two manifests
before committing to all 20 games. The subdomain escape hatch (finding: `solitaire.fun.croft.ing`)
is the recorded fallback, not a surprise.

## Not earned by this transcript

**`ECOSYSTEM.md`** — Mozilla, Chromium/Google, and Apple appear here as **platform substrate**, not
as peers in the homage / build-on / partner / rebroadcast / learn↔ sense the register tracks. No row
added, deliberately.
