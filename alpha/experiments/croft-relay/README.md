# croft-relay — atproto-gated admission for an iroh relay

An experiment building the app-side enforcement core for a calling system whose
identity lives in atproto and whose transport is iroh. The relay is the
enforcement point for *admission*; the app layer enforces pairwise call policy.
The relay never learns call content and never holds social-graph state.

Three product modes, as discrete dials on one credential:

1. **Registered-only reception** — only endpoints holding a valid,
   DID-bound credential may attach.
2. **Coordination tier** — admitted but rate-limited so hard that holepunch
   coordination works while sustained relayed media is starved.
3. **Full-broker tier** — generous or absent limits.

Full narrative and phasing: the build plan (in the run summary and the ADRs).

## What is built here (RUN-CROFT-RELAY-01)

A compiling, tested, mutation-checked crate: **`crates/croft-admit`** — the
*relay-agnostic* admission authority. It has **no `iroh` dependency**; the same
logic serves the Phase-1 HTTP hook and a Phase-2 embedded verifier.

| Plan phase | Built | Module |
|---|---|---|
| 1 — admission | DID-bound enrollment (mocked PDS), deny-closed access check, axum `/access` service | `enroll`, `pds`, `registry`, `access`, `http_api` |
| 2 — signed tokens | JWT/EdDSA mint + three-gate verify, full deny matrix | `token` |
| 3 — tier buckets | `tier` -> `RateBucket` mapping (core) | `tier` |

- **36 tests** (unit + `phase1_access` + `phase1_http` + `phase2_token` +
  `phase3_tier`), red-first discipline, all green.
- **`cargo mutants`: 0 surviving mutants** across the crate (52 mutants: 45
  caught, 7 unviable). The plan's policy — no surviving mutant in admission or
  token-verification paths — is met, and then some. Evidence:
  `evidence/mutants-full.txt`, `evidence/green-tests.txt`.
- Clean `clippy` (all targets) and `cargo fmt --check`.

### Run it

```
cargo test    -p croft-admit
cargo clippy  -p croft-admit --all-targets
cargo mutants -p croft-admit
```

## Phase 0 reconnaissance (grounded, with one honest gap)

The plan's §2 claims about upstream were re-verified against **real source** —
`iroh-relay 1.0.0-rc.1` read from the crates.io registry — not trusted from the
doc. Findings and corrections are in **ADR-0001**; the headline ones:

- A public `AccessControl` trait (`Access::Allow`/`Deny`) makes **embed
  (Option A) feasible with a zero-line upstream diff** for Phases 1-2.
- The endpoint id in the access check is cryptographically authenticated — the
  hinge Phase 2's anti-replay gate stands on.
- **Correction:** the HTTP-hook header is actually `X-Iroh-NodeId`, not the
  `X-Iroh-Endpoint-Id` the docs/plan state. Handled in `http_api.rs`.
- The one capability the library cannot express — per-connection rate override
  at admission — is the minimal upstream candidate for Phase 3.

**The github clone was blocked by egress policy this session**, so the plan's
Phase-0 acceptance ("run the stock relay in all three modes; prove endpoint A
reaches B via our relay") was **not** performed. Reconnaissance was done against
the pinned crate source instead. This is the honest boundary of the run.

## Deferred (named, not built) — the seams for the next session

- **Live relay legs:** stand up `iroh-relay --features server` in each access
  mode; prove A→B through it; wire `croft-admit`'s `/access` as the hook
  (Phase-0/Phase-1 acceptance). Needs network + multi-process sandbox.
- **The `AccessControl` adapter** (the only iroh-dependent code) that hands a
  verified token's decision to the relay (Phase-2 embed).
- **Phase-3 calibration:** measure a real holepunch coordination exchange and
  re-derive the coordination bucket. Current numbers are a `SPEC-DELTA`
  placeholder (ADR-0004).
- **Phase 4/5:** hardening (deny-path cheapness, metrics, token-parser fuzz),
  and upstream packaging (signed-token access mode; per-connection rate
  override).

## Decisions awaiting the human

See `OPEN-QUESTIONS.md` (plan §7): token format (defaulted JWT/EdDSA), repo
shape, coordination hard-cap stance, Phase-1-first deploy, metrics cardinality.

## Layout

```
Cargo.toml                     workspace
crates/croft-admit/            the built crate
  src/{endpoint_id,did,pds,registry,enroll,access,http_api,token,tier}.rs
  tests/{common,phase1_access,phase1_http,phase2_token,phase3_tier}.rs
docs/adr/0001..0004            decisions (embed-vs-fork, deny-closed, token, tiers)
evidence/                      test + mutation output
OPEN-QUESTIONS.md              plan §7
```
