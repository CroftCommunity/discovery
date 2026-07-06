# Drystone delivery: a technical pitch

`For the engineer who wants the better mousetrap, with the tradeoffs shown`

---

## The thesis

Most secure-messaging architectures put a delivery service in the structural critical path, then bolt privacy on top. Drystone's delivery layer is built the other way around: content protection (MLS) and transport (iroh) are the load-bearing primitives, and every helper above them, store-and-forward, gossip, push, is a removable, content-blind, resource-asymmetry role. Take any helper out and you lose convenience, never function or standing. That property is the design, not a feature.

The non-obvious engineering move is to stop treating "delivery model" as a single flat choice and split it into three independent planes: carriage (the path a message travels), durability (where bytes persist for an offline recipient), and presence (who learns a message exists and wakes the recipient). That split is what lets you combine a gossip overlay and a delivery store cleanly instead of treating them as rival whole-system designs, and it keeps the swarm honestly filed as a carriage path rather than a durability source it cannot be.

A second decoupling, orthogonal to the planes, does the rest of the work: the population that **routes** sealed messages is independent from, and may be larger than, the population **entitled** to read them. The routing layer (the Delivery Fabric: a blind, content-agnostic overlay of gossip, direct links, and relays) can span and overlap many Groups; the entitlement layer is the Group (the set of members holding the leaf keys). Because the sealed message carries its own author-signed index, one artifact is simultaneously payload (for the entitled), delivery (for the recipient), and gap-definition (for anyone who sees it pass). Routing scales independently of membership, and reconciliation is a membership-gated act that needs no particular position on the fabric. This is not novel (Hyperledger Fabric ships the same channels-over-a-larger-overlay shape); what is Drystone's is doing it center-free in ordering and governance, with cryptographic rather than routing-policy entitlement.

---

## The substrate (settled, grounded)

- **MLS (RFC 9420 / 9750)** realizes the Group, its Membership, and leaf-keying, providing group key agreement and message protection. PrivateMessage gives E2E confidentiality, integrity, authenticity to the epoch, plus forward secrecy and post-compromise security, independent of transport. Trusted Authentication Service, untrusted Delivery Service: a DS can drop, delay, or observe metadata, never forge or decrypt.

- **iroh core 1.0** (stable 2026-06-15) for transport: dial an endpoint public key, not an IP; QUIC + TLS 1.3; ~90% direct hole-punch with stateless encrypted relay fallback that sees identifiers, not content.

- **iroh-gossip** (separate crate, currently 0.101.0, builds against stable iroh 1.0.1, confirmed by a live build) for the overlay: HyParView membership plus PlumTree broadcast. PlumTree uses a lazy message-hash internally, but a falsified experiment (E1.1) showed it is *not* surfaced to applications: the app sees a message arrive with content, or a "you fell behind" signal, never a content-free presence hash. This does not cost you presence signalling, because you do not need presence-*without*-content: a carrier receives the sealed blob, treats arrival as the signal, fires a bare wake, and discards the blob. No companion channel, and a separate announce channel is deliberately rejected (it would split "exists" from "here it is" across two channels and reintroduce a TOCTOU race for only a bandwidth saving). Gossip's strength here is fast live delivery; durability and what-you-missed detection come from elsewhere.

---

## The three planes

A delivery model is a combination of one choice from each plane. The planes are orthogonal axes, sources within a plane are non-exclusive (they can race), and where one mechanism serves two planes by construction the design says so.

**Plane C, carriage** (the path a message travels):

- **C-direct**: a direct QUIC dial, no helper. Most center-free. When participants carry their own traffic this fuses with D-self (one act both delivers and persists).

- **C-swarm**: the gossip overlay carries to whoever is live on the topic and heals live-tree breaks. Efficient live delivery, but it keeps no replay log, so a fully-offline node recovers nothing from the swarm itself (confirmed: a late joiner recovered zero). C-swarm is carriage with no durability of its own; an offline recipient needs a separate durability source. Filing the swarm here, as a path rather than a store, is what lets the design say "the swarm does not persist" cleanly instead of listing it as a durability option and apologizing that it is not durable.

- **C-relay**: when no direct path forms, an iroh relay forwards the encrypted packets. Content-blind carriage assist.

**Plane D, durability** (where sealed bytes persist for an offline recipient):

- **D-self** (floor): participants hold the buffer (queue-and-retry). The no-helper path, always available.

- **D-meer**: a blind store-and-forward node holds byte-identical sealed bytes. Never keyed, revocable, redundant. It keeps the MLS Delivery Service's store-and-forward function and sheds its ordering function, which is not a sacrifice: ordering is sourced intrinsically from the messages' own signed indices, not imposed by a sequencer, so a blind store has no ordering job to do.

- **D-peer**: members are already entitled to all the group's messages, so two members reconcile their held history directly (same RBSR as the device group), making the group its own distributed replay buffer with no central store. Sound under hard invariants (self-verifying records only, governance-gated, current-membership-only, exit-final), and voluntary per member. The honest cost it carries that the blind roles do not: a sync partner is not blind, reconciliation leaks a coarse activity-pattern between members, so whether it defaults on is a per-scope threat-model dial.

**Plane P, presence/wake** (who learns mail exists and can poke the recipient):

- **P-none**: poll a meer with your cursor on your own schedule. No wake, no device-token binding.

- **P-gossip**: a fabric carrier sees a sealed message arrive, treats arrival as a content-free signal, and fires a wake, reading nothing and discarding the blob. Works on stock gossip, no companion channel. A detector that hands off to P-push (the actuator) to reach a sleeping device.

- **P-meer**: the holder pokes.

- **P-push**: a byte-free push-notify node fires a content-free wake via APNs/FCM.

The meer is worth noting as one node on three planes: it persists (D-meer), you carry-fetch from it, and it can poke (P-meer), three separable roles in one helper.

Because MLS PrivateMessage bytes are byte-identical across paths, dedup is just hashing, so **racing is free**: C-direct, C-swarm, D-meer, and D-peer can all attempt at once, first wins, duplicate dropped (confirmed: one seal relayed twice dedups to one entry; the selector delivers exactly once down to the D-self floor). That is the maximum-robustness posture, and it is exactly the "gossip and a store-and-forward together" combination that a flat taxonomy cannot express.

---

## The three helper roles, and why they are all the same kind of thing

Relay (reachability), meer (durable blind storage), and push-notify (wake signal) are three instances of one category: a resource-asymmetry role. Each is content-blind, revocable, and redundantly held; none is an authority. The asymmetry is in what the node *has* (uptime, storage, credentials), never in what rights or standing it holds.

The push-notify node is the interesting one because it is the platform accommodation that could have become a structural dependency and does not:

- It sends a **content-free wake**, not the message. Two reasons converge. The rights model wants the smallest metadata footprint; and the platform forces it, since the reliable push channel is the user-facing alert and the silent/background channel is aggressively throttled and droppable (Apple's own guidance: a few per hour, dynamic, may be skipped). A wake-then-fetch design degrades to "catch up on next foreground" cleanly.

- It is **doubly removable**: foreground app needs no push host at all, and poll-a-meer is a non-push wake path.

- The one irreducible cost is the device-token-to-EndpointId binding, and because polling avoids it, that cost is **opt-in**.

Note the platform reality that makes MLS non-negotiable here: FCM's own docs state the transport is not E2E encrypted. The push provider is a blind intermediary by force, and MLS is what makes that safe.

---

## The device group: a secondary convergence backplane

A user's devices are each independently, equally, leaves in the primary Group (MLS flatness preserved, no invented leaf-to-leaf authority). *Separately*, a user's own devices form **their own Group** to reconcile history. It is not a special construct: it is a first-order Group like any other, distinguished only by scope (lineage-restricted admission, single owner).

State it precisely, because the loose version ("plaintext device pool") describes the wrong layer. The device group moves **sealed PrivateMessage bytes** among its leaves, on the wire, exactly as every Group does. Its leaves can decrypt what they receive because they hold the leaf key, but that is just what membership is; plaintext exists only locally after a leaf decrypts, the ordinary state of any member of any Group, not a property of the sync. So the device group is a **secondary history-convergence backplane with a stronger membership story**: a backplane because it is a second Group whose job is reconciling history across your devices, stronger because admission is lineage-restricted (verifiable cryptographic descent, not an asserted invite). A tighter scope and a tighter join rule, neither a new exposure: every leaf is the same already-entitled owner, so convergence widens entitlement to no one. The convergence invariants that matter (trust on self-verification not the partner's word; dataplane history only, never governance) hold here for the ordinary reason they hold for every Group's convergence.

Because it is a first-order Group, it is **eligible for the full delivery stack, including fabric/swarm sync**, not only direct local links. That is the durability payoff: devices converge asynchronously, without ever being simultaneously reachable, through whichever device comes online next. The resulting property, with honest bounds: **if any leaf of a user's lineage receives a message, every enrolled device of that user eventually sees it**, bounded by reachability (eventual consistency, not instant, gap stays visible via the high-water mark while it persists) and by voluntary enrollment. Safe for the same reason all of it is safe: blind carriers, same-owner entitlement. Whether the device group rides the fabric is a dial, default on (durability), with direct-link-only as the tightening for a user who prefers their device sync never touch the swarm.

This buys, from one mechanism: complete multi-device history with no server syncing your devices, and a personal replay buffer that rescues standalone C-swarm (your phone recovers from your laptop, now even when they are never co-present). It is the same gap-aware convergence as D-peer at the tightest scope, the only differences in the user's favor (no metadata leak to a separate principal, lineage-governed admission).

Admission control is where the having-read-is-irreversible argument bites: content once read cannot be clawed back, so the moment a device is admitted as a history holder is where control is most valuable. Hence the lineage-restricted join: admit a device only if it proves descent from your rooting key (verifiable), where "I invited it" is merely asserted. RFC 9750 names multi-device-same-user as application policy and RFC 9420 puts credential validation at the AS, so this is a designed extension point. Convergence stays a continuous dial (which scopes, how often), so the seized-device defense is the ongoing ability to cut future convergence, even though already-synced history is gone.

---

## The cheap part: range-based set reconciliation

Device sync uses RBSR (Meyer 2212.13567; Willow; Negentropy/Nostr). Recursively partition an ordered domain, compare fingerprints, descend only on mismatch. Honest cost: logarithmic rounds, communication within a log factor of optimal (optimal = the difference). Not "difference only," but cheap for mostly-agreeing replicas, and it is deployed tech, not theory.

Two properties make it fit Drystone specifically:

- The ordering key can be **any clock-free monotonic criterion** (Negentropy: anything fitting a u64), so it does not smuggle a wall-clock into a design whose whole ordering spine is timestamp-free.

- It needs a **range-summarizable backend** (a tree with subtree fingerprints), not a flat log. The lean: a monotonic storage index as the canonical order, with the wall-clock timestamp as a UI display attribute only.

---

## The unifying mechanism: gap-aware history convergence

C-swarm hole detection, D-peer, and device-group sync are not three features, they are one: **detect a nameable gap, fill it from a self-verifying source.** This is inherited anti-entropy (Demers 1987: reconcile, resolve differences, converge), refined to RBSR.

Detection is the per-author high-water mark, and the nice identity is that this mark *is* the dataplane half of the catch-up cursor, not a second structure. Each message carries an author-signed monotonic index; the highest index you hold from an author asserts the complete expected range below it, so any index you lack in that range is a *nameable* gap ("I lack A's #14"), not a vague unease. A single fresh message over any path re-establishes the range, which is why gossip's lossy delivery still sharpens detection even though it never replays. (The one thing it cannot detect is an author you have *never* heard from, no mark to anchor on, which is an explicit, honest non-goal.)

Fill is priority-ordered by metadata leak, not by validity (records self-verify regardless of source): **your own devices first** (zero leak, often a local link), then **several members** (bounded leak, multi-source to defeat any single member withholding), **never the swarm** (unentitled). Reconciliation is a membership-gated act over any transport, the partner need not be on the fabric at all. That is the whole decoupling paying off: detect from anywhere, fill from an entitled member wherever they are.

---

## The ordering discipline (where the rigor shows)

One ordering structure, the timestamp-free causal fold, consumed by two layers under two policies:

- **Governance**: concurrent mutually-exclusive ops do not linearize, they **escalate** (fork-not-verdict). A wall-clock here would manufacture a false verdict, which is precisely the failure the design refuses.

- **Dataplane**: ordering is a **UI service**. Concurrency is linearized because nothing that converges or resolves authority depends on display order. The concurrent tiebreak is the **content hash of the sealed message**: deterministic, clock-free, already canonical as the dedup key, so every device sorts identically. The timestamp is a label, never the sort key.

That is the whole timestamp posture: out of anything that converges or resolves authority, present only as human-readable metadata.

---

## What it does not pretend

- **No real unsend.** Plaintext seen is plaintext kept, for any recipient in any group, device pool included. We build the controls that work (admission, cutting off future convergence) and refuse the delete button that does not delete.

- **C-swarm can lose messages.** A group may accept that, as a dial. What the design forbids is *invisible* loss: loss-tolerance is group-set, loss-*visibility* is floored. You never get a partial state presented as the whole.

- **Helpers are honest conveniences.** The default deployment is D-meer + P-push at parity with a normal app, because few users trade UX for ideology. The principles ride underneath a default that simply works; the no-helper path is guaranteed by being available to the minority who exercise it, not by being forced on everyone.

---

## Why this is a better mousetrap

The standard architecture makes you choose: pleasant UX with a structural center, or center-free with degraded UX. Drystone's claim is that the choice is false once you (a) make content protection transport-independent (MLS), (b) make addressing key-based and connection direct-first (iroh), and (c) split delivery into orthogonal durability and presence planes so every helper is a removable, blind, racing option rather than a dependency. The parity default and the removable helpers are then the same design seen from both ends.

None of the primitives are novel, and that is stated as a strength, not hidden. Epidemic dissemination and anti-entropy are Demers et al. 1987; the broadcast-tree refinement is HyParView/PlumTree 2007; the routing-overlay-larger-than-entitlement-group shape ships in Hyperledger Fabric. The synthesis is where the engineering lives: three planes (carriage, durability, presence) with the fusions named where they occur, a blind Delivery Fabric decoupled from cryptographic entitlement, sources that race for free on byte-identical seals, a device group as an encrypted-content replay backplane, gap-aware convergence as one mechanism across three relationships, and one clock-free order read two ways, made center-free in ordering and governance where the prior art keeps a center.

And it is not paper-only. Round 1 turned thirteen load-bearing claims into falsifiable experiments against the real libraries (iroh 1.0.1, iroh-gossip 0.101.0, mls-rs 0.55.2): twelve confirmed (byte-identical cross-path dedup, local-link delivery with the relay disabled, clock-free ordering convergence, exactly-once delivery down to the D-self floor, wake-then-fetch recovering everything with push suppressed), one falsified (the gossip presence signal), which reshaped one branch, exactly what a falsifiable plan is for. Two entitlement claims initially passed only in model form and were flagged as debts, not glossed. Round 2 then ran eleven more experiments, all confirmed at real-library fidelity, retiring both model-form debts against real mls-rs (the entitlement boundary turns out to be cryptographic via HPKE-bound Welcomes, not a policy check; lineage admission is rejected at the real credential-validation hook) and validating the newer mechanisms: gap-aware convergence on real signed records, the blind non-member carrier that relays but cannot decrypt, the device-group backplane, exit finality with authorship-relative validity surviving epoch transitions, and deterministic ordering over real ciphertext. No model-form debts remain.
