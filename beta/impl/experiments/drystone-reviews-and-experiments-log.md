# Drystone: reviews and experiments log

`Purpose: a durable record of external feasibility reviews and convergence-experiment runs against the specs, each with its verdict and the spec changes it produced. This is a catalogue, not an authoritative spec. The three authoritative docs are Part 1 (principles), Part 2 (mechanics), and the terms-and-definitions file (conventions-and-decisions).`

## 2026-07-04, Feasibility review, first pass

Scope: the two-part spec, but the reviewer could not read the spec files and reconstructed internal-mechanics findings from a description, tagging them [Design, spec-unverified]. The independent primary-source verification was complete and is the load-bearing contribution.

Verified accurate against primary sources: RFC 9420 and RFC 9750 (application-data carriage, ordering in §15.3, the vector ceiling, padding, external commits, ReInit non-atomicity, epoch_authenticator, FS and PCS, metadata, the DS and AS trust split), CALM Theorem 1, CRDT strong-eventual-consistency, the Matrix CVE-2025-49090 and MSC4289 / MSC4291 / MSC4297, BLAKE3, and iroh 1.0.

Verdicts: Bar 1 (implementability) conditionally feasible, blocked only by the open `[gates-release]` encodings. Bar 2 (cryptographic and security soundness) sound on the MLS substrate. Bar 3 (load-bearing open problems) the completeness beam is closable but unearned, and the §4 / §7 hash split is low-difficulty.

Spec effect: none directly. The internal-mechanics findings were carried into the second pass to be checked against the text.

## 2026-07-04, Feasibility review, second pass

Scope: read Part 1 and Part 2 in full, and re-verified the external claims.

Corrections to the first pass: withdrew the forward-secrecy-versus-durable-history tension (W1), since §8.1 is correct and scopes forward secrecy to the key schedule rather than to ciphertext. Downgraded the metadata-floor finding, since §6.4 is drawn more precisely than the review was.

Cross-check outcome: most internal-mechanics findings were already handled in the text. The beam was classified as liveness-only and safety-monotone under a fail-closed gate, which matches §7.3.8. Carry-not-bind (E6) was already stated completely in §6.2.2. The §4 length-extension check was run and found no §4 use is a secret-prefix MAC, so re-basing on BLAKE3 is a clean substitution.

Spec effect: one addition. The §4 length-extension check and the bundle-and-re-prove wire-freeze caution were recorded in the Part 2 Appendix B hash entry. Everything else the review recommended was already present, often more thoroughly than the review's own draft.

## 2026-07-04, Convergence experiment v2

Fold under test: a faithful reference fold implementing R1 through R4 and the A12 layered fold, not a production fold, of which none exists in the repository.

Stage 1 (R1 through R4 and A12, properties A through F with D6 and D7): 27 tests, all pass, including the discriminating D6 and D7 that a flat id-only fold fails. Establishes permutation-invariance and the resolution rules as reference-model logic.

Stage 2 (gap-detection): referenced-gap detection passes, where a node holding a fact whose referenced predecessor is absent returns a gap error rather than folding an incomplete set as complete. The unreferenced-tail case is the documented limit, which is the completeness-ahead beam. Convergence-after-fill passes for a single gap and for multi-node reconciliation.

Stage 4 (finality, groups G through K): 16 tests, all pass. Quorum folding (A1), non-exclusive recognition (A2), the ceiling (A3), and the now (A7).

Stage 3 (adversarial scheduler, equivocation surfacing a fork, bounded model checking): specified, not implemented.

Beam status: unchanged, `Load-bearing, unearned`. The experiment demonstrates completeness behind a checkpoint (referenced-gap detection) but not completeness ahead (the unreferenced tail), which is the open half.

Spec effect: R1 through R4, A12, and the A1 through A3 and A7 finality mechanics are now reference-model-demonstrated. Two Part 2 status upgrades from `Design` to `Modeled`: the A12 tier-boundary projection consistency (§7.3.2) and the R3 no-fold-time-rejection with referenced-gap detection (§7.3.2). The experiments-consolidated file §1 records the full v2 result. System-level order-independence stays `Load-bearing, unearned` pending the beam.
