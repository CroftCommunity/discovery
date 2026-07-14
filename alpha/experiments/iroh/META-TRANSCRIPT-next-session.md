# Meta-transcript — next session brief (SSH-driven, two sandbox hosts)

date: 2026-06-15 (written for a future session to execute)

status: **plan, not yet run.** This is a forward brief — a "meta-transcript" of what the next
working session should do. It is self-contained: a fresh session driving the two AWS sandbox boxes
over SSH from this Mac should be able to run it without prior context. When executed, capture the
results and fold them back (collect transcripts the same way `sandbox-transcripts/` was populated).

Two goals, now run across **three** real sandbox peers (a third node was added — see §0). The
third peer is the important upgrade: two same-VPC boxes could only test point-to-point over trivial
direct UDP, which proves little. A third node unlocks the claims that genuinely need 3+ parties:

- the **superpeer two-mode** test (capability-vs-right) — you need a third party to *be* the broker;
- **multi-source** blob transfer and **gossip** transitive delivery — meaningless with only 2 nodes;
- a **3-way fork** reconcile — a stronger determinism demonstration than 2-way; and
- if node-three is off-VPC, the **first real NAT-traversal / relay-fallback** test (the thing real
  phones depend on, which the same-VPC "Spike 2 PASS" never exercised).

- **Part A** — convert the lineage-group "fork + deterministic reconcile" reframe from *argued* to
  *demonstrated*: disconnected peers independently compute the same surviving membership state (2-way
  baseline + 3-way), and the superpeer is shown to be a *capability, not a right* via the two-mode
  test with node-three as broker. (`../../discovery/thinking/experiment-suite.md`,
  `../../Proofs/lineage-groups`.)
- **Part B** — advance the Alt.Drive iroh spike: finish Spike 2 (bigger-scale + multi-source +
  off-VPC path), add a 3-node gossip test, then Spikes 1 / 4 (iroh-docs, pairing); reconcile the
  Alt.Drive ↔ Croft naming/scope while in there.

---

## 0. Operational facts (verified 2026-06-15)

**Hosts** (try the same key first: `/Users/cpettet/Downloads/chase-sandbox-one.pem`, user `ubuntu`):

| alias | public IP | VPC IP / AZ | usual role |
|---|---|---|---|
| secroute-testing-one | 54.172.175.109 | 172.31.43.122 / us-east-1c | peer / fetcher |
| secroute-testing-two | 34.207.146.151 | 172.31.19.13 / us-east-1b | provider |
| secroute-testing-three | 3.84.55.217 | **TO VERIFY** | broker / 3rd peer (NEW) |

(Verify each with `hostname -I` before assuming which is which — the notes warn they flip.)

- **Step 0 for node-three (do this first, it changes the test design):** SSH in and determine its
  network position — same VPC/SG as the other two (private 172.31.x path) or only reachable over the
  public internet? Confirm the key works (if `chase-sandbox-one.pem` fails, ask for its key). Record
  AZ, VPC IP, and whether UDP 2112 is open to/from it. This matters because the two original boxes
  are **same-VPC** — their "Spike 2 PASS" only exercised trivial direct UDP. If node-three sits on a
  genuinely separate network path, it is the **first real test of iroh's actual value prop** — NAT
  traversal, hole-punching, and relay fallback — which is what real-world phones depend on and what
  a same-VPC test proves nothing about. Prefer using node-three to force the non-direct path.
- Node-three's role **flips by test**: an always-on **broker/superpeer** for the two-mode tests
  (A3, B-queue), and an equal **third peer** for the 3-way fork (A1b), multi-source (B1), and gossip
  (B-gossip) tests.
- The boxes share one VPC; **iroh pins UDP 2112** — the AWS Security Group must allow UDP 2112
  among all three SG/IPs. If a fresh direct-path test fails, check the SG rule first (and confirm the
  rule was extended to node-three).
- On each box the repo is at `/mnt/data/alt.drive` (symlink `/home/ubuntu/alt.drive`); `CARGO_HOME`
  and `target/` are symlinked onto a `/mnt/data` EBS volume because the 8G AMI root can't hold
  iroh's ~300-crate build. There is an **unattached 150G EBS volume in us-east-1a** (wrong AZ to
  attach to either box — EBS is AZ-locked).
- Repo remote on the boxes: `git@github.com:AltID/alt.drive.git`, push key `~/.ssh/id_secroute`
  (`GIT_SSH_COMMAND="ssh -i ~/.ssh/id_secroute -o IdentitiesOnly=yes" git push origin main`).
- **Commit identity on the boxes:** `git -c user.email="chase@owasp.org" -c user.name="Chase Pettet"`
  (the box default may be the L360 identity — always override per-command; never modify git config).
- **Discipline:** `altdrive-core` is strict TDD (RED→GREEN→MUTATE→REFACTOR, Zeroize on secrets, no
  `unwrap` outside tests). **Phase-0 spikes are TDD-exempt throwaway crates** (`crates/altdrive-spike-*/`).
  Wait for explicit user approval before any `git commit` on the boxes.

**Driving from here:** SSH/scp need network + the key, so run those Bash calls with the sandbox
disabled. Collect artifacts with the proven `tar -C ~/… -cf - <paths> | tar -xf - -C <local>`
pattern. Do **not** collect `~/.claude/.credentials.json` or the MCP auth cache. Secret-scan any
collected logs before committing (literal `PRIVATE KEY-----`, `ghp_…`, `AKIA…`, `xox[bap]-…`).

**iOS / binding reference (do not re-derive):** Delta Chat ships iroh on iOS in production. Study
`chatmail/core` (the Rust core) + `deltachat-rpc-server` (their JSON-RPC-over-stdio binding, which
they found far simpler to maintain than per-method C-FFI/JNI), and their iroh usage: ephemeral
channels started **lazily**, bootstrap by exchanging `NodeAddr` (relay URL + pubkey) + a random
32-byte `TopicId`, **direct IP deliberately excluded from the invite**, and blocking calls
dispatched on a background thread rather than Rust→native callbacks. This is the prior art for the
eventual mobile path; it is existence-proof that iroh-on-iOS is hard-not-unproven.

---

## Part A — lineage-group reconcile, on three real partitioned peers

**The claim under test** (`design-notes-addendum.md`, §"two-mode convergence" and §"capabilities
not rights"): membership changes never require the superpeer; when peers diverge, complementary
divergence auto-converges and contradictory divergence is detected + presented but never
auto-resolved — and crucially, **two fully-disconnected peers can independently compute the same
surviving state from only the histories they hold.** If they can, the decentralization claim is
real. If they need the superpeer to break the tie, that's a finding too (it answers the
"is the superpeer a capability or a right" question).

**Step A0 — run the simulation suite first (fast, single-process).** Build/run the harness from
`discovery/thinking/experiment-suite.md` (real SHA-256 Merkle ancestry; modeled MLS epochs +
transport; scripted social scenarios; assertions). Confirm groups A (ancestry/LCA stable across
peers, tamper detection), B (complementary convergence + order-independence stress), C (the
ejected-and-re-added hard-stop, social-resolution-as-input, the C4 negative test that auto-resolve
*cannot* be bypassed), D (trap door + divergent-history still searchable). Decide the language
(Rust on the boxes is natural given the stack; the suite is logic, not transport). Capture the
pass/fail table.

**Step A1 — the headline two-host test (the whole point).** Put the *real* reconcile logic on both
boxes. Construct a shared group with a known common ancestor. Then, with the two boxes unable to
reach each other (drop the UDP 2112 path / firewall it, or just run them offline-to-each-other),
have each box independently apply a *contradictory* membership op (e.g. box-two ejects a member;
box-one re-adds the same member) in the same epoch. Each box then runs the deterministic
survivor-selection rule **locally, with no superpeer present**. Reconnect. Assert:

- both boxes detect the fork (the commit-hash-exchange mechanism), and
- both boxes select the **same** surviving version by the deterministic tiebreak (the Willow-style
  cascade: timestamp → payload digest → length, or whatever the suite encodes), **without** a
  third party ordering it, and
- the *losing* op is preserved + attributable (not silently dropped), and
- a member ejected on the surviving branch can re-form a group minus the removers, and the common
  lineage makes that re-formation legible.

If all four hold → the reframe is demonstrated, the superpeer is confirmed a *capability not a
right*. Record exactly which step (if any) needed the superpeer.

**Step A1b — the 3-way fork (third peer; stronger determinism test).** Now use all three boxes as
equal peers sharing one ancestor, mutually partitioned, each applying a *different* contradictory
same-epoch op (node-1 re-adds X; node-2 ejects X; node-3 changes a role/threshold). Reconnect.
Assert all **three** converge on the *same* surviving version by the deterministic cascade with no
orderer — and that convergence is independent of reconnection order (reconnect 1↔2 first, then 3;
then redo with 2↔3 first). A 2-way tiebreak can accidentally look deterministic; a 3-way fork that
converges regardless of merge order is the real evidence the join is a true semilattice.

**Step A3 — the two-mode superpeer test (the headline the third node unlocks).** Designate
node-three as the always-on **broker**. This is the direct measurement of *capability vs right*:

- **Mode 1 (broker present):** node-1 and node-2 act as "phones" that sync *through* node-three.
  Test the durable-queue: node-1 posts a message + a membership commit while node-2 is **offline**;
  node-three holds it; node-2 comes online and syncs the missed state from node-three. Let
  node-three order any concurrent membership commits. Record latency and that nothing is lost.
- **Mode 2 (broker absent):** kill node-three. node-1 and node-2 must still reach the *same end
  state* directly (this is Step A1), just slower / with more coordination. Then bring node-three
  back and confirm it re-converges as an equal replica, holding no authority the peers didn't grant.
- **The verdict:** is there any outcome reachable in Mode 1 that is **not** reachable in Mode 2?
  Faster/cheaper/less-coordination in Mode 1 is fine. *Possible-only-with-broker* is a leak —
  node-three would then be exercising a right, not a capability. Record precisely.

**Step A2 — the conformance check.** For every superpeer-assisted shortcut, write down the
no-superpeer path that reaches the same end (even if slower). Any reachable-only-with-superpeer
outcome is a leak — flag it.

**Capture for A:** the suite pass/fail table; the 2-way and 3-way fork/reconcile results; the
two-mode (Mode 1 vs Mode 2) verdict on capability-vs-right; the conformance notes. Fold into
`Proofs/lineage-groups` (or a new `Proofs/lineage-reconcile`) with a CODING-TRANSCRIPT, and update
`crystallized/proof-ledger.md`.

---

## Part B — advance the Alt.Drive iroh spike

**Where it stands (2026-06-05):** Spike 2 (iroh-blobs hello-world) PASS — 54-byte payload
round-tripped, BLAKE3-verified on receive, VPC direct-UDP path, UDP 2112. `altdrive-core` has the
crypto primitives (SymKey, XSalsa20-Poly1305 secretbox, Argon2id KEK; 12 tests). No iroh in core
yet. Spike crates are throwaway under `crates/altdrive-spike-*/`.

**Step B1 — finish Spike 2 (bigger-scale + genuine multi-source + off-VPC path).** Per
`docs/phase-0-spikes.md`: transfer a **5 GB** blob; verify BLAKE3 integrity end-to-end; test
**resume** (kill the fetch midway, restart, confirm it resumes not restarts). With three nodes,
**multi-source is now real**: have node-1 and node-2 both seed the blob and node-3 fetch from both
(confirm chunks come from both providers, not one), or seed on one and fetch concurrently from two.
And the high-value addition: if node-three is off-VPC (per §0 Step 0), **run at least one transfer
over the non-direct path** to actually exercise hole-punching / relay fallback — the same-VPC boxes
never did. Record throughput, wall-clock, peak `target`/disk use (mind the EBS volume), whether the
path was direct/hole-punched/relayed, and the multi-source speedup if any. Watch disk — 5 GB + build
artifacts must fit on `/mnt/data`.

**Step B-gossip — iroh-gossip 3-node mesh (new, needs the third peer).** Stand up an iroh-gossip
topic across all three nodes and test **transitive delivery**: a message published by node-1 reaches
node-3 even when node-1↔node-3 can't talk directly (it routes via node-2). Then test mesh
resilience: drop one node and confirm the other two still deliver. This is the empirical check on the
epidemic-broadcast (HyParView/PlumTree) claim that underpins the interaction-tiers design
(the broadcast / quiet-large tiers in `discovery/research/germ-xchat-features.md`). Record fan-out
behavior and whether a 3-node mesh holds when a relaying node leaves.

**Step B2 — Spike 1 (iroh-docs).** Stand up an iroh-docs replica synced across the three boxes;
exercise LWW key-value sync, reconnection after partition, and the "always-on peer as durable queue"
pattern with **node-three as the broker** (two phones offline at different times, broker holds, each
syncs on reconnect — the multi-node version of the B-queue / A3 pattern). Note where iroh-docs' flat
LWW model is too weak for the lineage-group needs (empirical input to the iroh-docs → Willow-shaped
migration decision). Confirm the iroh version actually in use (the spike pinned an older iroh).

**Step B3 — Spike 4 (pairing).** Implement the device-pairing bootstrap: exchange `NodeAddr` (relay
URL + pubkey) + a 32-byte `TopicId`, **excluding direct IP from the invite** (the Delta Chat
pattern), and establish a channel from that alone. This is the seam that later becomes the
"join in ten seconds" UX and feeds the still-open identity/key-recovery problem — note recovery
implications as you go.

**Step B4 — Spike 3 (macFUSE) is NOT for these boxes.** macFUSE is macOS-only; the sandbox boxes
are Linux. Either defer Spike 3 to a local-Mac session, or substitute the Linux equivalent (FUSE)
purely to de-risk the mount-as-a-folder concept, and clearly label it as not the macOS path.

**Step B5 — reconcile Alt.Drive ↔ Croft.** `discovery/NAMING.md` records **Croft** as the name
center of gravity, superseding Alt.Drive/Altis. The boxes' repo and memory still say "Alt.Drive."
Decide and write down the relationship: is Alt.Drive (the encrypted vault substrate) *subsumed by*
Croft, a *sibling layer under* Croft, or the *substrate Croft is built on*? Then make the minimal
consistent updates — at least the `iroh/` dir's docs that hardcode `AltID/alt.drive` and the
Mac-vs-Linux `coding-agents` `@`-include paths in `iroh/CLAUDE.md` — so the folded-in copy is
coherent in its new `CroftCommunity/experiments` home. (This is the deferred half of "update to
live in this new repo.")

**Capture for B:** the spike results (throughput / resume / multi-source numbers, and the
direct-vs-hole-punched-vs-relayed path for the off-VPC transfer), the gossip transitive-delivery +
resilience result, the iroh-docs findings, the pairing-bootstrap result, and the naming decision.
Collect the boxes' updated Claude Code transcripts into `sandbox-transcripts/` (same exclude+scan
rules) and fold the status into `README.md` / the spike's own `VALIDATION.md`.

---

## Done-criteria for the session

0. Node-three's network position recorded (same-VPC vs off-VPC) — it determines whether B1/B-pairing
   exercise real NAT traversal or just direct UDP again.
1. Fork/reconcile has a clear verdict: 2-way (A1) **and** 3-way (A1b) converge deterministically with
   no orderer, or needs-superpeer with the reason recorded — Part A's reason for existing.
2. The two-mode test (A3) has a capability-vs-right verdict: any Mode-1 outcome unreachable in Mode 2
   is flagged as a leak.
3. Spike 2 closed at 5 GB with resume + genuine multi-source numbers and at least one off-VPC
   (hole-punch/relay) transfer, or a precise reason it stalled.
4. iroh-gossip 3-node transitive delivery + drop-a-node resilience characterized.
5. iroh-docs sync behavior characterized; the Willow-shaped migration input recorded.
6. Alt.Drive ↔ Croft relationship decided and the folded-in `iroh/` docs made internally consistent.
7. Fresh transcripts collected (credentials excluded, secret-scanned) and the status folded back;
   `proof-ledger.md` / `VALIDATION.md` updated.

Open risks to keep in view while running: identity/key recovery is still unsolved and B-pairing
touches it; the superpeer's true role is exactly what A3 measures; if node-three turns out to be
same-VPC too, the real NAT-traversal test is still unproven (note it and find another path); and
watch disk headroom on the EBS volumes during the 5 GB spike.
