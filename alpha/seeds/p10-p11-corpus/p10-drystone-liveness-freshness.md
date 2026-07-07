# Drystone: cross-tier liveness and freshness, a candidate proposal

`Status: candidate proposal for discussion, staging. Not normative, not one of the three reader resolutions. Grounds a future §7 addition. Claims tagged [GROUNDED] (cited primary source) or [PROPOSAL] (open design choice).`

This is the working proposal for unifying liveness and freshness signaling across the Drystone stack. It
supersedes the earlier research draft of the same content; the corrections from that discussion, the MLS
counter precision and the reframing of solicitation as a local trust dial, are folded in here.

## TL;DR

- **The move.** A hybrid: opportunistically harvest the liveness signals already flowing through every tier
  (QUIC path liveness, iroh-gossip membership, MLS traffic, Pkarr republishes, relay connectivity,
  KeyPackage refresh), and add one explicit `LocateLatest` (ask-for-HEAD) primitive whose answer is a set
  of head attestations from distinct persona lineages.

- **The load-bearing distinction.** Liveness ("is a peer reachable or recently active") is monotonic and
  cheap. Freshness ("is my view of HEAD the latest that exists") is non-monotonic and, by CALM, has no
  coordination-free local detector. A partitioned node cannot self-certify currency; absence of
  corroboration must fail closed, never assert currency.

- **Solicitation is a local trust dial, not a group parameter.** A node can never know how many
  corroborations any other node saw, or what any other node concluded, so the solicitation threshold is
  unenforceable and unobservable by anyone else. It is how much evidence this node wants before it chooses
  to act as if current. The per-plane "defaults" are suggested starting values, not shared parameters.

- **Governance already carries its own corroboration; the dataplane does not.** When k lineages sign to
  meet a sealing threshold, that signature set is a cryptographic fact that k lineages were at this head,
  produced as a byproduct of the decision. So governance leans on the sealing signatures and solicitation
  is secondary there. The dataplane has no sealing event, is lower-stakes and recoverable, and wants a much
  lower local trust floor, which is where solicitation actually earns its keep.

- **Counting is over persona lineage, not MLS clients.** Three devices of one persona are one corroboration.

## 1. The core distinction: liveness is not freshness [GROUNDED + PROPOSAL]

Two questions run through every tier and are constantly conflated.

**Liveness** asks whether a peer is reachable or recently active. It is monotonic in the useful direction:
evidence of activity can only raise confidence that the peer was alive at a point in time. The classical
theory is the unreliable failure detector (Chandra and Toueg, *Unreliable Failure Detectors for Reliable
Distributed Systems*), characterized by completeness (every crashed process is eventually suspected) and
accuracy (correct processes are not wrongly suspected); perfect accuracy is unachievable in a fully
asynchronous system because a slow process is indistinguishable from a crashed one. The phi-accrual
detector (Hayashibara, Defago, Yared, Katayama, SRDS 2004, DOI 10.1109/RELDIS.2004.1353004) replaces the
binary suspicion with a continuous value derived from the distribution of heartbeat inter-arrivals. SWIM
and its Lifeguard extension (Dadgar, Phillips, Currey, 2018) add a suspect state and local-health-aware
timeouts to cut false positives.

**Freshness** asks whether this node's view of the governance HEAD, or the dataplane HEAD, is the latest
that exists. That is the question "have I seen every committed event," and it is non-monotonic: new
information can invalidate a prior "I am current" belief. By CALM it therefore has no coordination-free
local detector.

Why both matter: a node can be perfectly live (fast RTT, healthy gossip neighbors, recent MLS traffic)
while stale (partitioned away from the branch where the latest Commit landed). Liveness signals are
abundant and cheap; freshness is scarce and expensive. This maps onto the Drystone principle that the
substrate furnishes provenance and possibility and never renders a verdict: liveness and freshness signals
are inputs a node weighs, never a verdict the substrate pronounces.

## 2. What MLS actually exposes, and how it differs from our stamp [GROUNDED]

The earlier draft conflated two different MLS counters. They are distinct, and only one of them leaks.

- **The per-sender generation counter.** This is the counter RFC 9750 describes for gap detection: a member
  who sees a gap in a sender's generation sequence knows it missed a message from that sender. It is
  **per-sender and epoch-scoped**, it lives in the **encrypted** content, and it **resets at each epoch**.
  It is not an infinite monotone integer, and it is not visible to a network observer.

- **The epoch.** This is the counter that is actually visible on the wire. RFC 9750 (§8.1.2) describes the
  observable header metadata as the opaque group_id, the epoch, and the content type (application, proposal,
  or commit). The epoch is the count of changes made to the group, so it is the value that climbs roughly
  monotonically and is correlatable by a passive observer. `[GROUNDED]`

So the leaked number is the **epoch** (a governance-change odometer), and the encrypted gap-detection number
is the **per-sender, per-epoch generation**. My earlier "leaked infinite per-message counter" was wrong on
both counts: the leaked one counts group changes not messages, and the per-message one does not leak.

**How our governance-generation stamp relates.** Drystone stamps each dataplane delivery with a monotonic
governance-generation value that locates the payload against the authority chain: it answers "which
governance head was this authored under." That is a **provenance pointer from content to authority**, a
Drystone construct. It is not the MLS epoch (a cryptographic-state odometer MLS maintains regardless of us)
and not the MLS per-sender generation (an encrypted, epoch-scoped anti-gap counter). The only overlaps
worth naming are that all three let a receiver notice it is behind, and the epoch and our stamp are both
correlatable metadata a watcher can log. They are not redundant, because they answer different questions:
the epoch is "how many times has this group's crypto state advanced," the per-sender generation is "did I
miss a message from this sender in this epoch," and our stamp is "which authority head governs this
payload."

The stamp closes the **behind-via-traffic** half of gap detection: if I receive a payload stamped at
generation g and I only hold g minus two, I know I am behind. What no stamp can do is reveal the
**unreferenced ahead-tail**, committed history I have never received any pointer to, because in a partition
nothing arrives to reference it. `LocateLatest` exists to bound that ahead-tail, and it can only bound it,
never eliminate it.

## 3. Tier-by-tier opportunistic liveness signals [GROUNDED]

| Tier | Signal | Tells you | Does not tell you |
|---|---|---|---|
| Transport (iroh/QUIC) | Idle timeout not firing; PING keep-alive; PATH_CHALLENGE and PATH_RESPONSE; per-path RTT | A path to the endpoint key is reachable; endpoint holds the key | Anything about governance or dataplane state; whether the persona (vs one device) is engaged; freshness |
| iroh-gossip (HyParView) | NeighborUp, NeighborDown; active-view membership; shuffle | A swarm neighbor arrived or left; overlay health | Whether you hold the latest content; partition on the far side |
| iroh-gossip (PlumTree) | Received; Lagged; IHAVE, GRAFT, PRUNE repair | A message propagated; (Lagged) you fell behind the stream | Committed history never gossiped to your partition |
| MLS governance | Commit advancing epoch; Proposal; epoch counter | Sender held current group state at commit; epoch gap shows you are behind | That nothing newer exists ahead of the last epoch you saw |
| MLS dataplane | Application message; per-sender generation | Sender was live and current at send; generation gap shows a miss | The unreferenced ahead-tail |
| Discovery (Pkarr, mainline DHT) | Signed record republish; BEP44 seq | The persona or device refreshed its address record recently (default TTL 7200 s) | Real-time presence; governance freshness |
| Discovery (mDNS) | LAN record refresh | A device is present on the local segment | Anything WAN-scale or governance-related |
| Relay | Home-relay handshake complete (endpoint online) | Endpoint is registered and reachable via relay | Direct-path liveness; freshness |
| MLS KeyPackage | Publication or refresh at the delivery service; lifetime | Persona is provisioning for async adds; recently active | Engagement in any specific group's HEAD |

**Layering principle [PROPOSAL].** Higher-tier signals are semantically richer but sparser; lower-tier
signals are abundant but empty about state. Prefer the highest-tier signal available, and treat transport
liveness as a corroborator only.

## 4. The QUIC-surfacing question [GROUNDED facts, PROPOSAL recommendation]

QUIC connections have a negotiated idle timeout equal to the minimum of the two endpoints' max_idle_timeout
(RFC 9000 §10.1). An endpoint defers it with PING frames (§19.2) or application data. RFC 9308 §3 warns
against PINGs more often than every 30 seconds over long idle periods for power reasons, layered on RFC
8085 §3.5 (BCP 145), which says not to send keep-alives more often than every 15 seconds. Path validation
(PATH_CHALLENGE with 8 unpredictable bytes echoed in PATH_RESPONSE, RFC 9000 §8.2) confirms reachability on
a path; note CVE-2023-49295 (CVSS 7.5), where unbounded PATH_CHALLENGE queuing exhausts memory. iroh 1.0
exposes reachability only indirectly (endpoint online state, per-path RTT, a path-event stream,
connection-close futures) with no app-facing "last received packet" timestamp, and it removed the coarse
connection-type accessor for 1.0.

**Recommendation [PROPOSAL]: keep QUIC keepalive opaque to the governance layer.** Reasons: transport
liveness is semantically empty about state; its cadence is tuned for NAT and power, not governance; it is
the signal most cheaply spoofed (an adversary keeps a connection warm while withholding all state); and
iroh already keeps it low-level. The compromise: the transport adapter emits one coarse derived boolean per
touchpoint, recently-reachable(lineage), as one weak corroborator among many, never a freshness input. The
governance layer never sees PING frames or idle timers.

## 5. The explicit `LocateLatest` mechanism [PROPOSAL]

`LocateLatest` is the explicit backstop and the concrete form of the beam's owed freshness attestation.

```
LocateLatestRequest {
  plane:            GOVERNANCE | DATAPLANE
  scope:            group_id (+ optional subspace/path selector for dataplane)
  known_head:       { generation_stamp, epoch (if governance), digest }
  solicitation_k:   distinct persona lineages of corroboration THIS node wants
  nonce:            fresh unpredictable bytes, binds responses to this query
  freshness_window: max attestation age this node will accept
}

HeadAttestation {                      // one per responding persona lineage
  plane, scope
  asserted_head:    { generation_stamp, epoch (if governance), digest }
  as_of:            attester's local monotonic time or observed generation
  lineage_id:       persona-lineage identifier (NOT a client or device id)
  device_tag:       opaque, so several devices of one persona collapse to one lineage
  nonce_echo:       the request nonce, signed
  signature:        over (asserted_head, as_of, nonce_echo), key chaining to the
                    persona's Meadowcap / credential authority
  optional_proof:   inclusion pointer letting the requester fetch the asserted head
}
```

**Corroboration procedure.** The requester verifies each signature and nonce echo, collapses attestations by
lineage_id so a persona's devices count once, groups distinct lineages by asserted_head, and if at least
solicitation_k distinct lineages assert a head at least as new as known_head (and it is newer) it fetches
and validates that head and advances. This is a quorum read in the Dynamo lineage, but with the count over
personae and the value a monotone head pointer. If corroboration is insufficient, the node fails closed on
any currency-gated action; it does not conclude it is current.

**Governance vs dataplane.** Governance answers with {epoch, commit digest} and validation is strict, chaining
to a Commit the requester can verify against the ratchet tree, because a stale governance view can
mis-authorize. Dataplane answers with {generation_stamp, entry digest} scoped to a subspace or path, and
validation can be looser because dataplane staleness is recoverable; Willow's own join rule (newest
timestamp, then payload-digest tiebreak) gives a natural order for reconciling dataplane heads.

## 6. The threshold model, reframed [PROPOSAL]

This is the heart of the design, and the discussion reframed it: solicitation is not a quorum with shared
defaults, it is a local trust dial.

**Two thresholds, kept separate.**

| | Sealing threshold | Solicitation threshold |
|---|---|---|
| Question | How many corroborations to enact a governance decision? | How many corroborations before THIS node trusts a HEAD answer? |
| Serves | Authorization and safety of the action | This node's confidence in its own view's currency |
| Who sets it | The group, per governance knob | Nobody but the local node; a per-plane default is only a suggestion |
| Observable to others | Yes, the signatures are on the record | No, unobservable and unenforceable by anyone else |
| Analogue | Write quorum, or a threshold signature | A local read confidence dial |
| If unmet | Action under-authorized, does not enact | This node stalls, does not assert currency |

- **Per-knob, not group-global [PROPOSAL mechanism for a fixed requirement].** The group's k-of-n is a dial
  attached to each governance knob, stored as governance state. The same group can be permissive about
  trivia and strict about consequential or self-amending actions, which is "under-authorize rather than
  mis-authorize."

- **Solicitation is local and unenforceable [GROUNDED by the impossibility, PROPOSAL framing].** No node can
  know how many corroborations another node gathered or what it concluded; three nodes cannot know each
  other's tallies. So the solicitation threshold is a purely local confidence knob, unobservable to others
  by construction, and that is correct rather than a gap. Framing it as a group parameter would promise an
  enforcement no peer-symmetric system can deliver.

- **The plane asymmetry is a consequence, not a stipulation.** Governance carries its own corroboration: the
  k signatures that meet a sealing threshold are a cryptographic fact that k lineages were at this head when
  they signed, produced as a byproduct of the decision itself, so governance currency leans on the sealing
  signatures and solicitation is a secondary check there. The dataplane has no sealing event, is ongoing
  day-to-day traffic, and is lower-stakes and recoverable, so it wants a much lower local trust floor, and
  this is exactly where solicitation earns its keep, as a data point rather than a gate, which is also the
  safer place for it to live.

- **Counting over persona lineage, not MLS clients [GROUNDED rationale].** MLS supports one user running
  several clients (RFC 9750 §6.7). If corroboration counted clients, a persona on three devices would forge
  a 3-of-n quorum alone. Counting must collapse a persona's clients to a single lineage weight. This is the
  direct analogue of the exploited quorum bug where counting signature blobs instead of distinct recovered
  signers lets one validator forge a quorum, so Drystone counts distinct lineages and deduplicates devices
  within a lineage.

- **Node-local trust floor, tighten-only [GROUNDED posture, PROPOSAL mechanism].** Any node may set its own
  solicitation threshold higher than the plane default. This is unilateral tightening: it can only make that
  node more cautious, never loosen safety for anyone, and the cost (its own latency or availability) is
  borne only by that node. Loosening below the plane default is not offered, because that would export risk
  onto the group.

**Proposed starting values, explicitly open [PROPOSAL].** Governance solicitation, higher, suggest 3
distinct lineages or min(3, n minus 1) in small groups. Dataplane solicitation, lower, suggest 1 to 2.
Sealing, per knob, with a majority-of-lineages floor for any knob that changes authority or the thresholds
themselves. All are starting points for discussion, not claimed optima.

## 7. What solicitation can and cannot certify [GROUNDED reasoning]

The distinction is the whole safety argument. Solicitation can **never** certify "I am at latest," because
that is certifying the unknown; no signature set can attest to the absence of a newer event that a partition
could be hiding. What the governance **sealing signatures can** certify is the positive statement "k lineages
were at this same head when they signed." That is the reliable backstop precisely because it is a positive
fact about what was seen, not a claim about what was not. A solicitation response, by contrast, is only ever
notable to the asking node, because no other node witnesses it.

The one alternative that would make a solicitation response mean more, and which the discussion rejected as a
default, is to have responders sign an attestation that is then committed into the chain as part of this
node's advance, turning the corroboration into a witnessed governance fact rather than a private one. That is
not a sane default: it turns every currency check into a governance write, it is an added burden on top of
the sealing threshold, and it drags the impossibility of proving currency back in through a heavier door. It
should remain available as an opt-in for a node that wants a very high, witnessed trust floor, and never be
the standard.

The consequence for a genuine divergence follows the rest of Drystone. Because we cannot prove that no one
advanced HEAD without our awareness, we do not try to. If two advances turn out mutually exclusive, that is a
social-utility judgment resolved by fork, which is the group's call and not the substrate's. If they turn out
compatible, they compose at the group's discretion or by determinism. Solicitation only sets the confidence
at which a node chooses to move forward, and its honest ceiling is corroboration of a positive head, never
proof of a negative.

## 8. Fail-closed under partition, and the CALM framing [GROUNDED]

The one load-bearing unearned property is the completeness-ahead beam: a partitioned node cannot self-certify
that nothing newer exists. CALM (Hellerstein and Alvaro; proven for relational transducers by Ameloot et al.,
arXiv:1901.01930) states that a program has a consistent, coordination-free distributed implementation if and
only if it is monotonic. "Have I seen every committed event" is non-monotonic, so no purely-local,
coordination-free completeness detector can exist; non-monotonic problems must wait until they know all
information has arrived, which requires coordination.

Consequences [PROPOSAL following the theorem]. Absence of corroboration must stall, not assert currency: a
node that cannot gather its solicitation floor of lineage attestations must treat its currency as unknown and
fail closed on currency-gated actions, never reading "I heard nothing newer" as "nothing newer exists." The
admissible substitute for a center is a corroboration-witnessed checkpoint, here the set of distinct-lineage
head attestations meeting the local solicitation floor; this is the coordination the theorem requires, done
peer-symmetrically. And this bounds rather than eliminates the ahead-tail: `LocateLatest` converts "I silently
assume I am current" into "I have positive, corroborated, replay-bound evidence from k distinct personae, or
else I stall." The beam stays unearned in the limit, since a total partition can hide an arbitrary tail, but
the unearned region becomes explicit and gated rather than silent.

## 9. Attack and abuse surface [GROUNDED where noted, else PROPOSAL]

- **Liveness spoofing.** Cheapest against transport signals: keep a QUIC connection warm while withholding all
  state. Mitigation: never treat transport liveness as freshness; require signed, nonce-bound head
  attestations for currency.

- **Corroboration withholding as griefing.** A set of lineages refuses to answer to force cautious nodes to
  stall. It cannot cause a safety breach, since stalling is fail-closed, only reduced availability.
  Mitigation stays advisory, because the substrate renders no verdict; opportunistic signals still let a node
  lower its confidence gracefully.

- **Metadata leakage.** The MLS epoch is already observable header metadata (RFC 9750 §8.1.2), and
  `LocateLatest` traffic, head digests, Pkarr republishes, and relay presence add correlatable timing.
  Mitigation: run it over the metadata-confidential transport MLS assumes, minimize plaintext head
  identifiers, and consider padding or batching solicitation traffic.

- **Amplification and DoS.** An attacker triggers many solicitations to amplify traffic. The direct precedent
  is CVE-2023-49295, fixed by capping queued PATH_RESPONSE frames at 256 (about 4 kB per connection).
  Mitigation: rate-limit solicitation per requester, cap outstanding attestation state, and make the request
  nonce bind work.

- **Presence privacy.** The whole apparatus is a presence-tracking surface. Mitigation: make opportunistic
  beaconing opt-in per touchpoint, and prefer the pull (`LocateLatest`) over continuous push beacons where
  privacy matters.

- **Sybil and lineage forgery.** The entire threshold model rests on unforgeable device-to-persona binding.
  If lineage attribution is forgeable, the dedup rule collapses and one attacker forges a quorum. Mitigation:
  lineage identity must chain to the credential and capability layer. This is the single most safety-critical
  assumption.

## 10. Open sub-questions to talk through [PROPOSAL]

- What constitutes a persona lineage cryptographically, and who authorizes device-to-lineage binding, the
  persona's own key, a Meadowcap owned namespace, or group governance?

- Should solicitation attestations be individually signed or aggregated via a threshold signature that proves
  "at least k distinct lineages" without revealing which? Aggregation helps privacy but complicates the
  dedup-to-lineage count.

- How fresh is fresh? What freshness_window default per plane, and is it wall-clock, generation-relative, or
  epoch-relative?

- Is the finality gate's currency requirement (solicitation) mandatory for all governance knobs or only
  high-stakes ones?

- Is there a principled partial-confidence state between full corroboration and hard stall, or is fail-closed
  strictly binary?

- How do opportunistic signals feed the confidence estimate quantitatively, a phi-accrual-style continuous
  score or a simple recency boolean per touchpoint?

- What is the interaction with MLS out-of-order tolerance and the delivery service's single-Commit-per-epoch
  selection, and can that role be discharged center-free or does `LocateLatest` partially replace it?

- What concrete rate limits and attestation-state caps keep solicitation safe without crippling legitimate
  catch-up after a partition heals?

## 11. Suggested rollout [PROPOSAL]

- **Stage 1, ratify the split.** Adopt "liveness is not freshness" as an invariant and forbid any code path
  that derives a currency conclusion from a transport or gossip liveness signal alone.

- **Stage 2, opportunistic adapters, read-only.** Per-tier adapters that emit one coarse
  recently-reachable(lineage) corroborator. Keep QUIC keepalive opaque.

- **Stage 3, ship `LocateLatest` with conservative defaults.** Governance solicitation default 3 distinct
  lineages, dataplane 1 to 2, per-knob sealing dials with a majority-lineage floor for authority-changing
  knobs. Wire the finality gate so a governance actor needs both the sealing threshold met and its own
  solicitation floor satisfied.

- **Stage 4, harden.** Solicitation rate limits and attestation-state caps, everything over the
  metadata-confidential transport, opt-in beaconing per touchpoint, and the cryptographic definition of
  persona lineage pinned before any of this is load-bearing.

## Caveats

- This is a discussion candidate. Message shapes are conceptual and all numeric defaults are proposals, not
  validated optima.

- The beam remains unearned in the limit. No mechanism lets a totally-partitioned node certify completeness;
  `LocateLatest` bounds and makes explicit the ahead-tail, it does not eliminate it.

- iroh and iroh-gossip API names evolve; identifiers should be re-checked against the pinned versions at
  implementation time.

- Lineage binding is the critical assumption, and everything safety-relevant rests on unforgeable
  device-to-persona attribution.
