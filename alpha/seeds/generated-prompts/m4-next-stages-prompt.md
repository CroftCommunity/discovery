# Handoff prompt — croft M4 close-out and what comes after (written 2026-08-21)

Copy everything below the rule into a fresh session working under
`/Users/cpettet/git/chasemp/CroftC`.

---

## Where the calling ladder stands (all claims device-validated or committed)

You are continuing Phase 11 **M4 (call-time admission)** across three repos.
As of 2026-08-21, everything buildable without owner-gated infrastructure is
built, tested, and device-validated:

- **Relay side (croft-stack, through `531958f`)**: the tiered-admission plan's
  Phase 8 build surface is complete — cap evaluation (contract §7 mirror,
  mutation-clean), atproto service-auth verification (proven against live
  atproto), the `/grantCall` mint on the running `croft-relay-admit` binary
  (grant deletion → next mint refuses, tested over HTTP), the usage transport
  across the relay→admit process boundary (two-real-binaries test), and the
  DECLARED deploy (`services/croft-admit.toml` + private `services/ciss-admit.toml`
  — NOT activated; prerequisites in croft-stack `TODO.md`). Production relay is
  `croft-relay` v0.1.1 at `admission = "open"` on relay.croft.ing.
- **Client side (croft, through `5c23bfe`)**: M4a (admit client, ticket secret
  retained through redeem), M4b (service-auth proof w/ DPoP `ath`, refresh
  rotation — discovery E113 closed), M4c (mint-at-dial: `DialAdmission` pure
  core, `CallPeer.rebindWithToken` with an EndpointId-stability check,
  refusals never dial and say why, an admit OUTAGE dials tokenless with a
  note). A **first-class workflow harness** exists
  (`android/.../workflow/FixtureExchange.kt` + journey tests over the real
  ports, including the full OAuth arc over real sockets); every chunk lands
  with its journey. 135+ unit/journey tests green.
- **Device runs (2026-08-21, croft `ops/RUNBOOK-two-device-call-test.md` §11
  + addendum)**: real mint from a phone (local admit + production atproto +
  the real invite secret), minted-token dial connected direct, live
  revocation refused with words and no dial (`cap_revoked` — the seen-grants
  memory held), recovery, AND the identity path — fresh sign-in under the new
  scope, `getServiceAuth` live, the admit verifying the real ES256K proof
  against the caller's DID document, `registeredCallers` admitting. **Both
  proof paths are device-validated.**
- **O2 is closed**: OAuth scope is now `atproto transition:generic` (bare
  `atproto` cannot `getServiceAuth`; granular `rpc:` scopes are implemented
  upstream but not yet advertised by bsky.social — NARROW the scope when
  `scopes_supported` grows rpc scopes). Hosted client metadata (connect
  `c4413ef`) and `AuthManager.SCOPE` both carry it.

## Canonical documents (read in this order)

1. `croft/plans/2026-08-20-1-plan-m4-call-time-admission.md` — the M4 plan;
   chunk statuses, the harness section, open questions O1/O3.
2. `croft/ops/RUNBOOK-two-device-call-test.md` §11 + addendum — the device
   runs and the rig, including the exact adb/Playwright recipes.
3. `discovery/alpha/plans/2026-08-07-1-plan-croft-relay-tiered-admission.md`
   Review Log (chunks C–F + the 2026-08-21 device-run entry) — every
   relay-side decision with reasoning.
4. croft-stack `TODO.md` — the croft-admit activation prerequisites.
5. connect `docs/PHASE11-HANDOFF.md` and `docs/contract.md` §7 (note the
   recorded `evaluateRules` malformed-expiry divergence, port-back pending).

## The next stages, in recommended order

1. **O1 — the callee's camping token under enforce (OWNER DECISION, blocks
   the enforce rung).** Admission at attach applies to the callee's camping
   connection; `/grantCall` returns only the caller's token. Candidates named
   in the M4 plan: callee self-mints against its own repo with a self-proof;
   or the mint returns a second token the caller relays in-band; or camping
   admission rides membership rather than grants. Run this as a talk-through
   (plain english + user stories, one point at a time — the owner's preferred
   D-decision format), record it in the tiered-admission plan Review Log.
2. **The TLS staging enforce rung** (owner-gated, touches the production
   box): a second `croft-relay` listener on real certs (enforce mode, a real
   mint key from `croft-relay-admit --keygen`), because **phones cannot
   attach to a plain-HTTP relay** (runbook §11; croft-stack
   `relay/source/crates/croft-relay-bin/examples/attach_probe.rs` isolates it
   in one command — rust client attaches, iroh-ffi endpoints do not).
3. **croft-admit activation prerequisites** (croft-stack `TODO.md` item):
   musl release artifact for `croft-relay-admit` (the release-relay workflow
   builds only `croft-relay` today), on-box provisioning per the manifest
   comments, `admit.croft.ing` DNS, optional did:web document. Activation
   also unlocks journal attribution (`admitted sponsorship=…` needs a relay
   `[token]` sharing the mint's real key).
4. **The three call-endings** (M4 client scope, no infra needed): hang-up,
   remote-end, error — currently the app has no hang-up at all, which the
   device runs worked around by force-stopping.
5. **Port-backs and small closures**: the `evaluateRules` malformed-expiry
   fail-open in connect `web/resolver.js` (divergence recorded in
   contract.md §7; fix under vitest + stryker); callability could pass the
   just-redeemed ticket secret into `CallerContext` so a redeemed callee
   shows Callable rather than MayNotPermit (observed on-device, cosmetic);
   `Redeem.redeemTicket` in the app still uses `getRecord`-per-rkey — fine
   under contract v2 but check against the page's listRecords behavior if
   touched. Also file these into `discovery/alpha/ROADMAP_TODO.md` (they
   could not be filed 2026-08-21 — a concurrent session held uncommitted
   edits in that file): O1, TLS staging rung, scope-narrowing trigger,
   resolver.js port-back, native iroh logging on Android (setLogLevel
   produced no logcat output), call-endings.

## Operational facts you will need

- **Devices**: Samsung SM-S947U1 = standing test callee (no SIM, WiFi; its
  endpoint id matches the published `self` record of the callee account).
  Pixel 9 Pro = owner's personal, borrowed per-run. Debug builds take
  `-PcroftRelayUrl` / `-PcroftAdmitBase` gradle properties (production
  defaults); a debug-only cleartext manifest exists for LAN admit.
- **Test bed**: callee `ngvalidation2112.bsky.social`
  (`did:plc:xyfhcaweaeyew3zrgk6jaln7`, PDS stropharia) with grants
  `m1ticket` / `m3registered` / `m3mutuals`; caller
  `bobzmudacroft.bsky.social` (`did:plc:l5xigmplwu7eyxjobjr23iza`, PDS
  fibercap), mutual with the callee. Creds + ticket secret in `CroftC/.env`
  (git-ignored — NEVER commit or echo values; never log tokens/JWTs/keys).
  The caller session on the Pixel is signed in under the new scope.
- **Browser driving**: Playwright over the DevTools socket
  (`adb forward tcp:9222 localabstract:chrome_devtools_remote`;
  `playwright-core` importable from `croft-pwa/node_modules`). The Pixel's
  default browser is Brave — same socket name. Never adb-inject passwords.
- **Local rig**: local croft-admit over LAN http works for phones (allow the
  binary in the macOS application firewall or the POST times out); a local
  croft-relay does NOT (TLS finding above). Point phones back at the
  production relay to clear polluted discovery records.
- **discovery repo is shared with another live session**: commit only your
  own files; if the tree is dirty in a file you must touch, use index-only
  staging against HEAD; do NOT push discovery.
- **Git identity**: chasemp everywhere under `CroftC` (`chase@owasp.org`,
  remotes on `github-personal`); `gh auth switch --user chasemp` before gh.

## Discipline that applied throughout (keep it)

- TDD RED-first; workflow journeys land with every chunk (the harness is a
  first-class outcome, owner's ruling in the M4 plan); mutation runs on
  authorization-path modules with commit-before-mutate.
- External-API rule: wire shapes from source (`mint.rs`, `resolver.js`,
  `contract.md`), never inferred.
- Never log a secret, a token, or a (caller, callee) pair; DIDs alone are
  fine. Never type checksums or SHAs from memory.
- Milestone = candidate = owner promotes (`ops/RELEASING.md`); commits are
  prose in the house voice; journal environment changes (G4).
