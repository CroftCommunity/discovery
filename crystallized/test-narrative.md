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

## Cross-cutting open surface (what these narratives keep pointing at)

1. **A staleness/freshness signal.** AR-2 + multi-device both rely on "stale is visible," but a peer
   that hears from no one can't tell it's behind. A heartbeat/freshness mechanism is an undesigned
   requirement. *(New — promote to the design backlog.)*
2. **Open/public-regime Sybil + quiet membership (S3).** AR-1 only covers permissioned groups; the
   social layer's join-is-the-point model is the harder, unsolved case.
3. **The Ed25519-over-the-wire gap.** MD-G2/T11 proved the structural half over transport; carrying
   the real signed `lineage-history` message end-to-end is the remaining faithful step.
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
