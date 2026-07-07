# Raw transcript: Kleppmann, "Designing Data-Intensive Applications" (key points), 2026-07-07

`Provenance caveat (PLAYBOOK §4): content-faithful cleaned paste, not a byte-pristine export. Reference
notes on a foundational distributed-systems text, routed to impl as design guidance for the reference core.`

Martin Kleppmann's "Designing Data-Intensive Applications" (the "Wild Boar book") gives mental models for
how databases, streaming platforms, and distributed systems work under the hood, rather than teaching a
specific tool.

## The three design imperatives (every decision is a trade-off among these)

- **Reliability.** Keep working correctly under hardware faults, software bugs, human error. Design for
  failure rather than trying to prevent it.

- **Scalability.** A clear path to handle growth (data volume, read/write traffic, complexity). Understand
  "load parameters" (writes/sec, cache hit rate) and "performance metrics" (99th-percentile latency).

- **Maintainability.** Most cost is ongoing maintenance, not initial build. Prioritize operability
  (easy to run), simplicity (manage complexity via abstractions), evolvability (easy to adapt).

## Core lessons

- **Storage engines: B-Trees vs LSM-Trees.** B-Trees (Postgres, MySQL): mutable pages overwritten in
  place, read-optimized, rely on a write-ahead log for crash recovery. LSM-Trees (Cassandra, RocksDB):
  append-only sequential files, write-optimized, background compaction, inherently crash-robust.

- **ACID is not uniform.** True serializable isolation is rare (big performance penalty); most databases
  default to weaker levels (read committed, snapshot isolation), exposing subtle bugs (write skew, phantom
  reads).

- **Distributed systems are hostile.** You cannot trust the network (drops, unpredictable delays) or time
  (clock skew is inevitable). Agreement needs consensus algorithms (Raft, Paxos) and careful leader
  election.

## Memorable examples

- **Twitter home timeline (the fan-out problem).** Read-time approach (query everyone you follow, merge,
  sort) got too slow; switched to a write/push model (on tweet, insert into all followers' precomputed
  timelines). Lesson: architecture must adapt to the shape of the load.

- **"Doctors on call" write skew.** Two on-call doctors both see "two are on call" and both cancel
  simultaneously; the DB commits both (neither overwrote the other's row), leaving zero doctors. Lesson:
  weak isolation needs application-level safeguards.

## Defining quotes

> There is no such thing as an 'ideal' data model for all circumstances... The relational model and the
> document model are becoming more similar over time.

> Complexity is accidental if it is not inherent in the problem that the software solves (as seen by the
> users) but arises only from the implementation.

> In a distributed system, you can no longer assume that the network is reliable, or that time is accurate.
> You must design your system to tolerate faults, because preventing them entirely is impossible.

> We are moving away from treating a database as a black box that hides all complexity, toward unbundling
> the database, treating data infrastructure as a stream of events.

Design relevance for Drystone's impl: the fan-out example maps directly onto the §11.9 delivery-as-race and
the "size on the live set, not the roster" tier work; "unbundling the database into a stream of events"
rhymes with the content-addressed history DAG and the governance-chain-as-log; "tolerate faults, do not
prevent them" and "commit liveness not membership consensus" are the same posture the spec takes on
eventual consistency.
