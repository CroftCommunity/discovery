# R2 bucket topology

## Default: one bucket, per-service prefixes, one scoped token

The generator lays every tenant's backups into a single R2 bucket under a
per-service prefix:

```
<bucket>/<service>/state      # litestream replicas of canonical sqlite
<bucket>/<service>/<blobdir>   # rclone mirror of each blob dir
```

Concretely, with the staging bucket `croft-appview-staging`:

```
croft-appview-staging/stellin-appview/state
croft-appview-staging/stellin-appview/blobs
croft-appview-staging/croft-groups/state
croft-appview-staging/croft-groups/blobs
```

The bucket name is a non-secret render variable (`RENDER_BUCKET`, staging
default `croft-appview-staging`); endpoints and credentials come only from the
environment (guardrail 4). One R2 free tier (10 GB, 1M writes/mo) covers this
scale; Phase 1.5 asserts usage stays inside it.

## The tradeoff (OWNER call — guardrail 5)

**One bucket + one scoped token (default).** Simplest to provision and rotate:
one Cloudflare R2 API token, scoped to the one bucket. The cost is blast radius
— **one leaked token exposes every tenant's canonical state** (all
`<service>/state` prefixes), because R2 token scoping is per-bucket, not
per-prefix. Per-service prefixes give clean separation for restores and quota
accounting, but they are **not** a security boundary: a bucket-scoped token can
read every prefix.

**Per-service buckets + per-service tokens (the isolation upgrade).** Each
tenant gets its own bucket and its own token. A leaked token then exposes only
that one tenant. The cost is more provisioning and rotation surface (N buckets,
N tokens, N sets of env credentials), and the free tier's per-bucket limits are
counted separately. The generator supports this with no code change — set
`RENDER_BUCKET` per render, or extend the manifest with a per-service bucket
field — but the default stays one bucket until the owner asks otherwise.

## Decision requested

- Stay on one bucket + one scoped token for staging and first production, or
  move to per-service buckets now?

Recommended: **one bucket for staging** (cheapest, simplest, and the blast
radius is bounded to *our own* tenants pre-launch), revisit at the first tenant
that carries third-party canonical state. Recorded as an owner decision; the kit
does not decide it.
