# Beta fact-check — pass 2 (post-correction re-audit)

date: 2026-06-24

scope: second adversarial fact-check of `discovery/beta/` and `alpha/BETA-ROLLUP.md`, run after the
pass-1 corrections (`62d5585`) and the "Croft Group" working-name tag (`effb075`) landed on top of the
first-pass synthesis (`4d45ac8`) and report (`7dc899a`). Method: re-derive from source; trust no label,
including the corrections log's own claims. Fan-out — one verification agent per beta theme, plus an
external-quote web sweep, a proof-flag + recurring-numbers cross-doc sweep, and a
ledger/tier-discipline/do-not-carry sweep. All agents ran on the session's primary model (Opus).

**No edits were made to `beta/` or `alpha/` in this pass.** This is a findings report only. No surfaced
decision gate (MPL license, recovery anchor, cooperative legal review, the Noria name, CroftC Phase-0 IP,
genome-vs-strategy) was resolved.

---

## Resolution (applied 2026-06-25)

All seven actionable findings were corrected after this report was filed (the audit body below is preserved
as-found):

- **R1** (MAJOR) — `01:8-9` + `01:327` now list **Ostrom only**; Peirce/Popper dropped, matching their
  Verified body flags.
- **N1** (MAJOR) — `06:104-105` no longer claims the mesh/SFU caps are `green-real`; reattributed to
  design/physics (`abuse-resistance-and-the-rave-trap.md` / `realtime-media-over-iroh.md`) with the relay
  lab credited for what it actually proved (E11 characterized; blind meer green-real).
- **N2** (MINOR) — `04:51-52` + `04:237`: spec-vs-code item moved out of the carried-open list and the
  footer note changed to "*§2 pre-images reconciled 2026-06-17 (see §5)*".
- **N3** (MINOR) — `06:91`: meer proof relabeled "E9" → "meer P0+P1" (matches `04` and the ledger).
- **N4** (MINOR) — `03:217`: added an in-body nod substantiating the footer/rollup declaration of
  COHESION §17/§31 (iroh corroboration) and §32 (open-social landscape); declared set left intact.
- **N5** (MINOR) — corrections-log "sole alpha change" wording widened to acknowledge the
  `generated-prompts/` working-surface edits.
- **N6** (MINOR/escalation) — `02:40`: added "(attribution `[UNVERIFIED]`; see §4)" to the "Magna Carta"
  phrase in the overview.
- **N7** (escalation) — `07` `501(c)(3)`: left as-is per the finding's own "no correction recommended"
  (carried reasoning, mirrored in the rollup's surfaced-gate wording).

No surfaced decision gate was resolved. The verdict below stands as **GO** once these land — they have.

---

## Headline

Beta held up well. Every pass-1 correction (the 6 MAJORs, all MINORs, and the Croft Group ruling) landed
and re-derives correctly against source — including M4, which was the over-correction risk, and it is not
over-corrected. The external quotes are publication-safe. The do-not-carry exclusions are all-clear. Proof
flags and recurring numbers are consistent across all eight docs.

Two items warrant a fix before the next stage, and both are **internal-consistency lag, not factual error**:

1. **REGRESSION (MAJOR) — `01` framing lines contradict the body** the pass-1 edits created. The header
   and the closing "does not establish" line still list Peirce and Popper as quotes that "must be confirmed
   before publication," but the body upgraded both to **Verified** in pass 1. Only Ostrom should remain on
   that list.
2. **NEW (MAJOR) — `06` §2 over-claims a proof status.** "The physics caps (mesh dies ~5, an SFU holds
   dozens) are green-real from the relay lab" attaches `green-real` to capacity numbers the ledger never
   measured; they are design/physics statements.

Neither is a BLOCKER. Recommendation: **conditional GO** — apply the two MAJOR fixes plus the cosmetic
MINORs below, then beta is fit for the next stage.

---

## REGRESSION findings (pass-1 corrections that did not fully land, or that introduced a new error)

### R1 — MAJOR — `beta/01-epistemic-foundation.md:8-9` and `:327` — Peirce/Popper still flagged "must confirm" while the body now says Verified
- **As written (`:8-9`):** "three attributions (Peirce, Popper, Ostrom) must be confirmed against primary
  editions before any external publication." **(`:327`):** "nor are the Peirce / Popper / Ostrom quotes
  publication-ready until confirmed against primary editions."
- **What is true:** pass 1 upgraded **Peirce** (`:142-145`, "Do not block the way of inquiry.", CP 1.135)
  and **Popper** (`:151-155`, "can be only finite", C&R p. 30) to **Verified** in the body, and the
  external-quote web sweep independently confirms both are verbatim-correct against the primary editions.
  Only **Ostrom's subsidiarity passage** (`:186-188`, drawn from the 2013 generalization, correctly *not*
  marked Verified) still legitimately needs confirmation. The framing lines were not updated with the body,
  so the doc now contradicts itself (body: Verified; framing: must-confirm).
- **Severity:** MAJOR (internal contradiction in the doc's own verification ledger; not a misquote).
- **Recommended correction:** change both lines from "Peirce / Popper / Ostrom" to **Ostrom only**, e.g.
  header → "one attribution (Ostrom's subsidiarity passage) must be confirmed against *Governing the
  Commons* before any external publication"; closing → "nor is the Ostrom subsidiarity quote
  publication-ready until confirmed against the primary edition."

*(No other pass-1 correction regressed. M1–M6 and all MINORs are confirmed landed and correct — see the
corrections-verified checklist.)*

---

## NEW findings (pass 2, not in pass 1)

### N1 — MAJOR — `beta/06-safety-without-surveillance.md:104-105` (§2) — proof-status over-claim on mesh/SFU caps
- **As written:** "*Verification:* design; the physics caps (mesh dies ~5 peers, an SFU holds dozens) are
  **green-real from the relay lab**."
- **What is true:** there is no `proof-ledger.md` row that measures a mesh-death-at-~5 or SFU-capacity
  result. Those numbers are architectural/physics statements in `alpha/thinking/realtime-media-over-iroh.md`
  (~`:56-57`) and `alpha/thinking/abuse-resistance-and-the-rave-trap.md` (~`:97`, "Mesh dies at ~5 and SFU
  at dozens **by physics**"). What the relay lab actually proved is different: E10 = congestion-control
  bound (characterized), E11 = lazy fan-out / blind relay / metadata admission (characterized), E12 =
  SFrame-over-MLS keying (green-real), meer P0+P1 = blind Tier-0 mirror (green-real). None of those is the
  capacity cap.
- **Severity:** MAJOR (a `green-real` flag attached to an unmeasured number — the single proof-status
  overstatement found in the whole corpus).
- **Recommended correction:** drop "green-real from the relay lab" for the caps; reattribute, e.g.
  "*Verification:* design — the physics caps (mesh ~2–5 peers, an SFU holds dozens) are architectural
  (`realtime-media-over-iroh.md`); the relay lab proves the blind metadata-admission lever (E11,
  characterized) and the blind meer (green-real), not the capacity numbers."

### N2 — MINOR — `beta/04-the-protocol-we-proved.md:237` (Sources footer) and `:51-52` (overview) — stale "open" framing of a now-resolved item
- **As written:** the footer still annotates `CROFT-PROTOCOL.md` with "*reconcile §2 pre-images (see §5)*",
  and the overview still lists the spec-vs-code item among "carried-open" risks.
- **What is true:** M4 correctly reframed §5 to "surfaced AND resolved (2026-06-17) … no live spec/code
  divergence remains." The load-bearing §5 statement is right and well-supported (`proof-ledger.md:180`
  "RESOLVED"; `test-narrative.md:620-628`; `CROFT-PROTOCOL.md:55-66` incl. the addendum). The footer's
  imperative "reconcile" and the overview's "carried-open" listing now lag behind it.
- **Recommended correction:** footer → "*§2 pre-images reconciled 2026-06-17 (see §5)*"; drop the item from
  the overview's carried-open sentence (or qualify "(now resolved)").

### N3 — MINOR — `beta/06:91` vs `beta/04:187` — meer proof labeled inconsistently ("E9" vs "P0+P1")
- `06` calls the blind-meer proof "E9"; `04` calls it "P0+P1". Both are `green-real` and refer to the same
  result (`proof-ledger.md:176`). There is **no formal E9 experiment row** in the ledger — "E9" is only the
  run-directory name (`E9-meer-tier0-2026-06-17`), so a reader who hunts for an "E9" row finds nothing.
- **Recommended correction:** standardize on "meer P0+P1" (optionally "P0+P1 / run E9") in both docs.

### N4 — MINOR — `beta/03-the-living-ecosystem.md:250` footer + `alpha/BETA-ROLLUP.md:127` — §17/§31/§32 declared but not cited in-body
- The expanded theme-03 COHESION set is otherwise correct (§3/§8/§9/§11/§13 are genuinely collapsed and
  now declared in both the footer and the rollup row — M-level fix confirmed good). But the footer/rollup
  also declare §17/§31/§32, which the `03` body never cites by section number (the body cites
  §3/§8/§9/§11/§13/§25/§26/§28/§29/§36). The rollup glosses §17/§31/§32 as "iroh corroboration; open-social
  landscape," which maps to §9 — so it is an over-declaration, not a false content pointer.
- **Recommended correction:** either add a one-clause in-body nod for §17/§31/§32 or trim them from the
  declared set so the footer matches what the body demonstrably collapses.

### N5 — MINOR — `alpha/plans/2026-06-24-beta-factcheck-corrections-log.md:148-149` — "sole alpha change" wording understates `effb075`
- The corrections log states "No `alpha/` corpus file was edited; the sole `alpha/` change is
  `BETA-ROLLUP.md`." Git confirms `62d5585` touched only `alpha/BETA-ROLLUP.md` + `alpha/plans/` — but the
  follow-on `effb075` also modified `alpha/seeds/generated-prompts/README.md` and added
  `alpha/seeds/generated-prompts/beta-factcheck-pass2-prompt.md`. These are handoff working-surface files
  (the thin-pointer lane), **not** frozen corpus (no thinking/research/crystallized/narrative/seeds-
  transcript file was touched), so tier discipline is intact — but the log's "sole alpha change" wording is
  narrowly inaccurate.
- **Recommended correction:** widen the log's wording to acknowledge the `generated-prompts/` edits as a
  working-surface change distinct from the corpus.

### N6 — MINOR / escalation — `beta/02-enclosure-and-its-inversion.md:40` — "Magna Carta" phrase unflagged in the overview
- The phrase "the Magna Carta of the Highlands and Islands" appears unflagged in the theme-narrative
  overview (`:40`), while its load-bearing use at §4 (`:179-186`) carries the full `[UNVERIFIED]`
  attribution caveat (correctly — the web sweep confirms the attribution rests on secondary sources). A
  skimmer sees the grand phrasing unflagged.
- **Recommended action (author judgment, non-blocking):** add a parenthetical at `:40` ("(see §4 for the
  attribution caveat)"), or accept as-is since §4 carries the flag.

### N7 — escalation (no correction) — `beta/07-sustainability-and-stewardship.md:65,142,262` — `501(c)(3)` brushes the NOT-LEGAL-ADVICE banner
- The banner excludes "IRS form," and `501(c)(3)` is the one IRS-form token that crosses that literal
  wording. It reads as a structural entity-type label (carried *reasoning*, mirrored in the rollup's own
  surfaced-gate wording at `BETA-ROLLUP.md:205` "coop-vs-coop+501(c)(3) form"), not a carried citation.
  Judgment call; flagged for the author, no correction recommended.

### Soft notes (no action)
- **Popper page citation** (`01:151-155`): p. 30 is edition-dependent; some editions cite p. 38. Does not
  affect quote accuracy.
- **Ashby page** (`01:192-193`): the wording "Only variety can destroy variety." is web-verified; the
  exact p. 207 locus is widely cited but was confirmed by wording, not by the online page image.

---

## Corrections-verified checklist (pass-1 fixes independently re-confirmed good)

| Pass-1 fix | Verdict |
|---|---|
| **M1** `01` Ashby "absorb→destroy" (p. 207) + 2nd block reframed to explicit paraphrase | ✅ landed & correct (web-verified) |
| **M2** `01` Beer de-quoted to paraphrase (no quote marks, not re-attributed); Cybersyn "functioned until destroyed from outside (1973 coup)" | ✅ landed & correct (not verbatim-sourceable → de-quoting was right) |
| **M3** `02` trap/balance cross-ref → "Collapses COHESION §34" only (§30 removed) | ✅ landed & correct (§34 is the trap/balance refinement; §30 is Groundmist/HF23) |
| **M4** `04` spec-vs-code reframed open → "surfaced AND resolved (2026-06-17)" | ✅ landed & correct — **not over-corrected** (ledger "RESOLVED", test-narrative, CROFT-PROTOCOL §2 + addendum all agree) |
| **M5** `05` §8 FACTCHECK over-attribution split (blessed-methods+PLC-expansion → FACTCHECK; 72h+k256/p256 → plc-identity-resilience.md; PBC/transparency-log/governance-handoff → dialogue+COHESION §21) | ✅ landed & correct (FACTCHECK greps for 72h/k256/p256/PBC = zero, as required) |
| **M6** `06` §1 E3.4 green-model → green-real | ✅ landed & correct (ledger `:109`; now agrees with `04:156`) |
| `01` Plato 21d → verbatim Fowler/Loeb | ✅ web-verbatim |
| `01` Mill colon after "truth" | ✅ web-verbatim |
| `01` Peirce "Do not block the way of inquiry." + CP 1.135 + Verified | ✅ web-verbatim (but see R1: framing lines stale) |
| `01` Popper "can be only finite" + C&R p.30 + Verified | ✅ web-verbatim (but see R1) |
| `01` self-reference "01§6" → "§6" | ✅ landed |
| `01` Ashby/Beer re-flagged off "Verified"; Socrates 22d / Mill / Hayek / Scott still hold | ✅ all confirmed |
| `02` Sheriff Ivory title reorder | ✅ verbatim vs source |
| `03` §3/§11 SSB clause uncrossed (§3 = cautionary tale; §11 = lesson applied forward) | ✅ correct vs COHESION §3/§11 |
| `03` Germ quote re-attributed to ECOSYSTEM §6 + FACTCHECK addendum | ✅ facts present there; germ-xchat-features.md predates them |
| `03` declared COHESION set expanded (§3/§8/§9/§11/§13 in footer + rollup) | ✅ correct (over-declares §17/§31/§32 — see N4) |
| `06` re-formation citation "C3" → "A1 re-formation (trap door)", green-real → green-real-multimachine | ✅ matches ledger `:164` |
| `07` paraphrase "valuation" → "number bankers are pricing" | ✅ matches block quote `07:243` and `discord-dominance.md:258` |
| `08` relay-fallback split (browser-peers-relayed/hole-punch → README + COHESION §19; iroh 1.0.0/DERP→relays/Tauri → FACTCHECK) | ✅ both source sets independently confirmed |
| `08` "Croft Group" working-name tag at `08:30` | ✅ reads cleanly; not asserted-as-decided elsewhere (banner + `08:215-218` consistent) |
| External quotes (all) publication-safe | ✅ web sweep: all verbatim or correctly de-quoted |
| Do-not-carry exclusions absent across all 8 docs | ✅ all-clear (every banned term only in explicit excluded/REFUTED traces) |
| Proof flags + recurring numbers cross-doc | ✅ consistent (openmls 0.8.1; iroh 1.0.0; 66/0; HF23 $6.3M; AR-5 1.4KB→11KB; RFC 9420) |
| Tier discipline (no beta→alpha forward dep; alpha corpus frozen) | ✅ clean (working tree clean; only BETA-ROLLUP.md + plans/ + generated-prompts/ touched) |

---

## Severity summary (by theme)

| Theme | BLOCKER | MAJOR | MINOR | OK / clean |
|---|---|---|---|---|
| 01 epistemic foundation | 0 | 1 (R1) | 0 (+2 soft notes) | corrections all good |
| 02 enclosure & inversion | 0 | 0 | 0 | clean (N6 escalation, author judgment) |
| 03 living ecosystem | 0 | 0 | 1 (N4, shared w/ rollup) | clean |
| 04 the protocol we proved | 0 | 0 | 1 (N2) | M4 not over-corrected |
| 05 identity you carry | 0 | 0 | 0 | clean |
| 06 safety without surveillance | 0 | 1 (N1) | 1 (N3) | corrections good |
| 07 sustainability & stewardship | 0 | 0 | 0 | clean (N7 escalation, no fix) |
| 08 Croft the product | 0 | 0 | 0 | clean |
| Ledger / cross-cutting | 0 | 0 | 1 (N5) | numbers/flags/do-not-carry all clean |
| **Total** | **0** | **2** | **5** | — |

---

## Go / no-go

**Conditional GO for the next stage.** Zero blockers; the corpus is factually sound and the pass-1
corrections all hold. Apply the two MAJORs before promotion:

1. **R1** — fix `01`'s header (`:8-9`) and closing (`:327`) to list **Ostrom only**, not Peirce/Popper.
2. **N1** — fix `06` §2 (`:104-105`) to stop claiming the mesh/SFU caps are `green-real`; reattribute to
   design/physics + the actual relay-lab results.

The five MINORs (N2–N6) are cosmetic/consistency lag and can ride in the same correction pass. The two
escalations (N6 author-judgment flag on `02:40`; N7 `501(c)(3)` wording) are non-blocking and left to the
author. No surfaced decision gate was touched or resolved in this audit.

---

## Method note

Fan-out: 8 per-theme verification agents (Job A re-verify corrections + Job B fresh full sweep) + 1
external-quote web-verification agent (fetched primary editions / authoritative sources for all 8 changed
quotes + the enclosure primary texts) + 1 proof-flag/recurring-numbers cross-doc sweep (re-derived every
flag against `proof-ledger.md`/`test-narrative.md`; grepped every recurring number across all 8 docs) + 1
ledger/tier-discipline/do-not-carry sweep (spot-checked every BETA-ROLLUP "landed in §X" pointer; git-stat
confirmed alpha-edit confinement; grepped every excluded item across all 8 docs). All agents ran on the
session's primary model and produced findings keyed to `file:line`; this report collates them.
