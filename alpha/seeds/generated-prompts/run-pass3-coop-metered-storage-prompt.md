# Prompt: run phase-plan Pass 3 on the cooperative metered-storage build plan

Copy this into a fresh session (after a clear). It is a **thin pointer** by design — the phase-plan
procedure lives in the `phase-plan` skill (`skills/phase-plan/pass3.md`) and the full reasoning lives in
the plan doc, which is the **single handoff artifact**. This prompt only carries the task-specific state
a fresh session can't otherwise know.

---

You're in the CroftC workspace (`/Users/cpettet/git/chasemp/CroftC`). We have a **phase-plan** for an
experimental product and Passes **1 and 2 are done**. **Run Pass 3 (quality gates).**

## Orient first (source of truth)

- `discovery/AGENTS.md` + `discovery/PLAYBOOK.md` — workspace orientation + filing discipline. Don't
  commit/push unless I ask (commit-as-you-go is fine for the plan doc; **no push**).
- The **`phase-plan` skill** — invoke it and load `skills/phase-plan/pass3.md`. Pass 3 is **analysis
  only** (no code). It *extends* the plan (adds quality gates), it does not rewrite Passes 1–2.

## The plan doc (the artifact to refine — read it end to end first)

`discovery/alpha/plans/2026-07-31-1-plan-coop-metered-storage-service.md`

It is the build plan for the **cooperative metered-storage service** — an experimental product: a
network-accessible, **custom PDS-like storage service in Rust** with **metering built in** (the
`item-storage-protocol` E0–E9 ledger: postage=bytes-transferred, rent=byte-days, co-signed
balance-forward statements, cost-priced audit dial, seal/tombstone/grace) and an **S3-compatible
interface**, deployed on the VPS via **croft-stack**. It also doubles as the substrate for the **MLS
history-convergence server** (one store, two consumers). Backlog: **ROADMAP_TODO E82**; advances **D5**.
Lane overview: `plans/2026-07-31-coop-storage-metered-hosting-lane.md`.

## What Pass 3 must apply (the quality gates — per `pass3.md`)

Walk the plan and layer these on, phase by phase; add to the Review Log for every change:

1. **TDD ordering** — confirm every implementation phase is RED-first: the wiring test named in each
   phase is written and failing before the code, GREEN at phase end. The plan already names a wiring
   test per phase (`tests/e*_*.rs`, `tests/wiring_s3_metered.rs`, `tests/wiring_pds_blob.rs`) and a
   deploy smoke test — verify the ordering is explicit and that unit tests never stand in for the
   wiring test (the anti-dead-code gate). rust-enforcer discipline applies (no `unwrap()` in prod,
   `Result`/`thiserror`, `Zeroize` on key material, doc comments, `clippy::pedantic`).
2. **Diagnostic / observability readiness** — the service **deploys under croft-stack telemetry +
   governed envelopes**, so check each phase declares the logging/metrics/diagnostics needed to debug it
   in place (esp. Phases 7–9: the metering byte-path, receipt/ledger writes, the HTTP boundary, and the
   telemetry poller integration at deploy). Metering is the product — under-observing the byte path is a
   real gap.
3. **Validation calibration** — the plan declares Narrow/Moderate/Broad per phase; sanity-check each
   against its risk (Phases 7–9 touch network/real-blob/real-VPS → Broad; confirm they say so and name
   the out-of-harness checks, e.g. `curl` put/get, `systemd-analyze security`, telemetry-within-envelope).
4. **Documentation-impact coverage** — the plan has a Documentation Impact section; confirm every
   add/rename/reference is scheduled **in the phase that makes it stale**, not a trailing docs phase
   (E82 status, ECOSYSTEM §5c-3 at deploy, the item-storage-protocol README cross-ref at Phase 1, the
   croft-stack service registration at Phase 9, COHESION §65).

## Confirmed decisions — do NOT re-litigate (Passes 1–2 settled these)

- **Boundary = both, from the start:** an S3-compatible interface as the storage+metering plane + a thin
  **atproto PDS blob-endpoint layer** on top (`getBlob`/`uploadBlob`). **Phase 8 is in v0.**
- **Two-layer split:** a deliberately-**dumb, pluggable backend** (Layer 1: FS first → Garage/SeaweedFS/R2;
  `BlobStore` trait) under a **boundary metering/provenance layer** (Layer 2: the E0–E9 ledger). The
  backend never does metering — provenance is the parties' keys + the customer's signed manifest.
- **Port oracle = the dependency-free `item-storage-protocol-standalone/`** (the 81/81 build), not the
  full version; module names `item/receipt/statement/clock/rng/pricing.ts`. `item-store` is a
  **standalone Rust crate** (no experiments-wide workspace) under `experiments/`.
- **Metering records live in per-user SQLite co-located with the user repo** (official-PDS pattern), as
  same-shaped signed records; **rollup + purge** (balance-forward close) bounds growth; manifest is
  single-author (in the repo), receipts/statements are bilateral co-signed (a structure alongside).
- **Two-mode transfer receipt** — `Unilateral` (provider-signed our-side measurement) | `Bilateral`
  (co-signed), **social-trust-layer-selected**; bilateral is the co-attested form the deferred capital
  layer will require.
- **E11–E14 funder-diligence deferred** (out of v0). **Repo/IP home + service name** are PHASE-GATED to
  Phase 9 (placeholder `item-store`; coop/storage name TBD = ROADMAP_TODO A21; Drystone stays the
  protocol). **legal-review (D5)** deferred to the coop-layer pile.

## Environment + git

- **No browser extension, but network is open** (`WebFetch`/`WebSearch`, Playwright available) — Phase 0
  discovery can fetch `rsky-pds` + atproto docs live; Pass 3 itself is analysis, no fetching needed.
- Branch **`claude/amble-coop-filing`** (HEAD ~`ba6caeb`), **not pushed**. This branch also carries the
  session's Amble-naming + coop-layer filing (behavior-scale cherry-picked onto current main; 3 distilled
  bodies; the lane + Pass 1/2 of this plan). **Branch cleanup is deferred to a comprehensive pass** —
  don't prune. Commit the Pass-3 plan-doc changes; **don't push** unless I ask.

## Output

Follow `pass3.md`: extend the plan, add a Pass-3 Review Log entry, walk me through any new open questions
one at a time (there should be none new — Passes 1–2 confirmed all six), and close out with the plan-file
path. Then we decide whether to execute (Phase 0 discovery first).
