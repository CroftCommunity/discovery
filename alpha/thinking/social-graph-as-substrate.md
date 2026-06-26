# The social graph is the substrate — chat is a tenant; storage, identity, and the recursive principal

date: 2026-06-26

status: thinking (new) — the synthesis layer for the social-graph-as-substrate dialogue
(`../seeds/transcripts/raw/social-graph-substrate-redb-storage-dialogue-2026-06-26.md`). Companion to
`algedonic-and-peerhood-as-adjudication.md` (peerhood = where adjudication lives) and
`historical-peer-rights.md`. Couples the Drystone protocol spec (`../../beta/drystone-spec/`) and the redb
build prompt (`../seeds/generated-prompts/redb-social-graph-layer-build-prompt.md`).

Web-searched facts here (Keet, ATProto, redb, Automerge, MLS RFC 9420/9750) are **[confirm before publish]**
— not yet in the FACTCHECK SoT.

---

## 1. The thesis: invert the pyramid

The same conflation the session corrected for Drystone-vs-Croft was running under the app design: **chat
was being treated as the bottom of the pyramid**, with games/calls/media bolted onto a thread. The
inversion: **the social graph is the bottom; chat is one tenant on top, peer to games, calls, photos,
shared activities.** The durable thing is the **group**; a chat is a time-bound surface *attached* to the
group, as disposable as the group is durable.

This dissolves the Delta-Chat pain (a game polluting a thread, un-pinnable, unmanageable long-term) because
it's a category error made structural: a thread is forced to be both the communication medium *and* the
index of everything the group ever did, and it's bad at the second. Give each activity its own surface
hanging off the shared group node: the **group is the durable index**, chat is one attached surface, the
game is a sibling not a guest. "Groups durable, chats need not be" finally coheres — when chat is just a
tenant, a chat can end while the group persists, and you can spin up a fresh chat with the same group later
with the game and photos still attached.

**The graph must be load-bearing and invisible at the same time.** Foundational as architecture, almost
entirely absent as UX — the user experiences "me and these people and the stuff we do together," never
"administering a graph." The hardest UX problem is the group's "home/face": a place you and these people
share, not a settings page for a graph node. Entry points are plural but convergent (many doors, one room).

This is an **app-layer** claim (Croft / theme 08) built on a **protocol-layer** fact (the relationship
graph is core Drystone). Keep the layers distinct: Drystone = the protocol substrate (the social graph as a
signed-assertion DAG); Croft = an app that surfaces it; redb = a local implementation detail.

## 2. Group identity, and the implicit/sticky lifecycle

- **Tree vs graph are different levels, not competing models.** A single group's *history* is tree-shaped
  (root = formation; chats/sessions branch). The *groups* form a **graph** (overlapping memberships). The
  group node sits in a graph; what hangs off a node can be a tree.
- **Group identity is NOT the member set.** Same people in two groups = two nodes. The group has its own
  stable ID independent of membership, which is what lets membership change without the group becoming a
  different group. **"Same members" is not the identity key.** Stable internal ID + a presentation name that
  is shareable but **locally overridable** ("one identity, many faces"); the ID exists from moment zero, the
  name is a nullable per-user attribute.
- **Implicit and explicit creation produce the identical object.** Start-a-chat (group forms behind it) and
  make-a-group-then-attach are two affordances for one primitive; the only difference is the moment of
  naming/registration. An implicit and explicit group **must be indistinguishable once they exist.**
- **"Sticky" resolves the privacy tension.** Every group has a stable ID and accrues history, but only
  **sticky** groups are *matchable* for reconciliation ("oh, it's us again"). Three lifecycle states:
  **sticky** (persistent, surfaced, matchable) · **live-but-non-sticky** (real, accrues, not matchable,
  prunes when quiet) · **pruned** (deliberately ended, never resurrected — reforming the same people is
  definitionally new). Reconcile-vs-fresh-vs-prune is a **per-formation human choice, never forced** — the
  member set *suggests* a match, never forces one (critical for family-safety: "the system grouped me back
  with someone" is the surprise to avoid).

## 3. The seam: local projection vs shared anchor

**Presentation and association are local; access and cross-participant group identity need a shared anchor.**

- **Local (no consensus):** naming, stickiness, local reconciliation, personal presentation.
- **Shared (needs an anchor):** **membership changes**, **pruning/"never resurrect,"** and **new attachments
  landing as "the same group" for everyone.** The thin shared layer is **stable group IDs participants
  recognize + the event/artifact log.**
- **Membership must be shared — but shared membership ≠ shared access.** Membership = consensus about who is
  *currently* in, going forward; you still **cannot retroactively revoke what someone already holds** (local
  removal ≠ global revocation). The family-safety failure is believing removal did more than it did. Authority
  model: **signed membership changes from a designated authority, a CRDT-flavored membership log folded to
  "current membership"** — not full consensus, not always-online voting.

## 4. Storage architecture: authoritative DAG + derived projection (local-first CQRS)

- **No global graph exists anywhere.** Each node holds its own view assembled from independently-synced
  pieces; the union is never materialized. "Large graphs" never materialize — you assemble a local one from
  shards, bounded by participation.
- **Two layers, never collapsed:** (1) **authoritative** = signed assertions / CRDT documents (source of
  truth, synced, access-gated, optimized for sync+integrity); (2) **derived index** = a rebuildable
  projection (fast traversal/query). CRDT docs are bad at query; embedded DBs are bad at sync — use each for
  its strength, rebuild the index from the source.
- **Relationships, not entities, are the Merkle tree.** Groups/games/notes are payloads; the
  **relationships** (split-from, attached-to, member-via) form a signed, hash-linked, append-only **Merkle
  DAG** with provenance. Store relationship **assertions** as immutable signed hash-linked entries; let a
  CRDT (Automerge) be the **grow-only set** that accumulates them — union of immutable hash-identified
  assertions is the trivial conflict-free CRDT. Each node's set is the assertions it learned through its
  group history → each reconstructs a **differently-shaped but locally-canonical subgraph**, with **exact
  agreement wherever two views overlap** (content-addressed + signed → bit-identical). Provenance needs
  **signatures, not just hashes** (hash = integrity; signature = authorship). "Leaving" is a superseding
  assertion, not a deletion.
- **Automerge mapping:** the **group is one thin Automerge document** (stable ID, membership log, an index of
  attachments *by reference*) — references, not contents. **Each attachment is its own document or blob.**
  Separate documents for sync granularity, access scope, and lifecycle. **Automerge is not a graph DB —
  references are unmanaged; the graph is maintained on top.** Unlinking ≠ deleting.
- **Governance log + declarative snapshot + verifiable roll-up** (event sourcing): the **governance chain is
  the imperative log** (ordered, append-only, source of truth, never replayed hot-path); the **declarative
  snapshot** is a materialized fold carrying its `computed_at_gov_head`, **valid iff that head == the current
  governance head**, never synced, never trusted from a peer, recomputable by anyone. It holds "latest values
  that passed authorization at each step" (the log is self-validating under replay). **You forward the
  imperative chain and meet on the declarative — never blind trust; threshold logic is authorization, never
  truth.** **Roll-up = a signed checkpoint committing to (governance head hash, state commitment)** — because
  the head is hash-linked, the truncation is **re-expandable and back-verifiable** ("verifiable truncation,"
  not "trusted summary"). **Option A:** each peer independently folds and self-checkpoints; co-signing is
  *corroboration of independent identical folds*, never a substitute for local validation; liveness never
  gates on a quorum. **Compaction settled at genesis, two tiers:** governance permanent/uncompacted (the
  spine a dormant node needs to reconstruct the authorized signer set), content compactable into
  head-committed Merkle-root checkpoints. **Built-in but off by default** (SSB lesson: design forgetting in or
  it calcifies); catch-up never *depends* on it.

## 5. redb's role (local-implementation, not protocol)

redb is the local **derived-state engine**: the materialized governance snapshot + the graph adjacency index
as typed tables; **MVCC single-writer/many-readers** so UI reads never block the folding writer;
**constant-time savepoints = rollback checkpoints** for re-folding when a causally-earlier event arrives.
The authoritative signed log lives durably; the redb projection is a fast rebuildable cache, **never synced,
never trusted from a peer.** Blobs live in iroh-blobs by hash, never in redb (B-tree bloat). The full,
vetted build is specified in the build prompt; the frozen-vs-fluid boundary is the discipline that lets the
UI iterate without breaking peers. **This is "complementary enabling ecosystem stack + local tech stack" in
the user's four-layer model — not the peer protocol.**

## 6. One identity system; the recursive principal; composition vs valuation

- **One identity/capability system, not two** (two needs a binding layer = the confused-deputy / "I thought I
  removed them" failure at the architecture level). **One identity layer, many scoped capabilities;** the
  membership record is the single authoritative recipient list that makes revocation (= key rotation)
  well-defined. **Access is a key-management model.** A **stable identity key** (what membership/signatures
  refer to) is separate from **rotating content keys.** iroh node identity is **per-device plumbing beneath**
  the per-user identity — never the member identity.
- **Every group is a group of groups.** A user is a group of devices (base case, shared key lineage); a
  community is a group of users; **one recursive principal primitive.** A dormant device rejoins its **user
  group** (small, trusted, sibling-device ack), and community membership flows through the user principal —
  rejoin at the lowest recursion level with the context to decide it; history-fill is independent best-effort
  sync.
- **Two distinct edge types over the one primitive** — keep crisp or trust leaks into key access:
  **composition** (shared MLS lineage / authoritative-state-merging — device-pool-in-user-in-community) and
  **valuation** (directional, weighted trust between cryptographically-separate groups — no shared keys).
- **Adversarial posture is a per-edge property, not a global stance.** Device pool = high-trust/low-
  adversarial; stranger valuation = low-trust. Forcing Byzantine rigor on your own device pool is as wrong as
  omitting it from a stranger edge.

This is the load-bearing refinement to `P-Peer-Equality` and to peerhood-as-adjudication: **you must define
the peer (a locus of adjudication) before peer rights, and the peer is recursively a group.**

## 7. MLS multi-device: devices are leaves; user-principal as self-AS

**[confirm before publish — MLS RFC 9420/9750]** MLS **does not nest** (leaves are clients; a leaf can't be a
group's root) and **forbids duplicate signature keys in a group** → multi-device = **multiple leaves, one per
device, each its own key**, with "these leaves are the same user" left to an **application credential policy**
(RFC 9750 names this). So:

- **Cryptographic layer:** every device is an **independent MLS leaf** (per-device PCS + revocation; cost:
  tree sized by device count, device add/remove = group-wide epoch churn). Your first instinct, mandated.
- **Identity layer:** the **device-pool-as-principal is real** via credentials attesting "I'm a device of
  user-principal P." MLS assumes an **Authentication Service** but says little about it because it's not a
  cryptographic question — in serverless P2P the **user-principal key is its own tiny CA signing its device
  credentials** (user-principal-as-self-AS; mls-rs custom credentials is the hook). Your second instinct,
  living at the right layer.
- **This resolves the lamport seam:** assertions are signed **per-device** (`author_device` +
  `author_principal`), **lamport is per-device** (no cross-device coordination, no collisions), and the
  principal's stream is the **deterministic fold-time merge.** Storage and crypto agree.

**Forward secrecy constrains the live ratchet keys, not retained history.** Once decrypted, plaintext is
yours to keep; history is local, shared at discretion as **range-sync over crypto ranges**; a group you
weren't in is just history you may request or forgo (completeness vs isolation, both legit). MLS concurrent
commits from one epoch = a **fork by construction**; undesired forks self-heal by deterministic tiebreak in
the retention window; desired/unresolved forks become independent forward-secret groups reconciled by a fresh
social act.

## 8. Trust philosophy — provenance is certain, trust is a social/utility call

- **MLS factors out trust-establishment and only solves trust-expression.** **All trust roots in social
  trust, even the root CAs** (in your browser because humans vouch for and police them). **"Whether a key is
  really Anna is a utility call, not a provenance one."** Crypto answers provenance (same key as before) with
  certainty; "is this key the person I mean" is **always a human judgment.** A QR-scan doesn't *establish*
  trust — it **binds existing trust to a key** so crypto can carry it forward. Pretending crypto *establishes*
  rather than *records/amplifies* the human decision is itself a failure mode.
- **Web of trust failed because PGP made transitive trust automatic and scalar.** Trust is **indexed by
  purpose ("to what")** and is **non-transitive.** So **vouching is a graded, contextual, non-transitive
  signal whose strength must match the recognized utility — input to a human/policy decision, never a
  computed verdict.** Both roads coexist: **direct deliberate acts** (QR scan, conference key-exchange,
  mutual-follow) are the high-strength cryptographic anchor; **ambient vouches** are honestly-low-strength
  contextual signals. **The moment a vouch becomes an automatic grant, you've rebuilt PGP's mistake.** The
  `vouch` assertion therefore *requires* a context + a strength, and `get_trust_signals` returns signals,
  never a verdict.

## 9. Prior art (web-searched, [confirm before publish])

- **Holepunch Pear/Keet** — closest: apps *into a conversation*, P2P, private-by-default, Hypercore + Hyperbee
  + Hyperswarm. **But room/conversation is the substrate, not the social graph** (chat-bottomed + plugins).
  Validates "activities as siblings"; the wedge is *does group identity outlive a room.*
- **AT Protocol** — shared identity/graph, many app surfaces via lexicons. **But public-by-design** and the
  graph is the **public follow graph**, not the durable small private group. Nails "shared substrate, many
  surfaces"; wrong privacy + graph shape.
- **No one** has put the durable private group with shared membership/governance at the bottom and hung
  chat/games/calls/photos off it as co-equal siblings, graph load-bearing but invisible. **That gap is the
  wedge.** (Candidate ECOSYSTEM.md additions: Keet/Holepunch, the Keet-vs-this contrast.)
- **Gun / OrbitDB / SurrealDB / Fluree / Veilid** — concept-adjacent but tool-wrong for the iroh/Willow/
  BLAKE3/Automerge stack; study as references. **redb + iroh-blobs + Automerge over iroh** is the fit.

## Where this lands

- **Drystone spec (protocol):** Part 1 — trust-vs-provenance (under the razor) + the recursive principal
  (composition/valuation edges, extending peerhood-as-adjudication). Part 2 — per-device authorship + lamport
  + user-principal-as-self-AS credential model + devices-as-MLS-leaves; declarative-snapshot-as-cache +
  verifiable roll-up/compaction sharpening §7.
- **Croft app (theme 08):** the social-graph-as-substrate reframe (chat as tenant, attachments as siblings,
  sticky-group lifecycle, the invisible-graph UX) — staged as an OPEN-THREADS reframe, not a unilateral 08
  rewrite.
- **Local implementation:** the redb build prompt (vetted, adaptable, staged).
- **ECOSYSTEM.md:** Keet/Holepunch + the chat-bottomed-vs-graph-bottomed contrast (candidate).
