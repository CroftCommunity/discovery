# Cross-platform identity provenance: the hub-and-spoke provenance chain

`Status: cairn layer (Layer 3, the open field). Register: survey / design-grounding. Resolution: library — the
prior-art and reasoning behind Croft's cross-platform identity model; the on-the-wire method choice and the
per-platform verifier live in the identity spec, not here. External facts carry verification flags; the
atproto-native-method-set fact cites the FACTCHECK source of truth and is not re-verified here. Two user
decisions this design depends on (A9, A10) are now decided and recorded below.`

## Overview

There is no cryptographic way to make one identity be the *same* identity across two social platforms that
each insist on being their own root of authority. Every cryptographic-identity arena — atproto's DID methods,
a blockchain's key hierarchy, a web-of-trust — treats itself as the final word on who a key belongs to, and
none of them will accept another arena's say-so as binding. So the honest question is not "how do I hold one
identity everywhere" but "how do I let anyone verify, out of band, that these several arena-local identities
are the same principal." The load-bearing answer this document grounds is that **out-of-band, mutually-anchored
or root-signed provenance attestation is the only real cross-platform identity linkage.** It is a claim you can
verify, not a key you can present.

That answer has a shape: *hub-and-spoke*. A single portable *root anchor* — an append-only key history the
principal controls independently of any platform — is the hub. Each platform identity is a *spoke*: a
platform-local identifier (for atproto, a `did:plc`) that the arena resolves natively. The hub and each spoke
are tied together by an `alsoKnownAs` *equivalence ladder* that is **evidentiary, not authoritative** — a
mutually-referencing set of claims a verifier walks and checks, not a single key that opens many locks.

## Charter: what this document covers

- **In scope:** why cross-platform linkage must be provenance-by-attestation rather than one-key-many-arenas;
  the hub-and-spoke model and the portable-root-anchor mechanics (`did:webvh` SCID log, pre-rotation,
  genesis-only portability); the negative result that atproto does not natively resolve the root method and the
  standards-compliant workaround; `plc.directory` as a transparency-log-not-CA and its centralization soft
  spot; and the supporting prior art (tooling, the offline-principal delegate, and the equivocation-detection
  lineage).
- **Out of scope (and where it lives):** the on-the-wire preferred-DID-method choice and the per-platform
  verifier write-up (field used, what Croft claims and does not claim, backlink mechanism, verifier steps)
  belong to the identity spec; NSID / Lexicon naming authority lives in
  `atproto-nsid-and-lexicon-mechanics.md`; the wider atproto ecosystem positioning lives in
  `atproto-ecosystem.md`. This document is the *why*, not the *how*.
- **Boundary call:** this stops at the model and its prior art. Two design decisions it depends on (the
  anchor-URI stability contract and the location of the rotation key) have now been decided and are recorded
  below.

## The thesis: provenance attestation, not a shared key

The instinct is to want a single identity that "logs in everywhere." That instinct is wrong for a federated,
cryptographic world, and the reason is worth carrying because the whole design falls out of it.

Each cryptographic-identity arena is self-certifying: it defines what a valid identifier is, how a key binds to
it, and how that binding changes over time, and it will not delegate that authority outward. atproto will
resolve its blessed DID methods and no others; a chain will honor its own key hierarchy; a web-of-trust honors
its own signatures. There is no arena that says "whatever some other system vouches for, I accept as my own
identity." So a key that is authoritative in one arena is inert in the next. Worse than inert: reusing a single
operational key across arenas is the *key-correlation* anti-pattern the did:plc spec warns against — the same
key surfacing in each arena's public records is exactly what lets an observer prove two identities are the same
person, collapsing the privacy the separation otherwise preserves. So the shared key is both structurally
useless and actively harmful. `[did:plc key-correlation warning: cite the FACTCHECK source of truth, do not
re-verify]`

What *does* cross the boundary is a verifiable claim. If a principal controls a durable root anchor, and each
platform identity carries a signed, mutually-referencing pointer to that root (and the root points back), then
any third party can walk the links and check the signatures and conclude "these are the same principal" — to
whatever confidence the weakest link supports. That is provenance by attestation. It is *evidentiary*: it
produces evidence a verifier evaluates. It is explicitly **not authoritative** — no arena is being asked to
cede control, and no single key is being presented as universally valid. The mental model to reject is "one
key in many locks"; the model to keep is "one root whose ownership every arena-local identity can be shown to
share."

This is also why the linkage is *attestation*, not key derivation. The tempting shortcut — derive every
platform key from one seed, BIP32-style — buys key-management convenience but not public provenance:
hierarchical derivation is a secret-side relationship, provable only by revealing the parent. The link that
crosses arenas has to be a signed *statement about* an independent key (the shape X.509 and verifiable
credentials already use), so that anyone can verify it by checking signatures to the root, revocation is per
child, and a compromised child exposes neither its siblings nor the root.

This is why the architecture is hub-and-spoke rather than a mesh of peer equivalences. A mesh (every identity
linked to every other) has no privileged point of recovery and degrades to as many trust roots as there are
platforms. A hub — one portable root the principal owns outright — gives the equivalence ladder a single anchor
to climb to, and gives the principal one place to prove continuity and one place to exit from if a platform
turns hostile.

## The hub: a portable root anchor (`did:webvh`)

The root anchor is a DID method built for exactly this: web-plus-verifiable-history.

**`did:webvh`** (formerly `did:tdw`) anchors an identifier to a *self-certifying identifier* (SCID) computed
over the genesis entry of an append-only key-history log (a `did.jsonl` chain). Because the SCID is derived
from the log's origin rather than from the hosting domain, the identifier's continuity does not depend on the
domain never changing — the log is the authority, the domain is merely where it is served.
`[did:webvh material: dialogue-sourced 2026-06-20, pending independent verification]`

Three properties make it the right hub:

- **Append-only history with pre-rotation.** Each log entry commits, via `nextKeyHashes`, to the hash of the
  key that will authorize the *next* entry. An attacker who compromises the current key still cannot forge the
  next rotation, because the log already committed to a hash they cannot preimage. This is the mechanism that
  lets the root survive key compromise rather than being defined by whatever key is current.
  `[dialogue-sourced 2026-06-20, pending independent verification]`

- **Genesis-only portability for credible exit.** A `portable:true` flag, settable only at genesis, permits
  the identifier to move its hosting location later without breaking continuity. This is the *credible exit*
  property: a principal can leave a hosting provider and take the root with them, and verifiers can still walk
  the unbroken log. Portability is a decision made once, at creation, precisely so it cannot be revoked by a
  later custodian. `[dialogue-sourced 2026-06-20, pending independent verification]`

- **A discovery surface.** The method carries a `/whois` LinkedVP presentation surface for publishing
  credentials tied to the root, which is where the outward-facing provenance claims can live.

The reason the root is a `did:webvh` and not simply a platform identity promoted to root: a platform identity
is only as durable as the platform. The point of the hub is to be the one thing that outlives any single
arena, so it has to be a method the principal can host and move independently.

## The negative result (load-bearing): atproto does not resolve the root

The single most important constraint in this whole design is a negative one, and it must be carried explicitly
because the workaround only makes sense once you feel the constraint.

**atproto does not natively resolve `did:webvh`.** atproto's blessed DID-method set is `did:plc` and `did:web`
only. `[atproto native-method-set: cite the FACTCHECK source of truth, do not re-verify; couples thread T7]`
A `did:webvh` identifier, however good a portable root it is, is therefore not usable *as an atproto identity*.
You cannot present the hub to atproto and have atproto treat it as an account.

There is a deeper reason atproto will not simply add it, and it is per Newbold: `did:webvh` is not a blessed
atproto method, and its portability model — an identifier whose hosting can move — does not fit atproto's
immutable-account-DID model, in which an account's DID is fixed for the life of the account. A `did:webvh`
string effectively mutates when its hosting location changes; atproto's account model assumes it never does.
So the mismatch is structural, not merely a missing feature. `[attribution: per Newbold; dialogue-sourced
2026-06-20, pending independent verification]`

The standards-compliant workaround is the bidirectional **`alsoKnownAs`** link. `alsoKnownAs` is the
W3C-standard equivalence field on a DID document. The principal's atproto spoke (`did:plc`) lists the
`did:webvh` root in its `alsoKnownAs`, and the root's log lists the `did:plc` spoke in its `alsoKnownAs`; a
verifier confirms the claim is asserted from both ends before trusting it. This is the equivalence ladder in
practice: a mutually-referenced, checkable claim, not a shared key. Two stronger fields exist, and the reason
they *cannot* be used is the reason `alsoKnownAs` is the right transport. `equivalentId` carries a
mutually-*guaranteed* (not merely-asserted) meaning, and `canonicalId` names a single upstream canonical
reference — but both must be *enforced by the governing DID method*, which makes them single-method-only and
unable to span platforms. What the design actually wants is the *semantics* of `canonicalId` (one upstream root
every spoke points back to) carried over the *transport* of `alsoKnownAs`, because only the weak, unenforced
field crosses arena boundaries. Two mechanical consequences follow. First, the link has to live *inside*
`alsoKnownAs` — atproto's PLC operations reject unknown top-level fields, so an invented `provenanceAnchor`
field is not an option. Second, validation degrades gracefully: bidirectional presence is what the verifier
checks, and the *absence* of the backlink means unverified, not false (per DID-Core, it is "best practice not
to consider two identifiers equivalent in the absence of the inverse relationship").
`[alsoKnownAs / equivalentId / canonicalId distinctions: cite the FACTCHECK source of truth, do not re-verify]`

This is why the model is hub-and-spoke and not "run the root as your atproto identity": atproto structurally
cannot host the root, so the root stays outside and the atproto-native spoke links back to it.

## The spoke, and its soft spot: `plc.directory` is a transparency-log, not a CA

The atproto spoke is a `did:plc`, and it is essential to be precise about what backs it, because it is the
known centralization risk in the whole chain.

**`plc.directory` is a transparency-log, not a certificate authority.** It resolves `did:plc` identifiers
(`GET /{did}`) and exposes a per-identifier audit log (`/{did}/log/audit`), and it holds 12M+ operations. Its
records are *self-certifying*: each operation is signed by a key the identifier itself controls, and the
directory's job is to order and publish those operations, not to vouch for identities the way a CA vouches for
certificates. The directory cannot mint a valid operation on your behalf; it can only serve (or, at worst,
withhold or reorder) the operations you signed. `[dialogue-sourced; consistent with the plc-identity design
discussion]`

That distinction is what makes `did:plc` acceptable as a spoke despite being operated by a single party — the
authority is the signing key, not the directory. But it is also where the soft spot lives: the directory is a
single well-known operator, and a single operator is a single point of availability and a single point of
potential equivocation (serving different histories to different clients). A governance handoff of the
directory to a nonprofit has been planned but **not done**. `[dialogue-sourced, pending independent
verification]` A second, quieter property sharpens why the spoke is a spoke: the full PLC operation history is
permanently public and non-redactable — every past handle and PDS URL a principal ever used stays exposed even
after deactivation, so any PII in a handle is exposed forever. `[dialogue-sourced, pending independent
verification]` So the spoke is a spoke by design: `did:plc` is where atproto resolution happens, but it is not
the root of the principal's identity — the `did:webvh` hub is. The reason to keep the hub external is exactly
this soft spot: if the spoke's directory misbehaves or disappears, the principal's durable identity is still
anchored in a log they control.

## The convergence bet: why keep the root anchored even though nothing reads it

There is a reason to maintain the `did:webvh` root even while no atproto component resolves it, and it is less a
prediction than cheap insurance. `did:plc` and `did:webvh` are structurally almost the same object: both are a
self-certifying identifier (a hash of a genesis operation) over an append-only signed log, resolved by
replaying that log. The only real differences are *where the log is hosted* and *whether the resolver checks
the genesis hash against the identifier*. An atproto discussion (#2705) sketched the convergence: if a PDS
served the full PLC operation log back to inception and validated the genesis op against the CID embedded in
the `did:plc` string, each PDS-hosted `did:plc` would itself be a valid `did:webvh` — one identity, two
resolution paths (though not every `did:webvh` would be intelligible to atproto). The design rule that falls out
is to keep the `did:webvh` SCID as the anchor now, even though nothing reads it: if convergence ever lands, an
already-SCID-anchored identity slots straight in; if it never lands, the SCID anchor is independently useful as
the provenance root. This is hedge-positioning, not a roadmap bet — and it is consistent with the same Newbold
stance recorded above: adopting `did:webvh` as a *method* is conditionally on the table (gated on validation
complexity and library support), while `did:webvh` *portability* is what does not fit atproto's
immutable-account-DID model. `[atproto #2705 sketch: dialogue-sourced 2026-06-20, pending independent
verification]`

## Supporting prior art

The model is not invented from nothing; each piece has prior art that makes it a build-it-today direction
rather than a hope.

- **`didwebvh-rs` / `didtoolbox`** — implementations and validators for `did:webvh`: log-chain validation, SCID
  continuity checking, and pre-rotation key provisioning. These are the tooling that makes the hub buildable
  now in Rust. `[dialogue-sourced, pending independent verification]`

- **`goat` (the Go atproto CLI)** — demonstrates the real `did:plc` operation flow end to end: recommend the
  operation, edit it, request an email token, sign, and submit, with the PDS signing and forwarding. It is the
  concrete evidence that spoke-side operations are a well-trodden path, not a paper design.
  `[dialogue-sourced, pending independent verification]`

- **DIDComm Mediator Coordination / Pickup** — hold-and-forward messaging for offline DID controllers. This is
  near-exact prior art for a capability-only, offline-principal *delegate*: a component that holds and forwards
  on behalf of a principal who is not online and holds no authority of its own beyond the capability it was
  handed. The lesson is that the offline-principal case is a solved shape in the DID world, not a novel problem.
  `[dialogue-sourced 2026-06-20, pending independent verification]`

- **Certificate Transparency (RFC 6962) + CT gossip, and CONIKS** — the equivocation-*detection* lineage, and
  the most instructive prior art for making a single-operator directory trustworthy without trusting the
  operator. CT's model is not "prevent a bad log entry" but "make an inconsistent one detectable": each
  principal monitors its own binding, and gossip cross-checks between observers force the log toward
  non-equivocation, with no trusted center required. CONIKS (USENIX Security 2015) applies the same shape to
  end-user key directories: each user monitors their own key binding so that a directory cannot substitute a
  key without the owner noticing. This is directly the model for keeping `plc.directory`'s soft spot honest —
  the directory need not be trusted if every principal monitors its own history and observers gossip to catch a
  fork. `[RFC 6962: published standard; CONIKS: USENIX Security 2015; framing dialogue-sourced 2026-06-20]`

The through-line of the prior art is the same as the thesis: none of these rely on a trusted center. CT and
CONIKS detect misbehavior by distributed monitoring; the delegate holds only a delegated capability; the SCID
log's authority is the log itself. The provenance chain is trustworthy because it is checkable from the edges,
not because any node is believed.

## Two decisions this depends on (now decided)

This design does not stand entirely on its own; it rests on two decisions that were the user's to make. Both are
now decided, and the resolutions are recorded here so the dependency and its answer are visible together.

- **A9 — the anchor-URI stability contract.** What guarantee does the root's hosting URI carry over time, and
  who is on the hook for it? The `did:webvh` `portable:true` property makes moving the hosting *possible*, but
  the operational contract — how stable the anchor URI is expected to be, and what a verifier should assume
  when it changes — is a policy decision, not a mechanism the method settles. **Decided:** the anchor is a
  *stable logical URI* (mutable content at a domain/path committed to staying resolvable or redirecting), not a
  content-addressed immutable identifier, **plus a portable, self-held offline proof-of-prior-identity** so that
  a location change (`foo@bar` → `foo@baz`) or a domain lapsing or folding does not sever the provable link.
  Mechanically this rests on what the model already carries: the `did:webvh` SCID over the `nextKeyHashes`
  pre-rotation log is the portable, self-anchored history, and the bidirectional `alsoKnownAs` equivalence
  ladder binds the old and new locations — so the old→new binding lives in a self-held signed chain and does
  not depend on the old domain still resolving.

- **A10 — PDS-held vs self-controlled `did:plc` rotation key.** Does the principal's atproto spoke rotation key
  live with the PDS (convenient, but the host can rotate you) or under the principal's sole control (sovereign,
  but the principal bears the recovery burden)? This directly sets how much the spoke depends on its host and
  how "credible" the credible-exit really is in practice. **Decided:** rotation-key custody is *pluggable with
  a safe default* — PDS-held is acceptable as a governance choice (not a protocol mandate), self-control is the
  available sovereignty option, and both persist long-term. A9's portable proof preserves credible exit
  regardless of which custody a principal runs.

Both feed the identity spec's on-the-wire method choice and the per-platform verifier write-up.

## What this establishes (and does not)

Establishes that cross-platform identity linkage in a federated cryptographic world is necessarily
provenance-by-attestation — evidentiary, out-of-band, mutually-anchored or root-signed — because every arena is
its own final authority and will not accept a foreign key as its own; that the resulting architecture is
hub-and-spoke, with a portable `did:webvh` root anchor (SCID-anchored append-only log, `nextKeyHashes`
pre-rotation, genesis-only `portable:true` for credible exit) as the hub and platform-local identifiers as
spokes, tied by an `alsoKnownAs` equivalence ladder that is evidentiary and not authoritative; that the
governing constraint is the negative result that atproto resolves only `did:plc` and `did:web`, so the root is
not natively usable as an atproto identity and the standards-compliant workaround is the bidirectional
`alsoKnownAs` link; that `plc.directory` is a self-certifying transparency-log rather than a CA, which is what
makes `did:plc` acceptable as a spoke and also names its centralization soft spot (and its permanently-public,
non-redactable history); that `did:plc` and `did:webvh` are structurally near-identical, which is the
convergence bet that makes keeping the SCID anchor cheap insurance rather than wasted effort even though nothing
reads it today; and that each piece has prior art — `didwebvh-rs`/`didtoolbox` tooling, the `goat` PLC-op flow, the DIDComm hold-and-forward delegate,
and the CT/CONIKS equivocation-detection lineage — that makes the model buildable and its trust model
center-free.

Does **not** decide the on-the-wire preferred-DID-method or write the per-platform verifier (those belong to
the identity spec), does **not** re-verify the atproto native-method-set fact or the `alsoKnownAs`/`equivalentId`
distinction (both cite the FACTCHECK source of truth, coupling thread T7), does **not** certify the
dialogue-sourced `did:webvh` and prior-art rows (they carry verification flags and need a refresh pass against
primary sources before external use). Decisions A9 (the anchor-URI stability contract) and A10 (PDS-held vs
self-controlled `did:plc` rotation key) are now decided and recorded above — A9 as a stable logical URI plus a
portable, self-held offline proof-of-prior-identity, and A10 as pluggable rotation-key custody with a safe
default.
