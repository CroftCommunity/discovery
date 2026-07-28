# Phase 3 — Governance + telemetry wired

← [02-adopt-box-declaratively.md](02-adopt-box-declaratively.md) · [roadmap](README.md) · next →
[04-stub-bringup.md](04-stub-bringup.md)

**Status:** planned (box envelope known: 6 vCPU / 11 GiB / 94 GB) · **Depends-on:** Phase 2 (box
adopted) · **Gate-out:** every generated unit carries limits + accounting by default; the Python
telemetry client reports live per-process usage on the box.

## Concrete design (2026-07-28)

- **Telemetry client** (`croft-stack/telemetry/`, **Python**, pytest): poll each unit's cgroup-v2 files
  under `/sys/fs/cgroup/system.slice/<unit>.service/` — `memory.current`, `memory.peak`, `cpu.stat`
  (usage_usec), `pids.current`, `io.stat` — on an interval, append to a local time-series (SQLite under
  a data dir), + a tiny CLI (`telemetry show <unit>` / `--since`). Deployed by Ansible as a systemd
  **timer→oneshot** poller (not a daemon — lower held state). TDD: parse cgroup fixtures, no live box
  needed for the unit tests. No off-the-shelf exporter (metrics-stack non-goal, Open decision 8).
- **Governance stanzas** (generator default on **every** unit): `MemoryAccounting/CPUAccounting/`
  `IOAccounting/TasksAccounting=yes`; `MemoryHigh`/`MemoryMax`, `CPUQuota`, `TasksMax`, `IOWeight` sized
  per role. Starting envelopes for an 11 GiB / 6-vCPU box (tune from observed telemetry): `canary`
  tiny (`MemoryMax=64M`, `CPUQuota=10%`); an API sidecar small (`256M`, `25%`); a cache/index server
  larger (`MemoryHigh=1G`/`MemoryMax=2G`, `CPUQuota=150%`); the relay bandwidth-heavy (its own box
  eventually — WHEN-TO-SPLIT). Prefer `MemoryHigh` (soft) + generous `MemoryMax` so a spike throttles
  before it is killed.

---

## Problem

Resource limits and per-process accounting added after the fact are how a runaway ingest silently
starves a broker's cursor. Every unit must be born governed and observable — before any real service.

## Approach

Make cgroup limits + accounting a **generator default** (not sidecar-only), and stand up a small local
telemetry client that reads per-unit cgroup-v2 stats into a queryable local time-series.

## Steps (sketch — fill on arrival)

1. Add governance stanzas to the generator defaults for every unit: `MemoryHigh`/`MemoryMax`,
   `CPUQuota`, `TasksMax`, `IOWeight`; `MemoryAccounting`/`CPUAccounting`/`IOAccounting`/`TasksAccounting=yes`;
   isolation (empty caps, `ProtectSystem=strict`, `PrivateTmp`, `ReadOnlyPaths` on API units).
2. Size limits per role from the Phase-2 resource envelope (ingest vs broker vs cache differ).
3. Build the telemetry client **in Python** (per the Languages policy — utility, no real-time need, no
   shared-lib win) (Open decision 8 — *rec:* self-rolled cgroup-v2 reader over
   `/sys/fs/cgroup/system.slice/<unit>/` → local SQLite/append-only + tiny query CLI). TDD (`pytest`),
   red-first.
4. Prove it reads live per-unit usage on the stub unit.

## TODO (decide on arrival)
- [ ] Open decision 8: self-rolled cgroup-v2 reader vs off-the-shelf exporter (rec: self-rolled).
- [ ] Concrete limit values per role (needs the Phase-2 envelope + relay's real load from Phase 6).
- [ ] Where the telemetry time-series lives (data dir; retention).

## Risks & cautions
- Limits set too tight will `MemoryMax`-kill a legitimate spike; start with `MemoryHigh` (soft) +
  generous `MemoryMax`, tighten against observed usage (esp. after the relay lands in Phase 6).
- Keep the telemetry client low-state (matches the ethos); no new daemon if a periodic reader suffices.

## Validation
Every generated unit shows limits+accounting in `systemctl show`; the telemetry CLI reports live
`memory.current`/`cpu.stat`/`pids.current` per unit.

## References
Roadmap → Resource governance & telemetry; relay lab E5 (cgroup group accounting) / E6 (tc fairness).
