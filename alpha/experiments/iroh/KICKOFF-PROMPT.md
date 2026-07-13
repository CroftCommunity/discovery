# Kickoff prompt — Relay & Placement Lab (paste after /clear)

Paste the block below into a fresh Claude Code session (cleared context) on this workstation to
resume the Croft Relay & Placement Lab. It is self-contained and points only at in-repo files.

---

```
Drive the Croft "Relay & Placement Lab" from this workstation (Mac) over SSH. The Mac is BOTH the
driver (runs Claude Code, holds the repo + SSH key) AND a test node (node-4, behind real NAT).
Working dir: /Users/cpettet/git/chasemp/CroftC. Git identity: chasemp (chase@owasp.org,
github-personal). Press shift+tab now for accept-edits mode.

═══ THE TEST FABRIC (4 nodes; all testing happens across these) ═══
SSH key ~/Downloads/chase-sandbox-one.pem, user ubuntu, for all three boxes. All three AWS boxes are
in the SAME VPC (vpc-217f0f5c); only UDP 2112 is open in the Security Group among them (widen it if
the relay/LVS/DNS-controller need more ports — check first).

 node | public IP        | VPC IP / AZ              | disk                  | scratch dirs
 -----+------------------+-------------------------+-----------------------+---------------------------
 1    | 54.172.175.109   | 172.31.43.122 / 1c      | /mnt/data EBS         | /mnt/data/croft-iroh, /mnt/data/croft
 2    | 34.207.146.151   | 172.31.19.13  / 1b      | /mnt/data EBS         | /mnt/data/croft-iroh, /mnt/data/croft
 3    | 3.84.55.217      | 172.31.88.18  / 1a      | 128G root, NO /mnt/data | ~/croft-iroh, ~/croft
 4    | this Mac         | behind real NAT (off-VPC) | local                 | experiments/iroh/ (+ .node4-* builds)

node-4 (Mac) is the only genuinely NAT'd / off-VPC node — it's what forces real hole-punch/relay
paths the same-VPC boxes can't. Boxes have rust toolchains; node-3 needed build-essential (installed).

═══ CONTAINMENT (hard rule) ═══
ALL lab code and artifacts stay inside the `experiments` repo (experiments/iroh/...). Nothing pollutes
the workstation. On the Mac, build with CARGO_HOME + CARGO_TARGET_DIR set under
experiments/iroh/.node4-cargo and .node4-target, and any blob store under .node4-stores — all three
are gitignored and fully deletable. Boxes are keyless compute (no git remote): this repo is the
source of truth; sync code TO boxes and collect results BACK via tar-over-ssh. Commits happen on the
Mac only, on my explicit approval (chasemp/chase@owasp.org); the three repos are
discovery / Proofs / experiments (git@github-personal:CroftCommunity/*, branch main).

═══ READ FIRST — and what each file teaches (the testing modality) ═══
WHAT to run:
  • experiments/iroh/RELAY-PLACEMENT-LAB-SPEC.md   — the 10-experiment program E0–E9, the §5 build
                                                     list, the relay-vs-meer architecture, terminology.
  • experiments/iroh/NEXT-SESSION-2026-06-16.md    — the plan: Step-0 topology decision, priorities,
                                                     deferred TODOs, operational reminders.
HOW to drive the fabric (study these for the modality before building):
  • experiments/iroh/sandbox-transcripts/VALIDATION-METHODS.md — the canonical SSH-driving mechanics:
        the topology table, the UDP-reachability recipe, and the gotchas (no sandbox-disable needed;
        detached-subshell for long procs; fuser-not-pkill to kill remote procs).
  • experiments/iroh/crates/altdrive-spike-iroh/   — pattern for: relay-aware tickets
        (endpoint.online().await + endpoint.addr()), seeding a MemoryLookup from a ticket addr,
        SPIKE_STORE_DIR, provide/fetch, relay-only EndpointAddr for forced passthrough. (rc.1 API.)
  • experiments/iroh/crates/altdrive-spike-gossip/ — pattern for: one-node-per-host, bootstrapping by
        relaying a peer's NodeAddr JSON between hosts via the Mac, binding UDP 2112, subscribe/join.
  • Proofs/lineage-groups/crates/{reconcile-harness,history-harness}/ — pattern for: deterministic
        cross-machine harnesses, JSON exchange between boxes, byte-identical-verdict checks, the
        tar-over-ssh sync + build-on-box loop.
WHERE results go:
  • experiments/iroh/TEST-LOG.md        — chronological source of record (match its format).
  • experiments/iroh/TESTING-DESIGN.md  — campaign spine + per-test status tables.
  • experiments/iroh/CAPABILITIES.md    — plain-language summary (update if capabilities change).
  • discovery/crystallized/proof-ledger.md — fold claims/status here.
  • discovery/AGENTS.md                 — repo orientation.

═══ THE BUILD / SYNC / RUN / COLLECT LOOP (how every experiment runs) ═══
1. Edit code in experiments/iroh/crates/<crate> (source of truth on the Mac).
2. Sync to a box:  tar -C experiments -cf - iroh --exclude='iroh/target' --exclude='iroh/.node4-*' \
   | ssh ... ubuntu@<box> 'tar -xf - -C <scratch>'
3. Build on the box: ssh ... 'source ~/.cargo/env; cd <scratch>/iroh; cargo build -p <crate>'
   (Mac/node-4: export CARGO_HOME/CARGO_TARGET_DIR under .node4-* and build locally — contained.)
4. Run: launch long-lived procs detached — ( setsid CMD >log 2>&1 </dev/null & ) — inside a
   FOREGROUND ssh (top-level remote `&` and tool-level background SSH both exit 255). Relay
   tickets/NodeAddrs between hosts by scp'ing through the Mac. Kill remote procs with
   `sudo fuser -k <port>/udp`, never `pkill -f <pat>` (it self-matches the ssh command → 255).
5. Collect logs/results back via ssh cat / scp; fold numbers into TEST-LOG.md + the spec §6
   (replace [HYPOTHESIS] tags) + proof-ledger.md.

═══ ALREADY DONE & PUSHED (do NOT repeat) ═══
The full 2026-06-15 validation campaign: Part A lineage reconcile (A0/A1/A1b/A3/A2 + re-formation
backstop), Part B iroh transport (blob integrity/resume/multi-source-failover/off-VPC-NAT, gossip
transitive+resilience, iroh-docs characterized, pairing), local-first history (multi-device + group
voluntary backfill), and reconcile/history-over-iroh capstones — committed across all three repos.

═══ YOUR TASK ═══
Execute RELAY-PLACEMENT-LAB-SPEC.md.
  • STEP 0 first: inventory the 4 nodes above and decide topology — the spec assumes 6–8 nodes
    (relay-1/2/3, lb-1, ctrl-1, gen-1/2); we have 3 boxes + the Mac. Decide expand-EC2 vs. MULTIPLEX
    roles onto the boxes (relay processes under cgroup slices + a generator + the placement
    controller co-located, accepting that co-located generators understate the relay wall). Record
    the mapping in a run manifest. Check/adjust the Security Group for needed ports.
  • Then run E0→E9 IN ORDER (each result gates the next). Smallest useful increments first: the E0
    `--mode matchmaking` generator, then the placement controller, then the meer binary.
  • Build a NEW `relay-loadtest` crate under experiments/iroh/crates/ per spec §5. PIN one iroh 1.0.x
    SHA and re-verify every API against that exact source — the spec was verified vs iroh 1.0.0, but
    the existing spikes pin 1.0.0-rc.1 where names differ (MemoryLookup vs MemoryAddressLookup,
    RelayMap ctors). Never guess an API; read it from the pinned source.

═══ EXPLICIT TODO — DO NOT TOUCH unless I open it ═══
Identity & key-recovery is DEFERRED. Notes in NEXT-SESSION-2026-06-16.md: (1) quorum "recovery group"
social recovery via the validated lineage quorum-override/standing path; (2) a minimal 3rd-party VC
issuer for identity+recovery (ties to discovery/thinking/plc-identity-resilience.md). Leave it set
aside. Other non-blocking deferred items: B4 macFUSE, HashSeq single-file striping, the old iroh/
README/DESIGN Alt.Drive→Croft rename.

Commits only on my explicit approval. Start by reading the docs above, then give me your Step 0
topology proposal (expand vs. multiplex, port plan) BEFORE building anything.
```
