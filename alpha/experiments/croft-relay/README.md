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

## What is built here

Two crates:

- **`crates/croft-admit`** — the *relay-agnostic* admission authority. **No
  `iroh` dependency**; the same logic serves the Phase-1 HTTP hook and the
  Phase-2 embedded verifier. (RUN-CROFT-RELAY-01)
- **`crates/croft-relay-embed`** — the **one** iroh-dependent crate: the
  `iroh_relay::server::AccessControl` adapter wiring `croft-admit` onto a real
  relay. (RUN-CROFT-RELAY-02)

| Plan phase | Built | Where |
|---|---|---|
| 0 — baseline | iroh cloned; `iroh-relay 1.0.3 --features server` built + tested (80 pass); embed-vs-fork validated in code | ADR-0001, ADR-0005 |
| 1 — admission | DID-bound enrollment (mocked PDS), deny-closed access check, axum `/access` service | `croft-admit`: `enroll`, `pds`, `registry`, `access`, `http_api` |
| 2 — signed tokens | JWT/EdDSA mint + three-gate verify, full deny matrix; **embed `AccessControl` adapter over real iroh types** | `croft-admit`: `token`; `croft-relay-embed` |
| 3 — tier buckets | `tier` -> `RateBucket` mapping (core); tier+bucket computed at admit | `croft-admit`: `tier`; `croft-relay-embed` |
| live leg | real relay on localhost gated by our admission; A→B relayed; deny/anti-replay over the wire | `croft-relay-embed`: `tests/live_relay.rs` |

- **46 tests**, red-first, all green: 36 in `croft-admit` + 7 embed-unit +
  3 live-relay (a real `iroh-relay 1.0.3` server on localhost gated by our
  `TokenAccess`, with real relay clients; incl. the anti-replay hinge over the
  actual relay handshake).
- **`cargo mutants`: 0 surviving mutants** across `croft-admit` (52: 45 caught,
  7 unviable) — the plan's no-survivor policy for admission/token paths, met.
- Clean `clippy` (all targets) and `cargo fmt --check`.
- Baseline: `iroh-relay 1.0.3` own test suite **80 pass / 0 fail**
  (`evidence/iroh-relay-baseline.txt`).

### Run it

```
cargo test                       # whole workspace (43)
cargo clippy --all-targets
cargo mutants -p croft-admit     # 0 survivors
```

## Phase 0 reconnaissance (grounded)

The plan's §2 claims were re-verified against **real source**, first
`iroh-relay 1.0.0-rc.1` from the registry (ADR-0001), then — once GitHub git
access opened mid-project — a clone of `main` at **1.0.3**, plus building and
running its test suite (ADR-0005). Headline findings:

- A public `AccessControl` trait (`Access::Allow`/`Deny`) makes **embed
  (Option A) feasible with a zero-line upstream diff** for Phases 1-2 — now
  *validated in code* by `croft-relay-embed`, not just on paper.
- The endpoint id in the access check is cryptographically authenticated — the
  hinge Phase 2's anti-replay gate stands on, exercised over real
  `ClientRequest` in the embed tests.
- **Correction (holds on 1.0.3):** the HTTP-hook header is actually
  `X-Iroh-NodeId`, not the `X-Iroh-Endpoint-Id` the docs/plan state. Handled in
  `http_api.rs`; a clean upstream doc-fix candidate.
- The one capability the library cannot express — per-connection rate override
  at admission (`Access::Allow` has no rate field) — is confirmed on 1.0.3 and
  is the minimal upstream candidate for Phase 3.

## Live leg (RUN-CROFT-RELAY-03) — done, localhost

`croft-relay-embed/tests/live_relay.rs` stands up a **real `iroh-relay` server
on localhost gated by our `TokenAccess`** and connects real relay clients:

- a valid croft-admit token is **admitted**; a bogus token and a token
  **replayed from another endpoint** are both **denied at the relay handshake**
  (the anti-replay hinge, now over the wire);
- **endpoint A's datagram reaches endpoint B through our gated relay**, payload
  and origin id intact;
- a byte-accounting of a minimal relayed contact round-trip, as a
  calibration datapoint (`evidence/live-relay.txt`).

## Deferred (named, not built) — what still remains

- **Real holepunch calibration:** the live leg above is a relay-client exchange
  on localhost, not the full holepunch *disco* total. Measuring that needs two
  `iroh` magicsock endpoints doing NAT traversal on separate networks. The
  coordination bucket stays a `SPEC-DELTA` placeholder until then (ADR-0004).
- **Phase 4/5:** hardening (deny-path cheapness, metrics, token-parser fuzz),
  and upstream packaging (signed-token access mode; per-connection rate
  override; the header doc-fix).

Done since RUN-01: the `AccessControl` embed adapter, the iroh-relay build+test
baseline (80/0), and the localhost live leg (relay + 2 clients through our gate).

## Decisions awaiting the human

See `OPEN-QUESTIONS.md` (plan §7): token format (defaulted JWT/EdDSA), repo
shape, coordination hard-cap stance, Phase-1-first deploy, metrics cardinality.

## Layout

```
Cargo.toml                     workspace
crates/croft-admit/            relay-agnostic admission authority (no iroh dep)
  src/{endpoint_id,did,pds,registry,enroll,access,http_api,token,tier}.rs
  tests/{common,phase1_access,phase1_http,phase2_token,phase3_tier}.rs
crates/croft-relay-embed/      the AccessControl adapter (the one iroh-dep crate)
  src/lib.rs   tests/embed.rs
docs/adr/0001..0005            decisions (embed-vs-fork, deny-closed, token,
                               tiers, phase-0 baseline + embed)
evidence/                      test + mutation + baseline output
OPEN-QUESTIONS.md              plan §7
```
