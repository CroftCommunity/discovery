# Croft Relay & Placement Lab — Test Specification

Version 0.1. Target: a multi-node lab driven by Claude Code.

> **Provenance & version caveat.** This spec was developed in a separate research thread and
> **verified against iroh / iroh-relay 1.0.0 (released)**. The validation spikes already in this repo
> (`crates/altdrive-spike-*`) pin **iroh `1.0.0-rc.1`**, where some APIs differ (e.g. the in-memory
> address lookup is `MemoryLookup` in rc.1 vs. `MemoryAddressLookup` referenced below; `RelayMap`
> constructors differ). **Before building: pin a single iroh 1.0.x SHA for the relay lab and
> re-verify every API against that exact source** (the "verify against source, never guess" rule).
> Every API and behavioural claim in this spec was read from that source tree. Where a quantity is a
> prediction rather than a measurement or source fact it is tagged `[HYPOTHESIS]`; the point of the lab
> is to replace those with numbers.

---

## 0. Purpose and the one architectural fact everything hangs on

We are characterising the load, tuning, and topology landscape for relays in a decentralized cooperative platform (Croft), and deciding where relays end and superpeers begin.

The single fact that shapes every experiment:

**An iroh relay only delivers a packet to a destination endpoint that is connected to that same relay process. There is no relay-to-relay mesh.** (`Clients::send_packet` in `iroh-relay/src/server/clients.rs` looks the destination up in the local client registry and drops the packet if it is not present.) Two peers can use relay fallback to reach each other only if they share a home relay.

Consequence: any horizontal scaling scheme must guarantee that peers who need each other are co-located on the same relay process. We solve that with DNS-driven placement keyed on the rendezvous (namespace/group), not on individual identity. The experiments validate that this holds and measure what it costs.

A second fact that softens the first: in eventual-sync workloads (Willow/WGPS, Automerge) a peer rarely needs one *specific* partner — it needs *enough* up-to-date partners to converge. Co-location is therefore a per-namespace property, not a per-pair one. Several experiments test whether this reframing actually removes the constraint in practice.

---

## 1. How iroh placement and discovery actually work (the mechanisms we exploit)

Read these before building; the stubs below depend on them.

- **Discovery carries the relay assignment.** An endpoint's record is published under `_iroh.<z32-endpoint-id>.<origin-domain>` as DNS TXT, containing `relay=<url>` among other key=value pairs (`iroh-dns/src/endpoint_info.rs`). When peer A resolves peer B, it learns B's home relay from this record. This is the placement hook.

- **The origin domain is configurable.** `DnsAddressLookup::builder(origin_domain)` points discovery at a custom DNS origin instead of n0's (`iroh/src/address_lookup/dns.rs`). Running our own origin means the `relay=` value is our placement decision.

- **Records can be injected in-process for tests.** `MemoryAddressLookup::new()` + `add_endpoint_info(..)` (`iroh/src/address_lookup/memory.rs`) lets a test inject "endpoint B lives at relay R" without standing up real DNS. Use this for fast iteration; use real DNS + pkarr publishing for the integration runs.

- **Relay assignment can be external, not latency-chosen.** Left to its own devices a peer picks its lowest-latency relay (`net_report` sets `preferred_relay` to the lowest-latency record). For Croft we override that: we publish the relay we want a peer to home to. The placement controller owns this decision.

- **DNS does not gate; the relay gates.** DNS is an unauthenticated public lookup — it proposes placement. Admission is enforced at the relay's `AccessControl::on_connect`, which runs *after* the handshake cryptographically authenticates the endpoint ID (`iroh-relay/src/server/main.rs`: `everyone` | `allowlist` | `denylist` | `shared_token` | `http`). The HTTP hook receives `X-Iroh-Endpoint-Id` and returns `200`+`true` to admit. DNS and the relay gate must agree, but the relay is the wall.

- **Throttle is per-client, ingress, app-layer.** `[limits.client_rx] bytes_per_second` + `max_burst_bytes` token-bucket on the relay read path (`iroh-relay/src/server/streams.rs`). `accept_conn_limit`/`accept_conn_burst` exist but are **not implemented in 1.0** — connection-rate limiting must live at the LB/firewall.

- **A peer may hold more than one relay connection.** It has one *home* relay for its own discovery record, but it dials peers whose records point at other relays, so a multi-namespace peer naturally fans out across relays. This is the basis for the cross-tunnel experiments.

---

## 2. Lab topology

Minimum viable lab is 6 nodes; 8 is comfortable. All on a private L2 segment plus a control network.

- **relay-1, relay-2, relay-3**: relay shards. Each runs one or more `iroh-relay` processes under cgroup slices. Size them identically (e.g. 4 vCPU / 16 GB) so cross-shard numbers compare.

- **lb-1**: LVS director (IPVS). Fronts a relay shard's process pool for HA + within-shard persistence.

- **ctrl-1**: placement controller (custom DNS origin + pkarr publish endpoint) and metrics/Prometheus + the experiment orchestrator.

- **gen-1, gen-2**: load generators. Keep these on separate boxes from the relays and size them *larger* than any single relay, so you measure the relay's wall and not the generator's. Two generators let you place "both ends" of cross-shard pairs on different hosts.

Clocks: run NTP/chrony on all nodes; several measurements are latency- and window-sensitive and skew will corrupt them.

Version discipline: pin one iroh git SHA across every binary in a run and record it in the run manifest. Relay byte-cost per connection changes across versions (the multipath NAT-traversal path moved relay byte volume), so cross-run comparisons are only valid at a fixed SHA.

---

## 3. Global instrumentation (collect on every experiment)

Relay process, per shard and per process:

- `relay_bytes_sent`, `relay_bytes_recv` — throughput accounting.

- `relay_accepts`, `relay_disconnects`, `relay_unique_client_keys` — connection accounting.

- `relay_send_packets_sent`, `relay_send_packets_recv`, `relay_send_packets_dropped` — **dropped is the co-location signal**: a non-zero dropped count means packets arrived for an endpoint not connected to this process (a placement miss).

- `relay_bytes_rx_ratelimited_total`, `relay_conns_rx_ratelimited_total` — throttle activity.

- `relay_clients_inactive_added`, `relay_clients_inactive_removed` — idle pruning.

Host, per relay node:

- Process RSS via cgroup `memory.current`; CPU via cgroup `cpu.stat`; per-core utilisation (we care whether one process pins one core).

- NIC throughput in/out, and qdisc stats if tc is engaged.

- File-descriptor count and socket counts.

Generator side:

- Connections attempted / established / failed; for each, whether the live path was relay or direct (`conn.paths().iter().any(|p| p.is_relay())`).

- For sync experiments: convergence time per namespace, and per-peer "time to first useful partner".

Meer (superpeer) process — **we instrument this ourselves; iroh provides none of it**, because a meer is an application-protocol participant, not a known iroh component. Emit these as custom counters/gauges from the meer binary:

- `meer_sync_sessions_active` (gauge) / `meer_sync_sessions_total` (counter) — concurrent and cumulative sync partners served.

- `meer_convergence_ms` (histogram) — time for a peer syncing against this meer to reach convergence. This is the meer's headline quality number, the analog of relay throughput.

- `meer_state_bytes` (gauge, per namespace) and `meer_namespaces_held` (gauge) — resident state. Track growth over the run, not just a point value; CRDT state grows.

- `meer_merge_ops_total` (counter) and `meer_merge_cpu_seconds` (counter) — merge/diff work. Unlike the relay, this **is** payload-CPU, so CPU-per-op is a real and important number.

- `meer_fanout_peers` (gauge) — peers this meer currently keeps converged. Capacity is expressed in fan-out, not connections.

- `meer_bridge_namespaces` (gauge) and `meer_bridge_bytes` (counter) — cross-shard bridging activity, only non-zero when the meer straddles shards (E7/E8).

Host, per meer node: same RSS / CPU / cgroup / NIC / fd telemetry as relay nodes, so the two roles compare on identical host axes.

Record everything to a per-run directory with the iroh SHA, node sizes, kernel version, and the experiment parameters in a `manifest.json`. Every chart must be reproducible from the manifest.

---

### 3a. Performance characterization: relay vs. meer

The two roles are measured differently because they fail differently and bind on different resources. Capture both so the lab produces a symmetric picture and the relay/meer boundary (E8) is drawn on evidence.

**Why they are not the same measurement.** A relay never decrypts; it forwards opaque frames. Its cost is I/O and memory, never payload compute, and its metrics are built into iroh. A meer is an always-available application-protocol participant — it holds state and actually processes content (merges, diffs), so its cost is state plus compute, and you instrument it yourself. A meer is **not** a relay that stays awake; the relay has nothing to forward when no live peer is on the other end, whereas the meer's entire purpose is to *be* that always-up peer so others can converge while offline. (The name fits: a meer is the peer that took modafinil and stayed awake so the rest can sleep and still sync.)

**Relay — the three ceilings.** "Relay performance" is not one number; it is three ceilings that bind in an order set by traffic mix:

- Idle connections held → **memory wall.** Cost per held connection is TLS session + socket buffers + task. [HYPOTHESIS] ~50–150 KB/conn; E0 measures the real figure.

- Active passthrough throughput → **NIC wall.** Binds far sooner than memory once a meaningful fraction is actually relaying. For Croft's mobile/CGNAT population the relayed fraction likely exceeds n0's stated ~10%, so plan for this wall.

- Accept rate → **handshake-CPU wall.** Bursty, independent of concurrency; TLS handshakes on reconnect storms spike it.

Relay headline metrics to report: concurrent connections held, accepts/sec sustained before handshake latency degrades, bytes/sec forwarded, RSS/connection, CPU/accept, dropped-packet rate. The relay cannot report latency (measure from the client via `conn.paths()` RTT) or per-endpoint accounting (that is the HTTP admit hook's job — the only component that sees identity and usage together).

**Meer — capacity is fan-out, not connections.** Meer headline metrics: convergence time it provides (`meer_convergence_ms`), concurrent sync sessions sustainable (`meer_sync_sessions_active`), CPU per merge op (`meer_merge_cpu_seconds` / `meer_merge_ops_total`), state held and its growth rate (`meer_state_bytes` over time), and peers kept converged (`meer_fanout_peers`). The meer degrades into convergence lag and unbounded memory growth, not dropped packets.

**The comparison that matters.** Put relay and meer on the same host axes (RSS, CPU, NIC) so a given box's capacity is expressed as "N relayed connections" vs. "M converged peers," and E8 can state the cross-namespace density at which a meer is cheaper than every multi-homed peer holding its own relay connections. Measure separately the **offline-data fraction** — requests for state whose owner is not currently online — because no relay tuning serves it; it is meer-only demand and it sizes the meer fleet independently of relay load.

---

### 3b. Meer confidentiality tiers: what "blind" actually means

A meer's plaintext access is a **design choice, not a property of being always-on.** An always-on node can facilitate sync without ever decrypting payloads. But "blind" has a precise, limited meaning that must be designed around rather than assumed, and the limit comes from the sync protocol itself, not from iroh.

**Verified against the Willow e2e encryption spec.** Willow explicitly anticipates "an always-on server in the cloud that facilitates data exchange without being privy to the data." Payloads encrypt cleanly: pad to a fixed length, append a nonce, encrypt, and the ciphertext plus its digest and length are meaningless to a node that lacks the key, yet that node can still store and replicate the entry. **However, not every field can be encrypted, because peers must compare certain metadata to compute store joins (which entry overwrites which).** Per the spec, the unavoidable cleartext (or order-preserving) fields are:

- **Timestamps** — must be compared numerically for overwrite semantics, so Willow "deals in plaintext timestamps only." Mitigation is obfuscation (day-resolution, or logical-counter timestamps), not encryption.

- **Payload digest and length** — a storing node computes these anyway, so encrypting them is pointless; padding hides true length, nonces hide equality.

- **NamespaceId / SubspaceId** — encrypting these per-subspace would break overwrite detection, so they are effectively public; hash the meaningful parts before handing them to Willow.

- **Path** — encryptable, but via a hierarchical per-component key-derivation scheme, and a node needs the relevant key to reason about a path range.

So the honest statement for Croft: a blind mirror **never sees payload plaintext**, but it does see **metadata** — namespace/subspace identity, entry counts, sizes, timestamps, and access patterns (who syncs when). That is enough to do range reconciliation (range-based set reconciliation operates over fingerprints of authorised entries, not plaintext), which is exactly why a blind mirror is useful. It is not enough to read content. This corrects an earlier overstatement that "a meer can read what it syncs": only the semantic tier does.

**The three tiers, as a per-group policy dial (E9 tests all three):**

- **Tier 0 — Blind mirror.** Stores and serves encrypted payloads; sees only the unavoidable metadata above. Can replicate and can answer "do you have entries in this range" via fingerprints. Maximum reliability, minimum trust. This is the Croft default for groups that want always-on convergence.

- **Tier 1 — Double-enveloped mirror.** An outer envelope (keyed to the authorised exchange set, including the mirror) wraps the inner app E2EE payload. The mirror strips the outer envelope to route/store and verify the sender is authorised, but the inner envelope stays sealed. Pushing sync metadata into the inner envelope where the join semantics allow reduces what the mirror learns below even Tier 0 — at the cost that the mirror can do less reconciliation (a fully sealed blob is just bytes; it can't help range-diff). This is the security/reliability dial made literal: more sealing = less mirror knowledge = less mirror usefulness.

- **Tier 2 — Semantic meer.** Holds the decryption key, merges CRDTs, answers queries, validates content. The only tier that reads plaintext. Run only where a cooperative explicitly designates a trusted always-on member. This is the tier whose merge-CPU cost §3a describes.

**The no-mirror group is the fourth point on the dial, not a separate architecture.** A group can choose a **fixed-list peer set with no mirror in it**: it converges only when members are online together, no third party holds bytes in any form. Enforced with defense in depth (per the design decision for this spec):

- **Connection allowlist** at the endpoint accept side (and relay `on_connect` if a relay is used for NAT traversal): the group only exchanges with its listed authenticated endpoint IDs; no mirror ID is in the set.

- **Namespace capability** (Meadowcap/MLS): even a node that connects holds no read/write capability for the namespace, so admission and authorisation are independent gates.

The cost is explicit and not solvable away: two members never online simultaneously never sync, because no awake node bridges them. For a cooperative this is a governance decision — a group can reasonably let members vote where they sit on the dial — so the spec treats it as configuration, not a fork.

---

## 4. Experiments

Each is written as: **what we test → why → method → what we vary → success/measurement → expected result [HYPOTHESIS] → failure modes to watch.** Run them in order; later experiments depend on results from earlier ones.

### E0 — Single-relay baseline: matchmaking vs. passthrough cost

**What.** Establish the per-connection cost of (a) a normal connection that hole-punches and hands off, vs. (b) a connection forced to stay on the relay (true passthrough).

**Why.** This is the denominator for everything. Real load is mostly cheap matchmaking blips plus a relayed tail. The gap between (a) and (b) is the entire sizing argument: if matchmaking is nearly free and only the ~10% tail is expensive, capacity planning targets the tail, not the total.

**Method.** One relay process, no LVS, no tc, no sharding. Drive with the existing loadgen. Add a third generator mode (see §5, "E0 mode"): normal connect with hole-punch allowed, hold, and record whether the path went direct. Compare against the forced relay-only mode already implemented.

**Vary.** Connection count (ramped); payload mode {idle, throughput}; bytes/sec in throughput mode.

**Measure.** Per-connection RSS delta and CPU for each of: matchmaking-then-direct, idle-relayed, active-relayed. Bytes through the relay in each case (expect near-zero for the direct case after handoff; the dogfooding data suggests tens of KB even on successful holepunches at recent SHAs, so measure it, don't assume zero).

**Expected [HYPOTHESIS].** Direct-after-handoff costs the relay a small fixed setup blip then ~nothing. Idle-relayed binds on memory in the 50–150 KB/conn range. Active-relayed binds on NIC throughput long before memory. Crossover (where bytes/sec flatlines while conn count climbs) is the per-instance ceiling.

**Failure modes.** Generator self-bottleneck (move generators to bigger boxes); silent fallback to public relays (assert custom relay map); path misclassification (assert via `is_relay()` not by inference).

### E1 — Vertical scaling: does one relay process use all cores?

**What.** Whether a single `iroh-relay` process scales across the 4 cores of its box, or pins one core / contends on a shared lock in the client registry.

**Why.** If one process saturates one core and leaves three idle, splintering into multiple processes per box is justified *before* any sharding cleverness, and it changes the process-per-node count for every later experiment. If it scales near-linearly, we run one fat process per node and simplify.

**Method.** One process, pinned vs. unpinned. Drive accepts and throughput hard. Profile per-core utilisation and look for lock contention around `Clients` (the registry is behind a lock; contention there caps accept throughput).

**Vary.** Accept rate (independent of concurrency); concurrent connection count; with/without CPU pinning.

**Measure.** Accepts/sec sustained before handshake latency degrades; per-core utilisation distribution; throughput ceiling for a single process.

**Expected [HYPOTHESIS].** Tokio multithreaded runtime spreads connection tasks across cores, but the accept path and the shared client registry may serialise, capping accepts/sec below what raw cores suggest. Steady-state forwarding should spread well.

**Failure modes.** Measuring concurrency when the real limit is accept rate (hold them as separate axes); attributing generator-side TLS cost to the relay.

### E2 — DNS-driven placement: peers home where we tell them

**What.** That the placement controller can override latency-based relay selection and home a peer to an assigned relay via its published record.

**Why.** This is the load-bearing mechanism for the whole architecture. If we cannot control placement, sharding by namespace is impossible and we fall back to the co-location lottery.

**Method.** Stand up the placement controller (§5). For the fast loop, inject records with `MemoryAddressLookup`. For the integration loop, publish real records to the custom DNS origin and have peers resolve via `DnsAddressLookup::builder(origin)`. Assign peer B to relay-2 explicitly; have peer A resolve B and confirm the relay path used is relay-2, not A's lowest-latency relay.

**Vary.** Assigned relay vs. lowest-latency relay (make them different by adding artificial latency to the "natural" choice with tc netem); record TTL.

**Measure.** Fraction of peers that home to the assigned relay; time from record publish to peer honoring it; behaviour when the assigned relay is *not* the lowest-latency one (does the peer obey placement or drift?).

**Expected [HYPOTHESIS].** Peer uses the relay in its resolved record. If a peer self-publishes and *also* measures latency, confirm which wins — this determines whether placement must be server-published (controller writes the record) or can be peer-cooperative.

**Failure modes.** Peer caching a stale record past TTL; peer falling back to a default relay map when resolution fails (must fail closed to "no relay" or to the assigned one, never to public).

**Decision gate.** Resolve the open question here: **server-published placement** (controller writes records on peers' behalf, fully authoritative) vs. **peer-cooperative** (peers publish, controller only runs the origin). Server-published is more controllable and is the recommended default unless E2 shows peers reliably honor a relay hint they publish themselves.

### E3 — Namespace sharding with fan-out sync (the core thesis)

**What.** That placing a whole namespace's members on one relay shard lets any member converge by meeting *some* local peer, with no point-to-point requirement, and that this makes co-location automatic rather than a fought constraint.

**Why.** This is the reframing that dissolves the no-mesh problem for eventual-sync. If it holds, sharding-by-namespace gives us horizontal scale, free group accounting, and no cross-shard pair failures simultaneously.

**Method.** Define K namespaces. Assign all members of namespace i to relay-(i mod 3) via the controller. Run a real eventual-sync workload per namespace — Automerge document with concurrent edits, or Willow/WGPS namespace sync. Force relay-only paths (relay-only addresses) so we measure the relay-mediated case, the hard one. Verify convergence without any two specific peers being required to meet.

**Vary.** Namespace population (10, 50, 200, 1000 members); edit/write rate; churn (members join/leave during sync); fan-out degree (how many partners each peer syncs with).

**Measure.** Convergence time vs. population; per-peer time-to-first-useful-partner; relay bytes per converged update; `send_packets_dropped` (must stay ~0 within a correctly-sharded namespace — non-zero means a placement miss).

**Expected [HYPOTHESIS].** Convergence time grows sub-linearly with population given fan-out (gossip-like), and relay cost per update is dominated by fan-out degree, not population. Dropped packets stay near zero because every member of the namespace is co-located.

**Failure modes.** A member mis-placed onto the wrong shard (shows as dropped packets and a peer that never converges); fan-out degree set so high the relay carries redundant copies; treating a presence/liveness signal (which *does* need specific live peers) as if it were eventual-sync.

### E4 — LVS frontend: HA and within-shard persistence

**What.** That LVS provides HA and within-shard load balancing *without* breaking co-location, because DNS placement has already chosen the shard before the connection reaches the director.

**Why.** Earlier analysis flagged that naive L4 round-robin scatters co-located peers. DNS placement removes that problem: LVS no longer chooses the shard, it balances within a shard's process pool. We need to prove the division of labor holds and that persistence keeps a peer pinned across reconnects.

**Method.** Put lb-1 (IPVS) in front of relay-1's process pool (multiple `iroh-relay` processes from E1, if E1 justified multiple). Use a persistence mode (`-p` timeout, or source-hash `sh`) so a given client re-lands on the same backend process across reconnects. Drive reconnect churn and confirm pinning.

**Vary.** Scheduler {rr, wrr, sh}; persistence timeout; backend process count; reconnect rate.

**Measure.** Fraction of reconnects that re-land on the same backend (persistence effectiveness); load distribution across backends; whether co-located namespace members stay co-located on the same *process* within the shard (they must, or intra-shard relay fails at the process level).

**Expected [HYPOTHESIS].** Source-hash persistence keeps peers pinned. But note the sub-trap: co-location must hold at the *process* level, not just the node level — if a namespace's members land on different processes behind the same LVS VIP, relay fails exactly as if they were on different nodes. So either run one process per shard VIP, or make the persistence key co-locate a whole namespace (hard at L4). **Likely finding: one process per shard VIP is simpler than trying to make L4 persistence namespace-aware.**

**Failure modes.** L4 can't see the namespace key (it's not in the L4 header), so `sh` on source IP scatters a namespace across processes — the same packets-dropped signature as a placement miss. Watch `send_packets_dropped` per process.

### E5 — Splintered processes + cgroup group accounting

**What.** That running one relay process per group/cooperative under its own cgroup slice gives hard resource isolation and makes accounting fall out for free (per-process metrics = the group's bill).

**Why.** This is the group-accounting idea. If usage is partitioned by process, a noisy cooperative can't starve others, and billing needs no per-endpoint attribution — you read the process's counters.

**Method.** M relay processes, each in a `memory.max` + `cpu.max` cgroup slice, each serving one namespace/group (placement via E2). Drive asymmetric load: one group hammered, others nominal.

**Vary.** Per-slice limits; the asymmetry of the load; number of slices per box.

**Measure.** Isolation: does the hammered group's slice cap its own RSS/CPU without degrading neighbors' latency/throughput? Accounting fidelity: does per-slice `relay_bytes_*` match an independent count of that group's traffic?

**Expected [HYPOTHESIS].** cgroup isolation holds for memory and CPU; a group that hits `memory.max` degrades only itself (OOM-kills its own process, which clients then reconnect away from or retry). Per-process metrics are an accurate group bill.

**Failure modes.** Shared kernel resources (NIC, conntrack, ephemeral ports) leak cross-slice contention that cgroups don't bound — this is what tc in E6 addresses; noisy-neighbor via the network even when CPU/mem are isolated.

### E6 — tc traffic shaping: egress fairness and matchmaking priority

**What.** That `tc` (HTB + netem + DSCP) bounds the network dimension that cgroups can't, and keeps latency-critical matchmaking traffic snappy while bulk relay passthrough is capped.

**Why.** `client_rx` only throttles ingress, app-layer. The uplink is the wall for active passthrough, and that's egress. tc gives hierarchical egress fairness across the splintered processes from E5 and lets matchmaking win over bulk.

**Method.** HTB tree on the relay NIC egress: one class per group's process (guaranteed rate + borrowable ceiling). Optionally DSCP-mark coordination/handshake traffic into a high-priority class and bulk relay forwarding into a bulk class. Run E5's asymmetric load underneath.

**Vary.** Per-class guaranteed/ceiling rates; with/without DSCP prioritisation; degree of oversubscription.

**Measure.** Whether matchmaking tail latency stays flat while the bulk class is saturated; whether HTB borrowing gives idle groups' bandwidth to busy ones without starving guarantees; how QUIC's own congestion control reacts to tc-induced delay/loss (graceful backoff vs. thrash).

**Expected [HYPOTHESIS].** HTB egress isolation closes the network noisy-neighbor gap left by E5. DSCP priority keeps connection setup fast under bulk load. QUIC backs off gracefully to shaped capacity rather than collapsing — but verify, since tc-induced loss interacts with QUIC loss recovery in ways worth observing directly.

**Failure modes.** Shaping the wrong direction (remember ingress is app-layer `client_rx`, egress is tc); classifying encrypted relay traffic incorrectly (you can mark by process/cgroup via `cgroup` qdisc filters rather than trying to DPI the encrypted stream); QUIC interpreting shaping as congestion and over-throttling.

### E7 — Placement churn / split-brain window

**What.** The transient partition when a namespace is reassigned from one shard to another (rebalance, or shard failure): members re-home at different times, and during the window fan-out can't see across the split.

**Why.** This is the sharp edge of dynamic placement. We need to know how long the window is, whether sync tolerates it, and whether a straddling superpeer closes it.

**Method.** Under active E3 sync, force a namespace reassignment (publish new `relay=` records pointing members from relay-1 to relay-2). Vary how synchronized the re-homing is. Then repeat with a superpeer connected to *both* relays during the migration.

**Vary.** TTL (controls re-home speed); migration strategy (flash-cut vs. staggered); presence/absence of a straddling superpeer; sync type (Automerge eventual vs. live presence signal).

**Measure.** Duration of the partition window; convergence delay introduced; whether a straddling superpeer eliminates the partition; data loss (should be none for CRDT/eventual; presence signals may gap).

**Expected [HYPOTHESIS].** Window ≈ TTL + reconnect time. Eventual-sync tolerates a multi-minute window with only delayed convergence and no loss. Live presence/signaling does *not* tolerate it. A straddling superpeer closes the gap by bridging both shards during migration.

**Failure modes.** TTL set so long the window is unacceptable; assuming all workloads tolerate the window (presence doesn't); superpeer itself becoming the placement bottleneck.

### E8 — Multi-namespace peers vs. superpeer bridge (the relay/superpeer fork)

**What.** Two ways to serve a peer that belongs to multiple namespaces living on different shards: (a) the peer holds connections to multiple relays and fans out across them itself, vs. (b) a superpeer straddles the shards and bridges, keeping individual peers single-homed.

**Why.** This is where we decide the narrow, defensible role for superpeers. The fork has a governance dimension too: a relay is dumb infrastructure that can't read traffic; a superpeer is a trusted protocol participant a cooperative runs and is accountable for. We want the engineering numbers to inform the governance choice.

**Method.** Construct peers in N namespaces each. Approach (a): peer dials peers across all its namespaces' relays, holding N relay connections. Approach (b): single-home each peer, run a superpeer on the shards that carries cross-namespace state. Compare.

**Vary.** N (namespaces per peer); fraction of peers that are multi-namespace; superpeer count and placement.

**Measure.** Approach (a): extra held-connection cost per multi-namespace peer (memory, keepalive traffic) on both peer and relays. Approach (b): superpeer resource cost, the trust/exposure surface (a superpeer that bridges sees more), and whether a small number of superpeers suffices. Convergence quality under both.

**Expected [HYPOTHESIS].** For a low multi-namespace fraction, multi-relay peers (a) are cheaper and avoid introducing a trusted component. As cross-namespace density rises, superpeers (b) win on aggregate connection count but concentrate trust and cost. The crossover density is the number that tells Croft when superpeers stop being optional.

**Failure modes.** Conflating "peer needs offline data" (genuinely superpeer/sync territory — a relay can't help when there's no live peer) with "peer needs a live cross-namespace partner" (relay-able via multi-homing). Measure the offline-data fraction separately; it's a different requirement wearing a similar costume, and no relay tuning addresses it.

### E9 — Meer confidentiality tiers and the no-mirror group

**What.** The cost and capability of each point on the confidentiality dial from §3b: blind mirror (Tier 0), double-enveloped mirror (Tier 1), semantic meer (Tier 2), and the no-mirror fixed-list group. We test all four because Croft will offer them as a per-group policy choice, not pick one globally.

**Why.** The whole "always-on node that shuffles encrypted state without reading it" premise rests on Willow's metadata-vs-payload split (§3b). We need to confirm in the lab that a blind mirror actually drives convergence using only metadata, measure what each added envelope costs in reconciliation capability, and quantify the reliability hit of running no mirror at all. This turns the security/reliability tradeoff from an argument into a measured curve a cooperative can vote on.

**Method.** Reuse the E3 namespace sync workload. Configure the meer at each tier:

- Tier 0: meer holds encrypted payloads + cleartext join-metadata (timestamps, digests, namespace/subspace ids, paths per the key scheme). It runs range reconciliation but holds no payload key. Assert it never receives a payload key and that convergence still completes.

- Tier 1: add the outer envelope keyed to the authorised exchange set. Meer strips outer, verifies sender authorisation, stores inner-sealed. Vary how much sync metadata is pushed into the inner envelope and watch reconciliation degrade.

- Tier 2: meer holds the payload key, merges/serves. This is the §3a merge-CPU case.

- No-mirror: fixed-list peer group, defense in depth — connection allowlist at the accept side plus namespace capability (Meadowcap/MLS). No meer in the set. Members converge only when co-online.

**Vary.** Tier; for Tier 1, the inner/outer metadata split; for no-mirror, the overlap schedule of when members are online together (from fully overlapping to disjoint).

**Measure.** Convergence completion and time per tier; reconciliation efficiency (bytes/round-trips to converge) as sealing increases — expect it to worsen Tier 0 → Tier 1 as the mirror loses metadata visibility; for Tier 2, the merge-CPU from §3a; for no-mirror, the fraction of updates that *never* converge as a function of online-overlap, which is the reliability cost stated plainly. Across all tiers, log exactly what metadata the meer observed (namespace ids, sizes, timing) to document the residual leak even at Tier 0.

**Expected [HYPOTHESIS].** Tier 0 converges using metadata alone, confirming the blind-mirror premise. Tier 1 converges but with more round-trips/bytes the more metadata is sealed, and at full sealing the mirror degrades to dumb storage (can replicate, can't help reconcile). Tier 2 converges fastest but is the only tier exposing plaintext. No-mirror converges fully when online-overlap is sufficient and leaves a measurable, growing un-converged fraction as overlap drops to zero.

**Failure modes.** Assuming "encrypted" means "zero knowledge" — Tier 0 still leaks the metadata in §3b, and the experiment must record it rather than wave it away; order-preserving timestamp leakage being worse than expected (test the day-resolution and logical-counter obfuscations); a no-mirror group that silently falls back to a relay-stored path (assert no state persists anywhere outside the listed peers).

> **Meer build:** E8/E9 are the meer's *behaviours*; the meer binary's scope, its three blind roles
> (message broker / RoQ SFU / MoQ relay), the confidentiality dial, and the build phases (P0–P6) are
> designed in `../../discovery/thinking/meer-superpeer-design.md`. P1 (Tier-0 blind message mirror) is
> the first real milestone and makes E9 Tier 0 a running binary.

---

## 4a. Media round (real-time audio/video) — E10–E12

The media path rides the same iroh transport, NAT traversal, relay placement, and blind-meer shape as
the messaging spine; design in `../../discovery/thinking/realtime-media-over-iroh.md`. Two protocols,
two interaction types: **RoQ** (conversational) and **MoQ** (broadcast). These experiments convert the
media unknowns into numbers and reuse the E6 netem rig and the E5 cgroup/metrics machinery.

### E10 — Conversational media floor: RoQ over iroh under network stress (the C1 test)

**What.** Reproduce n0's **`callme`** (or a minimal `iroh-roq` clone: Opus + cpal over the QUIC
**datagram** flow) between two AWS boxes and the NAT'd Mac, and drive it through the **E6 `tc netem`
rig** (delay / loss / bandwidth-cap).

**Why.** The transport primitive is confirmed (`iroh-roq` uses `conn.send_datagram`), but the **one
live technical unknown** is whether iroh's QUIC congestion control / pacing **fights** the media path
under loss — and whether audio stays intelligible across our real NAT fabric. This is the cheapest
attack on the biggest media unknown, using the most-proven component.

**Method.** Baseline (no netem) → +delay → +loss (10/30%) → bandwidth-cap, mirroring E6. Run a raw-UDP
baseline alongside to isolate any iroh-CC interference. **Headless note:** the AWS boxes have no audio
devices, so drive a **synthetic Opus frame source** (generated/looped fixture) into the RoQ datagram
path instead of `cpal` device capture. Simulating the media transaction is standard, trustable media
perf-testing practice — the congestion/latency/jitter measurement is unaffected by where the frames
originate; only the box↔NAT-Mac legs need a real device if we want a true end-to-end ear test.

**Measure.** One-way latency / mouth-to-ear proxy; packet-loss-concealment events; jitter-buffer depth;
whether the bitrate estimator converges to the netem cap or over/undershoots (the "two controllers
fight" detector). **Vary.** netem condition; mesh (direct, needs E0-NAT ingress) vs meer-relayed.

**Expected [HYPOTHESIS].** Audio holds through 30% loss with visible (not silent) degradation, as E6
showed for messaging; CC interaction is tolerable on the datagram flow. **Failure modes.** QUIC pacing
hides loss from the estimator → overshoot; reliable-stream accidentally used → HOL latency; mesh path
untestable until E0-NAT hole-punch ingress is opened (then it runs meer-relayed only).

> **MEASURED (2026-06-17, `relay-lab-runs/E10-roq-netem-2026-06-17/`, harness `roq-send`/`roq-recv`).**
> Synthetic CBR (64 kbps / 20 ms / 160 B datagrams, sequence + timestamp stamped) over the iroh 1.0
> datagram path (`conn.send_datagram`; `max_datagram_size` negotiated 1162 B), DIRECT node-1→node-2,
> through the E6 netem rig. **The C1 unknown is RESOLVED (characterized):**
> - **Loss/delay are transparent — the controllers do NOT fight.** netem loss passes straight through
>   as sequence gaps (5%→4.6%, 30%→30.9%), path RTT stays flat (~2 ms) and tracks added delay exactly
>   (100 ms→102 ms), jitter sub-ms. Audio holds to 30% loss with visible (31% PLC) — never silent
>   degradation. The estimator gets accurate loss + RTT + jitter. **No `send_datagram` errors, no HOL.**
> - **Over-cap source bufferbloats — iroh will NOT rate-adapt for you.** At a 40 kbit cap (below the
>   ~84 kbit wire rate) iroh QUEUES + PACES datagrams IN ORDER rather than dropping to fit: RTT balloons
>   537→8829 ms and the receiver gets a CONTIGUOUS, ever-delayed prefix (208/1000 frames, **zero gaps**,
>   ~11 kbps = link rate). `send_datagram` returns `Ok` throughout (drops oldest silently when its own
>   buffer fills — even an 8 KiB `datagram_send_buffer_size` never errors); receiver-side loss reads 0%
>   while 70–80% of the stream never arrives in time.
> - **Design consequence.** The media engine's bitrate estimator MUST be authoritative and MUST back
>   off on the **path-RTT trend** (+ per-stream sequence loss + arrival jitter) — exactly the C1
>   proposed-solution point 2. You cannot rely on `send_datagram` back-pressure (it doesn't error) nor on
>   receiver loss alone (the delayed prefix shows none). All three needed signals are exposed and
>   accurate. Failure-mode "QUIC pacing hides loss → overshoot" is CONFIRMED specifically for the
>   bandwidth-cap case, and the mitigation (RTT-driven estimator) is identified. Reliable-stream HOL
>   failure mode did NOT occur (datagrams never fall back to streams).

### E11 — Broadcast media: MoQ pub/sub lazy fan-out

**What.** Stand up **`iroh-live`** (moq-rs + GStreamer/ffmpeg, h264/Opus) as a one-to-many broadcast:
one publisher, N subscribers, fan-out through a **MoQ relay** (a meer role).

**Why.** Broadcast media is a *different* interaction type (stage/watch-party). Validate the **lazy**
property (no encode/transmit until a subscriber connects — the battery/privacy win) and measure
fan-out cost + the relay's blindness (it forwards Tracks it needn't decode).

**Method.** Start publisher with zero subscribers (confirm no encode/egress); attach subscribers
incrementally; sweep N. **Measure.** Time-to-first-frame on subscribe; per-subscriber bandwidth and
relay CPU/NIC vs N; confirm zero publisher egress while no subscriber is attached (lazy); confirm the
relay holds no media key. **Vary.** N subscribers; with/without the NAT Mac; bitrate/codec.

**Expected [HYPOTHESIS].** Lazy holds (no traffic until subscribe); fan-out is relay-NIC-bound (the E0
active-passthrough wall, heavier than messaging); the MoQ relay stays blind. **Failure modes.** Eager
encode defeating the lazy claim; the relay needing to parse media (not just Tracks) for fan-out
decisions; fan-out at large N hitting the broadcast-tier wall (then hand off / cascade relays).

> **Abuse-sensitive tier.** Broadcast is the mass-distribution surface that got Rave pulled
> (`../../discovery/thinking/abuse-resistance-and-the-rave-trap.md`). E11 must *also* measure the
> **scale/admission policy a BLIND meer can enforce from metadata alone** (max audience, fan-out rate,
> members-only) — never content. The lever against piracy/abuse here is scale + peer restriction, not
> inspection; capture what the relay can refuse to serve without reading a byte.

> **MEASURED (2026-06-17, `relay-lab-runs/E11-moq-lazy-2026-06-17/`, `crates/moq-lazy-spike`).**
> **characterized — the relay LOGIC is green** (full moq-rs/GStreamer + transport-carried deferred as
> meer P5, de-risked by n0's iroh-live + the proven iroh fabric). Deterministic in-process
> publisher/relay/subscriber, ChaCha20-Poly1305 opaque frames:
> - **Lazy.** Publisher egress = **0** across 100 produce-ticks with no subscribers; it produces the
>   instant one attaches — "nothing to fan out if nobody is watching" (the battery/compute/privacy win).
> - **Fan-out cost linear in N.** relay egress for a 10-frame track = `(0→0, 1→10, 3→30, 10→100)`, zero
>   when unwatched.
> - **Blind.** The relay forwards opaque frames, `payload_keys_held=0`; every subscriber decrypts
>   **locally** (end-to-end blindness, the media analog of the message blind broker).
> - **The abuse lever (Rave trap).** A max-audience cap (5) refuses 3 of 8 join attempts and a
>   members-only policy refuses 1 non-member of 3 — **all from subscribe metadata alone, reading zero
>   frame bytes.** Piracy/abuse resistance here is scale + peer restriction, never content inspection.
> Open: real moq-rs Tracks / GStreamer-Opus / WebTransport browser reach (assembly, not build);
> transport-carried = meer P5; TC-META1/2 (the AR-4-for-media bound + CBR padding) are separate.

### E12 — Blind media-meer: SFrame-over-MLS through a forwarder that can't read it

**What.** The SFU/MoQ meer forwards **SFrame-encrypted** media keyed off a real **MLS** group; verify
it forwards (header-only) without ever recovering plaintext, and that media revocation tracks
membership.

**Why.** This is the media analog of the faithful path + MD-G5: a forwarding meer must be **blind**
(E3.4 for media) and a **revoked member's subsequent media must be undecryptable** while pre-revocation
frames already received stay decryptable.

**Method.** Reuse the openmls / faithful-path machinery for per-sender SFrame key derivation
(`realtime-media-over-iroh.md` C3). Send a frame stream with 10% loss + reorder; do a mid-stream
membership change. **Measure.** Receivers decrypt surviving frames + reject replays (loss-tolerant, not
contiguity-requiring); a **non-member cannot derive any key** (media `UnauthorizedAuthor`); the meer
recovers **zero** plaintext from headers alone; post-revocation frames undecryptable to remaining
members. **Vary.** loss/reorder rate; revoke timing. **Expected [HYPOTHESIS].** All hold — media
keying is message keying with loss-tolerance. **Failure modes.** SFU needing payload (not just header)
for layer selection; a rekey gap dropping audio at the epoch boundary; replay window too tight under
real loss.

> **MEASURED (2026-06-17, `relay-lab-runs/E12-sframe-mls-2026-06-17/`, `crates/media-sframe-spike`).**
> **green-real — all four TC-KEY pass against a REAL openmls 0.8.1 group.** Per-sender SFrame base key =
> `HKDF-SHA256(MLS exporter_secret, "croft/sframe/v1" ‖ epoch ‖ leaf)`; frames are ChaCha20-Poly1305 with
> the clear `(epoch, leaf, counter)` header bound as AAD.
> - **TC-KEY1.** Two senders (alice, carol) derive **distinct** keys from the same group secret + their
>   leaf indices; a **non-member cannot derive any key** — it has no group, so `epoch_proof()` errors (the
>   media analog of the faithful-path `UnauthorizedAuthor`, from signed/keyed state alone).
> - **TC-KEY2.** 100 frames, 10 dropped, adjacent-pair reordered: **90/90 surviving frames decrypt OUT OF
>   ORDER**; replaying the whole stream is **rejected 90/90** by a sliding per-sender counter window. Loss-
>   tolerant, no contiguity requirement (unlike the message hash-chain).
> - **TC-KEY3.** Removing carol advances the MLS epoch 1→2 and **rotates the exporter secret**; removed
>   carol is stuck at epoch 1 (no S2), her later frame is **rejected** (stale epoch + non-member = media
>   **MD-G5**), and she **cannot forge** an epoch-2 frame; meanwhile bob's cached **pre-revocation frame
>   still decrypts** (history not clawed back) and a current member still sends/decrypts at epoch 2.
> - **TC-KEY4.** The **blind SFU** holds no secret: it routes/selects all 92 frames from the clear headers
>   yet recovers **0 plaintext bytes** (ciphertext never contains plaintext) — extends the E3.4/AR-4 blind
>   property to media.
> The hypothesis holds in full. Failure modes did NOT occur: header-only selection sufficed (TC-KEY4); no
> rekey gap (pre-revoke frames retained across the epoch boundary, TC-KEY3); the width-16 window survived
> 10 % loss + reorder (TC-KEY2). **C3 keying is de-risked.** Open: synthetic frames (not real Opus/RTP);
> modeled SFrame header (not the RFC 9605 wire format); run locally (a transport-carried version = this
> keying over the E10 datagram rig, a small follow-on).

---

## 5. Build list for Claude Code

Extend the existing `relay-loadtest` crate. New components, in dependency order:

1. **E0 generator mode** (`loadgen --mode matchmaking`): normal connect with hole-punch allowed (do *not* use a relay-only address), hold, record whether the live path is relay or direct via `conn.paths().iter().any(|p| p.is_relay())`. Lets E0 measure the matchmaking-vs-passthrough gap.

2. **Placement controller** (`ctrl/`): owns endpoint-id → namespace → relay-url mapping. Two backends behind one interface:

   - fast/in-process: build `EndpointInfo` and inject via `MemoryAddressLookup::add_endpoint_info` for unit-speed iteration.

   - integration: a custom DNS origin serving `_iroh.<z32-id>.<origin>` TXT records with `relay=<url>`, plus a pkarr publish path; peers resolve via `DnsAddressLookup::builder(origin)`. Use `EndpointInfo::with_relay_url` to set the assignment.

   Expose an HTTP control API the orchestrator calls to (re)assign a namespace to a shard and to flush/adjust TTL — E2 and E7 drive this.

3. **Relay access HTTP hook** (`ctrl/admit`): the endpoint the relay's `access.http.url` POSTs to. Implements the rolling-allowance gate: look up the posted `X-Iroh-Endpoint-Id`, check it belongs to the namespace this shard serves and is within quota, return `200`+`true` or deny. This is the authenticated wall behind DNS placement.

4. **Namespace sync workload** (`sync/`): a real eventual-sync driver. Start with Automerge (concurrent edits to a shared doc, measure convergence) since it's already in the Croft stack; add a Willow/WGPS namespace driver if available. Must support forcing relay-only paths and measuring per-peer time-to-first-partner and convergence time.

5. **Meer binary** (`meer/`): an always-on iroh endpoint that participates in the sync protocol rather than just forwarding. Holds one or more namespaces' state (Automerge docs / Willow namespaces), accepts sync sessions, and emits the `meer_*` metrics from §3. Two placement modes: single-shard (homes to one relay, serves one namespace's offline-data demand) and straddling (connects to multiple relays to bridge namespaces, for E7 migration and E8 fork). Three confidentiality tiers (E9): Tier 0 blind (holds encrypted payloads + cleartext join-metadata, runs range reconciliation, never holds a payload key); Tier 1 double-enveloped (strips an outer authorised-set envelope, stores inner-sealed); Tier 2 semantic (holds the key, merges/serves). The binary must assert at Tiers 0/1 that it never receives a payload key, and log exactly what metadata it observed. This is the component E3, E7, E8, and E9 exercise; without it those experiments only measure the live-peer case and miss the offline-data demand entirely.

   Also build the **no-mirror group harness**: a fixed-list peer group with defense in depth — connection allowlist at the endpoint accept side plus namespace capability (Meadowcap/MLS), no meer in the set — for the E9 no-mirror arm. It must assert no state persists outside the listed peers.

6. **Orchestrator** (`orchestrator/`): reads an experiment manifest (which experiment, node assignments, parameters, iroh SHA), provisions relay and meer processes (with cgroup slices for E5), configures LVS (E4) and tc (E6) via templated config, drives the generators, sync workload, and meers, scrapes all of §3 (relay, meer, host, generator) into the per-run directory, and emits the charts each experiment specifies. Everything reproducible from the manifest.

7. **Config templates**: `relay.toml` per shard (access mode, `client_rx` limits, metrics bind); meer config (namespaces held, home/straddle relays); IPVS config (scheduler, persistence) for E4; tc/HTB+netem scripts for E6; cgroup slice units for E5.

Reuse from the existing crate: the relay-only `EndpointAddr::from_parts(id, [TransportAddr::Relay(url)])` passthrough technique, the ramped connection establishment, and the metrics-scraping/SSH-driving patterns in `sandbox-transcripts/VALIDATION-METHODS.md`.

Compile-check note: the lab nodes need `static.rust-lang.org` reachable for the toolchain, and `crates.io` for deps. Pin the iroh git SHA in every `Cargo.toml` and record it in each run manifest.

---

## 6. What a complete result looks like

By the end you should be able to state, with measured numbers at a fixed iroh SHA:

- The per-connection relay cost for matchmaking-then-direct vs. idle-relayed vs. active-relayed, and the concurrency/throughput crossover that sets per-instance capacity (E0).

- Whether to run one fat relay process per node or several, and the accept-rate vs. concurrency ceilings of each (E1).

- That placement is controllable, whether it must be server-published or can be peer-cooperative, and the publish-to-honor latency (E2).

- That namespace sharding makes co-location automatic for eventual-sync, with convergence-time-vs-population and relay-cost-per-update curves (E3).

- The correct division of labor between DNS placement and LVS, and the one-process-per-VIP finding if it holds (E4).

- That cgroup slices give hard CPU/mem isolation and free per-group accounting, and where they leak (E5).

- That tc closes the network noisy-neighbor gap and keeps matchmaking snappy under bulk load, and how QUIC reacts to shaping (E6).

- The placement-churn partition window, what tolerates it, and whether a straddling superpeer closes it (E7).

- The cross-namespace density at which superpeers stop being optional, plus the separately-measured offline-data fraction that no relay tuning can serve (E8).

- A meer's capacity expressed in fan-out: convergence time provided, concurrent sync sessions, CPU per merge, and state-growth rate — on the same host axes as the relay, so a given box's capacity reads as "N relayed connections" vs. "M converged peers" (§3a, E3/E8).

- The confidentiality/reliability curve: that a blind mirror converges on metadata alone, what each added envelope costs in reconciliation efficiency, and the measured un-converged fraction of a no-mirror group as online-overlap drops — the tradeoff a cooperative votes on, stated in numbers (§3b, E9).

That set is a defensible, measured map of the relay-and-superpeer landscape for Croft, with the relay/superpeer boundary drawn on evidence rather than intuition.

---

## 7. Terminology

- **Relay** — stateless, single-hop, co-location-required forwarder of opaque encrypted frames. Cannot read traffic. Dumb infrastructure; rentable/federatable without trust.

- **Meer** — an always-available peer that participates in the sync protocol and serves convergence for peers whose data owner is offline. Runs at one of three confidentiality tiers: blind (encrypted payloads + cleartext join-metadata only, never reads content), double-enveloped (even less metadata, less reconciliation ability), or semantic (holds the key, reads content — a trusted component a cooperative is accountable for). Being always-on does **not** imply reading content; that is the tier choice. Name from *modafinil* (the wakefulness drug pilots and military use to stay alert): a meer is the peer that stays awake so the others can sleep and still converge. Spelling note: the drug is **modafinil**.

- **Blind mirror** — a Tier 0 meer. Stores and serves encrypted payloads and drives range reconciliation using only the metadata Willow cannot encrypt (timestamps, digests/lengths, namespace/subspace ids, paths). Sees that-and-when, never what.

- **No-mirror group** — a fixed-list peer set with no meer in it, enforced by connection allowlist + namespace capability. Converges only when members are co-online; the privacy-maximal, reliability-minimal end of the dial.

- **Placement controller** — the service that decides endpoint → namespace → relay assignment and publishes it via DNS/pkarr records. Proposes placement; does not enforce it.

- **Admit hook** — the relay's authenticated `on_connect` gate (HTTP variant). Enforces admission against the cryptographically-proven endpoint ID. The wall that DNS placement only proposes.
