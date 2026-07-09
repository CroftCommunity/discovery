# Drystone delivery layer: provenance and prior art

`Status: grounding (prior-art lineage for the delivery design)`

`Purpose: state honestly what Drystone's delivery layer inherits, and what is genuinely ours`

`Companion to: 01-delivery-architecture.md, 02-references.md`

---

## Why this doc exists

The delivery layer is not a novel mechanism. It is a composition of well-established distributed-systems primitives, several of them decades old, plus a smaller set of choices that are distinctively Drystone's. Honesty about that line is worth more than a claim of novelty: it grounds the design in literature a reviewer already trusts, it tells us which properties are battle-tested versus which are ours to defend, and it keeps us from re-coining names for solved primitives. This doc states the lineage with primary citations, then isolates what is actually new.

The governing finding: **the mechanism is inherited; the composition and the rights framing are ours.**

---

## 1. The root: epidemic algorithms (Demers et al., 1987)

The foundational source is Demers et al., "Epidemic Algorithms for Replicated Database Maintenance," Xerox PARC, ACM PODC 1987. It was written for the Clearinghouse name service on the Xerox Corporate Internet, a large database replicated across many sites that had to converge without a central coordinator. That is our problem, forty years upstream.

The paper introduces the three primitives the whole field still uses, and Drystone uses two of them directly:

- **Rumor mongering** (a *complex epidemic*): a new update becomes a *hot rumor* and is spread probabilistically to random peers; a node stops spreading once it keeps hitting peers that already have it. Fast, low-traffic, but with a non-zero chance of not reaching everyone. *This is the direct ancestor of gossip broadcast (PlumTree's eager/lazy spreading, which iroh-gossip implements), and it is Drystone's C-swarm.* Its honest limitation, that it does not guarantee delivery to all, is exactly the weak-durability property we ground C-swarm on.

- **Anti-entropy** (a *simple epidemic*): every site periodically picks a random peer, exchanges contents, and resolves differences, reliably converging but at higher cost. The paper even gives the efficiency trick we depend on: two sites reconcile by exchanging updates and incrementally recomputing checksums until the checksums agree. *This is the direct ancestor of range-based set reconciliation (RBSR), and it is Drystone's gap-aware history convergence.* RBSR is modern range-based anti-entropy: the checksum-until-agreement loop, made logarithmic by comparing range fingerprints.

The paper's own pairing of the two is, almost verbatim, our architecture: rumor mongering spreads cheaply but incompletely, so anti-entropy is run to guarantee convergence, and a discrepancy found during anti-entropy can be re-injected as a hot rumor. Drystone's "C-swarm for the fast common case, gap-aware convergence to close what it misses" is that 1987 pairing, with a Group entitlement plane and center-free governance added on top.

The epidemiological vocabulary the design lives in (a fabric that "carries" sealed updates) is also the paper's own: it adopts the epidemiology terms, a site is infective if it holds and will share an update, susceptible if it has not received it, removed otherwise. Naming the carrying layer in that register (fabric, carry, vector) is staying inside the established metaphor, not inventing one.

The paper also introduced **death certificates** (and dormant death certificates) for propagating deletions through an epidemic system without old data resurrecting. This is worth flagging as prior art for the history-modes work (`07-history-modes.md`): the problem of "how does a delete propagate and stay deleted in an eventually-consistent replicated store" is 1987-old, and Willow's tombstone-leaving prefix-prune is a modern descendant of the death-certificate idea. We should ground the mutable-mode deletion discussion in that lineage rather than treating convergent deletion as novel.

---

## 2. The broadcast-tree refinement: HyParView + PlumTree (2007)

Drystone's gossip is not raw 1987 rumor mongering; it is the membership-plus-broadcast-tree refinement that iroh-gossip implements:

- **HyParView** (Leitao, Pereira, Rodrigues, DSN 2007): a membership protocol maintaining a small active view (a connected overlay for dissemination) and a larger passive view (a reserve for repair), so no node needs global membership knowledge. This solves the "rumor mongering needs to pick random peers" problem at scale without global state.

- **PlumTree** ("Epidemic Broadcast Trees," Leitao et al., SRDS 2007): builds spanning trees over the HyParView overlay, pushing content eagerly along tree links and lazily announcing message hashes (IHave) along the rest, so the tree carries content while the mesh repairs breaks. This is the eager/lazy split that experiment E1.1 examined.

These are the established refinement of Demers rumor mongering for large dynamic networks, and they are what iroh-gossip gives us off the shelf. Drystone inherits them; it does not modify them.

---

## 3. The closest whole-system prior art: Hyperledger Fabric

Fabric is the most directly comparable shipped system, because it already decouples the routing population from the entitlement population the way Drystone does, and it is worth grounding against because the comparison both validates the shape and sharpens where Drystone differs.

What Fabric does that Drystone also does:

- A **gossip data dissemination protocol** spreads signed messages across peers, where every gossiped message is signed so Byzantine participants sending faked messages are identified and excluded. (Drystone: sealed, author-signed MLS records, self-verifying regardless of carrier.)

- **State reconciliation**: a peer behind on blocks is brought current by pulling missing blocks from peers that hold them. (Drystone: gap-aware history convergence, RBSR pull of missing self-verifying records. *State reconciliation* is Fabric's name for our fill step; we adopt the term.)

- **Channels as the entitlement boundary, larger overlay across them**: channels are segregated so peers on one channel cannot share information on another, while anchor peers let the overlay span multiple organizations larger than any single channel. *This is exactly Drystone's decoupling, a routing fabric larger than and overlapping the entitlement-scoped groups that run over it.* Fabric proves the pattern ships.

Where Drystone differs (and why it is not just Fabric):

- **No central ordering service.** Fabric has an ordering service that sequences blocks; its leader peers pull from it. Drystone refuses any central ordering (timestamp-free causal fold, forks first-class, §2.0.1 / §2.5). This is the deepest difference: Fabric is center-free in *dissemination* but center-ful in *ordering*; Drystone is center-free in both.

- **Content confidentiality to a key-agreement group.** Fabric's channel segregation is policy-enforced routing; Drystone's is MLS cryptographic entitlement (a non-member holds only ciphertext). Drystone's blind carriers are blind by cryptography, not by routing policy.

- **CA-rooted identity vs cooperative/lineage identity.** Fabric authenticates peers via a root CA; Drystone uses MLS leaves and persona lineage with cooperative governance, no root authority.

So Fabric grounds the *decoupling pattern* (routing fabric over entitlement-scoped groups, gossip dissemination plus state reconciliation) as real and shipped, while Drystone's contribution is doing it center-free in ordering and governance, with cryptographic rather than policy entitlement.

---

## 4. The reconciliation primitive: anti-entropy to RBSR

The fill step's lineage, specifically:

- **Anti-entropy** (Demers 1987): reconcile by exchanging contents and recomputing checksums until they agree.

- **Range-based set reconciliation** (Meyer, arXiv 2212.13567): recursively partition an ordered domain, compare range fingerprints, descend only on mismatch; logarithmic rounds, communication within a log factor of optimal. Modern range-based anti-entropy.

- **Deployed instances**: Willow's 3d-RBSR; Negentropy (Nostr NIP-77, strfry). Drystone's device-pool sync, D-peer, and gap-aware convergence are all instances of this primitive.

The key inheritance: anti-entropy needs *an order* to reconcile against, and Demers used timestamps. Drystone deliberately does not (timestamps order nothing that converges, §2.0.1); it uses the clock-free per-author monotonic index. That the ordering criterion is swappable is itself established (Negentropy: any monotonic criterion fitting a 64-bit integer works), so using a clock-free order is a parameter choice within the established primitive, not a departure from it.

---

## 5. What is actually Drystone's (the honest novelty boundary)

Inherited, not ours (cite the lineage, do not claim invention):

- gossip / epidemic dissemination (Demers 1987; HyParView + PlumTree 2007)

- anti-entropy / state reconciliation, and its RBSR refinement (Demers 1987; Meyer; Willow; Negentropy)

- eventual consistency as the consistency model of a coordinator-free system (universal)

- the routing-fabric-larger-than-entitlement-group decoupling (Fabric ships it)

- propagating deletion through an eventually-consistent store (death certificates, 1987; Willow tombstones)

Genuinely Drystone's (the composition and the framing):

- **The three-plane decomposition** (carriage vs durability vs presence/wake) as explicit design axes, with fusions flagged where one mechanism serves two planes, and the non-exclusive race across sources.

- **The MLS entitlement plane over a blind delivery fabric**, so carriers are blind by cryptography and the sealed message is simultaneously payload, delivery, and gap-definition.

- **Center-free ordering and governance** (timestamp-free causal fold, fork-not-verdict) combined with epidemic dissemination, Fabric and the literature keep a central ordering or CA; Drystone removes it.

- **The rights framing**: blind, revocable, redundant helper roles (relay, meer, push, fabric carriers); the no-helper floor; incompleteness-is-safe; exit finality. The mechanisms are inherited; treating them as rights constraints is ours.

- **Gap-aware history convergence as a named, unified mechanism** (high-water-mark detection that is identically the cursor, plus entitled-member anti-entropy fill, plus device-pool-first priority), assembled from inherited parts into one feature with a safety-derived source ordering.

The honest one-line claim Drystone may make: *not a new way to move messages, but a particular, rights-disciplined composition of forty-year-old epidemic dissemination and anti-entropy with modern key-agreement (MLS) and modern transport (iroh), made center-free in ordering and governance where prior art kept a center.*

---

## 6. Naming, anchored to the lineage

Following the "established names for primitives, our names only for our compositions" rule:

- Use the literature's terms for inherited primitives: *gossip / epidemic dissemination*, *anti-entropy / state reconciliation*, *eventual consistency*. These need no coinage and ground us in prior art.

- Coin only for Drystone's distinctive compositions: the blind routing overlay (the **Delivery Fabric**), and the detect-plus-fill outcome (**gap-aware history convergence**). The epidemiological register (infective/susceptible/removed, a carrier that transmits without being changed) is the established metaphor these names live in, so they read as native to the field rather than invented.

**DS / meer / DF, settled.** "Delivery Service (DS)" is the MLS-spec role (RFC 9420/9750): an untrusted store-and-forward-plus-ordering function. Drystone inherits it, refuses the ordering half (ordering is the timestamp-free causal fold), keeps the store-and-forward half, and names that kept half the **meer**. So there is no separate Drystone "DS" to coin: DS is the upstream role, the meer is our instantiation of the part we use. The one genuinely new name is the **Delivery Fabric (DF)**, the blind routing/dissemination overlay, which is a *different* concept from the meer (the fabric *carries*; the meer *holds*). A meer sits on the fabric but is not the fabric. So DS (upstream role), meer (our store-and-forward node), and DF (our routing overlay) are three distinct terms with no collision.
