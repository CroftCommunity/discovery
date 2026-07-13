# Relay & Placement Lab — Conclusions (plain language)

Living, plain-language record of what the Relay & Placement Lab has actually shown. Companion to the
detailed `TEST-LOG.md` (chronological source of record) and `RELAY-PLACEMENT-LAB-SPEC.md` (the E0–E9
program). This doc answers "what do we now know?" in sentences; the spec/log hold the numbers and
method. Proofs that rise to durable invariants get folded into `discovery/crystallized/proof-ledger.md`
and, where they verify an invariant, the `Proofs/` repo.

Updated as the lab runs. Newest conclusions at the top of each section.

---

## Status at a glance

| Step / Experiment | What it decides | Status |
|---|---|---|
| Step 0 — topology | expand vs multiplex; port plan; iroh SHA pin | **inventory done; decision pending** |
| E0 — single-relay baseline | matchmaking vs idle vs active per-conn cost; capacity crossover | **MEASURED: mem ~31 KiB/conn, passthrough ~186 MiB/s on 2 vCPU, same-VPC matchmaking 100% direct. Hole-punch-FAILS case needs NAT'd Mac** |
| E1 — vertical scaling | one fat process vs several per box | **MEASURED: multi-threaded (5 threads, 2.34 cores used) — one fat process scales** |
| E2 — DNS-driven placement | server-published vs peer-cooperative placement | **MEASURED (fast loop): placement honored by id-resolution; wrong placement → no connection (co-location proven). DNS/pkarr integration pending** |
| E3 — namespace-sharded fan-out sync | the co-location thesis; dropped≈0 within a shard | **MEASURED: 12/12 and 30/30 members converge (Automerge), relay dropped = 0. Thesis confirmed** |
| E4 — LVS frontend | DNS-vs-LVS division of labor | not started |
| E5 — cgroup group accounting | isolation + per-group bill for free | not started |
| E6 — tc traffic shaping | network noisy-neighbor fairness; QUIC's reaction | not started |
| E7 — placement churn | the split-brain window + what tolerates it | not started |
| E8 — relay-vs-meer fork | cross-namespace density where superpeers stop being optional | not started |
| E9 — meer confidentiality tiers | blind mirror converges on metadata alone; cost of each envelope | not started |

---

## Step 0 — Node inventory & topology

### Verified node inventory (2026-06-16, live-checked + cross-referenced with prior docs)

Prior docs (`META-TRANSCRIPT-next-session.md`, `TEST-LOG.md`, `VALIDATION-METHODS.md`,
`croft-validation-session-2026-06-15` memory) recorded the qualitative asymmetry — node-1/2 carry a
`/mnt/data` EBS volume, node-3 is a 128 GB-root box with no `/mnt/data`, node-3 builds on 2 cores —
but never the exact vCPU/RAM. Live `nproc`/`free`/`df` this session fills that in:

| node | host | vCPU | RAM | disk | AZ | prior role |
|---|---|---|---|---|---|---|
| 1 | secroute-testing-one | **4** | 15 GiB | `/mnt/data` 147G EBS (120G free) | us-east-1c | peer / fetcher |
| 2 | secroute-testing-two | **2** | 7.7 GiB | `/mnt/data` EBS | us-east-1b | provider / broker |
| 3 | ip-172-31-88-18 | **2** | 3.8 GiB | 124G root (106G free), no `/mnt/data` | us-east-1a | 3rd peer / broker |
| 4 | this Mac | — | — | local (`.node4-*`, gitignored) | behind real NAT (off-VPC) | NAT-traversal peer |

Other verified facts:

- **The boxes are NOT identically sized** (4/15, 2/7.7, 2/3.8). The spec wants identically-sized
  relays so cross-shard numbers compare — this is the central tension Step 0 must resolve.
- All three same VPC `vpc-217f0f5c`; **only UDP 2112 open** in the Security Group today.
- cgroup **v2** (`cgroup2fs`) with `cpu`/`cpuset`/`memory`/`io` controllers on node-1; `tc` present;
  **`ipvsadm` absent** (E4/LVS needs it installed); kernel `7.0.0-1004-aws`.
- `cargo 1.96.0` on all three; build toolchains present (node-3 has build-essential from last session).
- iroh currently pinned **`1.0.0-rc.1`** in `Cargo.lock`; spec was verified vs **1.0.0** — must pin a
  single 1.0.x SHA for the lab and re-verify every API against that exact source.
- **No AWS credentials on the Mac** (`aws sts` → NoCredentials) → SG edits are not self-serve from
  here; widening the SG is a user action (console, or configure creds).
- An **unattached 150G EBS volume in us-east-1a** exists (attachable to node-3 only — EBS is AZ-locked).

### Conclusion (Step 0 decided 2026-06-16)

- **Multiplex, not expand.** Relays under test run as **cgroup-pinned 2 vCPU / 4 GB slices** on
  node-1 and node-2, so the host-size asymmetry (4c vs 2c boxes) cannot contaminate cross-shard
  comparison — and this hands us E5's per-process accounting for free. node-3 (smallest, 3.8 GB)
  hosts the control plane (DNS origin + pkarr + admit hook + Prometheus). The Mac is the off-VPC
  NAT'd generator + driver. Accepted caveat: co-located generators understate the relay wall; we
  move a generator off-box and re-measure if any number looks generator-bound.
- **SG: all-from-self intra-VPC** (user opening it; not self-serve from the Mac — no AWS creds).
  Cross-box relay runs are gated on that rule going live; UDP 2112 alone is insufficient.
- **iroh pinned `=1.0.0`** (+ `iroh-relay =1.0.0`) — the exact version the spec was verified against,
  so the rc.1 API drift (`MemoryLookup` vs `MemoryAddressLookup`, `RelayMap` ctors) is moot. Pin
  held by `=` requirement + committed `Cargo.lock` checksum.

Full mapping + tooling-to-install in `relay-lab-runs/MANIFEST.md`.

---

## E0 — Single-relay baseline (matchmaking vs passthrough)

### Harness built + locally validated (2026-06-16)

`crates/relay-loadtest` ships the first three E0 components (spec §5 item 1): `relay` (spawns one
self-signed `iroh-relay` with `AllowAll`), `responder` (homed on the relay, echoes bi-streams,
advertises its `EndpointAddr` as JSON), and `generate` (opens N connections, classifies the live
*selected* path as relay vs direct via `conn.paths()`, records RTT, optional bulk bytes + hold
window). Pinned iroh `=1.0.0`; builds clean on the Mac.

Loopback smoke test (Mac, 5 conns each, 4 KiB roundtrip) confirms the two modes are real and
distinguishable:

| mode | live relay | live direct | selected-path RTT p50 |
|---|---|---|---|
| passthrough (forced relay) | 5/5 | 0 | 9.19 ms |
| matchmaking (hole-punch allowed) | 0 | 5/5 | 6.61 ms |

(Loopback RTTs are tiny and not the real numbers — the point is the classifier and the forced-relay
mechanism work. Real per-connection cost numbers come from the fabric run.)

### Finding worth keeping (corrects a prior-spike assumption)

**A relay-only *dial address* does NOT force relay passthrough in iroh 1.0.** Stripping direct IPs
from the `EndpointAddr` (the rc.1-era technique noted in the kickoff) only removes the bootstrap
hint; iroh still upgrades to a direct path via relay-coordinated hole-punch — first smoke test
showed passthrough going 5/5 *direct*. Forcing relay-only requires building the endpoint with
`.clear_ip_transports()` (and *not* calling `bind_addr`, which re-pushes an IP transport). iroh's
own tests confirm this contrast (`endpoint_two_relay_only_becomes_direct` vs
`endpoint_two_relay_only_no_ip`). Implication for the lab: forced-passthrough load must clear the
generator's IP transport; for a genuine "hole-punch *failed*" measurement use the NAT'd Mac (node-4)
or firewall the direct UDP path. Folded into `IROH-1.0.0-API-VERIFIED.md`.

### Measured: the idle-relayed memory wall (2026-06-16, node-1)

Single-box run (relay in a 4 GiB / 2-vCPU cgroup slice on node-1; generator forced relay-only;
relay RSS = cgroup `memory.current`). Co-located, so this measures the **memory** ceiling, not the
NIC ceiling — but per-connection relay RSS doesn't depend on where the generator runs, so the number
is valid. Sweep, all connections established + relay-classified, zero failures:

| N relay client conns | relay RSS | per-conn (incl. base) |
|---|---|---|
| baseline (1) | 2.04 MB | — |
| 100 | 5.77 MB | 37.3 KiB |
| 200 | 9.01 MB | 34.9 KiB |
| 400 | 15.13 MB | 32.7 KiB |

**Marginal RSS ≈ 31 KiB per idle relayed client connection** (slope across the sweep, cancelling the
~2.5 MB fixed relay overhead). Linear in N. This is **below** the spec's `[HYPOTHESIS]` of 50–150
KiB/conn. Sizing consequence: a 4 GiB relay slice holds **~130k idle relayed connections** on the
memory ceiling alone (CPU/NIC bind sooner under active load — measured next). Raw data:
`relay-lab-runs/E0-memwall-2026-06-16/manifest.json`.

### SG update (2026-06-16): all-from-self is LIVE

Re-probed with a reliable Python TCP-accept + UDP-sink (the `nc` probe was unreliable — it missed
even the known-open 2112). TCP 3343 OPEN and UDP 3478/2112 delivered from both node-2 and node-3 →
the all-from-self intra-SG rule is live. Same-VPC cross-host relay runs can proceed. (The NAT'd Mac
still needs **public** ingress on 3343/3478 — separate from intra-SG — for the holepunch-*fails*
matchmaking case.)

### Measured: cross-host throughput + the matchmaking/passthrough gap (2026-06-16)

relay=node-1 (cgroup 2 vCPU/4 GiB), responder=node-2, generator=node-3 — all same VPC, different
AZs, real cross-VPC network on every hop. Data: `relay-lab-runs/E0-crosshost-2026-06-16/`.

- **Matchmaking, same-VPC:** 20/20 hole-punched to **direct**, relay carried **0** post-handoff
  (direct RTT ~21.5 ms cross-AZ). The cheap side of the cost gap: where hole-punch succeeds, relay
  steady-state cost per pair ≈ 0.
- **Active passthrough throughput:** 50 conns × 4 MiB → relay forwarded ~400 MiB in 5.5 s on the
  2-vCPU slice → **~93 MiB per CPU-second (~186 MiB/s CPU-bound on 2 vCPU)**; +8.3 MiB relay RSS
  under 50 active conns. At this scale the passthrough wall is **CPU**, not NIC.
- **Relay path RTT** cross-host ~9.5 ms p50.

### The E0 picture (so far)

| metric | measured | vs hypothesis |
|---|---|---|
| idle relayed conn (memory) | ~31 KiB RSS/conn (+~2.5 MB fixed) | below 50–150 KiB |
| active passthrough | ~93 MiB/cpu-s (~186 MiB/s on 2 vCPU) | new |
| matchmaking, hole-punch OK | relay load ≈ 0 (100% direct, same-VPC) | new |

**Sizing takeaway:** a 2 vCPU / 4 GiB relay holds ~130k idle relayed conns (memory) but only sustains
~186 MiB/s of active passthrough (CPU). So the bind under real load is CPU/throughput, not memory —
and every pair that hole-punches to direct removes itself from that budget entirely.

### Still open (needs the NAT'd Mac + public ingress)

- **The expensive half of the cost gap** — matchmaking where hole-punch *fails* and falls back to
  relay (the CGNAT/mobile population). Needs node-4 (the Mac, real NAT) reaching the relay, which
  needs **public** ingress on 3343/3478 (the intra-SG all-from-self rule doesn't cover the off-VPC Mac).
- **Handshake-CPU wall** — accepts/sec under reconnect storms (a dedicated reconnect-storm driver).
- A **sustained-transfer** generator mode to separate forwarding CPU from establishment CPU.

---

## E1 — Vertical scaling (one fat process vs many)

Relay on node-1 with 4-core headroom (`CPUQuota=400%`); sustained passthrough from two generators
(node-2 + node-3). Measured the relay's average cores-used = Δ`cgroup cpu.stat` / Δwall over 10 s.

- **Relay threads: 5. Average cores used: 2.34** (4-core headroom). iroh-relay 1.0 is multi-threaded
  (tokio) and spreads forwarding across cores; the lock-free `papaya` `Clients` registry does **not**
  serialize it onto one core.
- **Decision:** one fat process scales vertically — you don't need multiple relay processes per box
  for core utilization. The multiple-processes-per-box rationale is **E5** (per-group cgroup
  accounting + isolation), not CPU.
- **Caveat:** not saturated to 4.0 cores — the 2-vCPU generator/responder boxes were the bottleneck,
  so the per-process ceiling is ≥2.34 (multi-core confirmed), exact ceiling unreached on this fabric.

Data: `relay-lab-runs/E1-vertical-2026-06-16/manifest.json`.

---

## E2 — DNS-driven placement (fast loop) + the co-location proof

Two relays (relay-A node-1, relay-B node-2), responder homed on relay-B, generator on node-1 knowing
**both** relays. A `MemoryLookup` seeded with `peer-id → relay-url` stands in for the controller's
published record; the generator connects by **bare id** so iroh resolves the relay from the record.

| test | assigned | result |
|---|---|---|
| correct | relay-B (where peer is) | **connected via relay-B** (`is_relay_path=true`) |
| wrong | relay-A (peer is on relay-B) | **timed out — no connection** |

- **Placement is controllable + authoritative:** the generator knew both relays but reached the peer
  via the *assigned* relay, resolved from the record by id — not latency. The E2 decision gate
  resolves to **server-published placement is authoritative for reachability** (peer-cooperative is
  the same mechanism with the peer as writer).
- **Co-location is mandatory (the spec's load-bearing fact, now empirical):** a wrong assignment →
  no connection, because relays don't mesh. This is E3's thesis in miniature.
- **Pending:** the integration form — custom DNS origin + pkarr publish so peers resolve via real DNS
  instead of in-memory injection (APIs verified present; not yet wired into a run).

Data: `relay-lab-runs/E2-placement-2026-06-16/manifest.json`.

---

## E3 — Namespace-sharded fan-out sync (THE CORE THESIS)

Real eventual-sync workload: **Automerge** anti-entropy over iroh bi-streams. One relay shard
(node-1); members (node-3) all **forced relay-only** and homed on that shard converge a shared CRDT
by meeting a rendezvous peer (node-2) — no point-to-point arrangement between members. Relay
`send_packets_dropped` scraped from the relay's OpenMetrics endpoint.

| population | converged | relay dropped | wall |
|---|---|---|---|
| 12 | **12/12** | **0** | 1 s |
| 30 | **30/30** | **0** | 1 s |

- **Every member converges** to the full namespace state, forced relay-only, by meeting a local
  co-located peer — not the data owner, not a specific partner.
- **`send_packets_dropped` stayed exactly 0** across both runs — zero co-location misses within a
  correctly-sharded namespace. That counter is the spec's co-location-miss signal; it being 0 is the
  measured form of "co-location is sufficient."
- **With E2's negative test** (wrong placement → no connection, because relays don't mesh), E2+E3
  prove the thesis end to end: **DNS-placement-by-namespace co-locates members on one shard, and
  within that shard everyone converges with dropped ≈ 0.** This is the single architectural fact the
  whole spec hangs on — now evidence, not intuition.
- Caveats: rendezvous-hub topology (vs full random-peer gossip mesh — a refinement); convergence used
  an anti-entropy retry loop (expected — a single session converges to the hub's state at that
  moment); edit-rate/churn axes not yet swept.

Data: `relay-lab-runs/E3-namespace-sync-2026-06-16/manifest.json`.
