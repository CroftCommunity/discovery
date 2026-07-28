# Plan: croft-stack telemetry client (Phase 3)

date: 2026-07-28 · phase-plan (3 passes, combined) · component of
[03-governance-telemetry.md](03-governance-telemetry.md).
**Status: BUILT (2026-07-28).** All 4 phases executed TDD, committed per phase in `croft-stack`
(`43d4830`→`3c589b3`→`6437a84`→`73c25e5`); 32 pytest + 6 bats green; validated on the box's real
cgroups. Session: `croft-stack/sessions/2026-07-28-phase-3-telemetry.md`.

The Python, stdlib-only, cgroup-v2 per-process resource reader + query CLI, deployed on the box as a
systemd timer. Code lives in `croft-stack/telemetry/`; deployed by the Phase-4 Ansible converge.

---

## Problem Statement

Every croft-stack unit runs in its own systemd cgroup (Phase 3 governance made accounting default), so
the kernel exposes each unit's live resource use as files under
`/sys/fs/cgroup/system.slice/<unit>.service/`. We need a **local** way to read those per-unit and keep a
short history, so we can see whether a service is near its limits or starving a neighbour (the E5/E6
governance bet) — **without** a metrics stack (Prometheus/cAdvisor are an explicit non-goal). Constraints:
stdlib-only (runs on the box's system python, no pip deps, reconstructible from the repo alone), minimal
held state, TDD, hexagonal (ports/adapters) so the cgroup filesystem and the store are swappable and the
core is testable without a live box.

## Reasoning

**Approach — hexagonal, stdlib-only.** A pure core (parse cgroup file contents → a `Sample`), two ports
(`CgroupSource` reads raw per-unit file contents; `SampleStore` persists/queries samples), two real
adapters (`FilesystemCgroupSource` over `/sys/fs/cgroup`, `SqliteSampleStore`), and a thin `argparse`
CLI (`poll`, `show`). The core has no I/O, so it is fully testable from fixtures; the box is only needed
for the filesystem adapter's one integration check. This mirrors the pads' Transport-port style and the
`effective-design-overview`/`hexagonal-architecture` standards.

**Why stdlib-only.** Verified: the box has python 3.13 + `sqlite3` 3.46. `argparse` + `sqlite3` +
`pathlib` cover everything. Zero pip deps means the client deploys as plain files and the box is
reconstructible from `croft-stack` alone — no venv, no wheels. `pytest` is a dev-time dependency for the
tests only; the shipped client imports nothing outside stdlib.

**Why a self-discovering poll (glob), not a configured unit list.** The poller globs
`system.slice/*.service` and records whatever it finds. This needs no coupling to the manifests/ports
and self-adjusts as tenants are added/removed — code with no data to maintain. `show` filters by unit at
query time.

**Why defensive parsing is load-bearing (verified).** On the real box, `io.stat` is **empty** and
`cgroup.controllers` is only `memory pids` until a controller is enabled per-unit. So files can be
absent, empty, or partially populated. The core treats every field as optional: missing/empty → `None`,
never a crash. This was found by probing the box, not assumed.

**Why a timer, not a daemon.** A `poll` oneshot fired by a systemd timer (interval) holds no long-lived
process — lower held-state, restart-free, and it lands in the same governed-unit model as everything
else. Retention: each `poll` prunes samples older than a window (default 14d) so the SQLite stays small.

**Alternatives rejected.** (a) Off-the-shelf exporter (node_exporter/cAdvisor) — pulls in the
metrics-stack non-goal (Open decision 8). (b) A long-running daemon — more held state, no benefit at
this cadence. (c) Reading `systemctl show <unit> -p MemoryCurrent` instead of the cgroup fs — shells out
per unit, slower, and gives a subset; the cgroup files are the source of truth and cheaper to read.

## Verified Assumptions

- **cgroup v2 unified** — `stat -fc %T /sys/fs/cgroup` → `cgroup2fs` on the box (Debian 13). Path for a
  service unit is `/sys/fs/cgroup/system.slice/<unit>.service/`.
- **File formats** (read from `dbus.service` on the box):
  - `memory.current` / `memory.peak` → a single integer, bytes (`2441216` / `3096576`).
  - `pids.current` → a single integer.
  - `cpu.stat` → key-value lines; `usage_usec <n>` is total CPU time in µs (also `user_usec`,
    `system_usec`, `nice_usec`).
  - `io.stat` → per-device lines `MAJ:MIN rbytes=.. wbytes=.. ...`; **can be empty** (was, for dbus).
  - `cgroup.controllers` → space-separated enabled controllers; was `memory pids` (no `io`/`cpu`
    delegated by default — they enable per-unit when accounting/limits are set, Phase 3 governance).
- **Runtime** — box has `python 3.13.5` + stdlib `sqlite3 3.46.1`. Client is stdlib-only; `pytest` is
  dev-only.
- **Governance is live** — Phase 3 (`2f596a9`) makes every generated unit set `*Accounting=yes` +
  limits, so the cpu/io controllers become available on our tenants' cgroups (unlike the bare defaults).

## Documentation Impact

- `croft-stack/telemetry/README.md` — new; usage (`poll`/`show`), the stdlib-only + no-deps note, the
  cgroup-fields table. Added in Phase 3 (the CLI phase).
- `discovery/alpha/plans/croft-stack/03-governance-telemetry.md` — link to this plan + mark the
  telemetry client "planned → built" as phases land. Updated when the client is built.
- `discovery/alpha/plans/croft-stack/README.md` (roadmap) — Phase 3 status (telemetry done) + the
  dev-toolchain followup gains `pytest`. Updated at completion.
- `discovery/alpha/plans/croft-stack/04-stub-bringup.md` — the Ansible `telemetry` role installs
  `telemetry/deploy/*.{service,timer}`; note the artifact path. Updated when Phase 4 role is authored.
- Grepped `telemetry` in croft-stack: only the plan/session references — no code refs to break.

## Concurrency Map

Sequential spine: Phase 1 → Phase 2 → Phase 3 → Phase 4.

All phases sequential. Reason: Phase 2 imports the `Sample`/parse API from Phase 1; Phase 3 wires
Phase 1 core + Phase 2 adapters through the CLI; Phase 4 packages/deploys what Phase 3 produced. The
write-sets are within one new module tree (`croft-stack/telemetry/`) and each phase reads what the prior
wrote, so there is no parallel-safe pair. No shared mutable state beyond the file write-sets (all work is
local file authoring + `pytest`; no box mutation until the Phase-4 Ansible converge, which is separately
gated). Re-entry verification: n/a (sequential).

## Phases

### Phase 1 — cgroup parsing core (pure, no I/O)
**Goal:** Turn raw cgroup file contents into a typed `Sample`, tolerating absent/empty/partial files.
**Changes:**
- [ ] `telemetry/croft_telemetry/core.py` — `@dataclass(frozen=True) Sample` (unit, ts, memory_current,
  memory_peak, cpu_usage_usec, pids_current, io_rbytes, io_wbytes; all resource fields `int | None`) +
  pure parsers: `parse_int(text) -> int|None`, `parse_cpu_stat(text) -> int|None` (extract `usage_usec`),
  `parse_io_stat(text) -> tuple[int|None,int|None]` (sum rbytes/wbytes across devices; empty → (None,None)),
  and `build_sample(unit, ts, files: dict[str,str|None]) -> Sample`.
- [ ] `telemetry/tests/test_core.py` — behaviors below.
**Call chain:** (core; wired in Phase 3) `cli poll` → `FilesystemCgroupSource.read(unit)` →
`build_sample(...)`. Named here so Phase 3 knows the seam.
**Wiring test:** n/a for the pure core — its wiring is proven in Phase 3. (Not a defect: Phase 1 ships no
entry point.)
**Depends on:** nothing.
**Read-set:** none (pure). **Write-set:** `telemetry/croft_telemetry/core.py`,
`telemetry/tests/test_core.py`. **Shared-state contract:** none beyond the file write-set.
**Risks:** mis-parsing an edge format. Mitigated by fixture-driven tests using the **real** formats
captured in Verified Assumptions.
**Done when:**
1. **Behavioral:** given raw cgroup file contents (incl. empty `io.stat`, missing `cpu.stat`),
   `build_sample` returns a `Sample` with correct ints and `None` for absent/empty, never raising.
2. **Verification:** `pytest telemetry/tests/test_core.py -v`.
**Validation:** Narrow (pure functions). Wiring test + unit tests sufficient. Tests must name **edges**:
`memory.current="2441216"`→2441216; `""`→None; missing key in `cpu.stat`→None; empty `io.stat`→(None,None);
multi-device `io.stat` sums; whitespace/trailing-newline tolerated.

### Phase 2 — ports + adapters (filesystem source, sqlite store)
**Goal:** Real adapters behind narrow ports; the store persists and queries samples.
**Changes:**
- [ ] `telemetry/croft_telemetry/ports.py` — `CgroupSource` (`.units() -> list[str]`,
  `.read(unit) -> dict[str,str|None]`) and `SampleStore` (`.init()`, `.insert(Sample)`,
  `.query(unit, since_ts) -> list[Sample]`, `.prune(before_ts)`) as `typing.Protocol`s.
- [ ] `telemetry/croft_telemetry/cgroup_fs.py` — `FilesystemCgroupSource(root=".../system.slice")`:
  `units()` globs `*.service` dirs; `read()` reads the 5 files, missing → `None`.
- [ ] `telemetry/croft_telemetry/store_sqlite.py` — `SqliteSampleStore(path)`: schema
  `sample(ts, unit, memory_current, memory_peak, cpu_usage_usec, pids_current, io_rbytes, io_wbytes)` +
  index `(unit, ts)`; insert/query/prune.
- [ ] `telemetry/tests/test_store.py`, `telemetry/tests/test_cgroup_fs.py` (fixture cgroup tree).
**Call chain:** (adapters; wired in Phase 3) `cli poll` → `source.units()`/`source.read()` +
`store.insert()`; `cli show` → `store.query()`.
**Wiring test:** n/a (adapters; wired Phase 3).
**Depends on:** Phase 1 (`Sample`).
**Read-set:** `telemetry/croft_telemetry/core.py`. **Write-set:** `telemetry/croft_telemetry/ports.py`,
`.../cgroup_fs.py`, `.../store_sqlite.py`, `telemetry/tests/test_store.py`,
`telemetry/tests/test_cgroup_fs.py`, `telemetry/tests/fixtures/cgroup/**`.
**Shared-state contract:** tests use `tmp_path` for the sqlite db + a fixture cgroup dir; no ambient
state, no box.
**Risks:** sqlite schema/query bugs. Mitigated by in-memory/`tmp_path` db round-trip tests.
**Done when:**
1. **Behavioral:** `FilesystemCgroupSource` over a fixture tree yields the right units + raw files;
   `SqliteSampleStore` round-trips a `Sample` and `query(unit, since)` returns only in-range rows;
   `prune` deletes older rows.
2. **Verification:** `pytest telemetry/tests/test_store.py telemetry/tests/test_cgroup_fs.py -v`.
**Validation:** Narrow/Moderate. Unit tests over `tmp_path`. Edges: query `since` boundary
(inclusive/exclusive at the cutoff ts); prune leaves the boundary row per its rule; empty db → `[]`.

### Phase 3 — CLI (`poll`, `show`) + wiring
**Goal:** The entry point that wires core + adapters; the poll→store→show path is live.
**Changes:**
- [ ] `telemetry/croft_telemetry/cli.py` — `argparse`: `poll [--cgroup-root R] [--db PATH] [--retain-days N]`
  (source.units → source.read → build_sample → store.insert; then `store.prune(now - retain)`);
  `show <unit> [--since DURATION] [--db PATH]` (store.query → print a compact table: ts, mem, cpu, pids,
  io). `now` injectable for tests. `main()` entry.
- [ ] `telemetry/croft_telemetry/__init__.py`, `telemetry/pyproject.toml` (console_script
  `croft-telemetry`; stdlib-only, no runtime deps).
- [ ] `telemetry/README.md`.
- [ ] `telemetry/tests/test_cli.py` — the **wiring test**.
**Call chain:** `croft-telemetry poll` → `main()` → `FilesystemCgroupSource` + `SqliteSampleStore` →
core → store. `croft-telemetry show <unit>` → `main()` → `SqliteSampleStore.query` → formatter.
**Wiring test:** `test_poll_then_show_end_to_end` — run `poll` (with `--cgroup-root` = a fixture tree,
`--db` = tmp) then `show <unit> --db tmp`; assert the fixture unit's memory value appears in `show`
output. RED before `cli.py` exists, GREEN after. This proves the entry point reaches the store, not just
that components work alone.
**Depends on:** Phases 1, 2.
**Read-set:** `core.py`, `ports.py`, `cgroup_fs.py`, `store_sqlite.py`. **Write-set:** `cli.py`,
`__init__.py`, `pyproject.toml`, `README.md`, `tests/test_cli.py`. **Shared-state contract:** tests use
`tmp_path` + fixture cgroup tree; no box, no ambient state.
**Risks:** the classic "components pass, nothing calls them." The wiring test is the guard.
**Done when:**
1. **Behavioral:** `croft-telemetry poll --cgroup-root <fixture> --db <tmp>` records samples;
   `croft-telemetry show <unit> --db <tmp>` prints them; run against the fixture tree end-to-end.
2. **Verification:** `pytest telemetry/tests/test_cli.py -k end_to_end -v` (through the CLI entry point).
**Validation:** Moderate. Wiring test + unit tests, **plus** run `croft-telemetry poll`/`show` by hand
once against a fixture tree (and, opportunistically, against the box's real `/sys/fs/cgroup` read-only)
to confirm behavior outside the harness. Edges: unknown unit → empty table (not a crash); `--since`
parsing (`30m`,`24h`,`7d`); empty db.

### Phase 4 — deployment artifacts (systemd timer + service)
**Goal:** Ship the units the Phase-4 Ansible converge installs, so `poll` runs on an interval, governed.
**Changes:**
- [ ] `telemetry/deploy/telemetry-poll.service` — oneshot `ExecStart=/opt/telemetry/current/croft-telemetry
  poll --db /var/lib/telemetry/samples.db --retain-days 14`, runs as a dedicated `telemetry` user,
  hardened + the same cgroup-governance stanzas (tiny envelope) as any unit; needs **read** access to
  `/sys/fs/cgroup` (no extra privilege — cgroup fs is world-readable) and RW only on its data dir.
- [ ] `telemetry/deploy/telemetry-poll.timer` — `OnBootSec=2min`, `OnUnitActiveSec=60s`.
- [ ] `telemetry/tests/test_deploy_units.bats` — assert the unit is hardened, non-root, governed, and
  the timer cadence is present (static-file assertions; bats, consistent with the kit).
**Call chain:** systemd `timer` → `telemetry-poll.service` → `croft-telemetry poll`. (Installed onto the
box by the Ansible `telemetry` role in roadmap Phase 4.)
**Wiring test:** the bats assertions here prove the unit invokes the CLI with a real db path + retention;
the live timer→poll→db path is proven at the Ansible converge (roadmap Phase 4), logged in `sessions/`.
**Depends on:** Phase 3 (the CLI it launches).
**Read-set:** the Phase-3 CLI contract (flags). **Write-set:** `telemetry/deploy/telemetry-poll.service`,
`telemetry/deploy/telemetry-poll.timer`, `telemetry/tests/test_deploy_units.bats`.
**Shared-state contract:** static files only; no box interaction in this phase.
**Risks:** the poll unit over-restricted and can't read `/sys/fs/cgroup`. Mitigated: cgroup v2 files are
world-readable; the unit needs no capabilities, only `ProtectSystem=strict` with RW on its data dir and
default (read) access to `/sys`. Confirm `ProtectKernelTunables`/`ProtectControlGroups` don't block the
**read** (they restrict writes) — verify at converge.
**Done when:**
1. **Behavioral:** the deploy units exist, are hardened+governed+non-root, and launch `croft-telemetry
   poll` with a real db + retention on a 60s cadence.
2. **Verification:** `bats telemetry/tests/test_deploy_units.bats`.
**Validation:** Narrow now (static-file assertions); the true end-to-end (timer fires → db grows on the
box) is validated at the Ansible converge (roadmap Phase 4), not here.

## Open Questions

- [RECOMMENDED: ADVISORY] Poll interval + retention defaults (60s / 14 days). *Tunable in the timer/CLI
  flags; safe to pick now and adjust from observed db growth.*
- [RECOMMENDED: ADVISORY] Poll all `system.slice/*.service` vs only croft-stack tenants. *rec: all
  (self-discovering, no manifest coupling); `show` filters. Low stakes; reversible.*
- [RECOMMENDED: PHASE-GATED (roadmap Phase 4 / converge)] Does the hardened poll unit actually read
  `/sys/fs/cgroup` under `ProtectControlGroups=yes`/`ProtectSystem=strict`? *Read should be fine (those
  restrict writes), but confirm live at the converge; the reader already tolerates missing files.*

## Review Log

### Pass 1: Plan development — 2026-07-28
Built the base: problem, hexagonal reasoning, phases 1–4 (TDD-first, wiring test in Phase 3), doc impact,
concurrency map. Grounded in a firsthand box probe (cgroup formats, empty `io.stat`, controllers, python
+ sqlite versions) captured in Verified Assumptions.

### Pass 2: Gap Analysis — 2026-07-28
**Found:** (a) the pure core (Phase 1) and adapters (Phase 2) have no entry point — flagged explicitly
that their wiring is proven in Phase 3, not a per-phase wiring-test defect. (b) `show` must not crash on
an unknown unit — added as a Phase 3 edge. (c) the poll unit's ability to *read* `/sys/fs/cgroup` under
hardening is a real risk → captured as a PHASE-GATED open question + a Phase 4 risk. (d) retention/prune
belongs in `poll` (not a separate command) so held-state stays bounded without extra ops.
**Concurrency:** no changes — map confirmed sequential (each phase imports/wires the prior; single new
module tree; no box mutation until the separately-gated converge).
**Changed:** added the prune-on-poll step to Phase 3; added the read-under-hardening risk to Phase 4;
added `show` unknown-unit edge.
**Confirmed:** stdlib-only is viable (box python 3.13 + sqlite verified); defensive parsing is required
(empty `io.stat` observed), not speculative.

### Pass 3: Quality Gates — 2026-07-28
**TDD ordering:** every phase lists tests before/with the code; Phase 3 has the load-bearing wiring test
(`test_poll_then_show_end_to_end`) whose verification runs through the CLI entry point, not an isolated
module. Test specs name edges (parse boundaries, `since` cutoff, prune boundary, unknown unit) —
mutation-resistant, not single-point.
**Observability:** the client *is* the observability; `poll` should log (stderr) a one-line summary
(units polled, rows written, rows pruned) so a failing timer is diagnosable via journald — added to
Phase 3.
**Debugging readiness:** each phase is independently `pytest`-verifiable; the box converge is a separate
gated step with its own session log.
**Validation calibration:** Phase 1/2 Narrow (tests sufficient); Phase 3 Moderate (hand-run the CLI +
opportunistic real-`/sys` read); Phase 4 static now, true end-to-end deferred to the converge — matches
scope.
**Concurrency honesty:** Map confirmed; sequential plan; write-sets are disjoint per phase but
read-depends-on-prior forces order; no ambient state (no box mutation in phases 1–4).
**Documentation impact:** README (Phase 3), 03/04/roadmap updates scheduled in the phases that make them
stale; grep recorded (no code refs).
**Coherence:** plan solves the stated problem (per-unit resource visibility, no metrics stack); scope
did not creep. **Confirmed ready:** yes — no BLOCKING items.

**Add (Pass 3):** Phase 3 — `poll` emits a one-line stderr summary (units/rows/pruned) for journald.
