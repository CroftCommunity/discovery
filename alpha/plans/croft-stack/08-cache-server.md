# Phase 8 — Cache/index server, cache mode

← [07-auth-helper.md](07-auth-helper.md) · [roadmap](README.md) · next → [09-stellin-index.md](09-stellin-index.md)

**Status:** SCAFFOLD (fill on arrival) · **Depends-on:** Phase 7 (auth helper up, for arecipe's authed
paths) · **Gate-out:** a live pad reads through the cache, PDS load drops, and the pad is unaffected
when the cache is stopped.

---

## Scope for discussion (2026-07-29)

Phase 8 builds the **cache/index server** (Rust, extends `appview-validation`) and runs it in **cache
mode** for the live pads. Framing + the forks to decide before building:

**What it is.** One Rust program, a `StateSource` seam with `DemandCache` (Phase 8) and `FirehoseIndex`
(Phase 9) adapters; a shared XRPC/hydrated-read serve surface. In cache mode: demand-driven, TTL'd,
disposable, **no firehose** — a miss proxies the PDS / public AppView and caches the response. Holds
nothing canonical; degrade-to-serverless (pad `baseUrl` falls back to `public.api.bsky.app` / PDS).

**Targets + why.**
- **bluebird** first — reads `app.bsky.feed.getAuthorFeed` from `public.api.bsky.app`; its client
  already defaults `baseUrl` there, so pointing it at `bluebird-cache.croft.ing` is a one-line swap
  with a built-in fallback, and it needs **no auth broker** (public reads). Cleanest first target.
- **arecipe** next — its reads (`app.arecipe.*` / `exchange.recipe.*` from the PDS) are friends-scoped
  today; the cache accelerates + lightens the PDS. Cache under `cache.arecipe.app` (same-site). Its
  authed paths depend on the **Phase-7 auth broker** — so arecipe-cache is gated on Phase 7; bluebird is not.

**Forks to decide (the discussion):**
1. **Cache backing store:** in-memory (LRU, simplest, lost on restart — fine, disposable) vs on-disk
   SQLite (survives restart, `--disposable`). *Lean: in-memory LRU for cache mode (nothing canonical).*
2. **What to cache + TTL policy:** which read methods (getAuthorFeed / getPosts / getProfile / arecipe
   record reads), and per-method TTLs. Freshness vs hit-rate.
3. **CORS:** `bluebird-cache.croft.ing` is cross-origin to `bluebird.croft.ing` → needs CORS headers;
   `cache.arecipe.app` is same-site (simpler). Confirm the pads' expectations.
4. **Ordering vs Phase 7:** bluebird-cache can ship now (no auth); arecipe-cache waits on the broker.
   Do we ship bluebird-cache first, or hold both until Phase 7?
5. **Does it also serve iroh NodeId discovery reads** (the cohesion win — PDS record lookups cached)?
   In scope for cache mode or later?
6. **Reproducibility:** Rust binary cross-compiled for linux-x86_64, `get_url`+checksum deploy (like the
   relay), governed unit + Caddy vhost (like canary), TDD (`cargo test` + wiring), Ansible role.

**Not in Phase 8:** index mode / firehose (that's Phase 9, Stellin). Backups N/A (cache = disposable).

## Problem

The pads work serverless today. A passthrough cache lightens the PDS and improves latency for reads the
public AppView / PDS already answer — holding **nothing canonical**, degrading cleanly to serverless.

## Approach

Build the cache/index server as **one program with a `StateSource` seam** and **both** adapters
(`DemandCache`, `FirehoseIndex`), TDD. Deploy in **cache mode** for the pads. Cut both manifests;
validate **bluebird first** (simplest — no auth-helper dependency, `baseUrl` swap with built-in
fallback), then **arecipe** immediately after under its own domain.

## Steps (sketch — fill on arrival)
1. Build the serve surface (XRPC, hydrated views, `/healthz`) over the `StateSource` port. TDD.
2. `DemandCache` adapter: hit → serve; miss → proxy PDS / `public.api.bsky.app`, store w/ TTL. No
   firehose, nothing canonical.
3. Deploy `bluebird-cache` at `bluebird-cache.croft.ing`; point bluebird's `baseUrl` at it (one-line swap).
   Prove: cache hit path, miss→proxy path, and **pad still works with the cache stopped** (falls back
   to `public.api.bsky.app`).
4. Deploy `arecipe-cache` at `cache.arecipe.app` (same-site with the pad; uses the auth helper for
   authed paths). Same three proofs.
5. Also serves the **iroh NodeId discovery reads** (PDS record lookups) — the cohesion win.

## TODO (decide on arrival)
- [ ] TTL policy per read type; cache size/eviction (bounded, disposable).
- [ ] CORS headers for any cross-origin pad (bluebird-cache.croft.ing ← bluebird.croft.ing is
      cross-origin; arecipe is same-site).
- [ ] own-data API addendum scope (read-only, self-scoping by verified DID) — needed now or with index?

## Risks & cautions
- Must **never** become load-bearing: verify the pad-works-with-cache-off path explicitly for each pad.
- Cache staleness vs freshness: TTLs tuned so a stale hit is acceptable for the read type.
- `arecipe.app` cache depends on the auth helper (Phase 7) for authed paths — if Phase 7 slipped
  (pivot), do bluebird-only here.

## Validation
Per pad: hit + miss→proxy observed; PDS request volume drops; pad unaffected when the cache unit is
stopped.

## References
Roadmap → the cache/index server (two modes); `appview-validation/` (serve/index fragments);
`bluebird/src/atproto/client.ts` (injectable `baseUrl`), `arecipe/src/social/*`.
