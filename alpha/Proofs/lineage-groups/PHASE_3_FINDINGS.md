# Phase 3 Findings — End-to-End Thin Slice + Blind Broker

**Date:** 2026-06-14
**Crate:** `lineage-iroh` (transport seam, blind broker, node wiring).
**Run:** `cargo test -p lineage-iroh`

## Gate result: **GO, with a documented transport caveat** ✅⚠️

The thesis-critical Phase 3 behaviour runs end-to-end across nodes over a
transport, wiring the *real* Phase 1 MLS and Phase 2 governance: genesis →
fork → recombine, partition/reconnect convergence, conflict hard-stop,
total-device-loss recovery, and a structurally-blind broker. The one honest
caveat — explicitly anticipated by the plan ("a documented negative result is
an acceptable deliverable") — is that the **real `iroh` dependency is not
vendored**; see below.

## Experiment results

| Exp  | Proves | Invariant | Result |
|------|--------|-----------|--------|
| E3.1 | Partition → reconnect; broker carries the missed commit; all nodes reach one live epoch | I10, I4 | **PASS** |
| E3.2 | Contradictory remove/keep under partition hard-stops + escalates; signed op travels opaquely via broker; no silent re-admit | I6 | **PASS** |
| E3.3 | New device for the same DID recovers to the live epoch via broker snapshot + external commit; decrypts forward only | recovery / I4 | **PASS** |
| E3.4 | Broker observed only ciphertext + routing; never plaintext or membership | blind-broker | **PASS** |
| Demo | genesis → fork → recombine across nodes; fresh third epoch unrelated to parents; lineage DAG preserves provenance + standing | composition | **PASS** |

## The real-iroh decision (the documented negative result)

A feasibility probe (`cargo add iroh`) showed adding iroh:

- **Locks ~395 packages**, including *pre-release* crypto crates
  (`ed25519-dalek 3.0.0-pre.6`, `curve25519-dalek 5.0.0-pre.6`,
  `sha2 0.11.0-rc.5`, `der 0.8.0-rc`, …) that **collide with our pinned stable**
  `=2.2.0` / `=0.10.9`, forcing duplicate major versions into the tree.
- Would **dramatically expand the Cycode license-scan surface** (≈395 crates)
  and pull *unstable* deps — both against the dependency discipline maintained
  through Phases 0–2 (exact pins, committed lock, no floating/pre-release).
- **Cannot be exercised here anyway:** real P2P (QUIC/UDP, relays) is outside
  this environment's network policy.

Per the plan (§4 Phase 3 gate, §6.4), the honest move is to **document where the
real transport breaks an assumption that held in sim** and keep transport behind
a seam. So:

- Transport is the [`transport`] module + [`broker::BlindBroker`]; the logic
  never knows whether bytes travel over iroh or an in-process queue.
- **What an iroh binding would change, and *only* that:** replace the
  `BlindBroker` queue with iroh `Endpoint`s + an `iroh-gossip` topic per group
  (Delta Chat pattern: random topic id, lazy P2P bootstrap), keeping the exact
  same `Envelope { to, topic, kind, ciphertext }` seam. No logic crate changes.
- This is a drop-in for a network-enabled environment with a dependency budget
  for iroh's tree; it was deliberately not vendored into this sandbox.

## Design decisions and honesty boundaries

1. **Broker blindness is structural, not promised (E3.4).** `broker.rs` imports
   nothing from `lineage-mls`, `lineage-core::gov`, or `lineage-history`; it only
   handles `Envelope`s of opaque bytes + a `NodeId`/`GroupTopic`/coarse kind tag.
   It therefore *cannot* read plaintext or membership. The test also checks no
   observed buffer (incl. recovery snapshots) contains any known plaintext.

2. **Residual metadata is acknowledged.** A relay still sees routing metadata —
   destination, topic id, a coarse Welcome/Handshake/App framing tag, timing and
   volume — exactly the residual Signal's sealed sender leaves. The topic id is a
   *random* id (not membership-derived), so routing by topic reveals nothing
   about who is in the group. We claim blindness to *content and membership*,
   not to network-layer traffic analysis.

3. **Governance travels over the same blind seam (E3.2).** Signed ops are
   serialized and shipped as opaque `Handshake` payloads; the broker relays them
   without understanding them, and conflict resolution still hard-stops to a
   human escalation hook — no silent re-admit.

4. **Recovery is real but bounded (E3.3).** A new device for the same DID
   rejoins the live epoch via an external commit built from a broker-held
   snapshot, and forward secrecy holds (it cannot read pre-recovery traffic).
   This composes the Phase 1 external-commit primitive (E1.2) with the broker
   snapshot store; it does **not** recover past message history (that would be
   consensual backfill, Phase 2 E2.7) and is not a full account-recovery design.

## What this does and does not establish

**Establishes:** the two-tree model composes end-to-end across nodes over a
transport; partition/reconnect converges to one live epoch; conflict hard-stops
over the wire; device-loss recovery via a blind broker snapshot works with
forward secrecy; and a blind broker is structurally expressible.

**Does NOT establish:** behaviour over real iroh/QUIC (not vendored here, by
deliberate decision), NAT traversal / relay behaviour, performance or scale,
network-layer metadata resistance, or any production/audited posture. The real
iroh integration is specified as the next step for a network-enabled, dependency-
budgeted environment.

## Overall

All ten invariants the thesis set out to falsify (I1–I10) held across the three
phases in deterministic simulation, with each phase's go/no-go gate cleared and
every corrected assumption recorded. The remaining unknown is real-transport
behaviour, isolated behind a single trait so it can be de-risked without
touching the validated logic.
