# appview-infra — experiment summary

**Goal.** Build and fully validate a generic multi-tenant hosting kit for the
Croft portfolio's small always-on services (single static binary, SQLite,
optional ingest, HTTPS) targeting one OVH VPS with Litestream-to-R2 backup and
Porkbun DNS — designed in discovery, staged for production, spending nothing.

**Approach.** Manifests (`kit/services/*.toml`) + a generator as the single
source of truth for systemd units, Caddy vhosts, `litestream.yml`, and rclone
timers. A shared contract stub stands in for real binaries so the whole kit, an
own-data API sidecar, an access-gated large-group tier, and a full local
destroy→restore fire drill all run with zero credentials. TDD red-first per
deliverable (`test(infra):` before `feat(infra):`); `make check` is the gate.
The `kit/` subtree is written as the future root of
`CroftCommunity/appview-infra` and extracted by D15; design corpus
(`GROUPS.md`, this summary, spec notes) stays in discovery.

**Effort.** 16 deliverables (D1–D16), 32 red/green commits, one session. All
tooling (bats, shellcheck, terraform, sqlite3, rclone, litestream, caddy,
actionlint, go, python3) installed and exercised; only `terraform validate` is
BLOCKED (provider registry egress).

**Result.** Phase 1 complete and green. `make check` passes end to end,
including the capstone fire drill: every tenant's stub + api brought up on
localhost from the generated config, canonical + blob markers planted, all local
state destroyed, restored from a file:// litestream replica + a local rclone
dir, restarted, and the full per-tenant assertion loop (healthz, canonical
marker, blob marker, api self-scoping, gated-group member/non-member) green. The
extracted tree passes `make check` standalone. Seven honest stand-ins are
registered. The group write-path fork, the scale boundary, and the OVH ordering
are surfaced as owner decisions; nothing was purchased.

Full detail: `../RUN-15-SUMMARY.md`. Design brief: `GROUPS.md`. Kit: `kit/`.
