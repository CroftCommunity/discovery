# Phase 9 — Stellin index mode (backups designed, paused)

← [08-cache-server.md](08-cache-server.md) · [roadmap](README.md) · next → [10-drystone-layer.md](10-drystone-layer.md)

**Status:** SCAFFOLD (fill on arrival) · **Depends-on:** Phase 8 (cache/index program built, both
adapters) · **Gate-out:** the index tenant serves a query no upstream can (discovery/search) from our
own index; the backup path is proven-out on non-canonical data but left off.

---

## Problem

Stellin (professional-network idea) is the one product whose value is **world-view discovery** and,
eventually, server-side search — precisely the queries a cache cannot answer, because no upstream
offers them. It still works without the index (friends-feed / Bluesky-direct), so the serverless floor
holds; the index just makes it good.

## Approach

Run the **same cache/index program in `--mode index`** for one tenant: ingest a filtered Jetstream for
the tenant's NSIDs → persistent SQLite index → serve. Wire viewer-aware serving via the proven
service-auth JWT verifier (RUN-14 EXP-A). Backups (Litestream→R2 for the cursor+index) are **wired but
paused** while data is experimental/non-canonical.

## Steps (sketch — fill on arrival)
1. Cut the index-tenant manifest: `--mode index`, filtered Jetstream (only the tenant's NSIDs — discard
   ~99.9% of the firehose), canonical data profile (cursor).
2. `FirehoseIndex` adapter serving the no-upstream queries (network-wide discovery / search).
3. Viewer-aware serving via `serviceauth.rs` (the real service-auth JWT verifier).
4. Wire Litestream→R2 for cursor+index but **leave it paused** (guarded no-op); prove restore on
   non-canonical data, then turn it back off.

## TODO (decide on arrival)
- [x] **Q5 — index tenant name/fqdn.** *RESOLVED:* `index.stellin.app` (its own domain, like
      arecipe.app; already live on the box via the auth-helper spike pad). Name clearance itself stays
      Open decision 6 (owner's legal call).
- [ ] Which NSIDs to filter (depends on the Stellin lexicon set — Open decision 6 / GROUPS/PUBLICATIONS).
- [ ] Stellin service DID / `aud`: `run14-A4` self-issued stand-in until a real `did:web:` + domain.
- [ ] Open decision 11: the exact trigger to enable backups (before real/canonical data).

## Risks & cautions
- **Baking the contested name** into DNS/lexicons before clearance (Open decision 6) — the naming
  caveat from Q1. Prefer a neutral name until cleared.
- Index mode holds canonical state (the cursor) — the moment real users arrive, backups must be ON
  (Open decision 11). Do not let real data land while paused.
- Firehose ingest is the resource-hungriest tenant — governance limits (Phase 3) matter most here.

## Validation
The index serves a discovery/search query that `public.api.bsky.app` / the PDS cannot; a restore from
R2 succeeds on non-canonical data; backups then confirmed off.

## References
`appview-validation/` (firehose/index/feed fragments; `serviceauth.rs`); `RUN-14-SUMMARY.md`;
Stellin name clearance `alpha/research/stellin-name-clearance-2026-07.md`; roadmap → index mode.
