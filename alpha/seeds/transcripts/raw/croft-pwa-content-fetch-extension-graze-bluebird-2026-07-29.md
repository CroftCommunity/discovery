# Raw: PWA/SPA cross-origin content-fetch (extension model) · "Graze by Croft" forum naming · Skylite→Bluebird rebrand — design dialogue (2026-07-29)

**Preservation status: preserved-condensed (cleaned-paste, content-faithful — NOT byte-pristine) —
PLAYBOOK §4.** Source: a claude.ai design dialogue pasted into the CroftC session on 2026-07-29. UI
render chrome stripped (the italic one-line "thinking summary" banners — e.g. "Investigated minimal
GitHub App setup…", "Architected…"; the trailing "Run bluebird plan / Document · MD" artifact marker).
Long assistant passages condensed with load-bearing claims retained. No secrets.

**SCOPE REDACTION (not a fidelity loss).** The pasted block also contained an unrelated **life360 /
prodsec-evidence** thread (Slack Workflow Builder → GitHub `workflow_dispatch`; fine-grained PAT vs a
GitHub App with auto-rotating installation tokens; `--author`/`committer` and `Co-authored-by` commit
attribution). That thread is **work-scoped, not Croft**, and is **deliberately excised** from this raw —
CroftC repos are Croft-scoped (top-level `CLAUDE.md`: "repo content belongs in its repo"). It is marked
`[EXCISED — life360 prodsec, out of Croft scope]` at its position below. One transferable idea is noted
in the distillation, nothing more.

**VOLATILE / UNVERIFIED.** Browser-platform claims in the content-fetch thread — that `http://localhost`
is a "potentially trustworthy" origin exempt from HTTPS mixed-content blocking, current Chrome
**Private Network Access** preflight rules, and Safari's stance — are `[UNVERIFIED]` and are exactly what
the extension spike (ROADMAP_TODO E72) exists to settle. Name-clearance claims for **Bluebird** (the Amex
"Bluebird" prepaid card; the legacy **Bluebird.js** promise library) are `[UNVERIFIED]` and tracked as a
clearance gate in `NAMING.md`. atproto/iroh/iOS baseline facts defer to the FACTCHECK SoT
(`atproto-atmospheric-web-iroh-mobile-FACTCHECK.md`); overlapping social-tree claims defer to
`research/2026-07-27-social-tree-factcheck.md` / `-2.md`. **This dialogue is the direct continuation of the
2026-07-27 social-tree / forum-reader-PWA body** (`bsky-reddit-forum-social-tree-pwa-2026-07-27.md`, E62)
— it extends that lineage; it is not net-new.

---

## Thread A — a user-friendly way for a PWA/SPA to fetch reader / API content

**User:** Is it at all possible to give a shortcut or a quick way for users to run a shell command to run
a local proxy service from a PWA/SPA so it can get [RSS/H]reader or API content? A user-friendly workflow?

*(No answer to this question appears in the pasted transcript — the paste jumped to the excised life360
thread. The question was carried live into the 2026-07-29 CroftC session and resolved there; see
"Working-session resolutions" at the end of this file.)*

---

## [EXCISED — life360 prodsec, out of Croft scope]

Two exchanges on triggering GitHub `workflow_dispatch` from a Slack Workflow Builder "Send a Web Request"
step: fine-grained PAT (one repo, `actions:write`, 90-day expiry) vs a GitHub App with auto-rotating
1-hour installation tokens (`actions/create-github-app-token`), the hybrid "minimal-trigger-PAT + App-token
does the real work" pattern, and commit attribution via `--author` (author vs committer identity) /
`Co-authored-by` / writing the reactor identity into the evidence JSON. **Not filed** — work topic,
`life360/prodsec-evidence`, unrelated to Croft.

---

## Thread C — naming the community-discussion forum: "Graze by Croft"

**User:** Need a name for a forum dedicated to community discussion in line with Croft and Drystone, but
not necessarily bound by that naming scheme directly — just in the same spirit.

**Assistant:** **"Graze by Croft"** is a good lockup. The "by X" construction lets Graze be its own place
with its own personality while Croft signs the work (cf. "Threads by Instagram", "Arc by The Browser
Company"). It answers "who runs this / why trust it" without making the forum feel like a feature of the
parent. It also dissolves the subdomain worry: the affiliation lives in the wordmark, so `graze.ing`
stands alone, "by Croft" appears in the logo lockup / footer / about page, and if Graze ever needs to
stand fully apart you drop two words instead of migrating a domain.

Imagery that stays accurate to real pastoral life:
- **Sheep as members.** Flocks mix on common grazings and are sorted at the **fank** (sorting pen).
  Avatars as ear-tag colors; communities as **hefts** (a heft = the patch of open hill a flock learns and
  passes to its lambs with no fence — territory held by culture, not walls).
- **Ruminating.** Grazing animals chew twice. "Still ruminating" as a draft state; slow-mode threads as
  cud-chewing — makes digestion the joke, keeps "graze = shallow" at bay.
- **The shepherd, not the algorithm.** Moderators as shepherds; the crook as mod-tool iconography.
- **Seasonal rhythm.** Transhumance (moving stock to hill pastures in summer) → a native frame for
  seasonal events / rotating featured communities.
- One accuracy line: keep it **working-landscape, not twee** — a hill farm, not a petting zoo. Croft and
  Drystone both name functional things people build and maintain.

**User:** Agreed, and I don't want to go too far with themes or branding feedback loops. But "welcome to
the herd" / "have you herd?" kind of stuff is warm and fun, and using some of the actions and the commons
tie-in from Croft.ing is really interesting.

**Assistant:** Right calibration — the theme earns its keep in small, high-warmth moments and stays out of
the way everywhere else (navigation, settings, thread UI stay plain so the metaphor is never a puzzle you
must decode to use the site). Where a light touch lands:
- **Onboarding:** "Welcome to the herd" as the signup confirmation, once, then never again.
- **Notifications digest:** "Have you herd?" as the activity-roundup subject line — a pun with a job.
- **Empty states / 404s:** "Nothing grazing here yet" for an empty community; a lone sheep on the 404.
- **The Croft tie-in:** the one structural link worth keeping is the **commons framing** — your croft is
  yours, Graze is the shared ground. An about-page sentence and maybe a cross-nav label ("out to the
  common"), not a system of terminology. Members, posts, communities keep ordinary names.
- **Feedback-loop trap** (rightly wary): when the community generates in-jokes about the theme and the
  product absorbs them back, new users hit a wall of shepherding vocabulary that gatekeeps. Guardrail: the
  theme lives in copy the team writes at the edges, never in the nouns users must learn. If someone can
  use Graze for a month and describe it to a friend without mentioning sheep, the volume is right.

**User:** I suppose `/g` could work too, but I still don't like it as much as `/f` yet.

**Assistant:** `/f` is the right call — path conventions work by **inheritance, not branding**. `/f` reads
instantly because "f = forum" is a learned pattern (4chan boards, Reddit `/r/`, Facebook `/groups`); the
letter maps to the *thing*. `/g` maps to the *brand* (Graze) — it only makes sense after you know the
brand, and the domain already says Graze, so `/g/gardening` says it twice. `graze.ing/f/gardening` parses
with zero context. Small collision argument too: `/g/` carries a 4chan technology-board connotation for a
chunk of internet-native users. The only case `/g` earns reconsideration: if the in-product noun for a
community became something g-flavored ("grounds"/"grazings") — but that cuts against keeping user-facing
nouns ordinary, so it argues itself out. `/f`, ordinary nouns, sheep at the edges.

**User:** Agreed.

**Assistant:** Stack settled: **Graze by Croft**, at **graze.ing**, forums under **/f/**, commons framing
on the about page, herd puns confined to onboarding / digests / empty states. Open action item: the
**registration** itself — verify the renewal price matches the registration price at checkout (standard
tier), and grab it before someone notices "graze" is sitting there cheap.

---

## Thread D — Skylite is taken; rebrand to "Bluebird"

**User:** Ugh — "Skylight" is already taken as a well-known atmosphere project name.
`https://github.com/CroftCommunity/skylite`. Need to think of an alternative.

**Assistant (ski-themed rebrand):** Treat the mountain as an **information architecture**, not a coat of
paint.
- **Session shape:** arrive at the **Lodge** (home), glance at the **Trail Map** (what your Patrol has
  open), ride a **run** top to bottom. A run has a top and a bottom — a finite groomed trail *ends*,
  unlike a feed ("You've reached the base, nice turns today") — the calm read-first ethos made literal.
- **Corduroy** (the ribbed texture a grooming machine leaves) as the **new-content affordance**: fresh
  items get a corduroy strip that fades once read. New-ness without badges/counts.
- The "on this device only" switch = **Cabin Mode** ("what happens in the cabin stays in the cabin").
  **Avoid "backcountry"** — in ski vocabulary that means *unpatrolled and dangerous*, the wrong valence
  for the safest mode.
- **Signage is a free design system:** real trail signs = rounded rect + icon + name + difficulty marker.
  Every list item / nav element can be a trail sign. **Green circle / blue square / black diamond** give a
  trust-gradient iconography at zero cost.
- **Voice:** patient ski instructor. "No fresh snow yet, check back tomorrow" (empty), "Whiteout, hang
  tight" (error). Onboarding = **Ski School**; first-run = the **Bunny Slope**.

Branding: palette falls out of the name — **bluebird sky**, snow white, evergreen, plus one **safety-orange
accent reserved for Patrol/sponsor surfaces** (mirroring how real patrol wears the one loud color). The
mark stays the **slalom flight-path** line with a small winged creature at the bottom — ambiguous between
bird and butterfly on purpose; the same line becomes the loading animation drawing itself downhill.
Typography: chunky, rounded, high-contrast (routed wooden trail signs) — also the accessible choice for
young readers, so aesthetic and audience point the same way.

Transforming the existing site (mapped from the repo's file list; page purposes **inferred from
filenames**): `index`→Lodge, `telescope.html`→`trailmap.html`, `saves.html`→`locker.html`,
`sponsor.html`→`patrol.html`, guide/help→Ski School, `mysky.html`→`my-mountain`. Keep old URLs as
redirects (installed PWA users). Lexicons: leave existing `ing.croft.skylite.*` records where they are;
mint new `ing.croft.bluebird.*` **only when a schema actually changes**. Display names rebrand
immediately; NSIDs baked into records do not — the two layers may disagree indefinitely (the
protocol-vs-product seam). Do it as a **RUN-BLUEBIRD** (red→green like the others). Provenance discipline
already solves the awkward part: `CONCEPT.md` and Skylite-era docs are **frozen historical seed**, so the
old name stays in the record untouched and the rebrand is an *event in history*, not a revision of it. One
honesty note: the manifest rename changes the installed app's identity, and how fast an installed PWA
picks up a new name/icon varies by platform — **verify current iOS behavior** `[UNVERIFIED]` before
promising existing explorers a seamless switch.

**User:** This is still alpha, no user content to keep, greenfield.

**Assistant:** Greenfield deletes every caution. New priority order — **do now, only free once:**
1. **Lexicon NSIDs.** Records make NSIDs sticky; you have no records, so this is the one window where
   `ing.croft.bluebird.*` costs nothing. Rename the schemas in `lexicons/` before anything writes a
   record — the protocol and product layers never have to disagree at all.
2. **Namespace grabs.** The name is "clear" only until someone else in the ATmosphere likes it too. Grab
   in one sitting: the Bluesky handle, the domain (on atproto the domain doubles as identity — pick it
   with the handle in mind), and the GitHub repo. Searches showed no atproto "Bluebird" `[UNVERIFIED]` —
   a snapshot, not a reservation.
3. **Repo rename, not migration.** `CroftCommunity/bluebird` (or `bluebird-app`, since GitHub search for
   plain "bluebird" surfaces the old Bluebird.js promise library). GitHub forwards renamed repo URLs
   automatically. Skylite-era `CONCEPT.md`/seeds stay in-tree as frozen seed; the rebrand is the next run
   on top.

Simplified because there's no one to migrate: no URL redirects needed beyond taste (pages born as `lodge`,
`trailmap`, `locker`, `patrol`, `ski-school`); the PWA-manifest identity question evaporates (born named
Bluebird — no need to verify iOS reinstall behavior); CNAME points wherever you want on day one. One
design decision cheaper now than ever again: **bake the trail-rating trust gradient into the schemas, not
just the UI** — if a garden-membership or search-scope record carries an explicit `tier` field
(green/blue/black enum), the safety model is legible at the protocol layer and future tooling reads it for
free. Retrofitting a trust tier into records after launch hurts. Collapses to **one run, RUN-BLUEBIRD:**
lexicons, pages, manifest, brand tokens, copy in a single pass behind the existing gate.

**User:** Domain and repo changed. Now give me another round of thinking on rebranding and identity, then
write out the comprehensive step-by-step transformation for Claude Code to execute.

**Assistant (round three — identity):**
- **Two-layer vocabulary, on purpose.** The README's principle: vocabulary is **role-based, never
  age-based**. Keep **sponsor** and **explorer** as durable role nouns in schemas and legal-ish docs; let
  the mountain terms (**Patrol, Ski School, Lodge**) be the UI skin over them. Skins can change in a
  future rebrand; role nouns baked into schemas shouldn't have to. Same protocol-vs-product seam, applied
  to words.
- **Define what a trail rating means before anyone else does.** To outsiders, "black diamond" could read
  as a content-maturity rating. It isn't — it's **trust distance**: green = inside the tended garden, blue
  = one hop beyond, black = the open network. One canonical sentence in the docs (same treatment as the
  bluebird etymology sentence) so the safety model can't be mischaracterized.
- **The icon is not the logo.** The slalom flight-path is a wordmark companion; at 48px it's noise. The
  app icon should be the simplest read: a **bluebird silhouette on a green circle** (smuggles the
  easiest-run marker into the icon). The butterfly survives as the creature at the bottom of the full mark
  — an Easter egg for the garden era, never explained.
- **Name lockup.** Plain "Bluebird" is crowded in search (Amex card, Bluebird.js) `[UNVERIFIED]`. Fix is
  the **lockup, not the name**: **"Bluebird for Bluesky"** in store listings / page titles; bare
  "Bluebird" everywhere inside the product. The metaphor never appears in the lockup; it appears the
  moment you're inside.
- **Retires vs survives:** *garden* and *telescope* retire from the product (frozen docs keep them);
  *explorer* survives untouched (explorers belong on mountains); *sponsor* survives in schemas, wears a
  Patrol jacket in the UI.

**Assistant (RUN-BLUEBIRD execution plan — structure):** One run in the existing discipline —
**red tests first**, including (a) a **dist-wide "no `skylite` string" test** and (b) a
**banned-vocabulary lint** that mechanically enforces the no-bird-verbs / no-groomer rules — then
**lexicon renames with the trust-tier enum baked in while schemas are still free**, then pages, manifest,
tokens, assets, copy, docs, oauth, gate, summary. Two things it **refuses to guess**: the new domain and
repo name, which it **reads from `CNAME` and the git remote and halts if either still says `skylite`**. A
**hard line around the frozen seed** — the README gets a one-line bridge above the preserved Skylite
section rather than any rewriting of history. The **oauth phase is the one most likely to bite silently**:
atproto OAuth fetches client metadata from your own domain, so a stale URL there breaks login while
everything else looks fine.

---

## Working-session resolutions (2026-07-29, CroftC session — NOT part of the pasted transcript)

Captured here because these close open items the dialogue left dangling (PLAYBOOK §2c). Durable homes:
`NAMING.md`, `ROADMAP_TODO.md`, `COHESION.md` §61, and the forum plan.

- **Thread A → extension model.** The content-fetch approach is a **browser extension** that grants the
  cross-origin fetch, **not** a user-run local proxy. The "user-friendly workflow" becomes install-an-
  extension. A Playwright-driven **test spike** is greenlit (ROADMAP_TODO **E72**), to be built under
  `experiments/alpha/`. (One transferable idea from the excised life360 thread, noted only: the
  "minimal-trigger credential + do-the-real-work-with-a-better-credential" split — relevant to Croft's
  auth-helper thinking, not filed as life360 content.)
- **Thread C → Graze.** `/f/`, commons framing, herd-puns-at-edges all adopted. **Work from
  `forum.croft.ing` now; purchase `graze.ing` when possible; "Graze by Croft" is a working name.** Backbone
  unchanged (reconciles the `2026-07-27-read-first-forum-mvp` plan, whose last open item was the product
  name).
- **Thread D → Bluebird.** **Ground truth confirmed:** `github.com/CroftCommunity/bluebird` and
  `https://bluebird.croft.ing/` are **live — the rename happened.** The local `skylite/` working copy's
  git remote is therefore **stale**, and RUN-BLUEBIRD's *local* execution (repo contents, remote URL) is
  **still pending** (deferred; that plan belongs in the bluebird repo, not discovery). "Bluebird" is a
  **working name** within the ski-beginner vocabulary, with fallbacks in that family; the Amex / Bluebird.js
  **clearance risk stays an open gate**, not resolved.
