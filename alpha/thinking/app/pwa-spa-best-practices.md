# PWA/SPA best practices for Croft-owned static apps

status: living standard (alpha). Seeded 2026-07-21 from arecipe.app (the first
working crop) and validated a second time by greetings.croft.ing. This is the
baseline a new Croft static app copies rather than re-deriving.

scope: backendless single-page apps / PWAs served from GitHub Pages, reading and
writing AT Protocol PDSes directly from the browser. No custom server, no
AppView — the PDS is the only store, the app is a pure client. "Server-blindness
is structural: there is no server to leak to."

## Why this shape

A static renderer on Pages makes the trust story checkable by construction: reads
are unauthenticated public `getRecord`/`getBlob`; the only privileged action is
the creator writing to their own PDS under their own session. Nothing the
operator runs sees user content. Two Croft apps now run this exact stack, so it
is a proven baseline, not a proposal.

## The stack (copy this)

- **Build:** TypeScript (strict, `noUncheckedIndexedAccess`) + **esbuild**
  (`scripts/build.mjs`), emitting **content-hashed** bundles (`[name]-[hash].js`,
  `chunk-[hash]`). Stable-named HTML references the hashed assets, so a deploy
  changes URLs and stale JS is structurally impossible.
- **Tests:** **vitest** for the pure modules (routing, crypto, codecs, record
  builders) — the fast, headless, sandbox-runnable floor; **playwright** for
  browser wiring (e2e) and, gated by `LIVE=1`, real-PDS suites. Split them: the
  unit tier is RED→GREEN locally; the e2e/live tier runs in CI or a
  browser-capable env.
- **Lint:** eslint (`typescript-eslint`). **Typecheck** in CI (`tsc --noEmit`).
- **Auth (when the app writes):** atproto **OAuth in the browser** via
  `@atproto/oauth-client-browser` (DPoP + PAR + PKCE), with a **committed,
  hosted `client-metadata.json`** whose `client_id` is its own public URL. The
  OAuth `redirect_uri` must be a **concrete path** — it cannot contain a `#`
  fragment, so a hash-routed app needs a real callback page, not a `#/callback`
  route. `token_endpoint_auth_method: "none"`, `dpop_bound_access_tokens: true`,
  `scope: "atproto transition:generic"`.
- **`authModeFor` gate:** offer sign-in only on the production origin or
  loopback; every other origin (PR previews, forks) degrades to **read-only**
  instead of crashing or writing to real accounts.

## Security posture

- **Strict CSP via `<meta http-equiv>`** (Pages sets no response headers):
  `default-src 'none'`, then explicit allowances. Inline `<script>` blocks are
  admitted by their **exact sha256 hash**, computed by the build from the real
  content so the hash can never drift — no `'unsafe-inline'`/`'unsafe-eval'`.
  Inject the CSP `<meta>` immediately after `<meta charset>` so it precedes every
  inline script (a `<meta>` CSP does not govern scripts before it). `connect-src`
  is scoped to the auth server + `plc.directory` + `https:` (arbitrary PDSes);
  `img-src 'self' data: blob: https:` (the `blob:`/`data:` cover the decrypted
  media object URLs of a server-blind app).
- **Subresource Integrity (sha384)** on the entry module + stylesheet(s),
  `crossorigin="anonymous"`, computed from the exact served bytes.
- **No-flash theme:** a tiny pre-paint inline script resolves the theme from
  `localStorage` before first paint (admitted by its CSP hash).
- **Secrets never touch the network beyond the PDS session.** For a server-blind
  app the symmetric key rides only in the URL **fragment** (`#k=`), which is
  never sent to any server, log, analytics, or the service worker. The build and
  the SW must be auditable for this: fragments are not sent in fetch requests, so
  the SW is structurally incapable of seeing the key — keep it that way.

## PWA

- **Manifest** (`manifest.webmanifest`): `start_url`/`scope` = `"./"` (relative,
  so the app also runs from a `/pr-preview/pr-N/` subdirectory), `display:
  standalone`, theme/background colors, at least one icon.
- **App-shell service worker:** precache the shell (HTML/JS/CSS/manifest/icons)
  with the build version + precache list **baked in at build time**; version the
  cache name so a deploy busts it. **Cache the shell only** — card/user data
  (cross-origin PDS reads) passes straight to the network; the SW never stores
  content and never handles the `#k=` fragment. Offline *data* is a separate,
  deferrable concern.

## Deploy

- GitHub **Actions builds and pushes `dist/` to the `gh-pages` branch root** via
  plain git (`scripts/pages-deploy.sh`) — no third-party deploy action with write
  access. Pages source = "Deploy from a branch → `gh-pages` / root". Copy `CNAME`
  into `dist/` and write `.nojekyll` (a branch source runs Jekyll by default,
  which would drop a pre-built SPA).
- **Per-PR previews** at `gh-pages:/pr-preview/pr-N/` (a second workflow), so a
  production deploy and previews own disjoint paths.
- **CI is hermetic:** lint + typecheck + unit + build + e2e against the built
  bundle, no credentials, no live-PDS calls. The `@live` real-PDS tier runs
  **locally as each phase's gate**, with a bsky **app password supplied via env**
  (never committed).
- **Gotcha:** flipping the Pages *source* via the API does not auto-trigger a
  rebuild — a one-time `POST /pages/builds` is needed. Subsequent pushes to
  `gh-pages` do auto-trigger.

## Consumers

- **arecipe.app** (`CroftCommunity/arecipe`) — first crop; multi-page (one HTML
  per view) + shared bundle.
- **greetings.croft.ing** (`CroftCommunity/greetings_site`) — single-page
  hash-routed shell; server-blind link-key cards. Plan:
  `alpha/plans/2026-07-21-greetings-croft-ing-mvp.md`.
