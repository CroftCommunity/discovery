# Drystone: The range-partitioned RBSR construction — Willow 3d-range versus Negentropy

`Status: read-and-report design brief (RUN-12 Part 3a). Verdict and construction proposed; the Willow and Negentropy primitives are checked against their live published specs this pass (cited with versions/dates below); the Drystone binding is Design, not frozen. No spec, register, or crate is edited by this document.`

`Serves: Part 2 §6.8.1 — the "range-partitioned production form open" residual. Reports what the steady-state anti-entropy repair actually requires over Drystone's (device, lamport) key space, at Drystone's scale and privacy posture; measures the two candidate constructions (Willow 3d-range-based set reconciliation, Negentropy's range-based protocol) against it; recommends one with costs; and lands the RED-able assertion set a build (Part 3b) would ship. Narrows the §6.8.1 residual and the open-threads "Range-partitioned steady-state anti-entropy" thread. Distinct from the governance-fact reconciliation surface (§6.8.5, §7.2), which is a separate, Willow-shaped key space — see §E.`

`Scope: the production construction for §6.8.1's steady-state history/message anti-entropy — the recursive range-partitioned form that replaces the RUN-09 whole-set diff (croft-chat/src/anti_entropy.rs missing_frames). Sits beneath Part 2 §6.8.1 (gap-aware history convergence) and above the transport. Does NOT decide the governance-fact reconciliation construction (§6.8.5/§7.2, Willow-shaped), does NOT pin any wire encoding (that stays [gates-release], Appendix B), and does NOT touch the membership gate (§6.8.1's "the gate is membership").`

This is a read-and-report brief for one open seam: which range-based set reconciliation (RBSR) construction Drystone's §6.8.1 steady-state anti-entropy should adopt for its production form, and the assertions a minimal build would land. Normative keywords are **MUST / SHOULD / MAY** (BCP 14). Each load-bearing claim carries a status flag: `Established` (an inherited primitive or a fact checked against a primary this pass), `[confirm]` (rests on an external fact/version not re-verified beyond the single fetch cited), `Design` (this brief's own binding proposal, judged as reasoning), `Synthesis` (assembled across sources), `[gates-release]` (a wire/byte choice deferred). Vocabulary inherits from `../../drystone-spec/conventions-and-decisions.md`.

> **I9 firewall.** This is a read. It decides no trust tier and freezes nothing. The Willow-versus-Negentropy production choice for the *governance-fact* surface (§E) is left to Appendix B / the owner; this brief recommends only for the §6.8.1 history/message anti-entropy surface it serves.

---

## A. What §6.8.1's steady-state repair actually requires

`Realizes: P-Knowable-Truth`

§6.8.1 is defined once and used at three scopes (C-swarm hole-visibility §6.5.3, D-peer recovery §6.6.3, device-Group sync §6.6.5). The RUN-09 build (`Modeled`/loopback, `steady_state_anti_entropy.rs`) proved the *repair itself* in its simplest whole-set form: `range_summary` collects the set of `(device, lamport)` keys a peer holds, the two summaries are compared, and `missing_frames` ships **only the diff**. The open residual — what this brief resolves — is the **range-partitioned production form**: the whole-set summary is O(n) to exchange when the divergence is small but the sets are large; the production form must **spend bandwidth proportional to the divergence, not the set size**.

What the repair reconciles, precisely, sets the requirement:

1. **A one-dimensional, totally-ordered key space.** The message/history log reconciles over `(device, lamport)` (`assertion_order_key`). Per-device lamport is strictly monotonic (§6.8.1 detect-beat; the fold's Step 5 lamport check, `fold_derived.rs`), so `(device, lamport)` is a total order and each key names exactly one frame. This is a **linear** key space — not the three-dimensional (subspace, path, timestamp) space Willow reconciles. `Established` (against `anti_entropy.rs` + the fold's monotonicity check).

2. **Source-agnostic, self-verifying fill.** Every filled record is accepted only on its author signature and its fold into the hash structure (§6.6.3 self-verifying invariant), never on the partner's word. The reconciliation construction therefore needs **no integrity guarantee of its own** — a lying partner cannot inject a bad record, only waste a round. This widens the acceptable construction set: the fingerprint's job is *efficiency*, not *trust*. `Established` (§6.8.1 "accepted only on its own author signature").

3. **The gate is membership, and reconciliation runs off-fabric.** A partner is reconciled with iff it is an entitled current member; the transport is irrelevant (§6.8.1 "No transport gate; the gate is membership"). The construction **MUST** be transport-agnostic — usable over a direct dial, a meer, or a history store — and **SHOULD** be stateless on the responder side, because a history store (§6.8.2) serves many members and holds no session. `Design` (transport-agnostic + stateless-responder, from §6.8.1/§6.8.2).

4. **Scale target: bounded rounds under a large gap.** M2 (the return-backfill sizing study, EXPERIMENT-BACKLOG §M2) frames gaps at 1/7/30/90 days. A member dormant 90 days over a busy Group returns to a large divergent range. The requirement is **O(log n) rounds** to localize the divergence, not O(n) — the whole-set form's single O(n) exchange is exactly what fails at that scale. The assertion a build lands is **round-count**, not wall-clock. `Design` (the scaling requirement §6.8.1 defers to the production construction).

5. **Privacy posture: fingerprints leak only to the entitled, over content-addressed identifiers.** Because the gate is membership (req. 3), the reconciliation partner is already entitled to the content it reconciles, so a range fingerprint that summarizes *which records a peer holds* leaks set-membership only to a party already inside the read scope. Against the §5.11 read-scoping (the fold-gated asset key: a persona reads content authored while it held the Role), this matters at exactly one seam: a **content-blind history store** (§6.8.2) reconciles *without* read entitlement, over the content-addressed record identifiers (the sealed lamport index is sealed against it, §6.8.1 detect-beat), so its fingerprints must reveal nothing about plaintext and **SHOULD** resist a store crafting identifiers to cancel a victim's records out of a fingerprint (a censorship/omission attack). `Synthesis` (§5.11 read-scoping ∧ §6.8.1 membership gate ∧ §6.8.2 content-blind store).

**Summary of the requirement.** A transport-agnostic, stateless-responder, recursive range reconciler over a **linear totally-ordered key space**, spending bandwidth proportional to the divergence in **O(log n) rounds**, needing no integrity of its own (self-verifying fill), whose fingerprints are safe to expose to entitled members and, at the content-blind-store seam, resist omission attacks.

---

## B. Candidate 1 — Willow 3d Range-Based Set Reconciliation

`Established` primitives, checked against the live spec `https://willowprotocol.org/specs/rbsr/` (fetched 2026-07-16; the spec carries no explicit version stamp on the page, so this is `[confirm]` against a dated snapshot).

- **Fingerprint shape.** A **commutative monoid**: `fingerprint_combine` is required "associative and commutative", with a `fingerprint_neutral` identity and a `fingerprint_finalise` map (pre-fingerprint → fingerprint, "allows for compression"). Commutativity is *required by design* because Willow "do[es] not wish to prescribe how to linearise three-dimensional data into a single order." `Established`.
- **Partition strategy.** Splits a non-equal range into **arbitrarily many** subranges per step ("Splitting into more subsets per step decreases the total number of communication rounds"), and — crucially — "it is crucial for overall efficiency to *not* split based on volume … but to split into subranges in which the peer holds roughly the same number of AuthorisedEntries" (count-balanced boundaries). `Established`.
- **Round complexity.** "Peers collaboratively drill down to the differences … in a **logarithmic number of communication rounds**", spending little bandwidth on regions held in common. `Established`.
- **Termination.** Two paths: fingerprint match → reply an empty entry set; or fingerprint mismatch on a "sufficiently small" range → transmit the items directly (no fixed threshold prescribed). `Established`.
- **Dimensionality.** The construction reconciles **3d product ranges** over (subspace_id, path, timestamp) within a namespace — its reason for existing is multi-dimensional data whose linearization Willow refuses to fix. `Established`.
- **Payload privacy.** The spec provides **no privacy analysis of fingerprints** and no omission-attack hardness claim ("a malicious peer can sabotage reconciliation in all sorts of interesting ways regardless"). `[confirm]` (absence-of-claim, against the fetched spec).

**Fit against §A.** Willow's fingerprint monoid and count-balanced recursive split meet reqs. 1–4 *in general*, and the 1-d log is a degenerate single-dimension case of a 3d range. But its 3-dimensional machinery — a commutative (order-free) fingerprint and 3d product ranges — is *built for the case Drystone's history log does not have* (req. 1: the log is linear and totally ordered by `(device, lamport)`). Adopting 3d-RBSR for the linear history repair pays the complexity of 3d ranges for a dimension count of one. Its privacy silence (req. 5) is acceptable only because Drystone's self-verifying fill (req. 2) covers integrity — but it offers nothing extra at the content-blind-store omission seam.

---

## C. Candidate 2 — Negentropy's range-based protocol

`Established` primitives, checked against the live spec `https://github.com/hoytech/negentropy` (README + the fingerprint note `https://logperiodic.com/rbsr.html`, fetched 2026-07-16; wire protocol "Negentropy Protocol V1", © 2023–2024).

- **Fingerprint shape.** A range fingerprint is the **addition mod 2²⁵⁶** of the element IDs (32-byte little-endian), concatenated with the element count (varint), SHA-256'd, first 16 bytes. It is **tree-friendly**: it composes additively over adjacent ranges (a monoid under addition). `Established`.
- **Partition strategy.** Split the records in a range into **N buckets**; "how to split the range is implementation-defined", the simplest being N equal-count buckets (count-balanced). `Established`.
- **Mode switch.** Each bucket is sent as an **`IdList`** (small → ship the actual IDs) or a **`Fingerprint`** (large → recurse into sub-buckets); the small/large threshold is implementation-defined. `Established`.
- **Round complexity.** **Logarithmic**: with branching factor 16, ~`log(10⁶)/log(16)/2 ≈ 2.5` rounds for a million elements, ~4 for a billion. `Established`.
- **Transport & state.** Explicitly **transport-agnostic**, an alternating sequence of range messages, with a **stateless server** (responds per-message, holds no session). `Established`.
- **Payload privacy / omission hardness.** Addition-mod-2²⁵⁶ is chosen over XOR *specifically* for **censorship resistance**: XOR fingerprint collisions are trivially found (seconds), while the carry-propagating addition makes crafting IDs that cancel a victim's records out of a fingerprint costly (~28 hours, substantial resources) — it "prioritis[es] security against censorship attacks where malicious actors craft inputs to cancel out victim elements." No anonymity claim about what a fingerprint reveals to an entitled peer. `Established` (omission hardness); `[confirm]` (no anonymity claim).

**Fit against §A.** Negentropy *is* one-dimensional RBSR (Meyer's original family) with a concrete fingerprint. It meets req. 1 (linear key space is its native shape), req. 2 (needs no integrity — Drystone's fill is self-verifying anyway), req. 3 (transport-agnostic, **stateless responder** — matching the content-blind history store exactly), req. 4 (logarithmic rounds, mode switch = the whole-set form's degenerate single `IdList` bucket generalized), and req. 5 (the additive fingerprint's omission hardness is precisely the property the content-blind-store seam wants; against entitled members the leak is to a party already in-scope). It maps directly onto the existing `(device, lamport)` key space and the `range_summary`/`missing_frames` code it replaces.

---

## D. Recommendation and costs

`Design.`

**For the §6.8.1 steady-state history/message anti-entropy surface, adopt a Negentropy-style one-dimensional range reconciler.** The repair reconciles a *linear, totally-ordered* `(device, lamport)` key space (req. 1); Negentropy is that construction natively, while Willow 3d-RBSR pays for three dimensions to serve one. Negentropy's stateless responder (req. 3) fits the content-blind history store with no session, its additive fingerprint's omission hardness (req. 5) is the one privacy property the store seam actually needs, and its `IdList`/`Fingerprint` mode switch is the exact generalization of the RUN-09 whole-set `missing_frames` (which is the degenerate "one `IdList` bucket" case). The construction is the smaller, better-matched fit; Willow's generality is unrewarded on a one-dimensional log.

**Concretely, the production form is:** sort the held keys by `(device, lamport)`; over a range whose two fingerprints disagree, split into N count-balanced sub-ranges and exchange each sub-range's fingerprint; recurse into only the disagreeing sub-ranges; when a sub-range is below a small-count threshold, ship its records (`IdList`) directly. The fingerprint is an additive monoid over the record identifiers, hashed with the count. This replaces the O(n) whole-set exchange with O(log n) rounds proportional to the divergence.

**Costs and what stays open:**
- **No lock-in of the governance surface (§E).** This recommendation binds only the §6.8.1 history/message repair. Negentropy is the 1-d specialization of the same RBSR family Willow generalizes, so a later unification of *both* surfaces on Willow 3d-RBSR — with the history log as a degenerate single dimension — remains open and is a substitution, not a redesign. `Design`.
- **Wire encoding stays `[gates-release]`.** The fingerprint bytes, bucket framing, and mode tags are Appendix B items; an experiment build uses a **test-only** fingerprint (e.g. the existing blake3/sha2 in-tree over the sorted keys) and pins nothing. `[gates-release]`.
- **Fingerprint hardness is a posture dial, not a mechanism gate.** Additive-vs-XOR is a censorship-resistance choice that matters only at the semi-trusted content-blind-store seam; against entitled members either suffices. A build asserts round-count and convergence, not fingerprint cryptographic hardness (that rides the `[gates-release]` pin). `Design`.
- **Real-transport loss stays open (X1).** Like the RUN-09 whole-set form, a build lands at **loopback** grade; real over-the-wire delivery under loss is the separate X1 residual. `Modeled`/loopback ceiling.

---

## E. The governance-fact surface is a distinct key space (not a FINDING, a clarification)

The spec reconciles **two** different surfaces by RBSR, and they are not the same key space — stating this prevents a false contradiction:

- **History/message anti-entropy (§6.8.1)** reconciles the **linear `(device, lamport)` log** — this brief's surface, recommended Negentropy-style (§D).
- **Governance facts (§6.8.5, §7.2)** live in a **namespace/subspace/path** structure and are "addressed and reconciled by range-based set reconciliation, which is a Willow-shaped data model … Drystone implements this *shape* directly … built Willow-*shaped*, not Willow-*dependent*" (§7.2, `Established` against the Willow spec). That surface is genuinely three-dimensional and its production reconciler may well be Willow 3d-RBSR.

These are **consistent**, not contradictory: §6.8.1 line 810 already frames the production construction as an open "Willow 3d-range versus Negentropy" §5 decision, and §7.2 commits only the governance-fact *data model* to Willow's shape, not the history-log reconciler. The clarification this brief adds is that the *answer differs by surface*: the linear history log wants the 1-d construction; the 3d governance-fact space wants (or already is) the Willow-shaped one. No spec text is contradicted, so **no FINDING** is raised. Anything a build discovers that *does* contradict this (e.g. the history log turning out to need a 3d range) would be a FINDING quoted both ways.

---

## F. The RED-able assertion set a build (Part 3b) would land

A minimal experiment-grade build replaces `croft-chat/src/anti_entropy.rs::missing_frames` (the whole-set diff) with the recursive count-balanced reconciler, under the **same** `steady_state_anti_entropy.rs` M2 shapes, and adds one scale-shaped case. The assertions, RED-able against the recommended construction:

1. **Diff-only equivalence (regression of the RUN-09 result).** Two connected peers, one loses a single live frame; the partitioned reconciler detects the 1-frame gap and repairs it, and the folds re-converge byte-identically — the *same* outcome as the whole-set form, so no behavior regresses. `RED-able`.
2. **Fingerprint composition.** The fingerprint of a range equals the monoid-combination of its sub-ranges' fingerprints (the tree-friendly property), and two peers holding the identical set over a range produce the identical range fingerprint — the equality that lets a matching range terminate with no records shipped. `RED-able`.
3. **Bandwidth proportional to divergence, not set size.** For a small divergence in a large held set, the records shipped equal the divergence (not the whole set) — the mode switch descends only into disagreeing buckets. `RED-able`.
4. **Scale case — O(log)-ish rounds (round-count, not wall-clock).** A large divergent range (e.g. one peer missing a contiguous block of K records out of N) is repaired in a number of reconciliation **rounds** bounded by ~`log_B(N)` (branching factor B), asserted as a round *count*, strictly fewer than the whole-set form's single O(N) exchange would cost in shipped records. The assertion is the round/shipped-record count, never wall-clock. `RED-able`.
5. **Convergence predicate holds after repair.** `converged(a, b)` is true and the range summaries have equal length — the whole-set convergence predicate still witnesses the partitioned repair. `RED-able`.

On green, the §6.8.1 residual clause updates to name the partitioned form landed at loopback grade (tag per A.9, when-in-doubt `Modeled`), and the whole-set `missing_frames` is retired to the degenerate single-bucket case. The wire encoding stays `[gates-release]`; real-transport loss stays X1.
