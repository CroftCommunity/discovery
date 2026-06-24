# Local-first as design imperative: the architecture that follows from one premise

**Context:** the 2026-06-20 architecture dialogue walked Croft from storage-constrained-peer tiering up
through blind peers, search modes, the delegate primitive, the functional-planes model,
governance-as-substrate, and BGP/postal/DNS-style federation — and found that **all of it is a single
premise (local-first state) followed all the way up.** This doc distills the *system/protocol* design
that falls out. It is the architecture companion to the civic/epistemic "why" in
[`../narrative/lineage-of-a-design-imperative.md`](../narrative/lineage-of-a-design-imperative.md) and
the settled principles in [`../crystallized/principles.md`](../crystallized/principles.md). Raw:
`../seeds/transcripts/raw/croft-architecture-design-dialogue-2026-06-20.md`.

**Status:** design synthesis. Cross-field grounding is verified in the dialogue (Mill/Hayek/Ostrom/
Ashby/Beer/Scott verbatim; Kleppmann BFT-CRDT & equivocation papers; commons-DAO research). The three
genuinely-new seams (below) are unbuilt and want their own threat models before code. Distinct from
`app/design-philosophy.md`, which is the *application/client-layer* philosophy; this is the
*protocol/substrate* philosophy.

---

## The generative premise

**Local-first state is the single seed; everything else is it taken seriously and followed up.** The
premise — *the primary copy lives with the unit, complete and usable alone; the network reconciles
between units rather than constituting them* — is the same sentence as the epistemology: *truth is
local and corroborated across peers, never certified from a center.* One is the engineering statement,
the other the epistemological statement. So the architecture and its own justification run on the same
engine, which is why it is internally cohesive: there was only ever one thing being said.

It propagates:

- **Identity** is local-first state — your DID is yours, lives with you, carried when you move. This is
  the **rights-floor**: you can't be cleared because your standing isn't held elsewhere.
- **History** is local-first state — the hash-DAG you hold; verify your copy locally, *reconcile* (not
  certify) across peers. Provenance-not-utility falls out.
- **Governance** is local-first applied to consent — standing/vote/share is state you hold, not
  permission granted. **Fork is local-first's native move** when peers disagree: two primary copies
  that stopped reconciling (exactly the DAG's multiple-heads).
- **Federation** is local-first at collective scale — the collective holds its own primary state,
  reconciles with peers, **cannot be reached into**.
- Even **resolution** (DNS) is local-first applied to dependencies — the addressing scheme is primary
  state, the resolver a secondary copy you reconcile through and can replace.

The two-part theorem the dialogue lands: **(1)** a system that respects humanity *must* be local-first,
because the person is the unit of composability (central primary state has already made the person
secondary — the architecture encodes the disrespect before a feature is built); **(2)** central-truth
design is *necessarily* faulty, expressing as permanent **friction**, because by requisite variety
(Ashby) the center is always below the variety it governs (much of it inarticulable — Hayek — so not
even transmittable to a center), and must force reality down to fit. **Corollary: friction is
diagnostic** — honest friction (real disagreement between real units, which should feel effortful) vs.
manufactured friction (a center forcing variety it can't hold). Necessary but **not sufficient**:
local-first is the required *form* of a humanity-respecting system, not a guarantee of one (the edge
can be wrong too — variety preserved ≠ humanity served).

## The storage / blind-peer / search substrate

- **Storage tiers for constrained devices:** ring buffer (predictable ceiling) with a soft time-hint;
  cap on both message-count and time, evict on whichever triggers first. Long-history illusion via
  **store-the-skeleton, stream-the-content** — keep root hashes / a sparse DAG skeleton, fetch leaves
  on demand and verify each against the held hash, so the larger peer is never a trust anchor.
- **The merkle unit:** chunk messages into **epochs/blocks** (shallow tree, efficient range-sync,
  "last N days" a natural block boundary), not per-message.
- **Membership model is per-group, not global:** fixed/known (roster/gossip locator, assigned pinning,
  eviction safe at replication target) vs. open/churning (DHT provider-record locator,
  replication-factor + re-replication). Make the merkle/DAG layer identical; push the difference into a
  pluggable **locator** and **durability policy**. A node holds **per-group** policy state; the
  cross-group storage allocator (what to evict when a fixed-group pin and an open-group cache contend)
  is the hardest real engineering — sketch it before the network layer.
- **Blind peers** shuffle encrypted packets and see the hash tree, not payloads — a sealed-sender relay
  that also exposes the skeleton. They answer **structural/locator** questions, not content, unless you
  add encrypted search (deterministic = leaks equality at rest; SSE = leaks access patterns in motion;
  neither is zero-knowledge — *you pick a leakage profile, not avoid one*).
- **Search is a bounded subtree scan** (origin head, termination cutoff/depth, predicate). The hash
  tree **is the shard map** — partition along child-subtrees, scatter to workers, and the gather is
  **cryptographically attestable** (each worker returns matches + the subtree root hash; the
  coordinator already knows what that root should be, so coverage is a checkable set-cover over hashes).
  **Fidelity is per-plane:** best-effort for chat, provable coverage for action/audit. Capability gates
  *completeness*, not *correctness* — a branch with no responsive worker is explicitly partial, named
  by missing subtree hashes (honest framing on limits).

## Discovery and fulfillment as layers; the search chooser

- **Discovery answers *where*** (locator plane, blind-safe): given query + branch head, which peers can
  serve and at what tier. **Fulfillment answers *give it to me*** (content plane): hydrate, decrypt as
  entitled, search, verify against the root.
- **Mode is an outcome, not a mode:** a search is discovery returning candidates + a fulfillment policy
  choosing among them. The chooser's **preference order per group** is the whole search security posture
  as one ranked list, e.g. `local → member-with-own-copy → enrolled-mediator(ephemeral) →
  enrolled-mediator(standing) → structural-only`, taking `(query, head, group-policy)` →
  `(fulfiller, key-mode)`, so fork/merge is just another input.
- **The two offload animals:** an **HA search member** has its own decryptable copy (decrypts with its
  own keys — no new exposure, the safe one) vs. a **pure search mediator** with no copy (must be
  *enrolled* with decryption it wouldn't otherwise have — the crown-jewel target). **Design bias:
  prefer HA members; fall back to mediators only when none available.** Offload (holder online) → key
  can be ephemeral; availability (holder offline) → key must persist → standing exposure, shorter
  rotation, membership-epoch revocation. Availability is strictly more dangerous; don't price it like
  offload.

## The delegate primitive: capability vs authority

The dialogue's central protocol abstraction. **Capability = "I am able to do this work"; authority =
"I am permitted to act as you." Never conflate the two in a single grant.** An offload peer acts as
itself (capability); a **delegate** stands in for an absent principal, carrying a slice of its
*authority*. Model a delegate as **a (predicate, sealed-payload) pair held by a peer or threshold**,
emitting the payload when the predicate fires — one primitive, three knobs:

- **trigger** (time / event / peer-online / quorum-attested condition),
- **threshold** (1 for capability; k-of-n for authority; k-of-n *across independent trust domains* for
  recovery/C2),
- **attribution** (the emission names its trigger and its delegates, for audit).

The safety rule: **pre-seal the payload so the delegate carries no abusable authority; delegate only
the trigger** (dead-man's-switch / timelock / escrowed pre-signed action). Only actions whose content
depends on *future* state can't be pre-sealed — those need live authority and full threshold treatment
(the courier-vs-agent boundary: decide it before building, it sets the blast radius). Payloads unify
chat escalation, recovery (a sealed key share), C2 (a signed order), failover (capability assumption),
and revocation (kill-switch). **Catalog by exposure:** capability-only (search/locator proxy, relay,
re-replication — can outlive the principal, work for orphans) vs. authority-bearing (ack-on-behalf,
merge-vote-as-principal, key custody, membership actions — require a *living* principal to root, want
thresholds). **Invariant: capability delegation can outlive the principal; authority delegation cannot
exist without a living principal. Liveness is the boundary** — which is why search, discovery, and
group-liveness are separately considerable.

**Recovery is two tiers (keep apart):** Tier 1 **the lock** (mechanism — Shamir/threshold shares,
sealed, release predicate, optional timelock — *buildable now* with the predicate pluggable) and Tier 2
**the trust** (who/when release is legitimate — the social problem, genuinely unsolved-in-general; the
TLS cert-chain-vs-issuer-verification analogy). Ship Tier 1 with the predicate as an interface; the
predicate should be **threshold across independent trust domains** (social peers AND ≥1 channel
attestation — DNS/OOB), never a single gate. Recovery custody is the highest-value attack target you
can build.

## The functional-plane model

A **plane** is a functional grouping that owns four choices — **fidelity** (best-effort vs. provable),
**trust** (capability / authority / threshold), **durability/ordering**, and **delegation namespace** —
over a shared substrate. **Planes are equivalence classes of blast radius:** what separates chat,
scheduling, OOB, action/C2, audit, governance is *the consequence of being wrong*. Low-stakes →
best-effort, loose order, capability delegation; high-stakes → provable coverage, strict order,
threshold authority, full attribution; catastrophic (recovery) → threshold-across-domains.

- **Planes are anchored to principals, not groups** — a principal is any identity that can hold and
  delegate authority (person, device, group, role, service), forming a **hierarchy** where delegation
  flows *down* (a device is subordinate to its owner). A delegation is scoped to **(principal, plane)**;
  a compromised token leaks **one cell of the (principal × plane) grid**.
- **The load-bearing invariant: namespace delegations never cross.** Bind every token to its plane's
  namespace as a non-removable caveat (macaroon caveats / ocap scoping), verified at emit time;
  cross-plane reuse fails closed by construction. The substrate is shared; the namespaces are not.
- **The confused-deputy seam:** an emission names two principals (originating + delegate); a verifier
  walks `originating signed payload → token authorizes this delegate for this plane → delegate signed
  emission`, all three or invalid — and must resist rearrangement when principal *types* mix.
- **Reconvergence policy is per-plane (asset-overridable), declared at intent-to-collaborate, bound
  immutably into the asset's hash.** This resolves the Kleppmann tension: SEC auto-merge for incidental
  concurrency (chat/docs); human-gated reconvergence where divergence is substantive disagreement
  (governance/action). The substrate can't tell concurrent-typing from fundamental-disagreement at
  merge time — only the *declared meaning* can — and the policy must be as non-equivocable as the
  content, else a Byzantine peer equivocates on the policy itself.

## Governance is the substrate; chat is one plane

**Chat is one plane of record and asset; governance is the actual substrate** — a set of rules bound to
principals to act in concert under mutually-agreed governance. A **group-principal is a threshold of
members** (a group can't sign; FROST-style threshold sigs make it externally one key, internally a
governed quorum — so every group-rooted delegation is a threshold operation by nature).

- **The dials are governance; the meta-dial is the hard part.** Setting thresholds is configuration;
  **changing them is governance-of-governance, and the meta-rule must dominate the rules it can alter**
  (the threshold to change a dial ≥ the strongest authority that dial gates — else a privilege-
  escalation path). Recovery/C2 dials get the highest change thresholds; chat dials can be cheap. Steal
  the **timelock** between decision and execution wholesale (defense against a captured quorum).
- **Three collision seams:** governance changes are themselves forkable (so **the governance plane must
  be the strictest plane** — never best-effort); threshold membership × threshold signing needs
  **proactive secret sharing / dynamic resharing** so shares survive membership change; **nested
  group-principals** (federation) make the meta-rule-dominates invariant hold *transitively*.

## Detection, dissent, and the rights-floor

- **The four substrate guarantees** that make the social overlay viable: **completeness**,
  **attribution**, **non-equivocation**, **cheap re-association**. Tampering is defeated by hashes;
  **equivocation** (different internally-consistent views to different observers) is *not* — it's
  detectable only by comparing across observers, so the defense is **gossip** (CT/CONIKS head
  cross-checking): structurally social, working only when honest peers stay in contact (also exactly
  when a community can govern itself). iroh-gossip plausibly suffices at modest private scale; verify
  eclipse-resistance for open/large.
- **The system computes provenance, never utility.** Detection surfaces a legible, corroboration-
  weighted divergence ("Tom's head diverges from N peers" — fact); it must not assert "therefore Tom is
  wrong" (utility). "Bye Tom" is the human's call, and **the fork stays available as the dignified exit
  for the Tom who isn't wrong, just different** — the refusal-to-decide is what stops majority-tyranny-
  by-protocol. *The walls move for Tom too.*
- **The one principled boundary: no right to remove the rights of others.** Variety is permitted in
  everything except the removal of another's standing to hold variety — a fork creates, a clearance
  destroys. **Rights** (standing — tenure, exit, voice, share — the precondition of a legitimate
  collective) are not the collective's to remove; **roles** (governed delegation) move freely. The
  **wolf test:** any action that, generalized, would remove the conditions of its own contestation is
  illegitimate by nature (the tell is self-cancellation). Rights-removal is **the only self-amplifying
  move toward collapse** (it lowers the variety available to resist the next removal — the monoculture
  mechanism in a polity), so it's a **consistency requirement and an equality requirement, not a moral
  overlay**. **Inverse-correlation:** where contestation is cheap, get out of the way (conditions
  self-defend via exit); where decisions are **irreversible/singleton-bound**, exit can't heal them, so
  the constitutional rigidity bites hardest. *Maximal freedom where exit protects you; maximal
  protection of rights where exit cannot.*

## Federation: equal rights generate variety of form

- **The collective is the peer primitive at a larger scale**, and its internal form is a **free
  variable** (household / co-op / foundation / township). Equal-in-rights means **all shapes are
  equally available** — a system that mandated one shape would smuggle a monoculture in at the
  structural level. *The architecture in one sentence: equal in rights, not in capabilities — applied
  not to a shape, but to the equal possibility of all shapes.* The rights-floor recurses: no peer clears
  a peer, no collective clears a member's exit, no federation clears a member-collective's fork.
- **Inter-collective peering = BGP autonomy + postal hierarchy + cryptographic trust, as a reflection
  of social relationships, not in demand of them.** Each collective is an autonomous system (sovereign
  interior, one external peering face, local policy, path-vector visibility). **Hierarchy beats the
  flat O(N) routing-table wall:** a federation advertises itself as one aggregable entity and resolves
  its interior privately (private-to-private neighborhoods); the **address is the path through the
  social hierarchy** (postal/DNS recursive delegation). **Advertise aggregates, resolve specifics** —
  A holds one bounded entry per peer collective and routes a resolution request *down* a path encoded
  in the name; A never enumerates B's members. **Per-node state O(peers-you-know + depth), independent
  of N; resolution O(log N); the global directory is never materialized.**
- **Identity vs locator:** identity is flat/permanent/cryptographic (the DID, the rights-floor); locator
  is hierarchical/routable/mutable (`x.householdB.coopfederation`) — move household, identity unchanged,
  only locator updates. Discovery is OOB through the social tie (no global directory — no contact
  without a relationship that already conveyed the address).
- **Why AP fractured:** it federated on the *operational server* (an instance), not a *social
  collective*, and let interior conventions leak across the seam — so delivery happens across boundaries
  that don't match social edges. The fix: **route to the collective, never into it** (boundary handoff;
  the interior stays sovereign and invisible, so there's no interior standard to fracture).
- **DNS is a reusable-but-not-binding resolver, not an identity root** — atproto already separates the
  handle (reassignable pointer) from the DID (identity). Keep the **addressing scheme** permanent and
  independent; the **resolver** is swappable (DNS today, opt-in **recursive workers** collectives run
  later, decaying the DNS-root dependency collective-by-collective with no flag day). *Don't depend on
  the center in a way you can't later remove.* When peer-native, resolution responses are DID-signed by
  the authority owning that hierarchy level, so the resolver **inherits non-equivocation** rather than
  needing a DNSSEC-style bolt-on.

## What's new (scrutiny goes here) and what to lean on

**Lean fully on prior art:** the merkle/DAG with fork/merge (Git, Automerge/Yjs), MLS (RFC 9420) for
group keying, DHT/gossip discovery (Kademlia/IPFS), sealed-sender relay (Signal), Shamir/threshold
custody (FROST), ocap/macaroon/SPKI token design, dead-man's-switch/timelock (tlock/drand) for
conditional actions, DIDComm mediators for offline-principal delegation, Willow for prunable stable
content. Shape-relatives to study: Secure Scuttlebutt, Matrix state-resolution (the merge-semantics
prior art *and* its bug history), IPFS/IPLD, Briar; RINA / Named Data Networking / Yggdrasil for the
recursive-federation routing.

**Genuinely new — write each up as its own threat model before code:**
1. **MLS-style epoch keying under P2P fork/merge with permanently-offline members** — `(epoch
   revocation) × (fork) × (offline delegate holding a pre-fork grant)`. MLS doesn't fork; most likely to
   hide a security-fatal bug. Primary research.
2. **The delegate unification across chat + recovery + C2** — especially whether C2/live-authority
   breaks the courier abstraction. Decide courier-vs-agent before building.
3. **Content-predicate search-coverage attestation** — structural coverage is solid; proving honest
   *plaintext* evaluation (didn't skip matches after decrypting) is the hard, possibly-defer-able gap.

## Open frontiers (surface, do not resolve — see ROADMAP_TODO)

- **Where a centerless federation meets a center-demanding world** (legal entity, money, name registrar,
  the scaling relay) without quietly growing a center at the seam. The largest-clothes version of the
  irreversible-singleton frontier; arranged to be deferrable/excisable, but *deferred, not solved*.
- **Governance at scale** — representative quorum vs. the cheap-fork sybil defense getting expensive;
  likely direction subsidiarity + liquid delegation (instantly-revocable weight), flagged open.
  Liquid-democracy's default failure is concentration (the Pirate Party lesson) — the antidotes (decay,
  caps, bounded chains, expiry, visibility) are the existing scoped-revocable-grant + transparency
  machinery, but whether a group stays fluid is a utility question the social layer owns.
- **Forward-only revocation under irreversible commitments** — revoking consent can't rewind a spent
  check; decisions must be tagged reversible-or-committing at decision time, and the record permanently,
  honestly attributes which consent supported which irreversible consequence.
- **The duty-of-care the center used to carry** (durability, recoverability) must be *re-homed*, not
  vanished — the delegate/recovery machinery is where it goes without re-centralizing standing.
- **Capture-detection + reconvene-coordination** — the residual that makes "resist like water" actually
  work; transparency (the four guarantees) is what lets honest constituents see capture and re-coordinate
  cheaply; the deterministic-fork-target Schelling point degrades against a *gradual* (slow-capture)
  adversary, which is the governance-plane frontier.
