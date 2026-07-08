# The redb storage contract: the concrete local engine under the derived-index pattern

`Status: impl layer (Layer 5, reference implementation). Register: engine contract. Resolution: contract —
the concrete obligations the chosen local-storage engine (redb) must meet to sit under the abstract
authoritative-vs-derived storage pattern the rest of the impl layer carries. redb feature facts carry
verification flags pending a pass against redb's own documentation; the no-linearizability caveat is a
standing crash-safety risk that travels with the choice. This is the home the redb layer (open thread T25)
is now given: the abstract pattern named the shape, this document pins the engine.`

## Overview

The impl layer already carries the storage *pattern* in the abstract: an authoritative tier of signed,
content-addressed facts and CRDT documents that is the source of truth, and a *derived index* — a
throwaway, rebuildable projection over that authoritative tier, optimized for fast query — with the rule
that the derived tier is never synced and never trusted from a peer (local-first CQRS). That pattern is
specified engine-agnostically in `fact-and-chain-representation.md` (§6 the now's derivation, §7 the
reconciliation-versus-local-storage decoupling) and summarized in `README.md`.

This document is the concrete engine contract *under* that pattern. It records that the derived-index tier
is built on **redb** — an embedded, transactional, pure-Rust key-value store — and pins the specific redb
properties the pattern leans on, the two table families that keep the authoritative and derived tiers from
collapsing into one, the single-writer discipline that makes the projection deterministic, and the one
honest risk that must travel with the choice: redb is ACID by design and well-tested, but there is no
published Jepsen-grade linearizability evidence for it. The abstract pattern says *what* the local tier
must be; this document says *which engine* realizes it and *why that engine*, so the choice is defensible
rather than assumed.

## Charter: what this document covers

- **In scope:** the redb-specific storage obligations under the abstract authoritative-vs-derived pattern —
  the engine properties relied on (MVCC, savepoints, multimap, stable file format), the two table families,
  the fold-as-sole-writer discipline, the blob-placement rule, and the linearizability caveat.
- **Out of scope (and where it lives):** the abstract pattern itself (`fact-and-chain-representation.md`
  §6–§7); the on-the-wire fact representation and canonical encoding (`fact-and-chain-representation.md`
  §2–§3); the governance fold's semantics (`fold-semantics.md`); the transport that syncs the authoritative
  tier (`../transport-iroh-gossip-and-quic.md`).
- **Boundary call:** this is the "which local engine, and what it must guarantee" register. The engine is a
  *local implementation detail* of a single node, never the protocol — nothing here is on the wire, nothing
  here is agreed between nodes. The authoritative truth lives in signed facts and hashes; redb holds only a
  node's private, rebuildable projection of them.

## The engine: redb, and what it is chosen for

**redb** (author Christopher Berner) is an embedded, transactional, ACID key-value store written in pure
Rust, its data held in copy-on-write B-trees in a design inspired by LMDB. Its interface is a persistent,
thread-safe, compile-time-typed analogue of a `BTreeMap`. It is a single-file, single-process, embedded
engine with no C or C++ dependency and no server — which is exactly the local-first, single-node, pure-Rust
profile the derived tier needs. `[confirm against redb docs]`

Four properties are load-bearing for the derived-index role, and each is chosen for a specific reason that
must travel with the choice:

- **MVCC: a single writer, many concurrent readers.** Readers see a consistent point-in-time snapshot and
  never block the writer. This is the reason redb fits the projection: the fold engine is the *one* writer
  materializing current state, while UI reads and query traversals run concurrently against a stable
  snapshot without ever stalling the fold or being stalled by it. A single-writer engine is a feature here,
  not a constraint — the fold is meant to be the sole writer (below). `[confirm against redb docs]`

- **Constant-time savepoints, purpose-built for the late-event rollback.** redb can capture a savepoint and
  later roll the store back to it in constant time, at small marginal storage cost. This is the property
  the derived tier is chosen *for*: when a causally-earlier event arrives late — after the projection has
  already folded past its position — the index must be rewound and re-folded from a point before that
  event. Savepoints make that rewind cheap and deterministic, rather than forcing a full rebuild from
  genesis on every out-of-order arrival. The savepoint is the local rollback checkpoint; re-folding forward
  from it restores a correct projection. `[confirm against redb docs]`

- **Multimap tables, with an orderable value type.** redb offers multimap tables where a key maps to many
  values, and the value type must be orderable. This is the primitive under adjacency: graph edges and
  reverse indexes are one-key-to-many-neighbors, and orderable values give range scans in a defined order.
  redb is an embedded KV engine, **not** a graph database — adjacency is *built on top* (composite-key
  ranges, or multimap tables, in both directions), never provided. Which of the two representations wins is
  an empirical build-time measurement, and because edge tables are derived and rebuildable, switching later
  is cheap. `[confirm against redb docs]`

- **A stable, committed file format with per-transaction durability tuning.** The on-disk format is stable
  with an upgrade-path commitment, and durability is tunable per transaction (a non-durable commit keeps
  atomicity, consistency, and isolation but drops the durability guarantee for speed). This maps cleanly
  onto the two tiers: the authoritative tier wants durable commits, while the derived tier can commit
  non-durably because it rebuilds from the authoritative tier if lost. `[confirm against redb docs]`

## Two table families: authoritative durable, derived rebuildable

The node's redb file holds **two table families that must not be collapsed into one**, and keeping them
distinct is what keeps the authoritative-vs-derived boundary honest at the storage layer:

- ***`auth_` — the authoritative family, durable.*** The signed, content-addressed facts and the governance
  log: the source of truth, committed durably, the thing that is synced and integrity-checked. This is the
  local persistence of the wire truth, not a projection of it.

- ***`idx_` — the derived family, rebuildable.*** The materialized current-state tables, the graph
  adjacency index, and any reverse indexes: a fast, queryable projection folded *from* the `auth_` family.
  It may be committed non-durably, it is never synced, and it is never trusted from a peer. If it is ever
  lost or suspect, it is deleted and re-folded from the authoritative family, and it comes back identical.

The two families live in one file, and writes that touch both are atomic across the families, so a single
transaction advances the authoritative log and the projection it implies without a window where they
disagree. The reason to keep the families separated rather than merged is the same reason the abstract
pattern separates the tiers: a derived table that could be written independently, synced, or trusted from a
peer would reintroduce exactly the confusion the pattern exists to prevent — a projection masquerading as
truth.

## The fold is the sole writer

The derived family has exactly one writer: the fold engine that reads the authoritative facts and
materializes the projection. Nothing else writes `idx_` tables. This is what makes the projection
deterministic and rebuildable — same authoritative log, same fold, same projection, on any node, with no
wall-clock and no second writer racing the first.

The fold is kept testable in isolation by taking its dependencies as **injected traits** rather than
reaching for them directly: the signature verifier, the credential resolver, the blob-presence check, and
the lamport / ordering source are all interfaces the fold is handed. The storage component therefore slots
into the surrounding stack without hard-wiring it, and the fold can be exercised against test doubles for
each injected dependency. redb's single-writer MVCC is the engine-level counterpart to this discipline: the
sole-writer fold and the single-writer store are the same commitment expressed at two layers.

## Blobs live in iroh-blobs, never in redb

Large binary payloads — files, media, any sizeable attachment — are **content-addressed blobs stored in
iroh-blobs and referenced by hash**, and they are **never stored in redb**. Putting blobs in the B-tree
would bloat it and defeat the fast, small, rebuildable character the derived tier depends on. The engine
holds *references* — the graph node, its identity, and an index of what is attached, whose entries are
pointers (hashes) into the blob store — not contents. This is the storage-layer expression of the same
reference-not-content rule the abstract pattern applies to the graph: the thin tier holds identity and an
attachment index; the heavy content lives in its own store, addressed by hash.

## The honest caveat: no published linearizability evidence

This risk is load-bearing and must travel with the engine choice, unrolled and prominent rather than folded
into a footnote.

redb is **ACID by design and well-tested**, with a copy-on-write plus checksum design intended to be
crash-safe. But there is **no Jepsen-grade, published linearizability or formal crash-safety evidence** for
it: no adversarial fault-injection test suite of the kind that has been run against many production
databases has been published for redb. `[confirm against redb docs]`

The reason this must be stated with the choice, and not silently assumed away, is the anti-rollup rule: a
dependency selection is only trustworthy if the reason it might *not* hold matures alongside it. The
mitigating factor is architectural rather than a claim about redb's internals — the derived `idx_` family
is rebuildable from the authoritative `auth_` family, so a corrupted or torn projection is recoverable by
re-folding, not a loss of truth. That mitigation is real but partial: it covers the derived tier, not a
torn write to the durable authoritative family, which is precisely where the absence of published
crash-safety evidence bites hardest. So the caveat stands as an open risk on the authoritative-tier
durability path, to be discharged by either primary evidence of redb's crash-safety or a node-level
integrity check over the authoritative family — not by assuming ACID-by-design is equivalent to
ACID-verified-under-fault.

## Where this sits under the abstract storage pattern

The abstract pattern in `fact-and-chain-representation.md` establishes the shape without naming an engine:
the wire carries the minimal authenticated truth (signed facts in canonical encoding), the local layer
carries a rich optimized view, and the local view is always a rebuildable, integrity-checkable cache of the
wire (§6 genesis-derivability, §7 the reconciliation-versus-storage decoupling). That document deliberately
leaves *local storage* as "whatever each node chooses."

This document is the concrete instance of that free choice: redb is the engine, the `auth_` / `idx_`
families are the durable-versus-rebuildable split the pattern implies, the sole-writer fold is the
deterministic materialization the pattern requires, and savepoints are the mechanism behind the pattern's
"re-fold when a causally-earlier fact arrives." Nothing here changes the wire or the shared truth — it
records how one node realizes the local projection the pattern permits, and pins the engine properties that
make the realization sound. The redb layer, tracked as open thread T25, is thereby given a home: the
pattern said what the local tier must be, and this contract says which engine meets it and why.

## What this establishes (and does not)

Establishes that the derived-index tier of the abstract storage pattern is realized on redb, and pins the
specific reasons: MVCC single-writer / many-reader so a sole-writer fold never blocks reads and reads never
block the fold; constant-time savepoints purpose-built for the cheap rollback-and-re-fold when a
causally-earlier event arrives late; multimap tables with orderable values as the primitive under adjacency
that redb does not provide as a graph database; a stable file format with per-transaction durability tuning
that maps onto durable-authoritative versus non-durable-derived; two table families (`auth_` durable,
`idx_` rebuildable) that keep the authoritative and derived tiers from collapsing; a fold-as-sole-writer
discipline realized through injected traits; and blobs held in iroh-blobs by hash, never in redb.

Establishes, and keeps prominent, the one honest risk: redb is ACID by design and well-tested but carries
**no published Jepsen-grade linearizability or crash-safety evidence**, a standing risk on the
authoritative-tier durability path that the derived tier's rebuildability only partially mitigates.

Does **not** re-specify the abstract pattern, the wire fact representation, or the governance fold's
semantics (they live in the sibling impl documents), does **not** finalize the edge-table representation
(composite-key versus multimap is an explicit build-time measurement), and does **not** certify the redb
feature facts or discharge the linearizability caveat — those carry verification flags and the caveat
remains an open risk until confirmed against redb's own documentation and, for crash-safety, against
primary evidence.
