# RUN-ANDROID-TWA — package arecipe as an Android app (TWA) and publish to GitHub Releases

Self-contained instruction file for Claude Code, repo `CroftCommunity/arecipe`.
Distribution target is GITHUB RELEASES (sideloadable APK), NOT Google Play —
Play publication is a possible later step and is Deferred, but nothing built
here may preclude it. Other feature runs may be in flight: this run touches
`scripts/build.mjs` (allowlist), workflows, docs, and one small site
affordance — rebase on main before merge and re-run the gate.
[verify-in-run] items are probed and recorded in the run summary before code
depends on them. Contradictions with grounding are FINDINGS.

## 0. Mission

Ship arecipe as an installable Android app using a Trusted Web Activity: the
live site rendered by Chrome full-screen, on the real `arecipe.app` origin.
Because a TWA is the real origin in the real browser engine, everything
recently built carries over UNCHANGED — service worker, signed-release
verification, version pin, origin-bound OAuth. The app is a thin shell
(~hundreds of KB); web content keeps updating via normal deploys, so shell
releases are OCCASIONAL and decoupled from web releases.

Deliverables: Digital Asset Links served by the site; a committed,
reproducible TWA build config; a tag-triggered GitHub Actions workflow that
builds, signs, and attaches the APK (with checksums and install notes) to a
GitHub Release; a keystore runbook; and a small "Get the Android app"
affordance pointing at the stable latest-release download URL.

## 1. Standing conventions (non-negotiable)

1. **TDD where testable, red first.** The dist/assetlinks plumbing, version
   derivation, and any site affordance get failing tests before
   implementation. Workflow YAML is not unit-testable: its verification is a
   dry-run job + the first-release checklist, both recorded in the summary.
2. **Gate green** (`npm test`) at every phase boundary; browse bundle-split
   guard untouched (nothing here adds page code beyond one link affordance).
3. **Style**: strict vanilla TS for anything in-app; committed configs pinned
   (Bubblewrap CLI version, JDK version); module/workflow comments explain
   why; no secrets in the repo ever.
4. **Plan file** `plans/2026-07-XX-N-plan-android-twa.md` before coding;
   Status updated at completion, house format.

## 2. Grounded context (verified 2026-07-17; Phase 0 re-grounds)

- Deployment: GitHub Actions → GitHub Pages (`ci.yml` build + gate +
  `scripts/pages-deploy.sh` on main; `preview.yml` per-PR). `scripts/
  build.mjs` is an ALLOWLIST copy into dist/ — a served `.well-known/
  assetlinks.json` requires extending the allowlist (precedent: the ICS work
  found only assets/ copies recursively). Root static files like
  `client-metadata.json` and `CNAME` are already served, so root-adjacent
  statics are established.
- PWA surface: `manifest.webmanifest` exists (installed-PWA flows work
  today); icon inventory under `assets/icons/`. [Phase 0: confirm the
  manifest carries what Bubblewrap requires — name, start_url, display,
  theme/background colors, 192 + 512 icons (maskable variant if present) —
  gaps are small manifest additions, not redesign.]
- TWA facts the design rests on (researched 2026-07-17): a TWA renders the
  site with the actual Chrome engine (not a WebView), full-screen when
  Digital Asset Links verify; verification is keyed to the SIGNING
  CERTIFICATE fingerprint — a mismatch shows the site as a Custom Tab with
  browser UI (the visible failure mode to test for); `assetlinks.json` must
  be publicly reachable at `/.well-known/assetlinks.json`. Since we are NOT
  using Play App Signing, the fingerprint is fully ours: the keystore in
  Actions secrets is the single signing identity.
- In-app release machinery (just shipped): monotonic `buildNumber` derived
  from `git rev-list --count HEAD`; RELEASE-SIGNING.md documents the
  manifest-signing ceremony — the Android keystore runbook mirrors its
  structure and tone.
- Android update rule that shapes the runbook: devices only accept updates
  signed with the SAME key. Losing the keystore means existing installs can
  never update in place. The runbook treats keystore backup as a first-class
  ceremony step.

## 3. Locked design decisions

- **D1 Package + identity.** Application id `app.arecipe.twa` (reverse-DNS
  of arecipe.app plus a discriminator; [verify-in-run: Bubblewrap accepts
  it and it collides with nothing]). Launcher name "arecipe". Display:
  standalone, default orientation. Host locked to `https://arecipe.app`.
- **D2 Asset links.** `assetlinks.json` committed in-repo (source of truth
  beside the other root statics), copied to `dist/.well-known/
  assetlinks.json` by an extended build.mjs allowlist, containing the
  release certificate's SHA-256 fingerprint. A GATE TEST (red first) asserts
  the file lands in dist AND that its fingerprint equals a committed
  expected-fingerprint constant — so a keystore rotation cannot silently
  desync the site. [verify-in-run: after the first deploy, curl
  `https://arecipe.app/.well-known/assetlinks.json` and record status +
  content-type — GitHub Pages is expected to serve dotted directories;
  proving it is a one-line probe in the house probe-doc style.]
- **D3 Reproducible TWA config.** `twa-manifest.json` committed at repo
  root (Bubblewrap's project config: package id, host, launcher name, icon
  URLs, display, fallback behavior). The Android project is GENERATED in CI
  from this config with a PINNED `@bubblewrap/cli` version — the generated
  android/ tree is NOT committed. [verify-in-run: `bubblewrap init`/`build`
  headless behavior in CI — JDK/SDK acquisition (setup-java + Bubblewrap's
  own SDK management), and that regeneration from the committed config is
  deterministic enough for repeatable builds; record exact versions used.]
- **D4 Versioning.** `versionCode` = `git rev-list --count HEAD` (the same
  monotonic integer the signed-release manifest uses — one version story);
  `versionName` = the existing date-sha display version. Both injected at
  build time, never hand-edited.
- **D5 Signing.** A dedicated Android release keystore generated ONCE by the
  owner (exact `keytool` one-liners in the runbook), stored as
  base64 Actions secrets (keystore + store/key passwords), used only in the
  release workflow. The workflow runs `apksigner verify --print-certs` after
  signing and FAILS if the certificate fingerprint differs from the
  committed expected-fingerprint constant (same constant the gate test
  checks — one source of truth). Runbook covers: generation, backup (two
  offline copies; key loss = existing installs orphaned), rotation
  consequences, and why this key is separate from the release-manifest
  Ed25519 key.
- **D6 Release workflow.** `.github/workflows/android-release.yml`,
  triggered by tag push `android-v*` AND manual dispatch. Jobs: (1) the
  normal gate; (2) build — generate project from twa-manifest.json, inject
  D4 versions, build + sign a universal APK; (3) verify — apksigner
  fingerprint check per D5; (4) publish — create/attach to a GitHub Release:
  the APK (stable asset name `arecipe.apk`), `SHA256SUMS`, and a release
  body from a committed template covering what the app is (the live site in
  Chrome), sideload install steps (unknown-sources prompt), and the note
  that in-app content updates continuously while the shell updates via new
  releases. AAB for Play is NOT built (Deferred).
- **D7 Site affordance.** One "Get the Android app" link on the Account page
  (with the other install/app matters) pointing at the stable URL
  `https://github.com/CroftCommunity/arecipe/releases/latest/download/
  arecipe.apk`, shown regardless of platform but phrased for Android.
  Small, tested, no new row budget violations.
- **D8 Deferred (verbatim in plan file):** Google Play publication (AAB,
  Play App Signing — note it would CHANGE the signing fingerprint story and
  require an assetlinks entry addition, which D2's two-fingerprint-capable
  file shape should not preclude); iOS anything; in-app update prompts for
  the shell; F-Droid.

## 4. Phases

### Phase 0 — ground against main
Confirm: build.mjs allowlist mechanics for `.well-known/`; manifest
completeness vs Bubblewrap's requirements (list gaps); icon inventory;
current ci.yml job shape to mirror conventions; no collision with in-flight
runs' files. Record findings; drift = FINDING.

### Phase 1 — assetlinks + dist plumbing (red first)
RED: gate/unit test asserting `dist/.well-known/assetlinks.json` exists
post-build, parses, targets `app.arecipe.twa`, and matches the committed
expected-fingerprint constant (placeholder fingerprint until Phase 5's
ceremony; the test reads the constant, so the ceremony updates ONE place).
RED: manifest-gap additions if Phase 0 found any (icon sizes etc.), each
with its assertion. GREEN: allowlist extension + committed files.

### Phase 2 — TWA config + version derivation
RED: unit test for the version-derivation helper (rev-list count →
versionCode; date-sha → versionName) shared with the release-manifest
buildNumber logic — one implementation, two consumers. GREEN: committed
`twa-manifest.json` (D1/D3 fields), pinned Bubblewrap version recorded in
package.json devDependencies or a pinned npx invocation — choose the more
reproducible per the Phase 0 probe and say why.

### Phase 3 — release workflow
Implement `android-release.yml` per D6 with a DRY-RUN path (workflow_dispatch
input `dry_run: true` builds and verifies but skips the release step) so the
pipeline is provable before any tag exists. Local-equivalent verification in
this run: a scripts/ helper that performs the generate-and-build steps
headlessly is exercised as far as the environment allows; whatever cannot
execute here is explicitly listed in the summary as first-release-checklist
items, never assumed.

### Phase 4 — site affordance + docs
RED: link presence/behavior test on Account (testid `android-app-link`,
correct stable URL). GREEN: link + `docs/ANDROID-APP.md` runbook (D5
ceremony incl. backup and key-loss consequences, fingerprint→assetlinks
update procedure, sideload notes, TWA failure mode: browser UI visible =
asset-links mismatch, with the D2 probe as the diagnostic) + README one-liner.
Release body template committed.

### Phase 5 — closeout
Plan Status; run summary: red → green per phase, [verify-in-run] outcomes
(Bubblewrap CI determinism, package id, Pages dot-directory probe result or
its post-deploy TODO), and the OWNER'S FIRST-RELEASE CHECKLIST in order:
generate keystore → install secrets → run ceremony step that stamps the real
fingerprint into the constant → merge → confirm live assetlinks probe → tag
`android-v1` → dry-run first if desired → install the released APK on a real
device and confirm full-screen (no browser UI) + signed-in state shared with
Chrome.

## 5. Acceptance criteria (each maps to a named test, probe, or checklist item)

1. The deployed site serves `/.well-known/assetlinks.json` with the release
   certificate fingerprint; the gate fails if dist or the fingerprint
   constant desyncs.
2. Tagging `android-v*` produces a GitHub Release carrying a signed
   `arecipe.apk` + `SHA256SUMS` + install-notes body, with the workflow
   failing on any certificate mismatch.
3. The APK is a TWA of `https://arecipe.app`: installed on-device it renders
   full-screen without browser UI (asset links verified), and web deploys
   update its content with no new APK.
4. versionCode is the shared monotonic buildNumber; versionName matches the
   web display version.
5. The keystore never touches the repo; the runbook covers generation,
   backup, loss consequences, and rotation; the release body explains
   sideloading honestly.
6. The Account link resolves to the latest release APK via the stable URL.
7. Nothing built precludes later Play publication; gate green throughout.
