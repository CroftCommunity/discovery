# Item Storage Protocol

A protocol-level cryptographic model of the co-op's item-storage offering, built end to end as a
deterministic experiment suite. Provider and Customer are two in-process actors, each holding their
own keypair and their own append-only ledger. A green run of the whole suite is the demo; the
generated `RUN_REPORT.md` is the narrative.

This is an `alpha/experiments/` spike (see the workspace `AGENTS.md`). It stands alone: zero external
dependencies, no network, no build step.

## What it is

We keep items for people. Every item is named by its own SHA-256 fingerprint, so an item cannot
quietly become a different item. The customer holds a signed manifest (the list of what we owe them);
bytes move under bilaterally signed receipts (postage); sitting still costs rent (byte-days); each
month closes into a co-signed, hash-linked statement. Anyone can spot-check at random, and the power
of a check is exact math. A collection can be sealed (immutability detectable at the protocol,
enforced at a key ceremony) or tombstoned (frozen for all parties, the co-op included). Mercy lives in
the books as a first-class grace ledger. The last experiment leaves the door open to erasure-coded
retrievability.

The full plain-language ↔ technical mapping and the experiment hierarchy are in
[`SPEC.md`](./SPEC.md).

## Run it

Requires Node >= 22.6 (the suite runs directly on Node's native TypeScript type-stripping — there is
no compile step). No `npm install`; there are no dependencies.

```sh
node scripts/run.ts          # run all experiments E0..E10 in order, write RUN_REPORT.md + ledgers
node scripts/verify-ledgers.ts   # standalone: signature-check every entry in every ledger file
node --test                  # run each experiment as an independent pass/fail test
```

Or via the package scripts: `npm run run`, `npm run verify`, `npm test`.

## What a run produces

- **`./ledgers/<EN>/{customer,provider,coop}.jsonl`** — append-only, one signed entry per line,
  human-inspectable, every signature independently checkable by `verify-ledgers.ts`.
- **`./RUN_REPORT.md`** — the E5 measured-vs-predicted detection table, per-period statement
  summaries, and each experiment's plain-language sentence in order, so the report reads as the
  narrative.

Both are generated and git-ignored; they are reproducible from source. The run is deterministic (all
randomness is seeded, and Ed25519 keys are derived from seeds), so two runs produce byte-identical
ledgers and report.

## The experiments

| ID | Proves | Plain sentence |
| --- | --- | --- |
| E0 | Identity is a key; recognition == counting | We recognize you the same way we count you. |
| E1 | An item's name is its content | An item cannot quietly become a different item. |
| E2 | Expected state is customer-authored and provable | The list is the bill's source of truth, and the customer wrote the list. |
| E3 | Postage by weight, signed both ends | Meter the boundary, not the machine. |
| E4 | Disputes bounded to one period via a co-signed chain | Last month was agreed, so this month only has to explain the difference. |
| E5 | Detection probability is exactly 1-(1-f)^k | Spot checks are probabilistic, and we know exactly how probabilistic. |
| E6 | Assurance is a declared setting at true, linear cost | Declare your setting, pay its true cost, no judgment encoded either way. |
| E7 | Seal: change is detectable, and key-destruction enforces it | Immutability detectable at the protocol, enforceable at the key ceremony. |
| E8 | Tombstone: frozen for all parties, audits still pass | The tombstone tier is a feature. |
| E9 | Mercy is on-book and nets to zero | The receipts make fairness legible; the margin makes mercy affordable. |
| E10 | Erasure coding is the upgrade path from spot checks | Erasure-coded retrievability is the upgrade path if spot checks ever are not enough. |

Each experiment includes at least one adversarial case (tamper, forgery, or walkaway) that must be
detected.

## Layout

```
src/         primitives and protocol modules
  prng.ts        seeded deterministic PRNG (mulberry32)
  canonical.ts   deterministic JSON serialization for signing
  crypto.ts      SHA-256 fingerprints; seed-derived Ed25519 keys; sign/verify; id derivation
  ledger.ts      append-only signed JSONL ledger + standalone verify
  actor.ts       keypair + ledger + pinned peer keys
  store.ts       content-addressed store (keyed by fingerprint) + tamper mutators
  manifest.ts    signed manifest, Merkle root, expected-bytes, provider recompute
  billing.ts     byte-day rent integrator over a manifest timeline
  receipts.ts    metered transfers, bilateral acks, reconciliation, walkaway
  statements.ts  balance-forward, co-signed, hash-linked statement chain
  audit.ts       spot-check audit + Monte Carlo detection math
  seal.ts        write-gate / rotation-capability ceremonies + rotation watch
  reedsolomon.ts GF(256) Cauchy Reed-Solomon (k-of-n), zero-dependency
  ...
experiments/  one module per experiment (E0..E10), each returns its sentence + report section
test/         one `node --test` wrapper per experiment
scripts/      run.ts (the single command), verify-ledgers.ts
```

## No hidden trust

Every place the mock stands in for a real mechanism — hex hashes for CIDv1/DAG-CBOR, in-process
actors for the network, deleted capabilities for destroyed keys, deterministic JSON for canonical
DAG-CBOR — is marked in the source with a comment beginning `SEAM:`. Enumerate the production gaps
with:

```sh
grep -rn "SEAM:" src experiments scripts
```
