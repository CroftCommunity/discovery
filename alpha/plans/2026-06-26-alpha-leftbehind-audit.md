# Alpha left-behind / left-out audit (stricter eye, post-spec context)

date: 2026-06-26 · status: IN PROGRESS (resumable; one file at a time)

## Problem statement

The user was frustrated that we have been "too lax on identifying things that need to be included or
decided" — leaving real items as inline notes or as "optional/low/borderline/DEFERRED" limbo rather than
either **determining** them (folded / cited-by-design / excluded-with-reason) or **surfacing** them (an
OPEN-THREADS thread or a named decision). This audit walks **every** alpha file one at a time and forces
each to a binary: **DETERMINED** (a clear disposition) or **OPEN** (tracked as a thread/decision). Anything
that is neither — a left-behind conclusion, an un-decided deferral, a stray inline pointer — is the find.

## Approach

Baseline = the 2026-06-25 per-file coverage audit (`2026-06-25-beta-coverage-per-file-audit.md`), which
dispositioned 165 files and is trusted for FOLDED-verification. This pass adds the **stricter eye** and the
**current context** (the Drystone protocol spec built 2026-06-26; theme 01 retired → `drystone-spec`; 08
reframed on the social graph; the "surface every decision" standard). For each file: confirm the baseline
disposition still holds, then hunt specifically for (a) items the prior audit left in limbo (optional folds,
partials, DEFERRED, borderline) that the stricter standard says must be decided; (b) dispositions the
2026-06-26 work changed; (c) genuinely new files. **Decide or surface each — no limbo.**

## Disposition codes

**DETERMINED-FOLDED** · **DETERMINED-CITED** (alpha-only by design, reason) · **DETERMINED-EXCLUDED**
(reason) · **DETERMINED-PROCESS/RAW** · **OPEN→Tn** (surfaced as a thread) · **OPEN→decision** (a named
decision in README gates / BETA-ROLLUP deferred) · **FIND** (left-behind/left-out — needs action this pass).

---

## crystallized/ (7) — DONE

| file | disposition (stricter eye) | left-behind finding + action |
|---|---|---|
| conclusions.md | DETERMINED-FOLDED (04/01/03/06/07/08; M0–M4 → T12) | none. "M2 = the social graph you hold" corroborates the 08 reframe. |
| principles.md | DETERMINED-FOLDED (Tier-1 drained; Tier-2 reflected 04/06/08; Tier-3 ADR→08, three-audiences→T17, LTS→T18) | **FIND-1:** Tier-1 "user-need-first / Google+ lesson" was in limbo (prior audit "intentionally unfolded"). **Decided:** DETERMINED — subsumed by 08 §1 (composability as the user-respecting stance) + the razor (provenance-not-utility = not-data-extraction-first); recorded, not separately folded. User may override (fold a named line into 08/07). **FIND-2 (bookkeeping):** the deeper-foundation block's folds were the K-table's `01 §X` landings; 01 retired → drystone-spec this session, so those pointers are stale → fixed via a redirect note at the K-tables. |
| proof-ledger.md | DETERMINED-CITED (04 §3 results table + §5 flags derive from it) | none. |
| test-narrative.md | DETERMINED-CITED (the "what each proof means" layer behind 04 §3/§5) | none. |
| conformance-suite.md | DETERMINED-CITED (04 §3 + CROFT-PROTOCOL §12 + now `drystone-spec` Part 2 §9 conformance) | none. |
| TEST-CORPUS.md | DETERMINED-PROCESS/INDEX (cross-repo test catalog) | none. |
| CROFT-PROTOCOL.md | **disposition CHANGED this session:** was CITED→04; now DETERMINED-FOLDED → `drystone-spec` Part 2 (matured 2026-06-26) + still CITED by 04. `croft-*` tag-rename flagged in spec App-B. | none new (recorded in BETA-ROLLUP "01 review → Drystone spec" intake). |

## thinking/ top-level (33) — DONE

Clearly-folded (baseline verified 2026-06-25 + stricter-eye confirmed, no left-behind):
abuse-resistance-and-the-rave-trap (06), atproto-atmospheric-web (03 §6), cooperative-social-union-model
(07; CM-A3→K15 folded), cross-platform-identity-provenance (05; per-platform doc→T6), failed-op-response
(06 §4), foundation-and-ip-stewardship (07 B1–B5), freshness-signal (06 §5), governance-and-survivability
(07 A8), group-privacy-lanes-design-note (06 §10), interaction-tiers (08 §5), ios-opportunistic-p2p (03;
T14), local-first-as-design-imperative (01→spec; K6/K7/K14 folded; T19 blind-search), meer-superpeer-design
(06 §1 K8), membership-vs-access-the-public-door (06 §8 + 08 §8), merge-split-corpus (04 §3; C4–C10→T20),
multi-device (05; per-device-lamport/self-AS added to spec §4.5.1 this session), plc-identity-resilience
(05), realtime-media-over-iroh (04 §4; T10), revocation-authority (06 §6), social-layer (06 §9),
thesis-lineage-groups (04), drystone-publication-and-defensive-disclosure (07 Pillar C K9).
PROCESS/INDEX: experiment-suite, model-holds-up-summary, open-edges. FOLDED+STAGED: open-considerations.

New-this-session (disposed): algedonic-and-peerhood-as-adjudication (→ spec P1 §3 / P2 §3.1/§5.2/§8/App-B;
T24), social-graph-as-substrate (→ spec P1/P2 + 08 + T25/T26/T27), rights-vs-capabilities-definitions
(→ spec §5 / 01-now-spec §5; T21/T22).

| file | finding + decision |
|---|---|
| design-notes-addendum.md | **CM-P1** (roll-up trust trilemma + accumulator/MMR end-state): **DETERMINED** — resolved by spec §7.3.3 choosing Option-A (each peer self-folds; no-trust checkpoint); accumulator/MMR is a minor unbuilt future direction, recorded here, not worth re-editing the spec. **CM-P2** (honest guarantee = "true attributable history + clean exit, NOT convergence"): **FOLDED 2026-06-26 (user-approved) → 04 §1** (new ¶ after "one fact with two payoffs"). BETA-ROLLUP CM-P2 row marked folded. |
| geer-gating-peer.md | **CM-P3** (report-gated / classifier-gated / full-key 3-rung enumeration): **DETERMINED-subsumed** — 06 §3 carries the load-bearing point ("least-invasive rung," "other rungs compellable"); the enumeration is alpha detail. |
| historical-peer-rights.md (NEW this session) | **FIND — new limbo I introduced:** the doc ends "eventual maturity home … a later call" (its own beta theme vs fold into history theme `02` vs stay alpha). **Surfaced as OPEN-THREADS T28** rather than left as an inline "later call." |
