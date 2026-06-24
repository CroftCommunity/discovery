# Multi-Device via Lineage: Spec and Phase 2.5 Experiments

author: ISaT / Product Security

date: 2026-06-13

status: draft spec, sequenced behind the Phase 1/2 experiments in THESIS.md

depends on: experiments/lineage-groups/THESIS.md (two-tree model, invariants I1-I10, backfill path)

---

## 1. Core idea

A user participates from multiple devices (Chase desktop, Chase phone, Chase laptop, Chase browser). Instead of sharing one key across devices (the SSB/fusion-identity trap) or running a server as source of truth (the Signal/Matrix centralization), **each device is a distinct MLS member with its own key, and the "same person" fact lives one layer up, in the DID lineage.**

Keys are not identity. Identity is the provable lineage. Keys are per-device actors stamped out under it.

Consequences, all of which reuse machinery already built for the group-fork problem rather than adding new subsystems:

- **Cohesiveness is a presentation concern.** Multiple leaves sharing a lineage collapse to one displayed actor in the UI. Under the covers they are genuinely distinct members, and that is fine.

- **Self-sync is backfill.** Reconciling your own devices is the same operation as catching up a forked branch: verify messages chain to a shared genesis. Your own devices trivially share genesis, so no special-case code is needed beyond the existing provenance-verifying backfill.

- **Device revocation is a normal governance op.** Lose a device, issue a remove against that device's key on the forward-only admin chain, MLS epoch rotates, the device can derive no new secrets.

- **Drift is honest.** Your devices' histories can diverge no more and no less than any two peers, and the model already treats peer drift as normal and reconcilable.

## 2. The two non-negotiables (and what they license)

These set what is allowed to be imperfect.

**Will not compromise:**

- **Availability.** From any authorized device you can always send forward. A device that is behind on history must still be able to participate going forward.

- **Administrative clarity.** Device management (which of my devices are in, adding one, removing one) is always legible and truthful. A behind device looks behind; it never pretends to be current.

**Will compromise (deliberately):**

- **History completeness across devices.** Scrolling your desktop's full history from your phone is a secondary use case. It syncs out-of-band, eventually. The primary use case is "hop in from this device and talk forward."

- **Real-time sync perfection.** No guarantee of identical scroll state across devices at any instant. Eventual consistency, made usually-fast by the superpeer when present.

This is fail-early/fail-clearly applied to multi-device: stale is allowed and must be visible; unavailable and murky are not allowed.

## 3. Identity and key model

- A **lineage** is a DID with a genesis anchor. It is the unit of "person."

- A **device leaf** is an MLS member: its own signing key + leaf node, carrying a verifiable credential that proves membership in its lineage.

- The lineage credential **must travel with the leaf** so any other group member can verify "this leaf belongs to that lineage" from signed data alone, without trusting an assertion. (See open dependency in 8.1 — this is the one protocol-level hook this design adds over the group-fork work.)

- The mapping leaf -> lineage is therefore computable by every client, which is what makes both the member-list fold (4) and lineage-counted thresholds (5) possible.

## 4. Presentation fold

- Every client folds leaves sharing a lineage into one displayed actor. The member list shows "Chase," not "Chase (laptop), Chase (phone), Chase (browser)."

- The fold is local presentation, but it is computed from the protocol-visible lineage credential, so all clients fold consistently.

- A management surface (per-user, on the lineage owner's own devices) can unfold to show the individual devices, for the owner to manage them. This panel is itself eventually-consistent; no perfection claimed.

## 5. Thresholds count lineages, not leaves

The genesis threshold rules (immutable, per I1) are evaluated over **lineages, not device leaves.** Two signatures from leaves of the same lineage count as one toward any social threshold. This prevents an actor from manufacturing a quorum with multiple of their own devices.

Then a deliberate asymmetry for operations targeting a device under a lineage:

- **Same-lineage device op (one signature).** Adding or removing a device leaf, when authored by another device leaf under the *same* lineage, requires one sign-off. The shared lineage *is* the authorization. This lets your devices self-organize: your laptop authorizes your new phone.

- **Cross-lineage device op (full threshold).** Any operation on a device leaf authored from outside that leaf's lineage pays the group's normal social threshold for that op type. This is how a *lost* device gets cleaned up when its own lineage cannot act, and it is the same as removing any member.

So removal of your own lost phone has two valid paths: another of your devices removes it (one sig, same-lineage), or the group removes it under the normal boot threshold (cross-lineage). Both must be expressible.

## 6. Self-removal ordering

"The moment you remove your laptop, you have the rights to modify the group; it just stops being a participant."

- A device authors the governance op that removes itself **while it still has standing.** The authorization precedes the removal taking effect.

- The last valid act of a leaf can be to drop its own standing. After the op is enacted (MLS epoch rotates), the leaf is out and derives no new secrets.

- **History already on the removed device is left behind, not clawed back.** Revoking participation is not wiping local state — no protocol can retract what was already decrypted. The truthful statement is "this device cannot participate going forward," never "this device forgot."

- Two distinct ops must exist:

  - **leave-this-leaf:** drop one device from the group.

  - **leave-all-under-lineage:** the whole person exits the group (drop every leaf sharing the lineage).

## 7. Two experience tiers

- **With superpeer:** the always-on blind broker queues and serves snapshots, so self-sync and forward availability feel prompt. This is the palatable common case.

- **Without superpeer:** pure co-present P2P. You accept stale history and slower sync. Availability forward and administrative clarity still hold; only history completeness and immediacy degrade. The tier difference must be visible to the user, not silent.

## 8. Phase 2.5 experiments

Sequenced behind Phase 1 (which already supplies add-a-member and remove-a-member-with-rekey) and Phase 2 (governance log, lineage DAG, backfill). Mostly data-model and threshold logic; no new crypto beyond Phase 1's external-commit primitive.

- **E2.9 - lineage fold for member lists.** Multiple leaves sharing a lineage render as one actor; assert the fold is computable from signed data every client holds. Guards the "six tapes" mistake at the member-list level.

- **E2.10 - thresholds count lineages not leaves (adversarial).** Two signatures from one lineage count once; assert an actor cannot manufacture a social quorum using multiple own-devices. This is the one that bites silently if wrong.

- **E2.11 - device revocation.** Remove one leaf under a lineage via the admin chain; assert epoch rotates, that device is dark for new secrets, the lineage's other devices are unaffected, and local history on the removed device is untouched (not wiped).

- **E2.12 - self-sync as backfill.** Two devices under one lineage with divergent histories reconcile via the existing backfill path; assert convergence with no server and no special-case code beyond lineage-shared-genesis.

- **E2.13 - leave-one vs leave-all.** Both governance ops expressible and distinct; assert leave-this-leaf drops one device while others persist, and leave-all-under-lineage drops the whole person.

- **E2.14 - same-lineage one-sig vs cross-lineage full-threshold.** Assert a same-lineage device add/remove needs one signature, a cross-lineage op on that leaf needs the full social threshold, and the two paths cannot be confused.

- **E2.15 - self-removal ordering.** Assert a leaf can author its own removal while it has standing, the op enacts, and a leaf cannot author an op after its standing is dropped.

- **E2.16 - tier degradation visibility.** Without-superpeer run: assert forward send still works and device management stays truthful while history is stale, and that the stale/behind state is surfaced rather than hidden.

### 8.1 Open dependency to verify first

The lineage credential riding on the MLS leaf (section 3) is the one protocol-level hook this design adds beyond the group-fork work. Before treating E2.9 and E2.10 as solved, confirm openmls lets you attach a lineage-proving credential or leaf-node extension that other members can verify. If the lineage link cannot ride on the leaf, the member-list fold and lineage-counted thresholds must source the mapping elsewhere, which is a design change. Check this against the real library at the start of Phase 2.5.

## 9. Gate

Phase 2.5 passes if E2.10 (no quorum manufacture from own devices) and E2.12 (serverless self-sync via existing backfill) hold, with E2.11 and E2.15 confirming clean per-device revocation and ordered self-removal. That set validates the core claim: per-device keys plus lineage-folding deliver individual device management and serverless self-sync without a new subsystem, while preserving availability and administrative clarity.

## 10. Transport realization: the per-lineage gossip group (2026-06-16)

The E2.12 "self-sync as backfill" claim is a *data-model* claim. This section names its
**transport** realization and the hypothesis behind it: **a user's own endpoints form an iroh
gossip group scoped to their lineage, epidemic-broadcasting their signed history branches to
keep in sync, while each device stays local-first.**

Why this is the natural shape, not a new subsystem:

- **It is the group mechanism turned inward.** A user's devices already share a genesis
  trivially (same lineage), so `shares_lineage` is always true among them and the backfill
  privacy boundary is satisfied by construction. Self-sync is the group-reconcile path with the
  membership set narrowed to one person's leaves.

- **The pieces are already green-real.** `B-gossip` proved an iroh-gossip mesh: a mesh formed
  from one bootstrap node, transitive delivery between peers that never exchanged addresses, and
  survival of killing the relaying node mid-run. `I7/I8/I9` proved per-device signed branches,
  consensual backfill, and lossless fold across 3 machines — explicitly "the same mechanism for
  multi-device and group." `B3` proved pairing bootstrap via `NodeAddr` + `TopicId` with no
  direct IP in the invite. What is *not* yet proven is these composed as the self-sync path
  (that is the new work — see cases below).

- **Local-first is preserved.** Each device holds its own Automerge history and can send forward
  from its own state at any time (the §2 availability non-negotiable). Gossip is how branches
  *find* each other; backfill is how they are *verified and imported*. A device that is behind
  looks behind (administrative clarity), never pretends to be current.

```
        lineage L (the "person")
   TopicId = derive(genesis_of(L))      ← the gossip topic is the lineage root
        ┌──────────┬──────────┬──────────┐
     phone      laptop     browser     (each: own key, own MLS leaf, own Automerge doc)
        └────── iroh gossip mesh ───────┘   epidemic broadcast of signed branches
                     │
            consensual backfill: verify chains-to-shared-genesis → import as navigable branch
            (never interleaved into one transcript — anti-"six tapes", E2.8)
```

### 10.1 The key-hierarchy fork — DECIDED 2026-06-16: (a) live + (b) backup-only

**Decision:** logical binding (a) for the live path, with an optional HD seed (b) reserved
*purely* as a lose-all-devices backup. The live path never uses derivation; the seed is not in
the normal-operation threat surface. This keeps "keys are not identity" for everything that
runs day-to-day, and gives the recovery gate (G1) a concrete anchor without making a single
seed the crown jewel of live operation. It mirrors the COHESION #12 framing ("delegation
primary + optional trust-minimized backup").

Concretely:

- **Live path (a).** Parent = lineage DID/genesis (identity). Children = *independent* device
  signing keys. Each child carries a lineage-proving credential **signed by the lineage root
  key** (`lineage_sig`); that signature is what makes the claim unforgeable (T1). Compromise of
  one device compromises only that device. No key is derived from another in normal operation.
- **Recovery backup (b), optional.** An HD seed (mnemonic) can re-derive a fresh *authorized*
  device when every device is lost. The seed is backup-only — held offline, never used in live
  ops — so it is not a live single-point-of-compromise. Proving this mechanism is T12; this
  decision sets its anchor (device-delegation primary + optional seed backup), closing the
  open part of gate G1 for the multi-device-lineage case.

This is the decision; the two readings it chose between were:

- **(a) Logical binding (current model).** The "parent" is the lineage DID/genesis (identity);
  "children" are *independent* per-device signing keys, each carrying a lineage-proving
  credential the parent root key signed (the T1 leaf-credential work). No key is derived from
  another. Compromise of one device compromises only that device. Total-device-loss = no key
  survives → recovery is an *external* anchor decision (G1). This is "keys are not identity."

- **(b) Cryptographic derivation.** Children keys are HD-derived from a parent seed. The parent
  seed *is* a recovery anchor (re-derive children from a mnemonic), which directly addresses G1 —
  but it reintroduces a single high-value secret whose compromise threatens every child, and it
  softens "keys are not identity" toward "the parent seed is identity." It is not the SSB
  shared-key trap (children still hold distinct keys), but the seed is the new crown jewel.

The chosen synthesis is exactly (a) for the running model with an *optional* (b)-style seed as
the lose-all-devices backup — recorded above. T1's credential design follows (a); T12 proves the
optional (b) backup.

### 10.2 New proof cases (self-sync over real gossip)

These extend E2.12 from data-model to real transport, and connect to the merge/split corpus
(`merge-split-corpus.md` S3/M6/C4):

- **MD-G1 — per-lineage gossip topic.** A `TopicId` derived from the lineage genesis admits
  only that user's device leaves; assert a device joins via `NodeAddr + TopicId` (no direct IP)
  and the topic is not guessable from public data alone.
- **MD-G2 — branch broadcast + backfill import over gossip.** Two offline-diverged devices
  reconcile by gossiping signed branches and importing via the existing backfill path; assert
  convergence with no server and no special-case code beyond shared-genesis (the live-transport
  form of E2.12, and the T11 follow-on for the single-user case).
- **MD-G3 — drop-a-device resilience.** Kill one device mid-sync; assert the remaining devices
  still converge (the B-gossip drop-a-node result, applied to the self-sync topic).
- **MD-G4 — add-vs-add fold (corpus C4).** The same person comes online from two devices that
  both authored while partitioned; assert they fold to one actor and do not double-count toward
  any lineage-counted threshold (E2.10 interaction — the unmodeled gap C4).
- **MD-G5 — revoked device cannot rejoin the topic.** After device revocation (E2.11), the
  revoked leaf cannot re-enter the gossip topic or derive new secrets; assert the epoch rotation
  and the topic-membership check agree.
