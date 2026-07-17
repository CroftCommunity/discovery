# appview-infra

A generic multi-tenant hosting kit for the Croft portfolio's small always-on
services: one OVH VPS, single static binary + SQLite per tenant, HTTPS via Caddy,
Litestream-to-R2 for canonical state, rclone-to-R2 for blobs, Porkbun for DNS.

Manifests (`services/*.toml`) + the generator (`scripts/render.sh`) are the
single source of truth; everything under `generated/` is a build product.
`make check` is the whole gate. See `CONTRACT.md` for the service contract,
`docs/RUNBOOK.md` for operations, and `docs/EXTRACTION.md` for how this repo was
produced.

## Quick start

```
make generate     # manifests -> generated/ (systemd, Caddy, litestream, rclone)
make check        # the full gate (hygiene, generator, backup-audit, drill, ...)
make local-up     # bring the whole stack up on localhost (no credentials)
make local-drill  # destroy + restore + assert, end to end, locally
```

## Provenance

This repo was **extracted from `CroftCommunity/discovery`** (the design corpus)
by `scripts/extract-to-repo.sh`. The design material that produced it —
`GROUPS.md` (the large-group-tier brief), the RUN-15 summary, and the spec-facing
notes — **stays in discovery** and is not copied here; see `PROVENANCE.md` for
the exact source commit and pointers back.
