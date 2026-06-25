# 05 — Identity you carry across platforms

date: 2026-06-24

status: synthesis (spine-complete at the design level; one decision and one artifact remain the user's).

verification: this theme is largely design-synthesis; external quotes are few. The precise atproto facts
carry their verification and cite FACTCHECK SoT — not re-verified here.

---

## Theme narrative (overview)

Croft's identity model rests on a single inversion: a key is not who you are, it is an actor stamped out
under a who. The "who" is a **DID lineage** with a genesis anchor; each device (phone, laptop, browser) is
a genuinely distinct MLS member carrying its own signing key plus a credential proving membership in that
lineage. This dissolves two failure modes the field keeps repeating — sharing one key across devices (the
SSB / fused-identity trap) and running a server as the source of truth (the Signal/Matrix centralization).
Cohesiveness ("show me as one person") becomes a *presentation fold* computed from protocol-visible data,
not a shared secret.

Because identity lives one layer up from keys, machinery already built for the group-fork problem (`04`) is
reused wholesale: self-sync across your own devices is just backfill; device revocation is a normal
governance op; drift between your devices is honest and reconcilable like any peer drift. The single
load-bearing protocol invariant is that **thresholds count lineages, not leaves** — N of your own devices
count once toward any social quorum, so an actor cannot manufacture consensus from their own hardware.

The same DID that roots private MLS membership is also the person's *public* identity across networks — and
here the structural wall appears. Each network (atproto, ActivityPub, Hive) is a closed cryptographic root
of trust that will not delegate authority to an outside key. So a cross-platform *authority* key cannot
exist; the only real linkage is out-of-band, mutually-anchored or root-signed provenance **attestation** —
a hub-and-spoke shape with a `did:webvh` root that makes verifiable *claims* about per-network spokes, each
keeping its own native key. The root's value is evidentiary, not operational: a correlation anchor, not a
controller.

Lineage here is **attestation, not derivation**: the root *signs statements about* independent child keys,
it does not derive them. The linkage field is `alsoKnownAs`, and its weakness is the feature — bidirectional
presence is the mechanical validation; absence means *unverified*, not *false*, so nothing written today
becomes a false claim later. A cheap forward hedge rides alongside: `did:plc` and `did:webvh` are nearly the
same data structure, so keeping the `did:webvh` SCID as the anchor costs nothing today and slots cleanly
into a dual-resolvability convergence if it ever lands.

What remains genuinely open is recovery when **every** device is lost. The live model never uses derivation,
so when no key survives, recovery is an external-anchor decision — and that anchor decision, plus the
per-platform trust-model document the design keeps asking for, are the surfaced open items. This is the top
unresolved design decision across the whole corpus.

## Charter — what this theme covers

**In scope.**

- "Keys ≠ identity; person = DID lineage; device = distinct MLS member under one lineage."
- The presentation fold, self-sync-as-backfill, device revocation, self-removal ordering, the two
  experience tiers — *as they bear on identity continuity*.
- The DID-method choice for the MLS root (`did:plc` vs `did:web` vs `did:webvh`) and the validating PLC
  read-replica.
- Cross-platform provenance: hub-and-spoke `alsoKnownAs` attestation, the equivalence ladder, the
  `did:plc↔did:webvh` convergence hedge.

**Out of scope (and where it lives).**

- The "evidentiary, not operational" rights-floor framing as an *epistemic principle* → `01` (cross-ref,
  don't re-derive).
- The `threshold-counts-lineages` invariant's *proof* and MLS epoch mechanics → `04` (stated here as a
  load-bearing fact).
- The verified atproto/iroh substrate facts → `03` / the FACTCHECK SoT (cite, don't re-verify).
- The recovery-anchor *decision* as a cooperative/decision matter → `07` (surfaced here, the user's call).

**Boundary calls.**

- The recovery-anchor decision (trust-minimized backup vs device delegation) is **surfaced here, decided by
  the user.** The decided *shape* (delegation-primary + optional offline seed backup) is recorded; the open
  fork for *total-device-loss* remains the user's call and overlaps `07`. The promotion of "evidentiary, not
  operational" into `crystallized/principles.md` is settled-as-conclusion but is the user's call to insert
  (cross-ref `01`). The per-platform trust-model doc is *named* here as the highest-leverage next artifact,
  not authored inside this theme.

## 1. Keys ≠ identity; identity is the provable lineage

> "Keys are not identity. Identity is the provable lineage. Keys are per-device actors stamped out under
> it."

> "each device is a distinct MLS member with its own key, and the 'same person' fact lives one layer up, in
> the DID lineage."

*Verification:* internal design statements, in-source. *Grounds:* a lineage is a DID with a genesis anchor —
the unit of "person" — which rejects both the shared-key (SSB) and server-of-truth (Signal/Matrix) models.

## 2. Device = distinct MLS member; thresholds count lineages, not leaves

Each device is its own MLS leaf with its own signing key, carrying a lineage-proving credential that rides
on the leaf so any member verifies "this leaf belongs to that lineage" from signed data alone. Cohesiveness
is a *presentation fold* (the member list shows "Chase," not three devices).

> "The genesis threshold rules (immutable, per I1) are evaluated over **lineages, not device leaves.** Two
> signatures from leaves of the same lineage count as one toward any social threshold."

*Verification:* internal design statement; the invariant (INV-LINEAGE-NOT-LEAF) is **green-real** in `04`.
*Grounds:* an actor cannot manufacture a quorum from their own hardware. A deliberate asymmetry follows:
same-lineage device ops need one signature (your laptop authorizes your phone); cross-lineage ops on a leaf
pay the full social threshold (how a lost device is cleaned up):

> "The moment you remove your laptop, you have the rights to modify the group; it just stops being a
> participant."

*Cross-ref `04`* for the invariant's proof and the one library dependency this design adds — that openmls can
carry the lineage-proving credential on the leaf (§8.1; logic CLOSED, library dependency tracked).

## 3. The DID-method choice for the MLS root

An MLS root needs, in priority order: durable resolvability, recoverability from key compromise, host
portability, self-authentication. The scorecard outcome:

- **`did:webvh` on a domain you operate** = best fit (the SCID + verifiable history restore the recovery /
  portability / self-auth that plain `did:web` lacks) — *if and when* atproto native support is confirmed
  (`[UNVERIFIED]`).
- **`did:plc` + an independent high-priority backup rotation key held off-PDS** = the choice for maximum
  network interop today; the archive covers reads, but writes do not survive a full PLC disappearance
  (frozen-and-unrotatable — a genuine failure mode for a long-lived root).
- **Plain `did:web`** = rejected for an MLS root unless domain-loss-equals-unrecoverable is explicitly
  accepted.

The choice is effectively **permanent per identity** — changing a member's DID forces re-establishing their
credential across every MLS group. *Buried conclusion harvested:* the method choice *resizes* the PLC
archive — if high-value identities are `did:web(vh)`, the PLC archiver stops being existential and is only
for resolving outside-set `did:plc` identities.

## 4. The validating PLC read-replica (store evidence, not answers)

> "a cache stores answers; a replica stores and re-validates the evidence. We want the replica."

*Verification:* internal design statement. *Grounds:* store the signed *op-log*, not resolved documents;
order by chain (`prev_cid`), never by timestamp (PLC concurrency means ops are first-seen out of order);
insert idempotently on `cid`. Validation = signature against the rotation key as-of-parent-state + hash
chain + the fork/recovery rule. This is what catches a primary that deletes an op to roll back a DID — the
replica retains both branches, so rollback is *detectable*, not silent. It is also the verifier that walks
the Bluesky half of any provenance graph (§5).

## 5. Cross-platform = attestation, not derivation; no cross-network authority key

> "So a cross-platform *authority* key cannot exist. There's no key any two of these networks both accept
> as the thing that controls the account; the wall is structural, not tooling-immaturity."

> "out-of-band, mutually-anchored or root-signed provenance attestation is the only real cross-platform
> linkage mechanism, precisely because every cryptographic-identity arena insists on being its own
> authority. The root is a correlation and provenance anchor, not a controller — its value is evidentiary,
> not operational."

*Verification:* in-source dialogue formulations. *Grounds:* the only real linkage is a hub-and-spoke with a
`did:webvh` root (offline update key) as a correlation anchor; spokes (`did:plc` Bluesky / AP HTTP-Sig actor
/ Hive on-chain keys) each keep their own native authority. Lineage is the root *signing statements about*
independent child keys, not derivation:

> "The provenance is the signed log entry, not a derivation path."

The linkage field is `alsoKnownAs`, and its weakness is the feature — bidirectional presence is the
mechanical validation; absence means *unverified*, not *false*. The three ranked goals resolve cleanly:
provable common ownership = **fully achievable**; a recovery anchor = **partial, network-dependent** (real
on Bluesky via a root-controlled rotation key, none native on AP, attestation-only on Hive); one key to
operate everything = **drop it.** ("Evidentiary not operational" flagged for `crystallized/principles.md`,
cross-ref `01`.)

## 6. The `did:webvh ↔ did:plc` convergence — a cheap, non-foreclosing hedge

`did:plc` is structurally "almost already" a `did:webvh`: both are a self-certifying ID (hash of genesis) +
an append-only signed log + resolve-by-replay; the only differences are *where the log lives* and *whether
the resolver checks the genesis hash*. atproto discussion #2705 sketched dual-resolvability (a PDS hosting
the full oplog → each `did:plc` becomes a valid `did:webvh`). Design rule: treat the Bluesky `did:plc` as a
spoke, keep the `did:webvh` SCID as the root anchor even though nothing reads it today — hedge-positioning,
not a roadmap bet.

## 7. Total-device-loss recovery — the open fork (the headline problem)

The live path never uses derivation (compromise of one device compromises only that device). When *every*
device is lost, no key survives → recovery is an external-anchor decision. The fork: **trust-minimized key
backup** (escrow operator-can't-read / threshold-shared) vs **device delegation** (an existing device
authorizes a new one; needs a device present).

**Decided shape (2026-06-16):** delegation-primary for the live path + an *optional* offline HD seed
reserved purely as a lose-all-devices backup — mirroring the corpus-wide conclusion. The seed is
backup-only, never in live ops, so it is not a live single-point-of-compromise. The *anchor decision
itself* remains the user's call (cross-ref `07`); it is the top unresolved design decision across the
whole corpus.

## 8. The same DID roots both planes

The public-side atproto DID and the private-side MLS-credential root are *the same lineage*. The
cross-platform `did:webvh` root (§5/§6) is the same object as the MLS root method choice (§3); the PLC
replica (§4) is the verifier for the Bluesky half of any provenance graph (§5). This is what makes "identity
you carry across platforms" one theme rather than two.

**Precise atproto facts.** `did:plc` and `did:web` are atproto's two blessed methods, and `did:plc` =
"Public Ledger of Credentials" (*cite FACTCHECK SoT*). The mechanism specifics are sourced to the
identity-resilience analysis, not the FACTCHECK: `did:plc` has a 72h recovery window where a
higher-priority rotation key overrides, and rotation keys must be k256 or p256; `plc.directory` is run by
Bluesky PBC as a transparency log (not an authority over keys), with a governance handoff planned, not
done.

---

## What this theme establishes (and does not)

**Establishes:** the identity model is *resolved* at the design level — keys≠identity, person=DID lineage,
device=distinct member; the cross-platform structural answer (no cross-network authority key; attestation,
not control) is closed; and the multi-device key-hierarchy fork was already decided (delegation + optional
offline seed).

**Does not establish:** total-device-loss recovery (the anchor decision is the user's, overlapping `07`);
native atproto `did:webvh` support (`[UNVERIFIED]`, gates §3's preferred option); whether PDS/PLC tooling
preserves extra `alsoKnownAs` entries on write (needs a real test); and the per-platform trust-model doc
remains unwritten — the highest-leverage next artifact.
