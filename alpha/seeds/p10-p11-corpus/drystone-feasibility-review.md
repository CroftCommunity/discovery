# Drystone Protocol: Feasibility Review

author: Independent Feasibility Review (Lead Researcher)

date: 2026-07-04

reviewed: Drystone two-part specification (Part 1 Principles, Part 2 Mechanics), together with independent verification of the specification's external technical claims against primary sources

feasibility-definition: Feasibility is tested across three bars. Bar 1 (Implementability): can two independent teams build interoperating implementations from the spec as written. Bar 2 (Cryptographic and security soundness): does the MLS integration and the timestamp-free governance model hold up under an adversary. Bar 3 (Load-bearing open problems): does the tail-gap close, and is the §4/§7 hash split reconcilable.

---

## Reviewer's note on source access (read first)

I must be transparent about a limitation that bounds part of this review. The two specification files at `/mnt/user-data/uploads/p10-full-part1-principles.md` and `/mnt/user-data/uploads/p10-full-part2-mechanics.md` could not be read during the first research pass. No filesystem-read tool was available in that environment: the web fetcher rejects `file://` URIs and only fetches URLs already seen in the conversation, and the Google Drive tools require a Drive document ID rather than a local path.

Consequently, this review separates cleanly into two epistemic tiers:

- The INDEPENDENT PRIMARY-SOURCE VERIFICATION (RFC 9420, RFC 9750, iroh, CALM, CRDTs, Matrix CVEs and MSCs, BLAKE3) is complete and fully sourced. This is the part of the task flagged as critical, and it is delivered in full.

- The analysis of Drystone's INTERNAL mechanics (its section numbering, its exact normative clauses, its `[gates-release]` markers, its Appendix B items) is reconstructed from the detailed description in the task prompt, not from the spec text itself. Wherever a finding depends on the spec's internal wording, it is tagged **[Design, spec-unverified]**. These findings are framed as the questions a reviewer with the text would test, plus the external evidence that bears on them.

I did not fabricate section-level confirmations I could not perform. This is consistent with the stated preference: accuracy before fluency, and explicit flagging where a fact cannot be sourced.

---

## Executive Summary

**Bar 1, Implementability: Conditionally feasible, but NOT yet at two-team interoperation parity.** The spec's own `[gates-release]` markers are an accurate self-diagnosis: a protocol whose byte-level encodings are declared open cannot yet be independently reimplemented to interoperate, because interoperation is precisely a byte-level property. "Build-against shape complete" is a legitimate milestone, but it is one milestone short of interop-ready. The MLS substrate it builds on (RFC 9420) is genuinely interop-tested with multiple independent implementations, so the shape Drystone is targeting is achievable; the gap is finishing work, not research. **[Design + Verified-RFC]**

**Bar 2, Cryptographic and security soundness: The foundation is sound and, importantly, the spec's characterizations of the MLS hazards it claims to document are accurate against the primary sources.** Every MLS-level hazard the task says Drystone flags (external-join via external commit, ReInit non-atomicity, epoch_authenticator semantics, the trusted-AS / untrusted-DS split, insider fragmentation) is real and is documented in RFC 9420 or RFC 9750 in the manner described. The genuine unresolved tension is structural, not a bug: **forward secrecy and durable history are in real conflict**, because MLS forward secrecy is defined by the active deletion of key material, while durable history requires that material (or the plaintext) to survive. Any center-free design can have at most a carefully bounded version of both. **[Verified-RFC]**

**Bar 2 addendum, MLS as an application-data and hash-tree exchange plane: Confirmed suitable.** MLS is a sound carriage plane for application-layer payloads, including Merkle/hash-tree data and content-addressed structures. RFC 9420 defines an application message as a PrivateMessage carrying application data, and the PrivateMessage guarantees (confidentiality, integrity, member-attributed authentication, epoch-scoped) apply to arbitrary opaque bytes. The suitability is real and inherited from the same guarantees the rest of the design already relies on. The one boundary that must be drawn sharply is between *carrying* a hash tree (the `application_data` path, robust and sufficient) and *binding* a hash tree into MLS's own authenticated group state (the exporter / `authenticated_data` path, a different and not-yet-verified question). See the dedicated section below. **[Verified-RFC]**

**Bar 3, Load-bearing open problems: The tail-gap / completeness-beam is the correct single thing to worry about, and it is closable in principle but currently unearned.** The impossibility of a node knowing its authority state reflects ALL committed governance events is a real, deep property, and CALM tells us why: detecting completeness is a non-monotonic problem, so it cannot be both consistent and coordination-free. A fail-closed checkpoint is the right shape of answer, but it necessarily reintroduces a coordination point, which is in tension with "center-free." The §4/§7 hash split (SHA-256 vs BLAKE3) is a real reconciliation problem but a LOW-difficulty one. **[Verified-paper + Design]**

Overall verdict: Drystone is a coherent and unusually well-self-aware design. It is not yet feasible for two-team interop (Bar 1 blocked by open encodings). It is cryptographically plausible on a sound substrate (Bar 2 conditionally met), and its use of MLS as a carriage plane for hash-tree payloads is well-founded. Its central research risk (Bar 3, tail-gap) is closable but not yet closed, and the proposed closure must be proven to not smuggle in a coordinator.

---

## Findings by severity

### (a) Blocking issues

**B1. Open byte-level encodings (`[gates-release]`) block interoperation by definition. [Design, spec-unverified as to exact locations; Verified as to principle]**

- Issue: The task states the spec marks byte-level encodings as `[gates-release]`, i.e. build-against-shape-complete but not wire-frozen. Interoperation between two independent implementations is a byte-for-byte property: two teams cannot produce mutually decodable messages if the field ordering, length-prefix conventions, canonicalization, and signature-input serialization are not fixed.

- Reasoning and precedent: RFC 9420 shows exactly how much of a spec this consumes. MLS interoperability rests on the TLS presentation language (RFC 8446), a minimum-length variable integer encoding adapted from RFC 9000 Section 16 (with the explicit rule that values using more bits than necessary MUST be treated as malformed, and that the prefix "11" is invalid and MUST be rejected), and labeled signature/encryption inputs of the form `"MLS 1.0 " + Label`. That is a large body of normative byte-level text. Until Drystone has the equivalent, two teams will diverge on the first non-trivial message.

- Judgment: This is blocking for Bar 1 but is finishing work, not research. It should be the first thing frozen.

**B2. The tail-gap ("completeness beam") is unearned and load-bearing. [Verified-paper + Design]**

- Issue: A node cannot currently detect whether its authority state reflects all committed governance events. Appendix B (per the task) flags this as the single "Load-bearing, unearned" item.

- Reasoning from primary source: This is not an incidental gap; it is the non-monotonic core of the whole design. The CALM theorem (Hellerstein & Alvaro, "Keeping CALM: When Distributed Consistency Is Easy," Communications of the ACM 63(9), 2020; arXiv:1901.01930, Theorem 1) states verbatim: "A problem has a consistent, coordination-free distributed implementation if and only if it is monotonic." Knowing that you have seen ALL events is the canonical non-monotonic query: its truth can be falsified by the arrival of one more message, so, in the authors' words, non-monotonic problems cannot proceed until they know all information has arrived, requiring them to coordinate. Therefore a fully coordination-free "I have everything" detector is provably impossible. This is why the item is load-bearing: it is the exact point where the center-free ambition meets a hard theorem.

- Judgment: Closable, but only by accepting a bounded coordination point (see the dedicated section below). The spec must not claim to have closed it until it exhibits either (i) the fail-closed checkpoint with an explicit statement of what coordination it requires, or (ii) a proof that the specific governance predicate it needs is monotonic (e.g. expressible as a grow-only set / semilattice join) and therefore exempt.

**B3. Ambiguity in normative clauses will cause divergent implementations unless BCP 14 usage is audited clause-by-clause. [Design, spec-unverified]**

- Issue: The task asks whether the MUST/SHOULD/MAY clauses are precise enough to interoperate on. The clauses could not be read in the first pass. But the failure mode is well-established and worth flagging as a gate: RFC 9750 (the MLS architecture) explicitly warns that if two deployments differ on application-level parameters they will interoperate to some extent but may experience unexpected failures in certain situations, such as extensive message reordering, and that policy must be uniform across clients or the clients might end up in different cryptographic states, breaking their agreement.

- Judgment: Any clause that is SHOULD where interop actually requires MUST (validation rules, canonical ordering, rejection behavior on malformed input) is a latent fork. This requires a line-by-line pass by someone with the text; treat it as blocking for interop until done.

### (b) Serious issues (investigate, may not block)

**S1. The external-join hazard is real and inherited from MLS. [Verified-RFC]**

- RFC 9420 Section 3.3 describes external joins via an external Commit, usable in the case of an "open" group that can be joined by new members without asking permission from existing members. The joiner downloads a GroupInfo object and forms a special Commit (Section 12.4.3.2, using the previous epoch's external key pair via Section 8.3 External Initialization). This is a genuine hazard surface: whoever publishes GroupInfo, and the DS that serves it, gate who can inject themselves. Drystone is correct to treat this as a hazard. Investigate: who is authorized to publish GroupInfo in a center-free deployment, and how external-commit authorization is bound to Drystone's governance plane rather than to a DS.

**S2. ReInit non-atomicity is real and can strand a group. [Verified-RFC]**

- RFC 9750 states verbatim that committing a ReInit immediately freezes the existing group and triggers the creation of a new group with a new group_id, that ideally the same member finishes it, but this operation is not always atomic, so it is possible for a member to go offline after committing a ReInit proposal but before creating the new group, and if this happens it is necessary for another member to continue the reinitialization by creating the new group and sending out Welcome messages. In a center-free setting with no orchestrator to guarantee follow-through, this is more dangerous than in a server-mediated deployment: the group can be frozen with no member reliably designated to complete the transition. Investigate the recovery protocol Drystone specifies for a stranded ReInit.

**S3. Insider replay and group fragmentation by malicious insiders. [Verified-RFC]**

- RFC 9420 Section 16.12 (Group Fragmentation by Malicious Insiders) and RFC 9750 both acknowledge that an insider or a compromised or misbehaving DS can present different members with different views. RFC 9750 notes the DS is trusted to select the single Commit message that is applied in each epoch, and that this trust can enable the DS or a malicious insider to undermine the post-compromise security guarantees. A center-free protocol removes the trusted DS but does NOT automatically remove the fragmentation risk; it redistributes it to whatever chooses the canonical next commit. Drystone's fork-not-verdict primitive (see S6) is the intended answer, but its soundness needs to be demonstrated against this specific attack.

**S4. epoch_authenticator overlap. [Verified-RFC, partial]**

- RFC 9420 Section 8.7 defines epoch authenticators; the mechanism lets members confirm they agree on group state for an epoch (a divergence detector). The section's existence and its role in the key schedule are verified; the full verbatim text of 8.7 was not extracted, so the specific "overlap hazard" the spec claims to document is tagged **[confirm]**: the reviewer should verify against 8.7 and 8.2 (transcript hashes) exactly what the claimed overlap is. The plausible reading is that epoch_authenticator equality across two members proves same-epoch agreement but does NOT prove either member has seen the complete governance tail, which ties directly back to B2. If that is Drystone's claim, it is correct and important.

**S5. Forward secrecy vs durable history is a genuine structural tension, not a solvable bug. [Verified-RFC]**

- RFC 9420 Section 1 and Section 16.6 ground FS/PCS in the Double Ratchet lineage and in key rotation. RFC 9750 makes the mechanism explicit: because FS and PCS rely on the active deletion and replacement of keying material, any client which is persistently offline may still be holding old keying material and thus be a threat to both FS and PCS if it is later compromised. Forward secrecy is, definitionally, the destruction of the ability to decrypt past ciphertext. Durable history is, definitionally, the retention of the ability to read past content. You cannot have strong FS over exactly the same material you durably retain. The reconcilable version is: durable history is retained as re-encrypted-at-rest plaintext under a separately managed archival key, decoupled from the MLS epoch secrets that provide transport FS. Drystone's §8.1 FS claims should be read as claims about the transport/epoch layer only, and the review should confirm the spec does not overclaim FS over the archival layer. If §8.1 claims FS AND durable history over the same key material, that is an overclaim.

**S6. Fork-not-verdict and "ban is a fork" — sound in shape, and notably better-grounded than Matrix's approach. [Verified-RFC + Design]**

- The design instinct is corroborated by the Matrix cautionary tale (see verification section). Matrix resolves conflicting authority by computing a single verdict via State Resolution v2, and that verdict-computation is exactly where CVE-2025-49090 (state resets) lived. A "fork, don't adjudicate" primitive avoids the class of bug where a deterministic resolver silently reverts state. "Ban is a fork" is coherent: rather than mutating shared authority (which invites reversion), a ban partitions the membership graph. This is sound IF (i) both sides of a fork converge to the same fork boundary (a semilattice/monotonic property, which CALM would then bless), and (ii) there is no path by which a banned party's later-arriving events can re-merge across the fork. Investigate (ii) specifically: it is the analogue of Matrix's "Hotel California" and re-sync manipulation (MSC4297) problems.

**S7. Metadata floor (§6.4) — what the protocol structurally cannot hide is real and inherited. [Verified-RFC]**

- RFC 9420 Section 16.4 is candid that MLS does not hide all metadata: Section 16.4.1 addresses GroupID, Epoch, and Message Frequency, and RFC 9750 states that the transport layer is present to keep metadata private from network observers, while the MLS protocol provides confidentiality, integrity, and authentication guarantees for the application data. In other words, MLS itself leaks group identifiers, epoch numbers, message timing and frequency, and (to the DS in the PublicMessage model) membership-change signatures; confidentiality of that metadata is delegated to transport. For Drystone the transport is iroh, and here is a subtle structural leak worth naming: iroh's own documentation notes that direct connections always require the disclosure of each device's IP address to the other endpoint, a structural property of NAT traversal, not something iroh can engineer away (mitigable only via the Tor transport at a latency cost). So Drystone's metadata floor is: peer IP addresses to direct peers, plus group-id/epoch/timing to anyone who can observe the encrypted flows. A §6.4 that states this honestly is accurate; one that claims to hide it is overstated.

**S8. iroh-gossip dependency is pre-1.0 and carries no wire-stability guarantee. [Verified]**

- See verification section. This is a real supply-chain/roadmap risk for anything Drystone builds on gossip-layer dissemination.

### (c) Minor issues, ambiguities, nits

**M1. Dating precision on iroh.** The spec's "wire-and-API-stable as of June 2026" is correct and can be made precise: iroh 1.0 shipped June 15, 2026 (see verification). Recommend the spec cite the exact date and the wire-stability wording rather than the month.

**M2. CALM is tagged `[confirm]` in the spec; it can be promoted to Verified-paper.** The statement is exact (see B2). No hedge is needed.

**M3. "declarative snapshot as cache" and the read/enforce line (§7.3.3), the finality gate (§7.3.8), freshness/no-false-current (§7.4). [Design, spec-unverified]** These are internal §7 constructs not readable in the first pass. Conceptually they are the right primitives: a snapshot that is treated as a cache (never as authority) is exactly the discipline that avoids the Matrix state-reset failure mode, where cached/resolved state was treated as truth. The read/enforce line (permitting reads from possibly-stale state while refusing to ENFORCE governance from it) is a sound way to stay available under partition while remaining fail-closed on authority. The finality gate and no-false-current freshness property are the local-node embodiment of the completeness-beam and should be evaluated together with B2. Flagging as needing the text to confirm the constructs are internally consistent with the fork-not-verdict primitive.

**M4. Prior-art characterization (Appendix A/C) of the "decentralized MLS / forward-secrecy cost of center-free ordering" tradeoff.** The framing is well-supported. Recent formal work confirms external operations in RFC 9420 were under-analyzed until recently: the first computational security analysis of the final RFC 9420 including external proposals and external commits (IACR ePrint 2025/229, "Extended-Operations TreeKEM") post-dates the RFC by two years. So Drystone's caution about external ops is justified by the literature, not merely defensive. Willow/Meadowcap is accurately positioned as a capability-based, primitive-agnostic access model (it does not prescribe the cryptographic primitives you use).

**M5. Application-message ordering must not be leaned on for hash-tree reconstruction. [Verified-RFC]** RFC 9420 §15.3 (Delayed and Reordered Application Messages) establishes that the framing layer does not provide a total order over application messages. For a Merkle DAG this is normally harmless, since hash linkage is self-ordering, but any part of the content layer that assumes MLS reports send order is unfounded. See the dedicated MLS-exchange-plane section (E3).

---

## MLS as an exchange plane for application data and hash-tree payloads

This section addresses a specific suitability question: is MLS robust enough to serve as the exchange plane for application-layer content structures, in particular Merkle/hash-tree data and content-addressed payloads. The short answer is yes, with one boundary that must be drawn sharply. All claims here are verified against RFC 9420 directly.

### E1. The core answer: MLS carries arbitrary application data, hash trees included. [Verified-RFC]

RFC 9420 Section 2 (Terminology) defines an Application Message as a PrivateMessage carrying application data, and states that a PrivateMessage provides encryption and authentication for both Proposal/Commit messages as well as any application data. "Any application data" is the operative phrase. From MLS's perspective, Merkle nodes, roots, and content-addressed payloads are opaque bytes in the `application_data` field, encrypted under the epoch's AEAD keys and authenticated as coming from a specific member in a specific epoch.

Nothing about tree-structured or hash-linked content is special to MLS; it neither knows nor cares what the plaintext encodes. The property Drystone wants from a carriage plane, namely authenticated, confidential, member-attributed exchange of content structures, is exactly what the framing layer delivers, and it is the same PrivateMessage guarantee the rest of the design already leans on.

### E2. The boundary that must be drawn: carrying a hash tree vs being bound into MLS's own trees. [Verified-RFC]

Two distinct things share the word "tree" in this setting, and conflating them is the one real error to avoid.

1. The application Merkle tree, which rides inside `application_data` as opaque payload. MLS carries it. This is Drystone's case and it is sound.

2. MLS's own internal trees: the ratchet tree (membership and key agreement) and the transcript-hash chain (Section 8.2) with the epoch authenticators (Section 8.7). RFC 9420 keeps these strictly internal. The transcript hashes commit to MLS's own handshake history (Proposals and Commits), not to application content. Through the normal framing path you cannot fold an application Merkle root into the group's cryptographic transcript.

If the requirement is only transport of hash-tree data, path 1 is sufficient and robust. If the requirement is for MLS to bind the content structure into the group's own authenticated state, that is a different and more involved question, and it turns on the exporter (Section 8.5) and the `authenticated_data` field of `PrivateContentAAD` rather than on `application_data`. Drystone should state explicitly which of the two it depends on, because the feasibility answer diverges there. See the open item E5.

### E3. Constraint: ordering is not guaranteed by the framing layer. [Verified-RFC, §15.3]

MLS gives epoch-scoped authentication, not a total order over application messages. RFC 9420 has a dedicated Section 15.3 (Delayed and Reordered Application Messages) precisely because application messages can arrive out of order; the generation counter in the secret tree is for key derivation, not for reconstructing send order.

For a Merkle DAG this is usually a non-issue, since the structure is self-ordering by hash linkage: parent nodes reference children by digest, so causal order is recoverable from the content itself regardless of arrival order. The design consequence is a constraint, not a blocker: if any part of the scheme leans on MLS to report the sequence in which payloads were sent, that assumption is unfounded and must live in Drystone's own content layer.

### E4. Constraint: the 2^30-byte vector ceiling, and why the real limit is lower. [Verified-RFC, §2.1.2]

The `application_data` field is a variable-length vector, so its hard encoding cap is about 1 GiB. RFC 9420 Section 2.1.2 states these vectors can represent values with length from 0 bytes to 2^30 bytes, and warns that implementations should take care not to allow vectors to overflow available storage.

In practice the ceiling is far lower and is set by the delivery layer, not the framing layer, since a single AEAD-sealed frame must be buffered and transmitted whole. The design consequence for a hash tree is concrete: chunk large blobs into content-addressed leaves sized to the transport MTU or streaming budget, and send the Merkle structure as many small frames rather than one large one. This is the standard content-addressed-store pattern and it aligns naturally with a hash tree.

### E5. Constraint: padding is fixed-zero, so size-based traffic analysis is the application's responsibility. [Verified-RFC, §15.1 and §16.4]

MLS supports padding but does not automatically obscure length. RFC 9420 Section 15.1 is strict: the padding field MUST be filled with all zero bytes, and a receiver MUST verify that there are no non-zero bytes in the padding field, and if this check fails the enclosing PrivateMessage MUST be rejected as malformed. Padding lets a sender round sizes up, but the policy is the sender's choice.

This matters more for a hash tree than for chat text, because content-addressed chunking tends to produce characteristic size distributions, and fixed-size chunks leak less than natural-boundary chunks. Combined with the metadata floor (S7), MLS does not hide `group_id`, epoch, or message frequency at the framing layer, so an observer counting frames can estimate the shape and growth rate of the tree even without reading it.

### E6. Open item: bind vs carry must be resolved against the exporter and authenticated_data. [confirm]

The suitability of MLS for Drystone's specific Merkle scheme depends on a question the framing sections cannot answer: does Drystone need the content structure bound into MLS's authenticated group state (the exporter / `authenticated_data` path), or only carried confidentially and attributably (the `application_data` path)? The framing and restriction sections (§6, §15) are verified. Section 8.5 (Exporters) and the precise semantics of the `authenticated_data` field in `PrivateContentAAD` are not yet pulled; that is where the "bind, don't just carry" answer lives. If binding is what Drystone's §4/§6 requires, this path must be verified against the primary text before the yes is complete.

---

## Primary-source verification results

Legend: **Accurate** = spec's characterization matches the primary source; **Overstated/Understated/Wrong** as labeled; **Unverifiable** = could not be confirmed from a primary source.

**iroh 1.0 "wire-and-API-stable as of June 2026" — Accurate (and datable to June 15, 2026).**
iroh 1.0 shipped June 15, 2026 from N0, Inc., after 65 pre-release versions over 4+ years. Per n0's release post (iroh.computer/blog/v1): any change that affects the wire stability of iroh will always coincide with a major release, and 1.0 asserts stability for both the wire protocol and language APIs. N0's public relays have seen more than 200 million endpoints created in a recent 30-day window. Official bindings now cover Rust, Python, Node.js, Swift, Kotlin. The spec's phrase "public-key identity, direct-first-with-blind-relay-fallback, post-handshake authentication" is consistent with the documented model (dial by cryptographic key; direct hole-punch first, relay fallback where relays see only node identifiers, not content). One structural caveat the spec should carry: iroh's docs note direct connections always require the disclosure of each device's IP address to the other endpoint.

**Pkarr / Mainline DHT address lookup — Accurate.**
Pkarr (Public-Key Addressable Resource Records) publishes DNS resource records signed by an Ed25519 keypair, storable on the BitTorrent Mainline DHT via BEP44 mutable items. iroh uses this via `_iroh.<endpoint-id>` TXT records served by iroh-dns-server. Important nuance verified from iroh's own docs: publishing Pkarr packets directly onto the Mainline DHT is not yet supported in iroh natively (the default path is the Pkarr relay / iroh-dns-server over DNS). Records are ephemeral (dropped after hours; must be republished) and capped at ~1000 bytes. If the spec implies native DHT publishing is turnkey in iroh, that is slightly **Overstated**.

**iroh-gossip pre-1.0 — Accurate.**
iroh-gossip's latest published version is 0.96.0, confirming it remains pre-1.0 and is separately versioned from the stabilized iroh 1.0.0 crate. Its CHANGELOG shows it tracking iroh via breaking changes. It implements HyParView (membership) plus PlumTree (broadcast). The spec's claim that iroh-gossip is pre-1.0 is correct and should be treated as a live wire-stability risk.

**CALM theorem — Accurate, verbatim.**
Hellerstein & Alvaro, "Keeping CALM: When Distributed Consistency Is Easy" (Communications of the ACM 63(9), 2020; arXiv:1901.01930), Theorem 1: "A problem has a consistent, coordination-free distributed implementation if and only if it is monotonic." The spec's `[confirm]` tag can be promoted to Verified-paper. The mechanism (non-monotonic problems cannot proceed until they know all information has arrived, requiring them to coordinate) is exactly what makes the tail-gap load-bearing.

**CRDTs / convergence without consensus — Accurate.**
Shapiro, Preguiça, Baquero & Zawirski (2011; INRIA Research Report and SSS 2011) formalized Conflict-free Replicated Data Types, establishing Strong Eventual Consistency achievable without consensus for the monotonic (semilattice/commutative) class. The spec's use of CRDTs to justify convergence for its monotonic governance subset is well-founded. Note the boundary: CRDTs give convergence for the monotonic class only, which is the same class CALM identifies; this consistency across the two citations is a point in the spec's favor.

**RFC 9420 application data / PrivateMessage carriage — Accurate, verbatim.**
Section 2 (Terminology): an Application Message is a PrivateMessage carrying application data, and a PrivateMessage provides encryption and authentication for both Proposal/Commit messages as well as any application data. Confirms MLS as a carriage plane for arbitrary application payloads including hash-tree data (see the MLS-exchange-plane section).

**RFC 9420 application-message ordering (§15.3) — Accurate.**
Section 15.3 (Delayed and Reordered Application Messages) confirms the framing layer does not guarantee a total order over application messages; ordering that a content layer needs must be supplied by that content layer.

**RFC 9420 vector size ceiling (§2.1.2) — Accurate.**
Variable-length vectors can represent values from 0 to 2^30 bytes; implementations should guard against overflow. Confirms the ~1 GiB framing ceiling and the practical need to chunk large payloads.

**RFC 9420 padding rule (§15.1) — Accurate, verbatim.**
The padding field MUST be filled with all zero bytes, and a receiver MUST verify there are no non-zero bytes, rejecting the enclosing PrivateMessage as malformed otherwise. Confirms padding is a sender-chosen length-hiding policy, not automatic length obfuscation.

**RFC 9420 external joins / external commits — Accurate.**
Section 3.3: external Commit lets a new member join without asking permission from existing members via a published GroupInfo; detailed in Section 12.4.3.2 and Section 8.3 (External Initialization). The external-join hazard is real.

**RFC 9420 / RFC 9750 ReInit non-atomicity — Accurate, verbatim.**
RFC 9750: committing a ReInit immediately freezes the existing group and triggers the creation of a new group with a new group_id, and this operation is not always atomic, so it is possible for a member to go offline after committing a ReInit proposal but before creating the new group.

**RFC 9420 epoch_authenticator (Section 8.7) — Accurate as to existence and role; overlap-hazard detail tagged [confirm].**
Section 8.7 defines epoch authenticators used to confirm group-state agreement for an epoch. The precise "overlap hazard" the spec claims should be checked against the verbatim 8.7 text (not fully extracted in this pass).

**RFC 9420 / RFC 9750 DS/AS trust model — Accurate, verbatim.**
RFC 9420 Section 3: MLS assumes a trusted AS but a largely untrusted DS. A compromised AS (Section 16.10) can impersonate or mint credentials; a compromised DS (Section 16.9) cannot read content but can drop, delay, reorder, and deny, and is trusted to select the single Commit message that is applied in each epoch. RFC 9750 confirms this trust can enable the DS or a malicious insider to undermine the post-compromise security guarantees. The spec's two-plane framing (identity/AS vs delivery/DS) maps correctly onto this.

**RFC 9420 forward secrecy / PCS (Section 16.6) — Accurate.**
FS and PCS are core guarantees, grounded in key rotation and active deletion and replacement of keying material (RFC 9750). The persistently-offline-client caveat is explicit in the primary source. The FS-vs-durable-history tension (S5) follows directly.

**RFC 9420 metadata confidentiality (Section 16.4) — Accurate.**
MLS does not itself hide GroupID, epoch, message frequency, or (in the PublicMessage model) membership changes; that is delegated to transport. Spec's §6.4 metadata floor is inherited-accurate.

**Matrix state resolution folds authority and a wall-clock into ordering — Accurate.**
State Resolution v2 uses mainline ordering and, as an explicit tiebreak, `origin_server_ts` (the wall-clock). Drystone's motivation to be timestamp-free is well-grounded: this is exactly the fragility it avoids.

**CVE-2025-49090 (Matrix state reset) — Accurate.**
Per the CVE record: the Matrix specification before 1.16 (i.e., with a room version before 12 and State Resolution before 2.1) has deficient state resolution. The state-reset class: room state reverts to an earlier value when a change is optimistically applied while, concurrently, the actor loses permission. Fixed via room version 12 / State Resolution v2.1 (matrix.org "Project Hydra"), coordinated release August 2025. It allows a room member to potentially reset the room's state to an earlier value but does not grant administrative or creator privileges.

**MSC4289 / MSC4291 / MSC4297 — Accurate.**
MSC4289 (Explicitly privilege room creators): room creators get infinitely high power level, and the default power to send `m.room.tombstone` is raised to stop ordinary admins from seizing creator privileges. MSC4291 (Room IDs as hashes of the create event): room IDs become the cryptographic hash of the `m.room.create` event, guaranteeing exactly one immutable create event. MSC4297: State Resolution 2.1, hardening the federation re-sync mechanism against intentionally triggered state resets. All three are bundled into room version 12 (Matrix 1.16, September 2025).

**Willow / Meadowcap / decentralized-MLS prior art (Appendix A/C) — Accurate.**
Meadowcap is a capability-based access model for the Willow data model, deliberately primitive-agnostic (does not prescribe the cryptographic primitives you use). The claim that external MLS operations carry a forward-secrecy/ordering cost is supported by IACR ePrint 2025/229, the first computational analysis of RFC 9420's external operations, published only in 2025.

---

## The tail-gap / completeness-beam problem

**Is it closable in principle?** Yes, but only in a bounded sense, and the boundary is dictated by a theorem, not by engineering effort.

- Why it is hard: "Has my node seen ALL committed governance events?" is a non-monotonic predicate. Its answer can flip from true to false when one more event arrives. By CALM (Theorem 1, arXiv:1901.01930), a predicate that is not monotonic has NO consistent, coordination-free implementation. So a purely local, purely coordination-free "I am complete" detector cannot exist. This is the deepest thing in the spec and the reason Appendix B is right to call it load-bearing and unearned.

**Is the fail-closed checkpoint approach sound?** Directionally yes, with a mandatory caveat.

- A fail-closed checkpoint is the correct response to a non-monotonic completeness query: when you cannot prove completeness, you refuse to ENFORCE authority (you fail closed) rather than acting on possibly-incomplete state (which is how Matrix produced state resets). Fail-closed is strictly safer than fail-open here.

- The caveat: a checkpoint that lets a node conclude "I now have everything up to point X" is, by CALM, a coordination point. It is a barrier at which nodes agree that the set of prior governance events is sealed. That is coordination, and coordination is in tension with "center-free." This does not sink the design, because center-free is not the same as coordination-free: you can have a checkpoint that any member can propose and that converges without a privileged center, so long as the seal itself forms a monotonic structure (a grow-only chain of checkpoints where each checkpoint is a semilattice join over all events causally prior to it). The reviewer's synthesis: the checkpoint must be constructed so that the ONLY non-monotonic step (declaring completeness) is quorum-witnessed or causally-sealed, while everything else stays monotonic and coordination-free.

**What a closure would need to prove.** To move Appendix B from "unearned" to "earned," the spec should exhibit, at minimum:

1. A precise statement of the completeness predicate and a proof that it is either (a) monotonic (hence CALM-exempt and closable coordination-free), or (b) non-monotonic, in which case the exact coordination the checkpoint requires is stated explicitly.

2. A liveness argument: under partition, does the fail-closed checkpoint deadlock governance, and if so, what is the degraded-but-safe mode (this ties to the §7.3.3 read/enforce line and §7.4 no-false-current)?

3. A safety argument against the Matrix failure class: show that a late-arriving pre-checkpoint event cannot silently reverse enforced authority (the state-reset guarantee that CVE-2025-49090 lacked before State Resolution 2.1).

4. A fork-consistency argument: show the checkpoint mechanism composes with fork-not-verdict, i.e. two honest partitions produce checkpoints that either merge cleanly or fork explicitly, never silently disagree.

---

## The §4/§7 hash split (SHA-256 vs BLAKE3)

**Is it a real problem?** Yes, but it is a low-severity, low-difficulty one.

- The situation as described: §4 material is proven/analyzed on SHA-256, while §7 is designed on BLAKE3. This is a real reconciliation task because a protocol that computes hash-based identifiers, transcript hashes, or Merkle/tree structures MUST fix ONE hash function per context for interoperability; two implementations that disagree on the hash produce non-matching identifiers and fail to interoperate (a Bar 1 blocker if left ambiguous). MLS itself illustrates the discipline: the hash algorithm is fixed per cipher suite, and hash-based identifiers are domain-separated with labels like "MLS 1.0 KeyPackage Reference."

**How hard to close?** Low difficulty, for three reasons grounded in primary sources:

1. Security-equivalence at the 256-bit level. BLAKE3 and SHA-256 both target ~128-bit collision resistance and 256-bit preimage resistance (BLAKE3 spec / BLAKE3 team; NIST FIPS 180-4 for SHA-256). So the CHOICE between them is not a security downgrade; the §4 proofs done on SHA-256 carry over to BLAKE3 under the standard collision-resistant-hash / random-oracle assumptions, because they rely on those generic properties rather than on SHA-256-specific structure. The reviewer's judgment: unless a §4 proof exploits a SHA-256-specific property (e.g. it deliberately reasons about Merkle-Damgard length-extension), re-basing the proofs on BLAKE3 is a mechanical substitution.

2. BLAKE3 is length-extension resistant (tree/Merkle construction), whereas SHA-256 is not. This means moving §4 FROM SHA-256 TO BLAKE3 removes a footgun rather than adding one. The only real analytic care needed is the reverse direction: if any §4 construction implicitly assumed length-extension resistance it did not have under SHA-256, that is a latent §4 bug to find, not a §7 problem.

3. BLAKE3's extendable-output (XOF) and KDF/MAC modes give §7 flexibility SHA-256 lacks, but the spec must domain-separate uses (distinct context strings per purpose), exactly as MLS does with its labeled inputs, to avoid cross-protocol collisions.

**What closure requires:** (i) pick ONE function per hashing context and freeze it as a MUST (this is part of B1); (ii) confirm no §4 proof relies on a SHA-256-specific property; if one does, re-derive it under the generic collision-resistant-hash model or under BLAKE3 explicitly; (iii) specify BLAKE3 output lengths and per-context domain-separation labels. This is days-to-weeks of careful spec work, not a research program.

---

## Recommendations (staged, with thresholds that would change them)

**Stage 1 (unblock interop — do first):**

1. Freeze all `[gates-release]` byte-level encodings into normative MUST-level text: field ordering, variable-length integer convention, canonicalization, and the exact signature/encryption input serialization (mirror RFC 9420's labeled-input discipline). Threshold to consider Bar 1 met: two independent prototypes exchange and validate every message type without shared code.

2. Resolve the §4/§7 hash split: one function per context, frozen as MUST; audit §4 proofs for SHA-256-specific dependence. Threshold: a written note stating, per hashing context, which function is used and that no proof depends on a function-specific property.

3. Line-by-line BCP 14 audit: promote to MUST every SHOULD/MAY that interop actually depends on (validation, canonical ordering, rejection-on-malformed). Threshold: no interop-critical behavior left at SHOULD.

**Stage 2 (earn the load-bearing item):**

4. Produce the completeness-beam proof obligations enumerated above (monotonicity classification, liveness under partition, anti-state-reset safety, fork composition). Threshold to move Appendix B from "unearned" to "earned": a written proof that the completeness predicate's single non-monotonic step is the ONLY coordination point and is center-free (quorum-witnessed or causally-sealed).

5. Specify the stranded-ReInit recovery protocol (S2) and the external-commit authorization binding (S1) for the center-free setting. Threshold: an adversary who goes offline mid-ReInit, or who attempts an unauthorized external commit, cannot strand or hijack the group.

6. Resolve the bind-vs-carry question for hash-tree payloads (E6): confirm against RFC 9420 §8.5 (Exporters) and the `authenticated_data` semantics whether Drystone needs content bound into MLS's authenticated state or only carried in `application_data`. Threshold: the spec states which path §4/§6 uses, and if binding is required, the exporter path is verified against the primary text.

**Stage 3 (harden and de-risk dependencies):**

7. Pin and abstract the iroh-gossip dependency behind an interface, given it is pre-1.0 (0.96.0) with no wire-stability guarantee. Threshold to relax: iroh-gossip reaches 1.0 with a wire-stability commitment.

8. State the metadata floor (§6.4) honestly and completely, including the iroh direct-peer IP disclosure, the MLS-inherited group-id/epoch/timing leakage, and the application-payload size-distribution leakage for hash-tree chunking (E5). Threshold: §6.4 enumerates every field the protocol structurally cannot hide, and §8.1 scopes its FS claim to the transport/epoch layer, not the archival layer.

9. Independently confirm the epoch_authenticator overlap claim against verbatim RFC 9420 Section 8.7 (one of two remaining `[confirm]` items).

---

## Caveats

- The most important caveat is stated at the top: the two Drystone spec files could not be read in the first pass. All findings that depend on the spec's internal text are tagged **[Design, spec-unverified]** or **[confirm]** and are framed as the questions a reviewer WITH the text should test. The independent primary-source verification, which the task flagged as critical, is complete and is the load-bearing contribution of this review. The MLS-as-exchange-plane section (E1 through E6) is verified directly against RFC 9420.

- Two RFC items remain `[confirm]`: the epoch_authenticator Section 8.7 verbatim text and its specific overlap hazard (S4), and the bind-vs-carry exporter path for hash-tree payloads (E6, RFC 9420 §8.5 and the `authenticated_data` semantics). Both are identified, and neither is load-bearing to the verdicts already given.

- The tail-gap analysis rests on CALM, which is a theorem about consistent-AND-coordination-free implementations. If Drystone's actual requirement is weaker than consistent (e.g. it tolerates a bounded, detectable inconsistency window), the impossibility argument softens correspondingly; the reviewer should confirm which consistency target §7.4 actually demands.

- Third-party summaries (tech-press coverage of iroh, secondary explainers of CALM/CRDTs) were used only to locate primary sources; every load-bearing external claim is anchored to a primary source (the RFCs, the CACM/arXiv CALM paper, the INRIA CRDT report, the CVE record, the Matrix MSCs and matrix.org disclosures, the BLAKE3 spec/team, and iroh's own release posts and docs). Where a secondary source and a primary source could conflict, the discrepancy is noted rather than resolved, and no such figure is load-bearing to the verdict.
