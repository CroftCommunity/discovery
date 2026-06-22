# Raw Artifacts Manifest

date: 2026-06-15

purpose: honest inventory of every raw input that fed this consolidation, where it is
preserved, and **what is NOT yet preserved** — so we can review and confirm nothing is lost.

status legend: **preserved-verbatim** · **preserved-condensed** (a readable rendering, not
the exact paste) · **distilled-only** (analyzed into outputs, raw not kept) · **MISSING**.

---

## Preserved

| Raw artifact | Where | Status |
|---|---|---|
| GroupDynamics.zip (4 produced docs) | `discovery/GroupDynamics.zip` + `seeds/groupdynamics-unpacked/` | preserved-verbatim |
| Sovereign Commons dossier | `discovery/SOVEREIGN-COMMONS-DOSSIER.md` | preserved-verbatim |
| Achilles-heel research prompt | `seeds/generated-prompts/` | preserved-verbatim |
| V1–V9 visibility-test prompt | `seeds/generated-prompts/` | preserved-verbatim |
| PR #3 conversation (desc + 70 reviews + 30 inline) | `Proofs/encrypted-local-first-atproto/PR-CONVERSATION.md` | preserved-verbatim (from gh) |
| PR #4 conversation | `experiments/public-roundtrip/PR-CONVERSATION.md` | preserved-verbatim (from gh) |
| PR #6 conversation | `experiments/appview-validation/PR-CONVERSATION.md` | preserved-verbatim (from gh) |
| PR #8 conversation | `Proofs/lineage-groups/PR-CONVERSATION.md` | preserved-verbatim (from gh) |
| PR #9 conversation | `Proofs/lineage-group-model/PR-CONVERSATION.md` | preserved-verbatim (from gh) |
| PR #3/#4/#6/#8/#9 full code trees | the `Proofs` and `experiments` repos | preserved-verbatim |
| Crofting narrative re-telling (pasted 2026-06-22) | `seeds/transcripts/raw/croft-crofting-narrative.md` | preserved-verbatim (quotes/anecdotes `[UNVERIFIED]`; tertiary sources — see COHESION §16) |
| AT-Proto atmospheric-web / Iroh mobile dialogue (Gemini, pasted 2026-06-22) | `seeds/transcripts/raw/atproto-atmospheric-web-iroh-mobile-dialogue.md` | preserved-verbatim + **fact-checked** (`...-FACTCHECK.md`, 2026-06-22; 2 REFUTED fabrications, several PARTLY — see COHESION §17) |

## Code — verbatim confirmed

Each proof/experiment tree was `git clone`d from its croftc SecurityPolicy PR branch and
copied unchanged. `diff -rq` against the branches is **empty** (only the added
PR-CONVERSATION/CODING-TRANSCRIPT files and excluded SecurityPolicy plumbing differ). Code
provenance is verbatim.

## Coding transcripts — now preserved verbatim (raw archive) + condensed

The verbatim raw coding transcripts are archived in `seeds/transcripts/raw/`. A readable,
condensed rendering also sits next to each artifact as `CODING-TRANSCRIPT.md`.

| Coding transcript | Verbatim raw | Condensed |
|---|---|---|
| PR #6 appview-validation | `seeds/transcripts/raw/pr6-appview-validation.md` | `experiments/appview-validation/CODING-TRANSCRIPT.md` |
| PR #4 public-roundtrip / capstone / moderation | `seeds/transcripts/raw/pr4-public-roundtrip.md` | `experiments/public-roundtrip/CODING-TRANSCRIPT.md` |
| PR #9 lineage-group-model | `seeds/transcripts/raw/pr9-lineage-group-model.md` | `Proofs/lineage-group-model/CODING-TRANSCRIPT.md` |
| PR #8 lineage-groups Phase 0/1 | `seeds/transcripts/raw/pr8-lineage-groups.md` | `Proofs/lineage-groups/CODING-TRANSCRIPT.md` |
| PR #3 encrypted-local-first | `seeds/transcripts/raw/pr3-encrypted-local-first.md` | `Proofs/encrypted-local-first-atproto/CODING-TRANSCRIPT.md` |
| PR #7 android-p2p-app | `seeds/transcripts/raw/pr7-android-p2p.md` | `experiments/android-p2p-app/CODING-TRANSCRIPT.md` |
| PR #5 encrypted-blob-share | `seeds/transcripts/raw/pr5-encrypted-blob-share.md` | `experiments/encrypted-blob-share/CODING-TRANSCRIPT.md` |
| Germ/X Chat design dialogue | `seeds/transcripts/raw/germ-xchat-design-dialogue.md` | distilled → thinking/interaction-tiers.md + principles Tier 3 |
| Croft crofting research | `seeds/transcripts/raw/croft-crofting-research.md` | distilled → NAMING.md |
| P2P architecture **origin** dialogue (2026-06-02/03) | `seeds/transcripts/raw/p2p-architecture-origin-dialogue.md` | **preserved-condensed** — earliest seed (stack + economics + coop first reasoned out); verbatim re-drop can replace |

## Session 2026-06-15 (web-research imports — deliverables filed by taxonomy)

Five web-research sessions were imported. Deliverables filed (research = analytical lens;
thinking = our evolving design); raw dialogues kept where unique, per "file what's unique, keep the
raw transcripts."

| Deliverable | Filed at | Source dialogue |
|---|---|---|
| social-platform-cycle, discord-dominance, public-social-protocols, discord-matrix-groupchat, germ-xchat-features | `research/` | various web sessions (cite tags stripped on import) |
| group-chat-failure-modes (+ -plain) | `research/` | peer-fracture / failure-modes dialogue |
| p2p-founder-motivations-adoption | `research/` | founder-motivations web session |
| model-holds-up-summary, design-notes-addendum, experiment-suite | `thinking/` | peer-fracture / failure-modes dialogue |
| group-privacy-lanes-design-note, plc-identity-resilience | `thinking/` | realtime-chat & socials-compare sessions |
| governance-and-survivability, open-considerations | `thinking/` | distilled from the feasibility + governance + design-review dialogue |

Source zips (chainvalidation, socialscompare, realtimechatcompare, peercrypto) were unpacked into
the repo and deleted from `discovery/` root.

### Still to preserve verbatim (re-drop pattern)

- **Peer-fracture / group-chat-failure-modes design dialogue** — fully distilled into the five docs
  above; the raw back-and-forth (Merkle-ancestry unification, capabilities-not-rights, the trap
  door, complementary-vs-contradictory boundary) is **not yet kept verbatim**. Re-drop to save as
  `seeds/transcripts/raw/peer-fracture-design-dialogue.md`.
- **Founder-motivations web session** — deliverable filed; dialogue is mostly search logs,
  distilled-only.
- **"Maintenance phase" quote research** — minor; finding captured in the origin-dialogue closing
  note above.

Fidelity note: the raw files reproduce the pastes verbatim, except (a) in-session test
credentials are redacted, and (b) two large embedded briefs already saved verbatim elsewhere
(the lineage thesis → `thinking/thesis-lineage-groups.md`; the experiment-suite spec →
`Proofs/lineage-group-model/CODING-TRANSCRIPT.md`) are referenced rather than triplicated.

## Previously MISSING — now CLOSED

| Raw artifact | Status | Note |
|---|---|---|
| **The original design-dialogue transcript** (Delta Chat/SSB/Signal research + the multi-device → social-layer → scale-axis design conversation, incl. the voice notes, "six tapes," "teenage gossip heaven," fold/unfold, recombine, the two-regime/propagation-geometry material) | **preserved-verbatim** | Re-dropped by the user and filed at `seeds/transcripts/design-dialogue-2026-06-13-to-14.md`. The richest single seed — where the design reasoning happened. Closes the last provenance gap. (Assistant research-phase source citations preserved as they appeared.) |

## Upstream source of record

The five croftc/SecurityPolicy PRs (#3, #4, #6, #8, #9) remain the canonical upstream for
the code and conversations. They are OPEN as of capture; if branches are later merged/deleted,
our imported copies are the preserved record.

## To close the gap

1. Re-drop the original design-dialogue transcript → save verbatim as
   `seeds/transcripts/design-dialogue-2026-06-13-to-14.md`.

2. (Optional) Replace the condensed `CODING-TRANSCRIPT.md` files with the verbatim pastes if
   exact fidelity is wanted for review.

## 2026-06-22 intake — Croft app design dialogue + artifact zip (the app/client layer)

A new body of work: the **application/client layer** ("Croft" the product), distinct from the
protocol. This intake lands the design thinking; the experiment (code) is deferred (CroftC PR #10).

| Raw artifact | Status | Note |
|---|---|---|
| **Croft app design dialogue (2026-06-20→22)** — the full architecture / values / stack / iroh-tiers / appview / ponds-pads / games / super-apps / palette / brand / session-review / Phase-0-1-2 conversation | **preserved-condensed** (cleaned-paste, content-faithful — not a pristine export) | Filed at `seeds/transcripts/raw/croft-app-design-dialogue-2026-06-20-to-22.md`. Per user, no canonical export exists, so this is the best-available raw: UI render chrome stripped, dialogue wording preserved as faithfully as the rendered paste allowed. Header carries the §4 caveat. Two large mid-paste blocks (the CroftC PR #10 page; full re-pastes of the then-current docs) were themselves truncated in the source — represented as bracketed pointers to the frozen artifacts / the PR. |
| **`multiecosystemapp.zip`** — 6 artifacts (design-philosophy, design-criteria, brand-and-voice-notes, BUILD-SPEC, BUILD-SPEC-PHASE-1-2, games-pond-research-prompt) | **preserved-verbatim** | Unpacked byte-identical to `seeds/multiecosystemapp-unpacked/` (verified). Working copies distilled to `thinking/app/` (+ `build-specs/`) and `seeds/generated-prompts/`. Zip retire pending user authorization (contents preserved). |

Carried-forward: the dialogue's open risks (infra-sustainability ↔ cooperative *mechanism*,
moderation/safety vs the kid-friendly goal, cold-start for the owned pond, the **CroftC
IP/ownership entanglement**) → `thinking/open-considerations.md` + `thinking/app/README.md`. The
embedded industry research (iroh-in-browser, webxdc/Delta-Chat games + the WebRTC-transport-swap
porting recipe, super-apps / W3C MiniApp, atproto appview routing, Rust client libs, Crux/FCIS) is
not yet distilled into `research/` / `ECOSYSTEM.md` — flagged as a follow-on in ROADMAP.
