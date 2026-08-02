# ADR-0005: Phase-0 baseline against iroh main, and the embed adapter

- Status: Accepted
- Date: 2026-08-02 (follow-up run; GitHub git access restored mid-session)
- Phase: 0 (baseline, completed) + 2 (embed adapter, built)

## Context

ADR-0001 recorded that `github.com` was egress-blocked, so the plan's Phase-0
baseline ("clone iroh, build `iroh-relay --features server`, run its tests")
could not run, and recon fell back to reading the pinned `1.0.0-rc.1` crate from
the registry. Later in the session the **git protocol to GitHub became
reachable** (clone/fetch over HTTPS succeed; the web UI, REST API, and codeload
tarball endpoints still 403). That unblocked the baseline and the embed adapter.

## What was done

### 1. Baseline (Phase 0 acceptance, the buildable half)

- Cloned `n0-computer/iroh` (shallow, `main`). `iroh-relay` on main is
  **1.0.3** (the latest published release; the sibling experiment and our recon
  used `1.0.0-rc.1`).
- Built and ran `cargo test -p iroh-relay --features server`: **80 tests pass,
  0 fail** (`evidence/iroh-relay-baseline.txt`). This is the recorded baseline.
- The multi-process leg of Phase-0 acceptance (two endpoints, A reaches B via a
  running relay, holepunch) is still **not** done — it needs live networking
  beyond a clone-and-test. It remains the open item, now the *only* thing
  blocking full Phase-0/Phase-3 sign-off.

### 2. Re-verification against 1.0.3 (ADR-0001 findings still hold)

- **Header discrepancy persists on main.** `iroh-relay/src/main.rs`:
  `const X_IROH_ENDPOINT_ID: &str = "X-Iroh-NodeId";`, and it is *used* to set
  the outgoing hook header (`.header(X_IROH_ENDPOINT_ID, endpoint_id...)`),
  while the doc-comment still says `X-Iroh-Endpoint-Id`. Our `http_api.rs`
  already keys on the real name. This is a clean doc-fix upstream candidate.
- **`AccessControl` / `Access` / `ClientRequest` unchanged** in shape:
  `Access` is still `Allow` / `Deny { reason }` with **no rate-limit field** on
  `Allow`, so the per-connection-rate-override gap (ADR-0004, the Phase-3/5
  upstream candidate) is confirmed on 1.0.3, not just rc.1.
- New, useful for testing: `ClientRequest::new(endpoint_id, protocol_version,
  parts)` and `ClientRequest::auth_token()` (walks `Authorization: Bearer`,
  falls back to `?token=`) are **public**, so the adapter is testable without a
  live relay.

### 3. The embed adapter (`croft-relay-embed`)

New crate — the **one** iroh-dependent crate — implementing
`iroh_relay::server::AccessControl` over `croft-admit`:

- `TokenAccess::on_connect` reads `request.auth_token()` and
  `request.endpoint_id()` (the handshake-authenticated id), runs
  `croft_admit::TokenVerifier::verify(token, connecting, now)`, and maps the
  result to `Access::Allow` / `Deny`. The real clock is read only at this edge;
  the injected-clock `decide` underneath stays deterministic.
- `RegistryAccess` is the Phase-1 in-process equivalent of the HTTP hook.
- `EmbedDecision::Admit { tier, bucket }` computes the tier's bucket so the
  rate decision is ready and tested, even though `Access::Allow` cannot carry
  it yet (the gap above). Applying it awaits the upstream change; a
  stream-wrapping `Bucket` is the interim fallback.

**7 tests, all green, against real iroh types** (`evidence/embed-tests.txt`),
including the anti-replay hinge over an actual authenticated `ClientRequest`
(token minted for endpoint A, presented on a connection authenticated as B ->
deny) and the async trait impl on the wall clock.

## Decision

ADR-0001's embed choice is **validated in code, not just on paper**: the public
`AccessControl` seam carries Phases 1-2 with a zero-line upstream diff, on the
current released `iroh-relay 1.0.3`. No fork is needed for anything built so
far; the single upstream candidate remains the per-connection rate override.

## Version note

The adapter depends on `iroh-relay = "1.0.3"` (+ `server` feature). MSRV: 1.0.3
raises the effective toolchain floor well above `croft-admit`'s 1.75; the
embed crate tracks stable, while `croft-admit` stays low-MSRV and iroh-free.

## Still deferred (unchanged)

Live two-endpoint holepunch + Phase-3 bucket calibration (needs a running relay
and NAT traversal), Phase 4 (metrics/fuzz/deny-path), Phase 5 packaging.
