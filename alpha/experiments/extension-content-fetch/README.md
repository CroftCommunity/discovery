# Spike: extension-granted cross-origin content-fetch for a static Croft PWA

**Question (ROADMAP_TODO E72).** How does a zero-backend static Croft PWA/SPA reach arbitrary reader/API
content the browser's same-origin policy blocks? Decision (user, 2026-07-29): the **browser-extension
model** (an extension grants the cross-origin read and relays it to the page), not a user-run local proxy.
This spike proves the mechanism works, hermetically.

## Result: GREEN (3/3), 2026-07-29

```
PASS  extension service worker registered
PASS  direct PWA fetch is CORS-blocked (baseline)      TypeError: Failed to fetch
PASS  extension bridge delivers cross-origin content   status 200, marker found

SPIKE GREEN — 3/3 checks
```

## What it does

```
Playwright (channel:'chromium', new-headless, --load-extension=ext/)
┌───────────────────────────────┐         ┌──────────────────────────────┐
│ origin A  http://localhost:5601│  ✗ CORS │ origin B  http://localhost:5602│
│ pwa/  (static page)            │────────►│ reader/feed.xml                │
│  window.__directFetch(B)  ─────┼─ blocked│  served with NO                │
│     → TypeError (baseline)     │         │  Access-Control-Allow-Origin   │
│                                │         │                                │
│  window.__viaExtension(B)      │  ✓      │                                │
│   page → content.js → bg SW ───┼────────►│  bg SW holds host_permission   │
│   ◄─── marker relayed back     │         │  for :5602 → CORS-exempt read  │
└───────────────────────────────┘         └──────────────────────────────┘
```

- `ext/` — MV3 extension: `background.js` (holds `host_permissions` for origin B, does the fetch),
  `content.js` (page↔background bridge, injected only into origin A), `manifest.json`.
- `pwa/` — the static page (origin A) with two probes: `__directFetch` and `__viaExtension`.
- `reader/feed.xml` — origin B content, served **without** a CORS header (the deliberate block).
- `run-spike.mjs` — starts both origins, launches Chromium with the extension, runs the two assertions.

## Run

```
node run-spike.mjs      # exit 0 = GREEN. No network egress; both origins are localhost.
```

Playwright is borrowed from `../../../croft-pwa/node_modules` (per `CroftC/.claude/CLAUDE.md`); the full
`chromium-*` build is used via `channel: 'chromium'` (MV3 extensions need it, and it runs new-headless).

## Findings

1. **The mechanism works, headless.** A static page that cannot read a no-CORS cross-origin resource
   receives it verbatim once an extension with the matching `host_permissions` relays it. This is the
   E72 "user-friendly workflow" reframed as install-an-extension rather than run-a-command.
2. **The extension model sidesteps the mixed-content problem the local-proxy model has.** In the
   proxy model an HTTPS page (github.io) must itself `fetch('http://localhost:PORT')` — the exact
   HTTPS→HTTP request browsers scrutinize (`[UNVERIFIED]` localhost exemption / Private Network Access).
   In the extension model the **page never makes the cross-origin/insecure request** — the extension
   service worker does, and content is delivered in-process via `postMessage`. So the page stays clean
   HTTPS and the PNA/mixed-content edge does not gate the page at all. Net: one of the reasons to prefer
   extension over proxy is not just UX, it is a smaller browser-policy surface.
3. **PNA did not trigger here** because both origins are localhost (no public→private transition). The
   real deployment (HTTPS PWA + a *remote* reader host in `host_permissions`) is a different path and is
   the honest next check.

## v2 — edges walked hermetically: GREEN (6/6), 2026-07-29

`node run-edges.mjs` (throwaway self-signed cert generated into the OS temp dir, never committed):

```
PASS  page is a genuine secure context (HTTPS)                              isSecureContext=true
PASS  HTTPS page cannot fetch HTTP reader directly (mixed-content/CORS)     TypeError: Failed to fetch
PASS  extension delivers HTTP content to the HTTPS page (mixed-content sidestep)   status 200
PASS  extension REFUSES a non-allowlisted origin (consent gate)            origin not allowlisted: :5603
PASS  PWA detects extension ABSENCE (install-flow signal)                  __extReady=false
PASS  without the extension the bridge fails gracefully (no hang)          extension timeout

EDGES GREEN — 6/6 checks
```

- **Mixed-content sidestep — proven.** A genuine HTTPS secure-context page cannot fetch the HTTP reader
  itself (blocked), but receives it via the extension. Finding #2 is now demonstrated, not just argued:
  the page never makes the insecure request, so being a secure context does not gate the read.
- **Consent gate — proven.** `ext/background.js` enforces an origin allowlist; a non-allowlisted origin is
  refused by the extension (`refused:true`), not by CORS. The bridge is not a blanket proxy (static
  allowlist here stands in for a real per-host user-consent list).
- **Install-flow core — proven.** The PWA detects extension presence via a content-script ready-ping, and
  correctly reports **absence** (a second context launched without the extension) — the signal a real PWA
  uses to show "not connected, install the extension" and to degrade gracefully instead of hanging.

## @live — real remote reader: GREEN, 2026-07-29

`CROFT_LIVE_READER_URL="https://news.ycombinator.com/rss" node run-live.mjs` (operator supplies the URL —
no guessed endpoints; a live extension variant scoped to that origin is generated in the OS temp dir, so
the committed `ext/` stays hermetic-only):

```
direct PWA fetch: blocked — TypeError: Failed to fetch     (CORS — the feed sends no ACAO)
extension read:   ok (status 200, 11442 bytes)             real RSS bytes returned
LIVE GREEN — extension read a real remote reader
```

- The mechanism holds against a **real** remote server, not just localhost. This is a **public→public**
  read, so PNA does **not** apply (PNA only bites the rejected public→localhost proxy model).
- Note on egress: the general sandbox "browser egress is blocked" caveat did **not** hold for this path —
  a plain extension-SW `GET` to a public host succeeded. The block appears narrower (host/OAuth-specific)
  than a blanket wall, at least for simple reads. One host, one run — re-confirm against the intended real
  feeds before relying on it.

## Still open

- **Firefox MV3 parity — parked** (user 2026-07-29, "leave FF out for now to get started"). Playwright
  cannot `--load-extension` in Firefox anyway → would need `web-ext`/manual. Chromium-first.
- **Content-script injection vs `externally_connectable`** (stable extension id) as the page↔ext channel —
  a design choice for the real extension, not a correctness question the spike needs to settle.
- **A real per-host user-consent UI** — the allowlist here is a static stand-in; the product extension
  needs the actual approve-a-reader-host flow.
