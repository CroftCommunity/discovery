# Croft Protocol — wire specification (with inline proofs)

date: 2026-06-16
status: DRAFT v0.1 — normative spine, built up from the spikes + TEST-PLAN. Each section pairs the
normative rule with the proof that demonstrates it (or marks it design-only). This is the document a
second implementer would build against; the conformance suite (`conformance-suite.md`) is what such
an implementation must pass, and the open-edges review (`open-edges.md`) is what this spec does not
yet pin down.

> **How to read this.** Normative keywords are **MUST / SHOULD / MAY** (RFC 2119 sense). Every
> normative block is followed by a **Proof:** line citing where it is demonstrated and its status:
> `green-real` (real crypto/transport), `green-model` (proven in the TS/Rust model), `design` (specified,
> not yet proven). "Verify against source, never guess" — all encodings below are transcribed from the
> implementing code, not invented; where a value is illustrative it is marked.

---

## 0. Conventions and versioning

- All multi-byte integers are **little-endian** unless stated. Hashes are **SHA-256** (32 bytes).
  Signatures are **Ed25519** (64 bytes), RFC 8032, deterministic.
- On-the-wire envelopes in the current spikes are **JSON** (serde); a production profile MAY adopt a
  canonical binary encoding, but the **signed pre-images below are byte-exact and MUST NOT change**
  with the envelope encoding (signatures are over the pre-image, not the JSON).
- **Version negotiation (design).** Every signed pre-image begins with a version tag
  (e.g. `"msg-v1"`, `"croft-lineage-genesis:"`). An implementation **MUST** reject a pre-image whose
  tag it does not recognize, and **MUST NOT** silently coerce an unknown version. Version is part of
  the signed bytes, so it cannot be downgraded in transit without breaking the signature.
  *Proof: design — the tag discipline is present in every pre-image below; a negotiation handshake is
  not yet specified.*

---

## 1. Cryptographic foundation — identities and signatures

- A **device** holds an Ed25519 keypair. Its public half is its verifying identity; the secret half
  signs. A **DID** (`Did`, a string) names a logical actor; multiple devices may act under one DID's
  **lineage** (§5).
- A signature **MUST** verify against the author's published verifying key before any other check.
  A valid signature is **necessary but not sufficient** — standing (§6) is also required.

  Proof: Phase 1 crypto gate (real Ed25519/MLS, `Proofs/lineage-groups`); carried over live iroh in
  the **faithful path** (`experiments/iroh/.../altdrive-spike-faithful-sync`): a FORGED message →
  `REJECT BadSignature` on both joiners incl. the NAT Mac. *green-real.*

---

## 2. Identifiers and derivations

The **wire identity** derivations are SHA-256 over a *tagged* pre-image (tag = version + domain
separator, so one identifier kind can never collide with another's input). The structural `GenesisId`
(a hash of a `Genesis` struct's canonical bytes, used internally by governance) is **not** a wire
identity and is intentionally untagged — it is computed only over already-structured bytes.

| identifier | pre-image | source (canonical) |
|---|---|---|
| lineage genesis | `sha256("croft-lineage-genesis:" ‖ lineage_id)` | `lineage-core::ids::lineage_genesis` |
| group genesis | `sha256("croft-group-genesis:" ‖ group_id)` | `lineage-core::ids::group_genesis` |
| group gossip topic | `TopicId = sha256("croft-group-topic:" ‖ group_id)` | `lineage-core::ids::group_topic` |
| `GenesisId` (structural, untagged) | `sha256(canonical_bytes)` | `lineage-core::ids::GenesisId::from_bytes` |
| content id | `sha256(json{groupId, regime, authorId, content, timestamp})` | `lineage-group-model` |

*(2026-06-17: the three tagged wire derivations were promoted from the `altdrive-spike-lineage-sync`
spike into `lineage-core::ids` as canonical, conformance-tested functions — byte-identical to the spike.
`lineage-iroh::GroupTopic::from_group_id` now uses the §2 `"croft-group-topic:"` form; its old
`from_seed(u64)` is a test stand-in only, not a wire derivation.)*

- An implementation **MUST** derive these identically; they are the interop anchor. The topic seed
  **MUST** be high-entropy / salted, not a guessable human handle (else an adversary computes the
  topic and joins/observes — §10).

  Proof: derivations transcribed from code; topic-guessability bound characterized in **AR-4**.
  *green-real (derivations) / characterized (leak).*

---

## 3. The signed message (the unit of history)

The real history unit (`lineage_history::Message`):

```
Message { author: Did, seq: u64, branch: GenesisId, payload: bytes, sig: Ed25519 }
```

The **signed pre-image** (`Message::signing_bytes`) binds author, position, branch, and payload so a
message cannot be replayed onto another branch or position:

```
signing_bytes = "msg-v1" ‖ branch(32) ‖ seq(LE u64) ‖ author_did_bytes ‖ 0x00 ‖ payload
```

- A receiver **MUST** recompute `signing_bytes` and verify `sig` against the author's key. The `sig`
  is carried as 64 raw bytes (hex in the JSON profile).

  Proof: **faithful path** — the real `Message` traveled live iroh-gossip and was verified by the
  real `backfill_import`; HONEST member message → ACCEPT. *green-real.*

---

## 4. Integrity & ordering vs authorship — two distinct guarantees

- **Integrity + ordering (structural):** a branch is a sequence chained by
  `hash = sha256(prev ‖ seq(LE) ‖ payload)`; receivers **MUST** reject a branch with a broken chain
  or non-contiguous seqs. This proves in-transit integrity and contiguous ordering — **not** who
  wrote it.
- **Authorship + standing (authority):** the Ed25519 signature (§3) + standing (§6). A receiver
  **MUST** apply both; integrity alone **MUST NOT** be treated as authorization.

  Proof: structural half — **MD-G2 / T11** (hash-chained branch carried, tampered rejected) over live
  iroh. Authority half — **faithful path** (a valid-chain branch from a non-member is accepted by the
  hash-chain spike but **rejected** by the real check as `UnauthorizedAuthor`). *green-real.* This
  separation is the protocol's central honesty boundary: §3+§6 are what make a branch trustworthy,
  not §4's chain.

---

## 5. Multi-device fold (device-count ≠ actor-count)

- Devices of one actor share a `lineage_id`; each device carries a distinct `device_did`. Receivers
  **MUST** fold absorbed branches by `lineage_id` into one actor (`fold_by_lineage`). A group topic
  carries multiple lineages; the fold is what every peer computes identically to agree on the member
  list and on **lineage-counted thresholds** (§6).

  Proof: **E2.9/C4** (`fold_by_lineage`, green-real model) + **MD-G4** over live iroh — alice's two
  devices folded to one actor, bob a second; all three nodes agreed `folded_actors=2`. *green-real.*

---

## 6. Membership, standing, and revocation authority

- **Standing** is decided from recorded, signed data — never the actor's own assertion. A message is
  authorized iff its author held standing on a branch sharing the relevant lineage root.

  Proof: **I3 / E2.7**; over the wire in the **faithful path** (`standing` check rejects the
  non-member). *green-real.*

- **Revocation** removes a device/actor from the accepted set going forward. Survivors **MUST** reject
  the revoked party's *subsequent* branches and **MUST NOT** claw back history contributed before
  removal (standing ≠ membership; history is not erased).

  Proof: **MD-G5** over live iroh (witness retains pre-revoke branch + marks revoked; revoker rejects
  post-revoke as `(revoked)`) + **E2.11**. *green-real (mechanics); revoke-authority is structural
  stand-in over the wire — see below.*

- **Revocation/add authority is a threshold dial** (k-of-n): default 1-of-any, up to k-of-any or
  role-restricted admins. A membership op is authorized iff it carries signatures meeting the group's
  **current, replicated** policy. Policy lives in versioned group state (the admin chain); changes are
  themselves governance ops under the current policy. Threshold gathering is either a co-signed op
  (self-certifying, one broadcast) or proposal+votes (eventual, auditable). A membership change
  authorized against a **stale** group view **MUST NOT** be acted on (freshness gates authority, §9).

  Proof: threshold-signed governance op shape — **T3 / F2** (green-real). The over-the-wire authority
  signature is now **carried as a real k-of-n Ed25519 bundle** and verified on receipt with
  `gov::meets_threshold_by_lineage` over live iroh-gossip — **C-faithful-revoke (2026-06-17, green-real):
  REVOKE-AUTHORIZED (2 admin lineages ≥ 2) accepted, REVOKE-UNDERTHRESHOLD (1 lineage) rejected.** The
  MD-G5 transport MAC is retired. Still open: the co-sign-vs-vote accumulation **ordering** decision
  (`revocation-authority.md`) — orthogonal to the now-real signature mechanism.

- **Roles are revocable delegations, never impositions.** A group **MAY** grant a role (admin,
  moderator, a content-gating `geer` §6.1, an always-on `meer` §8) that carries enumerated rights, for
  ease and consistency. Every such role **MUST** be a **revocable delegation** from the group's
  members (granted and withdrawn by the same threshold authority), **MUST** carry only **scoped,
  enumerated, non-creeping rights**, and **MUST NOT** be immutable, forced, or held by structural
  right. A role's capability **MUST** be downstream of the grant and revoked with it — no peer holds a
  right because it merely *can* (capability), only because the group *granted* it (election).

  Proof: revocation mechanics **MD-G5 / E2.11** (green-real); threshold grant shape **T3 / F2**. *design
  — see `principles.md` "delegated authority, never imposed".*

- **Delegation MUST be *materially* reversible, not just formally.** Because a resourced, always-on,
  state-holding peer can entrench by circumstance, an implementation **MUST** make replacement real:
  (a) a meer/geer holds only **encrypted** state and the group holds the keys, so the group **MUST** be
  able to **re-host on, or migrate to, a different holder** (no data hostage); (b) a group **MUST** be
  able to **stand up a different meer/geer (different host/party) and elect it in place of the
  incumbent** — the role is a re-issuable grant, not bound to a box; (c) the **re-formation fork** (§7)
  remains the adversarial backstop when an election is captured. Routine replacement is the normal
  check; the fork is the backstop.

  Proof: re-formation backstop **C3 / D-series** (green-real + green-model); state-portability +
  stand-up-and-elect is *design*. See `meer-superpeer-design.md` (anti-entrenchment).

### 6.1 The geer — opt-in content-visible moderation role

- The default group is **blind** (no peer reads content). A group **MAY** consensually elect a
  **`geer`**: a disclosed, scoped, revocable role that **MAY decrypt content for moderation only**.
  A geer **MUST** be (1) **opt-in by the group's threshold authority** — never imposed or default;
  (2) **disclosed** to all members as a named role (informed consent — not a covert capability);
  (3) **scoped** to the least-invasive rung that serves the need — **report-gated** (no key; sees only
  member-disclosed items) **SHOULD** be preferred over **classifier-gated** over **full-key (Tier 2)**;
  (4) **accountable and revocable** (replaceable per the materially-reversible rule above).
- A geer **SHOULD** emit **labels** (advisory metadata, the atproto/Ozone model), **not** unilateral
  enforcement: the geer labels, and **group governance or each member's client decides the action**
  (hide/warn/remove/ban). Content-level labels **MUST** stay in-group (exporting them leaks private
  content — §10/S2); account-level labels **MAY** be portable under the shared DID.
- A geer is **never offered for the most-private lane** (Lane 1 intimate groups stay blind/self-
  moderated). Ban/block **MUST NOT** require a geer — it is available blind via reports + governance
  (§11); a geer is justified only for *proactive content* moderation.

  Proof: *design exploration — see `geer-gating-peer.md`.* **Honesty boundary (normative to disclose):**
  any content-visible role weakens the "cannot comply" property (compellability) — so the system
  default **MUST** remain blind and the geer **MUST** remain strictly per-group opt-in; a rung-2/3 geer
  that has seen content cannot un-see it on revocation. Legal review required before shipping.

---

## 7. Reconcile and the contradiction hard-stop

- When two histories are merged, an implementation **MUST** detect membership contradictions
  (e.g. removed-then-included) and **hard-stop** — it **MUST NOT** silently auto-resolve (LWW or
  otherwise). Resolution is a social/governance input, not an automatic merge. A clean, attributable
  **re-formation fork** is the sanctioned exit for a minority.

  Proof: reconcile corpus **C1–C10**, **I6**, **AR-2** (green-real + green-model); re-formation
  backstop (identical reformed genesis across 3 boxes). *green-real.*

---

## 8. Transport — gossip topics, relay placement, interaction tiers

- Membership of a group maps to a gossip **topic** (§2). The transport is iroh: encrypted QUIC,
  relay-fallback for NAT'd peers. A relay forwards opaque frames and **MUST NOT** be required to read
  content; it routes by `EndpointId`, not by topic.

  Proof: **MD-G1** (NAT path via relay), **E3.4** (blind broker sees only ciphertext+routing). *green-real.*

- **Co-location is mandatory:** two peers reach each other over relay-fallback only if they share a
  home relay (no relay-to-relay mesh). Placement (which relay a peer homes on) is **server-published
  and authoritative**, keyed on the rendezvous/namespace, not on identity.

  Proof: **E2** (assigned relay authoritative; wrong assignment → no connection), **E3** (namespace
  shard converges, `dropped`=0), **E7** (re-home converges; stale assignment fails = the partition
  window). *green-real (measured).*

- **A relay process meters and isolates per-tenant via cgroups** (accounting + isolation), and
  **degrades visibly under network stress** (never silently).

  Proof: **E5** (per-slice ~249:1 CPU billing; CPUQuota cap isolates without stranding), **E6** (under
  +100ms / 10–30% loss all conns still established, RTT/establish rise visibly). *green-real (measured).*

- **Interaction tiers** are chosen at creation, not toggled: **interactive** (strong-ish: prompt
  delivery + real failure signal), **quiet-large** (eventual: "it'll arrive or you'll be told it
  didn't"), **broadcast** (best-effort rolling log). The broadcast tier **MUST** disable the embedded
  MLS ratchet-tree (O(N) commits) and ship the tree out-of-band.

  Proof: tiers — `interaction-tiers.md` (design); ratchet-tree O(N) — **AR-5** (measured). *design + green-real.*

- **Real-time media** (voice/video/stage) rides the **same iroh transport** as messaging, but over
  **QUIC datagrams** (unreliable, no retransmit) rather than reliable streams — carried as **RTP-over-
  QUIC (RoQ)**. Media frames **MUST** use the datagram flow (latency over reliability); media is
  E2EE end-to-end via **SFrame keyed off the MLS epoch** (so a forwarding meer stays blind), and a
  group-scale call **SHOULD** use a **blind SFU-meer** (forwards opaque frames, header-only routing —
  the E0–E7 shape) rather than full mesh past a handful of peers. MCU-style server mixing is
  **forbidden** (requires plaintext). Media keying rekeys on membership change exactly as messages do
  (MD-G5).

  Proof: transport primitive — n0's **`iroh-roq`** sends RTP over `conn.send_datagram(...)` and the
  **callme** app ships P2P Opus audio over iroh with no WebRTC (verified 2026-06-16). Full design,
  topologies, the str0m/RoQ engine lines, and the SFrame-over-MLS keying are in
  `thinking/realtime-media-over-iroh.md`. *transport primitive green-real (external); the Croft media
  stack is design.*

---

## 9. Freshness signal — no-false-current

- A peer/broker **SHOULD** periodically emit a signed, **content-free** tip beacon
  `{group_id, epoch, head, seq_high, sig}` (head/epoch/routing only — blind-broker-safe).
- A peer **MUST** track time-since-last-heard locally (liveness is a local measurement, never trust
  peers' wall-clocks) and **MUST NOT** display a view as "current" unless it is both caught-up to the
  best-seen tip **and** heard a beacon within the tier's freshness horizon. Otherwise the view **MUST**
  surface as "behind" or "unverified" — silence **MUST NOT** be rendered as currency.

  Proof: **E2.16a/b/c** (green-model): availability without a superpeer; no-false-current (silence →
  unverified, even when the peer advanced its own head); tiers degrade visibly. Design:
  `freshness-signal.md`. *green-model.*

---

## 10. Visibility and the social layer

- A group's **regime** (`intimate`/`public`), `openness_class`, `outward_propagation_depth`, and
  `inward_visibility` are **born in at genesis and immutable** (part of the signed genesis). Content
  carries its origin regime in its signed hash. There is **no silent regime crossing**; a republish is
  a distinct authored act carrying a reference + author-chosen public content, never the intimate
  original. Outward propagation depth is enforced **by every verifier**, capped by openness class
  (`fully_open` ⇒ depth 0).

  Proof: **V1–V9** (green-model, `lineage-group-model`). *green-model.*

- **Scoped visibility, not opaque structure (S2):** an implementation **MUST NOT** offer a
  structure-only share (topology revealed, identities withheld) — graph topology is re-identifying and
  such a share is unrepresentable. The only safe share is **consented-distance/resolution-scoped**: a
  viewer at distance *d* sees exactly what was consented for *d*, never topology.

  Proof: **S2a/S2b** (green-model) — modelled town of 4,000, target's connection shape has anonymity
  set 1; `attemptStructureOnlyShare` throws. *green-model.*

---

## 11. Failed-operation response (design)

- Detection of an invalid op is deterministic (§3–§7: identical inputs → identical verdict). The
  **response** is a governance dial: **loud** (signed, corroborated rejection-event → group immune
  memory), **silent** (reject, no signal), or **blackhole** (tarpit). An implementation **MAY** offer
  any point on this dial per group; a serious auto-response (rate-limit, removal-vote input) **SHOULD**
  require k-observer corroboration. Note: "silent" is an application-layer property — the relay still
  observes the connection attempt (AR-4).

  Proof: design — `failed-op-response.md`; determinism prerequisite satisfied by the faithful path. The
  residual-leak quantification is an open spike (`open-edges.md`). *design.*

---

## 12. Conformance and honesty boundaries

- A conformant implementation **MUST** pass the vectors and must-reject cases in `conformance-suite.md`
  (derivations, signed pre-images, fold/revocation invariants, the C1–C10 reconcile corpus, the
  AR-1…AR-6 adversarial set, the V/S visibility invariants, the freshness no-false-current rule).
- **Honesty boundaries this spec still carries** (tracked in `open-edges.md`): (1) MLS key-distribution
  is not yet exercised over the wire (the faithful path models the key registry as agreed state);
  (2) threshold revoke-**authority** is not yet a real signature over the wire (§6); (3) freshness is
  proven in the model, not over live transport; (4) the failed-op leak/immune dial is design-only;
  (5) the **geer** (§6.1) and the role/governance guards (§6) are design — and the geer's
  **compellability** tradeoff (a content-visible role weakens the system's "cannot comply" property)
  is an unresolved policy/legal question, not an engineering one. The default-blind posture is the
  mitigation; legal review gates any geer implementation.

---

*Provenance: `experiments/iroh/TEST-LOG.md`, `experiments/iroh/relay-lab-runs/`,
`Proofs/lineage-groups/`, `Proofs/lineage-group-model/`, and the ledger + narrative in
`discovery/crystallized/`. Status legend per §0.*
