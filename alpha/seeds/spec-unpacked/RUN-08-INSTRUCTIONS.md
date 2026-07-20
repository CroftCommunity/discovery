# RUN-08 — Conformance half one, the BIP39 Tier-1 lock, and three riders

`Branch: fresh off main (RUN-07 merged), e.g. run-08-conformance-and-recovery. Phase A markdown;
Phases B and C code, one commit (or series) each.`

## The I9 firewall (governs the whole run)

The identity/key-recovery **trust tier** — who may trigger a release, threshold shares, quorum
social recovery vs VC issuer, the revoke-authority model — is the owner's open design call (I9).
Nothing in this run touches it: Phase B stops short of the revoke-authority vector and anything
gated on threshold-revoke-over-wire; Phase C builds the Tier-1 **lock only**. If any task appears
to require a trust-tier decision, stop that task and report.

## Phase A — riders (markdown, first commit)

A1. **Banner the original X3 record.** One line atop
    `local_storage_projection/X3-CROSS-PACKAGE-SWEEP.md`: its central conclusion ("the 61
    survivors are cross-package-covered instrument artifacts") was partially refuted by
    `X3-AUTOMATED-SWEEP.md` (RUN-07) — 31 of the 61 were dead-duplicate `fold_auth` survivors,
    never linked into the consumer path; see the `fold-auth-duplicate` register row. Frozen record
    otherwise untouched.

A2. **File the Vouch residual.** New EXPERIMENT-BACKLOG item: Vouch payload-validation is
    genuinely uncovered (the 10 justified survivors in `X3-AUTOMATED-SWEEP.md`); retirement
    condition: consumer-path Vouch tests that kill them, or an explicit experiment-grade
    justification recorded against the sweep. Cross-reference the sweep report.

A3. **Frontier-head pinning note.** In `alpha/thinking/reconciliation-horizon.md`, add a short
    note under the horizon-checkpoint section: EXP-H1 (RUN-07) found the naive last-ingested head
    is arrival-order-dependent, so the experiment's manifest leads with an order-independent
    digest of converged state; when `[gates-release]` pins the §7.6.9 manifest encoding, decide
    whether the first element is that converged-state digest or the sorted set of DAG tip hashes
    (closer to §7.3.3's governance-head commitment). Decision deferred to the pinning pass.

A4. Reviews-log RUN-08 entry (end of run) additionally records: the fold_auth deletion
    authorization confirmed by the owner 2026-07-15.

## Phase B — conformance categories, half one (key-distribution over the wire)

Context: Part 2 §10.5's ledger footnote — categories 7/8/9 and the revoke-authority vector are
"specified but not yet emitted, gated on two over-the-wire pieces." This phase lands the
key-distribution piece; the threshold-revoke piece stays gated (firewall).

1. Read first: the §10.5 ledger and its footnote; the F7 annotation; the existing conformance
   harness that emits categories 1–6 (locate it empirically — likely in or beside `mls-replant`);
   the `alpha/experiments/iroh/crates/mls-welcome-over-iroh` crate and the
   `relay-lab-runs/C-mls-welcome-2026-06-17` record.
2. Determine from those sources exactly which of categories 7/8/9 the key-distribution piece
   alone unlocks; record the reading in the summary before writing code. Anything requiring
   threshold-revoke-over-wire is out of scope.
3. Wire it: drive a real MLS Welcome across a real iroh connection (loopback grade is the
   requirement; a relay-path run is a stretch goal, not required) and emit the unlocked
   conformance vectors through the existing harness in its existing format — no changes to the
   vector format itself.
4. Conditional edits, only on green emission: update the §10.5 footnote and the F7 annotation to
   name precisely what is now emitted (at loopback grade) versus still gated; MASTER-INDEX,
   backlog, changelog, reviews-log accordingly.
5. Stop rules: vector-format changes; mls-replant production-logic changes beyond emission
   plumbing; anything touching the firewall.

## Phase C — BIP39 paper-recovery round-trip (the Tier-1 lock)

Per the backlog's sketched item: "recoveryKey ↔ 24-word mnemonic (KAT-verified) then
secretbox-wrap the masterKey — cheapest first step."

1. New spike crate, placed to match sibling layout (e.g. beside the other recovery/iroh spikes).
2. Assertions:
   - recoveryKey → 24-word BIP39 mnemonic → recoveryKey round-trips bit-exact;
   - the implementation passes the standard BIP39 English-wordlist test vectors (KAT), including
     checksum-failure negatives (a corrupted word or transposed pair is rejected, not silently
     accepted);
   - masterKey secretbox-wrapped under the recoveryKey unwraps bit-exact; wrong-key unwrap fails
     cleanly.
3. Dependencies: vetted crates only, versions pinned, and the report states explicitly that crate
   choice is experiment-grade, not a `[gates-release]` decision.
4. On green: backlog item Sketched → done (RUN-08); one-line status note on the open-threads
   recovery item (Tier-1 lock spike landed; the trust tier remains the open call) — open-threads
   is a living doc, this is in bounds.
5. Firewall reminder: no share-splitting, no release predicate, no threshold anything. The spike
   proves the lock exists and round-trips; who may open it is I9.

## Guardrails (all phases)

- Both suites + clippy green at every commit boundary; zero new warnings vs baseline.
- Verbatim-anchor rule for every doc edit; minimal diffs; conditional spec/ledger edits only on
  the stated evidence — when in doubt, report instead of updating.
- No wire/byte encodings pinned anywhere; `[gates-release]` untouched.

## Output

`alpha/experiments/RUN-08-SUMMARY.md`: per-phase status; Phase B's category-unlock reading and the
emitted-vector list; Phase C's KAT results including the negatives; every conditional edit applied
or withheld and why; files changed.
