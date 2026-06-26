# Social-graph-as-substrate / storage architecture / redb dialogue (claude.ai, 2026-06-26)

> **Fidelity caveat (§4).** Cleaned-paste — content-faithful, **not a byte-pristine export**. UI render
> chrome ("Searched the web", "Fetched:", "Show more", timestamps, citation widgets) stripped; web-citation
> sources condensed into per-claim notes. The dialogue's arc, conclusions, and the load-bearing framings
> are preserved. This is the second transcript handed off "to benefit from the current context" (the
> Drystone-spec session). The comprehensive **redb build prompt** the dialogue produced is filed separately
> at `seeds/generated-prompts/redb-social-graph-layer-build-prompt.md`.
>
> **Verification disposition (carried forward):** the web-searched facts here are **not yet in the FACTCHECK
> SoT** — **[confirm before publish]**: Holepunch/Keet (apps-in-conversation, Hypercore/Hyperbee/Hyperswarm);
> ATProto (shared identity/graph, public-by-default, lexicons, the app ecosystem); Gun/OrbitDB/SurrealDB/
> Fluree/Veilid characterizations; redb 3.x API (copy-on-write B-trees, MVCC single-writer/multi-reader,
> constant-time savepoints, multimap value-must-be-orderable); Automerge (per-document sync unit, no
> secondary indexes, delete-is-local-only); MLS RFC 9420 (leaves-are-clients, no nesting, duplicate
> signature keys forbidden, AS assumed); MLS Extensions draft + RFC 9750 multi-device credential policy.
> The MLS facts align with the corpus's existing MLS understanding but should still be primary-confirmed.

---

## The thesis (user)

The user had been conflating chat with the underpinning, the same way the session corrected Drystone-vs-Croft.
The realization: **the "equal in rights, not capabilities" social graph is the actual core of Drystone**, and
once separated from chat, concrete thinking became possible. **Groups may not be a "pond" (app group) at all
but the substrate under how all social interactions are structured — chat is one attachment to that social
graph; games, calls, photos are siblings.** A much more versatile, cohesive view of how humans actually
interact and build history. Crucially: being able to spin up a new chat with the same members because the
**functional group is distinct from the chat group but determinate** is safer than chats living forever and
coupling membership and governance by nature.

## Group vs chat; graph vs tree; identity ≠ member set

- **Separate the durable social group from the time-bound chat surface.** Most apps conflate them (the chat
  *is* the group, so membership/governance/history die with the thread). Decoupling makes governance and
  membership durable and communication disposable. Spinning up a new chat with the same members works
  because **membership is a property of the group, not the chat.**
- **Tree vs graph (the tension named):** a single group's *history* is tree-shaped (one root = formation,
  chats/sessions branch off). But the *groups themselves* form a **graph** (overlapping memberships, the same
  person in family and climbing group). These are different levels: the group node sits in a graph; what
  hangs off any one node can be a tree. Don't pick — they're different layers.
- **Group identity is NOT the member set.** Same five people in two groups = two distinct nodes, each with
  its own history. The group has its own identity independent of current membership, which is what lets
  membership change without the group becoming a different group. **"Same members" can't be the identity
  key.** Stable internal ID, separate from a presentation name that is shareable but **locally
  overridable** ("the group has one identity, many faces"). The ID exists from moment zero; the name is a
  nullable, per-user attribute.

## Implicit vs explicit; sticky groups; lifecycle

- **Both implicit and explicit creation produce the identical underlying object.** Start a chat → a group
  forms behind it (implicit); or make a group then attach a chat + game (explicit). Same primitive, two
  affordances; the only difference is whether it's *named/registered* at creation or lazily after. **An
  implicitly-created and explicitly-created group must be indistinguishable once they exist** (same
  capabilities, governance, ability to spawn attachments) — the only thing that ever differed was the moment
  of naming.
- **"Sticky" resolves the privacy tension.** Every group has a stable ID and accrues history, but only
  **sticky** groups are eligible to be *matched against* when you add people (reconciliation candidates). An
  unsaved non-sticky group is real while live but invisible to reconciliation — it can't pull you back into
  old history because nothing looks it up. Three lifecycle states: **sticky** (persistent, surfaced,
  matchable), **live-but-non-sticky** (real, accrues history, not matchable, prunes when quiet), **pruned**
  (deliberately ended, never resurrected; reforming the same people is definitionally new). Most casual
  interaction lives and fades in the middle without the user curating.
- **Reconcile vs fresh vs prune is a per-formation choice, never forced**, because identity is a stable ID,
  not the member set. The member set *suggests* a match; it never forces one. Keeps the human in control
  (critical in family-safety: "the system grouped me back with someone" is the surprise to avoid).

## Local projection vs shared anchor — the seam

- **Presentation and association are local; access and cross-participant group identity need a shared
  anchor.** Naming, stickiness, local reconciliation, personal presentation = purely local, no consensus.
  But three things are inherently relational and can't be unilateral: **membership changes** (local removal
  ≠ global revocation — you can't claw back what someone already holds), **pruning/"never resurrect"** (binds
  only you unless there's a shared signal), and **new attachments landing as "the same group" for everyone**
  (needs a shared group-ID reference). The thin shared layer is: **stable group IDs participants recognize +
  the event/artifact log.** Everything richer is local interpretation on top.
- **Membership must be a shared construct** — but **shared membership ≠ shared access.** Membership is
  consensus about who is *currently* in the group going forward; you still cannot retroactively revoke what
  someone already holds. The dangerous family-safety failure is believing removal did more than it did
  ("X removed and won't see anything new" is true and sayable; "X can no longer see what was already shared"
  is generally not). Authority model: **signed membership changes from a designated authority, propagated as
  events, each client converging on the current set** — a CRDT-flavored membership log (changes are signed
  ops, merge deterministically, "current membership" is a fold). Not full consensus; not always-online voting.

## Storage architecture — thin substrate, fat attachments, derived index

- **Don't store one big graph; store this node's local view assembled from independently-synced pieces.**
  There is **no global graph anywhere** — each node has its own, the union is never materialized. "Break up
  large graphs" dissolves: you only ever assemble a local one from shards, naturally bounded by participation.
- **Two layers you must not collapse:** (1) **authoritative pieces** = the CRDT documents / signed
  assertions, the source of truth, synced, access-gated, optimized for sync+integrity not query; (2) **a
  derived local query index** = a throwaway projection materialized by reading the documents, optimized for
  fast traversal, rebuildable at any time. **CRDT documents are bad at query; embedded DBs are bad at sync** —
  use each for what it's good at, rebuild the index from the source (local-first CQRS).
- **Automerge mapping:** the **group is one thin Automerge document** (stable ID, membership log, presentation
  metadata, an **index of attachments by reference**) — it holds references, not contents. **Each attachment
  is its own document or blob** referenced by hash (chat = its own doc, game = its own doc, photo = blob).
  Separate documents because of **sync granularity** (a noisy chat doesn't re-sync the whole group; large
  histories are slow and must load into memory), **access scope** (a document is the unit you can grant/
  withhold), and **lifecycle** (end a chat by unlinking it; group persists). **Automerge is not a graph DB —
  references are unmanaged; the graph is something you maintain on top.** Unlinking ≠ deleting (the child
  still exists in others' repos — storage-layer echo of "local removal isn't global revocation").
- **Relationships, not entities, are the Merkle tree.** Groups/games/notes are payloads; the **relationships
  between them** (split-from, attached-to, member-via) form a hash-linked, signed, append-only **Merkle DAG**
  with provenance. A split is a node with the parent as hash-ancestor; a halted/contested merge is a fork in
  that tree, and that's a valid permanent state. **Store relationship *assertions* as immutable signed
  hash-linked entries; let Automerge be the grow-only set (G-Set) that accumulates them** — union of
  immutable hash-identified assertions is the trivial conflict-free CRDT. **Each node's set is the assertions
  it learned through its group history**, so each reconstructs a differently-shaped but **locally-canonical**
  subgraph; where two peers' views overlap they agree *exactly* (content-addressed + signed → bit-identical).
  Provenance requires **signatures, not just hashes** (hash = integrity; signature = authorship). "Leaving"
  is a new superseding assertion, not a deletion (grow-only, immutable; current state is a fold).

## Governance log (imperative) + declarative snapshot (cache) + verifiable roll-up

- **Event sourcing with a materialized snapshot.** The **governance chain is the imperative log** (ordered,
  append-only, source of truth, never replayed hot-path). The **"latest" is the declarative snapshot** —
  current membership/roles/rules — derived and materialized for O(1) reads, carrying a pointer to the
  governance head it was computed from. **The snapshot is never authoritative, never independently writable,
  never synced as truth, never trusted from a peer** — a pure deterministic fold of the log, valid **iff its
  head pointer == the current governance head**, recomputable by anyone. Determinism (same log, same order →
  same snapshot, no wall-clock), verifiability (received snapshots recomputed, not trusted), rebuildability
  (delete it, replay, get it back — it's a cache). The snapshot isn't "latest values," it's **"latest values
  that passed authorization at each step"** — events are admitted to the fold only if authorized under the
  rules in force at their position; the log is **self-validating under replay**.
- **User's correction (load-bearing):** *you forward the imperative governance chain and meet the version of
  the declarative chain* — or you don't and there's an explicit disagreement, but **there is never any blind
  trust.** Threshold logic applies to **authorization, never to truth** ("3 of 5 admins co-signed adding Bob,
  meeting the threshold" is a local deterministic check; "3 of 5 say history is valid" is the blind trust the
  model rejects). Both validated locally.
- **Verifiable truncation via roll-up.** A roll-up is a signed checkpoint committing to **(governance head
  hash H, state commitment S)**. Because H is hash-linked, signing H signs the whole prefix → the truncation
  is always **re-expandable and back-verifiable** ("verifiable truncation," not "trusted summary"). **User's
  resolution = Option A:** each peer independently folds and self-checkpoints; co-signing (if used) is
  *corroboration of independent identical folds*, never a substitute for local validation. No blind trust;
  liveness never gates on a quorum (you have your own checkpoint); divergence surfaces as missing/conflicting
  signatures and escalates.
- **Compaction settled at genesis, two tiers:** **governance history is permanent/uncompacted** (small; it's
  exactly what a dormant node needs to reconstruct the authorized signer set and validate everything else) —
  this single choice resolves dormant-node validation. **Content/artifacts compact** by a deterministic
  age/depth rule into head-committed checkpoints with a Merkle root for completeness. Catch-up = "replay the
  small complete governance chain, validate the content-checkpoint against it, then fetch or forgo old
  artifacts." SSB lesson: design forgetting in from the start or it calcifies — so roll-up is **built-in but
  off by default**, scale-motivated, with explicit tradeoffs; **catch-up never depends on it.**

## Certainty vs use; the social plane adjudicates

- **User's reframe:** *we were conflating certainty with actual use.* A node too old to validate isn't a
  correctness failure — it's a **social event** (rejoin is a membership decision). Three honest rejoin states:
  current-enough-to-fold; **behind-but-holds-a-common-ancestor** (re-entry is a social decision, executed as a
  normal MLS add — provenance, not state-reconstruction); **too-far-gone** (just a new join). A read-only
  degraded state is a humane middle rung. **Lost-device tolerance scales inversely with intimacy** (a 5-person
  family recovers generously; a large diffuse group treats a year-dark device with more suspicion) — same
  mechanism, parameterized by the group's **trust character**. The protocol guarantees **honest disagreement**
  (agree / fork / can't-verify, never silent falsehood); the social plane decides what those mean; the UX
  provides **clear decision boundaries with good defaults**.

## MLS forks; forward secrecy constrains keys, not history

- **Membership is bound to the MLS ratchet, so divergence at a cryptographic node is a fork by construction.**
  MLS commits are sequential — two commits from the same epoch put the group in an inconsistent state (the
  fork is total: different ratchet trees, different epoch secrets, can't decrypt each other). Most deployments
  serialize commits with a server; serverless, you **accept the fork as legitimate** and push reconciliation
  to the social layer. **Undesired fork** self-heals by deterministic tie-break inside the ratchet's retention
  window; **desired/unresolved fork** = two independent forward-secret groups, "merge" later is a fresh social
  act, not a cryptographic un-forking.
- **User's correction (load-bearing):** *forward secrecy constrains the live ratchet keys, not your history.*
  Once decrypted, plaintext is yours to retain; deleting the key doesn't oblige deleting the plaintext. So
  history-completeness and forward-secrecy aren't in tension — history is local, peers share it at discretion
  as **range-based sync over crypto ranges**, and a group you weren't in is just history you don't have and
  may request or forgo (completeness vs isolation, both legit).

## redb concretely; the layered storage

- **redb** = embedded, transactional, **pure-Rust** KV in copy-on-write B-trees, ACID, LMDB-inspired; **MVCC
  single-writer / many concurrent readers** (readers see a consistent snapshot, never block the writer);
  **constant-time savepoints** (≈64 kB per 1 GB; cheap rollback) — *purpose-built for the snapshot-rollback
  on late-arriving causally-earlier events*; **multimap tables** (value must be orderable); per-transaction
  durability (the authoritative log wants durable, the derived cache can be non-durable since it rebuilds).
  Stable file format (vs sled mid-rewrite). It's an embedded engine, **not a graph DB** — you build adjacency
  (composite-key ranges, both directions) on top.
- **Layered, not monolithic:** the **graph layer is thin** (relationships, identity, the attachment index —
  references, not contents); **attachments are their own stores** (chat = append-log, game = CRDT, files =
  content-addressed blobs in iroh-blobs by hash). The "tree rooting off the graph" = the graph holds group
  identity + an index of what's attached, whose leaves are pointers into attachment stores. Blobs never go in
  redb (bloats the B-tree). **User:** *I use iroh and can sync automerge or a blob equally well* — so the
  question is "what is a CRDT doc vs a blob, and how do docs reference each other and blobs."
- **redb's role:** the local **derived-state engine** — the materialized governance snapshot + the graph
  adjacency index as typed tables; MVCC so UI reads never block the folding writer; **savepoints = rollback
  checkpoints** for re-folding when a causally-earlier event arrives; the authoritative signed log lives
  durably; the redb projection is a fast, rebuildable cache, never synced, never trusted from a peer.

## One identity system; recursive principal; composition vs valuation

- **One identity/capability system, not two** (two would need a binding layer = the weakest link, the
  confused-deputy / "I thought I removed them" failure at the architecture level). **One identity layer, many
  scoped capabilities** issued to those identities (each member = one stable identity; each attachment = its
  own access unit/key). Revocation has a **single authoritative recipient list** (the membership record) only
  because it's one system. **Access is a key-management model:** "add" = give keys; "remove" = rotate keys so
  no new content; "end a chat" = stop syncing/rotating its key. The **document boundary, the encryption-key
  boundary, and the access unit should be the same boundary.** A **stable identity key** (long-lived, what
  membership/signatures refer to) is separate from **rotating content keys** (granted to identities). **iroh
  node identity is per-device plumbing beneath the per-user identity** — don't let node identity be member
  identity or you rebuild the multi-device problem.
- **Every group is a group of groups.** A user is a group of devices (the device pool, base case, shared key
  lineage); a "group" is a group of users; **same recursive primitive.** The dormant device rejoins its
  **user group** (small, trusted, sibling-device ack), and family membership flows through the user principal
  — rejoin happens at the lowest recursion level with the context to decide it; history-fill is independent
  best-effort sync. **Two distinct edge types over the one principal primitive:** **composition** (shared MLS
  lineage / authoritative-state-merging — device-pool-in-user-in-community) and **valuation** (directional,
  weighted trust between cryptographically-separate groups — no shared keys; group A weights group B's
  assertions). Keep them crisp or trust leaks into key access. **Adversarial posture is a per-edge property,
  not a global stance** (device pool = high-trust/low-adversarial; stranger valuation = low-trust) — forcing
  Byzantine rigor on your own device pool is as wrong as omitting it from a stranger edge.

## MLS multi-device: devices are leaves; user-principal as self-AS

- **MLS does not nest** (leaves are clients; a leaf can't be another group's root) and **forbids duplicate
  signature keys in a group** — so multi-device is handled by **multiple leaves, one per device, each its own
  key**, with **"these leaves are the same user" left to an application credential policy** (RFC 9750 names
  this exactly). So: **cryptographic layer = every device an independent leaf** (per-device PCS + revocation;
  cost: tree sized by device count, device add/remove = group-wide epoch churn). **Identity layer = the
  device-pool-as-principal is real**, via credentials attesting "I'm a device of user-principal Anna." MLS
  assumes an **Authentication Service** but says almost nothing about it because it's not a cryptographic
  question — in serverless P2P the **user-principal key is its own tiny CA, signing its device credentials**
  (user-principal-as-self-AS). mls-rs supports user-defined custom credentials, the concrete hook.
- **This resolves the lamport seam cleanly:** devices are separate leaves with separate keys → assertions are
  **authored and signed per-device**, the envelope carries `author_device` + `author_principal`, **lamport is
  per-device** (no cross-device coordination, no collisions), and the principal's stream is the
  **deterministic fold-time merge** across its devices. Storage and crypto agree without strain.

## Trust philosophy — provenance is certain, trust is a social/utility call

- **MLS factors out trust-establishment and only solves trust-expression** — it assumes an AS, presumes you
  can decide who a key belongs to. **All trust roots in social trust, even the root CAs** (in your browser
  because humans vouch for and police them — DigiNotar/Symantec got removed). The cryptographic chain is
  downstream of a human judgment it can't make. **"Whether a key is really Anna is a utility call, not a
  provenance one."** Provenance (is this the same key that made that earlier assertion) crypto answers with
  certainty; "is this key the person I mean" is **always a human utility judgment.** The QR-scan to add your
  wife's device doesn't *establish* trust — it **binds existing trust to a key** so crypto can carry it
  forward. Pretending crypto *establishes* rather than *records/amplifies* the human decision is itself a
  failure mode (it makes people trust the math where judgment was weak).
- **Web of trust gets a bad rap because PGP made transitive trust automatic and quantified/scalar.** Trust
  isn't transitive that way, and it's **indexed by purpose ("to what")** — "my family group, infinitely, for
  private things" vs "my work group, commensurate with what I know." Contextual trust makes naive transitivity
  obviously wrong. So **vouching is a graded, contextual, non-transitive signal whose strength must match the
  recognized utility — input to a human/policy decision, never a computed verdict.** Both roads coexist: the
  **direct deliberate act** (QR scan, conference key-exchange, mutual-follow) is the high-strength
  cryptographic anchor; **ambient vouches** (your wife vouches for a coworker, employer attests a person, group
  vouches for group) are honestly-low-strength contextual signals. **The moment a vouch becomes an automatic
  grant rather than a surfaced signal, you've rebuilt PGP's mistake.** This is materially more concrete than
  the phone-number/email trust most interaction runs on today, with durable provenance to extrapolate from.

## The build target — the redb layer, vetted and adaptable

- **The user has the rest of the stack elsewhere; the missing piece is the redb layer**, wanted **aligned,
  vetted with lots of tests and mutations, so the surface is well proven and can be adapted.** Layering the
  user named: **peer protocol → complementary enabling ecosystem stack → local tech stack to bind/enable →
  platform presentation.** redb is ecosystem+local, **not the protocol**. UI/UX must be built on a local
  framework that **guides rather than binds**.
- **Decisions locked for the build** (full spec in the build prompt): the **frozen core** (assertion envelope
  with per-device authorship; **typed ids** = `kind_tag ‖ hash`, kind self-describing, raw hash for
  content-addressing vs typed id for graph references; BLAKE3; value-version tags; authoritative table key
  shapes); **two redb table families** (auth_ durable vs idx_/state_ derived-rebuildable) in one file, writes
  atomic across families; **fold engine = sole writer**, validating via **injected traits** (Signer/Verifier,
  CredentialResolver, BlobPresence, lamport source) so the component slots into the existing stack and is
  testable in isolation; **governance covenants mutable** — the **amendment threshold is just the strictest
  covenant**, changed by meeting itself, **disagreement = fork** (no immutable amendment field; genesis fixes
  only the cryptographic root + initial covenant values); **fold knows the concrete assertion types**
  (concrete over generic — proven, adaptable by localized test-guarded change); **multimap-vs-composite-key
  for edges = empirical build-time exploration** with composite-key default (edge tables are derived/
  rebuildable, so switching later is cheap). Public surface = **sync queries (view-models, never storage
  types), async commands (honest CommandResult enums incl. PendingSignatures/Rejected/fork outcomes), a
  change-notification stream.** `get_trust_signals` returns **graded signals, never a verdict**; `vouch`
  **requires context + strength** (the data model enforces the trust philosophy). Test architecture is
  **first-class and per-stage**: property tests (order-insensitive convergence, rebuildability, authoritative-
  vs-derived consistency), **mutation testing (cargo-mutants) with high kill rate on fold+validation**,
  adversarial/malformed-input, fork/divergence, partial-knowledge (four states), compaction + rebuild-after-
  compaction, interface-contract, scale/diversity. "Trivial generators prove nothing" — the corner not to cut.

## Prior art found (web-searched, [confirm before publish])

- **Holepunch Pear/Keet** — the closest: apps launched *into a conversation*, P2P, private-by-default, local
  state, on Hypercore (signed append-only logs) + Hyperbee + Hyperswarm/DHT. **But its substrate is still the
  conversation/room, not the social graph** — chat-bottomed with a plugin model. Validates "activities as
  siblings"; still gets the foundational layer "wrong" by this design's lights. The wedge: *does group identity
  outlive a room, or is the room the only durable object?*
- **AT Protocol** — separates identity + social graph from apps (one identity, many app surfaces via
  lexicons); a real sibling-app ecosystem. **But public-by-design** (profiles/posts/follows public; DMs
  centralized) and its shared graph is the **public follow graph**, not the durable small private group; apps
  needing different relationship semantics opt out of the Bluesky follow graph (a tell that "the graph" isn't
  one reusable thing). Nails "shared substrate, many surfaces"; wrong privacy + graph shape for this use case.
- **Neither has put the durable private group with shared membership/governance at the bottom and hung chat/
  games/calls/photos off it as co-equal siblings, graph load-bearing but invisible.** That gap is the wedge.
- **Gun** (state-based-CRDT P2P graph DB, graph-native, aging JS), **OrbitDB** (op-based Merkle-CRDT on IPFS,
  log-bottomed, IPFS baggage), **SurrealDB/Fluree** (graph *model* but server-oriented, not local-first sync),
  **Veilid** (P2P networking/identity, not storage). Gun/OrbitDB are concept-adjacent but tool-wrong given the
  iroh/Willow/BLAKE3/Automerge stack — study as references, not foundations. **redb + iroh-blobs + Automerge
  over iroh** is the better fit for this stack.
