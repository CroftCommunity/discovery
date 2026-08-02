# RUN-CROFT-RELAY-01 — atproto-gated iroh-relay admission: the app-side core

`Run summary, 2026-08-02. Branch claude/croft-relay-iroh-atproto-by891s. Rust 1.94.1.
Experiment lands at alpha/experiments/croft-relay/ (own workspace, one crate: croft-admit).
TDD, mutation-gated. Recon done against real iroh-relay 1.0.0-rc.1 source (read from the
crates.io registry). Honest boundary: github clone is blocked by egress policy this session, so
the live-relay legs (stand up the relay, prove A→B, holepunch calibration) were NOT run; what is
built is the relay-agnostic admission core, which needs zero relay changes for Phases 1-2.`

## HEADLINE

A compiling, tested, **mutation-clean** admission core for the three product
modes — registered-only, coordination tier, broker tier — built as the plan's
Phases 1-3 *app side*, with a grounded Phase-0 embed-vs-fork decision. **36
tests green, red-first; `cargo mutants` leaves 0 survivors (52: 45 caught, 7
unviable), including zero in every admission/token path** — meeting the plan's
strictest policy. One material upstream correction was found by reading source:
the HTTP-hook header is `X-Iroh-NodeId`, not the documented `X-Iroh-Endpoint-Id`.

## What ran, what it proves

| Phase | Claim proven (app-side) | Evidence |
|---|---|---|
| 0 | Embed (Option A) is feasible: `iroh-relay` exposes a public `AccessControl` trait (`Access::Allow/Deny`) over a cryptographically-authenticated endpoint id. Per-connection rate override is the one missing seam. | ADR-0001 (source-cited) |
| 1 | DID-control enrollment binds only on a matching PDS record; access check is deny-closed; axum `/access` answers the relay's `200`+`true` / `403`+`false` contract on the real header. | `phase1_access` (9), `phase1_http` (5) |
| 2 | Signed JWT/EdDSA token; three-gate verify (signature / expiry / bound-id); full deny matrix incl. anti-replay (`sub` must equal the authenticated endpoint), clock-skew boundaries, capability-not-nonce replay. | `phase2_token` (12) |
| 3 (core) | `tier` claim -> per-connection `RateBucket`; coordination capped far below media, broker unlimited. | `phase3_tier` (4) + unit |

Environment preflight: `rustc`/`cargo` 1.94.1; crates fetch via proxy OK;
`cargo-mutants` installed OK; **github.com 403 (egress policy)** — recon fell
back to registry source, live-relay legs deferred.

## Recon corrections (reality wins, per plan §2)

- **Header literal is `X-Iroh-NodeId`** (`iroh-relay/src/main.rs:35`), not the
  `X-Iroh-Endpoint-Id` the doc-comment and plan state. Handled in code
  (`http_api.rs` reads the real header + accepts the alias).
- iroh-relay also ships `Allowlist`/`Denylist` access modes (not just
  open/token/hook). Bearer auth in rc.1 is on the *hook request*, not a
  client-presented relay token.
- `Access::Allow` carries no rate limit → per-connection tiered buckets need
  an embedding-layer `Bucket` (v1) or the upstream `Access::Allow { rate_limit }`
  patch (the Phase-3/5 candidate). Confirmed as the single fork-worthy gap.

## Declared stand-ins / SPEC-DELTA

- `tier.rs` coordination bucket magnitudes are a **placeholder**
  (`SPEC-DELTA(phase-3-calibration)`), not a measured calibration — that needs
  a live holepunch the sandbox can't host. Pinned by a regression test that the
  calibration will edit.
- PDS is a `PdsResolver` trait with an in-memory fixture; the real async
  `com.atproto.repo.getRecord` adapter is named, not built.
- The `AccessControl` adapter (the only iroh-dependent code) is designed
  (ADR-0001) but not built — it is the Phase-2 embed step.

## Deferred (named): live-relay legs, Phase 4 (metrics/fuzz/deny-path), Phase 5
(upstream packaging: signed-token access mode + per-connection rate override).

## Owner calls open (OPEN-QUESTIONS.md, plan §7)

Q1 token format (defaulted JWT/EdDSA), Q2 repo shape, Q3 coordination hard-cap
(defaulted), Q4 Phase-1-first deploy (undecided), Q5 metrics cardinality
(defaulted tier-level). None block the core; all flagged for review.

## Where it lives

`alpha/experiments/croft-relay/` — `crates/croft-admit/` (code+tests),
`docs/adr/0001-0004`, `DESIGN.md`, `README.md`, `OPEN-QUESTIONS.md`,
`evidence/` (test + mutation output).
