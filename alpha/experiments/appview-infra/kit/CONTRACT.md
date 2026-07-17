# Service contract

Every artifact the kit hosts — the stub today, real service binaries later — MUST
satisfy this contract. The generator, the local rehearsal, and the fire-drill all
assume it, so a binary that honours it drops in with no kit changes.

A shared stub (`stub/`) implements the whole contract plus every addendum below,
which is why the kit, the local rehearsal (D13), and the drill run before any real
binary exists. The stub's identity verifier is a **stand-in** for the atproto
service-auth JWT verifier proven in RUN-14 EXP-A — same interface, swapped at the
seam. That stand-in is registered (`SPEC-DELTA[run15-stub-verifier | stand-in]`).

## Core contract (every tenant)

1. **Flags.** The binary accepts `--data-dir <path>` and `--listen <host:port>`.
   Nothing else is required to start.
2. **Health.** `GET /healthz` returns `200` with body `ok` once the process is
   ready to serve (data dir opened, listener bound). Before ready, connections may
   be refused; once ready, `/healthz` is fast and side-effect-free.
3. **All state under the data dir.** The process writes nothing outside
   `--data-dir` (no `$HOME`, no `/tmp` state, no cwd files). This is what lets the
   generated systemd unit set `ProtectSystem=strict` with a single
   `ReadWritePaths=` on the data dir, and what lets the drill destroy and restore
   state by touching one directory.
4. **No root.** The process runs as an unprivileged, dedicated user and never
   needs to escalate. `NoNewPrivileges=yes` in the unit must not break it.
5. **No low ports.** `--listen` is always a port ≥ 1024; TLS termination and
   :443 are Caddy's job, not the service's.

## Data-profile addendum (state taxonomy, §1.2)

The binary is told its data profile so it can create/own its files under the data
dir. The stub takes them as flags; a real binary may hard-code its own layout as
long as the paths match its manifest's `data_profile`:

- `--canonical <relpath>` (repeatable): observation/grant-born state nothing on the
  network can rebuild (cursors, telemetry, grants, rosters, server-held keys).
  SQLite only, so Litestream can stream it. **Backed up.**
- `--disposable <relpath>` (repeatable): projections rebuildable from
  firehose/backfill/sealed history. **NEVER backed up** (D5 enforces).
- `--blobs <reldir>` (repeatable): opaque bytes the service stores and distributes
  without reading. Mirrored by rclone, not Litestream.

On start the binary MUST create every declared path (an empty initialized SQLite
file for canonical/disposable; an empty dir for blobs) so the drill can assert
their existence and plant/restore markers.

## Own-data API addendum (`serve_api` tenants, §1.3)

The API is a **separate process** (`<name>-api`) serving each user only their own
observation/grant-born state. It is self-scoping and containment-hardened:

- **Read-only opens.** Every SQLite connection is opened read-only
  (`mode=ro`); the process is additionally `ReadOnlyPaths=` on the data dir at the
  OS level, so it is *incapable* of writing even if the code tried.
- **Self-scoping.** The verified caller DID is the only subject. Every query is
  implicitly `subject_did = <caller>`; there is no parameter that widens it.
  `getMyRows` returns only the caller's rows; caller A can never see caller B's.
- **Statement timeout.** A per-statement timeout bounds any single read so a slow
  or hostile query cannot pin the WAL; it returns `503`/timeout, never hangs.
- **Pagination limits.** Bulk `export` streams in bounded pages (short
  transactions); no unbounded full-table scan in one transaction.

Two API modes (manifest `api_mode`):
- `shared-wal`: reads the live SQLite files read-only, concurrent with the writer
  (WAL permits it). The "replica" is free.
- `snapshot`: serves a periodically `VACUUM INTO`-produced snapshot under
  `serve/` (classed **disposable**); zero writer interaction, minutes of staleness.

## Gated-groups addendum (`gated_groups` tenants, §1.4)

Large groups (past `group_scale_boundary`, working number 5000 — a parameter, not
this kit's decision) serve **private but not E2EE**: the AppView reads content and
gates *offering* it by verified roster membership. This is the honest posture, not
a leak — past that scale any single member can leak, so cryptographic
confidentiality is a mirage; the kit states the trusted-gatekeeper posture plainly.

Contract for such tenants, behind a `GroupStore` interface both write-path variants
(D11) can implement:

- **Verifier interface.** A caller identity is established by the same verifier
  used for the API (stubbed here, real service-auth in RUN-14 EXP-A). No verified
  identity → `401`.
- **Roster gate.** `content(group, cursor)` is served only to a verified DID in
  `roster(group)`. A verified non-member → `403`; an anonymous caller → `401`; a
  member removed from the roster → `403` on the next call.
- **Roster and grants are canonical.** They are observation/grant-born; the
  manifest classes them canonical and D5 enforces a backup target exists.
- The offering-vs-reading distinction (RUN-14 EXP-B) does **not** apply to this
  tier: the server reads by design. The honest posture is the feature.
