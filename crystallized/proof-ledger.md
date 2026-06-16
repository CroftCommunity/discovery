# Proof & Experiment Ledger

date: 2026-06-15

purpose: one consolidated, trackable list of every falsifiable claim across the
lineage-groups design — protocol invariants (I), experiments (E), social-layer visibility
tests (V), safety invariants (S) — with current status and where the proof lives.

repos: code lives in the sibling **`Proofs`** repo (durable) and **`experiments`** repo
(code-forward). This ledger (in `discovery`) tracks status and links each row to its proof.

> **Reasoning layer:** this table is *status*. For **why each test was run, what it tells us, what
> the outcome means, and the edge cases it surfaces**, see `test-narrative.md` (one entry per test).
> Every new proof should gain a narrative entry, not just a status row.

- `Proofs/lineage-groups` — Rust validation vs **real openmls 0.8.1** (PR #8). Phases 0–3.

- `Proofs/lineage-group-model` — TypeScript model suite (PR #9). Groups A–H, V. **Real
  SHA-256 ancestry; modeled MLS/transport.**

status legend: `green-real` = proven with real crypto/transport · `green-model` = proven in
simulation (modeled MLS/transport) · `spec` = defined, not built · `blocked` = waiting on a
dependency or decision

> HEADLINE: the Phase 1 crypto-feasibility gate — the make-or-break of the whole thesis —
> is **GO**. openmls 0.8.1 expresses survivor-epoch re-key with post-compromise security
> intact (E1.1–E1.4 pass against the real library). Every downstream phase was conditional
> on this; it held. Phases 0, 2, 2.5, 2.6, 3 also GO. Two real gaps were found and closed
> by the adversarial passes (see below) — the proofs did their job.

---

## Protocol invariants (I1–I10)

source: thinking/thesis-lineage-groups.md §3. Proven in both the Rust workspace (real MLS)
and the TS model (logic), as noted.

| ID | Invariant | Status | Proof |
|---|---|---|---|
| I1 | Genesis immutability | green-real + green-model | lineage-groups E2.2; lineage-group-model A3/E2 |
| I2 | Threshold soundness (no under-threshold op enacted) | green-real | lineage-groups E2.1; tightened in Phase 2.6 |
| I3 | Provenance / standing from signed data alone | green-real + green-model | lineage-groups E2.7; lineage-group-model A1 |
| I4 | Forward-key linearity (one live epoch) | green-real | lineage-groups E1.1, E1.2, E3.1 |
| I5 | Deterministic survivor selection | green-real | lineage-groups E2.3; adversarial A2.1 |
| I6 | No silent membership contradiction (hard-stop) | green-real + green-model | lineage-groups E2.4/E3.2; lineage-group-model C1/C4 |
| I7 | History never corrupts | green-real + green-model | lineage-groups E2.6; lineage-group-model B/D |
| I8 | Backfill verifiability | green-real | lineage-groups E2.7 + backfill_adversarial |
| I9 | Fold/unfold lossless and inert | green-model | lineage-group-model D3 |
| I10 | Convergence after no-conflict heal | green-real + green-model | lineage-groups E2.3/E3.1; model B3 |

## Phase 0 — Scaffold

| ID | Experiment | Status |
|---|---|---|
| P0 | Trivial scenario green + logically reproducible (MLS RNG not seedable — honest finding) | green-real |

## Phase 1 — Crypto/protocol feasibility (THE GATE) — **GO**

real openmls 0.8.1. The external-commit-builder + new-group/re-add path compose; PCS holds.

| ID | Experiment | Asserts | Status |
|---|---|---|---|
| E1.1 | Removed member cannot decrypt post-removal traffic (PCS) | I4 | green-real |
| E1.2 | External commit brings B-member into A's epoch; identical secrets (the survivor primitive) | I4 | green-real |
| E1.3 | Fresh genesis → clean third epoch; parents dead | mint-a-third | green-real |
| E1.4 | Queued remove commit applied later still rekeys; removed stays out | broker-carries-revocation | green-real |

## Phase 2 — Data model + merge semantics — **GO**

| ID | Experiment | Asserts | Status |
|---|---|---|---|
| E2.1 | Under-threshold remove rejected by all honest clients | I2 | green-real |
| E2.2 | Adding a member never confers admin standing; forged-genesis op rejected | I1 | green-real |
| E2.3 | Partition → non-conflicting ops → heal → identical DAG+epoch | I10, I5 | green-real |
| E2.4 | Contradictory remove/keep → hard-stop, no silent re-admit | I6 | green-real |
| E2.5 | Rejected conflict-merge leaves two valid groups (resting state) | resting-state | green-real |
| E2.6 | Forced fresh-genesis inherits both logs read-only; nothing reordered | I7 | green-real |
| E2.7 | Entitled backfill verifies/imports; forged branch rejected | I8, I3 | green-real |
| E2.8 | Reconcile produces distinct navigable branches, not a merged scroll | anti-"six tapes" | green-real |

## Phase 2.5 / 2.6 — Adversarial passes (found and closed real gaps)

| ID | Probe | Result |
|---|---|---|
| A2.1 | Order-independent convergence under fuzzed delivery (300×4 random orders) | green — no order-dependence |
| A2.2 | Governance equivocation | **gap found** — "fork-detecting/attributable" needed hardening |
| A2.4 | Does a removed/departed genesis admin still govern? | **gap found → closed** — authority is now per-epoch |
| A2.5 | Can a hard-stop be resolved only by explicit threshold-signed decision? | green — `quorum_override`; below threshold the stop stands |

## Multi-device (per-device keys under one lineage)

source: thinking/multi-device.md. Logic proven in lineage-group-model (B1 = INV-LINEAGE-NOT-LEAF);
the openmls leaf-credential dependency (8.1) is still a real-library item (dependency #2 below).

| ID | Experiment | Status |
|---|---|---|
| INV-LINEAGE-NOT-LEAF | N devices in one lineage change no threshold outcome | green-model (lineage-group-model B1) + **green-real (E2.10, 2026-06-16)** |
| E2.10 | thresholds count lineages not leaves (own-device quorum manufacture blocked) | **green-real** — `lineage-core` `meets_threshold_by_lineage`; test `e2_10_lineage_thresholds.rs` (the by-DID count is shown unsafe; lineage count rejects it). Rests on T1. |
| E2.9, E2.11–E2.15 | fold, revocation, self-sync, leave-one/all, asymmetry, self-removal ordering | **green-real (2026-06-16)** — see local proof batch |
| E2.16 | tier degradation visibility (forward-send works + stale surfaced without superpeer) | spec — transport/runtime, node-fabric tier |

## Phase 3 — Real iroh thin slice — **GO (with transport caveat)**

| ID | Experiment | Asserts | Status |
|---|---|---|---|
| E3.1 | Partition → broker carries missed commit → one live epoch | I10, I4 | green-real |
| E3.2 | Contradiction hard-stops + escalates; signed op travels opaquely via broker | I6 | green-real |
| E3.3 | Device-loss → new device for same DID joins via external commit + broker snapshot | recovery (open) | see Phase 3 findings; recovery still the largest residual risk |
| E3.4 | Broker observes only ciphertext + routing | blind-broker | green-real (IP/timing still observable, as expected) |

## Social-layer visibility tests (V1–V9) — all pass (modeled)

proven in `Proofs/lineage-group-model` (`V_visibility.ts` + `SOCIAL_LAYER_FINDINGS.md`).
**This discharges the V-prompt seed.** Genesis payload gained regime,
outward_propagation_depth, inward_visibility, openness_class.

| ID | Test | Status |
|---|---|---|
| V1 | Regime born-in and immutable | green-model (fully structural) |
| V2 | Content carries origin regime (signed) | green-model (fully structural) |
| V3 | No silent regime crossing | green-model **for automatic crossing only** — see finding |
| V4 | Republish is a distinct authored act | green-model (fully structural) |
| V5 | Outward depth enforced by verifier (hostile sender) | green-model (verifier-side) |
| V6 | Openness caps depth; fully-open = depth 0 | green-model (closed:3/open:1/fully_open:0) |
| V7 | Inward visibility and outward propagation independent | green-model |
| V8 | Public membership leaks only membership | green-model (DAG isolation) |
| V9 | Freeze-by-default across regimes | green-model |

**Highest-value finding (V3):** structural only against *automatic/silent* crossing — the
protocol cannot stop a human from typing intimate text into a public republish. That is a
UX-layer control. Must be addressed before shipping republish. (Carried in COHESION.md #2.)

## Social-layer safety invariants (S1–S4)

source: thinking/social-layer.md §4.

| ID | Invariant | Status |
|---|---|---|
| S1 | Freeze by default | green-model (lineage-group-model V9) |
| S2 | Scoped visibility, not opaque structure | spec |
| S3 | Asymmetric / quiet membership (reachable without being mapped) | spec — **unsolved**, the hard one |
| S4 | Multi-identity, no forced linkage | spec |

---

## Cross-machine validation (2026-06-15, SSH-driven, 3 AWS boxes + 1 NAT'd laptop)

Moves several previously single-process / modeled results onto **genuinely separate machines**
(AWS us-east-1a/1b/1c + a laptop behind a real NAT). New status tag: `green-real-multimachine` =
computed independently on ≥3 real hosts. Full detail: `experiments/iroh/TEST-LOG.md`; plain-language
summary `experiments/iroh/CAPABILITIES.md`; findings `Proofs/lineage-groups/{PART_A_RECONCILE,
LOCAL_FIRST_HISTORY}_FINDINGS.md`.

| ID | Claim | Result | Status |
|---|---|---|---|
| A1/A1b (I5,I6,I10) | Disconnected peers independently compute the same surviving membership state | 3 boxes produced a **byte-identical** reconcile verdict (`5d82a5df…`); contradiction hard-stops with loser preserved+attributed; survivor order-independent across all 4 merge orders | green-real-multimachine |
| A3/A2 | Superpeer is a **capability, not a right** | durable-queue end-state identical with/without broker; broker-tampered log rejected; contradiction-through-broker verdict == peer verdict (`5f79e073…`); no Mode-1-only outcome | green-real-multimachine |
| B1 (transport) | iroh-blobs: integrity, resume, multi-source failover, **off-VPC NAT** | 1 GiB BLAKE3-verified; resume from FsStore; failover when a provider is killed mid-transfer; NAT'd laptop fetched via **relay** (the real phone path the same-VPC tests never exercised) | green-real (real iroh + real NAT) |
| B-gossip | epidemic broadcast: transitive delivery + drop-a-node resilience | mesh formed from one bootstrap node; n1↔n3 delivered without exchanging addrs; survived killing the relaying node mid-run | green-real |
| B2 | iroh-docs sync behaviour | 0.100.0; eventual sync (8/10 in 60 s); **flat LWW silently overwrites on conflict → too weak** for the hard-stop/preserve governance model (Willow-migration input) | characterized |
| B3 | pairing bootstrap | NodeAddr (relay URL + pubkey) + 32-byte TopicId, no direct IP in the invite — the Delta Chat pattern, demonstrated by both the blob NAT fix and gossip | green-real (identity/key-recovery still open) |
| I7/I8/I9 (local-first history) | per-device signed branches; voluntary consensual backfill; **same mechanism for multi-device and group** | 3 boxes each absorbed others as separate navigable branches (no interleave); fold lossless; tampered → `BadSignature`, outsider → `ForeignGenesis` rejected | green-real-multimachine |

| A1 re-formation (trap door) | ejected member re-forms minus removers; legible descent | all 3 boxes → identical reformed genesis `338d8cc8…`; removers excluded from membership but retain lineage standing (history not erased); `shares_lineage_with_original` true | green-real-multimachine |
| **Capstone: reconcile over live iroh** | the reconcile op-log crosses a real iroh P2P transfer, then reconciles | node-1 served its log via iroh-blobs; node-2 fetched it over real iroh (54 ms, byte-identical sha `7a945964…`) and reconciled to the same contradiction verdict | green-real |
| **T2g/MD-G1: per-lineage gossip group over the NAT path (2026-06-16)** | a user's NAT'd device joins its own per-lineage gossip topic and exchanges over the real relay | topic = sha256(lineage genesis); **node-4 (NAT'd Mac) ↔ node-2 (box) bidirectional** via relay, bootstrap by NodeAddr+TopicId only (no direct IP). The path the same-VPC capstone never exercised. `experiments/iroh/TEST-LOG.md` §T2g | green-real |
| **T2g/MD-G2: signed branch carried + verified over the topic (2026-06-16)** | a lineage history branch travels the per-lineage gossip topic, receiver verifies + absorbs distinct, rejects tampered | `altdrive-spike-lineage-sync` (built node-4 + node-2): **bidirectional ABSORB (verified, 3 msgs) + REJECT tampered (broken hash chain)** across the NAT/relay. Transport form of `backfill_import` (E2.12), *structural* half (shared-genesis + contiguity + integrity); Ed25519/standing stays green-real in Proofs. | green-real (transport, structural half) |
| **T11: 3-way local-first history over live iroh (2026-06-16)** | three devices/members each absorb the other two's branches as distinct + reject tampered, over real iroh incl. NAT | `household-group-v1` topic; node-1 + node-2 + node-4(Mac) each `absorbed={other two}:3 rejected=2`. Promotes the file-relayed I7/I8/I9 result to live transport, 3-way; "same mechanism for multi-device AND group" on the wire. | green-real (live transport, structural half) |

**Honesty boundary (updated):** the Part A reconcile has now been run **over the live iroh transport**
(the capstone row) — node-2 fetched node-1's op-log via real iroh-blobs and reconciled it — so the
"file-exchange, not real transport" caveat is **discharged for the 2-way transfer**. The local-first
history exchange is still file-relayed (computation real-multimachine, delivery via the proven
transport is a small follow-on). MLS key schedule still modeled. Identity/key-recovery (E3.3) remains
the largest residual risk.

## 2026-06-16 local proof batch (T1 + multi-device + reconcile corpus + adversarial)

Executed on node-4 (Mac) against real openmls 0.8.1 + `lineage-core`. Full workspace **green: 24
suites / 56 tests, 0 failures**. Tests: `lineage-mls/tests/t1_lineage_credential.rs`,
`lineage-core/tests/{e2_10_lineage_thresholds,multidevice_adversarial,conflict_corpus}.rs`.

| ID | Claim | Status |
|---|---|---|
| T1 | signed lineage claim rides on real openmls leaf; read+verified by another member; forgery rejected; (spike: custom CredentialType accepted too) | green-real |
| E2.9 / C4 | devices of one lineage fold to one actor; add-vs-add of same person doesn't double-count | green-real |
| E2.10 | thresholds count lineages not leaves; own-device quorum manufacture blocked (by-DID count shown unsafe) | green-real |
| E2.11 | revoking one device rotates the epoch; the lineage's other devices are unaffected | green-real |
| E2.12 | self-sync = backfill: two devices of one lineage reconcile via the existing path, no server, branches distinct; foreign lineage refused | green-real |
| E2.13 | leave-one (single device) vs leave-all-under-lineage (every device of a person) are distinct ops | green-real |
| E2.14 | same-lineage device op = 1 sig; cross-lineage device op pays the full (lineage-counted) threshold | green-real |
| E2.15 | a leaf authors its own removal while it has standing, then loses authority | green-real |
| AR-1 | Sybil / fresh lineages never reach a threshold without authorized admin standing | green-real |
| AR-2 | malicious blind sequencer: reorder/duplicate can't change the converged state (200 fuzzed seeds); a dropped op leaves a visibly-behind head (not false "current"); an injected/forged op is rejected (can't manufacture membership) | green-real (sim; cross-machine broker is A3 green-real-multimachine) |
| AR-6 | a DID cannot be double-counted (sigs keyed by DID); a replayed op does not re-enact (BrokenChain) | green-real |
| C3 | concurrent identical remove heals (no false hard-stop) | green-real |
| C7 | dissolve-vs-continue hard-stops (new detector reason `DissolvedThenContinued`; quorum override cannot silently clear it) | green-real |
| C8 | diamond recombine: standing + shares_lineage hold over a two-parent DAG; outsider has none | green-real |
| C9 | governance equivocation detected + attributed (exercises `detect_equivocation`, A2.2) | green-real |
| C10 | ban-evasion: a removed member's new device cannot self-confer standing; re-admit needs a threshold Add | green-real |

Findings: `Proofs/lineage-groups/T1_LINEAGE_CREDENTIAL_FINDINGS.md`.

**Multi-device data-model tier E2.9–E2.15 is now complete (green-real).** Remaining: E2.16 (tier
degradation visibility — a transport/runtime concern, belongs to the node fabric), T3 (real
threshold-signed checkpoint F2), T9 (Merkle trust-proof). Social-model T5 (S2) / T8 (V3) live in
the TS `lineage-group-model` (needs node/npm). Node-fabric + hard-gated tiers per `../TEST-PLAN.md`.

## Incoming proofs

- Hashing-tree / Merkle thinking and code (per the dossier's "offline transitive trust via
  Merkle proofs," §5) — to be added to the `Proofs` repo and linked to I3/I8 and the
  dossier's trust-graph work.

## Real-library dependencies still to verify

1. ~~openmls external-commit + reinit express survivor re-key with PCS~~ — **CLOSED, Phase 1 GO.**

2. ~~openmls lets a lineage-proving credential ride on the MLS leaf (multi-device 8.1)~~ —
   **CLOSED 2026-06-16 (T1), structured path.** A signed, unforgeable `LineageClaim` rides on the
   real openmls 0.8.1 leaf, is read off another member's leaf, and verifies from signed data
   alone; forged claims rejected. Spike-both finding: openmls *also* accepts a custom
   `CredentialType::Other` at founding (no wall there) — structured-BasicCredential was a choice,
   not a forced fallback. Proof: `Proofs/lineage-groups/crates/lineage-mls/tests/t1_lineage_credential.rs`
   + `T1_LINEAGE_CREDENTIAL_FINDINGS.md`. Unblocks E2.9–E2.16.

3. Automerge change-metadata growth is compactable so snapshots beat SSB's unbounded-log trap
   — roll-up correctness proven in model (lineage-group-model F/G); real-crypto threshold-signed
   checkpoint (F2) still modeled.
