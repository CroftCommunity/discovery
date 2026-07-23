# Fact-check — the 2026-07-23 transcript batch (Phase 2)

date: 2026-07-23
purpose: verify the load-bearing technical/quantitative claims in the 2026-07-23 intake (the arecipe +
measurement + Bluesky-origin transcripts) against primary sources, so the corpus does not carry
dialogue-sourced errors forward. Method: four parallel research passes against official specs, MDN, RFCs,
IETF drafts, MediaWiki/Wikimedia/Wikibooks, CNIL, Cloudflare, and reputable press. **Not committed until
reviewed.**

**Headline: unusually accurate. Of ~30 claims, most CONFIRMED; four need correction, and one is REFUTED.**
The corrections that matter are collected first; the full per-domain tables follow. This supersedes the
blanket `[UNVERIFIED]` flags on the raw transcripts (raw headers updated to point here).

## Corrections that matter (owner-relevant)

1. **REFUTED — the sendBeacon reliability figures are inverted / mis-sourced (transcript G, RUN-MEASURE-01
   §E5).** The write-up says ~95.8% on `visibilitychange`/`pagehide` vs ~82.9% across all four unload events.
   Reality (NicJ.net "Beaconing in Practice"): **~82–83% when beaconing at unload time** (pagehide +
   visibilitychange + beforeunload) — the 82.9% is real and is the *hidden/unload* number, not the
   all-four-events number. The **~92–93%** rate comes from beaconing **at page load** ("avoid-abandon"), not
   from the hidden/pagehide events. The **95.8% appears in no source.** *Fix in the experiment:* E5 should
   expect ~82% landing at hidden/pagehide, treat ~92% as the load-time ceiling, and measure its own rates
   rather than reuse either.
2. **PARTLY — the 64 KiB sendBeacon cap is a browser/Fetch-`keepalive` behavior, not a W3C Beacon-spec
   mandate (transcript G).** The Beacon spec intentionally leaves max size unspecified; 64 KiB comes from the
   Fetch `keepalive` quota. **Chrome enforces it as a shared quota across all in-flight beacons** in the
   navigation context; Firefox/Edge enforce it per-beacon. *Fix:* E5's "64 KiB ceiling" test must account for
   the shared-quota behavior on Chrome (many small beacons can collectively hit it).
3. **PARTLY — the Wikibooks `{{Recipe summary}}` template has no `category` parameter (transcript E).**
   Categorization is **automatic** (driven by `difficulty`, suppressible with `noincludecat`). Actual params:
   `name, cuisine, origin, yield, servings, time, difficulty, image, energy, note, noincludecat`. *Fix:* the
   importer maps difficulty→category, it does not read a `category` field.
4. **PARTLY — the pipe-trick ingredient-link is not mandated by policy (transcript E).** Cookbook policy
   requires ingredients **in procedure order** (CONFIRMED) and that they "should be linked to the most
   appropriate ingredient page" — but it does **not** prescribe the `[[Cookbook:carrot|]]` pipe-trick syntax.
   *Fix:* the "ingredient extraction is a link parse" premise still holds (ingredients are wikilinked), but
   the parser must not assume pipe-trick specifically.

Two soft spots worth a word, not a rewrite:
- **keyboardLock exit-input is a spec SHOULD, not a MUST (transcript D).** The Fullscreen standard says UAs
  *should* reserve an additional input to exit; "the spec requires" overstates it. The enum is exactly
  `"browser"` / `"none"` (no third value). And `Escape` reaches an ordinary page — it is only UA-claimed
  inside fullscreen/keyboard-lock contexts.
- **Bluesky "spun out as a PBC" is loosely right (transcript B).** Precise: **incorporated Oct 4, 2021**;
  **became a Public Benefit Corporation Feb 2022**; independent from formation with ~$13M initial Twitter
  funding; the service/funding tie to Twitter was severed in **late 2022**. The "X" rebrand was **July 2023**.

## Transcript B — Bluesky / AT-Protocol origin (closes ROADMAP_TODO E52)

| # | Claim | Verdict | Correct value / note |
|---|---|---|---|
| 1 | Dorsey announced Bluesky 2019, Twitter-funded, open decentralized standard | CONFIRMED | Announced Dec 11, 2019; Twitter funded "a small independent team of up to five." |
| 2 | Jay Graber recruited to lead in 2021 | CONFIRMED | Hired Aug 2021 (Bloomberg, Aug 16 2021). |
| 3 | Spun out as an independent PBC | CONFIRMED (nuance) | Incorporated Oct 4 2021; PBC status Feb 2022; ~$13M initial Twitter funding; service tie severed late 2022. |
| 4 | Musk acquired Twitter 2022 → rebrand to X | CONFIRMED | Acquisition completed Oct 28 2022 ($44B); "X" rebrand July 2023. |
| 5 | "~40M-user-accessible scale" | PARTLY | **Registered** users: 41.41M at end-2025 (Bluesky transparency report, pub. Jan 29 2026); >45M by mid-2026. "~40M" was accurate late 2025, now understated. These are **registered accounts, not active** (DAU ~4.5M Jan 2026). The "40M-accessible-scale" phrasing is the project author's framing, not a Bluesky metric. |

Sources: cnbc.com (2019 announcement); Bloomberg (Graber hire); en.wikipedia.org/wiki/Bluesky;
en.wikipedia.org/wiki/Acquisition_of_Twitter_by_Elon_Musk; Bluesky 2025 transparency report.

## Transcript D — browser / PWA / DNS (all CONFIRMED)

| # | Claim | Verdict | Note |
|---|---|---|---|
| 1 | CNAME RDATA = single domain-name; no scheme/port/path (RFC 1035 §3.3.1) | CONFIRMED | Path is HTTP-layer. |
| 2 | CNAME can't coexist w/ other records; no apex CNAME; ALIAS/ANAME/flattening | CONFIRMED | RFC 1034 §3.6.2. |
| 3 | Ordinary shortcuts via keydown+preventDefault; reserved combos UA/OS-claimed; set not enumerated | CONFIRMED | Escape reaches an ordinary page (only UA-claimed in fullscreen). |
| 4 | keyboardLock in WHATWG Fullscreen standard (`FullscreenKeyboardLock` "browser"/"none", default "none") | CONFIRMED | Exit-input is a SHOULD, not "requires"; only two enum values. `navigator.keyboard.lock()` = older Chromium-only, experimental. |
| 5 | Firefox 151 shipped desktop keyboard lock (Bugzilla 2032302), off on Android | CONFIRMED | Pref `dom.fullscreen.keyboard_lock.enabled`. |
| 6 | Manifest `shortcuts` = app-icon jump list, not key bindings | CONFIRMED | W3C App Manifest. |
| 7 | Prompt API `responseConstraint` = JSON Schema output constraint | CONFIRMED | Chrome docs; since Chrome 137. |

Sources: rfc-editor.org/rfc/rfc1035; isc.org (CNAME-at-apex); fullscreen.spec.whatwg.org;
bugzilla.mozilla.org/show_bug.cgi?id=2032302; w3.org/TR/appmanifest; developer.chrome.com (Prompt API).

## Transcript E — Wikibooks / MediaWiki / atproto sync

| # | Claim | Verdict | Note |
|---|---|---|---|
| 1 | `{{Recipe summary}}` params incl. `category`; difficulty 1–5; all optional | PARTLY | **No `category` param** — categorization is automatic via difficulty (`noincludecat` suppresses). Difficulty 1–5 confirmed. |
| 2 | `{{Recipe}}` → Category:Recipes; `categorymembers` authoritative; ToC drifts | CONFIRMED | Query Category:Recipes (a category, not a namespace filter). |
| 3 | Ingredients in procedure order, linked via pipe trick | PARTLY | Ordering CONFIRMED; **pipe-trick NOT mandated** — policy only says "link to the most appropriate ingredient page." |
| 4 | ~3,600 pages (704/1,557/1,273/93); ~802 with images | CONFIRMED (refreshed) | Current: **Category:Recipes 3,824**; Very Easy 723, Easy 1,627, Medium 1,327, Difficult 102, Very Difficult 2; **832 with images.** Same order of magnitude. |
| 5 | CC BY-SA 4.0 since June 2023; older revisions 3.0 | CONFIRMED | Wikimedia ToU effective 7 June 2023; 3.0 upward-compatible with 4.0. |
| 6 | XML dumps deprecated → MediaWiki Content File Exports; still monthly | CONFIRMED | Content File Exports generate monthly from the 1st. |
| 7 | API: ≤50 pageids/query (500 apihighlimits); ≤500/list (5000 apihighlimits) | CONFIRMED | API:Query. |
| 8 | `$wgRCMaxAge` 30 days on Wikimedia → RC can't go >~30 days | CONFIRMED | Software **default is 90 days**; Wikimedia sets **30**. The 6-month-rerun conclusion stands. |
| 9 | atproto `getLatestCommit` = cid+rev; `getRepo` `since` → CAR diff of MST blocks | CONFIRMED | Lexicons confirm; output `application/vnd.ipld.car`. |

Sources: en.wikibooks.org (Template:Recipe_summary, Template:Recipe, Category:Recipes,
Cookbook:Policy/Recipe_template); foundation.wikimedia.org (ToU); creativecommons.org;
wikitech.wikimedia.org (Content File Exports); mediawiki.org (API:Query, $wgRCMaxAge);
github.com/bluesky-social/atproto lexicons.

## Transcript G — usage-measurement stack

| # | Claim | Verdict | Note |
|---|---|---|---|
| 1 | sendBeacon caps at 64 KiB per W3C Beacon spec | PARTLY | 64 KiB is real but a **Fetch-`keepalive` / browser** behavior, not a Beacon-spec mandate. Chrome = shared quota across beacons; Firefox/Edge = per-beacon. |
| 2 | ~95.8% (visibility/pagehide) vs ~82.9% (four unload events) | **REFUTED** | **~82–83%** at unload/pagehide/visibility; **~92–93%** at page LOAD. 95.8% is unsourced; the pairing is inverted. |
| 3 | unload/beforeunload break bfcache; visibilitychange/pagehide recommended | CONFIRMED | web.dev/bfcache. |
| 4 | CNIL July 2025 tool; ≤3 event types; nearest-ten rounding | CONFIRMED | Published July 4 2025; max 3 event types (page presence / functionality interactions / loading-time-scroll-time stats); recommends aggregating to the nearest ten (alternatives w/ justification). |
| 5 | Poplar1 (VDAF) = secret-shared prefix counting; needs ≥1 honest server | CONFIRMED | draft-irtf-cfrg-vdaf §8; DPF; two non-colluding operators. |
| 6 | R2 free tier 1M Class-A/mo; Litestream @1s ≈ ~2.6M PUT/mo | CONFIRMED | R2 free tier = 1,000,000 Class-A ops/mo. 2.6M ≈ 60×60×24×30 (only if written every second; idle DBs upload far fewer). |

Sources: blog.huli.tw + github.com/w3c/beacon/issues/38 (64 KiB); nicj.net/beaconing-in-practice;
web.dev/articles/bfcache; cnil.fr sheet n°16 (+ ppc.land); datatracker.ietf.org draft-irtf-cfrg-vdaf;
developers.cloudflare.com/r2/pricing.

## What this changes downstream

- **E52 (verify Bluesky origin) → DONE.** The beta autonomy-origin asset can cite the corrected dates
  (Dec 2019 / Aug 2021 / PBC Feb 2022 / Oct 2022 / X July 2023) and must say **registered accounts** (41.41M
  end-2025, >45M mid-2026) — not "users"/active. Keep the "~40M-accessible-scale" as the author's framing.
- **RUN-MEASURE-01 (E60):** fix E5's sendBeacon expectations (correction 1) and the 64 KiB test
  (correction 2) before running.
- **The Wikibooks importer (E56):** map difficulty→category (no `category` field), and don't assume the
  pipe-trick (corrections 3–4). Delta path via full enumeration + revid sweep stands (claim 8 confirmed).
- Everything else in D/E/G is confirmed as filed.
