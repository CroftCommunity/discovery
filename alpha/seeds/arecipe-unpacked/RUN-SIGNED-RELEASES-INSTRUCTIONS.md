# RUN-SIGNED-RELEASES — signed release manifests + verified-install default + version pin (Account)

Self-contained instruction file for Claude Code, repo `CroftCommunity/arecipe`.
Two parts, one branch, part boundaries gate-green. [verify-in-run] items are
probed empirically and recorded in the run summary before code depends on
them. Contradictions with this file's grounding are FINDINGS, reported not
silently absorbed.

## 0. Mission

A staged, honest increment of BUILD-PLAN Phase 3's signed-delivery design:

**Part 1 — the pipeline.** Every normal deploy (push to main → GitHub Actions
→ Pages) additionally emits and signs `release-manifest.json` — the spec's
manifest shape — with an INTERIM Ed25519 key held in GitHub Actions secrets.
Zero new manual steps for the owner; the release process is unchanged from
their side.

**Part 2 — the client.** An Account-page "Release & version" panel that
verifies the origin's manifest and surfaces verified / unsigned / invalid
states (rust banner app-wide for the bad states, dismissible); an
**install-only-verified** toggle, ON by default, that keeps this install on
its last verified version when an incoming build fails verification; and a
**version pin** — OFF by default, current-version-only — that refuses all
upgrades and shows no upgrade availability anywhere while active. Both
settings are DEVICE-LOCAL by ruling, with an explanatory note in the UI: a
pin is a fact about an install (another device may never have cached that
version and cannot downgrade to it), so it must not roam via the PDS.

Naming ruling: future record types are `app.arecipe.release` /
`app.arecipe.status`. The `fyi.recipe.*` names in BUILD-PLAN are early-test
leftovers — correct them (docs/sources/ stays frozen as historical; correct
the LIVING docs only). Publishing the manifest as a PDS record is Phase-3
future work, NOT this run; origin-only now.

## 1. Standing conventions (non-negotiable)

1. **TDD, red first.** Failing tests before implementation; fixtures (test
   keypair, signed + tampered manifest fixtures) before the features that
   consume them; run summary evidences red → green per phase.
2. **Gate green** at every part boundary; browse bundle-split guard (zero auth
   code) untouched — the verify/pin modules are auth-free and must stay so.
3. **Style**: strict vanilla TS, pure cores with injectable deps (fetch,
   storage, crypto), DOM wiring guarded by e2e, module comments explain why.
   No raw hex outside `styles.css`.
4. **Plan file** `plans/2026-07-XX-N-plan-signed-releases.md` before coding;
   Status updated at completion.
5. **Honesty in UI copy**: the panel labels the interim key as interim; the
   verified state names exactly what was checked. Never imply the Phase-3
   offline-key guarantees.

## 2. Grounded context (verified 2026-07-16 snapshot)

- Prior design: BUILD-PLAN Phase 3 + `docs/sources/arecipe-spec.md`
  §Release manifest — manifest fields (monotonic version, timestamp, per-file
  SHA-256, pubkey fingerprint, canary URI, signature), refuse-mode SW verify,
  offline key ceremony. This run implements the manifest format + client
  verification tier; refuse-everything + canary + PDS record + offline key
  remain Phase 3.
- Build (`scripts/build.mjs`): version = `${date}-${shortSha}` (line ~180) —
  human-ordered but NOT a monotonic integer; the script already computes
  SRI sha384 for the entry module + stylesheets and CSP inline hashes, so the
  integrity toolchain exists. Content-hashed bundle names.
- CI (`.github/workflows/ci.yml`): gate → build → `scripts/pages-deploy.sh`
  on main. `preview.yml` deploys per-PR previews (these will be UNSIGNED —
  correct and desirable; previews are not releases; the panel's unsigned
  state on previews is honest).
- SW (`src/sw.ts`): versioned caches; activate deletes every older-version
  cache; waiting worker applies on explicit SKIP_WAITING via the update toast
  (`src/update-toast.ts`, `sw-register.ts`); `build-info.json` is special-
  cased always-network. CRITICAL lifecycle fact the design rests on: a
  waiting SW activates once the last client of the old SW closes — consent
  holds within a session only. Hence pin AND enforcement act at the CONTENT
  layer (cache routing), not by suppressing activation. [verify-in-run:
  confirm with a probe test how the existing e2e exercises waiting workers.]
- Settings (`src/pages/settings.ts`): hosts the build-info section and
  `check-updates` button (`update-status` testid) — MIGRATES to Account this
  run, Settings keeps a one-line pointer (accepted default).
- Account (`src/pages/account.ts`): mounts members list etc.; gains the
  "Release & version" panel.
- Ed25519 verify support: WebCrypto Ed25519 shipped in all engines (Safari
  17, Firefox ~130, Chrome 137, May 2025 — researched 2026-07-16).
  Feature-detect; fallback per D6.

## 3. Locked design decisions

- **D1 Manifest.** `release-manifest.json` at the origin root, always-network
  in the SW (same treatment as build-info.json). Fields: `buildNumber`
  (monotonic integer = `git rev-list --count HEAD`), `version` (existing
  date-sha display string), `builtAt`, `files` (path → SHA-256 for every dist
  file except the manifest itself), `pubkeyFingerprint` (SHA-256 of the raw
  32-byte pubkey, hex), `sig` (Ed25519 over the canonical JSON of everything
  above). Canonicalization: stable key order, no whitespace — implemented
  once, shared by signer and verifier, pinned by committed test vectors.
- **D2 Interim key, GitHub Actions.** Owner generates the keypair locally
  (document the exact one-liners in the runbook doc); the private seed
  (base64) becomes an Actions repo secret available ONLY to the main-branch
  deploy job; the PUBLIC key is committed (constant in `src/release/keys.ts`)
  and its fingerprint baked via esbuild define. Signing happens inside
  `build.mjs` using Node's built-in `crypto` (no new dependency) when the
  secret env var is present; absent (local/PR builds) the manifest is emitted
  with `sig: null` — the client renders that as unsigned. Document plainly:
  this is the INTERIM tier (protected-branch CI can sign); Phase 3 rotates
  the pinned fingerprint to the ceremony key.
- **D3 One routing mechanism, two triggers.** The SW gains version-routing:
  a small IndexedDB config (SW-readable; localStorage is not) holding
  `{ lockedVersion?: string, requireVerified: boolean (default true),
  lastVerifiedVersion?: string }`. On fetch, if `lockedVersion` is set and
  its cache exists → serve from it; else if `requireVerified` and the active
  build's version failed verification while `lastVerifiedVersion`'s cache
  exists → serve from that. Activate-time cleanup EXEMPTS the locked and
  last-verified caches. Verification result is written by the page layer
  (panel/startup check) into the same config — the SW routes, the page
  verifies; factor the routing decision as a pure function unit-tested in
  isolation.
- **D4 Pin semantics (ruled).** OFF by default. Pin = the CURRENT running
  version only (no arbitrary picks). While pinned: update toast suppressed,
  the manual check renders `version locked at v<X>` and performs no
  reg.update(), NO upgrade availability is shown anywhere, and the SW serves
  the locked cache regardless of what activates above it. UI note verbatim
  intent: the lock applies to this install only. Unpin restores normal flow
  (toast fires if something newer is waiting). Build-stamp footer shows the
  RUNNING (locked) version while pinned, not the network-live one.
- **D5 Install-only-verified (ruled).** ON by default, device-local. When the
  origin's manifest for a newer build is missing/invalid: no toast, panel +
  banner show the bad state, and if such a build activates anyway (lifecycle),
  D3's routing keeps serving `lastVerifiedVersion`. When OFF: warn-only
  (banner + panel), updates offered normally.
- **D6 Verify path.** WebCrypto `Ed25519` first; feature-detect; fallback to
  `@noble/ed25519` so the ON-default never silently no-ops on older engines.
  [verify-in-run: noble's minified+gzip delta; if >10 KB, flag it — do not
  swap the decision unilaterally.] Panel verifies: manifest signature against
  the baked fingerprint, `buildNumber` monotonicity vs the running build, and
  SHA-256 of the entry bundle + sw.js as fetched from the origin. State
  strings name exactly that scope.
- **D7 Panel + migration.** Account "Release & version" panel: verify state
  (verified `quiet` / unsigned / invalid / couldn't check), running version,
  pin toggle with the install-only note, install-only-verified toggle, and
  the migrated check-updates button + build-info block. Settings section
  becomes a pointer line. Banner: app-wide, dismissible-per-session, rust
  family, ONLY for unsigned/invalid states on a production origin (dev/
  preview origins log instead of bannering — previews are always unsigned).
- **D8 Docs.** Correct `fyi.recipe.*` → `app.arecipe.*` in BUILD-PLAN (living
  doc); register `app.arecipe.release` + `app.arecipe.status` as **planned**
  rows in docs/LEXICONS.md; new `docs/RELEASE-SIGNING.md` runbook: key
  generation one-liners, secret installation, rotation-to-offline-key path,
  what the interim tier does and does not defend against.
- **D9 Deferred (verbatim in plan file):** PDS publication of the manifest;
  the status canary; full per-file verification at SW install time
  (refuse-mode); cosign/Sigstore transparency rider; offline key ceremony;
  multi-origin.

## 4. Phases

### Phase 0 — ground against main
Re-verify §2 (build.mjs version lines, sw special-cases, settings section,
toast/register flow). Drift = FINDING.

### Phase 1 — manifest + sign/verify core (pure, shared vectors)
RED: `tests/unit/release/manifest.spec.ts` — canonicalization is stable and
whitespace-free; committed test vectors (fixture keypair under
`tests/fixtures/release/`) round-trip: node-signed manifest verifies in the
browser-side verifier; a flipped byte in files/sig/fingerprint fails; sig:null
reports unsigned; buildNumber regression reports non-monotonic.
GREEN: `src/release/manifest.ts` (canonical form), `src/release/verify.ts`
(WebCrypto + noble fallback, injectable), signer function consumed by
build.mjs. Then RED→GREEN build integration: build.mjs emits the manifest
(sig:null without the env secret), unit-tested via the script's existing
testable seams or a dry-run invocation.

### Phase 2 — CI wiring
`ci.yml` deploy job gains the secret env; sign step runs only there;
`preview.yml` untouched (emits sig:null). In-run verification: since Actions
can't run here, add a `scripts/` self-check (`node scripts/build.mjs
--verify-manifest` exits nonzero on a bad or missing-when-expected sig) and
wire it into the gate so the FIRST real deploy proves itself; document the
owner's one-time secret-install step in RELEASE-SIGNING.md as the only manual
action ever.

### Phase 3 — SW routing (pin + enforce), pure-factored
RED: unit tests on the routing decision function (locked cache present/absent;
requireVerified with and without lastVerifiedVersion; normal path) and on
activate-cleanup exemptions; toast suppression when pinned or when the
waiting build is unverified (sw-register layer with fakes).
GREEN: IndexedDB config module (shared page+SW), sw.ts routing + cleanup
exemption, sw-register gating.

### Phase 4 — Account panel + migration + banner
RED: unit specs for panel states from injected verify results; pin toggle
writes/clears the config and reflects `version locked at v<X>`; migrated
check-updates behavior incl. the pinned inert state; settings pointer; banner
shows once per session on bad states, never on dev origin; build-stamp shows
running version under pin.
GREEN: implement; preserve existing testids (`check-updates`,
`update-status`, `build-stamp`), new ones for panel controls.

### Phase 5 — e2e + closeout
Hermetic routed fixtures: signed manifest (fixture key) → verified panel
state; tampered manifest → invalid + banner; sig:null → unsigned; pinned →
no toast on a routed newer build-info, manual check shows locked text, stamp
shows locked version; unpin → toast resumes. Mobile-fit: panel within
viewport, tap targets. Docs (D8), plan Status, run summary: red → green per
phase, [verify-in-run] outcomes (lifecycle probe, noble delta), the first-
deploy checklist for the owner.

## 5. Acceptance criteria

1. A normal push to main produces a signed manifest with zero owner action;
   PR previews and local builds honestly report unsigned.
2. The panel proves the four states against fixtures; bad states banner
   app-wide (production origin only) and name exactly what was checked.
3. Install-only-verified ON by default: an unverified newer build is never
   offered, and if it activates via lifecycle the install keeps serving the
   last verified version.
4. Pin: off by default, current-version-only; while pinned nothing anywhere
   offers or mentions an upgrade, the manual check shows the locked text, and
   the served content stays at the locked version across SW turnover; the UI
   carries the this-install-only note.
5. Both settings are device-local; nothing about them touches the PDS.
6. BUILD-PLAN carries app.arecipe.* naming; LEXICONS registers both planned
   records; RELEASE-SIGNING.md documents the interim tier honestly.
7. Bundle-split guard, row budgets, and the full gate stay green.
