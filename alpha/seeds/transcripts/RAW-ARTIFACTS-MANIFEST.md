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
| Beta 01 read-through review (voice-transcribed, recorded 2026-06-26) | `beta/thinking/raw/01_beta_review.txt` | **preserved-verbatim** (exact transcription file, not a cleaned chat paste). The user's own editorial read-through of `beta/01-epistemic-foundation.md`, dictated "for Claude refinement." No atproto/iroh/iOS claims → FACTCHECK doesn't bear; the user's own design reasoning, not external facts to verify. Drove the beta-01 → Drystone-protocol-spec build (`beta/drystone-spec/`); classified extraction in `plans/2026-06-26-beta-01-review-refinements.md`. See BETA-ROLLUP "01 review → Drystone spec 2026-06-26" |
| Beta open-threads read-through review (voice-transcribed, recorded 2026-06-26) | `beta/thinking/raw/open threads review Jun 26 at 8-17 PM.txt` | **preserved-verbatim** (exact transcription file, not a cleaned chat paste). The user's own editorial read-through of `beta/OPEN-THREADS.md`, dictated as follow-up notes. No atproto/iroh/iOS claims → FACTCHECK doesn't bear; the user's own design reasoning, not external facts to verify. Drove the OPEN-THREADS refactor (status/type taxonomy; promoted/closed moved out of the live queue; **T31** rights/role/capability disentanglement + **T32** legal-review added; structural proposals **S1–S6** surfaced). Classified extraction in `plans/2026-06-26-open-threads-review.md`. See BETA-ROLLUP "Open-threads review 2026-06-26" |
| Beer / algedonic / Cybersyn / OGAS dialogue (claude.ai, pasted 2026-06-26; dated 2026-06-25) | `seeds/transcripts/raw/beer-algedonic-cybersyn-ogas-dialogue-2026-06-25.md` | **preserved-condensed (cleaned-paste, content-faithful — §4)**. The "new Beer transcript" promised when Ashby/Beer were staged (T23). Web-searched historical facts **not yet in FACTCHECK SoT** — **[confirm before publish]**: Beer quotes "ride the dynamics" (*Brain of the Firm*) + "only hope" (*Designing Freedom* 1974) defensible-verbatim; "aids to human viability…" **confirmed secondary gloss → own synthesis**; Cybersyn/OGAS dates+figures (coup 1973-09-11; OGAS funding denied 1970-10-01; 20,000 terminals; "10 billion people" calc) web-sourced. Net-new for the spec: real Beer/algedonic grounding; the **adjudication-locus axis** (Cybersyn vs OGAS); **peerhood = where decision rights sit** (a peer adjudicates, not just senses); **exit-backed authority** + "what makes rights cost something to violate." Incorporated → Drystone spec Part 1 §3 + Part 2 §3/§5/§7/§8/App-B; synthesis → `thinking/algedonic-and-peerhood-as-adjudication.md`. Closes OPEN-THREADS T23. See COHESION §40 |
| Social-graph-as-substrate / storage architecture / redb dialogue (claude.ai, pasted 2026-06-26) | `seeds/transcripts/raw/social-graph-substrate-redb-storage-dialogue-2026-06-26.md` | **preserved-condensed (cleaned-paste, content-faithful — §4)**. The second "benefit from current context" handoff (Drystone-spec session). Web-searched facts **not yet in FACTCHECK SoT** → **[confirm before publish]**: Holepunch/Keet, ATProto, Gun/OrbitDB/SurrealDB/Fluree/Veilid, redb 3.x API, Automerge per-doc sync, MLS RFC 9420 (leaves/no-nesting/no-dup-keys/AS-assumed) + RFC 9750 multi-device credential policy. Net-new: **social graph as the substrate (chat as a tenant), group≠member-set, implicit/sticky group lifecycle, recursive principal (composition vs valuation edges), authoritative assertion-DAG + derived redb projection (local-first CQRS), governance-log + declarative-cache + verifiable roll-up, MLS devices-as-leaves + user-principal-as-self-AS credentials, trust-vs-provenance philosophy.** Produced a **redb build prompt** → `seeds/generated-prompts/redb-social-graph-layer-build-prompt.md`. Synthesis → `thinking/social-graph-as-substrate.md`; protocol refinements → Drystone spec Part 1/Part 2; app reframe + build staged in OPEN-THREADS. See COHESION §41 |
| Field-trades / four-property-impossibility / DMLS-FREEK + redb dialogue (claude.ai, pasted 2026-06-26) | `seeds/transcripts/raw/field-trades-four-property-impossibility-dmls-and-redb-dialogue-2026-06-26.md` | **preserved-condensed (cleaned-paste, content-faithful — §4)**. An adversarial fact-check of `beta/03`'s field framing + the four-property impossibility, plus a redb confirmation. Web-verified-in-dialogue → **[confirm before publish]**. **Corrections to `03` (folded):** Signal phone-rooted only at *registration* (usernames 2024, not the contact graph); **Delta Chat no longer "inherits email's metadata leak"** — RFC 9788 Header Protection in 2.48+ (Mar 2026) + chatmail relays store no metadata → only a **relational residue** at the relay (no Sealed Sender yet); the **four-property "impossibility" is overstated** — an engineering tension with a quantified FS cost and an **active counterexample (DMLS/FREEK; FREEK = Alwen/Mularczyk/Tselekounis, puncturable-PRF)**, ordering can be deterministic not a privileged peer, **no production deployment ships the escape** (Webex/Wire/Discord/RCS all server-ordered; DMLS + `draft-xue-distributed-mls` are drafts/PoC). SSB + Briar confirmed. redb facts confirmed (1.0 Jun 2023, savepoints, MVCC, per-txn durability, stable file format). Folded → `beta/03` (corrections, flagged); DMLS/FREEK prior art → OPEN-THREADS T29 + ECOSYSTEM + spec App-A; synthesis → `thinking/field-trades-and-the-ordering-tension.md`. See COHESION §42 |

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

## 2026-06-24 intake — Drystone publication & defensive-disclosure dialogue (+ spec skeleton)

A claude.ai design dialogue (content-dated 2026-06-24, filed 2026-06-25) on **how Drystone goes public and
stays open**: IETF/RFC document conventions, the one-document-principles-up-front structure, the iroh-first
feedback venue, and the defensive-publication mechanism (CC0 text + patent-non-assertion notice + Zenodo DOI
+ OpenTimestamps as the priority/prior-art vehicle). Produced a Drystone spec **v0.1 skeleton**.

| Raw artifact | Where | Status |
|---|---|---|
| Publication / defensive-disclosure dialogue (claude.ai) | `seeds/transcripts/raw/drystone-publication-defensive-disclosure-dialogue-2026-06-24.md` | **preserved-condensed (cleaned-paste, content-faithful — §4)**. UI chrome stripped; the embedded spec-skeleton block kept. |
| Drystone spec v0.1 skeleton (generated in-dialogue) | `thinking/drystone-spec/drystone-spec-v0.1-skeleton.md` | **working copy (DRAFT scaffold)** — the document scaffold the §2/§X deep drafts slot into |
| Distilled publication/IP thinking | `thinking/drystone-publication-and-defensive-disclosure.md` | distilled |

**Provenance / verification note:** all IETF/RFC-process facts, RFC numbers (2360, 6762, 6709, 2119/8174,
9000), the IKEv2 rationale-draft precedent, **Zenodo** specifics, **OpenTimestamps**, arXiv, and IETF-Trust
draft-reuse terms are **dialogue/web-sourced 2026-06-24, NOT independently re-verified** — `[UNVERIFIED]`
pending a primary-source pass. iroh consistent with SoT (`1.0.0`); no drift markers. **Beta impact:** this
dialogue **refines** beta `07` Pillar C's settled prior-art-vehicle posture (was "IETF Internet-Draft first
then arXiv"; now Zenodo DOI + OpenTimestamps first, IETF draft demoted as more-encumbered/later) — folded
into `07` 2026-06-25 per user approval (see `BETA-ROLLUP.md` K-table). The **text license is CC0 1.0**
(decided 2026-06-25, user-approved; folded into `07` C2 over CC-BY 4.0; Apache-2.0 reference-code license
unchanged; A14 closed). COHESION §38; ROADMAP_TODO A14 + E31.

## 2026-06-24 intake — Rights-vs-capabilities definitional dialogue

A claude.ai design dialogue (content-dated 2026-06-24, filed 2026-06-25) grounding beta `01` §5's boundary
("no right to remove the rights of others") by making the **rights-vs-capability line sharp**: the
discriminating test (a right is standing whose removal cancels the conditions of its own contestation; a
capability's removal leaves contestation intact), the four rights (tenure/exit/voice/share) defined by what
their removal forecloses, capabilities as the open remainder, the voice-vs-amplification edge case, and two
verify-before-hardening items (share-as-right; tenure-vs-04-survivor-rekey).

| Raw artifact | Where | Status |
|---|---|---|
| Rights-vs-capabilities definitional dialogue (claude.ai) | `seeds/transcripts/raw/rights-vs-capabilities-definitions-dialogue-2026-06-24.md` | **preserved-condensed (cleaned-paste, content-faithful — §4)**. UI chrome stripped; the 01-§5 framing paragraph kept verbatim. The session-opening paste of current `beta/01` is the live beta state, not re-filed. |
| Distilled definitions block | `thinking/rights-vs-capabilities-definitions.md` | distilled (the cite-able block) |

**Beta impact:** the rights-vs-capability grounding was **folded into `beta/01` §5 on 2026-06-25
(user-approved, K17)** — a tier-clean paragraph after the boundary bullets. Two verify-before-hardening
checks (share-as-right; tenure-vs-04-survivor-rekey) remain open and were deliberately left out of the beta
narrative (ROADMAP_TODO E32 b/c). No external facts (the user's own design reasoning; nothing to fact-check).
COHESION §39; ROADMAP_TODO E32; `BETA-ROLLUP.md` (Beta grounding, K17).

## 2026-07-06 intake — Drystone spec revision session (document-pass-1)

A claude.ai design dialogue: hypothesis discussion about local-truth supremacy in distributed systems, six
revisions to Part 1 and Part 2, and a primary-source confirmation pass. Outputs: updated spec Parts 1+2,
CHANGELOG.md, review-handoff.md — all filed into `discovery/beta/drystone-spec/`.

| Raw artifact | Where | Status |
|---|---|---|
| **Drystone spec revision session (claude.ai, 2026-07-06)** — hypothesis review (local-truth supremacy underrepresented in the field; Beer as exception); six revision moves; architecture-relativity of the timestamp constraint (two-sides-one-coin framing); equality model corrected (four properties: two equalities, two inequalities); capability demoted from peer-property to data-access mechanism; peer-vs-personhood keystone; device-group dial correction; dial-discipline principle; Matrix timestamp tiebreak confirmed verbatim (MSC1442 cleared); review-handoff created | `seeds/transcripts/raw/drystone-spec-revision-session-2026-07-06.md` | **preserved-condensed (cleaned-paste, content-faithful — §4)**. UI chrome (tool-call dividers, file-edit markers) stripped; substantive design reasoning preserved. |

**Outputs filed:**
- `discovery/beta/drystone-spec/part-1-reasoning-underpinnings.md` — full replacement (347→792 lines)
- `discovery/beta/drystone-spec/part-2-certifiable-design.md` — full replacement (819→1983 lines)
- `discovery/beta/drystone-spec/CHANGELOG.md` — new companion (493 lines; running revision log)
- `discovery/beta/drystone-spec/review-handoff.md` — new companion (217 lines; standalone reviewer brief)

**Confirmation status changes:** Matrix State Resolution v2 timestamp tiebreak (MSC1442 / `origin_server_ts`
tiebreak fields + authors' admission) confirmed verbatim — §7.3.1 marker cleared. MLS (RFC 9420/RFC 9750),
Meadowcap/Willow, Spritely/ActivityPub, PGP web-of-trust, SSH TOFU also confirmed this session.
Still `[confirm before publish]`: Matrix MSC/CVE specifics (CVE-2025-49090, CVE-2025-54315, MSC4289,
MSC4291, MSC4297); CALM/CAP/CRDTs/Lamport verbatim; Beer verbatim + Cybersyn/OGAS figures; Ostrom verbatim;
iroh relay-blindness; decentralized-MLS/FREEK. No drift markers vs existing FACTCHECK SoT.

**Context:** this intake is part of the beta layer-cake restructure — `drystone-spec/` is Layer 1
(protocol), `impl/` is Layer 2 (reference implementation, experiment-informed — themes 04/05/06), `croft/`
Layer 3, `governance/` Layer 4, `history/` Layer 5, `socialization/` Layer 6. Layer directories to be
populated across subsequent sessions.

## 2026-07-06 intake — Drystone spec review session (document-pass-2)

A structured consistency review against the review-handoff.md brief, followed by author rulings
applied from a voice transcript, and a complete em-dash removal pass.

| Raw artifact | Where | Status |
|---|---|---|
| **Drystone spec review session (2026-07-06)** — A2 peer/member/lineage model correction; A3/A4/A5 meer optional store-and-forward vs iroh relay (RFC 9750 verified); B1 rights floor reduced to three (tenure/voice/exit, share dropped); B2 group-principal/communal-namespace open seam; B5 escalation tolerance default; B6 capability mechanism deferral + Keyhive lean; B8 grounds of authority (variety-enabling, contextual mint-and-bind); D1–D3 citation fixes; 562 em-dashes removed | `seeds/transcripts/raw/drystone-spec-review-session-2026-07-06.md` | **preserved-condensed (cleaned-paste, content-faithful — §4)**. Voice transcript paraphrased faithfully; tool-call UI chrome stripped. |

**Outputs filed:**
- `discovery/beta/drystone-spec/part-1-reasoning-underpinnings.md` — updated (792→796 lines)
- `discovery/beta/drystone-spec/part-2-certifiable-design.md` — updated (1983→2095 lines)
- `discovery/beta/drystone-spec/open-items.md` — new companion (read-and-decide ledger)
- `discovery/beta/drystone-spec/CHANGELOG.md` — document-pass-2 entry prepended

**Confirmation status changes:** RFC 9750 §6.4 (DS optionality) and MLS asynchronous delivery model
verified. Tenure-under-rekey test shape written into Appendix B (B1). No drift vs FACTCHECK SoT.
Still `[confirm before publish]`: Beer, Ostrom, decentralized-MLS/FREEK, TLS/X.509, SSH, Keyhive,
Lamport, CALM, CAP, CRDTs, Matrix CVE/MSC specifics; capped-root soundness (B4) remains priority
security open item.

## 2026-07-06 intake — peer-standing → cooperative-form companion set (third.zip)

The philosophical/political spine that grounds the co-op ownership form: a five-document set
"assembled from conversation" that follows the word *peer* (Latin *par*, "equal") through relational
equality, non-domination, group-agency impossibility, Delaware corporate law, and the domain mismatch,
to the conclusion that an ad-funded securitized corporation cannot constitute peer standing and that a
cooperative form adopted from inception is required. Delivered as a zip because the originating web
client would not render the files. This is a **companion set, not a Drystone spec update** — no
`part-1`/`part-2` overwrite.

| Raw artifact | Where | Status |
|---|---|---|
| **peer-standing → cooperative-form set (5 docs)** — `peer-standing-and-the-cooperative-form.md` (full grounded argument, source of record), `structural-argument-principles.md`, `tilling-the-soil-essay.md`, `sixty-second-pitch.md`, `session-summary.md` | delivered via `beta/drystone-spec/dropoff/third.zip` (scratch — **removed 2026-07-06** after byte-identical copies confirmed in the tree); the **committed tree copies below are the durable verbatim record** | **preserved-verbatim** (deliverables byte-identical, `diff -q` confirmed twice before the zip was removed) |

**Outputs filed (split across layers per the user's 2026-07-06 decision):**
- `beta/governance/peer-standing-and-the-cooperative-form.md` — Layer 4 source of record
- `beta/governance/structural-argument-principles.md` — Layer 4
- `beta/governance/peer-standing-session-summary.md` — Layer 4 (renamed from `session-summary.md`)
- `beta/socialization/tilling-the-soil-essay.md` — Layer 6
- `beta/socialization/sixty-second-pitch.md` — Layer 6
- `beta/governance/README.md`, `beta/socialization/README.md` — new layer indexes (carry the cross-layer navigation; deliverable bodies untouched)
- `beta/OPEN-THREADS.md` — new **T33** (edge-preserving capital formation, the open problem the argument generates)

**Honest provenance note (§4).** The **deliverables** are preserved verbatim. The **originating
conversation** is NOT separately archived as a transcript: it spanned multiple prior web sessions
("assembled from conversation"; the summary references "four conversations ago"), only this session's
tail was visible, and it was chrome-laden claude.ai render — reconstructing it would be lossy, so per §4
it is flagged **not-preserved** rather than faked. The canonical conversation lives in the user's web
history if the full raw is ever wanted. Two grounding gaps the set itself flags remain **not sourced**:
Rochdale/ICA co-op legal mechanics and the platform-cooperativism capital-formation literature (tracked
in T33). Drift grep clean (no `rc.0` / `connect_to_peer` / "merkle search tree" / "AT Messaging working
group" / `ger.mx`); no atproto/iroh/iOS facts touched. Bodies retain their em-dashes (not run through the
Drystone-spec em-dash pass — finished companion docs, not spec).

## 2026-07-06 intake — batch four: activism research set + spec pass-3 + peer-standing update (four-research.zip + four-peers.zip)

Two zips. **`four-research.zip`** is the new *empirical* indictment — "platforms author the relational
field": a five-plank structural narrative, a 14-gap evidence brief (each claim with its primary source,
a verbatim line under the quote limit, and an epistemic tag), and a four-tier reference index. It seeds
a **new Layer 7 `activism/`** (the "why the incumbents are harmful" register, distinct from
`governance/`'s structural/legal case and `socialization/`'s presentation). **`four-peers.zip`** is a
full updated snapshot: the Drystone spec (part1/part2/open-items) plus the peer-standing set, with the
RFC 9420 §16.4 reconciliation and a new Part 1 §2.6 (voice requires field-integrity) folded in. Unlike
third.zip, this batch **does** overwrite the spec — the user's clean-history workflow (overwrite in
place + one commit per version).

| Raw artifact | Where | Status |
|---|---|---|
| **activism research set (3 docs)** — `structural-argument-narrative.md`, `relational-field-research-brief.md`, `reference-index.md` | delivered via `beta/drystone-spec/dropoff/four-research.zip`; filed to `beta/activism/` (below) | **preserved-verbatim** (byte-identical, `diff -q` confirmed) |
| **spec + peer-standing snapshot (8 docs)** — `drystone-part1/part2/open-items.md`, `peer-standing.md`, `tilling-the-soil.md`, `elevator-pitch.md`, `session-summary.md`, `drystone-part1-voice-bridge.md` | delivered via `beta/drystone-spec/dropoff/four-peers.zip`; spec files overwritten in place, peer-standing set overwritten in `governance/`+`socialization/` (below) | **preserved-verbatim** (byte-identical, `diff -q` confirmed); the committed tree copies are the record |
| **originating conversation (research + RFC 9420 threads)** | `seeds/transcripts/raw/platforms-author-relational-field-research-2026-07-06.md` | **preserved-condensed (cleaned-paste, content-faithful — §4)**. User-pasted tail of a longer multi-session conversation; UI chrome stripped, edit-narration condensed, research-brief opening truncated in the paste (noted inline). |

**Outputs filed (batch four):**
- `beta/activism/` — **new Layer 7**: `structural-argument-narrative.md`, `relational-field-research-brief.md`, `reference-index.md`, `README.md`
- `beta/governance/peer-standing-and-the-cooperative-form.md` — overwritten (555→820 lines; empirical grounding folded in), `peer-standing-session-summary.md` — overwritten; `README.md` — activism/spec-§2.6 cross-refs added
- `beta/socialization/tilling-the-soil-essay.md`, `sixty-second-pitch.md` — overwritten
- `beta/drystone-spec/part-1-reasoning-underpinnings.md` (796→875: new §2.6), `part-2-certifiable-design.md` (§7.4 back-ref + RFC 9420 claim-(c) correction), `open-items.md` (§2.6 note + companion-tracked + Project Mercury flag), `CHANGELOG.md` (document-pass-3 prepended) — overwritten
- `beta/README.md` — layer-cake table +activism row (Layer 7); `beta/OPEN-THREADS.md` — new **T34** (Project Mercury docket re-check, time-sensitive)

**Not filed:** `drystone-part1-voice-bridge.md` is the standalone *draft* of §2.6, superseded by the
merged §2.6 in Part 1; kept only as dropoff scratch.

**Honest provenance note (§4).** Deliverables preserved verbatim. The activism set carries its **own
source discipline** (four quality tiers; peer-reviewed / company-primary anchors; conflicts shown, not
resolved — the Meta teen-research and Project Mercury both have both primaries). **Time-sensitive:**
**Project Mercury** is live litigation at the knowledge edge (Nov 2025 filing, hearing set Jan 26 2026,
docs sealed) — pull PACER before any external publication (T34). The five-plank reflexivity
**decomposition is synthesis** ("ours"), not a sourced finding. Non-load-bearing follow-ups still open:
Tristan Harris to primary, "63 break-glass" count, the ~35-study internal corpus. RFC 9420 §16.4
reconciliation: (a) group ID and (b) epoch verified with **narrowed scope** (cleartext `PrivateMessage`
header, threat is the DS not a generic observer), (d) membership verified as written, **(c) corrected** —
`generation` is inside AEAD-encrypted `SenderData`, not visible in framing; the "gap reveals a missed
message" claim was unsourced in the RFC and removed. Drift grep clean; iroh cited at `1.0.0` per FACTCHECK
SoT. Spec bodies went through the em-dash discipline already; the companion/activism bodies retain their
em-dashes (finished companion docs, per the batch-three convention). *(Superseded 2026-07-06: em-dashes
subsequently normalized across the companion/activism docs to match the spec convention — see the
dropoff-cleanup note below.)*

## 2026-07-06 — dropoff cleanup + em-dash normalization

The `beta/drystone-spec/dropoff/` scratch area was removed once every batch was filed. Before deletion,
the **pre-edit raw inputs** of the Drystone spec (dropoff batches 1 and 2, which differ from the edited
tree) were frozen to `seeds/drystone-spec-inputs/` (`batch1-files.zip/`, `batch2-second.zip/`) as
**preserved-verbatim**; see that folder's `README.md`. Batches three and four had been filed
byte-identical, so their committed tree copies are the record and their zips were removed without a
separate seed. The superseded `drystone-part1-voice-bridge.md` (standalone draft of the merged Part 1
§2.6) was intentionally not preserved.

Em-dashes were then normalized across the companion/activism docs to match the spec's em-dash discipline
(role-based replacement: bullet-label/heading colons, appositive commas, clause-join semicolons,
parenthetical parens). Applied to `governance/structural-argument-principles.md` (30) and the three
`activism/` docs (`structural-argument-narrative.md` 29, `relational-field-research-brief.md` 57,
`reference-index.md` 46), plus the layer `README.md` files; the peer-standing, session-summary,
tilling-the-soil, and pitch docs had already arrived em-dash-free in batch four.

## 2026-07-06 intake — batch five: persona/peer vocabulary migration (five-persona.zip) + layer restructure

**`five-persona.zip`** is the persona/peer vocabulary migration (Drystone spec document-pass-4): *peer* is
sharpened to name only the relation; **persona** (plural **personae**, Latin form strict) names the entity
a human is manifested as (a key pair by which a system represents a person, one human may hold many across
many systems). Full spec snapshot + a new vocabulary-of-record companion + tracking artifacts. Overwrites
the spec per the clean-history workflow.

| Raw artifact | Where | Status |
|---|---|---|
| **persona migration set (9 files)** — `drystone-part1/part2.md`, `persona-definition.md`, `drystone-persona-{session-summary,delta,migration}.md`, `drystone-part1/part2.diff`, `peer-inventory-worksheet.txt` | delivered via `five-persona.zip` (discovery root scratch); spec + companion filed to `beta/drystone-spec/`, process artifacts frozen to `seeds/drystone-persona-migration/` (below) | **preserved-verbatim** (byte-identical, `diff -q` confirmed) |
| **originating conversation (persona naming + personae + voice congruence)** | `seeds/transcripts/raw/drystone-persona-vocabulary-migration-2026-07-06.md` | **preserved-condensed (cleaned-paste, content-faithful — §4)** |

**Outputs filed (batch five):**
- `beta/drystone-spec/part-1-reasoning-underpinnings.md` (875→890: persona definition note in §1), `part-2-certifiable-design.md` (2099→2235: Appendix D term lattice + §4.5/§5.2/§5.5 identity model) — overwritten; `CHANGELOG.md` (document-pass-4 prepended)
- `beta/drystone-spec/persona-definition.md` — new vocabulary-of-record companion
- `seeds/drystone-persona-migration/` (+README) — the 3 tracking docs, 2 diffs, worksheet frozen (process, not spec)

**Layer restructure (same session, from the user's layering refinements):**
- **New ordering** (why-first): 1 `history/` · 2 `philosophy/` · 3 `drystone-spec/` · 4 `impl/` · 5 `croft/` · 6 `governance/` · 7 `socialization/` · 8 `activism/`. (Supersedes the earlier "spec=Layer 1" numbering used in the third/fourth-batch entries above; those entries are left as dated records.)
- **Two histories** split: `history/` (Layer 1) = material (crofting, dry-stone, cairns); `philosophy/` (Layer 2) = intellectual (principles + thinkers).
- **`philosophy/` created** and the peer-standing argument set (`peer-standing-and-the-cooperative-form.md`, `structural-argument-principles.md`, `peer-standing-session-summary.md`) **moved from `governance/` → `philosophy/`** (git mv). `governance/` (Layer 6) rewritten as the *manifestation* layer (foundation + cooperative), now reserved (README only).
- **`beta/LAYERS.md`** created: canonical layer model, two traversals (build / justification), register discipline, the two-histories and philosophy-vs-manifestation splits.
- Cross-references repointed (spec §2.6 companion, `activism/README`, `socialization/README`, beta README table, OPEN-THREADS T33) from `governance/` → `philosophy/`.
- **OPEN-THREADS T35** added: the uncompensated-community-labor + data-opacity activism indictment (distinct from the relational-field harm case).

**Honest provenance note (§4).** Deliverables preserved verbatim; batch five arrived em-dash-clean. The
vocabulary migration did **not** touch the spec's `[confirm before publish]` external-source flags (Lamport,
Ostrom, Matrix CVEs, MLS, Spritely). Intentional near-duplication flagged by the source session: the term
oracle lives in both `persona-definition.md` and Part 2 Appendix D, both carry a "§5 governs" note; retiring
the standalone would remove it (deferred, the user's call). The peer-standing docs now in `philosophy/` still
use *peer* in the relational sense throughout, a reconciliation pass against the new persona vocabulary is a
worthwhile later check (flagged in `philosophy/README.md`), not done this session. `five-persona.zip` left in
the discovery root as scratch (filed byte-identical; can be removed like prior batches on request).

## 2026-07-06 intake — batch six: transport/identity integration + iroh 1.0 + §16.4 correction (six-transport.zip + six-integrating.zip)

**`six-integrating.zip`** is the final integrated Drystone spec (built on top of the batch-five persona
work, no regression): Part 2 §6 expanded from a 3-subsection stub to a full transport/identity/encryption
section (§6.1–6.8, +516 lines), iroh flags resolved against the released 1.0, and the RFC 9420 §16.4
metadata analysis corrected. **`six-transport.zip`** is the intermediate (the standalone transport-section
draft + the two figures), superseded by the merged §6. Part 1 is **byte-identical** to the tree (its only
iroh reference is version-agnostic); not overwritten.

| Raw artifact | Where | Status |
|---|---|---|
| **integrated spec + figures + artifacts (7 files)** — `drystone-part1/part2.md`, `drystone-integration.diff`, `drystone-integration-summary.md`, `drystone-exposure.svg`, `drystone-catchup-flow.svg`, `drystone-messaging-layer-research-prompt.md` | `six-integrating.zip`; part2 + figures → `beta/drystone-spec/`, diff+summary → `seeds/drystone-transport-integration/`, prompt → `seeds/generated-prompts/` (below) | **preserved-verbatim** (byte-identical, `diff -q` confirmed) |
| **transport-section intermediate (3 files)** — `drystone-transport-section.md` + the two SVGs | `six-transport.zip` (superseded; the draft not filed, figures identical to the integrated ones) | superseded; not separately preserved (figures carried via the integrated set) |
| **originating conversation (RFC §16.4 verify + iroh 1.0 + messaging prompt + DDD)** | `seeds/transcripts/raw/drystone-transport-integration-and-ddd-2026-07-06.md` | **preserved-condensed (cleaned-paste, content-faithful — §4)** |

**Outputs filed (batch six):**
- `beta/drystone-spec/part-2-certifiable-design.md` (2235→2751: §6 expanded) — overwritten; `part-1` untouched (identical)
- `beta/drystone-spec/drystone-exposure.svg`, `drystone-catchup-flow.svg` — figures added
- `beta/drystone-spec/bounded-contexts-and-vocabulary.md` — new spec-layer DDD/vocabulary note (per the user: DDD is spec-layer design/language input, not philosophy)
- `beta/drystone-spec/CHANGELOG.md` — document-pass-5 prepended
- `beta/drystone-spec/README.md` — file list updated (persona-definition, vocabulary note, figures) **and terminology table reconciled to the persona model** (peer = relation; persona/principal added; PeerSet → PrincipalSet; meer = infrastructure, not a principal) — a consistency gap that had been open since document-pass-4
- `seeds/drystone-transport-integration/` (+README) — integration diff + summary (process, frozen)
- `seeds/generated-prompts/drystone-messaging-layer-research-prompt.md` — forward handoff (MLS-over-iroh delivery models, the DS/meer/push-host question)

**Honest provenance note (§4).** Deliverables preserved verbatim; batch six arrived em-dash-clean (part2,
the research prompt). **Correctness result:** the RFC 9420 §16.4 generation-counter claim was **wrong and
removed** (`generation` is inside AEAD-encrypted `SenderData`); a false MLS statement caught before it
shipped. iroh 1.0 treated as the FACTCHECK SoT (`1.0.0`); the 1.0-vs-gossip-crate split is the organizing
fact; still `[confirm before publish]`: pin the three iroh subcrate versions, pull the Pkarr spec / RFC
8446 §5.4 / RFC 9420 §16.9, and lift the canonical §16.4 running-header line from rfc-editor.org.
**Remaining consistency debt, flagged not fixed:** (1) `review-handoff.md` predates the persona migration
(doc-pass-4) entirely (0 "persona", still "PeerSet") and needs a full reconciliation pass, not a one-word
patch; (2) a full peer→persona sweep of the theme/companion bodies that still use "peer" as the entity
noun; (3) optional em-dash tidy of the pre-existing docs never in the normalization scope (spec README,
CHANGELOG, beta README). All three zips (`five-persona`, `six-transport`, `six-integrating`) removed after
byte-identical filing, per the user's "clean up all the zips when done."

## 2026-07-06 intake — batch seven: the delivery-layer design corpus (seven-grounding.zip)

**`seven-grounding.zip`** is a 13-doc, self-numbered corpus (00–12): the design of Drystone's messaging /
delivery layer, the follow-on from the batch-six research prompt. It seeds a **new Layer 4 `impl/`**
(reference implementation, experiment-informed) as `impl/delivery-layer/`. This is design maturity, not
spec (`01` is explicitly "for folding into Part 2" once it holds). The user noted we are at **batch 7 of
11** with more changes coming, and deferred the two peer→persona / em-dash consistency sweeps ("not yet"),
so no spec/philosophy doc was modified to absorb this round's concepts.

| Raw artifact | Where | Status |
|---|---|---|
| **delivery-layer corpus (13 docs, 00–12)** — session summary, architecture, references, two pitches, three experiment plans, Delta Chat analysis, history modes, methodology, provenance, doc-method | `seven-grounding.zip`; filed to `beta/impl/delivery-layer/` | **preserved-verbatim** (13/13 `diff -q`; em-dash-clean as delivered) |
| **originating conversation (atomic-swap re-plant, CALM grounding, center-free framing)** | `seeds/transcripts/raw/drystone-delivery-layer-design-2026-07-06.md` | **preserved-condensed (cleaned-paste, content-faithful — §4)** |

**Outputs filed (batch seven):**
- `beta/impl/` — **new Layer 4** (+ `README.md` layer index) — `impl/delivery-layer/` holds the 13-doc corpus
- `beta/README.md` — layer-cake table: `impl/` marked seeded
- raw transcript preserving the reasoning threads (see below)

**Grounding results captured (from the corpus + transcript):** the **atomic-swap re-plant** (governance
chain is the membership authority; a fresh MLS tree is stamped over the current member set at boundary N
and the old tree cut down; nothing downstream reads the tree, so tree-byte nondeterminism is a dedup not a
fork). MLS mechanics grounded against primaries (unilateral O(N) creation, KeyPackage-per-member,
single-use with last-resort escape hatch, fresh stamp = group-wide leaf-key refresh = favorable PCS). The
**CALM theorem** verified against primaries (Hellerstein & Alvaro, *Keeping CALM*, arXiv:1901.01930 / CACM
2020; formal-proof lineage Ameloot, Neven & Van den Bussche 2013), with the attribution nuance (conjecture
vs statement vs formal proof) recorded; monotonicity is information-growth, not time. The **center-free =
plurality** framing ("raindrops on a lake, each the center of its own ripple, none the center of them all")
and the two-layer fence (**peers converge on governance facts; each acts on its own local state**) are
preserved in the raw transcript as durable concepts, not yet folded into the spec or philosophy (more
changes expected). The standalone `calm-session-summary.md` from the source session was **not** in the zip
(the CALM grounding lives in the corpus's `02-references.md`); can be filed if provided.

**Honest provenance note (§4).** Deliverables verbatim, drift-clean. The `iroh =1.0.0-rc.1` mention in
`00` is a correct note about **iroh-gossip's dependency pin** (integration-residue flag), not a claim about
iroh core (which is 1.0 final per the FACTCHECK SoT). Corpus residue flagged by the source session (not
resolved here): pin the iroh subcrate versions; confirm mls-rs ReInit/resumption-PSK exposure; the
KeyPackage-availability cost that tunes boundary N. `seven-grounding.zip` removed after byte-identical
filing, per the standing "clean up the zips" instruction.

## 2026-07-06 intake — batch eight: MLS-substrate bundle + Modular Politics prior-art (eight-mls.zip + eight-modular.zip)

Two deliverable sets. **`eight-mls.zip`** is the MLS-substrate understanding: overview/terms (RFC-anchored
vocabulary resolving the leaf/client/device conflation), hardcases-and-posture (nine hazards + concept-
alignment map + posture table), the threading candidate note, an updated shared writing method, and a
session summary. **`eight-modular.zip`** is a source-grounded analysis of *Modular Politics* (the
nearest-neighbor academic governance-as-protocol frame) plus its session summary. The user's two routing
decisions: promote **one canonical writing-method doc** to the `impl/` layer root, and file **Modular
Politics under `philosophy/prior-art/`**.

| Raw artifact | Where | Status |
|---|---|---|
| **MLS-substrate bundle (5 docs)** — `mls-overview-and-terms`, `mls-hardcases-and-posture`, `12-side-histories-and-threading`, `11-doc-method`, `session-summary` | `eight-mls.zip`; filed to `beta/impl/mls/` (threading renamed `side-histories-and-threading.md`, summary → `mls-session-summary.md`); the `11-doc-method` promoted to `beta/impl/doc-writing-method.md` | **preserved-verbatim** (byte-identical, `diff -q`; em-dash-clean) |
| **Modular Politics analysis (2 docs)** — `modular-politics-analysis`, `session-summary` | `eight-modular.zip`; filed to `beta/philosophy/prior-art/` (summary → `modular-politics-session-summary.md`, 2 em-dashes normalized) | **preserved-verbatim** (analysis byte-identical; summary em-dash-normalized) |
| **originating conversation (MLS terminology + grounding, Blacksky, Modular Politics, MLS journey/ecosystem)** | `seeds/transcripts/raw/mls-blacksky-modular-prior-art-2026-07-06.md` | **preserved-condensed (cleaned-paste, content-faithful — §4)** |

**Outputs filed (batch eight):**
- `beta/impl/mls/` (+ documented in `impl/README`) — the MLS-substrate bundle (4 docs)
- `beta/impl/doc-writing-method.md` — **single canonical** writing method (149-line, +§10 posture-table); the divergent `delivery-layer/11-doc-method.md` (137-line) was `git rm`'d and the corpus's "doc 11" references now resolve here
- `beta/philosophy/prior-art/` (+ `README`) — the Modular Politics analysis + summary; `philosophy/README` gains a prior-art pointer
- `seeds/generated-prompts/grounded-research-and-explanation-prompt.md` — the reusable search-first + quote-discipline + no-orphaned-concepts prompt (verbatim)
- `beta/README.md` — impl row updated

**Not distilled into docs (preserved in the raw transcript only):** the substantial **Blacksky** research
(People's-Assembly/Polis governance; thin-AppView-fork + Rust performance path; Community Posts as an
AppView-resident private-data lexicon inverting PDS-as-source-of-truth; 0→2M-users / full-network-AppView
scale; the participatory-governance-vs-corporate-form gap) and the **MLS journey/ecosystem** research (MIMI
as the interop half; the formal-proof record and its found limits — SUF-CMA/Ed25519 requirement, external-ops
lowering PCS to session-state; the TreeSync/TreeKEM/TreeDEM decomposition; adoption). Both are flagged in
`philosophy/prior-art/README` as candidates for a future `ECOSYSTEM.md` register update.

**Honest provenance note (§4).** Deliverables verbatim (one em-dash normalization in the modular summary).
The MLS docs' RFC claims are section-accurate but not all read verbatim (one `[confirm]` on the §8.2 hashing
formula remains, per the source session). MLS-ecosystem facts flagged at source: Discord "DAVE" from a
reference list not Discord's own blog; PQ-for-MLS from SEO-grade pages (unverified). Batch **8 of 11**;
peer→persona and em-dash sweeps still deferred. `eight-mls.zip` + `eight-modular.zip` removed after
byte-identical filing.

## 2026-07-06 intake — batch nine: the consolidation (p9), document-pass-6 (nine-consolidate.zip)

The definitive synthesis: a self-contained, consistent Part 1 + Part 2 with the transport/delivery and
deep-MLS designs folded in, delivered as a `p9-` prefixed set. **This supersedes the committed spec**
(overwrites part-1/part-2). The user flagged that the web agent had been **losing/missing content** across
iterations, so a content audit of the consolidated docs against all previously-incorporated material
follows this filing (to confirm nothing was dropped).

| Raw artifact | Where | Status |
|---|---|---|
| **consolidated spec set (7 docs)** — `p9-drystone-part1-principles`, `p9-drystone-part2-mechanics`, `p9-drystone-conventions-and-decisions`, `p9-drystone-doc-method`, `p9-drystone-part1-changelog`, `p9-drystone-part2-changelog`, `p9-drystone-session-summary` | `nine-consolidate.zip`; filed to `beta/drystone-spec/` (canonical names, not p9-prefixed) + `impl/doc-writing-method.md` (updated) + `seeds/drystone-consolidation/` (session summary) | **preserved-verbatim** (byte-identical, `diff -q`; em-dash-clean, drift-clean) |
| **originating conversation (consolidation reasoning, apex, proof-of-personhood, licensing)** | `seeds/transcripts/raw/drystone-consolidation-and-personhood-2026-07-06.md` | **preserved-condensed (cleaned-paste, content-faithful — §4)** |

**Outputs filed (batch nine, document-pass-6):**
- `beta/drystone-spec/part-1-reasoning-underpinnings.md` (890→975) + `part-2-certifiable-design.md` (2751→3961) — **overwritten** with the consolidation
- `beta/drystone-spec/conventions-and-decisions.md`, `part-1-changelog.md`, `part-2-changelog.md` — new
- `beta/drystone-spec/CHANGELOG.md` (document-pass-6 entry) + `README.md` (status → consolidated/pending-design-review; file list; terminology `PrincipalSet`→`Group Role Set`; superseded companions flagged)
- `beta/impl/doc-writing-method.md` — updated to the newest (p9, 279-line) canonical
- `seeds/drystone-consolidation/` (+README), raw transcript

**On the p9 naming:** filed under **canonical filenames**, not `p9-` prefixed. The p9 marker was the
consolidation session's way of tagging "current, pending-design-review"; in the repo, git + the CHANGELOG
carry that provenance and the README status line records "consolidated (p9), pending design review." (Flag
for the user: if they want the `p9-` prefix preserved on the tree filenames, easy to rename.)

**Deferred reconciliation (NOT executed — user deferred the sweeps; kept as a flagged temporary
inconsistency):** the consolidation subsumes several standalone companions, which are **retained pending
go-ahead to retire**: `persona-definition.md` (→ Part 2 §5.2 + Appendix D; still uses the retired
`PrincipalSet`), `open-items.md` (→ Appendix B), `bounded-contexts-and-vocabulary.md` (→
`conventions-and-decisions.md`), `review-handoff.md` (predates the persona migration). The two SVGs are
retained though Part 2 no longer references them by figure number. The `impl/doc-writing-method.md`
duplication (batch-8 decision) was resolved by updating it to the newest version.

**Honest provenance note (§4).** Deliverables verbatim, em-dash/drift-clean. The consolidation's own
verification (per the per-part changelogs) corrected two facts: RoQ is a draft not RFC 9714; Sigstore is
signature-transparency not countersigning. **Content audit pending** (this filing's follow-on): confirm the
p9 consolidation did not drop anything landed in document-passes 1–5 or the companions, given the web
agent's known content-loss. Not distilled into docs: the proof-of-personhood survey research (Shilina;
Siddarth et al. "Who Watches the Watchmen"; the decentralized-identity trilemma; web-of-trust/PGP lineage;
Idena) and the protocol-licensing analysis (CC BY + AGPL + trademark/conformance) — both in the raw
transcript, candidates for `philosophy/prior-art` and the governance layer. Batch **9 of 11**.

## 2026-07-07 — post-consolidation content audit + re-plant mechanism fold (document-pass-7)

**Content audit (durable record, per the user's request after batch 9).** The user flagged the web agent
had lost/missing content across iterations, so the p9 consolidation (document-pass-6) was audited against
the pre-consolidation spec (git `22982ea`, document-pass-5) and every companion, at both heading and
content level. **Result: nothing was dropped.** Every heading the diff flagged "missing" is a
renumber/retitle, confirmed present by content-search: §6 transport was *expanded* into the three-plane
model (Carriage/Durability/Presence + discovery §6.9, gossip §6.10, deployment §6.11); real-time media →
§6.12; honesty boundaries → §8.2; Group-as-principal → §5.10; the escalation two-member shape (contradiction
+ under-determination) → §7.6.1. Subsumed companions genuinely absorbed: `persona-definition.md` → §5.2 +
Appendix D (incl. "D.7 Invariants of record", 63 concept hits, *more* comprehensive than the standalone);
`open-items.md` → Appendix B (35 structured lines vs 20). The `impl/` design corpus (re-plant experiments,
MLS hard-cases) is intact in place. The single "not yet folded" item was the detailed re-plant mechanism
(closed by the fold below).

**Re-plant mechanism fold (document-pass-7).** On the user's instruction ("fold it now with needs-verification
tag; track it in open threads but meaningfully in context for the protocol"), the detailed MLS
re-plant/atomic-swap mechanism was folded from `impl/delivery-layer/12-replant-experiments.md` +
`01-delivery-architecture.md` and `impl/mls/mls-hardcases-and-posture.md` into **Drystone spec Part 2
§7.6.4** (new subsection), carrying `[confirm before publish] / needs verification` throughout: O(N)
instantiation, last-resort KeyPackage availability floor, group-wide leaf-key refresh (PCS) with the
last-resort exception, blank-node cost reset, planter-byte-nondeterminism-is-dedup, stale-`GroupInfo`
external-commit PCS integrity, and the `epoch_authenticator` fold-not-parallel candidate. Validation path
named (the E12.1–E12.7 experiment set on `mls-rs 0.55.2`; Rung A MLS mechanics, Rung B Drystone's own
structures). Tracked as **OPEN-THREADS T36** (status `open · gated`, needs the E12 run + the Appendix B
re-plant items). CHANGELOG document-pass-7 records both the audit and the fold. The `impl/` corpus is
retained as the derivation and experiment plan (not deleted). Part 2 stays em-dash-clean; the fold is
grounded per-claim against named RFC sections. **Committed and pushed** to `origin/main`
(`CroftCommunity/discovery`).

## 2026-07-07 intake — batch ten: the cairn layer + p10/p11 corpus snapshot (ten-feasibility.zip + ten-willlow.zip)

**New Layer 3 `cairn/`** (the user's proposal): the field of existing bolstering tech Drystone builds on,
the inverse of `activism/`. Where activism indicts the extractive incumbents, cairn credits and catalogues
the enabling tech (iroh, MLS, Willow/Meadowcap, CBOR-DAG, atproto/AT, ActivityPub, CRDT, QUIC; products
Roomy, Blacksky, p2panda, SimpleX, Matrix). It sits between philosophy and the spec because the spec had to
survey the field before it could assemble novelty practically. **Renumber:** cairn=3 inserted; drystone-spec
4, impl 5, croft 6, governance 7, socialization 8, activism 9 (history 1, philosophy 2 unchanged).

**`ten-willlow.zip` is a 30-file full-corpus snapshot**, not a clean incremental batch: it spans three
maturity iterations (p9 → p10 → p11), including the **newest Part 2 rebuild (`p11-full-part2-mechanics`,
470K)**, the current Part 1 (`p10-full-part1`), the ten p10 companions, the shorter tellings, the experiment
corpus, and superseded p9 leftovers. `ten-feasibility.zip` is the two feasibility reviews (v2 current).

| Raw artifact | Where | Status |
|---|---|---|
| **ecosystem material** — `p10-drystone-atproto-ecosystem.md`, `social-lexicon-group-research-brief.md` | filed to `beta/cairn/` | **preserved-verbatim** (byte-identical; em-dash-clean) |
| **feasibility review v2** — `drystone-feasibility-review-v2.md` | filed to `beta/drystone-spec/feasibility-review-v2.md` | preserved-verbatim (carries 13 em-dashes; deferred sweep) |
| **full p10/p11 corpus (32 files)** | frozen at `alpha/seeds/p10-p11-corpus/` (+README) | **preserved-verbatim** (both zip payloads byte-identical) |

**Outputs filed (batch ten):**
- `beta/cairn/` — **new Layer 3** (+ `README`): `atproto-ecosystem.md`, `social-lexicon-group-research-brief.md`
- `beta/drystone-spec/feasibility-review-v2.md` — the second-pass, spec-grounded feasibility review
- `beta/LAYERS.md`, `beta/README.md`, and the layer READMEs (`impl`, `governance`, `socialization`, `activism`, `philosophy`) — **renumbered** for the cairn insertion; LAYERS.md gains the cairn row, the field-survey rationale, and the cairn-is-inverse-of-activism framing
- `alpha/seeds/p10-p11-corpus/` (+README) — the full snapshot frozen

**Deferred, flagged for the user (NOT executed this batch):** the **spec swap** — making
`p10-full-part1` + `p11-full-part2` the canonical `beta/drystone-spec/` (superseding the batch-9 p9
consolidation). Deferred because p11 was "authoritative *on confirmation*" (never confirmed in the source
session), because the user's own batch-9 lesson was to audit a big consolidation for web-agent content-loss
before trusting it, and because the p10 companions + experiments then need routing into `impl/`. The full
corpus is frozen so nothing is lost; the swap is the next spec pass once the user confirms. **cairn
migration backlog** also recorded (MLS scaling survey, Willow/Meadowcap analysis, Blacksky research, the
local-authority ecosystem landscape, Roomy/p2panda tracking — all in raw transcripts, to migrate into
cairn). Batch **10 of 11**. Committed and pushed.

**Spec swap executed (document-pass-8, 2026-07-07, on the user's go).** After the content-loss audit: Part 1
← `p10-full-part1-principles` (clean heading-superset of the committed p9 part-1), Part 2 ←
`p11-full-part2-mechanics` (the richer rebuild, 470 KB, complete §0–§10 + Appendices, §7.6 expanded to ten
subsections). **The audit caught a real divergence:** p11 (parallel web lineage) did **not** carry the
tree-side document-pass-7 §7.6.4 re-plant fold (E12 0 vs 13). The block was extracted before overwrite and
**re-folded as §7.6.11**, so nothing was lost (E12 back to 13; both parts em-dash- and drift-clean). This is
exactly the web-agent content-loss the user warned about, caught by the audit gate. Companions routed:
ten p10 design companions → `beta/impl/drystone-design/`; four experiment docs → `beta/impl/experiments/`;
coffee-shop + elevator-pitch → `beta/socialization/`; `conventions-and-decisions.md` and
`impl/doc-writing-method.md` updated to the newer p10 versions; the p11 spec open-threads →
`beta/drystone-spec/open-threads.md`. Superseded standalones (persona-definition, open-items,
bounded-contexts, review-handoff, SVGs) still retained pending retirement (p11 subsumes them via Appendix
D / B). Full corpus stays frozen at `seeds/p10-p11-corpus/`. Committed and pushed.

**Superseded-standalone handling + cairn migration (2026-07-07, on the user's direction).** (1) The
superseded spec standalones (`persona-definition`, `open-items`, `bounded-contexts-and-vocabulary`,
`review-handoff`, the two SVGs) were **moved to `beta/drystone-spec/superseded/`** (+README) and marked as
provenance, treated as raw transcripts would be, kept not retired. (2) Items 2 (peer→persona reconciliation)
and 3 (em-dash tidy of pre-existing docs) deferred to the end, current-versions-only, per the user. (4) The
**cairn migration backlog was distilled into four survey docs** in `beta/cairn/`: `mls-and-mimi.md` (MLS +
MIMI + the scaling survey + the TreeSync/TreeKEM/TreeDEM decomposition), `willow-meadowcap.md` (the
state-based-CRDT mental model + the timestamp wrinkle + Meadowcap), `blacksky-and-atproto-community.md`
(Blacksky governance/AppView/transferable ideas), and `adjacent-systems.md` (Roomy, p2panda, SimpleX, Briar,
Cwtch, Matrix, Session, Nostr on the two axes). Drafted by four parallel opus subagents grounded strictly in
two raw transcripts: the batch-8 `mls-blacksky-modular-prior-art` and the newly-filed batch-10
`mls-scaling-willow-ecosystem-and-cairn-2026-07-07.md` (which preserves the MLS-scaling, Willow, adjacent-
systems, ATProto-lexicon, feasibility, and cairn-proposal threads that had not yet been captured). All four
em-dash-clean. cairn README + LAYERS.md updated (backlog → filed). Modular Politics stays in
`philosophy/prior-art/`. Honest note on scope: of the 30-file `ten-willlow.zip`, ~2 were swapped live, ~18
routed into layers, and the superseded/duplicate versions live only in the frozen seed, not made live.
Committed and pushed.

**Batch 11 of 11 (large-group scaling), 2026-07-07.** `eleven.zip` (parent-dir drop) carried two
deliverables, filed byte-identical (`diff -q` clean) with no existing spec text altered (purely additive):
(1) `drystone-large-group-scaling.md` -> `beta/drystone-spec/large-group-scaling.md`, a section-length
Drystone Part 2 large-group-scaling treatment (internal §7.1-§7.14: cost-on-live-set, scaled PCS, the
hot/cold split, the two-part re-entry credential, the single governance chain with lineage-scoped bans,
delivery-as-a-race, the §7.9.1 encryption posture and two forces, the experimental §7.9.3 public-by-default
regime, tiered performance with the §7.10.1 experiment matrix, §7.13 empirical basis, §7.14 research
obligation); and (2) `research-prompt-operational-rates.md` -> `beta/drystone-spec/`, the companion research
prompt §7.14 points to. Both em-dash- and drift-clean. **Audit finding (not a swap):** the section is
titled "Part 2 §7" and numbers itself §7.1-§7.14, but it depends on and cites the *existing* Part 2 §7
(Synchronization); both cannot be §7, so it was filed as a standalone section companion pending a
placement/renumber decision (open thread T37), nothing renumbered unilaterally. **New layer `fenced/`
created (on the user's call).** The batch surfaced a category the corpus had no home for: the centered
commercial platforms (Telegram, Discord, WhatsApp, Signal, Slack, Teams, Reddit, X, iMessage, Messenger,
LINE, WeChat), which are not a cairn building block and not the activism argument. `beta/fenced/` is the
descriptive map of that fenced field (extent, shape, capability, economics), the fenced-half counterpart to
cairn's open field and the substrate activism reads its harm case from; named "fenced" not "enclosure"
because these platforms were never an open commons that got enclosed, they were built fenced. Seeded with
two survey docs distilled by two parallel opus subagents grounded strictly in the batch-11 raw transcript:
`group-scale-versus-e2ee.md` (the 14-platform capability map + Force 1/Force 2) and
`operational-rates-and-platform-economics.md` (the three per-group rates + the Telegram economics). Both
em-dash-clean, no alpha refs, no drift. **The originating conversation** (the research report the §7.14
prompt produced, plus a Telegram broadcast-vs-discussion / monetization / Premium-features field-notes tail)
is not in the zip; it is preserved content-faithful (§4 caveat) as
`raw/drystone-large-group-scaling-e2ee-operational-rates-and-telegram-2026-07-07.md`, which is the
provenance the two fenced surveys were distilled from and the full-detail source behind spec §7.13/§7.9.1.
As-received batch frozen at `../large-group-scaling-batch11/eleven.zip`. Coherence updated: `fenced/README`,
`LAYERS.md` (field now two-halved, the triad), `beta/README.md`, `drystone-spec/README.md` +
`CHANGELOG.md` (document-pass-9), `cairn/README.md` (vs-fenced boundary), OPEN-THREADS T37-T40. Committed
and pushed.

**Batch 11 follow-up (2026-07-07): §7 -> §11 fold.** On the user's decision, the standalone
`large-group-scaling.md` was folded into `beta/drystone-spec/part-2-certifiable-design.md` as **§11**
(after §10, before Appendix A), resolving the numbering collision with the existing Part 2 §7
(Synchronization). Self-refs renumbered §7.N -> §11.N; external real-§7 citations preserved; headings
nested one level under `## 11.`; the §0 Map given a §11 line; the standalone live file removed (the
byte-identical original stays frozen in `../large-group-scaling-batch11/eleven.zip`). Spec document-pass-10;
downstream refs updated to §11 across README/LAYERS/beta-README/fenced-README/OPEN-THREADS; open thread T37
resolved and moved to Promoted & closed. Its own commit.

**Multi-layer intake (2026-07-07): "the rest of yesterday's material".** A batch of web-session
conversations (Gemini "deep-discussion" + technical Q&A) was extracted to discovery, skipping the arecipe
project (already filed under `CroftC/arecipe`). Eleven grouped raw transcripts were preserved
content-faithful (§4 caveat) in `raw/`: `philosophy-lifeworld-system-and-the-public-sphere-`,
`philosophy-commensurability-ledger-and-legibility-`, `epistemics-rosenhan-replication-crisis-async-comms-`,
`drystone-transport-iroh-gossip-quic-`, `rfc9420-metadata-floor-verbatim-`, `dag-cbor-and-merkle-primer-`,
`doc-tooling-rfc-diagrams-ascii-svg-mermaid-`, `kleppmann-designing-data-intensive-applications-notes-`,
`governance-improvement-paradox-and-preventative-work-`, `fenced-facebook-group-size-data-`, and
`drystone-logo-drystone-stacking-` (all `-2026-07-07.md`). Distilled by parallel opus subagents into layer
docs (all em-dash- and drift-clean, no alpha refs; AI-sourced quotes tagged [UNVERIFIED] confirm-vs-primary):
**philosophy/** `lifeworld-and-the-system.md`, `commensurability-and-the-two-ledgers.md`,
`epistemics-provenance-and-verification.md`; **governance/** `making-preventative-work-visible.md` (the
layer's first content doc); **socialization/** `logo-drystone-stacking.md` (brief only, no rendered asset);
**impl/** `transport-iroh-gossip-and-quic.md`, `references-designing-data-intensive-applications.md`, plus a
§17 Diagrams section in `doc-writing-method.md`; **drystone-spec/** `dag-cbor-and-content-addressing.md`
(companion primer). Augmented **fenced/** `operational-rates-and-platform-economics.md` with a Facebook
group-size data point. Two technical items (the RFC 9420 §16.4 metadata-floor verbatim, and the
iroh-gossip/QUIC mechanism) were found to CORROBORATE already-`Verified-RFC`/`Verified` spec text (Part 2
§6.3/§6.4 line-level and §6.10, §11.9), so they landed as raw provenance + an impl reference rather than a
spec edit; the RFC 9420 §16.4 verbatim confirms the spec correctly excludes the unsupported
"sender-generation-gap reveals a missed message" claim. Routing overrides on the user's call: Kleppmann ->
impl, dag-cbor -> spec companion, Rosenhan -> its own epistemics note, logo -> its own socialization stub.
Coherence: philosophy/impl/governance/socialization READMEs + LAYERS.md updated. Committed and pushed.

**atproto ecosystem intake (2026-07-08): Pixelfed backfill, Smoke Signal, NSID.** A web research
session grounding several atproto mechanics was preserved content-faithful (§4 caveat) as
`raw/atproto-pixelfed-backfill-smokesignal-nsid-2026-07-08.md`. Note: the session opened asking about
stream.place but the transcript excerpt jumps straight to the Pixelfed-backfill answer without ever
returning to it, so nothing about that site is captured. Distilled by two parallel opus subagents,
grounded strictly in the raw transcript, into cairn (Layer 3, the open field), both em-dash- and
drift-clean: `atproto-nsid-and-lexicon-mechanics.md` (NSID structure, the reverse-DNS authority binding,
the h3/hthree naming-rule tension left unresolved, the fetchability gap; Smoke Signal as the worked
example, incl. its founding motivation, the events.smokesignal.* to community.lexicon.* storage-path
migration, and its Rust/Postgres/Redis/AIP stack, preserving an in-session SQLite-to-Postgres
self-correction) and `atproto-content-portability-and-backdating.md` (the atproto write path for content
backfill, backdated createdAt, the Pixelfed export-format gotcha as a migration case study, the
tooling-gap survey (mastodon-to-bluesky/Bounce/Bridgy Fed), a preserved self-correction-discipline example
where the session retracted an unsourced claim about a tool author's motive, and the
@backdate.mozzius.dev labeler as a "detectable, not blocked" moderation-primitive example). Cross-references
added: `atproto-ecosystem.md`'s one-line Smoke Signal table entry now points to the new deep-dive doc;
`social-lexicon-group-research-brief.md`'s open research item A4 (lexicon/NSID mechanics) marked
partially answered with a pointer, firehose-indexing/coexistence questions left open. cairn/README
updated. Committed and pushed.

## 2026-07-10 intake — Croft.ing website / plot-tender design dialogue (pasted)

| Raw artifact | Where | Status |
|---|---|---|
| Croft.ing website / plot-tender design dialogue (pasted 2026-07-10) | `seeds/transcripts/raw/croft-website-plot-tender-design-dialogue-2026-07-10.md` | **preserved-condensed (cleaned-paste, content-faithful — §4)**; several assistant turns were **image generations** — the images are not in the paste, so each is preserved as a **bracketed pointer carrying the generation prompt verbatim** (the durable design record), and the final image request was declined in-session. No secrets/creds present. |

The **product/website realization of the already-filed "atmospheric web" thread**
(`atproto-atmospheric-web-iroh-mobile-dialogue.md` + its FACTCHECK; distilled to
`thinking/atproto-atmospheric-web.md`; ROADMAP_TODO E1/E2/E3): it takes "GeoCities-2027-on-atproto" and
makes it **Croft.ing** specifically — the Plot·Shed·Gate spatial model, the serverless single-renderer
"safety deposit box" (`plot.croft.ing?user=` handle *or* DID, resolving to the same PDS-backed canvas as
`name.croft.ing`), the social engine mapped onto standard atproto records (footprints = likes, guestbook =
replies on an Anchor Post; media = PDS blobs via CDN; themes = CSS-as-JSON), the plot-tender delight catalog
(Lantern Vigil, Stile, Stone Circle, Specimen Book, Greenhouse/Seeds, interactive homestead map), and the
Bluesky integration (Gate Feed custom feed, subdomain signpost, starter pack). Plus the tectonic visual
identity (granite/ruddy/moss palette with working hex), the progressive-depth website IA
(elevator-pitch → one-pager → library), and the **CROFT.ing wall-to-gate logo** direction.

**FACTCHECK disposition (carried forward).** Settled atproto mechanics used here — `did:plc` (immutable),
public unauthenticated AppView (`public.api.bsky.app`), `com.atproto.identity.resolveHandle`, PDS blobs +
CDN, custom feeds, firehose/relay — **cite the SoT
(`atproto-atmospheric-web-iroh-mobile-FACTCHECK.md` + the architecture-explainer FACTCHECK), do not
re-verify.** Genuinely new, **[confirm before publish]** (not re-verified this session): platform rug-pull
figures (GeoCities Yahoo ~$3.7B/1999, deleted 2009; MySpace 2019 loss ~12 yr / ~50M tracks / ~14M artists;
Cohost shutdown 2025-01-12); `spores.garden`/Hypha Co-op mechanics; exact XRPC method names
(`getLikes`/`getPostThread`/`getProfile`). Browser-capability claims (File System Access, Web Share Target,
WebRTC, Service Workers/IndexedDB, Background Sync) are standard web-platform features, not atproto-specific.

Distilled **directly into the beta layers** (user's call — a deliberate pull-up recorded in
`../../LAYER-ROLLUP.md`, not the usual alpha-first staging): product surface →
`beta/croft/croft-ing-the-website-and-the-plot.md`; visual system + website IA →
`beta/socialization/visual-identity-and-the-progressive-depth-website.md`; logo →
`beta/socialization/logo-croft-ing-wall-to-gate.md` (+ pointer from `logo-drystone-stacking.md`). Indexes:
ECOSYSTEM rows (spores.garden/Hypha, Cohost, Neocities, Linkat, Bio.blue, kibun.social; Standard.site
updated), ROADMAP_TODO app items, COHESION entry (E1 realization + atmospheric-web overlap). Not committed to
the strict alpha `thinking/` distillation (beta was the chosen home).

## 2026-07 intake — use-case fit / lexicon governance / kindred-work glossary / croft.ing landing (session + crofting.zip)

| Raw artifact | Where | Status |
|---|---|---|
| Multi-thread claude.ai session (~2026-07-17…20): dating/meetup use-case fit, lexicon.community governance synthesis, kindred-work glossary, croft.ing landing request | `seeds/transcripts/raw/croft-usecase-lexicon-glossary-landing-session-2026-07.md` | **preserved-condensed (cleaned-paste, content-faithful — §4)**. Mostly duplication of already-filed material (see COHESION §44); the raw **points to the filed artifacts rather than re-embedding** (PUBLICATIONS, ENGAGE-LEX, RUN-LEX-01). UI chrome stripped; no creds. Vendor numbers in the dating thread `[UNVERIFIED]`, carried in the filed report's Caveats. |
| `crofting.zip` — 10-file landing-page planning package (RUN-01/02/03 build specs, page-copy drafts, terms/treatise/arecipe-share, coop-messaging-research, kindred-work) | `seeds/crofting-unpacked/` | **preserved-verbatim** (frozen seed; the `.zip` archive was byte-verified then **retired 2026-07-20** per user authorization — contents extracted as individual files under `crofting-unpacked/`). Working copy placed: `kindred-work.md` → `../../beta/socialization/kindred-work.md`. Deferred placement: the landing-page docs (RUN-01…03, copy drafts) belong with the `CroftCommunity/crofting_site` build stream. `coop-messaging-research.md` — **filed** (2026-07, not a dupe) to `../../beta/socialization/coop-messaging-research.md`, the evidence base behind `brand-and-voice.md`'s voice registry. |

| `graph.zip` — the four attest-lane RUN briefs (RUN-ATTEST-01…04) | `seeds/graph-unpacked/` | **preserved-verbatim** (byte-verified, `.zip` **retired 2026-07-20**; contents extracted as files). **Already-executed:** each brief's outcome is filed as `experiments/RUN-ATTEST-0{1,2,3,4}-SUMMARY.md`, and the settled primitives live in `experiments/attest-family/PRIMITIVES-ATTEST.md` + FINDINGS + code/tests/lexicons (ten of ten verdicts settled, RUN-ATTEST-04). Frozen for provenance; the brief *content* is not re-filed (dupe). |
| `stellin.zip` — 10 appview/protocol RUN briefs (RUN-14…19, appview-infra runs, PUBLICATIONS-DESIGN) | `seeds/stellin-unpacked/` | **preserved-verbatim** (byte-verified, `.zip` **retired 2026-07-20**; contents extracted as files). **Already-executed:** RUN-14/15/16/17/18/19 summaries all filed under `experiments/`; brief content not re-filed (dupe). |
| Stellin name-clearance report (claude.ai research, ~442 sources, 2026-07; two passes) | `research/stellin-name-clearance-2026-07.md` | **preserved-condensed (content-faithful)**. **NOT a dupe** — no prior clearance doc existed. Verdict CONTESTED; live RDAP/USPTO/EUIPO/Bluesky checks flagged **could not verify** (must re-run). NAMING.md Stellin entry added. |
| `pdshistory.zip` — RUN-HIST-02 + RUN-HIST-02-revB (LIVE PDS history-backend experiment briefs) | `seeds/pdshistory-unpacked/` | **preserved-verbatim** (byte-verified, `.zip` **retired 2026-07-20**; contents extracted as files). These are the **briefs/plan**; the run is **EXECUTED + MERGED** (verified on main) at `spike/hist_live/HIST-LIVE-RESULTS.md` (RUN-HIST-02 rev B, branch `claude/hist-atproto-live-gentle`, PR #30). E1/OC-2 settled GREEN (CID = byte-head, zero re-hash with atproto canonicalization). Briefs frozen for provenance; not re-filed. The app-password-vs-OAuth Q&A in the same thread is general Bluesky knowledge (covered by `skylite/docs/custody.md`) — not filed. |

| `arecipe.zip` — 11 arecipe build briefs (RUN-RECIPE-SEARCH, SHOPPING-LIST, RESET-SURFACE +V2, AGENTS-PAGE, SIGNED-RELEASES +V2, COOK-FOLLOWS, ANDROID-TWA, FOLLOW-THROUGH, RECIPE-IMPORT) | `seeds/arecipe-unpacked/` | **preserved-verbatim** (byte-verified, `.zip` **retired 2026-07-20**; contents extracted as files). Arecipe-repo build instructions; executions live in the `arecipe` repo. **Verified against arecipe main 2026-07-20:** 9 of 11 executed + merged (✅ plan docs on main — recipe-text-search, shopping-list, reset-surface, agents-page, cook-follows-and-toolbar, follow-through, recipe-import). **RUN-ANDROID-TWA** built but **NOT merged** (branch `claude/android-twa-packaging-bhbxuf`, 5 phases + closeout). **RUN-SIGNED-RELEASES (+V2)** **NOT merged** (branch `claude/signed-releases-v2-2yifm4`, no plan doc/workflow on main). The truncated "Architectural Blueprints…client-side search" research (pasted, incomplete) is NOT filed — needs the full text. |

| `friends.zip` — CONTACT.md, dating-friendship-meetup-report-v2.md, ENGAGE-LEX.md | `seeds/friends-unpacked/` (raw; `.zip` retired) | **preserved-verbatim** (byte-verified). Dispositions: **CONTACT.md** = NEW first-contact design doc → filed to its self-declared home `experiments/appview-infra/CONTACT.md` (beside GROUPS/PUBLICATIONS; referenced by RUN-ATTEST-04 + PRIMITIVES-ATTEST). **dating report v2** = NEWER, **supersedes** the v1 filed earlier → replaced `research/dating-friendship-meetup-fit-2026-07.md` (v1 in git history + this seed). **ENGAGE-LEX.md** = OLDER pre-execution draft; the on-main `attest-family/ENGAGE-LEX.md` supersedes it (carries the RUN-LEX-01-EXECUTED status) — preserved here as raw only, main not overwritten. |
| `skylite.zip` — 9 skylite build/instruction briefs (SKYLITE-DIRECTIVES.md, RUN-BRAND/TRUEUP-INSTRUCTIONS, the 2026-07-14/15 run plans) | `seeds/skylite-unpacked/` (`.zip` retired) | **preserved-verbatim** (byte-verified). These are the **pre-execution instruction briefs**; their executions are the `skylite` repo's `RUN-*-SUMMARY.md` (13 runs incl. TRUEUP, all landed — verified on skylite main). Briefs were **absent from the skylite repo** → frozen here so nothing is lost when the zip is deleted. Grounding for the next skylite iteration plan. |
| PDSVIEW-RUN-01-INSTRUCTIONS (pasted 2026-07-20, no zip) — build brief for pdsview.croft.ing | `seeds/pdsview-run-01-instructions.md` | **preserved-verbatim** (the brief as pasted). **EXECUTED + MERGED** (verified on main): `CroftCommunity/pdsview` carries the full built site (run-01 phases 0–5, PR #1, run summary in-repo, built 2026-07-16 by a Fable-5 session). Frozen for provenance; living record is the pdsview repo. Registered in `BUILD-INVENTORY.md` as a live tool-pad sibling (pdsview.croft.ing). |
| `human.zip` — human-adjudication-language survey + Claude Code instruction file (9 edit blocks) | `seeds/human-unpacked/` (`.zip` retired) | **preserved-verbatim** (byte-verified). **EXECUTED on main** (verified): the adjudication-language pass landed as `beta/drystone-spec/conventions-and-decisions.md` §A.11 + §B.8 ("the local authority" / "planes of authority" coined; DR-1..DR-10), the §7.6/§7.6.1 escalation-shapes-vs-parties disambiguation, RFC 8126/7282 corroboration in Part 1 §3, and the iroh LWW cleanup. Frozen for provenance; not re-filed. |
| Human-adjudication design-intent voice dialogue (voice-transcribed, TV-bleed flagged) | `seeds/transcripts/raw/drystone-human-adjudication-language-dialogue-2026-07-20.md` | **preserved-condensed (cleaned-paste, §4)**. Schitt's Creek audio bleed removed + marked per the user's request. Carries the load-bearing **operator-is-not-the-villain / planes-of-authority** correction that reshaped the A.11 vocabulary, the fork-not-verdict framing, and the RFC-precedent question (answered: RFC 8126/W3C/7282/Beer). |

| E2EE-recovery post-mortem research report (claude.ai deep research, ~275 sources, 2026-07; no zip) | `research/e2ee-recovery-postmortem-and-trust-predicate-2026-07.md` | **preserved-condensed (content-faithful)**. **NOT a dupe** — no prior recovery/custody research doc existed. Tests + refutes "recovery killed the dead E2EE products"; derives a staged recovery-predicate recommendation for I9 / A2 / A12. Surfaced, not resolved (the default predicate + UX is the owner's call). Cross-links `beta/drystone-spec` §2.8/open-threads, `thinking/governance-and-survivability.md`, `thinking/multi-device.md`. |

| Mastodon-vs-ActivityPub-vs-ATProto explainer + RUN-AP-01 / RUN-HIST-01 briefs + Berjon ap-at request + live-spike log (pasted 2026-07, no zip) | `beta/cairn/activitypub-atproto-and-the-defacto-standard.md` (new bit); rest on main | **distilled-only** for the genuinely-new bit (the AP/Mastodon/de-facto-standard explainer → the cairn stone, content-faithful). RUN-AP-01 (`experiments/ap-ambassador/`), RUN-HIST-01 (`hist-atproto-spike/`), and the live spike (`spike/hist_live/`) are **already on main** (verified via `git ls-files`/`git log`) — dupe, not re-filed. Berjon ap-at walkthrough deferred → ROADMAP_TODO E41. |

**Genuinely new outputs from this intake:** `research/dating-friendship-meetup-fit-2026-07.md`,
`research/stellin-name-clearance-2026-07.md` (+ NAMING.md Stellin entry),
`beta/cairn/activitypub-atproto-and-the-defacto-standard.md`,
(dating/meetup use-case-fit report), `../../beta/cairn/lexicon-community-governance.md`
(governance-body stone, related-ecosystem), `../../beta/socialization/kindred-work.md` (kindred-work
glossary + capture-at-session-time procedure, now in PLAYBOOK §3). Everything else in the paste was
already filed. The croft.ing landing-page build itself is a **separate work stream** against
`CroftCommunity/crofting_site`, not this repo.
