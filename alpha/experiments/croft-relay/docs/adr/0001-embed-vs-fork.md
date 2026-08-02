# ADR-0001: Embed iroh-relay as a library; do not fork

- Status: Accepted (spike-grade; revisit if the Phase-3 seam lands upstream)
- Date: 2026-08-02
- Phase: 0 (baseline + reconnaissance)

## Context

`croft-relay` extends an iroh relay to gate calling on atproto identity. Two
shapes were on the table (see the build plan §3):

- **Option A — embed:** depend on `iroh-relay` as a library, supply our own
  admission logic, keep the upstream diff at or near zero.
- **Option B — patch fork:** carry a patch series on a fork of
  `n0-computer/iroh`.

The decision rule was: attempt A; fall back to B only for the specific
capability the library cannot express.

## Reconnaissance (against real source, not the plan's assumptions)

Environment constraint, recorded honestly: **`github.com` is blocked by egress
policy in this session (HTTP 403)**, so the plan's Phase-0 step "clone iroh,
build `iroh-relay --features server`, run its test suite" could not be
performed. Instead the pinned crate source was fetched from the crates.io
registry (which is reachable) and read directly:

- Pinned version: **`iroh-relay 1.0.0-rc.1`** (matches the sibling `iroh/`
  experiment's lockfile).
- Source read at:
  `~/.cargo/registry/src/index.crates.io-*/iroh-relay-1.0.0-rc.1/`.

Findings (each verified in source; file paths are within that crate):

1. **A public `AccessControl` trait exists** — the embed seam.
   `src/server.rs:284`:
   ```rust
   pub trait AccessControl { fn on_connect(&self, request: &ClientRequest)
       -> impl Future<Output = Access> + Send; }
   pub enum Access { Allow, Deny { reason: Option<String> } }   // :349
   ```
   Our admission decision (registry lookup in Phase 1, token verification in
   Phase 2) maps directly onto an `AccessControl` impl returning
   `Access::Allow` / `Access::Deny`. **This is the finding that makes Option A
   feasible.**

2. **The endpoint id is cryptographically authenticated**, not asserted:
   `ClientRequest::endpoint_id()` is available inside `on_connect`, and the
   relay runs a challenge/response (`X-Iroh-Challenge` / `X-Iroh-Response`,
   `src/server.rs:78`). This is the hinge Phase 2's anti-replay gate depends on.

3. **Existing access modes** (`src/main.rs`): `Everyone`, `Allowlist`,
   `Denylist`, and `Http` (POST-per-connection hook granting on `200` + body
   `true`). The plan mentioned open / bearer / HTTP-hook; the allow/deny-list
   modes were not in the plan. Bearer-token auth in rc.1 is on the *hook
   request* (`Authorization: Bearer`, or `IROH_RELAY_HTTP_BEARER_TOKEN`), not a
   client-presented relay token — a point the plan blurred.

4. **CORRECTION — the HTTP-hook header is `X-Iroh-NodeId`, not
   `X-Iroh-Endpoint-Id`.** `src/main.rs:35`:
   `const X_IROH_ENDPOINT_ID: &str = "X-Iroh-NodeId";`. The doc-comment two
   lines down *says* `X-Iroh-Endpoint-Id`, but the bytes on the wire are
   `X-Iroh-NodeId` (the NodeId->EndpointId rename did not reach the literal).
   `croft-admit`'s access service keys on the real header and accepts the
   documented alias (`http_api.rs`).

5. **Rate-limit primitives are public and exist per the plan:**
   `pub struct Bucket` (`src/server/streams.rs:350`) and
   `pub struct ClientRateLimit { bytes_per_second: NonZeroU32,
   max_burst_bytes: Option<NonZeroU32> }` (`src/server.rs:500`). Config is
   `[limits.client.rx]` (`bytes_per_second`, `max_burst_bytes`).

6. **The one gap: per-connection rate override at admission.** `Access::Allow`
   is a unit variant — it carries **no** rate limit. So the library can gate
   *whether* a connection is admitted, but cannot, through the access seam,
   say *at what rate*. Tiered per-connection buckets (Phase 3) therefore need
   either (a) our embedding layer to apply a `Bucket` around the connection
   itself, or (b) a small upstream change: `Access::Allow { rate_limit:
   Option<ClientRateLimit> }`. That is precisely the upstream candidate the
   plan predicted (§5, Phase-3 → Phase-5).

## Decision

**Option A (embed).** The `AccessControl` trait is a clean, public,
cryptographically-grounded seam that carries Phase 1 and Phase 2 with a
zero-line upstream diff. The single capability the library cannot express —
per-connection rate override — is small, generic, and useful beyond us, so it
becomes an upstream PR (ADR-0004 / plan Phase 5), with an in-embedding-layer
`Bucket` fallback until it lands. This is the sanctioned hybrid: embed + one
small, upstream-shaped patch.

## Consequences

- `croft-admit` (this experiment's built crate) is deliberately
  **relay-agnostic** (no `iroh`/`iroh-relay` dependency). It is the app-side
  core; the thin `AccessControl` adapter that wires it to a real relay is the
  next build step and is where the only iroh dependency will live.
- Keeping the core relay-free is also what keeps the eventual upstream slices
  (signed-token access mode; per-connection rate override) free of any atproto
  or tier vocabulary.
- **Deferred, and why:** standing up a live relay, two endpoints, and a
  holepunch exchange (the plan's Phase-0 acceptance and Phase-3 calibration)
  needs multi-process networking this session's sandbox cannot run, on top of
  the github-clone block. Those legs are named in the README and OPEN-QUESTIONS
  and are the first thing the next session with network + a relay should do.

## Verification pointers (re-check when unblocked)

- iroh-relay README + `src/main.rs` + `src/server.rs` at the pinned tag.
- https://github.com/n0-computer/iroh/tree/main/iroh-relay
- https://docs.iroh.computer/concepts/relays , /concepts/holepunching
