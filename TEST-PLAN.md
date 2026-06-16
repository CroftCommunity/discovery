# Extended Test Plan — the remaining proof surface, sequenced

date: 2026-06-16

status: planning artifact, pre-execution. Defines every still-open testable claim across the
corpus and stacks them in dependency order so they can be executed one tier at a time. This is
the sequencing doc ANALYSIS.md said was missing — the proof-ledger tracks *status*, ROADMAP
tracks *product/validation tracks*; neither sequences the remaining proof work into an
executable order. This does.

## Execution status (2026-06-16)

**Green-real this session (node-4/Mac, real openmls 0.8.1 + lineage-core; 26 suites / 61 tests, 0
failures):** T1, E2.9, E2.10, E2.11, E2.12, E2.13, E2.14, E2.15, AR-1, AR-6, C3, C4, C7, C8, C9,
C10. The **multi-device data-model tier E2.9–E2.15 is complete.** See `proof-ledger.md` →
"2026-06-16 local proof batch."

**Remaining, by what unblocks it:**
- *Local Rust (next turns):* E2.16 tier visibility (transport/runtime), T3 F2 checkpoint, T9 Merkle.
- *TS `lineage-group-model` (needs node/npm):* T5 S2, T8 V3.
- *AWS box fabric (SSH-driven):* T2g, T11, TI-1/3/4/5, AR-2/3/4/5, relay E5/E6/E7.
- *Hard-gated on resources only you can provide:* T10 (bsky app-password+egress), T13 (iOS host),
  E0-NAT (public 3343/3478), E4 (ipvsadm), E8/E9 (meer binary), TI-6 (G6), T6/T7 (G5 design),
  T4 Achilles (research session — I can run this).

---

companion docs: `crystallized/proof-ledger.md` (status of every I/E/V/S row),
`COHESION.md` (loose-end ↔ proof map), `ROADMAP.md` (two-track product/validation view),
`thinking/merge-split-corpus.md` (the three-tree merge/split case corpus that Tier 1b draws
from), `thinking/multi-device.md` §10 (the per-lineage gossip-group sync realization).

---

## Problem statement

Most of the falsifiable backbone is already green: Phase 1 crypto gate GO on real openmls
0.8.1, Phases 0/2/2.5/2.6/3 GO, V1–V9 social tests green-model, cross-machine validation on
3 AWS boxes + a NAT'd laptop. The proof-ledger moved past ANALYSIS.md's "unfinished" list the
same evening it was written.

What remains is a **specific, finite set** of claims that are still `spec`, `green-model`
(not yet real-crypto), modeled-but-library-unverified, unrun, or gated on a user decision.
These are scattered across the ledger, COHESION, ROADMAP, and the two unrun seed prompts.
Without one sequenced list, the temptation is to grab whichever is most visible rather than
the one that unblocks the most downstream work.

## Approach

Classify every remaining testable claim into eight tiers ordered by dependency and leverage.
Each item names **what it asserts**, **where the proof lives** (real repo path), its
**dependency / gate**, and a concrete **exit criterion**. Decisions that are the user's to
make (recovery anchor, license gate) are listed as explicit gates — not resolved here, per
the working rules (AGENTS.md: "Don't resolve the user's decisions").

## Reasoning

The ordering optimizes two things at once: (1) unblock the largest downstream surface first —
the openmls leaf-credential check (T1) gates the entire multi-device Rust tier (T2); (2) do
the cheap-but-high-signal work that needs no external resources before the work that needs
egress allowlists, mobile toolchains, or a user decision. Adversarial research (T4) is placed
early because it is pure analysis (no crypto code) and its findings may reshape later tiers —
running it before building more proofs lets it redirect effort. Recovery (T12) sits late not
because it is unimportant (it is the single largest residual risk) but because it cannot be
tested until its anchor is *chosen* — testing precedes nothing here; a decision does.

Legend for status carried from the ledger: `green-real` proven w/ real crypto · `green-model`
proven in sim · `spec` defined not built · `blocked` waiting on dep/decision.

---

## Gates (must precede the tiers that depend on them)

| Gate | Decision / resource needed | Blocks | Owner |
|---|---|---|---|
| **G1 — Recovery anchor** | **DECIDED 2026-06-16:** device-delegation primary (logical binding, `multi-device.md` §10.1) + **optional HD seed as lose-all-devices backup-only**. T12 proves the backup mechanism. | T12 | resolved |
| **G2 — Egress + creds** | Allowlist `bsky.social`, `plc.directory`, Jetstream host; provide app-password env. `live-bsky-validate/RESUME.md` is the self-contained handoff. | T10 | user (provisioning) |
| **G3 — Mobile toolchain** | Android NDK/SDK for APK assembly (`android-p2p-app/PATH_TO_APK.md`); an iOS build host for T13. | T13 | user (environment) |
| **G4 — License gate** | MPL-2.0 call on `hpke-rs` (mandatory for RFC 9420 HPKE; no permissive substitute). Not a test gate — gates *publishability* of lineage-groups. | (publish, not test) | user (compliance) |
| **G5 — S3/S4 design** | Quiet-membership (S3) and multi-identity (S4) have open protocol-expression questions (`social-layer.md` lines 75–77). A design pass must precede their tests. | T6, T7 | design-then-test |
| **G6 — Read-receipt bucket** | Choose: best-effort+opt-in (gossip bucket, zero-broker) vs durable-on-normal-path (CRDT, normal metadata profile). `interaction-tiers.md` says decide deliberately, never default silently. | TI-6 | user (design decision) |

G2 (V6 openness/size class scheme) from the original V-prompt is **already closed** — the model
fixed `MAX_DEPTH_FOR_CLASS` = closed:3 / open:1 / fully_open:0.

---

## Execution environment — the test fabric (and how the nodes differ)

This plan runs on the existing 4-node fabric from the 2026-06-15/16 validation campaigns
(`experiments/iroh/TESTING-DESIGN.md`, memory `croft-validation-session-2026-06-15`,
`croft-relay-lab-2026-06-16`). **The node differences are load-bearing for test placement** —
a test put on the wrong node proves nothing (e.g. a "NAT traversal" test on a same-VPC pair
silently takes the direct path and passes for the wrong reason).

| node | host | network position | hardware | storage | role bias |
|---|---|---|---|---|---|
| **node-1** | AWS `54.172.175.109` / us-east-1c | same-VPC `vpc-217f0f5c` — **direct private path** | **4c / 15G (fat)** | `/mnt/data` EBS, warm cargo/target | peer / fetcher / fat-relay |
| **node-2** | AWS `34.207.146.151` / us-east-1b | same-VPC — direct | 2c / 7.7G | `/mnt/data` EBS, fully provisioned (HEAD `83d4389`) | provider / broker / hub |
| **node-3** | AWS `3.84.55.217` / us-east-1a | same-VPC — direct | **2c / 3.8G (smallest)** | **128G root, NO `/mnt/data`** | 3rd peer / broker / generator |
| **node-4** | this Mac | **off-VPC, behind real NAT** | local | contained `CARGO_*` under `experiments/` | **the only NAT/relay-forcing peer** |

### Placement rules derived from the differences

1. **NAT / hole-punch / relay-path tests → must include node-4.** node-1/2/3 are same-VPC; a
   relay-only *dial address* does **not** force passthrough on iroh 1.0 (it upgrades to direct
   via hole-punch) — force relay-only with `.clear_ip_transports()` and never call `bind_addr`.
   E0 hole-punch *via the NAT'd Mac* still needs **public ingress on 3343/3478** (open gap).
2. **Scaling / load / accounting tests → cgroup-pin to identical 2vCPU/4GiB slices** (the
   MULTIPLEX decision) so the 4c/15G vs 2c/3.8G asymmetry doesn't contaminate the curve. node-1
   is the natural fat-relay host; node-3 the natural generator (co-located with what it drives).
3. **Storage-heavy tests (blobs, large stores) → node-1 / node-2 only** (`/mnt/data`).
   node-3 and the Mac lack `/mnt/data` — parameterize any hardcoded `/mnt/data/...` path.
4. **Co-location is mandatory for relay placement** (E2: relays don't mesh; wrong assignment =
   no connection). Matchmaking same-VPC resolves to **DIRECT** (relay carries 0 post-handoff),
   so relay-passthrough load must be driven from node-4 or with forced relay-only.
5. **Ports / SG:** only **UDP 2112** is open among the three on the private path. Verify
   reachability with a **Python TCP-accept + UDP-sink probe, not `nc`** (nc missed even the
   known-open 2112).

### Run mechanics (so the plan is executable as-is)

- **Boxes are keyless compute.** No git remote on the boxes. Sync code with
  `tar -C <src> -cf - <paths> | ssh box 'tar -xf - -C <scratch>'`; build/run there; collect
  results with the reverse tar-over-ssh. **Commits happen on the Mac only**, into `CroftC`, with
  the `chasemp` (`chase@owasp.org`) identity, on explicit approval (PLAYBOOK §3b).
- **Drive multi-host via `ssh box 'bash -s' <<'EOF'` heredocs** — NOT `"bash -lc '...'"` with
  nested quotes (broke detached launches). Rust stdout is **block-buffered** when redirected to a
  file for a long-lived process — flush after printing the addr line. `pkill -f <name>`
  self-matches the ssh argv — use `sudo fuser -k 2112/udp` or `systemctl stop <unit>`. SSH-driving
  gotchas: memory `ssh-driving-secroute-sandbox`.
- **Version pins differ by workspace** — note skew: lineage/transport spikes used iroh
  `1.0.0-rc.1` / blobs `0.102` / docs+gossip `0.100`; the relay-loadtest lab pins iroh `=1.0.0`
  (+ iroh-relay `=1.0.0`). Pin per-experiment; record the pin in each finding.
- **Honesty boundary to preserve:** Part-A reconcile determinism was proven by byte-identical
  verdicts across the 3 boxes via **file-exchanged** op-logs; the capstone moved the 2-way
  reconcile onto **live iroh**. Live-transport delivery for the 3-way and for local-first history
  is the remaining transport follow-on (T11). State modeled-vs-real-vs-live-transport explicitly.

### Node placement per test item

| Item | Node(s) | Why / constraint |
|---|---|---|
| T1, T1b, T3, AR-1, AR-6 | node-4 (Mac, logic) | pure Rust/openmls + reconcile logic; no network. Mac is fine; boxes optional for cross-machine determinism replay |
| T2 | node-4 build; cross-machine replay on 1/2/3 | E2.10/E2.12 logic local; replay determinism across boxes as A1b did |
| **T2g, T11** | **node-4 + node-2(hub) + node-1/3** | self-sync/history **over live iroh gossip**; node-4 forces the real NAT/relay path the same-VPC capstone never exercised |
| T4 | node-4 (research) | analysis; no nodes |
| T5–T8 (S2/S3/S4, V3) | node-4 (model) | `lineage-group-model` TS, sim-only |
| T9 (Merkle) | node-4 | new proof, local |
| T10 (live-bsky) | node-4 + egress (G2) | needs allowlist + app-password; not box-bound |
| T12 (recovery) | node-4 + 1/2/3 | seed re-derive + admit via governance across boxes |
| T13 (iOS) | iOS host (G3) | neither boxes nor Mac suffice — needs an iOS build/runtime host |
| T14 (Willow) | node-2 (same-proc) then 1/2/3 | as B2 was characterized; 3-replica cross-host follow-on |
| **TI-1, TI-4, TI-5** | node-4 + node-2(hub) + 1/3 | presence/ack/degrade over gossip; node-4 proves the broker-sees-nothing claim on the real NAT path |
| TI-2 | node-4 (model) | genesis-type immutability, sim |
| **TI-3** | node-1/2 (`/mnt/data`) + 3 | broadcast soak needs storage headroom + sustained posting |
| TI-6 | node-4 + hub (G6) | receipt bucket behaviour |
| **AR-2 (malicious sequencer)** | node-2 (broker) + 1/3 peers + node-4 | active drop/reorder at the broker; include node-4 for the relay-carried path |
| AR-3 (backfill DoS) | node-3 (smallest) as victim + node-1 flooder | put the victim on the **3.8G** box to surface exhaustion soonest |
| **AR-4 (metadata-leak)** | node-4 ↔ boxes via relay | only the off-VPC/relay path exposes the realistic routing/timing profile |
| **AR-5 (MLS-tree scaling)** | node-1 (fat 4c/15G) | the member×device leaf blow-up needs the largest box |
| **Relay-lab E0-NAT, E4–E9** | per lab spec (pinned slices) | see Tier 11 |

---

## Tier 1 — Real-library / real-crypto (modeled → real). Highest leverage.

Closes the gap between "logic proven in TS model" and "the real library actually does this."

### T1 — openmls leaf-credential carries lineage  `[modeled-sound, library-OPEN]`
- **Asserts:** a lineage-proving credential can ride on the MLS leaf, so other clients can fold
  devices and count thresholds by lineage. This is real-library dependency #2 (ledger) and
  COHESION #7. The TS model proves INV-LINEAGE-NOT-LEAF *assuming* the leaf↔lineage mapping;
  this proves openmls 0.8.1 can carry it.
- **Approach (decided 2026-06-16): spike both, document the wall.** First attempt a real custom
  `Credential` type / `LeafNode` extension to find exactly where openmls 0.8.1 stops (the crate
  already documented a comparable wall for reinit, `lineage-mls/lib.rs:19-26`); then fall back to
  a structured, signed `BasicCredential` identity `{lineage_id, device_did, lineage_sig}` where
  `lineage_sig` (lineage-root-signed) makes the claim unforgeable. The library-limitation finding
  is itself a deliverable.
- **Key-hierarchy (DECIDED, §10.1):** logical binding — `lineage_sig` is the **lineage-root
  signature over an *independent* device key** (not a derived one). The optional HD recovery seed
  (T12) is backup-only and does not enter the credential format. The credential format is now
  unblocked.
- **Where:** `Proofs/lineage-groups` (Rust, real openmls) — new experiment alongside Phase 2.5,
  in the `lineage-mls` wrapper crate.
- **Dependency:** none (key-hierarchy fork resolved).
- **Exit:** a leaf credential encoding a *signed, unforgeable* lineage claim round-trips through
  a real openmls commit + the `tls_codec` wire and is readable/verifiable by a second member; an
  outsider's forged lineage claim is rejected; OR a documented finding that openmls cannot carry
  it and the fold must source the mapping elsewhere. Update ledger dep #2 + E2.9–E2.16 status.

### T2 — Multi-device E2.9–E2.16 on real Rust  `[spec → real]`
- **Asserts:** fold, lineage-counted thresholds (no quorum from own devices, E2.10), device
  revocation as a governance op, self-sync-as-backfill (E2.12), leave-one-vs-leave-all,
  asymmetry, ordering, tier visibility — against real openmls, not the model.
- **Where:** `Proofs/lineage-groups`.
- **Dependency:** **T1** (the fold rests on the leaf credential being real).
- **Exit:** E2.9–E2.16 move from `spec (Rust) / partially green-model` to `green-real`; E2.10
  and E2.12 are the gates (no manufactured quorum; serverless self-sync).

### T2g — Self-sync over the real per-lineage gossip group  `[pieces green-real, composition new]`
- **Asserts:** the §10 hypothesis — a user's endpoints form an iroh gossip group scoped to their
  lineage and reconcile by gossiping signed branches + backfill import, each local-first. Cases
  MD-G1…MD-G5 (`multi-device.md` §10.2): per-lineage topic, branch-broadcast+backfill,
  drop-a-device resilience, add-vs-add fold (corpus C4), revoked-device-cannot-rejoin.
- **Where:** `experiments/iroh` (gossip + transport, B-gossip is here) composed with
  `Proofs/lineage-groups` (backfill + fold). This is the single-user instance of T11.
- **Dependency:** **T2** (fold + revocation logic) + transport (proven). Pieces green-real
  (B-gossip, I7/I8/I9, B3); composition is the new work.
- **Exit:** two offline-diverged devices of one lineage converge over real iroh gossip with no
  server; MD-G4 (add-vs-add fold) and MD-G5 (revoked device dark) hold. Promotes E2.12 from
  `spec` to `green-real` on live transport.

## Tier 1b — Reconcile-case corpus (the wide merge/split array)

Runs alongside Tier 1 — shares the `lineage-core` reconcile machinery. Source + full taxonomy:
`thinking/merge-split-corpus.md` §4. These widen `conflict.rs::detect` (today: one reason) to
cover the unmodeled cases the corpus surfaced. Each becomes a ledger row.

### T1b — Conflict-reason coverage  `[C1/C2/C6 green-real; C4/C7/C8/C10 GAP; C9 partial; C3 verify]`
- **Asserts (in leverage order):**
  - **C4 — multi-device add-vs-add fold** (gap): same person online from two partition-divergent
    devices folds to one actor, no double-count vs E2.10. *Highest priority — it is MD-G4.*
  - **C7 — dissolve-vs-continue** (gap): define intended resolution (likely resting-state +
    escalate), then prove it.
  - **C8 — diamond recombine** (gap): extend `detect` to multi-parent ancestry; conflict
    semantics over an A↔B↔C re-merge.
  - **C9 — equivocation hardening** (partial, A2.2): a forked author's two contradictory ops are
    *attributable*, not merely detected.
  - **C10 — ban-evasion re-add** (gap): a new device leaf under a previously-removed lineage must
    not silently re-confer standing (moderation; pairs with T4 FM8).
  - **C3 — concurrent-identical-remove** (verify, cheap): both sides booting the same DID heals,
    no false hard-stop.
- **Where:** `Proofs/lineage-groups/crates/lineage-core` (`conflict.rs`, `dag.rs`) + `reconcile-harness`.
- **Dependency:** C4 pairs with T2 (lineage fold); the rest are standalone on the reconcile core.
- **Exit:** each case green-real with a ledger row; the three-tree mapping in the corpus holds
  for every case (MLS / governance-DAG / history-CRDT columns all accounted for).

### T3 — Real threshold-signed checkpoint (F2)  `[green-model → real-crypto]`
- **Asserts:** roll-up/compaction checkpoints carry a *threshold* signature (not an
  authority/superpeer signature), and a checkpoint cannot span an open fork. Real-library
  dependency #3; COHESION #3/#4. Makes the "referee leak" (is the superpeer secretly the
  signer?) a visible real-crypto test result.
- **Where:** `Proofs/lineage-groups` (real-crypto re-run of `lineage-group-model` group F).
- **Dependency:** none hard; benefits from T1's credential work.
- **Exit:** F2 status `green-real`; ledger dep #3 closed; principle "snapshot/compaction is a
  first-class requirement" graduates with a real-crypto backing.

## Tier 2 — Adversarial research (no crypto code; may redirect later tiers).

### T4 — Run the Achilles-heel research prompt  `[unrun]`
- **Asserts (adversarially):** maps the lineage model against the field's eight recurring
  failure modes — escaped / partially escaped / relocated / inherited. Top targets, in the
  prompt's own priority: (1) the ordering/consensus "dirty secret" — is the superpeer secretly
  the MLS ordering service? Pressure-test against F3/F5 and the PR #3 finding that a membership
  sequencer *is* load-bearing (COHESION #4); (2) total-device-loss recovery (feeds G1);
  (3) governance/moderation with no server (captured quorum, malicious majority, ban-evasion
  via device-leave); (4) the composability seam; (5) the unnamed Achilles heel.
- **Where:** run as a research session; output → `discovery/research/achilles-heel-findings.md`;
  carried findings → COHESION + ledger. Prompt: `seeds/generated-prompts/achilles-heel-research-prompt.md`.
- **Dependency:** none. Placed early so findings can redirect T5–T15.
- **Exit:** the four-part output the prompt demands (field synthesis · head-to-head mapping ·
  strongest case against · open questions w/ options). The ordering-authority verdict is the gate.

## Tier 3 — Social-layer spec invariants (untested).

V1–V9 are green-model and discharge the V-prompt. The S-invariants are not.

### T5 — S2 scoped visibility, not opaque structure  `[spec]`
- **Asserts:** the model offers visibility scoped to consented distance/resolution, and does
  *not* offer "visible structure with hidden identities" (topology deanonymizes). The honest
  Kevin-Bacon-distance idea, made structural.
- **Where:** `Proofs/lineage-group-model` (extend `V_visibility.ts` / new `S_visibility.ts`).
- **Dependency:** none. **Exit:** S2 `green-model`; a structure-only share is shown to be
  re-identifying and therefore rejected/unrepresentable.

### T6 — S3 asymmetric / quiet membership  `[spec — UNSOLVED, the hard one]`
- **Asserts:** a leaf can be in a group for reachability while withholding its other edges from
  that group's view — "reachable without being mapped." This is where the inside-adversary
  problem lives.
- **Where:** model first, then real. **Dependency:** **G5** (design pass — `social-layer.md`
  line 75 is an open protocol-expression question). **Exit:** either a structural proof that
  quiet membership is expressible, or an honest finding that it is only partially achievable and
  what it costs (this is the most important social-layer result after V3).

### T7 — S4 multi-identity, no forced linkage  `[spec]`
- **Asserts:** multiple identities under one person's control with no provable linkage —
  distinct lineages with deliberately no linkage vs one lineage with scoped facets
  (`social-layer.md` line 77).
- **Where:** `Proofs/lineage-group-model`. **Dependency:** **G5**. **Exit:** S4 `green-model`;
  the no-forced-linkage property holds against an inside adversary trying to correlate facets.

### T8 — V3 republish UX-control  `[structural done, control unspecced]`
- **Asserts:** the highest-value V-finding — the protocol cannot stop a human typing intimate
  text into a public republish (V3 is structural only against *automatic* crossing). The
  UX-layer control that addresses it must be specified and then tested. COHESION #2.
- **Where:** spec in `thinking/social-layer.md` §8–10 backport; test home TBD by the control
  chosen. **Dependency:** none (but pairs with the §8–10 doc backport, ROADMAP #2).
  **Exit:** a named control + a test that a republish cannot silently carry private plaintext,
  recorded as a shipping requirement.

## Tier 4 — Trust-graph proof (incoming, not built).

### T9 — Hashing-tree / Merkle offline transitive-trust proof  `[incoming]`
- **Asserts:** offline transitive trust via Merkle proofs (dossier §5) — links I3/I8 to the
  trust-graph design. "Incoming" per the ledger; not yet in the Proofs repo.
- **Where:** new proof in `Proofs/`. **Dependency:** none. **Exit:** a verifiable Merkle proof
  that a trust assertion chains offline without a live authority; linked to I3/I8.

## Tier 5 — Live-network validations (built/partial → live).

### T10 — Live-bsky validation  `[built, waiting on egress]`
- **Asserts:** flips H2/H3/H5 from "validated locally" to "validated against the live network";
  surfaces custom-NSID acceptance and CID parity from DAG-CBOR canonicalization. ROADMAP #9.
- **Where:** `Proofs/encrypted-local-first-atproto/live-bsky-validate` (built; `RESUME.md` is
  the handoff). **Dependency:** **G2** (egress allowlist + app-password env).
  **Exit:** H2/H3/H5 status `green-real (live)`; findings folded into the dossier's atproto
  sections (COHESION #8).

### T11 — Local-first history exchange over live iroh  `[follow-on]`
- **Asserts:** the small follow-on to the cross-machine capstone — the local-first history
  branch exchange (currently file-relayed; computation already real-multimachine) crosses the
  proven iroh transport, not a file copy.
- **Where:** `experiments/iroh` + `Proofs/lineage-groups`. **Dependency:** none (transport
  proven). **Exit:** the I7/I8/I9 history exchange row gains a live-transport delivery, closing
  the last "file-exchange, not real transport" caveat.

## Tier 6 — Recovery (gated on a decision).

### T12 — Total-device-loss recovery proof  `[open — largest residual risk]`
- **Asserts:** once an anchor is chosen, prove the chosen mechanism — graduation of E3.3.
  Anchor is now decided (G1): device-delegation primary + **optional HD seed backup-only**.
  T12 proves the seed path — re-derive a fresh authorized device from a mnemonic after total
  device loss, with the seed never used in live ops (so it is not a live single-point-of-
  compromise), and the re-derived device admitted via the normal governance path.
- **Where:** `Proofs/lineage-groups` (extends E3.3). **Dependency:** G1 resolved.
  **Exit:** the chosen mechanism proven to recover a lineage after total device loss without
  violating the stated threat model; E3.3 status updated; COHESION #12 closed.

## Tier 7 — Platform feasibility.

### T13 — iOS runtime spike  `[Android proven, iOS unspiked]`
- **Asserts:** the iOS-specific unknowns the Android spike (PR #7) did not cover —
  battery/background/cellular-NAT for iroh-on-device via UniFFI. COHESION #10: Android ≠ iOS here.
- **Where:** new `experiments/ios-p2p-spike`. **Dependency:** **G3** (iOS build host).
  **Exit:** a two-peer Automerge-over-iroh sync demonstrated on iOS (or a documented blocker:
  background-execution / cellular-NAT limits).

## Tier 8 — Substrate & economics (designed, unproven; lowest priority).

### T14 — Willow migration vs iroh-docs flat-LWW  `[characterized → migration unproven]`
- **Asserts:** iroh-docs' flat LWW silently overwrites on conflict (B2, characterized) — too
  weak for the hard-stop/preserve governance model. Prove the Willow (or alternative) migration
  preserves contradiction rather than overwriting.
- **Where:** `experiments/iroh`. **Dependency:** none. **Exit:** a sync layer that hard-stops on
  governance conflict instead of LWW-overwriting; B2 superseded.

### T15 — Proof-of-attention / consumer-pull ad inversion  `[designed only]`
- **Asserts:** the economic layer's consumer-pull inversion + Lightning micropayments
  (dossier M3) — proof-of-attention is the open primitive. **Where:** new spike.
  **Dependency:** none (but lowest leverage; M3 is years out). **Exit:** a minimal
  proof-of-attention primitive demonstrated, or a documented reason it is deferred.

## Tier 9 — Interaction tiers & delivery guarantees (added 2026-06-16 pass)

source: `thinking/interaction-tiers.md` — the three-products-share-a-send-button model
(interactive / quiet-large / broadcast). Entirely `spec` today; B-gossip (green-real) underpins
the presence transport. This whole area was absent from the first plan.

### TI-1 — Presence/typing ride gossip, never the broker  `[spec; B-gossip green-real]`
- **Asserts:** the metadata-leak-dissolves claim — when peers hold a direct iroh connection,
  ephemeral presence/typing rides that channel and the broker sees nothing; with no direct path
  it silently degrades to nothing (the correct behaviour).
- **Where:** `experiments/iroh`. **Dep:** B-gossip. **Exit:** broker observes zero
  presence/typing in the direct-connection case; absence is graceful, not an error.

### TI-2 — Type-at-creation, no live conversion  `[spec]`
- **Asserts:** interactive vs broadcast are distinct object types fixed at genesis (reuse I1);
  no op converts one to the other in place ("becoming broadcast" is create-new-and-redirect,
  preserving MLS membership coherence).
- **Where:** `lineage-core` gov (genesis param) / model. **Dep:** I1. **Exit:** a convert-in-
  place op is unrepresentable/rejected; type is immutable like genesis thresholds.

### TI-3 — Broadcast = bounded rolling-forward log (SSB-shaped, anti-SSB)  `[spec; G-soak green-model]`
- **Asserts:** the broadcast tier keeps an append-only rolling window but drops immutable-
  infinite-history — bounded storage under sustained posting (the lesson SSB failed), gossip-
  replicated best-effort. COHESION #11.
- **Where:** `experiments/iroh` + extend `lineage-group-model` G-soak. **Dep:** G-soak.
  **Exit:** storage bounded under a long broadcast soak; old entries roll off; no unbounded growth.

### TI-4 — Guarantees-by-tier + the "you'll know it didn't get there" ack structure  `[spec]`
- **Asserts:** interactive = prompt delivery + a real failure signal (distinguish failed from
  not-yet-heard); quiet-large = eventual (arrives or you're told it didn't, not "when");
  broadcast = best-effort, no per-recipient accounting. The ack structure is real work.
- **Where:** `experiments/iroh`. **Dep:** transport + gossip. **Exit:** interactive distinguishes
  failed-vs-pending; each tier's degraded guarantee is surfaced, never silent (fail-clearly).

### TI-5 — Auto-degradation by behaviour/size  `[spec]`
- **Asserts:** presence/typing switch off above a size/rate threshold automatically, without
  changing object type; the absence is the communication (UI promises less, deterministically).
- **Where:** model/experiments. **Dep:** TI-1. **Exit:** crossing the threshold disables rich
  signals deterministically and visibly.

### TI-6 — Read-receipt bucket behaves as chosen  `[blocked on G6]`
- **Asserts:** read receipts behave per the G6 decision — best-effort+opt-in (gossip, zero-
  broker) OR durable CRDT on the normal sync path (normal metadata profile) — and never default
  silently into the wrong bucket.
- **Where:** `experiments/iroh` or `lineage-history`. **Dep:** **G6**. **Exit:** receipts honour
  the chosen bucket's metadata profile; the choice is explicit and surfaced.

## Tier 10 — Adversarial & robustness surface (added 2026-06-16 pass)

The corpus implies these but never enumerated them as proof targets. Several pair with T4
(Achilles research) — run T4 first; it may sharpen or add to this tier.

### AR-1 — Sybil / fresh-lineage threshold resistance  `[gap]`
- **Asserts:** distinct from E2.10 (own-device quorum) — minting many *fresh lineages* must not
  let an attacker meet a social threshold without authorized adds by existing members. Checks
  the open/public join flows don't leak a Sybil path to standing.
- **Where:** `lineage-core` gov + model. **Dep:** E2.10. **Exit:** no fresh-lineage population
  reaches any social threshold absent authorized adds.

### AR-2 — Malicious sequencer: censorship + reorder resistance  `[gap — sharpens the "dirty secret"]`
- **Asserts:** a *blind* sequencer/superpeer (E3.4) still cannot, by dropping or reordering
  commits, (a) change the deterministic survivor, (b) stall governance silently (stalls must be
  visible), or (c) manufacture a different membership state. The active-attack complement to F5
  (no rights escalation) — the strongest test of "ordering role is minimal, blind, not a rights
  authority" (COHESION #4). **Pairs with T4.**
- **Where:** `lineage-iroh` broker + `reconcile-harness`. **Dep:** F5/E3.4. **Exit:** broker
  reorder/drop changes nothing (determinism/idempotence) or is detected + surfaced; no silent
  outcome change.

### AR-3 — Backfill / gossip abuse (DoS)  `[gap]`
- **Asserts:** a hostile donor flooding garbage/oversized branches is rejected cheaply (forged →
  BadSignature/ForeignGenesis already proven) AND cannot exhaust the recipient (bounded
  verification cost, rate/size caps); gossip amplification is bounded.
- **Where:** `lineage-history` backfill + `experiments/iroh` gossip. **Dep:** backfill
  (green-real). **Exit:** hostile-donor flood bounded in CPU/memory/storage; honest sync unaffected.

### AR-4 — Transport metadata-leakage bound  `[characterize]`
- **Asserts/characterizes:** what a blind broker + relay can infer from routing/timing/volume
  (E3.4 noted IP/timing observable). Bounds it and links to social-layer S2 (topology
  deanonymization) and the inside-adversary/scanner model.
- **Where:** `experiments/iroh` + a written characterization. **Dep:** E3.4. **Exit:** an honest,
  documented metadata-leakage profile (observable vs not) — characterization, not pass/fail.

### AR-5 — MLS tree + rekey scaling under per-device-as-member  `[characterize]`
- **Asserts/characterizes:** ratchet-tree size and commit/rekey cost as group×devices grows
  (e.g. 50 members × 3 devices = 150 leaves) — the cost `multi-device.md` flagged as possibly
  underestimated (tree size, device-churn log noise).
- **Where:** `Proofs/lineage-groups` (real openmls bench). **Dep:** T2. **Exit:** a measured cost
  curve; the wall (if any) for the per-device-as-member choice named honestly.

### AR-6 — Signature replay / double-count  `[gap]`
- **Asserts:** a single signature cannot count twice toward a threshold; a revoked key's
  signature is not counted post-revocation; an op cannot be replayed into a later epoch to
  re-enact. Hardens I2/E2.1 against an active forger.
- **Where:** `lineage-core` gov. **Dep:** I2/E2.1. **Exit:** replay/double-count rejected by all
  honest verifiers.

*(Fold candidate: PCS-recovery — does survivor external-commit re-key restore post-compromise
security for the re-keyed side? — sits naturally in Tier 1 alongside E1.1/E1.2; noted here so it
is not lost.)*

## Tier 11 — Relay & Placement Lab continuation (E0-NAT, E4–E9)  `[E0–E3 done; rest open]`

source: `experiments/iroh/RELAY-PLACEMENT-LAB-SPEC.md`, conclusions in `RELAY-LAB-CONCLUSIONS.md`,
per-run data in `relay-lab-runs/`. Crate: `crates/relay-loadtest` (spike-class, TDD-exempt). This
half-run lab was never in the plan; it is the substrate-economics proof for "how big a relay/co-op
node needs to be." Pins iroh `=1.0.0`. **Already done:** E0 memory-wall (~31 KiB/relayed conn,
≈130k idle conns on a 4G slice), E0 cross-host (same-VPC → 20/20 DIRECT, passthrough ~186 MiB/s
CPU-bound on 2 vCPU), E1 (one fat process scales across cores), E2 (co-location mandatory),
E3 (Automerge anti-entropy over iroh, pop 30 → 30/30 converged, 0 drops).

### E0-NAT — hole-punch via the real NAT'd Mac  `[blocked]`
- **Asserts:** the hole-punch / relay-fallback path that the same-VPC boxes cannot exercise.
- **Node:** node-4 (Mac) ↔ boxes. **Blocked on:** **public ingress on 3343/3478** to the relay
  host (SG only opens 2112). **Exit:** Mac behind NAT either hole-punches direct or falls back to
  relay, measured.

### E4 — LVS / IPVS horizontal fan-out  `[open]`
- **Asserts:** whether an L4 VIP across relay processes is needed, or E1's "one fat process per
  shard" suffices. **Node:** node-1 (fat) + a VIP. **Needs:** `ipvsadm` + IPVS config (heavy infra).
  **Exit:** a measured verdict — likely confirms one-process-per-shard is simpler.

### E5 — cgroup group-accounting  `[light, mostly demonstrated]`
- **Asserts:** per-tenant/per-shard resource accounting via the cgroup slices already used in
  E0/E1/E3. **Node:** any box. **Exit:** accurate per-slice accounting under mixed load.

### E6 — `tc` traffic shaping  `[open]`
- **Asserts:** behaviour under shaped/degraded links (latency, loss, bandwidth caps). **Node:**
  box pair + `tc netem`. **Exit:** delivery guarantees (ties to TI-4) hold or degrade visibly
  under shaping.

### E7 — placement churn  `[open]`
- **Asserts:** correctness as endpoints are reassigned between relays under load (E2 said
  placement is authoritative; this stresses re-placement). **Node:** 2 relays + generators.
  **Exit:** churn converges without stranding connections.

### E8 — relay-vs-`meer` fork  `[blocked]`
- **Asserts:** comparison against the `meer` alternative. **Needs:** the `meer` binary.
  **Node:** box pair. **Exit:** a measured relay-vs-meer tradeoff.

### E9 — confidentiality tiers (`meer`)  `[blocked]`
- **Asserts:** the confidentiality-tier behaviour of the meer path. **Needs:** `meer`. **Node:**
  box pair. **Exit:** tiers behave as specified; ties to the blind-broker claim (E3.4 / AR-4).

Also outstanding in the lab: reconnect-storm handshake-CPU driver; admit-hook HTTP gate;
DNS-origin/pkarr placement integration (`DnsAddressLookup::builder`, `PkarrPublisher`).

---

## Execution order (the stack)

```
G-gates run as needed ──┐
                        ▼
T1  openmls leaf-credential ──► T2  multi-device E2.9–E2.16 ──► T2g self-sync over gossip
                        │            │  (real)                     (MD-G1…G5, = single-user T11)
                        │       T1b  reconcile-case corpus (C4 add-vs-add pairs w/ T2; C7/C8/C9/C10/C3)
                        │
T3  real threshold-signed checkpoint (F2)
                        │
T4  Achilles-heel research  ◄── findings may redirect everything below
                        │
T5  S2 ──► (G5 design) ──► T6 S3 (the hard one) ──► T7 S4
T8  V3 UX-control
                        │
T9  Merkle trust-proof
                        │
T10 live-bsky (G2) · T11 history-over-iroh
                        │
(G1 decision) ──► T12 total-device-loss recovery
                        │
T13 iOS spike (G3)
                        │
T14 Willow migration · T15 proof-of-attention

  ── added 2026-06-16 pass (largely independent of the T1→T2 spine) ──
Tier 9  interaction tiers:  TI-1 presence-via-gossip · TI-2 type-at-creation · TI-3 broadcast
                            bounded-log · TI-4 guarantees/ack · TI-5 auto-degrade · TI-6 receipts (G6)
Tier 10 adversarial:        AR-1 Sybil/fresh-lineage · AR-2 malicious sequencer (pairs T4) ·
                            AR-3 backfill/gossip DoS · AR-4 metadata-leak bound · AR-5 MLS-tree
                            scaling (needs T2) · AR-6 sig-replay/double-count
```

Note: tier numbers (Tier 9/10) are distinct from item ids (T9 Merkle, T10 live-bsky). New-tier
items use TI-/AR- prefixes to avoid the collision.

**Recommended first execution:** T1 (openmls leaf-credential) — no gate, real workspace ready,
and it unblocks the entire multi-device tier (T2). T4 (Achilles research) can run in parallel
since it needs no crypto code and its findings should land before the social-layer and recovery
tiers.

## What is NOT in this plan (housekeeping, not tests)

These are doc/cohesion tasks tracked in ROADMAP/COHESION, deliberately excluded so the plan
stays a *test* plan: social-layer §8–10 backport (ROADMAP #2 — but it gates T8); public-path
duplication reconcile (COHESION #5 / ROADMAP #10); sequencer-honesty framing (COHESION #4 /
ROADMAP #11); name-map pin (done — NAMING.md); cooperative charter specifics (ROADMAP #8).
