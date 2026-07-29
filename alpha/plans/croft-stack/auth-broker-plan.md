# Plan: production auth-helper broker (Rust) — Phase 7

date: 2026-07-29 · phase-plan for the production rebuild of the confidential-OAuth broker. Component of
[07-auth-helper.md](07-auth-helper.md). Spike (mechanism proven GO): `discovery/spike/auth-helper/`.

**Status: PLANNED — build pending.** The 2026-07-24 spike proved the mechanism end-to-end; this is the
hardened Rust rewrite. Security-sensitive (hand-rolled OAuth crypto) → TDD + `rust-enforcer` discipline,
built in small committed phases. Not a one-shot.

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

### Phase 0 — Discovery: pin the Rust crypto/HTTP/web crates
Probe + decide (a few `cargo` spikes, `throwaway` disposition): ES256 sign/verify + JWK
(`p256` + `ecdsa`/`signature`, or `ring`); JWT/JOSE (hand-roll compact JWS on the above vs a JOSE
crate); base64url/sha256 (`base64`, `sha2`); HTTP client for PAR/token/refresh (`reqwest` rustls vs
`ureq`); web server (`axum`); session-at-rest encryption (`aes-gcm` + `zeroize`); store (`rusqlite` vs
flat files). **Done when:** the crate set is chosen with a working ES256-sign + verify spike.

### Phase 1 — JOSE/crypto core (pure, TDD)
base64url, sha256, ES256 sign/verify, JWK (public jwks.json + private key load), compact JWS. Mirror
`oauth/jose.ts`. Secret key = `Zeroize` newtype, never `Debug`/serialized in the clear.

### Phase 2 — DPoP + PKCE + client assertion (TDD)
DPoP proof creation (+ the nonce retry), PKCE challenge, the `private_key_jwt` client assertion
(**including `token_endpoint_auth_signing_alg`**). Mirror `oauth/dpop.ts`, `oauth/pkce.ts`,
`assertion.ts`. Property-test the DPoP/JWS round-trips.

### Phase 3 — OAuth flow (resolve → PAR → token → refresh)
handle→DID→PDS→authorization-server resolution; PAR; authorization-code exchange; server-side refresh
(rotating token). Mirror `oauth/resolve.ts`, `confidential.ts`. Network calls behind a trait so units
use recorded fixtures; the live leg is a verify-in-run item (needs a test account + one interactive
authorize, exactly as the spike).

### Phase 4 — session store + opaque-ticket broker (TDD)
Encrypted-at-rest session store (`Zeroize` store key); mint/redeem opaque tickets; `whoami` acts on the
pad's behalf using the held session. Mirror `store.ts`, `keystore.ts`, the broker half of `server.ts`.

### Phase 5 — HTTP server + wiring test (axum)
`/healthz`, `/client-metadata.json`, `/jwks.json`, `/login`, `/callback`, `/api/whoami`. **Wiring test:**
drive login→ticket→whoami through the server (fixtured OAuth) end-to-end. Mirror `server.ts`.

### Phase 6 — deploy (Ansible `broker` role) + supersede the spike
Governed, hardened systemd unit (secrets addendum — key material 0600, `Zeroize`); `account.croft.ing`
Caddy vhost; Ansible `broker` role (pinned artifact). Then tear down the spike (its BOX-CHANGELOG).
Live gate: a pad holds a broker session past the browser-only TTL and falls back cleanly when stopped.

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
