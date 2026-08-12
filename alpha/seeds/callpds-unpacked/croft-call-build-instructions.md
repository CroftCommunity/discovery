# croft-call: build instructions for Claude Code

Goal: one new repo containing (a) the Croft Exchange lookup page, deployed to
GitHub Pages, and (b) the Croft Call Android app, built and tested in CI with
installable APK artifacts. TDD-first throughout. Each phase is one reviewable
commit series with its own why.

Two starting implementations may be provided alongside these instructions:
`croft-exchange.html` (the working lookup page) and `croftcall-android.zip`
(the app scaffold, including a README with a verified/to-verify API ledger).
If present, unpack and adopt them as the starting point, then refactor to the
layout below. If absent, build to the specs in sections 5 and 6.

---

## 1. Repo layout

```
croft-call/
├── README.md                  what this is, links to live page + latest APK
├── web/                       GitHub Pages site root
│   ├── index.html             the exchange page (thin shell)
│   ├── resolver.js            resolution pipeline as a testable ES module
│   ├── .nojekyll              serve dotfile/underscore paths as-is
│   ├── CNAME                  only if the custom domain is enabled (see 7)
│   └── .well-known/
│       └── assetlinks.json    Android App Links (placeholder until signed)
├── web-tests/                 vitest unit tests for resolver.js
│   └── resolver.test.js
├── android/                   the Kotlin project (from croftcall-android.zip)
│   └── ...                    unchanged layout; README's ledger travels with it
├── docs/
│   └── adr/                   one ADR per consequential decision
└── .github/workflows/
    ├── web.yml                web tests + Pages deploy on main
    └── android.yml            unit tests + assembleDebug, upload APK artifact
```

Monorepo rationale (record as ADR-0001): the page and the app share one
contract, the `croftcall://call` deep link and the `ing.croft.iroh.endpoint`
record shape. One repo keeps the contract's two halves versioned together,
and Pages can later serve the app's assetlinks.json from the same origin the
page lives on.

## 2. Environment Claude Code needs

- git, Node 20+ (web tests), JDK 17, Android SDK platform 35 + build tools
  (in CI use `android-actions/setup-android`; locally Android Studio's SDK).
- Network access: Maven Central (`computer.iroh:iroh`), Google Maven, npm,
  and for verification fetches: docs.iroh.computer, the Dokka reference at
  n0-computer.github.io/iroh-ffi/kotlin/, and
  github.com/n0-computer/hello-iroh-ffi.
- GitHub CLI (`gh`) authenticated, to create the repo and enable Pages.

## 3. Phase 0: scaffold and contract

- Create the repo, commit the layout above with placeholder files.

- Define the shared contract in one place, `docs/contract.md`, and treat it as
  the source of truth both halves test against:
  - Lexicon: collection `ing.croft.iroh.endpoint`, rkey `self`, fields
    `endpointId` (string, required), `homeRelay` (string, optional),
    `createdAt` (string, optional).
  - Deep link: `croftcall://call?endpoint=<id>&relay=<url>&handle=<h>&did=<did>`,
    `endpoint` required, all values URL-encoded.

- CI skeletons for both workflows that run and pass trivially (empty test
  suites green) before any feature work, so every later phase lands on a
  working pipeline.

## 4. Phase 1: web, test-first

Extract the resolution pipeline from the page into `web/resolver.js` exporting
pure async functions, each taking a `fetch` implementation as a parameter so
tests inject mocks (no msw needed, plain stub fetch is fine):

- `resolveHandle(fetch, handle) -> did` via
  `https://public.api.bsky.app/xrpc/com.atproto.identity.resolveHandle?handle=`

- `resolvePds(fetch, did) -> pdsUrl`: `did:plc:` via `https://plc.directory/{did}`,
  `did:web:` via `https://{host}/.well-known/did.json`; select the service with
  id ending `#atproto_pds` or type `AtprotoPersonalDataServer`.

- `fetchCallingRecord(fetch, pdsUrl, did) -> {endpointId, homeRelay}` via
  `com.atproto.repo.getRecord` with the contract's collection/rkey.

- `buildDeepLink({endpointId, relay, handle, did}) -> string` exactly per the
  contract, URL-encoding every value.

TDD order: write `resolver.test.js` cases first (happy path per function;
handle not found; DID doc without a PDS service; record missing; record
missing `endpointId`; did:web host extraction; deep link encoding of `+`, `/`,
`:` in values), then implement to green. `index.html` becomes a thin shell
importing `resolver.js`; keep the existing visual design and the live
resolution-trace UI intact. Manual smoke check: `npx serve web` and look up a
real handle (the page hits live public APIs; publish a test record to your own
repo with `com.atproto.repo.putRecord` if you need a positive case).

## 5. Phase 2: Pages deploy

`web.yml`, two jobs:

- test: checkout, setup-node, `npm ci` (workspace root package.json holds
  vitest), `npx vitest run`.

- deploy (needs: test, on push to main only): `actions/configure-pages`,
  `actions/upload-pages-artifact` with `path: web`, `actions/deploy-pages`.
  Workflow permissions: `pages: write`, `id-token: write`, `contents: read`;
  environment `github-pages`.

Enable Pages once via `gh api` or instruct the human: repo Settings > Pages >
Source: GitHub Actions. Acceptance: the page is live at
`https://<owner>.github.io/croft-call/` and a real lookup resolves. Note the
page is pure static + client-side fetch to third-party APIs, so Pages hosting
imposes no backend constraint; the APIs used (AppView, plc.directory, PDS
XRPC) are CORS-open by design of atproto's browser-app model, which Phase 1's
manual smoke check will confirm from the deployed origin as well.

## 6. Phase 3: Android, verify-then-test

Order matters: verification before compilation, because the scaffold's
`net/CallPeer.kt` carries `VERIFY` markers on the accept/connect/bi-stream
API names. The bind/identity/lifecycle calls are already verified against
docs.iroh.computer/languages/kotlin; the stream-level names were written
against the docs' "maps 1:1 to Rust" claim.

- Verification pass: fetch the Dokka reference
  (n0-computer.github.io/iroh-ffi/kotlin/) and the reference implementation
  `hello-iroh-ffi/kotlin-android/app/src/main/java/computer/iroh/dot/net/IrohPeer.kt`;
  correct every `VERIFY`-marked name in `CallPeer.kt` to the real API. Record
  the diff in ADR-0002. Do not touch architecture, only names/signatures.

- Build gotchas already encoded in the scaffold, keep them: JNA must come from
  the `@aar` variant (transitive jar excluded), Kotlin 2.2+, minSdk 26,
  `IrohAndroid.installAndroidContext` before first bind. If Gradle packaging
  fails for missing Android ABIs in the `computer.iroh` artifact, fall back to
  building iroh-ffi from source per docs.iroh.computer/languages/kotlin
  ("Building from source") and record it in the ADR; the docs and the
  reference app disagree on whether the Maven artifact covers Android, and
  the reference app (newer) says it does.

- TDD on the pure-JVM layer (no device needed): unit tests for
  `DeepLink.parse` (valid link; missing endpoint -> null; wrong scheme/host ->
  null; URL-decoded values; extra params ignored) and `WireFormat`
  (encode/length round-trip; oversize rejection; JSON escaping). These test
  the shared contract from the Android side, mirroring Phase 1's web tests.

- Instrumented/E2E is out of scope for CI (needs devices and two peers);
  instead document the manual loop in `android/README.md`: install on two
  devices, publish device A's endpoint id in a test record, look it up on the
  deployed page from device B's browser, tap Connect, expect the hello
  exchange. Also keep the adb deep-link one-liner for pageless testing.

`android.yml`: setup-java 17, setup-android, gradle cache, `./gradlew test`,
`./gradlew assembleDebug`, upload `app-debug.apk` as a workflow artifact. On
tags, attach the APK to a GitHub Release instead.

## 7. Phase 4: custom domain and App Links (optional, gated on the human)

Only with explicit go-ahead, since it touches DNS:

- Custom domain: add `web/CNAME` containing `call.croft.ing`, set the domain
  in Pages settings, and tell the human the DNS record to create (CNAME
  `call.croft.ing` -> `<owner>.github.io`). Wait for Pages TLS provisioning.

- App Links: populate `web/.well-known/assetlinks.json` with the app's package
  (`ing.croft.call`) and the release signing cert SHA-256 fingerprints (the
  human must supply these from their keystore; do not invent). Then uncomment
  the stubbed `https://call.croft.ing` intent filter in the manifest. Until
  then the `croftcall://` scheme link works everywhere with the chooser
  prompt, so this phase blocks nothing.

## 8. Working rules

- TDD: red, green, refactor, per phase. No feature commit without its tests
  in the same series.

- Ground before assert: any API name, action version, or endpoint not already
  verified in the scaffold's ledger gets checked against its primary source
  (Dokka, GitHub Actions docs, atproto specs) before use, and the ADR notes
  what was checked and when.

- Where sources conflict (the Maven-artifact-on-Android question), prefer the
  newer and more specific source, take the fallback path if reality disagrees,
  and write down which way it went.

- Keep the contract file authoritative: if any change touches the deep link or
  the record shape, update `docs/contract.md` first, then both halves' tests,
  then the implementations.

- Commit hygiene: conventional commits, one concern per commit, phase = PR.

## 9. Acceptance for the whole run

1. `web.yml` green: unit tests pass and the page deploys to Pages; a live
   lookup of a handle with a published record shows the entry and a correct
   deep link; a handle without one shows "not listed".

2. `android.yml` green: JVM unit tests pass, debug APK artifact downloadable.

3. All `VERIFY` markers in `CallPeer.kt` resolved with ADR-0002 documenting
   the corrections.

4. Manual loop documented and, if two devices are available, demonstrated:
   page lookup -> Connect -> app opens with callee -> dial -> hello exchange.

## 10. Open questions (defaults chosen so the run does not block)

1. Repo name/owner: default `croft-call` under the human's account, private.

2. Custom domain now? Default: no, Phase 4 waits for go-ahead.

3. Web test tooling: default vitest; switch only if the human prefers.

4. Release signing for the APK: default debug-signed CI artifacts only;
   release keystore setup is the human's call later.
