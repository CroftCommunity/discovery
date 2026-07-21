# greetings.croft.ing MVP — 1:1 link-delivered cards (phase plan, Pass 1)

Build target repo: `CroftCommunity/greetings_site` (live on GitHub Pages at
https://greetings.croft.ing/). This plan doc lives in the discovery corpus (where the E43 reasoning
and the card-ingest proofs live); execution code lands in `greetings_site`.

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
  `CNAME`, `LICENSE`). Deploy loop = push static assets to `main` root, Pages rebuilds.
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

**Unverified (Phase 0 targets):** CORS behavior of the bsky PDS/entryway for cross-origin **reads**
(`getRecord`, `getBlob`) from the `greetings.croft.ing` static origin; **DID→PDS resolution** in the
browser (resolve `did:plc:*` via `plc.directory` to the PDS endpoint before `getRecord`/`getBlob` — a
custom-NSID record is NOT served by the public appview/CDN, so the view path must resolve the DID and
hit the owning PDS directly); the exact **blob-ref object shape** a record must embed (the `uploadBlob`
response `{$type:"blob", ref:{$link:cid}, mimeType, size}`); GitHub Pages routing for a client-side-
routed SPA (hash routing vs 404 fallback) and the **build/deploy model** (esbuild → committed built
assets vs an Actions build-to-Pages, per arecipe's CI). *(Creator-auth path is no longer unverified —
DECIDED as OAuth reusing arecipe.)*

## Documentation Impact

- `greetings_site/README.md` — created/expanded in Phase 1 (what the app is, the two card modes, the
  link grammar, local dev + deploy). Currently the repo has only `CNAME` + `LICENSE`.
- `discovery/alpha/experiments/card-ingest/README.md` — add a pointer (Phase 1) that greetings_site is
  the product consumer of the link-key tier.
- `discovery/alpha/thinking/app/ponds/virtual-cards-and-guestbooks.md` — add a "productization"
  pointer to greetings_site + this plan (Phase 1).
- `discovery/alpha/ECOSYSTEM.md` (§5c Croft-owned live properties) — register greetings.croft.ing as a
  live Croft property (Phase 1). *(Grep confirmed arecipe/skylite are registered there; greetings is
  not yet.)*
- `discovery/alpha/ROADMAP_TODO.md` E43 — add a "productization underway: greetings.croft.ing (plan
  <this file>)" note (Phase 1).
- `discovery/alpha/thinking/app/pwa-spa-best-practices.md` — **new** (Phase 1): our PWA/SPA baseline,
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
- Shared-state contract: read-only against external services (bsky PDS, the live Pages site); no
  git/branch/port mutation; no shared writes.
- Re-entry verification: no app file changed by any probe; findings recorded in the plan's Verified
  Assumptions; scratch dir removable.

## Phases

### Phase 0: Discovery

**Goal:** Resolve the four unknowns before committing to the SPA structure. Discovery Exemption applies
(no TDD on probe code; each task declares a disposition).

- [ ] **D1: Confirm the AES-256-GCM browser round-trip + key/link format.** (Cipher DECIDED:
    AES-256-GCM; see Open Questions. D1 confirms the mechanics, it does not choose the cipher.)
  - **Probe:** In a browser context, WebCrypto `AES-GCM` 256-bit: `genKey` (fresh random 256-bit key),
    fresh random 96-bit IV, encrypt+decrypt round-trip of a UTF-8 card payload; confirm ciphertext ≠
    plaintext, decrypt-with-wrong-key fails, tamper fails; confirm raw-key export/import
    (`exportKey('raw')` → base64url → `importKey('raw')`) for the fragment; settle the fragment grammar.
  - **Success criteria:** A working `seal(payload,key)->{iv,ct}` / `open({iv,ct},key)` + `genKey` in
    browser JS with AES-256-GCM, and a decided fragment grammar (expected: locator in the path/hash,
    key as `#k=<base64url>`; IV stored with the record, never the key).
  - **Disposition:** `keep-as-fixture` — the crypto helper becomes `crypto.js` in Phase 3/4 (TDD
    applies to the promoted module).
- [ ] **D2: Confirm CORS reads and greetings' own OAuth client-metadata hosting.** (Auth mechanism
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
  - **Disposition:** `keep-as-fixture` — the client-metadata.json and CSP are promoted into Phase 1/2.
- [ ] **D3: Confirm routing + build/deploy on Pages; settle the arecipe reuse boundary.** (Stack
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
  - **Disposition:** `keep-as-fixture` — the stub + copied config become the Phase 1 build/shell.
- [ ] **D4: Confirm the cover-image blob round-trip (schema DECIDED; see Open Questions).**
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

### Phase 1a: Build + deploy foundation (reuse arecipe's toolchain)

**Goal:** greetings_site has a working esbuild+TS build, a test harness, a CI/deploy path, and a
minimal built page live on Pages. This exists because OAuth (`@atproto/oauth-client-browser`) is an
npm dependency and cannot be hand-authored static JS (Pass 2 finding).
**Changes:**
- [ ] `greetings_site/package.json` — deps (`@atproto/oauth-client-browser`, `@atproto/api`,
  `@atproto/oauth-types`, `@ipld/dag-cbor`, `multiformats`) + devDeps (`esbuild`, `typescript`,
  `vitest`, `happy-dom`, `@playwright/test`, `eslint`); scripts mirrored from arecipe.
- [ ] build/config copied+adapted from arecipe: `scripts/build.mjs` (esbuild), `tsconfig.json`,
  `vitest.config.ts`, `playwright.config.ts`, `eslint.config.js`.
- [ ] `greetings_site/.github/workflows/ci.yml` — lint + typecheck + unit + build (+ deploy per the
  D3-confirmed model).
- [ ] `greetings_site/src/main.ts` + a minimal `index.html` that loads the built bundle (hello-world,
  proves the build→Pages loop).
**Call chain:** `npm run build` (esbuild) → hashed bundle emitted → committed/deployed to the
Pages-served path → https://greetings.croft.ing/ loads the built bundle.
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
browser egress is blocked here).

### Phase 1b: App shell + router + PWA + doc pointers

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
- [ ] discovery-side (separate commit): doc pointers (ECOSYSTEM §5c, ROADMAP_TODO E43, card-ingest
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
**Validation:** Moderate. Unit-test the router (pure `hash → view` function) in vitest; manual/live +
CI-playwright for the DOM flow; discovery-side site gate for the doc changes.

### Phase 2: Creator sign-in (write capability)

**Goal:** The creator signs in via atproto OAuth (browser + DPoP, `@atproto/oauth-client-browser`) and
the app holds a session able to write to their PDS.
**Changes:**
- [ ] `greetings_site/client-metadata.json` — committed at repo root (OAuth `client_id` target),
  modeled on arecipe's: `client_id` = its own hosted URL, `redirect_uris` = the greetings callback,
  scopes, DPoP. (OAuth precondition — the client cannot start without it.)
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
callback route must be reachable on Pages (hash-route or a real callback page).
**Done when:**
1. **Behavioral:** a creator signs in and the app can make an authenticated call as them.
2. **Verification:** live sign-in shows the handle; an authenticated `getSession`/`describeRepo`
   succeeds.
**Validation:** Broad (external auth). Unit-test token/session handling; manually verify the live auth
round-trip against a real account (use an app password / test account, never a committed secret).

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
**Validation:** Broad. Unit-test the record builder + link encoder/decoder (pure functions); manually
verify the live create→view round-trip; confirm the created record on the PDS matches the schema.

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
- [RECOMMENDED: PHASE-GATED (Phase 1a)] **(NEW, Pass 2)** Adopt arecipe's full toolchain — esbuild +
  TypeScript + vitest (unit) + playwright (e2e/live) + eslint + a committed `client-metadata.json` +
  GitHub Actions CI — as the greetings base and the codified PWA/SPA standard? *Rationale: surfaced in
  Pass 2 — the Q1 OAuth decision uses `@atproto/oauth-client-browser`, an npm dependency, so a build
  step is mandatory (the phases cannot be hand-authored static JS). arecipe already runs this exact
  stack live, so reuse is the low-risk path and doubles as the best-practices baseline. Recommend
  adopting wholesale; Phase 1a sets it up.*
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
