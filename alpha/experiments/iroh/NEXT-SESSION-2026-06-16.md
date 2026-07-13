# Next-session test plan — 2026-06-16 (rev. relay/placement lab)

Self-contained brief for a future session driving the AWS sandbox boxes (+ NAT'd Mac) from this Mac
over SSH. Primary new track is the **Relay & Placement Lab** (`RELAY-PLACEMENT-LAB-SPEC.md`).
Everything from 2026-06-15 (Part A reconcile, Part B transport, local-first history, reconcile/
history-over-iroh capstones) is **done and pushed** — see `TEST-LOG.md` / `CAPABILITIES.md` /
`proof-ledger.md`. This plan is the next layer only.

## Operational reminders (learned 2026-06-15 — don't re-discover)

- **SSH/scp need NO sandbox-disable** — the sandbox allows network egress. Run ssh/scp normally;
  `dangerouslyDisableSandbox` only forces needless prompts.
- **Press shift+tab once at session start** (accept-edits mode) so file edits don't prompt
  (`.claude/settings.local.json` loads at startup, not mid-session).
- **Only UDP 2112 is open in the Security Group** among the boxes. Transport spikes that bind another
  port (gossip tried 2113) silently fail to connect same-VPC. Bind on **2112** unless/until the SG is
  widened (for the relay lab you may need more ports open — check/adjust the SG first).
- **Keyless compute, this repo is source of truth.** `tar -C <src> -cf - … | ssh box 'tar -xf - -C
  <scratch>'` to sync; build/run on the box; collect back the same way. **Commits on the Mac only,
  on explicit approval**, `chasemp`/`chase@owasp.org`. Repos: `discovery`, `Proofs`, `experiments`
  (all `git@github-personal:CroftCommunity/*`, branch `main`).
- **Cross-host iroh dials need the relay address in the invite**, not just the NodeId — embed the full
  `EndpointAddr` via `endpoint.online().await` + `endpoint.addr()`; on the receiver seed the in-memory
  address lookup from the ticket addr. (Verified in iroh source, not guessed.)
- **Pin one iroh SHA per run and record it in the manifest** — relay byte-cost per connection changes
  across versions. The relay lab spec was verified vs **iroh 1.0.0**; the existing spikes pin
  **1.0.0-rc.1** (some APIs differ — re-verify against the pinned source).
- Hosts: node-1 `54.172.175.109`/`172.31.43.122` (us-east-1c), node-2 `34.207.146.151`/`172.31.19.13`
  (us-east-1b), node-3 `3.84.55.217`/`172.31.88.18` (us-east-1a) — all same VPC `vpc-217f0f5c`;
  node-4 = this Mac (NAT'd), contained build under `experiments/iroh/.node4-*`. Key
  `~/Downloads/chase-sandbox-one.pem`, user `ubuntu`. Box scratch dirs: `/mnt/data/croft-iroh`
  (node-1/2), `~/croft-iroh` (node-3).

---

## TRACK 1 (PRIMARY) — Relay & Placement Lab

**Run `RELAY-PLACEMENT-LAB-SPEC.md`** — a 10-experiment stacked program (E0–E9) characterizing the
relay load/tuning/topology landscape and drawing the relay-vs-meer (superpeer) boundary on measured
evidence. The throughline: relays don't mesh, so co-location is mandatory, and DNS-placement-by-
namespace is what makes co-location automatic. Read that spec first; it has the architecture facts,
the per-experiment what/why/method/measure/hypothesis, the build list, and terminology.

**Step 0 — node inventory & topology (do first, it shapes the lab).** The spec assumes 6–8 nodes
(relay-1/2/3, lb-1, ctrl-1, gen-1/2); the current sandbox is 3 boxes + the Mac. Decide: expand the
sandbox (more EC2) or **multiplex roles** onto the 3 boxes (e.g. relay processes under cgroup slices +
a generator + a controller co-located), accepting that co-located generators understate the relay
wall. Record the chosen mapping in the run manifest. Also: check/adjust the Security Group for the
ports the relay + LVS + DNS controller need (only UDP 2112 is open today).

**Sequencing (per the spec's dependency order):** E0 baseline → E1 vertical scaling → E2 DNS
placement → E3 namespace-sharded fan-out sync → E4 LVS → E5 cgroup group-accounting → E6 tc shaping →
E7 placement churn → E8 relay-vs-meer fork → E9 confidentiality tiers. Each experiment's results
change whether/how the next is worth running, so go in order and fold numbers back into the spec
(replace `[HYPOTHESIS]` tags) + `TEST-LOG.md`.

**Highest-leverage first increments** (smallest useful work): the E0 `--mode matchmaking` generator
(gives the realistic-vs-worst-case relay cost gap immediately), then the placement controller (the
component most experiments depend on), then the meer binary (without it E3/E7/E8/E9 only measure the
live-peer case and miss offline-data demand).

**Build:** extend a `relay-loadtest` crate under `experiments/iroh/crates/` (new, alongside the
existing spikes), per the spec's §5 build list. Pin the iroh 1.0.x SHA; re-verify APIs (the rc.1
spikes used `MemoryLookup`/different `RelayMap` ctors — the 1.0 names differ).

---

## TRACK 2 — cross-host iroh-docs 3-replica (smaller, can run alongside / is subsumed by E3)

Originally the standalone B2-completion item. The relay lab's **E3** (namespace-sharded fan-out sync)
exercises real cross-host eventual-sync and largely subsumes it; run this standalone version only if
you want a focused iroh-docs result before the full lab harness exists.

**Problem.** B2 was only characterized same-process (node B reached 8/10 entries in 60 s; flat LWW
silently overwrites on conflict). Not yet shown across 3 separate machines, nor reconnect-after-
partition, nor durable-queue-via-broker.

**Approach.** Extend `crates/altdrive-spike-irohdocs` into one-node-per-host `create`/`join`
subcommands, bind UDP **2112**, `online().await`, `share(Write, RelayAndAddresses)` (already embeds
relay), relay the doc ticket between hosts via the Mac. Pass criteria: 3-replica convergence (record
time + explain the same-process 8/10 lag); reconnect-after-partition (firewall a replica's 2112,
insert, unblock, converge); durable-queue (a replica offline while another inserts, catches up on
rejoin); LWW evidence (concurrent same-key writes → one silently wins, the concrete Willow input).

---

## EXPLICIT TODOs — set aside, with notes (do NOT block the lab on these)

### Identity & key-recovery — DEFERRED (design exists in the user's head; capture before building)

The largest open problem (E3.3 in the proof-ledger). **Set aside for now** — the user believes there
are both social and technical solutions and wants to pursue them deliberately, not autonomously. Two
candidate models to spec out together when picked up (NOT yet decided):

1. **Quorum social recovery via a "core"/"recovery" group.** A designated recovery group holds a
   quorum that can, *acting as one*, recover/reissue a key for another member — i.e. recovery is a
   **governance op on the lineage**, not a secret restore. This composes directly with the already-
   validated machinery: the quorum-override / threshold-decision path (`conflict::quorum_override`,
   Part A A2.5/A3) and lineage `standing`. Attacker model to write: what stops a malicious recovery
   quorum from hijacking a DID (threshold size, transparency/attributability, member consent).
2. **Minimal central authority — a 3rd party that only issues a VC for identity + recovery.** Not a
   data-holding server; a credential issuer that attests identity (and underwrites recovery), nothing
   more — minimal trust surface, no payload access. Ties to the DID-method analysis already written in
   `discovery/thinking/plc-identity-resilience.md` (did:webvh recommendation + validating PLC
   read-replica) and the Delta Chat no-central-account prior art.

These are not mutually exclusive — a cooperative could offer both as a governance dial (like the
meer confidentiality tiers). Prior art to read, don't re-derive: `discovery/thinking/
plc-identity-resilience.md`, `discovery/thinking/multi-device.md`. A small throwaway BIP39
paper-recovery round-trip spike (recoveryKey ↔ 24-word mnemonic, KAT-verified, then secretbox-wrap
the masterKey) is the cheapest concrete first step **when this track is picked up** — but not now.

### Other deferred follow-ons (logged, non-blocking)

- **B4 macFUSE** — macOS-only; the boxes are Linux. Defer to a local-Mac session, or a Linux-FUSE
  substitute labelled not-the-macOS-path.
- **HashSeq simultaneous single-file striping** — `SplitStrategy::Split` OOMs on a single raw blob
  (it expands to size/32 range requests); striping one file across providers needs a HashSeq
  representation. We demonstrated source redundancy/failover instead, which is the property that
  matters for a flaky-phone world.
- **Full rename of the old `iroh/` README/DESIGN/CLAUDE.md** from Alt.Drive → Croft.Drive (the new
  docs already use Croft; the load-bearing old design docs were left untouched mid-campaign).

---

## Done-criteria for the relay-lab session

1. Node topology decided + recorded (expand vs. multiplex); SG adjusted for needed ports.
2. iroh 1.0.x SHA pinned; the `relay-loadtest` crate builds on the boxes; APIs re-verified vs source.
3. E0 baseline numbers (matchmaking vs idle-relayed vs active-relayed cost; capacity crossover).
4. As many of E1–E9 as the session reaches, in order, with `[HYPOTHESIS]` tags replaced by measured
   numbers folded back into `RELAY-PLACEMENT-LAB-SPEC.md` §6 + `TEST-LOG.md`.
5. Results folded into `proof-ledger.md`; `CAPABILITIES.md` updated if capabilities changed.
6. Key-recovery remains an explicit TODO (untouched) unless the user opens it. No commits without approval.
