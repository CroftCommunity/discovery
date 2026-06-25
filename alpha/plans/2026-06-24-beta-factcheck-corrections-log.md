# Beta fact-check — corrections accounting log

date: 2026-06-24

purpose: the full, auditable record of every correction applied to `discovery/beta/` (and the one
additive `alpha/` file, `BETA-ROLLUP.md`) in response to the findings in
`2026-06-24-beta-factcheck-report.md`. Each entry maps a report finding ID → the file:line touched →
before → after → source of truth. Items deferred for the user's decision and items intentionally left
unchanged are listed at the end.

scope: corrections were applied at the **beta layer**. The frozen `alpha/` corpus was not edited; the
only `alpha/` change is to `BETA-ROLLUP.md`, which the maturity method designates as the single additive
file for this transition. Where a quote error originated in alpha (Ashby, Beer), it was re-flagged/
corrected in beta and the alpha "Verified" appendix is noted as unreliable here — alpha itself stays
frozen.

re-verification: before applying the `01` quote fixes, the remaining alpha-"Verified" classical set
(Socrates / Mill / Hayek / Scott) plus the items under correction were re-verified against primary or
reputable secondary sources via web. Results drove the exact replacement strings below.

---

## MAJOR corrections (6)

### M1 — `beta/01` §2.5 — Ashby misquote ("absorb" → "destroy")
- Before: `"Only variety can absorb variety." — W. R. Ashby, An Introduction to Cybernetics (1956)`
- After: `"Only variety can destroy variety." — W. R. Ashby, An Introduction to Cybernetics (1956), p. 207 (the Law of Requisite Variety)`
- Source: Ashby 1956, §11/7 (panarchy.org full-text; widely cited p. 207). "Absorb variety" is a later
  restatement (popularized via Beer), not Ashby's text.
- Also: the second Ashby block ("When the variety or complexity of the environment exceeds…") was
  reframed from an implied verbatim quote to an explicit paraphrase/gloss; the §2.5 verification line
  now reads "Verified (requisite-variety line, p. 207; absorb→destroy corrected 2026-06-24); the
  survival-condition restatement is a paraphrase/gloss, not a verbatim Ashby line."

### M2 — `beta/01` §2.5 — Beer quote not sourceable; de-quoted to paraphrase
- Before: `treat computers, dashboards, simulations "as aids to human viability, not as excuses for automatic command." — S. Beer, Brain of the Firm (1972)`
- After: paraphrase, no quotation marks — `Beer held that computers, dashboards, and simulations should serve as aids to human viability, not as excuses for automatic command — technology in service of human autonomy, not technocratic control. — paraphrasing S. Beer, Brain of the Firm (1972), on humane management cybernetics`
- Source: the quoted wording traces to a secondary characterization (archania.org), not Beer's 1972
  text; could not be sourced verbatim. Verification line changed from "Verified" to "paraphrase," and
  the Cybersyn nuance corrected: "functioned until destroyed from outside (the 1973 coup)" rather than
  the bare "survived."

### M3 — `beta/02` §(history) — wrong COHESION section for trap/balance reconciliation
- Before: `*(Collapses COHESION §30/§34: the trap-reading and the balance-reading reconciled as one lesson.)*`
- After: `*(Collapses COHESION §34: …)*`
- Source: `COHESION.md` §34 is the trap-vs-balance refinement; §30 is the Groundmist/Hard-Fork-23
  dialogue (already correctly cited at the §6/provenance Hard-Fork-23 material). §30 removed from this
  in-body cross-reference.

### M4 — `beta/04` "Carried open items" — stale spec-vs-code claim reframed as resolved
- Before: "the tagged pre-images are now canonical in `lineage-core::ids`, but the spec text needs to
  be reconciled to match."
- After: "Spec-vs-code reconciliation — surfaced AND resolved (2026-06-17). … the tagged pre-images are
  now canonical in both the spec (CROFT-PROTOCOL §2, incl. the 2026-06-17 addendum) and the code
  (lineage-core::ids) … No live spec/code divergence remains; this is no longer an open item."
- Source: `proof-ledger.md` ("RESOLVED"), `test-narrative.md` ("surfaced and resolved 2026-06-17"),
  `CROFT-PROTOCOL.md` §2 + addendum.

### M5 — `beta/05` §8 — FACTCHECK over-attribution split
- Before: "**Precise atproto facts (cite FACTCHECK SoT):** … 72h recovery window … k256 or p256 …
  plc.directory is run by Bluesky PBC as a transparency log … governance handoff planned, not done."
- After: blessed-methods + "Public Ledger of Credentials" attributed to FACTCHECK; the 72h window +
  k256/p256 attributed to `plc-identity-resilience.md`; the PBC/transparency-log/governance-handoff
  facts attributed to the identity-provenance dialogue + COHESION §21.
- Source: the FACTCHECK contains only the blessed-methods + did:plc-expansion facts (grep of the
  FACTCHECK for `72h|k256|p256|PBC|transparency log|governance handoff` = zero hits).

### M6 — `beta/06` §1 — E3.4 proof status (green-model → green-real)
- Before: "the Tier-0 blind broker is green-model (E3.4)"
- After: "the Tier-0 blind broker is green-real (E3.4)"
- Source: `proof-ledger.md:109` (E3.4 = green-real) and `beta/04:156` (already states green-real).
  Resolves a cross-doc inconsistency for the same experiment.

---

## MINOR corrections applied

- **`beta/01` §2.1 — Plato 21d** rewritten to verbatim Fowler (Loeb): "this man thinks he knows
  something when he does not, whereas I, as I do not know anything, do not think I do either." — *Apology*
  21d. (Prior wording matched neither Fowler nor Jowett.)
- **`beta/01` §2.1 — Mill "exchanging error for truth"** semicolon → colon after "truth," per *On
  Liberty* ch. 2 standard editions (Oxford Reference / Norton).
- **`beta/01` §2.2 — Peirce** "block the road of inquiry" → "Do not block the way of inquiry." with the
  CP 1.135 cite; verification upgraded to "Verified."
- **`beta/01` §2.2 — Popper** "can only be finite" → "can be only finite," with the C&R p. 30 / §XVII
  cite; verification upgraded to "Verified."
- **`beta/01` §2.4 — in-document self-reference** "`01`§6" → "§6" (a reference from inside doc 01).
- **`beta/02` (Braes) — Sheriff Ivory title** reordered to match the source: "the Sheriff of
  Inverness-shire, William Ivory."
- **`beta/03` §2 — crossed COHESION clause** fixed: §3 = the SSB unbounded-log-growth cautionary tale;
  §11 = SSB's lesson applied forward as the bounded broadcast tier (was crossed).
- **`beta/03` §6 — Germ block-quote attribution** changed from `research/germ-xchat-features.md` (which
  predates and lacks the maturation facts) to `ECOSYSTEM.md §6 + FACTCHECK addendum`, with a note that
  the germ-xchat file supplies only the privacy-inversion.
- **`beta/03` Sources footer + `alpha/BETA-ROLLUP.md` theme-03 row** — declared COHESION set expanded to
  include the §3/§8/§9/§11/§13 the body actually collapses (was undeclared).
- **`beta/06` §7 — re-formation citation** "C3" → "A1 re-formation (trap door)" and status precision
  "green-real" → "green-real-multimachine," matching `proof-ledger.md:164`.
- **`beta/07` narrative overview — paraphrase tightened** "embedded in the valuation bankers are
  pricing" → "embedded in the number bankers are pricing," matching the verbatim block quote at `07:243`
  and the source.
- **`beta/08` §9 — relay-fallback FACTCHECK over-attribution** split: the "browser peers permanently
  relayed / hole-punch falls back to relays" claim re-attributed to `thinking/app/README.md` + COHESION
  §19; iroh `1.0.0` / DERP→relays / Tauri-native-WebView remain "cite FACTCHECK SoT."

---

## Re-verification of the remaining "Verified" classical set (Socrates / Mill / Hayek / Scott)

- **Plato *Apology* 22d** ("conscious that I knew practically nothing") — verbatim MATCH (Perseus,
  Fowler/Loeb). No change.
- **Mill "all mankind minus one"** — verbatim MATCH (Oxford Reference). No change. (The second Mill
  quote's colon was corrected — see MINOR.)
- **Hayek "dispersed bits of incomplete and frequently contradictory knowledge…"** — verbatim MATCH
  (AER 1945; SFI/Econlib). No change.
- **Scott, *Seeing Like a State*** — correctly presented as a referenced case, not a manufactured quote;
  no words put in Scott's mouth. No change. (Optional, not applied: name "legibility" explicitly — the
  passage already uses "single legible value.")
- **Net:** the two unreliable "Verified" flags were Ashby (M1) and Beer (M2), now corrected. The
  remaining classical "Verified" set holds up to primary-source spot-check.

---

## Resolved by the user's ruling

- **`beta/08:30` — "Croft Group" / "Croft the product" working handles.** User ruling (2026-06-24):
  **keep + add an explicit placeholder tag.** Applied: the first use of "Croft Group" (`08:30`) now
  reads "**Croft Group** [working name, pending brand reconciliation — see the decision-gated banner]"
  so a reader cannot mistake it for a settled brand decision. The names are retained as provisional
  working handles (consistent with the alpha sources and the decision-gated banner + `08:213-214`
  disclaimer); the brand DRIFT reconciliation against `NAMING.md` remains a surfaced gate, not resolved.

## Intentionally not changed (graded acceptable in the report)

- `beta/03` §6 — the atproto private-data block rendered as a `>` block is faithful synthesis; left as
  synthesis (every fact checks out). Re-style only if `>` is to be reserved strictly for verbatim.
- `beta/04` conformance row bare "green" — matches the ledger's own bare "green" (mixed
  green-real/green-model rows); annotation optional, not applied.
- `beta/05` "two blessed methods" — the did:web hostname-only constraint is omitted; acceptable for a
  synthesis doc.

---

## Verification of the edits

- All quote replacements use the primary-edition wording confirmed in the re-verification pass
  (sources cited inline above and in the report).
- No `alpha/` **corpus** file (thinking/research/crystallized/narrative/seeds-transcripts) was edited.
  The only `alpha/` corpus-adjacent change is `BETA-ROLLUP.md` (the designated additive file). Two
  working-surface files outside the corpus were also touched by the follow-on "Croft Group" tag commit:
  `alpha/seeds/generated-prompts/README.md` and the new `alpha/seeds/generated-prompts/beta-factcheck-pass2-prompt.md`
  (the thin-pointer handoff lane, not frozen corpus).
- The eight beta theme docs, `beta/README.md`, and the two `alpha/plans/` artifacts (the report and this
  log) are the only changed files.
- No surfaced decision gate (MPL license, recovery anchor, cooperative legal review, the Noria name,
  CroftC Phase-0 IP, genome-vs-strategy) was touched.
