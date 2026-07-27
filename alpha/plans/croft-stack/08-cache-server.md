# Phase 8 — Cache/index server, cache mode

← [07-auth-helper.md](07-auth-helper.md) · [roadmap](README.md) · next → [09-stellin-index.md](09-stellin-index.md)

**Status:** SCAFFOLD (fill on arrival) · **Depends-on:** Phase 7 (auth helper up, for arecipe's authed
paths) · **Gate-out:** a live pad reads through the cache, PDS load drops, and the pad is unaffected
when the cache is stopped.

---

## Problem

The pads work serverless today. A passthrough cache lightens the PDS and improves latency for reads the
public AppView / PDS already answer — holding **nothing canonical**, degrading cleanly to serverless.

## Approach

Build the cache/index server as **one program with a `StateSource` seam** and **both** adapters
(`DemandCache`, `FirehoseIndex`), TDD. Deploy in **cache mode** for the pads. Cut both manifests;
validate **skylite first** (simplest — no auth-helper dependency, `baseUrl` swap with built-in
fallback), then **arecipe** immediately after under its own domain.

## Steps (sketch — fill on arrival)
1. Build the serve surface (XRPC, hydrated views, `/healthz`) over the `StateSource` port. TDD.
2. `DemandCache` adapter: hit → serve; miss → proxy PDS / `public.api.bsky.app`, store w/ TTL. No
   firehose, nothing canonical.
3. Deploy `skylite-cache` at `skylite-cache.croft.ing`; point skylite's `baseUrl` at it (one-line swap).
   Prove: cache hit path, miss→proxy path, and **pad still works with the cache stopped** (falls back
   to `public.api.bsky.app`).
4. Deploy `arecipe-cache` at `cache.arecipe.app` (same-site with the pad; uses the auth helper for
   authed paths). Same three proofs.
5. Also serves the **iroh NodeId discovery reads** (PDS record lookups) — the cohesion win.

## TODO (decide on arrival)
- [ ] TTL policy per read type; cache size/eviction (bounded, disposable).
- [ ] CORS headers for any cross-origin pad (skylite-cache.croft.ing ← skylite.croft.ing is
      cross-origin; arecipe is same-site).
- [ ] own-data API addendum scope (read-only, self-scoping by verified DID) — needed now or with index?

## Risks & cautions
- Must **never** become load-bearing: verify the pad-works-with-cache-off path explicitly for each pad.
- Cache staleness vs freshness: TTLs tuned so a stale hit is acceptable for the read type.
- `arecipe.app` cache depends on the auth helper (Phase 7) for authed paths — if Phase 7 slipped
  (pivot), do skylite-only here.

## Validation
Per pad: hit + miss→proxy observed; PDS request volume drops; pad unaffected when the cache unit is
stopped.

## References
Roadmap → the cache/index server (two modes); `appview-validation/` (serve/index fragments);
`skylite/src/atproto/client.ts` (injectable `baseUrl`), `arecipe/src/social/*`.
