# Raw: arecipe — CNAME/paths, PWA keyboard, camera+AI, LLM import, timers, seasonality (2026-07-23)

**Preservation status: preserved-condensed (cleaned-paste, content-faithful — NOT byte-pristine) —
PLAYBOOK §4.** Source: a pasted Claude chat session, 2026-07-23. UI render chrome stripped (the italic
thinking-summary headers, "Document · MD" artifact markers, "Show more", "Read and edited memory"). No
secrets. Part of the 2026-07-23 batch (plan:
`alpha/plans/2026-07-23-transcript-intake-socialization-arecipe-measurement.md`).

**Flags — WHOLE-DOCUMENT technical claims `[UNVERIFIED]`** (Phase-2 fact-check, per the 2026-07-22
precedent): CNAME/DNS RFC details; PWA keyboard-lock in the WHATWG Fullscreen Living Standard (the July 2026
date, `keyboardLock` enum, the Firefox Fx151 / Bugzilla 2032302 shipping claim, the Chromium
`navigator.keyboard.lock`); the sendBeacon-adjacent and Prompt API `responseConstraint` claims. This is
arecipe-repo design thinking (`CroftCommunity/arecipe`); executions live there. **Three RUN instruction
files were produced and delivered as `timers.zip`** — **CAPTURED verbatim 2026-07-23** at
`seeds/timers-unpacked/` (RUN-TIMERS-SEASONALITY, RUN-GUIDE-HELPER, EXP-IMPORT-EXTRACTION; byte-verified).

**Fact-checked 2026-07-23** → `alpha/research/2026-07-23-batch-factcheck.md`. **All claims CONFIRMED.** Two
soft spots: keyboardLock's exit-input is a spec *SHOULD* not "requires" (enum is exactly `"browser"`/`"none"`);
`Escape` reaches an ordinary page (only UA-claimed inside fullscreen). CNAME/RFC 1035, apex-CNAME, Firefox 151
(Bugzilla 2032302), manifest-`shortcuts`=jump-list, and Prompt-API `responseConstraint`=JSON-Schema all
verified.

---

**User:** Can a CNAME forward to a full URL with a path?

**Assistant:** No. A CNAME record's RDATA is a single `<domain-name>` and nothing else (RFC 1035 §3.3.1) —
no field for a scheme, port, or path, because DNS resolves names to names/addresses and never sees the URL.
The path only exists at the HTTP layer, in the request line after the connection is open. What you actually
want, depending on the goal:
- **HTTP redirect.** Point the hostname at a server (A/AAAA, or CNAME to a host you control) that returns
  301/302/308 to the full target URL including the path. The only thing that genuinely "forwards" a browser.
- **Registrar / DNS-provider "URL forwarding" / "web forwarding."** Looks like a DNS feature but isn't: an A
  record pointing at the provider's redirect servers, which issue the HTTP redirect. Same mechanism, hosted.
- **Reverse proxy / path rewrite.** To add the path silently (not shown in the address bar), a proxy
  (Cloudflare Workers/Rules, nginx, Caddy) rewrites `/` to `/some/path` upstream. DNS gets you to the proxy;
  the proxy does the path work.
One gotcha: a CNAME cannot coexist with other records at the same name, so you generally cannot CNAME a zone
apex. Providers work around it with ALIAS/ANAME/CNAME-flattening (resolve the target, serve A records) — a
deployment-level behavior, not something RFC 1035 defines.

**User:** Can a PWA take keyboard shortcuts?

**Assistant:** Yes. A PWA is a web page, so ordinary in-app shortcuts are just `keydown` listeners plus
`preventDefault()`; no installation required. The interesting part is which keys the browser and OS refuse to
hand you — three layers, in dependency order:
1. **Ordinary keys.** Anything the browser hasn't reserved reaches your handler in a tab or installed window
   alike. `/`, `j`/`k`, `g then i`, `Cmd/Ctrl+K` — fine.
2. **Reserved keys.** Combos the UA or OS claims first (`Ctrl+T`, `Ctrl+W`, `Ctrl+N`, `Alt+Tab`, `Escape`)
   never reach the page. No spec enumerates the set; it is UA-defined. The Fullscreen standard notes certain
   system sequences can never be intercepted (e.g. `Ctrl+Alt+Del` on Windows).
3. **Keyboard lock.** The escape hatch, and it is **fullscreen-only.** Now folded into the WHATWG Fullscreen
   Living Standard (last updated 17 July 2026) as a `FullscreenOptions` member: `keyboardLock` is an enum of
   `"browser"` or `"none"`, defaulting to `"none"`, passed to `requestFullscreen()`. When active, key events
   that would normally trigger UA/system actions are redirected to the app (either no action, or still firing
   but cancelable via `preventDefault()`). The spec requires an additional reserved input for exiting
   fullscreen (common pattern: `Esc` requires a long press). Firefox shipped this to desktop in Fx151
   (Bugzilla 2032302), API disabled on Android. The older Chromium-only form is `navigator.keyboard.lock([…])`
   (MDN flags it experimental; requires secure context, active top-level browsing context, transient user
   activation).
Two flags: whether an installed standalone window frees up more keys than a tab is implementation behavior,
not spec, and inconsistent (a 2022 VS Code bug report on the vscode.dev PWA — corroboration only). And the
web-app-manifest `shortcuts` member is the **jump list on the app icon, not keyboard bindings** — the name
collision trips people up.

**User:** Let's dream but grounded on cool functions we could add realistically to arecipe.app. I was
thinking, can a PWA access the camera and on-board AI? Could it try to suggest a meal based on ingredients in
a picture and recipes in a cookbook? Other features that would be useful?

**Assistant:** On share target: it's not just declared — I read the deployed manifest and the mine bundle.
The manifest has `share_target` with `action: "./mine.html"`, GET, and title/text/url params; `mine.html`
reads them from `location.search`, normalizes, calls `history.replaceState` to clean the URL, then opens the
importer with `acquireFromUrl` and `acquireFromPaste` wired up; on success it saves a draft and redirects to
`editor.html?draft=`. One thing worth measuring first: with no backend, `acquireFromUrl` is a browser fetch
of a third-party site, so **CORS bounds how many recipe sites can actually be read.** If most shared URLs
fail and fall back to paste, that changes where a model belongs.

**LLM import — yes, with one rule that makes it safe: the model selects, it never writes.** Given the
agents-page posture (no legal claims, cite sources, describe-don't-license), a model that rewrites
instruction prose has produced a derivative and blurred provenance; a model that picks spans out of the
source has not. Constrain it to extraction and enforce mechanically: **every returned ingredient and
instruction string must appear verbatim in the input text; an extraction that fails the substring check is
rejected wholesale.** That turns it from a vibes feature into something with a hard acceptance criterion you
can write a failing test for. The Prompt API's `responseConstraint` gives the field shape via JSON Schema;
the substring check gives fidelity. Three follow-on constraints:
- **Deterministic wins, always.** If JSON-LD parses, the model never runs — it only fills a gap, never
  overwrites structured data.
- **The editor is already the confirmation surface.** Model output arrives at `editor.html?draft=` like any
  import, field by field, with a human looking before a record is written. No new UX.
- **Desktop-only is acceptable here** (unlike the fridge). Import is a sit-down task; the ladder degrades
  honestly (mobile share → JSON-LD or paste; desktop additionally → the assist).
The unglamorous thing to do first: **instrument the current miss rate before adding a model.** The
deterministic path is likely leaving sites on the table for boring reasons (microdata/RDFa rather than
JSON-LD; JSON-LD nested in `@graph` or an array; `recipeInstructions` as `HowToStep`/`HowToSection` objects;
ISO 8601 durations; HTML entities; h-recipe microformat). Closing those costs zero bytes and may cover more
real sites than the model would. Measure, then decide what the model is for.

**Timer page.** Making it a page (not a widget) is right — timers need to survive leaving the recipe. The
detail that decides whether it works: **store each timer's end timestamp, not its remaining seconds, and
recompute on every wake.** Interval countdowns drift and die when the tab is backgrounded or the device
sleeps; with absolute end times a timer that "ran" while asleep is simply already expired when you look —
the correct behavior. Linked from the reference, surfaced in focus, independent of both.

**Seasonality.** Static data, no model, no API; ages gracefully (the produce calendar barely changes). Two
notes: make the map hemisphere-aware / region-scoped from the start rather than retrofitting, and keep the
planner nudge soft ("Asparagus is out of season in November" is charming once and nagging the third time).

**User:** Seasonality should always be a boost, never a drag; it should be turn-off-able in settings. Timer
thinking is good. Can you create a comprehensive Claude Code instruction to create these 2? And another for
experiments for the enhanced recipe extraction with LLM? And another for the user-guide helper with
deterministic-to-LLM enhancement, and a helper that returns deep links to exactly where the user needs to go
is best.

**Assistant:** Three files (delivered as `timers.zip` — not reproduced here). What I encoded deliberately:
- **Seasonality's boost-only rule is enforced by a test, not a promise.** One case asserts applying boosts
  never removes a result and never lowers any entry's position relative to the unboosted list; another
  asserts that with the setting off the ranking output is deep-equal to the unboosted baseline. There's a
  grep across rendered pages for out-of-season phrasing so the drag can't creep back in through copy. Region
  is explicit with no geolocation call (inferring it is the fastest route to wrong badges).
- **The timer file makes the stored-countdown mistake unshippable.** Acceptance criterion 1 requires a grep
  proving no persisted remaining-duration field exists; one test simulates an eight-hour device sleep and
  asserts zero accumulated drift. It refuses to fake background alerts (no service-worker scheduler) and
  states the limit plainly in one sentence of settings copy.
- **The import experiment leads with the CORS ceiling, not the model.** With no backend, whatever fraction of
  the 40-source corpus can't be fetched client-side is a permanent bound on the URL rung, independent of
  parsing quality — recorded before any parser is touched. The verbatim-substring invariant is written as a
  provenance requirement, with an explicit clause that no result may be used to argue for relaxing it.
- **The guide helper is built to not need a model.** Layer A ships alone; Layer B is hand-written question
  phrasings; Phase 3 has a stop gate — if curated phrasings get top-1 accuracy where you want it, Layer C
  (the model) never gets built, and the file says that outcome is the better one. The anchor-validation rule
  mirrors the import invariant: a model may rank and summarize retrieved text; it may not invent a
  destination. No query is logged or transmitted — the one place not to bend, since help queries are a record
  of what someone does not understand.

(Artifacts delivered: **Run timers seasonality**, **Run guide helper**, **Exp import extraction** — as MD
documents, plus `timers.zip`. Not pasted into this session; capture at Phase 2 if the archive is provided.)
