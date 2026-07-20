# RUN-SIGNED-RELEASES v2 — signed manifests + verified-install default + version pin

Self-contained instruction file for Claude Code, repo `CroftCommunity/arecipe`.
SUPERSEDES v1 (unexecuted). Two parts, one branch, gate-green at part
boundaries. [verify-in-run] items are probed empirically and recorded in the
run summary before code depends on them. Contradictions with this file's
grounding are FINDINGS — report, don't silently absorb.

## 0. Mission

A staged, honest increment of BUILD-PLAN Phase 3's signed-delivery design:

**Part 1 — pipeline.** Every normal deploy (push to main → GitHub Actions →
Pages) additionally emits and signs `release-manifest.json` (the spec's
manifest shape) with an INTERIM Ed25519 key held in GitHub Actions secrets.
Zero new manual steps: the owner's release process is unchanged.

**Part 2 — client.** An Account "Release & version" panel (verified /
unsigned / invalid / couldn't-check states; dismissible rust banner app-wide
for the bad states on the production origin); an **install-only-verified**
toggle ON by default that keeps an install on its last verified version when
an incoming build fails verification; a **version pin** OFF by default,
current-version-only, that refuses all upgrades and shows no upgrade
availability anywhere while active. Both settings are DEVICE-LOCAL (ruled),
with a UI note that they apply to this install only — a pin references a
device-local cache another install may never have had, so it must not roam.

Naming ruling: future records are `app.arecipe.release` /
`app.arecipe.status`; correct `fyi.recipe.*` in the LIVING docs (BUILD-PLAN).
`docs/sources/` stays frozen as historical. PDS publication of the manifest,
the status canary, full per-file refuse-mode, and the offline key ceremony
all remain Phase 3 — deferred, listed in the plan file.

## 1. Standing conventions (non-negotiable)

1. **TDD, red first.** Failing tests before implementation; fixtures (test
   keypair, signed/tampered/stale manifest fixtures) before consumers; the
   run summary evidences red → green order per phase.
2. **Gate green** at every part boundary. Browse bundle-split guard (zero
   auth code in browse.html) untouched — every module this run adds is
   auth-free and must stay so.
3. **Style**: strict vanilla TS; pure cores with injectable deps (fetch,
   storage, crypto); DOM wiring guarded by e2e; module comments explain why;
   no raw hex outside `styles.css`.
4. **Plan file** `plans/2026-07-XX-N-plan-signed-releases.md` before coding;
   Status updated at completion, house format.
5. **Honest copy**: the panel labels the interim key as interim and names
   exactly what was checked; never imply the Phase-3 offline-key guarantees.

## 2. Grounded context (verified 2026-07-16; Phase 0 re-grounds)

- Prior design: BUILD-PLAN Phase 3 + `docs/sources/arecipe-spec.md`
  §Release manifest — manifest fields, refuse-mode SW verify, offline key,
  canary, dual origin+PDS publication. This run ships the manifest format and
  the warn/enforce client tier only.
- Build (`scripts/build.mjs`): `version = ${date}-${shortSha}` (~line 180) —
  display-friendly, NOT monotonic. The script already computes SRI sha384 on
  the entry module + stylesheets and CSP inline hashes: the integrity
  toolchain exists; the manifest extends it. Bundle names are content-hashed.
- CI (`.github/workflows/ci.yml`): gate → build → `scripts/pages-deploy.sh`
  on main. `preview.yml` = per-PR previews; previews will be UNSIGNED, which
  is correct (previews are not releases) and must not banner.
- SW (`src/sw.ts`): cache names carry the build version; activate deletes
  every older-version cache; waiting worker applies on explicit SKIP_WAITING
  via the toast (`update-toast.ts`, `sw-register.ts`); `build-info.json` is
  special-cased always-network. LIFECYCLE FACT the design rests on: a waiting
  SW activates once the last client of the old SW closes — toast consent
  holds within a session only. Hence pin AND enforcement act at the CONTENT
  layer (cache routing), never by suppressing activation.
- Origin classification: sign-in gating references authModeFor in auth code.
  The banner runs on ALL pages including browse, so this run needs a
  NEUTRAL origin classifier (production / preview / loopback) in an auth-free
  module; do not import auth anywhere new. [verify-in-run: where authModeFor
  lives and whether a shared neutral helper can be extracted without moving
  auth code into shared chunks.]
- Settings (`src/pages/settings.ts`): hosts the build-info block +
  `check-updates` button (`update-status` testid) — migrates to Account;
  Settings keeps a pointer line (accepted default).
- Ed25519: WebCrypto support shipped in all engines (Safari 17, Firefox ~130,
  Chrome 137 in May 2025 — researched 2026-07-16). Available in service
  workers as well as windows; feature-detect per D6.

## 3. Locked design decisions

- **D1 Manifest.** `release-manifest.json` at the origin root; SW treats it
  always-network like build-info.json. Fields: `buildNumber` (monotonic
  integer = `git rev-list --count HEAD`), `version` (existing date-sha
  string), `builtAt`, `files` (path → SHA-256 for every dist file except the
  manifest itself), `pubkeyFingerprint` (SHA-256 hex of the raw 32-byte
  pubkey), `sig` (Ed25519 over the canonical JSON of all preceding fields).
  Canonicalization: stable key order, no whitespace, one implementation
  shared by signer and verifier, pinned by committed test vectors. The
  running build learns its own `buildNumber` via esbuild define (added to
  build-info emission too) so monotonicity is comparable client-side.
- **D2 Interim key, GitHub Actions.** Owner generates the keypair locally
  (exact one-liners in `docs/RELEASE-SIGNING.md`); the private seed (base64)
  becomes an Actions secret exposed ONLY to the main-branch deploy job; the
  public key is committed (`src/release/keys.ts`) and its fingerprint baked
  via define. Signing runs inside `build.mjs` with Node's built-in `crypto`
  (no new dependency) when the secret env is present; otherwise the manifest
  is emitted with `sig: null` (local + preview builds → honest unsigned).
  The runbook states plainly: interim tier = protected-branch CI can sign;
  Phase 3 rotates the pinned fingerprint to the ceremony key.
- **D3 The SW is the authoritative verifier; one routing mechanism, two
  triggers.** A small IndexedDB config (SW-readable; localStorage is not),
  shared page+SW module: `{ lockedVersion?, requireVerified (default true),
  lastVerifiedVersion?, verdict? }`.
  At ACTIVATE, the new SW verifies ITSELF: fetch the origin manifest;
  outcomes —
  (a) sig valid against the baked fingerprint AND manifest.version equals the
  SW's own version AND buildNumber is not a regression → record verified,
  set `lastVerifiedVersion` to itself, clean older caches EXCEPT the locked
  one;
  (b) manifest is valid but for a DIFFERENT (newer) version → STALE-MISMATCH:
  a deploy raced this install; not an attack signal — keep the previous
  `lastVerifiedVersion`, do not banner, await the next update cycle;
  (c) sig missing (`null`) or invalid, or buildNumber regressed → record the
  bad verdict; if `requireVerified` and a `lastVerifiedVersion` cache exists,
  FETCH-ROUTE to that cache (this install keeps running the last verified
  version); never delete that cache.
  Fetch routing precedence: `lockedVersion` cache first (pin), then the
  enforcement fallback, then normal. The routing decision and the verdict
  logic are PURE functions unit-tested in isolation; the panel reads verdicts
  from the config and can re-run an on-demand origin check for display, but
  the install-time verdict is the SW's.
- **D4 Pin (ruled).** OFF by default; pin = the CURRENT running version only.
  While pinned: toast suppressed; the manual check renders
  `version locked at v<X>` and performs no reg.update(); NO upgrade
  availability appears anywhere; SW serves the locked cache regardless of
  what activates above it; activate cleanup exempts it; the build-stamp
  footer shows the RUNNING (locked) version, not network-live build-info.
  UI note: applies to this install only. Unpin restores the normal flow
  (toast fires if something newer is waiting).
- **D5 Install-only-verified (ruled).** ON by default, device-local. Page
  layer additionally verifies a WAITING build's manifest before offering the
  toast (pre-offer check): unverified → no offer, panel + banner state
  instead. If an unverified build activates anyway (lifecycle), D3(c) keeps
  the install on the last verified version. OFF → warn-only; updates offered
  normally.
- **D6 Verify path.** WebCrypto `Ed25519` first, feature-detected, in both
  window and SW contexts; fallback `@noble/ed25519` so the ON-default never
  silently no-ops. [verify-in-run: noble's min+gzip delta in page AND sw
  bundles; >10 KB total → FLAG, do not swap the decision unilaterally.]
  Verification scope this tier: manifest signature, version identity,
  buildNumber monotonicity, SHA-256 of the entry bundle as fetched from the
  origin. Panel state strings name exactly that scope.
- **D7 Panel + migration + banner.** Account "Release & version" panel:
  verify state, running version, pin toggle (+ install-only note),
  install-only-verified toggle, migrated check-updates button + build-info
  block; Settings keeps a pointer. Banner: app-wide, dismissible per session,
  rust family, ONLY for unsigned/invalid verdicts on the PRODUCTION origin —
  preview/loopback log instead (neutral classifier per §2). Browse hosts the
  banner too, so the banner module is auth-free by construction.
- **D8 Docs.** BUILD-PLAN: `fyi.recipe.*` → `app.arecipe.*`. LEXICONS.md:
  `app.arecipe.release` + `app.arecipe.status` as **planned** rows.
  New `docs/RELEASE-SIGNING.md`: key generation, one-time secret install (the
  only manual step ever), rotation-to-offline-key path, and an explicit
  what-this-does-and-does-not-defend-against section.
- **D9 Deferred (verbatim in plan file):** PDS record publication; status
  canary; full per-file refuse-mode at install; Sigstore transparency rider;
  offline key ceremony; multi-origin.

## 4. Phases

### Phase 0 — ground against main
Re-verify §2 (build.mjs version lines, sw special-cases and cache naming,
settings section, toast/register flow, authModeFor location). Probe: how the
existing e2e exercises waiting workers and whether Playwright routing can
fixture the SW's own manifest fetch; if it can't, factor SW verdict/routing
pure (they are anyway) and let e2e assert page-observable outcomes only.
Drift or probe surprises = FINDINGS.

### Phase 1 — manifest + sign/verify core (pure, shared vectors)
RED `tests/unit/release/manifest.spec.ts`: canonicalization stable and
whitespace-free; fixture keypair under `tests/fixtures/release/`; vectors
round-trip node-signed → browser-verifier; flipped byte in files / sig /
fingerprint fails; `sig: null` reports unsigned; buildNumber regression
reports non-monotonic; version-mismatch reports STALE, not invalid.
GREEN: `src/release/manifest.ts`, `src/release/verify.ts` (WebCrypto +
fallback, injectable), signer consumed by build.mjs. Then RED→GREEN build
integration: manifest emitted (sig:null without the env secret), buildNumber
in build-info + define, `node scripts/build.mjs --verify-manifest` self-check
exits nonzero on bad-or-missing-when-expected sig; wire the self-check into
the gate.

### Phase 2 — CI wiring
`ci.yml` main deploy job gains the secret env + sign; `preview.yml`
untouched. Actions can't execute in this run: the gate-wired self-check plus
a first-deploy checklist in RELEASE-SIGNING.md (install secret → push →
confirm panel shows verified) carries the proof to the first real deploy.

### Phase 3 — config + SW verification/routing
RED: unit specs for the shared IndexedDB config module; the verdict function
(valid / stale-mismatch / unsigned / invalid / regression per D3); routing
precedence (pin > enforcement fallback > normal); activate-cleanup exemptions
(locked + lastVerified); sw-register pre-offer gating (D5) and pin
suppression (D4) with fakes.
GREEN: config module, sw.ts activate-verify + routing + cleanup, sw-register
gating.

### Phase 4 — panel, migration, banner, stamp
RED: panel states from injected verdicts; pin toggle writes/clears config,
renders `version locked at v<X>`, manual check inert while pinned; migrated
check-updates behavior; Settings pointer; banner once-per-session on bad
verdicts, production origin only, present on browse (auth-free import graph
asserted); build-stamp shows running version under pin.
GREEN: implement; preserve `check-updates`, `update-status`, `build-stamp`
testids; new testids for panel controls.

### Phase 5 — e2e + closeout
Hermetic routed fixtures: signed (fixture key) → verified; tampered →
invalid + banner; sig:null → unsigned; stale-mismatch → quiet; pinned →
routed newer build-info produces NO toast, locked text on manual check,
stamp shows locked version; unpin → toast resumes; enforcement → unverified
waiting build never offered. Mobile-fit: panel in viewport, tap targets.
Docs per D8; plan Status; run summary: red → green per phase,
[verify-in-run] outcomes (SW e2e probe, noble delta, authModeFor extraction),
owner's first-deploy checklist.

## 5. Acceptance criteria (each maps to a named test or artifact)

1. A normal push to main produces a signed manifest with zero owner action;
   local and preview builds honestly report unsigned and never banner.
2. The panel proves all four states against fixtures; bad states banner
   app-wide on the production origin only and name exactly what was checked.
3. Install-only-verified ON by default: an unverified newer build is never
   offered, and if it activates via lifecycle the install keeps serving the
   last verified version — proven at the routing-function level and
   page-observably in e2e.
4. Pin: off by default, current-version-only; while pinned nothing anywhere
   offers or mentions an upgrade, the manual check shows the locked text, the
   stamp shows the locked version, and served content stays locked across SW
   turnover; the UI carries the this-install-only note.
5. Both settings are device-local; nothing about them touches the PDS.
6. A racing deploy (stale-mismatch) neither banners nor corrupts state.
7. BUILD-PLAN carries app.arecipe.* naming; LEXICONS registers both planned
   records; RELEASE-SIGNING.md states the interim tier honestly.
8. Bundle-split guard, toolbar row budgets, and the full gate stay green.
