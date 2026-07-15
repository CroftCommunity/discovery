# RUN-08 — build, then trace (two parts, one branch)

`Branch: claude/run-08-build-and-trace-39fnfj, off main (RUN-07 merged). 2026-07-15. Rust 1.94.1.
Part 1 lands the code work + riders; Part 2 traces the branch's own post-Part-1 state. Commits:
Part 1 phases one commit (or series) each; Part 2 markdown in one commit plus one comment-only code
commit. An extra infra commit folded the Proofs corpus into discovery (owner-authorized mid-run).`

## Per-phase status

| Part / phase | What | Status | Commit |
|---|---|---|---|
| **1A** | Riders (markdown) | ✅ done | `3b69093` |
| infra | Fold the Proofs corpus into `alpha/Proofs/` (frozen, canonical) | ✅ done (owner-authorized) | `b3ecf8f` |
| **1C** | BIP39 Tier-1 lock spike | ✅ done, 11/11 green | `93e0be3` |
| **1B** | Conformance: key-distribution over the wire | ✅ done (reproduced green-real; ledger reconciled) | `5534e05` |
| **2.2b** | Backward-link test doc-comments (comment-only) | ✅ done | `87ca3ce` |
| **2 (md)** | Forward/backward links, EVIDENCE-MAP, findings, summary | ✅ done | this commit |

Both suites (`local_storage_projection` 97 tests; `croft-chat` 25 test binaries) + clippy green at
every commit boundary; the BIP39 spike is green in isolation; zero new warnings vs the pre-existing
baseline (`tables.rs` `STATE_CHECKPOINTS` unused-const is a RUN-07-era warning, untouched).

---

# PART 1 — build

## 1A — riders (commit `3b69093`)

1. **Bannered the original X3 record.** One blockquote atop
   `local_storage_projection/X3-CROSS-PACKAGE-SWEEP.md`: its central conclusion (the 61 survivors are
   cross-package-covered instrument artifacts) was partially refuted by `X3-AUTOMATED-SWEEP.md`
   (RUN-07) — 31 were dead-duplicate `fold_auth` survivors never linked into the consumer path; see
   the `fold-auth-duplicate` register row. Record otherwise preserved verbatim.
2. **Filed the Vouch residual** as `EXPERIMENT-BACKLOG.md §2d`: the fold's I5 Vouch payload gate
   (`fold_derived.rs:447–472`) is uncovered by both suites (the 10 justified survivors in
   `X3-AUTOMATED-SWEEP.md`). Retirement condition: consumer-path Vouch tests that kill the 10, or an
   explicit experiment-grade justification recorded against the sweep. Register/spec tag: none.
3. **Frontier-head pinning note** in `alpha/thinking/reconciliation-horizon.md §3`: EXP-H1 (RUN-07)
   found the naive last-ingested head is arrival-order-dependent, so the experiment manifest leads
   with an order-independent digest of converged state; when `[gates-release]` pins the §7.6.9
   manifest encoding, decide between that digest and the sorted set of DAG tip hashes. Deferred.
4. The `fold_auth` deletion authorization is recorded (owner-confirmed 2026-07-15) in the reviews-log
   RUN-08 entry.

## infra — the Proofs fold-in (commit `b3ecf8f`)

The `mls-welcome-over-iroh` spike and the conformance-core live in the **Proofs** repo, absent from
this discovery-only session, so both were unbuildable here. The owner exported Proofs and made it
authoritative in discovery. It is folded to `discovery/alpha/Proofs/` (frozen; `FROZEN-NOTICE.md`) —
exactly where the folded spikes' Cargo path-deps (`../../../../Proofs/lineage-groups/crates/…`)
resolve, so `mls-welcome-over-iroh`, `media-sframe-spike`, and `altdrive-spike-faithful-sync` build
with zero Cargo edits. Firewall: `lineage-groups` + `conformance` are MLS/transport/proof code; they
do not touch the I9 trust tier.

## 1C — BIP39 paper-recovery round-trip, the Tier-1 lock (commit `93e0be3`)

New standalone spike `alpha/experiments/bip39-recovery-roundtrip` (own lockfile; `bip39 =2.2.2`,
`dryoc =0.8.0`, `zeroize =1.9.0`). **11 tests green, clippy clean.**

- **Round-trip bit-exact:** recoveryKey (32 B) → 24-word BIP39 English mnemonic → recoveryKey,
  byte-for-byte identical.
- **Standard BIP39 English KATs, both directions** (canonical 256-bit Trezor vectors): `00…00`→`art`,
  `7f…7f`→`title`, `80…80`→`bless`, `ff…ff`→`vote`. Forward (entropy→mnemonic) and backward
  (mnemonic→entropy, checksum validated) both hold. The KAT earned its keep: an initial transcription
  of the `80…80` checksum word as `bunker` was caught by the forward assertion and corrected to the
  official `bless`.
- **Checksum-failure negatives — rejected, never silently accepted:** a corrupted word (`abandon`×24
  → InvalidChecksum), a transposed pair (first two words of `7f…7f` swapped → rejected), an
  out-of-wordlist word (UnknownWord), and a wrong word count (23 words → BadWordCount). Each is an
  `Err`; none decodes to a wrong key.
- **secretbox wrap/unwrap:** masterKey secretbox-wrapped (XSalsa20-Poly1305 via `dryoc`, the vetted
  crate `altdrive-core` uses) under the recoveryKey unwraps bit-exact; a wrong key, a tampered
  ciphertext, and a too-short blob each fail cleanly (authentication error, never a plaintext).

Secret newtypes are `Zeroize` + no `Debug`. Crate choice is stated experiment-grade, **not** a
`[gates-release]` decision. Firewall: no share-splitting, no release predicate, no threshold anything
— the spike proves the lock exists and round-trips; who may open it is I9.

Conditional edits (on green): backlog §6g Sketched → ✅ done (RUN-08); MASTER-INDEX I9 notes the spike
landed; `open-threads.md §2` gains the one-line Tier-1-lock note (the trust tier remains the open call).

## 1B — conformance: key-distribution over the wire (commit `5534e05`)

**The category-unlock reading (recorded before editing).** The task premised an in-repo harness
emitting cats 1–6 with 7/8/9 gated. Ground truth after the Proofs fold-in is different, and it is the
load-bearing finding of this phase:

- The reference conformance-core (`alpha/Proofs/lineage-groups/crates/conformance`, `run-vectors`)
  **re-proves 66/0 across cats 1–9 in-environment**: cat 1 derivations (5), cat 2 signing (2), cat
  3+4 fold/thresholds (3), cat 5 revocation mechanics (2), **cat 5b revoke-authority *mechanism*
  (4)**, cat 6 reconcile C1–C10 (10), **cat 7 adversarial AR-1…AR-6 (8, real Rust)**, **cat 8
  visibility (11, TS-authoritative)**, **cat 9 freshness (3, TS-authoritative)**, manifest (18).
- So cats 7/8/9 and the revoke-authority *mechanism* are **already emitted** — the §10.5 footnote's
  "not yet emitted" is superseded for the vectors themselves. The genuine `not_yet_emitted` residual
  (the emitter's own manifest note) is the revoke-authority **over-the-wire distribution +
  co-sign-vs-vote ordering**.
- Of the two over-the-wire pieces the footnote gated on, the **key-distribution piece alone unlocks**
  the honest *sourcing* of the verifying-key/standing registry the standing-dependent vectors rest on
  (conformance-suite honesty boundary 1). The **threshold-revoke-over-wire + authority-ordering piece
  requires the revocation-authority trust model (I9) — out of scope, firewall.**

**Wired it (loopback grade met).** `mls-welcome-over-iroh` **reproduced green-real in-environment**: a
real 1466-byte openmls Welcome crosses a real iroh connection; the joiner derives the identical MLS
exporter secret (`epoch_secret_match: true`) and the identical lineage fold (alice's two devices → one
actor, bob → one; `fold_matches: true`). Captured as a RUN-08 relay-lab-run
(`relay-lab-runs/C-mls-welcome-2026-07-15-run08/`, byte-identical to the 2026-06-17 archive) with a
`serves:` header; the 66/0 re-prove log is co-located.

**Emitted-vector list.** The unlocked emission is realized in the existing formats without a
vector-format change: (i) the key-distribution evidence as the RUN-08 relay-lab-run verdict (the
spike's own language-neutral JSON), and (ii) the full conformance suite re-proven 66/0 by the existing
`run-vectors` harness. **No new over-the-wire vector schema was authored** — that would be a
vector-format change (stop rule) — and **no iroh was wired into the frozen conformance emitter**
(beyond emission plumbing); the emitter-integration is recorded as the residual.

**Conditional edits — applied vs withheld.**

| Edit | Condition | Decision | Why |
|---|---|---|---|
| §10.5 footnote reconciled | green emission (mls-welcome PASS + 66/0) | **APPLIED** | Names 66/0 emitted, (a) key-distribution `Verified` at loopback grade, (b) threshold-revoke gated (I9). |
| F7 annotation + summary row | same | **APPLIED** | RUN-08 update paragraph + enriched row. |
| MASTER-INDEX I1/I2; backlog §6d | same | **APPLIED** | I1 green-real reproduced; I2 cats emitted 66/0; §6d rows updated. |
| part-2-changelog entry | Part 2 edit made | **APPLIED** | §10.5 reconciliation entry. |
| A **new over-the-wire vector** | — | **WITHHELD** | Needs a new vector schema (stop rule); firewall bars the authority half. |
| §7.2 R7 / any status tag move | — | **WITHHELD** | Part 1B lands the key-distribution piece only; no trust-tier tag moves (firewall). |

**Stop rules — not triggered.** No vector-format change; no `mls-replant` production-logic change; the
firewall (threshold-revoke, co-sign-vs-vote, revoke-authority) untouched.

---

# PART 2 — trace (spec ↔ experiment traceability)

Ran against the post-Part-1 branch state. **No status tag was moved by Part 2** — Part 1's conditional
annotations are the run's only tag-adjacent edits.

## 2.0 inventory (counts)

- **A.9 ladder:** 10 rungs (`Verified`, `Verified-RFC`, `Modeled`, `Measured`, `Established`,
  `Design`, `Synthesis`, `Load-bearing, unearned`, `[gates-release]`, `[confirm]`).
- **Part 2 status tags:** 145 backtick-wrapped ladder tags; ~107 at or above `Modeled` (≈75
  `Verified`, ≈30 `Verified-RFC`, ~5 `Modeled`, 2 `Measured`).
- **SPEC-DELTA live code tags:** 2 (both `hermetic-gossip`, `iroh_convergence.rs` + `iroh_bus.rs`).
- **Register rows:** 2 Active (`hermetic-gossip`, `fanout-single-run`), 7 Reconciled, 4
  declared-caveats.
- **Backlog:** ~30 items; the RUN-08 additions are §2d (Vouch, open) and §6g (BIP39, done).
- **Experiment reports:** 5 load-bearing now carry a `Serves:` header (plus the pre-compliant BIP39
  README and the RUN-08 relay-lab-run `serves`). **Spec-earning tests:** 21 gained a §-ref
  doc-comment; `competing_quorums.rs` was the model.

## 2.1 forward links

- **2.1b resolution:** every cited test and report path checked this run resolves on grep/`ls` (see
  the summary's verification pass); every RUN number matches its summary. **0 dead pointers.**
- **2.1a/c/d:** the reconciled governance claims carry inline test+RUN evidence; the §4–§6/§10
  substrate band resolves to RFC/substrate references, not a test+RUN. The standardized
  `(evidence: …, RUN-NN[, grade])` parenthetical exists nowhere in Part 2 (0 occurrences) →
  recorded as **FND-T1 / FND-T4** rather than invented per-sentence.

## 2.2 backward links

- **2.2a (FIX):** `Serves:` headers added to `X3-AUTOMATED-SWEEP.md`, `FANOUT-M1.md`,
  `automerge…/REPORT.md`, `mls-replant/README.md`, `replant-continuity/README.md`.
- **2.2b (FIX, comment-only commit `87ca3ce`):** 21 spec-earning tests gained an `Earns/bounds: Part
  2 §X.Y` doc-comment line.
- **2.2c (FIX):** `handcrafted-assertions` register row gained its retirement landing (2026-07-13
  reconciliation). Every other retired row already names its run; every register spec/evidence
  pointer resolves.
- **2.2d (FIX):** backlog §2d (Vouch) and §6d/§6g rows carry spec-section/register pointers +
  retirement conditions / landing runs.

## 2.3 the living artifact

`beta/drystone-spec/EVIDENCE-MAP.md` — one row per tagged claim ≥ `Modeled` (governance-fold core,
delivery/transport, the beam + Tier-1 lock, and the substrate band), each with bounds, evidence,
register, gates. Header documents the regeneration recipe and the **index-not-source** rule. A pointer
sentence was added to the §8.2 preamble; MASTER-INDEX and part-2-changelog updated (Rule 15/changelog
bookkeeping).

## 2.4 vocabulary conformance

- Off-ladder token `Reviewer-judgment` in live §10.4 text → **FND-T5** (proposed A.9 mapping;
  not auto-rewritten).
- Former-tag vocabulary (`green-real`/`green-model`/`not_yet_emitted`) confined to alpha-tier/staging
  docs → **FND-T6** (one instance introduced into the §10.5 draft was removed this run).
- Bound-qualifier spelling drift `real-NAT` vs `real NAT` → **FND-T7** (canonicalization deferred to
  settlement).

## Link-resolution statistics

- **Claims indexed (tags ≥ `Modeled` + the beam + Tier-1 lock):** ~107.
- **Fully linked (named test/report + RUN + bound, or a resolved primary-source anchor):** the
  governance-fold core (≈14 rows), the RUN-driven substrate rows (fan-out, automerge, conformance,
  key-distribution, BIP39), and the `Verified-RFC` literature band — all resolve.
- **Carrying a FINDING id (not fully linked in the target form):** the ~40-tag substrate `Verified`
  band lacking a test/RUN pointer (**FND-T1**) and the missing standardized parenthetical
  (**FND-T4**).
- **FINDINGs opened:** 7 (`FND-T1…T7`) — 0 HIGH, 3 MED, 4 LOW. **FIXes applied:** 5 report headers,
  21 test doc-comments, 1 register pointer, backlog pointers, and the EVIDENCE-MAP index.

---

## Files (high level)

**New:** `bip39-recovery-roundtrip/` (crate); `alpha/Proofs/` (folded corpus + FROZEN-NOTICE);
`iroh/relay-lab-runs/C-mls-welcome-2026-07-15-run08/`; `beta/drystone-spec/EVIDENCE-MAP.md`; this
summary. **Modified:** the three riders (1A); §10.5 + §8.2 + changelog (Part 2 spec); F7
(proposed-changes); MASTER-INDEX; EXPERIMENT-BACKLOG; open-threads; SPEC-DIVERGENCE-REGISTER;
CONSISTENCY-FINDINGS (RUN-08 section); 21 test doc-comments; 5 report `Serves:` headers.
