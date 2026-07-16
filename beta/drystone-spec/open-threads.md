# Drystone open threads

`Status: working index. Maintained alongside Part 2 Appendix B.`

This is the cross-cutting working index over the tracked residuals. Part 2 **Appendix B** is the
authoritative in-spec residual list, and where this file and Appendix B disagree on a spec item, Appendix
B governs. What this file adds is consolidation in one place, the build and document-set threads that live
outside the spec, grouping by the kind of action each thread needs, and a priority read. Section
references without a part prefix are Part 2.

## Priority shape

One property is load-bearing and everything else is smaller. The **completeness-ahead beam** is the single
`Load-bearing, unearned` property the governance and scaling claims rest on. The **harness** is the vehicle
that discharges the beam's experimental half and, in the same build, the capped-root soundness question,
the tenure-under-re-key check, and the MLS stress-tests, so building it is the highest-leverage move. The
`[gates-release]` encodings gate a publication-final release and two-implementation interop but not the
design's correctness. The presets are 80/20 polish over settled structure. The document set is near
complete. So the honest next moves, in order: build the harness; within it run the convergence and
gap-detection experiment (the beam) and the capped-root coverage; then pin the `[gates-release]` encodings.

## Weighting follow-up by spec footprint

Each thread carries a **footprint**: its case-insensitive mention count across the complete Part 2 mechanics, taken as a load-bearing proxy. The more places lean on a seam, the more the design depends on resolving it, and the more a follow-up earns its cost. This is a coarse signal to weight the qualitative priority shape above, not a ranking that replaces it. Two cautions: raw term-counts over-weight seams whose terms recur in ordinary protocol discussion (KeyPackage runs far beyond the re-plant question; CRDT beyond the mutable-mode question), and the `[confirm]` figure aggregates many small independent flags rather than one seam. Refresh the counts as the spec settles; this is a snapshot over the current source.

Footprints, heaviest first:

- **Completeness-ahead beam** (§1): 22 mentions across 19 lines. Heaviest by a wide margin, which confirms it as the one load-bearing property and the top priority-shape move: the weighting agrees with the qualitative call.

- **Re-plant and KeyPackage exhaustion** (§2): 26 raw, inflated by generic KeyPackage mentions; the re-plant seam itself is materially smaller than the count.

- **`[gates-release]` wire encodings** (§4): 20, spread across many independent formats. Broad but shallow: many small items, none load-bearing on correctness.

- **Capped-versus-uncapped root soundness** (§2, `[priority]`): 15. The heaviest genuine design-open question after the beam, matching its priority tag.

- **Fork-versus-recreation consequence**: 15, now landed in §7.6.4 (see Recently closed); kept here to show its footprint weight.

- **Mutable-mode read-merge, CRDT family** (§2): 14 raw, inflated by generic CRDT mentions; the actual open family is smaller.

- **Track A/B capability mechanism** (§2): 13, the deferred grant-representation choice under §7.2.

- **Tenure under re-key** (§2): 10, the check the harness runs alongside capped-root coverage.

- **Self-destruct, retention-bounded content** (§2): 9, the smallest named design-open seam.

- **`[confirm]` flags** (§3): 53 aggregate, each individually small, many already narrowed by this session's verify-when-reachable passes.

## 1. The one load-bearing property

- **The completeness-ahead beam (§7.3.3, §7.3.7, §7.3.8, §7.9).** A partitioned node cannot self-certify
  that nothing newer exists. The membership tail-gap is closed by the ceiling (§7.3.5), and §7.4.3's
  generation stamp closes the behind-via-traffic half of detection; the open part is the ahead-tail. The
  content plane's union gap-detectability is the same beam, not a second one, discharged by the one mechanism
  that governance enforcement fails closed on while the content plane only qualifies freshness (§7.9.2).
  `Load-bearing, unearned.` Discharge: the convergence experiment's gap-detection half, plus the four
  checkpoint obligations (completeness-predicate monotonicity statement, liveness-under-partition,
  anti-state-reset safety, fork-composition, whose home is §7.6), plus the concrete freshness primitive
  still owed, a corroboration-on-latest request answered by a governed standard-of-care over head
  attestations from distinct personae. That primitive now has a candidate design in
  `../impl/drystone-design/liveness-freshness.md` (the `LocateLatest` mechanism), still a proposal to talk through.

- **Status discipline (hold these lines; do not upgrade on the strength of a run).** Two statuses stay
  load-bearing and must not drift upward. The fold's order-independence is constructive and exercised (the v2
  run passed causal-precedence, cascade-by-projection, and permutation-invariance against a reference fold)
  but is not established in production. Gap-completeness is `Load-bearing, unearned`, now narrowed and
  mechanized by the ceiling (§7.3.5) and the finality gate (§7.3.8) but not earned. In particular, the
  tail-gap experiment's success meant faithfully reproducing the gap, which is correctly exhibiting an
  unsolved problem and MUST NOT be read as having solved it: the ceiling, the now, and the finality gate are
  the intended resolution and are not yet tested.

## 2. Design decisions still open (need a call or a proof, not a confirmation)

- **Capped-vs-uncapped-root soundness (§7.3, Part 1 §2.3).** The priority security item. Matrix facts are
  confirmed; the Drystone-side work is to state which of backdating, root-forgery, and
  self-demotion-entrenchment, and which compositions, the tests actually exercised, and to frame the target
  as exit-as-remedy-for-capture versus apex-prevents-capture. `Design`, validation item.

- **Tenure under re-key (§5.3).** Does a survivor re-key strand a valid persona. Gates freezing the rights
  set (tenure, voice, exit; `share` dropped) into normative text. Discharge: drive a survivor re-key and
  check a valid persona can re-establish standing from retained lineage. `Design`.

- **KeyPackage-exhaustion seating trilemma (§7.4, §7.6)** and **re-plant seating default at a boundary
  (§7.6.2).** Welcome-seating versus external-commit-seating, with a PCS-integrity hazard on the
  external-commit path. Both likely resolve to posture dials. Open.

- **Mutable-mode read-merge mechanism (§7.7).** Which CRDT family fits which resource class, and the
  payload-CRDT versus governance-routed boundary. The normative guard (a semantic merge is a MUST, an
  application-level last-writer-wins a MUST NOT) holds regardless. Open.

- **Self-destruct specification (§7.7.3, §6.8.4).** Selector signaling, delivery semantics past expiry,
  mask-versus-remove, build-profile legibility. Open thread, pending dedicated investigation.

- **Tier-2 side history: feature or data-model note (§7.8).** Whether a separate-but-inherited side history
  is a first-class named construct or an emergent use of the data model. The central undecided question
  there. `[confirm.]`

- **Two grounding questions.** What makes a right cost something to violate (working answer: the rights
  floor is variety-enabling and therefore system-sustaining, so violation is systemic) and what binds a
  human to a persona (contextual mint-and-bind, not a single primitive). Open questions, not wire gaps.

- **Cross-tier liveness and freshness (candidate in `../impl/drystone-design/liveness-freshness.md`).** The hybrid
  design, opportunistic signal harvest plus the explicit `LocateLatest` ask-for-HEAD, is drafted, with
  solicitation reframed as a local and unenforceable trust dial, governance currency leaning on the sealing
  signatures rather than on solicitation, and the dataplane taking a low local floor. Open sub-questions to
  talk through, full list in its §10: the cryptographic definition of a persona lineage and who authorizes
  device binding; individually signed versus aggregated attestations; the freshness-window basis per plane;
  whether the finality gate's currency check is mandatory for all knobs or only high-stakes ones; a
  partial-confidence state versus strict fail-closed; how opportunistic signals feed a confidence estimate;
  the interaction with the delivery service's single-Commit-per-epoch role; and anti-amplification limits.
  `Design`, discussion candidate.

- **Checkpoint object format and cadence (§7.3.7).** The now's genesis-derivability floor and the
  checkpoint's role as a locally-verified pruning-and-acceleration point are settled; open are the checkpoint
  object's concrete representation, how often one is cut, and how a corroborated checkpoint is attested for
  the optional accept-from-others path. Ties to the completeness beam (§1). `Design`, open.

- **Signature scheme and signing-key-to-persona binding (§4.3, §4.4).** The three signing roles are settled
  in function (fact authorship, a decision as k signed facts, the now attestation); open are the concrete
  signature scheme and how a signing key binds to a persona through its MLS credential, which sits in the
  identity layer alongside tenure-under-re-key (§5.3) and the human-to-persona grounding question above.
  `Design`, open.

- **Per-author counter under fork and merge (§4.3, §7.6).** The per-author sequence orders an author's own
  facts within a lineage; open is its behavior across a fork and a merge, in particular whether counters are
  lineage-scoped, which interacts with continuity-through-merge (A16, §7.6.8). `Design`, open.

- **Escalation detection line: lag versus genuine disagreement (§7.6, §7.3.8).** The escalate-not-adjudicate
  resolution is folded (a baseline disagreement is detected and escalated to the fork, never resolved by the
  fold to a forced winner), and who-lands-where and hold-suspends-enactment are specified (§7.6.6, §7.6.7).
  What is owed is the precise line distinguishing a node that is merely behind, and will converge once a fact
  arrives, from a node that holds the same head and still computes a different outcome, which escalates. The
  rule is to escalate only after nodes can establish they hold the same head and still disagree, and defining
  and testing that threshold reuses the completeness signal of the beam (§1), so "do not over-fork on lag" is
  the same guarantee as "do not finalize on stale state." `Design` for the resolution, `[confirm]` for the
  threshold.

- **The all-members communal asset key and the whole-Group primary namespace (§5.10, §5.11).** §5.11
  resolves the read-scoped case of the communal-namespace seam, the asset key wrapped to a read-Role's folded
  set. The all-members case and the whole-Group primary namespace are not resolved there and may not use the
  same fold-gated provisioning, since the whole-Group key is not gated by a read-Role fold and likely has a
  different shape. `Design`, open. **Reframed (FND-R10-1, RUN-11, per
  `../impl/drystone-design/group-principal-seam.md`).** The group-principal seam brief dissolves the "how does
  the communal-namespace key rotate under churn" framing: a communal namespace has **no shared whole-namespace
  secret to rotate**, so the seam decomposes into per-subspace write authority (§4.5) and the fold-gated asset
  key (§5.11), leaving only a near-free identifier assignment (the genesis hash) plus the primary-versus-secondary
  choice (the brief recommends primary). What stays genuinely open here is the whole-Group primary-namespace
  identifier assignment and the all-members case, **not** a rotating secret. The Meadowcap-composition `[confirm]`
  (§3) is answered affirmatively by the same brief: capability issuance sits beneath asset-key wrapping and does
  not duplicate the fold's authority. **Group-principal identifier construction (§5.2 / §5.10, seam brief E.1).**
  The `SubspaceId` = persona-lineage and `NamespaceId` = genesis-hash `H(tag ‖ group_id)` mappings are shaped;
  the seam's capability re-issue model is exercised `Design`-grade (`group-principal-seam/tests/seam.rs`, RUN-11
  Part 3), and the **client→subspace lineage fold is now `green-real`** — a persona's several real openmls leaves
  fold to one subspace identity via the `Verified` `fold_by_lineage` (`subspace_fold_green_real.rs`, RUN-11
  follow-on). What stays open: the **`SubspaceId` byte encoding** is `[gates-release]` (E.1, Appendix B), and the
  revocation-authority **trust tier** is **I9** (firewall). Tracked as backlog §2e and the EVIDENCE-MAP §2e rows.

- **Durability-tier open items (§6.8.5, §7.7).** Three durability-layer choices are owed, none affecting the
  primitives. The dataplane checkpoint construction, a corroborated checkpoint at a content prune boundary
  analogous to §7.3.3, is required before safe pruning and lazy deep-history loading and is not yet pinned.
  Store admission wants a coarse proof-of-entitlement to talk to a content-blind store at all, since the
  store defers verification and otherwise accepts what it is given, to bound spam and denial of service. And
  reconciliation granularity, whether the store reconciles whole sealed entries or finer content-addressed
  chunks, is a durability-layer choice independent of the primitives. `Design`, open, with the chunking
  granularity `[confirm]`.

- **Range-partitioned steady-state anti-entropy (§6.8.1).** The steady-state half of §6.8.1 (recovering a
  frame dropped between already-connected peers, with no new join) is `Modeled` at loopback (RUN-09 Part
  4): a range-summary compare over the `(device, lamport)` key space detects the gap (invisible to
  live gossip, which carries no per-recipient ack) and a diff-only repair re-converges the folds byte-identically
  with no reconnect and no whole-log re-broadcast. The **range-partitioned production construction** was
  read-then-built (RUN-12 Parts 3a/3b): the Part 3a brief (`beta/impl/drystone-design/rbsr-construction.md`)
  measures **Willow 3d-range versus Negentropy** over this linear key space and recommends the Negentropy-style
  one-dimensional recursive reconciler; Part 3b landed it at loopback (`partitioned_anti_entropy.rs`) — a large
  divergent range repaired in O(log)-ish rounds shipping only the divergence, replacing the whole-set compare.
  `Modeled` (loopback). What stays open: the `[gates-release]` wire fingerprint (Appendix B) and real-transport
  loss (X1); the governance-fact reconciliation surface (§6.8.5/§7.2) is a distinct Willow-shaped 3d key space,
  a separate choice.

- **Total-device-loss recovery: the lock versus the trust (the bannered recovery-anchor decision).**
  Recovery splits into two separable tiers. Tier 1, the lock, is a mechanism (threshold or Shamir shares,
  sealed, released on a predicate, optional timelock); it is buildable now, and the release predicate should
  be a threshold across independent trust domains, never a single gate. Tier 2, the trust, is who and when a
  release is legitimate; this is the genuinely-unsolved-in-general social problem (the TLS
  certificate-chain-versus-issuer analogy). The consequence for the recovery anchor: the mechanism is
  shippable and only the trust predicate is undecided, which is a sharper framing of the open decision than
  backup-versus-delegation. **Direction confirmed 2026-07-07:** build the lock now (threshold across
  independent trust domains) and treat the trust predicate as a per-deployment / per-persona policy rather
  than a protocol constant. **Still an open confirm:** the trust predicate is not yet designed, so this is a
  committed direction, not a settled mechanism. **Tier-1 lock spike landed (RUN-08):**
  `alpha/experiments/bip39-recovery-roundtrip` proves the lock's cheapest first step — a KAT-verified BIP39
  recoveryKey ⇄ 24-word-mnemonic round-trip (incl. checksum-failure negatives) plus a secretbox-wrap of the
  masterKey, all bit-exact; the trust tier (Tier 2) remains the open call. `Design`, open (the trust-predicate
  design is the user's call).

## 3. Confirmations against primaries or stress-tests (`[confirm]`, reasoned not proven)

- **iroh crates past core-1.0 (§6.5, §6.9, §6.10, §10.3).** iroh core 1.0 is verified. The
  separately-versioned crates remain: `iroh-gossip` (the `Event` surface, the absence of per-recipient
  delivery confirmation, HyParView and PlumTree constants) and the address-lookup crates (republish and
  expiry behavior, the Pkarr integrity model). `[confirm.]`

- **MLS hard cases.** Each a specific stress-test, not a design gap: external-join far-behind node
  (§7.4.2), in-place-secret-restore-never (§7.4.2), re-plant-intent-recorded-before-freeze (§7.6.3),
  epoch_authenticator overlap (§10.2.1), resumption-PSK cross-group linking (§10.2.1), and the epoch-number
  metadata leak versus re-plant frequency (§6.4, §7.6.2). `[confirm.]`

- **Dataplane modes (§7.7).** Mode migration between forward-only and Willow-mutable (suspected
  fixed-at-creation) and the size bound between the two modes (an unmeasured engineering estimate).
  `[confirm.]`

- **External facts (§7, Part 1 §3).** The Beer, Cybersyn, and OGAS grounding and the Appendix C comparisons
  are web-verified in source dialogues only, to confirm against primary sources before they harden.
  `[confirm.]`

- **Scaling-analysis externals (§7.9).** Three empirical or secondary items the scaling section rests on,
  none affecting the design's correctness. The roughly two-second commit-serialization window is `Measured`
  from a single study (Soler et al. 2025, OpenMLS, groups to about 5,000) and is not a protocol constant, so
  deployment figures may differ (O3). The small-groups topology win is pending a realistic group-membership
  distribution, since it holds only while groups stay scoped and personae are not in pathologically many
  groups (§7.9.3). And the batched-designated-committer handling of membership bursts is secondary-sourced
  (Webex, Cloudflare, from the research report) and not re-verified against primary sources (§7.9.4).
  `[confirm.]`

- **Asset-keying externals (§5.11, §7.7).** Two checks the read-scoped and mutable planes rest on. Meadowcap
  composition is the decisive one: whether Meadowcap's own communal read-capability issuance composes with
  fold-gated asset-key wrapping without introducing a second authority path, the narrowed form of the §5.10
  Meadowcap-alignment step, reduced to whether capability issuance sits cleanly beneath key wrapping or
  duplicates the authority the fold already holds (`[confirm against willowprotocol.org/specs/meadowcap`]).
  And CRDT-on-Willow efficiency: storing operation-based CRDT change payloads as Willow entries is workable
  but reportedly not efficient, with independent local-first reports favoring chunked compress-then-encrypt
  of change ranges. `[confirm.]`

## 4. Presets and byte encodings (structure settled, values and formats open)

- **Presets (`[confirm]`).** Enactment-dial rung and fallback intervals (§7.3.6), per-archetype posture
  defaults (§7.6.9), the now's wire schema (§7.3.7), tiebreak-key and instance-weighting defaults (§7.3.1),
  and the false-positive escalation tolerance format and guidance (§7.4.1).

- **`[gates-release]` wire encodings.** Eighteen markers gating a publication-final release and
  two-implementation interop: the canonical governance-fact byte encoding (§7.3.1, the base all others
  extend, including the operation-type enumeration and a schema-version marker for evolution), the content-id
  pre-image now that timestamp is removed (§4.2), frontier-commitment and
  acceptance-record (§7.5.1), frontier-closure-and-subgraph-closure before sort (§7.5.2, the highest-risk
  divergence point), the gating-versus-read relationship (§5.8.1), the capability and membership-graph wire
  format (gated on the Track A/B decision), the returning-member (G, D) cursor and checkpoint (§6.6.2,
  §7.4), and the data-plane governance-generation stamp (§7.4.3, counter alone versus full governance head
  hash).

- **Reconciliations, folded into one wire-freeze.** Two signed-encoding changes that both re-open the §4 signature proofs, so they land together, not piecemeal. Vendor-neutral naming: the `croft-*` domain-separation tags move to a `drystone-*` namespace. Hash function: the choice is now decided, BLAKE3 is the committed suite (§7 is designed on it and it matches Willow's Earthstar payload hash, tracking Willow's planned WILLAM3), with SHA-256 the legacy side §4 is proven on today. What remains open is the validation, not the choice: proving §4 out end-to-end on BLAKE3, re-deriving the §4 proofs and re-running the §9 conformance suite on it, after which §4's verified status moves from SHA-256 to BLAKE3.

## 5. Build work (the largest actual effort)

- **The harness.** The v2 convergence experiment ran and passed Stages 1, 2, and 4 on the reference fold.
  Remaining: Stage 3 (adversarial), Stage 5 (real crypto; enactor and commit idempotence is still
  `Design`), Stages 6 through 10 (integration against real iroh and real MLS), Stage 7 (the capped-root
  soundness question in section 2 above), and Stage 8 (the bounded key-recovery primitive, stubbed at
  §7.3.9). Building this is what turns the beam and several `Design` claims into results.

## 6. Document-set threads

- **Elevator pitch.** Marked `Status: proposal`; reconcile with the prior founding tagline and paragraph,
  then drop the proposal marker.

- **Coffee-shop length.** Sits at 985 words, the top of the one-to-one-and-a-half-page range. Open whether
  to trim toward 650 to 700 for a tight single page.

- **Equal-branch examples.** Four now exist (the mute view at §7.6.5, the fork landing at E8 and §7.6.6, the
  re-composition at E10 and §7.6.10). Candidates for more: §5.9 exitability and a partition-view example,
  pending a read on whether four is the right density.

- **The §11-cast chained appendix (Ada/Boreas/Cyrus re-entry and ban arcs) — RESOLVED 2026-07-10.** §11 was
  folded in from a standalone companion and its in-body beats pointed to a "§11 appendix (external to this
  section file)" carrying the chained Ada/Boreas/Cyrus journey that was never reconciled into Part 2's
  appendix structure. It is now written as the **§11 continuation of Appendix E** (beats L1–L6: Ada's steady
  state, Boreas's dormancy and self-service re-entry, Cyrus's ban and the lagging-node heal), the large-group
  parallel to the E1–E10 arc; the in-body §11 beats now point there. No new normative content or actors were
  introduced — the beats chain §11.4/§11.6/§11.7/§11.8 as written.

## 7. Spec-shape and rationale residuals (Phase-0 coverage-audit staging, `needs-content`)

Three residuals from the drystone-spec-layer Phase-0 coverage audit: the reasoning behind the spec's
document shape was executed in beta but never captured in the spec itself, so it is staged here rather than
lost. These are `needs-content` items awaiting a reasoned home in a spec §/appendix, not resolved threads.

- **IETF-norm rationale for the spec's document shape.** Why Drystone is a Part-1-reasoning +
  Part-2-mechanics document with an "Alternatives Considered" appendix is not recorded in the spec; the
  structure was executed in beta but the decision-rationale lived only in the alpha publication dialogue.
  The precedents that drove it: RFC 2360 (a standards-writing guide recommending the standard be split into
  a concise protocol section plus separate explanatory text, so narrative reasoning is a recommended
  technique, not tolerated); the IKEv2 rationale-doc split (a companion document that records design choices,
  the alternatives, and the roads not taken — documenting the changes considered-but-rejected is valued in
  the culture); RFC 6762 (Multicast DNS, the canonical "body says do X, an appendix says why X and not Y"
  demonstration); and RFC 6709 (a design-philosophy document in its own right). The load-bearing principle:
  philosophy lands only when each design principle cashes out into a mechanic, or a reviewer bounces off it
  as free-floating political theory. Three options were weighed — rationale in appendices (RFC 6762 model), a
  substantial "Design Principles" section up front then mechanics, or two separate documents (IKEv2 model) —
  and option 2 was chosen because Drystone's reasoning is generative (it produces the design in front of the
  reader) rather than retrospective, so splitting it would force two files open to understand one idea. `[UNVERIFIED]`
  on the RFC specifics (web-verified in the source dialogue only). Remaining: fold this rationale into a spec
  front-note or an "Alternatives Considered / document shape" appendix so the document shape carries its own
  justification. Source: `../../alpha/seeds/transcripts/raw/drystone-publication-defensive-disclosure-dialogue-2026-06-24.md`
  (~34–129). `needs-content`, open.

- **The Willow-launch interop pre-emption mandate.** The normative section MUST answer "what does
  Drystone-compatible mean, and where exactly does the wire format force two independent implementations to
  agree" — but this is not yet recorded as a standing spec obligation. Interop is tested empirically (the
  `[gates-release]` two-implementation interop check, §4), yet the mandate to answer the question in the spec
  is not itself captured. The reasoning: a commenter on the Willow launch pressed the obvious interop
  question — what value a protocol has if two "compatible" implementations are not actually compatible — and
  this is the sharpest test of whether the philosophy cashes out into mechanics. If peer-equality-in-rights
  is a principle, a reader will want to see exactly where in the wire format two independent implementations
  are forced to honor it. The publication-dialogue skeleton reserved a normative Interoperability section
  (§9 in that sketch) to answer this directly and to show peer-equality-in-rights enforced by mechanism.
  Remaining: record as a spec obligation — a normative Interoperability section defining "Drystone-compatible"
  and pointing at the wire-format agreement points, so the reviewer question is pre-empted rather than left to
  the empirical harness alone. Source: same publication dialogue (~159–164, 533–534). `needs-content`, open.

- **The Matrix one-way-latch correction as durable grounding for `P-Durable-Enablement` (Part 1 §2.4).**
  The originating dialogue asserted Matrix E2EE could be "bilaterally disabled" by a frustrated user,
  degrading the room for everyone; that claim was verified and retracted. Matrix encryption is a **one-way
  latch** — the `m.room.encryption` flag "must not be cleared" by a later event, specifically to defeat a
  MITM downgrade — so the "bilateral disable" mechanism does not exist. The correct antagonist is "an
  immutable channel flag that protects configuration while abandoning access": Matrix guarantees "this room
  is encrypted" while failing "you can read this room." The route-around insight that originates
  `P-Durable-Enablement` is the realized-vs-paper-posture reasoning — even removing the disable option did
  not save the feature, because users route around it by leaving, so a guarantee whose cost forces users to
  abandon it has negative value; only ship a guarantee you can keep enabled by default in the common case.
  This currently survives only as a do-not-reintroduce guard in `beta/CLOSED-THREADS.md` and has no reasoned
  home in the spec's open-items, so the §2.4 principle does not yet carry its Matrix-derived grounding.
  Remaining: fold the corrected grounding into Part 1 §2.4 (`P-Durable-Enablement`) so the principle carries
  its Matrix-derived reasoning, preserving the do-not-reintroduce note (never restate the false "bilateral
  disable" claim). `[confirm]` the §2.4 anchor and the Matrix facts (web-verified in the source dialogue only)
  before they harden. Source: `../../alpha/seeds/transcripts/raw/drystone-peers-rights-governance-matrix-dialogue-2026-06-24.md`
  (~38, 152–154) + `beta/CLOSED-THREADS.md`. `needs-content`, open.

## Recently closed (so they are not re-opened)

- **Fork-versus-recreation consequence enumeration.** Landed in §7.6.4 as a four-way comparison of a Drystone fork against a Matrix room recreation, across history, governance, membership and standing, and provenance of the split. Verified against MSC1501 (the room-upgrade tombstone and predecessor mechanism, with the continuation link being a mutable, power-level-authored pointer) and room version 12 with MSC4291 (room-id as the hash of the create event, creator holding infinite power). The load-bearing point is now precise: a Drystone fork carries full history and an intact governance chain onto every side and leaves every member whole, whereas the Matrix remedy strands prior history behind a mutable pointer, re-establishes governance from a fresh create-event root, and makes continued standing depend on re-admission by the recreator. Was flagged inline in §5.7.

- **Asset-keying file, verified concept by concept.** Walked all sections against Part 2. The read-scoped key
  model (C), fold-gated provisioning (D), and no-continuous-key (I) are in §5.11; the two-anchor content
  header (F) is in §7.4.3; the epoch/governance decoupling and two-phase (H) are in §7.3.6, §7.3.7, and
  §7.9.1; the convergence-model core (E) is in §7.7 and §5.10; the concurrency trace (G) is worked reasoning,
  not spec text. Two settled sub-points were thin and are now folded into §5.11: the re-key trigger rule
  (governance-fold-initiated, never epoch-tick, and the epoch-not-authority-signal) and the
  R5-causal-not-temporal partition safety. E's fuller three-question and four-flavor taxonomy is left
  unfolded to avoid duplicating the §7.7 modes and the §5.10 and §5.11 material. Open items persisted: the
  all-members and whole-Group key (§2), and Meadowcap composition and CRDT-on-Willow efficiency (§3); header
  encoding and hash reconciliation were already in §4.

- **History-durability file, verified concept by concept.** Walked all sections against Part 2; it is
  self-contained with no content gap. The two tiers (D), mirror group and nested sealing (E), the store's
  Role Set (F), and the reconciliation envelope (G) are in §6.8.5; the device-subspace and one-per-subspace
  counter (H) are in §6.8.5 and §5.2; member-to-member and store convergence (I, J) are in §6.8.1 and
  §6.8.5; ordering-is-content (K), pruning-and-checkpoints (L), and Willow and MLS conformance (M) are in
  §7.3.1, §7.7, and the Willow treatment. Open items persisted to §2 as durability-tier items: the dataplane
  checkpoint construction, store admission and spam bound, and store reconciliation granularity. The
  G-hist-to-G resumption-PSK binding is carried inline in §6.8.5 and under the §3 MLS hard cases.

- **Scaling-and-ordering file, verified concept by concept.** Walked the whole file against Part 2's §7.9; it
  is a faithful, complete fold. The four disclaimers, bottleneck one (commit rate is key-change rate, content
  and authority-only governance off the epoch chain, two-phase removal narrowing the collision surface),
  bottleneck two (the DS ordering-versus-fan-out split, ordering relocated not eliminated, the beam, the
  measured window, and the catch-up seam with the ratchet tree not offloaded), the topology, and the honest
  costs are all in §7.9.1 through §7.9.4 with their epistemic tags. §7.9.2 unifies the content-union
  gap-detectability with the governance beam (one mechanism discharges both), so O2 is not a second open
  item. No Part 2 gap. The threads items that were missing are now folded: the content-half beam note into
  §1, and the measured window (O3), the topology win, and the designated-committer industry practice into §3.

- **Governance and enactment file, verified concept by concept.** Walked Part A (A0 through A17) against Part
  2's actual text; all seventeen are folded with reasoning: sign-the-state (§7.3.4), quorum-not-votes
  (§7.3.1), non-exclusive recognition, decision-versus-enactment, the enactment dial, and the two-phase
  interval (§7.3.6), the ceiling (§7.3.5), the now (§7.3.7), the finality gate (§7.3.8), the read/enforce
  best-known-versus-final line (§7.3.3, which closes B2), ban-is-a-fork (§7.6.4), the three registers
  (§7.6.5), who-lands-where (§7.6.6), hold-and-audience-split (§7.6.7), being-in-both and cheap merge
  (§7.6.8), and posture-and-dials (§7.6.9). Part A is self-contained in Part 2 with no gap. Of Part B, B2 is
  folded (A10 into §7.3.3), B3 is the beam (§1), and B4, B5, B6, B9 are tracked in §4; B1's open
  detection-threshold residual and B7's epistemic-status discipline were missing and are now folded into §2
  and §1.

- **Fact-and-chain representation, verified concept by concept.** Walked all eight sections of the companion
  doc against Part 2's actual text. Seven were already folded: §1 and §6 at §7.3.7, §2 and §4 across §4.3,
  §7.3.1, §7.3.4, and §7.3.7, and §3, §7, §8 at §4.6. §5's mechanism was present (§7.3.1, §7.5.2) but its
  reference-form decision was thin, so §4.6 now carries the single-committed-reference-versus-explicit-frontier
  choice, concurrency as a derived property, the forbidden bare-parent-pointer, and the local DAG index, and a
  loose "DAG index of §7.5" reference was corrected. The four open items O1 through O4 were folded into the
  threads above (O1 into the §4 gates-release fact encoding; O2 through O4 into §2).

- **Companion-doc fold (the referenced-but-not-folded reviewer note).** Walked the four companion docs
  against Parts 1 and 2 and folded every genuine miss into the specs, so a reader needs only the two. Added
  §5.11 (read-scoped content keys, per-scope asset keys, fold-gated provisioning, no continuous key
  agreement), §6.8.5 (the content-blind history store: mirror group, nested sealing, Role Set, reconciliation
  envelope, and the rejected third tier), and §4.6 (canonical dag-cbor with the why-not-JSON argument and the
  reconciliation-versus-local-storage decoupling). Byte layouts stay `[gates-release]` where noted. Full
  coverage recorded in `drystone-fold-coverage-audit.md`. Two placement calls left for a possible later pass,
  neither blocking: the key model sits in §5 as enforcement of the §5.5 read role, and the history store sits
  in §6.8 as the untrusted-node case of gap-aware convergence.

- **Transcript-hash construction (§7.6.3).** Read verbatim against RFC 9420 §8.2: a strict single-predecessor
  chain with no branch or merge representation. `Verified-RFC`.

- **The standing-at-vote tally (§7.3.1).** Decided: a vote counts at its own causal position, and the only
  thing that un-counts it is a ban causally prior to it. The tally open-item is removed.

- **The behind-half of the beam (§7.4.3).** The governance-generation stamp makes the behind-via-traffic
  case detectable from the data-plane side.

- **The re-composition rubric (§7.6.10).** Written with the merge-as-view framing and the sliding-scale
  adjudication rubric, MUST and MUST NOT with reasoning, and beat E10.

- **Two Design-to-Modeled upgrades (§7.3.2).** A12 tier-boundary projection consistency and R3
  no-fold-time-rejection with referenced-gap detection, backed by the v2 Stage 1 and Stage 2 results.

- **MLS counter conflation, corrected.** Two distinct MLS counters were being run together. The wire-visible
  one is the epoch (RFC 9750 §8.1.2), a governance-change odometer a passive observer can correlate. The
  gap-detection one is the per-sender generation, which is epoch-scoped, encrypted, and resets each epoch, so
  it does not leak and is not an infinite per-message integer. Both differ from our governance-generation
  stamp, a content-to-authority provenance pointer. Recorded in `../impl/drystone-design/liveness-freshness.md` §2.
