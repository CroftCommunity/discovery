# Item Storage Protocol — RUN REPORT

Deterministic run of the twelve-experiment suite (E0-E11). Every figure below is reproducible byte-for-byte from the master seed; nothing here depends on wall-clock time or unseeded randomness.

- Master seed: `croft-item-storage-protocol-v1`
- Assertions: **81/81 passed**
- Statements in the chain: 19
- Simulated span: 570 days
- Co-op grace account absorbed: 213 cents

## The story, in one line each

- **E0 Identity** — We recognize you the same way we count you.
- **E1 Items and fingerprints** — An item cannot quietly become a different item.
- **E2 The manifest** — The list is the bill's source of truth, and the customer wrote the list.
- **E3 Transfer receipts** — Meter the boundary, not the machine.
- **E4 Balance-forward statements** — Last month was agreed, so this month only has to explain the difference.
- **E5 Spot checks and the detection math** — Spot checks are probabilistic, and we know exactly how probabilistic.
- **E6 The dial** — Declare your setting, pay its true cost, no judgment encoded either way.
- **E7 The seal, revocable tier** — Immutability detectable at the protocol, enforceable at the key ceremony.
- **E8 The tombstone, permanent tier** — The tombstone tier is a feature.
- **E9 The grace ledger** — The receipts make fairness legible; the margin makes mercy affordable.
- **E10 Stretch: erasure-coded upgrade path** — Erasure-coded retrievability is the upgrade path if spot checks ever are not enough.
- **E11 The financing ledger: extinguishing royalty** — The return is bounded because the extraction is bounded; the ledger is how we keep that promise.

## Experiments

### E0. Identity

_We recognize you the same way we count you._

Assertions: 4/4 passed.

- PASS — customer signature verifies under customer's pinned key
- PASS — customer signature does NOT verify under provider's key
- PASS — identifier derivation is deterministic
- PASS — distinct keys yield distinct identifiers

**Pinned identities**

| actor | id | public key (hex, truncated) |
| --- | --- | --- |
| Ada (customer) | id:bcf96e5687e673ff | 6d0ca0566db6407e88016e86… |
| Co-op (provider) | id:2a2ee1be7349983d | c374c0525453e0bbcbb432e4… |

### E1. Items and fingerprints

_An item cannot quietly become a different item._

Assertions: 6/6 passed.

- PASS — every untampered item round-trips (retrieve + re-fingerprint)
- PASS — all fingerprints are distinct
- PASS — tampered item fails verification
- PASS — detection identifies exactly which item was tampered
- PASS — tamper is localized: all other items still verify
- PASS — restored item verifies again

**Ada's items**

| label | size (bytes) | fingerprint (truncated) |
| --- | --- | --- |
| wedding-photo.jpg | 4096 | 862b9b67f78c1c59de8c… |
| will.pdf | 512 | 0ad997c3f8d6d6621b49… |
| family-post.txt | 128 | e205dfb9be0973af005e… |
| backup.tar | 8192 | 6c5ff6b4abdefdb0e3a4… |
| voice-note.ogg | 1500 | e1678d5d6d1668d3f47b… |

### E2. The manifest

_The list is the bill's source of truth, and the customer wrote the list._

Assertions: 6/6 passed.

- PASS — manifest signature verifies under customer's pinned key
- PASS — provider-computed root equals customer-signed root
- PASS — expected bytes is a pure function of the manifest
- PASS — provider's stored bytes match expected bytes
- PASS — inflated storage claim rejected by arithmetic alone
- PASS — root over an incomplete set mismatches the signed root

**Manifest summary**

| field | value |
| --- | --- |
| items | 5 |
| total bytes | 14428 |
| root (truncated) | bb91ed532f965b70aced3101… |
| signed by | id:bcf96e5687e673ff |

### E3. Transfer receipts

_Meter the boundary, not the machine._

Assertions: 6/6 passed.

- PASS — every acknowledged receipt verifies under both pinned keys
- PASS — both ledgers reconcile to identical postage totals
- PASS — forged byte count fails the receipt's own signature check
- PASS — walkaway leaves exactly one unsigned increment
- PASS — unsigned exposure equals one increment, never more
- PASS — walkaway recorded as a reputation event in the ledger

> Reconciled postage this exchange: 8704 bytes.

**Transfer summary**

| direction | item | bytes | increments |
| --- | --- | --- | --- |
| upload | backup.tar | 8192 | 4 |
| download | will.pdf | 512 | 1 |

### E4. Balance-forward statements

_Last month was agreed, so this month only has to explain the difference._

Assertions: 7/7 passed.

- PASS — statement chain verifies from genesis
- PASS — period 1 opens where period 0 closed
- PASS — period 2 opens where period 1 closed
- PASS — rent equals the independently recomputed byte-day integral
- PASS — editing a historical figure fails chain verification
- PASS — chain failure is located at exactly the edited link
- PASS — a fabricated inserted period cannot pass chain verification

**Statement chain (cents)**

| period | byte-days | rent | postage | total | closing root |
| --- | --- | --- | --- | --- | --- |
| 0 | 432840 | 43 | 8 | 51 | bb91ed532f96… |
| 1 | 507840 | 50 | 3 | 53 | 146b42bd3535… |
| 2 | 492840 | 49 | 0 | 49 | f13bf4c4a77c… |

### E5. Spot checks and the detection math

_Spot checks are probabilistic, and we know exactly how probabilistic._

Assertions: 4/4 passed.

- PASS — an honest provider passes every audit
- PASS — measured detection matches 1-(1-f)^k within 0.03 (max err 0.0124)
- PASS — audit cost is independent of corpus size (same k, same item size)
- PASS — audit cost equals k * item size

> Monte Carlo: 5000 trials per cell, corpus N=5000, tolerance 0.03.

**E5 detection: measured vs predicted (1-(1-f)^k)**

| f | k | predicted | measured | abs err |
| --- | --- | --- | --- | --- |
| 0.001 | 1 | 0.0010 | 0.0012 | 0.0002 |
| 0.001 | 5 | 0.0050 | 0.0050 | 0.0000 |
| 0.001 | 20 | 0.0198 | 0.0216 | 0.0018 |
| 0.001 | 100 | 0.0952 | 0.0980 | 0.0028 |
| 0.01 | 1 | 0.0100 | 0.0110 | 0.0010 |
| 0.01 | 5 | 0.0490 | 0.0514 | 0.0024 |
| 0.01 | 20 | 0.1821 | 0.1786 | 0.0035 |
| 0.01 | 100 | 0.6340 | 0.6464 | 0.0124 |
| 0.05 | 1 | 0.0500 | 0.0482 | 0.0018 |
| 0.05 | 5 | 0.2262 | 0.2254 | 0.0008 |
| 0.05 | 20 | 0.6415 | 0.6414 | 0.0001 |
| 0.05 | 100 | 0.9941 | 0.9956 | 0.0015 |
| 0.2 | 1 | 0.2000 | 0.1982 | 0.0018 |
| 0.2 | 5 | 0.6723 | 0.6724 | 0.0001 |
| 0.2 | 20 | 0.9885 | 0.9896 | 0.0011 |
| 0.2 | 100 | 1.0000 | 1.0000 | 0.0000 |

### E6. The dial

_Declare your setting, pay its true cost, no judgment encoded either way._

Assertions: 8/8 passed.

- PASS — statement records the chosen tier (monthly)
- PASS — statement records the chosen tier (weekly)
- PASS — statement records the chosen tier (daily)
- PASS — statement records the chosen tier (hourly)
- PASS — weekly audit cost = 4 x monthly (same k, linear in count)
- PASS — hourly audit cost = 24 x daily (same k, linear in count)
- PASS — mid-period dial change bills the sum of the two pro-rated legs
- PASS — pro-rated bill sits between a full-weekly and a full-daily period

> Per-audit overhead: 2 cents; avg item ~3185 bytes.

**Audit tiers billed through the statement (cents)**

| tier | k | audits/period | audit cost | period total |
| --- | --- | --- | --- | --- |
| monthly | 5 | 1 | 17 | 64 |
| weekly | 5 | 4 | 68 | 115 |
| daily | 20 | 30 | 1950 | 1997 |
| hourly | 20 | 720 | 46800 | 46847 |
| weekly->daily | 5/20 | 17 | 1009 | 1056 |

### E7. The seal, revocable tier

_Immutability detectable at the protocol, enforceable at the key ceremony._

Assertions: 7/7 passed.

- PASS — all sealed-period audits passed against the pinned root
- PASS — postage over the sealed periods equals audit reads exactly
- PASS — write through the normal path fails: no credential
- PASS — direct byte mutation is caught by audit against the pinned root
- PASS — customer-signed rotation is classified customer-initiated
- PASS — a non-customer root change raises an alarm
- PASS — every observed root change is either customer-signed or alarmed

**Seal state**

| property | value |
| --- | --- |
| collection | ada-family-vault |
| pinned root | f13bf4c4a77cf3d9a518… |
| write credential | destroyed |
| unseal capability | held by customer |
| sealed audit reads (bytes) | 129856 |

### E8. The tombstone, permanent tier

_The tombstone tier is a feature._

Assertions: 7/7 passed.

- PASS — unseal capability is destroyed
- PASS — unseal now fails closed (returns no rotation)
- PASS — audits still verify against the pinned root in the tombstone tier
- PASS — statement chain continues cleanly with rent only
- PASS — provider write path fails (no credential)
- PASS — customer unseal path fails (capability destroyed)
- PASS — collection is frozen for all parties

**Tombstone state**

| property | value |
| --- | --- |
| collection | ada-family-vault |
| write credential | destroyed |
| unseal capability | destroyed |
| audits still verifying | yes |

### E9. The grace ledger

_The receipts make fairness legible; the margin makes mercy affordable._

Assertions: 8/8 passed.

- PASS — fee waiver: member's total is rent only (fee fully waived)
- PASS — deceased hold period 1: estate owes nothing
- PASS — deceased hold period 2: estate owes nothing
- PASS — deceased hold period 3: estate owes nothing
- PASS — throttle: service continues and the shortfall is carried, not lost
- PASS — grace events net to zero against the co-op grace account
- PASS — grace totals are reportable per period
- PASS — all grace events are forward entries (no history edited)

> Co-op grace account absorbed 213 cents this run.

**Grace events (cents)**

| period | kind | rent | fee | grace credit | member total |
| --- | --- | --- | --- | --- | --- |
| 14 | fee-waiver | 47 | 25 | -25 | 47 |
| 15 | deceased-hold | 47 | 0 | -47 | 0 |
| 16 | deceased-hold | 47 | 0 | -47 | 0 |
| 17 | deceased-hold | 47 | 0 | -47 | 0 |
| 18 | throttle | 47 | 0 | -47 | 0 |

### E10. Stretch: erasure-coded upgrade path

_Erasure-coded retrievability is the upgrade path if spot checks ever are not enough._

Assertions: 5/5 passed.

- PASS — recovery succeeds from any 4 of 6 shares (drop up to 2)
- PASS — recovery fails beyond the threshold (3 shares)
- PASS — recovered items verify against the original manifest
- PASS — tampered share is detected by its fingerprint
- PASS — recovery still succeeds using only the good shares

> Scope note: any tier marketed as "archive" ships with a labeled redundancy floor (e.g. 4-of-6 coding or 3 copies); E10 is a stretch for the general tiers only.

**Loss story: uncoded vs 4-of-6 coding (detection math beside it)**

| p (per-share loss) | uncoded loss | coded loss | detect@k=4 |
| --- | --- | --- | --- |
| 0.001 | 0.0010 | 1.996e-8 | 0.0040 |
| 0.01 | 0.0100 | 1.955e-5 | 0.0394 |
| 0.05 | 0.0500 | 2.230e-3 | 0.1855 |
| 0.2 | 0.2000 | 9.888e-2 | 0.5904 |

### E11. The financing ledger: extinguishing royalty

_The return is bounded because the extraction is bounded; the ledger is how we keep that promise._

Assertions: 13/13 passed.

- PASS — cumulative payout equals exactly m*P to the cent
- PASS — investors' cumulatives sum exactly to the cap
- PASS — Bram extinguishes at exactly m * his principal
- PASS — Cleo extinguishes at exactly m * her principal
- PASS — all investors extinguish simultaneously (one event)
- PASS — pro-rata splits sum exactly every year
- PASS — final payment is partial (clamped to the cap)
- PASS — flat-case years-to-extinguish matches ceil(closed form)
- PASS — no payment accrues after extinguishment
- PASS — editing a historical royalty figure breaks chain verification at that link
- PASS — loss years exist in the downside future
- PASS — in a loss year royalty due is zero and the obligation is unchanged (no penalty)
- PASS — a low rate on a small base fails to extinguish within the horizon (visible misalignment)

> Live scenario: flat future, r=0.05, base=revenue; extinguished at year 32 (closed form 31.58). Pool cap 300000 cents = m*P.

**Years-to-extinguish (cap = m*P = 300000 cents; horizon 40y)**

| future | base | r=0.02 | r=0.05 | r=0.10 |
| --- | --- | --- | --- | --- |
| flat | profit | >40 | 40 | 20 |
| flat | revenue | >40 | 32 | 16 |
| linear | profit | >40 | 40 | 27 |
| linear | revenue | >40 | 27 | 18 |
| scurve | profit | >40 | >40 | 32 |
| scurve | revenue | >40 | 27 | 20 |
| downside | profit | >40 | >40 | >40 |
| downside | revenue | >40 | >40 | 25 |

## Statement chain (per-period summary, cents)

| period | days | tier | rent | postage | audit | fees | grace | total |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 0 | 0-30 | none | 43 | 8 | 0 | 0 | 0 | 51 |
| 1 | 30-60 | none | 50 | 3 | 0 | 0 | 0 | 53 |
| 2 | 60-90 | none | 49 | 0 | 0 | 0 | 0 | 49 |
| 3 | 90-120 | monthly | 47 | 0 | 17 | 0 | 0 | 64 |
| 4 | 120-150 | weekly | 47 | 0 | 68 | 0 | 0 | 115 |
| 5 | 150-180 | daily | 47 | 0 | 1950 | 0 | 0 | 1997 |
| 6 | 180-210 | hourly | 47 | 0 | 46800 | 0 | 0 | 46847 |
| 7 | 210-240 | weekly->daily | 47 | 0 | 1009 | 0 | 0 | 1056 |
| 8 | 240-270 | sealed | 47 | 45 | 8 | 0 | 0 | 100 |
| 9 | 270-300 | sealed | 47 | 37 | 8 | 0 | 0 | 92 |
| 10 | 300-330 | sealed | 47 | 46 | 8 | 0 | 0 | 101 |
| 11 | 330-360 | tombstone | 47 | 0 | 0 | 0 | 0 | 47 |
| 12 | 360-390 | tombstone | 47 | 0 | 0 | 0 | 0 | 47 |
| 13 | 390-420 | tombstone | 47 | 0 | 0 | 0 | 0 | 47 |
| 14 | 420-450 | none | 47 | 0 | 0 | 25 | -25 | 47 |
| 15 | 450-480 | none | 47 | 0 | 0 | 0 | -47 | 0 |
| 16 | 480-510 | none | 47 | 0 | 0 | 0 | -47 | 0 |
| 17 | 510-540 | none | 47 | 0 | 0 | 0 | -47 | 0 |
| 18 | 540-570 | none | 47 | 0 | 0 | 0 | -47 | 0 |

## Reproduce

```
node src/run.ts      # runs the suite, writes ledgers/ and this report
node src/verify.ts   # re-verifies every ledger entry's signature and chain
node --test          # the same assertions under the Node test runner
```
