# RUN-08 — Two parts, one branch: build (Part 1), then trace (Part 2)

`Branch: fresh off main (RUN-07 merged), e.g. run-08-build-and-trace. Part 1 lands the code work
and riders; Part 2 then runs the spec↔experiment traceability pass against the branch's own
post-Part-1 state — which is the point of combining them: the new work gets traced the day it is
born, and no separate run has to be gated on a merge. Commits: Part 1 phases one commit (or
series) each; Part 2 markdown in one commit plus one comment-only code commit.`

## Run-wide rules

- **The I9 firewall.** The identity/key-recovery trust tier (who may trigger a release, threshold
  shares, quorum-vs-VC, the revoke-authority model) is the owner's open call. Nothing in this run
  touches it. Part 1B stops short of the revoke-authority vector; Part 1C builds the Tier-1 lock
  only. Any task that appears to need a trust-tier decision: stop it, report.
- **FIX vs FINDING** (Part 2, and any Part 1 doc surprise): mechanical and provable → FIX;
  judgment or meaning-adjacent → FINDING in CONSISTENCY-FINDINGS-2026-07.md.
- **Frozen-record rule** with its ratified narrow exception (mechanical normalization of
  tag-format/paths on preserved blocks).
- Both suites + clippy green at every commit boundary; zero new warnings; verbatim-anchor rule for
  every doc edit; minimal diffs; conditional edits only on their stated evidence.
- **Pre-compliance:** every report, README, and spec-earning test Part 1 creates is written
  already carrying Part 2's templates (the `Serves:` header and the test doc-comment, defined in
  Part 2 §2 below), so Part 2 traces them instead of retrofitting them.

---

# PART 1 — build: riders, conformance half one, the BIP39 lock

## 1A — riders (markdown, first commit)

1. **Banner the original X3 record.** One line atop
   `local_storage_projection/X3-CROSS-PACKAGE-SWEEP.md`: its central conclusion ("the 61
   survivors are cross-package-covered instrument artifacts") was partially refuted by
   `X3-AUTOMATED-SWEEP.md` (RUN-07) — 31 of the 61 were dead-duplicate `fold_auth` survivors,
   never linked into the consumer path; see the `fold-auth-duplicate` register row. Otherwise
   untouched.
2. **File the Vouch residual.** New EXPERIMENT-BACKLOG item: Vouch payload-validation is genuinely
   uncovered (the 10 justified survivors in `X3-AUTOMATED-SWEEP.md`); retirement condition:
   consumer-path Vouch tests that kill them, or an explicit experiment-grade justification
   recorded against the sweep.
3. **Frontier-head pinning note** in `alpha/thinking/reconciliation-horizon.md`: EXP-H1 (RUN-07)
   found the naive last-ingested head is arrival-order-dependent, so the experiment's manifest
   leads with an order-independent digest of converged state; when `[gates-release]` pins the
   §7.6.9 manifest encoding, decide between that converged-state digest and the sorted set of DAG
   tip hashes (closer to §7.3.3's governance-head commitment). Decision deferred to the pinning
   pass.
4. The end-of-run reviews-log entry additionally records: the fold_auth deletion authorization
   confirmed by the owner 2026-07-15.

## 1B — conformance categories, half one (key-distribution over the wire)

Context: Part 2 §10.5's ledger footnote — categories 7/8/9 and the revoke-authority vector are
"specified but not yet emitted, gated on two over-the-wire pieces." This lands the
key-distribution piece; the threshold-revoke piece stays gated (firewall).

1. Read first: the §10.5 ledger + footnote; the F7 annotation; the existing conformance harness
   that emits categories 1–6 (locate empirically — likely in or beside `mls-replant`); the
   `alpha/experiments/iroh/crates/mls-welcome-over-iroh` crate and
   `relay-lab-runs/C-mls-welcome-2026-06-17`.
2. **Record the reading before coding:** which of categories 7/8/9 the key-distribution piece
   alone unlocks, per those sources. Anything requiring threshold-revoke-over-wire is out of
   scope.
3. Wire it: drive a real MLS Welcome across a real iroh connection (loopback grade is the
   requirement; a relay-path run is a stretch goal) and emit the unlocked conformance vectors
   through the existing harness in its existing format — no changes to the vector format itself.
4. Conditional edits, only on green emission: update the §10.5 footnote and F7 annotation to name
   precisely what is now emitted (at loopback grade) versus still gated; MASTER-INDEX, backlog,
   changelog.
5. Stop rules: vector-format changes; mls-replant production-logic changes beyond emission
   plumbing; anything touching the firewall.

## 1C — BIP39 paper-recovery round-trip (the Tier-1 lock)

Per the backlog's sketched item: "recoveryKey ↔ 24-word mnemonic (KAT-verified) then
secretbox-wrap the masterKey — cheapest first step."

1. New spike crate, placed to match sibling layout.
2. Assertions: recoveryKey → 24-word BIP39 mnemonic → recoveryKey round-trips bit-exact; the
   standard BIP39 English-wordlist KATs pass, **including checksum-failure negatives** (a
   corrupted word or transposed pair is rejected, never silently accepted); masterKey
   secretbox-wrapped under the recoveryKey unwraps bit-exact; wrong-key unwrap fails cleanly.
3. Dependencies: vetted crates only, versions pinned; the report states crate choice is
   experiment-grade, not a `[gates-release]` decision.
4. On green: backlog item Sketched → done (RUN-08); one-line status note on the open-threads
   recovery item (Tier-1 lock spike landed; the trust tier remains the open call).
5. Firewall: no share-splitting, no release predicate, no threshold anything. The spike proves the
   lock exists and round-trips; who may open it is I9.

---

# PART 2 — trace: the spec ↔ experiment traceability pass

Runs against the branch state after Part 1's commits. **This part never moves a status tag** —
Part 1's conditional annotation updates are the run's only tag-adjacent edits; from here on, a tag
that looks wrong is a FINDING, no matter what.

## What "correct annotation" means

- **Forward (spec → evidence):** every Part 2 status tag above `Design` resolves to its evidence —
  named test(s), report file, RUN number — with environment bounds stated (loopback / substrate /
  cross-package / single-run / real-NAT-pending).
- **Backward (experiment → spec):** every experiment crate, report, spec-earning test, SPEC-DELTA
  tag, register row, and backlog item names the spec section and claim it serves.

## 2.0 — inventory (no edits)

Read `conventions-and-decisions.md` A.9 (the evidence ladder — the canonical tag vocabulary) and
A.11. Build, in the summary: every evidence tag in Part 2 (and any in Part 1) with its section and
evidence parenthetical; every SPEC-DELTA code tag; every register row; every backlog status; every
experiment report and spec-earning test file — including Part 1's new ones.

## 2.1 — forward links

a. Every tag ≥ `Modeled` carries an evidence pointer (test name(s)/report path, RUN). Missing but
   unambiguous in exactly one RUN summary or report → add (FIX); ambiguous → FINDING.
b. Every cited test exists under the cited name (grep); every cited path resolves; every RUN
   number matches the summary that did the work. Fix renames (FIX).
c. Environment bounds: wherever the register or a report bounds a claim, the tagged sentence
   states the bound. Missing bound with fixed wording elsewhere in the doc set → FIX by reusing it
   verbatim; needing fresh wording → FINDING with proposed sentence.
d. Standardize the evidence parenthetical to `(evidence: <test or report>, RUN-NN[, grade])`,
   applied mechanically only where all components already exist; otherwise FINDING.

## 2.2 — backward links (templates; the comment-only code commit)

a. **Report headers.** Every experiment report and spike README opens with:
   `Serves: Part 2 §X.Y (<claim, one clause>) — earns/bounds: <tag or bound> — register: <row(s)
   or none> — landed: RUN-NN.` Add where missing (FIX), populated only from verified 2.0/2.1 data;
   uncertain → FINDING.
b. **Spec-earning tests.** Every test file whose green earns or bounds a spec claim carries a
   doc-comment naming the section and claim (the `competing_quorums.rs` header is the model). Add
   where missing.
c. **SPEC-DELTA and register.** Every SPEC-DELTA tag resolves to a live register row; every row's
   spec and evidence pointers resolve; every retired row names its retirement run. Dead pointers →
   FIX; substance → FINDING.
d. **Backlog.** Every open item names the spec section or register row it would move plus its
   retirement condition; every done item names its landing run. Unambiguous missing pointers → FIX.

## 2.3 — the living traceability artifact

Create `beta/drystone-spec/EVIDENCE-MAP.md`: one row per tagged claim — section | claim (short
clause) | tag | bounds | evidence (tests, reports, RUN) | register rows | gates
(`[gates-release]`/X-items). Populated ONLY from links verified in 2.1/2.2; unresolved links
appear with their FINDING id, never an invented pointer. Header documents the regeneration recipe
(the 2.0 scan) and the rule that the map is an index and never sources a status — the spec
sentence is authoritative. Add one pointer sentence in Part 2 (the §8.2 preamble or the Map
header, wherever house style allows), plus MASTER-INDEX and Rule 15/changelog bookkeeping.

## 2.4 — annotation vocabulary conformance

a. A.9 is the only tag ladder. Off-ladder tag-like tokens in live text → FINDING each with the
   proposed A.9 mapping; do not auto-rewrite body-text tags (the normalization exception covers
   preserved blocks' tag *format*, not live sentences' tag *meaning*).
b. One spelling per bound qualifier (loopback / substrate / cross-package / single-run /
   real-NAT): FIX pure spelling drift; FINDING where two spellings might mean two things.

---

## Output (one summary, both parts)

1. `alpha/experiments/RUN-08-SUMMARY.md`: Part 1 — per-phase status, 1B's category-unlock reading
   and emitted-vector list, 1C's KAT results including negatives, every conditional edit applied
   or withheld and why. Part 2 — the inventory counts, per-phase FIX list, and link-resolution
   statistics (N claims, N fully linked, N FINDINGs).
2. CONSISTENCY-FINDINGS-2026-07.md: new `## Traceability findings (RUN-08)` section
   (HIGH/MED/LOW, quoted text, proposed resolution) — these get the walk-through and a settlement
   pass, same as RUN-05 → RUN-06.
3. `beta/drystone-spec/EVIDENCE-MAP.md` as specified; reviews-log gets one RUN-08 entry covering
   both parts (including the fold_auth confirmation line from 1A.4).
