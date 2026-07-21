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

**Unverified (Phase 0 targets):** CORS behavior of the bsky PDS/entryway for cross-origin XRPC from
the `greetings.croft.ing` static origin (reads and writes); the exact creator-auth path in a static
browser client (OAuth vs app-password); GitHub Pages routing behavior for a client-side-routed SPA
(hash routing vs 404 fallback); the card content/record schema.

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

Sequential spine: Phase 0 (D1–D4) → Phase 1 → Phase 2 → Phase 3 → Phase 4.

All implementation phases are **sequential**: each builds on the prior (shell → auth → public card →
private card), sharing the same small set of app files (`index.html`, `app.js`, `atproto.js`,
`crypto.js`), so their write-sets overlap and they cannot be parallelized. Within Phase 0, the four
discovery probes (D1–D4) are independent and **may run in parallel** (each is a throwaway/keep probe
touching only its own scratch files), but they are cheap enough that sequential is fine; parallelism
here is optional, not required.

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
  - **Probe:** (1) unauthenticated `getRecord` of a public record via browser fetch from a static
    origin, confirm CORS allows it; (2) draft `greetings.croft.ing/oauth-client-metadata.json` modeled
    on arecipe's, confirm the OAuth redirect/callback works from the Pages origin (arecipe is the
    working reference — diff its client-metadata + CSP `connect-src`).
  - **Success criteria:** unauthenticated `getRecord` works cross-origin; a valid greetings
    client-metadata.json + confirmed redirect on Pages, matching arecipe's proven shape.
  - **Disposition:** `keep-as-fixture` — the client-metadata.json and CSP are promoted into Phase 1/2.
- [ ] **D3: SPA/PWA shell + routing on GitHub Pages, and reuse-vs-fork of the Croft.ing surface.**
  - **Probe:** Confirm client-side routing works on Pages for the link grammar (hash routing needs no
    server rewrites; a path-routed SPA needs a `404.html` fallback) by deploying a 2-route stub. Decide
    the stack (vanilla + WebCrypto, or a light framework) and how much of the Croft.ing single-renderer
    conventions (atproto-records-as-state, tectonic visual system) to reuse.
  - **Success criteria:** A decided routing scheme (expected: hash routing) proven on the live Pages
    site, and a decided stack + reuse boundary.
  - **Disposition:** `keep-as-fixture` — the stub becomes the Phase 1 shell.
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

### Phase 1: App shell + Pages deploy loop + PWA scaffold

**Goal:** A deployed SPA shell on greetings.croft.ing that loads, routes client-side, and is
installable, with a placeholder card-view route.
**Changes:**
- [ ] `greetings_site/index.html` — app entry, loads the SPA; hash-routing bootstrap.
- [ ] `greetings_site/app.js` — router + view shells (home / create / view-card placeholder).
- [ ] `greetings_site/manifest.webmanifest` + `greetings_site/sw.js` — PWA install + offline shell.
- [ ] `greetings_site/styles.css` — minimal tectonic-aligned styling.
- [ ] `greetings_site/README.md` — what it is, link grammar, dev + deploy.
- [ ] doc pointers (ECOSYSTEM, card-ingest README, design note, E43) per Documentation Impact.
**Call chain:** browser loads `index.html` → `app.js` router parses `location.hash` → renders the
matching view shell (home / `#/create` / `#/c/<locator>` placeholder).
**Wiring test:** loading `https://greetings.croft.ing/#/c/placeholder` renders the card-view shell (not
a 404), and `/#/create` renders the create shell. (Browser E2E: run in a browser-capable env or manual
on the live site; see Validation.)
**Depends on:** Phase 0 (D3 routing/stack decision).
**Read-set:** none (greenfield).
**Write-set:** `greetings_site/{index.html,app.js,manifest.webmanifest,sw.js,styles.css,README.md}`;
doc pointers in `discovery/alpha/{ECOSYSTEM.md,ROADMAP_TODO.md,experiments/card-ingest/README.md,thinking/app/ponds/virtual-cards-and-guestbooks.md}`.
**Shared-state contract:** deploys to Pages via push to `greetings_site` `main`; no other ambient
state. (Cross-repo: the discovery doc pointers are a separate commit in the discovery repo.)
**Risks:** GitHub Pages caching/propagation delay; service-worker cache staleness (version the SW).
**Done when:**
1. **Behavioral:** the live site loads the SPA, client-side routes resolve (no 404 on `#/c/...`), and
   the app is installable (manifest valid).
2. **Verification:** load the two routes on https://greetings.croft.ing/ and confirm the shells render;
   Lighthouse/PWA manifest check passes.
**Validation:** Moderate. Unit-test the router (pure function: hash → view name). Manually load the
live site (browser E2E cannot run hermetically in this sandbox — egress-blocked headless browser).

### Phase 2: Creator sign-in (write capability)

**Goal:** The creator can sign in with their atproto account in-browser and the app holds a session
able to write to their PDS.
**Changes:**
- [ ] `greetings_site/atproto.js` — auth module implementing the D2-decided path (OAuth browser flow
  with DPoP, or app-password `createSession`), session storage (memory/session-storage, not persisted
  insecurely), `whoami`, and a `writeRecord` helper.
- [ ] `greetings_site/app.js` — sign-in view + state ("signed in as @handle").
**Call chain:** `#/create` → "sign in" → `atproto.js` auth flow → session → app shows handle →
`writeRecord` available to later phases.
**Wiring test:** signing in on the live site shows "signed in as @handle" and a subsequent
no-op/whoami call succeeds with the session. (Manual/live; the auth flow hits real bsky.)
**Depends on:** Phase 1; Phase 0 D2.
**Read-set:** `greetings_site/app.js`.
**Write-set:** `greetings_site/atproto.js`, `greetings_site/app.js`.
**Shared-state contract:** creates a real atproto session against bsky (external); token in memory only;
no persistence of secrets to disk/localStorage in plaintext. No git/port state.
**Risks:** OAuth client-metadata hosting + redirect URI setup (if OAuth); app-password UX friction (if
app-password); CORS on the auth endpoint (resolved in D2).
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
- [ ] `greetings_site/atproto.js` — `getRecordPublic(locator)` (unauthenticated read), `uploadBlob`
  (authenticated cover upload) + `getBlob(did, cid)` (unauthenticated cover read), and `createCard`
  (writes an `ing.croft.greeting.card` record, `mode: "public"`, cover blobref per the D4 schema).
- [ ] `greetings_site/app.js` — create form (public mode: text + theme + optional cover image picker)
  → upload cover blob → build record → write → show share link; view route reads the public record,
  fetches the cover via `getBlob`, and renders greeting + cover + author.
**Call chain:** `#/create` (signed in) → fill form (+ pick cover) → `uploadBlob(cover)` →
`createCard(public, coverRef)` → `writeRecord` → `at://did/collection/rkey` → share link
`#/c/<did>/<rkey>` → recipient opens → `getRecordPublic` + `getBlob(cover)` → render.
**Wiring test:** create a public card with a cover image, open its share link in a fresh session (no
login), see the rendered greeting, cover image, and author handle. RED before Phase 3, GREEN after.
**Depends on:** Phase 2; Phase 0 D4.
**Read-set:** `greetings_site/{app.js,atproto.js}`.
**Write-set:** `greetings_site/{app.js,atproto.js}`.
**Shared-state contract:** writes a real record to the creator's PDS (external); test records use a
disposable test account/collection and are deletable. No git/port state.
**Risks:** record schema churn; CORS on getRecord from the static origin (resolved in D2); rkey/link
encoding edge cases.
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
- [ ] `greetings_site/crypto.js` — the promoted D1 helper: `seal(payload, key)->{iv,ct}`,
  `open({iv,ct}, key)`, `genKey()`, `sealBytes`/`openBytes` (for the cover image), base64url key
  encode/decode (WebCrypto AES-GCM per D1).
- [ ] `greetings_site/atproto.js` — `createCard(sealed)` stores `{mode:"sealed", iv, ciphertext,
  cover?, coverIv?}` (no plaintext fields); the cover blob holds AES-GCM ciphertext of the image bytes.
- [ ] `greetings_site/app.js` — create form (private mode): `genKey` → `seal` payload + `sealBytes`
  cover → `uploadBlob(ciphertext-cover)` → write sealed record → share link
  `#/c/<did>/<rkey>#k=<base64url>`; view route: read record → key from fragment → `open` payload +
  `getBlob` + `openBytes` cover → render; if no/invalid key, show "you need the link's key to read
  this" (no content leak).
**Call chain:** `#/create` (private) → `genKey` + `seal(payload,key)` + `sealBytes(cover,key)` →
`uploadBlob(coverCiphertext)` → `createCard(sealed)` → link with `#k=` → recipient opens →
`getRecordPublic` (ciphertext) + `getBlob` (cover ciphertext) + key from `location.hash` → `open` +
`openBytes` → render.
**Wiring test:** create a private card with a cover image; assert (a) the stored record contains
`{iv,ciphertext}` and NO plaintext greeting/sender, and the cover blob is ciphertext (no plaintext
image); (b) opening the full link (with `#k=`) renders the decrypted greeting and cover; (c) opening
the same locator WITHOUT the key cannot render text or cover. RED before Phase 4, GREEN after.
**Depends on:** Phase 3; Phase 0 D1.
**Read-set:** `greetings_site/{app.js,atproto.js}`.
**Write-set:** `greetings_site/{crypto.js,app.js,atproto.js}`.
**Shared-state contract:** writes a real ciphertext record to the creator's PDS (external, disposable
test data). Key never leaves the browser / never sent to any server (fragment only). No git/port state.
**Risks:** fragment handling (ensure the key is never put in a query param, never logged, never sent in
a fetch); base64url edge cases; SW/analytics accidentally capturing the URL with the fragment (audit
the SW and any analytics to strip/ignore the fragment).
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
