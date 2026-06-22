# Cohesion Map: loose ends ↔ the work that addresses them

date: 2026-06-15

purpose: the fragments in this effort were written at different times and are often unaware
of each other — one document declares something a loose end or "unproven," while another
(sometimes in the experiments repo, sometimes a later transcript) walks out exactly that
thing. This map notices those linkages so the corpus reads as one coherent body, and flags
where a finding needs to be backported into an earlier doc.

legend: **CLOSED** = addressed elsewhere, backport the result · **OPEN** = genuinely
unresolved · **DRIFT** = two docs disagree or one is behind the other.

repos: `discovery` (thinking/synthesis) · `experiments` (code: proofs/ + spikes/).

---

## 1. "Modeled MLS, re-run on the real stack" ↔ the Phase 1 crypto gate

- **Loose end:** the `lineage-group-model` proof (experiments/proofs) models MLS — it
  treats "two commits on one epoch = a fork" by fiat and defers forward-secrecy timing and
  real fork mechanics to "real-stack validation."

- **Walked out by:** `Proofs/lineage-groups` (PR #8) executed exactly this against **real
  openmls 0.8.1**: external-commit survivor re-key (E1.2), reinit/fresh-genesis (E1.3), PCS
  on removal (E1.1), queued-revocation (E1.4). **All four pass — Phase 1 gate is GO.** The
  TS model's deferral list and the Rust workspace's Phase 1 are the same boundary, now
  joined: model proves the logic, Rust proves the crypto.

- **Status: CLOSED.** The make-or-break of the whole thesis held. crystallized/proof-ledger.md
  dependency #1 is closed. The downstream phases (2, 2.5, 2.6, 3) all went GO too, and the
  adversarial passes found and closed two real gaps: governance equivocation (A2.2) and a
  departed genesis-admin who still governed (A2.4 → authority is now per-epoch). The proofs
  earned their keep — they falsified, not just confirmed.

## 2. The V1–V9 visibility prompt ↔ the proof that ran it

- **Loose end:** `seeds/generated-prompts/structural-tests-visibility-regimes-prompt.md`
  was drafted and unrun; `thinking/social-layer.md` still ends at section 7 and its header
  promises sections 8–10 (regimes, propagation geometry) that were never written.

- **Walked out by:** `lineage-group-model` implemented all nine — `core/visibility.ts`,
  `experiments/V_visibility.ts`, and `SOCIAL_LAYER_FINDINGS.md`. All 9 pass. The genesis
  payload gained `regime`, `outward_propagation_depth`, `inward_visibility`,
  `openness_class`; `MAX_DEPTH_FOR_CLASS` = closed:3 / open:1 / fully_open:0.

- **Headline finding to propagate:** **V3 is only structural for *automatic* crossing.** The
  protocol cannot stop a human from typing intimate text into a public republish — that is a
  UX-layer control, not a data-model one. This is the most important social-layer finding and
  it is currently buried in a proof, not in any thinking doc.

- **Status: CLOSED in code, DRIFT in thinking.** Action: write `social-layer.md` §8–10 from
  the proof's actual model (not the transcript guess), and record V3's limitation as a
  principle/conclusion. The thinking doc is now behind the proof.

## 3. SSB's "unbounded log growth" cautionary tale ↔ the soak + roll-up proofs

- **Loose end:** `research/messaging-solutions-landscape.md` names SSB's unbounded-log growth
  as the trap we must not repeat, and flags Automerge change-metadata growth as "the new
  append-only log if not compacted." `multi-device.md` worries about governance-log churn
  from per-device events.

- **Walked out by:** `lineage-group-model` group G (soak) — G1 reproduces unbounded growth
  *on purpose* without roll-ups and shows bounded cost *with* them; G2 is the explicit
  "month-eighteen" newcomer-renders-member-list scenario. Group F proves roll-up correctness
  and that a checkpoint can't span an open fork.

- **Status: ADDRESSED in model.** Action: the roll-up/checkpoint mechanism graduates toward a
  principle ("snapshot/compaction is a first-class requirement"). Still needs the real-crypto
  re-run for threshold-signed checkpoints (F2) — see #5.

## 4. The "blind superpeer broker / is it secretly the ordering authority?" worry ↔ F-group

- **Loose end:** the Achilles-heel research prompt (seed) names the ordering/consensus "dirty
  secret" as the top worry — is the superpeer secretly the MLS ordering service? The dossier's
  HA/anchor peer is flagged "triply load-bearing." The landscape synthesis concludes the
  broker is "de-facto mandatory" and "optional = graceful degradation."

- **Walked out by:** `lineage-group-model` group F — F3 (two-mode convergence equivalence:
  assert no outcome reachable with the superpeer is unreachable without it), F5
  (availability-as-rights-escalation probe: assert no right is escrowed to the superpeer's
  presence), F2 (threshold-signed vs authority-signed checkpoint — makes the "referee leak"
  a visible test result).

- **Status: PARTIALLY CLOSED in model.** F3/F5 are the honest test of the central bet. The
  Achilles-heel research prompt is still unrun and is the adversarial complement — run it to
  pressure-test what F-group asserts. **OPEN:** the real threshold-signature crypto (F2) is
  modeled, not real.

- **Sharpened by PR #3:** the `concurrent-membership` / `membership-sequencer` sub-experiments
  found that **a membership sequencer is load-bearing, not optional** — the early briefs waved
  it off; under concurrent membership changes it is required. This is the honest, partial
  "yes" to the dirty-secret question: there *is* an ordering dependency. The design's claim is
  not "no ordering authority" but "the ordering role is minimal, blind, and not a rights
  authority" (F5). State that plainly rather than implying pure orderless P2P.

## 5. PR #3's public-path sub-experiments ↔ the focused public-path spikes (DUPLICATION)

- **The seam:** `Proofs/encrypted-local-first-atproto` (PR #3) contains `jetstream-appview`,
  `local-appview`, `end-to-end-slice`, and `local-pds-bridge` — which cover much of the same
  ground as the standalone spikes `experiments/appview-validation` (PR #6) and
  `experiments/public-roundtrip` (PR #4). These were authored on separate branches and are
  partly unaware of each other.

- **Status: DRIFT / reconcile.** PR #3 is the comprehensive parent; #4 and #6 are
  deeper-but-narrower public-path probes (#4 has the crypto chain-of-custody capstone and
  moderation; #6 has the AppView-lifecycle bootstrap and the live publish loop). Action:
  decide the canonical home for the public/atproto validation, fold the unique findings of
  #4/#6 into the PR #3 accounting, and avoid maintaining three overlapping AppView slices.

## 6. PR #3's refuted/revised hypotheses ↔ the thinking docs (backport)

- **removal ≠ revocation of access.** PR #3 re-scoped "removing a member revokes access" to
  *forward secrecy only*; reading already-decrypted/stored content requires re-encryption,
  bounded to the stored copy. The thesis/multi-device docs imply revocation is cleaner than
  this. Backport the caveat (it matches the honest "can't claw back what was decrypted" line
  already in multi-device.md §6 — make them consistent).

- **public reference to a private record is itself a leak.** Matches social-layer V4 (a
  republish reference must not enumerate the intimate group) and the "born-into-a-regime"
  rule. Same finding, two proofs — link them.

- **rkeys must be pinned for stable identity.** From `stable-record-identity`; a concrete
  data-model requirement to record in the substrate principles.

## 7. Multi-device "lineage credential on the MLS leaf" ↔ INV-LINEAGE-NOT-LEAF

- **Loose end:** `multi-device.md` §8.1 names one protocol dependency — the lineage credential
  must ride on the MLS leaf so others can fold devices and count thresholds by lineage —
  "verify against the real library first."

- **Walked out by:** `lineage-group-model` proves INV-LINEAGE-NOT-LEAF (group B1: adding N
  devices to one lineage changes no threshold outcome) — but in the *model*, where the
  leaf↔lineage mapping is assumed available. The proof confirms the *logic* is sound; it does
  not confirm `openmls` can carry the credential on the leaf.

- **Status: logic CLOSED, library dependency OPEN.** Same shape as #1: modeled-sound, needs
  the openmls credential/leaf-extension check (ledger dependency #2).

## 8. appview-validation spike ↔ the dossier's atproto/identity open questions

- **Loose ends (dossier §4.3, §5, §10):** does did:plc resolution work in practice; do custom
  lexicons need registration; is "referenceable but not public" achievable; PDS vs entryway
  topology.

- **Walked out by:** `appview-validation` spike found, against the live network: custom NSIDs
  propagate on Jetstream with **no pre-registration** (registration is a consumer-side
  convention, not a network gate); writes go through the entryway (`bsky.social`) but repo
  reads require the account's real PDS, resolved from the DID doc; the firehose is
  collection-agnostic.

- **Status: SEVERAL CLOSED with live evidence.** Action: fold these facts back into the
  dossier's atproto sections as verified (they currently read as open/unverified). Note: this
  is the public-content / atproto path, distinct from the private group path — keep that
  boundary explicit.

## 9. atproto chain-of-custody + moderation ↔ dossier identity/moderation open items

- **Loose ends:** the dossier (§5) favours self-certifying DIDs and names "composable
  moderation — ATProto labelers (subscribe to independent label services) as the model";
  the lineage thesis rests on provenance (I3/I8). All treated as design intent.

- **Walked out by:** the `public-roundtrip` experiment (PR #4) proved, live, the full chain:
  verified DID → signing key (from the DID doc) → signed commit → MST root → record CID →
  record bytes — checkable with zero trust in the PDS/relay. And it validated the labeler
  model: labels are independently-signed assertions, a labeler is a signed identity with a
  *distinct* `#atproto_label` key, distribution is **pull-only** (subscribeLabels 404), and
  signature verification has sharp edges (label `cid` is a lexicon string, not a CBOR link).

- **Status: CLOSED for the public/atproto path, with live evidence.** Key takeaway to fold
  into the dossier and principles: **atproto gives cryptographic trust (identity + integrity)
  for free, but zero semantic trust — own your schema, threading, moderation policy.** This
  is the same boundary as the lineage thesis's "compose MLS, own governance." Note: this is
  the *public* path; the private group path is the lineage-groups proof. Keep that boundary
  explicit.

- **Carried finding:** the experiment's own runtime logging dumps raw event JSON incl. user
  DIDs + content (PII-in-logs, flagged in the PR). A real ingester must redact to
  did/cid/collection/rkey. Relevant to any production telemetry design.

## 10. android-p2p-app ↔ the dossier's mobile-feasibility + Automerge-over-iroh bets

- **Loose ends (dossier §4.4, §4.6):** "messaging = vault artifacts, Automerge when
  interactive" (one substrate, one sync engine over iroh) was a unifying *claim*; iOS/mobile
  feasibility was flagged UNPROVEN (no documented iroh-on-device reference; build via UniFFI =
  the RustDesk/Mullvad/LibXMTP pattern, "high confidence" but unspiked).

- **Walked out by:** `experiments/android-p2p-app` (PR #7) — a Rust core over **UniFFI**
  bindings doing **two-peer Automerge sync over real iroh**, Tier-1 verified (cargo test
  green). It proves the Automerge-over-iroh sync engine works peer-to-peer and that the
  UniFFI mobile-binding pattern holds; APK assembly is toolchain-gated (NDK/SDK), not
  code-gated (PATH_TO_APK.md).

- **Status: PARTIALLY CLOSED.** The dossier's Automerge-over-iroh unifying move and the
  UniFFI mobile path are validated on Android. The dossier's specific *iOS-runtime* unknown
  (battery/background/cellular-NAT) is still unspiked — Android ≠ iOS here. Also note this
  is the Delta-Chat-cousin lesson made concrete (the app is Delta-Chat-inspired).

## 11. Broadcast tier ↔ SSB cautionary tale (the lesson, applied forward)

- **The seam:** the interaction-tiers model's broadcast tier (thinking/interaction-tiers.md)
  is explicitly "Scuttlebutt-shaped, without SSB's fatal flaw" — a rolling-forward
  announcement log that keeps SSB's good half (append-only gossip, no real-time pretense) and
  drops the bad half (immutable infinite history) via a bounded/rolling window.

- **Status: COHESIVE.** This is the research doc's SSB log-growth cautionary tale
  (research/messaging-solutions-landscape.md; COHESION #3) turned into a *design choice*
  rather than a risk. Connect the broadcast tier to the roll-up/compaction proofs
  (lineage-group-model F/G). The "group size is three products" reframe also answers the
  field's recurring failure: pretending the 1000-person room works like the 5-person room.

## 12. Multi-device reaffirmed as THE open problem ↔ recovery-anchor decision

- **The seam:** the Germ/X Chat dialogue independently lands on multi-device + total-device-
  loss recovery as the single weakest point — the same conclusion as the landscape synthesis,
  the thesis (E3.3), and COHESION #1's residual. X Chat's entire Juicebox design exists to
  solve exactly this, paying with encryption integrity (server-held keys, no PFS).

- **Status: OPEN, sharpened.** The fork is now explicit: trust-minimized key backup
  (escrow operator-can't-read / threshold-shared) vs. device delegation (existing device
  authorizes a new one; needs a device present). Likely answer: delegation primary + optional
  trust-minimized backup for lose-all-devices. Juicebox is the studied prior art (ECOSYSTEM).
  This is the top unresolved design decision across the whole corpus.

## 13. PR #5 encrypted-blob-share ↔ the dossier's media path + realtime/durable split

- **The seam:** the dossier specs large media as "encrypted content-addressed blobs over
  iroh-blobs, referenced from the document," and the landscape doc validated the Delta-Chat
  realtime(P2P)/durable(store-and-forward) split.

- **Walked out by:** `experiments/encrypted-blob-share` (PR #5) proved encrypt → content-
  address → store → reference → fetch → decrypt over **real iroh-blobs** with MLS epoch
  rotation. **Status: CLOSED for the media path.** Carried finding: encrypt-then-content-
  address **loses cross-user dedup** — a real design tradeoff to record (identical plaintext
  encrypts differently per recipient-set, so it can't dedupe across groups).

## 14. "Croft" name ↔ the dossier's civic/commons material

- **The seam:** the crofting research (seeds/transcripts/raw/croft-crofting-research.md)
  deepens and grounds the dossier's commons/Ostrom/enclosure/Winstanley material (§3) and
  resolves the open "pin the name map" item (§1.1).

- **Status: CLOSED (naming) + DEEPENS civic vertical.** NAMING.md records Croft as the center
  of gravity with the unromantic rationale (croft = engineered dependency dressed as tenure;
  partial win; community-ownership echo; maintenance burden). Honest disanalogy noted (land
  rivalrous, virtual space not). Feeds narrative/verticals/the-civic-why.md.

## 15. The peer-equality principle — wording has evolved

- **DRIFT:** the dossier says **"equal in ability, not capacity."** The current formulation is
  **"all peers are equal in rights, but not capabilities."** Same idea, sharper: it is about
  *rights* (what a peer is permitted to do) vs *capabilities* (what its hardware/uptime lets
  it do). F5 (availability-as-rights-escalation) is the proof that operationalizes it — a
  well-resourced superpeer must not acquire *rights* by virtue of its *capability*.

- **Status: reconcile.** crystallized/principles.md uses the refined wording; the dossier's
  phrasing is the ancestor.

## 16. Crofting narrative re-telling ↔ the crofting research (same subject, two registers)

- **The seam:** a narrative re-telling of the crofting story
  (`seeds/transcripts/raw/croft-crofting-narrative.md`, pasted 2026-06-22) covers the **same
  subject** as the scholarly research file (`croft-crofting-research.md`) but in a popular,
  quotable register.

- **Status: DUPLICATION + DRIFT (both intentional, both kept).** *Duplication:* the arc, the
  1886 Act, "Magna Carta of Gaeldom," common grazing, and the modern legacy are already in the
  research file and distilled in `NAMING.md`. *Drift:* the narrative leans on the heroic "free
  clan → cleared → fought back → won" arc that the research file explicitly flags as the myth
  needing qualification (crofting was largely *invented* by the clearing landlords; 1886
  secured tenancy, not land). The raw file is headed with that caveat, so the de-romanticized
  ground truth is not lost.

- **What it adds (promoted):** five vivid, `[UNVERIFIED]` quotable items — Chambers' 1827
  "useless human beings"; "the lairds' four-footed clansmen"; the Shetland eviction-curse
  anecdote; the Bernera Riot (1874); and the modern Mackintosh-v-Cameron land-court echo.
  These are distilled into a "Vivid grounding" subsection in `NAMING.md` and are seed material
  for the still-unwritten `narrative/verticals/the-civic-why.md` (the loose end in §14).

- **Source-quality flag:** the narrative's citations are tertiary (tourism/retail blogs,
  Goodreads), weaker than the research file's secondary scholarship — verify before any
  external use.

## 17. AT-Proto atmospheric-web / Iroh mobile dialogue ↔ the existing Iroh + lineage-groups work

- **The seam:** a Gemini design dialogue (`seeds/transcripts/raw/atproto-atmospheric-web-iroh-mobile-dialogue.md`,
  fact-checked in `...-FACTCHECK.md`) explores an AT-Proto "atmospheric web" / Neo-GeoCities /
  open-LinkedIn product vein and an iOS opportunistic-mobile-P2P-over-Iroh vein. Both touch
  ground the corpus already holds.

- **CLOSED / CONFIRMED (Iroh):** the dialogue's Iroh substance was checked against this project's
  own pinned-`=1.0.0` verified source (`experiments/iroh/relay-lab-runs/IROH-1.0.0-API-VERIFIED.md`)
  and the relay-lab spikes (iroh-docs 0.100 / gossip 0.100 / blobs 0.102). Range-based set
  reconciliation, HyParView/Plumtree gossip, `unstable-custom-transports`, QUIC-multipath
  migration, and BLAKE3 all confirmed — consistent with `thinking/realtime-media-over-iroh.md`
  and the [[croft-relay-lab-2026-06-16]] findings.

- **DRIFT / corrected:** the dialogue claims **iroh-docs uses Merkle Search Trees** — REFUTED, it
  uses range-based set reconciliation (MSTs are AT Proto's structure). Don't let this conflation
  into any thinking doc. The Rust snippets use a non-existent `connect_to_peer` API.

- **CLOSED — the private-groups gap is *our* thesis.** The dialogue's claim of an official
  "AT Messaging" MLS working group is REFUTED; real AT-Proto E2EE/groups are third-party today
  (**Germ Network**/MLS — already in corpus via `germ-xchat-design-dialogue.md` /
  `research/germ-xchat-features.md` — and the XMTP bridge). This is exactly the gap Croft's
  **lineage-groups Phase-1 MLS proof** (GO on openmls 0.8.1) answers. Link: the atmospheric-web
  product vein needs private groups; Croft already proved the crypto for them.

- **OPEN / flagged:** the "opportunistic mobile scavenger mesh" (two locked phones auto-waking
  over BLE and gossiping) is shakier than the dialogue implies — CoreBluetooth restoration does
  NOT relaunch on new-advertiser discovery, and Berty reports background P2P dies within seconds.
  Treat as aspirational. Relates to `thinking/multi-device.md` and the meer/superpeer model.

- **Status: DISTILLED (2026-06-22).** Two thinking docs written —
  `thinking/atproto-atmospheric-web.md` (product vein) and `thinking/ios-opportunistic-p2p.md`
  (mobile vein), carrying only CONFIRMED/PARTLY claims with verdict flags. `ECOSYSTEM.md` updated:
  iroh row corrected to `1.0.0` (+ EndpointId/custom-transports/iroh-ffi), new community-transports
  row, and new **§5b** (atmospheric-web apps + Rust tooling). Germ DM (§6) is the standing
  atproto+MLS link; the private-groups gap ↔ Croft lineage-groups is recorded in both thinking
  docs. The mobile-scavenger-mesh caution is flagged OPEN (needs a real spike).

## 18. The Croft app (client layer) ↔ the proven lineage-groups substrate + open risks

The 2026-06-20→22 design dialogue opened a **new body of work**: the app/client layer ("Croft" the
product), distinct from the protocol thinking it rides on. Material landed at `thinking/app/`
(README + philosophy + criteria + brand-draft + build-specs), seed at
`seeds/transcripts/raw/croft-app-design-dialogue-2026-06-20-to-22.md`, frozen artifacts at
`seeds/multiecosystemapp-unpacked/`.

- **CLOSED (rides the substrate):** the **Croft Group** pond *is* the lineage-groups work surfaced
  — its iroh transport tiers (Tier-1 public bridge / Tier-2 browser-as-peer relay-only-but-E2EE /
  Tier-3 native full peer) sit directly on the iroh substrate (`ECOSYSTEM.md` §1, the 1.0 row) and
  the meer/blind-broker model (`thinking/meer-superpeer-design.md`). The app's **scoped appview** (custom lexicons + private block-lists/stats
  via service-proxy/service-auth) is the same shape as the atproto extension work (§5b, COHESION
  #8). So the app does **not** re-open the protocol — it consumes it. That it slots on with no
  protocol change is itself evidence the substrate decisions hold.

- **OPEN (carried, not resolved):** the app surfaced four risks now tracked in
  `thinking/open-considerations.md` §8-10 + `thinking/app/README.md`: infra-sustainability ↔ the
  cooperative *mechanism* (existential; links to ROADMAP §8 charter + `governance-and-survivability.md`);
  moderation/safety vs the kid-friendly goal (links to `geer-gating-peer.md`); cold-start for the
  owned pond; and the **CroftC IP/ownership entanglement** (Phase 0 code in CroftC PR #10 — import
  deferred, ROADMAP §13, the user's decision).

- **DRIFT to watch:** the app's brand work introduces "Croft" as the *product* name and "Croft
  Group" as the chat pond (plus the pond/pad taxonomy + "Grow your own"). `NAMING.md` pinned Croft at
  the *umbrella* level; reconcile product-vs-umbrella naming when `brand-and-voice-notes.md` settles
  (ROADMAP §14). Per the README convention, do not propagate unsettled product names into structure.

- **Status: PARTLY DISTILLED (2026-06-22).** Thinking + provenance landed and connective tissue
  updated (this entry, ROADMAP §12-15, open-considerations §8-10, RAW-ARTIFACTS-MANIFEST). The
  embedded industry research (iroh-in-browser, webxdc/Delta-Chat games, super-apps/W3C-MiniApp,
  appview routing, Rust client libs, Crux/FCIS) is **not yet** distilled into `research/` /
  `ECOSYSTEM.md` rows — flagged OPEN as the follow-on (ROADMAP §14).

## 19. The ponds & pads / games deep-dive ↔ the app body + the substrate

The 2026-06-20→22 ponds/games dialogue (the **run** of `seeds/generated-prompts/games-pond-research-prompt.md`)
deepens §18's app body into *what actually fills the garden and how to build it*. Material at
`thinking/app/ponds/` (8 artifacts), seed at
`seeds/transcripts/raw/croft-app-ponds-games-dialogue-2026-06-20-to-22.md`, frozen at
`seeds/apps-unpacked/`.

- **CLOSED (rides the substrate, sharpened):** the games/utility/ritual pads sit on **iroh 1.0** —
  `iroh-gossip` (ephemeral/live), `iroh-docs` (accreting state; LWW per (author,key) forces an
  **event-sourced** data model — the split-the-check ledger and the guestbook share that shape),
  `iroh-blobs` (content-addressed files; `sendme` is the reference). One reusable **fair-reveal
  (commit-reveal)** primitive powers voting + dice + hidden-info games (`fair-reveal-primitive-spec.md`).
  Inclusion is three pathways: **build-fresh** / **wrap** (a webxdc-compatible shim makes the whole
  ArcaneCircle catalog wrappable for one layer's cost) / **port** (WebRTC→iroh transport swap).

- **Honest asterisk on "serverless" (record, don't over-claim):** browser iroh peers are
  **permanently relayed** and direct hole-punch falls back to relays (n0's by default). "No
  application server" holds; connection-bootstrap leans on relays — self-host if it grows. Folded
  into `open-considerations.md` §8 (the infra-sustainability point) and ROADMAP_TODO.

- **Security finding (genuinely ours):** the Cure53 webxdc audit shows **CSP alone does not contain a
  webview** (WebRTC + DNS-prefetch exfiltration). Since iroh QUIC is the transport, **disabling the
  webview's WebRTC is pure upside** and closes that hole — an action item specific to wrapping our own
  Tauri webview. Plus: games get a **hard-separate webview context** + an **ephemeral per-match
  pseudonym** (a game must not read a stable DID). See `webxdc-security-and-competitive-games.md`.

- **Economic frame (backbone for the charter, ties to governance-and-survivability + ROADMAP §8):**
  **durable maintenance** (bounded by entropy) is categorically different from **extractive
  attention** (unbounded by appetite → dark patterns on schedule); the cooperative makes the
  maintenance curve fundable because funder and beneficiary are the same body, so no actor's interest
  is escalation. The moat is *promises with no expiry date*, structurally impossible for anyone
  carrying server cost or an extraction mandate. This is conversational-only (preserved in the
  transcript), the most charter-relevant output of the round.

- **DEFERRED/CONSTRAINT:** the on-device-LLM navigator is a **great-to-have, never a requirement**
  (hardware coverage; ROADMAP_TODO E10); seamless cold-install deep-linking is **not privately
  achievable** (Instant Apps/Firebase-Dynamic-Links dead, MMPs need fingerprinting) → claim-code
  one-more-tap (ROADMAP_TODO).

- **Status: PARTLY DISTILLED (2026-06-22).** Thinking + provenance landed; connective tissue updated
  (this entry, ROADMAP_TODO E8-E11, ECOSYSTEM §5d, open-considerations §8, manifest, raw index). The
  build-order is the sequencing home (`thinking/app/ponds/build-order.md`); the games/tooling
  ECOSYSTEM rows (§5d) still want a final license glance at bundle time.

## 20. `appframework.zip` ↔ the already-imported app-layer docs (DUPLICATION / superseded)

- **DUPLICATION (superseded):** the 2026-06-22 `appframework.zip` contained earlier snapshots of two
  docs — `BUILD-SPEC.md` (25,766 B) and `design-philosophy.md` (23,282 B) — that are already imported,
  in a *more-developed* form, as `thinking/app/build-specs/BUILD-SPEC.md` (27,378 B) and
  `thinking/app/design-philosophy.md` (37,350 B), byte-identical to the frozen
  `seeds/multiecosystemapp-unpacked/` seed. Both contain all five locked DECISIONS (1–5); the repo
  copies add a §3a cursor-invariant *proof*, the §1a garden thesis, and §4a credit/traceability that
  the zip lacks. Net-new content in the zip vs. repo: **none** (zip-unique lines are only earlier
  phrasings of text the repo states better).

- **Status: NOT IMPORTED (correctly). The docs were left untouched** — overwriting the more-developed
  repo copies with the earlier snapshot would have regressed them. What *was* genuinely new and got
  filed: the **derivation dialogue** that produced the zip (the port-ownership argument; how DECISION
  1–5 were reached), absent from `croft-app-design-dialogue-2026-06-20-to-22.md`, now at
  `seeds/transcripts/raw/croft-app-portdecision-review-2026-06-21.md`. The zip itself remains in the
  workspace root, superseded; recommend deletion at bundle time (the multiecosystemapp seed is the
  canonical verbatim — discard pending user OK, not deleted unprompted).

## 21. Cross-platform identity provenance ↔ the PLC/identity-resilience work + the sovereignty thesis

- **EXTENDS (new axis):** the 2026-06-20 identity-provenance dialogue
  (`seeds/transcripts/raw/croft-identity-provenance-dialogue-2026-06-20.md`) opens a distinct axis from
  `plc-identity-resilience.md`. That doc answers "which DID method roots an MLS identity, and how to
  build a validating PLC replica." This one answers "how does one person prove ownership of accounts
  across Bluesky/ActivityPub/Hive." Distilled to `thinking/cross-platform-identity-provenance.md`;
  the two are cross-linked.

- **CLOSED (the structural answer):** a cross-platform *authority* key is impossible — each network is
  a closed cryptographic root of trust and won't delegate it. The only real linkage is **out-of-band,
  mutually-anchored or root-signed provenance attestation** (the hub-and-spoke: did:webvh root as a
  correlation anchor, *evidentiary not operational*; spokes keep their own native keys). Key lineage is
  **attestation, not derivation** (pre-rotation vertical chain via `nextKeyHashes`; verification-method
  horizontal chain — provenance is the signed log entry, not a BIP32 path). This is a clean expression
  of the sovereignty thesis (identity survives platform change because it never depended on one
  platform's authority) — flagged as a `crystallized/principles.md` candidate, **not yet inserted**
  (promotion is the user's call).

- **CONFIRMED / consistent (no drift):** the dialogue's atproto facts align with
  `plc-identity-resilience.md` (72h recovery window; rotation keys k256/p256; PLC as a self-certifying
  transparency-log-not-CA; central `plc.directory` run by Bluesky PBC; governance handoff planned, not
  done). It also independently surfaces the **did:plc↔did:webvh convergence** (#2705 dual-resolvability)
  as cheap hedge-positioning — keep the did:webvh SCID as anchor even though nothing reads it yet.

- **OPEN (carried, not resolved → ROADMAP_TODO):** the **anchor-URI stability contract** (stable
  logical URI, mutable payload — not content-addressed/frozen); **PDS-held vs self-controlled did:plc
  rotation key** (relates A2 recovery-anchor); and the **not-yet-written per-platform trust-model doc**
  (the dialogue's repeatedly-offered, highest-leverage next artifact). `[UNVERIFIED]` carried forward:
  native atproto did:webvh support; PLC governance handoff status; whether PDS/PLC tooling preserves
  extra `alsoKnownAs` entries on write (needs a real test).

## 22. The design-imperative / architecture body ↔ the existing principles + lineage-groups + identity work

- **GROUNDS (the deep "why" under everything):** the 2026-06-20 design-imperative dialogue
  (`seeds/transcripts/raw/croft-architecture-design-dialogue-2026-06-20.md`) supplies the cross-field,
  cross-millennium lineage (Socrates → Mill → Peirce/Popper → Hayek → Ostrom → Ashby → Beer → Scott)
  that *grounds* principles the corpus already held as design choices. It turns several from preference
  into theorem: **equal-in-rights** (a consistency/requisite-variety requirement, not a moral overlay),
  **forks-are-a-feature** (local-first's native move; "never algorithmically adjudicate a social
  dispute" generalized to *compute provenance, never utility*), **provenance-is-the-dual-purpose
  primitive** (the razor). Filed: essay → `narrative/lineage-of-a-design-imperative.md`; architecture →
  `thinking/local-first-as-design-imperative.md`; deepest principles → `crystallized/principles.md`
  ("The deeper foundation" section).

- **CLOSED (the Kleppmann reconvergence tension):** the open question of whether Croft's fork/merge
  contradicts CRDT Strong Eventual Consistency is resolved — **reconvergence policy is per-plane
  (asset-overridable), declared at intent-to-collaborate, bound into the asset's hash.** SEC auto-merge
  for incidental concurrency (chat/docs); human-gated reconvergence where divergence is substantive
  disagreement (governance/action). Kleppmann's model is correct for one class of planes, Croft's for
  another. Backport target: this is the formal statement behind Tier-2 "forks are a feature" and the
  "six tapes in a room" no-merge rule.

- **CONFIRMED / consistent (no drift):** the substrate facts align with existing work — MLS epochs as
  the re-key backstop (Tier-2), blind-broker/superpeer anti-entrenchment (`meer`/`geer`), the
  freshness/non-equivocation guarantees (`freshness-signal.md`; gossip = CT/CONIKS). The atproto/iroh
  facts (DID portability, iroh-gossip = HyParView+Plumtree, plc.directory governance) align with the
  FACTCHECK and `plc-identity-resilience.md`. The trailing bridge-doc verification corrections are
  folded into `thinking/cross-platform-identity-provenance.md`.

- **OPEN (carried, not resolved → ROADMAP_TODO):** the **centerless-meets-center frontier** (legal
  entity / money / name registrar / scaling relay — the largest-clothes irreversible-singleton, deferred
  not solved); **governance-at-scale** (subsidiarity + liquid delegation, concentration the default
  failure); **the federation/peering PoC** (BGP-autonomy + postal-hierarchy + signed routing; recursive
  workers over atproto, DNS as swappable resolver); **forward-only revocation under irreversible
  commitments**; **the duty-of-care re-homing** (durability/recoverability the center carried); and the
  three new threat-model seams (epoch×fork×offline-grant; the delegate courier-vs-agent unification;
  content-predicate search-coverage attestation). DRIFT-to-watch: the architecture's "design philosophy"
  is the *protocol/substrate* layer — distinct from `thinking/app/design-philosophy.md` (the
  *client/app* layer); keep the two from being conflated.

---

## How to use this map

When a document says "unproven," "open," "TBD," or "verify later," check here first — the
thing may already be walked out in a proof, a spike, or a later transcript. When a proof
surfaces a finding (like V3's limitation), add a row here and backport it into the relevant
thinking doc so the synthesis never lags the code. This file is the seam-tracker; keep it
current as fragments keep arriving.
