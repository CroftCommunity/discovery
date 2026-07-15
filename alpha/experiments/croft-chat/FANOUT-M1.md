# A4 / M1 fan-out — gossip fan-out cost & convergence curve (EXP-1)

Serves: Part 2 §11.11 measurement #1 and §11.4–§11.5 (fan-out cost scales on the live set) — earns/bounds: `Measured` (fan-out shape; loopback, single-run magnitude indicative) — register: `fanout-single-run` — landed: RUN-01 (EXP-1).

`RUN-01 EXP-1. Branch claude/experiments-run-01, 2026-07-14. Runnable-now, no new infra.`

## What this earns

§11.11 measurement #1 asks for **per-commit and fan-out** re-key cost. The **per-commit** half is
already measured on real openmls in `mls-replant` (M1: an O(N) floor ↔ O(log N) ceiling per commit).
This experiment earns the **fan-out** half: how gossip cost and convergence latency scale as the live
set grows, measured over **real iroh-gossip** on the loopback testbed (no relay, no Internet).

It is the fan-out complement, not a re-measurement of the MLS commit cost: in this testbed a
membership boundary is a **governance fact** broadcast over gossip, not an openmls commit. So the
quantities here are **gossip-message volume + convergence latency vs live-N**; the cryptographic
per-commit re-key band stays with `mls-replant` M1. That scoping is a boundary, not a stand-in.

## Method

`scripts/fanout-measure.sh N…` brings up **N local `serve` processes** on one host, converging over
real iroh-gossip across `127.0.0.1` with `relay_mode = "disabled"` (the `RelayChoice::LocalDirect`
path). Topology (seeds 11..10+N) is generated per run. `local-1` is the creator — it creates the
group, enrolls every node, and sends one message; `local-2..N` bootstrap from the creator's published
address and each send one message. So the fully-converged timeline length is exactly **N**.

Each process reports a `metrics` line (instrumentation added to `iroh_bus.rs` / `serve` for this
experiment — it observes the wire, it does not change what is broadcast):

- `live_sent` — distinct frames this node broadcast once in steady state (`TAG_LIVE`).
- `resync_sent` — per-frame connect-time re-broadcasts (`TAG_RESYNC`), summed over `NeighborUp` events
  (the sync-on-connect catch-up: the whole retained log re-flooded with fresh ids when a neighbor joins).
- `received` — inbound frames folded.
- `head_ms` — time from loop start to the full N-message timeline being folded (**the conversation
  has converged**; fingerprint matches).
- `converged_ms` — stricter: `head_ms` **and** `pending == 0` (nothing buffered — **fully settled**).

Raw output is in `fanout-data/` (`run-30s.txt`, `run-45s.txt` — two independent runs; the shape is
identical across both). Values below are from the 45 s run.

## Results

| N (live set) | `live_sent`/node | creator `resync_sent` | head-convergence latency (min–max) | fully-settled (`pending==0`) | fingerprints match |
|---|---|---|---|---|---|
| 2  | **5**  | 3   | 0.30–0.78 s | **2 / 2** | ✅ all equal |
| 4  | **9**  | 15  | 0.86–1.05 s | **4 / 4** | ✅ all equal |
| 8  | **17** | 64  | 1.01–1.10 s | 2 / 8 | ✅ all equal |
| 16 | **33** | 479 | 2.97–3.20 s | 0 / 16 | ✅ all equal |

## The shape, stated honestly

**1. Per-node steady-state gossip cost is exactly linear in the live set: `live_sent = 2N + 1`**
(5, 9, 17, 33 at N = 2, 4, 8, 16). Each node ends up broadcasting each distinct fact once — the ~2N
governance/message facts plus genesis. This is the load-bearing PASS signal: **per-node cost scales on
the live set (linear), corroborating §11.4/§11.5.** Aggregate volume across N nodes is N·(2N+1) =
**O(N²)** — inherent to flood gossip, where every node relays every fact once.

**2. Head/state convergence holds at every N.** All nodes reach the **same** fingerprint at N = 2, 4,
8, 16 — order-insensitive convergence (I5) scales across the fan-out. Head-convergence latency is
~flat at ~1 s through N = 8, rising to ~3 s at N = 16 (roughly linear-ish at the top, sub-linear
below). The *conversation* converges at all N tested.

**3. FLAG (reported prominently, per the FALSIFY clause): the connect-time resync path is
super-linear, and full-settle degrades past N ≈ 8.**
- The creator's `resync_sent` grows **super-linearly**: 3 → 15 → 64 → 479 for N = 2 → 4 → 8 → 16
  (≈ O(N²) on the bootstrap hub). It is `(NeighborUp events) × (retained-log size ≈ 2N)`, and in a
  star bootstrap the creator absorbs every joiner's `NeighborUp`.
- `pending == 0` (fully settled — no buffered per-device gaps / governance antecedents) is reached by
  every node at N ≤ 4, only 2/8 at N = 8, and **0/16 at N = 16** within a 45 s window. Nodes sit at
  `pending 11–18` **with matching fingerprints** — this is not head divergence, it is the honest
  incompleteness signal not draining.

**Is this a FALSIFY of "cost scales on the live set"?** No — the *per-node live* cost is linear (finding
1) and heads converge (finding 2). The super-linearity is in the **resync / connect-time
anti-entropy** path (finding 3), which the corpus already flags as open: the register notes
"steady-state anti-entropy … is still future work — the resync covers connect-time catch-up," and
proposed-changes **F3** records that the whole-retained-log re-broadcast is "a coarser push cousin of
RBSR, not the diff-only range reconciliation." This measurement is concrete evidence for **why RBSR
matters**: a diff-only range reconciliation would replace the O(N²) hub re-flood and is the mechanism
that would let full-settle scale past N ≈ 8. Recorded as the finding, not papered over.

## Honesty caveats (register: `fanout-single-run`, proxy-measurement)

- **Magnitude is indicative, not tight.** 1–2 runs per N (two independent runs agree on shape and on
  `live_sent = 2N+1` exactly), no averaging over many trials.
- **Latency resolution is ±250 ms** — `head_ms`/`converged_ms` are sampled on the 250 ms replication
  tick.
- **Star-bootstrap topology.** All joiners bootstrap from the creator, so the resync super-linearity is
  partly a property of the hub; a mesh bootstrap would spread it. The *per-node live* linearity and the
  head-convergence result are topology-robust; the resync magnitude is topology-sensitive.
- **Loopback only** (`hermetic-gossip` still active) — no relay/NAT path (that is X1, needs the boxes).

## Verdict

**PASS** — a fan-out cost/latency curve is captured across N = 2, 4, 8, 16, and its shape is stated
honestly: **per-node gossip cost linear (`2N+1`), aggregate O(N²), head-convergence holding at all N**,
with a prominently-reported **flag** that the connect-time resync path is super-linear on the hub and
full-settle does not complete past N ≈ 8 in the measured window — corroborating the open RBSR /
steady-state-anti-entropy gap rather than contradicting the live-set posture.

## Reproduce

```sh
cargo build -p croft-chat --features iroh-it
BIN=target/debug/croft-chat RUN_SECONDS=45 ./scripts/fanout-measure.sh 2 4 8 16
```
