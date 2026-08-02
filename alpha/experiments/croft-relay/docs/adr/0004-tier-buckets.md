# ADR-0004: Tiered per-connection rate buckets

- Status: Accepted (mapping); calibration DEFERRED (SPEC-DELTA)
- Date: 2026-08-02
- Phase: 3 (tiered per-connection rate buckets) — core only

## Context

The `tier` claim in the token must become enforcement. One credential, one
primitive (a token-bucket rate limiter), three product dials:

1. **Registered-only** — admitted at all (Phases 1/2).
2. **Coordination** — admitted, but metered so hard that holepunch
   coordination succeeds while sustained relayed media is starved.
3. **Broker** — generous or absent limits; the relay will carry media.

## Decision

### `tier` -> `RateBucket` is a pure mapping (`tier.rs`)

- `Coordination` -> a finite bucket (`bytes_per_second`, `max_burst_bytes`).
- `Broker` -> `RateBucket::UNLIMITED` (both fields `None`).

`RateBucket` mirrors iroh-relay's `[limits.client.rx]` shape. Mapping to the
real type (verified in `iroh-relay 1.0.0-rc.1`,
`server.rs:500`): `Coordination` becomes
`Some(ClientRateLimit { bytes_per_second: NonZeroU32, max_burst_bytes:
Option<NonZeroU32> })`; `Broker` becomes *no* `ClientRateLimit` attached to the
connection (fall through to the relay's global default / unlimited).

### Why volume, not content

Content-based splitting is impossible **by design**: the relay carries
encrypted frames and cannot distinguish disco (holepunch coordination) from app
data. Volume is the only honest proxy. This is a property to preserve, not a
limitation to fix — the moment the relay could tell coordination from media, it
could tell *anything* about the traffic, which the encryption exists to
prevent.

### Where the bucket is applied

Per ADR-0001, `Access::Allow` carries no rate limit, so v1 applies the bucket
in the embedding layer (wrap the admitted connection in a `Bucket`), and the
upstream candidate is `Access::Allow { rate_limit: Option<ClientRateLimit> }`.
The mapping in `tier.rs` is identical either way; only the application site
differs.

## SPEC-DELTA: the coordination numbers are placeholders, NOT measured

`grep SPEC-DELTA tier.rs`. The current constants —
`COORDINATION_BYTES_PER_SECOND = 4 KiB/s`, `COORDINATION_MAX_BURST_BYTES =
16 KiB` — are a reasoned placeholder, not a calibration:

- A holepunch disco exchange is a handful of small frames over a second or two;
  a few KiB/s with a small burst clears that while sitting one-to-two orders of
  magnitude below any usable audio/video bitrate (tens of KB/s and up).

The plan's Phase-3 calibration — instrument the harness, measure the bytes of a
*successful* holepunch coordination, set the bucket with headroom above that
and far below usable media bitrate — is **partially advanced, not closed**:

- **Done (RUN-CROFT-RELAY-03):** a live relay-client harness exists
  (`croft-relay-embed/tests/live_relay.rs`). It stands up a real relay gated by
  our admission and measures the app-payload a client pushes through the relay
  for a minimal contact round-trip (localhost datapoint: ~3 B/endpoint for a
  syn/ack, `evidence/live-relay.txt`). This confirms the sizing *direction* —
  the 4 KiB/s coordination bucket clears such an exchange ~1000x over while
  sustained media (>=24 kB/s) would exhaust it — and gives the measurement
  method.
- **Still open:** the figure above is a relay-client contact round-trip on
  localhost, **not** the full holepunch *disco* total, which requires two
  `iroh` magicsock endpoints attempting NAT traversal on separate networks.
  Until that is measured, the constant stays a placeholder.

Before any deployment, re-derive the number from a real holepunch and update
`tier.rs` + the pinned test
`coordination_bucket_has_the_calibrated_placeholder_values` together.

## Testing / mutation disposition

- `tests/phase3_tier.rs`: coordination is capped and its cap sits far below a
  media-bitrate floor; broker is unlimited; the burst clears a disco exchange.
- Inline unit test pins the exact placeholder magnitudes (the regression guard
  the calibration will edit).
- `cargo mutants`: no surviving mutant in `tier.rs`.

## Deferred (named, not built)

Priority scheduling between tiers, and pairwise `(src, dst)` policy in the
forwarding path, are explicitly out of scope for v1 (plan §1). The tier bucket
is per-connection and content-blind; it does not and must not read who is
talking to whom.
