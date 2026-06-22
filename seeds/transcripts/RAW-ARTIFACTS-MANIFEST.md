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

## 2026-06-22 intake (2) — Croft ponds & pads / games deep-dive + `apps.zip`

Continuation of the app/client-layer body: the deep dive into what fills the garden (games pond,
utility pads, presence-and-ritual pond) and how to build it. The **run** of the
`seeds/generated-prompts/games-pond-research-prompt.md`.

| Raw artifact | Status | Note |
|---|---|---|
| **Croft ponds/games dialogue (2026-06-20→22)** — games hunt, webxdc security model, the moat-from-not-having-things, maintenance-vs-attention + cooperative economics, utilities + presence/ritual ponds, complexity/UX + deep-linking, on-device-LLM feasibility, build-order, fair-reveal | **preserved-condensed** (cleaned-paste, content-faithful — not a pristine export) | Filed at `seeds/transcripts/raw/croft-app-ponds-games-dialogue-2026-06-20-to-22.md`. Same §4 status as the companion app-design dialogue. The conversational-only threads (the moat analysis, the economic frame, the Presence & Ritual pond, the channel-adaptive deep-link breakdown) are preserved in it since they never became artifacts. |
| **`apps.zip`** — 8 artifacts (p2p-games-pond-launch-set, webxdc-security-and-competitive-games, games-pond-authoritative-list, apps-pond-utility-list, build-shape-pass, on-device-llm-feasibility, build-order, fair-reveal-primitive-spec) | **preserved-verbatim** | Unpacked byte-identical to `seeds/apps-unpacked/` (verified). Working copies at `thinking/app/ponds/` (indexed in `thinking/app/README.md`). Zip retire pending user authorization (contents preserved). |

Note on verification: unlike intake (1), this dialogue did real in-session web verification (the
iroh-docs data model, app licenses via the GitHub mirror, on-device-LLM platform APIs, deferred-link
platform state); those verdicts live in the artifacts. The `ECOSYSTEM.md` §5d rows added from it
carry the dialogue's own verification status, still worth a final glance at bundle time.

## 2026-06-22 intake (3) — Croft-app Phase-0 port-decision review + `appframework.zip` (superseded)

A focused review session on the then-current Phase-0 docs, and a zip of its output.

| Raw artifact | Status | Note |
|---|---|---|
| **Croft-app port-decision review (2026-06-21)** — gap analysis of the Phase-0 `BUILD-SPEC.md` / `design-philosophy.md`; the two-sided port-ownership argument (the `update` signature forces shell-holds-the-port); derivation of DECISION 1–5; no-fabricated-fixtures rule | **preserved-condensed** (cleaned-paste, content-faithful — not a pristine export) | Filed at `seeds/transcripts/raw/croft-app-portdecision-review-2026-06-21.md`. Filed because the *derivation reasoning* was absent from `croft-app-design-dialogue-2026-06-20-to-22.md`; the *outcomes* (DECISION 1–5) already live, further-developed, in the repo docs. The session's final edited docs = the `appframework.zip`. |
| **`appframework.zip`** — 2 artifacts (`BUILD-SPEC.md`, `design-philosophy.md`) | **superseded / NOT imported** | Earlier snapshots (25,766 B / 23,282 B) of docs already imported in a more-developed form at `thinking/app/build-specs/BUILD-SPEC.md` + `thinking/app/design-philosophy.md` (27,378 B / 37,350 B = the frozen `multiecosystemapp-unpacked/` seed, which adds §3a/§1a/§4a). Net-new vs. repo: none. Docs left untouched (overwrite would regress). Zip remains in workspace root; discard recommended at bundle time, pending user OK. See COHESION §20. |

## 2026-06-20 intake — Croft cross-platform identity provenance dialogue

Distinct identity dialogue (the *cross-platform linkage* axis, separate from the MLS-root / PLC-replica
work in `plc-identity-resilience.md`).

| Raw artifact | Status | Note |
|---|---|---|
| **Croft identity-provenance dialogue (2026-06-20)** — verification of a did:webvh↔did:plc bridge doc; closed-root-of-trust → out-of-band attestation as the only cross-platform linkage; hub-and-spoke (did:webvh root, did:plc/AP/Hive spokes); attestation-not-derivation key lineage (`nextKeyHashes` pre-rotation + verification methods); `alsoKnownAs` equivalence ladder; did:webvh portability (`portable` genesis-only); per-platform field-support table; did:plc↔did:webvh convergence (#2705) | **preserved-condensed** (cleaned-paste, content-faithful — not a pristine export) | Filed at `seeds/transcripts/raw/croft-identity-provenance-dialogue-2026-06-20.md`. Did real in-session web verification, cites sources inline (did:plc spec, W3C DID-core, did:webvh spec, atproto #2705/#2821). Distilled to `thinking/cross-platform-identity-provenance.md`. **Artifact gap (§4):** the produced "Webvh bluesky bridge" doc's final corrected text was NOT in the paste — only the input draft + the assistant's summary of changes; marked inline as a pointer, not fabricated. See COHESION §21. |

## 2026-06-20 intake — Croft architecture & design-imperative dialogue (the deep "why" + system architecture)

The deepest "why + how" walk: storage tiers → blind peers → search modes → the delegate primitive →
functional planes → governance-as-substrate → BGP/postal/DNS-style federation → the local-first thesis.
Produced the cross-field lineage essay artifact. The civic/epistemic foundation under the whole project.

| Raw artifact | Status | Note |
|---|---|---|
| **Croft architecture & design-imperative dialogue (2026-06-20)** — capability-vs-authority delegates; (predicate, sealed-payload) primitive; planes by blast-radius + "namespace delegations never cross"; reconvergence-policy-per-plane (the Kleppmann resolution); governance-as-substrate + meta-rule-dominates; no-right-to-remove-rights / wolf-test / inverse-correlation; equal-rights-generates-variety-of-form; BGP-autonomy + postal-hierarchy + signed-routing federation; identity-vs-locator; DNS-as-swappable-resolver; **local-first as the generative premise** (architecture = epistemology); the cross-field lineage (Socrates→Mill→Peirce/Popper→Hayek→Ostrom→Ashby→Beer→Scott) | **preserved-condensed** (cleaned-paste, content-faithful — not a pristine export) | Filed at `seeds/transcripts/raw/croft-architecture-design-dialogue-2026-06-20.md`. Long dialogue: every USER turn + every distinct ASSISTANT claim preserved, verbatim repetition lightly compressed. Real in-session web verification with inline citations (Mill/Hayek/Ostrom/Ashby/Beer/Scott verbatim; Kleppmann BFT-CRDT/equivocation papers; commons-DAO Frontiers paper). Distilled to `thinking/local-first-as-design-imperative.md` + `crystallized/principles.md` ("deeper foundation"). See COHESION §22. |
| **`Croft-Lineage-of-a-Design-Imperative.docx`** — the cross-field, cross-millennium lineage essay (the verified "why") | **artifact, content reproduced** | Working copy filed at `narrative/lineage-of-a-design-imperative.md` (full text from the artifact; verification status is the dialogue's own, appendix preserves the verified-vs-confirm-before-publishing flags). The `.docx` itself was not provided as a file — text reproduced from the paste. |
| **Kleppmann letter** — mock letter contrasting Croft's per-plane reconvergence with CRDT Strong Eventual Consistency | **inline-only (not a file)** | Written inline in the dialogue, never saved as a file; preserved verbatim inside the raw transcript. |
| **trailing did:webvh/did:plc bridge re-verification** (search-down session) — genesis `prev`=null, Ed25519 not valid as PLC rotation key, webvh log-entry wrapper, "Equivalency Assertion" not standard | **corrections folded** | In the raw transcript's appendix; distilled into `thinking/cross-platform-identity-provenance.md` ("Bridge-doc technical corrections"). `[UNVERIFIED]` items flagged (search was down). |

## 2026-06-22 intake — Croft-app Phase 0 code (CroftC PR #10) → `experiments/croft-app-phase0/`

The deferred A8 import: the Phase-0 functional core + shell stack built externally, now on
chasemp/CroftCommunity infra at the user's direction (A8 IP/ownership decision exercised — surfaced,
not auto-resolved).

| Artifact | Status | Note |
|---|---|---|
| **`croftc/SecurityPolicy#10` code** — Rust Cargo workspace: `crates/{core,bluesky,cli,design,shell,web,desktop}` + as-built `BUILD-SPEC.md`/`design-philosophy.md` + real recorded Bluesky fixtures + ~20 PNG web snapshots | **preserved-verbatim (code)** | Placed at `experiments/croft-app-phase0/` from head branch `claude/experiments-pcl2ym`; verified **byte-identical** (`diff -rq`, empty). Only the PR's `experiments/` subtree imported; SecurityPolicy root `.github/`/`Publish/`/`README.md` excluded. No secrets/build-artifacts (`.gitignore` excludes `/target`, `crates/web/dist/`). Fixtures are real recorded public `getTimeline` responses (no fabrication). |
| **PR #10 conversation** — description, milestones, the deliberate fixtures gap, cycode license findings (`webpki-roots` CDLA-Permissive-2.0; `r-efi` tri-licensed, resolved in-PR), 8 CodeRabbit nitpicks | **preserved-verbatim** | Captured at `experiments/croft-app-phase0/PR-CONVERSATION.md`. Carried findings recorded there + COHESION §23. |
| **coding transcript** | **linked, not separate** | No separate verbatim coding transcript; the design reasoning is the 2026-06-20→22 dialogues already filed (`croft-app-portdecision-review`, `croft-app-design-dialogue`, `croft-architecture-design-dialogue`). Cross-referenced in PR-CONVERSATION.md §"Coding-transcript linkage". |
