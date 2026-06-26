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

## thinking/app/ (15) — DONE

All DETERMINED or OPEN; **no new finds.** FOLDED → 08: design-philosophy (§1/§2/§3/§6; games-data-layer
§13 → T15), client-architecture-adr (§3/§4), design-criteria (§6), ponds/build-order (§7/§9),
ponds/games-pond-authoritative-list (§7), ponds/on-device-llm-feasibility (§10 K12),
ponds/webxdc-security-and-competitive-games (§6). CITED (alpha-by-design detail): build-specs/BUILD-SPEC
(Phase-0 green-real backs 08 §3), ponds/apps-pond-utility-list (catalog), ponds/build-shape-pass (license
table + iroh-docs detail), ponds/fair-reveal-primitive-spec (commit-reveal crypto; per-use-case mitigations
are local spec detail), ponds/p2p-games-pond-launch-set (superseded by authoritative-list). PROCESS/INDEX:
README, build-specs/BUILD-SPEC-PHASE-1-2. OPEN: brand-and-voice-notes → T4; games outcome-attestation
open-question → T15. *Note:* app docs already reference "social-graph discovery" — consistent with the 08
substrate reframe (corroboration, not an orphan). T4/T15/T17 confirmed present in OPEN-THREADS.

## thinking/drystone-spec/ (4) — DONE — **disposition CHANGED this session + T1 was stale (fixed)**

All four were **STAGED (T1)** at 2026-06-25; this session they were **matured into the beta Drystone
protocol spec**, so the disposition changed and **T1 was materially stale** (still said "not built / `P-*`
unwritten"). **FIND fixed:** T1 updated → **PROMOTED → drystone-spec**, residuals carried to the spec.

| file | disposition (now) |
|---|---|
| drystone-spec-v0.1-skeleton.md | DETERMINED-FOLDED → `drystone-spec` README + structure |
| section-2-peers-rights-capabilities.md | DETERMINED-FOLDED → spec Part 2 §5 |
| section-x-governance-conflicts.md | DETERMINED-FOLDED → spec Part 2 §7 |
| README.md | DETERMINED-FOLDED (its `P-*` E30 gap → now written in spec Part 1 §2); residual open items (Track A/B, key-custody A12, geer-name A13, ENABLING formats) → spec App-A/App-B + ROADMAP |

## narrative/ (6) — DONE

All DETERMINED or OPEN; no new finds (two minor recorded notes).
- lineage-of-a-design-imperative.md — DETERMINED-FOLDED → (01 →) `drystone-spec` Part 1 §3. **Note:** this
  session cut Socrates/Peirce, de-emphasized "2,400 years," dropped the Ashby-gloss/Beer-paraphrase-as-quote,
  and relocated Hush-A-Phone — all recorded in BETA-ROLLUP "01 review → Drystone spec." So the per-theme 01
  table row's "every verbatim quote reproduced whole" is **superseded** by that intake section (not re-edited;
  redirect note + intake section cover it). The deferred "reinforcements" (von Foerster / Jane Jacobs / ecology
  diversity-stability) remain DETERMINED-DEFERRED (recorded in BETA-ROLLUP 01; optional low-value Part-1
  expansion).
- long-form.md / short.md — DETERMINED-PROCESS/INDEX (synthesis skeletons; adoption-curve risk → T11).
- messaging-and-quotes.md — DETERMINED-OPEN → T4 (brand/voice reservoir; K11 crossed → 07 A2b).
- verticals/croft-the-name-and-the-commons.md — DETERMINED-FOLDED → 02.
- verticals/README.md — DETERMINED-PROCESS/INDEX. **Confirmation, not a find:** verticals #1–6 are unwritten
  stubs, but their core ideas already folded — "renting-our-relationships-back" → K13 (02 §6),
  "linear-vs-cyclical" → K16 (07 A0). Unwritten ≠ left-behind; the conclusions landed.

## research/ (16) — DONE — **2 genuinely left-out items found + surfaced**

Clearly-folded/cited/staged (baseline holds, no left-behind): messaging-solutions-landscape (03),
public-social-protocols (03), germ-xchat-features (03), atproto-private-data-architecture (03 §6; T7),
atproto-sovereign-appview-club (03 §8), discord-dominance (03 §7 + 07 B6), social-platform-cycle (02 §6 +
07), open-publication-and-ip-protection (07 C K9), socialization-and-publication-venues (07 §C4 K9),
iroh-realtime-media-references (CITED 04 §4; T10), str0m-production-readiness (CITED; T10), README
(PROCESS). STAGED: group-chat-failure-modes(-plain) → T5; p2p-founder-motivations-adoption → T11;
discord-matrix-groupchat → T16. (T5/T7/T10/T11/T16 confirmed present.)

**FINDs (from `messaging-solutions-landscape.md` §"top unresolved questions"):**
- **Q3 — MLS group state ↔ governance-log/Automerge consistency** ("the exact problem Matrix is solving for
  MLS-in-federation"): a real open design binding **not surfaced** anywhere → **OPEN-THREADS T29** (+ spec
  App-B cross-ref). Q1 (multi-device DID flow)→05/spec §4.5.1; Q2 (recovery)→the bannered gate + K14; Q4
  (metadata padding/mixing vs scope-the-claim)→DETERMINED (the honest-metadata posture, spec §8; mixing a
  future-ROADMAP enhancement).
- **Q5 — a pre-production crypto/security audit** ("none of our crypto has been audited; every credible system
  has been"): **not tracked anywhere** → added to backlog **ROADMAP_TODO B11** (a standing pre-prod gate).
