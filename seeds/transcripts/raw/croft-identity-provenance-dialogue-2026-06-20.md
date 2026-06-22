# Croft — cross-platform identity provenance dialogue (did:webvh ↔ did:plc ↔ AP ↔ Hive) (2026-06-20)

> **Provenance status: preserved-condensed (cleaned-paste, content-faithful — NOT a
> byte-pristine export).** Per PLAYBOOK §2b/§4: a chat the user pasted into the session; no
> canonical export exists, so this is the best-available raw. UI render chrome stripped ("Searched
> the web", "Created a file, read a file", "Document · MD", "Show more", date dividers); the
> assistant's web-search and file actions are bracketed inline as pointers. Dialogue wording is
> reproduced as faithfully as the rendered paste allowed.
>
> **Fact-check posture (PLAYBOOK §4):** this dialogue did real in-session web verification and cites
> sources inline (the did:plc spec, W3C DID-core, the did:webvh spec, atproto GitHub discussions
> #2705/#2821, Hive/Mastodon/GoToSocial docs). Those citations are the dialogue's own; volatile
> claims (did:webvh native atproto support, PLC governance handoff status) remain `[UNVERIFIED]`
> here and align with `thinking/plc-identity-resilience.md`. The atproto/iroh source of truth
> remains `atproto-atmospheric-web-iroh-mobile-FACTCHECK.md`; this dialogue covers *cross-platform
> provenance linkage*, which that file does not.
>
> **Artifact gap (§4):** the session produced a markdown artifact, "Webvh bluesky bridge" (a
> corrected PLC-section how-to). The user's pasted input *draft* of it is reproduced below; the
> assistant's final corrected artifact full text was NOT in the paste — only its summary of changes.
> Marked inline as a pointer rather than fabricated. Distilled to
> `thinking/cross-platform-identity-provenance.md`.

---

## USER — [pasted a document for verification]

> The W3C Decentralized Identifier (DID) core specification solves this cross-network mapping
> challenge through **Equivalency Assertion**. Instead of forcing a non-native DID method into an
> ecosystem that doesn't compile it, you configure a bidirectional, cryptographically signed graph.
> Any downstream … [the document described linking a did:webvh sovereign identity to a Bluesky
> did:plc via `alsoKnownAs` on both sides, with a "serialize, sign, POST" PLC submission flow.]

## ASSISTANT

I'll verify the technical claims here before endorsing the workflow, since several details are
checkable against the actual specs. [Verified the Bluesky/PLC side via web search.]

**What checks out.** The fundamental premise is correct. The linkage between a DNS domain and a
did:plc identifier is bidirectional: the DID document points at the domain via `alsoKnownAs`, and the
domain points back at the DID. Using `alsoKnownAs` for equivalency assertion is the right mechanism
on both sides. The did:webvh outbound side is mostly accurate: the spec requires stripping
whitespace, appending each entry as a single JSON line, signing with the authorized update key, then
publishing to the `did.jsonl` log.

**Errors and problems:**

1. **The did:plc `verificationMethods` format is wrong.** The document uses bare multikey values
   like `"atproto": "zQ3shun..."`. The spec requires full did:key encoding: `"atproto":
   "did:key:zQ3shun..."`. The bare-key version fails validation.

2. **The PLC submission flow omits the email token, which is a hard blocker.** You cannot just sign
   and POST a PLC operation. For security you need a token from email to upload changes: request it,
   add it to the identity document as a `"token"` field before signing and submitting. The doc's
   three-step "serialize, sign, POST" skips this entirely, so it won't work as written.

3. **"Sign the raw output bytes" is imprecise.** In normal atproto practice you don't hand-assemble
   DAG-CBOR and sign raw bytes. The standard flow edits an unsigned operation JSON, then submits it
   to the PDS to be signed with one of the PLC rotation keys the PDS holds. Hand-rolling is possible
   but is not the documented path.

4. **"W3C DID core … Equivalency Assertion" is a fabricated label.** DID Core defines the
   `alsoKnownAs` property; there is no named feature called "Equivalency Assertion." The mechanism is
   real; the formal name is invented.

5. **The did:webvh DIDDoc block is incomplete as a log entry.** An actual `did.jsonl` line wraps the
   DIDDoc inside a `state` property, alongside `versionId`, `versionTime`, `parameters`, and a
   `proof`. The bare DIDDoc shown is the state payload, not the line you append.

6. **Terminology slip.** The doc calls did:plc "centralized-but-transparent" while also calling the
   setup "sovereign." did:plc control rests entirely in the rotation keys, and `plc.directory` is run
   by Bluesky PBC. The sovereign half is real; the Bluesky half is not.

**On the security claim:** "cannot be faked" holds only if both key sets are uncompromised. An
attacker who can issue a PLC operation could add their own rotation key or change the `alsoKnownAs`
binding, hijacking the identity. The bidirectional handshake resists outsider spoofing but is not
robust against rotation-key compromise or a malicious PDS. Better stated as that than the flat
"cannot be faked." If you want, I can rewrite the PLC section to reflect the actual `goat account plc`
request-token → sign → submit flow.

## USER

> Yes

[Pasted the bridge doc draft — "## The Goal / The Approach: Bidirectional Equivalency Assertion
(Sovereign-to-Social outbound via did.jsonl alsoKnownAs sealed by an offline Update Private Key;
Social-to-Sovereign inbound via a signed PLC rotation op inserting the did:webvh URI into
alsoKnownAs) / ## Validation Strategy (1. Ledger & Operation Integrity Check — webvh log via
didwebvh-rs, PLC registration via HTTP GET; 2. Automated Handshake Graph Walk — resolve each side,
confirm the bidirectional loop closes; 3. Application Layer Run-Time Check — log into a Bluesky client
by domain handle, confirm posts/likes/followers index, confirming the extra alsoKnownAs URI doesn't
degrade function)."]

## ASSISTANT

I'll write the corrected PLC section. [Verified the PLC query endpoint format and what the goat
submission flow returns, via web search.] Two corrections to your validation strategy first: the
resolution endpoint is `https://plc.directory/{did}` with the full DID as a single path segment (not
the colon-split form you wrote), and the more rigorous check uses the audit log endpoint.

[**Artifact created: "Webvh bluesky bridge" (markdown).** Full corrected text was not in the paste;
the assistant summarized the changes — reproduced below as the faithful record.]

Key changes from the original draft:

- The PLC submission flow is now real: `goat account plc recommended → edit → request-token → sign →
  submit`. The email-token step the original skipped is in place. The PDS signs and forwards (a
  did:plc operation managed by the PDS is signed by the PDS and submitted on the user's behalf).
- `verificationMethods` now uses full `did:key:` encoding, matching the spec rather than the bare
  multibase.
- Handle ordering is called out: the first syntactically valid handle in `alsoKnownAs` is the claimed
  handle; other handle URIs are ignored. Your did:webvh URI goes *after* the `at://` handle, not in
  place of it.
- Validation-strategy fixes: resolution endpoint is `https://plc.directory/{full-did}` as one path
  segment; added the audit-log check (`https://plc.directory/{did}/log/audit`) so a verifier can
  fetch the operation/audit log and fully re-validate rather than trust the directory's returned
  document.
- "Cannot be faked" softened to "resistant to third-party spoofing," because a rotation-key
  compromise or malicious PDS can rewrite the did:plc side.

**Open decision flagged (not assumed):** whether the did:plc uses a PDS-held rotation key or a
self-controlled one — it changes who can issue future operations.

## USER

> The goal is to have a root key pair used in various applications like bluesky, hive, AP chat etc

## ASSISTANT

That goal changes the architecture meaningfully, and exposes a hard constraint. Separate two things
you might mean by "root keypair":

**If you mean one key that signs/controls every account directly:** this is the anti-pattern. The
did:plc spec is explicit that reusing rotation or signing keys across accounts leaks identity linkage
publicly (key correlation in the audit log can reveal two identities are the same person). Beyond
privacy, the networks are cryptographically incompatible at the key level: atproto's signing key must
be p256 or secp256k1 in did:key form tied to a PDS; Hive uses its own secp256k1 posting/active/owner
hierarchy on its own chain (no DIDs); ActivityPub actors use an RSA (or increasingly Ed25519) keypair
for HTTP Signatures bound to an actor URL. There is no single key material all three accept as a
control key.

**If you mean one root identity that anchors and proves ownership of the others:** this is the
achievable and good version, and it's what the did:webvh bridge points toward. The root is your
did:webvh. It does not sign for the other networks. It holds per-network keys as verification methods,
or asserts the per-network identifiers via `alsoKnownAs`, and each network keeps its own native key.
The root proves "these all belong to me"; it does not operate them.

So the correct shape is hub-and-spoke, not one key in many locks:

- **Root:** did:webvh on your domain. Update key offline. The thing with verifiable history that
  survives platform changes.
- **Bluesky spoke:** native did:plc with its own rotation + atproto signing keys. Bidirectional
  `alsoKnownAs` link to the root.
- **ActivityPub spoke:** actor with its own HTTP-Signature keypair. Link back via the actor's
  `alsoKnownAs`.
- **Hive spoke:** Hive account with its own key hierarchy. Linkage is weaker — a signed claim posted
  on-chain plus a reference in the root's `/whois` verifiable presentation, since Hive has no DID to
  point back with.

**Q: What's the primary thing the root keypair should give you?** → *A: All three, ranked*

**Q: How much self-custody do you want for the Bluesky side specifically?** → *A: Not sure, advise
me*

## ASSISTANT

Good. "All three, ranked" is the honest answer, because they trade against each other.

1. **Provable common ownership — fully achievable.** The hub-and-spoke delivers this cleanly. Root
   asserts each spoke via `alsoKnownAs`, each spoke points back by its native mechanism, a verifier
   walks the graph. No shared key material, no privacy leakage. Load-bearing and it works.

2. **Recovery anchor — partially achievable, network-dependent.** Bluesky: real recovery — a
   root-controlled key in the did:plc `rotationKeys` array can rotate out a compromised PDS-held key.
   ActivityPub: no native recovery; the actor keypair is bound to the actor URL on a server — lose the
   server, the identity is gone; the root can prove you *were* that actor but can't reconstitute it
   ("migration with portable proof," not key rotation). Hive: account keys are on-chain, recovery is
   Hive's own owner-key mechanism, which doesn't know your DID; the root can only hold an attestation.

3. **One key to operate everything — not achievable; drop it as a goal.** Incompatible key types +
   the privacy anti-pattern. What you can have is one offline root that authorizes/anchors per-network
   keys — the felt experience of "one identity" without the shared-secret risk.

**Self-custody on Bluesky — advice:** go full self-custody on the rotation key, but stage it. A
PDS-managed did:plc means the PDS holds the rotation key, so the PDS (or whoever controls it) can
rewrite your identity, including the `alsoKnownAs` link to your root. If recovery matters, the
rotation key has to be one you control. The catch: a self-controlled rotation key is only as good as
your key management; the did:plc recovery window is 72 hours and depends on a higher-priority rotation
key still under your control. Sensible staging: keep the PDS-managed rotation key in the array as a
fallback; generate a self-controlled rotation key offline (hardware token / offline store); add it to
`rotationKeys` as the highest-priority entry (priority order in that array is the recovery hierarchy);
once you've exercised a test rotation, decide whether to drop the PDS key. [Offered to verify whether
current goat/PDS tooling registers a self-controlled rotation key cleanly, then write the per-network
linking mechanics.]

## USER

> Being able to generate keys with provenance to use for cryptographic validated lineage is the
> goals, not one key pair but a key chain up to a root identity

## ASSISTANT

That reframes it cleanly, and it's better-formed than "one root keypair." You're describing a key
hierarchy with cryptographic provenance: a root identity that attests to derived/child keys, each
carrying a verifiable chain back to the root. Two mechanisms get called "key chains" and only one
gives you what you want:

- **Hierarchical derivation (BIP32-style):** one seed deterministically generates a tree of child
  keys. But derivation is a *secret-side* relationship — a child being derived from a parent is not
  publicly provable without revealing the parent (the whole point of HD wallets is child keys look
  unlinkable). Management convenience, not public provenance. Wrong primitive if the goal is a
  verifier confirming lineage.
- **Attestation chains (what you want):** the root key *signs a statement about* each child key. The
  child is an independent keypair; the link is a signature, not a derivation. Anyone verifies the
  chain by checking signatures up to the root. This is how X.509 chains, DID verification
  relationships, and verifiable credentials work — public, verifiable, revocable, and compromise of
  one child doesn't expose siblings or the root.

did:webvh is well-suited as the root because its premise is a verifiable append-only history of key
state. Two composing mechanisms: (1) **pre-rotation** (`nextKeyHashes` commits, in each log entry, to
the hash of the key(s) authorizing the next entry) — the vertical chain of root-authority keys over
time; (2) **verification methods + signed assertions** — the root DIDDoc declares child/application
keys as `verificationMethod` entries with relationships (`assertionMethod`, `authentication`), or
issues a verifiable credential served via the `/whois` LinkedVP endpoint. [Wanted to verify the exact
`nextKeyHashes` semantics and whether the VM model expresses "derives authority from root" directly;
searched the web.]

## ASSISTANT

The spec confirms the mechanism, with one important correction to how I framed it.

**Pre-rotation (the vertical chain) is real:** when active, all multikey representations of the public
keys in the `updateKeys` parameter (other than in the initial version) MUST have their hash in the
`nextKeyHashes` array from the previous log entry. Each root-authority key is committed-to by hash one
entry in advance; a compromise of the current keys can't take control because the attacker hasn't also
compromised the pre-committed key pairs. This is your root's own key lineage over time.

**The correction — "child keys" are not a derivation tree.** did:webvh does NOT give you a
hierarchical derivation tree. It gives you two distinct key roles: **authorization keys**
(`updateKeys`, controlling the DID, forming the pre-rotation chain — each log entry carries a Data
Integrity proof signed by an active `updateKeys` key with `proofPurpose: assertionMethod`) and
**verification methods in the DIDDoc** (your application keys, declared with explicit purposes, e.g.
`#auth-key-01` / `#assert-key-01`). The "lineage" of an application key is not that it was derived
from the root — it's that a root-authorized log entry *published a DIDDoc declaring that key with a
stated purpose*. The provenance is the signed log entry, not a derivation path. You're building an
append-only log where the root authority attests to application keys, and the root authority itself
has a pre-rotation chain over time.

**The full structure, corrected:** root authority chain (vertical, `updateKeys` + `nextKeyHashes`,
offline); application keys (horizontal, per purpose/network, declared as verification methods,
introduced by a root-authorized entry); off-document keys / external identifiers (Hive key, AP
HTTP-Sig key, the did:plc — root issues a signed VC or asserts via `alsoKnownAs`, served via
`/whois`). **One genuine limitation:** a verifier on Bluesky/Hive/AP won't walk this chain out of the
box — they check their own native key. The lineage is verifiable by anyone who chooses to resolve your
did:webvh; it's not automatically enforced by those networks. That requires custom verifier logic on
your side.

## USER

> So am OOB available provenance attestation is the only real cross platform mechanism that exists bc
> cryptographic identity based arenas all expect their identity to be authoritative independently

## ASSISTANT

Yes. That's the right conclusion, stated more precisely than I had. The core fact: each arena is a
closed cryptographic root of trust. atproto treats the did:plc (or did:web) as authoritative; Hive
treats on-chain account keys as authoritative; ActivityPub treats the actor's HTTP-Signature key,
bound to the actor URL, as authoritative. None delegates its root of trust to an external authority —
doing so would import a foreign trust model into its verification path and stop it being
self-authenticating. So a cross-platform *authority* key cannot exist. There's no key any two of these
networks both accept as the thing that controls the account; the wall is structural, not
tooling-immaturity.

What remains is out-of-band provenance attestation: the root doesn't control the spokes, it makes
verifiable *claims* about them, living outside any single network's authoritative path. A willing
verifier resolves your root, reads the attestation, independently confirms the linked identity.

**One sharpening — "out-of-band" splits into two strengths of claim.** A bare `alsoKnownAs` pointer is
an *unbacked assertion* unless the other side points back (anyone can claim it). Provenance becomes
trustworthy only when it's **bidirectional** (both identities reference each other, each signed by its
own native authority — forging it requires compromising both independent roots) or a **signed
credential** the root issues (verifiable because the root's signature chains back through the
did:webvh log). A lone outbound link does not carry weight. So: *out-of-band, mutually-anchored or
root-signed provenance attestation is the only real cross-platform linkage mechanism, precisely
because every cryptographic-identity arena insists on being its own authority.* The root is a
correlation and provenance anchor, not a controller — its value is evidentiary, not operational.
[Offered a written per-network trust model.]

## USER

> Is there a way to have an identity tied to did:web that can be portable? That was the original goal
> to separate identity from reachability. Can I move to a new domain, use a new did:web that is
> cryptographically tied to the prior in a way bluesky can understand

## ASSISTANT

Short answer: yes for the cryptographic chain, but not in a way Bluesky tracks automatically. And the
did:web vs did:webvh distinction is the whole crux. [Verified the portability specifics via web
search.]

**Plain did:web: not portable, by design.** The identifier is the domain. The spec says did:web is
inherently tied to the domain name and provides no mechanism for migration or recovering from loss of
control of the domain. Move domains → brand-new unrelated identity. `alsoKnownAs` pointers between old
and new are bare unbacked claims, and the old one dies when you lose the domain. Wrong primitive for
"identity separated from reachability" — it welds them together.

**did:webvh: yes, this is the mechanism.** The stable anchor is the SCID, not the domain. It provides
optional portability via the SCID, letting the DID's web location move and the DID string update while
retaining a connection to the predecessor and preserving verifiable history. The rename rules are
exact: the DID Log of the renamed DID contains all entries from creation; the rename entry is valid
and builds on prior entries; `portable` is `true`; the SCID is the same; and the DIDDoc contains the
prior DID string as an `alsoKnownAs` entry. **Two easy-to-miss points:** `portable` is a genesis-only,
one-way switch — it can ONLY be set true in the first log entry; if not set there it MUST be false,
and once false MUST NOT be set back to true. You cannot retrofit portability. And the history travels
with you — the new domain serves the entire `did.jsonl` from genesis plus the rename entry plus the
old DID in `alsoKnownAs`. So identity (SCID + key history) is separated from reachability (whatever
domain hosts the log).

**The Bluesky problem: it won't follow the move automatically.** atproto's blessed methods are did:plc
and did:web; it doesn't resolve did:webvh and doesn't know what an SCID is. So Bluesky can't use your
portable did:webvh as the account DID directly, and SCID continuity is invisible to it. The bridge is
the attestation layer plus atproto's one native tool — handle changes: a did:plc account can change
its handle while keeping the same did:plc identifier. So: **use a did:plc as the Bluesky account DID**
(stable to atproto, survives handle/domain changes natively), **use your portable did:webvh as the
off-network root/provenance anchor,** bidirectionally linked. When you move domains, update the
did:webvh handle on the did:plc side and add a rename entry on the did:webvh side. The portability is
real and verifiable out-of-band; it is not enforced inside atproto. [Offered the trust-model doc with
the move procedure.]

## USER

> Can you research if this is of interest or in progress? "DID methods are did:plc and did:web. It
> does not resolve did:webvh, and it does not know what an SCID is."

## ASSISTANT

[Researched whether atproto supporting did:webvh / SCIDs / portable DID methods is discussed or in
progress.] Yes — discussed directly, including by Bryan Newbold (atproto's protocol lead) and the
did:webvh authors. There's mutual interest, conditional openness, and an unresolved fundamental
incompatibility.

**Bluesky's position (Newbold, in the "did:tdw!" discussion — did:tdw being did:webvh's former
name):** on adoption as a method, depending on validation-logic complexity and ecosystem/library
support, they'd consider adding it — conditionally on the table, not rejected. But on portability he
was explicit it does NOT fit: a mechanism where the DID changes while securely proving the new DID is
the same entity "does not fit or work with atproto in its current form: the DID must stay exactly the
same over time, there is no concept of DID migration or mutation." atproto's data model assumes the
account DID is an immutable primary key (repo, follows, post references all hang off that exact
string); did:webvh portability deliberately lets the string change while preserving the SCID. Direct
architectural tension.

**The did:webvh side wants this too:** the author framed moving a DID as a powerful enabler of
"credible exit" — taking your SCID and DID history when you move services. They acknowledge the same
wall: a new domain means an entirely new DID from atproto's perspective; social graph and post threads
wouldn't migrate.

**The most interesting in-progress idea (discussion #2705):** did:plc is already web-resolvable, and
with a few PDS-API tweaks a did:webvh could be deterministically derivable from each did:plc's
`atproto_pds` service endpoint, so a single DID doc could be dual-resolvable as did:plc OR did:webvh OR
did:web. A "both/and" path: keep did:plc as the immutable atproto key, expose a did:webvh view for
verifiable-history / credible-exit. Comment-thread idea, not a roadmap item, but the shape of the only
thing that could reconcile the two.

**Where it stands:** nothing in active development (2024-era threads); the blessed set remains did:plc
and did:web with documented intent to keep it small; PLC governance handoff to a nonprofit is planned
but hasn't happened. **Design read:** do not architect around atproto natively understanding did:webvh
portability. Even the friendly version keeps did:plc as the immutable atproto anchor. That validates
the split — did:plc as the account key atproto trusts, did:webvh as your portable provenance root,
linked by attestation.

## USER

> Some way to cryptographically link that is even a known as or an associated canonical reference may
> be fine, the goal is provenance and as long as it's mechanically validatable and preserves platform
> local identity but links to an upstream in each case it may be a good enough for now without
> restricting the future, as in, aka that has a routable element like a domain but not frozen in time
> like a did:web but it's a bit academic for now to preserve the option that cannot be truly
> validated later

## ASSISTANT

You've landed on the right design, and the key insight resolves the whole tension: you don't need the
link to be *authoritative*, you need it to be *evidentiary and non-foreclosing*. Those are different
requirements and the second is much cheaper. The shape: each platform keeps its own local
authoritative identity; alongside, each carries an `alsoKnownAs` (or equivalent) pointing upstream to
a single provenance anchor that's routable today (so it resolves and can be checked) but not frozen
like a did:web (so a future domain move doesn't orphan it). The link is mechanically validatable now,
and you don't assert more than you can back, so nothing you write today becomes a false claim later.
You're optimizing for *not foreclosing options*, not maximal proof. [Verified `alsoKnownAs` semantics
permit a routable-but-mutable provenance URI; searched the web.]

**`alsoKnownAs` is the right field, and its weakness is the feature you want.** DID Core: the
assertion that two DIDs/URIs identify the same subject can be made using `alsoKnownAs`, but its
presence "does not prove that this assertion is true," so a requesting party is "strongly advised to
obtain independent verification." Exactly "good enough for now, non-foreclosing." The validation rule:
"best practice not to consider two identifiers equivalent in the absence of the inverse relationship"
— so bidirectional presence is the mechanical validation; absence just means "unverified," not
"false." Value must be an ordered set of RFC 3986 URIs; a routable `https://` URI is valid and need
not be a DID. So your "routable element like a domain but not frozen like a did:web" can be an HTTPS
URL pointing at your anchor.

**The equivalence ladder (the useful find):** `alsoKnownAs` (weakest — bare claim, verifier-checkable
via backlink, never authoritative, cross-method/cross-platform, mutable — your "good enough");
`equivalentId` (stronger but the equivalence MUST be guaranteed by the governing DID method — only
works within a single enforcing method, can't span Bluesky/Hive/AP); `canonicalId` (like
`equivalentId` but a single canonical value within the document scope). You want the *semantics* of
`canonicalId` (one upstream canonical reference) but can't use the *property* (method-level
enforcement that doesn't exist cross-platform). So: use `alsoKnownAs` as the transport, let the value
be your canonical upstream URI — canonical-reference meaning without the strong property that would
over-claim.

**The one design decision this surfaces:** the anchor URI's *stability contract*. `alsoKnownAs` wants
an RFC 3986 URI but you don't want it frozen. So the anchor should be a **stable logical URI** (a
domain/path you commit to keeping resolvable or redirecting) whose **content is allowed to evolve**.
The URI is the promise; the payload is mutable. If you instead point at something content-addressed
and immutable, you re-introduce the freezing you're avoiding. Decide deliberately. [Offered the
trust-model doc with per-platform backlink mechanics and an explicit "what we claim / don't claim"
boundary.]

## USER

> Do most blockchains allow those fields for identity? Does ATProto? Does AP?

## ASSISTANT

[Verified across systems via web search.] Splits sharply:

- **ActivityPub (protocol layer): yes, most permissive.** `alsoKnownAs` exists on AP actors and is NOT
  type-constrained — the value must be a URI, nothing constrains it to an AP actor; it can link
  identifiers in different systems. Your upstream anchor URI goes there directly. Caveat: it's
  borrowed-from-DID (Mastodon implemented it for migration), and migration use is bidirectional (Move
  requires `alsoKnownAs` both directions, old actor gets `movedTo`); a non-actor provenance URI should
  be ignored rather than mishandled, but test it. Still an unbacked claim unless the other side points
  back.
- **atproto: yes, but the authoritative slot is reserved.** The handle slot must be an `at://` URI and
  the first valid one wins; extra entries are syntactically allowed, and atproto's resolver only cares
  about the `at://` handle and ignores the rest. Ideal for "non-foreclosing": the platform-local
  authoritative identity stays intact, your upstream link rides along as inert-to-atproto metadata a
  separate verifier can read. Confirm by testing whether PDS/PLC tooling preserves extra entries on
  write.
- **Blockchains: entirely method-dependent.** `alsoKnownAs` is a W3C DID-core property, so any method
  whose docs conform to DID-core can carry it (did:ion, did:ethr, did:dht, Hyperledger Indy methods);
  `equivalentId` only binds when the governing method guarantees it (no cross-chain method does).
  Native account systems not DID-document-based (Hive) have no `alsoKnownAs` — you'd use free-form
  metadata (Hive's `json_metadata`) plus a signed attestation, bespoke per chain.

In every case the platform keeps its own authoritative local identity; your upstream link is additive
metadata whose trust comes from the bidirectional backlink, never from the platform enforcing it.

## USER

> Yes and HIVE too and AP along with mastadon and threads too

## ASSISTANT

[Checked persistence / extra-field survival across atproto-PLC, Hive, raw AP, Mastodon, GoToSocial,
Threads; multiple web searches.]

**atproto/PLC: tolerates extra `alsoKnownAs` entries, but rejects unknown top-level fields.** The PLC
server does not cross-validate `alsoKnownAs` or `service` entries (any DID can claim any identity);
`alsoKnownAs` is an array that can have more than one entry, and atproto's parser consumes only the
first valid `at://` handle and ignores the rest — your provenance URI sits untouched. **But**
(Newbold's validation announcement): the array can't contain duplicates and no additional/unspecified
fields are allowed in operations. So you cannot invent a `provenanceAnchor` field; the link must live
inside `alsoKnownAs` (or `services`, the other uncross-validated map). The exact questions — which DID
URIs / key types / service types can be added without Bluesky rejecting the doc — "are questions only
Bluesky PBC can currently answer." Persistence is real today but governed unilaterally and could
tighten — soft guarantee, not a contract. (Also the dual-resolvability sketch has a concrete form: if
the PDS hosted the entire oplog/DID-doc history back to inception, allowing validation of the genesis
document against the CID in the did:plc URI, "this would close the delta and make did:plc a variant
expression of did:webvh, where each did:plc a PDS hosts is a valid did:webvh, but not every did:webvh
needs to be intelligible to atproto.")

**Hive: no DID and no `alsoKnownAs`, but the most open field.** Identity is an on-chain account with
owner/active/posting/memo key hierarchies. But `json_metadata` / `posting_json_metadata` (profile) and
`custom_json` (arbitrary developer data) let you write a signed attestation fully under your control
via `account_update` / a `custom_json` op. Bespoke — no other system looks there; "validation" is just
"signed by the Hive account's key." Gotcha: when modifying authority arrays, Hive requires them
alphabetically sorted or the chain rejects the fields (applies to auth arrays, not free metadata, but
indicative of on-chain serialization strictness).

**Mastodon / GoToSocial: field exists, interpreted as migration aliases.** Both support `alsoKnownAs`
but with migration semantics (Move checks the alias on the target). UIs expose it only as
account-aliases; a non-actor provenance URI coexists but isn't surfaced or honored. Inert-but-present.

**Threads: don't rely on it.** Partial, one-directional AP implementation; no evidence it preserves
custom `alsoKnownAs`, and you don't control the server. Treat it as a place you point *at* from your
anchor (outbound claim), not a place you plant a backlink — the bidirectional handshake won't close
there.

**Synthesis table:**

| System | Field | Accepts arbitrary upstream URI | Honored/validated by platform | Backlink quality |
|---|---|---|---|---|
| AP protocol | `alsoKnownAs` | Yes | N/A (just data) | Good |
| Mastodon | `alsoKnownAs` | Tolerated, UI-limited to actor aliases | Only as migration alias | Coexists, ignored |
| GoToSocial | `alsoKnownAs` | Tolerated | Only as migration alias | Coexists, ignored |
| Threads | unknown/partial | Not reliably | No | Don't rely on it |
| atproto/PLC | `alsoKnownAs` array | Yes (extra entries); no new top-level fields | First `at://` only; rest ignored, not cross-validated | Good, but governed unilaterally |
| Hive | `json_metadata` / `custom_json` | Yes, fully arbitrary | No (bespoke) | Strong as signed attestation, bespoke |

Every one lets you write a provenance pointer; not one validates it for you. The fields are carriers,
not enforcers. Trust always comes from the bidirectional backlink (each side signed by its own native
authority) or a signed attestation, checked by a verifier you build. **Two next steps** (start with
the first): the per-platform trust-model document (field used, what you claim / don't claim, backlink
mechanism, verifier steps); and a note that the cleanest future convergence is the atproto thread's
own sketch — did:plc as a did:webvh expression once the PDS hosts the full oplog — so keep the
did:webvh SCID as the anchor identity even though no platform reads it yet.

## USER

> Explain "And separately, a note that the cleanest future convergence is the one the atproto thread
> itself sketched, did:plc as a did:webvh expression once the PDS hosts the full oplog, so your design
> should keep the did:webvh SCID as the anchor identity even though no platform reads it yet." And did:PLC
> has some global mapping table that bluesky corporate hosts?

## ASSISTANT

**Yes — did:plc has a central directory Bluesky operates.** did:plc is not self-resolving like did:web;
you query a server, and that server is `plc.directory`, run by Bluesky PBC. It holds the operation log
for every did:plc identifier (12M+). Resolve via `GET https://plc.directory/{did}`; operation history
at `GET https://plc.directory/{did}/log/audit`. This is the well-known soft spot in atproto's
decentralization story, and Bluesky doesn't hide it — the spec says they're "enthusiastic about" moving
governance/registry operation out of their sole control (snapshots, mirroring, third-party auditing
considered); the move to a nonprofit hasn't happened.

**The important nuance:** the directory is a *transparency log, not an authority over your keys.* Every
operation is self-certifying, signed by your rotation keys; the did:plc identifier is a hash of your
genesis operation. Bluesky hosts the mapping and log but can't silently forge your DID document. They
could censor (refuse to serve/update) or in an adversarial scenario equivocate, but can't produce
operations your keys didn't sign. Closer to a Certificate Transparency log than a root CA — and that's
the hinge for the convergence.

**The convergence — why did:plc is "almost already" a did:webvh.** A did:plc is: a stable
self-certifying identifier (hash of genesis op); an append-only signed log of operations; resolution by
fetching and replaying that log. A did:webvh is: a stable self-certifying identifier (the SCID, hash of
genesis log entry); an append-only signed log (the `did.jsonl`); resolution by fetching and validating
the chain. Nearly the same data structure — the differences are where the log lives and whether the
resolver checks the genesis hash. The proposal: if the PDS were required to host your full did:plc
oplog and history back to the genesis operation, anyone could fetch it and validate the genesis
operation against the hash embedded in the did:plc string — exactly the did:webvh validation procedure
— making did:plc "a variant or alternate expression of did:webvh." The asymmetry: each did:plc a PDS
hosts would be a valid did:webvh, but not every did:webvh need create a did:plc or be intelligible to
atproto. One identity, two resolution paths.

**What this means for your design:** keep the did:webvh SCID as your anchor identity even though
nothing reads it today. did:plc and did:webvh are converging data structures, and the most credible
decentralization path for atproto looks like "expose the oplog in a did:webvh-validatable form." If
that lands, an identity already anchored on a did:webvh SCID slots in cleanly. If it never lands you've
lost nothing — the SCID anchor is independently useful as the provenance root. **The concrete design
rule:** don't treat your Bluesky did:plc as your root identity; treat it as a spoke, with the did:webvh
SCID as the root, and link them. **Honest caveat:** this is a comment-thread proposal a maintainer
engaged with, not a roadmap item. "Keep the SCID as anchor" is hedge-positioning — cheap insurance,
forward-compatible, not a prediction — exactly the non-foreclosing posture you've been optimizing for.

---

## End of dialogue

Filed 2026-06-22. Distilled to `thinking/cross-platform-identity-provenance.md`; extends
`thinking/plc-identity-resilience.md`. Open decisions surfaced (not resolved): the anchor-URI
stability contract; PDS-held vs self-controlled did:plc rotation key; the not-yet-written per-platform
trust-model doc. See COHESION §21 and ROADMAP_TODO.
