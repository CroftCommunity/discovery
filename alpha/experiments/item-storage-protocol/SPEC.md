# Item Storage Protocol

Experiment hierarchy and narrative. This is the design brief the code in this directory implements;
`README.md` covers how to run it, and a suite run generates `RUN_REPORT.md`.

audience: implementer, plus any human reader, technical or not

goal: walk the provider-and-customer offering end to end in protocol-level cryptographic terms, with
mock ledgers, explicit assertions, and a reliable cold-storage tier where the plan is no movement and
the verification proves it.

---

## Part 1. The offering in plain language

We keep items for people. An item is any thing a person asked us to keep: a photo, a document, a post,
a backup. Every item is named by its own fingerprint, a number computed from its exact bytes. If even
one byte changes, the fingerprint changes. So an item cannot quietly become a different item.

The customer holds the list: a signed inventory of every item we are supposed to be keeping for them,
with each item's fingerprint and size. The list is signed by the customer's own key, so what we owe
them is written in their handwriting, not ours.

Every time bytes move between us, both sides sign a receipt. Postage is what movement costs. Rent is
what sitting still costs, measured in byte-days. At the end of each month, a statement closes the
books: opening state, plus this month's receipts, equals closing state, and both sides sign it. Next
month only has to check the change, because last month was already agreed.

Anyone can run a spot check at any time: pick items at random, ask for the bytes, compute the
fingerprint, compare against the signed list. How often you check is the dial. Checking costs real
work, so the dial has a price, set at cost. Your paranoia, your bill, no judgment either way.

A sealed item collection is one the customer has locked. From that moment the plan is no movement:
rent accrues, postage should be near zero, and every spot check must return exactly the sealed
fingerprints forever. Sealing is not a promise that change is impossible; it is a guarantee that any
change is caught, because change cannot happen without producing a new signature that monitoring will
see.

Nothing above loses fidelity when translated to the technical terms. The mapping:

| Plain word | Technical term | What it guarantees |
| --- | --- | --- |
| item | content-addressed object (blob or record) | identity is inseparable from content |
| fingerprint | cryptographic hash / CID (SHA-256) | tamper evidence per item |
| the list | signed manifest (Merkle root over item CIDs and sizes) | expected-state is customer-authored and provable |
| receipt | bilaterally signed transfer acknowledgment | billing by agreement, not assertion |
| statement | balance-forward ledger commit, co-signed | disputes are bounded to one period |
| rent | byte-days at rest, derived from the manifest | storage bill computable by the customer |
| postage | bytes transferred, summed from receipts | delivery bill computable by the customer |
| spot check | audit: random retrieval plus hash verification | probabilistic detection of loss or tamper |
| the dial | audit cadence tier, priced linearly | assurance level is declared and paid at cost |
| seal | pinned root plus key ceremony | immutability is detectable, and with keys destroyed, enforced |

---

## Part 2. Ground rules for the implementation

Build this as a single repository with no network dependency. Provider and Customer are two in-process
actors, each holding their own keypair and their own ledger file. Determinism matters more than
realism: seed all randomness, so every experiment run is reproducible and every assertion is exact.

- Language: TypeScript (Node). Platform crypto library. Ed25519 for signatures, SHA-256 for
  fingerprints. Hex-encoded hashes; production atproto uses CIDv1 over DAG-CBOR, and this is the one
  deliberate simplification.
- Ledgers are append-only JSON Lines files under `./ledgers/`, one per actor. Every entry is a signed
  object. Nothing is ever edited in place; corrections are new entries.
- Every experiment is a test file that can pass or fail. Assertions are the deliverable. A green run
  of the whole suite is the demo.
- Every experiment ends by printing a one-line plain-language sentence of what was just proven. The
  suite output reads as the Part 4 narrative.
- Adversarial cases are not optional. Each experiment includes at least one tamper, forgery, or
  walkaway scenario that must be detected.

---

## Part 3. The experiment hierarchy

Ordered by dependency. Each experiment builds only on the ones before it.

### E0. Identity
Both parties exist as keys. Derive a stable identifier from each public key; exchange and pin. A
message signed by Customer verifies under Customer's pinned key and fails under Provider's; identifier
derivation is deterministic. — "We recognize you the same way we count you."

### E1. Items and fingerprints
An item's name is its content. Customer creates items of varied sizes; fingerprint each; Provider
stores by fingerprint; Customer retrieves and re-fingerprints. Adversarial: Provider flips one byte of
one item; retrieval fails for that item and only that item.

### E2. The manifest
Customer builds a manifest (sorted (fingerprint, size), a root hash, a signature over the root).
Provider independently computes expected bytes at rest and the same root from stored copies.
Adversarial: an inflated stored-total claim is rejected by arithmetic alone; a root recomputed with
one item missing mismatches. — "The list is the bill's source of truth, and the customer wrote the
list."

### E3. Transfer receipts
Postage by weight, not by trips, signed on each end. Upload and download in fixed-size increments;
after each, receiver signs an ack and sender countersigns; both append. Adversarial: (a) a party
alters a byte count in its own ledger copy — cross-checking signatures exposes it; (b) walkaway — the
receiver takes the final increment unsigned; exposure is exactly one increment, recorded as a
reputation event. — "Meter the boundary, not the machine."

### E4. Balance-forward statements
Each month stands on the last. Three periods of mixed activity; each close co-signs a statement
(opening/closing roots, rent as byte-days, postage as summed receipts, fees), and statement N+1
references N by hash. Adversarial: rewriting a historical figure fails chain verification at exactly
that link; a fabricated extra period fails to attach. — "Last month was agreed, so this month only has
to explain the difference."

### E5. Spot checks and the detection math
Audit: choose k items at random, retrieve, fingerprint, verify. Monte Carlo: drop a fraction f; over
many trials measure detection; compare to 1-(1-f)^k. Sweep f in {0.001, 0.01, 0.05, 0.2}, k in {1, 5,
20, 100}; emit a measured-vs-predicted table. Honest provider passes all audits; audit cost scales
with k and item sizes, independent of corpus size. — "Spot checks are probabilistic, and we know
exactly how probabilistic."

### E6. The dial
Assurance is a declared setting with a true, linear cost. Tiers (monthly k=5, weekly k=5, daily k=20,
hourly k=20); each tier's cost is per-audit bytes plus a fixed overhead, times audit count. The chosen
tier is a signed declaration, billed through an E4 statement; a mid-period change pro-rates. — "Declare
your setting, pay its true cost, no judgment encoded either way."

### E7. The seal, revocable tier
Customer pins the root and signs a seal declaration. Provider destroys its write-path credential (the
write function fails closed without it). A rotation watch treats any new signed root as an event.
Scheduled audits run against the pinned root; postage over the sealed period equals audit reads only.
Adversarial: (a) a normal-path write fails; (b) a compromised path mutates bytes directly and the next
audit catches it; (c) a customer-signed unseal is classified customer-initiated, an attacker-signed one
is alarmed. — "Immutability detectable at the protocol, enforceable at the key ceremony."

### E8. The tombstone, permanent tier
Repeat the seal, then destroy the customer's rotation capability too (unseal fails closed). Audits keep
verifying against the pinned root. Adversarial: every unseal and write path, from both actors, fails.
The statement chain continues cleanly with rent only. — "The tombstone tier is a feature."

### E9. The grace ledger
Mercy is on-book. Grace events are first-class signed entries: fee waiver with a reason code;
deceased-member hold (rent to the co-op's own account for a three-period term); throttle-instead-of-
cutoff during a payment lapse. Each nets to zero against a co-op grace account; totals are reportable
per period; no grace event edits history. — "The receipts make fairness legible; the margin makes mercy
affordable."

### E10. Stretch: erasure-coded upgrade path
Split a collection's items into n shares with k-of-n recovery. Drop up to n-k shares; recover; verify
fingerprints. Re-run the E5 math to show how coding changes the loss story. Recovery succeeds at the
threshold and fails beyond it; audits over coded shares still verify against the original manifest. —
"Erasure-coded retrievability is the upgrade path if spot checks ever are not enough."

### E11. The financing ledger: extinguishing royalty
The cap table is just another balance-forward ledger. Principal P per investor, cap multiple m (mock
m=3), royalty rate r on a base (both percent-of-profit and percent-of-revenue), payments continue until
cumulative reaches m·P then extinguish permanently — no interest, no accrual in loss years, no residual
claim. Investors hold pro-rata slices of one pool. Simulated across four futures (flat, linear, S-curve,
downside-with-loss-years) × rates × bases, with each payment a co-signed entry chained into the E4
statement, and the extinguishment its own signed entry. A sensitivity table shows years-to-extinguish
across the grid (flat closed form: years = m·P / (r·base)); a low rate on a small base runs multi-decade
or never. Adversarial: a payment after extinguishment is rejected; a rewritten historical figure breaks
the chain at that link; a loss year charges zero with the obligation unchanged and no penalty. Cumulative
payout equals exactly m·P to the cent; all investors extinguish simultaneously; pro-rata splits sum
exactly. — "The return is bounded because the extraction is bounded; the ledger is how we keep that
promise."

### E12. The outside reader: diligence from the files alone
A Funder who holds no keys and gets no private access underwrites the co-op from the published ledger
files, the actors' public keys, the audit transcripts, and the public randomness source, using a
verifier that shares no code with the actors (enforced by module boundary). She confirms, from files
alone: revenue is co-attested (every entry carries a valid customer countersignature and the total
matches); service was delivered (the public-randomness audit challenges replay and the transcripts
verify); the statement chain is intact from genesis; grace is on-book and the books balance; and the E11
royalty payments match the instrument terms. One cooked-books scenario per trust problem — un-co-attested
revenue, an off-book waiver, a retroactive edit, a fabricated audit transcript — is detected and
classified, from the files. — "Revenue isn't asserted, it's co-attested; the loan officer can check it
from her desk."

### E13. Covenants as code, and the underwriting packet
Loan covenants become executable checks over the ledger: salary ratio within the chartered cap, surplus
by a fixed published formula, repayment priority (workers before investors), grace within a declared
band. One compliant year passes; one violation scenario per covenant is flagged with the exact entries
responsible. `DILIGENCE_REPORT.md` is generated entirely by the Funder-side verifier, so the co-op cannot
influence its contents except by changing the underlying signed facts. — "The loan application is a build
artifact."

### E14. The soft-unit counterexample: proving the scope condition
The ledger is trustworthy only where the unit is countable at the boundary by both sides. A "consulting
hours" ledger where both parties sign every entry, but an hour of advice has no boundary-observable
count, is classified by the Funder as attested-but-unverifiable — distinct from verified — so the
standard's boundary is demonstrated, not claimed. — "The signature attests the count, and the count
disciplines the signature; without a countable unit, it's signed vibes."

---

## Part 4. Narrative thread

A customer, Ada, brings her items to the co-op. First the two parties become keys to each other (E0).
Ada's items get names no one can forge (E1) and she signs the list of what she is owed (E2). Bytes move
and both sides sign every receipt (E3), months close into a chain of agreed statements (E4), and Ada
picks how suspicious she feels like being, at cost (E5, E6). Years later she seals the collection for
her family (E7), and eventually chooses the tombstone (E8). Along the way the co-op waives a fee once,
because it could afford to and the books say so out loud (E9). E10 is the door left open for a stronger
guarantee, deliberately not walked through yet except where the word archive is on the label. And the
co-op's early investor, Bram, holds a slice of the royalty pool whose every payment is a co-signed entry
in the same kind of ledger Ada's rent lives in, until the year the cap is reached, the extinguishment
entry is signed, and Bram is simply a well-wisher with receipts (E11). Late in the story a loan officer
at a cooperative fund, June, underwrites the co-op without ever logging in: she verifies Ada's
countersigned rent, replays the public-randomness audits, checks the covenants, and her diligence report
falls out of the verifier as a build artifact (E12, E13). The suite ends by showing June the one thing
her verifier cannot bless, a signed ledger of consulting hours, so the standard's boundary is
demonstrated rather than claimed (E14).

---

## Part 5. Definition of done

- All experiments runnable via a single command, deterministic, all assertions green.
- Ledger files inspectable by a human after a run, with every entry signature-checkable by a standalone
  verify script; the E12 Funder verifier implemented as an independent module (`funder/`) sharing no
  code with the actors.
- A generated `RUN_REPORT.md` containing the E5 measured-vs-predicted table, the E11 years-to-extinguish
  sensitivity table, per-period statement summaries, and the plain-language sentence from each experiment
  in order, so the report reads as the Part 4 narrative; plus the E13 `DILIGENCE_REPORT.md` as a separate
  artifact, generated by the Funder-side verifier alone, with the E14 side-by-side page included.
- No hidden trust: any place the mock stands in for a real mechanism (hex hashes for CIDs, in-process
  actors for the network, deleted files for destroyed keys, an in-process packet for published public
  inputs) is marked with a comment beginning `SEAM:` so the production gaps are enumerable by grep.
