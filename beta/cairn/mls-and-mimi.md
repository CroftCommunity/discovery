# Drystone: MLS (RFC 9420) and MIMI, as Building Blocks

`Status: survey, current as of mid-2026. MLS is a finished RFC; MIMI is still Internet-Drafts and moves. Treat MIMI draft numbers and the proof/analysis citations as a snapshot, not a fixed state.`

`Scope: what MLS provides, how it decomposes into three provable layers, where the interop story (MIMI) reintroduces a hub, what actually caps MLS at scale (spoiler: not the crypto), and how Drystone uses MLS as a subordinate key-distribution backplane rather than as its delivery layer.`

`Companion to: cairn/adjacent-systems (the two-axis landscape where MLS/MIMI sit as the standards row), the drystone-spec layer (which cites RFC 9420/9750 for the parts it uses), and beta/impl/mls/ (the RFC-grounded MLS substrate bundle this doc summarizes).`

This is a survey and positioning document, not a specification. Provenance flags: `Grounded` (drawn from the source transcripts, which themselves grounded the claim in the RFC or a named paper), `Framing` (this layer's own synthesis or emphasis), `[confirm]` (flagged in the source as not fully verified and worth re-checking as it evolves).

---

## Why MLS is in the cairn

MLS is one of the parts that bubbles up into the spec. The Drystone spec cites RFC 9420/9750 directly for the pieces it uses; this doc is the wider survey behind that citation, kept here so the reasoning for the choice, and the honest caveats, live in one place rather than being scattered across the spec's footnotes.

The short version of the case, assembled and defended in the source research: MLS is the only group end-to-end-encryption protocol that is simultaneously an open IETF standard, formally verified at its core, deployed at consumer scale, and backed by multiple vendors. `Grounded.` That combination is rare enough that reaching for it, rather than designing a new group-key protocol, was the pragmatic call.

## What MLS provides

MLS (RFC 9420) standardizes group end-to-end encryption. `Grounded.` Its guarantees, as grounded against the RFC in the source material:

- **Group E2EE with forward secrecy (FS) and post-compromise security (PCS).** A group is a logical collection of clients sharing a common secret at a given time; its state advances as a linear sequence of epochs, each depending on its predecessor. `Grounded.`

- **Delivery-agnostic.** MLS deliberately does not specify how messages reach members. RFC 9750 leaves the Delivery Service (fan-out client-side, server-side, or mixed) to the application. `Grounded.` This is the property that lets Drystone carry MLS over its own transport rather than a vendor's.

- **A clean actor model.** The source resolved a common conflation against the RFC: a user has one or more devices; a device may host one or more clients; each client is one group member holding one MLS leaf, one signature key, and one credential. The Delivery Service is the largely-untrusted node that orders and relays; it can censor and observe metadata but cannot forge or decrypt. `Grounded.`

One narrow but load-bearing use: Drystone carries hash-tree payloads inside MLS `application_data` as opaque bytes. The feasibility review confirmed this is a sound use of MLS as an exchange plane, and that it is distinct from binding those payloads into MLS's own transcript hash (which Drystone does not need). `Grounded.`

## The three-layer decomposition

The clean way to explain MLS, from the Wallez / Protzenko / Beurdouche / Bhargavan work, is a one-directional stack of three layers:

- **TreeSync**: authenticated group state. Signatures plus Merkle-style hashes. Its job is "the server cannot forge who is in the group," a property motivated by the double-join attack. `Grounded.`

- **TreeKEM**: the group key agreement: log-rekey, and the source of FS and PCS. `Grounded.`

- **TreeDEM**: per-message encryption keys, giving fine-grained forward secrecy. `Grounded.`

The dependency runs one way (TreeSync, then TreeKEM, then TreeDEM). `Grounded.`

### The proofs, and where they weaken

MLS is unusually heavily proven, and the proofs found real limits. Machine-checked symbolic proofs (Dolev-Yao, primitives treated as black boxes; symbolic, not computational) exist on bit-precise specs: TreeSync (USENIX '23, Distinguished Paper and Internet Defense Prize) and TreeKEM (IEEE S&P 2025). These two compose. `Grounded.`

The external-operations analysis (Cremers et al., Eurocrypt 2026) found the shipped standard weaker than earlier drafts had been proven to be, in two specific ways:

- **Signature strength.** MLS needs an SUF-CMA signature (as with IETF Ed25519), not merely EUF-CMA (as with ECDSA), or the proof does not hold. `Grounded.` Cipher-suite choice is therefore security-relevant, not cosmetic.

- **PCS is lowered by external operations.** External operations reduce PCS to session-state healing rather than full-state healing. An attacker holding your long-term key can re-insert themselves via external resync. `Grounded.` This is why RFC 9750's HSM and long-term-key protection is load-bearing rather than optional.

These are caveats to raise first, not fine print: MLS alone is not interop (it needs MIMI, still draft), PCS has provable limits and degrades under external ops, and the cipher-suite choice materially affects the proofs. `Grounded.`

## MIMI: the interop half that reintroduces a hub

MLS is half of a two-protocol story. It standardizes the encryption; cross-vendor interoperability needs a second protocol, MIMI (More Instant Messaging Interoperability, an IETF working group), for identity, transport, addressing, and the introduction problem. `Grounded.` As of the source research MIMI was still Internet-Drafts (draft-ietf-mimi-protocol-05, October 2025), authored cross-vendor (Barnes/Cisco, Hodgson and Ralston/Matrix, Kohbrok and Robert/Phoenix R&D, Mahy); its identity layer contemplates X.509 and W3C Verifiable Credentials. `Grounded.`

The structurally important point: MIMI reintroduces a hub. Each MIMI room is hosted at a single provider, and one provider (the hub) orders messages and is trusted to enforce room policy, a cost the draft accepts explicitly "for simplicity." `Grounded.`

So the field splits into three pieces: the privacy standard exists (MLS), the interop standard chose a per-room hub (MIMI), and the capture-resistant delivery layer is exactly the piece left unstandardized. That last piece is the seam Drystone occupies. `Framing`, drawn directly from the source's own framing.

## The scaling reality: coordination, not crypto

The headline from the MLS scaling research is that commit serialization and the Delivery Service, not cryptography, are the true scaling bottlenecks. `Grounded.`

- **Commit serialization.** MLS advances the group key in a strict chain of epochs; only one commit can close each epoch. Two members committing concurrently off epoch N both aim at N+1; one wins, the other is rejected and must rebuild on the new epoch and retry. Application messages do not hit this (they are encrypted once under the current group key, at a cost independent of group size); only key-changing operations (join, leave, key rotation) become commits. So the limiter is the rate of concurrent commits, not member count. `Grounded.` The crypto per commit is milliseconds; the contention is the cost.

- **It is not literally one global lock.** The Delivery Service can pick a winner and batch or retry, and schemes like CoCoA relax "one commit per epoch" by allowing concurrent updates at the price of extra rounds. But the default shape is single-file. `Grounded.`

- **The Delivery Service is where latency lives.** Something must pick the winning concurrent commit and impose ordering: the DS, the untrusted server layer that relays and sequences. The Soler et al. 2025 measurement study (OpenMLS, up to 5,000 members) found the inconsistency window (commit sent, not yet processed by all members) reached roughly 2 seconds, dominated by DS plus network, orders of magnitude above the millisecond crypto. `Grounded.` RFC 9750 deliberately does not standardize the DS, so scale, fan-out, and the storage and serving of the large Welcome and GroupInfo objects are the deployer's problem. `Grounded.`

- **The production fix.** Webex and Cloudflare converge on the same answer: a single designated committer plus batched membership changes, which sidesteps collisions rather than making the crypto faster. `Grounded.` The source notes that the designated-committer role is mechanical and needs no governance authority, so it composes with peer-symmetry. `Grounded.`

The takeaway for anyone reading MLS's near-logarithmic TreeKEM cost as "MLS scales fine": TreeKEM is cheap and near-logarithmic in the good case; what caps a real deployment is coordination (one ordered slot) and the server round-trips to enforce it. `Grounded.`

## How Drystone uses MLS

Drystone uses MLS as a subordinate key-distribution backplane, and routes around both scaling bottlenecks by moving most ordering off the epoch chain. `Grounded`, with the framing below drawn from the source's "explain both" follow-on (which became the spec's scaling-and-ordering material and Part 2).

- **Most changes are not key changes.** Content and authority-only governance changes are not key changes, so they do not hit the single commit slot. `Grounded.`

- **The governance fold is order-independent.** This is the load-bearing assumption, and the source explicitly tracks it as such. `Grounded.`

- **History and catch-up are served off a content-blind store**, not off the epoch chain. `Grounded.`

The source is careful about the honest concessions, and this doc keeps them:

- You still inherit the roughly 2-second inconsistency window per commit, though commits are rarely entered. `Grounded.`

- The MLS-state half of catch-up (the ratchet tree a joiner ingests via Welcome/GroupInfo) is still MLS's and still linear in group size. The tiering offloads content-history catch-up, not cryptographic-state catch-up. `Grounded.`

- The ordering work is relocated to per-node deterministic computation plus convergence bandwidth, not eliminated. No free lunch. `Grounded.`

## Adoption (why the bet is not lonely)

For completeness, the source's adoption survey, with its own confidence flags: GSMA RCS Universal Profile 3.0 with Apple and Google (2026); Discord "DAVE" (E2EE audio/video, a non-chat use of MLS as a group-key primitive, flagged `[confirm]` because it came from a reference list rather than Discord's own blog); Wire; Cisco and RingCentral via OpenMLS; AWS `mls-rs` (Rust, RFC 9420 conformant, with no full third-party audit noted). `Grounded`, subject to the source's own caveats.

## What Drystone takes vs leaves

**Takes:**

- MLS (RFC 9420) as the group-E2EE and key-distribution layer, used delivery-agnostically and carried over Drystone's own transport. `Grounded.`

- The `application_data` channel, to carry hash-tree payloads as opaque bytes without binding them into MLS's transcript. `Grounded.`

- The cipher-suite discipline the proofs demand (Ed25519-class SUF-CMA signatures) and the RFC 9750 long-term-key protection the external-ops result makes load-bearing. `Grounded.`

- The production scaling insight (route ordering off the epoch chain; keep key-changing commits rare), adapted rather than adopted wholesale. `Framing.`

**Leaves:**

- MIMI's per-room hub. Drystone occupies the unstandardized capture-resistant delivery seam precisely instead of accepting a single trusted per-room provider. `Framing`, from the source.

- MLS's own Delivery Service model as the delivery layer. Drystone supplies its own delivery and ordering rather than inheriting the DS bottleneck as designed.

- Any claim that MLS alone solves the problem. It does not: it is one subordinate layer under Drystone's governance and delivery, with FS/PCS limits (session-state PCS under external ops) that Drystone must account for rather than assume away. `Grounded.`
