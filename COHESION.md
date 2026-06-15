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

---

## How to use this map

When a document says "unproven," "open," "TBD," or "verify later," check here first — the
thing may already be walked out in a proof, a spike, or a later transcript. When a proof
surfaces a finding (like V3's limitation), add a row here and backport it into the relevant
thinking doc so the synthesis never lags the code. This file is the seam-tracker; keep it
current as fragments keep arriving.
