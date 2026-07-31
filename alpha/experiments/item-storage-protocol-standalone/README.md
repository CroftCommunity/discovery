# Item Storage Protocol — standalone E0-E11 variant

> Companion to the sibling `alpha/experiments/item-storage-protocol/` (which carries
> E0-E14 plus a funder diligence machine). This directory is an independent,
> **zero-dependency / zero-build** second implementation of E0-E11 — a clean-room
> take that runs `.ts` directly with only Node built-ins. The two were written in
> parallel against the same SPEC; kept side by side deliberately, not merged.

A single, self-contained, **dependency-free** experiment suite that walks the
provider-and-customer storage offering end to end in protocol-level cryptographic
terms: content-addressed items, a customer-signed manifest, bilaterally signed
transfer receipts, a hash-chained statement ledger, quantified spot checks, a
priced assurance dial, a sealed cold-storage tier, a tombstone tier, a grace
ledger, an erasure-coded upgrade path, and an extinguishing-royalty cap table —
all as mock ledgers with explicit assertions and adversarial cases.

Determinism is the point: every keypair, signature, and random draw is seeded, so
every run reproduces byte-for-byte and every assertion is exact.

Status: **first-pass spike (alpha-tier).** Language: TypeScript on Node (built-in
`node:crypto` + `node:test`; no `npm install`). Ed25519 for signatures, SHA-256
for fingerprints.

## Run it

Requires Node >= 22.6 (the suite runs `.ts` directly via type stripping — no build
step, no dependencies).

```
node src/run.ts      # run the suite; write ledgers/ and RUN_REPORT.md; print the story
node src/verify.ts   # standalone: re-verify every ledger entry's hash-chain + signature
node --test          # the same assertions under the Node test runner
```

A green `node src/run.ts` is the demo. It prints one plain-language sentence per
experiment; in order, they are the narrative (Ada brings her items; the two parties
become keys; the list is signed; bytes move under receipts; months close into a
chain; Ada picks her paranoia at cost; she seals, then entombs; the co-op waives a
fee it can afford; the erasure-coded door is shown but not walked through; and
Bram's royalty pool extinguishes on the same ledger machinery).

## Plain word -> technical term (the mapping this suite proves)

| Plain word | Technical term | Where |
| --- | --- | --- |
| item | content-addressed object | `src/item.ts` (E1) |
| fingerprint | SHA-256 hash / CID | `src/crypto.ts` (E1) |
| the list | customer-signed manifest (Merkle root) | `src/manifest.ts` (E2) |
| receipt | bilaterally signed transfer ack | `src/receipt.ts` (E3) |
| statement | balance-forward, co-signed, hash-chained | `src/statement.ts` (E4) |
| rent | byte-days at rest, from the manifest timeline | `src/world.ts`, `src/pricing.ts` (E4) |
| postage | bytes transferred, summed from receipts | E3/E4 |
| spot check | random retrieval + hash verify | `src/audit.ts` (E5) |
| the dial | audit cadence tier, priced at cost | `src/exp/e6_dial.ts` (E6) |
| seal | pinned root + write-credential ceremony + rotation watch | `src/seal.ts` (E7/E8) |
| grace | first-class signed ledger entries netting to a co-op account | `src/exp/e9_grace.ts` (E9) |
| retrievability | k-of-n erasure coding | `src/erasure.ts` (E10) |
| extinguishing royalty | capped cap-table ledger (m*P then stop) | `src/financing.ts` (E11) |

## The experiments

Each builds only on the ones before it. Each includes at least one tamper, forgery,
or walkaway that MUST be detected.

- **E0 Identity** — both parties are keys; identity derivation is deterministic; a
  forged attribution fails under the wrong pinned key.
- **E1 Items and fingerprints** — retrieval round-trips; a one-byte flip is caught,
  localized to exactly one item.
- **E2 The manifest** — provider-computed root equals the customer-signed root;
  an inflated storage claim is rejected by arithmetic alone; a missing-item root
  mismatches.
- **E3 Transfer receipts** — both ledgers reconcile; a forged byte count fails the
  receipt's own signatures; a walkaway leaves exactly one unsigned increment,
  recorded as a reputation event.
- **E4 Balance-forward statements** — the chain verifies from genesis; rent equals
  the independently recomputed byte-day integral; a historical edit fails at exactly
  that link; a fabricated period cannot be inserted.
- **E5 Spot checks and the detection math** — measured detection matches
  1-(1-f)^k across a sweep of f and k; an honest provider always passes; audit cost
  is independent of corpus size.
- **E6 The dial** — audit cost is linear in audit count; the chosen tier appears in
  the statement; a mid-period dial change pro-rates.
- **E7 The seal, revocable tier** — no write path succeeds after the ceremony;
  direct mutation is caught by audit against the pinned root; every root change is
  customer-signed or alarmed; sealed postage equals audit reads exactly.
- **E8 The tombstone, permanent tier** — the rotation capability is destroyed and
  fails closed; audits still pass; the chain continues cleanly with rent only; every
  write/unseal path from both actors fails.
- **E9 The grace ledger** — every grace event (fee waiver, deceased-member hold,
  throttle-instead-of-cutoff) nets to zero against a co-op grace account; totals are
  reportable per period; all are forward entries.
- **E10 Erasure-coded upgrade path (stretch)** — recovery succeeds at the k-of-n
  threshold and fails beyond it; recovered items verify against the manifest; a
  tampered share is caught while recovery still succeeds. The loss math is shown
  beside the E5 detection math. (Any tier marketed "archive" ships with a labeled
  redundancy floor; E10 is a stretch for the general tiers only.)
- **E11 The financing ledger: extinguishing royalty** — cumulative payout equals
  exactly m*P to the cent (final payment partial); all investors extinguish
  simultaneously; pro-rata splits sum exactly; the flat case matches the closed form;
  a post-extinguishment payment is rejected; a rewritten royalty figure is located;
  loss years accrue nothing with no penalty; and the sensitivity sweep makes a
  non-extinguishing (low-rate, small-base) misalignment visible.

## No hidden trust: the SEAM index

Every place the mock stands in for a real mechanism is marked with a comment
beginning `SEAM:`, so the production gaps are enumerable by grep:

```
grep -rn "SEAM:" src
```

The load-bearing ones: hex SHA-256 stands in for CIDv1-over-DAG-CBOR
(`src/crypto.ts`, `src/canonical.ts`); in-process actors stand in for networked
services (`src/actor.ts`); deleting in-memory key material stands in for an
irreversible key-destruction ceremony (`src/seal.ts`); and a compact Cauchy-matrix
Reed-Solomon stands in for a hardened erasure-coding library (`src/erasure.ts`).

## Layout

```
src/
  canonical.ts crypto.ts rng.ts clock.ts   # deterministic primitives
  ledger.ts                                 # append-only, hash-linked, signed log + verifier
  actor.ts item.ts manifest.ts receipt.ts   # E0-E3 domain
  statement.ts pricing.ts world.ts          # E4 statements + shared world
  audit.ts seal.ts erasure.ts financing.ts  # E5-E11 mechanisms
  exp/e0_identity.ts ... e11_financing.ts   # one file per experiment (assertions = the deliverable)
  suite.ts run.ts report.ts verify.ts       # orchestration, single-command runner, report, verifier
test/suite.test.ts                          # the same assertions under `node --test`
RUN_REPORT.md                               # generated; reads as the narrative
ledgers/                                    # generated (gitignored); one JSONL per actor + keyring.json
```

## Place in the corpus

A code-forward spike under `alpha/experiments/item-storage-protocol-standalone/`, in
the storage/retrievability and cooperative-economics lane, alongside the E0-E14
`item-storage-protocol/` sibling. It is a protocol-and-economics model, not a wire
implementation: the one deliberate wire simplification (hex hashes for CIDs, sorted
JSON for DAG-CBOR) is noted at every `SEAM:`. The atproto content-addressing and
history-durability facts it gestures at are settled in the sibling `cairn/` and
`hist-atproto-spike/` material; this suite does not re-verify them.
