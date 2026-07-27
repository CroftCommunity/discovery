# Phase 0 — Agree the model + lock the manifests

← [roadmap](README.md) · next → [01-extract-croft-stack.md](01-extract-croft-stack.md)

**Status:** **COMPLETE — gate met (2026-07-24).** · **Depends-on:** nothing · **Gate-out (MET):** the
concrete `services/*.toml` set is agreed on paper (names, fqdns, ports, modes, data profiles, limits),
and the model is confirmed. Q1–Q6 all resolved.

No box work in this phase. It produces the paper artifact every later phase consumes: the manifest set.

---

## Problem

Before any `services/<name>.toml` is cut, the model and the concrete manifest set must be agreed —
otherwise the generator emits units/vhosts/ports for services whose names, domains, and roles are still
in flux, and the reorder churns everything downstream (DNS, Caddy, the auth `client_id`, ports).

## Approach

Confirm the model (below, all confirmed), then pin the concrete manifest set. Walk the remaining naming
questions one at a time; record each as it lands. The gate is a manifest table nobody needs to
re-litigate.

## Model — confirmed

- **Optional-accelerator ethos** — serverless is the floor; every accelerator is independently
  removable; minimal held state; mini-stack per thing; governed by default. (roadmap → Design ethos)
- **One program, two modes** — the cache/index server is one program with a `StateSource` seam
  (`--mode cache` | `--mode index`).
- **Auth helper is shared** — one deployment (`account.croft.ing`) for the estate.
- **iroh relay is the first real service** — off-the-shelf, connectivity-only, infra shakedown.
- **Governance + telemetry first-class** — limits + accounting on every unit from the first.
- **Declarative from the start** — OpenTofu (resource boundary) + generator/bootstrap (in-box).

## Decisions locked this phase

| # | Decision | Value |
|---|---|---|
| Q1 | Naming scheme | role-based subdomains; croft-infra + croft-domain pads under `croft.ing`; a pad on its own domain gets its cache under that domain |
| Q2 | First cache pad | both cut in Phase 8; **skylite first** (`skylite-cache.croft.ing`), **arecipe next** (`cache.arecipe.app`) |
| Q3 | Production repo name | `croft-stack` (`CroftCommunity/croft-stack`) |
| — | Auth helper fqdn | `account.croft.ing` (owner prefers "account" over an "auth"-named domain) |
| — | Relay ordering | first real service (Phase 6), before the pad accelerators |
| — | Declarative tooling | OpenTofu (resource boundary only) |

## The manifest set (draft — the paper artifact this phase produces)

Ports — *explicit scheme confirmed (Q6)*, all localhost-only behind Caddy: `8001` auth helper · `8100`
canary · `8101`/`8102` skylite-cache (main/api) · `8103`/`8104` arecipe-cache · `8201`/`8202`
stellin-index; the relay uses its own UDP/QUIC port. The generator still enforces no collision.

| Manifest (`services/<name>.toml`) | fqdn | Port | Mode / kind | Data profile | Phase |
|---|---|---|---|---|---|
| `canary` | `canary.croft.ing` | 8100 | contract canary — bring-up vehicle **and permanent** health/smoke target | disposable | 4 |
| `iroh-relay` | `relay.croft.ing` | UDP/QUIC (+TLS) | off-the-shelf relay | disposable | 6 |
| `account` (auth helper) | `account.croft.ing` | 8001 | confidential OAuth broker | **canonical** (keys/sessions) | 7 |
| `skylite-cache` | `skylite-cache.croft.ing` | 8101 | `--mode cache` | disposable | 8 |
| `skylite-cache-api` | (same, path/host) | 8102 | own-data API (read-only) | — | 8 |
| `arecipe-cache` | `cache.arecipe.app` | 8103 | `--mode cache` | disposable | 8 |
| `arecipe-cache-api` | (same) | 8104 | own-data API (read-only) | — | 8 |
| `stellin-index` | `index.stellin.app` | 8201 | `--mode index` | **canonical** (cursor) | 9 |
| `stellin-index-api` | (same host) | 8202 | own-data API (read-only) | — | 9 |

## Phase-0 questions — status

- **Q4 — stub tenant name/fqdn.** *RESOLVED:* `canary.croft.ing` — and it is a **permanent** infra
  canary (health/smoke target for the external pinger + an always-on contract smoke test), not torn down.
- **Q5 — Stellin index tenant name.** *RESOLVED:* `index.stellin.app` (its own domain, like arecipe.app;
  keeps the contested name off croft.ing). The name-clearance itself stays Open decision 6 (owner's
  legal call); it is already in active use on `stellin.app` via the auth-helper spike pad.
- **Q6 — port scheme.** *RESOLVED:* explicit scheme confirmed — `8001` auth · `8100` canary ·
  `8101`/`8102` skylite-cache · `8103`/`8104` arecipe-cache · `8201`/`8202` stellin-index; relay on its
  own UDP/QUIC port. Hand-pinned for predictable telemetry/debugging; generator enforces no collision.

## Reasoning

- **Manifests-on-paper before code** matches the kit's "the manifest is the single source of truth"
  model — the generator is deterministic, so an agreed manifest set fully determines units, vhosts, and
  timers. Getting names/ports right here is cheap; getting them wrong after generation churns DNS,
  certs, and the auth `client_id`.
- **Neutral stub + (maybe) neutral index name** keep the uncleared "Stellin" name out of DNS until it
  clears, per Open decision 6 — the one place the role-based scheme (Q1) needs an escape hatch.

## Risks & cautions

- The role-based naming scheme is convenient but **bakes product names into DNS** — the Stellin-name
  collision (Q5) is the concrete instance to resolve before Phase 9.
- Ports are cheap to change now, expensive after certs/DNS/clients bind to them — confirm Q6 here.

## Validation

Gate-out is a paper artifact, not a run: the manifest table above with Q4–Q6 resolved and no `<TBD>`.

## References

- Roadmap: [README.md](README.md) · kit manifests: `alpha/experiments/appview-infra/kit/services/*.toml`
  and the generator `scripts/render.py`.
