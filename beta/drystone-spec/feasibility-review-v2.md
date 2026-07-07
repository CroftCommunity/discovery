# Drystone Protocol: Feasibility Review (Second Pass)

author: Independent Feasibility Review (Lead Researcher)

date: 2026-07-04

status: Second pass. Spec text now read directly; first-pass [Design, spec-unverified] findings converted to verified-or-refuted against the text.

reviewed: Drystone Part 1 (Principles, 1192 lines) and Part 2 (Mechanics, 4999 lines), read in full, plus independent re-verification of external claims against primary sources (RFC 9420, RFC 9750, BLAKE3, iroh, CALM, Matrix CVE/MSCs).

feasibility-definition: Three bars. Bar 1 (Implementability): can two independent teams build interoperating implementations from the spec as written. Bar 2 (Cryptographic and security soundness): does the MLS integration and the timestamp-free governance model hold up under an adversary. Bar 3 (Load-bearing open problems): does the tail-gap close, is the §4/§7 hash split reconcilable.

---

## Reviewer's note on epistemic tiers (read first)

The first pass could not read the spec files and tagged its internal-mechanics findings **[Design, spec-unverified]**. This pass read Part 1 and Part 2 in full. The tagging convention is retained and sharpened into three tiers:

- **[Verified-spec]**: the reviewer read the exact section and confirms what it says.

- **[Verified-RFC]** / **[Verified-source]**: confirmed against a primary source retrieved this pass, with the location named.

- **[Reviewer-judgment]**: the reviewer's own synthesis or assessment, never attributed to the spec or a source.

Where the spec itself carries a `[confirm]` or `[gates-release]` marker, that is the spec's own status and is reported as such, separately from the reviewer's assessment of it. The single most useful distinction this pass can give, per the request, is **present-but-insufficient (genuine underspecification)** versus **absent (missing mechanism)**. That distinction is made explicit on every finding below.

Headline correction: the first pass raised an FS-versus-durable-history tension as a Serious issue (old S5). Having read §8.1 and re-checked RFC 9420 §16.6 and RFC 9750, **that finding was wrong and is formally withdrawn** (see W1). The spec's treatment is correct and better-grounded than the first-pass review was.

---

## Executive Summary

**Bar 1, Implementability: Unchanged in verdict, sharpened in locus. Conditionally feasible; not yet at two-team interop parity, and the spec says so itself.** The `[gates-release]` markers are exactly the byte-level encodings that block independent interop, and the spec's own §9 confirms the gap precisely: the §4/§5/§6 layer has a **passing 66-vector conformance suite**, while the §7.3–§7.5 governance-resolution vectors **depend on the `[gates-release]` encodings and are not yet in the suite**. The single highest-risk divergence point is named by the spec (§7.5.2, frontier-closure-and-subgraph-closure before sort) and correctly gated. This is finishing work, not research. **[Verified-spec, §9, §7.5.2]**

**Bar 2, Cryptographic and security soundness: Materially stronger than the first pass credited.** Every MLS hazard is not merely flagged but mechanically addressed with the RFC citations verified: external-join (§7.4.2), ReInit non-atomicity (§7.6.3), insider replay and nonce-reuse (§7.4.2), the DS/AS split with the AS filled by principal-as-CA (§10.2), and the metadata floor (§6.4) drawn more precisely than the first-pass review drew it. The forward-secrecy treatment (§8.1) is correct and the first pass's objection is withdrawn. Two MLS residuals remain genuinely open and the spec marks them `[confirm]` (external-join far-behind node; in-place secret restore). **[Verified-spec + Verified-RFC]**

**Bar 3, Load-bearing open problems: Both resolved to a clear status.** The completeness-ahead beam is the correct single load-bearing item, the spec isolates it honestly (§7.3.3, §7.3.8, Appendix B), and this pass can now classify it: the enforcement predicate is **fail-closed and safety-monotone**, so the beam bears only on liveness, never safety, which is the strongest honest claim available and matches the spec's own statement. The §4/§7 hash split is a **low-difficulty, near-closed** reconciliation: §4's uses are collision-resistance and preimage uses with no secret-prefix MAC, so BLAKE3 is a clean substitution, and the spec's Appendix B already identifies BLAKE3 as the single suite to converge on. **[Verified-spec + Verified-RFC + Reviewer-judgment]**

Overall: reading the text moved the review substantially toward "sound and honest." The design's self-awareness is not marketing; the open items are correctly located and correctly labeled. The remaining work is (1) freeze the `[gates-release]` encodings, (2) run the governance-resolution conformance vectors, (3) discharge the completeness-beam convergence experiment, (4) close the handful of `[confirm]` MLS residuals. None is a research dead-end.

---

## Withdrawals and corrections from the first pass

**W1. WITHDRAWN, the first pass's S5 ("forward secrecy vs durable history is a genuine structural tension"). [Verified-RFC, RFC 9420 §16.6; Verified-spec, §8.1]**

The first pass asserted FS and durable history are in real conflict. Reading §8.1 and re-checking the RFC shows this is wrong, and the spec is right. §8.1's argument, verified correct: **forward secrecy assumes the adversary already holds all the ciphertext**, and delivers its guarantee by requiring the *keys* to be deleted on schedule; the deletion schedule operates on keys, never on ciphertext. RFC 9420 §16.6 and RFC 9750 confirm FS/PCS rely on active deletion and replacement of *keying material*. Therefore a durability node holding sealed bytes indefinitely is squarely inside the scenario FS is built to survive, provided keys were deleted on schedule. The spec correctly relocates the real friction to **key retention** (two named seams: retaining keys to process reordered commits, and the persistently-offline node), not ciphertext retention. §8.1 does **not** overclaim FS over an archival layer; it correctly scopes FS to the key schedule and treats chosen-ephemeral content retention as a separate content-layer policy that is cooperative-non-retention, never enforced deletion. The first-pass concern was a category error the spec does not make.

**W2. CORRECTED, the first pass's S7 phrasing on the metadata floor understated the spec. [Verified-spec, §6.4]**

The first pass said a §6.4 that names the iroh direct-peer IP disclosure "honestly" would be accurate. §6.4 in fact draws it *more* precisely than the first-pass review did: it separates three observers by layer (swarm member at the overlay, relay at the iroh layer, gateway at the IP layer), and states the IP exposure is **transient** (the relay drops out once a direct hole-punch forms, ~90% of the time), sees only **pair-and-byte-count never topic or content**, and that the gateway sees only **ephemeral, churning IPs distinct from the EndpointId**. This is not a gap; it is a more careful treatment than the review's own. S7 is downgraded to a nit (see M-list).

---

## Findings by severity

### (a) Blocking issues

**B1. Open byte-level encodings (`[gates-release]`) block interoperation by definition. STATUS: confirmed present-and-correct as an open item; the spec's own conformance status proves the gap. [Verified-spec, §9, §4.2, §7.3.1, §7.5.2]**

- What the text shows: §9 states two implementations are Drystone-compatible when they agree on the identifier derivations (§4.2), signed pre-image (§4.3), integrity-vs-authority separation (§4.4), lineage fold (§4.5), rights decomposition (§5), and the append-only fold and total order (§7.3), **and, once its `[gates-release]` encodings are pinned, the frontier-closure-and-subgraph-closure rule (§7.5.2)**. The reference suite is **66/0 passing** but covers the §4/§5/§6 layer only; the §7.3–§7.5 governance-resolution vectors are explicitly **not yet in the suite** because they depend on the open encodings.

- The open encodings enumerated by the spec (Appendix B, verbatim list): the canonical governance-fact byte encoding (§7.3.1, the base all others extend); the content-id pre-image now that `timestamp` is removed (§4.2); frontier-commitment and acceptance-record format (§7.5.1); frontier-closure-and-subgraph-closure before sort (§7.5.2, self-identified as the highest-risk divergence point); the gating-vs-read wire format (§5.8.1); the capability/membership-graph wire format; and the returning-member `(G, D)` cursor and checkpoint encoding (§6.6.2, §7.4).

- Reviewer-judgment: this is **present-but-incomplete**, not a design gap, and the spec's self-diagnosis is accurate. Interop is a byte property; a passing suite on §4/§5/§6 with an empty suite on §7.3–§7.5 is precisely a design that is shape-complete but not yet wire-frozen on its governance core. Precedent for the volume of work: RFC 9420 fixes the TLS presentation language, a minimum-length varint from RFC 9000 §16, and labeled signature inputs. Drystone needs the equivalent for its governance-fact encodings. Blocking for Bar 1 until frozen and until the §7.3–§7.5 vectors pass.

**B2. The completeness-ahead beam is the single load-bearing property. STATUS: correctly isolated, honestly labeled, and now classifiable. It is present-and-sound as a fail-closed design; what is unearned is the convergence experiment, not the mechanism. [Verified-spec, §7.3.3, §7.3.7, §7.3.8, §7.4, Appendix B; Verified-source, CALM; Reviewer-judgment on classification]**

- What the text shows: §7.3.3 draws the read/enforce line and defines two states precisely. **Best-known state** is the fold of every governance fact a node currently holds (always computable). **Final state** is best-known state the node has additionally established is current to the group's leading edge, ruling out an unseen causally-later fact that would change the relevant slot. This *is* forward-completeness. §7.3.8 states the finality gate as one normative clause: an irreversible authority-enforcing action MUST use final state and MUST fail closed (stall) if final state cannot be established; reads and content-plane liveness MUST NOT be gated. §7.4 supplies the freshness cursor and the guarantee that the protocol **never asserts "this is the latest"** and instead asserts liveness-over-a-window plus causal-independence.

- The spec's own honest statement (§7.3.8, verbatim sense): what the gate cannot do is turn corroborated completeness into proven completeness; a node establishes "I have seen all committed facts to this point" by the freshness mechanism, which corroborates but does not prove the absence of an unseen fact. The gate is shaped so the beam bears **only on liveness (an over-cautious stall), never on safety (a wrongful enforcement)**. Appendix B labels this `Load-bearing, unearned` and names the discharge path: the convergence experiment (permutation-invariance of the fold, and the load-bearing half, gap-detection, does a node with a gap detect it rather than fold an incomplete set as complete).

- The four obligations the first pass enumerated, now checked against the text:

  1. *Monotonicity classification.* See the dedicated section below. Verdict: the enforcement predicate is **safety-monotone under a fail-closed reading**; the beam is a liveness property, not a safety one, which sidesteps the strict-consistency horn of CALM. The spec reaches the same place by construction.

  2. *Anti-state-reset proof.* Present and rigorous (§7.5.2). Drystone adopts the exact MSC4297 conflicted-subgraph-closure that fixed CVE-2025-49090, and its append-only monotonic fold makes the reversion class structurally impossible: a lagging node under-authorizes, never reverts. See the dedicated section. **[Verified-spec, §7.3, §7.5.2; Verified-source, MSC4297/CVE-2025-49090]**

  3. *Fork-composition proof.* Present in mechanism (§7.3.5 ceilings union within a lineage and diverge across a fork; §7.6.4 ban-as-fork). What is not yet present is the *proof* that the two compose cleanly in all cases; the spec carries this as the beam's convergence experiment. Present-but-unproven.

  4. *Liveness under partition.* Explicitly addressed and explicitly bounded: an isolated node MUST fail closed on enforcement while continuing to read. The spec states this liveness limit is **intrinsic and stated, not closed**.

- Reviewer-judgment: this is the strongest part of the design and the labeling is exactly honest. It is **present-and-sound as a fail-closed safety design**; the residual is a convergence *experiment* (permutation-invariance and gap-detection), which is a testable engineering obligation, not an open research question. Blocking only in the sense that the experiment must pass before the "governance-ordering-needs-no-referee" claim graduates from conditional to earned. The spec says exactly this.

**B3. BCP 14 clause strength for interop. STATUS: substantially better than the first pass feared; the interop-critical clauses are already MUST. The residual is confined to the still-open encodings, i.e. it collapses into B1. [Verified-spec, §4.2–§4.4, §7.3.1, §7.3.2, §7.5.2, §9]**

- What the text shows: the clauses that actually force interop are already at MUST where they must be. §4.2: implementations MUST derive identifiers identically ("they are the interop anchor"). §4.4: a receiver MUST reject a broken chain or non-contiguous sequence, and integrity alone MUST NOT be treated as authorization. §7.3.1: every honest persona MUST resolve a conflict the same way; the ordering, the causal-precedence rule, and the content-address tiebreak are stated as MUST-level with the reasoning for each. §7.3.2: cross-slot effects MUST NOT be applied by mutating state mid-fold (the order-dependence trap). §7.5.2: the input set MUST be closed under both frontier-closure and conflicted-subgraph-closure before sorting.

- Reviewer-judgment: the first pass flagged the risk that interop-critical behavior might sit at SHOULD. Reading the text, it does not; the normative spine is correctly MUST. The one genuine residual is that several of these MUSTs point at encodings that are still `[gates-release]` (the content-address pre-image, the governance-fact byte layout, the closure computation's serialization). A MUST over an unfrozen encoding is not yet interoperable. So B3 is real but is **the same gap as B1**, not an independent one: freeze the encodings and the MUSTs become executable. No separate SHOULD-should-be-MUST audit is needed; the audit was run this pass and the clauses pass.

### (b) Serious issues (investigate; may not block)

**S1. External-join hazard. STATUS: present-and-addressed; one residual `[confirm]`. [Verified-spec, §7.4.2, §10.2.1; Verified-RFC, RFC 9420 §3.3, RFC 9750 §8.1.4]**

Confirmed as the client stated. §7.4.2: a `GroupInfo` is treated as a **claim** corroborated against the authoritative governance chain the node already holds, not as authority; two distinct defenses discharge the two halves (the monotonic fold means a forged assertion forks itself out rather than corrupting anyone; the per-Group threshold quantifies the attack MLS leaves unquantified). The RFC hazard is real and correctly cited (RFC 9750 §8.1.4 / §5.3, verified). The residual the spec itself carries: a rejoining node far enough behind that its own view has not advanced past the epoch a recent-but-superseded `GroupInfo` points at; the under-authorize-never-mis-authorize property suggests it holds but it is reasoned, not proven (Appendix B, `[confirm]`). Present-but-one-case-unproven, correctly labeled.

**S2. ReInit non-atomicity / stranding. STATUS: present-and-named with the tradeoff explicit; one residual `[confirm]`. [Verified-spec, §7.6.3; Verified-RFC, RFC 9750 §6.1/§7]**

Confirmed as the client stated, and documented at §7.6.3 with unusual precision. The freeze-then-strand window is named; the freeze-first ordering is stated as a **deliberate tradeoff** purchasing replay-immunity at the cost of the stranding window; the candidate resolution (the governance chain records the re-plant intent to membership M **before** the freeze, so any member can complete a stranded re-plant from the authoritative instruction) is stated with the residual explicitly open: whether the ordering is genuinely intent-recorded-before-freeze is `[confirm]` against the delivery and governance-chain ordering (Appendix B). This is **present-but-insufficient** in exactly the way the client asked to have flagged: the mechanism shape is right and the completion path is identified, but the ordering guarantee that discharges it is not yet proven. Not a missing mechanism; an unproven ordering property.

**S3. Insider replay and nonce-reuse-on-restore. STATUS: present-and-addressed; one residual `[confirm]`. [Verified-spec, §7.4.2; Verified-RFC, RFC 9750 §8.6, RFC 9420 §6.3.1/§16.7]**

Confirmed. §7.4.2 isolates both hazards by the same structural separation: gap-aware history convergence runs **out of band** (§6.8.1) and never re-injects into the live MLS stream, so a replayed old message arrives as a content-addressed history-tree entry (idempotent, non-advancing under the monotonic fold), not as a live MLS message to process, hence inert. The RFC facts are correctly cited and verified. The residual the spec carries: this holds only if recovery is always "re-plant or re-join fresh, converge history out of band" and **never** resurrects a live group's epoch secrets in place; if any path does the latter, the hazard returns (Appendix B, `[confirm]`). Present-but-conditional-on-an-invariant-not-yet-audited-across-all-paths.

**S4. epoch_authenticator overlap. STATUS: RFC role verified this pass; the spec correctly carries it as an `[confirm]` adoption question, not a hazard. [Verified-RFC, RFC 9420 §8.7; Verified-spec, §10.2.1 "Underused", Appendix B]**

Resolved to a clear status. RFC 9420 §8.7 defines the epoch authenticator as a per-epoch value members compare out of band to confirm they share the same group state; two members on divergent branches derive different authenticators (verified via the RFC ToC and the MLS-extensions description of the epoch authenticator as an application-facing agreement-confirmation function). The spec does not claim an overlap *hazard*; it correctly frames this as an **Underused** MLS construct (§10.2.1) and asks whether Drystone's whole-group consistency check can use the epoch_authenticator directly rather than a separately-built comparison, and how it relates to the governance chain's own consistency signal (Appendix B, `[confirm]`). Reviewer-judgment, and the tie to B2 the first pass suspected: epoch_authenticator equality proves **same-epoch key-state agreement** but says nothing about **governance-chain completeness-ahead**; the two consistency signals are orthogonal, which is precisely why folding them (rather than running them in parallel) is the open question. The spec's instinct is right and the item is correctly a design-adoption question, not a soundness gap.

**S5. WITHDRAWN. See W1.** Forward secrecy and durable history are not in tension; the spec is correct.

**S6. Fork-not-verdict and "ban is a fork". STATUS: present-and-sound; the re-admission ceiling closes the case the first pass asked to investigate. [Verified-spec, §7.3.5, §7.6.2, §7.6.4; Verified-source, MSC4289/MSC4291]**

Confirmed and stronger than the first pass credited. The first pass asked to investigate whether a banned party's later-arriving events can re-merge across the fork (the Matrix "Hotel California" analogue). §7.3.5 answers it with the **membership ceiling**: the quorum-crossing fact MUST stamp the governance head as-of which the removed member's authority ends; the ceiling carries the fact's k-of-n authority, is stamped by the completing signer, and is **unforgeable by construction** (the removed member is not the completing signer and cannot assemble a quorum excluding the required others). An action at or beyond the ceiling is **void regardless of finality** (§7.3.8), so a banned party's later events cannot re-authorize above the ceiling. Fork composition is specified: concurrent ceilings **union within a lineage** and **diverge across a fork** (§7.3.5), never forced into false concordance. "Ban is a fork" is mechanically sound (§7.6.4): a ban MUST NOT be a deletion; it forks the member off whole, withdrawing only the group's forward corroboration. The client's claim that the ceiling stops re-admission above the established bound is verified. Present-and-sound. Residual: the general (non-membership) slot completeness still rests on the beam (B2), which the spec states.

**S7. DOWNGRADED to nit. See W2 and the M-list.** §6.4 handles the metadata floor more precisely than the first-pass review did.

**S8. iroh-gossip pre-1.0 dependency. STATUS: present-and-acknowledged as a realization, not a requirement. [Verified-spec, §6.10, §10.3; Verified-source, iroh-gossip crate status]**

Confirmed. The spec treats the gossip overlay (HyParView + PlumTree) as the *current realization* of C-swarm (§6.10), explicitly a substrate realization behind a requirement (§10.3), not a hard dependency. iroh-gossip remains pre-1.0 (latest published 0.96.0) and separately versioned from iroh 1.0.0, so the wire-stability risk is real and the spec's requirement-vs-realization framing is the correct hedge. Present-and-correctly-abstracted; pin the dependency and keep the interface boundary.

### (c) Minor issues, nits, and confirmations

**M1. iroh 1.0 dating. [Verified-source]** The spec states iroh core is 1.0 (June 2026), wire-and-API-stable, and uses `EndpointId` (noting pre-1.0 material used `NodeId`). Verified: iroh 1.0 shipped June 15, 2026, asserts wire and API stability, dial-by-key with direct-first hole-punch (~90%+) and blind relay fallback. The spec's dating and terminology are accurate. The one precision nit: the spec could cite the exact date rather than the month.

**M2. §6.4 metadata floor is a model finding, not a nit against the spec. [Verified-spec, §6.4, §6.2.3]** Formerly S7. The three-observer decomposition (swarm/relay/gateway) and the transient-IP treatment are more careful than the first-pass review. The RFC-sourced residues (RFC 9420 §16.4.1 group-id/epoch/frequency to the DS; §16.4.3 membership via the tree and Public-message Add/Remove; the §16.3 sender-data concealment) are all cited correctly in §6.2.3. No action beyond the metadata-confidential-transport note the RFC itself recommends.

**M3. The now, the finality gate, freshness. STATUS: all present and internally consistent with fork-not-verdict. [Verified-spec, §7.3.7, §7.3.8, §7.4]** The first pass flagged these as [Design, spec-unverified] and asked whether they cohere with the fork primitive. They do: the now is bound-by-reference to the chain head and reconciles by re-derivation (§7.3.7); the finality gate composes with the ceiling and does not replace it (§7.3.8); freshness refuses to render silence as currency (§7.4). The declarative snapshot is explicitly a cache, never authoritative, never trusted from a participant (§7.3.3), which is the exact discipline that avoids the Matrix state-reset failure mode. Confirmed coherent.

**M4. Prior-art characterization. [Verified-spec, Appendix A.1/C; Verified-source]** Accurate. The decentralized-MLS forward-secrecy-cost framing (Appendix A.1, §8.1.1) correctly characterizes DMLS/FREEK as preliminary and not a dependency, adopts two ideas (content-derived epoch identifiers, PPRF-retained init secrets) while declining its two-coordinating-server consolidation. Willow/Meadowcap positioning is accurate. The spec's caution about external MLS operations is justified by the literature.

**M5. Application-message ordering not leaned on. STATUS: present-and-explicit. [Verified-spec, §10.2.1, §4.5.1]** Confirmed as the client stated. The spec explicitly declines to use MLS ordering: the logical clock is per-client and strictly logical (§4.5.1), ordering is structural not temporal (§4.3), and §10.2.1 places MLS's linear transcript as subordinate. M5 is present and the decline-to-use-MLS-ordering is explicit.

**M6. Naming reconciliation (`croft-*` to `drystone-*`). [Verified-spec, §4.2, Appendix B]** The spec flags that its reference tags use the historical `croft-*` namespace and that a vendor-neutral `drystone-*` rename is a real wire change re-opening the §4 signature proofs, so it must be re-proven not silently swapped. Correctly identified as a signed-encoding change. This interacts with B1.

---

## Primary-source verification results (this pass)

Legend: **Accurate** = spec matches the source; **Corrected** = spec is more accurate than the first-pass review; **[confirm]-remaining** = the spec's own open check, not yet closed.

**RFC 9420 application-data carriage (§2 terminology): Accurate, verbatim.** "An Application Message is a PrivateMessage carrying application data"; PrivateMessage provides encryption and authentication for application data. Confirms the carry-only path (see the MLS-exchange-plane section). Re-verified this pass against the RFC HTML.

**RFC 9420 §8.2 transcript hash (single-predecessor chain): Accurate, verbatim.** The spec quotes `confirmed_transcript_hash[n] = Hash(interim_transcript_hash[n-1] || ConfirmedTranscriptHashInput[n])`, seeded from zero-length strings, a strict single-predecessor chain with no branch/merge representation. The spec's Appendix B marks this "resolved this revision" and the reviewer confirms the formula and the "one linear sequence" consequence §7.6.3 relies on.

**RFC 9420 §8.7 epoch_authenticator: role Accurate; adoption question is the spec's open `[confirm]`.** §8.7 defines the per-epoch authenticator for out-of-band group-state agreement confirmation. The spec correctly frames adoption (Underused, §10.2.1) rather than claiming a hazard. Reviewer-judgment: epoch_authenticator proves same-epoch agreement, not completeness-ahead; orthogonal to the beam.

**RFC 9420 §16.6 / RFC 9750 forward secrecy: Accurate; first-pass objection withdrawn.** FS/PCS rely on active deletion and replacement of keying material; FS holds against an adversary holding all ciphertext provided keys are deleted. Confirms §8.1. The persistently-offline-node caveat is in the primary source and the spec carries it.

**RFC 9420 §3.3 / RFC 9750 §8.1.4 external commit and stale-GroupInfo: Accurate.** External Commit lets a joiner enter via a published GroupInfo; a stale/corrupted GroupInfo is a PCS-defeat or DoS vector. Spec's §7.4.2 corroboration-against-chain defense is sound; one far-behind-node case remains `[confirm]`.

**RFC 9750 §6.1/§7 ReInit non-atomicity: Accurate, verbatim.** Committing a ReInit immediately freezes the group and triggers creation of a new group; the operation is not always atomic, so a member can go offline after committing but before creating the new group. Confirms §7.6.3.

**RFC 9750 §8.6 / RFC 9420 §6.3.1, §16.7 insider replay and reuse_guard: Accurate.** MLS does not protect against insider replay (per-sender counter detects a gap but does not prevent replay); reuse_guard exists because reverting state can reuse a nonce and break AEAD. Confirms §7.4.2.

**RFC 9750 §8.2.4 client-correlation metadata (K5 cost): Accurate.** Representing each client as its own member lets other members identify which client/device sent each message and detect client add/remove; may be an unacceptable privacy breach for some applications; mitigated by careful leaf-node/credential handling. The spec logs this as an accepted tradeoff (§10.2), not a hidden cost.

**RFC 9750 §8.1.2 epoch-number metadata leak: Accurate, verbatim (per spec).** MLS header metadata is an opaque group_id plus a numerical epoch (count of changes); an observer correlating it may reconstruct sensitive information; mitigation is a metadata-confidential transport. The spec's open part is the re-plant-frequency interaction, correctly carried as an open thread.

**BLAKE3 length-extension and security level: Accurate, verbatim from the source.** The official BLAKE3 repository states BLAKE3 is "secure against length extension, unlike SHA-2," and is "a Merkle tree on the inside." Corroborated: BLAKE3 targets ~128-bit collision / 256-bit preimage resistance, equivalent to SHA-256's claims. This confirms both the reviewer's §4/§7 analysis and the spec's Appendix B reasoning. See the dedicated hash-split section.

**CALM theorem: Accurate (spec's `[confirm]` can be promoted).** "A problem has a consistent, coordination-free distributed implementation if and only if it is monotonic" (Hellerstein & Alvaro, CACM 63(9) 2020; arXiv:1901.01930). The spec's Part 1 §1.2 states this and the reviewer confirms verbatim.

**Matrix CVE-2025-49090 / MSC4297 / MSC4289 / MSC4291: Accurate.** The spec's §7.3 and §7.5.2 treatment is verified: the CVE root cause was starting-state and replay-scope (not the timestamp tiebreak); the MSC4297 fix begins iterative auth checks from the empty set and replays the conflicted subgraph (SCC / forward-backward intersection); MSC4289 gives creators infinite power; MSC4291 makes room-id the hash of the create event. The spec adopts the closure without adopting the ordering, correctly separated. Re-verified against the Matrix state-res v2.1 material and CVE record.

**iroh 1.0 (June 15, 2026), EndpointId, direct-first-with-blind-relay: Accurate.** Re-verified. Wire-and-API-stable, dial-by-key, post-handshake identity (the §6.1 seam), ~90%+ direct hole-punch. iroh-gossip separately pre-1.0 (0.96.0).

---

## MLS as an exchange plane for application data and hash-tree payloads

The client's earlier question (is MLS robust enough to carry application and hash-tree payloads) is now answerable against both the RFC and the spec's own §6, and the answer resolves the first-pass `[confirm]` on bind-versus-carry.

**The spec depends on carry only, not bind, and this is the right and sufficient choice. [Verified-spec, §6.2.2, §7.6.3, §10.2.1; Verified-RFC, RFC 9420 §2, §8.2, §8.5]**

- What the text shows: §6.2.2 states application content rides as an MLS PrivateMessage carrying application data (RFC 9420 §2, verified). §7.6.3 states the conversation persists across a sequence of MLS groups, carried by the **application-layer structures** (dataplane history and governance chain), and the MLS group identity is **not** the conversation identity. §10.2.1's hard-case table (row: "Linear epoch chain") states continuity lives in **app-layer hash structures (§7.6.3)**, with MLS subordinate. Nowhere does the spec fold an application Merkle root into the MLS transcript hash or derive it through the MLS exporter.

- Confirming the client's own belief: the client stated "we believe we only carry, in application_data, and do not bind into the transcript or exporter, but we do not say so explicitly." The reviewer confirms the belief is correct and the design is consistent with it. **The one gap is exactly the one the client named: it is nowhere stated explicitly.** The carry-only choice is load-bearing for §7.6.3 (the conversation-outlives-the-key-layer property depends on content *not* being bound to any single MLS group's transcript), so leaving it implicit is a genuine, if small, underspecification.

- Reviewer-judgment on right-and-sufficient: carry-only is correct for Drystone because (i) binding content into the transcript would tie the conversation's integrity to one MLS group's linear epoch chain, which §7.6.3 explicitly needs to outlive across forks and re-plants, and the transcript cannot represent a fork (RFC 9420 §8.2, verified); (ii) the content's own integrity is already provided by the app-layer hash chain (§4.4, `H(prev ‖ seq ‖ payload)`) and signature+standing (§4.3, §4.4), so transcript binding would be redundant; (iii) the exporter (RFC 9420 §8.5) is the right tool only if one needed a group-epoch-derived secret bound to content, which Drystone does not, because it deliberately keeps entitlement (PSK-carried) and content (dataplane-carried) separate (§7.6.3). So carry-only is not a limitation worked around; it is the correct layering.

**Recommended one-line addition (present-but-unstated fix):** state explicitly, in §6.2.2 or §10.2.1, that Drystone carries all application and content-structure payloads in the MLS `application_data` field and does **not** bind them into the MLS transcript hash or derive them via the MLS exporter, with a one-clause reason (the conversation must outlive any single MLS group, §7.6.3). This converts an implicit invariant into a checkable one and closes the first-pass E6 `[confirm]`.

**Carriage constraints (unchanged from the RFC, all consistent with the spec):**

- Ordering is not provided by MLS framing (RFC 9420 §15.3); the spec correctly supplies its own structural order and declines MLS ordering (§4.3, §4.5.1, §10.2.1). Consistent.

- The `application_data` vector caps at 2^30 bytes (RFC 9420 §2.1.2); the practical ceiling is lower (a frame is buffered/sealed whole), so large content must be chunked into content-addressed leaves, which is exactly the dataplane hash-tree model (§7.7). Consistent.

- Padding is a sender-chosen length-hiding policy, not automatic (RFC 9420 §15.1; RFC 8446 §5.4 for TLS); the spec addresses size-distribution leakage in §6.4 and the high-threat-dial residue in §6.2.3. Consistent.

---

## The tail-gap / completeness-beam problem

**Is it closable in principle?** Yes, and reading §7.3.3/§7.3.8/§7.4 changes the framing from the first pass in the client's favor.

**The consistency-target question (the client's item 3), answered against §7.4. [Verified-spec, §7.4, §7.3.8]**

The first pass framed the beam against CALM's strict-consistency horn. Reading §7.4 shows Drystone's actual target is **not strict consistency**. §7.4 states the protocol **never asserts "this is the latest"** because "most current" would presume a global vantage no node has and would rest on the wall-clock Part 1 §2.0.1 rules out; instead a node establishes **liveness-over-a-window** and **causal-independence**, both corroborable. The enforcement side is **fail-closed** (§7.3.8): an irreversible action stalls unless final state is established.

- Reviewer-judgment: this is a **bounded, detectable, fail-closed** target, not strict consistency. That materially softens the CALM impossibility argument in the way the client anticipated. CALM says a *non-monotonic, coordination-free, consistent* implementation is impossible. Drystone does not attempt that. It splits the problem:

  - **Reads / content plane:** monotonic (best-known state, grow-only fold), hence CALM-*permitted* coordination-free. Always live. No beam dependency.

  - **Irreversible enforcement:** treated as requiring a coordination point (final state), and when that point cannot be reached, the action **fails closed** rather than proceeding inconsistently. This converts the non-monotonic "have I seen everything?" question from a *safety* risk into a *liveness* cost.

- So the classification of the completeness predicate is: "have I seen every governance fact causally prior to point X" is **non-monotonic** (one more arrival can flip it), therefore not closable coordination-free, exactly as CALM says. Drystone does not pretend otherwise. What it does is ensure the **only** consequence of the non-monotonic step is a fail-closed stall (liveness), never a wrongful enforcement (safety). Under a fail-closed reading the *enforceable* predicate ("proceed only if final") is **safety-monotone**: additional facts can only turn a stall into a proceed, never turn a correct proceed into a breach. That is the precise sense in which the beam is "liveness-only," and it is what the spec claims in §7.3.8.

**How the CALM softening changes the beam obligations (the client's explicit ask):**

- Obligation 1 (monotonicity classification) is **answered**: the predicate is non-monotonic, but the *enforcement gate built on it* is safety-monotone by fail-closed construction. No coordination-free completeness detector is needed or claimed.

- Obligations 2 (anti-state-reset) and 3 (fork-composition) remain the substantive ones, and they are **safety** properties independent of the beam. Anti-state-reset is discharged by the append-only fold plus §7.5.2 subgraph-closure (see below). Fork-composition is specified (§7.3.5, §7.6.4) and needs the convergence experiment to be *proven*, not merely specified.

- Obligation 4 (liveness under partition) is the **only** place the beam bites, and the spec correctly states the limit is intrinsic: an isolated node cannot self-certify completeness and must stall on enforcement. This is a liveness cost, accepted as "delay over breach."

**What a full closure still needs (the discharge path, matching the spec's Appendix B):**

1. Run the convergence experiment: (a) permutation-invariance of the fold (fold the same fact set in many orders, confirm identical authority state), and (b) the load-bearing half, gap-detection (a node with a missing in-between fact **detects** the gap rather than folding an incomplete set as complete). Threshold to move Appendix B from `unearned` to `earned`: both pass on an adversarial corpus including omitted conflicted-subgraph facts.

2. Prove fork-composition: two honest partitions produce ceilings/nows that either merge cleanly or fork explicitly, never silently disagree (extends §7.3.5's within-lineage-union / across-fork-diverge rule to a stated invariant with a test).

3. State the freshness-cursor completeness argument for **non-membership** slots (roles, thresholds), which lack the ceiling's durable marker and rest on the now + finality gate + freshness attestation. This is the part §7.3.5 explicitly does **not** close.

**Anti-state-reset, verified as present and rigorous. [Verified-spec, §7.3, §7.5.2; Verified-source, MSC4297/CVE-2025-49090]**

The first pass asked for this proof; it is in the text. §7.3's append-only monotonic fold makes reversion structurally impossible (a lagging node under-authorizes, never reverts). §7.5.2 adopts the exact MSC4297 fix that closed CVE-2025-49090: close the resolution input set under **frontier-closure** (backward-reachable authorizing grants) **and** **conflicted-subgraph-closure** (the SCC containing the contracted conflicting facts, computed as the forward-backward-reachable intersection), before sorting. The spec is explicit that adopting the closure does not adopt Matrix's power-and-wall-clock ordering (it keeps its content-address tiebreak). The reviewer verified the MSC4297 two-change characterization against the Matrix state-res v2.1 material and confirms the adoption is faithful. A subtle point the spec makes well: an incomplete-closure bug in Drystone does **not** produce Matrix's reversion; it produces two honest peers stuck at different heads, which §7.6 surfaces as a false human-escalation, hence the closure is normative and byte-specified (a B1 `[gates-release]` item) precisely to protect the escalation channel's trustworthiness.

---

## The §4/§7 hash split (SHA-256 vs BLAKE3)

**Is it a real problem?** Yes but low-severity, and reading §4 plus verifying BLAKE3 makes it **near-closed**. The spec's own Appendix B has already done most of the resolution.

**The §4 length-extension check (the client's item 4), run against the actual §4 text. [Verified-spec, §4.1–§4.4; Verified-source, BLAKE3 repo]**

- §4's hash uses, enumerated from the text: tagged identifier derivations `H(tag ‖ id)` where `tag = version ‖ domain-separator` (§4.2); the content id `H(canonical{group_id, regime, author_id, content})` (§4.2); the branch hash-chain `hash = H(prev ‖ seq(LE) ‖ payload)` (§4.4). Authorship is a separate **Ed25519 signature over a pre-image** (§4.3, `signing_bytes = "msg-v1" ‖ branch ‖ seq ‖ author_id ‖ 0x00 ‖ payload`), not a hash-based MAC.

- The length-extension question, answered: length extension breaks exactly one construction, the secret-prefix MAC `H(secret ‖ message)` on a Merkle-Damgard hash, because the hash output *is* the internal state and an attacker can continue hashing. **No §4 construction is a secret-prefix MAC.** The identifier and content-id uses are collision-resistance/preimage uses over public, domain-separated pre-images; the branch chain is a collision-resistance use; authentication is Ed25519, not `H(secret ‖ m)`. Reviewer-judgment: **no §4 proof relies on a SHA-256-specific property, and none implicitly assumes a length-extension resistance SHA-256 lacks.** There is no latent §4 length-extension bug to find. Re-basing §4 on BLAKE3 is the mechanical substitution the first pass expected.

- Corroborating direction: BLAKE3 is length-extension resistant (official repo, verbatim: "secure against length extension, unlike SHA-2"), so moving §4 from SHA-256 to BLAKE3 **removes** a footgun rather than adding one, and the domain-separation tags (§4.2) already provide the per-purpose separation that any hash choice needs.

**How hard to close?** Low, and partly already done by the spec. Appendix B's hash-reconciliation entry already: (i) identifies BLAKE3 as convergent with Willow/Earthstar's BLAKE3-256 payload hashing (verified: Willow's Earthstar instantiation uses BLAKE3), giving mild evidence the single committed suite should be BLAKE3 with §4's SHA-256 as the legacy side; (ii) notes the pin must track Willow's "to be replaced by WILLAM3" note. What remains:

1. Pick one suite per hashing context and freeze it as MUST (this is a B1 `[gates-release]` item; the content-address pre-image and governance-fact encoding must be byte-fixed anyway).

2. Confirm (this pass did, at the construction level) that no §4 proof depends on a SHA-256-specific property; the conformance vectors that currently pass on SHA-256 (§9, 66/0) must be re-run on the committed suite.

3. Specify BLAKE3 output length (256-bit) and per-context domain-separation labels, mirroring §4.2's existing tag discipline.

Reviewer-judgment: this is days-to-weeks of spec-and-test work, not a research program, and the direction (converge on BLAKE3) is already the spec's stated lean. The only real caution is operational: the `croft-*` to `drystone-*` tag rename (M6) and the suite pin are both **signed-encoding changes** that re-open the §4 signature proofs, so they should be bundled into one wire-freeze and re-proven together, not applied piecemeal.

---

## Recommendations (staged, with thresholds)

**Stage 1 (unblock interop):**

1. Freeze the `[gates-release]` encodings as one bundle: governance-fact byte layout (§7.3.1), content-id pre-image (§4.2), frontier-and-subgraph-closure serialization (§7.5.2), acceptance-record and frontier-commitment (§7.5.1), `(G, D)` cursor and checkpoint (§6.6.2, §7.4), gating-vs-read (§5.8.1). Bundle the `croft-*` to `drystone-*` rename and the hash-suite pin into the same freeze and re-prove the §4 signature proofs once. Threshold for Bar 1: two independent prototypes exchange and validate every message type without shared code, and the §7.3–§7.5 governance-resolution conformance vectors join the suite and pass.

2. Pin the hash suite (converge on BLAKE3-256 per the spec's Appendix B lean); re-run the 66-vector suite on the committed suite. Threshold: a written per-context note stating which function each hashing context uses and that no proof depends on a function-specific property (this pass supplies the construction-level check; the note formalizes it).

**Stage 2 (earn the load-bearing item):**

3. Run the completeness-beam convergence experiment: permutation-invariance of the fold **and** gap-detection on an adversarial corpus that includes omitted conflicted-subgraph facts. Threshold to move Appendix B from `unearned` to `earned`: gap-detection demonstrably fires rather than folding an incomplete set as complete.

4. Prove fork-composition as a stated invariant with a test (extends §7.3.5): two honest partitions' ceilings/nows merge cleanly or fork explicitly, never silently disagree.

5. State the non-membership-slot (role/threshold) completeness argument that §7.3.5 explicitly leaves to the now + finality gate + freshness attestation.

**Stage 3 (close the MLS `[confirm]` residuals):**

6. Discharge the four MLS residuals the spec carries: external-join far-behind node (§7.4.2), in-place secret restore invariant across all recovery paths (§7.4.2), re-plant intent-recorded-before-freeze ordering (§7.6.3), and the epoch_authenticator fold-versus-parallel adoption (§10.2.1). Each is a specific stress-test, not a design gap.

**Stage 4 (small, present-but-unstated fixes):**

7. State explicitly that Drystone carries content in MLS `application_data` and does not bind into the transcript or exporter, with the §7.6.3 reason (closes the first-pass E6 and the client's own noted gap).

8. Add the exact iroh 1.0 date (June 15, 2026) where the spec says "June 2026" (M1), and keep the iroh-gossip dependency behind its interface until it reaches 1.0 (S8).

---

## Caveats

- This pass read Part 1 and Part 2 in full. The referenced folded companion documents (history-durability, governance-finality, fold-semantics, conventions-and-decisions, asset-keying, fact-and-chain-representation) were **not** in the uploads directory; where the spec points into them for byte-level detail (e.g. the now's wire schema, the asset-keying retained-copy floor), those specifics could not be independently read and are reported as the spec's own cross-references, not as verified. This does not affect the findings above, which rest on Part 2's own normative sections.

- The completeness-beam classification (liveness-only, safety-monotone under fail-closed) is **[Reviewer-judgment]** built on the verified text of §7.3.3/§7.3.8/§7.4; it agrees with the spec's own statement but is the reviewer's framing, not a quotation. The convergence experiment that would *earn* the beam has not been run; its status is the spec's `Load-bearing, unearned`, unchanged.

- The §4 length-extension check is **[Reviewer-judgment]** at the construction level: it confirms no §4 use is a secret-prefix MAC and therefore none is length-extension-vulnerable. It is not a machine-checked proof; the recommendation formalizes it as a written per-context note plus a re-run of the conformance suite on the committed suite.

- One RFC item (epoch_authenticator §8.7) was verified for role and purpose via the RFC ToC and the MLS-extensions description; the fully verbatim §8.7 derivation text was not extracted this pass. The role claim is not in doubt; the fold-versus-parallel adoption is the spec's open `[confirm]`.

- Every load-bearing external claim is anchored to a primary source (the RFCs, the CALM paper, the BLAKE3 repository, the Matrix state-res v2.1 material and CVE record, iroh's release). Secondary sources were used only to locate primaries. Where a figure could vary across secondaries (iroh direct-connection rate ~90%), it is not load-bearing to any verdict.
