# Cross-platform identity provenance: the hub-and-spoke attestation model

**Context:** Croft's public side integrates with atproto; a person will also carry presence on other
networks (ActivityPub/Mastodon, Hive, etc.). The recurring question — "can I have one root identity
across all of them?" — has a definite, structurally-forced answer. This doc captures it. It is the
*cross-platform linkage* companion to [`plc-identity-resilience.md`](plc-identity-resilience.md),
which covers the narrower question of *which DID method roots an MLS identity* and *how to build a
validating PLC read-replica*. Read that for the MLS-root decision; read this for "how do I prove one
person owns these accounts on different networks."

**Status:** Research + design, distilled from the 2026-06-20 identity-provenance dialogue
(`seeds/transcripts/raw/croft-identity-provenance-dialogue-2026-06-20.md`, which did real in-session
web verification and cites the did:plc spec, W3C DID-core, the did:webvh spec, and atproto
discussions #2705/#2821). Volatile facts (native atproto did:webvh support; PLC governance handoff)
remain `[UNVERIFIED]` and align with `plc-identity-resilience.md`. The atproto/iroh source of truth
is `seeds/transcripts/raw/atproto-atmospheric-web-iroh-mobile-FACTCHECK.md`; cross-platform provenance
is not covered there.

---

## The forced conclusion

**Each network is a closed cryptographic root of trust.** atproto treats the did:plc (or did:web) as
authoritative; Hive treats on-chain account keys as authoritative; ActivityPub treats the actor's
HTTP-Signature key (bound to the actor URL) as authoritative. None delegates its root of trust to an
external authority — doing so would import a foreign trust model and stop it being self-authenticating.

Two consequences:

1. **A cross-platform *authority* key cannot exist.** There is no key any two of these networks both
   accept as the thing that controls the account. The networks are also cryptographically incompatible
   at the key level (atproto: p256/secp256k1 in did:key tied to a PDS; Hive: its own secp256k1
   owner/active/posting hierarchy; AP: RSA/Ed25519 HTTP Signatures bound to an actor URL). Reusing one
   key as the operational key across accounts is *also* the privacy anti-pattern the did:plc spec warns
   against (key correlation in the public audit log reveals two identities are the same person). The
   wall is structural, not tooling-immaturity.

2. **Out-of-band, mutually-anchored or root-signed provenance attestation is the only real
   cross-platform linkage mechanism.** The root does not *control* the spokes; it makes verifiable
   *claims* about them, living outside any single network's authoritative path. A willing verifier
   resolves the root, reads the attestation, and independently confirms the link. The root's value is
   **evidentiary, not operational** — a correlation/provenance anchor, not a controller.

## The shape: hub-and-spoke, not one key in many locks

```
                       ┌─────────────────────────────┐
                       │   ROOT  =  did:webvh         │
                       │   SCID-anchored, verifiable  │
                       │   history; update key OFFLINE│
                       │   (correlation anchor, not   │
                       │    a controller)             │
                       └───────────┬─────────────────┘
            alsoKnownAs / signed VC │ (out-of-band attestation, each
                                    │  side signed by its OWN native authority)
        ┌───────────────┬──────────┴────────┬──────────────────┐
        ▼               ▼                   ▼                  ▼
   ┌─────────┐    ┌──────────┐       ┌────────────┐     ┌──────────┐
   │ Bluesky │    │ ActivityPub│      │   Hive     │     │ (others) │
   │ did:plc │    │ actor +    │      │ on-chain   │     │          │
   │ own rot+│    │ HTTP-Sig   │      │ acct keys  │     │          │
   │ sign key│    │ key        │      │ json_meta  │     │          │
   └─────────┘    └──────────┘       └────────────┘     └──────────┘
   each spoke keeps its OWN authoritative local identity; the link is additive metadata
```

- **Root:** did:webvh on a domain you operate. The durable thing with verifiable history that survives
  platform changes. Update key offline.
- **Bluesky spoke:** native did:plc with its own rotation + atproto signing keys. Bidirectional
  `alsoKnownAs` to the root.
- **ActivityPub spoke:** actor with its own HTTP-Signature keypair; backlink via the actor's
  `alsoKnownAs`.
- **Hive spoke:** account with its own key hierarchy; linkage via a signed claim in `json_metadata` /
  `custom_json` plus a reference in the root's `/whois` presentation (weaker, bespoke — Hive has no DID
  to point back with).

## Goals, ranked by what's actually deliverable

The dialogue forced an honest ranking of what a root identity gives you (they trade against each
other; you cannot max all three):

1. **Provable common ownership — fully achievable.** The hub-and-spoke delivers it. Root asserts each
   spoke, each spoke points back by its native mechanism, a verifier walks the graph. No shared key
   material, no privacy leak. The load-bearing goal.
2. **Recovery anchor — partial, network-dependent.** Bluesky: *real* — a root-controlled key in the
   did:plc `rotationKeys` array can rotate out a compromised PDS-held key. ActivityPub: *none native* —
   actor key is bound to the server; lose the server, lose the identity (the root proves you *were*
   that actor — "migration with portable proof," not key rotation). Hive: recovery is Hive's own
   owner-key mechanism, which doesn't know your DID; root holds only an attestation.
3. **One key to operate everything — not achievable; drop it as a goal.** Incompatible key types + the
   privacy anti-pattern. What you get instead is the *felt* experience of "one identity" via one
   offline root that authorizes/anchors per-network keys.

## Key lineage = attestation, not derivation

The user's better-formed goal was "a key chain up to a root identity — generate keys with provenance
for cryptographically validated lineage." The critical distinction:

- **Hierarchical derivation (BIP32-style):** one seed generates a tree of child keys. But derivation
  is a *secret-side* relationship — not publicly provable without revealing the parent. Management
  convenience, **not** public provenance. Wrong primitive.
- **Attestation chains (what's wanted):** the root *signs a statement about* each child key. The child
  is an independent keypair; the link is a signature, not a derivation. Anyone verifies by checking
  signatures to the root. How X.509, DID verification relationships, and verifiable credentials work —
  public, verifiable, revocable; compromise of one child doesn't expose siblings or the root.

**did:webvh supports this with two composing mechanisms (verified against the spec):**

- **Vertical chain (root authority over time) — pre-rotation.** When active, every multikey in the
  `updateKeys` parameter (after the genesis entry) MUST have its hash pre-committed in the previous
  entry's `nextKeyHashes`. A compromise of the current keys can't seize control without also having
  compromised the pre-committed keys.
- **Horizontal chain (application keys) — verification methods.** Correction to a common mental model:
  did:webvh is **not** a derivation tree. Application keys are declared as `verificationMethod` entries
  with explicit purposes (`authentication`, `assertionMethod`), introduced by a root-authorized log
  entry. **The provenance is the signed log entry, not a derivation path.** Off-document / external
  keys (the did:plc, an AP HTTP-Sig key, a Hive key) are attested by a signed verifiable credential
  served via the `/whois` LinkedVP endpoint, or asserted via `alsoKnownAs`.

**Limitation to name loudly:** a verifier on Bluesky/Hive/AP will not walk this chain out of the box —
each checks its own native key. The lineage is verifiable by anyone who *chooses* to resolve the
did:webvh; it is not enforced by those networks. Enforcing it = custom verifier logic on our side.

## The linkage field: `alsoKnownAs`, and the equivalence ladder

The right field is `alsoKnownAs`, and its weakness is the feature:

- DID-Core: `alsoKnownAs` asserts that two identifiers identify the same subject, but its presence
  "does not prove that this assertion is true" — a verifier is "strongly advised to obtain independent
  verification." Validation rule: "best practice not to consider two identifiers equivalent in the
  absence of the inverse relationship." So **bidirectional presence is the mechanical validation;
  absence just means unverified, not false** — graceful degradation.
- Value is an ordered set of RFC 3986 URIs; a routable `https://` URI is valid and need not be a DID.

**The equivalence ladder** (the useful framing): `alsoKnownAs` (weakest — bare claim, checkable via
backlink, never authoritative, cross-platform, mutable) → `equivalentId` (stronger but MUST be
guaranteed by the governing method → single-method only, can't span platforms) → `canonicalId` (a
single canonical value, same method-enforcement catch). **We want the *semantics* of `canonicalId` (one
upstream canonical reference) but must use the *transport* of `alsoKnownAs`**, because the strong
properties require method-level enforcement that doesn't exist cross-platform. Canonical-reference
meaning without over-claiming.

## Per-platform field support (what actually persists)

| System | Field | Arbitrary upstream URI? | Honored/validated by platform | Backlink quality |
|---|---|---|---|---|
| AP protocol | `alsoKnownAs` | Yes (not type-constrained) | N/A (just data) | Good |
| Mastodon | `alsoKnownAs` | Tolerated, UI limited to actor aliases | Only as migration alias | Coexists, ignored |
| GoToSocial | `alsoKnownAs` | Tolerated | Only as migration alias | Coexists, ignored |
| Threads | unknown/partial | Not reliably | No | Don't rely — outbound claim only |
| atproto/PLC | `alsoKnownAs` array | Yes (extra entries); **no new top-level fields** | First `at://` only; rest ignored, not cross-validated | Good, but governed unilaterally by Bluesky PBC |
| Hive | `json_metadata` / `custom_json` | Yes, fully arbitrary | No (bespoke) | Strong as signed attestation, bespoke format |

Notes that bite:
- **atproto:** extra `alsoKnownAs` entries are tolerated and uncross-validated, but PLC operations
  reject unknown top-level fields and duplicate AKA entries — the link MUST live inside `alsoKnownAs`
  (or `services`), not in an invented `provenanceAnchor` field. The `at://` handle slot is reserved
  (first valid wins); your provenance URI goes *after* it. Persistence is real today but a *soft*
  guarantee — only Bluesky PBC governs what's accepted, and it could tighten. **Verify by testing**
  that PDS/PLC tooling preserves extra entries on write.
- **AP-family:** the field exists but Mastodon/GoToSocial co-opt it for migration (Move checks the
  alias); a non-actor provenance URI is inert-but-present. Threads: don't plant a backlink, only point
  at it from the root.
- **Hive:** no DID, no `alsoKnownAs`, but the most open substrate — a signed attestation in
  `json_metadata` / `custom_json` under your own account keys. Bespoke; "validation" = "signed by the
  Hive account key."

## Bridge-doc technical corrections (the "Webvh bluesky bridge" how-to)

A separate verification pass over the draft bridge how-to (the one that proposed a `serialize → sign →
POST` flow) surfaced concrete errors. Folded here so the how-to is corrected at the distilled layer
(raw: the appendix of `seeds/transcripts/raw/croft-architecture-design-dialogue-2026-06-20.md`, plus
the goat request-token→sign→submit flow in `croft-identity-provenance-dialogue-2026-06-20.md`). Some
points are `[UNVERIFIED]` — the verification session had web search down; confirm against the did:webvh
spec (identity.foundation) and the did:plc spec before acting.

- **"W3C DID Core *Equivalency Assertion*" is invented terminology.** `alsoKnownAs` is real; DID Core
  distinguishes it (asserted, not verified) from `equivalentId`/`canonicalId` (method-enforced). The
  *equivalence ladder* above is the accurate framing.
- **A did:webvh log line is not a bare DID document.** Each entry wraps `versionId`, `versionTime`,
  `parameters`, the DID-doc state, and a Data-Integrity proof, chained to the prior entry hash; the
  proof covers the canonicalized entry. Hand-stripping newlines and appending is not how a conformant
  log is produced — use reference tooling (didwebvh-rs / didtoolbox). `[UNVERIFIED — entry schema
  changed across the did:tdw → did:webvh rename; confirm against the current spec.]`
- **Genesis `prev` must be `null`.** Only updates carry a real `prev` CID; a genesis example showing
  `prev: bafyrei…` misleads anyone creating a fresh identity.
- **Cross-system key encodings are not interchangeable.** PLC currently restricts key types to
  **secp256k1 / p256**; an Ed25519 `z6Mk…` key valid in a did:webvh verificationMethod would **not** be
  valid as a PLC rotation key. `verificationMethods` values use full `did:key:` encoding (not bare
  multibase). `[UNVERIFIED — confirm PLC's allowed key types.]`
- **The PLC submission flow needs the email token** (omitting it is a hard blocker): `goat account plc
  recommended → edit → request-token → sign → submit`; the PDS signs and forwards. Hand-rolling DAG-CBOR
  and signing raw bytes is possible but is not the documented path.
- **"Cannot be faked" is too strong.** `alsoKnownAs` is an unverified assertion unless a validator
  performs the bidirectional round-trip; security comes from the verifier *choosing* to require both
  directions, not from the data structure. (Consistent with the equivalence-ladder framing: bidirectional
  presence is the mechanical validation; absence means *unverified*, not *false*.)

## did:web vs did:webvh — portability is the whole point

- **Plain did:web is not portable, by design** — the identifier *is* the domain; no migration/recovery
  mechanism. Lose the domain, lose the identity. Wrong primitive for "separate identity from
  reachability"; it welds them.
- **did:webvh is portable via the SCID.** The stable anchor is the SCID, not the domain; the web
  location can move while the verifiable history is preserved. **Rename rules (exact):** the renamed
  DID's log contains all entries from creation; the rename entry builds validly on prior entries;
  `portable` is `true`; the SCID is unchanged; the DIDDoc carries the prior DID string in
  `alsoKnownAs`. **`portable` is genesis-only and one-way** — it can ONLY be set true in the first log
  entry; if not set there it MUST be false, and once false MUST NOT be set true again. **You cannot
  retrofit portability — decide at creation.**

So: identity (SCID + key history) is separated from reachability (whatever domain hosts the log) — for
any verifier that speaks did:webvh. **But atproto will not follow the move automatically** (it doesn't
resolve did:webvh or check SCIDs). The bridge: use a **did:plc as the Bluesky account DID** (immutable
to atproto, survives handle/domain changes natively) and the **portable did:webvh as the off-network
root/provenance anchor**, bidirectionally linked.

## The convergence bet (cheap insurance, not a prediction)

`plc.directory` is a central directory run by Bluesky PBC (`GET https://plc.directory/{did}`; audit
log at `/{did}/log/audit`) — the well-known soft spot in atproto decentralization. **But it is a
transparency log, not an authority over your keys:** every operation is self-certifying, signed by
your rotation keys; the did:plc identifier is a hash of the genesis op. Bluesky can censor or (worst
case) equivocate, but cannot forge operations your keys didn't sign. Closer to Certificate
Transparency than a root CA.

**did:plc is structurally "almost already" a did:webvh:** both are a self-certifying ID (hash of
genesis) + an append-only signed log + resolve-by-replay. The only differences are *where the log
lives* and *whether the resolver checks the genesis hash*. The atproto thread (#2705) sketched: if the
PDS hosted the full oplog back to inception (validating the genesis op against the CID in the did:plc
URI), did:plc becomes "a variant expression of did:webvh" — each PDS-hosted did:plc a valid did:webvh,
but not every did:webvh intelligible to atproto. One identity, two resolution paths.

**Design rule that falls out:** don't treat the Bluesky did:plc as the root identity — treat it as a
spoke, with the did:webvh SCID as the root, and link them. Keep the did:webvh SCID as the anchor *even
though nothing reads it today*. If the convergence lands, an identity already SCID-anchored slots in;
if it never lands, the SCID anchor is independently useful as the provenance root. Newbold's stance:
adopting did:webvh as a *method* is conditionally on the table (gated on validation complexity +
library support), but did:webvh *portability* explicitly does not fit atproto's immutable-DID data
model. So this is hedge-positioning, not a roadmap bet — the non-foreclosing posture throughout.

## Open decisions (surface, do not resolve — see ROADMAP_TODO)

- **Anchor-URI stability contract.** `alsoKnownAs` wants an RFC 3986 URI, but it must NOT be frozen.
  The anchor should be a **stable logical URI** (a domain/path committed to staying resolvable or
  redirecting) whose **content is allowed to evolve** — the URI is the promise, the payload mutable.
  Pointing at a content-addressed immutable thing re-introduces the freezing being avoided. Deliberate
  choice, not default.
- **PDS-held vs self-controlled did:plc rotation key.** Changes who can issue future operations and
  whether the root genuinely functions as a recovery anchor for the Bluesky spoke. Staging advice in
  the dialogue: keep the PDS key as fallback, add a self-controlled higher-priority key offline,
  exercise a test rotation before dropping the PDS key. (72h recovery window; key custody is the risk.)
- **The per-platform trust-model document is not yet written.** The dialogue repeatedly offered it and
  it remains the highest-leverage next artifact: per network — field used, what we claim / don't
  claim, backlink mechanism, exact verifier steps + pseudocode. Captured as a ROADMAP_TODO item so it
  isn't lost.

## How this connects to the rest of the corpus

- Extends [`plc-identity-resilience.md`](plc-identity-resilience.md): that doc's Part-1 DID-method
  scorecard already prefers did:webvh-on-a-domain-you-operate for the MLS root *if/when* atproto
  support is confirmed; this doc explains the *cross-platform* role of that same did:webvh root (it
  anchors the other-network spokes too) and why the linkage is attestation, never control.
- The validating PLC read-replica in that doc's Part 2 is what lets a Croft-side verifier walk the
  Bluesky half of any provenance graph without trusting `plc.directory` live.
- The "evidentiary, not operational" framing is a clean expression of the sovereignty thesis: identity
  that survives platform change because it never depended on any one platform's authority. Candidate
  for `crystallized/principles.md` (flagged, not yet inserted — the conclusion is settled, but
  promotion is the user's call).
