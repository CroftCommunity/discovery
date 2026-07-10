# Drystone: History Convergence and the Durability Tier

`Status: draft. Model settled; one internal dependency (dataplane checkpointing) and one granularity choice (chunking) open. See Open items.`

`Scope: how a Group's dataplane history is converged among members and served by a durability tier, over one cryptographic primitive (MLS over iroh), while keeping content confidential and metadata exposure at the meer level. Covers the two reconciliation questions (live delivery versus history convergence), the member and history-store tiers, the mirror-group transport, the reconciliation envelope, and the member-to-member and store convergence flows.`

`Companion to: asset-keying.md, which defines the per-scope content keys under which payloads here are sealed and the fold-gated provisioning that governs them. This document uses those keys and does not redefine them. Also companion to social-mapping.md, whose private-overlay distribution topologies are a specialization of this document's tier model, and whose records this document's transport and stores carry. Also companion to ../../drystone-spec/part-1-reasoning-underpinnings.md and ../../drystone-spec/part-2-certifiable-design.md; vocabulary inherits from the conventions-and-decisions reference.`

This is the "what" for the durability and history layer: the tier that stores and serves dataplane history, and the reconciliation by which members reach a complete view. Normative keywords are **MUST / SHOULD / MAY** (BCP 14). Status flags per load-bearing claim: `Verified` (checked this cycle against a primary source, RFC 9420, RFC 9750, or the Willow specifications, cited inline), `green-real` (exercised against real crypto or transport), `Design` / `Synthesis` (the design's own reasoning), `[confirm]` (load-bearing and resting on an external fact or an internal item not yet pinned).

> **One primitive, one boundary.** The governing constraint of this document, and the reason it exists as a separate concern, is that every distinct cryptographic envelope is a distinct threat boundary, and boundaries compose worse than linearly. The durability tier therefore uses the same primitive as messaging, MLS over iroh, and achieves content-blindness by nested sealing under a key a node lacks, never by a second protocol. Willow's own end-to-end encryption scheme is a second envelope and is deliberately not used here (see §M). `Synthesis.`

> **Center-free framing.** No node is a privileged or canonical source. "Is this current" resolves to corroboration against held facts, and "am I missing something" resolves to a structural gap check over hash-linked chains, never to a query against an authoritative vantage (Part 1 §2.2; Part 2 §2.0.1, §7.4). `Synthesis.`

---

## A. Terms

**history store**: a node whose role is to hold and serve dataplane history with high availability. It is an MLS member of a mirror group for transport, and is content-excluded by a Group Role Set and by key-withholding. Defined in §E, §F.

**mirror group (G-hist)**: an MLS group whose membership mirrors a main group G plus one or more history stores, used only as the transport channel between members and stores. Distinct from G, which carries content among members. Defined in §E.

**content-blind**: able to hold, forward, and reconcile sealed data without any key that decrypts its content. A property of the history store. Enforced cryptographically by key-withholding and stated at the governance layer by a Group Role Set.

**device-subspace**: a single device's Willow subspace. Each device holds a key in its persona's key lineage (the persona root key, or a key that root stamped out), and writes into its own subspace. A persona is the set of device-subspaces sharing a lineage. Single writer per subspace by construction. Defined in §H.

**chain**: the per-subspace hash-linked sequence of a device-subspace's entries, each referencing its predecessor. The structure that makes a gap nameable. Defined in §I.

**reconciliation envelope**: the minimal, content-free metadata carried in clear to a history store (inside the mirror-group transport) that lets it detect gaps and serve spans without seeing content, paths, wall-clock, or capabilities. Defined in §G.

**checkpoint**: a corroborated commitment to a pruned chain prefix, so validation can terminate at the checkpoint rather than walking to genesis. The precondition for safe local pruning. Open, see §L.

**(G, D) cursor** (inherited, Part 2 §7.4): a node's two-dimensional position, folded governance generation and dataplane position. History convergence advances the D component.

---

## B. Cast

**Alice** and **Bob**: two personae, both members of group G, each holding G's keys and a partial dataplane history. Alice has been offline and holds less. They converge.

**S**: a history store. A member of the mirror group G-hist, not of G. Holds sealed blobs and reconciliation envelopes. Content-blind.

**G**: the main group. Its members hold content keys and read content. **G-hist**: the mirror group. Its members are G's personae plus S, and it carries transport only.

---

## C. Two reconciliation questions, not one

`Realizes: P-Knowable-Truth`

The dataplane has two distinct reconciliation needs, and conflating them is the modeling error to avoid. They are different questions, not two speeds of one question.

- **Live delivery** answers "what just happened." It is a push, ordered, forward-moving, and forward-secret: the MLS message stream. MLS handles it, within a bounded reordering window (§M).

- **History convergence** answers "what am I missing." It is not a push and not about the newest thing. It is a set-difference between two parties who each hold partial histories: "I hold this and this, I can tell there are entries between them I lack, do you have them." It is shaped nothing like live delivery.

Both ride MLS. History convergence is not a separate cleartext channel; its requests and the transferred entries are MLS application messages, so the exchange stays inside the envelope, and any store involved relays sealed blobs it cannot read. The reconciliation logic runs between parties that hold keys; the store is storage and pipe. `Synthesis.`

MLS is explicitly not the backfill mechanism for the second question. It provides only a bounded tolerance for out-of-order delivery (`Verified`, RFC 9750, application parameters for kept nonce and key pairs and reorder tolerance). Beyond that window, catch-up is this layer's job.

---

## D. Two tiers, one primitive

`Realizes: P-Peer-Equality, P-Durable-Enablement`

There are two node tiers, and only two, on a single primitive.

- **Members** hold G's keys, reconcile finely, and read content.

- **History stores** are MLS members of the mirror group G-hist, hold sealed blobs and reconciliation envelopes, reconcile by chain over the envelope, and read nothing.

A prior draft considered a third tier, a structure-aware replica speaking Willow Confidential Sync over end-to-end-encrypted entries. It is **rejected** here, for two reasons that are the whole point of the tier design. It is a second cryptographic primitive, so soundness would depend on both MLS and Willow-e2e. And it is a second threat boundary, with its own envelope, metadata surface, and failure modes, whose interaction with MLS must also be reasoned about. The mirror-group store achieves the same availability goal on one primitive and one boundary, at the cost of coarser (chain-level, not path-level) reconciliation, which is an acceptable trade because path-level reconciliation by an untrusted node is not a requirement (§J). `Synthesis.`

**Both tiers are helpers with capability, not standing.** A history store is a helper: a persona or a k-of-n group admits it, it does work (holds and serves history), and it can be removed the same way. Neither tier holds any authority over a persona. It cannot foreclose on a member, remove anyone, or override governance; it can do nothing a persona could not authorize and revoke. So a history store is capability granted by delegation, never standing, and it is not a center however much it holds or serves (the general principle is stated in authority-and-complement.md). A content-blind store (holding sealed blobs, this document) and a clear-text helper admitted to a content scope (for search or indexing, which reads content by the same kind of grant) are two points on one spectrum: capability delegated without authority, differing only in how much the peers chose to reveal. The honest tradeoff is that revealing clear text to a helper expands the confidentiality surface to that helper, but it never transfers authority, and it is revocable. `Synthesis.`

---

## E. The mirror group and nested sealing

`Realizes: P-Knowable-Truth`

**The store is a member of G-hist for transport, never of G.** Membership in G-hist gives the store the mirror group's transport secrets, so members and the store share an authenticated, forward-secret channel over iroh with no IP exposure. Membership in G-hist does not give the store any content key.

**Content is sealed under G's content key (the per-scope asset key of the companion doc), which the store lacks.** A member takes an entry already sealed for G's members and sends it to the store as the opaque payload of a G-hist message. The store decrypts the G-hist transport layer, obtaining the reconciliation envelope in clear and the G-sealed content blob as opaque bytes. On retrieval, the store returns the blob over G-hist, and the member decrypts the G-hist layer and then the G content layer.

**Why nested sealing is mandatory, not stylistic.** MLS membership implies the ability to decrypt. The definition is that `Verified`, RFC 9420, RFC 9750: a member is a client with access to the group's secrets, and only members of a group can decrypt the payload of an application message. There is no MLS notion of a member who cannot decrypt. So a store that held G's key would read all content. Content-blindness is therefore achieved by the store holding only G-hist's key and never G's, with content sealed under G's key. `Verified` for the MLS property; `Design` for the nested construction.

**The envelope rides in the encrypted G-hist payload, not the AAD.** MLS additional authenticated data is authenticated but sent unencrypted (`Verified`, RFC 9420 framing, RFC 9750), so a relay would see it. The reconciliation envelope MUST therefore travel inside the encrypted G-hist payload, visible to the store as a G-hist member but not to iroh or any relay, with only the sealed content blob nested within. `Design.`

**Optional cryptographic binding.** MLS can derive a resumption pre-shared key from an epoch and use it to link epochs across groups (`Verified`, RFC 9420, RFC 9750). If desired, G-hist membership can be bound to G membership via such a link, so a persona's presence in G-hist is cryptographically tied to its presence in G rather than maintained as a separate roster. Whether to use this is open; the roster is in any case derived from the same governance chain (§F). `[confirm: exact binding construction.]`

---

## F. The store's Group Role Set

`Realizes: P-Peer-Equality`

**The store is admitted with a Group Role Set granting only carriage-and-service and excluding every other Group Role.** The single grant is a broad-plane carriage-and-durability role: hold sealed blobs, answer reconciliation, forward. Its mutual-exclusion property (Part 2 Group Role Set semantics) excludes every content role and every governance role, so the store can never hold a read grant, a content-write grant, a governance vote, or capability-issuing authority. The exclusion is folded from the governance chain as an attributable fact, so no silent escalation is possible: granting the store any further role would either violate its Role Set's exclusion (rejected by the fold) or require a visible, governed change every member folds. `Design.`

**The Role Set and key-withholding are complementary, and both are required.** MLS enforces no application access control (`Verified`, RFC 9750 §6.4, as cited in Part 2 §5.5), so the Role Set does not cryptographically prevent the store from decrypting a layer whose key it holds. It is the authorization statement, folded and auditable. Key-withholding (store holds G-hist's key, never G's) is the cryptographic enforcement. A compromised store is bounded by what it holds, which is sealed blobs and envelopes, so the residual exposure is metadata, not content. `Synthesis.`

---

## G. The reconciliation envelope

`Realizes: P-Knowable-Truth`

**Requirement.** A history store MUST detect gaps and serve missing spans of a member's history, over MLS transport, without holding any content key and without learning path structure, wall-clock time, or the access-control structure. It MUST expose no more than a blind meer already would, namely which chains exist, their lengths, their approximate append timing, and padded per-entry sizes, and MUST NOT expose Willow paths, plaintext payloads, or Meadowcap capabilities.

**Placement.** Each stored item is a pair inside a G-hist message: a cleartext-to-G-hist envelope, and a content blob sealed under G's content key (the companion doc's per-scope asset key), which the store lacks.

**Envelope fields (minimal).**

- `subspace_id`: the author device-subspace, as a hash or pseudonym. Per Willow e2e guidance, hash the meaningful id rather than expose it (`Verified`, Willow e2e). Lets the store bucket entries by chain.

- `predecessor_digest`: the content digest of this entry's predecessor in its chain, or a genesis or checkpoint marker. The hash link that makes a gap nameable.

- `entry_digest`: the content digest of this entry, and the address of its sealed blob. The reconciliation identity.

- `counter`: the per-subspace logical counter, monotonic and single-writer per device-subspace. Enables range reconciliation over (subspace, counter) and orders the chain without wall-clock.

- `size_hint`: the padded byte length of the sealed blob, for response sizing and storage planning. Padded so true payload length does not leak.

**Explicitly excluded.** The Willow `path` (hierarchy stays inside the sealed content, reconstructed by members after decrypt). Any wall-clock timestamp (a display time lives in the sealed payload as content, per §K). Any Meadowcap capability or AuthorisationToken (verification is deferred to member endpoints, §J). The raw `subspace_id` (hashed, not plain).

**Leak profile, stated.** The store learns which device-subspaces are active, each chain's length and append rate via counters, and padded per-entry sizes. It does not learn paths, content, wall-clock, or access structure. This is meer-level exposure, one tier below a member, on one primitive. `Synthesis.`

---

## H. Device-subspaces and the logical counter

`Realizes: P-Knowable-Truth`

**The write unit is the device-subspace, not the persona.** Each device holds a key in its persona's lineage and writes into its own subspace. Attribution to a persona is recovered from the lineage, not from a shared subspace. `Synthesis.`

This makes the logical counter clean. The Willow timestamp field carries a per-entry incrementing logical counter, which Willow explicitly blesses and which fully preserves deletion semantics while obscuring physical time, in the case of a single device writing a subspace (`Verified`, Willow data-model and e2e). One device per subspace is exactly that case, so the counter stays monotonic with no cross-device coordination, and the multi-device coherence problem is dissolved by making each device its own subspace rather than sharing one. `Design.`

Device pool membership is strictly key-lineage-based. A device is in a persona's pool if and only if it holds the persona's root key or a key that root stamped out. There is no partial membership. `Design.`

---

## I. Member-to-member history convergence

`Realizes: P-Knowable-Truth`

Alice and Bob converge over their pairwise sealed channel (a store may relay the sealed blobs, seeing only ciphertext).

- **Frontier exchange.** Each sends the other a frontier: a map of {device-subspace to (head digest, high-water counter)} for the subspaces it holds. Compact, roughly one head per active device-subspace.

- **Gap detection.** For each subspace, compare. If Bob's counter exceeds Alice's, Bob holds entries beyond Alice's head, so Alice has a gap. The head digests do the integrity work: if at equal counters the heads differ, the chain has diverged (a fork, handled by Part 2 §7.6), and if Alice's head appears earlier in Bob's chain, it is a clean linear gap. This mirrors the "which message am I missing" shape of MLS, applied per author chain.

- **Gap request.** Alice requests the missing span by naming the two bounding digests: her current head for that subspace, and Bob's head. Both lie in Bob's chain, so the request is unambiguous to both parties.

- **Sized response.** For a small span, Bob sends the entries. For a large span, Bob first signals the count and byte size so Alice can decide and allocate, then sends the sealed entries with their in-payload authorization.

- **Verify and fold.** Alice verifies each entry as she folds it, the author signature or in-payload token and the `predecessor_digest` linkage, walking from her old head to Bob's. Verification is at her endpoint, after unwrap, which is why intermediaries never needed the capabilities.

- **A valid signature is necessary but not sufficient for admission.** Backfill acceptance MUST require, before absorbing anything, both (a) a contiguous sequence over the span (a gapped or reordered `seq`, each message individually well-signed, is rejected) and (b) author standing on the branch's lineage at the relevant position (a message validly signed by a persona that never held standing on this lineage is rejected). A signature-deep check alone lets two illegitimate branches through: a stranger's perfectly-signed "history," and a tampered ordering of genuinely-signed entries. Binding admission to recorded standing plus structural contiguity, not faith in a signature, is what makes "validating an epoch chain you were not present for" safe. This is the gap-found-and-closed shape: a reconcile that accepted on signature alone imported forged history until the check was strengthened. `Verified` (backfill admission requires standing plus contiguity, Proofs Phase 2.5 A2.3).

- **Symmetry.** They run the request-and-transfer in both directions. When all subspace heads agree, they hold the union and are converged.

**Engine.** Willow's range-based set reconciliation subsumes the frontier scheme, since the subspace is one axis of the range, so the clean design is range-based reconciliation with a cheap head-digest first pass: exchange heads, skip where they match, recurse over counter sub-ranges by fingerprint where they differ. The explicit per-subspace frontier is the special case for a small or known-changed author set. Among members this can use full Willow reconciliation over plaintext paths, since members share keys. `Design.`

---

## J. History-store convergence

`Realizes: P-Knowable-Truth`

A member converges with the store S the same way, but S is content-blind, so the reconciliation runs over the envelope only.

- S reconciles over the (subspace, counter) space with `entry_digest` fingerprints, exactly the member-to-member engine, but blind: it compares digests and counters it cannot interpret as content.

- S does not run Willow's WGPS, because WGPS needs paths and S has none. It runs range reconciliation over the reduced envelope axes instead. This is the deliberate coarsening of §D. `Design.`

- **S performs no authorization verification.** Willow's authorization is a pluggable parameter, not necessarily Meadowcap (`Verified`, Willow data-model: the AuthorisationToken type and is_authorised_write function are parameters, Meadowcap is the default). So the write authorization rides inside the sealed payload and is verified at the receiving member, and S never sees a capability. S's only integrity duty is that `entry_digest` matches the blob it serves; a receiving member rejects unauthorized or malformed entries after unwrapping. `Design.`

- **Availability role.** S exists so a member can converge against a highly-available node rather than needing a specific peer online. A member reports its frontier to S, S serves the sealed spans S holds beyond it, the member unwraps and verifies. S can hold as much or as little history as configured (§L). `Synthesis.`

---

## K. Ordering is content, not trust

`Realizes: P-Knowable-Truth`

Two orderings, two trust levels, and the untrusted one is quarantined to presentation.

- **Convergence ordering** decides which entry overwrites which and how history links. It must be trustworthy, and it comes from the monotonic cryptographic chain, the `predecessor_digest` linkage, which is causal and cannot be forged. The logical `counter` is the local expression of chain position; what is trusted is the chain, not the counter's honesty, so even a wrong counter on a node's own entry degrades to a local presentation glitch, not a convergence error. `Design.`

- **Presentation ordering** decides what a UI shows first. It is content. A wall-clock `createdAt` lives in the sealed payload as a display hint and optional secondary sort, and is never trusted for anything structural. A lie about it misorders a feed, a social annoyance, not a security breach. `Design.`

The Willow timestamp field carries the logical counter, which is structural and must be trustworthy within a subspace and is anchored by the chain. Wall-clock time is payload. The two never share a field. `Design.`

---

## L. Pruning and checkpoints

`Realizes: P-Durable-Enablement`

**A node MAY prune old history to bound storage, but pruning below an anchor breaks reverse validation, so a checkpoint is required.** The `predecessor_digest` chain lets a verifier walk back to a trusted anchor. If a node prunes the tail below point X, it can no longer walk past X, and a fresh peer cannot validate the pruned prefix. The resolution is a verifiable checkpoint at the prune boundary, analogous to the governance snapshot cache with verifiable truncation (Part 2 §7.3.3): a corroborated commitment to the pruned prefix's head, so validation terminates at the checkpoint rather than genesis. Lazy loading of deeper history then means fetching older spans from a node that kept them and validating them against the same chain. `Design.`

This is the precondition for safe pruning and for lazy history loading, and its exact construction is open (Open items).

**Removal is not redaction: any "delete from history" surface MUST bound its promise to future access.** Pruning the stored copy, re-keying on a member removal, and re-encrypting all bound *future* access to the content: they stop the removed party from reading anything sealed after their removal folds, and they can shrink or drop what a store still holds. None of them can retract what an ex-member already saw or already pulled down. A removed member keeps the keys and the plaintext they held up to their ceiling (governance-finality.md A3, ban-is-not-deletion), and re-encryption controls only the copy the store or the remaining members now hold, not the copy that already left. So a product feature that offers "delete this from history" or "redact this message" must be honest that it forward-excludes, not that it un-sees: it removes the content from the going-forward record and from those who did not already have it, and it cannot guarantee the content is gone from a party who already read it. Overpromising here (implying deletion retracts past exposure) is the failure mode to avoid; the honest framing is that removal and re-encryption are forward-secrecy operations, not time machines. `Synthesis.`

---

## M. Willow and MLS conformance

`Realizes: P-Knowable-Truth`

The load-bearing external facts this design rests on, checked this cycle against primary sources.

**MLS (RFC 9420, RFC 9750), Verified.**

- A member has access to the group's secrets, and only members of a group can decrypt an application message payload, which includes sender information. So content-blindness requires nested sealing under a withheld key, not a special membership (§E).

- MLS enforces no application-level access control (RFC 9750 §6.4). So the store's Group Role Set is an application-layer authorization statement, complementary to key-withholding, not a substitute for it (§F).

- Additional authenticated data is authenticated but unencrypted. So the reconciliation envelope rides in the encrypted payload, not the AAD (§E, §G).

- The Delivery Service selects a single Commit per epoch, giving a linear commit order. So placing the store in a separate mirror group avoids store churn contending with content commits, consistent with the governance and epoch decoupling of the companion doc.

- MLS tolerates only bounded out-of-order delivery. So MLS is not the history-backfill mechanism, and history convergence is this layer's responsibility (§C).

- A resumption PSK can link epochs within or across groups, an available mechanism for binding G-hist to G if desired (§E).

**Willow (willowprotocol.org specifications), Verified.**

- The Entry carries metadata (namespace, subspace, path, timestamp, payload length and digest) and the Payload is content-addressed and stored separately. The envelope-and-content split maps onto this, except that the Entry contains the path, so the store holds a reduced envelope rather than a full Entry and is a log layer beneath the Willow model, not a Willow store (§G, §J).

- The AuthorisationToken type and is_authorised_write function are parameters; Meadowcap is the default, not a requirement. So authorization can ride in the sealed payload and be verified at the member, leaving the store capability-blind (§J).

- A Willow store is a state-based CRDT under join, with the newer relation ordered by timestamp, then payload digest, then payload length. This is the members' local convergence for same-path entries.

- The timestamp is a U64 that may be a logical counter, and this is clean for a single device writing a subspace, which the device-subspace framing guarantees (§H).

- End-to-end Willow encryption would hash the subspace id rather than encrypt it, use prefix-preserving path encryption, and cannot encrypt Meadowcap capabilities from a verifier that lacks keys. This document avoids that second envelope entirely by using the MLS transport and deferred verification, so those constraints do not bind here (§D). The subspace-id hashing guidance is nonetheless adopted for the envelope's `subspace_id` (§G).

**Willow last-writer-wins, mutable-mode merges, and completeness, in one place.** Three implications of the points above, stated together because they are otherwise scattered across §H, §K, and §L:

- **Per-device subspace neutralizes Willow's last-writer-wins.** Willow's newer relation (timestamp, then digest, then length) resolves only same-subspace, same-path collisions, and the device-subspace framing (§H) removes those: each subspace has one writer ordering by its own monotonic counter, so within a chain there is no concurrency to resolve, and across chains entries coexist under the union, since Willow's prefix-pruning is same-subspace only. Two devices of one persona are two subspaces, so the case Willow resolves silently, two offline writers overwriting one path, does not arise; the excluded `path` (§A) in the forward-only tier means there is no path-addressed overwrite at all. The unverified wall-clock is presentation content, never the ordering key (§K).

- **Mutable-mode shared resources need a semantic read-merge.** Where a bounded Group opts into the Willow-mutable tier (Part 2 §7.8), path-addressed overwrite returns, but only within a subspace, where it is one writer replacing its own earlier entry under the monotonic counter, which is intentional and not a silent drop. A resource written from several device-subspaces still yields several coexisting values, and reconciling them to one is a read-time merge that **MUST** be semantic, a CRDT in the sealed payload or a mutation routed through the causal governance plane, and **MUST NOT** be an application-level timestamp last-writer-wins, because that would re-inherit at the application layer exactly the silent concurrent-overwrite loss the device-subspace framing removes.

- **Completeness is the one beam, discharged the same way as governance.** The union gives convergence but not completeness: a node must know whether it holds every entry, or it can present an incomplete union as complete. This is not a second open problem; it is the same gap-completeness beam the governance fold carries (Part 2 Appendix B), and the same mechanism discharges it. The per-chain structure earns the provable half directly, since counter-contiguity plus predecessor hash-links prove completeness *behind* a checkpoint (§L) for each device-subspace. The frontier half, holding every subspace's latest head, is completeness-*ahead*, established by the same quorum corroboration the governance now uses, and applied here at the weaker read and enforce setting (Part 2 §7.4): the content plane qualifies freshness on it and never gates reads, rather than failing closed as governance enforcement does. One mechanism, one beam, two strengths.

---

## N. Posture summary

| Concern (§) | Naive assumption | Drystone posture | Forcing principle |
|---|---|---|---|
| Two reconciliation needs (§C) | One mechanism, two speeds | Distinct questions: live delivery (MLS push) versus history convergence (set-difference over chains) | MLS bounded reorder window (Verified) |
| Node tiers (§D) | Blind relay or full member only | Two tiers: members, and content-blind history stores; a Willow-e2e replica tier is rejected | One primitive, one threat boundary |
| Store as MLS member (§E) | Member can be told not to read | Member implies decryption, so blindness is nested sealing under a withheld key | RFC 9420, RFC 9750 (Verified) |
| Store authority (§F) | Configure it as read-only | Group Role Set excludes all other roles, folded and auditable, plus key-withholding | RFC 9750 §6.4, no MLS access control (Verified) |
| Envelope placement (§E, §G) | Put metadata in AAD | AAD is unencrypted, so envelope rides in the encrypted payload | RFC 9420 framing (Verified) |
| Store reconciliation (§J) | Store runs Willow WGPS | WGPS needs paths, so store runs chain-range reconciliation over the reduced envelope | Willow Entry contains path (Verified) |
| Store verification (§J) | Store checks capabilities | Willow auth is pluggable, so auth rides in payload and members verify; store is cap-blind | Willow pluggable AuthorisationToken (Verified) |
| Ordering (§K) | Timestamp orders history | Chain linkage orders history (trusted); wall-clock is presentation content (untrusted) | P-Knowable-Truth |
| Pruning (§L) | Prune freely | Pruning below an anchor needs a corroborated checkpoint for reverse validation | Part 2 §7.3.3 analog |

---

## O. Residuals owned

- **The store sees meer-level metadata.** Active device-subspaces, chain lengths and append rates, padded sizes. Content, paths, wall-clock, and access structure stay hidden. This is the stated cost of a highly-available durability tier, and it is one tier and one boundary, not two.

- **Coarser reconciliation at the store.** Chain-level, not path-level. Acceptable because path-level reconciliation by an untrusted node is not a requirement, but it means the store cannot serve path-scoped range queries, only chain-position spans.

- **A compromised store leaks metadata, not content**, bounded by what it holds. This is the reason to prefer the withheld-key plus Role-Set construction over making the store a full member.

- **Reverse validation depends on a reachable anchor.** Until the checkpoint construction (§L) is pinned, local pruning below genesis is unsafe, because a pruned prefix cannot be validated.

---

## Open items

- **Dataplane checkpointing (§L).** The construction of a corroborated checkpoint at a prune boundary, analogous to Part 2 §7.3.3, is required before safe pruning and lazy deep-history loading. Not yet pinned. `Design; open.`

- **Reconciliation granularity (chunking).** Whether the store reconciles at the granularity of whole sealed entries (coarse, one blob per entry) or finer content-addressed chunks (so a large entry or a large history reconciles in pieces, closer to the chunked compress-then-encrypt idea from the local-first prior art). A durability-layer choice, independent of the primitives. `[confirm.]`

- **G-hist to G binding (§E).** Whether to bind mirror-group membership to main-group membership via a resumption PSK link, and the exact construction. `[confirm.]`

- **Store admission and spam bound.** Since the store defers verification, it accepts what it is given, so a coarse proof-of-entitlement to talk to the store at all is wanted to bound spam and denial of service. Its shape is unspecified. `Design; open.`

---

## Changelog

`Working draft; per doc-method §1, transitions are recorded here so the body stays an end state.`

- **Draft, first consolidation.** Establishes the two reconciliation questions (§C), the two-tier model with the Willow-e2e replica tier rejected (§D), the mirror group and nested sealing (§E), the store's content-excluding Group Role Set (§F), the reconciliation envelope schema (§G), the device-subspace and logical-counter framing (§H), the member-to-member and store convergence flows (§I, §J), the ordering-is-content principle (§K), and pruning-needs-a-checkpoint (§L). Consolidates the MLS and Willow conformance checks performed this cycle (§M).

- **Companion linkage.** Content payloads here are sealed under the per-scope asset keys defined in asset-keying.md, and the fold-gated provisioning there governs their rotation. That document and this one share the same MLS-transport and one-boundary posture and should be read together. Repoint against Part 2 §7.6 (fork) and §7.3.3 (snapshot cache) when this material is folded.

- **Tag posture.** External facts about MLS and Willow are `Verified` against RFC 9420, RFC 9750, and the Willow specifications this cycle (§M). The tier model, mirror-group construction, envelope, and convergence flows are `Design`. Checkpointing, chunking, G-hist binding, and store admission are open or `[confirm]`. Nothing here is `green-real`.

- **Doc-model normalization.** Normalized epistemic tags to the suite's canonical set for spec-readiness: the compound `design/Synthesis` (and `design/ENABLING`) became `Design`, `Verified, research pass` became `Verified`, and bare lowercase `design` became `Design`. The `Realizes: P-<Principle>` linkage tags are retained as the principle-crosswalk layer, which now resolves against the catalog in authority-and-complement.md. No substantive claims changed.
