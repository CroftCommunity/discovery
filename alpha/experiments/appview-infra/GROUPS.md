# The group tier model — canonical model text (v2)

`Corpus deliverable. Design material, stays in discovery; excluded from the
appview-infra extraction (D15). Rewritten 2026-07-18 (RUN-16) around the
canonical model in Section A. This v2 supersedes and extends the two-tier
framing of the RUN-15 D11 brief (below) — one lineage, one envelope, one
delivery plane, one catalogue; the tiers become policy values on a scope. It
reverses no landed decision: the RUN-15 Variant A/B write-path analysis is
preserved intact as its own section, and the owner decisions it asked are
restated (unchanged, plus one new) in A.10. Section A is the canonical text;
the spec-facing implications are staged, not applied, in
beta/drystone-spec/proposed-changes-2026-07-experiment-reconciliation.md
(RUN-16 update) with a reviews-log row — the reviewed spec (part-*, conventions)
is untouched.`

Companion: [PUBLICATIONS.md](PUBLICATIONS.md) — the publications positioning (what vanilla
atproto proves natively, the degeneration principle, the subscriber reframe; RUN-18).

---

# SECTION A — THE MODEL (canonical text)

## A.0 One sentence

Groups are one lineage, one envelope, one delivery plane, and one catalogue; the tiers are
policy values on a scope, and the only thing that changes between a 2-person sealed circle and
a 100k open forum is which proofs the scope pays for and which transports its traffic rides.

## A.1 Principals and names

A group's permanent identifier is its genesis hash: the hash of its canonical genesis record.
First hash sticks. Display names are mutable metadata changed by governed rename operations
over the immutable principal. Names (and roster entries) MAY be blinded: hashed with a salt
held as sealed state inside the steward group (A.6), so outsiders cannot correlate which
lineage is which community. Blinding consequences, stated up front: members do not hold the
salt and instead receive individual attestations of their own commitment (provable
own-membership without enumerable co-members); salt rotation is a steward epoch plus O(members)
republished commitments, a deliberate act; a leaked salt degrades unlinkability only, never
membership integrity, which is always carried by signatures. Blinding opposes discoverability
and is a per-scope dial, not a default.

## A.2 Two policy axes, not one

Every scope carries two independent policy fields in its genesis (amendable by governance):

- **Membership policy**: `open` | `gated` | `sealed` — who may join and how joining is proven.

- **Write policy**: `open` | `members` | `named-set` | `single` — who may author into the scope.

The familiar shapes are coordinates: open forum (open/open), newsletter (open/single),
announcement channel (gated/named-set), working group (gated/members), sealed circle
(sealed/members). The catalogue displays both fields; the same gate enforces both, at serve
time and at relay time (A.8).

**Reception completeness for write-restricted scopes.** In `single` and `named-set`
scopes, authors MUST chain their envelopes: each envelope carries the
author's previous envelope in its antecedents. Any reader holding envelope N can then
verify the complete stream N-1..1 exists, detect any gap as a known omission, and retrieve
it via interval backfill from any role or peer — a subscriber's completeness is verifiable
from public data alone, and open enrollment never weakens it (verification requires
no standing, only the envelopes). The honest limit, per the completeness-ahead doctrine: a
withheld TAIL is undetectable until anything newer arrives by any path; multimodal delivery
(DS plus optional swarm) is the mitigation, and freshness/solicitation posture remains a
governed dial, never a mechanism that closes the limit. Delivery is best-effort;
DETECTION of incompleteness up to the newest held envelope is the guarantee.

## A.3 Membership facts (gated tier; the open and sealed tiers are its degenerate and augmented forms)

Membership is a two-sided public fact, split across repos by who owns the act, in atproto's
agent-centric grain (every record is a speech act of its author; "I granted X membership" is
the steward's own history):

- **Request/claim**: the joiner writes a request record to their own PDS. It travels the relay
  like any record.

- **Grant**: stewards write grant records to their own PDSes, co-signed per the scope's
  ratified admission rule (R7 shape: the approval subject is the hash of the canonical
  payload; approvals are antecedents; the claim's hash is among the grant's antecedents).
  Roster inclusion requires both halves: the grant prevents unilateral self-admission; the
  claim prevents conscription (nobody appears on a roster without a record they authored).

- **Leave**: self-sovereign; the member deletes or supersedes their claim; no steward signs
  anything. Proof is authenticated absence in the member's signed repo tree.

- **Revocation**: steward-side only (nothing enforceable may depend on the sanctioned party's
  repo); a governed record with a causal cut point in its antecedents. Enforcement is
  when-seen; the causal gap between a compromise and a propagated revocation is a named,
  permanent ambiguity, not a bug.

- **Silence is not a verdict.** A pending request stays pending mechanically, forever; decline
  records are optional UX courtesy; decay is presentation, never expiry.

- **Who is a steward** is itself the same kind of fact (role grants/revocations), verified
  recursively at each op's causal position, grounded at genesis: prior rule governs.

- **Ordering** of all of the above is by antecedents and the fold, never by claimed
  timestamps.

- **Membership intervals.** A member's history in a scope is the set of causal intervals
  between their effective grant (or self-registration) and their leave or revocation cut, per
  interval. Intervals are derivable by anyone from the public fold and are the unit of
  history access (A.7, history convergence).

**Archive habit**: a service folding these facts archives the ops as canonical state
(kilobytes per group). While author repos live, the archive is redundant corroboration; if a
steward PDS dies, it is the surviving copy, and because every op is co-signed it remains
verifiable rather than trusted. The service never becomes the authority even when it becomes
the last librarian. Anyone can walk the chain from genesis, discover the steward set at each
position, fetch those repos, and rebuild the entire governance history.

## A.4 The open tier: one signature, zero canonical bytes

`membership = open` means the request IS the grant: membership is the presence of the joiner's
own signed registration record; leaving is deleting it. The joiner's repo signature chain
proves self-registration with no secondary confirmation; the record is simultaneously consent,
proof, and roster entry. Consequences: nobody can register you (only your repo produces the
record) and a catalogue cannot invent members that survive audit (re-folding the firehose
exposes fake rows). Everything in this tier lives in repos, so a service's catalogue, rosters,
and content indexes are pure projections: rebuildable by anyone, classed disposable, never
backed up, and identical across any two honest services folding the same firehose. Honest
posture: open means nobody can admit you AND nobody can expel you; there is no authority in
the scope; abuse controls are display-layer only (labelers, mutes, catalogue curation policy,
an optional moderation reference in the genesis). The self-signature proves the ACCOUNT acted,
not the person; compromise remedies live in the identity layer.

## A.5 The assertion layer: what survives of MLS at each tier

One envelope shape at every tier: `{scope, causal antecedents, payload, signature}`, with the
signature over scope id + antecedents + payload (MLS's context-binding discipline kept
everywhere; a message cannot be replayed into another scope or position).

**Message identity is the hash of the canonical envelope.** Canonical dag-cbor encoding is
load-bearing precisely so every node computes identical bytes, and therefore identical hashes,
regardless of arrival path. The hash is author-bound with no extra step because the signature
is inside the hashed bytes. Deduplication across delivery modes is `seen(H(envelope))` in
every tier; the sealed tier's ciphertext-byte comparison was always this same rule applied to
that tier's canonical wire artifact. Positional races are never resolved by comparison in any
tier; they are the fold's job (antecedents and contradiction detection).

- **Sealed tier**: real MLS (RFC 9420). Authorship and membership are fused: per-epoch
  ratchets encrypt, the key-schedule membership MAC proves current membership, forward secrecy
  and PCS hold. Application messages ride within an epoch; only commits roll epochs.

- **Backplane (gated/open) tiers**: no key schedule, therefore no membership MAC, therefore
  the fused check SPLITS into two independent ones: the signature (verified against the
  author's DID-document key) proves authorship; a roster lookup at the message's causal
  position proves membership. Membership becomes universally verifiable public computation.
  Named losses, accepted for these tiers: no PCS (a stolen signing key stays valid-looking
  until a revocation fact lands; the response is governance plus identity-layer rotation, not
  cryptographic healing) and the causal-gap ambiguity above.

**Keys and identity (the Bluesky/atproto integration).** Key AUTHORITY lives in the
DID document (PLC or did:web); the PDS repo holds signed ATTESTATION records delegating per-device
messaging keys to the DID (deletion is revocation and rides the relay, so verifier caches
invalidate on the same firehose they already tail). Direct reuse of the repo signing key as an
MLS leaf key is possible only for P-256 accounts (MLS ciphersuites exclude secp256k1) and is
poor hygiene regardless; delegation is the pattern. In MLS terms the leaf carries a
BasicCredential whose identity is the DID string; validation policy (resolve DID doc, check
attestation, admit) is external to the ciphersuite. Welcome-package growth collapses in the
backplane tiers because there is no ratchet tree: "welcome" is a roster head, a catalogue
entry, and (A.8) optional swarm rendezvous, constant size at any N. Light clients verify
membership via log-N inclusion proofs against a steward-co-signed roster head (the horizon
checkpoint shape); full clients sync the roster itself (~100 bytes/member).

## A.6 Governance: the small real MLS group, and the true ceiling

Every gated or sealed scope is governed by a steward group: a REAL MLS group, small (order
7–50), where epochs, welcomes, forward secrecy, and confidential deliberation all genuinely
work. Deliberation is sealed; VERDICTS (grants, revocations, role and policy changes, rename
ops) are published facts, plain or blinded per the scope's dial: private reasoning, public
rulings. The steward group also custodies the blinding salt (A.1). Scale analysis: mechanical
costs are trivial (a 100k-member scope at 1% daily churn is ~1,000 governance ops/day ≈
0.01/s against a node measured at 357 ev/s with 64x headroom; op storage is hundreds of
bytes; rebuild-by-walking is O(stewards)). The TRUE ceiling is steward attention: per-join
human co-signing drowns at scale, so scale is bought with governed admission policy — rules
the group ratifies once ("any member may vouch", "auto-grant on criteria X; stewards handle
appeals") that make routine admissions mechanical while keeping the rule, and every
exception, human-governed.

## A.7 The delivery plane: roles realized as separate processes, none of them authorities

The delivery plane is a SET OF ROLES, each realized as its OWN PROCESS, conceivably co-hosted
on one machine but never fused into one primitive. Every role is content-blind for sealed
scopes, optional in both the scale-down and scale-up directions, and simpler to operate on a
real stable public endpoint without ever requiring one. None holds ordering or membership
authority; concurrent commits are detected contradictions for human adjudication, and any
sequence numbers a role assigns are delivery cursors, never order (the covert-clock and
no-arbiter rules; this is the deliberate break from conventional MLS deployments, which
quietly grant their DS ordering authority).

- **The web-native Delivery Service (the AppView-facing role).** A known public stable
  endpoint serving the backplane tiers to ordinary web clients: browsers connect DIRECTLY
  over QUIC via WebTransport/HTTP-3 (WebSocket fallback), no overlay, no bridge. Roster-gated
  offering; readable payloads in backplane scopes under the stated trusted-gatekeeper
  posture; search, moderation, and helpers native there. For sealed scopes this role, where
  run, offers ciphertext only (blindness at a compile/dependency boundary).

- **The swarm peer (sealed-tier presence).** A separate process joining the iroh overlay as
  an ordinary peer: holds ciphertext, is always reachable, gives every sealed group a stable
  member-adjacent presence without authority. It exists because no member node is expected to
  have a static public IP; the overlay and its relays are what make that tolerable.

- **The history convergence node.** A separate process that converges envelope sets across
  transports (set reconciliation over envelope hashes; the RBSR machinery) and serves
  BACKFILL SCOPED BY MEMBERSHIP INTERVAL: a requester who proves membership receives history
  from any causal point forward at which the proof holds, i.e. their intervals per A.3, and
  nothing earlier. In backplane scopes the proof is the public fold; in sealed scopes the
  node holds and serves ciphertext only, and interval scoping governs offering (what
  decryption the requester can perform remains bounded by the keys they hold; forward secrecy
  is not overridden by delivery). Interval-forward is the default access rule; widening it is
  a per-scope governance dial, not an operator choice.

- **Helpers** as previously defined: content helpers are members by grant; distribution
  helpers (including the DID-doc key cache, invalidating on firehose identity events) touch
  ciphertext and public data only. In backplane scopes the web-native DS is simply the
  standing content helper.

**Replaceability invariant, restated for roles**: because no role holds ordering or
membership authority, any operator holding the same grants can run any role for the same
scopes; roles can run on one host, several hosts, or (scale-down) not at all, with pure
peer-to-peer operation remaining valid for sealed scopes. The stable endpoint is a
convenience concentrator, not a dependency.

**Sealed-scope history honesty** (unchanged and now attached to the convergence role):
forward secrecy makes stored ciphertext age out of decryptability; delivery-side history
serves offline catch-up within members' key retention, never archival memory or new-device
recovery (recovery is the I9 problem). Corollary for state taxonomy: index rows a content
helper derives from sealed scopes are OBSERVATION-BORN, not projections; class them canonical
(a small sidecar in state.db) or knowingly accept them as losable in the scope's posture
language.

## A.8 Transports: the split, the swarm, and one planned crossing

**The split.** The iroh overlay and its relays are loaded ONLY by sealed scopes and steward
governance. Backplane scopes ride the plain web stack end to end. Consequences, intended:
the tier with the most users uses the boring transport and browsers are first-class clients
with zero overlay machinery; relay load scales with sealed-group count, not user count;
failure isolation holds in both directions (a relay outage degrades sealed scopes only; a DS
outage degrades backplane serving while swarm-capable native clients continue and sealed
scopes barely notice).

**Swarm delivery for backplane scopes remains an option for native clients.** Because any
member may read any message in a cleartext scope, per-group gossip is valid: the topic is
derived deterministically, `topic = H(genesis || "delivery")`, salted-preimage for blinded
scopes so outsiders cannot locate the traffic; rendezvous (topic id, bootstrap peers, the DS
as the always-on peer) rides the constant-size welcome; participation is an opt-in DELIVERY
PREFERENCE distinct from membership (browsers live DS-only; desktops may gossip). The one
survival rule for an open topic: VALIDATE BEFORE RELAY — peers verify each envelope
(signature vs DID doc; roster and write policy at the causal position) before re-gossiping
and drop invalid envelopes unpropagated, so every peer is a filter and spam dies one hop from
source. Dedup across DS and swarm is the A.5 envelope hash; the convergence role stitches the
sets. For write-restricted scopes the second path also carries the A.2 reception guarantee's
mitigation: any envelope arriving by swarm converts a withheld tail from silent to detected
(the reader's chain check sees the newer envelope and names everything missing behind it).

**Steward governance is a ship in the night with one planned crossing.** Deliberation stays
sealed on the overlay (small, private, IP-shielded by relays). Verdicts become public by the
stewards' clients writing grant/revocation records to their PDSes over ordinary web
infrastructure. The two planes touch at exactly one point, the published fact.

**Honest costs, named**: native clients carry two transport stacks (overlay + web), browsers
one; swarm participation exposes member IPs to each other within a group's own expectations
(one sentence in the posture language for blinded scopes: hidden roster, mutually visible
mesh); and the sealed tier's BROWSER story is deliberately deferred (WASM MLS plus a relay
bridge; native apps are the sane vessel for that tier for now).

## A.9 The tier table

| | Open / broadcast | Gated (backplane) | Sealed |
|---|---|---|---|
| Membership proof | own signature (1 sig) | own claim + steward quorum | quorum + key custody |
| Authorship proof | signature vs DID doc | signature vs DID doc | ratchet + signature (fused) |
| Content | cleartext | cleartext, signed | E2EE |
| Membership visibility | public | public or salt-blinded | private |
| Canonical server state | none | roster/op archive (+ Variant A scope key) | ciphertext store, blind |
| Removal semantics | self only; no expulsion | 403-when-seen, causal cut | cryptographic, next epoch |
| Transport | web-direct (QUIC/WebTransport); swarm optional for native clients | same | iroh overlay + relays; DS/convergence roles optional |
| History access | public (reception completeness per A.2 in write-restricted scopes) | membership-interval backfill (same A.2 guarantee where writes are restricted) | interval-scoped offering; decryption bounded by held keys |
| Practical ceiling | delivery economics | delivery economics | low hundreds (churn + no-arbiter adjudication) |

Tier transitions are re-plants under an unchanged lineage (same genesis, stewards, roster
fold); crossing changes which machinery is switched on, and MUST surface a plain-language
statement where the meaning of past acts changes (chiefly: what "removed" means).

## A.10 Owner decisions preserved (unchanged by this model)

1. Large-tier write path: Variant A (repo-canonical ciphertext, server-held scope key) vs
   Variant B (server-canonical content). 2. The `group_scale_boundary` number — reframed: the
   sealed-tier ceiling is MEASURABLE (churn simulation over the croft-group loopback harness)
   and should be measured before ratifying a number. 3. Launch order — softened: the open
   tier is a zero-decision on-ramp. 4. New, from A.7: whether to self-host an iroh relay
   (a service-manifest question for the hosting kit) or use public relays initially.

---

# SECTION B — THE RUN-15 D11 WRITE-PATH FORK ANALYSIS (preserved intact)

`Preserved verbatim in substance from the RUN-15 D11 brief. This is the write-path fork the
model's A.10(1) restates and the trusted-gatekeeper posture A.9's "Canonical server state"
column carries. Nothing here is reversed by the v2 model above; the two-tier framing it opens
with is the part v2 supersedes (the tiers are now the two policy axes of A.2, and the
"large tier" is the gated/backplane tier serving readable payloads under the trusted-gatekeeper
posture). D12 built only the fork-agnostic serving behind a storage trait.`

## B.1 The tier: private, roster-gated, honestly not confidential

Below a scale boundary, groups stay **MLS-sealed** (the croft-group crates) with
the AppView as a content-blind distributor — the §H hybrid topology, whose serve
half RUN-14 EXP-B demonstrated (offer ciphertext only to a verified roster
member; the server holds no key). Confidentiality there rests on encryption.

Past that boundary the design takes a different, deliberately honest stance.
**At large scale, cryptographic group confidentiality is a mirage.** The
member-leak equivalence argument: in a group of thousands, confidentiality
against outsiders requires that *no* member ever leaks plaintext — but any one
member can screenshot, copy, or re-publish, and the probability that some member
does approaches one as membership grows. End-to-end encryption at that scale
protects against the network operator while giving members a false belief in
confidentiality *from each other's onward sharing*, which it never provided.

So large groups serve **private but not E2EE**: the AppView reads content and
gates *offering* it by verified roster membership. The posture is stated plainly
— **roster-gated, private, trusted-gatekeeper, NOT cryptographically
confidential**. The AppView operator is a trusted gatekeeper for this tier, and
saying so is the feature: search, helpers, and moderation become native and
honest instead of bolted onto a confidentiality claim the scale already broke.

### The boundary is a parameter, not this run's decision

The crossover is a manifest/policy parameter **`group_scale_boundary`**, working
number **5000** members. It is **deferred to the owner** — 5000 is a placeholder,
not a finding. Below it: MLS-sealed, content-blind AppView. At or above it:
roster-gated AppView serving per this brief. A group can be born large or cross
the boundary; the migration story is part of the fork analysis below.

## B.2 The write-path fork (OWNER DECISION — guardrail 5)

The one thing the owner must decide: **where large-group content canonically
lives.** Both variants serve the same way (roster-gated reads); they differ in
who holds the canonical bytes. D12 builds behind a `GroupStore` trait so the
serving path is identical either way.

### Variant A — repo-canonical ciphertext, AppView decrypts to serve

Authors publish ciphertext records to **their own repos**, sealed to a
server-held **group scope key**. The AppView holds that scope key + the roster,
decrypts to build its index, and serves roster-gated.

- **State taxonomy (§1.2).** Server-canonical = **scope key + roster only**
  (small, in `state.db`, backed up by Litestream). Content stays
  **repo-canonical** (the authors' PDSes). The AppView index is **disposable**
  — rebuildable by re-reading repos + re-decrypting.
- **Backup cost.** Minimal. Canonical state is a key and a member list; kilobytes.
- **Restore-drill shape.** Restore `state.db` (key + roster), then rebuild the
  disposable index from repos. Content is never at risk on the AppView because
  the AppView never held the canonical copy.
- **Moderation / helpers.** The AppView decrypts, so search/moderation/helpers
  are native — same as Variant B. Removal from the roster stops future offering;
  already-distributed plaintext cannot be recalled (honest, same as B).
- **Migration.** To move to Variant B, ingest repo content into server-canonical
  storage and stop relying on repos. Reversible-ish while repos retain history.
- **§H posture.** Closest to §H: content-blindness is *given up on purpose* for
  this tier (the AppView decrypts), but canonical custody still rests with
  authors. The trusted-gatekeeper acceptance is scoped to *serving*, not custody.

### Variant B — members write to the AppView API, server-canonical content

Members write content **directly to the AppView**; the server holds the
canonical bytes (`blobs/` + `state.db`).

- **State taxonomy (§1.2).** Server-canonical = roster + grants + **content**
  (`state.db` + `blobs/`, all Litestream/rclone-backed). Index stays disposable.
- **Backup cost.** Heaviest — every message and blob is canonical server state
  that must be backed up. This is the tier that stresses the R2 free tier first.
- **Restore-drill shape.** The full D13/P2-6 drill matters most here: canonical
  content lives only on the box + R2, so a restore must bring back `state.db`
  **and** the blob mirror, then rebuild the disposable index.
- **Moderation / helpers.** Native and simplest — content is right there, no
  repo round-trip, no decrypt step.
- **Migration.** To move to Variant A, authors would have to re-home content into
  their repos — the **weakest portability story**; content born server-canonical
  has no author-side copy by construction.
- **§H posture.** Furthest from §H: the AppView is the canonical home. Simplest
  immediate build; the honest cost is backup weight and portability.

### Scoring summary

| Dimension            | Variant A (repo-canonical)     | Variant B (server-canonical)   |
|----------------------|--------------------------------|--------------------------------|
| Canonical on server  | key + roster (tiny)            | roster + content (heavy)       |
| Backup cost          | minimal                        | heaviest (blobs + content)     |
| Restore-drill stakes | low (content safe in repos)    | high (only copy is box + R2)   |
| Moderation/helpers   | native (after decrypt)         | native (simplest)              |
| Migration path       | A→B straightforward            | B→A weak (no author copy)      |
| Portability          | strong (repo-canonical)        | weak                           |
| Immediate build      | more moving parts (scope key)  | simplest                       |
| §H alignment         | closer (custody stays authors) | further (server is home)       |

**No recommendation is forced here** — it is a values call (portability &
custody vs immediacy & simplicity). The kit is fork-agnostic; D12 builds the
shared serving so either can be chosen without rework.

## B.3 Spec-status: taking the trusted-gatekeeper arm, this tier only

RUN-14 left social-mapping's **AppView-provisioned scope key** item open
(`Design; open` — how the audience scope key is provisioned/granted/rotated;
stop rule 5b). This brief deliberately takes the **trusted-gatekeeper arm** of
that open item — **for the large tier only**: past `group_scale_boundary`, the
AppView is a trusted gatekeeper that reads and serves, and the scope key (Variant
A) or direct custody (Variant B) is provisioned to that end. Below the boundary,
the content-blind §H stance is unchanged.

This is a **proposed reconciliation note, staged**, not a spec edit. It is
appended to `beta/drystone-spec/proposed-changes-2026-07-experiment-reconciliation.md`
(RUN-15 section; extended by the RUN-16 update) with a row in
`beta/impl/experiments/drystone-reviews-and-experiments-log.md`, in the same
commit. The reviewed spec files (`part-*`, conventions) are **untouched**
(guardrail 2).

## B.4 What D12 builds (fork-agnostic)

Behind a `GroupStore` trait — `roster(group)` and `content(group, cursor)` —
both variants can implement. The stub implements a fixture store. Roster and
grants are classed **canonical** in the manifests (D3 asserts; D5 enforces a
backup target exists); the search/moderation index is **disposable**. The
offering-vs-reading distinction (RUN-14 EXP-B) does **not** apply to this tier:
the server reads by design, and the honest posture is the feature.

## B.5 Decision request → now A.10

The three RUN-15 D11 owner questions (write-path variant; the
`group_scale_boundary` number; croft-groups launch order before or with
stellin-appview) are **restated, unchanged, in A.10**, which adds a fourth from
A.7 (self-host an iroh relay vs public relays initially). The owner answers; the
run does not.
