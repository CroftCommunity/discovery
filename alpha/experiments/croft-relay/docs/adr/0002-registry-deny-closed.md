# ADR-0002: Registry semantics and deny-closed admission

- Status: Accepted
- Date: 2026-08-02
- Phase: 1 (admission service)

## Context

Phase 1 makes the stock relay's HTTP-hook do our bidding with zero relay
changes: only endpoints bound to a controlling DID may attach. Two questions
need settling: what the registry *means*, and how admission behaves under
doubt.

## Decision

### Registry = `EndpointId -> Did` bindings, written only on proven control

- The registry (`registry.rs`) maps an authenticated `EndpointId` to the `Did`
  that controls it. Keyed by `EndpointId` because that is what the relay hands
  the access check; the `Did` is retained for audit and for minting tokens in
  Phase 2.
- A binding is written **only** by `enroll::verify_and_bind`, and only after
  the DID's PDS repo is shown to publish that endpoint in its
  `ing.croft.iroh.endpoint` / rkey `self` record (`pds.rs`). Fetching that
  record requires control of the DID's repo, so publication is the proof of
  control. We do not challenge the endpoint separately — the relay's
  cryptographic attach already proves key possession at connect time.
- Re-enrollment is idempotent (overwrites the same endpoint's binding).

### Deny-closed is the invariant

Every non-affirmative outcome denies and writes nothing:

- PDS `NotFound` / `Timeout` / `Malformed` -> `EnrollError::PdsUnavailable`,
  no binding. Absence of a "yes" is a "no"; a PDS outage must never fail open.
- PDS record names a *different* endpoint -> `EnrollError::EndpointMismatch`,
  no binding.
- At the access check (`access.rs`, `http_api.rs`): missing header, malformed
  header (not hex / wrong length), or a well-formed-but-unenrolled endpoint all
  return deny (`403` + `false`). There is no code path that admits on
  uncertainty; the decision is a total function with an exhaustive match, not a
  chain of early `true` returns.

## Rationale

The relay is the enforcement point for *admission*; the app layer enforces
pairwise call policy. An admission control that fails open under load or PDS
flakiness would silently become "open relay for a moment," which is exactly the
failure the registered-only mode exists to prevent. Deny-closed keeps the
security property monotone: doubt never widens access.

## Testing / mutation disposition

- `tests/phase1_access.rs` and `tests/phase1_http.rs` cover allow, deny,
  mismatch, PDS-timeout-denies-closed, PDS-not-found, missing/malformed header,
  and the exact `200`+`true` / `403`+`false` HTTP shape (incl. the real
  `X-Iroh-NodeId` header and its documented alias).
- `cargo mutants` leaves **no surviving mutant** in `access.rs`, `enroll.rs`,
  or `registry.rs` (see `evidence/mutants-full.txt`).

## Seam left open

Registry v1 is in-memory. Durability, and a firehose-driven invalidation story
(the plan's explicitly-deferred item), are not built. Phase 2 deliberately
moves the per-connection decision off the registry and onto stateless token
verification, so registry durability is an enrollment-authority concern, not an
attach-path one.
