# Croft validation campaign — detailed test log

Chronological log of the SSH-driven multi-node validation session executing
`META-TRANSCRIPT-next-session.md`. Each entry records the method, the exact commands (or their
shape), the key outputs, and the conclusion. This log is the **source of record** that the final
conclusions, `Proofs/`, `crystallized/proof-ledger.md`, and the roadmap are derived from when the
campaign closes. Spine/plan: `TESTING-DESIGN.md`. SSH mechanics: `sandbox-transcripts/VALIDATION-METHODS.md`.

- **Session date:** 2026-06-15
- **Driver:** Claude Code on the Mac, over SSH to three AWS sandbox boxes + the Mac itself as a NAT'd 4th node.
- **Identity for any commits:** `chasemp` / `chase@owasp.org` (Mac only, on explicit approval).

Status legend: done (✅) · in progress (~) · todo (⏳)

---

## 0. Network position & provisioning ✅

**Goal:** determine node-three's network position (it shapes whether B exercises real NAT
traversal), confirm UDP 2112 reachability, and bring all nodes to a buildable state.

**Findings:**

| node | public IP | VPC IP / AZ | subnet | position |
|---|---|---|---|---|
| node-1 | 54.172.175.109 | 172.31.43.122 / us-east-1c | 172.31.32.0/20 | same VPC `vpc-217f0f5c` |
| node-2 | 34.207.146.151 | 172.31.19.13 / us-east-1b | 172.31.16.0/20 | same VPC |
| node-3 | 3.84.55.217 | 172.31.88.18 / us-east-1a | 172.31.80.0/20 | **same VPC** |
| node-4 | Mac | behind real NAT | — | **off-VPC** |

- All three AWS boxes share one VPC and Security Group. node-3 is **same-VPC** (the contingency the
  plan flagged) → it alone cannot prove NAT traversal. **Decision:** the Mac (node-4), behind a real
  NAT, is the off-VPC peer for Part B's hole-punch/relay tests. Local Rust contained under
  `experiments/` (`CARGO_TARGET_DIR`/`CARGO_HOME`) so the workstation isn't polluted.
- **UDP 2112 reachability test** (method in VALIDATION-METHODS.md): a detached `python3` UDP sink on
  node-2:2112, datagrams fired from node-1 and node-3 over the private path. Result: node-1
  (172.31.43.122) and node-3 (172.31.88.18) each delivered **3/3** → SG allows UDP 2112 among all
  three; the rule was extended to node-3.
- **Provisioning:** node-2 was ready (repo + cargo on `/mnt/data` EBS + prior `test-5g.bin`). node-1's
  repo dir was gone → re-cloned; cargo/target/5G artifacts survived on `/mnt/data`. node-3 was bare
  (128 GB root, no `/mnt/data`) → installed rustup stable + build-essential.

**Conclusion:** topology fixed (3 same-VPC + 1 NAT'd Mac); UDP 2112 open; all nodes buildable. The
real NAT-traversal evidence must come from node-4, recorded as such.

---

## 0b. Workflow decision ✅

`AltID/alt.drive` is the **dead** prior coordination repo. Going forward: **this repo
(`CroftC/experiments/iroh`) is the source of truth**; boxes are **keyless compute** (tar-over-ssh
sync, no git remote, no live deploy key); commits happen on the Mac only, on approval. The iroh spike
crates were imported into this repo from node-2; the build pipeline was validated (blob spike builds
clean on rust 1.96, 10.6 s incremental). The `id_secroute` deploy key copied to node-3 during setup
was removed (dead repo).

---

## Part A — fork + deterministic reconcile (gates the decentralization claim)

Substrate: the existing `Proofs/lineage-groups` workspace (Phases 0–3, 2.5, 2.6 all GO in sim).
A throwaway, TDD-exempt `reconcile-harness` CLI wraps the tested `lineage-core` to drive the logic
across real machines via op-log exchange. "Partition" = each box applies its op in isolation;
"reconnect" = op-log exchange; determinism = byte-identical verdicts across machines.

### A0 — deterministic suite as the experiment-suite substrate ✅

Ran `cargo test` on the full lineage-groups workspace: **39 passed, 18 suites, 0 failed**.
Mapping to the experiment-suite groups (`discovery/thinking/experiment-suite.md`):

- **Group A (ancestry/integrity):** `genesis_id_is_deterministic`, key sign/verify/no-forge,
  `e2_2_genesis_rules_immutable` (append-only / INV-IMMUTABLE-ADMIN), `a2_2_admin_equivocation…`,
  backfill forgery rejection (`a2_3a/b`, `e2_7`, `e2_8`).
- **Group B (complementary convergence):** `e2_3_partition_heals_and_converges`,
  `e2_survivor_and_detect_are_order_independent_fuzzed` (commutativity), `a2_1…fuzzed_delivery`,
  `e3_1_partition_reconnect_converges`.
- **Group C (contradiction hard-stop):** `e2_4_conflict_hard_stops`,
  `e2_5_rejected_merge_is_resting_state`, `e3_2_conflict_reconnect_hard_stops`,
  `e2_1_under_threshold_remove_rejected`, `a2_4_departed_admin_loses_authority`.
- **Group D (trap door):** `e2_6_fresh_genesis_inherits_logs_unreordered`,
  `e3_genesis_fork_recombine_demo`, `i9_fold_is_lossless_and_inert`, `e3_3_device_loss_recovery`.

**C4 (auto-resolve cannot be bypassed):** covered structurally — `conflict::detect`/`reconcile` only
*classify* and never change membership, and `quorum_override` *requires* an explicit,
threshold-meeting signed op per contested member (`a2_5`); there is no auto-merge code path to
bypass. Honest note: this is the invariant covered, not a literal "adversarial bypass attempt"
named test; the boundary is structural.

**Conclusion:** the deterministic substrate is green and exercises groups A–D. A0 satisfied at the
invariant level.

### A1 — 2-host fork ✅

Shared world: admins `{alice,bob,carol,dave}` (so honest partitions use **disjoint** signer sets per
branch), founders `{alice,bob,carol,dave,erin}`, Remove threshold 2, Add threshold 1. The 4-admin
world was a deliberate fix: a 2-admin world forced the same admin onto both branches (Remove needs
2 of 2), which the equivocation detector correctly flagged — masking the honest-partition case.

- **Genesis identical on all three machines:** `4f8c1f28c517171172ed85d195b4aeaf0e03166f54775e780479e6712c4162bc`
  (each box computed it independently from the same compiled code).
- **Contradiction** (node-1 keeps erin via `add frank`/carol | node-2 boots erin via
  `remove erin`/alice,bob): reconcile → `HardStop` · `RemovedThenIncluded(erin)` · loser preserved+attributed.
- **Complementary** (node-1 `add frank` | node-3 `add grace`, both keep erin): reconcile → `Heal`,
  deterministic survivor head `241d2718…`.

### A1b — 3-way fork across real separate machines ✅ (headline)

Scenario: node-1 `add frank`(carol), node-2 `remove erin`(alice,bob), node-3 `add grace`(dave),
each applied in isolation. Logs exchanged via the Mac, redistributed to every box. Each box ran
`reconcile n_one n_two n_three` **locally**.

- **Verdict byte-identical on all three independent machines:**
  `sha256 = 5d82a5df4890c7aaa2811bf866bae6b43188d3c1323e168c7c457861e819bf37` on node-1, node-2, node-3.
- Verdict (`Proofs/lineage-groups/part-a-evidence/verdict-canonical.json`): `fork_detected=true`,
  `distinct_heads=3`, `contradiction=true`, `contested=["erin"]` (RemovedThenIncluded(erin) on the
  1↔2 and 2↔3 pairs; 1↔3 = Heal), `survivor_order_independent=true`, `equivocations=[]`.
- **Reconnect/merge-order independence:** survivor `241d2718…` and contested `erin` invariant across
  all four merge orders tested (`[1,2,3]`, `[3,2,1]`, `[2,3,1]`, `[2,1,3]`).

**Conclusion (A1/A1b):** disconnected peers independently compute the same surviving state from only
the histories they hold; contradictions hard-stop with both branches preserved + attributed, never
auto-resolved; complementary divergence converges; convergence is merge-order-independent — **no
superpeer present.** The decentralization claim is demonstrated on real separate machines.

### A3 — two-mode superpeer test (capability vs right) ✅

node-3 designated always-on broker. File movement relayed via the Mac (boxes lack inter-box keys).

- **A3.1 durable queue:** node-1's commit (`add frank`) held by the broker (node-3) while node-2 is
  offline, then synced by node-2. Broker preserved exact bytes (sha `7a945964…` origin = broker =
  delivered). node-2's end-state (`state`) is **identical** whether the commit arrived via the
  broker (Mode 1) or directly (Mode 2): head `241d2718…`, members include erin+frank. → broker adds
  **availability** (a capability), not state.
- **A3.2 tamper-evidence:** the broker altered one signature char in the held log; node-2 **rejected**
  it on replay — `signature from did:4c26d9074c failed verification`. → the broker cannot alter held
  state undetectably.
- **A3.3 capability-vs-right crux:** a contradiction (`remove erin` | keep-erin `add grace`) routed
  through the broker (node-3) vs reconciled by a peer (node-1) produced the **identical verdict**:
  `sha256 = 5f79e0736fb2a0e53c5c52a65a9882c93740df02838dc18912c381bc171c7db7` (both `HardStop`,
  `RemovedThenIncluded(erin)`). → the broker has **no resolving power a peer lacks**.

**Verdict (A3):** the superpeer is a **capability, not a right**. No outcome reachable in Mode 1 is
unreachable in Mode 2 — the broker only adds availability/convenience. No leak.

### A2 — conformance ✅ (settled by A3)

For every broker shortcut, the no-broker path reaches the same end: durable-queue delivery ≡ direct
delivery (A3.1); concurrent **complementary** commits are commutative (merge-order independence,
A1b); a **contradiction** hard-stops identically with or without the broker (A3.3). No
reachable-only-with-broker outcome was found.

**Part A status: COMPLETE.** Detailed conclusions in `Proofs/lineage-groups/PART_A_RECONCILE_FINDINGS.md`;
evidence in `Proofs/lineage-groups/part-a-evidence/`.

---

## Part B — iroh transport spikes (~)

**B-setup — iroh builds on all four nodes (done):**
- Imported the iroh spike crates into this repo; made the blob spike's store path configurable
  (`SPIKE_STORE_DIR`, default `/mnt/data`) so node-3 (big root, no `/mnt/data`) and the Mac can run.
- node-3: cold build **2m21s** (2 cores), then 6s incremental after the store-path fix; 287M debug binary.
- node-4 (Mac, NAT'd): cold **contained** build **52s** (14 cores, rust 1.94.1, `CARGO_HOME`/`CARGO_TARGET_DIR`
  under `experiments/iroh/.node4-*`, gitignored); 66M debug binary. iroh `1.0.0-rc.1` builds fine on rust 1.94.
- node-1/2: rebuilt from the repo source (registry cached from prior alt.drive iroh work).
- iroh dep tree pinned by `Cargo.lock`: iroh `1.0.0-rc.1`, iroh-blobs `0.102`, iroh-docs/gossip `0.100`.

**Blob sizing decision:** the user is on hotel wifi, and only node-4 (Mac) transfers traverse it
(same-VPC box↔box data is AWS-internal). So: **1 GiB** for the AWS-internal tests (big enough that
resume is interruptible) and **250 MiB** for the off-VPC Mac leg. Verification (BLAKE3 integrity,
resume, multi-source split, NAT path) is size-independent; the prior run already touched 5 GB for raw
scale. Test files generated **deterministically** (AES-256-CTR keystream over zeros) so two seeders
produce byte-identical content (same blob hash) with zero cross-traffic: 1 GiB sha `d37dfb4c…`
(identical on node-1 & node-2), 250 MiB sha `0725bf6e…`.

### B1 — blob transfer

- **B1.1 baseline (done):** node-2 provides 1 GiB → node-3 fetches over the direct same-VPC path.
  rc=0, 1073741824 bytes, received sha `d37dfb4c…` == source (independent BLAKE3 verify). 44.4 s @
  184 Mbit/s (debug build — throughput is not the property under test).
- **B1.2 resume (done):** fetch killed mid-transfer (510 MB / half in the FsStore); restart against
  the **same store** completed in **24.2 s ≈ half** the full time (only the remainder fetched, not a
  restart), final sha `d37dfb4c…` == source. Resume confirmed. (Kill the fetcher via `fuser -k
  2112/udp` — `pkill -f 'altdrive-spike-iroh fetch'` self-matches the SSH command and kills the
  session.)
- **B1.4 off-VPC NAT (done — headline):** the NAT'd Mac (node-4) fetched the 250 MiB blob from
  node-2, which it **cannot reach directly**. rc=0, 262144000 bytes, sha `0725bf6e…` == source.
  128 s @ **15.6 Mbit/s** — the relay signature (vs 184 Mbit/s same-VPC direct); node-2's private IP
  is unreachable from off-VPC, so the path is relayed (n0 relay `use1-1`).
  - **Required a code fix** (verified against iroh-rc.1 source, not guessed): the original ticket
    embedded only the NodeId, so an off-VPC fetcher had no relay/address and failed in ~0.5 s.
    Fix: provider calls `endpoint.online().await` then builds the ticket from `endpoint.addr()`
    (full `EndpointAddr` = relay URL + direct addrs); fetcher seeds a `MemoryLookup::from_endpoint_info`
    from `ticket.addr()` so id→relay resolves immediately. This is the canonical iroh NodeAddr
    bootstrap and **directly informs B3 pairing**. Also added `tracing-subscriber` so `RUST_LOG`
    surfaces the path, and made the blob-store dir configurable (`SPIKE_STORE_DIR`).
- **B1.3 multi-source (done):** node-1 + node-2 both seed the identical 1 GiB (same blob hash
  `61a92911…`); node-3 fetches with both providers via a `Shuffled` set + a `MemoryLookup` seeded
  with both addrs.
  - **baseline:** rc=0, sha `d37dfb4c…` == source, 33.6 s — both providers available.
  - **failover (the strong evidence):** started the fetch, then **killed node-1's provider
    mid-transfer**; node-3 **still completed from the surviving node-2** (rc=0, sha `d37dfb4c…`,
    1073741824 bytes). Proves the fetcher genuinely uses the provider *set*, not one pinned source.
  - **Finding:** `SplitStrategy::Split` (simultaneous striping across providers) is for HashSeq
    *collections* — on a single large raw blob it expands to `size/32` (~33M) range-requests and
    **OOM-killed** node-3 (rc=137). Striping one blob across providers requires representing it as a
    HashSeq; recorded as a follow-on. The `Shuffled` redundancy/failover path is the right
    single-blob multi-source primitive and works.

**B1 COMPLETE** — baseline, resume, multi-source (redundancy+failover), and off-VPC NAT all pass with
independent BLAKE3/sha256 integrity verification.

### B-gossip — 3-node iroh-gossip mesh (done)

New crate `crates/altdrive-spike-gossip` (one node per host; bootstrap by reading a peer's
`EndpointAddr` JSON, seeding it into a `MemoryLookup`, passing its id to `gossip.subscribe`). Topic =
a fixed 32-byte `TopicId`. Binds **UDP 2112** (the only SG-open port — gossip on 2113 silently failed
to form a mesh because the SG blocks 2113; that mis-step is itself the SG finding).

- **Transitive delivery (done):** node-2 = hub; node-1 and node-3 bootstrap **only via the hub** and
  never exchange addresses. Result: the mesh became fully connected and **n1 received n3's broadcasts
  and n3 received n1's** (each `senders_received={hub, <other>}`). Messages propagate across a mesh
  formed from a single bootstrap point — epidemic broadcast (HyParView/PlumTree) confirmed.
- **Drop-a-node resilience (done):** killed the hub mid-run; afterward n1 still received n3's
  rounds 14–16 and n3 still received n1's rounds 15–17 (rounds after the kill). The mesh survives
  losing the bootstrap/relay node because membership had formed direct n1↔n3 links.

### B3 — pairing (demonstrated via the NodeAddr+TopicId bootstrap)

Both the blob NAT fix (B1.4) and the gossip mesh bootstrap from a **`NodeAddr` (relay URL + pubkey)
+ a random 32-byte `TopicId`**, with no direct IP required in the invite — exactly the Delta Chat
pairing pattern. So the pairing seam is demonstrated. Identity/key-recovery remains the open problem
(a fresh keypair per node here; binding devices to a recoverable identity is future work).

### B2 — iroh-docs characterization (done)

Built + ran the same-process 2-node `altdrive-spike-irohdocs` on node-2. iroh-docs **0.100.0**;
doc create → share-ticket → import (auto-sync) works. Node B converged to **8 of 10** entries within
the 60 s window — sync is *eventual*, and the tail can lag. **Willow-migration input (the key
finding):** iroh-docs is flat **last-writer-wins** — on a key conflict it silently *overwrites*,
which is exactly what the lineage/governance model must NOT do (it hard-stops and preserves
contradictions, see Part A). So raw iroh-docs is too weak for membership/governance; it's fine as a
dumb key-value sync layer but the lineage semantics need the richer model. (A cross-host 3-replica
run would extend the same-process spike — noted; the LWW-too-weak conclusion is architectural, not
timing-dependent.)

### Local-first history — multi-device & group voluntary backfill (done; user-requested)

New crate `Proofs/lineage-groups/crates/history-harness` over the tested `lineage-history`. Each
party keeps its own signed branch off a shared ancestor; sync = voluntary `backfill_import` that
absorbs a donor branch as a *separate navigable* branch (never interleaved).
- **Multi-device on the boxes** (node-1/2/3 = dev1/dev2/dev3): each absorbed the others → 3 separate
  navigable branches, identical ids across machines, no interleave; fold hides/restores losslessly.
- **Group on a box** (alice/bob/carol): the *same* mechanism — bob voluntarily absorbed alice+carol.
- **Rejection:** a tampered branch → `BadSignature`; an outsider (mallory) → `ForeignGenesis`.
- Verdict + capabilities: `Proofs/lineage-groups/LOCAL_FIRST_HISTORY_FINDINGS.md`; plain-language
  write-up of the whole campaign in `CAPABILITIES.md`.

### Re-formation backstop (A1 criterion 4) — done, cross-machine

`reconcile-harness reform erin alice,bob carol,dave` on all three boxes → identical reformed genesis
`338d8cc8…`: erin re-forms with `{carol, dave, erin}`, removers `{alice, bob}` excluded from
membership, `shares_lineage_with_original: true` (legible descent, no false ancestry),
`removers_retain_lineage_standing: true` (history not erased — standing ≠ membership). The honest
anti-capture guarantee: a minority gets a clean, provable exit. Detail in `PART_A_RECONCILE_FINDINGS.md`.

### Capstones — both mechanisms run over the LIVE iroh transport (honesty boundary closed)

No new code — Part A/history binaries + the iroh-blobs spike, composed:
- **Reconcile over iroh:** node-1 served its op-log via iroh-blobs; node-2 fetched it over **real
  iroh** (54 ms, byte-identical sha `7a945964…`) and reconciled → same contradiction verdict
  (`RemovedThenIncluded(erin)`, hard-stop, order-independent).
- **History over iroh:** node-1 served a history branch via iroh-blobs; node-2 fetched it over real
  iroh (963 B) and `backfill_import` ABSORBED it (verified, separate navigable branch).
These discharge the "partition/reconnect was file-exchange, not real transport" caveat for the 2-way
transfer — the op-log/branch crosses a genuine P2P transfer and reconciles deterministically.

### B4 — macFUSE: DEFERRED

macFUSE is macOS-only; the sandbox boxes are Linux. Deferred to a local-Mac session (or a Linux-FUSE
substitute purely to de-risk the mount-as-a-folder concept, clearly labelled as not the macOS path).
Not run this session.

### B5 — Croft ↔ Croft.Drive ↔ Alt.Drive naming reconcile (decided)

Decision: **Croft** is the anchor/name center of gravity (per `discovery/NAMING.md`); **Croft.Drive**
is the encrypted-vault substrate formerly called Alt.Drive; `AltID/alt.drive` is the **dead** prior
coordination repo (this repo is now the source of truth, boxes are keyless compute — see §0b). New
docs created this session (`TESTING-DESIGN.md`, `TEST-LOG.md`, `CAPABILITIES.md`, the findings docs)
use Croft/Croft.Drive. The older `iroh/` docs (`README.md`, `DESIGN.md`, `CLAUDE.md`) still say
Alt.Drive and hardcode `AltID/alt.drive`; a full rename of those is a follow-on (left as-is to avoid
churning load-bearing design docs mid-campaign).

- B1 — 5 GB blob: resume + genuine multi-source + ≥1 off-VPC (node-4 NAT) transfer.
- B-gossip — 3-node transitive delivery + drop-a-node resilience.
- B2 — iroh-docs 3-replica sync; Willow-migration input.
- B3 — pairing (NodeAddr + TopicId, IP-excluded invite).
- B4 — macFUSE deferred (Linux boxes) / Linux-FUSE substitute.
- B5 — Croft ↔ Croft.Drive ↔ Alt.Drive naming reconcile; make `iroh/` docs consistent.

---

## Honesty boundaries (carry into final conclusions)

- **Part A transport is modelled as op-log file-exchange, not a live network partition.** The
  *computation* runs on genuinely separate machines (different AZs/instance types) and is provably
  deterministic; the *transport-level* partition/reconnect behaviour (loss, reorder, real iroh
  hole-punch) is **not** exercised in Part A — that is Part B's job and the `lineage-iroh` Phase 3
  caveat. Re-running the reconcile exchange over real iroh would close this gap.
- MLS epoch key-schedule/forward-secrecy timing is still modelled (see `PHASE_1_FINDINGS.md`).
- Survivor rule used is `MemberCountThenGenesis` (a deterministic total order); the thesis permits
  the rule to vary — only determinism is required.
- node-3 same-VPC means the only real NAT-traversal evidence will come from node-4 (the Mac).

---

## Running roadmap implications (to finalize at campaign close)

- Part A demonstrated ⇒ the "superpeer = capability not right" framing is **earned**, not asserted —
  fold into `crystallized/proof-ledger.md` and the governance/survivability notes.
- The genesis-immutability finding (no op can change a threshold/role) is a feature (I1), and it
  means "change a dial at runtime" is out of scope by construction — note in the governance dials design.
- Pending: whether flat-LWW (iroh-docs, B2) is too weak vs the lineage model → Willow-migration input.
- Pending: identity/key-recovery remains open and B3 (pairing) touches it.

---

# Relay & Placement Lab — session 2026-06-16

Driver: Claude Code on the Mac over SSH. Spec: `RELAY-PLACEMENT-LAB-SPEC.md`. Plain-language running
conclusions: `RELAY-LAB-CONCLUSIONS.md`. Per-run data: `relay-lab-runs/`.

## Step 0 — topology & pins ✅

- Nodes live-inventoried: node-1 4 vCPU/15 GiB (`/mnt/data` EBS, 1c), node-2 2 vCPU/7.7 GiB (EBS, 1b),
  node-3 2 vCPU/3.8 GiB (128 G root, 1a), node-4 = Mac (NAT'd). **Not identically sized** — prior docs
  had only the qualitative asymmetry. cgroup v2, `tc` present, `ipvsadm` absent, `cargo 1.96`.
- **Decision (user): MULTIPLEX** with relays cgroup-pinned to identical 2 vCPU/4 GiB slices (neutralizes
  host asymmetry + free E5 accounting); ctrl on node-3; generators co-located + Mac off-VPC.
- **iroh pinned `=1.0.0`** (+ `iroh-relay =1.0.0`) — the version the spec was verified against. Full
  E0 API surface re-verified against source: `relay-lab-runs/IROH-1.0.0-API-VERIFIED.md`.
- **SG: only UDP 2112 open** (TCP 3343 / UDP 3478 probed BLOCKED between boxes). User opening
  all-from-self intra-SG; cross-host relay runs gated on it. No AWS creds on the Mac.

## relay-loadtest crate (spec §5 items 1) ✅

New standalone-workspace crate `crates/relay-loadtest`: `relay` (one self-signed `iroh-relay`,
AllowAll), `responder` (homed on relay, echoes bi-streams, advertises `EndpointAddr` JSON),
`generate` (N independent endpoints = N relay client connections; classifies live path relay/direct
via `conn.paths()`; forced relay-only via `clear_ip_transports`). Builds clean Mac + node-1 (release
4m52s cold / 26s incremental). Loopback smoke test: passthrough → 5/5 relay, matchmaking → 5/5 direct.

- **Finding (corrects a prior-spike assumption):** a relay-only *dial address* does NOT force relay
  passthrough in iroh 1.0 — iroh upgrades to direct via relay-coordinated hole-punch. Forcing
  relay-only needs `.clear_ip_transports()` on the endpoint (and not calling `bind_addr`, which
  re-pushes an IP transport). Confirmed by iroh's own tests.

## E0 — single-relay baseline ✅ (hole-punch-fails matchmaking case ⏳ on NAT'd Mac)

**SG update:** all-from-self intra-SG verified live (Python TCP-accept/UDP-sink probe; the earlier
`nc` probe was unreliable — missed even the known-open 2112). Cross-host relay runs proceeded.

**Cross-host (relay=node-1 cgroup 2vCPU/4G, responder=node-2, generator=node-3; real cross-VPC):**
- Matchmaking same-VPC: 20/20 → **direct**, relay carried 0 (direct RTT ~21.5 ms cross-AZ). The
  cheap side of the cost gap — hole-punch success ⇒ relay steady-state ≈ 0 per pair.
- Active passthrough: 50×4 MiB → relay forwarded ~400 MiB in 5.5 s → **~93 MiB/cpu-s ⇒ ~186 MiB/s
  CPU-bound on 2 vCPU**; +8.3 MiB relay RSS under 50 active conns. Passthrough bind = CPU, not NIC.
- Relay path RTT ~9.5 ms p50. Data: `relay-lab-runs/E0-crosshost-2026-06-16/`.

**Still open:** matchmaking where hole-punch *fails* (CGNAT/mobile) — needs the NAT'd Mac + public
ingress on 3343/3478; a reconnect-storm handshake-CPU driver; a sustained-transfer mode.

## E3 — namespace-sharded fan-out sync ✅ (THE CORE THESIS)

Real Automerge anti-entropy over iroh bi-streams. One relay shard (node-1); members (node-3) forced
relay-only + homed on the shard converge a shared CRDT by meeting a rendezvous peer (node-2) — no
member-to-member point-to-point. Relay `send_packets_dropped` scraped from its OpenMetrics endpoint.
**Population 12 → 12/12 converged, dropped=0. Population 30 → 30/30 converged, dropped=0.** Within a
correctly-sharded namespace every member converges and the co-location-miss counter stays exactly 0.
With E2's negative (wrong placement → no connection, relays don't mesh), the thesis is proven end to
end: DNS-placement-by-namespace co-locates members on one shard; within it everyone converges,
dropped≈0. Anti-entropy retry loop used (hub loops accept_bi; member re-syncs until it holds the full
namespace). Refinements: edit-rate/churn axes, full gossip mesh. Data: `relay-lab-runs/E3-namespace-sync-2026-06-16/`.

## E2 — DNS-driven placement (fast loop) ✅ (DNS/pkarr integration ⏳)

Two relays (A=node-1, B=node-2), responder homed on relay-B (node-3), generator (node-1) knowing
both relays. `MemoryLookup` seeded `peer-id → relay` = the controller's published record; connect by
**bare id** so iroh resolves the relay from the record. **Correct assignment (relay-B): connected
via relay-B, `is_relay_path=true`. Wrong assignment (relay-A, peer is on relay-B): timed out — no
connection.** ⇒ (1) placement is controllable + authoritative for reachability (not latency); (2)
**co-location is mandatory** — wrong placement → no connection because relays don't mesh (E3's thesis
in miniature). Pending: DNS-origin + pkarr integration form. Data: `relay-lab-runs/E2-placement-2026-06-16/`.

## E1 — vertical scaling ✅

Relay on node-1 with 4-core headroom; sustained passthrough from two generators (node-2 + node-3);
relay avg cores-used = Δcgroup `cpu.stat` / Δwall over 10 s. **5 threads, 2.34 cores used** (4-core
headroom) → iroh-relay 1.0 is multi-threaded and spreads forwarding across cores; lock-free `papaya`
`Clients` registry doesn't serialize. ⇒ one fat process scales vertically; multiple-per-box is for
E5 accounting/isolation, not CPU. Not saturated to 4.0 (2-vCPU generators were the bind, so ceiling
≥2.34). Data: `relay-lab-runs/E1-vertical-2026-06-16/`.

### E0 idle-relayed memory wall (single-box, node-1)

Single-box on node-1: relay in a 4 GiB / 2-vCPU `systemd` transient cgroup unit; generator forced
relay-only; relay RSS = cgroup `memory.current`; N-sweep, all established + relay-classified, 0 fail:

| N | relay RSS | per-conn incl. base |
|---|---|---|
| 1 (baseline) | 2.04 MB | — |
| 100 | 5.77 MB | 37.3 KiB |
| 200 | 9.01 MB | 34.9 KiB |
| 400 | 15.13 MB | 32.7 KiB |

**Conclusion:** ~31 KiB marginal RSS per idle relayed client connection (+~2.5 MB fixed), linear in
N — below the prior 50–150 KiB/conn hypothesis. ⇒ a 4 GiB relay slice ≈ 130k idle relayed conns on
the memory ceiling alone. Co-located run, so this is the **memory** ceiling only; NIC/handshake-CPU
walls and the matchmaking-vs-passthrough cost gap need the SG widening + the NAT'd Mac. Data:
`relay-lab-runs/E0-memwall-2026-06-16/manifest.json`.

## E5 — cgroup per-tenant accounting + isolation ✅ (node-1, 2026-06-16)

Two relay "tenants", each a transient `systemd` service with its own cgroup (CPU+memory accounting).
**Accounting (asymmetric load):** tenantA 40×32 MiB (1.28 GiB) vs tenantB 4×1 MiB (4 MiB) →
per-slice `cpu.stat` delta **206.6 vs 0.83 CPU-s (~249:1)** + per-slice RSS (8.6 vs 5.4 MB). The
heavy tenant's bill tracks the byte asymmetry; a co-op meters tenants from the cgroup, no app metering.
**Isolation:** cap tenantA `CPUQuota=50%`, drive BOTH 30×32 MiB → tenantA held to **0.49 cores**,
tenantB ran **0.51**, both established 30/30 (cap throttles CPU without stranding). ⇒ splintered-
per-tenant is for ACCOUNTING + ISOLATION, orthogonal to E1's one-fat-process-for-CPU. Co-located
generators understated absolute throughput (the measured quantity is accounting/isolation fidelity,
which co-location doesn't distort). `relay-lab-runs/E5-cgroup-accounting-2026-06-16/manifest.json`.

## E6 — tc netem traffic shaping ✅ (node-1↔node-2, 2026-06-16)

Relay+responder on node-1, generator (relay-only passthrough) on node-2; `netem` applied to the
node-1→node-2 egress band only (prio qdisc + u32 dst filter), so the Mac's SSH path stays unshaped +
a 240 s watchdog clears tc regardless.

| condition | established | establish_ms | relay RTT p50 / max |
|---|---|---|---|
| baseline | 12/12 | 5 585 | 25.3 / 101 ms |
| +100 ms delay | 12/12 | 9 608 | 249.9 / 485 ms |
| 10 % loss | 12/12 | 6 358 | 35.8 / 141 ms |
| 30 % loss | 12/12 | 28 431 | 344.9 / 946 ms |

**Conclusion:** under every shaping condition all 12 conns still established (0 failed) — eventual
delivery holds — while client-measurable RTT/establish time rose **visibly** (delay: RTT p50 25→250
ms; 30 % loss: establish 5.6→28.4 s). QUIC degrades gracefully (retransmit), never a silent drop ⇒
**TI-4 tiers hold or degrade visibly, never silently**: interactive can surface the rising RTT as a
fail-signal; eventual completed even at 30 % loss. Egress-only/asymmetric; bandwidth-cap + steady-
state goodput are follow-ons. `relay-lab-runs/E6-tc-shaping-2026-06-16/manifest.json`.

## E7 — placement churn / split-brain window ✅ (node-1+node-2, 2026-06-16)

relay-A (node-1) + relay-B (node-2); one mobile peer = a responder with a **pinned secret** (stable
endpoint id across restarts); connector = `e2-connect` resolving the peer by bare id through the
controller-assigned relay. **pre-churn** assign=A → `connected, relay_used=A`. **churn:** re-home the
peer A→B (kill on node-1, restart on node-2, **same id** confirmed). **window:** stale assign=A →
`connected=false, timed out 12s` — the peer moved, the stale assignment points at the now-empty relay,
and there is no relay-to-relay mesh = the partition window. **post-churn** assign=B → `connected,
relay_used=B`. ⇒ reassignment **converges** (peer reachable on its new relay, no endpoint stranded
once placement updates); the window is real and bounded; eventual-sync tolerates it, live presence
doesn't; a straddling meer would close it (E8 fork). Confirms E2's co-location-mandatory finding under
motion. `relay-lab-runs/E7-placement-churn-2026-06-16/manifest.json`.

## T2g — multi-device self-sync over a per-lineage gossip group (2026-06-16)

**MD-G1 (per-lineage topic admits the user's devices) + the NAT path — PROVEN.** The TopicId is
`sha256("croft-lineage:alice-lineage-root-v1")` = `6eccb47b…` (the lineage genesis derives the
topic). node-2 (AWS, same-VPC) started as the bootstrap origin; **node-4 (this Mac, off-VPC behind
a real NAT)** joined the same topic bootstrapping only from node-2's `EndpointAddr` (relay URL
`use1-1.relay.n0…` + key — the Delta-Chat NodeAddr+TopicId pattern, no direct IP needed).

Result — **bidirectional delivery across the NAT via relay:**
- node-4 (Mac): `NeighborUp 71d35505f7`; `RECV 'hello-from-node-2-{13..17}'`;
  `SUMMARY senders_received={"node-2"}`.
- node-2 (box): `NeighborUp 8f17060044`; `RECV 'hello-from-node-4-mac-{0..4}'`;
  `SUMMARY senders_received={"node-4-mac"}`.

This is the path the same-VPC reconcile capstone never exercised: a user's NAT'd device
participating in its own per-lineage gossip group over the real relay. It is the transport
foundation under E2.12 (self-sync) and the multi-device tier proven green-real in
`Proofs/lineage-groups` this session.

**MD-G2 (carry a signed branch over the topic + verify + absorb; reject tampered) — PROVEN
bidirectionally (2026-06-16).** New carrier `altdrive-spike-lineage-sync` (built on node-4 Mac and
node-2) ships a **chain-hashed lineage branch** on the per-lineage topic; the receiver verifies it
(shared lineage genesis + contiguous seqs + intact sha-256 hash chain) and absorbs it as a
*distinct* branch (no interleave), and rejects a deliberately tampered branch. Result across the
NAT/relay path:
- node-4 (Mac): `ABSORB author=node-2 msgs=3 (verified)` · `REJECT node-2-TAMPER — broken hash chain
  at seq 1` · `SUMMARY absorbed_branches={"node-2":3} rejected=1`.
- node-2 (box): `ABSORB author=node-4-mac msgs=3 (verified)` · `REJECT node-4-mac-TAMPER` ·
  `SUMMARY absorbed_branches={"node-4-mac":3} rejected=1`.

This is the transport form of the green-real `lineage-history::backfill_import` (Proofs E2.12) —
the *structural* half (shared-genesis + contiguity + integrity). Honesty boundary: the Ed25519
**signature/standing** check stays green-real in `Proofs/lineage-groups` (E2.7/E2.12) and is not
re-implemented in the spike; the hash chain proves in-transit integrity + ordering, not authorization.

**Full T2g now closed over transport:** MD-G3 (drop-a-device, below), MD-G4 (multi-device fold) and
MD-G5 (revoked device can't rejoin) are all demonstrated live (MD-G4/G5 below). They were already
green-real in `Proofs` (E2.9/C4 fold, E2.11 revocation); only the live-transport demonstration
remained, and is now done including the NAT Mac.

## T11 — 3-way local-first history exchange over live iroh (2026-06-16)

Ran the same `altdrive-spike-lineage-sync` carrier as a **3-node group** (`household-group-v1`):
node-2 (origin) + node-1 + node-4 (NAT'd Mac), all on the per-lineage/group topic. **Every node
absorbed the other two's branches** as distinct, verified, navigable branches (no interleave) and
**rejected every tampered branch**, over real iroh gossip including the NAT path:
- node-4 (Mac): `absorbed_branches={node-1:3, node-2:3} rejected=2`
- node-1: `absorbed_branches={node-2:3, node-4-mac:3} rejected=2`
- node-2: `absorbed_branches={node-1:3, node-4-mac:3} rejected=2`

This promotes the file-relayed local-first-history result (I7/I8/I9, `LOCAL_FIRST_HISTORY_FINDINGS`)
to **live transport, 3-way**, and demonstrates the thesis's "same mechanism for multi-device AND
group" on the wire. Honesty boundary unchanged: structural half (shared-genesis +
contiguity + hash-chain integrity) over the wire; Ed25519/standing stays green-real in Proofs.

## MD-G3 — drop-a-device mid-run (2026-06-16)

Same carrier, `dropnode-group-v1`: node-1 + node-4(Mac) bootstrapped **only via node-2**; node-2
(the bootstrap relayer) was `fuser -k`'d ~14 s into the run. Both survivors converged with state
intact — node-1 `absorbed={node-2, node-4-mac}`, Mac `absorbed={node-1, node-2}` — each holds the
*other survivor's* branch despite never exchanging addresses directly (transitive mesh) and despite
the relayer's death. Honesty boundary: absorptions are monotonic and the log has no per-line
timestamps, so this proves the mesh forms transitively and **survives the relayer's death with
state intact**; B-gossip separately proved *new* delivery continues post-kill. Together, drop-a-node
resilience holds for the multi-device/group carrier including the NAT'd device.

## MD-G4 — multi-device fold over transport (2026-06-16)

Extended `altdrive-spike-lineage-sync`: the gossip topic is now derived from the **group** id (not
the lineage), so all member lineages share one topic; each `Branch` carries a distinct `device_did`
plus its actor `lineage_id`, and receivers **fold absorbed branches by `lineage_id`** — the
transport form of green-real `fold_by_lineage` (Proofs E2.9 / C4). Run on group `g1`: node-1
`alice.laptop` + node-4 (NAT Mac) `alice.phone` (one actor, two devices) + node-2 `bob.phone` (a
second actor). Every node folded identically:
- node-1 / node-2 / node-4(Mac), all three:
  `SUMMARY folded_actors=2 actors={"alice":["alice.laptop","alice.phone"],"bob":["bob.phone"]} revoked={} rejected=2`

**Exit met:** `folded_actors == 2`; alice's two device-branches collapse to one actor while bob is a
second — over real iroh incl. the NAT Mac. The two rejects per node are the deliberately tampered
branches (broken hash chain), proving integrity is still enforced and a rejected branch never enters
the fold. Honesty boundary unchanged: structural fold over the wire; Ed25519/standing stays
green-real in Proofs.

## MD-G5 — revoked device can't rejoin over transport (2026-06-16)

Same carrier, group `g2`, with a `--revoke <device>` flag that broadcasts a MAC'd `Revoke` marker
(keyed to the group genesis — the structural stand-in for the green-real Ed25519 authority check,
E2.11). node-1 `alice.laptop` is issuer/revoker of `carol.tablet`; node-2 = `carol.tablet` (target,
keeps broadcasting); node-4 (Mac) `bob.phone` is a witness survivor. The two survivors show the two
honest halves:
- **Witness (Mac, bob.phone):** `ABSORB device=carol.tablet …` (pre-revoke) → `REVOKE target=carol.tablet by=alice.laptop (pre-revoke branches retained)` →
  `SUMMARY folded_actors=3 actors={…,"carol":["carol.tablet"]} revoked={"carol.tablet"}` — carol's
  pre-revoke branch is **retained** in the fold and the device is marked revoked. **History is not
  clawed back** (E2.11 honesty).
- **Revoker (node-1, alice.laptop):** first hears carol *after* its round-6 self-applied revoke →
  `REJECT branch device=carol.tablet — (revoked)` → `SUMMARY folded_actors=2 actors={"alice",…,"bob"} revoked={"carol.tablet"}` —
  the revoked device is **kept out of the accepted set**.
- **Target (node-2, carol.tablet):** `received REVOKE of SELF by=alice.laptop — cannot re-enter accepted set`.

**Exit met:** post-revocation, carol's branch shows `REJECT … (revoked)` on a survivor; pre-revoke
branches stay. Honesty boundaries: (1) the revoke is integrity-checked by a sha-256 MAC, not a real
Ed25519 authority signature (green-real in Proofs E2.11); (2) the split between "retain pre-revoke"
(witness) and "refuse first-contact post-revoke" (revoker) is the honest consequence of iroh-gossip
**de-duping identical payloads** — a peer that already holds carol's (byte-identical) branch isn't
re-delivered the repeats, so it never re-evaluates them, whereas a peer whose first carol delivery
lands after the revoke rejects it. Both behaviours are correct; together they cover the full claim.
A single run on one node showing both transitions would need per-round-varying branch bytes.

## FAITHFUL — real signed message + authority verified over the wire (2026-06-16)

**Closes the honesty boundary every transport spike carried.** Until now the spikes shipped a
sha-256 hash chain (in-transit integrity + ordering) but NOT *who wrote it* — a valid chain claiming
`device=alice` is forgeable, and authorship/standing/authority lived green-real only in `Proofs/`,
never on the wire. New standalone spike `altdrive-spike-faithful-sync` path-deps the **real** Proofs
crates (`lineage-core`, `lineage-history`) — pinned stable `ed25519-dalek =2.2.0` / `sha2 =0.10.9` —
alongside iroh `1.0.0-rc.1` (pre-release crypto, two `curve25519` majors). They compose: cargo allows
multiple major versions in one tree and no crypto type crosses the iroh↔lineage boundary (built clean
on the Mac, node-1, node-2). It carries the real `lineage_history::Message` (Ed25519 `sig` over
`Message::signing_bytes`) over live iroh-gossip and runs the **real** `HistoryStore::backfill_import`
(real signature verify + real `Lineage::standing`) on receipt.

Three canonical vectors broadcast on group `ff1`; **both joiners — node-2 and the NAT'd Mac —
independently reached the identical verdict** over real iroh incl. the NAT path:
- **HONEST** (alice, a member, real signature) → **ACCEPT**.
- **FORGED** (alice's branch, payload tampered after signing) → **REJECT** `message 0 from did:2bd806c97f
  failed signature verification` — the real Ed25519 verify (`BadSignature`).
- **NONMEMBER** (mallory, **valid real signature**, never a member) → **REJECT** `message author
  did:c0a497761b never held standing on this branch` — the real `standing` check (`UnauthorizedAuthor`).

The NONMEMBER case is the load-bearing one: the signature is genuine and *passes* verification, yet
the message is rejected for lack of standing — **exactly the attack a hash chain cannot catch**
(the hash-chain spike would absorb a well-formed branch from a non-member). The same bytes that are
signed-and-authority-checked in the Proofs model are now checked on the wire by the same code.

Honesty boundaries that remain: (1) the verifying-key registry + lineage membership are modelled in
the spike as the agreed group state (MLS distributes these green-real in Proofs E2.x); the spike does
not run the MLS key-distribution over the wire. (2) node-1 *as origin* showed empty verdicts — a
gossip artifact (it exited before peers' broadcasts meshed back; own broadcasts aren't echoed) — so
the verdict is carried by the two joiners, which independently agree. (3) **Threshold revocation
authority** (who-may-revoke, the k-of-n dial) is the next layer up and is NOT in this spike — see
`alpha/thinking/revocation-authority.md`; this spike proves the per-message authorship+standing
gate, which is the foundation the threshold ops compose on.

## AR-4 — transport metadata-leak bound (characterization, 2026-06-16)

What a relay / on-path observer can infer, derived from the transport behaviour observed across
the T2g/MD-G2/T11/MD-G3 runs (not a fresh packet capture — that is the follow-on):

- **Content is opaque.** Gossip rides encrypted QUIC; the relay forwards endpoint-to-endpoint and
  never sees branch content (consistent with E3.4 blind-broker). The carrier's branch payloads are
  never visible to the relay.
- **The relay is topic-agnostic.** iroh relays route by `EndpointId`, not by gossip `TopicId`, so
  the relay does **not** learn the per-lineage topic or the lineage id. Membership-by-topic is an
  *app-layer* exposure (a co-subscriber sees co-subscribers — the V8 boundary), not a relay one.
- **What the relay DOES see (classic traffic-analysis metadata):** the `EndpointId`s that connect to
  it, the source IP as it appears to the relay (e.g. the NAT'd Mac's egress IP), connection timing,
  and packet volume/size. For a relayed delivery it can infer that two endpoints communicate, and
  when, and roughly how much. The `EndpointAddr` we published also embeds the relay URL + the peer's
  IPs (e.g. `{relay, 34.207.146.151:2112, 172.31.19.13:2112}`).
- **Topic guessability is a real design input.** TopicId = `sha256(derive(lineage_id))`. An adversary
  who *knows or guesses* a lineage id can compute the topic and attempt to join/observe it. ⇒ lineage
  ids used as topic seeds must be **high-entropy / salted**, not human-guessable handles. (New design
  requirement — record it.)
- **Open / follow-on:** an actual relay-side timing+volume capture to *quantify* the leak; a co-op-run
  relay's logging policy (must redact to endpoint/timing, never content — cf. the PII-in-logs flag on
  the public-path ingester); and mitigations for who-talks-to-whom (cover traffic, batching) if the
  threat model demands it. This is the AR-4 *bound*; tightening it is unspecified.

## E10 — RoQ media under netem: the C1 datagram-CC unknown ✅ (node-1↔node-2, 2026-06-17)

Synthetic CBR over the iroh 1.0 QUIC **datagram** path (`conn.send_datagram`/`read_datagram`), the
media analog of E6: 64 kbps / 20 ms frames (160 B datagrams, 50 fps), each stamped with a sequence +
send-timestamp so the receiver tallies loss / reorder / RFC3550 jitter / goodput / gap-bursts, while
the sender samples the CC signals (selected-path RTT, `datagram_send_buffer_space`, `send_datagram`
errors). No codec/cpal — the headless synthetic-source rule; C1 is a *transport*-CC question.
Harness `crates/relay-loadtest/src/roq.rs` (`roq-send`/`roq-recv`); relay on node-1, receiver on
node-2, sender on node-1; media flows node-1→node-2 over the DIRECT (same-VPC hole-punched) path,
through the E6 netem rig (u32 dst-filtered so the Mac's SSH path stays unshaped). `max_datagram_size`
negotiated **1162 B** (ample for Opus 160 B frames). Orchestrators `e10-orch.sh`, `e10-ratecap.sh`.

| condition | sent | received | loss | send_err | RTT min/mean/max ms | jitter | goodput |
|---|---|---|---|---|---|---|---|
| baseline | 1000 | 1000 | 0.0 % | 0 | 1.5 / 1.7 / 2.0 | 0.2 ms | 64.0 kbps |
| +100 ms delay | 1000 | 1000 | 0.0 % | 0 | 101.6 / 102.2 / 128.1 | 0.2 ms | 64.0 kbps |
| 5 % loss | 1000 | 954 | 4.6 % | 0 | 1.7 / 1.8 / 2.3 | 0.1 ms | 61.1 kbps |
| 30 % loss | 1000 | 689 | 30.9 % | 0 | 1.7 / 2.0 / 2.7 | 0.1 ms | 44.1 kbps |
| delay+5 % loss | 1000 | 951 | 4.9 % | 0 | 101.7 / 102.5 / 129.2 | 0.2 ms | 60.9 kbps |
| **rate 40 kbit** (default buf) | 1000 | — | — | 0 | **537 / 4224 / 8829** | — | — |
| **rate 40 kbit** (8 KiB buf) | 1000 | — | — | 0 | 538 / 4205 / 8821 | — | — |
| **rate 40 kbit** (recv side) | 750 | **208** | 0.0 % | — | — | 69.9 ms | 11.1 kbps |

**Conclusion — C1 RESOLVED (characterized).** Two regimes:

1. **Loss + delay are transparent; the controllers do NOT fight.** netem loss reaches the application
   verbatim as sequence gaps (5 %→4.6 %, 30 %→30.9 %); path RTT stays flat (~2 ms) and tracks added
   delay *exactly* (100→102 ms); jitter stays sub-ms; zero `send_datagram` errors; no reliable-stream
   HOL fallback. Audio holds to 30 % loss with **visible** (31 % packet-loss-concealment) — never
   silent — degradation, the same graceful-degradation property E6 showed for messaging. A media
   estimator gets accurate loss + RTT + jitter from the datagram path.
2. **An over-cap source bufferbloats; iroh will NOT rate-adapt for you.** Capped at 40 kbit (below the
   ~84 kbit wire rate of the 64 kbps source) iroh **queues and paces datagrams in order** rather than
   dropping to fit. RTT balloons 537→8829 ms; the receiver gets a **contiguous, ever-delayed prefix**
   (208/1000 frames, *zero gaps*, ~11 kbps = link rate) — the rest never arrives in time.
   `send_datagram` returns `Ok` throughout: when its own buffer fills it **drops oldest silently**, so
   even an 8 KiB `datagram_send_buffer_size` produces no error and the same RTT inflation (the
   bottleneck queue is the link/qdisc, faithfully reported by iroh's path RTT).

**Design consequence (feeds `realtime-media-over-iroh.md` C1).** The media engine's bitrate estimator
must be **authoritative** and must back off on the **path-RTT trend** (+ per-stream sequence loss +
arrival jitter). It cannot rely on `send_datagram` back-pressure (never errors) nor on receiver-side
loss alone (the delayed prefix shows 0 % while most frames are undelivered). All three required signals
are exposed and accurate — C1's proposed solution point 2 is the right one, now evidenced. The one
confirmed failure mode is "QUIC pacing hides loss → overshoot," specifically under a bandwidth cap;
mesh-vs-meer and a raw-UDP side-baseline remain follow-ons (E0-NAT ingress still gated).
`relay-lab-runs/E10-roq-netem-2026-06-17/manifest.json`.

## E12 — blind media-meer: SFrame-over-MLS ✅ green-real (local, real openmls, 2026-06-17)

The media analog of the FAITHFUL path + MD-G5: media frames E2EE'd with **SFrame** keyed off a **real
openmls group**, forwarded by a meer that never holds a key. Spike `crates/media-sframe-spike`
path-deps the real `lineage-mls` (openmls 0.8.1) so the per-sender key is HKDF over the genuine MLS
**exporter secret** (`epoch_proof`) bound to `(epoch, leaf)`; frames are ChaCha20-Poly1305 with a clear
`(epoch, leaf, counter)` header as AAD; a sliding per-sender counter window gives loss-tolerance + replay
rejection. Real 3-member group (alice founder + bob + carol via add/welcome; removal = epoch advance).

| case | asserts | result |
|---|---|---|
| TC-KEY1 | distinct per-sender keys; non-member can't derive | alice≠carol keys; outsider `epoch_proof()` errs (no standing) ✅ |
| TC-KEY2 | loss + reorder decrypt out of order; replays rejected | 100 sent, 10 dropped, reordered → **90/90 decrypt OOO**, **90/90 replays rejected** ✅ |
| TC-KEY3 | revoked sender's later frames undecryptable; pre-revoke retained | remove carol → epoch 1→2, secret rotated; carol stuck@1 (no S2), her later frame rejected (stale+non-member = media MD-G5), can't forge epoch-2; **bob's pre-revoke frame still decrypts**; alice still works@2 ✅ |
| TC-KEY4 | blind SFU routes from headers, recovers zero plaintext | SFU routed **92/92** frames from clear headers, recovered **0** plaintext bytes ✅ |

**Conclusion (green-real).** SFrame keyed off a real MLS group's exporter secret realizes the DAVE-shape
**blind-SFU media E2EE**: per-sender keys, loss-tolerant out-of-order decrypt with replay rejection,
epoch-rotation revocation (media **MD-G5** + history retention), and a content-blind forwarder all hold
against openmls 0.8.1. "Media keying = message keying + loss-tolerance" is real, not modeled. The C3
keying unknown is closed. Open: synthetic frames (not real Opus/RTP packetization); modeled SFrame
header vs RFC 9605 wire format; run locally — a transport-carried version is this keying over the E10
datagram rig (small follow-on). `relay-lab-runs/E12-sframe-mls-2026-06-17/{manifest,verdict}.json`.

## meer P0+P1 — always-on blind Tier-0 message mirror ✅ green-real (node-1↔node-2, 2026-06-17)

"E9 Tier-0, made real" (`discovery/thinking/meer-superpeer-design.md` P0+P1). An always-on iroh
endpoint (`relay-loadtest meer`, `src/meer.rs`) homed on the relay; members (`meer-member`) publish
**ChaCha20-Poly1305-encrypted** blobs the meer stores keyed by `sha256(ciphertext)` — **holding no
payload key**. relay + meer A on node-1; members + replacement meer B on node-2.

| step | asserts | result |
|---|---|---|
| P1 publish | member stores encrypted blobs | `{"published":5}` ✅ |
| P1 sync | an offline/behind member syncs through the blind meer + decrypts locally → converges | `fetched 5, converged 5, all_converged true` ✅ |
| blindness | the meer holds zero payload keys; sees only §3b metadata | `meer_payload_keys_held=0`, blobs_stored=5, bytes_mirrored=140, namespaces=[household-v1] ✅ |
| anti-entrenchment | export encrypted store → import into replacement meer B → re-home + converge | exported 5, imported 5, re-homed `all_converged true` ✅ |
| admission (P0) | a meer with an allowlist denies a non-listed peer | member got `closed by peer: not admitted (code 1)` ✅ |

**Conclusion (green-real).** The thesis property is demonstrated on the live fabric: a blind, always-on
superpeer that **provably holds zero payload keys** yet serves offline state to a rejoining member, and
whose encrypted state is **portable** so a group can re-host on a replacement meer and converge — losing
a meer costs availability, never data (the materially-reversible anti-entrenchment guard). Converts
E3.4's modeled blind broker into a running binary. This is a spike harness; the production-TDD `meer`
crate (Workstream B) productionizes exactly this protocol. P2/P3 (bridge, Tier-1, no-mirror), P4/P5
(media SFU/MoQ — see E12/E11), P6 (Tier-2) remain. `relay-lab-runs/E9-meer-tier0-2026-06-17/manifest.json`.

## Conformance vectors + runner ✅ green (local, 2026-06-17)

Workstream D: a black-box conformance suite a second Croft implementation must pass, **derived from the
real `lineage-core`/`lineage-history` code** (never hand-typed). New `Proofs/lineage-groups/crates/
conformance/` — `emit-vectors` runs the real API to write language-neutral JSON (`conformance/vectors/`),
`run-vectors` re-feeds each vector through the public API and diffs. Independently re-verified here:

```
[PASS] cat1 derivations        2/0   [PASS] cat2 signing            2/0
[PASS] cat3+4 fold/thresholds  3/0   [PASS] cat5 revocation         2/0
[PASS] cat6 reconcile C1..C10 10/0   [PASS] manifest integrity     15/0
TOTAL: 34 pass, 0 fail   (touched-crate tests: conformance 9/0, lineage-core 14/0, all green)
```

Must-reject teeth confirmed: corrupting a good signature while leaving `expect:accept` flips the runner
to FAIL; the one-bit-flipped signature and the one-lineage-3-device quorum pass *because* the real API
rejects them. `design` rows not faked — revoke-**authority** threshold is a `PLACEHOLDER` blocked on
Workstream C; cats 7/8/9 (AR / visibility / freshness) recorded `not_yet_emitted` (8/9 are green-model
in the TS `lineage-group-model`, a different stack).

**⚠️ Spec-vs-code discrepancy surfaced (code is truth).** CROFT-PROTOCOL.md §2 specifies **domain-tagged**
genesis/topic pre-images (`"croft-lineage-genesis:" ‖ id`, `TopicId = sha256("croft-group-topic:" ‖ id)`),
but the Rust workspace computes plain `sha256(canonical_bytes)` for `GenesisId` and tags the gossip topic
`"lineage-topic-v1"` (not `"croft-group-topic:"`). The tagged §2 derivations are a *different stack* (the
iroh `altdrive-spike-lineage-sync` spike) from `lineage-core`. cat-1 vectors were derived from what
`lineage-core` actually computes; the divergence is recorded in the vector header, not papered over.
**Reconciliation item:** decide whether `lineage-core` and the iroh spike must share the tagged pre-images.

## E11 — MoQ broadcast: lazy fan-out + blind relay + metadata admission ✅ characterized (local, 2026-06-17)

The broadcast-media relay LOGIC (`crates/moq-lazy-spike`), deterministic in-process; the codec/transport
form is meer role P5 (moq-rs/iroh-live over the proven iroh fabric), de-risked by n0's iroh-live.

| claim | result |
|---|---|
| lazy (no egress until watched) | 100 produce-ticks @ 0 subscribers → **0 egress**; first watcher → produces ✅ |
| fan-out cost linear in N | (subs→relay_egress for a 10-frame track) = (0→0, 1→10, 3→30, 10→100) ✅ |
| blind relay | `payload_keys_held=0`; every subscriber decrypts **locally** ✅ |
| metadata admission (abuse lever) | cap 5 refuses 3 of 8; members-only refuses 1 non-member of 3 — **reading zero frame bytes** ✅ |

**Conclusion (characterized).** The Croft-novel MoQ relay properties hold: **lazy** (nothing encoded/sent
until a subscriber asks — the media instance of the interaction-tiers philosophy), **blind** (forwards
opaque frames, holds no key), and an **abuse lever that is scale + peer restriction enforced from
metadata alone** (the answer to the Rave trap — never content inspection). Full form deferred to meer P5:
real moq-rs Tracks + GStreamer/Opus + WebTransport browser reach ("mostly assembly, not build"). The
AR-4-for-media metadata bound (TC-META1) + CBR padding (TC-META2) are separate measurements.
`relay-lab-runs/E11-moq-lazy-2026-06-17/manifest.json`.

## E10c / TC-CC2 — the media estimator, closed loop ✅ (node-1↔node-2, 2026-06-17)

Follow-on to E10: E10 showed the engine MUST drive its own bitrate off the RTT signal. Here we built a
delay-based AIMD estimator into `roq-send --adaptive` (baseline = running-min RTT; RTT > base+50 ms →
bitrate ×0.7 down to an 8 kbps floor; else +8 kbps up to target) and ran the **bandwidth-ramp** (start
unshaped, drop a 40 kbit cap mid-call at t=8 s) — the case where adaptation can actually help.

| sender | base RTT | after the cap drops | receiver |
|---|---|---|---|
| **fixed 64 kbps** | 2 ms | RTT **diverges**: 401→891→1837→3033→4980→**7024 ms** and climbing | unusable backlog |
| **adaptive** | 1.6 ms | backs off 64→14→**8 kbps in ~1 s**; RTT **bounded** ~0.8–1.6 s (mean **689** vs 2446, peak **1609** vs 7024) | **1100 frames, 26 kbps, 0 % loss** — continuous degraded-but-live stream |

**Conclusion.** The C1 estimator is not just *informed* by iroh's path-RTT — it *acts* on it effectively.
On a mid-call bandwidth drop, fixed bitrate bufferbloats unbounded (call dead); delay-based AIMD backs
off in under a second and turns it into a bounded, lossless (degraded) call — peak RTT cut 4.4×, mean
3.6×. **Caveat:** this helps the *degradation* case, not the *join-an-already-saturated-link* case (cap
from start → baseline locks high, standing queue never drains regardless of source rate or send-buffer
size — see `e10both.sh`). Residual ~1 s RTT reflects the pre-backoff overshoot + netem/relay standing
queue; a raw-UDP side-baseline stays the follow-on. `relay-lab-runs/E10-roq-netem-2026-06-17/adaptive-findings.json`.

## TC-CC3 — datagram/stream isolation (media vs bulk on one connection) ✅ (node-1↔node-2, 2026-06-17)

Does a bulk file transfer starve the real-time media datagram flow when they share one iroh
connection? 64 kbps media datagrams ± a 20 MB bulk reliable stream on the SAME connection, under a
1 mbit cap so they contend (`roq-send --bulk-bytes`; `roq-recv` drains the bulk bi-stream in the bg).

| condition | media RTT min/mean/max | media result |
|---|---|---|
| media only @ 1 mbit | 4 / 5 / 50 ms | 1000/1000, 0 % loss, jitter 0.4 ms — flawless |
| media + 20 MB bulk @ 1 mbit | **61 / 4604 / 9485 ms** | unusable backlog — **starved** |

**Conclusion (negative result, confirms a design requirement).** Datagram/stream isolation does **not**
hold on a shared iroh connection under link pressure: the bulk reliable stream fills the constrained
link and the unreliable media datagrams queue behind it (the same bufferbloat mechanism E10 found — the
bulk just creates the over-cap condition). Media RTT goes 4–50 ms → ~9.5 s. This makes **C1 solution
point 3 mandatory, not optional: real-time media and bulk transfers MUST run on separate
flows/connections** so a file transfer can't starve the call. Co-locating them on one connection is a
design error. `relay-lab-runs/E10-roq-netem-2026-06-17/cc3-findings.json`.

## E10 raw-UDP baseline — attributing the bufferbloat (node-1↔node-2, 2026-06-17)

E10's headline caveat was "iroh queues+paces an over-cap source → multi-second bufferbloat." Is that
iroh-specific? Ran a **non-iroh** CBR UDP stream (python: 84 kbit = 50 pps × 210 B, matching the media
wire rate, 15 s) over the SAME 40 kbit netem cap (port 2112 — the SG-open one).

**Result:** `received 750/750, lost 0, recv_span 37.75 s` — every packet arrived, **zero loss**, but
spread over **37.75 s for a 15 s send** (the tail delayed ~22 s).

**Conclusion.** The over-cap bufferbloat is the **bottleneck queue (the netem `rate` qdisc's deep
default buffer), not an iroh defect** — raw UDP bloats identically (no drops, tens-of-seconds queue
delay). iroh's own ~1 MiB send buffer can add to it, but in E10 it stayed mostly untouched (the qdisc
was the bottleneck; `send_buf_space` ~989 KB free). This **reinforces** E10's lesson rather than
weakening it: because a dumb deep-buffered bottleneck will *queue* (not promptly drop) an over-rate
stream for tens of seconds, the application MUST rate-adapt off RTT (E10c) — you cannot rely on the
network to shed excess in time. (Real bottlenecks vary: shallow buffers / AQM like fq_codel drop sooner;
the estimator must handle the bloat case, which E10c shows it does.)
`relay-lab-runs/E10-roq-netem-2026-06-17/rawudp-baseline.json`.
