# Reference: Kleppmann, "Designing Data-Intensive Applications"

status: reference note (design guidance for the impl reference core); register entry, not normative spec

## Overview

Martin Kleppmann's "Designing Data-Intensive Applications" (the "Wild Boar book") gives mental models
for how databases, streaming platforms, and distributed systems behave under the hood, rather than
teaching a specific tool. Its value for the impl layer is the vocabulary and the trade-off frames: it
names the forces that any data-intensive system negotiates, and it does so at the level of design
posture rather than product. This note captures the parts that bear directly on Drystone's design and
connects them to the reference core.

## The three design imperatives

Every design decision is a trade-off among three concerns.

- **Reliability.** Keep working correctly under hardware faults, software bugs, and human error. The
  posture is to design for failure rather than to try to prevent it. Faults are assumed, not treated as
  anomalies.

- **Scalability.** A clear path to handle growth in data volume, read/write traffic, and complexity.
  The book insists on making this concrete: describe load with explicit "load parameters" (writes per
  second, cache hit rate, fan-out) and describe performance with distribution metrics, in particular
  tail latency (99th-percentile), not averages.

- **Maintainability.** Most of a system's cost is ongoing maintenance, not the initial build. Three
  sub-goals: operability (easy to run and observe), simplicity (manage complexity via good
  abstractions), and evolvability (easy to adapt as requirements change).

## Core lessons

- **Storage engines: B-Trees vs LSM-Trees.** B-Trees (Postgres, MySQL) overwrite mutable pages in
  place, are read-optimized, and rely on a write-ahead log for crash recovery. LSM-Trees (Cassandra,
  RocksDB) write append-only sequential files, are write-optimized, run background compaction, and are
  inherently crash-robust because the on-disk log is never mutated in place. The append-only,
  compaction-driven shape is the one that rhymes with an event-log design.

- **ACID isolation is not uniform.** True serializable isolation is rare because it carries a large
  performance penalty; most databases default to weaker levels (read committed, snapshot isolation).
  Weak isolation exposes subtle bugs (write skew, phantom reads). The lesson is not "use stronger
  isolation" but "know which guarantee you actually have and add application-level safeguards where the
  guarantee does not cover you."

- **Distributed systems are hostile.** You cannot trust the network (it drops messages and delays them
  unpredictably) or time (clock skew is inevitable). Agreement across nodes needs consensus algorithms
  (Raft, Paxos) and careful leader election, and consensus is a cost you pay only where the problem
  genuinely requires it.

## Two memorable examples

- **Twitter home timeline (the fan-out problem).** The read-time approach (on read, query everyone a
  user follows, then merge and sort) grew too slow at scale, so the design switched to a write/push
  model (on tweet, insert into every follower's precomputed timeline). The lesson is that the
  architecture must adapt to the shape of the load, and that read cost and write cost trade against each
  other explicitly.

- **The "doctors on call" write skew.** Two on-call doctors each see "two are on call" and each cancel
  their own shift simultaneously. The database commits both writes because neither overwrote the other's
  row, and the invariant (at least one doctor on call) is violated, leaving zero. The lesson is that weak
  isolation needs application-level safeguards; the constraint that mattered was not visible to the
  row-level conflict check.

## Defining quotes

The quotes below were surfaced in an AI-assisted reading session. They are marked [UNVERIFIED] and should
be confirmed against the book before any publish; the book and author attribution stand regardless.

> There is no such thing as an 'ideal' data model for all circumstances ... The relational model and the
> document model are becoming more similar over time. [UNVERIFIED]

> Complexity is accidental if it is not inherent in the problem that the software solves (as seen by the
> users) but arises only from the implementation. [UNVERIFIED]

> In a distributed system, you can no longer assume that the network is reliable, or that time is
> accurate. You must design your system to tolerate faults, because preventing them entirely is
> impossible. [UNVERIFIED]

> We are moving away from treating a database as a black box that hides all complexity, toward unbundling
> the database, treating data infrastructure as a stream of events. [UNVERIFIED]

## Relevance to Drystone's impl

The book's frames map onto choices the impl layer has already made, which is why it earns a reference
entry rather than a reading recommendation.

- **Fan-out maps onto delivery-as-race.** The Twitter read-vs-write model is the same trade the delivery
  layer works through as delivery-as-race (delivery-layer §11.9), and it is the reasoning behind sizing
  cost "on the live set, not the roster." The write/push model pays at send time to keep receive cheap;
  the tier work makes the same call about where the fan-out cost should land, and against which set it
  should be counted.

- **Unbundling the database into a stream of events** rhymes with the content-addressed history DAG and
  the governance-chain-as-log. Drystone does not hide state behind a black-box store; it treats the
  authoritative record as an append-only, content-addressed sequence of facts, which is the same move
  the book describes as the industry unbundling the monolithic database into an event stream. The
  LSM-Tree lesson (append-only, compaction, never mutate in place) is the storage-engine echo of the
  same posture.

- **"Tolerate faults, do not prevent them"** is the posture the spec takes on eventual consistency, and
  it is the same intent as "commit liveness, not membership consensus." Drystone assumes the network and
  the clock are unreliable and designs the commit path so that liveness does not depend on a global
  agreement about membership. Consensus is treated as a cost paid only where a problem is genuinely
  non-monotonic, which is consistent with the book's framing of consensus as the coordination you buy,
  not the default you assume.

## What this establishes (and does not)

Establishes a shared reference vocabulary (the three imperatives, the storage-engine and isolation
lessons, the fan-out and write-skew examples) and records how those frames connect to design choices the
impl layer has already made. It is a lens for reasoning and cross-referencing, not a source of
requirements. It does **not** add any normative claim to the spec, does not verify the quotes (those are
flagged for confirmation against the book), and does not substitute for the primary distributed-systems
sources the design is actually grounded against.
