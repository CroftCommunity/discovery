# item-store — graduated to CroftCommunity/CISS (2026-08-03)

This crate (the Rust port of the `item-storage-protocol` — the E0–E9 metering
ledger core, plus the Phase-7 S3-compatible metered boundary) **graduated into
its own product repo** after Phase 7:

- **Repo:** `CroftCommunity/CISS` — **CISS, the Croft Item Storage Server**
  (a PDS+ metered-storage server). Crate/binary renamed `item-store` → `ciss`.
- **Why here is now a pointer:** the crate is a deployable product headed for a
  croft-stack VPS deploy, so it owns its own repo (one source of truth). The
  E0–E9 per-phase commits remain in this repo's git history.

Thinking/provenance stays in `discovery`:

- Build plan (reasoning, per-phase design, decisions, per-phase SHAs):
  `discovery/alpha/plans/2026-07-31-1-plan-coop-metered-storage-service.md`
- Lane overview: `discovery/alpha/plans/2026-07-31-coop-storage-metered-hosting-lane.md`
- Backlog: `ROADMAP_TODO.md` E82 (lane), E83–E87 (tracked post-v0 threads).

Phases 8 (atproto PDS blob API) and 9 (croft-stack VPS deploy) continue in CISS.
