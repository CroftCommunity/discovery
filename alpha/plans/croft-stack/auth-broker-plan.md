# Plan: production auth-helper broker (Rust) — Phase 7

date: 2026-07-29 · phase-plan for the production rebuild of the confidential-OAuth broker. Component of
[07-auth-helper.md](07-auth-helper.md). Spike (mechanism proven GO): `discovery/spike/auth-helper/`.

**Status: LIVE — Phases 0–6 done and converged (2026-07-29).** The 2026-07-24 spike proved the
mechanism; this is the hardened Rust rewrite (built TDD + `rust-enforcer`, in small committed phases).
Built + tested in `croft-stack/broker/` (70/70 cargo tests, clippy/fmt clean, deploy bats 6/6) and
**live at `account.croft.ing`** (governed unit, keys 0600, prod-LE TLS; `/healthz`, `/jwks.json`,
`/client-metadata.json` all serving). No spike to supersede — the box had been reimaged, so this was a
clean first deploy. Session logs `sessions/2026-07-29-phase-7-broker-crypto.md`, `-broker-build.md`,
`-broker-converge.md`. Hands-on verification: `croft-stack/reviews/2026-07-29-stack-review.md`. Optional next: a live
round-trip (one interactive authorize).

---

## Problem Statement

Pads hold short-lived browser-only (public-client) atproto OAuth sessions (~2 weeks). A shared,
server-side **confidential** OAuth client can refresh server-side and broker much longer-lived sessions
(≤180d, effectively unlimited via refresh), preferred-when-reachable, degrading to browser-only when
absent. The spike (throwaway TypeScript) proved this works live; we now need a **production Rust
broker** — hardened, multi-account, maintainable — at `account.croft.ing`.

## Reasoning

- **Why Rust:** operational + shares crypto/atproto libs with the other server components (Languages
  policy). The broker holds secret key material (`Zeroize`) and runs long-lived — Rust's guarantees fit.
- **Why a hand-rolled port, not a library:** the spike verified there is **no atproto-OAuth library** in
  play — its ~1,340 LOC implement DPoP, `private_key_jwt`, PAR, PKCE, JOSE, session encryption, and the
  broker ticket on stdlib crypto alone. Rust is the same shape: assemble from crypto crates, don't
  expect a turnkey atproto-OAuth crate. This is the load-bearing scope fact.
- **Reuse the spike as the executable spec:** the spike's module boundaries, request/response shapes,
  and the hard-won gotchas (below) are the reference. Port module-by-module, TDD each.

## Verified Assumptions (from the spike — `FINDINGS.md`, `FLOW-SPEC.md`, and its source)

- Endpoints the broker serves: `/healthz`, `/client-metadata.json`, `/jwks.json`, `/login`,
  `/callback`, `/api/whoami`. Confidential client_id = `https://account.croft.ing/client-metadata.json`.
- Confidential-client deltas over the pads' public client: `token_endpoint_auth_method=private_key_jwt`
  + a hosted `jwks` + a signed client assertion at the token endpoint. **Gotcha (spec-divergence,
  registered):** `token_endpoint_auth_signing_alg` is spec-optional but **bsky.social rejects without
  it** — must be sent.
- Cross-domain session handoff = **opaque ticket in the pad's first-party storage** (bearer), NOT a
  cross-site cookie (WebKit/Safari purge those — the account-kernel K1 lesson). The token never leaves
  the broker; the pad calls `/api/whoami` with the ticket and the broker acts on its behalf.
- TTLs (spec-cited): access ~1h; refresh public ≤2wk vs confidential ≤180d; refresh token **rotates**
  (single-use). Secret material (ES256 assertion key, AES store key) lives only on the box, mode 0600.

## Documentation Impact
- `croft-stack/broker/README.md` (new) — endpoints, the confidential-client model, the ticket pattern.
- `07-auth-helper.md` / roadmap `README.md` — status transitions as phases land.
- `croft-stack/ansible/roles/` — a `broker` role (replaces the spike's ad-hoc deploy); `account.croft.ing`
  Caddy vhost (reverse-proxy to the broker), like the relay.
- Spike teardown when the broker supersedes it (its `BOX-CHANGELOG` teardown).

## Concurrency Map
All phases sequential — each layer (crypto → oauth flow → session → server → deploy) consumes the prior.
Single new crate tree; no box mutation until the deploy phase (separately gated). No shared mutable state.

## Phases (TDD; crates to confirm in Phase 0/1, don't assume)

### Phase 0 — Discovery: pin the Rust crypto/HTTP/web crates DONE (2026-07-29)
Probe + decide (a few `cargo` spikes, `throwaway` disposition): ES256 sign/verify + JWK
(`p256` + `ecdsa`/`signature`, or `ring`); JWT/JOSE (hand-roll compact JWS on the above vs a JOSE
crate); base64url/sha256 (`base64`, `sha2`); HTTP client for PAR/token/refresh (`reqwest` rustls vs
`ureq`); web server (`axum`); session-at-rest encryption (`aes-gcm` + `zeroize`); store (`rusqlite` vs
flat files). **Done when:** the crate set is chosen with a working ES256-sign + verify spike.
**Outcome:** crypto floor pinned via a passing spike — `p256[ecdsa,jwk]` 0.13 (hand-rolled compact
JWS, no JOSE crate), `rand_core[getrandom]` 0.6, `sha2` 0.10, `base64` 0.22, `zeroize`,
`serde`/`serde_json`, `thiserror`. HTTP-client / axum / aes-gcm / store decisions deferred to the
phases that first need them (Phases 3–5). ES256 `.to_bytes()` = raw 64-byte r‖s (JWS-ready).

### Phase 1 — JOSE/crypto core (pure, TDD) DONE (2026-07-29)
base64url, sha256, ES256 sign/verify, JWK (public jwks.json + private key load), compact JWS. Mirror
`oauth/jose.ts`. Secret key = `Zeroize` newtype, never `Debug`/serialized in the clear.
**Outcome:** `broker/src/jose.rs` + `error.rs` — `b64url`/`b64url_decode`, `sha256`, `Es256Key`
(`generate`/`from_jwk_json`/`public_jwk_json`/`verifying_key`/`sign_jws`), `verify_jws`. `Es256Key`
has redacted `Debug` (p256 zeroizes secret scalar on drop). 7/7 tests, `clippy::pedantic` + `fmt`
clean, `#![forbid(unsafe_code)]`. Commit `c771d53`.

### Phase 2 — DPoP + PKCE + client assertion (TDD) DONE (2026-07-29)
DPoP proof creation (+ the nonce retry), PKCE challenge, the `private_key_jwt` client assertion
(**including `token_endpoint_auth_signing_alg`**). Mirror `oauth/dpop.ts`, `oauth/pkce.ts`,
`assertion.ts`. Property-test the DPoP/JWS round-trips.
**Outcome:** `pkce.rs`/`dpop.rs`/`assertion.rs` + jose helpers; `iat` injected (pure). Commit `cacf13a`.

### Phase 3 — OAuth flow (resolve → PAR → token → refresh) DONE (2026-07-29)
handle→DID→PDS→authorization-server resolution; PAR; authorization-code exchange; server-side refresh
(rotating token). Mirror `oauth/resolve.ts`, `confidential.ts`. Network calls behind a trait so units
use recorded fixtures; the live leg is a verify-in-run item (needs a test account + one interactive
authorize, exactly as the spike).
**Outcome:** `HttpClient`/`Clock` ports + `FakeHttp`; `resolve.rs`; `oauth.rs` (begin/complete/refresh/
`pds_authed_get` + `use_dpop_nonce` retry), all hermetic. Commits `79404f3` (ports+resolve), `6be262c` (flow).

### Phase 4 — session store + opaque-ticket broker (TDD) DONE (2026-07-29)
Encrypted-at-rest session store (`Zeroize` store key); mint/redeem opaque tickets; `whoami` acts on the
pad's behalf using the held session. Mirror `store.ts`, `keystore.ts`, the broker half of `server.ts`.
**Outcome:** `store.rs` (AES-256-GCM, zeroizing `StoreKey`), `keystore.rs` (load-or-create 0600),
`vault.rs` (pending/session/ticket). Commit `2c449e9`.

### Phase 5 — HTTP server + wiring test (axum) DONE (2026-07-29)
`/healthz`, `/client-metadata.json`, `/jwks.json`, `/login`, `/callback`, `/api/whoami`. **Wiring test:**
drive login→ticket→whoami through the server (fixtured OAuth) end-to-end. Mirror `server.ts`.
**Outcome:** `broker.rs` (hexagonal core, refresh-on-401 whoami), `server.rs` (axum + CORS + wiring test),
`net.rs` (ureq), `main.rs`. 70/70 tests. Commit `c0ed4e0`.

### Phase 6 — deploy (Ansible `broker` role) DONE + LIVE (2026-07-29)
Governed, hardened systemd unit (key material 0600); `account.croft.ing` Caddy vhost; Ansible `broker`
role (builds on the box from this repo's source, `Cargo.lock`-pinned).
**Outcome:** role + unit + vhost + bats (6/6) + `broker/README.md`; `site.yml`/`group_vars` wired.
**Converged live** — full `ansible-playbook site.yml` `ok=53 changed=9 failed=0`, then idempotent
`changed=0`. `account.croft.ing` serves over prod-LE TLS. Two converge-time fixes (committed): the
`src/` copy needed contents-into-dir semantics, and the box's apt rustc (1.85) was too old for the
crate tree (icu_* need 1.86) → pinned a rustup toolchain (`broker_rust_toolchain=1.86.0`). No spike
teardown needed (reimaged box). Commits `17ca385` (artifacts), `0db7ba8` + `db33077` (converge fixes).

## Open Questions
- [RECOMMENDED: PHASE-0] Crate choices (p256 vs ring; reqwest vs ureq; rusqlite vs files). *Resolve in
  Phase 0 with a compile+sign spike.*
- [RECOMMENDED: ADVISORY] Multi-account model — the spike was single-account; the production broker
  brokers for multiple pad users. Session keying + isolation shape.
- [RECOMMENDED: PHASE-GATED (Phase 3)] How to run the live OAuth leg in test — needs a test account +
  one human authorize (as the spike did); everything else fixtured.
- [RECOMMENDED: ADVISORY] Build/vendor: cross-compile the Rust broker for linux-x86_64 (like other
  server binaries) → Ansible deploys the binary (get_url from a release, or vendored).

## Review Log
### Pass 1 — 2026-07-29
Base plan from the spike's proven shape. Key scope fact surfaced: no atproto-OAuth library — this is a
hand-rolled crypto port (~1000+ LOC Rust), so it is phased small + TDD, with a Phase-0 crate-pinning
discovery. Gotchas carried forward: `token_endpoint_auth_signing_alg` required; opaque-ticket (not
cookie) cross-domain; refresh rotation; `Zeroize` for all key material.
