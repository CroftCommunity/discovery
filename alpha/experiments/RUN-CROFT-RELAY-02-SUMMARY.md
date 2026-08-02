# RUN-CROFT-RELAY-02 — Phase-0 baseline + the embed AccessControl adapter

`Run summary, 2026-08-02 (same-day follow-on to RUN-CROFT-RELAY-01). Branch
claude/croft-relay-iroh-atproto-by891s. Rust 1.94.1. Trigger: GitHub git access
opened mid-session (clone/fetch over HTTPS now succeed; web UI / REST API /
codeload still 403), unblocking the two legs RUN-01 had to defer for lack of a
clone: the iroh-relay build+test baseline, and the real AccessControl adapter.`

## HEADLINE

The embed decision (ADR-0001) is now **proven in code against real iroh**, not
just argued from source. Cloned `n0-computer/iroh`, built and ran
`iroh-relay 1.0.3 --features server` (**80 tests pass, 0 fail** — the recorded
Phase-0 baseline), and built `croft-relay-embed`: an
`iroh_relay::server::AccessControl` impl wiring `croft-admit`'s stateless token
verification onto a real, handshake-authenticated `ClientRequest`. **7 embed
tests green**, including the anti-replay hinge over actual iroh types. Workspace
total now **43 tests** (36 + 7); `croft-admit` mutation gate unchanged (0
survivors).

## What ran

| Leg | Result |
|---|---|
| Clone iroh (git now reachable) | `main`, `iroh-relay` = **1.0.3** (latest published) |
| Baseline: `cargo test -p iroh-relay --features server` | **80 pass / 0 fail** (`evidence/iroh-relay-baseline.txt`) |
| Re-verify ADR-0001 findings vs 1.0.3 | all hold (see below) |
| `croft-relay-embed` adapter + tests | **7 pass / 0 fail** (`evidence/embed-tests.txt`) |

## Re-verification vs 1.0.3 (reality still wins)

- **Header discrepancy persists on main.** `main.rs` sets the hook header from
  `const X_IROH_ENDPOINT_ID = "X-Iroh-NodeId"` while its doc-comment says
  `X-Iroh-Endpoint-Id`. Our `http_api.rs` already keys on the real name. Clean
  doc-fix PR candidate.
- **`Access` is still `Allow`/`Deny{reason}` with no rate field on `Allow`** —
  the per-connection-rate-override gap (ADR-0004) is confirmed on the released
  1.0.3, cementing it as the Phase-3/5 upstream candidate.
- New and useful: `ClientRequest::new(...)` and `ClientRequest::auth_token()`
  are public, so the adapter is testable without a live relay.

## The adapter (`crates/croft-relay-embed`, the one iroh-dependent crate)

- `TokenAccess::on_connect` = read `auth_token()` + authenticated
  `endpoint_id()` -> `croft_admit::TokenVerifier::verify` -> `Access`. Real
  clock only at this edge; the injected-clock `decide` underneath stays
  deterministic and is what the tests drive.
- `RegistryAccess` = the Phase-1 in-process equivalent of the HTTP hook.
- `EmbedDecision::Admit { tier, bucket }` computes the tier's bucket at admit
  time so the rate decision is tested and ready, even though `Access::Allow`
  can't carry it yet (applied via a stream-wrapping `Bucket` until the upstream
  change lands).

## Declared stand-ins / still deferred

- **Live two-endpoint holepunch + Phase-3 calibration** — the ONE remaining
  piece of Phase-0/Phase-3 acceptance. Needs a running relay + NAT traversal,
  beyond clone-and-test; not attempted. The coordination bucket stays a
  `SPEC-DELTA` placeholder.
- Phase 4 (metrics/fuzz/deny-path) and Phase 5 (upstream packaging) unchanged.
- Env note: GitHub web/REST/codeload remain 403; only the git protocol opened.

## Docs / ledger

ADR-0005 added (baseline + embed). README + MASTER-INDEX row + EXPERIMENT-
BACKLOG §6j updated (baseline + adapter marked done). Evidence:
`evidence/iroh-relay-baseline.txt`, `evidence/embed-tests.txt`.
