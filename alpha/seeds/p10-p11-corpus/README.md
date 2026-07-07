# p10 / p11 Drystone corpus — frozen snapshot (2026-07-07)

The full 30-file `ten-willlow.zip` corpus plus the 2-file `ten-feasibility.zip`, frozen here as provenance.
This is a **full workspace snapshot** from the web sessions, spanning three maturity iterations (p9 → p10 →
p11), not a clean incremental batch. It is preserved intact because it contains the newest spec versions and
because a spec-swap reconciliation is pending (below).

**What's current vs superseded in this snapshot:**
- `p11-full-part2-mechanics.md` (470K) is the **newest Part 2** (a rebuild that supersedes both the p9
  consolidation currently in `beta/drystone-spec/part-2-certifiable-design.md` and `p10-full-part2`). In the
  source session its status read "Supersedes p10 as authoritative Part 2 **on confirmation**."
- `p10-full-part1-principles.md` (104K) is the **current Part 1** (supersedes the committed p9 part-1).
- `p10-full-part2-mechanics.md` is superseded by p11.
- The `p10-drystone-*` companions (asset-keying, history-durability, fold-semantics, governance-finality,
  liveness-freshness, authority-and-complement, fact-and-chain-representation, scaling-and-ordering,
  cast-beat-map, conventions-and-decisions, doc-method, social-mapping) are the design corpus the
  consolidation drew from.
- `p10-drystone-atproto-ecosystem.md` and `social-lexicon-group-research-brief.md` were filed to
  `beta/cairn/` (ecosystem).
- `p10-drystone-coffee-shop.md` / `-elevator-pitch.md` are the shorter tellings.
- `drystone-feasibility-review-v2.md` (filed to `beta/drystone-spec/`) supersedes `-review.md`.
- The experiment docs (`drystone-experiments-consolidated`, `-convergence-experiment-brief-v3` and older,
  `-reviews-and-experiments-log`, `-fold-coverage-audit`) are the validation corpus.
- The `p9-drystone-*` files are superseded p9 leftovers, kept for provenance.

**Pending reconciliation (flagged for the user, not executed):** swapping the canonical
`beta/drystone-spec/` spec (currently the batch-9 p9 consolidation) to `p10-full-part1` + `p11-full-part2`.
Deferred because (a) p11 was "authoritative on confirmation," not confirmed, (b) the user's own batch-9
lesson was to audit a big consolidation for web-agent content-loss before trusting it, and (c) the p10
companions + experiments then need routing into `impl/`. This is the next spec pass, once confirmed.

**Preservation status: preserved-verbatim** (byte-identical to the two zip payloads). See
`../transcripts/RAW-ARTIFACTS-MANIFEST.md` for the batch-ten intake.
