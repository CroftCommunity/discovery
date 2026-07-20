# Capture audit: July-10-onward work — status level + narrative/thinking integration

**Status:** Done (audit pass, 2026-07-20). Status gaps fixed inline; narrative/thinking gaps
**surfaced for owner review, not written** (per the agreed scope: identify + fix status, gate narrative).

## Problem Statement

The 2026-07-18 and 2026-07-20 mobile back-and-forth produced ~50 discovery commits across ~7 work
streams. The owner asked: verify all of it is captured **at the status level** (registers/tags reflect
what landed) **and integrated into narratives/thinking** (the design implications reached the story
layer, not just the experiments + spec). This ledger is that verification.

## Approach

Enumerated the streams from `git log --since=2026-07-10`. For each, verified **against main** (not
inference): (a) status presence in MASTER-INDEX / EXPERIMENT-BACKLOG / SPEC-DIVERGENCE-REGISTER / spec
status tags / ROADMAP_TODO / COHESION; (b) whether the *specific finding* (not just the topic) reached
`beta/` narrative + `alpha/thinking`. Two method notes, both learned the hard way this pass:

- **Coarse greps lie.** An early for-loop grep (mangled under the token-proxy, SIGPIPE) reported "0
  narrative files" for concepts I had *filed this session*. Clean per-concept greps corrected it
  (attestation/vouch 38 files, federation/AP 36, wasm/WebTransport 14, lexicon 5, PDS-history 3). No
  coverage claim here rests on a coarse grep.
- **Verify against main, not the plan-in-hand.** The SPEC-ALIGNMENT plan's §4 *looked* stale
  ("proposals, not applied"), but its header + §7 already recorded F1–F8 landed (RUN-02). Reading one
  section misled; reading the whole file + the landed spec corrected it.

## Per-stream findings

| Stream | Status capture | Narrative/thinking integration | Action |
|---|---|---|---|
| **Attest lane** (RUN-ATTEST-02/03/04) | ✅ MASTER-INDEX §1 (all 4 runs, V1–V10); PRIMITIVES-ATTEST; lexicons; §A.11 landed | ✅ broad (attestation/vouch 38 files); this session added `social-layer.md` §7 (utility-graph), CONTACT.md, dating v2 — all use the attest primitives | none |
| **wasm-seal** (RUN-19) | ❌→✅ **was ABSENT from MASTER-INDEX** — **fixed** (added §1 row); §A.8 upgrade staged | ⚠️ **partial** — the finding is in `realtime-media-over-iroh.md` + spec §A.8, but the *design-narrative* (the sealed tier reaches the browser with **no iroh bridge** — a real deployment/sovereignty story) isn't in the app/client thinking | **narrative gap surfaced** |
| **HIST lane** (RUN-HIST-01 + RUN-HIST-02 revB) | RUN-HIST-01 ✅; **RUN-HIST-02 revB / `spike/hist_live/` was ABSENT** — **fixed** (extended the hist MASTER-INDEX row) | ⚠️ **top gap** — PDS-as-history-cold-storage + the **personal-deep-history tier** ("PDS as cold storage with cryptographic receipts", the local reference tail) appears only in `cairn/` as *atproto substrate mechanics* (3 files), not as a Drystone design implication in `thinking/` | **narrative gap surfaced (highest)** |
| **AP-ambassador** (RUN-AP-01) + fed-shim + RUN-FS-01 | ✅ MASTER-INDEX §1 (both); ROADMAP E34–38/A15–17; FINDINGS-AP; the AP/Mastodon cairn stone (this session) | ~ mostly ✅ (federation/AP 36 files incl. the cairn stone); the *gateway-attested receipt-lane + delivery-plane-role-holds-no-governance-conductivity* concept is in cairn/experiments, lightly in narrative | minor — optional narrative note |
| **Lexicon-community** (RUN-LEX-01) + ENGAGE-LEX + T54 | ✅ MASTER-INDEX §1; lexicon-community dir; ENGAGE-LEX; `beta/OPEN-THREADS` T54; the cairn governance stone (this session) | ✅ adequate (cairn is the right home for an ecosystem-engagement lane) | none |
| **Spec-reconciliation** F1–F8 (RUN-02/03/05) | ✅ landed in Part 2 (R7 etc.); SPEC-ALIGNMENT header + §7 record it; proposed-changes marked historical | n/a (spec-internal hygiene) — the §4 intro line read stale → **fixed** (added a "landed" status note) | none (status tidied) |
| **This session's intake** (8 commits) | ✅ COHESION §44–51, manifest, ROADMAP (A18/B16/D11/E39–E42/C17–18), BUILD-INVENTORY | ✅ research/design filed + cross-linked (dating v2, recovery→I9, CONTACT, AP/Mastodon, lexicon gov, kindred-work) | none |

## Status fixes applied this pass (inline)

1. **MASTER-INDEX §1** — added the **`wasm-seal/` (RUN-19)** row (was absent).
2. **MASTER-INDEX §1** — extended the HIST row to record **RUN-HIST-02 rev B** (`spike/hist_live/`, E1–E8 green, E1/OC-2 GREEN, merged PR #30).
3. **SPEC-ALIGNMENT-AND-ACTION-PLAN §4** — added a "F1–F8 landed (RUN-02)" status note so the intro line no longer reads as a pending worklist.

## Narrative/thinking gaps surfaced (NOT written — owner-gated)

The design layer is broadly current; two specific implications from the July bursts have **not** reached
`thinking/`, and both are genuine design stories worth a deliberate write (your pen, not mine):

1. **[highest] PDS as the personal deep-history backend.** RUN-HIST proved a person's older history can
   live on their own PDS as cold storage, authenticated by a local **reference tail** (checkpoint
   byte-heads), with the CID serving directly as the reconciliation byte-head. Today that lives only as
   atproto *mechanics* in cairn. As a Drystone design implication — *"you keep a small local proof-tail;
   your deep history rides your PDS; any store re-hydrates from the repo"* — it belongs in `thinking/`
   (near `multi-device.md` / `governance-and-survivability.md`). Candidate ROADMAP backport.
2. **[medium] The sealed tier reaches the browser with no bridge.** RUN-19 removed the A.8
   deferred-iroh-bridge: a browser is a full sealed-tier MLS client over WebTransport to the content-blind
   DS. That's a deployment-sovereignty story (kids/grandparents on a PWA get real E2EE, no native app)
   that connects to the skylite/pdsview PWA lineage — currently only in `realtime-media-over-iroh.md` +
   spec §A.8, not the app/client thinking.

(Optional, lower: an AP-ambassador narrative note — "Croft federates by respecting the other protocol's
customs and carries no governance conductivity across the bridge" — but the cairn AP/Mastodon stone
mostly covers this.)

## Verdict

**Status capture: strong.** Registers were fed per-run; the only real gaps were two missing MASTER-INDEX
rows (wasm-seal, RUN-HIST-02), now fixed, plus one cosmetic (SPEC-ALIGNMENT §4), now tidied. **Narrative/
thinking: broadly current, two real implication-gaps** (PDS-deep-history, browser-sealed-tier) surfaced
for a deliberate owner-authored write. No work from the July bursts is lost; the design-layer integration
of two findings is the outstanding piece.
