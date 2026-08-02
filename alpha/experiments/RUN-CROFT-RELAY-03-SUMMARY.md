# RUN-CROFT-RELAY-03 — the local live leg: a real relay gated by our admission

`Run summary, 2026-08-02 (follow-on to RUN-02). Branch
claude/croft-relay-iroh-atproto-by891s. Rust 1.94.1. Scope: the "live" half of
Phase-0/Phase-3 acceptance, done on localhost — a real iroh-relay Server gated
by our TokenAccess, real relay Clients through it, and a relayed A->B exchange
with a byte datapoint for calibration.`

## HEADLINE

Our admission gate now runs **inside a real relay**. `croft-relay-embed/tests/
live_relay.rs` spawns an `iroh_relay::server::Server` on `127.0.0.1:0` with our
`TokenAccess` as its `AccessControl`, then drives real
`iroh_relay::client::Client`s against it. **3 live tests green**: a valid
croft-admit JWT is admitted and carries traffic; a bogus token and a token
**replayed from a different endpoint** are both denied at the relay handshake;
and **endpoint A's datagram reaches endpoint B through our gated relay**,
payload + origin intact. Workspace total now **46 tests**.

## What ran (all over the real relay wire)

| Test | Proves |
|---|---|
| `valid_token_admits_and_bogus_or_expired_is_denied` | our `TokenVerifier` gates a live handshake: valid admits + pings; bogus denied; A's token replayed from endpoint C denied (anti-replay over the wire) |
| `endpoint_a_reaches_endpoint_b_through_our_relay` | A `Datagrams` to B's endpoint id is forwarded by our gated relay; B receives it tagged with A's id, contents byte-identical |
| `coordination_bytes_datapoint_for_calibration` | a minimal relayed contact round-trip; app-payload pushed per endpoint measured and asserted to fit the coordination bucket |

Wiring mirrors iroh-relay's own `tests/runtime_auth.rs`
(`RelayConfig::new((LOCALHOST,0))` + `relay.access = Arc<TokenAccess>` +
`Server::spawn`; clients via `ClientBuilder...auth_token(tok).connect()`, the
token riding as `Authorization: Bearer` straight into our verifier).

## Calibration datapoint (honest scope)

`CALIBRATION-DATAPOINT relayed_contact_roundtrip a_to_relay=3B b_to_relay=3B
coord_bucket=4096B/s` (`evidence/live-relay.txt`). This is a **localhost
relay-client** figure for a single syn/ack contact round-trip: it confirms the
sizing *direction* (4 KiB/s clears such an exchange ~1000x over; sustained media
>=24 kB/s would exhaust it) and gives the measurement method. It is **not** the
full holepunch *disco* total — that needs two `iroh` magicsock endpoints doing
NAT traversal on separate networks. The `SPEC-DELTA(phase-3-calibration)`
placeholder therefore **stays**; ADR-0004 updated to record the harness + the
datapoint + what remains.

## Still deferred

- Real holepunch-disco calibration on two NAT'd iroh endpoints (the only piece
  of Phase-3 still open).
- Phase 4 (metrics/fuzz/deny-path), Phase 5 (upstream packaging).

## Docs / ledger

ADR-0004 calibration section updated. README live-leg section + counts (46).
`evidence/live-relay.txt` added. MASTER-INDEX row + EXPERIMENT-BACKLOG §6j
updated (live two-endpoint leg -> DONE local; calibration harness exists).
