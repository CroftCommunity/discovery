# Plan: transcript intake 2026-07-23 — socialization + arecipe + a new measurement helper

date: 2026-07-23
identity: chasemp (`chase@owasp.org`, `github-personal`); repo `discovery`
status: **Phase 1 (preserve raw + connective tissue + the two approved distillations) — executed this pass.**
Phase 2 (deep fact-check of the volatile technical claims + remaining distillation) deferred.

## Problem statement

A batch of seven transcripts arrived in one session. The user framed them as "socialization mostly,"
but on inspection they span three areas: two socialization/narrative dialogues, one client-architecture
verification, three arecipe feature dialogues, and one net-new cross-property helper concept. They must be
folded into the corpus the same way every time (PLAYBOOK) so provenance stays intact and nothing is lost —
without disturbing the still-uncommitted 2026-07-22 intake in the working tree.

## Approach

Classify each, preserve each raw under `alpha/seeds/transcripts/raw/…-2026-07-23.md` (cleaned-paste,
content-faithful, PLAYBOOK §4 — the sources are pasted chat sessions with no canonical export), then update
the standing indexes each earns. The seven, by destination batch:

**Socialization batch (distilled this pass, per user decision):**
- **A** `hansel-gretel-enshittification-gilded-weaver-fable` (Gemini) — Hansel & Gretel as a teaching-tool
  analogy for enshittification (Doctorow), the cross-cultural deceptive-trap survey, the villain-name
  iteration landing on **the Gilded Weaver / "The Weaver of the Golden Hall,"** the advertising-as-benign-
  but-crushing-in-aggregate refinement, several fable retellings, and a **white-duck logo** design branch
  (deliberately distinct from DuckDuckGo).
- **B** `bluesky-atproto-autonomy-origin-story` (Gemini) — the Twitter→Bluesky→Jay-Graber-autonomy→PBC-
  spinout→Musk/X→AT-Protocol origin narrative; the "why Croft builds serious apps at ~40M-user scale on
  atproto without reinventing the wheel" premise.

**Account-kernel batch (folds into the uncommitted 2026-07-22 kernel thread):**
- **C** `croft-account-sso-storage-vs-cookie-browser-support` (Claude) — the `account.croft.ing` SSO
  storage-vs-cookie split, the three auth options and their browser support, the single-origin-core +
  isolated-subdomains architecture. **Independently web-verified this pass** (see "Verified findings").

**arecipe batch (raw + connective tissue this pass; fact-check/distill Phase 2):**
- **D** `arecipe-cname-pwa-keyboard-camera-ai-import-timer-seasonality` (Claude) — CNAME-can't-carry-a-path;
  PWA keyboard shortcuts (reserved keys, `keyboardLock` is fullscreen-only, manifest `shortcuts` = jump list
  not bindings); camera + on-device AI meal-suggestion dreaming; the LLM-import rule (**model selects spans,
  never writes**, enforced by a verbatim-substring invariant; deterministic JSON-LD wins first); the timer
  page (**store absolute end timestamps, never remaining seconds**); seasonality (**boost-only, never a drag,
  toggleable**). Produced three RUN instruction files delivered as `timers.zip` (**not pasted**).
- **E** `arecipe-wikibooks-corpus-import-and-static-precache` (Claude) — importing
  `en.wikibooks.org/wiki/Cookbook` recipes (the `{{Recipe summary}}` sparse map, `categorymembers`
  enumeration is authoritative over the ToC, pipe-trick ingredient links make extraction a link-parse, CC
  BY-SA licensing as the design constraint, MediaWiki Action API over deprecated dumps); the static-list
  precache via atproto sync (`getLatestCommit`/`getRepo since rev`, CAR/MST decode, treat the bundle as a
  cold-start cache never authority). Produced three RUN files delivered as `wiki.zip` (**not pasted**).
- **F** `arecipe-empty-tile-chip-mobile` (Claude) — the pictureless-tile mobile vertical-real-estate fix
  (inline chip at single-column, media zone at multi-column; the 3:2-vs-16:7 ratio bug). Includes the full
  **RUN-EMPTY-TILE-CHIP** instruction file, pasted verbatim → preserved.

**New helper concept (arecipe-piloted, cross-property):**
- **G** `usage-measurement-edge-counter-privacy-kit` (Claude) — a privacy-preserving usage-measurement kit:
  **an edge counter over a declared graph** (counters, not logs; `nav.a__to__b`), a registry that generates
  the client calls + disclosure panel + fixtures, the panel doing double duty (rich local view vs the exact
  counts queued to leave), `expires` honored **at runtime** in the generated client, and the Poplar/VDAF
  cryptographic-heavy-hitters path noted-and-skipped (needs two non-colluding operators). Includes the full
  **RUN-MEASURE-01** experiment file (E0–E8), pasted verbatim → preserved.

## Reasoning

- **Preserve-raw-first** is non-negotiable provenance (PLAYBOOK §4). These are pasted chats with no canonical
  export, so cleaned-paste (content-faithful, chrome-stripped) is the accepted best-available raw.
- **The socialization two were approved for distillation now** (user, 2026-07-23): the Gilded Weaver fable
  and the autonomy-origin story become human-facing beta socialization assets; the Gilded Weaver and the
  white-duck logo are registered as **working candidates at beta tier** (OPEN-THREADS T61), and Jay Graber is
  added to `kindred-work.md`.
- **The arecipe/measurement five carry many volatile technical claims** (browser API behaviors, MediaWiki
  limits, licensing dates, sendBeacon/CNIL/Litestream figures). Following the 2026-07-22 precedent, these are
  flagged `[UNVERIFIED]` and verification is deferred to Phase 2 — **except C**, which the user explicitly
  asked to be researched, so it was web-verified this pass and its raw records the corrected mechanism.
- **The measurement kit is a genuinely new helper**, not an arecipe feature — it is cross-property (Skylite's
  sponsor-visible-vs-child-private stance and the existing Caddy/SQLite/Litestream/R2 kit shape both bear on
  it), with arecipe as the recommended low-risk pilot. It lands as alpha thinking + ROADMAP_TODO, not a beta
  doc.

## Verified findings (transcript C — researched 2026-07-23, primary sources)

The transcript's top-line conclusion is **expected and correct**; two mechanism explanations in it needed
correction, and one nuance was missing. The raw for C records the corrected version so the settled record
does not bake in the wrong "why":

1. **Serverless shared-storage SSO is Chromium-only.** WebKit partitions web storage by the *top-level
   page's domain* and treats sibling subdomains as separate partition keys, even though they share the
   registrable domain; Chrome/Firefox partition by the registrable domain, so the shared store survives
   across `*.croft.ing`. (Apple developer forums; cookiestatus.com; webkit.org/tracking-prevention.) Matches
   the K1 measurement.
2. **The failure is same-party (top-level-subdomain) partitioning, not classic third-party isolation.**
   `account.croft.ing` embedded under `app.croft.ing` is same-site; the write-up's "third-party partitioning"
   framing would mislead a later reader into thinking SameSite / Storage Access API / first-party status
   fixes it (it does not, absent a user gesture + prompt).
3. **A cookie can't carry the DPoP session because the key is non-extractable, not because it is
   partitioned.** The atproto browser client generates a **non-extractable WebCrypto `CryptoKey`** persisted
   in **IndexedDB** (the only web storage that holds live `CryptoKeyPair` objects); JS can never read its
   bytes, so it cannot be serialized into a cookie even in principle. (atproto OAuth spec; Bluesky OAuth
   client docs; InfoQ "DPoP Storage Paradox.")
4. **Added nuance:** a `Domain=croft.ing` cookie does cross subdomains on Safari as first-party, but a
   cookie set via `document.cookie` (JS) is capped at 7 days by ITP; only a server-`Set-Cookie` (HTTP,
   `HttpOnly`) first-party cookie persists (~400 days). This *reinforces* the BFF conclusion — the durable
   SSO cookie has to be server-set, which is what a BFF does. And whether the DPoP key sits client- or
   server-side is an *unsettled* atproto design question (the token-mediating-backend proposal).

Net: serverless storage-SSO → Chromium-only (dead on iOS); cross-browser sign-in-once → **server-set cookie
+ BFF** (KC1, sound); per-app login → works everywhere, serverless.

## Phase 2 (deferred)

- Fact-check the `[UNVERIFIED]` claims in D/E/G against primary sources (browser APIs, MediaWiki limits and
  page counts, CC-BY-SA revision dates, sendBeacon landing rates, the 64 KiB cap, CNIL's 2025 tool, the
  Litestream PUT arithmetic, Poplar/VDAF).
- Capture the six un-pasted RUN files (`timers.zip`, `wiki.zip`) verbatim under `seeds/*-unpacked/` if the
  user provides the archives (matches the `arecipe-unpacked/` precedent).
- Distill D/E/F/G into `thinking/` / `research/` (and, for the measurement kit, decide its name and home)
  once the facts are verified.
- Verify the transcript-B Bluesky origin dates and the "~40M-user scale" claim against primary sources.

## Commit

Not committed. This repo set is reviewed before commit (PLAYBOOK §3b); commit only on request. The
uncommitted 2026-07-22 intake is left untouched.
