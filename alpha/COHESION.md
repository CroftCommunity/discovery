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

## 23. Croft-app Phase 0 code (CroftC PR #10) ↔ the app body + the design-imperative spine

- **CLOSED (the deferred import landed):** the Phase-0 functional core + shell stack built externally
  (CroftC PR #10) is imported to `experiments/croft-app-phase0/` (byte-identical, 87 files), at the
  user's direction — exercising the **A8 IP/ownership decision** (surfaced, not auto-resolved). This
  closes the long-standing "Phase 0 built, import deferred" loose end (was AGENTS.md headline + A8).

- **CLOSED (the code is the dialogues' executable Phase 0):** the import is the running proof that the
  app-design DECISIONS held — DECISION 1 (async port in `bluesky`, consumed by the shell, never the
  core), DECISION 2 (native post type in the model), DECISION 4 (cursor-bearing states carry the
  cursor), and the **no-fabricated-fixtures** rule (real recorded `getTimeline` responses present; M6
  live adapter deferred). 20 acceptance tests green (A1–D2, P1–P7). The derivation lives in
  `seeds/transcripts/raw/croft-app-portdecision-review-2026-06-21.md` + the app/design-imperative
  dialogues; this is its code.

- **DRIFT (as-built spec vs. thinking/app):** the PR's `BUILD-SPEC.md` / `design-philosophy.md` are
  the spec the code was *actually written to* and differ from the more-developed
  `discovery/thinking/app/` copies (design-philosophy 550 vs 765 lines; BUILD-SPEC ~14 lines diff —
  thinking/app has §3a cursor-invariant proof, §1a garden thesis, §4a). Both kept on purpose: the
  experiment carries its as-built spec verbatim; thinking/app is the evolved design. Backport target:
  when the code graduates, reconcile to the thinking/app spec (and address the CodeRabbit doc nits —
  "written-down shortcut" undefined; DECISION-5 burden on the CLI fake).

- **Carried findings (license discipline):** cycode flagged `webpki-roots` (CDLA-Permissive-2.0 — *is*
  permissive) and `r-efi` (tri-licensed, used under MIT/Apache, UEFI-only transitive, resolved in-PR).
  CroftC-policy-scoped; on chasemp infra no gate blocks, but re-check under the destination policy if
  the code moves to a project repo. Same license-discipline theme as A1 (MPL-2.0/`hpke-rs`), distinct
  dep.

- **DECIDED (the two-CLIs question, 2026-06-22):** the client architecture is settled —
  `thinking/app/client-architecture-adr.md`: **one shared functional core + thin per-platform shells**
  (each supplying its own `effects.rs` callout), with two orthogonal callout axes (platform +
  implementation-behind-a-port), proven by the Phase-0 import. The user named it the most mature model
  we have; **prior client work adapts to it.** `croft-chat-cli` already has the implementation seam
  (`Transport` port + in-proc fake) but not the core/shell, so adoption is **greenfield growth, not a
  refactor**. Captured as a Tier-3 principle + the ADR; tracked as ROADMAP_TODO E19 (plan not yet
  drafted — the user's next-step call). **Decomposition RESOLVED 2026-06-22 (option C):** per-pond
  domain cores (bounded contexts) unified by the shared `shell` composition layer — group-core +
  Transport port symmetric to feed-core + Bluesky port; cross-pond **awareness** = read-only
  composition in the shell (resolve an `at://` reference via the other pond's port), cross-pond
  **interactivity** = a deferred idiom-translating broker between cores (honest-seams). DRIFT-to-watch:
  the `iroh/` experiment is still **not in the `experiments/README` index** — close that gap.

## 24. Drystone protocol-naming dialogue ↔ the rights-vs-capability principle + the meer/F5 guards

- **CLOSED (the P2P protocol now has a name):** the user chose the **Drystone P2P Protocol** as the
  name for Croft's peer-to-peer protocol (the thing `crystallized/CROFT-PROTOCOL.md` specifies).
  Recorded in `NAMING.md` ("Protocol-layer naming"). A sub-product name under the Croft umbrella, so it
  **partially closes ROADMAP_TODO A7** (the sub-product name map) at the protocol layer — the app/brand
  layer (A7/C6) is still open. Source: `seeds/transcripts/raw/croft-drystone-protocol-naming-dialogue-2026-06-22.md`.

- **CONFIRMS (a name for a principle we already hold):** the dialogue's design imperative — *peers
  equal in rights, not in capability* — is **already** pinned as `principles.md` Tier 1 ("all peers are
  equal in rights, but not capabilities") and proven by **F5** (availability-as-rights-escalation). The
  Drystone name doesn't add a principle; it gives the existing one a load-bearing metaphor and a
  memorable handle. No drift.

- **CLOSES a framing gap (the anti-pattern is now named):** the user flagged the **"Princeps
  Problem"** — nominal peer equality masking capability asymmetry, so the highest-capacity node fills
  the vacuum and becomes a de-facto authority — as *the* anti-pattern to guard against in any P2P
  system. This is exactly what the **meer anti-entrenchment** design (`thinking/meer-superpeer-design.md`,
  materially-reversible delegation) and **"different, not weaker"** already defend against; the dialogue
  supplies the name (Jo Freeman's *"The Tyranny of Structurelessness"*, 1970/72, applied to P2P) and
  the real instances (BitTorrent seeder dependence; PoW/PoS pool/validator concentration). Backport: the
  capability/rights split in `principles.md` Tier 1 and the meer doc can cite the Princeps Problem by
  name as the failure they prevent.

- **FACTCHECK (Gemini, heavily verified):** the substance is grounded; isolated provenance drift was
  caught and is recorded in the `-FACTCHECK.md` companion — fabricated Greek terracing terms ("Skartsia
  and Tomi" → real: xerolithia/pezoules), "adelphity" as an old English word, Farid's Arabic root, the
  "musha-gaeshi" gloss, and the non-standard stone terms "builders"/"packing." **None affect the design
  payload**; flagged so a superseded claim isn't laundered into a published doc.

- **DESIGN-ONLY (don't over-claim the metaphor):** the footings/through-stones/hearting ↔
  seed/desktop/mobile mapping is design *vocabulary*, not an implemented routing tier. Do not let it
  harden into a claimed mechanism in `CROFT-PROTOCOL.md` without a proof (status `design`).

---

## 25. Crypto-wars → mobile-P2P → PDS-economics dialogue ↔ the substrate bet, the MLS rationale, the sustainability problem, and the rights razor

- **CONFIRMS the substrate bet has production prior art (build-on / learn↔):** **Peat, by Defense
  Unicorns** — fact-checked as **real** despite smelling fabricated — is essentially Croft's exact
  protocol stack (**Rust + iroh transport + Automerge CRDTs + MLS**, plus `peat-gateway`→Okta/Keycloak)
  proven in *denied/degraded/contested* (ATAK / tactical) conditions. This is the strongest external
  validation yet that Croft's bet is sound and survivable off-grid. Registered in `ECOSYSTEM.md` §1.
  Source: `seeds/transcripts/raw/crypto-wars-to-p2p-pds-economics-dialogue-2026-06-22.md` + `-FACTCHECK.md`.

- **CONFIRMS the "unequal peer" we already accept (no drift):** the dialogue's **four-property
  impossibility** — group moderation + multi-device + PFS + offline-mesh cannot coexist without an
  unequal/privileged peer — is the same constraint our lineage-groups work already lives inside: MLS
  (RFC 9420) **assumes a Delivery Service** for ordering. So Croft's design isn't dodging the
  impossibility, it's making the honest trade openly (a sequencing service for the group plane), which
  is the `principles.md` Tier-1 stance restated. Backs `thinking/ios-opportunistic-p2p.md` and the
  protocol spine. The protocol comparison table (Matrix/Briar/Cwtch/Quiet/SimpleX/Keet/Wesh) + the
  "secondary dials" (MLS wire-overhead, metadata-vs-scale, eviction-lag, log-pruning, Energy-Depletion,
  Sybil, traffic-analysis, DHT warm-up) are an accurate field map → ECOSYSTEM §6.

- **FEEDS the top open problem (surface, don't resolve):** the **PDS-hosting + P2P-blended business
  model** (consumer/creator/operator/**enterprise-compliance** tiers) is *real demand* — the compliance
  cluster (SEC 17a-4 / FINRA 4511 / 2210; $3.5B+ off-channel fines; Deloitte $200k; Velox $1.8M;
  Smarsh/Global Relay) is **CONFIRMED**. This is direct input to the existential open item
  (**sustainability ↔ the cooperative *mechanism***, `open-considerations.md` §8 / ROADMAP_TODO). **Open
  tension to hold:** the dialogue frames this as a for-profit SaaS; Croft's stance is cooperative /
  non-extractive. Do **not** let the model's framing silently become Croft's answer — it's the user's
  call. Tracked ROADMAP_TODO E20.

- **CONFIRMS a legal ancestor of the rights razor:** the Bazelon **Hush-A-Phone** standard (1956) —
  a user's right to use a device "in ways which are **privately beneficial without being publicly
  detrimental**" (CONFIRMED: Bazelon authored it) — plus the **Carterfone → Ma Bell → Apple** arc is a
  precise legal lineage for `principles.md`'s **"no right to remove the rights of others"** and the
  private-benefit/public-detriment line. Reinforces the design-imperative body (§22). The crypto-wars
  history (PGP/Bernstein/Barlow/Diffie-Hellman) is "why"-grounding for the same body.

- **FACTCHECK (Gemini, heavily verified — better than feared):** isolated drift caught in the
  `-FACTCHECK.md` companion — a **fabricated Zimmermann "Stalin" quote** (not in his 1993 testimony),
  **"Voskop"** (no such Matrix term; real = Megolm / Vodozemac), the **Meyer-letter exact quote** (real
  event, July 1977, quote unsourced), and **Pear** over-described (a dev platform on the *Bare* runtime,
  not a browser-replacing core; Keet's transport is Hypercore). Several historical quote-wordings are
  UNVERIFIABLE (Keane letter; Zuboff/Acquisti/Solove). **None affect the design payload.** iroh/atproto
  facts: cite the project FACTCHECK; this dialogue does not re-introduce the MST/"Keen" errors.

## 26. AT Proto / PDS / Germ / private-data dialogue ↔ the lineage-groups differentiation + the blind-broker line

- **SHARPENS Croft's differentiation (no drift; better-evidenced):** a **real, community-led ATProto
  Private Data Working Group** now exists (atproto.wiki / discourse.atprotocol.community, Boris Mann;
  GitHub #3363 "Namespaces"→"buckets/realms", #121 "Encryption for private content"; Paul Frazee
  *informally*). It is converging on **access-controlled, PDS-gated** private data — the **PDS as a
  trusted agent (like a browser)** — and explicitly treats **true E2EE / zero-knowledge as the deferred,
  harder problem.** That is exactly the seam Croft's **lineage-groups MLS proof** occupies: real E2EE
  group state that *doesn't trust the host*. So the corpus headline "no native AT-Proto E2EE; real E2EE
  is third-party" **stays true and is now better-evidenced**, and the trusted-PDS-vs-zero-knowledge
  debate maps onto Croft's **blind-broker / content-blind-mule** stance (Croft sits on the
  zero-knowledge side the atproto core team is reluctant to take). Source:
  `seeds/transcripts/raw/croft-atproto-pds-germ-privatedata-dialogue-2026-06-22.md` + `-FACTCHECK.md`.

- **UPDATES the standing source of truth (not a contradiction):** the real private-data WG refines the
  "no native AT-Proto E2EE / *fictional* 'AT Messaging' working group (REFUTED)" note in
  `atproto-atmospheric-web-iroh-mobile-FACTCHECK.md` — a **dated addendum was added there** so
  cite-don't-re-verify stays correct. The refuted thing (a Bluesky-chartered MLS-standardizing
  "AT Messaging" body) is still fictional; the real WG is community-led and about access control, with
  E2EE later. **MST confirmed as atproto's structure** (aligns with the prior iroh-conflation correction).

- **UPDATES the Germ ECOSYSTEM row (matured):** Germ graduated from beta to *the first native-launched
  private messenger from a Bluesky profile* (2026-02-18): Mark Xue (ex-Apple iMessage/FaceTime);
  open-source **Autonomous Communicator (AC) Protocol** on MLS; IETF **`draft-xue-distributed-mls`**
  (IETF 124, "TwoMLS", Naval Postgraduate School); Protocol Labs Cypherpunk Fellowship; identity bound
  via an **"Anchor Key" published in the atproto profile**. The Anchor-Key-in-profile trick rhymes with
  Croft's cross-platform-identity-provenance work (publishing an anchor in a public profile field).
  ECOSYSTEM §6 Germ row enriched. Overlaps the §25 crypto-wars intake (Germ as MLS prior art) — same
  cousin, complementary detail; no duplication.

- **NEW idea for the blind-broker line (explore, not verified):** the **PDS-as-selective-file-proxy**
  pattern — serve a blob the network believes is PDS-native while the bytes live in your own object
  store, zero duplication (reverse-proxy `getBlob` interceptor + a blob-row in the PDS DB) — rhymes with
  Croft's **content-blind mule** and the `encrypted-blob-share` experiment. But it is an unverified,
  Gemini-self-described-**fragile** recipe (depends on the PDS's internal SQLite schema). Treat as an
  [explore] prototype, not a recipe. Tracked ROADMAP_TODO.

- **FACTCHECK (Gemini, heavily verified — strong):** federation numbers (100 accts; 2,600/hr, 50/s
  burst; 21,000/day) and the May-2024 DM date are **exact**; the architecture (PDS/Relay/AppView, MST,
  CID, public-by-default) and the #3363/#121 design discussions are primary-source-confirmed. Isolated
  drift in the `-FACTCHECK.md`: invented `ger.mx`, wrong draft name "distributed-mls-id", unverified
  `/android-waitlist`, the WG over-claimed as "officially formed" (it's community-led), a false **Vultr
  1-Click PDS app** (the real one is **DigitalOcean**), and a clean Gemini **miss** (peers.org *was* a
  real, now-wound-down sharing-economy org — mildly relevant to Croft's cooperative lineage). Pricing
  tables are volatile — not enshrined.

- **DISTILLED (2026-06-22):** analysis at `research/atproto-private-data-architecture.md` (the
  trusted-PDS-vs-ZK / cheap-self-host / key-revocation contentions, the Germ Anchor-Key-in-profile
  idiom, the file-proxy idea); related projects/tools registered in `ECOSYSTEM.md` §5e; and the
  **differentiation framing backported** into `crystallized/principles.md` ("the recurring inversion"
  now carries the host-untrusted/zero-knowledge note — Croft sits on the ZK side atproto's WG declines).
  Remaining open work is only the **E23 file-proxy prototype** (unverified idea).

---

## 27. AT Proto architecture explainer ↔ the settled atproto mechanics (restatement) + the relay-economics update

- **MOSTLY RESTATES, doesn't add (low distill yield):** this Gemini explainer (AppView/PDS/Relay,
  Lexicon=schema, did:web vs did:plc, CAR/DAG-CBOR, rev/seq dedup, `requestCrawl`, feed-gen skeleton
  vs AppView hydration) is a **teaching pass over facts already settled** in the corpus
  (`thinking/atproto-atmospheric-web.md`, `plc-identity-resilience.md`,
  `cross-platform-identity-provenance.md`, and the atproto source-of-truth FACTCHECK). No new design.
  Filed for provenance, not for distillation. Source:
  `seeds/transcripts/raw/atproto-architecture-appview-relay-explainer-2026-06-22.md` + `-FACTCHECK.md`.

- **UPDATES the source of truth (Addendum 2):** three web-verified items folded into
  `atproto-atmospheric-web-iroh-mobile-FACTCHECK.md` — **(1)** `did:plc` = **"Public Ledger of
  Credentials"** (the explainer's "Public Liaison Corporation" is a **fabrication** — REFUTED);
  **(2)** the relay is **non-archival since Sync v1.1 (2025)** — a current full-network relay is
  **~2 vCPU / 12 GB RAM / ~$34-month / Raspberry-Pi-capable** *because* it no longer stores every
  repo (keeps a configurable backfill window), so "the relay holds a full backup of every repo" is the
  superseded **legacy (BGS)** model; **(3)** **Tap** is a real official Go repo-sync/backfill tool.

- **INFORMS existing items (no new seam):** the relay-economics fact bears on **ROADMAP_TODO B5**
  (relay capacity ceilings — the cheap-but-non-archival relay is the current reality) and the
  **operator-relay tier** in `open-considerations.md` §8 / E20 (running a Croft relay is genuinely
  ~$34/mo cheap). **Tap** is registered as atproto tooling prior art (ECOSYSTEM §5b) — useful if Croft
  builds any AppView/indexer/backfill. The relay figure also refines the "Zeppelin 16 TB full AppView"
  number in the source-of-truth Cluster 2 (a *full AppView* is heavy; a *relay* is now light).

- **FACTCHECK (Gemini — unusually accurate):** beyond the did:plc fabrication and the legacy-relay
  framing, the mechanics are correct (MST *is* atproto's structure — no drift); `atblueprints/awesome-atproto`
  exists but the active list is `awesome-atproto/awesome-atproto` (beeman's was archived Jul 2025).

---

## 28. Solid / WebID / Scaling-Trust / DSNP dialogue ↔ the ecosystem register + Croft's positioning between the poles

- **LANDSCAPE, not design (register, don't distill):** this Gemini explainer compares **Solid**
  (Berners-Lee Pods/WebID/Solid-OIDC) and **DSNP** (Project Liberty's blockchain social-graph) to the
  Bluesky PDS, plus the Atlantic Council **"Scaling Trust on the Web"** report. All real, all
  accurately described (FACTCHECK: no fabrications). Filed for the ecosystem register, not for a
  thinking doc. Source: `seeds/transcripts/raw/solid-pds-webid-scalingtrust-dsnp-dialogue-2026-06-22.md` + `-FACTCHECK.md`.

- **SHARPENS Croft's positioning (no drift):** three poles of "own your social data" — **Solid** =
  private-by-default, *direct app↔Pod access*, RDF; **atproto/PDS** = public-by-default, *indexed
  pipeline* (Relay→AppView), Lexicons; **DSNP** = *blockchain consensus layer* holding the graph.
  Croft is **none of them and borrows from each**: it rides atproto for public social but adds an
  **E2EE private layer** (lineage-groups MLS) that Solid does with app-mediated ACLs and atproto
  doesn't do natively, and it **rejects the chain** DSNP requires while sharing DSNP's *unbundle the
  social web* + *delegation-without-surrendering-keys* goals (the latter rhymes with Croft's
  capability-delegate primitive). Registered in `ECOSYSTEM.md` §5 (Solid, DSNP rows).

- **"Middleware" ↔ composable moderation (confirms an existing stance):** the Scaling-Trust report's
  **middleware** recommendation (user-chosen third-party moderation/feeds, Fukuyama-style) is the same
  shape as atproto's composable labelers/feed-generators and Croft's **"compute provenance, never
  utility" / moderation-as-a-chosen-lane** principle. No new design — it's external corroboration that
  the chosen-lane approach is where T&S policy thinking is heading. ECOSYSTEM §7 row added.

- **C2PA = media-provenance complement (note, not adopt):** C2PA (Content Credentials) answers
  *"is this asset synthetic?"* at the media layer — **orthogonal** to Croft's *authorship*-provenance
  (who signed this record). Relevant only if Croft ever renders external media; registered ECOSYSTEM §7.

- **FACTCHECK (Gemini — clean):** Solid/WebID/Solid-OIDC/DPoP(RFC 9449)/Inrupt and DSNP (token-free
  core, delegation) all CONFIRMED; **DSNP's reference chain = Frequency (Polkadot)** added (Gemini
  omitted it); "Scaling Trust" date/publisher CONFIRMED, the five recommendations on-topic but
  **exact wording UNVERIFIED** (PDF wouldn't render — not a fabrication); Bluesky "public-by-default
  PDS" is a fair simplification (PARTLY).

## 29. Sovereign PDS/AppView "club" + open-social naming/interop dialogue ↔ the read-layer expression of Croft's principles

- **CONFIRMS a buildable shape of Croft's stance (the user flagged it "esp good"):** owning the
  **AppView** (read/index) + PDS (data/write) for a small group unlocks **private blocking (inbound =
  effective + un-scrapable; outbound = structurally impossible → experience-shaping / "local shadow
  ban")**, off-repo private feeds, encrypted-blob vault, asymmetrical "gated-castle" federation, private
  cooperative Labelers, multi-source AppView (ATProto + AP + Nostr + RSS), and CAR/MST offline mesh. Each
  maps to an existing principle — *experience-shaping ≈ social-layer "visibility sink / structural-not-
  runtime"; off-repo + encrypted blobs ≈ blind-broker / content-blind mule (§26); private Labelers ≈ the
  **geer**; multi-source ≈ honest-seams **ponds** at the index layer.* Distilled to
  `research/atproto-sovereign-appview-club.md`. Source:
  `seeds/transcripts/raw/croft-atproto-sovereign-appview-open-social-dialogue-2026-06-22.md` + `-FACTCHECK.md`.

- **DUPE/FORK consolidation:** the PDS/AppView middle was pasted twice (primary + a re-run that diverged
  at the tail with net-new open-source-client recs + the Twitter Circles trilogy); filed once as the
  superset. **Overlaps** §27 (architecture explainer, PDS/Relay/AppView mechanics), §26 (Germ), §25
  (crypto-wars) — those are cited, not re-derived.

- **CORROBORATES social-layer V3 with a billion-user failure:** **Twitter Circles** (gated public-plane
  posts → leaked when ranking logic changed → killed) + Communities (killed ~May 2026) vs Group DMs/
  XChat (won by being a hard public-or-E2EE binary) is independent proof of *"content is born into a
  visibility regime and cannot silently change it"* and **structural-not-runtime enforcement** — don't
  build a semi-private overlay on a public broadcast plane; make private data *structurally* private.
  ✅ **Backported 2026-06-22** → `thinking/social-layer.md` **invariant S5** ("Private must be
  structural, not a runtime gate — the Twitter Circles lesson").

- **STRATEGIC BACKGROUND (no design change):** **Aggregation Theory** (Thompson) + Bluesky's VC rounds +
  Doctorow's switching-cost enshittification thesis frame Croft's credible-exit / no-data-hostage bet and
  feed the sustainability↔cooperative-mechanism question (open-considerations §8; §25/§26 PDS-economics).

- **ATTENTION-ECONOMY framing is Croft's verbatim:** the user's "Nielsen rating vs Meta manipulation /
  community not extraction" = `principles.md` Tier-1 (non-extraction; user-need-first) + Tier-3
  (shapeability + stability). The dialogue's "calm/dwell-time feed" is the product-layer expression.

- **FACTCHECK (Gemini — unusually accurate):** every named project real and correctly attributed
  (Bridgy Fed/A New Social, Bounce, Groundmist, AppViewLite, Blacksky/rsky-wintermute, Zeppelin, Colibri,
  Ouranos/Heron/atcute, goat/PDS MOOver, Jetstream). Minor drift only: "501(c)(3)" (→ "nonprofit"), "AT
  Community Fund" (→ **Free Our Feeds**), Series B 2025-close/2026-disclose, Communities date ~May 2026,
  Heron "WriteQueue" unverified, Rhizome=stem-not-root. **Honesty caveats carried:** the offline-mesh
  unattended-wake inherits the iroh FACTCHECK's "OS kills background P2P" caveat; the **dual-PDS one-
  identity** is **not native** (sidecar service endpoints / off-repo, not delegate keys); Groundmist's
  "private by default" is intent, not yet security (sync server ships auth-disabled); the SQLite-schema-
  dependent **PDS hacks are fragile** (ties E23).

- **REGISTER + RESERVOIR:** projects registered in `ECOSYSTEM.md` §5f (bridge/AppView/client tooling);
  the open-ecosystem naming sweep (**Till/Tillage** + Commons/Agora/Bazaar/Rhizome/…) filed as a
  **reservoir** in `NAMING.md` (nothing settled — "mostly background and research").

---

## 30. Groundmist / Hive / identity-chain / iroh-games dialogue ↔ the already-filed bodies + the new marketing/quotes reservoir

- **HEAVY OVERLAP, cite-don't-redo:** this sprawling Gemini dialogue restates bodies already filed
  and fact-checked in parallel intakes — the **Sun/Steem/Hive/coops** saga (§ cooperative-social-union),
  **Groundmist/grjte** (§29 sovereign-appview), the **did:webvh↔did:plc identity chain** (`cross-platform-identity-provenance.md`,
  `plc-identity-resilience.md`), and **iroh voice/video + godot-iroh** (`realtime-media-over-iroh.md`,
  iroh-quic dialogue). Those FACTCHECKs are the source of truth. Source + net-new verdicts:
  `seeds/transcripts/raw/groundmist-hive-identity-chain-iroh-games-dialogue-2026-06-22.md` + `-FACTCHECK.md`.

- **NET-NEW corrections (the only things this intake changes):** **(1)** Hard Fork 23 confiscated
  **~$6.3M / 23.6M STEEM**, not the dialogue's "$5M" (64 accounts + Hive-supporters + Notestein
  correct) — update `cooperative-social-union-model.md` if it cites a figure. **(2)** **atproto resolves
  `did:plc` and `did:web` ONLY — NOT `did:key`** (the dialogue lists three); the conclusion holds
  (did:webvh isn't atproto-resolvable → did:plc + bidirectional `alsoKnownAs` is the workaround;
  `equivalentId` is a stronger-but-less-supported form) — note in the identity docs. **(3)** the
  **door-holding "corporation vs person" anecdote** is **attribution-UNVERIFIABLE** (*The Corporation*
  (2003) / Robert Hare are real; the exact exchange is not sourced) — matters for marketing use.

- **NEW prior art (ECOSYSTEM §5d):** all named iroh games/tools are **real** — **libmarathon**
  (Bevy+iroh+gossip+CRDT, the closest gossip-game prior art), **ascii-royale**, **iroh-lan** (the
  "tunnel localhost / emulator-netplay over iroh" pattern), **godot-iroh**, **DataBeam** (croc+sendme).
  Reinforces the iroh-substrate bet at the *app/game* layer.

- **STARTS the marketing/quotes reservoir (`narrative/messaging-and-quotes.md`, new — per user request
  2026-06-22):** this dialogue + a **Euphoria** branding idea seeded a standing **marketing/advertising/
  brand-voice** file with **usage tags** (✅ ours / 📚 cite / ⚠️ clearance / ❓ unverified). Key entries:
  Croft-original lines (*"foundational layer, not the whole house"; "purism builds manifestos, convenience
  builds monopolies"; "you can buy the tokens but you can't buy the crowd"*); the crowding-out theme
  (Gneezy-Rustichini daycare study 📚; Thurlow 1844 ✅PD; the door-holding anecdote ❓; Ariely, Ostrom 📚);
  and the **Euphoria "10 people can feel like the whole world"** tie-in (⚠️ **HBO IP — inspiration only,
  not verbatim commercial**; recontextualize toward closeness; "The Ten" / "small circles, big worlds" are
  our IP-clean paraphrases). Indexed in the discovery README; promote winners into
  `thinking/app/brand-and-voice-notes.md` when the app brand firms up (ROADMAP_TODO C6/A7).

---

## 31. Iroh / QUIC / local-first ecosystem dialogue ↔ the iroh substrate bet

- **CORROBORATES the iroh-substrate choice from the transport up:** the QUIC properties Croft relies on
  (NAT traversal, no head-of-line blocking, TLS-1.3-by-design, EndpointId=pubkey identity, connection
  migration/**multipath via noq**, **QAD**, streams+datagrams) are restated and fact-checked accurate.
  Reinforces `realtime-media-over-iroh.md` and the relay-lab findings.
- **NEW prior art (ECOSYSTEM §5d) — the Automerge+iroh local-first stack:** **Peat/peat-mesh** (Defense
  Unicorns — Iroh+Automerge turnkey w/ BLE fallback, also in §25 crypto-wars), CRDT alternatives
  (iroh-docs / **Loro** / **Y-CRDT** / **Diamond Types**), and a deep app roster (Prime Intellect,
  Tandemn, Bones/godot-iroh, cross.stream, iroh-ssh, Obsiroh, Hubris, Teamtype, Zeco, Dash Chat,
  **Holochain rewriting Kitsune2 onto iroh**). Overlaps §30's iroh-games tail (libmarathon/ascii-royale/
  iroh-lan) — cite, don't redo.
- **FACTCHECK (Gemini — unusually accurate, no fabrications):** fixes only — ALPN is `iroh/automerge/2`
  (not `/iroh-automerge/1`); Huitema is a parallel QUIC-draft co-author, **not** an iroh endorser; Peat's
  ring→aws-lc-rs FIPS detail `[UNVERIFIED]`. iroh source-of-truth unchanged
  (`atproto-atmospheric-web-iroh-mobile-FACTCHECK.md`: iroh 1.0.0; iroh-docs = range-set reconciliation,
  not MST). Source: `seeds/transcripts/raw/iroh-quic-localfirst-ecosystem-dialogue-2026-06-22.md` + `-FACTCHECK.md`.

## 32. Open-social protocols & aggregators dialogue ↔ the open-social landscape + the fork-a-client strategy

- **MAPS the competitive/composable landscape** Croft federates into: Nostr (+ **Blossom** media,
  **Marmot/MLS + White Noise** private groups), Farcaster (Frames, storage rent, **Neynar acq. + $180M
  return** — the SocialFi→infrastructure pivot), Lens (**Mask stewardship**), thirdweb, and the aggregator
  field (**Firefly/Bridgy-Fed/Flare/SkyFeed/Mixpost/CrossPoster**). Overlaps §29 sovereign-appview
  (multi-source AppView, Bridgy Fed) — cite, don't redo.
- **DIRECTLY feeds the "fork an open client" tactic** in `cooperative-social-union-model.md`: the
  forkable aggregators carry real license constraints — **Flare AGPL-3.0** (host-as-service → must
  publish source), **Bridgy Fed CC0** (total freedom), yup-live archive, **SkyFeed EUPL-1.2**. Flag the
  license in any build plan.
- **FACTCHECK (Gemini — solid; the suspect acquisitions CONFIRMED real):** fixes — Farcaster rent ~$7/unit
  not $5; **Clanker = AI token-launchpad, not a "trading tool"**; cumulative rev ~$1.9M peak not $2.8M;
  Clovyr/thirdweb exact prices `[UNVERIFIED]`. ECOSYSTEM rows from this body flagged dialogue-sourced.
  Source: `seeds/transcripts/raw/opensocial-nostr-farcaster-aggregators-dialogue-2026-06-22.md` + `-FACTCHECK.md`.

## 33. Cooperative "Social Union" model ↔ the sustainability-mechanism open item (the existential seam)

- **CLOSES (provisionally) the corpus's biggest open economic question:** `open-considerations.md` §8 +
  ROADMAP_TODO named the **sustainability ↔ cooperative *mechanism*** as existential and unresolved (and
  §25/§26/§29 only *framed* it). This body supplies a concrete, legally-grounded mechanism — a **Missouri
  Chapter 351** Limited Cooperative Association ("Social Union") with multi-class membership, **progressive
  decentralization** (hardcoded founder-sunset), a **capped revenue royalty** (RBF/Demand-Dividend),
  indivisible reserve + internal credits (the anti-Steemit: no speculative token), a **501(c)(3) tandem**
  for grant-funded open-source labor, and **labor-as-a-first-class moral good** (the anti-Ampled). Distilled
  to `thinking/cooperative-social-union-model.md`. Mark OPEN→PARTIALLY-WALKED (legal-review gate remains).
- **LINKS the economic layer to the technical thesis:** federate on **AT Protocol day-one** (import the
  Bluesky graph; own PDS+AppView+Lexicons for the private "cozy union"). The private-union channels are
  exactly where **lineage-groups MLS** (`thesis-lineage-groups.md`) and **group-privacy-lanes** plug in —
  atproto's own **Private Data WG** does in-transit ACLs, *not* E2EE-at-rest (the gap our MLS proof fills;
  see §26). Connects to `social-layer.md` (V3 visibility regimes — §29's Twitter-Circles lesson) and
  `governance-and-survivability.md`.
- **VERIFIED failure-case lineage hardens principles:** Ello (VC capture), Ampled (volunteer burnout),
  Steemit (hyper-financialization), Diaspora (architectural naïveté), **Coomappa** (white-label dependency)
  — all CONFIRMED real — are the anti-patterns the design answers. Feed `crystallized/principles.md`
  (non-extractive / anti-fragile / non-mimicry-moat / labor-as-first-class razors).
- **NOT-LEGAL-ADVICE + FACTCHECK:** the MO Chapter 351 framework is real and accurate, but confirmed-wrong
  specifics must not be relied on without counsel — **§351.1015→§351.1036**, **CA-41 fee $100 not $105**,
  **name reservation $25 not $20**, **Git/Inkscape are SFC not CS&S**, CHS ~$45.6B, DFA=Kansas City *Kansas*.
  Source: `seeds/transcripts/raw/cooperative-social-union-governance-dialogue-2026-06-22.md` + `-FACTCHECK.md`.

---

## 34. Croft etymology + commons-rebellion tradition ↔ the naming rationale (the trap-vs-balance refinement)

- **DEEPENS, does not duplicate, the two earlier crofting files.** `croft-crofting-research.md`
  (de-romanticized Clearances scholarship + the naming argument) and `croft-crofting-narrative.md`
  (the popular re-telling) are Clearances-and-1886 focused. The new dialogue
  (`seeds/transcripts/raw/croft-etymology-enclosure-tradition-dialogue-2026-06-23.md`) is the
  **etymological + commons-pattern** register: the word is Old English and names a *concept* (enclosed
  ground put to use), proven by the MED's four-way sense split + the West Germanic cognates + the
  surname/place-name dominance — *ubiquity, not sole meaning*. Distilled to
  `narrative/verticals/croft-the-name-and-the-commons.md`.
- **REFINES `NAMING.md` (the seam worth logging — both faces are true):** `NAMING.md` foregrounds the
  croft-as-**trap** (engineered undersized holdings → labour sold back to the landlord = the platform
  relationship). The new material foregrounds the croft-as-**balance** (a secure private plot *coupled to
  a retained common grazing* — the inversion of historical Enclosure, which dissolved the common). Held
  together: trap at origin (runrig → clearance), balance at the 1886 settlement (the freeze). The balance
  reading is the *positive* shape the name points at — local-first plot + cooperative commons — and maps
  to `crystallized/principles.md` ("equal in rights, not capabilities"; "no right to remove the rights of
  others") and to **Drystone** (`NAMING.md`). NAMING.md updated with an "Etymology deepens the rationale"
  section pointing here. Status: **CLOSED** (refinement backported).
- **FEEDS the still-unwritten `the-civic-why.md` (verticals #5).** The global-enclosure pattern (Rome,
  Bengal, Stolypin's Russia, Mexico's ejido, the American West) + Ostrom-not-Hardin are exactly that
  vertical's planned spine; this vertical is its etymological front half.
- **Provenance / honesty:** cleaned-paste (§4); historical/lexicographic sources (no atproto/iroh claims,
  so the FACTCHECK source-of-truth doesn't bear). Open `[UNVERIFIED]` items carried to ROADMAP_TODO: the
  exact 1772 Manchester Directory wording (bleaching-vs-farming sense), and two scholarly anchors to read
  (Greer, *Property and Dispossession* 2018; Ostrom, *Governing the Commons* 1990).
- **Companion source added 2026-06-23:** the full public-domain texts of John Clare's enclosure poems
  (*The Mores* / *Remembrances* / *To a Fallen Elm*) — the interior witness the dialogue deferred — filed
  at `seeds/transcripts/raw/croft-clare-enclosure-poems-2026-06-23.md` (separate from the cleaned-paste
  raw, to keep that faithful to its session). Texts editor-dependent; Featherstone background attributed
  not reproduced (copyrighted). *To a Fallen Elm*'s "freedom"-as-enclosers'-cant turn is the tightest
  poetic statement of the inversion this body turns on.

---

## 35. Foundation + IP-stewardship layer + the foundation name ↔ the cooperative mechanism (the IP half of D5/D8)

- **COMPLEMENTS `cooperative-social-union-model.md` (§33) — the two halves of the sustainability answer.**
  That doc is the *economic/governance* mechanism (MO Chapter 351 LCA); the 2026-06-23 dialogue
  (`seeds/transcripts/raw/croft-foundation-coop-ip-naming-dialogue-2026-06-23.md`) supplies the
  *IP-stewardship + foundation* layer: a three-layer **code (AGPL-3.0-or-later + DCO) / brand (mark held
  by a neutral foundation) / coop (free, conditioned mark license; repeatable for many coops)**
  architecture + entity **phasing** (Aspiration as the interim foundation) + **assignment-with-goodwill**
  transfer. Distilled → `thinking/foundation-and-ip-stewardship.md`. Together they answer the
  existential **D5** and partially walk **D8** (the centerless-meets-center seam: legal entity / name
  registrar / the money). Status: **D5 PARTIALLY-WALKED further; D8 PARTIALLY-WALKED** (legal-review +
  entity gates remain, the user's).
- **CLOSES the IP-layer expression of the principles.** The DCO/AGPL lock (no party — incl. a future
  founder/captured board — can relicense or close the source), the foundation-held-mark-licensed-to-coops
  (un-hoardable brand), and the foundation-separate-from-and-named-independently-of the first coop are
  the IP-layer form of `crystallized/principles.md` Tier 1 ("equal in rights, not capabilities"; "no
  right to remove the rights of others") and the candidate non-negotiable "neutral trust anchor."
- **NAMING (decision surfaced, NOT resolved).** Foundation-name leading candidate **Noria** is recorded
  in `NAMING.md` as a **CANDIDATE pending legal clearance, not a decision** ("Watershed" rejected on the
  $1.8B Watershed Technology collision; "Wellspring" on crowding + the billionaire-fund association).
  **Croft (umbrella/app) and Drystone (protocol) remain settled.** The foundation name is deliberately
  mission-flavored, not flagship-fused, to preserve the even-footing neutrality. Exercises A7.
- **Provenance / honesty:** cleaned-paste (§4); **NOT-LEGAL-ADVICE** — all legal/financial specifics are
  dialogue-sourced from web search and require counsel (the user flagged the whole area "pending legal
  advice"). ECOSYSTEM SPI/SFC/Aspiration rows flagged dialogue-sourced. Tracked ROADMAP_TODO **E28**.

---

## 36. Discord money/IPO/onboarding dialogue ↔ the ten-second door (E11), the cooperative extraction-inversion, and membership-vs-access

- **REINFORCES `research/discord-dominance.md` (no drift).** The 2026-06-22 dialogue
  (`seeds/transcripts/raw/croft-discord-money-ipo-onboarding-dialogue-2026-06-22.md`) restates that
  doc's core thesis — **zero-friction "ten-second" joining** is Discord's single biggest advantage, and
  frictionless onboarding vs Sybil resistance is a real tension resolved per-server. Filed as a **dated
  Update** in that doc (don't start a parallel Discord doc), adding net-new IPO figures (confidential
  S-1 Jan 6 2026; ~$725M ARR; $15B target slipping; positive adj-EBITDA) — all **third-party estimates,
  `[UNVERIFIED]`** (Discord is private).
- **FEEDS the cooperative thesis (D5/E25) with a clean counter-illustration.** Net-new framing: Discord's
  **volunteer moderator/contributor labor is uncompensated enterprise value embedded in the IPO
  valuation** (they hold no equity, no governance voice) — the clean illustration of the
  contribution/ownership-decoupled default that the member-owned model
  (`cooperative-social-union-model.md`, `foundation-and-ip-stewardship.md`) inverts. Backs
  `crystallized/principles.md` (non-extractive / labor-as-first-class).
- **GROUNDS the tier-zero deep-link resolver (E11) and adds the membership-model half.** New design note
  `thinking/membership-vs-access-the-public-door.md` decouples **two axes Discord conflates** — *who
  holds a stake* (membership/governance = the pond/infra layer) vs *who can walk in* (access = the
  pad/room layer). A stakeholder can hold open a **public/anonymous door** into a pad while the pond stays
  member-governed; the guest carries **no governance weight → Sybil softens** (spam in a guest room can't
  capture the co-op). This is the access-layer expression of **D9** ("member ≠ governance-constituent")
  and composes with **S3 quiet membership** (`social-layer.md`) and the **geer** (`geer-gating-peer.md`).
  The unsolved protocol question (anonymous guest's first iroh-pad entry as fast as a Discord invite) is
  **E11**; this note fixes the membership *semantics* around it. Status: **CLOSED** (filed; E11/D9 remain
  the open engineering/governance edges).
- **Provenance:** cleaned-paste (§4); no atproto/iroh claims → FACTCHECK doesn't bear. Tracked
  ROADMAP_TODO **E29**.

---

## 37. Drystone peers/rights/capabilities + governance-conflict spec ↔ the meer/geer/revocation thinking + the Matrix close-cousin contrast

The 2026-06-24 Matrix-contrast dialogue (`seeds/transcripts/raw/drystone-peers-rights-governance-matrix-dialogue-2026-06-24.md`)
distilled two Drystone spec sections (`thinking/drystone-spec/section-2-*.md`, `section-x-*.md`). It
**refines several existing thinking docs** — backport-on-promotion, not contradictions:

- **REFINES `thinking/meer-superpeer-design.md`.** The **meer** is recast from a peer *type* to a
  **PeerSet** — a named, pinned capability bundle `floor + requires{availability} + forbids{read}`. It is
  a **full peer with full standing** that satisfies read-your-own-local-history **vacuously** (it holds
  no plaintext of its own); blindness is the *enforced absence of a delegated capability*, not a lesser
  nature. "A meer is really a peer missing a delegation." Status: **CLOSED-as-refinement** (backport the
  PeerSet framing into the meer doc when §2 promotes).
- **REFINES `thinking/geer-gating-peer.md`.** Gating is reframed as a delegated **capability/role**, not a
  species — but it is flagged as the **one capability that bumps the read right** (§2.7 open), the one
  that most needs an explicit `forbids` clause. The **name "geer" is disliked** (ROADMAP_TODO **A13**).
- **REFINES `thinking/revocation-authority.md` + `freshness-signal.md`.** Revocation is an
  **epoch-rotating, expulsion-shaped governance fact** that reuses the §X total order + fold; honest limit
  (protects the future, not the past) is stated as correct, not a shortcoming. Composes with the
  freshness/membership-fresh reasoning.
- **NEW: the governance-conflict model (§X).** Authority = a monotonic **fold over an append-only
  governance log** (no resolved state to reset → **no Matrix-style state reset**); a **timestamp-free
  causal total order** (issuer-rank / precedence / causal-length / BLAKE3 tiebreak); an **unconflictable
  capped root**; the **R1–R6** capability interface; **attributable-acceptance** dichotomy (knowingly-
  vs concurrently-stale, no silent third category); and a **regress-breaking termination** construction
  (causal ordering spine separated from the authority forward-pass). This is the Drystone answer to the
  problem Matrix's State Resolution v2 solves at the cost of a CVE + a 2-week outage.
- **NEW principles surfaced (not yet written → ROADMAP_TODO E30):** `P-Durable-Enablement` ("a feature too
  costly to implement uniformly becomes one users route around"), the **peer-capability-floor** (a node
  that can't meet the floor isn't a lesser peer, it isn't a peer — simplicity-as-formality), and the
  **asymmetry of expressible range** (the flexible model can present as the rigid one, not the reverse;
  checkable in principle). These belong in `crystallized/principles.md` §1 alongside `P-Local-Truth` /
  `P-Peer-Equality` / `P-Knowable-Truth`, which the spec `Realizes` but which **do not yet exist** by
  those names.
- **OPEN / surfaced (don't resolve):** capability mechanism **Track A (Meadowcap) vs Track B (Keyhive)**
  (**A11**); key-custody default **blind-relay vs trusted-delegate** + "does Option-B-as-default rebuild a
  readable homeserver?" (**A12**); the `ENABLING` wire formats (canonical encoding, **frontier-closure
  §X.8.5** = highest divergence risk).
- **Provenance / DRIFT-WATCH:** cleaned-paste (§4). The Matrix/Willow/Meadowcap/Keyhive facts were
  **web-verified in-session only** and are **not yet in the FACTCHECK SoT** — confirm before beta (see the
  manifest 2026-06-24 intake note). One **self-correction** is preserved in the raw (the false "Matrix
  E2EE bilateral-disable" claim → corrected to the one-way encryption latch); **do not re-introduce the
  wrong claim** when distilling. Status: **CLOSED-as-filing** (raw + spec drafts + indexes). **Staged
  for beta as `../beta/OPEN-THREADS.md` T1 (`gated`)** — held out of the settled theme docs until its
  gates clear (DRAFT status, A11/A12/A13, fact-confirmation, the unwritten `P-*` principles), then
  promoted into a theme + a `BETA-ROLLUP.md` row. Tracked ROADMAP_TODO **E30** (+ A11/A12/A13).

## 38. Drystone publication & defensive disclosure ↔ beta 07 Pillar C (the prior-art-vehicle DRIFT) + T1 spec

The 2026-06-24 publication/defensive-disclosure dialogue
(`seeds/transcripts/raw/drystone-publication-defensive-disclosure-dialogue-2026-06-24.md` →
`thinking/drystone-publication-and-defensive-disclosure.md` + the spec scaffold
`thinking/drystone-spec/drystone-spec-v0.1-skeleton.md`) settled *how Drystone goes public* and, in doing so,
**refined a previously-settled beta conclusion**:

- **DRIFT → resolved (beta `07` Pillar C / K9).** `07` C3 recorded the settled prior-art posture as **"IETF
  Internet-Draft first then arXiv."** This dialogue concludes the opposite is correct: the load-bearing
  vehicle is a **Zenodo DOI (CERN-witnessed, third-party custody) + an OpenTimestamps anchor + a public Git
  release**; an **IETF draft is the wrong *first* move and is *more* encumbered** (IETF Trust holds reuse
  rights), a possible later destination only; arXiv is gatekept, Zenodo the better fit. The user confirmed
  Zenodo is the correct conclusion → **folded into `07` C3 + narrative/charter 2026-06-25**; trace in
  `BETA-ROLLUP.md` K-table. Status: **CLOSED (beta refined)**.
- **RESOLVED → CC0 (A14, 2026-06-25).** The *text-license* half is now decided (user-approved): the
  specification text is **CC0 1.0** (over attribution-only CC-BY 4.0 — maximal "no one can claim or restrict
  the idea"). Folded into `07` **C2** + narrative/charter/banner; Apache-2.0 reference-code license
  unchanged. Intersects but does not resolve A1 (MPL/AGPL substrate); **NOT-LEGAL-ADVICE** (attorney review
  of the patent-non-assertion paragraph still advised). Status: **CLOSED (beta updated)**.
- **LINKS `thinking/drystone-spec/` (T1).** The generated **spec v0.1 skeleton** is the overall document
  scaffold the §2/§X deep drafts (E30/T1) slot into, structured **principles-up-front → mechanics-as-outcome**.
  The defensive disclosure is only *enabling* once **§7 Synchronization** is field-by-field — that gates
  minting the v0.1 DOI (do not freeze off the skeleton). Status: **OPEN-as-design** (tracked ROADMAP_TODO E31).
- **Provenance / DRIFT-WATCH:** cleaned-paste (§4). The IETF/RFC/Zenodo/OpenTimestamps/arXiv/IETF-Trust facts
  were **dialogue/web-sourced 2026-06-24, NOT re-verified** — `[UNVERIFIED]`, confirm before they harden
  (manifest 2026-06-24 publication-intake note). iroh consistent with SoT (`1.0.0`). New ECOSYSTEM rows
  (Zenodo, OpenTimestamps, IETF, Malleable Systems §9; p2panda, iroh-rings §1) flagged dialogue-sourced.

## 39. Rights-vs-capabilities definitions ↔ 01 §5 boundary + 01 §6.1 (K6) + the rights-floor / label-not-enforce / survivor re-key

The 2026-06-24 rights-vs-capabilities dialogue
(`seeds/transcripts/raw/rights-vs-capabilities-definitions-dialogue-2026-06-24.md` →
`thinking/rights-vs-capabilities-definitions.md`) **grounds a boundary beta `01` asserts but does not
define**:

- **GROUNDS `beta/01` §5 ("no right to remove the rights of others").** The boundary can't be load-bearing
  until "right" vs "capability" is sharp. The **discriminating test** — a *right* is standing whose removal
  cancels the conditions of its own contestation; a *capability*'s removal leaves contestation intact — is
  the standing-side companion to **01 §6.1**'s already-folded data-plane(capabilities)/control-plane(rights)
  cut (K6). **Folded into 01 §5 2026-06-25 (user-approved, K17)** — a tier-clean paragraph after the boundary
  bullets (discriminating test + four-rights-by-removal + the voice-vs-amplification cut); the two open
  checks below were deliberately left out of the beta narrative. Status: **CLOSED (beta grounded)**.
- **LINKS `beta/05` (rights-floor) / `beta/06` (label-not-enforce) / `beta/04` (survivor re-key).** Tenure =
  the 05 rights-floor (you cannot be cleared because your standing is not held elsewhere). The
  voice-vs-amplification edge case resolves *into* 06's label-not-enforce (declining to relay = a peer's own
  standing, not a voice removal). Two **verify-before-hardening** seams: (1) **is `share` a right or a
  membership-class capability** (→ 07 / cooperative model — name the inviolable floor); (2) **does the 04
  survivor re-key strand `tenure`** (→ name tenure's implementation exception if so). **Staged beta
  `../beta/OPEN-THREADS.md` T21 / T22** (2026-06-26). Status: **OPEN** (ROADMAP_TODO E32 b/c).
- **LINKS `thinking/drystone-spec/` (T1).** Roles decompose as `floor + [capabilities]` (§2 draft); this
  block is the definitional backing the spec's §2.3/§5 cite. The unwritten `P-Peer-Equality` /
  `P-Durable-Enablement` principles (E30/T1) realize the same cut.
- **Provenance:** cleaned-paste (§4); the user's own design reasoning — **no external facts to fact-check**.
  New thinking artifact; no drift. Tracked ROADMAP_TODO **E32**; manifest 2026-06-24 rights-intake note.

---

## 40. Beta 01 → Drystone protocol spec (the why becomes Part 1) + the Drystone/Croft layering cut

The 2026-06-26 voice review of `beta/01` (`beta/thinking/raw/01_beta_review.txt`) drove a reframe: **01's
reasoning becomes Part 1 of a new vendor-neutral Drystone protocol spec** (`beta/drystone-spec/`), and the
alpha `thinking/drystone-spec/` mechanics + `crystallized/CROFT-PROTOCOL.md` become Part 2. This **closes
the §37/§39/T1 seam** that the spec's `P-*` principles were referenced-but-undefined (ROADMAP E30): Part 1
defines `P-Local-Truth` / `P-Knowable-Truth` / `P-Peer-Equality` / `P-Durable-Enablement`.

- **Drystone ≠ Croft (naming cut, user-directed).** Drystone is the protocol; Croft is *one* ecosystem on
  it (app + a Drystone-compliant cooperative hosting Peer/PeerSet), intended not to be the only one;
  IP/marks → an independent foundation (candidate *Noria*, decision-gated). The spec names no ecosystem in
  normative text. **OPEN reconciliation:** the reference impl's signed `croft-*` domain-separation tags
  must become `drystone-*` and be **re-proven** (the tag is signed over). Status: **OPEN** (spec Part 2
  Appendix B).
- **Hash-layer seam.** Part 2 §4 is green-real on **SHA-256**; §7 governance is designed on **BLAKE3**.
  The committed suite must be pinned. Status: **OPEN** (Appendix B).
- **Content cuts from settled 01** (author's review direction): **Socrates** + **Peirce** quotes excluded;
  "2,400 years" de-emphasized; **Ashby gloss + Beer paraphrase** dropped as quotations (real Ashby line
  kept; verbatim Beer **pending a user transcript** — T23). **Hush-A-Phone/Bazelon relocated** out of the
  reasoning layer to `thinking/historical-peer-rights.md` (vendor-neutral historical alignment).
- **Corpus disposition — CLOSED (executed 2026-06-26, user-directed "01 goes away as superseded").**
  `beta/01` **deleted**; README front-anchor + reading order now point at `drystone-spec`; all 26 `01`
  cross-refs rewired (`02`/`03`/`04`/`05`/`07`/OPEN-THREADS → `drystone-spec`, Part 2 §5 for the rights
  set; T21/T22 repointed). `02`–`08` numbering retained for stability; no `01` theme doc. Stale handoff
  prompt banner-retired. Status: **CLOSED**.
- **Beer / algedonic / Cybersyn-OGAS intake — CLOSES T23 and sharpens the spec.** The promised Beer
  source (`seeds/transcripts/raw/beer-algedonic-cybersyn-ogas-dialogue-2026-06-25.md` →
  `thinking/algedonic-and-peerhood-as-adjudication.md`) added: real Beer quotes + the **algedonic** channel
  (Part 1 §3); the **adjudication-locus axis** and **peerhood = where decision rights sit** with the OGAS
  anti-pattern (Part 2 §3.1/§5.2); **label-not-enforce as peerhood-preserving** + hard-stop-as-algedonic
  (Part 2 §7.6/§8); **exit-backed authority** + "what makes a right cost something to violate?" (Part 2
  App-B). This is a **load-bearing refinement to P-Peer-Equality**: you must define the peer (a locus of
  adjudication) before peer rights. **OPEN-THREADS T23 CLOSED.**
- **Provenance:** the user's own design reasoning + already-filed alpha mechanics + the Beer dialogue.
  **[confirm before publish]:** the Beer quotes + Cybersyn/OGAS dates/figures, and the
  Matrix/Willow/Meadowcap/Keyhive comparisons (Part 2 App-B); iroh cites FACTCHECK SoT. Trace in
  `BETA-ROLLUP.md` "01 review → Drystone spec 2026-06-26"; plan
  `plans/2026-06-26-beta-01-review-refinements.md`.

---

## 41. Social graph as the substrate + the storage architecture (the layered intake)

The 2026-06-26 social-graph-substrate dialogue
(`seeds/transcripts/raw/social-graph-substrate-redb-storage-dialogue-2026-06-26.md` →
`thinking/social-graph-as-substrate.md`) develops three things at three layers, kept distinct on purpose:

- **Protocol (Drystone).** The **"equal in rights, not capabilities" social graph is core Drystone** — a
  signed-assertion relationship DAG. Folded into the spec: **trust-vs-provenance** (Part 1 §2.0 — provenance
  is cryptographically certain, "is this key really Anna" is a social/utility call, all trust roots in social
  trust incl. root CAs; vouching is a graded non-transitive signal, never a verdict — PGP's lesson); the
  **recursive principal** + **composition-vs-valuation edges** + per-edge adversarial posture (Part 1 §2.3,
  extending peerhood-as-adjudication / §40); **per-device authorship + lamport + user-principal-as-self-AS**
  credentials + **devices-as-MLS-leaves** (Part 2 §4.5.1); the **declarative-snapshot-as-cache + verifiable
  roll-up + two-tier compaction** (Part 2 §7.3.3). Status: **folded**.
- **App / product (Croft, `08`).** **Social graph as the substrate, chat as a tenant** (group is the durable
  index; attachments are siblings; group≠member-set; implicit/sticky group lifecycle; local-projection-vs-
  shared-anchor; invisible-graph UX). **FOLDED into `08` 2026-06-26 (user-approved "yes we should reframe")**:
  narrative + charter re-anchored; new §1/§1.1/§1.2; §4 re-pointed; establishes rewritten. T26 **PROMOTED**.
  Residual design gates kept open (the group's-face UX on the T25 framework; reconcile sticky lifecycle with
  `06`). Status: **shape settled in 08; residual UX gates open**.
- **Local implementation (redb).** Authoritative assertion-store + governance log + rebuildable redb
  projection (local-first CQRS), behind a typed surface with injected crypto/MLS/blob traits — a **vetted,
  adaptable** component. **Not the protocol.** Build spec at
  `seeds/generated-prompts/redb-social-graph-layer-build-prompt.md`; staged **OPEN-THREADS T25**.
- **Prior art** (Keet/Holepunch chat-bottomed-vs-graph-bottomed; ATProto public-follow-graph; Gun/OrbitDB):
  candidate ECOSYSTEM additions, **[confirm before publish]**.
- **Provenance:** the user's own design reasoning + web-searched facts (Keet/ATProto/redb/Automerge/MLS RFC
  9420/9750) **not yet in FACTCHECK SoT** → **[confirm before publish]**. Trace in `BETA-ROLLUP.md`
  "Social-graph-as-substrate / storage-architecture intake 2026-06-26".

---

## 42. Field-trades fact-check + the decentralized-MLS ordering tension (corrects 03; sharpens the spec)

The 2026-06-26 adversarial field-check
(`seeds/transcripts/raw/field-trades-four-property-impossibility-dmls-and-redb-dialogue-2026-06-26.md` →
`thinking/field-trades-and-the-ordering-tension.md`) tested `beta/03`'s framing. All facts **[confirm
before publish]**.

- **Corrections folded into `03`** (over-claims softened / outdated facts fixed): **Signal** is phone-rooted
  only at *registration* (usernames 2024); **Delta Chat** under chatmail no longer leaks email metadata
  (RFC 9788 Header Protection 2.48+) — only a relational residue; the **four-property "impossibility" is an
  engineering tension with a quantified FS cost**, not an impossibility — which **strengthens** the
  honest-trades framing. SSB + Briar confirmed. Status: **folded (flagged)**.
- **The decentralized-MLS ordering tension** (the spec's own design space): removing the Delivery Service →
  forks → out-of-order commits **degrade forward secrecy** (retain key material). **DMLS/FREEK**
  (puncturable PRF, storage cost ∝ fork frequency) and **`draft-xue-distributed-mls`** are sibling
  serverless approaches; **every production MLS is server-ordered** (no shipped escape). Routed → **T29**
  (MLS↔governance-log binding), **`drystone-spec` Part 2 App-A.1** (related work), **ECOSYSTEM §2**.
  **Load-bearing consequence:** Drystone's fork/reconcile + survivor/re-key model carries a **real FS cost**
  (the price of holding the tie-break window open) — couples **T22**. Status: **OPEN** (design against or
  adopt FREEK).
- **redb** facts (1.0/savepoints/MVCC/per-txn durability/stable file format) **confirmed** the build-prompt
  reliance (3.x API still web-sourced).
- **Provenance:** web-verified-in-dialogue; `BETA-ROLLUP.md` "Field-trades fact-check + DMLS/FREEK intake
  2026-06-26"; couples §37/§41 (Drystone governance / spec).

---

## 43. Croft.ing website / plot-tender dialogue = the product realization of the atmospheric-web thread

The 2026-07-10 design dialogue
(`seeds/transcripts/raw/croft-website-plot-tender-design-dialogue-2026-07-10.md`) is the concrete
**product/website realization** of the atmospheric-web thread — it takes "GeoCities-2027-on-atproto"
(E1/E2/E3, `thinking/atproto-atmospheric-web.md`, §17) and makes it **Croft.ing** specifically. Distilled
**directly into the beta layers** (user's call; recorded in `LAYER-ROLLUP.md`), not through the usual
alpha-first staging.

- **Loose-end closed/advanced.** E1 ("atmospheric web product surface") was an abstract demand-side
  framing; this dialogue gives it a concrete shape: Plot·Shed·Gate, the single-renderer "safety deposit
  box" (`plot.croft.ing?user=` = `name.croft.ing`), and the serverless social engine (likes = footprints,
  replies = guestbook, PDS blobs = media, CSS-as-JSON = themes) on an Anchor Post. Marks E1
  **PARTIALLY CLOSED** (product surface now specified), advances **C6** ("grow your own" brand naming) and
  the **T4** brand gate (a second logo direction — the CROFT.ing wall-to-gate wordmark — now sits beside
  the drystone-stacking badge; primacy unsettled). Landed → `beta/croft/croft-ing-the-website-and-the-plot.md`
  + `beta/socialization/` (visual-identity, logo).
- **DUPLICATION watch (managed, not a defect).** Heavy thematic overlap with the earlier atmospheric-web
  dialogue (§17) and its FACTCHECK SoT — the durable GeoCities-successor argument, the no-native-atproto-E2EE
  point, PDS/AppView mechanics. Handled by **citing the FACTCHECK SoT and not re-deriving** the settled
  atproto facts; the new doc adds only the *product* surfacing on top. The rug-pull-graveyard argument
  (GeoCities/MySpace/Cohost) is the demand-side "why serverless-and-sovereign," complementary to §17's
  durability framing, not a contradiction.
- **New ECOSYSTEM prior art (dialogue-sourced).** `spores.garden`/Hypha Co-op (DID-derived palette + flower
  identicons, "plant flowers"), Neocities, Linkat, Bio.blue, kibun.social, and Cohost (defunct, cautionary)
  → `ECOSYSTEM.md` §5c-2, flagged `[UNVERIFIED — dialogue-sourced 2026-07-10]`; Standard.site (§5b, verified)
  named as the adoptable `site.standard.document` blogging lexicon.
- **Open (surfaced, not resolved).** The abuse/moderation model for public-XRPC-write widgets; the
  user-HTML/CSS sandbox-safety posture (the dialogue hand-waves "no JavaScript"); subdomain-mapping
  automation. Tracked as **ROADMAP_TODO E33**.
- **Provenance:** cleaned-paste §4 (image turns preserved as prompt-carrying pointers); atproto facts cite
  the FACTCHECK SoT, genuinely new claims flagged **[confirm before publish]**;
  `RAW-ARTIFACTS-MANIFEST.md` "2026-07-10 intake"; `LAYER-ROLLUP.md` "Direct-to-beta intake (2026-07-10)".
  Couples §17 (atmospheric web), §18 (the app/client layer).

---

## 44. Use-case-fit / lexicon-governance / kindred-work session (2026-07) — mostly duplication, three genuinely-new artifacts

A multi-thread claude.ai session (raw:
`seeds/transcripts/raw/croft-usecase-lexicon-glossary-landing-session-2026-07.md`) that was
**largely already filed** by the time it was reviewed. Recorded here so a future paste of the same
material is not re-filed.

- **DUPLICATION (already filed, not re-filed).** The **PUBLICATIONS** design doc → authoritative at
  `experiments/appview-infra/PUBLICATIONS.md` (RUN-18 landed; the pasted brief-payload variant is in
  git history, commit 265f875). The **lexicon engagement brief + five open calls (EL OC-1…OC-5)** →
  `experiments/attest-family/ENGAGE-LEX.md` (RUN-LEX-01 EXECUTED; `experiments/lexicon-community/`).
  The **crofting/drystone "plot tended with care / no mortar / no VC glue" philosophy** → already in
  `NAMING.md`, `../beta/socialization/logo-drystone-stacking.md`, the `../beta/history/`
  enclosure-inversion pair, and the naming + website-plot-tender raw dialogues. The **Lexicon
  Lenses / Polite Goshawk** seam → `experiments/lexicon-community/LENS-SEAM-WORKED-EXAMPLE.md`.
- **Loose-end closed: use-case fit.** The dating/friendship/meetup research
  (`research/dating-friendship-meetup-fit-2026-07.md`) answers the use-case-fit question left open by
  `../beta/cairn/social-lexicon-group-research-brief.md` — does this domain fit the group substrate.
  Verdict: **meetup/community is the recommended beachhead** (roster/governance/auditable-counts map
  ~1:1 onto documented Meetup-under-Bending-Spoons pain); dating's core loop is utility-shaped
  (matching/anonymity/safety), a narrow trust-sidecar only after a real private layer. Surfaces
  ROADMAP_TODO D11 (beachhead direction). Couples PUBLICATIONS/GROUPS (the substrate it maps onto).
- **New governance-body depth (grounded-from-site).** `../beta/cairn/lexicon-community-governance.md`
  adds the lexicon.community *organization/governance* synthesis (7-member TSC, WG chartering,
  incident-report transparency, credible-exit, door/seam/resonance) that the existing NSID-mechanics
  stone and the register row did not carry; the register row now points at both. Pending
  governance-repo verification (ROADMAP_TODO C-item).
- **New thread (OPEN): cairn orientation taxonomy.** Per the user, cairn stones split three ways —
  **Drystone-oriented** / **Croft-oriented** / **related-ecosystem** — a distinction not previously
  labeled. The convention + key is now in `../beta/cairn/README.md`; only the newest stone
  (`lexicon-community-governance.md` = related-ecosystem) is tagged. Tagging the existing stones is a
  deferred review pass (ROADMAP_TODO C-item).
- **New standing practice: kindred-work glossary.** `../beta/socialization/kindred-work.md` (11
  people, 7 orgs) is an influences-and-kindred-work edge list; its capture-at-session-time procedure
  is folded into PLAYBOOK §3. Outreach status stays out of the public repo, keyed by the slugs.
- **Provenance:** cleaned-paste §4 (dupes pointed-to, not re-embedded; UI chrome stripped);
  `crofting.zip` frozen at `seeds/crofting.zip` + `seeds/crofting-unpacked/` (byte-verified). The
  croft.ing landing-page build (RUN-01…03 in the zip) is a separate work stream against
  `CroftCommunity/crofting_site`, not this repo. `RAW-ARTIFACTS-MANIFEST.md` "2026-07 intake".

---

## 45. Multi-persona / "knows-you" social-graph idea ↔ the built attest lane (mostly realized) + one new design sliver

The 2026-07 design note on multi-persona graphs (work/home, school/home hard splits), choosing what to
advertise, hiding the broader graph, and "someone asserting that they know you" as a verified credential
paired with a scoped descriptor ("friend from school"). Pasted alongside the RUN-ATTEST-04 session and
`graph.zip` (the RUN-ATTEST-01…04 briefs).

- **~95% already built/filed (DUPLICATION).** The attestation half is a shipped primitive family:
  `experiments/attest-family/PRIMITIVES-ATTEST.md` — **attestation** = the mutual co-signed edge ("the
  friend case"), **vouch**, **scope** = the "friend from school" descriptor, **resolvability** = per-persona
  visibility control (graded-resolvability default V4, motivated by workplace personas), **anchor persona**
  = the hard-split multi-persona case (work/home). Ten of ten owner verdicts settled (RUN-ATTEST-04;
  summaries at `experiments/RUN-ATTEST-0{1,2,3,4}-SUMMARY.md`). The visibility/multi-persona half is the
  standing `thinking/social-layer.md` thesis (S3 quiet membership, S4 multi-identity, §3 label layer,
  §7 per-connection advertise controls). The user's own session says it went "from a conversation about
  'friend from school' to settled, tested primitive language in two days."
- **The one genuinely-new sliver (captured).** The *utility-representative graph vs happenstance* framing
  and the fatigue concern — attestations should surface as a **permanent, low-noise overlay** that informs
  only on genuinely meaningful connection, never auto-inferring meaning from broad-strokes overlap (same
  high school + volleyball league, no real overlap = fine). Filed as a new open-design question in
  `thinking/social-layer.md` §7, cross-referencing the built attest primitives; ROADMAP_TODO E40.
- **Provenance:** `graph.zip` frozen at `seeds/graph.zip` + `seeds/graph-unpacked/` (byte-verified);
  brief content not re-filed (summaries already exist). See `RAW-ARTIFACTS-MANIFEST.md` "2026-07 intake".

---

## 46. Stellin name-clearance ↔ NAMING + the built appview-infra feasibility

The Stellin naming/clearance thread (2026-07): **Stellin** as the front-runner name for the
LinkedIn-alternative professional-networking pad, branded "Stellin by Croft" on stellin.app.

- **New (filed).** `research/stellin-name-clearance-2026-07.md` — the clearance/collision report
  (no prior clearance doc existed). Verdict **CONTESTED, not blocked**: no dominant software
  incumbent, but a one-letter SEO/phonetic adjacency to Vaonis **"Stellina" / "Stellinapp"**, a live
  exact-name Italian machinery firm (unrelated class), and the **.app HSTS/HTTPS** hard requirement.
  **Live RDAP / USPTO / EUIPO / Bluesky checks could not be verified across two passes** — a
  primary-source re-run is the gate before filing/purchase (ROADMAP_TODO A18).
- **Naming captured.** `NAMING.md` now carries the Stellin app-layer entry (Scots "stell"/"stellin"
  etymology, the drystone-register + stella/star-register duality, NSID strategy `app.stellin.*` vs
  `ing.croft.stellin.*`), flagged **front-runner PENDING CLEARANCE** — analogous to Noria at the
  foundation layer; do not propagate into durable structure until the live clearance clears.
- **DUPLICATION / already-built.** `stellin.zip` = the appview/protocol RUN briefs (14–19); their
  summaries are all filed under `experiments/` — frozen as a seed, content not re-filed. The
  **feasibility half** the user asked about (custom AppView vs Bluesky primitives) is answered by
  **RUN-14 "Stellin AppView caller-identity spikes"** + RUN-15 appview-infra + `appview-infra/kit/`:
  a thin custom AppView on Bluesky identity, not a full-network mirror; the caller-identity +
  offer-gating seam is proven. An earlier name idea "guilds" was set aside.
- **Provenance:** `seeds/stellin.zip` + `seeds/stellin-unpacked/` (byte-verified); report is
  content-faithful with the "could not verify" caveats intact. `RAW-ARTIFACTS-MANIFEST.md`
  "2026-07 intake".

---

## 47. PDS-as-history-backend thread ↔ RUN-HIST-01 (modeled) + RUN-HIST-02 rev B (live, merged)

The 2026-07 design thread on storing the history-reconciliation dataset on an atproto PDS behind a
thin fetch/caching layer (personal deep-history tier; group-write / permissioned-data-bucket shapes).

- **Mostly already designed/built.** The mechanical seam is modeled in **RUN-HIST-01**
  (`experiments/hist-atproto-spike/`, Part B: envelope↔record lossless, rkey order, gap detection,
  order-independent fold) against `HIST-ATPROTO-MATCHUP.md` (Part A) and
  `beta/impl/drystone-design/history-durability.md` (§G/§I/§J/§L). Single-writer-per-repo,
  retention/deletion-is-the-holder's (degrades completeness, not correctness), and public-by-default
  are the known seams; **permissioned-data** is cairned (ATTEST-ATPROTO-MATCHUP, atproto-ecosystem).
- **The canonical-form seam — SETTLED LIVE (verified on main).** The question of whether a repo
  record's **CID can serve directly as a Drystone reconciliation byte-head or needs a re-hash** (the
  atproto-canonical-dag-cbor vs Drystone-canonical comparison; HS OC-2) was **tested live and
  answered GREEN**: the CID serves directly with **zero re-hash**, provided the encoder honors two
  atproto data-model rules (map keys sorted length-then-bytewise-lex; `$bytes`→Bytes, `$link`→CBOR
  tag 42); a non-atproto-canonical downstream gets a divergent CID and must re-hash. Evidence:
  `spike/hist_live/HIST-LIVE-RESULTS.md` §E1 + `evidence/live/e1_cid_identity.json` + `canonical.rs`.
- **RUN-HIST-02 rev B — EXECUTED + MERGED (verified on main).** The LIVE proof landed at
  **`spike/hist_live/`** (branch `claude/hist-atproto-live-gentle`, PR #30 → main), not pending. It
  ran the gentle profile (one throwaway bsky.social account + app password, 100-record/3-blob budget,
  1 rps, full teardown + CAR archive), settled E1/OC-2 (above), found `rkeyStart/rkeyEnd`
  deprecated/non-filtering (`sync.getBlocks(cids)` is the "name the bounding digests" primitive, not
  `listRecords`), and validated the **personal deep-history tier** (repo commit authenticates present
  state; the local reference tail authenticates the offloaded archive; PDS = cold storage with
  cryptographic receipts). Remaining OCs (repo ownership, scribe custody / PLC rotation,
  sealed-posture backend) still owner-walked. The `pdshistory.zip` briefs are the *plan* behind this
  merged run. Tracked ROADMAP_TODO B16 (✅ DONE).
- **Not filed (dupe).** The app-password-vs-OAuth guidance in the same thread is general Bluesky
  knowledge; `skylite/docs/custody.md` already covers custody/auth. Provenance:
  `seeds/pdshistory.zip` + unpacked (byte-verified); `RAW-ARTIFACTS-MANIFEST.md` "2026-07 intake".

---

## 48. ActivityPub-vs-ATProto / Mastodon-de-facto-standard paste — one new cairn stone, rest already on main

The 2026-07 paste with the Mastodon vs ActivityPub vs ATProto explainer, the RUN-AP-01 / RUN-HIST-01
briefs, the Berjon ap-at request, and the live-spike log.

- **DUPLICATION (verified on main, not re-filed).** RUN-AP-01 → `experiments/ap-ambassador/` (crate +
  AP-AMBASSADOR.md + FINDINGS-AP.md + summary; the five AP-V verdicts and the Berjon-triggered
  role-boundary captured). RUN-HIST-01 → `hist-atproto-spike/` + `HIST-ATPROTO-MATCHUP.md`.
  RUN-HIST-02 rev B → `spike/hist_live/` (merged; see §47). All confirmed present via `git ls-files` +
  `git log` before concluding dupe.
- **New (filed).** `beta/cairn/activitypub-atproto-and-the-defacto-standard.md` (orientation:
  related-ecosystem) — the AP-vs-ATProto layering, Mastodon-as-de-facto-AP (WebFinger/HTTP-sigs/
  FEP/REST-not-C2S), and the **HTML living-standard cautionary lesson** for Drystone (an underspecified
  protocol whose reference deployment becomes normative — the steer is spec/conformance rigor before a
  dominant impl sets the wire behavior). `grep` confirmed none of this was in `cairn/` or `thinking/`;
  it complements `atmospheric-web-and-aggregators.md` (fee/aggregator angle), not duplicates it.
- **Open item.** Berjon ap-at walkthrough (AP atop an ATProto PDS; indie-AP-with-custom-domains focus)
  → ROADMAP_TODO E41; the stone carries a Berjon section as its landing spot.

---

## How to use this map

When a document says "unproven," "open," "TBD," or "verify later," check here first — the
thing may already be walked out in a proof, a spike, or a later transcript. When a proof
surfaces a finding (like V3's limitation), add a row here and backport it into the relevant
thinking doc so the synthesis never lags the code. This file is the seam-tracker; keep it
current as fragments keep arriving.
