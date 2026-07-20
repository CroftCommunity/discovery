# Seed: PDSVIEW-RUN-01-INSTRUCTIONS (the executed brief)

`Provenance seed, pasted 2026-07-20 (no zip). This is the Claude Code build brief for **pdsview**
— a standalone SPA/PWA atproto PDS content browser at pdsview.croft.ing. **EXECUTED + MERGED**
(verified against main): CroftCommunity/pdsview main carries the full site (src/, test/, assets/,
docs/, CI+deploy), landed as "run-01: pdsview — scaffold through PWA (phases 0–5)" + a run-01
summary, via PR #1 (branch claude/pdsview-run-01-scaffold), built 2026-07-16 by a Fable-5 session.
Frozen here for provenance; the living record (run summary, code) is the pdsview repo. Registered in
BUILD-INVENTORY.md as a live sibling tool. Content below is the brief verbatim as pasted.`

---

# PDSVIEW — RUN-01 INSTRUCTIONS

Self-contained and executable. No other chat context is required. If anything here conflicts with repository reality, stop and report rather than improvising.

## What is being built

**pdsview** — a standalone SPA/PWA browser for public content on ATProto PDSes (Personal Data Servers), in the spirit of atproto-browser.dev's per-account browse page, but purely client-side, with no live feed, and with export.

- Repo: `CroftCommunity/pdsview` (assume it exists and is empty except LICENSE/README; if not, report).
- Domain: `pdsview.croft.ing`, GitHub Pages via Actions, CNAME file in the built artifact.
- License: AGPL-3.0 to match the org (crofting_site precedent). [confirm] — if a different LICENSE is already present, keep it and note in the run summary.
- v1 user story: paste a handle or DID → land on that account's repo page → browse collections → open records (with inline images) → export.

## Standing conventions (non-negotiable)

1. **TDD, red first, always.** Every acceptance bullet below becomes a failing test before implementation. Fixtures before features. The run summary must evidence red-to-green order (test commit or failing output before the implementing change). Visual-only work (pure CSS) is exempt from unit tests but not from the contrast tests defined in Design.
2. **Zero runtime dependencies.** `dependencies` in package.json stays empty forever. Dev toolchain: strict TypeScript, esbuild for bundling, Vitest for unit tests, Playwright for e2e. Nothing from node_modules may appear in the shipped bundle.
3. **Hermetic gate.** All tests run offline. Unit tests use recorded JSON fixtures; Playwright serves the built `dist/` locally and mocks XRPC routes. Live-network probes live in a separate, manually-invoked script (see Verify ledger), never in CI.
4. **Named invariant tests** (write these first, keep them forever):
   - `zero-runtime-deps` — bundle contains no third-party module code; package.json `dependencies` is empty.
   - `no-unexpected-origins` — the app's fetch layer refuses any origin not in: the resolved PDS host, `plc.directory`, `api.bsky.app`, and the handle's own domain (well-known probe). Test by asserting the allowlist function, and in Playwright by failing on any other request.
   - `deep-links-are-did-canonical` — every internally generated route contains a DID, never a handle; handle input resolves then navigates to the DID URL.
   - `verified-requires-bidirectional` — the UI may only mark a handle "verified" when the DID document claims that handle back (alsoKnownAs). One-way resolution renders as unverified.
5. **Layer honesty in code and copy.** The spec guarantees `com.atproto.sync.*` is unauthenticated; permissive CORS on `com.atproto.repo.*` reads is deployment behavior of the reference PDS, not a spec guarantee. Error UI must say "this PDS does not allow browser reads (CORS)" as a distinct state, never a generic failure.
6. **Commits small and phase-scoped**, branch `run-01`, PR per phase or one PR with phase-tagged commits.

## Design directives — croft.ing feel

Source of truth is the Croft tectonic board. All color lives in one tokens file (CSS custom properties); no raw hex anywhere else (lint-grep test).

Tokens (name — hex — role from the board):

- `--deep-schist` — `#2F3539` — dark charcoal; JSON/code panel background, footer.
- `--light-granite` — `#A2A8A5` — muted grey; borders, surfaces, subtle labels. NOT body text unless the contrast test passes.
- `--ruddy-orange` — `#B75C34` — terracotta/rust; accents, links, primary buttons, headings.
- `--dark-moss` — `#3D8548` — earthy green; success and "verified" states, growth highlights.
- `--iron-ore-black` — `#1C1E20` — primary text.
- `--oatmeal-canvas` — `#E9E1D8` — warm cream; page background.

Contrast tests come FIRST (red), computing WCAG ratios from the tokens file: every body-text pair ≥ 4.5:1, large-text/heading pairs ≥ 3:1. Where a board-natural pair fails (expect ruddy-on-oatmeal to be large-text-only, granite-on-oatmeal to fail for body), the test encodes the restriction and the stylesheet must respect it — derive a darker ink variant rather than shipping a failing pair.

Typography: Lora (serif) for headings and the wordmark, Inter for body and UI — both self-hosted woff2 with OFL license texts committed, zero external font requests (crofting_site precedent; copy the approach, not the files).

Feel: quiet, papery, tool-like. Signature element: the record path rendered as a drystone "course" breadcrumb — `at:// › did › collection › rkey` as stacked-stone segments (echoes crofting_site's `.course` divider). JSON views sit on Deep Schist panels like the board's dark screen mock. One signature; everything else restrained. Responsive to phone width, visible keyboard focus, `prefers-reduced-motion` respected.

Wordmark: lowercase `pdsview`, Lora. [confirm]

## Phase 0 — scaffold and deploy

Acceptance (tests first where testable):

- Toolchain in place: strict tsconfig, esbuild build to `dist/`, Vitest, Playwright, CI workflow running typecheck + unit + e2e on PR, deploy workflow publishing `dist/` to Pages on main.
- `dist/` contains `index.html`, bundled JS/CSS, `CNAME` with exactly `pdsview.croft.ing`, and the invariant tests above pass.
- Placeholder shell: wordmark, one-line description, input box (non-functional yet).
- README: what pdsview is, how to run tests, the zero-dependency stance.

Manual steps for the owner (list at top of run summary): DNS CNAME `pdsview` → `croftcommunity.github.io`; Pages source set to GitHub Actions; Enforce HTTPS once the cert provisions.

## Phase 1 — identity core (pure modules)

All pure functions in `src/identity/`, unit-tested from string fixtures before any UI.

- **Classifier**: input string → typed result: `did:plc`, `did:web`, `handle`, or `at://` URI (which itself may carry a DID or handle authority plus optional collection/rkey). Reject garbage with a typed error. Strip a leading `@` from handles.
- **at-uri parse/format**: round-trip tests; formatter never emits a handle authority (canonical form is DID).
- **DID document fetch**: `did:plc` → `GET https://plc.directory/{did}`; `did:web:{domain}` → `GET https://{domain}/.well-known/did.json`. Extract: PDS endpoint = the `service` entry with id ending `#atproto_pds` / type `AtprotoPersonalDataServer` → `serviceEndpoint`; claimed handle = first `alsoKnownAs` entry of form `at://{handle}`. did:web CORS failure is an expected, distinctly-typed error (spec does not require CORS on that path).
- **Handle ladder**: (1) `GET https://{handle}/.well-known/atproto-did`, expect a bare DID in text, strip surrounding whitespace — succeeds only if the site sends CORS; (2) fallback `GET https://api.bsky.app/xrpc/com.atproto.identity.resolveHandle?handle={handle}`. Record which rung succeeded (shown in UI as resolution provenance).
- **Bidirectional verification**: after handle→DID, fetch the DID doc and check it claims the handle back. Only then "verified" (Dark Moss); otherwise "unverified" with a plain-language note.
- **Hash router**: `/#/at/{did}[/{collection}[/{rkey}]]`. Parse and generate; unknown routes → home. Handle input never appears in a URL: resolve, then navigate to the DID route, displaying the handle on-page.

## Phase 2 — browse pages

Views are functions of fetched data; render logic unit-tested against fixtures captured from real responses (record the fixtures in `test/fixtures/`, note their source account in a comment).

- **Home**: input box (handle / DID / at:// URI), short what-is-this blurb in Croft voice (draft it, mark the copy [confirm before publish] in the run summary), footer links to croft.ing and the repo.
- **Repo view** (`/#/at/{did}`): identity card — handle + verification state, DID with copy button, PDS host, link to the DID document; then the collection list from `com.atproto.repo.describeRepo?repo={did}` (GET on the account's PDS). Export button lives here (Phase 3).
- **Collection view**: `com.atproto.repo.listRecords?repo={did}&collection={nsid}&limit=50[&cursor=…]`; render rkey + a one-line preview (createdAt and `$type`-aware snippet where trivially available); "Load more" drives the cursor; stop when no cursor returns.
- **Record view**: `getRecord?repo&collection&rkey` → collapsible JSON tree. Every `at://` URI and bare DID in values renders as an internal link; the record's CID is displayed; raw-JSON toggle; copy-link and copy-JSON buttons.
- **Error states are first-class and distinct**: CORS-blocked PDS; record/collection/repo not found; account deactivated or taken down (surface the PDS error message); network failure. Each has its own copy, in interface voice, stating what happened and what the user can do.

## Phase 3 — export

- Repo view: **Export repo (.car)** — an anchor with `download` pointing at `{pds}/xrpc/com.atproto.sync.getRepo?did={did}`. A CAR (Content Addressable aRchive) is the protocol's canonical full-repo format — spec-guaranteed unauthenticated; the same file used for account migration. One sentence of UI copy says exactly that.
- Collection view: **Export collection (.ndjson)** — cursor-walk all pages, streaming each record as one JSON line `{uri, cid, value}`; progress indicator (records so far); assembled as a Blob download. Abortable. Politeness: sequential requests, no parallel fan-out.
- Record view: **Download record (.json)**.
- Tests: NDJSON assembly and cursor-termination logic from fixtures; filename conventions (`{did}.car`, `{did}.{collection}.ndjson`, `{did}.{collection}.{rkey}.json`).

## Phase 4 — inline blobs

- Detect blob references in record values: objects with `$type: "blob"`, `ref.$link` (CID), `mimeType`, `size`. (Shape is on the verify ledger — confirm against a live record fixture before building.)
- Image mimeTypes render inline in the record view via `{pds}/xrpc/com.atproto.sync.getBlob?did={did}&cid={cid}`, lazy-loaded (`loading="lazy"`), with alt text naming the field path, max-height constrained.
- Non-image blobs: labeled download link with mimeType and human-readable size.
- Broken/blocked blob loads degrade to the download-link presentation, not a broken image icon.

## Phase 5 — PWA

- Web app manifest: name pdsview, theme/background from tokens, icons derived from the wordmark (simple monogram acceptable this run).
- Service worker: cache-first for the app shell (HTML/JS/CSS/fonts/icons) only; **network-only for every XRPC, plc.directory, and well-known request** (test this routing decision); `skipWaiting` + `clients.claim` update flow; offline visit shows the shell with an offline notice.
- Playwright: app loads from cache with network disabled; API calls are never served from cache.

## Verify-in-run ledger

Empirical probes via the manual live-probe script, run once and results recorded in the run summary BEFORE building on them:

1. Unauthenticated `describeRepo` / `listRecords` / `getRecord` against the reference PDS from a browser origin (expected to work; docs banner says "usually require authentication" — reality check).
2. CORS behavior of at least one third-party PDS on the same reads (pick any non-bsky.network PDS from the wild) — informs how common the CORS error state is.
3. `getBlob` in an `<img>` cross-origin: content-type and any content-disposition behavior.
4. Exact blob-ref JSON shape from a live record with an image.
5. `plc.directory` CORS from a browser origin.
6. Any visible rate-limit headers during a multi-page `listRecords` walk (informs NDJSON pacing).

If a probe contradicts this file, stop that phase and report; do not silently re-architect.

## Run summary requirements

- Manual owner steps up top.
- Red-to-green evidence per phase (failing test reference → implementing commit).
- Probe results for the six ledger items.
- Contrast ratios actually shipped for every text pair, before/after any derived ink variants.
- All [confirm] items restated with what was assumed: license, wordmark case, landing blurb copy (quoted in full for line-by-line review).
- Anything deferred, with why.

## Appendix — API quick reference (all GET, no auth)

- Repo description: `{pds}/xrpc/com.atproto.repo.describeRepo?repo={did}` → `{ did, handle, didDoc, collections[] }`
- List records: `{pds}/xrpc/com.atproto.repo.listRecords?repo={did}&collection={nsid}&limit={n}&cursor={c}` → `{ records: [{uri, cid, value}], cursor? }`
- Single record: `{pds}/xrpc/com.atproto.repo.getRecord?repo={did}&collection={nsid}&rkey={rkey}` → `{ uri, cid, value }`
- Full repo CAR: `{pds}/xrpc/com.atproto.sync.getRepo?did={did}` (binary)
- Blob: `{pds}/xrpc/com.atproto.sync.getBlob?did={did}&cid={cid}` (binary, original bytes)
- DID doc (plc): `https://plc.directory/{did}`
- DID doc (web): `https://{domain}/.well-known/did.json`
- Handle → DID, rung 1: `https://{handle}/.well-known/atproto-did` (text/plain DID)
- Handle → DID, rung 2: `https://api.bsky.app/xrpc/com.atproto.identity.resolveHandle?handle={handle}` → `{ did }`
