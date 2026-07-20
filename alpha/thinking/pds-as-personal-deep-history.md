# The PDS as your personal deep-history backend

author: ISaT / Product Security

date: 2026-07-20

status: design thinking — grounded in RUN-HIST-01 (`Modeled`) + RUN-HIST-02 rev B (live, hosted PDS,
E1–E8 green); the trust-tier/ownership open calls (HS OC-1..5) stay the owner's

relationship: rides on the history-reconciliation helper (`beta/impl/drystone-design/history-durability.md`
§G/§I/§J/§L) and the group substrate; it is an opt-in durability choice, not a protocol change. Sibling
to `multi-device.md` (device loss) and `governance-and-survivability.md` (what survives).

---

## 1. The thesis (the inversion, applied to time)

A person accumulates more history than a phone should hold forever. The usual answer is a backup you
must trust someone to hold honestly. Drystone's answer inverts that: **your older history lives on your
own PDS as cold storage, and a tiny local reference tail is what lets no one lie to you about it.**

The move works for the same reason the whole system works. Ordering is cryptographic and every node
re-derives fold state locally, so the reconciliation dataset **never needs a trusted store — only a
durable, fetchable one.** An AT Protocol PDS is exactly that, and better than a dumb store: the repo is a
content-addressed Merkle tree of signed commits, so a thin fetch-and-cache layer can verify integrity
against the account's signing key *before* the fold's own byte-head checks even run. Two verification
planes, both free. (evidence: RUN-HIST-01 Part A anchors, 2026-07-20)

## 2. What was proven (and at what grade)

- **The CID *is* the byte-head — zero re-hash.** A history envelope maps to an `ing.croft.hist.entry`
  record, and the record's CID serves **directly** as the reconciliation byte-head, provided the encoder
  honors atproto's canonical dag-cbor rules (map keys sorted length-then-bytewise-lex; `$bytes`→bytes,
  `$link`→CBOR tag 42). A downstream that isn't atproto-canonical gets a divergent CID and MUST re-hash —
  measured, not guessed. (evidence: RUN-HIST-02 rev B E1/OC-2 GREEN, `spike/hist_live/`, live bsky.social
  PDS; `canonical.rs`) This settles the DRISL-vs-Drystone-canonical-form question that the matchup left
  thin.
- **The mechanics hold on a real PDS.** rkey lexicographic order = (subspace, counter) order across the
  padding boundary; gap detection over cursored `listRecords` pages by bounding digests; fold is
  order-independent (repo/commit/firehose order is a delivery cursor, never fold input); CAR re-hydration
  rebuilds the index with *named* incompleteness; omission is detected by predecessor linkage regardless
  of store honesty. (evidence: RUN-HIST-01 B1–B7 `Modeled`; RUN-HIST-02 rev B E1–E8 green at hosted grade)

## 3. The personal deep-history tier (the cleanest fit)

Of the three shapes a PDS-backed history can take, the **personal** one is the cleanest, because a
personal repo is **single-writer by construction** — the cross-repo assembly problem and the
repo-ownership open call both dissolve. The layering that makes it sound:

- The **repo commit** authenticates what the PDS holds *now* (signed, content-addressed).
- The **local reference tail** — checkpoint byte-heads over the pruned region, kept on the person's own
  devices — authenticates that the archive is the *same* history they offloaded.

Put together: **the PDS is cold storage with cryptographic receipts.** Without the tail, a compromised
account could rewrite and re-sign deep history and you couldn't tell. With it, the PDS can serve you your
own past and cannot lie about it — a missing or altered entry shows as a named gap against the tail, not a
silent substitution. Things further back than a year or three ride the PDS; the phone keeps only the tail.

## 4. The seams (named, not waved away)

- **Single-writer per repo → the thin layer is an AppView.** A *group's* shared DAG spans its members'
  personal repos, so serving group history is cross-repo assembly — which is what an AppView is in
  atproto's own terms. It stays *thin* only because folding happens at the edges; it never holds the
  folded state authoritatively. (This is where the personal tier's simplicity earns its keep — no
  assembly.)
- **Retention and deletion are the holder's, not the group's.** A member deleting records **degrades
  completeness, never correctness** — it folds cleanly into the completeness-ahead dials. The
  reconciliation manifest must tolerate referenced facts going unfetchable.
- **Public by default is a posture, not a default-swap.** Blob mechanics force it: a PDS-resident blob is
  garbage-collected unless a *current record references it*, so if history blobs live on the PDS, some
  index record is public repo content on the firehose. Fine for a public-posture scope; MLS-private facts
  need the blinded-marker treatment or stay off-PDS. PDS-backed history is therefore the **public-marker
  tier of the two-tier model, chosen per scope as a consent dial** — never a silent swap. (evidence:
  RUN-HIST-01 row-1 publicity fact, anchored to the blob-lifecycle guide)
- **The fetch path is client-suited, not capability-gated.** Members can keep speaking G-hist over the
  overlay (meer-level, member IP never touches the PDS), take the service-auth XRPC rung (the layer sees
  one client), or the raw public-sync rung (the audit path). All three serve identical sealed bytes,
  differing only in metadata exposure.

## 5. What this establishes (and does not)

Establishes that a person's deep history has a durable, self-verifiable home that isn't a trusted store —
the PDS as cold storage, the local tail as the receipt. It does **not** decide the open calls that gate a
*group's* PDS-backed history: repo ownership (service DID vs per-group DID, with the F-HIST-1 enumerability
cost), scribe key custody / PLC rotation, the sealed-posture backend shape, and the §L checkpoint
construction (HS OC-1..5, all pending — the owner's). Nor does it size the return-backfill cost (M2, a
modeled lower bound only). Grade ceiling today: `Modeled` (RUN-HIST-01) plus live-at-hosted-PDS
(RUN-HIST-02 rev B); real-transport loss and the `[gates-release]` wire fingerprint remain open.
