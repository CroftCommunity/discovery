# Test narrative — why each test, what it tells us, what it means, what's still open

date: 2026-06-16

purpose: the *reasoning* layer over the proofs. `proof-ledger.md` says what passed; this says
**why we ran it, what it tells us, what we believe the outcome means, and the edge cases /
design questions it surfaces** — so the open testing and design surface stays visible and we
don't mistake "green" for "done." One entry per test (this session's batch); earlier phases'
narrative lives in their findings docs (cross-referenced, not duplicated).

Convention per entry: **Why** · **Tells us** · **Means** · **Open edges**.

Earlier-phase narrative (not repeated here): E1.1–E1.4 → `Proofs/lineage-groups/PHASE_1_FINDINGS.md`;
E2.1–E2.8 → `PHASE_2_FINDINGS.md`; A2.* → `PHASE_2_5/2_6_FINDINGS.md`; cross-machine A/B → `experiments/iroh/TEST-LOG.md` + `PART_A_RECONCILE_FINDINGS.md` / `LOCAL_FIRST_HISTORY_FINDINGS.md`; V1–V9 → `Proofs/lineage-group-model/SOCIAL_LAYER_FINDINGS.md`; relay E0–E3 → `experiments/iroh/relay-lab-runs/`.

---

## T1 — lineage-proving credential rides on the real openmls leaf

- **Why.** The whole multi-device model assumes a leaf can carry a verifiable lineage claim
  (so devices fold and thresholds count by lineage). The TS model *assumed* the leaf↔lineage
  mapping; if real openmls couldn't carry it, the fold and lineage-thresholds would need a
  different home (a design change). It was the top open real-library dependency (#2).
- **Tells us.** openmls 0.8.1 carries a signed `LineageClaim` on the `BasicCredential` identity;
  a second member reads it off the leaf and verifies it from signed data alone; a forged claim
  (named lineage not signed by its root) is rejected. The spike found openmls *also* accepts a
  custom `CredentialType::Other` at founding — no wall there.
- **Means.** The fold (E2.9) and lineage-counted thresholds (E2.10) rest on real library
  behaviour, not a model assumption. "Keys are not identity; the lineage credential travels with
  the leaf" is real. Structured-BasicCredential was a *choice*, not a forced fallback.
- **Open edges.** (1) The probe founded a group with a custom credential type but did not verify a
  *second member's* client accepts a custom-type leaf through add/commit **validation** — deferred,
  not needed for the chosen path. (2) The lineage signature is checked at our governance layer, not
  by MLS (MLS treats identity bytes as opaque) — fine, but means a non-Croft MLS client wouldn't
  enforce it. (3) Cross-machine replay of claim verification is a small follow-on (mirrors A1b).

## E2.9 / C4 — devices of one lineage fold to one actor (no double-count)

- **Why.** A member list and any threshold must treat "Chase's 3 devices" as one actor, or the UI
  lies and quorums can be gamed. C4 is the partition variant: the same person comes online from two
  devices that both authored while split.
- **Tells us.** Folding by lineage id yields one actor per lineage from the leaf credentials every
  client holds; two devices of one lineage collapse, a third lineage stays separate.
- **Means.** The member-list fold is computable identically by every client (no server, no
  divergence), which is the prerequisite for both legible membership and E2.10.
- **Open edges.** The fold is proven in-process; **over live transport it's only shown implicitly**
  (T11 carries distinct branches but doesn't render a folded member list). A "fold renders
  identically on N real clients" test is still open. Also: fold behaviour when a device's lineage
  credential is *present but unverifiable* (revoked root key?) is unspecified.

## E2.10 — thresholds count lineages, not leaves

- **Why.** The single "bites silently if wrong" risk: if thresholds counted device keys, a person
  could manufacture a quorum from their own devices and boot/admit unilaterally.
- **Tells us.** `meets_threshold_by_lineage` counts distinct lineages among valid admin signers;
  one person's two devices count once (below a threshold of 2); two distinct lineages meet it. The
  old by-DID count is demonstrably unsafe (the test asserts it would wrongly pass).
- **Means.** Self-owned-device quorum manufacture is structurally blocked. This is the security
  payoff of "keys are not identity."
- **Open edges.** Relies on a correct leaf→lineage map (T1). **Adversary who controls one device of
  *several distinct* victims' lineages** (compromise-spread) is a different threat — not modeled.
  And the lineage map is per-evaluator; a peer with a *stale* map (missed a device-add) could
  miscount — interaction with stale/partition state is untested.

## E2.11 — revoking one device leaves the lineage's other devices intact

- **Why.** Device loss/theft must be handleable without nuking the person's whole identity; revoke
  one leaf, keep the rest.
- **Tells us.** Removing one device leaf rotates the MLS epoch and removes only that leaf; the
  lineage's other device remains a member.
- **Means.** Device revocation is a normal membership op; "lose my phone" ≠ "lose my account."
- **Open edges.** PCS for the revoked device (can't decrypt *new* traffic) is E1.1; **but
  already-synced history on the revoked device is not clawed back** (no protocol can) — this is a UX
  honesty requirement, untested as such. Revoking the *last* device of a lineage = the recovery
  problem (T12, open).

## E2.12 — self-sync IS backfill (data-model) + T11 (live transport)

- **Why.** If syncing your own devices needs special code (or a server), that's the SSB/Signal trap.
  The claim is it's the *same* operation as catching up any forked branch.
- **Tells us.** Two devices of one lineage reconcile via the existing `backfill_import` path — no
  server, branches stay distinct/navigable, a foreign-lineage branch is refused. T11 then ran this
  3-way over **live iroh** including a NAT'd device.
- **Means.** "Self-sync = backfill" is real, and it's the same mechanism for multi-device and group
  — one subsystem, not two. The thesis's unification holds on the wire.
- **Open edges.** **No conflict in self-sync was tested** — two devices that edit the *same logical
  thing* still just produce two branches (no merge), which is correct but the UX of presenting "your
  own divergence" is unexplored. Large-history backfill cost (compaction/roll-ups, group G) over
  real transport is untested. Ordering of interleaved device + group ops under partition is open.

## E2.13 — leave-one vs leave-all-under-lineage are distinct ops

- **Why.** "Drop this device" and "I (the whole person) am leaving this group" are different acts and
  must not be confusable.
- **Tells us.** `devices_of_lineage` cleanly separates the single-device set from the whole-lineage
  set; leave-all subsumes leave-one for that lineage; other members untouched.
- **Means.** Both intents are expressible and structurally distinct.
- **Open edges.** This proves the *set computation*, not the *op enactment ordering* — e.g. a
  leave-all racing a concurrent add-a-device on another of the person's devices (does the new device
  get caught by leave-all?) is untested (corpus C-shaped). Tie to C7/dissolve semantics.

## E2.14 — same-lineage 1-sig vs cross-lineage full threshold

- **Why.** Your devices should self-organize cheaply (your laptop authorizes your new phone) while
  someone *else* acting on your device must pay the full social cost.
- **Tells us.** A device op authored entirely by the subject's own lineage needs one signature;
  any outside signer forces the full (lineage-counted) threshold; a lone outsider can't.
- **Means.** The asymmetry that makes multi-device usable without weakening cross-person governance
  is real.
- **Open edges.** **Mixed-lineage authorship** (one own-device + one outsider) falls to full
  threshold — correct, but the *boundary* cases (is one own-signature + one outsider "cross"? yes
  here) deserve explicit enumeration. And same-lineage-1-sig assumes the lineage isn't compromised;
  a stolen device of your lineage could remove your other devices with 1 sig — a real attack surface
  (mitigation: the recovery/anchor design, T12).

## E2.15 — self-removal ordering

- **Why.** A device must be able to author its own departure *while it still has standing*; after,
  it must be inert.
- **Tells us.** A leaf authors its own Leave while a member (enacts), then a later op it signs is
  rejected as a departed admin.
- **Means.** "The last valid act of a leaf can be to drop its own standing" holds; no
  author-after-departure.
- **Open edges.** Ordering under *concurrency* (the device leaves on one branch while still acting on
  a partitioned branch) is the partition case — covered structurally by per-epoch authority (A2.4)
  but not specifically tested for self-removal.

## AR-1 — Sybil / fresh-lineage threshold resistance

- **Why.** Distinct from E2.10: instead of own devices, an attacker mints many *fresh* identities.
- **Tells us.** Fresh (non-admin) identities have no standing — rejected outright; no admit-authority
  accrues to fresh lineages regardless of headcount.
- **Means.** In a permissioned group, Sybils can't self-admit or vote; standing is gated by the
  immutable genesis admin set + authorized adds.
- **Open edges.** **Open/public-regime join flows** (social-layer, where joining is the point) are a
  different model — Sybil resistance there is *unproven* and is the harder case (ties to S3/S4 and
  the social-layer threat model). This test only covers the permissioned group.

## AR-2 — malicious blind sequencer (reorder / drop / inject)

- **Why.** The "dirty secret" worry: is the superpeer secretly an ordering authority that can change
  outcomes? Active-attack complement to A3 (capability-not-a-right) and A2.1 (order-independence).
- **Tells us.** A blind broker that reorders/duplicates can't change the converged state (200 fuzzed
  seeds); a dropped op leaves a *visibly-behind* head (a real earlier head, not a false "current");
  an injected/forged op is rejected (no keys → no manufactured membership).
- **Means.** The ordering role is minimal and blind: it can degrade (drop → stale, which is visible)
  but cannot *manufacture* an outcome or silently stall. The honest "there is an ordering dependency,
  but it holds no rights" claim is substantiated.
- **Open edges.** **Selective delivery to manufacture a partition** (give peers disjoint op sets) is
  not explicitly tested here — it reduces to the partition+reconcile case (A1/E2.4, contradiction
  hard-stops), but a dedicated "broker engineers a fork, peers detect it" test is worth adding. Also
  untested: a broker that **withholds selectively over a long period** to keep a victim perpetually
  stale without them noticing — the "stale is visible" guarantee needs a *liveness/staleness-alert*
  mechanism (how does a peer know it's behind if it never hears from anyone?). This is a real design
  gap → a heartbeat/freshness signal.

## AR-3 — backfill DoS resistance (bounded rejection cost)

- **Why.** Rejection correctness (forged/unauthorized → no) was already green (backfill_adversarial),
  but a hostile donor could still try resource exhaustion: flood the victim with huge/garbage
  branches so that *rejecting* them is itself expensive (unbounded crypto, accumulating state).
- **Tells us.** `backfill_import` checks `shares_lineage` before touching any message, so a 10k-message
  foreign branch is rejected with **zero** signature verifications (proven by a panic-on-call
  verifier); a forged branch on a shared lineage is rejected at the **first** defect (1 verify call
  for a 5k-message payload); 1000 rejected branches leave `branch_count == 0`.
- **Means.** An attacker cannot convert payload size into victim CPU: cost is bounded by the genesis
  boundary (foreign) or the first defect (shared), not the attacker's volume. The "fail early, fail
  cheap" posture holds against a flood.
- **Open edges.** This is the *application-layer* bound. The **transport/gossip layer** size cap
  (iroh-gossip per-message max) is cited but not independently tested here, and a **cross-host flood
  measurement** on the 3.8G box (node-3 as victim, node-1 as flooder) — actual RSS/CPU under sustained
  flood — is a follow-on. Also untested: **connection/handshake-level** DoS (many endpoints dialing),
  which is the relay-lab's reconnect-storm driver, still open.

## AR-5 — MLS-tree + rekey scaling under per-device-as-member

- **Why.** Making every device a distinct MLS member multiplies leaves (50 people × 3 devices = 150).
  multi-device.md flagged this as a possibly-underestimated cost. We measured the real per-add and
  rekey commit size on openmls 0.8.1 as the tree grows.
- **Tells us.** With `use_ratchet_tree_extension(true)` (our default, so newcomers join without an
  out-of-band tree), commit size grows **~linearly**: 1.4 KB @ 8 leaves → 3.5 @ 32 → 6.1 @ 64 →
  11.4 @ 128 (~1.8× per doubling); a rekey (remove) at ~129 leaves is ~11 KB. It is **not** the
  ~O(log N) MLS path cost — the embedded tree dominates.
- **Means.** Per-device-as-member is **affordable at human group scale** (hundreds of leaves → tens
  of KB per commit), so the multi-device model is fine for interactive/quiet tiers. But the cost is
  O(N), so the **broadcast tier (1000s) must disable the embedded-tree extension** and ship the tree
  out-of-band (via the broker snapshot) to recover O(log N). This is a concrete tier-design rule the
  measurement produced.
- **Open edges.** Measured commit *size*, not *CPU time* per commit at scale, nor memory of holding a
  large tree, nor the rekey storm when many devices churn at once. The O(log N) claim for the
  no-extension mode is asserted by theory, **not yet measured** (a follow-on: re-run with the
  extension off). Cross-host (node-1 fat box) timing under concurrent adds is unrun.

## AR-6 — replay / double-count

- **Why.** Can a signature be counted twice, or an old op replayed to re-enact?
- **Tells us.** Sigs are keyed by DID (one signer can't appear twice → one person signing twice still
  counts once); a replayed op fails `BrokenChain` against the advanced head.
- **Means.** Double-count and replay are structurally prevented, not runtime-guarded.
- **Open edges.** Replay *across a fork/reformation* (an op valid on one branch replayed onto a
  sibling branch) — the genesis/seq/prev anchoring should reject it, but it's not explicitly tested.

## T3 — real threshold-signed compaction checkpoint (F2)

- **Why.** History must be compactable (roll-ups/checkpoints) or it grows unbounded (the SSB trap).
  The security question is *who finalizes a checkpoint* — if the always-on superpeer could sign one
  alone, it would be a de-facto finality/ordering authority (the "dirty secret"). The model proved
  roll-up correctness (F/G); T3 is the real-crypto F2: threshold-signed, not authority-signed.
- **Tells us.** A checkpoint verifies only with signatures from a threshold of distinct admin
  *lineages* (lineage-counted, so own-device padding doesn't help); a broker-only or below-threshold
  checkpoint is rejected; the checkpointed head must match the real log (no tampered summary); and a
  checkpoint is bound to one branch's head, so two forked branches cannot share one.
- **Means.** Compaction is legitimate-by-threshold, and the broker is provably **not** a finality
  authority — it can carry/serve a checkpoint but cannot mint one. This is the concrete answer to
  "is the superpeer secretly the orderer?": for finality, no. Closes ledger dependency #3.
- **Open edges.** This proves the *checkpoint's authority + integrity + branch-binding*. It does
  **not** yet prove the *Automerge-side compaction* (that old change-metadata is actually discarded
  and a newcomer renders correctly from the checkpoint + tail) over real crypto — that stays
  model-proven (F/G). Also untested: a checkpoint over a range that *itself* contained a since-healed
  fork (does the head-binding suffice, or does the checkpoint need to attest the heal?).

## C3 / C7 / C8 / C9 / C10 — reconcile/merge-split corpus

- **Why.** The original detector knew one conflict reason; the corpus (merge-split-corpus.md)
  enumerated more. We ran the unmodeled ones.
- **Tells us.** C3 concurrent-identical-remove **heals** (agreement, no false stop); C7
  dissolve-vs-continue **hard-stops** (new reason, quorum-override can't silently clear it); C8
  diamond recombine preserves standing/shares_lineage over a 2-parent DAG; C9 equivocation is
  detected+attributed; C10 a removed member's new device can't self-confer standing (re-admit needs a
  threshold Add).
- **Means.** The "forks are a feature, conflicts hard-stop, nothing auto-adjudicated" model extends
  cleanly to the wider case space; ban-evasion and dissolve-races are handled.
- **Open edges.** Still unmodeled from the corpus: **multi-reason conflicts** (a reconcile that is
  *both* removed-then-included *and* dissolve-vs-continue — does the override handle them
  independently?); **C8 conflict semantics** (we proved standing over a diamond, not contradiction
  *detection* across a diamond's two histories); **deep fork chains** (fork-of-a-fork-of-a-fork
  reconciliation). These are the next reconcile tests.

## T9 — offline transitive trust via Merkle proofs

- **Why.** Transitive trust ("I trust what my trusted set vouches for") usually reintroduces a
  trusted online party to query. The dossier's claim is you can prove set-membership of a trust
  assertion **offline** with a Merkle proof, against a published root — no authority. Links I3
  (standing from signed data alone) and I8 (backfill verifiability).
- **Tells us.** A domain-separated (RFC-6962-style) Merkle root commits to a set of assertions; a
  compact inclusion proof verifies a specific assertion against the root with a pure, offline
  function; a forged leaf, a doctored path, or the wrong root all fail; the root is deterministic
  and order-sensitive.
- **Means.** The trust-graph primitive holds: a party can publish one root and let anyone verify
  individual facts offline, without becoming a queryable authority — consistent with the
  non-extractive, no-trusted-server thesis.
- **Open edges.** This proves *inclusion* against a known root. It does **not** yet bind the root to
  an *identity* (who published it — needs a signature over the root, trivial but unproven here), nor
  prove **non-membership / revocation** (a sparse Merkle tree or an accumulator), nor the
  *transitive* composition itself (chaining "A trusts B's root which includes C"). Those are the next
  trust-graph steps. Also: the set is order-sensitive, so a canonical leaf ordering must be defined
  before two parties can compute the same root from the same facts.

## T2g/MD-G1 — per-lineage gossip group over the NAT path

- **Why.** The same-VPC capstone never exercised a real NAT'd device in a gossip group; "your phone
  behind home wifi joins your lineage's group" is the real deployment path.
- **Tells us.** A NAT'd Mac joins the per-lineage topic (= sha256(lineage genesis)) bootstrapping by
  NodeAddr+TopicId only (no direct IP), and exchanges bidirectionally with a box via relay.
- **Means.** The Delta-Chat pattern (relay-coordinated, IP-excluded invite) works for Croft's
  per-lineage groups; a real phone can participate.
- **Open edges.** Used the **n0 public relays**; a Croft co-op would run its own relay (the relay-lab
  E-series sizes this). **Direct hole-punch** (vs relay fallback) from the NAT'd Mac is still blocked
  on public 3343/3478 ingress (E0-NAT). Metadata leakage at the relay (AR-4) is uncharacterized.

## AR-4 — transport metadata-leak bound (characterization)

- **Why.** Even with content encrypted, a relay or on-path observer learns *metadata* — who talks to
  whom, when, how much. We need an honest bound on what's exposed, tied to the social-layer S2
  (topology deanonymization) threat model.
- **Tells us.** Content is opaque (encrypted QUIC; the relay never sees branch content). The relay is
  **topic-agnostic** (routes by EndpointId), so it doesn't learn the lineage topic — membership-by-
  topic is an app-layer exposure (V8), not a relay one. The relay *does* see EndpointId pairs,
  timing, volume, and egress IP — classic traffic-analysis metadata.
- **Means.** The blind-broker claim holds for *content* and even for *topic*, but the design does
  **not** hide who-communicates-with-whom at the transport layer (as expected, and consistent with
  E3.4's "IP/timing observable"). A surfaced design requirement: **topic = sha256(lineage id), so
  lineage ids used as topic seeds must be high-entropy/salted** — a guessable handle yields a
  joinable/observable topic.
- **Open edges.** This is a *derived* characterization, not a fresh capture — an actual relay-side
  timing/volume measurement to **quantify** the leak is a follow-on. Mitigations (cover traffic,
  batching) for who-talks-to-whom are unspecified. A co-op relay's logging policy (redact to
  endpoint/timing, never content) must be written (cf. the public-path ingester's PII-in-logs flag).

## MD-G2 — signed branch carried + verified over the topic

- **Why.** MD-G1 carried strings; the real claim is a *history branch* travels and is *verified*
  before it's absorbed.
- **Tells us.** A chain-hashed branch travels the topic; the receiver absorbs it only if it shares
  the lineage genesis, is contiguous, and the hash chain is intact; a tampered branch is rejected —
  bidirectionally over the NAT path.
- **Means.** The structural half of `backfill_import` (E2.12) runs over live transport: integrity +
  ordering + privacy-boundary enforced on receipt, exactly as the in-process proof.
- **Open edges.** **Honesty boundary:** the spike uses a sha-256 hash chain, not the Ed25519
  signature/standing check (that stays green-real in Proofs). A faithful end-to-end would carry the
  real `lineage-history` `Message` (Ed25519) over the wire — deferred to avoid the dalek/cross-repo
  build friction. So "the *exact* proven code ran on the wire" is not yet true; the *equivalent
  structural checks* did.

## MD-G3 — drop-a-device mid-run

- **Why.** A user's devices (or a group) must keep syncing if one node — especially a relaying/
  bootstrap node — disappears. The carrier had only been shown in a stable 3-node mesh (T11).
- **Tells us.** With node-1 + the NAT Mac bootstrapped *only* via node-2, killing node-2 ~14 s in
  left both survivors holding each other's branch — they had formed a transitive link and it held
  through the relayer's death.
- **Means.** The mesh is not dependent on the bootstrap/relaying node's continued presence; drop-a-
  device resilience holds for the multi-device/group carrier, including a NAT'd survivor.
- **Open edges.** Absorptions are monotonic and the log is un-timestamped, so this proves "survives
  the death with state intact," and B-gossip separately proves "*new* delivery continues post-kill"
  — but a single test that shows **new** branch content propagating between survivors *after* the
  kill (timestamped) would tie the two together cleanly. Also untested: killing the node that two
  others depend on *before* they've absorbed each other (kill at t≈2 s, not 14 s).

## MD-G4 — multi-device fold over transport

- **Why.** The thesis claim is "one person's many devices are one actor; the *same* mechanism scopes
  a group." Proofs proved `fold_by_lineage` green-real (E2.9/C4); it had never been shown over the
  wire. Doing so required moving the gossip topic from per-lineage to per-**group** (so multiple
  lineages share a topic) and carrying a distinct `device_did` alongside the actor `lineage_id`.
- **Tells us.** On group `g1` with alice.laptop (node-1) + alice.phone (NAT Mac) + bob.phone
  (node-2), **all three nodes folded identically** to `folded_actors=2` — alice's two device
  branches collapse to one actor, bob is a second — over real iroh including the NAT path. Tampered
  branches were rejected and never entered the fold.
- **Means.** The "device-count ≠ actor-count" invariant holds on the wire: every peer computes the
  same actor view from the branches it receives, which is the prerequisite for a consistent member
  list and lineage-counted thresholds (E2.10) in a live group.
- **Open edges.** Fold is over hash-chained branches, not the real Ed25519-signed
  `lineage-history::Message` (same honesty boundary as MD-G2). Each actor here has ≤2 devices and
  3-msg branches — not a scale test of fold cost. The fold is recomputed from scratch each receive,
  not incrementally; cost under churn is unmeasured.

## MD-G5 — revoked device can't rejoin over transport

- **Why.** Revocation is governance over the wire: once a device is removed, peers must refuse its
  later branches, but **must not** retroactively erase the history it contributed (E2.11's
  standing-≠-membership honesty). Proven green-real in Proofs; never shown over transport.
- **Tells us.** With alice.laptop broadcasting a MAC'd revoke of carol.tablet, the two survivors
  showed the two halves cleanly: the **witness** (Mac/bob) that had already absorbed carol **retained**
  her branch in the fold *and* marked `revoked={carol.tablet}`; the **revoker** that first heard carol
  *after* the revoke **rejected** the branch `(revoked)`, keeping the device out of the accepted set;
  the **target** saw a revoke-of-self and could not re-enter.
- **Means.** Removal is enforceable on the live mesh without a central authority, and it is honest —
  history contributed before removal is not clawed back. A revoked device is barred from the
  *accepted set* going forward, which is the membership half of the standing-vs-membership split.
- **Open edges.** The revoke is integrity-bound by a sha-256 MAC, not a real Ed25519 **authority**
  signature — *who is allowed to revoke whom* stays green-real in Proofs (E2.11), unverified on the
  wire. The retain-vs-refuse split is a consequence of iroh-gossip **de-duping identical payloads**
  (a peer holding carol's byte-identical branch isn't re-delivered the repeats); showing both
  transitions on one node needs per-round-varying branch bytes. Revoke ordering vs. a racing
  legitimate branch (revoke and a fresh carol op broadcast in the same round) is untested.

## FAITHFUL — real signed message + authority verified over the wire

- **Why.** Every transport spike to date carried a sha-256 hash chain: it proved in-transit integrity
  and ordering but not *authorship*. A valid chain claiming `device=alice` is forgeable, and the
  Ed25519 signature + standing/authority checks lived green-real only in `Proofs/`, never on the wire.
  No single artifact showed the real signed message verified for authority end-to-end. This closes it.
- **Tells us.** A standalone spike (`altdrive-spike-faithful-sync`) path-deps the real Proofs crates
  (stable `ed25519-dalek 2.2.0`) alongside iroh rc.1 — they compose despite the pre-release-crypto
  collision that PHASE_3_FINDINGS hit in the other direction — and runs the **real**
  `backfill_import` on messages received over live iroh-gossip. Both joiners (node-2 + the NAT Mac)
  independently reached the same verdict: HONEST member → ACCEPT; FORGED (tampered) → REJECT
  `BadSignature`; **NONMEMBER with a valid signature but no standing → REJECT `UnauthorizedAuthor`**.
- **Means.** The same bytes signed-and-authority-checked in the model are now checked on the wire by
  the same code. The load-bearing result is the NONMEMBER case: a *genuine* signature that *passes*
  verification is still rejected for lack of standing — the precise attack a hash chain admits. The
  "who wrote it / may they?" half of the transport claim is now green-real, not promissory.
- **Open edges.** (1) The verifying-key registry + lineage membership are the agreed group state
  modelled in the spike; **MLS key-distribution itself is not run over the wire** (green-real in
  Proofs). (2) **Threshold revocation authority** (who-may-revoke; the k-of-n dial) is the next layer
  and is not in this spike — `thinking/revocation-authority.md`. (3) node-1-as-origin showed empty
  verdicts (gossip timing: exited before peer broadcasts meshed back; own broadcasts not echoed) — the
  two joiners carry the verdict. (4) **What leaks when an op is rejected** (the failed-op observable —
  leak vs immune-signal vs silent/blackhole) is its own spike — `thinking/failed-op-response.md`.

## T11 — 3-way local-first history over live iroh

- **Why.** Promote the file-relayed I7/I8/I9 result to live transport, 3-way, and show "same
  mechanism for multi-device AND group" on the wire.
- **Tells us.** Three nodes (incl. the NAT'd Mac) each absorbed the other two's branches as distinct,
  verified, navigable branches and rejected every tampered one.
- **Means.** The local-first, branch-not-interleave history model works over real iroh at 3 parties;
  the same carrier serves a person's devices and a group.
- **Open edges.** **No mid-run node kill** (explicit drop-a-device resilience, MD-G3) — B-gossip
  proved drop-a-node among boxes, not yet in this carrier with the Mac. **No compaction/roll-up**
  over the wire (long-history cost, group G). Same Ed25519-vs-hash-chain honesty boundary as MD-G2.
  **Convergence timing/scale** (3 nodes, 3-msg branches, ~30s) is a demo, not a load test.

---

## S2 — scoped visibility, not opaque structure

- **Why.** The social layer's most seductive feature is "show the useful structure but hide the
  names" — let people see the shape of a community without exposing who is in it. S2 is the claim
  that this is *unsafe by construction*: graph topology deanonymizes. We needed a proof that the
  protocol must *refuse* to offer structure-with-hidden-identities, not just warn against it.
- **Tells us.** Two green-model results (`lineage-group-model` S2a/S2b). S2a models the canonical
  attack — a town of 4,000 where only one person's connection shape touches both the Henderson family
  group and the Oak-St school-parents group — and the **anonymity set collapses to 1**: withholding
  every name does not help, the shape alone re-identifies. So the structure-only share is made
  *unrepresentable* (the constructor throws, the V1-style idiom). S2b shows the only constructible
  share is a consented-distance map carrying no topology: a viewer at distance *d* gets exactly the
  content consented for *d*, and over-reach beyond the horizon returns silence, not anonymized shape.
- **Means.** An entire class of "anonymized social graph" features is ruled out at the type level —
  you cannot build the invasive version even if you want to. Distance gates *consented content*,
  never the graph. This is the honest form of the Kevin-Bacon-distance idea and the highest-value
  social-layer result so far.
- **Open edges.** The re-identification is modelled with a synthetic fingerprint (degree + anchor
  groups); a real deployment needs a calibrated anonymity-set estimator over actual graph features.
  **S3 (quiet membership) and S4 (multi-identity)** are NOT done — they need design gate G5
  (`thinking/social-layer.md` §75–77) and a user decision before any test; S3 is where the
  inside-adversary problem lives. The consent map's *distance metric* itself (who is at distance d,
  and who decides) is unmodelled — distance is taken as given here.

## E2.16 — tier-degradation visibility (the freshness signal, tested)

- **Why.** The freshness design (`thinking/freshness-signal.md`) made a claim; E2.16 tests it. The
  prior gap: without a superpeer a peer might render a confident, current-looking view that is in
  fact stale, with no way to know. The protocol owes three things — keep working, tell the truth
  about staleness, and degrade per-tier without ever lying.
- **Tells us.** Green-model on `core/freshness.ts` (E2.16a/b/c). (a) **Availability:** a peer alone,
  hearing no one, still applies its own ops locally (seq advances) — forward progress never blocks on
  connectivity. (b) **No-false-current:** "current" requires *both* caught-up *and* heard-within-
  horizon; silence yields "unverified", a later-seen tip yields "behind", and — the trap — a peer
  that advances its *own* head while hearing no one stays "unverified", never falsely current. (c)
  **Visible tier degradation:** the same 1h silence flips interactive (15s horizon) to unverified
  promptly, stays "current" for quiet-large (6h horizon, within its eventual contract), and is judged
  by position for broadcast (no clock); and no tier ever shows "current" while behind and a week stale.
- **Means.** The multi-device tier is now complete *including tier visibility*: behind-ness is a
  first-class surfaced state, not merely structurally present. Availability and honesty are
  decoupled — you keep working offline AND you are told you cannot prove currency. This is the
  liveness complement to the comparative "stale is visible" results (AR-2, multi-device).
- **Open edges.** Modelled in the sim (monotonic time, modelled beacon — signature/relay elided; the
  faithful-path crypto + AR-4 metadata bound are where the real beacon's authenticity/leak live).
  The **fresh-but-wrong partition** (a clique keeps each other "fresh" while collectively behind the
  true tip) is out of scope here — freshness proves liveness, not global currency; the reconcile
  hard-stop on reconnect is what catches that. Horizon constants (15s / 6h) are placeholders pending
  calibration on the real fabric.

## E10 — RoQ media over iroh datagrams: the congestion-control unknown, resolved

- **Why.** Real-time voice/video is the one Discord capability the messaging spine doesn't touch, and
  the whole "keep WebRTC's media engine, throw away its transport" plan rested on a single unproven
  cell: does iroh's QUIC congestion control *fight* a media bitrate estimator on the datagram path
  (C1)? Everything else in `realtime-media-over-iroh.md` was "proven shape + known integration"; this
  was the genuine technical risk. Cheapest possible attack: a synthetic CBR source (no codec, no mic)
  over the exact `conn.send_datagram` primitive callme ships on, through the same netem rig E6 used.
- **Tells us.** Two clean regimes. Under **loss and delay** the datagram path is *transparent*: netem
  loss reaches the app verbatim as sequence gaps (5 %→4.6 %, 30 %→30.9 %), path RTT stays flat (~2 ms)
  and tracks added delay exactly (100→102 ms), jitter stays sub-millisecond, and `send_datagram` never
  errors or falls back to a reliable stream — so a media estimator gets accurate loss + RTT + jitter,
  and audio holds to 30 % loss with *visible* (31 % concealment), never silent, degradation. Under a
  **source-over-cap** (40 kbit cap below the ~84 kbit wire rate) iroh **queues and paces datagrams in
  order** instead of dropping to fit: RTT balloons 537→8829 ms and the receiver gets a contiguous,
  ever-delayed prefix (208/1000 frames, *zero gaps*, ~11 kbps = link rate). `send_datagram` returns
  `Ok` throughout — when its buffer fills it drops oldest silently, so even an 8 KiB send buffer never
  errors and RTT inflates identically (the bottleneck queue is the link, faithfully reported by iroh's
  RTT estimate).
- **Means.** C1 is resolved: the controllers do **not** fight destructively, but iroh will **not**
  rate-adapt media for you. The media engine's bitrate estimator must be *authoritative* and must back
  off on the **path-RTT trend** (plus per-stream sequence loss and arrival jitter) — exactly the
  proposed C1 solution, now evidenced, with all three required signals confirmed exposed and accurate.
  The transport is a "dumb measured channel," which is precisely what the fold wants. The audio MVP
  (L1 / callme line) is de-risked on our real fabric; the path to L2/L3 (str0m) is unchanged.
  **Follow-up (E10c / TC-CC2): the estimator is now demonstrated, not just required.** A delay-based
  AIMD controller on iroh's path-RTT, run on a mid-call bandwidth drop, backs off 64→8 kbps in under a
  second and bounds RTT to ~1 s with a continuous lossless stream, where a fixed-bitrate sender
  bufferbloats unbounded past 7 s. The RTT signal is actionable. (It rescues the *degradation* case,
  not the *join-an-already-saturated-link* case; residual RTT ~1 s, not ~50 ms — a real engine would
  pace down faster / flush.)
- **Open edges.** (1) Synthetic CBR, not real Opus through a jitter buffer — the transport CC question
  is answered, but a true mouth-to-ear quality run still wants a real codec on at least the Mac leg.
  (2) Direct same-VPC path only; the mesh-vs-meer-relayed comparison and a raw-UDP side-baseline (to
  attribute the bufferbloat purely to the link vs any iroh contribution) remain follow-ons, and the
  mesh/direct-media case stays gated on E0-NAT hole-punch ingress. (3) Default congestion controller
  was used; whether a different controller (NewReno vs the default) changes the over-cap pacing is
  untested. (4) TC-CC3 (media + bulk transfer contending on one connection — datagram/stream
  isolation) is not yet run.

## E12 — blind media-meer: SFrame-over-MLS (media E2EE through a forwarder that can't read it)

- **Why.** The media end-state (`realtime-media-over-iroh.md`) rests on the same blind-broker guarantee
  the messaging spine makes: a forwarding meer (the SFU) must carry voice/video without being able to
  read it, and a revoked member's media must stop decrypting — the media analog of the faithful path's
  standing check and MD-G5. C3 was the keying unknown: can SFrame keyed off a real MLS group deliver
  per-sender keys, loss-tolerance, revocation, and a blind forwarder all at once?
- **Tells us.** Yes — all four TC-KEY cases pass against a **real openmls 0.8.1 group** (`lineage-mls`),
  not a fixture. The per-sender SFrame base key is HKDF over the group's genuine MLS **exporter secret**
  bound to `(epoch, leaf)`. (1) Two senders get distinct keys; a non-member has no group secret and so
  cannot derive any key — the media `UnauthorizedAuthor`, from keyed state alone. (2) Under 10 % loss +
  intra-window reorder, every surviving frame decrypts **out of order** (90/90) and a replayed stream is
  rejected (90/90) by a sliding per-sender counter window — loss-tolerant, with **no contiguity
  requirement** (the key difference from the message hash-chain). (3) Removing a member advances the MLS
  epoch and **rotates the exporter secret**: the removed sender is stuck at the old epoch, her later
  frames are rejected (stale epoch + non-member = media **MD-G5**), she can't forge a new-epoch frame,
  yet a receiver's **pre-revocation frames still decrypt** (history not clawed back) and the group keeps
  working. (4) The blind SFU routes/selects all 92 frames from the clear `(epoch, leaf, counter)` headers
  and recovers **zero** plaintext.
- **Means.** "Media keying = message keying + loss-tolerance" is **real**, not a hand-wave. The DAVE-shape
  blind SFU is keyed correctly off our already-proven MLS machinery; media revocation IS membership
  revocation (the MD-G5 mechanism, re-used); and the blind-broker guarantee (E3.4 / AR-4) extends to
  continuous media. With E10 (the datagram transport/CC) and E12 (the keying) both answered, the two
  genuine media unknowns the design named are now evidence, not hope — the remaining media work is
  integration (a real codec, the RFC 9605 SFrame wire header, carrying this over the E10 datagram rig)
  and the gated items (str0m video maturity; NAT hole-punch for direct/mesh media; the meer binary).
- **Open edges.** (1) Synthetic frames (string payloads), not real Opus through RTP packetization — the
  keying is proven; codec/packetization is separate. (2) The SFrame header is modeled `(epoch, leaf,
  counter)`, not the RFC 9605 wire format. (3) Run locally as pure crypto — a transport-carried version
  is exactly this keying over the E10 datagram rig, a small follow-on. (4) MLS key-distribution is real
  here (openmls add/welcome/remove); the separate "key registry over the wire" honesty boundary
  (Workstream C) concerns the faithful *messaging* crate, not this media path.

## E11 — MoQ broadcast: lazy fan-out, blind relay, and the abuse lever

- **Why.** Broadcast media (stage / watch-party / livestream) is the mass-distribution surface — the one
  that got Rave pulled (`abuse-resistance-and-the-rave-trap.md`). The design's claim is that a MoQ relay
  can be **blind** (forwards Tracks it needn't decode) and **lazy** (encodes/sends nothing until a
  subscriber asks), and — crucially — that the only honest lever against piracy/abuse on a blind relay is
  **scale + peer restriction enforced from metadata alone**, never content inspection. That last claim is
  the Croft-specific answer to "how do you moderate a thing you can't read?"
- **Tells us.** All four hold (relay logic, deterministic): (1) a publisher emits **zero** frames across
  100 produce-ticks while nobody is watching, and produces the instant a subscriber attaches — the media
  instance of the interaction-tiers "nothing to fan out if nobody is watching" philosophy. (2) Fan-out
  cost is exactly linear in audience size (0→0, 1→10, 3→30, 10→100 frames). (3) The relay forwards opaque
  frames holding **no payload key**; every subscriber decrypts locally. (4) A blind relay enforces a
  **max-audience cap** and **members-only** from subscribe metadata alone — it refused 3 of 8 over-cap
  joins and 1 non-member — **reading zero frame bytes**.
- **Means.** The abuse lever for broadcast is real and honest: a co-op-run blind relay can refuse to
  *serve at scale* or *serve non-members* without ever inspecting content — scale + peer restriction, not
  surveillance. Combined with E12 (the conversational SFU keying) the meer's three blind roles (message
  broker, RoQ SFU, MoQ broadcast relay) are all evidenced in their Croft-novel parts.
- **Open edges.** Relay logic only — not real moq-rs Tracks, GStreamer/Opus encode, or WebTransport
  browser reach (the design calls those "mostly assembly, not build", de-risked by n0's iroh-live). The
  transport-carried form is meer role P5 over the iroh fabric E9/E10 proved. The full AR-4-for-media
  metadata bound (TC-META1) and CBR-padding to defeat per-stream bitrate inference (TC-META2) are
  separate measurements not run here.

## meer P0+P1 — the always-on blind superpeer, made real

- **Why.** Every "Discord-feel" feature that assumes an always-present server — history when you rejoin,
  a channel that stays converged, a livestream that exists when you open it — needs an always-on
  participant. The relay gives connectivity; the meer gives *presence-of-state*. The whole project's
  differentiator over Matrix self-hosting (the homeserver sees metadata + plaintext) and Discord
  (central, can un-blind) is a superpeer that is **provably blind** and a **revocable delegation**, not
  infrastructure that accrues power. E3.4 modeled the blind broker; the question was whether it runs.
- **Tells us.** It runs, on the live fabric. A member publishes ChaCha20-Poly1305-encrypted blobs the
  meer stores keyed by `sha256(ciphertext)`; an offline/behind member syncs through the meer and decrypts
  all of them **locally** with a key the meer never held — converged 5/5. The meer's own stats report
  `meer_payload_keys_held=0` and show the only thing it learned is the §3b metadata (digests, lengths,
  timestamps, the namespace label) — the AR-4 surface, made explicit. The admission gate denies a
  non-listed peer at connect (`not admitted, code 1`). And the anti-entrenchment guard is real, not just
  asserted: the meer's **encrypted** store exports, imports into a *replacement* meer, and the member
  re-homes and converges identically — losing a meer costs availability, never data.
- **Means.** The headline P1 milestone is reached: "a Tier-0 meer that provably holds no key is 'run your
  own infrastructure' with a guarantee neither competitor offers" is a running binary, and the
  "materially reversible delegation" principle (state portability → stand up a replacement) is
  demonstrated, not promised. Every later meer role (bridge, Tier-1, the media SFU/MoQ roles) builds on
  this foundation.
- **Open edges.** (1) Spike harness, not the production-TDD meer crate (Workstream B) — same protocol,
  to be re-built under TDD + Zeroize + no-prod-unwrap. (2) Range reconciliation is a have-set diff, not
  the richer Willow-style range reconciliation on (timestamp, digest, path) metadata (P3). (3) The member
  key is a lab fixture; in product it is the MLS-derived group key (E12) — the point proven is only that
  the meer LACKS it. (4) Rogue-meer detection over time (a meer that quietly logs more, or upgrades its
  tier) stays a design open edge. (5) P2 (bridge/cross-namespace = E8), P3 (Tier-1 + no-mirror +
  reliability-vs-overlap), P4/P5 (media roles), P6 (Tier-2) remain.

## Conformance suite v0.1.0 — the protocol made checkable by a second implementation

- **Why.** "A protocol is only real when a *second*, independent implementation can interoperate." Until
  now the behavior lived in our Rust/TS spikes; there was no black-box suite an alternate Croft impl must
  pass. Without it, "Croft the protocol" and "Croft the spike" are indistinguishable.
- **Tells us.** The green-real/green-model contract is now executable and **derived from the real code,
  never hand-typed**: an emitter runs the actual `lineage-core`/`lineage-history` API to produce
  language-neutral JSON vectors, and a runner re-feeds them and diffs — 34 pass / 0 fail across genesis/
  topic derivations, signed pre-images (with a one-bit-flip that MUST reject), fold + lineage-counted
  thresholds (incl. the one-lineage-3-device quorum that MUST reject), revocation mechanics, the C1–C10
  reconcile corpus, and manifest integrity. The must-reject cases pass *because the real API rejects
  them*, and corrupting a good vector flips the runner to FAIL — the suite has teeth. The honesty
  boundaries are respected: revoke-**authority** threshold is a declared `PLACEHOLDER` blocked on
  Workstream C, not faked, and the AR / visibility / freshness categories are recorded as not-yet-emitted.
- **Means.** Croft now has the beginning of a real interop contract, not just a TEST-PLAN. A second
  implementation has concrete input→expected-output pairs and must-reject teeth to satisfy.
- **Open edges. ⚠️ A real spec-vs-code discrepancy surfaced (code is truth):** CROFT-PROTOCOL.md §2
  specifies **domain-tagged** genesis/topic pre-images (`"croft-lineage-genesis:" ‖ id`, `TopicId =
  sha256("croft-group-topic:" ‖ id)`), but the Rust workspace computes plain `sha256(canonical_bytes)`
  for `GenesisId` and tags the gossip topic `"lineage-topic-v1"`. The tagged §2 derivations live in a
  *different stack* (the iroh `altdrive-spike-lineage-sync` spike) than `lineage-core`. The vectors were
  derived from what `lineage-core` actually computes, with the divergence recorded — but **whether the
  two stacks must share the tagged pre-images is a genuine reconciliation item** the suite forced into
  the open. Also: cats 7/8/9 (AR / visibility-S2 / freshness) await emission (8/9 are green-model in the
  TS stack); the cat-3 fold is exercised via the lineage-counted-threshold path, not the heavier OpenMLS
  `fold_by_lineage`.

## Cross-cutting open surface (what these narratives keep pointing at)

1. **A staleness/freshness signal — DESIGNED (2026-06-16).** AR-2 + multi-device both rely on "stale
   is visible," but a peer that hears from no one can't tell it's behind. Now designed in
   `thinking/freshness-signal.md`: a signed content-free **tip beacon** (head/epoch/seq, blind-broker-
   safe) + a local time-since-heard clock + the **no-false-current** invariant (never render silence
   as currency), with per-tier horizons and freshness-gates-authority. Principle filed in
   `principles.md`. This unblocks **E2.16** (tier-degradation visibility), now ready to test.
2. **Open/public-regime Sybil + quiet membership (S3).** AR-1 only covers permissioned groups; the
   social layer's join-is-the-point model is the harder, unsolved case.
3. **The Ed25519-over-the-wire gap — CLOSED (2026-06-16).** MD-G2/T11 proved the structural half;
   the FAITHFUL spike now carries the real signed `lineage-history::Message` and verifies signature +
   standing over live iroh (both joiners incl. NAT Mac). Remaining faithful steps narrowed to: MLS
   key-distribution over the wire, and threshold revoke-authority (`thinking/revocation-authority.md`).
4. **Recovery (T12) shadows several edges** — last-device revocation, stolen-device-same-lineage-1-sig,
   total loss. The anchor is decided (delegation + optional seed); the proof is open.
5. **Scale/cost untested over transport** — compaction, long histories. All transport demos are
   small-N. AR-5 measured tree-commit growth (≈linear with the embedded tree) and produced a tier
   rule: **the broadcast tier must disable the embedded ratchet-tree extension** (ship out-of-band)
   to keep commits O(log N). Per-device-as-member is affordable only at human group scale otherwise.
6. **Topic-seed entropy (new, from AR-4).** TopicId = sha256(lineage id), and the relay is
   topic-agnostic, so topic privacy rests entirely on the lineage id being unguessable. Lineage ids
   used as gossip-topic seeds must be **high-entropy / salted**, never human-readable handles.
   *(Promote to the design backlog alongside the staleness signal.)*
