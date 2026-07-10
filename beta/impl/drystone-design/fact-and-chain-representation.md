# Drystone: Governance Fact and Chain Representation

`Status: merge-ready specification draft for Part 2, companion to the governance mechanics. Specifies the concrete representation the rest of the governance spec has rested on without defining: how a governance fact is encoded, hashed, and signed; how the history chain and the now relate; how causal references work on the wire versus in local storage; and how a node establishes a now's correctness. Mechanism, not principle.`

`Terms: history chain (the append-only, hash-linked log of signed governance deltas); the now (the derived, replaceable snapshot of current authority state plus in-flight tallies); fact (one governance delta, a single signed operation); FactId (the hash of a fact's canonical encoding); observed head (the governance-state hash a fact was folded against); derived view (any locally built, integrity-checkable index or materialization over the chain).`

`Scope: the representation of the governance chains specifically (not content history, which is history-durability.md, though the wire-versus-local principles here generalize). Covers the fact structure and FactId, canonical dag-cbor encoding and why, the three signing roles, the single committed-head causal reference with local DAG reconstruction, the now's derivation with genesis as the integrity floor and checkpoints as acceleration, and the decoupling of reconciliation format from local storage.`

`Companion to: governance-finality.md (A0 sign-the-state, A1 quorum-as-k-facts, A3 ceiling, A7 the now, A8 finality; this doc is the byte-level substrate under all of them), fold-semantics.md (R1 through R4 fold the facts this doc represents; the observed-head reference here is what the fold walks), history-durability.md (the content-history analogue; the §7.3.3 truncation is the checkpoint referenced here), ../../drystone-spec/part-2-certifiable-design.md (merge target, §7.3).`

Tag on the design choices is `Design`; dag-cbor's canonical and deterministic property is an `Established` property of that encoding; the ATProto and IPLD precedent is `Verified` from the ecosystem survey.

---

## 1. Two chains, linked

Governance state is represented as two linked structures with different jobs, different shapes, and different integrity models.

- **The history chain** is an append-only, hash-linked log of signed governance *deltas*. Its job is audit and reconciliation: it is the authoritative record of what changed, who asserted it, and in what causal order. It is authenticated intrinsically (each entry is signed and hash-linked) and it grows by one entry per governance fact.

- **The now** is the *derived, replaceable* snapshot of current authority state (members, roles, thresholds) plus in-flight tallies (pending decisions and their vote counts). Its job is consumption: fast reads, the tally a fallback enactor checks (governance-finality A6), the state a freshness attestation vouches for (A7, A8). It is not appended to; each now supersedes the last. Its integrity is not from independent signing but from *derivability*: it is bound by hash to the history-chain head it was folded from, so anyone can recompute it and check.

The two are linked in both directions by hash, which is what makes cross-reference possible: the now carries the hash of the history head it derives from (so a now names its provenance), and each history fact carries the hash of the observed governance head it was folded against (§5), which is a state the now materializes. So a now points back to a definite point in the history chain, and every fact points back to the governance state it built on. The history chain is the truth of *what happened*; the now is the truth of *where we are*, derivable from it. `Design.`

## 2. The fact: one signed delta, addressed by hash

Each entry in the history chain is one *fact*: the smallest atomic governance change, and nothing larger. A fact is a delta, not a snapshot; it says "this one thing changed," never "here is the whole world now."

A fact's canonical structure is, minimally:

- the **operation** (for example `RemoveMember(m)`, `GrantRole(m, r)`, `SetThreshold(slot, k)`, or a vote toward one of these),

- the **author** (the persona asserting it),

- the **observed head** (a single hash naming the governance state the author folded against; §5),

- a **per-author monotonic counter** (the author's own sequence number, for author-local gap and replay detection, distinct from the cross-author causality the observed head provides),

- and a **signature** by the author over all of the above.

The **FactId** is the hash of the fact's canonical encoding (§3), excluding the signature, so the signature signs the FactId's preimage and the FactId names the fact independently of who later signs a corroboration of it. A fact is thus content-addressed: its identity is a function of what it says and what it built on, not of when or by whom it was transmitted.

Why deltas and not snapshots for the history chain: a log of full-state snapshots is quadratic, since every checkin re-serializes the entire membership and role table, storing roughly state-size times fact-count, nearly all of it redundant. Deltas store constant-size facts. And the fold (fold-semantics R1 through R4) resolves *facts*: causal precedence, tiebreaks, and projections all operate on individual signed operations, so a snapshot log would have discarded exactly the per-operation causal structure the fold needs and forced it to be reconstructed. The history chain is deltas because both storage cost and the fold demand it. `Design.`

## 3. Canonical encoding, and why it is not JSON

Facts are encoded in a **canonical, deterministic binary encoding (dag-cbor)** for the purpose of hashing and signing. JSON, or any non-canonical form, is at most a *presentation lens* over a fact and MUST NOT be the thing that is hashed or signed. This separation, canonical storage versus presentation, is load-bearing, and the reason must be spelled out because getting it wrong fails silently.

The reason: hashing and signing require a *single, reproducible byte sequence* for a given logical fact, and JSON does not provide one. Key ordering is unspecified, whitespace is free, number formatting varies (integers versus floats, exponent forms), and string escaping and unicode normalization differ across encoders. So the same logical fact can serialize to different byte sequences on different nodes or in different implementations, producing *different FactIds* and *breaking signature verification*, even though nothing about the fact's meaning changed. Two honest nodes would then disagree on a fact's identity, a cross-node divergence introduced purely by the encoding. This is the same class of error as using wall-clock time for ordering (governance-finality A13) or reconstructing causality from a local index (§5): a value that must be identical across nodes is allowed to vary, and the disagreement leaks in through a side door.

A canonical encoding closes the door by fixing all of those degrees of freedom. Dag-cbor specifies map key ordering, a single number model, and deterministic encoding, so a given fact has exactly one valid byte sequence and therefore exactly one FactId on every node and in every conforming implementation. This is an `Established` property of dag-cbor and the reason the surrounding ecosystem already uses it: ATProto encodes its records in dag-cbor, and IPLD uses it as its canonical codec (`Verified`, ecosystem survey).

So the rule is a clean split. The **canonical form** (dag-cbor bytes) is what is hashed, signed, stored authoritatively, and reconciled. The **presentation form** (JSON, or any human-facing or API-facing rendering) is a lens computed *from* the canonical form for display or convenience, is never hashed or signed, and can differ freely across tools because it carries no authority. Authenticate in the canonical form; present in whatever form is convenient. `Design`, resting on the `Established` determinism of dag-cbor.

## 4. Signatures: three distinct roles

Signing appears in three distinct roles in the governance chains, and keeping them distinct is what makes the quorum and freshness models (governance-finality A0, A1, A7) coherent.

- **Fact authorship: one author-signature per fact.** Every history-chain fact carries exactly one signature, by its author, over the FactId preimage. This attests "persona P asserts this operation, built on this observed head." It is the fact's authenticity and nothing more, and it confers no outcome by itself.

- **A decision: k author-signed facts (a quorum), not a separate signature.** A governance decision that needs a k-of-n threshold is *not* a single signed object. It is k concordant facts, each singly authored and signed, that the fold assembles (governance-finality A1). The completing (k-th) fact is where the ceiling is stamped (A3), authored and signed like any other. So a decision's authority *is* the set of k signatures on k facts; there is no distinct "decision signature." This is the sign-the-state model (A0): each vote is a fact about the intended change, they union, and k of them crossing threshold *is* the decision. A vote is itself a delta, which is why the quorum model and the delta representation fit together with no seam.

- **The now: corroborating attestation-signatures over bound state.** When a now is signed, the signatures are *attestations*, each saying "I vouch this now is the correct fold of the history at head H." They corroborate the now's freshness and correctness (the basis for establishing current-to-the-edge in governance-finality A8); they are not authorship of the state, and there may be many, unioning as corroboration. The now's *integrity* comes from derivability (§6), not from these signatures; the signatures add *freshness corroboration* on top.

So facts carry one author-signature each, decisions are k such facts, and the now carries a corroborating set of attestation-signatures over its bound state. All three sign a fact or a state and never claim standing. `Design.`

## 5. Causal references: one committed head on the wire, the full DAG in local storage

A predecessor reference does two jobs, and they must be separated. It establishes **causal truth** (what this fact was created with knowledge of, which the fold needs to order concurrents and gap-detection needs to spot omissions), and it provides **traversal convenience** (walking the graph, finding heads). Only the second can be a local optimization; the first must be authenticated and on the wire.

The rule: **a fact carries a single causal reference, the hash of the observed governance head it was folded against, and that reference transitively commits to the fact's entire causal past.** Because the observed head hash-chains back through every prior fact (the now it names is bound to a history head, which references its predecessors, and so on), one hash closes over the whole causal history the author saw, the way a Merkle root is one hash committing to a whole tree. This is compression, not amnesia: full causal fidelity is preserved, in a single authenticated reference of constant size.

This depends on a sharp distinction that MUST be honored:

- A single reference to the **observed head** (a content hash that closes over the frontier) preserves causal truth compressed. This is the specified form.

- A single **bare parent-pointer** (a fact pointing only to the previous fact by the same author) does *not*. It authenticates author-local order but discards cross-author causality (whether my fact saw yours), which is exactly the information the fold needs and which no local index can recover because it was never attested. This form is forbidden for the causal reference. Author-local order is carried separately by the per-author counter (§2), which is *additional to*, not a substitute for, the committed-head reference.

Causal truth so anchored is identical on every node, because the order is determined by these authenticated head-references, not by anyone's local view. **Traversal and concurrency are then derived locally**: each node builds a DAG index with explicit forward and backward edges, a frontier index, and reverse indexes by member or slot, so that "walk the causal graph" or "what is concurrent with this" is a cheap local lookup rather than a hash-chase. The index is rebuildable from the wire facts and integrity-checkable against them, so it is a pure acceleration with no trust weight.

The honest cost, stated plainly: with the compressed single-reference form, **concurrency is a *derived* property that requires the local DAG index**, so a node computes "these two facts are concurrent" by resolving their head-references against its DAG. An explicit multi-hash frontier (each fact listing every head it saw) would make concurrency *self-evident per fact*, readable without an index, at the cost of larger facts on the wire. The tradeoff is real: compressed is smaller on the wire and preserves full fidelity but makes concurrency computed; explicit-frontier is self-describing but larger. Local-first breaks the tie, because the local index is cheap to maintain and always present in steady state, so a node pays the index cost once, locally, and reaps smaller facts and full fidelity thereafter; the explicit frontier only pays off when reasoning about concurrency *without* an index, chiefly during initial sync, which is the unusual case. So "reconstruct frontier fidelity locally" is precise, and its one caveat is that it means "maintain a local DAG index," which is cheap but not nothing. `Design.`

## 6. The now's derivation: genesis as the integrity floor, checkpoints as acceleration

How does a node establish that a given now is *correct*? The answer is chosen so the base case never depends on the expensive, partition-fragile thing (cross-node corroboration) and instead pays cheap, local, one-time work.

- **Genesis-derivability is the integrity floor.** A now MUST be re-derivable from the history chain from genesis: fold every fact from the beginning and check that you arrive at this now. This is maximal *local* work but requires *nothing external*, so a node can self-certify a now's correctness in complete isolation, from raw signed history alone. No node is ever *required* to trust a state it did not itself verify.

- **Checkpoints are a local, verified pruning-and-acceleration convenience, not a trust substitute.** A checkpoint is a folded state that a node (or its lineage) verified when it held the history, allowing it thereafter to discard raw history before the checkpoint (§7.3.3 truncation) and to fold incrementally *from* the checkpoint rather than re-folding from genesis. This is caching one's *own verified result*, not trusting someone else's summary. A node that has been present folds cheaply from its last verified checkpoint forward.

- **Accepting someone else's corroborated checkpoint is an explicit, optional trust decision, never a base-case dependency.** A fresh node MAY choose to accept a quorum-corroborated checkpoint to skip ahead and save the genesis fold, but that is a deliberate optimization it opts into when corroboration is available and worthwhile, not something the design requires. When corroboration is unavailable (isolation, partition), the node falls back to the always-possible genesis fold.

So corroboration (checkpoints accepted from others, freshness attestations) is an *optimization layered on a locally-sufficient floor*, not a requirement of establishing a now's correctness. This is the same shape as the encoding split (§3) and the causal-reference split (§5): the authenticated, always-available truth is the floor, and corroboration and indexing are optional accelerations on top. Truth never depends on the expensive path; the expensive path only makes truth faster to reach.

The honest cost: a fresh node in a large, old group faces real folding work to self-certify from genesis (all facts). This is accepted because the work is on the *cheap axis* (local, not network), is *one-time* (the node then checkpoints its own verified result and never re-folds), and is *always possible* (needs no one, works in isolation). The alternative, making checkpoint-trust the base case, would have made the *common* operation depend on the *expensive, fragile* axis, corroboration, which under partition is precisely when it cannot be had. Paying a bounded, one-time, local cost to keep the base case free of an unbounded, recurring, partition-fragile cost is the correct trade.

The residual, stated honestly: this settles the *derivation* question and does not touch the *completeness* question. Genesis-derivability establishes "is this now a correct fold of the history I hold," which is fully local and solved. It does *not* establish "is the history I hold complete to the group's edge," which still requires corroboration and is still the open load-bearing property (governance-finality B3). Knowing you have folded all history up to your head is not knowing that nothing newer exists beyond it. So this decision closes the representation underdefinition while leaving the tail-gap exactly where it was. `Design`; the completeness residual is `Load-bearing, unearned` and tracked in governance-finality B3.

## 7. Reconciliation format versus local storage: decoupled

Because agreement between nodes is on *facts and their hashes*, not on *storage layout*, the wire format and the local format are decoupled, and this is a local-first advantage a shared-state chain does not have.

- **The reconciliation format (the wire)** is fixed by the protocol: signed facts in canonical dag-cbor, carrying the committed-head reference. This is what must agree across nodes and what the fold consumes, and it is not negotiable per node.

- **Local storage** may be whatever each node chooses: it may prune raw facts behind a verified checkpoint (§6), keep only the folded now plus a tail, and store facts in any on-disk structure, because any dropped fact can be re-requested and re-verified. Storage depth is a local choice bounded only by how much history the node wishes to be able to serve to others.

- **Local consumption** may build any number of *derived views*: materialized current-state tables, reverse indexes, the DAG index of §5, all of which are integrity-checkable caches of computations over the signed chain, rebuildable at will, and none of which is authoritative on its own. If a view is ever suspect, it is rebuilt from the chain and the signatures re-verify.

A shared-state chain cannot do this freely, because every node reads the same structure, so representation must agree across nodes and cannot be reindexed per node. A local-first chain agrees on the *facts*, so each node stores and indexes however it likes. The balance between reconciliation efficiency, local storage efficiency, and consumption speed is therefore not one dial but three independent choices: the protocol fixes the wire, and each node independently trades its own disk and CPU against its own query speed and its willingness to serve history, with integrity guaranteed throughout because everything traces to signed deltas. `Design.`

## 8. The through-line: authenticate on the wire, optimize locally

The four decisions above are one principle in four places, and stating it once makes the whole representation coherent: **authenticate the minimal truth in a canonical, committed form on the wire, and optimize everything else locally, so integrity always traces from the local view back to the authenticated wire form, and the expensive or fragile paths are never load-bearing.**

- Canonical dag-cbor separates *what is hashed and signed* (the canonical bytes) from *what is presented* (JSON, a lens). §3.

- The single committed-head reference separates *what is causally attested* (one transitive commitment on the wire) from *what is locally indexed* (the full DAG, an acceleration). §5.

- Genesis-derivability separates *the authenticated integrity floor* (fold from genesis, always possible) from *corroboration-based acceleration* (checkpoints and attestations, optional). §6.

- The reconciliation-versus-storage decoupling separates *the fixed wire* from *free local storage and views*. §7.

In every case the wire carries the minimal authenticated truth, the local layer carries the rich optimized view, and the local view is always a rebuildable, integrity-checkable cache of the wire. This is what local-first buys that a shared-state chain cannot: the freedom to optimize consumption and storage per node without weakening the shared, authenticated truth, because the truth lives in the signed facts and their hashes and everything else is a local projection of them. `Synthesis.`

---

## Open items

`Distinct, per the doc-method.`

- **O1. The concrete fact schema.** The field set in §2 is the minimal logical structure; the exact dag-cbor schema (field tags, operation-type enumeration, a version marker for schema evolution) is not yet pinned. `[confirm.]`

- **O2. Checkpoint object format and cadence.** §6 establishes the checkpoint's *role* (a locally-verified pruning-and-acceleration point, the §7.3.3 truncation) but not its concrete representation, how often one is cut, or how a corroborated checkpoint is attested for the optional accept-from-others path. Ties to governance-finality B3. `Design, open.`

- **O3. Signature scheme and key binding.** The three signing roles (§4) are specified in function; the concrete signature scheme and how signing keys bind to personas (the MLS credential relationship) is deferred to the identity layer and not fixed here. `Design, open.`

- **O4. Per-author counter semantics under fork.** The per-author counter (§2) sequences an author's facts within a lineage; its behavior across a fork and merge (whether counters are lineage-scoped) is not yet specified and interacts with continuity-through-merge (governance-finality A16). `Design, open.`

## Changelog

`Working draft; transitions recorded here per the suite's doc-method.`

- **Draft, first specification of the governance representation.** Created to close the underdefinition the governance spec rested on. Specifies the two linked chains (§1), the fact as one signed delta addressed by FactId (§2), canonical dag-cbor with the spelled-out reason it is not JSON and the storage-versus-presentation split (§3), the three signing roles (§4), the single committed-head causal reference with local DAG reconstruction and its honest cost (§5), the now's genesis-derivability floor with checkpoints as acceleration and the explicit note that this leaves B3 untouched (§6), the reconciliation-versus-local-storage decoupling (§7), and the authenticate-on-the-wire-optimize-locally through-line (§8). Cross-linked to governance-finality (A0, A1, A3, A7, A8), fold-semantics (R1 through R4 and the observed-head reference), and history-durability (the content analogue and the §7.3.3 checkpoint).
