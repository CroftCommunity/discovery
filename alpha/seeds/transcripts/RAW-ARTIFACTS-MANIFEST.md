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
| Drystone protocol-naming dialogue (Gemini, pasted 2026-06-22) | `seeds/transcripts/raw/croft-drystone-protocol-naming-dialogue-2026-06-22.md` | **preserved-condensed (cleaned-paste, content-faithful — §4)** + **fact-checked** (`...-FACTCHECK.md`, 2026-06-22; 1 REFUTED fabrication "Skartsia and Tomi", 4 PARTLY — substance grounded). Decision: P2P protocol named **Drystone** (`NAMING.md`); see COHESION §24 |
| AT Proto / PDS / Germ / private-data dialogue (Gemini, pasted 2026-06-22) | `seeds/transcripts/raw/croft-atproto-pds-germ-privatedata-dialogue-2026-06-22.md` | **preserved-condensed (cleaned-paste, content-faithful — §4)** + **fact-checked** (`...-FACTCHECK.md`, 2026-06-22; strong — federation numbers + DM date exact, architecture + #3363/#121 confirmed; errors: `ger.mx`, draft name, Vultr 1-click, WG "officially formed", peers.org miss). **Updates the source-of-truth** `atproto-atmospheric-web-iroh-mobile-FACTCHECK.md` (dated addendum: real community-led **Private Data WG**; Germ matured). See COHESION §26 |
| AT Proto architecture explainer (Gemini, pasted 2026-06-22) | `seeds/transcripts/raw/atproto-architecture-appview-relay-explainer-2026-06-22.md` | **preserved-condensed (cleaned-paste, content-faithful — §4)** + **fact-checked** (`...-FACTCHECK.md`, 2026-06-22; **unusually accurate, mostly restates settled atproto mechanics**). 1 REFUTED (did:plc ≠ "Public Liaison Corporation" → **"Public Ledger of Credentials"**); 1 OUTDATED (relays non-archival since Sync v1.1 — don't store every repo); CONFIRMED-despite-suspicion: current 2 vCPU/12 GB relay (~$34/mo, RPi-capable) + **Tap** (official Go repo-sync tool). **Updates the source-of-truth FACTCHECK** (Addendum 2). See COHESION §27 |
| Sovereign PDS/AppView "club" + open-social naming/interop dialogue (Gemini, pasted 2026-06-22) | `seeds/transcripts/raw/croft-atproto-sovereign-appview-open-social-dialogue-2026-06-22.md` | **preserved-condensed (cleaned-paste, content-faithful — §4); DUPE/FORK consolidated** (PDS/AppView middle pasted twice; superset filed once; tail diverged → clients + Twitter Circles trilogy) + **fact-checked** (`...-FACTCHECK.md`, 2026-06-22, 5 passes; **unusually accurate — all named projects real**). Minor drift: "501(c)(3)"→nonprofit, "AT Community Fund"→**Free Our Feeds**, Series B 2025-close/2026-disclose, Communities ~May 2026, Heron "WriteQueue" unverified, Rhizome=stem. Overlaps §27/§26/§25. Distilled → `research/atproto-sovereign-appview-club.md` + ECOSYSTEM §5f; naming → `NAMING.md` reservoir (Till/Tillage). See COHESION §29 |
| Solid / WebID / Scaling-Trust / DSNP dialogue (Gemini, pasted 2026-06-22) | `seeds/transcripts/raw/solid-pds-webid-scalingtrust-dsnp-dialogue-2026-06-22.md` | **preserved-condensed (cleaned-paste, content-faithful — §4)** + **fact-checked** (`...-FACTCHECK.md`, 2026-06-22; **well-grounded, no fabrications**). Solid/WebID/Solid-OIDC/DPoP(RFC 9449)/Inrupt all CONFIRMED; DSNP CONFIRMED (token-free core + delegation; reference chain **Frequency**/Polkadot, which Gemini omitted); "Scaling Trust" date/publisher CONFIRMED, exact recs UNVERIFIED (real+on-topic; middleware/C2PA not fabricated, just not verbatim-extractable); Bluesky "public-by-default PDS" = PARTLY. Comparative landscape, low distill yield. See COHESION §28; ECOSYSTEM §5 (Solid, DSNP) + §7 (C2PA, Scaling-Trust) |
| Croft etymology + commons-rebellion tradition + global enclosure dialogue (pasted 2026-06-23) | `seeds/transcripts/raw/croft-etymology-enclosure-tradition-dialogue-2026-06-23.md` | **preserved-condensed (cleaned-paste, content-faithful — §4)**. Historical/lexicographic (open MED/Bosworth-Toller/DSL checkable; OED dates relayed second-hand). No atproto/iroh claims → FACTCHECK doesn't bear. `[UNVERIFIED]`: 1772 Manchester Directory wording (bleaching-vs-farming sense), Goose-poem dating, chronicle-tradition anecdotes. Distilled → `narrative/verticals/croft-the-name-and-the-commons.md` + `NAMING.md` (etymology section); deepens the two earlier crofting files. See COHESION §34 |
| Discord money / IPO / moderator-value / onboarding dialogue (dialogue 2026-06-22, pasted 2026-06-24) | `seeds/transcripts/raw/croft-discord-money-ipo-onboarding-dialogue-2026-06-22.md` | **preserved-condensed (cleaned-paste, content-faithful — §4)**. Reinforces `research/discord-dominance.md` (ten-second-door thesis); all Discord financials are **third-party estimates, `[UNVERIFIED]`** (private co.). Net-new: IPO figures + **moderator-labor-as-captured-value** framing + the **membership-vs-access** design insight. Distilled → `research/discord-dominance.md` (Update 2026-06-22) + `thinking/membership-vs-access-the-public-door.md`. Maps to E11 (deep-link resolver), D9, D5/E25. See COHESION §36, ROADMAP_TODO E29 |
| Foundation / coop / IP-structure + foundation-name dialogue (pasted 2026-06-23) | `seeds/transcripts/raw/croft-foundation-coop-ip-naming-dialogue-2026-06-23.md` | **preserved-condensed (cleaned-paste, content-faithful — §4)**; long Q&A (user turns close, assistant turns condensed to substance). **NOT-LEGAL-ADVICE** — all legal/financial specifics dialogue-sourced (web search), verify with counsel. No atproto/iroh claims → FACTCHECK doesn't bear. **Naming: Noria = CANDIDATE pending legal clearance, NOT decided** (Watershed/Wellspring rejected); Croft + Drystone settled. Distilled → `thinking/foundation-and-ip-stewardship.md` + `NAMING.md` (foundation-name + Croft collision-check + domains) + ECOSYSTEM §8 (SPI/SFC/Aspiration). Advances D5, partially walks D8. See COHESION §35, ROADMAP_TODO E28 |
| John Clare enclosure poems — companion source (pasted 2026-06-23) | `seeds/transcripts/raw/croft-clare-enclosure-poems-2026-06-23.md` | **public-domain full texts** of *The Mores* / *Remembrances* / *To a Fallen Elm* (Clare d. 1864), supplied as the interior-witness source the etymology dialogue (Part 3) deferred. Texts from The Land Is Ours + David Sutton (editor-dependent spelling/punctuation flagged); Featherstone background essay attributed + key-quoted (copyrighted, not reproduced); Poetry Foundation / Full Text Archive pointers `[UNVERIFIED]`. Linked from the dialogue raw + `narrative/verticals/croft-the-name-and-the-commons.md`. See COHESION §34 |
| Groundmist / Steem-Hive / identity-chain / iroh-games dialogue (Gemini, pasted 2026-06-22) | `seeds/transcripts/raw/groundmist-hive-identity-chain-iroh-games-dialogue-2026-06-22.md` | **preserved-condensed (cleaned-paste, content-faithful — §4); heavy overlap** with already-filed intakes (cites their FACTCHECKs: cooperative-social-union, sovereign-appview, cross-platform-identity, realtime-media). **Fact-checked** (`...-FACTCHECK.md`, 2026-06-22, 3 passes). Net-new verdicts: **Hard Fork 23 ≈ $6.3M / 23.6M STEEM (not $5M)**, 64 accts ✓; **atproto resolves did:plc + did:web ONLY, NOT did:key** (REFUTED); **door-holding "corporation vs person" anecdote attribution UNVERIFIABLE** (The Corporation/Hare real, exact exchange not sourced); all iroh games/tools **real** (libmarathon, ascii-royale, iroh-lan, DataBeam/croc/sendme, callme, iroh-live); Hive DAU + TRON "8M tx/day" UNVERIFIABLE. **Seeds the new marketing/quotes reservoir** `narrative/messaging-and-quotes.md`. See COHESION §30; ECOSYSTEM §5d |

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

## 2026-06-22 intake — crypto-wars → mobile-P2P limits → PDS-hosting economics (Gemini dialogue)

A wide three-body Gemini dialogue. Distinct from the companion atproto/atmospheric dialogue (no
topical overlap with GeoCities/atmospheric-web); confirmed not previously filed (distinctive
anchors — Hush-A-Phone, Carterfone, BassOmatic, Smarsh, Peat, Zimmermann, AltStore — absent from
the corpus before this intake).

| Raw artifact | Status | Note |
|---|---|---|
| **Crypto-wars → P2P → PDS-economics dialogue (Gemini, 2026-06-22)** — (a) the digital-liberty lineage; (b) the mobile-P2P four-property impossibility + protocol landscape; (c) a PDS-hosting + P2P-blended business model with enterprise-compliance grounding | **preserved-condensed (cleaned-paste, content-faithful — not a pristine export)** + **fact-checked** | Filed at `seeds/transcripts/raw/crypto-wars-to-p2p-pds-economics-dialogue-2026-06-22.md`. No UI render chrome in the source; only the inline citation source-name lines and turn markers handled; user/Gemini typos preserved verbatim. Header carries the §4 caveat. Companion `...-FACTCHECK.md` (2026-06-22): **unusually accurate for Gemini.** 3 REFUTED (a fabricated Zimmermann "Stalin" congressional-testimony quote; "Voskop" as a Matrix protocol; the Meyer-letter exact quote on a real July-1977 event); several historical quote-wordings UNVERIFIABLE (Keane letter, Zuboff/Acquisti/Solove). **Notable CONFIRMED-despite-suspicion:** Proton case-no. `4:25-cv-05450` + Judge Martínez-Olguín + Andy Yen quote; China App-Store 30→25% (Mar 2026); **Peat by Defense Unicorns** (real Rust+iroh+Automerge-CRDT+MLS stack); Deloitte $200k + Velox $1.8M off-channel fines; $3.5B+ aggregate. |

Carried-forward: **Peat/Defense Unicorns → ECOSYSTEM §1** as the strongest prior-art match for
Croft's exact substrate bet (Rust+iroh+CRDT+MLS, proven in denied/degraded). The **PDS-hosting
business model + enterprise-compliance demand** (CONFIRMED real) feeds the top open problem
(**sustainability ↔ the cooperative *mechanism*** — ROADMAP_TODO; surface, don't let the model's
for-profit framing become Croft's answer). The **four-property impossibility** backs the existing
lineage-groups MLS rationale (the accepted "unequal peer" = a Delivery Service). The Bazelon
**"privately beneficial without being publicly detrimental"** standard (CONFIRMED) is a legal
ancestor of the **"no right to remove the rights of others"** razor. iroh/atproto facts: cite the
project FACTCHECK; this dialogue does not re-introduce the MST/"Keen" errors.

## 2026-06-22 intake — iroh ecosystem / open-social-aggregators / cooperative "Social Union" (Gemini)

One continuous Gemini dialogue split into topic bodies (filed as raw + per-body FACTCHECK; four
parallel web-research passes 2026-06-22). A fourth body (atproto-PDS/Germ/private-data) was a
**duplicate** of the concurrently-filed `croft-atproto-pds-germ-privatedata` set and was **deleted
after reconciliation** (canonical covered all turns; see note below).

| Raw artifact | Status | Note |
|---|---|---|
| **Iroh / QUIC / local-first ecosystem dialogue (Gemini)** — QUIC-for-P2P, noq multipath/QAD, relay-vs-relayless, Automerge + ALPN + CRDT alternatives (Loro/Y-CRDT/Diamond-Types), Peat/peat-mesh, ecosystem apps, Delta Chat/Chatmail/ArcaneChat | **preserved-condensed (cleaned-paste, content-faithful — not a pristine export)** + **fact-checked** | `seeds/transcripts/raw/iroh-quic-localfirst-ecosystem-dialogue-2026-06-22.md` (+`-FACTCHECK.md`). UI citation chips + image captions stripped (§4). **Unusually accurate, no fabrications.** Fixes: ALPN `iroh/automerge/2` not `/iroh-automerge/1`; Huitema = parallel QUIC-draft co-author not endorser; Peat ring→aws-lc-rs `[UNVERIFIED]`. iroh source-of-truth unchanged. COHESION §31. |
| **Open-social protocols & aggregators dialogue (Gemini)** — Nostr/Blossom/Marmot-MLS, Farcaster (Neynar acq.), Lens (Mask stewardship), thirdweb, Yup, aggregators (Firefly/Bridgy-Fed/Flare/SkyFeed/Mixpost/CrossPoster), Mask + Arbitrum | **preserved-condensed (cleaned-paste, content-faithful — not a pristine export)** + **fact-checked** | `seeds/transcripts/raw/opensocial-nostr-farcaster-aggregators-dialogue-2026-06-22.md` (+`-FACTCHECK.md`). **Solid skeleton; the suspect acquisitions CONFIRMED.** Fixes: Farcaster rent ~$7 not $5; Clanker = token-launchpad; rev ~$1.9M peak not $2.8M; Clovyr/thirdweb exact prices `[UNVERIFIED]`. ECOSYSTEM rows flagged dialogue-sourced. COHESION §32. |
| **Cooperative "Social Union" governance & economics dialogue (Gemini, user's own design thinking)** — MO Chapter 351 LCA, progressive decentralization, capped royalty/RBF, 501(c)(3) tandem, labor-as-first-class, UX-inertia wall, the verified failure-case lineage, federate-on-AT-Proto, MVC/DBA roadmap, MFA/Hirth history | **preserved-condensed (cleaned-paste, content-faithful — not a pristine export)** + **fact-checked (NOT-LEGAL-ADVICE)** | `seeds/transcripts/raw/cooperative-social-union-governance-dialogue-2026-06-22.md` (+`-FACTCHECK.md`). Distilled → `thinking/cooperative-social-union-model.md`. MO Chapter 351 framework real & accurate; **confirmed-wrong specifics flagged inline** (§351.1015→§351.1036; CA-41 $100 not $105; name-res $25 not $20; Git/Inkscape are SFC; CHS ~$45.6B; DFA=Kansas City *Kansas*). All tax-side + case studies (incl. Coomappa) CONFIRMED. **The major new body** — partially walks ROADMAP_TODO D5 (sustainability ↔ cooperative mechanism). COHESION §33; ROADMAP_TODO E25. |

Reconciliation note (DEDUP): the fourth body, originally filed as
`atproto-pds-architecture-technical-dialogue-2026-06-22.md` (+FACTCHECK), duplicated the
concurrently-filed `croft-atproto-pds-germ-privatedata-dialogue-2026-06-22.md` set (same Gemini
content/snippets). The canonical set covers all turns (Wasabi/Glacier/peers.org/object-store
pricing); my duplicate was **deleted**. **Fact-check correction carried:** I had flagged the
"current" Bluesky federation limits (100 accts / 2,600-hr / 21,000-day) as fabricated — that was a
**false positive** (my research pass checked the PDS `env.ts`, not the rate-limits doc). They are
**real** (`docs.bsky.app/docs/advanced-guides/rate-limits`); the canonical FACTCHECK correctly marked
them CONFIRMED.

## 2026-06-24 intake — Drystone peers/rights/governance + Matrix-contrast dialogue (+ §2/§X spec drafts)

A single claude.ai design dialogue (content-dated 2026-06-24, filed 2026-06-25) that diagnosed the
group-chat landscape, ran a deep **Matrix architecture-and-UX contrast**, and produced two Drystone
spec sections (§2 Peers/Rights/Capabilities/PeerSets, §X Concurrent & Conflicting Governance Changes).

| Raw artifact | Where | Status |
|---|---|---|
| Drystone peers/rights/governance + Matrix-contrast dialogue (claude.ai) | `seeds/transcripts/raw/drystone-peers-rights-governance-matrix-dialogue-2026-06-24.md` | **preserved-condensed (cleaned-paste, content-faithful — §4)**. UI chrome stripped; inline source-domain citations kept. **Self-correction preserved:** the assistant's false "Matrix E2EE is bilaterally disable-able" claim, corrected to the spec's **one-way encryption latch**. |
| §2 final spec draft (canonical; the §2 pasted at the top of the session was an earlier **dupe**, not re-filed) | `thinking/drystone-spec/section-2-peers-rights-capabilities.md` | **preserved-verbatim** (working DRAFT) |
| §X final consolidated spec draft | `thinking/drystone-spec/section-x-governance-conflicts.md` | **preserved-verbatim** (working DRAFT / ENABLING) |

**Provenance / verification note (NEW facts, not yet in the FACTCHECK SoT):** this dialogue adds a large
body of **Matrix / Willow / Meadowcap / Keyhive** claims web-verified *in-session only* — confirm the
load-bearing ones (CVE-2025-49090; room v12 / MSC4289; Megolm/UTD; Seshat desktop-only encrypted search;
Meadowcap "no native revocation"; Willow unenforceable plaintext U64 timestamp + `is_authorised_write`;
matrix.org Postgres-corruption postmortem; Karlsruhe SACMAT 2020; Element X vs Classic) before they
harden into beta. iroh consistent with SoT (`1.0.0`); no drift markers. COHESION §37; ROADMAP_TODO
A11/A12/A13 + E30. Beta integration is a **separate pass** (not done in this intake).
