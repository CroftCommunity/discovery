# Drystone delivery layer: experiment plan for Claude Code

`Status: executable validation plan, EXECUTED`

`Purpose: turn the design's load-bearing empirical claims into falsifiable experiments`

`Companion to: 01-delivery-architecture.md (the claims under test live there)`

`Results: executed against iroh 1.0.1, iroh-gossip 0.101.0, iroh-base 1.0.1, mls-rs 0.55.2. 13 experiments, 12 CONFIRMED, 1 FALSIFIED (E1.1). Full per-experiment source, raw output, and verdicts live in the separate results document (05-Drystone-Delivery-Experiment-Results.md). Verdicts are folded into the design doc 01 §11. Summary below.`

---

## Results summary (folded into 01 §11)

- E0.1 iroh 1.x builds and runs: CONFIRMED. E0.2 Ed25519 32-byte key: CONFIRMED.

- E1.1 presence-without-content event exists: **FALSIFIED** (the one branch-reshaping result, narrowly: P-gossip still works via discard-the-blob on stock gossip, a companion channel is rejected as a TOCTOU hazard; the real surviving constraint is that standalone-D-swarm hole-detection rides the payload-embedded signed index). E1.2 no gossip replay for offline nodes: CONFIRMED.

- E2.1 byte-identical ciphertext dedups (one seal, two paths): CONFIRMED. E2.2 local-link delivery with relay disabled: CONFIRMED.

- E3.1 RBSR cost scales with difference not history: CONFIRMED. E3.2 clock-free ordering converges identically: CONFIRMED. E3.3 entitlement boundary (model-form, XOR stand-in): CONFIRMED. E3.4 lineage-gated admission (model-form, hash-chain stand-in): CONFIRMED.

- E4.1 wake-then-fetch recovers all with wake suppressed: CONFIRMED. E4.2 push-payload guard rejects ciphertext: CONFIRMED.

- E5.1 selector delivers exactly once under all path-survival combos: CONFIRMED. E5.2 backgrounded-phone parity: CONFIRMED.

E3.3 and E3.4 used stand-in crypto and validate the logic, not the mls-rs hook wiring; re-running them end-to-end against live mls-rs is in the 01 §11.3 residue.

---

## How to read and run this

Each experiment states:

- **Claim under test:** the specific design assertion, with the doc section it backs.

- **Hypothesis:** what we expect to observe.

- **Method:** what to build and run.

- **Confirms if / Falsifies if:** the observable outcome that decides it.

- **Design consequence:** what changes in the design doc if it falsifies.

A guiding rule before any of this: **do not assert an API shape from memory.** Every experiment begins by reading the then-current crate docs or running `cargo doc`, because at least one crate version moved during the design session itself (iroh-gossip went 0.100.0 to 0.101.0 in a week, and its iroh pin changed from `=1.0.0-rc.1` to `^1`). Pin exact versions in each experiment's output so results are reproducible against a known state.

Experiments are ordered by leverage: the ones that can invalidate a whole design branch come first.

---

## Tier 0: substrate reality checks (fast, do first)

### E0.1 Version and build resolution

**Claim under test:** iroh-gossip builds against stable iroh 1.0 (§2.2; the version-skew [confirm]).

**Hypothesis:** As of the design session, iroh-gossip 0.101.0 pins `iroh ^1`, so a fresh build resolves against iroh 1.0.x and compiles. (This flipped during the session; verify, do not trust.)

**Method:**

- `cargo new` a probe crate. Add `iroh`, `iroh-gossip`, and `iroh-base` with no version pin, then `cargo update` and capture `cargo tree`.

- Record the exact resolved versions of iroh, iroh-base, iroh-gossip, noq, and the feature flags enabled by default.

- Compile a trivial program that binds an `Endpoint`, builds a `Gossip`, and spawns a `Router`, the minimal example from the crate docs.

**Confirms if:** it compiles and resolves iroh at 1.0.x with iroh-gossip at 0.101.x or later.

**Falsifies if:** resolution forces an iroh release candidate, or the build fails on a version conflict.

**Design consequence:** if falsified, the gossip-dependent cells (D-swarm, P-gossip) carry a hard dependency on a pre-stable iroh, which must be stated as a maturity caveat in §3.3 and §4 rather than treated as buildable today.

### E0.2 Endpoint key curve

**Claim under test:** the iroh endpoint key is Ed25519 (§2.1 [confirm], carried from prior rounds).

**Method:** read `iroh-base` 1.0 docs for the `EndpointId` / `PublicKey` / `SecretKey` types and record the named curve. Generate a keypair and inspect the key length and any exposed algorithm identifier.

**Confirms if:** the type documentation or key material is unambiguously Ed25519.

**Falsifies if:** it is a different curve or a curve-agile wrapper. Either way, record the ground truth and update the prior-rounds note in §11.

---

## Tier 1: the claim that can invalidate a design branch (highest leverage)

### E1.1 Can a subscribed non-member observe message presence without content?

**Claim under test:** P-gossip is real, a topic-subscribed node that is not an MLS member can learn that a sealed message exists (via the lazy Ihave hash) without receiving its content (§2.2, §4 P-gossip). Standalone D-swarm's swarm-local hole detection depends on the same observability (§3.3).

**Hypothesis (now in doubt):** the proto layer describes a lazy Ihave (hash-before-content) push. BUT the application-facing `iroh_gossip::proto::topic::Event` enum exposes only `NeighborUp`, `NeighborDown`, and `Received(GossipEvent)`, with no `IHave` variant. So the hash may be consumed internally to drive pulls and never surfaced to the application. If so, an application only learns of a message at `Received`, i.e. when it already holds the content, and pure presence-without-content is not observable on the public API.

**Method:**

- Read the current `iroh_gossip` `api` and `net` module docs and the `proto::topic::Event` and `GossipEvent` definitions. Enumerate every event or callback an application can subscribe to. Specifically search for any exposure of Ihave / lazy-push / message-hash-seen.

- Build a three-node test on a local network (no internet needed): nodes A, B, C all subscribe to one topic. A broadcasts a message. Instrument B and C to log every `Event` received, with timestamps, distinguishing "hash/announcement seen" from "content received."

- Force B into the lazy set relative to A if the API allows influencing eager/lazy assignment (it may not; record whether it does). Observe whether B ever emits an event indicating awareness of the message before the content arrives.

- Separately, inspect `iroh-metrics` output and any tracing spans for Ihave/lazy events, presence may be observable via metrics/tracing even if not via the `Event` stream. Record this as a distinct finding ("observable via instrumentation" is not the same as "available to application logic").

**Confirms if:** there exists a public, application-consumable signal that a message exists prior to (or independent of) receiving its content.

**Falsifies if:** the only application signal of a message is `Received`, which already carries content.

**Design consequence (large):**

- If **confirmed**, P-gossip stands as written and standalone-D-swarm hole detection can read swarm-local presence.

- If **falsified**, two things change. First, P-gossip as "observe presence, never content" is **not** available on the stock gossip API; a presence-only role would require either a custom ALPN protocol alongside gossip (announce sealed-message hashes on a separate channel) or an upstream change. Second, standalone-D-swarm cannot detect holes from gossip presence alone and MUST layer its own causal/sequence metadata inside the sealed payload (the suspected outcome already flagged in §3.3). State this as the resolved answer and revise §4 P-gossip to "requires a companion announce channel, not stock gossip."

  **[Resolution note, post-experiment]:** E1.1 was falsified, but the consequence above was *partially* over-predicted. The second half holds exactly: standalone-D-swarm hole detection rides payload-embedded signed-index metadata. The first half does **not**: P-gossip does not need a companion channel, because it does not need presence-*without*-content. A carrier receives the sealed blob via `Received`, treats arrival as the signal, fires a bare wake, and discards the blob. A companion channel was considered and **rejected** (it reintroduces a TOCTOU race and a new metadata surface for only a bandwidth saving). See design doc §4 for the corrected, no-extra-channel design.

### E1.2 Does an offline node get nothing from gossip (no replay)?

**Claim under test:** D-swarm offers nothing to a node that was fully offline during a message's live window (§3.3), which is why it is weak durability and why the meer or device-pool replay is needed.

**Method:**

- Three nodes on a topic. Node C is taken fully offline (process paused or network cut). A broadcasts N messages. Bring C back online and rejoin the topic.

- Log what, if anything, C receives for the messages sent during its absence, over a generous wait window.

**Confirms if:** C receives none of the messages sent while it was offline (it may receive only messages sent after rejoin).

**Falsifies if:** C recovers some or all missed messages purely from gossip, which would mean the overlay has more durability than the design credits.

**Design consequence:** confirmation hardens the "gossip is not durability" claim and justifies the meer and device-pool replay as the durability story. Falsification would be a pleasant surprise and would let D-swarm carry more weight; either way, quantify the recovery rate rather than asserting zero.

---

## Tier 2: the planes and the combinator

### E2.1 Byte-identical ciphertext enables trivial cross-path dedup

**Claim under test:** the same MLS PrivateMessage arriving via two paths is recognized as one message by hashing the sealed bytes, which is what makes the race-both cell free (§3.4) and is reused as the concurrent-display tiebreak (§7.4).

**Method:**

- Using an RFC 9420 implementation (e.g. `mls-rs`, confirmed to exist this session), create a group, seal one application message, and obtain the wire bytes.

- Deliver the same sealed bytes to a receiver via two simulated paths (two in-process channels standing in for meer and swarm). Confirm the receiver, keyed by a hash of the sealed bytes, stores exactly one message and that both deliveries produce identical hashes.

- Confirm the receiver decrypts the deduped message correctly once.

**Confirms if:** identical sealed bytes yield identical hashes across paths and dedup to one stored, correctly-decrypted message.

**Falsifies if:** the same logical message produces differing wire bytes across independent send paths (e.g. if per-send nonces or framing differ such that the ciphertext is not byte-stable for the same plaintext+epoch). This is the subtle risk: MLS sender-side encryption uses per-message nonces and generation counters, so "the same message sent twice by the sender" is NOT byte-identical, whereas "one sealed blob relayed down two paths" IS. The experiment must test the latter (one seal, two relays), not the former.

**Design consequence:** if the byte-identical property only holds for one-seal-relayed-twice and not for independent re-sends, the design must state that dedup keys are on the relayed sealed blob, and that the author seals once and the blob fans out, never re-seals per path. This is almost certainly the correct model anyway; the experiment makes it explicit and catches any place the implementation might re-seal.

### E2.2 Local-link delivery with no internet

**Claim under test:** D-self local-link delivery works with no internet, two nodes that can see each other exchange MLS over direct QUIC with no gateway (§3.1).

**Method:**

- Two iroh endpoints on an isolated LAN segment (or network namespaces with no route to the internet and no relay configured). Use mDNS / local discovery only.

- Establish a connection and exchange an MLS-sealed message. Confirm delivery and decryption with no relay and no internet reachable.

**Confirms if:** the message is delivered and decrypted with relay disabled and no internet route.

**Falsifies if:** connection requires a reachable relay or any internet-facing discovery even on a local segment.

**Design consequence:** confirmation makes D-self's "most center-free cell" claim concrete. Falsification would mean local-link delivery has a hidden discovery dependency, which must be named (and which discovery modes work airgapped becomes a §2.1 sub-finding).

---

## Tier 3: device-pool sync and RBSR (the cheapness and correctness claims)

### E3.1 RBSR cost scales with the difference, not the history size

**Claim under test:** device sync via RBSR costs logarithmic rounds plus difference-proportional data, not history-proportional (§7.1). This is the load-bearing efficiency claim.

**Method:**

- Use a real RBSR implementation (the Negentropy library, confirmed deployed this session) or implement the Meyer protocol against its spec. Do not hand-wave the algorithm; use or follow a primary implementation.

- Construct two sets representing two devices' message-id histories: identical except for a controlled difference of D items, over a total history of size H.

- Sweep H across several orders of magnitude (1e3, 1e4, 1e5, 1e6) holding D small and fixed. Measure bytes transferred and round count for each H.

- Then hold H fixed and sweep D. Measure the same.

**Confirms if:** transferred bytes grow with D and only logarithmically (or sublinearly) with H; round count grows logarithmically with H.

**Falsifies if:** transferred bytes grow linearly with H for fixed D.

**Design consequence:** confirmation grounds the "cheap personal replay buffer" claim with a measured curve (include the plot/table in the findings). Falsification would mean the chosen construction or backend does not deliver the bound, pointing back to the storage-backend obligation (§7.3): re-run with a range-summarizable backend before concluding.

### E3.2 Clock-free ordering suffices for RBSR partitioning

**Claim under test:** RBSR works with a clock-free monotonic order (a per-device monotonic index plus a deterministic tiebreak), so no wall-clock enters the reconciliation (§7.2, §7.4).

**Method:**

- Define the item key as (monotonic-index, content-hash) with NO timestamp. Populate two device sets using only this key for ordering.

- Run RBSR to convergence. Confirm both devices reach the identical union and that the order is stable and identical on both sides.

- Run a control where two items are deliberately concurrent (same logical position, different authors) and confirm the content-hash tiebreak orders them identically on both devices.

**Confirms if:** convergence is correct and order is identical across devices using only clock-free keys, including for concurrent items.

**Falsifies if:** convergence requires a timestamp, or concurrent items sort differently on the two devices.

**Design consequence:** confirmation validates the §7.4 resolution (one clock-free order, content-hash tiebreak, timestamp as label). Falsification would reopen the ordering question; capture exactly what broke.

### E3.3 Sync moves plaintext only within entitlement, and a non-member backup gets only sealed bytes

**Claim under test:** device-pool sync of a user's own devices moves plaintext between entitled readers; a backup device not in group A receives only sealed bytes it cannot read (§6.2).

**Method:**

- Device 1 and Device 2 are both members of MLS group A. Device 3 is in the user's device pool but NOT a member of group A.

- Device 1 receives and decrypts an A message (plaintext). Simulate sync of Device 1's store to Device 2 (also entitled) and to Device 3 (not entitled).

- Confirm Device 2 holds readable plaintext. Confirm Device 3, if configured as a sealed-byte backup, holds only ciphertext and cannot decrypt (no group-A key); and that if Device 3 is NOT set up as a sealed backup, it receives nothing for group A because Device 1 has no mandate to hand A's plaintext to a non-member device.

**Confirms if:** entitlement is respected exactly: plaintext only to members, ciphertext-or-nothing to the non-member.

**Falsifies if:** the sync path can move group-A plaintext to Device 3 without Device 3 being an A member.

**Design consequence:** confirmation grounds the "entitlement preserved by construction" claim (§6.2). Falsification is a confidentiality bug in the sync design and must block normative text until the sync path enforces the entitlement boundary explicitly.

### E3.4 Lineage-gated admission via application AS policy

**Claim under test:** an Add can be rejected unless the joining leaf proves lineage descent, using MLS application/AS policy, member-side (§6.4).

**Method:**

- Using `mls-rs` or another RFC 9420 implementation, implement a custom credential-validation hook that accepts an Add only if the joining leaf's credential carries a verifiable signature chaining to a designated rooting key.

- Attempt two joins: one device whose credential chains to the root (should be admitted), one that does not (should be rejected by the policy).

- Confirm the rejection happens at credential validation, member-side, without a central gatekeeper, and that all simulated members running the same policy agree on validity.

**Confirms if:** the lineage-bearing join is admitted and the non-lineage join is rejected by policy, consistently across members.

**Falsifies if:** the implementation offers no hook to gate Adds on custom credential logic, or members diverge on validity.

**Design consequence:** confirmation downgrades the §6.4 [confirm] to grounded-and-demonstrated and yields a concrete credential-encoding sketch for Part 2 §5.2. Falsification would mean lineage-gating needs a mechanism outside stock MLS validation; document what.

### E3.5 D-peer reconciles self-verifying records only; tampered or mis-positioned records are rejected

**Claim under test:** member-to-member reconciliation (D-peer, §3.4) is provenance-preserving: the syncing member is a blind relay, and the receiver discerns authorship and position from the record's own author signature and signed monotonic index, trusting the syncer for nothing.

**Method:**

- Two MLS group members, Alice and Bob. Bob holds a set of author-signed records (some authored by Bob, some by others, all signed by their respective authors). Reconcile Bob's set into Alice's via RBSR.

- Inject three adversarial cases from Bob: (a) a record with a tampered body but intact-looking framing; (b) a record re-labeled to a different monotonic position than its signed index; (c) a fabricated record Bob signed himself claiming to be authored by Carol.

- Confirm Alice accepts only the genuine, correctly-positioned, correctly-authored records and rejects all three adversarial cases on signature/index verification, without reference to Bob's word.

**Confirms if:** all three tampered cases are rejected by local verification; genuine records are accepted and placed by their signed index.

**Falsifies if:** any tampered case is accepted, or position is taken from the syncer rather than the signed record.

**Design consequence:** confirmation grounds the §3.4 Case-1 invariant as enforced, not merely asserted. Falsification means the record format does not carry enough self-verifying provenance (signature and signed position), which must be fixed in Part 2 §5/§7.3.1 before D-peer is safe.

### E3.6 Replay during D-peer is deduped or rejected by MLS context, never accepted as new

**Claim under test:** a member replaying an old record cannot inject it as a new or differently-positioned message; replay is bounded by content-hash dedup and MLS epoch/generation context (§3.4 residual-risk envelope).

**Method:**

- Alice already holds record R at its position. Bob offers R again during reconciliation, and separately offers R re-framed to claim a new position/epoch.

- Confirm the straight replay is deduped (same content hash, one stored copy) and the re-framed replay fails validation against Alice's epoch/generation state.

**Confirms if:** straight replay dedups to one copy; re-framed replay is rejected.

**Falsifies if:** a replayed record is accepted as a second distinct message, or a re-framed replay validates.

**Design consequence:** confirmation closes the replay item in the §3.4 risk envelope. Falsification points to missing anti-replay context in the record/epoch binding; specify it in Part 2.

### E3.7 Multi-party corroboration defeats single-member withholding, and holding-count never affects validity

**Claim under test:** multi-party assertion corroboration (§3.4) raises coverage and defeats single-member withholding, while a record's validity remains individual and holding-count-independent (the knife: alignment, never truth).

**Method:**

- Four members hold overlapping sets. Designate one member (Mallory) as a withholder who omits record 65 during reconciliation. Confirm Alice still obtains 65 by reconciling with the other members (coverage restored by corroboration).

- Critically, run the validity control: take a record that only *one* member holds and a record that *all four* hold, and confirm both have identical standing in Alice's store (accepted iff self-verifying), i.e. holding-count changed Alice's *coverage* of 65 but never the *validity* of any record.

- Add a governance-gate sub-case: present a *removed* member as a sync partner. Confirm Alice refuses to open dataplane reconciliation with the removed member (forward-entitlement gate), while records that member legitimately *authored* before removal, obtained from other members, remain valid.

- Add an exit-finality sub-case: a *former* member (removed at epoch N) requests history from epochs they were part of. Confirm the request is refused outright, the eligible convergence range is bounded to *current* members only, with no per-requester historical-range authorization path existing at all. Then confirm the complementary case: a *current* member continuously present from an early epoch can converge the full range they have continuous standing for (epoch boundaries gate record validity, not their eligibility as a current requester).

**Confirms if:** withholding by one member is covered by others; a record held by one member has the same validity as one held by all; the removed member is refused as a partner but its pre-removal authored records stay valid via other sources; a former member's history-pull request is refused outright; a continuously-present current member converges their full eligible range.

**Falsifies if:** withholding cannot be covered (single member is a sole dependency); or a record's acceptance changes with how many members hold it (consensus-as-authority crept in); or the gate either admits a removed member as a partner or wrongly invalidates that member's pre-removal records; or any path serves a former member history after departure (the dissolved attack surface has reopened).

**Design consequence:** this is the experiment that proves the safe/unsafe boundary of the whole D-peer corroboration idea. Confirmation validates §3.4 in full. A holding-count-affects-validity falsification is the alarm that consensus has leaked into authority; the §2.5-forbidden quorum apex; fix before any normative text.

---

## Tier 4: the mobile/push reality (constrained, honest about limits)

### E4.1 Content-free wake then fetch, and the silent-push throttle

**Claim under test:** a wake-then-fetch design degrades correctly because silent/background push is throttled and unreliable, while the durable fetch path (meer drain) is unaffected (§2.4, §5.1).

**Honest scope limit:** fully reproducing APNs/FCM throttling requires real Apple/Google credentials, real devices, and Apple's dynamic, undisclosed throttle. This cannot be cleanly unit-tested in CI. Treat this tier as an integration harness plus a documented manual procedure, not an automated pass/fail.

**Method (two parts):**

- **Automated part (the part that matters for the architecture):** build the wake-then-fetch path with the push as an abstract "wake signal" trait. Simulate the push host as a byte-free trigger that emits only "EndpointId X, wake." Confirm that on wake, the device drains its (G, D) cursor from a meer and recovers all waiting messages, and that with the wake signal entirely suppressed, the device still recovers everything on its next foreground poll. This proves the degradation path without needing Apple.

- **Manual/integration part (documented, run when device credentials exist):** send a content-free background push (`content-available: 1`, no alert) to a real backgrounded iOS device at increasing rates; record how many wake the app vs are dropped. Compare against Apple's "a few per hour, dynamic" guidance. Record numbers but treat them as observations of a dynamic system, not a fixed limit.

**Confirms if:** the automated path recovers all messages with the wake suppressed (proving push is a pure optimization), and the manual part shows silent pushes are indeed throttled/dropped at higher rates.

**Falsifies if:** the device cannot recover messages without the push (push is structural, not an optimization), which would violate the removability requirement.

**Design consequence:** the automated result is the load-bearing one: it proves P-push is removable and the system degrades to poll-a-meer. Confirmation validates §5.1 and §9.2. Falsification would mean the fetch path is not self-sufficient and must be fixed before push can be called optional.

### E4.2 Payload cannot carry meaningful content (size and E2E)

**Claim under test:** push payloads are too small and not E2E, so the push must be a wake signal, not a content carrier (§2.4).

**Method:** this is largely a spec check, already grounded (APNs 4 KB, FCM 4096 bytes and not E2E). The experiment is a guard test: assert in code that the wake payload the design emits is content-free (carries no ciphertext, only a wake token and at most an opaque message-count or topic hint), and add a test that fails if any sealed message bytes are ever placed in a push payload.

**Confirms if:** the guard holds: no path puts content in a push.

**Falsifies if:** any code path attempts to ship ciphertext in the push (would be both a size risk and an unnecessary metadata exposure).

**Design consequence:** this is a regression guard for the byte-free invariant, keep it in the test suite permanently.

---

## Tier 5: end-to-end integration (the adaptive selector and parity)

### E5.1 The selector races sources and first-delivery wins

**Claim under test:** the adaptive selector can run D-self, D-meer, and D-swarm concurrently, with first delivery winning and duplicates dropped, degrading gracefully as paths fail (§3.4, §8). (Note: D-peer was later added as a fourth source; it races identically as one more self-verifying channel, so the exactly-once property tested here extends to it unchanged.)

**Method:**

- Build a harness with all three durability sources as pluggable channels and a selector that subscribes to all available ones. Deliver a message and confirm exactly-once delivery to the application despite arrival on multiple paths.

- Kill paths one at a time (drop the swarm, then the meer) and confirm delivery still succeeds via the survivor, down to D-self only, with no loss and no duplicate.

**Confirms if:** exactly-once delivery holds under all path-survival combinations down to the D-self floor.

**Falsifies if:** any combination drops the message or delivers a duplicate to the application.

**Design consequence:** confirms the §8 selector and the §3.1 "no-helper floor is real" claim end to end. This is the experiment that most directly "shows the thinking" as a working system.

### E5.2 Backgrounded-phone parity (the named design stance, §9.1)

**Claim under test:** under the default deployment (D-meer + P-push over a D-self floor), a backgrounded device's observed behavior matches a normal messaging app: messages are waiting and delivered on wake, with no user-visible difference.

**Method:** simulate a backgrounded device (no live connection) with a meer holding its messages and a wake trigger. On wake, measure time-to-first-message and completeness of catch-up. Compare against a baseline "always-connected" client for the same message stream.

**Confirms if:** the backgrounded device, on wake, presents the same complete, ordered history a connected client has, within a comparable latency envelope.

**Falsifies if:** the backgrounded path loses messages, mis-orders them, or has a latency profile that would read as broken to a user.

**Design consequence:** this is the testable consequence of the parity stance. Confirmation lets §9.1 stand as a demonstrated property, not an aspiration.

---

## What these experiments deliberately do NOT test

Stated honestly, so the findings doc does not overclaim:

- **The rights/principle claims** (peer-equality, fork-not-verdict, field-integrity) are design rationale derived from Part 1, not empirical propositions. No experiment "proves" them; they are validated by argument and by the mechanisms honoring them, which the experiments above do check (e.g. removability, entitlement boundaries, no-helper floor).

- **Real-world APNs/FCM throttle numbers** are a moving, undisclosed target (E4.1 manual part); we observe, we do not pin.

- **Adversarial / Byzantine device-group behavior** (the seized-device threat) is a threat-model judgment; E3.3 and E3.4 check the entitlement and admission mechanisms that bound it, but the threat model itself is decided by the deploying group, not by a test.

---

## Suggested execution order and output

1. Tier 0 (minutes): pin reality, catch version drift.

2. E1.1 first among Tier 1: it can invalidate P-gossip and reshape standalone-D-swarm, so its result gates how much of §3.3/§4 needs rewriting.

3. E1.2, then Tier 2, Tier 3 (the substantive correctness and cheapness work).

4. Tier 4 automated parts; defer the manual push parts until device credentials exist.

5. Tier 5 last: the integration experiments that show the whole thing working.

For each experiment, Claude Code should emit: the exact crate versions resolved, the code, the raw observations, and a one-line CONFIRMED / FALSIFIED / INCONCLUSIVE verdict with the measured evidence. Feed every verdict back into the §11 residue list of the design doc, promoting confirmed items out of [confirm] and rewriting any branch a falsification reshapes (E1.1 most likely).
