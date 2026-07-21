# greetings.croft.ing MVP — 1:1 link-delivered cards (phase plan, Pass 1)

Build target repo: `CroftCommunity/greetings_site` (live on GitHub Pages at
https://greetings.croft.ing/). This plan doc lives in the discovery corpus (where the E43 reasoning
and the card-ingest proofs live); execution code lands in `greetings_site`.

## Status

**Executing** — Pass 1–3 complete; Phase 0 done; **Phases 1a + 1b + 2 SHIPPED and live** at
https://greetings.croft.ing/ (routed PWA shell, strict CSP + SRI, Croft design palette, creator OAuth
sign-in). `greetings_site` at `CroftC/greetings_site` on `main`; Pages = `gh-pages`/root. Phase 2's
sign-in wiring + hosted client-metadata are live; **the interactive OAuth round-trip is pending
user browser-confirmation.** **Next: Phase 3 (public card).**

## Outcome Summary

| Phase | Outcome | Ref | Note |
|-------|---------|-----|------|
| Phase 0 Discovery | ✅ D1–D3 resolved; D4 read-leg (write leg gated on creds) | discovery findings + `scratchpad/` spikes | getBlob-CORS gate cleared; deploy model corrected to gh-pages |
| Phase 1a Build + deploy | ✅ SHIPPED + live | greetings_site `33c0e89` | deploy loop green; byte-identical bundle live; Pages = `gh-pages`/root |
| Phase 1b Shell + router + PWA + docs | ✅ SHIPPED + live | greetings_site `ca67803`; discovery `8fdafb0` | hash router (TDD 6/6) + view shells + strict CSP/SRI + installable PWA; CI e2e 6/6; pwa-spa-best-practices.md + pointers |
| Phase 2 Creator OAuth | 🟢 SHIPPED + live (interactive OAuth = user-confirm) | greetings `5734581` | BrowserOAuthClient + hosted client-metadata live (`client_id`===URL); sign-in form; auth-core TDD 7/7; CI e2e 9/9; Croft palette adopted |
| Phase 3 Public card | ☐ not started | — | next |
| Phase 4 Sealed card | ☐ not started | — | — |

## Problem Statement

Group/personal e-card incumbents (GroupGreeting and similar) have weak UI, are functionally basic, and
custody all content in plaintext on their servers behind an unguessable link. Croft can beat that
baseline on privacy without asking recipients to sign in, using mechanics already proven this cycle
(the card-ingest link-key tier: `alpha/experiments/card-ingest/`, ROADMAP_TODO E43, design note
`alpha/thinking/app/ponds/virtual-cards-and-guestbooks.md`).

**This plan is the first product slice only:** a **1:1 greeting card** the creator makes and delivers
via a link, in two privacy modes:
- **(a) server-blind link-key** — a symmetric key rides in the URL fragment; the card content is
  encrypted client-side and stored as ciphertext on the creator's PDS; only link holders can read it.
- **(b) public** — plaintext, bare atproto records; anyone with the link (or the record) can read.

Delivered as a **PWA/SPA** on GitHub Pages. Creator signs in to write to their PDS; recipient opens a
link with no login to read.

**Explicitly deferred (do NOT build in this slice), noted so the architecture leaves room:**
single-creator + multi-write **anonymous** cards (the content-blind ingest / no-login mediation, which
needs OAuth+DPoP scoped delegation and a mediating service), **guestbooks**, and **registries** (the
repo description names all four; they are the same collaborative-ingest primitive at higher write
policies). The scheduled **reveal** (blind-until-open) is also deferred — it needs the shim; MVP 1:1
delivery is "share the link when ready."

## Reasoning

**Why this slice first.** The link-key tier is the part of the model that is *fully proven and needs
no new trust surface*: client-side seal/open, content-address, and PDS create/read/delete were all
validated this cycle (the last leg live against a real PDS). The deferred pieces (anon multi-write)
require the DPoP scoped-delegation OAuth flow and a mediating service — real new work with an external
trust dependency. Shipping the 1:1 card first delivers a usable product on proven rails and de-risks
the rest.

**Why static SPA on Pages (no server).** The repo is already configured for it (Pages built, CNAME
`greetings.croft.ing`), and it matches the Croft.ing "safety-deposit-box single-renderer, no custom
AppView" design (`beta/croft/croft-ing-the-website-and-the-plot.md`). A static renderer makes
server-blindness *structural*: there is no server to leak to. The PDS is the only store; the SPA is a
pure client. Reads are unauthenticated public `getRecord`; the only privileged action is the creator
writing to their own PDS, which happens client-side under the creator's own session.

**Why the browser reimplements the crypto rather than reusing the Rust crates.** The card-ingest
crates are Rust (ChaCha20-Poly1305). The MVP card is a self-contained browser artifact: a card sealed
by the browser is opened by the browser, so cross-implementation interop with the Rust crates is not
required for this slice. That means we can use **WebCrypto natively** rather than shipping a WASM/JS
ChaCha build. WebCrypto natively supports **AES-256-GCM** (an AEAD, same security shape as
ChaCha20-Poly1305) but not ChaCha, so AES-GCM is the likely choice — confirmed in Phase 0 (D1). The
Rust proof still stands as the model's evidence; the browser is a second faithful implementation of
the same "encrypt client-side, store ciphertext, key in the fragment" shape.

**Why the key goes in the fragment, not a query param** (carried from the design note): the URL
fragment is never sent to the server (or to Pages, or logged), so the store only ever sees ciphertext.
A query param would be logged everywhere and defeat the property.

**Alternatives considered and rejected.**
- *A server/AppView backend now.* Rejected: unneeded for 1:1 (the deferred anon-multi-write is the
  only part that needs a mediating service), and it would forfeit the structural server-blindness.
- *Reuse the Rust crates via WASM in the browser.* Rejected for MVP: adds a build toolchain and a
  larger bundle for no benefit, since the card is browser-sealed and browser-opened. Revisit only if
  cross-impl interop (a Rust client opening a browser-sealed card) becomes a requirement.
- *Fork the Croft.ing single-renderer wholesale.* Deferred to a D-item: reuse its conventions
  (atproto-records-as-state, hash routing, tectonic visual system) but greetings is a distinct, simpler
  surface; decide reuse-vs-fork in D3 rather than assume.

## Verified Assumptions

- **greetings_site hosting** — public repo, default branch `main`; GitHub **Pages is enabled and
  built**, source `main`/root, live at https://greetings.croft.ing/ (CNAME `greetings.croft.ing`).
  Confirmed via `gh api repos/CroftCommunity/greetings_site/pages` and `/contents` (root has
  `CNAME`, `LICENSE`). Pass-1 finding: Pages source was `main`/root. **Phase 0 correction (2026-07-21):**
  adopting arecipe's model (Q7) changes the deploy loop to **Actions build (`scripts/build.mjs` →
  `dist/`) → push to the `gh-pages` branch root** via `scripts/pages-deploy.sh`, so greetings needs a
  **one-time flip** of its Pages source from `main`/root to `gh-pages`/root (see Phase 1a).
- **Card-ingest link-key tier proven** (this cycle): `card-seal` seal/open (real ChaCha20-Poly1305),
  `content_address` (blake3), and the PDS storage leg (create/read/delete of an encrypted contribution
  round-tripping as opaque bytes) validated live against a real bsky PDS
  (`alpha/experiments/card-ingest/`, CAPABILITIES.md).
- **atproto reads/writes** — `com.atproto.repo.getRecord` is an unauthenticated public read;
  `createRecord`/`uploadBlob` require a session; custom NSIDs propagate with no pre-registration
  (validated live this cycle). atproto OAuth is DPoP-bound, tokens short-lived, refresh single-use
  rotating (verified against atproto.com/specs/oauth in the design note).
- **WebCrypto** — natively supports AES-256-GCM (AEAD); does NOT support ChaCha20-Poly1305. (General
  WebCrypto fact; the *choice* to use AES-GCM for the browser tier is D1, not yet decided.)
- **arecipe.app is our reference PWA/SPA (live on Pages) and settles the auth pattern.** Inspecting the
  live site (fetched arecipe.app/signin.html + its JS chunks, 2026-07-21): sign-in uses **atproto OAuth
  in the browser** — the `@atproto/oauth-client-browser` shape, with **DPoP** (56 refs), **PAR**
  (`pushed_authorization`, 11), **PKCE** (`code_challenge`, 5), a hosted **`/oauth-client-metadata.json`**,
  and `client_id`. `createSession`/`identifier`/`password` appear only as part of the bundled
  `@atproto/api`, not the sign-in flow. The shell also ships: a **strict CSP** (`default-src 'none'`;
  hashed inline scripts; `connect-src` scoped to `bsky.social` / `public.api.bsky.app` / `plc.directory`
  + `https:`); **SRI** (`integrity="sha384-..."` + `crossorigin`) on every script/style/font;
  **content-hashed bundles** from a TS + bundler build; a **PWA manifest**; and a **no-flash theme**
  inline script. This is the existence proof that **OAuth-on-Pages works** and the baseline greetings
  reuses — it also seeds our PWA/SPA best-practices doc (see Documentation Impact).
- **arecipe's full build/test stack (confirmed from its `package.json` + repo root, 2026-07-21) is the
  greetings baseline** — and it settles a gap Pass 1 missed: OAuth via `@atproto/oauth-client-browser`
  is an **npm dependency**, so a **build step is mandatory** (the phases cannot be hand-authored static
  JS). arecipe's stack: **esbuild** bundler (`scripts/build.mjs`) + **TypeScript**; **vitest** unit
  tests (`happy-dom` + `fake-indexeddb` for DOM/storage); **@playwright/test** for e2e and `LIVE=1`
  live tests; **eslint**; deps `@atproto/oauth-client-browser` + `@atproto/api` + `@atproto/oauth-types`
  + `@ipld/dag-cbor` + `multiformats`; a **committed `client-metadata.json`** at repo root; and
  `.github/workflows/{ci,preview}.yml`. greetings reuses this wholesale (it is exactly the PWA/SPA
  best-practices baseline the user asked to start).

**Phase 0 execution findings (2026-07-21, firsthand — non-browser legs):**
- **[D1 — RESOLVED]** AES-256-GCM browser round-trip confirmed in WebCrypto (`crypto.subtle`, run under
  Node v25 whose engine is identical to the browser's). All checks pass: `genKey` (256-bit) → raw export
  to **43-char base64url** → reimport → seal/open **round-trips**; ciphertext ≠ plaintext; **wrong-key
  fails** and **tampered ciphertext fails** (GCM auth tag); **fresh 96-bit IV per op**; the cover-byte
  path (`sealBytes`/`openBytes`) round-trips with a **distinct IV** from the payload. **Fragment grammar
  DECIDED:** `#/c/<did>/<rkey>#k=<base64url-raw-key>`; the record stores `{iv, ct}` (+ `coverIv`); the
  key lives only in `#k=`. Evidence: `scratchpad/d1-crypto.mjs`, all 9 assertions PASS. (Browser-runtime
  `location.hash` handling is a Phase-1b/4 wiring concern, not a crypto-mechanics one.)
- **[D2 — reads/CORS RESOLVED; own-OAuth-hosting still Phase 2]** DID→PDS resolution works
  (`bsky.app` → `did:plc:z72i7hdynmk6r22z27h6tvur` → PDS `puffball.us-east.host.bsky.network` via
  `plc.directory`). Unauthenticated `getRecord` (profile/self) and `getBlob` (avatar, `image/jpeg`,
  256 KB) both return **HTTP 200 with `Access-Control-Allow-Origin: *`**, and the CORS **OPTIONS
  preflights return 204 with `ACAO: *`** (GET allowed). **This retires the plan's biggest risk — the
  getBlob-CORS gate is clear** for bsky-network-hosted accounts. *Caveat: Node does not enforce CORS;
  this is header inspection, but `ACAO: *` is a wildcard a browser honors unconditionally for a simple
  GET. Self-hosted PDSes could differ; MVP targets bsky-hosted.* Evidence: `scratchpad/d2d4-reads.mjs`.
  **New finding:** an OAuth `redirect_uri` cannot contain a `#` fragment, so greetings (hash-routed)
  needs a **concrete callback path**, not a `#/callback` route (arecipe uses `signin.html`).
- **[D3 — build/deploy + reuse boundary RESOLVED; live-routing folds into Phase 1a]** From arecipe's
  actual files: build = `node scripts/build.mjs` (esbuild) → `dist/`; deploy = **GitHub Actions builds
  and pushes `dist/` to the `gh-pages` branch root** via `scripts/pages-deploy.sh` (plain git, no
  third-party action), Pages serving from `gh-pages/root`, per-PR previews at
  `gh-pages:/pr-preview/pr-N/`. CI is hermetic (lint + typecheck + unit + build + e2e on the built
  bundle, node 22, `npm ci`, no creds); the `@live` real-PDS tier runs **locally per phase**, not in CI.
  Confirmed present at arecipe root: `package.json` (deps exactly as the VA claims — `@atproto/*`,
  `@ipld/dag-cbor`, `multiformats`; devDeps esbuild/typescript/vitest/happy-dom/fake-indexeddb/
  playwright/eslint), `tsconfig.json` + `tsconfig.tests.json` + `vitest.config.ts` + `playwright.config.ts`
  + `eslint.config.js`, committed `client-metadata.json`, `manifest.webmanifest`, `.github/workflows/
  {ci,preview}.yml`. **Reuse boundary:** copy toolchain + configs + CI + `pages-deploy.sh` +
  client-metadata pattern + CSP + the `authModeFor` gate (sign-in only on production-origin/loopback,
  else read-only); greetings diverges as a **single-page hash-routed shell** (not arecipe's multi-page
  per-view HTML). Evidence: `scratchpad/arecipe_{package.json,client-metadata.json,ci.yml}`,
  `gh api .../contents`.
- **[D4 — read leg RESOLVED; write leg needs credentials]** Unauthenticated `getBlob` read + CORS
  confirmed above (200, `ACAO: *`, `image/jpeg`, 256 KB observed). The blob-ref embed shape is the
  atproto standard `{$type:"blob", ref:{$link:cid}, mimeType, size}`. **NOT verified here (gated):** the
  authenticated `uploadBlob` write leg, the exact blob **size cap**, allowed image types, and the
  sealed-ciphertext-as-blob round-trip — these need a real creator session (a **rotated bsky app
  password**, not available in this sandbox). Folds into Phase 3/4's live gate.

**Still unverified after Phase 0 (browser- or credential-gated; fold into the implementation phase that
runs in that environment):**
- **Live hash routing on Pages** (hash routing needs no server rewrite; confirmed as a fact, but the
  live 404-vs-hash behavior on greetings' Pages is confirmed by Phase 1a's first real deploy).
- **greetings' own OAuth flow** — hosting `greetings.croft.ing/client-metadata.json`, the concrete
  callback path, and the redirect round-trip on Pages — needs a browser (Phase 2 gate).
- **Authenticated `uploadBlob`** — the write leg, exact blob size cap, allowed image types, and the
  sealed-ciphertext-as-blob round-trip — needs a rotated bsky app password (Phase 3/4 gate).
*(Resolved in Phase 0: CORS reads + getBlob CORS, DID→PDS resolution, blob-ref shape, AES-GCM
round-trip, and the build/deploy + reuse-boundary model — see "Phase 0 execution findings" above.
Creator-auth mechanism remains DECIDED as OAuth reusing arecipe.)*

## Documentation Impact

- `greetings_site/README.md` — created/expanded in Phase 1b (what the app is, the two card modes, the
  link grammar, local dev + deploy). Currently the repo has only `CNAME` + `LICENSE`.
- `discovery/alpha/experiments/card-ingest/README.md` — add a pointer (Phase 1b) that greetings_site is
  the product consumer of the link-key tier. *(Pass 3: file confirmed present.)*
- `discovery/alpha/thinking/app/ponds/virtual-cards-and-guestbooks.md` — add a "productization"
  pointer to greetings_site + this plan (Phase 1b). *(Pass 3: file confirmed present.)*
- `discovery/alpha/ECOSYSTEM.md` (**§5c-3** Croft-owned live properties — Pass 3: the register is
  §5c-3, not §5c; heading confirmed at line 186) — register greetings.croft.ing as a live Croft
  property (Phase 1b). *(Grep confirmed arecipe/skylite are registered there at lines 192/195;
  greetings is not yet.)*
- `discovery/alpha/ROADMAP_TODO.md` E43 — add a "productization underway: greetings.croft.ing (plan
  <this file>)" note (Phase 1b). *(Pass 3: E43 confirmed at line 166.)*
- `discovery/alpha/thinking/app/pwa-spa-best-practices.md` — **new** (Phase 1b): our PWA/SPA baseline,
  seeded from arecipe's live patterns (OAuth-browser via `@atproto/oauth-client-browser` + hosted
  client-metadata.json; strict CSP with `default-src 'none'` + hashed inline scripts + scoped
  `connect-src`; SRI on every asset; content-hashed bundles from a TS+bundler build; PWA manifest;
  no-flash theme). greetings is the second consumer; the doc is the reusable standard the user asked to
  start building. *(Grep: no existing pwa/spa best-practices doc under `thinking/app/`.)*

## Concurrency Map

Sequential spine: Phase 0 (D1–D4) → Phase 1a → Phase 1b → Phase 2 → Phase 3 → Phase 4.

All implementation phases are **sequential**: each builds on the prior (build → shell → auth → public
card → private card), sharing the same `greetings_site/src/*` module set (`router.ts`, `auth.ts`,
`atproto.ts`, `pds.ts`, `crypto.ts`, `views/*`), so their write-sets overlap and they cannot be
parallelized. Phase 1 was split into **1a (build/deploy foundation)** and **1b (shell + PWA + docs)**
to honor the 4-file rule; 1a must precede 1b (the shell builds on the toolchain). Phase 1b writes to
**two repos** (greetings_site app files + discovery doc pointers/best-practices doc) as two separate
commits — no shared write-set between the repos, but it is one logical phase.

Within Phase 0, the four discovery probes (D1–D4) are independent and **may run in parallel** (each is
a throwaway/keep probe touching only its own scratch files), but they are cheap enough that sequential
is fine; parallelism here is optional, not required.

Phase 0 parallel set {D1, D2, D3, D4} (optional):
- Disjoint write-sets: each probe writes only its own scratch file under a `spike/` scratch dir
  (`spike/d1-crypto.html`, `spike/d2-cors.*`, etc.); none writes app files.
- Shared-state contract (Pass 3 — corrected, was inaccurately blanket "read-only / no shared writes"):
  the probes are **not uniformly read-only**, so state the per-probe reality:
  - **D1** is **fully local** (browser WebCrypto in a scratch HTML file) — no network, no external state.
  - **D2, D4** are **read-only against external services** (bsky PDS/entryway, `plc.directory`;
    `getRecord`/`getBlob`; D4 also does one *authenticated* `uploadBlob` to a **disposable test
    account's** PDS — an external write, but to a throwaway repo, deletable, disjoint from any other
    probe's target).
  - **D3 writes to shared external state:** its probe (1) deploys a 2-route stub to the
    **greetings_site repo / live Pages site**. This is an external write, disjoint in *target* from
    D1/D2/D4 (none of them touch greetings_site), so the parallel set is still safe — but it is a write,
    not a read. Because D3 mutates the same greetings_site `main`/Pages that Phase 1a will initialize,
    **D3 must land (and its stub be reconciled) before Phase 1a starts** (already sequential: 0 → 1a).
  - No probe mutates local git HEAD/branch, binds a port, or starts a daemon.
- Re-entry verification: no app file changed by any probe; `git status` clean except the `spike/`
  scratch dir; D4's test-account records/blobs deleted (or noted for GC); **D3's stub deploy either
  removed or explicitly carried forward as the Phase 1a base (decide in D3, record in the Review Log)**;
  findings recorded in the plan's Verified Assumptions; scratch dir removable.

## Phases

### Test execution split (RED→GREEN under sandbox constraints) — Pass 3

RED→GREEN wiring discipline applies to every phase, but the tests execute in two tiers because this
sandbox cannot drive a headless browser (Chromium egress is blocked — see the memory note
"sandbox-browser-egress-blocks-live-tests"; it is a network block, not a cert issue):

- **vitest unit tier (runs here, in-sandbox):** the pure modules — `router.ts` (`hash → view`),
  `crypto.ts` (seal/open), the link codec, the record builder, the DID resolver's URL/`did:plc`
  parsing. These are the sandbox-runnable floor and follow RED→GREEN **in this environment**: write the
  failing test, watch it fail, implement, watch it pass.
- **playwright e2e/live wiring tier (runs in CI or a browser-capable env, NOT here):** every phase's
  named wiring test (deploy loop, route render, OAuth round-trip, create→view, create→decrypt +
  keyless-fails). These are authored RED (failing or absent) at phase start and driven GREEN in CI / a
  local browser; here they are exercised **manually on the live site**. A phase is not "done" until its
  wiring test is GREEN in a browser-capable env — a green vitest suite alone does not close a phase.

Each phase's **Wiring test** and **Validation** fields already name which tier applies; this note is the
cross-cutting statement of the discipline so no phase is read as "unit tests are sufficient."

### Observability, error surfacing & live-testing credentials (cross-cutting) — Pass 3

**Error surfacing (user-facing) and diagnostics.** Every failure path that a recipient or creator can
hit must surface a plain, non-technical message in the UI and a diagnostic in the dev console — never a
silent failure or a blank card (fail-loud). Specifically:
- **Auth (Phase 2):** OAuth start/callback failures (PAR/PKCE/DPoP errors, redirect mismatch, session
  restore failure) → a user-facing "sign-in failed, try again" state + a console diagnostic naming the
  step. **Never** log or surface tokens, the DPoP key, or refresh material.
- **View path (Phase 3/4):** DID→PDS resolution failure, `getRecord` not-found/error, and **`getBlob`
  CORS/fetch failure** → distinct user-facing messages ("this card link looks broken", "couldn't load
  the cover image") + a console diagnostic naming which leg failed (resolution vs record vs blob), so a
  post-deploy failure is traceable to a phase.
- **Decrypt (Phase 4):** missing key → "you need the link's key to read this"; present-but-wrong key or
  GCM auth-tag failure (tamper) → a **distinct** "this card couldn't be decrypted" message. **Hard
  rule:** the `#k=` fragment, the raw/imported key, and decrypted plaintext must never enter
  `console.log`, an error report, analytics, the service worker, or any network request. The Phase 4
  Risks item already names the fragment-never-networked audit; this makes the logging side explicit.

**Live-testing credentials.** Any live leg (Phases 2/3/4 against a real bsky PDS) uses a **bsky app
password for a disposable test account, supplied via an environment variable, never committed** and
never pasted into the plan or a fixture. The password shared in a prior session must be treated as
compromised and **rotated** before any live run; do not reuse it.

### Phase 0: Discovery

**Goal:** Resolve the four unknowns before committing to the SPA structure. Discovery Exemption applies
(no TDD on probe code; each task declares a disposition).

- [x] **D1: Confirm the AES-256-GCM browser round-trip + key/link format.** — ✅ **RESOLVED 2026-07-21**
    (all 9 checks pass; fragment grammar decided; see Phase 0 execution findings). (Cipher DECIDED:
    AES-256-GCM; see Open Questions. D1 confirms the mechanics, it does not choose the cipher.)
  - **Probe:** In a browser context, WebCrypto `AES-GCM` 256-bit: `genKey` (fresh random 256-bit key),
    fresh random 96-bit IV, encrypt+decrypt round-trip of a UTF-8 card payload; confirm ciphertext ≠
    plaintext, decrypt-with-wrong-key fails, tamper fails; confirm raw-key export/import
    (`exportKey('raw')` → base64url → `importKey('raw')`) for the fragment; settle the fragment grammar.
  - **Success criteria:** A working `seal(payload,key)->{iv,ct}` / `open({iv,ct},key)` + `genKey` in
    browser JS with AES-256-GCM, and a decided fragment grammar (expected: locator in the path/hash,
    key as `#k=<base64url>`; IV stored with the record, never the key).
  - **Disposition:** `promote` (Pass 3: was mislabeled `keep-as-fixture`; the spike is executable
    logic, not reference data, so it is a promotion). The spike's seal/open/genKey/base64url logic is
    promoted to production **`src/crypto.ts` in Phase 4**, where it is (re)written under TDD — the D1
    spike is the reference, and Phase 4 is the named follow-up phase that applies RED→GREEN to the
    promoted code. The fragment-grammar decision D1 settles is recorded in Verified Assumptions.
- [x] **D2: Confirm CORS reads and greetings' own OAuth client-metadata hosting.** — ✅ **reads/CORS
    RESOLVED 2026-07-21** (DID→PDS resolution + `getRecord`/`getBlob` all HTTP 200 with `ACAO: *`,
    preflights 204; getBlob-CORS gate cleared). Residual (greetings' own client-metadata hosting +
    redirect round-trip) is browser/Phase-2 wiring, not a discovery unknown. (Auth mechanism
    already DECIDED: OAuth browser client reusing arecipe; see Open Questions / Verified Assumptions.
    D2 no longer chooses the mechanism, it confirms greetings' own setup.)
  - **Probe:** (1) **DID→PDS resolution + reads:** resolve a `did:plc:*` via `plc.directory` to its PDS
    endpoint, then unauthenticated `getRecord` and `getBlob` (by DID+CID) from a static origin; confirm
    CORS allows both (custom-NSID records/blobs are not served by the public appview/CDN, so the view
    path must resolve+hit the owning PDS). (2) draft `greetings.croft.ing/client-metadata.json` modeled
    on arecipe's committed one, confirm the OAuth redirect/callback works from the Pages origin (diff
    arecipe's client-metadata + CSP `connect-src`).
  - **Success criteria:** DID resolution + unauthenticated `getRecord`/`getBlob` work cross-origin from
    the static origin; a valid greetings client-metadata.json + confirmed redirect on Pages, matching
    arecipe. **Gate:** if `getBlob` CORS is blocked from the static origin, the cover-image view path is
    at risk (no server to proxy) — surface before Phase 3 (see Risks).
  - **Disposition:** `keep-as-fixture` — the client-metadata.json and CSP are config/data (not
    executable logic), so they are copied as reference into Phase 1b (CSP) / Phase 2 (client-metadata).
- [x] **D3: Confirm routing + build/deploy on Pages; settle the arecipe reuse boundary.** — ✅
    **RESOLVED 2026-07-21** (build = esbuild→`dist/`; deploy = Actions→`gh-pages` root via
    `pages-deploy.sh` + one-time Pages-source flip; hermetic CI, `@live` local per phase; reuse boundary
    listed; greetings = single-page hash-routed shell). Live 404-vs-hash confirmation folds into Phase
    1a's first deploy. (Stack
    DECIDED: reuse arecipe's esbuild + TypeScript + vitest + playwright + eslint; see Verified
    Assumptions. D3 does not choose the stack.)
  - **Probe:** (1) confirm client-side routing on Pages for the link grammar (hash routing needs no
    server rewrites; a path-routed SPA needs a `404.html` fallback) by deploying a 2-route stub;
    (2) confirm the **build/deploy model** by reading arecipe's `scripts/build.mjs` + `.github/workflows`
    (esbuild → does it commit built assets to the Pages-served path, or Actions-deploy?); (3) decide the
    **reuse boundary** — which arecipe files to copy as the greetings base (`scripts/build.mjs`,
    `tsconfig.json`, `vitest.config.ts`, `playwright.config.ts`, `eslint.config.js`, the OAuth
    `client-metadata.json` shape, CSP) vs greetings-specific.
  - **Success criteria:** routing scheme proven on live Pages; a decided build/deploy model matching
    arecipe; a listed reuse boundary (files copied vs written fresh).
  - **Disposition:** `keep-as-fixture` — the routing stub + copied config files (build/test/lint) are
    reference/config, copied as the Phase 1a/1b build + shell base (not logic under TDD).
- [ ] **D4: Confirm the cover-image blob round-trip (schema DECIDED; see Open Questions).** — 🟡
    **read leg RESOLVED 2026-07-21** (unauthenticated `getBlob` 200 + `ACAO: *` + `image/jpeg` 256 KB;
    blob-ref shape is standard `{$type:"blob", ref:{$link:cid}, mimeType, size}`). **Write leg NOT
    verified — gated on credentials:** authenticated `uploadBlob`, exact size cap, allowed types, and
    the sealed-ciphertext-as-blob round-trip need a rotated bsky app password → confirmed at Phase 3/4's
    live gate. This box stays unchecked until the write leg runs.
  - **Probe:** Confirm the atproto blob path both directions from a static browser client: (1)
    `uploadBlob` a cover image (authenticated, creator session) and reference it in an
    `ing.croft.greeting.card` record; (2) unauthenticated `getBlob` (by DID + CID) of that cover from
    the static origin, confirming CORS allows it (or whether the bsky image CDN is needed). Record the
    max acceptable cover size (uploadBlob size limit) and the image types allowed. Confirm the sealed
    path works too: upload AES-GCM *ciphertext* bytes as the blob, `getBlob`, decrypt with the card
    key + `coverIv`.
  - **Success criteria:** a blob create→reference→read round-trip confirmed from the static origin
    (both plain and ciphertext bytes), with CORS confirmed on `getBlob`, plus a recorded size cap and
    allowed types.
  - **Disposition:** `throwaway` (findings recorded; the blob helpers are written TDD in Phase 3/4).

**Done when:** D1–D4 answered with firsthand evidence; Verified Assumptions updated; Open Questions
2–3 resolved; phases adjusted if a probe invalidates an assumption (recorded in the Review Log).

### Phase 1a: Build + deploy foundation (reuse arecipe's toolchain) — ✅ SHIPPED (`33c0e89`), live

**Delivered (2026-07-21, commit `33c0e89` on `greetings-mvp`):** all files below created; `npm test`
(lint + typecheck + test:unit + build) exits 0; `dist/` contains the content-hashed bundle
(`main-<hash>.js`), the rewritten `index.html`, `CNAME`, and `.nojekyll`. **Two deviations from the
Pass 3 spec, both deliberate:** (1) the `build.mjs` is the minimal single-page core — CSP/SRI/SW/manifest
are Phase 1b as specified, not 1a. (2) The aggregate `npm test` **omits `test:e2e`** for 1a (there are
no routes to exercise yet); playwright e2e joins `npm test` in Phase 1b. **Flag (plan write-set gap):**
wiring e2e into CI in 1b requires editing `package.json` (`test` script) and `.github/workflows/ci.yml`
(add `npx playwright install --with-deps chromium`) — neither is in Phase 1b's declared write-set;
add them there. **Behavioral gate MET (2026-07-21):** `greetings-mvp` merged to `main`, pushed; Actions
`test`+`deploy` both green; `pages-deploy.sh` created `gh-pages` with `dist/`; Pages source flipped to
`gh-pages`/root; the live site serves a **byte-identical** hashed bundle
(`https://greetings.croft.ing/main-LQHKV6LT.js` == local build). Deploy loop proven end-to-end.

**Goal:** greetings_site has a working esbuild+TS build, a test harness, a CI/deploy path, and a
minimal built page live on Pages. This exists because OAuth (`@atproto/oauth-client-browser`) is an
npm dependency and cannot be hand-authored static JS (Pass 2 finding).
**Changes:**
- [ ] `greetings_site/package.json` — deps (`@atproto/oauth-client-browser`, `@atproto/api`,
  `@atproto/oauth-types`, `@ipld/dag-cbor`, `multiformats`) + devDeps (matching arecipe's confirmed set:
  `esbuild`, `typescript`, `typescript-eslint`, `@types/node`, `vitest`, `happy-dom`, `fake-indexeddb`,
  `@playwright/test`, `eslint`); scripts mirrored from arecipe (`build`, `typecheck`, `test:unit`,
  `test:e2e`, `test:live` = `LIVE=1 playwright test`, `lint`, `test`).
- [ ] build/config copied+adapted from arecipe: `scripts/build.mjs` (esbuild → `dist/`), `tsconfig.json`,
  `tsconfig.tests.json`, `vitest.config.ts`, `playwright.config.ts`, `eslint.config.js`.
- [ ] `greetings_site/.github/workflows/ci.yml` — **D3-confirmed model:** hermetic `test` job (lint +
  typecheck + unit + build + e2e on the built bundle, node 22, `npm ci`, no creds) → a `deploy` job
  (on `main` push) that runs `scripts/build.mjs` and pushes `dist/` to the **`gh-pages` branch root**
  via `scripts/pages-deploy.sh` (plain git, `contents: write`, `concurrency: gh-pages`). Copy
  `scripts/pages-deploy.sh` and `.github/workflows/preview.yml` (PR previews at
  `gh-pages:/pr-preview/pr-N/`) from arecipe.
- [ ] **One-time setup (not a file):** flip greetings' Pages source from `main`/root to
  **`gh-pages`/root** (`gh api ... /pages` or repo settings); ensure the build copies `CNAME` into
  `dist/`. This is the D3 deploy-model correction (Pass-1 assumed `main`/root).
- [ ] `greetings_site/src/main.ts` + a minimal `index.html` that loads the built bundle (hello-world,
  proves the build→`gh-pages`→Pages loop).
**Call chain:** `npm run build` (esbuild) → hashed bundle emitted to `dist/` → Actions `deploy` job
pushes `dist/` to `gh-pages` root (`pages-deploy.sh`) → Pages serves `gh-pages`/root →
https://greetings.croft.ing/ loads the built bundle.
**Wiring test:** a pushed change to `src/main.ts` appears on the live site after the CI/deploy loop
runs (build → Pages). Validated on the live site (see Validation).
**Depends on:** Phase 0 (D3 build/deploy + reuse-boundary decision).
**Read-set:** arecipe's config files (as copy source, read-only).
**Write-set:** `greetings_site/{package.json,scripts/build.mjs,tsconfig.json,vitest.config.ts,playwright.config.ts,eslint.config.js,.github/workflows/ci.yml,src/main.ts,index.html}`.
**Shared-state contract:** deploys to Pages via `greetings_site` `main` (CI or committed build per D3);
no other ambient state. (These are template config files copied from arecipe, low implementation risk —
the 4-file-rule concern, partial feature completion, does not apply to a config bootstrap.)
**Risks:** Pages caching/propagation delay; CI deploy permissions; esbuild config drift from arecipe.
**Done when:**
1. **Behavioral:** a `src/` change builds and appears live on greetings.croft.ing via the deploy loop.
2. **Verification:** `npm run build` succeeds locally + `npm run test:unit` runs; the built page loads
   on the live site after deploy.
**Validation:** Moderate. `npm run build` + `vitest` run locally/CI; confirm the live deploy loop
manually (browser E2E via playwright runs in CI / a browser-capable env, not this sandbox — headless
browser egress is blocked here). **TDD ordering (Pass 3):** 1a is a mechanical config/toolchain
bootstrap copied from arecipe — it carries **no pure business logic to unit-test first**, so its gate is
the **deploy-loop wiring test** (a `src/main.ts` change reaching the live site), not a RED→GREEN unit
cycle. `vitest` here only proves the harness wires up (a smoke test / zero-or-one trivial test is
acceptable). The **first genuine RED→GREEN vitest cycle is the Phase 1b router**; do not manufacture
hollow unit tests for the bootstrap to satisfy a TDD checkbox.

### Phase 1b: App shell + router + PWA + doc pointers — ✅ SHIPPED (greetings `ca67803`, discovery `8fdafb0`), live

**Delivered (2026-07-21):** `src/router.ts` (pure `hash → Route`, TDD 6/6 unit incl. edges + `#k=`
strip + malformed→notfound + unknown→home) wired via `src/main.ts` to `src/views/{home,create,card}.ts`
shells (`#app` `data-view` marker); strict CSP (`default-src 'none'`, inline-script sha256, scoped
`connect-src`, `img-src` with `data: blob: https:`) + SRI (sha384) on the module + stylesheet, injected
by `build.mjs`; installable PWA (`manifest.webmanifest` + `assets/icons/icon.svg` + app-shell `src/sw.ts`
— shell-only cache, never card data, never the `#k=` fragment); `styles.css`; greetings `README.md`.
Discovery side (separate commit `8fdafb0`): new `pwa-spa-best-practices.md` + pointers (ECOSYSTEM §5c-3,
E43, card-ingest README, design note). **CI:** unit 6/6 + **e2e 6/6** (routing + hashchange nav +
manifest) green in a real browser; deployed to `gh-pages`; live at greetings.croft.ing.
**Resolved the flagged plan gap:** wired e2e into `npm test` + `ci.yml` (`package.json` + `ci.yml`
edited, as flagged in the Phase 1a header). **Deviation:** the PWA icon is a single SVG (`sizes: "any"`,
modern-Chrome installable) rather than PNG sizes — refine with branded PNGs later (a follow-up, not a
stub).

**Goal:** A routed, installable SPA shell on the Phase-1a build, with home / create / view-card view
shells, plus the discovery-side doc pointers and the PWA/SPA best-practices doc.
**Changes:**
- [ ] `greetings_site/index.html` — strict **CSP** (adopt arecipe's: `default-src 'none'`, hashed
  inline scripts, `connect-src` scoped to the auth server + `plc.directory` + `https:` for arbitrary
  PDSes, `img-src 'self' data: blob: https:` for covers), theme-no-flash inline script.
- [ ] `greetings_site/src/{router.ts,views/*.ts}` — hash router + home / `#/create` / `#/c/<locator>`
  view shells; SRI applied to emitted assets by the build.
- [ ] `greetings_site/manifest.webmanifest` + `greetings_site/src/sw.ts` — installable PWA + app-shell
  cache (no card-data caching; SW never handles `#k=` fragments per Q6).
- [ ] `greetings_site/styles.css` — minimal tectonic-aligned styling.
- [ ] `greetings_site/README.md` — what it is, link grammar, dev + deploy, **pointer to the plan**
  (`discovery/alpha/plans/2026-07-21-greetings-croft-ing-mvp.md`) per Q5.
- [ ] discovery-side (separate commit): doc pointers (ECOSYSTEM §5c-3, ROADMAP_TODO E43, card-ingest
  README, design note) + **new `discovery/alpha/thinking/app/pwa-spa-best-practices.md`** codifying the
  arecipe baseline (esbuild+TS, vitest+playwright, OAuth-browser + committed client-metadata.json,
  strict CSP + SRI + hashed bundles, PWA manifest, CI).
**Call chain:** built bundle loads `index.html` → `src/router.ts` parses `location.hash` → renders the
matching view shell (home / `#/create` / `#/c/<locator>` placeholder).
**Wiring test:** loading `https://greetings.croft.ing/#/c/placeholder` renders the card-view shell (not
a 404) and `/#/create` renders the create shell; the app is installable (manifest valid). Playwright
e2e in CI/browser env; manual on the live site here.
**Depends on:** Phase 1a.
**Read-set:** `greetings_site/{package.json,scripts/build.mjs}`; arecipe CSP/manifest as reference.
**Write-set:** `greetings_site/{index.html,src/router.ts,src/views/*,manifest.webmanifest,src/sw.ts,styles.css,README.md}`;
discovery `alpha/{ECOSYSTEM.md,ROADMAP_TODO.md,experiments/card-ingest/README.md,thinking/app/ponds/virtual-cards-and-guestbooks.md,thinking/app/pwa-spa-best-practices.md}`.
**Shared-state contract:** two repos, two commits (greetings_site app + discovery docs); Pages deploy
as in 1a; no other ambient state.
**Risks:** CSP too strict breaks the OAuth flow later (validate connect-src against arecipe's working
set); SW cache staleness (version the SW; hashed bundles bust cache).
**Done when:**
1. **Behavioral:** the live site loads the SPA, client-side routes resolve (no 404 on `#/c/...`), the
   app is installable, and the CSP is in place; the best-practices doc + pointers exist.
2. **Verification:** load the two routes on https://greetings.croft.ing/ + PWA manifest check; the
   discovery docs reference greetings and the best-practices doc exists (site gate green).
**Validation:** Moderate. Unit-test the router (pure `hash → view` function) in vitest, naming the
edges (Pass 3, mutation-resistant): `#/` and empty hash → home; `#/create` → create; `#/c/<did>/<rkey>`
→ card with the locator parsed out; an **unknown route** → the fallback (home, not a crash); a
**malformed locator** (`#/c/` with missing/extra segments) → a defined error/fallback, not an
exception. Manual/live + CI-playwright for the DOM flow; discovery-side site gate for the doc changes.

### Phase 2: Creator sign-in (write capability) — 🟢 SHIPPED (`5734581`), live; interactive OAuth = user-confirm

**Delivered (2026-07-21):** `src/auth-core.ts` (pure `authModeFor` gate / `isLoopbackHostname` /
`normalizeHandle` / `isOAuthCallback`, TDD 7/7) + `src/auth.ts` (`BrowserOAuthClient`: hosted
client-metadata on the production origin, atproto loopback client on 127.0.0.1, read-only elsewhere;
DPoP session via `init()` → `new Agent(session)`; best-effort handle via `describeRepo`, confirmed API).
`client-metadata.json` committed + copied into `dist/`, **live and self-consistent** (`client_id` ===
`https://greetings.croft.ing/client-metadata.json`, root redirect, `transition:generic`, DPoP).
`views/create.ts` gates on auth (signed-out sign-in form / signed-in confirmation / read-only origin);
`main.ts` boots auth, routes an OAuth callback (root `?code&state`) to `#/create`. **CI:** unit 13/13,
**e2e 9/9** (sign-in form reachable, empty-submit validates, client-metadata served). **Design decision
(user, mid-phase):** adopted the **Croft palette** from `crofting_site` (schist/granite/ruddy/moss/ink/
canvas, Lora serif + Inter sans) across the shell + sign-in form + icon/manifest/theme-color — recorded
in `pwa-spa-best-practices.md` scope as the shared visual system. **Deviations:** (1) single-page root
redirect_uri (not arecipe's dedicated page) — the D2 "concrete path, not a hash route" finding is
honored (`/` is a concrete path). (2) Lora/Inter are referenced via font stacks with Georgia/system
fallback; self-hosting the woff2 files is a deferred polish. (3) `@atproto/api` is in the main bundle
(178 KB gz) — code-splitting to defer it is a deferred optimization. **NOT verified (user-confirm):**
the interactive OAuth round-trip (redirect → bsky consent → callback → authenticated call) needs a
browser + the test account; the app-password `@live` port test (arecipe's pattern) is the automatable
proxy for the authenticated-agent capability and can be added when the password is supplied via env.

**Goal:** The creator signs in via atproto OAuth (browser + DPoP, `@atproto/oauth-client-browser`) and
the app holds a session able to write to their PDS.
**Changes:**
- [ ] `greetings_site/client-metadata.json` — committed at repo root (OAuth `client_id` target),
  modeled on arecipe's confirmed shape (`client_id` = the hosted `https://greetings.croft.ing/client-metadata.json`,
  `client_uri`, `logo_uri`, `scope: "atproto transition:generic"`, `grant_types:
  ["authorization_code","refresh_token"]`, `response_types: ["code"]`, `token_endpoint_auth_method:
  "none"`, `application_type: "web"`, `dpop_bound_access_tokens: true`). (OAuth precondition — the
  client cannot start without it.) **Phase 0 D2 finding:** `redirect_uris` must be a **concrete path**
  (OAuth redirect URIs cannot contain a `#` fragment) — arecipe uses `signin.html`; greetings needs a
  dedicated callback page/path (e.g. `/callback.html` or the app entry that parses the OAuth params
  and then routes internally), NOT a `#/callback` hash route.
- [ ] `greetings_site/src/auth.ts` — wraps `@atproto/oauth-client-browser`: `signIn(handle)` (starts
  the OAuth/PAR/PKCE flow), the **callback handler** (the redirect route completes the flow), session
  restore, `whoami`, and a `writeRecord`/`agent` accessor for later phases.
- [ ] `greetings_site/src/router.ts` + a view — sign-in view + the OAuth **callback route**; state
  "signed in as @handle".
**Call chain:** `#/create` → "sign in" → `auth.signIn(handle)` → atproto OAuth (authorize + PAR +
PKCE, DPoP) → redirect back to the callback route → `auth` completes + stores the DPoP-bound session →
app shows handle → authenticated agent available to Phases 3/4.
**Wiring test:** signing in on the live site completes the OAuth redirect and shows "signed in as
@handle"; an authenticated `describeRepo`/`getSession` succeeds. Playwright `LIVE=1` in a browser env;
manual on the live site here.
**Depends on:** Phase 1a/1b; Phase 0 D2 (client-metadata + redirect confirmed).
**Read-set:** `greetings_site/src/router.ts`.
**Write-set:** `greetings_site/{client-metadata.json,src/auth.ts,src/router.ts,src/views/signin.ts}`.
**Shared-state contract:** creates a real DPoP-bound atproto session against bsky (external); session
persisted only where `@atproto/oauth-client-browser` manages it (IndexedDB, its own store) — no
plaintext secrets to localStorage. No git/port state.
**Risks:** client-metadata `client_id`/redirect mismatch (must equal the hosted URL exactly); CSP
`connect-src` must allow the auth server + `plc.directory` (set in Phase 1b, validate here); OAuth
callback must be a **real callback page/path reachable on Pages, NOT a `#/` hash route** (D2 finding —
redirect URIs cannot carry a fragment).
**Done when:**
1. **Behavioral:** a creator signs in and the app can make an authenticated call as them.
2. **Verification:** live sign-in shows the handle; an authenticated `getSession`/`describeRepo`
   succeeds.
**Validation:** Broad (external auth). Unit-test the pure, sandbox-runnable pieces (Pass 3): OAuth
**callback-param parsing** (extract/validate `code`/`state`/error params from the redirect), session
**restore/expiry** branching, and `whoami`/handle extraction — including the error edges (missing
`code`, mismatched `state`, expired session). The OAuth flow itself is the live/playwright wiring test.
Manually verify the live auth round-trip against a real account, using a **bsky app password for a
disposable test account supplied via env, never a committed secret** — and rotate the previously-shared
password first (see "Observability, error surfacing & live-testing credentials" above).

### Phase 3: Public card — create → link → view

**Goal:** A signed-in creator makes a public card; the app writes a public record to their PDS and
yields a share link; opening the link (no login) renders the card.
**Changes:**
- [ ] `greetings_site/src/pds.ts` — **DID→PDS resolver** (`resolveDidToPds(did)` via `plc.directory`),
  used by the view path: a custom-NSID record/blob is not served by the public appview/CDN, so reads
  must resolve the DID and hit the owning PDS directly.
- [ ] `greetings_site/src/atproto.ts` — `getRecordPublic(pds, did, rkey)` (unauthenticated),
  `uploadBlob(agent, bytes)` (authenticated cover upload → returns the blob-ref object
  `{$type:"blob", ref:{$link:cid}, mimeType, size}` embedded verbatim in the record),
  `getBlobUrl(pds, did, cid)` (an unauthenticated `com.atproto.sync.getBlob` URL usable as `<img src>`
  for a public cover), and `createCard` (writes `ing.croft.greeting.card`, `mode:"public"`, with `from`
  = creator handle, `to` = free-text recipient name (the recipient need NOT have an account), `theme`,
  and the cover blob-ref).
- [ ] `greetings_site/src/views/{create.ts,card.ts}` — create form (public: text + theme + recipient
  name + optional cover picker) → `uploadBlob` → `createCard` → show share link; card view resolves the
  DID, reads the record, and renders greeting + `<img>` cover (via `getBlobUrl`) + author/recipient.
**Call chain:** `#/create` (signed in) → fill form (+ pick cover) → `uploadBlob(cover)` →
`createCard(public, coverRef)` → agent write → `at://did/collection/rkey` → share link
`#/c/<did>/<rkey>` → recipient opens → `resolveDidToPds(did)` → `getRecordPublic` + `<img
src=getBlobUrl>` → render.
**Wiring test:** create a public card with a cover, open its share link in a fresh session (no login),
see the rendered greeting, cover image, author, and recipient name. Playwright e2e (CI/browser env);
manual/live here. RED before Phase 3, GREEN after.
**Depends on:** Phase 2; Phase 0 D2 (DID resolution + getBlob CORS) + D4 (blob round-trip + schema).
**Read-set:** `greetings_site/src/{router.ts,auth.ts}`.
**Write-set:** `greetings_site/src/{pds.ts,atproto.ts,views/create.ts,views/card.ts}`.
**Shared-state contract:** writes a real record + blob to the creator's PDS (external); test records use
a disposable test account and are deletable. No git/port state.
**Risks:** **getBlob CORS** from the static origin (D2 gate — if blocked, the cover view breaks with no
server to proxy); **blob orphan** if `createRecord` fails after `uploadBlob` (upload-then-reference
ordering; orphaned blobs are GC'd, acceptable); DID-resolution failure/latency; rkey/link encoding
edge cases; record-size (payload is small text, inline is fine).
**Done when:**
1. **Behavioral:** a public card round-trips create → link → view live, no login to view.
2. **Verification:** on the live site, create a public card and open its link in a private window.
**Validation:** Broad. Unit-test the record builder + link encoder/decoder (pure functions), naming the
edges (Pass 3, mutation-resistant): link encode→decode **round-trips** for a valid `did:plc`+rkey;
decode **rejects** a malformed/truncated locator and a missing rkey (defined error, not an exception);
the record builder emits the exact public shape (`mode:"public"`, `$type`, `from`/`to` present when
given, cover blob-ref embedded verbatim when a cover exists, omitted when not). Manually verify the live
create→view round-trip; confirm the created record on the PDS matches the schema.

### Phase 4: Private (server-blind link-key) card — create → link(#k) → view+decrypt

**Goal:** A creator makes a private card; content is encrypted client-side, stored as ciphertext, and
the key rides in the link fragment; opening the link decrypts and renders; the PDS holds only
ciphertext.
**Changes:**
- [ ] `greetings_site/src/crypto.ts` — the promoted D1 helper: `genKey()`, `seal(payload,key)->{iv,ct}`,
  `open({iv,ct},key)`, `sealBytes(bytes,key)->{iv,ct}` / `openBytes({iv,ct},key)` (cover image),
  base64url key encode/decode (WebCrypto AES-256-GCM, fresh IV per op).
- [ ] `greetings_site/src/atproto.ts` — `createCard(sealed)` stores `{mode:"sealed", iv, ciphertext,
  cover?, coverIv?}` (no plaintext fields); the `cover` blob holds AES-GCM **ciphertext** of the image
  bytes (uploaded via the same `uploadBlob`).
- [ ] `greetings_site/src/views/{create.ts,card.ts}` — create (private): `genKey` → `seal` payload +
  `sealBytes` cover → `uploadBlob(coverCiphertext)` → write sealed record → link
  `#/c/<did>/<rkey>#k=<base64url>`; card view: resolve DID → read record → key from `location.hash` →
  `open` payload, then **fetch the cover blob as bytes** (not `<img src>`, since it is ciphertext) →
  `openBytes` → `URL.createObjectURL(blob)` → render `<img>`. If no/invalid key, show "you need the
  link's key to read this" (no content, no cover).
**Call chain:** `#/create` (private) → `genKey` + `seal(payload,key)` + `sealBytes(cover,key)` →
`uploadBlob(coverCiphertext)` → `createCard(sealed)` → link with `#k=` → recipient opens →
`resolveDidToPds` → `getRecordPublic` (ciphertext) + fetch cover bytes via `getBlob` + key from
`location.hash` → `open` + `openBytes` → `blob:` URL → render.
**Wiring test:** create a private card with a cover; assert (a) the stored record has `{iv,ciphertext}`
and NO plaintext greeting/sender, and the cover blob is ciphertext (not a valid image); (b) the full
link (with `#k=`) renders the decrypted greeting and cover; (c) the keyless locator renders neither
text nor cover. Playwright e2e (CI/browser env); manual/live here. RED before Phase 4, GREEN after.
**Depends on:** Phase 3; Phase 0 D1.
**Read-set:** `greetings_site/src/{router.ts,auth.ts,pds.ts}`.
**Write-set:** `greetings_site/src/{crypto.ts,atproto.ts,views/create.ts,views/card.ts}`.
**Shared-state contract:** writes a real ciphertext record + ciphertext cover blob to the creator's PDS
(external, disposable test data). Key never leaves the browser, never sent to any server (fragment
only). No git/port state.
**Risks:** fragment handling — the key must never enter a query param, a fetch, a log, the SW, or any
analytics (audit `src/sw.ts` + confirm no analytics; fragments are not sent to SW fetch events, verify);
base64url edge cases; the sealed cover must NOT be rendered via `<img src=getBlobUrl>` (that would try
to render ciphertext) — it must go bytes→decrypt→`blob:` URL; getBlob CORS (D2 gate).
**Done when:**
1. **Behavioral:** a private card round-trips create → link(#k) → decrypt-and-view; the PDS record is
   ciphertext only; the key never touches the network.
2. **Verification:** on the live site, create a private card, confirm via `getRecord` the stored record
   has no plaintext, open the link and see the greeting, and open the keyless link and confirm it
   cannot read.
**Validation:** Broad. Unit-test `crypto.js` (seal/open round-trip, wrong-key fails, tamper fails) and
the link encoder (key in fragment only); manually verify the live round-trip and inspect the stored
record for plaintext absence; grep the SW/analytics for any handling that could exfiltrate the
fragment.

## Open Questions

- [CONFIRMED: PHASE-GATED (Phase 2)] Creator auth: **DECIDED — atproto OAuth browser client
  (`@atproto/oauth-client-browser`) reusing arecipe's pattern**, with a hosted
  `greetings.croft.ing/oauth-client-metadata.json`. *Resolved 2026-07-21 by inspecting arecipe.app,
  which does exactly this live on Pages (DPoP + PAR + PKCE + client-metadata), so OAuth-on-Pages is
  proven, not speculative, and app-password is off the table for a public product. Severity stays
  PHASE-GATED: Phase 2 wires it; nothing before Phase 2 depends on it. D2 narrows to confirming
  greetings' own client-metadata hosting + redirect, not choosing the mechanism.*
- [CONFIRMED: PHASE-GATED (Phase 4)] Browser cipher: **DECIDED — AES-256-GCM** (native WebCrypto, no
  dependency, hardware-accelerated, vendor-audited), with a **fresh random 256-bit key + fresh random
  96-bit IV per card**. *Resolved 2026-07-21. Both AES-GCM and ChaCha20-Poly1305 are equivalently
  strong AEADs; ChaCha's only edge here is byte-interop with the Rust `card-seal` crate, which is not a
  requirement for a browser-sealed/browser-opened card. Dropping interop, AES-GCM wins on bundle,
  supply chain, speed, and audit. The only condition that flips this: a future non-browser client (e.g.
  the Rust CLI) needing to open a browser-sealed card, at which point revisit with a shared cipher.
  D1 confirms the round-trip + raw-key export/import for the fragment; IV-uniqueness is the one
  discipline (structurally satisfied by fresh key+IV per card).*
- [CONFIRMED: PHASE-GATED (Phase 3)] Card content model + record schema: **DECIDED — text greeting +
  `theme` + one cover image** (just the cover; no galleries/inline images), NSID
  **`ing.croft.greeting.card`**. Public: `{ $type, mode:"public", text, theme?, from?, to?,
  cover?:<blobref>, createdAt }`. Sealed: `{ $type, mode:"sealed", iv, ciphertext, cover?:<blobref>,
  coverIv?, createdAt }` where `ciphertext` = AES-GCM of the whole payload `{text,from,to,theme}` and
  the `cover` blob holds **AES-GCM ciphertext of the image bytes** (same card key, distinct IV
  `coverIv`), so the sealed tier leaks neither text nor sender/recipient nor image. *Resolved
  2026-07-21 (user: cover image from the start, capped at the cover). Adds a blob path to the MVP;
  D4 confirms the blob round-trip and size limits.*
- [CONFIRMED: ADVISORY] Scheduled reveal: **DEFERRED** (user, 2026-07-21). MVP is share-on-demand: the
  creator shares the link when ready; the recipient cannot open what they have not been given, so a
  time-gated blind-until-open reveal is unnecessary for 1:1. The scheduled reveal (offer-gating on a
  trusted clock, CAP-3) needs the deferred shim and a server component, out of the MVP static model; it
  rides in with the anon-multi-write work later.
- [CONFIRMED: ADVISORY] Location: **plan stays in `discovery/alpha/plans/`; `greetings_site` README
  links back to it** (user, 2026-07-21). The *why* lives with the reasoning corpus next to the model it
  builds on; the product repo stays lean with a pointer. Pointer is captured in Documentation Impact
  (greetings_site README, Phase 1).
- [CONFIRMED: PHASE-GATED (Phase 1a)] **(NEW, Pass 2)** Adopt arecipe's full toolchain — esbuild +
  TypeScript + vitest (unit) + playwright (e2e/live) + eslint + a committed `client-metadata.json` +
  GitHub Actions CI — as the greetings base and the codified PWA/SPA standard. **DECIDED — adopt
  wholesale** (user, Pass 3 walk-through 2026-07-21: "1, obv unless we have a reason we need not to").
  *Rationale: surfaced in Pass 2 — the Q1 OAuth decision uses `@atproto/oauth-client-browser`, an npm
  dependency, so a build step is mandatory (the phases cannot be hand-authored static JS). arecipe
  already runs this exact stack live, so reuse is the low-risk path and doubles as the best-practices
  baseline. Severity stays PHASE-GATED (Phase 1a): nothing before 1a depends on it. Escape hatch: if a
  concrete reason to diverge surfaces during Phase 0/1a (e.g., a stack incompatibility), revisit then.*
- [CONFIRMED: ADVISORY (Phase 1)] PWA scope: **installable PWA with a minimal app-shell service worker;
  offline card data DEFERRED** (user, 2026-07-21). Ship manifest + a small SW caching the app shell
  (HTML/JS/CSS/fonts) for installability and fast repeat loads (matching arecipe); do not cache card
  data (cards are inherently online, reading the PDS/blob). **Hard rule carried to Phase 4:** the SW
  must never cache or log a URL carrying the `#k=` fragment (fragments are not sent to SW fetch events,
  but audit it); the key stays client-only.

## Review Log

- **Pass 1 (2026-07-21):** Initial plan. Grounded against the live `greetings_site` repo (Pages built,
  CNAME greetings.croft.ing) and the card-ingest proofs. Hosting model (static SPA on Pages) confirmed
  firsthand, so it is a Verified Assumption rather than a Phase 0 unknown. Four Phase 0 discovery items
  (D1 crypto, D2 CORS+auth, D3 routing/stack, D4 schema) cover the remaining unknowns. Deferred scope
  (anon multi-write, guestbooks, registries, scheduled reveal) recorded but not built.
- **Pass 1 walk-through Q1 (2026-07-21):** Inspected arecipe.app (our live sibling PWA) to set the
  creator-auth POV. Found it uses atproto OAuth browser client (`@atproto/oauth-client-browser`: DPoP,
  PAR, PKCE, hosted client-metadata.json) with strict CSP + SRI + hashed bundles + PWA manifest. **Q1
  resolved: OAuth, reusing arecipe's pattern** (severity confirmed PHASE-GATED (Phase 2)). Updated
  Verified Assumptions (arecipe as reference impl + OAuth-on-Pages existence proof), narrowed D2 to
  confirming greetings' own client-metadata hosting, and added a new PWA/SPA best-practices doc to
  Documentation Impact (seeded from arecipe) per the user's intent to start standardizing PWA/SPA work.
- **Pass 1 walk-through Q2 (2026-07-21):** Cipher **DECIDED — AES-256-GCM** (native WebCrypto, fresh
  key+IV per card). ChaCha's only edge (byte-interop with Rust `card-seal`) is not an MVP requirement.
  D1 narrowed to confirm-the-round-trip. Severity confirmed PHASE-GATED (Phase 4).
- **Pass 1 walk-through Q3 (2026-07-21):** Schema **DECIDED — text + theme + one cover image**, NSID
  `ing.croft.greeting.card`. User chose cover image from the start (capped at the cover). Sealed tier
  encrypts the whole payload AND the cover image bytes (same key, distinct IVs). Threaded the blob path
  into D4, Phase 3 (plain cover), and Phase 4 (`sealBytes`/`openBytes` ciphertext cover). Severity
  confirmed PHASE-GATED (Phase 3).
- **Pass 1 walk-through Q4–Q6 (2026-07-21):** Q4 scheduled reveal **DEFERRED** (share-on-demand for
  1:1; ADVISORY). Q5 plan stays in `discovery/alpha/plans/` with a `greetings_site` README pointer
  (ADVISORY). Q6 **installable PWA + app-shell SW, offline card data deferred**; SW must never handle
  the `#k=` fragment (ADVISORY, Phase 1). All 6 open questions confirmed: 0 BLOCKING, 3 PHASE-GATED
  (Q1→Phase 2, Q3→Phase 3, Q2→Phase 4), 3 ADVISORY (Q4/Q5/Q6). Pass 1 complete.

### Pass 2: Gap Analysis — 2026-07-21
**Found:**
- **Build toolchain was missing (biggest gap).** Q1's OAuth choice uses `@atproto/oauth-client-browser`,
  an npm package, so a build step is mandatory; the Pass-1 phases assumed hand-authored static JS.
  Verified arecipe's `package.json` + repo root: esbuild + TS + vitest + playwright + eslint, committed
  `client-metadata.json`, `.github/workflows`. Resolved by reusing arecipe's stack wholesale.
- **View path under-specified.** A custom-NSID (`ing.croft.greeting.card`) record/blob is not served by
  the public appview/CDN, so reads must **resolve the DID→PDS (via `plc.directory`)** and hit the owning
  PDS directly. Added `src/pds.ts` resolver and threaded it into Phase 3/4 view paths.
- **Cover render differs by mode.** A public cover can be `<img src=getBlobUrl>`; a sealed cover is
  ciphertext and must be fetched as bytes → decrypted → `blob:` URL. Made explicit in Phase 3 vs 4.
- **OAuth preconditions.** A committed `client-metadata.json` + an OAuth callback route are required
  before sign-in works; added to Phase 2, with CSP `connect-src` authored in Phase 1b.
- **Blob-ref shape + orphan ordering.** A record must embed the exact `uploadBlob` response object;
  upload-then-reference can orphan a blob on a failed `createRecord` (GC'd, acceptable). Noted Phase 3/D4.
- **from/to semantics.** `from` = creator handle; `to` = free-text recipient name — the recipient need
  not have an atproto account. Noted in Phase 3.
- **Test infra + validation calibration.** Reuse arecipe's vitest (unit: router, crypto, link codec,
  record builder) + playwright (e2e/live, run in CI / a browser-capable env — NOT this sandbox,
  headless-browser egress is blocked). Calibrated per phase.
**Concurrency:**
- Phase 1 split into 1a (build/deploy foundation) → 1b (shell + PWA + docs) to honor the 4-file rule;
  spine updated to `0 → 1a → 1b → 2 → 3 → 4`. Still fully sequential (shared `src/*` write-sets). Phase
  1b is a two-repo phase (greetings_site + discovery docs), two disjoint-across-repos commits. D1–D4
  remain parallel-optional. No new parallelism justified.
**Changed:**
- Added the arecipe full-stack Verified Assumption; expanded D2 (DID resolution + getBlob CORS gate),
  D3 (build/deploy model + reuse boundary; stack decided), D4 (blob-ref shape + render distinction).
  Split Phase 1 → 1a/1b; added `client-metadata.json` + callback to Phase 2; added `src/pds.ts` + cover
  render paths + from/to to Phase 3/4; aligned all phases to the TS `src/` layout. Added Q7 (adopt
  arecipe's toolchain). Added Risks (getBlob CORS gate, blob orphan, fragment-never-networked).
**Confirmed:**
- The link-key model and its proofs hold; static-SPA-on-Pages hosting holds (arecipe is the live proof
  for the whole stack). MVP scope (1:1, two modes, deferred anon/guestbooks/registries/reveal) unchanged.
  Q1–Q6 decisions all survive the gap analysis unchanged.

### Pass 3: Quality Gates — 2026-07-21
**Spot-check (Step 2):** Confirmed firsthand: `ROADMAP_TODO.md` E43 at line 166; ECOSYSTEM live-
properties register is **§5c-3** (line 186), arecipe/skylite at 192/195, greetings absent; the design
note and card-ingest `README.md`/`CAPABILITIES.md` exist; `pwa-spa-best-practices.md` does not yet exist
(correct — new in 1b). arecipe and greetings_site are **not checked out locally**, so the arecipe diff
runs via `gh`/live fetch or at execution — matches the plan's approach.
**TDD ordering:**
- Added a cross-cutting **"Test execution split"** note: vitest units are the in-sandbox RED→GREEN
  floor; playwright e2e/live wiring tests are authored RED and driven GREEN in CI / a browser-capable
  env (headless-browser egress is blocked here). A phase is not done on a green vitest suite alone.
- Clarified **Phase 1a** is a mechanical config bootstrap with no logic to unit-test first; its gate is
  the deploy-loop wiring test, and the first genuine vitest RED→GREEN is the 1b router (guards against
  hollow bootstrap unit tests).
- Added **mutation-resistant edge specifications** to the Phase 1b router test (unknown route, malformed
  locator, empty hash) and the Phase 3 link codec + record builder (round-trip, reject malformed/missing
  rkey, exact public shape). Added Phase 2 pure-unit specifics (callback-param parsing, session
  restore/expiry, `whoami`, with error edges).
**Observability:**
- Added a cross-cutting **"Observability, error surfacing & live-testing credentials"** subsection: user-
  facing + console diagnostics for auth failures (Phase 2), DID-resolution/getRecord/**getBlob-CORS**
  failures (Phase 3/4), and decrypt failures (Phase 4, distinct missing-key vs wrong-key/tamper
  messages), with the **hard rule** that the `#k=` fragment, key, and plaintext never enter a log, error
  report, analytics, the SW, or any network request. Fail-loud everywhere (no blank cards).
**Debugging readiness:**
- The per-leg console diagnostics make a post-deploy failure traceable to a phase/leg. The D2 getBlob
  CORS gate remains the key early-warning checkpoint de-risking Phases 3/4 (already in Risks).
**Validation calibration:**
- Per-phase validation strategies confirmed calibrated to scope (1a/1b Moderate, 2/3/4 Broad — external
  auth/PDS/crypto). No "tests are sufficient" on any external-integration phase.
- **Live-testing credentials:** codified the bsky app-password-via-env rule and flagged the previously-
  shared password for **rotation** before any live leg.
- **Disposition fix:** D1 relabeled `keep-as-fixture` → **`promote`** (executable crypto logic, promoted
  to production `src/crypto.ts` under TDD in Phase 4 — the named follow-up phase; also fixed "Phase 3/4"
  → "Phase 4"). D2/D3 stay `keep-as-fixture` (config/data) with "promoted" softened to "copied" and
  Phase references updated to the 1a/1b split.
- **Phase 0 not resolvable now:** D1 (browser WebCrypto + fragment) needs a browser; D2/D3/D4 need live
  network to bsky/plc.directory/Pages — all egress-blocked in this sandbox. Phase 0 stays execution work
  in a browser/network-capable env.
**Concurrency honesty:**
- Corrected the Phase 0 parallel-set **shared-state contract**, which was inaccurately blanket "read-only
  / no shared writes": **D1 is local-only**; **D2/D4 read-only** (D4 also does one authenticated
  `uploadBlob` to a disposable test account); **D3 writes to shared external state** (a stub deploy to
  greetings_site/Pages). Write targets are still disjoint, so the optional parallel set is safe, but D3's
  write is now explicit, sequenced before Phase 1a, with a re-entry check that its stub is removed or
  carried forward as the 1a base. No new parallelism warranted — implementation phases share `src/*`
  write-sets and stay sequential.
**Coherence:**
- Plan still solves the stated problem (1:1 card, two modes, PWA on Pages); no scope creep (cover image
  and PWA were user-approved via Q3/Q6). All open questions tagged; Q1–Q6 user-confirmed in Pass 1, Q7
  user-confirmed in this Pass 3 walk-through.
**Documentation impact:**
- Every Documentation Impact entry has a Phase 1b item; corrected **§5c → §5c-3** and stale **"Phase 1"
  → "Phase 1b"** references (Phase 1 was split in Pass 2). All doc updates are same-phase (Phase 1b), no
  end-loaded docs phase. New `pwa-spa-best-practices.md` confirmed to have no existing references.
- **Conventions (ADVISORY, not changed):** the plan filename `2026-07-21-greetings-croft-ing-mvp.md`
  follows the *dominant* local `plans/` convention (`YYYY-MM-DD-<slug>.md`); only the newest sibling uses
  the skill's `-N-plan-` form. Not renamed — a rename would break the Q5 greetings_site README pointer
  and cascading references. Optional to align later.
**Confirmed ready:** yes — 0 BLOCKING; 4 PHASE-GATED (Q7→1a, Q1→2, Q3→3, Q2→4); 3 ADVISORY (Q4/Q5/Q6).

### Phase 0 execution — 2026-07-21 (Discovery Exemption; non-browser legs)
**Sandbox-capability correction (mid-execution discovery):** my Pass 3 note claimed the sandbox is
egress-blocked for D2/D3/D4. That was too broad — the block is **browser/Chromium-only**. Plain HTTPS
via Node `fetch`/`curl` works here (`plc.directory` 302, `public.api.bsky.app` 200), and Node exposes
WebCrypto (`crypto.subtle`). This let Phase 0 verify far more than expected without a browser.
**Ran (firsthand, recorded in Verified Assumptions → "Phase 0 execution findings"):**
- **D1 ✅ RESOLVED** — AES-256-GCM round-trip in WebCrypto, 9/9 assertions pass; fragment grammar
  `#/c/<did>/<rkey>#k=<base64url>` decided; raw key = 43-char base64url; wrong-key + tamper fail;
  fresh IV per op; cover-byte path round-trips. Spike `scratchpad/d1-crypto.mjs` (disposition `promote`
  → `src/crypto.ts` in Phase 4 under TDD).
- **D2 ✅ reads/CORS RESOLVED** — DID→PDS resolution + `getRecord`/`getBlob` all 200 with `ACAO: *`
  (preflights 204). **The getBlob-CORS gate — the plan's biggest risk — is cleared** for bsky-hosted
  accounts. New finding: OAuth `redirect_uri` cannot be a hash route → concrete callback path needed
  (threaded into Phase 2). Spike `scratchpad/d2d4-reads.mjs` (`keep-as-fixture`).
- **D3 ✅ RESOLVED** — read arecipe's `package.json`/`ci.yml`/`client-metadata.json`/tree. Build/deploy
  model = esbuild→`dist/`→Actions push to **`gh-pages` root** (`pages-deploy.sh`), NOT `main`/root
  (**corrected the Pass-1 VA**); hermetic CI, `@live` local per phase; reuse boundary decided
  (toolchain/configs/CI/deploy/client-metadata/CSP/`authModeFor` copied, greetings = single-page
  hash-routed shell). `keep-as-fixture` (arecipe files in `scratchpad/`).
- **D4 🟡 read leg RESOLVED, write leg gated** — `getBlob` read + CORS confirmed; blob-ref shape known.
  `uploadBlob` write leg + size cap + allowed types + sealed-bytes round-trip **not run** (needs a
  rotated bsky app password) → Phase 3/4 live gate. `throwaway`.
**Plan changes made (material — trigger a user checkpoint before Phase 1a per execute.md):**
1. Deploy model corrected to Actions→`gh-pages` root + one-time Pages-source flip (VA + Phase 1a).
2. OAuth callback must be a concrete path, not a hash route (VA + Phase 2 changes + Risks).
3. VA "Unverified" split into resolved vs still-gated (browser/credential) legs.
**Not run here (honest gaps, deferred to their implementation phase's live gate):** live hash routing on
Pages (Phase 1a first deploy), greetings' own OAuth redirect round-trip (Phase 2), authenticated
`uploadBlob` (Phase 3/4). **Credential note:** the live legs need a rotated bsky app password via env;
the previously-shared password must be treated as compromised.
**Checkpoint:** Phase 0's discovery unknowns are resolved; the two material changes are recorded. Await
user go-ahead before Phase 1a — and Phase 1a onward requires the `greetings_site` repo checked out
locally plus push access (it is not currently local).

### Phase 1a execution — 2026-07-21 (local-green; `greetings_site` `33c0e89`)
**Setup:** user authorized proceeding + push (chasemp); cloned `CroftCommunity/greetings_site` into
`CroftC/greetings_site` (`github-personal` remote, `chase@owasp.org` identity), branched `greetings-mvp`
off `main` (repo started with only `CNAME` + `LICENSE`). App-password decision: reuse the existing
test-only password via env for the later live legs, rotate afterward (user, not a blocker).
**Built (from arecipe's confirmed files):** `package.json` (+ lockfile for `npm ci`), `tsconfig.json`,
`tsconfig.tests.json`, `vitest.config.ts`, `playwright.config.ts`, `eslint.config.js`, `.gitignore`,
`scripts/build.mjs` (single-page core), `scripts/pages-deploy.sh` (verbatim), `.github/workflows/ci.yml`
(hermetic test + gh-pages deploy) + `preview.yml` (arecipe.app→greetings.croft.ing), `src/main.ts` +
`index.html` (hello-world shell).
**Verified locally (the runnable floor):** `npm install` clean (180 pkgs, 0 vuln); `npm run build` →
`dist/{main-<hash>.js, index.html (ref rewritten), CNAME, .nojekyll, build-info.json}`; `npm run lint`,
`npm run typecheck`, `npm run test:unit` (passWithNoTests) all exit 0; aggregate `npm test` exits 0.
**Deviations (see Phase 1a header):** `build.mjs` minimal (CSP/SRI/SW/manifest → 1b); `npm test` omits
e2e until 1b; wiring e2e into CI in 1b needs `package.json` + `ci.yml` edits (outside 1b's declared
write-set — flagged).
**Not done — the behavioral wiring test (deploy loop):** push `main` → Actions → `gh-pages` + one-time
Pages-source flip to `gh-pages`/root. This is the outward leg; paused for user go-ahead before pushing.
**Discovery repo:** this plan doc's Pass 3 + Phase 0 + Phase 1a updates committed separately (user
approved).
**Go-live (2026-07-21, user chose "go live now"):** merged `greetings-mvp`→`main` (ff), pushed; CI
`test`+`deploy` green; `gh-pages` created (`253e43f`) with the built `dist/`; flipped Pages source to
`gh-pages`/root; live site verified serving a byte-identical hashed bundle. **Deploy loop wiring test:
GREEN. Phase 1a SHIPPED.** **Operational learning:** flipping the Pages source via `PUT /pages` does
NOT auto-trigger a rebuild — the site kept serving the old `main`/root build (raw source: unbuilt
`./main.js`, 404 bundle) until a one-time `POST /pages/builds` forced a build from `gh-pages`.
Subsequent deploys push directly to `gh-pages`, which auto-triggers a build, so this manual step was a
one-time source-flip artifact, not part of the steady-state loop. Benign CI annotation: the pinned
`actions/checkout`/`setup-node` SHAs target Node 20 (deprecation warning, not a failure) — bump later.

### Phase 1b execution — 2026-07-21 (SHIPPED; greetings `ca67803`, discovery `8fdafb0`)
**Method:** router built TDD (test RED → `parseHash` GREEN, 6/6) in-sandbox; views/main/sw/build/CSP/SRI
authored; local gate (lint + typecheck + unit + build) green. Because the sandbox can't run a browser,
pushed the **`phase-1b` branch first** so CI ran the playwright e2e (the wiring test) with no risk of
gating a production deploy (deploy is main-only). Branch CI green (unit 6/6, **e2e 6/6**), so merged
`phase-1b`→`main` (ff), pushed; main CI test+deploy green; `gh-pages` auto-rebuilt (`e19163c`); live
site verified serving the shell with CSP + hashed bundle + manifest + sw.js. This branch-first pattern
is the right steady-state for browser-tested phases in this sandbox — adopt it for Phases 2–4.
**Deviations (also in the Phase 1b header):** (1) closed the Phase-1a-flagged CI gap by editing
`package.json` (`test` now runs e2e) + `ci.yml` (playwright install) — the gap the plan's 1b write-set
missed. (2) PWA icon is a single SVG (`sizes: "any"`), installable in modern Chrome; branded PNG sizes
are a later polish, not a stub. (3) SW registration failure is logged, not thrown (progressive
enhancement) — consistent with fail-loud for core paths, tolerant for enhancements.
**Confirmed live:** routed PWA shell at greetings.croft.ing (home / #/create / #/c/… / notfound),
strict CSP + SRI, installable. **Next: Phase 2 (creator OAuth) — first credential-gated live leg
(uses the test app password via env, per the user).**

### Phase 2 execution — 2026-07-21 (SHIPPED; greetings `5734581`)
**Ground truth first (no-assumed-API discipline):** read arecipe's `src/auth/{oauth-client,boot,
session-provider}.ts` + inspected the installed `@atproto/oauth-client-browser@0.4.9` types to confirm
`BrowserOAuthClient({clientMetadata, handleResolver})`, `init()→{session}|undefined`, `signIn(handle)`,
`session→new Agent(session)`; verified `agent.com.atproto.repo.describeRepo` exists before using it.
**Built** exactly to that pattern; split pure logic into `auth-core.ts` so it TDDs headless (7/7 RED→
GREEN). **Branch-first CI** (the recorded steady-state): pushed `phase-2`, CI green (unit 13/13, e2e
9/9), merged to `main` (ff), deployed; `gh-pages` auto-rebuilt (`d0dd3a6`); verified `client-metadata.json`
live with `client_id`===its-own-URL and the new bundle serving.
**User instruction mid-phase:** "keep a croftish design palette, look in crofting_site." Fetched
`crofting_site/styles.css`, adopted its tokens (schist/granite/ruddy/moss/ink/canvas + Lora/Inter) as
greetings' palette; recorded it as the shared Croft visual system. This is a design decision layered on
Phase 2, not scope creep — it restyles the existing shell + the new sign-in view.
**Verification honesty:** the interactive OAuth round-trip (redirect + bsky consent) cannot run in this
sandbox (no browser) and is not cleanly automatable headless (consent step). Deployed the working wiring
+ live hosted client-metadata; the round-trip is **user-confirmed in a browser** (click-path handed
over). The app-password `@live` port test (arecipe's approach — inject an app-password Agent through the
session port to exercise the authenticated-agent capability without the consent screen) is the automatable
proxy; deferred until the test password is supplied via env. **Next: Phase 3 (public card).**
