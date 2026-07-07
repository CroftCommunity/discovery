# Raw transcript — MLS scaling, Willow/Meadowcap, the adjacent-systems landscape, the feasibility reviews, and the cairn layer proposal (2026-07-07)

**Preservation status: preserved-condensed (cleaned-paste, content-faithful — §4).** User-pasted tail of a
long multi-thread claude.ai session (batch ten). UI chrome and repeated edit/search narration condensed to
substance; the ecosystem-survey threads (MLS scaling, Willow, adjacent systems, ATProto lexicons) are
preserved content-faithfully because they are the source for the `cairn/` layer. The spec-process threads
(p11 rebuild, feasibility reviews) are summarized (their outputs are the committed spec + the feasibility
review doc). Content-faithful, not a byte-pristine export.

Outputs from this session (filed batch ten + document-pass-8): the cairn layer, the p10/p11 spec swap, the
feasibility review v2 (`beta/drystone-spec/feasibility-review-v2.md`), the frozen corpus
(`seeds/p10-p11-corpus/`), and the cairn distillation docs this transcript sources.

---

## Thread A — MLS scaling: what's demonstrated (the survey behind cairn/mls-and-mimi)

Deep research on MLS scaling. Headline: **commit serialization and the Delivery Service, not cryptography,
are the true scaling bottlenecks.**

- **Commit serialization.** MLS advances the group key in a strict chain of epochs; only one commit can
  close each epoch. Two members committing concurrently off epoch N both aim at N+1; one wins, the other is
  rejected and must rebuild on the new epoch and retry. Application messages do **not** hit this (encrypted
  once under the current group key, cost independent of group size); only key-changing operations (join,
  leave, key rotation, and in Drystone's case every key-changing governance action) become commits. So the
  limiter is the **rate of concurrent commits**, not member count. The crypto per commit is milliseconds;
  the contention is the cost. Not literally one global lock: the DS can pick a winner and batch/retry, and
  schemes like CoCoA relax "one commit per epoch" by allowing concurrent updates at the price of extra
  rounds. But the default shape is single-file.
- **The Delivery Service.** Something must pick the winning concurrent commit and impose ordering: the DS,
  the untrusted server layer that relays and sequences. Latency lives here, not in crypto: the Soler et al.
  2025 measurement study (OpenMLS, up to 5,000 members) found the inconsistency window (commit sent, not yet
  processed by all) reached ~2 seconds, dominated by DS + network, orders of magnitude above the
  millisecond crypto. And RFC 9750 deliberately does not standardize the DS (fan-out client-side,
  server-side, or mixed, left to the application), so scale, fan-out, and storage/serving of the large
  Welcome/GroupInfo objects are the deployer's problem.
- **Why not cryptography.** TreeKEM is cheap and near-logarithmic in the good case. What caps a real
  deployment is coordination (one ordered slot) and the server round-trips to enforce it. Production systems
  (Webex, Cloudflare) converge on the same fix: a **single designated committer** plus batched membership
  changes, which sidesteps collisions rather than making crypto faster. (The designated-committer role is
  mechanical and needs no governance authority, so it composes with peer-symmetry.)
- **Drystone's relation to this** (the "explain both" follow-on, which became `impl/drystone-design/
  scaling-and-ordering` and Part 2): Drystone routes *around* both bottlenecks by moving most ordering off
  the epoch chain: content and authority-only governance changes are not key changes, so they don't hit the
  slot; the governance fold is order-independent (the load-bearing assumption, tracked); and history/catch-up
  is served off a content-blind store. The honest concessions: you inherit the ~2s window per commit (rarely
  entered); the MLS-state half of catch-up (the ratchet tree a joiner ingests via Welcome/GroupInfo) is
  still MLS's and still linear in group size (the tiering offloads content-history catch-up, not
  cryptographic-state catch-up); and the ordering work is relocated to per-node deterministic computation +
  convergence bandwidth, not eliminated (no free lunch).

## Thread B — MLS as a building block: MIMI, the proofs, the decomposition (cairn/mls-and-mimi)

(This complements the batch-8 transcript's MLS-journey thread; both feed `cairn/mls-and-mimi.md`.)

- **MLS is half a two-protocol story.** MLS (RFC 9420) standardizes the encryption; cross-vendor interop
  needs **MIMI** (More Instant Messaging Interoperability, IETF WG, drafts as of 2026) for identity,
  transport, addressing, and the introduction problem. MIMI reintroduces a hub: each MIMI room is hosted at
  a single provider, and one provider (the hub) orders messages and is trusted to enforce room policy, a
  cost the draft accepts "for simplicity." So the privacy standard exists, the interop standard chose a
  per-room hub, and the capture-resistant delivery layer is exactly what is left unstandardized. **That is
  the seam Drystone occupies.**
- **The three-layer decomposition** (Wallez/Protzenko/Beurdouche/Bhargavan): TreeSync (authenticated group
  state, signatures + Merkle-style hashes; the layer whose job is "the server can't forge who's in the
  group," motivated by the double-join attack) → TreeKEM (the group key agreement, log-rekey, FS/PCS) →
  TreeDEM (per-message keys, fine-grained forward secrecy). Machine-checked symbolic proofs on bit-precise
  specs for TreeSync (USENIX '23) and TreeKEM (IEEE S&P 2025), which compose. The external-ops analysis
  (Cremers et al., Eurocrypt 2026) found the shipped standard weaker than earlier drafts: needs SUF-CMA
  (Ed25519) not merely EUF-CMA (ECDSA), and external operations lower PCS to session-state rather than
  full-state.

## Thread C — Willow and Meadowcap: the correct mental model (cairn/willow-meadowcap)

The user's question: is Willow a data-struct-object exchanged between peers, or closer to a CRDT structure
nodes hold locally and reconcile? The grounded answer (also verified in the feasibility pass):

- Willow's **data model is a state-based CRDT (a join-semilattice)**: merging replicas is order-independent
  and idempotent. An Entry is metadata (namespace_id, subspace_id, path, timestamp, payload_length,
  payload_digest); the subspace is the writer dimension. Merge = set union then a deterministic reduction:
  prefix pruning (a newer entry at a prefix prunes older beneath it), then tiebreak by greater timestamp,
  then greater payload_digest, then greater payload_length. So single-writer union is last-writer-wins per
  path by timestamp, plus hierarchical delete, over a pruned grow-set. Nodes hold state locally and reconcile
  ranges; they don't ship the whole thing (Willow has its own confidential sync + range-based set
  reconciliation).
- **The wrinkle worth remembering.** The timestamp is a writer-assigned, "claimed" U64 (recommended µs of
  TAI since 2000, but only a recommendation, unenforceable). It is not a logical/causal clock. So
  single-writer removes inter-author concurrency but not same-author-across-devices concurrency; different
  timestamps → later wins, earlier silently pruned; identical timestamps → the digest tiebreak fires and the
  survivor is content-arbitrary. Willow tracks no causality, so a concurrent conflicting edit and an
  intentional sequential overwrite are indistinguishable: convergence is guaranteed, semantic fidelity is
  not, and loss is silent. **Relevance:** any layer that needs to detect concurrency or preserve conflicting
  events (a governance/authority fold) must carry that itself (per-writer subspaces + its own logical clock
  + merge rule on top). This is the same completeness concern as Drystone's cheap-merge open thread.
- **Meadowcap** is Willow's capability system: unforgeable grants with attenuation by subsetting (the
  Alfie/Betty minting example). **Maturity:** Data Model + Meadowcap are Final; Confidential Sync, the Drop
  Format, and the Willow'25 parameter set took breaking changes into 2026; the implementations are pre-1.0
  (TS 0.x, Rust newer, Earthstar 11 beta). Same "not-ready-as-a-dependency" profile as Roomy and p2panda hit.

## Thread D — the adjacent-systems landscape (cairn/adjacent-systems)

From the local-authority-collaboration notes' ecosystem survey, rated on two axes (capture-resistance and
privacy):

- **Roomy** (atproto): a two-pivot journey. Reached for p2p stacks (Willow → Keyhive → Jazz), each dropped
  because the tech wasn't ready; built its own off-protocol server (Leaf, event-sourced per-space sync) in
  late 2025 for private data / realtime / notifications; swung back atproto-native in 2026 once Bluesky's
  permissioned-data proposal landed, adding **The Arbiter** (a per-community group-membership service: root
  DID, eight cumulative access levels, recursive space-in-space delegation, an $admin space; access levels
  govern only the arbiter, not the app). GA July 1 2026.
- **p2panda**: one ground-up rewrite (late 2024) to modular, CRDT-agnostic crates reusing iroh; then an
  additive build-out of hard parts (offline networking, persistence, group + message encryption ~
  Double-Ratchet-for-offline-groups, decentralized access control with a "pull" level that relays without
  reading, p2panda-spaces). Pre-1.0 mid-2026. **The contrast:** Roomy is a customer changing suppliers
  because the tech wasn't ready; p2panda is a supplier doing the research and shipping the hard parts as they
  ripen.
- **Standards:** MLS (RFC 9420) is the group-E2EE standard, delivery-agnostic; MIMI reintroduces a per-room
  hub. The privacy standard exists; the capture-resistant delivery layer is the unstandardized seam.
- **Implementations on the two axes:** SimpleX (closest to the spec: no user identifiers at all, double
  ratchet + PQ, envelope-encrypted delivery metadata, self-hostable relays, multiple Trail of Bits audits;
  achieves graph-blindness by *deleting identity*; groups least mature). Briar / Cwtch (strongest
  capture-resistance: no servers / Tor-only, device-to-device, metadata-resistant, audited; niche,
  small-group). Matrix + Element (most mature "both": federated, self-hostable, E2EE; but room/membership
  metadata on homeservers, heavy de-facto centralization; the Foundation co-authors MIMI). Session
  (decentralized but reportedly dropped forward secrecy). Nostr (capture-resistant, credible exit, but
  public-by-default; private messaging bolted-on and leaky). **Conclusion:** the "both" corner is either
  niche-and-young or mature-and-metadata-leaky, and the standards bodies took the per-room-hub shortcut,
  which is the structural reason the corner is empty and the space Drystone occupies.

## Thread E — the ATProto lexicon research (already filed to cairn/social-lexicon-group-research-brief)

A Research-feature run (288 sources) produced the ATProto content-lexicon + group/community data-model
report (D1–D7: the Bluesky lexicon catalog with grow-set vs single-writer-mutable flavors; a cross-platform
group feature matrix Reddit/Facebook/LinkedIn/Discord; a membership/moderation lifecycle model; the
reuse-vs-custom grid; a professional-vs-casual finding; prior-art notes Frontpage/Roomy/the Arbiter; a draft
group lexicon sketch). Two fact-corrections grounded first: the atproto MST **fanout is 4** (not 16; count
leading zero bits in 2-bit chunks, SHA-256); and an atproto repo is **not append-only** (single-author,
signed, but a mutable key/value store with put/update/delete and a causal commit chain; last-writer-wins per
key is the substrate default, grow-set is an application-imposed discipline). Filed as
`cairn/social-lexicon-group-research-brief.md` + `cairn/atproto-ecosystem.md`.

## Thread F — the feasibility reviews (spec process; output = feasibility-review-v2)

Two passes reviewing the Drystone spec for feasibility. Pass 1 (before filesystem access) raised issues
including an FS-vs-durable-history concern. Pass 2 (grounded in the full spec text + RFC re-verification)
**withdrew** that finding: §8.1 is correct against RFC 9420 §16.6 (FS assumes the adversary already holds
the ciphertext and delivers by deleting keys, so a durability node holding sealed bytes is inside the
scenario FS survives; the real friction is key retention). Confirmed: MLS is a sound exchange plane for
hash-tree payloads carried in `application_data` (opaque bytes), distinct from binding into MLS's own
transcript (which Drystone does not need); the completeness beam is non-monotonic (CALM) but the enforcement
gate is safety-monotone under fail-closed; §7.4 targets bounded/detectable/fail-closed consistency, not
strict "this is the latest." Output: `beta/drystone-spec/feasibility-review-v2.md`.

## Thread G — the cairn layer proposal

> **User:** I think I have another layer split, we could call it ecosystem or "cairn": our stackable parts.
> It's between history, philosophy, cairn, drystone. Drystone first had to survey the field to understand if
> the ecosystem has the parts needed to assemble a safe amount of novelty practically; designing iroh, MLS,
> CBOR-dag, or willow first was too heavy a blocker. We want to catalogue the solutions, products, libraries,
> and offerings in the broader space (cairn), inclusive of AP, atmosphere, AT, p2p, CRDT, QUIC, etc. Some
> bubbles into the spec, some doesn't, but the spec is focused on the spec narrative. Roomy has nowhere to be
> represented; tracking and linking it is the network-effect-producing cairn's job. It's almost the inverse
> of activism: the field of existing bolstering tech.

Confirmed as **Layer 3 `cairn/`**, between philosophy (2) and drystone-spec (4), the inverse of activism
(activism indicts the incumbents; cairn credits the enabling tech). Renumber applied; `cairn/` seeded.

---

*End of preserved tail. The next session (Claude Code, filing) created the cairn layer, executed the p10/p11
spec swap (document-pass-8, audit-gated, re-plant fold preserved), and distilled this transcript's ecosystem
threads (A–D) plus the batch-8 Blacksky/MLS-journey threads into the `cairn/` layer docs.*
