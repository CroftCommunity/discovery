# ATTEST-ATPROTO-MATCHUP — the attestation family vs ATProto's current abilities

`Class: read-and-report abilities brief (RUN-ATTEST-03 Part B). No spec, register,
crate, or frozen record is edited by this file. It maps; it decides nothing. OC-1's
doc-home question stays open; this brief does not graduate anything.`

`Form: what does this design need from a substrate → here is ATProto's current
version of it → gap → where the gap closes. Every claim about current ATProto state
carries a primary-source anchor fetched in-session on 2026-07-18 (§4). Quotes are
verbatim from those anchors.`

`Reads: attest-family crate (RUN-ATTEST-01/02/03); PRIMITIVES-ATTEST.md;
FINDINGS.md; the anchors in §4; appview-infra (RUN-14) and the Stellin EXP-A
viewer-gate as in-house precedent.`

---

## 1. The two-tier frame this brief assumes

The attestation family's model (PRIMITIVES-ATTEST.md) is substrate-agnostic:
signed, content-addressed claims, folded deterministically, supersede-never-revoke,
corroboration structure instead of scores. The intended deployment is two-tier:

- **Public/ATProto tier** — author-published attestation records under stable
  identity, aggregated by AppViews. This is where V2's claw-back lives: the author
  removing the canonical copy from their own PDS.
- **Drystone/private tier** — everything the public tier must not hold:
  resolvability policy enforcement, ceremony session privates, issuer retained
  state (the seam), and the deferred `unilateral_private` artifact (V3).

The brief's job is to say, row by row, which of the design's required abilities
ATProto supplies today, which it supplies in shape but not in guarantee, and which
stay on the Drystone tier until the protocol moves.

## 2. The required-abilities inventory

### Row 1 — Author-signed durable records under stable identity → **Native**

**Need.** Every attestation is a signed object by one persona; the envelope author
is load-bearing (T-AT6.2: a predicate cannot be detached from who asserted it).

**ATProto.** Repo records signed under an account DID: records live at
`<collection>/<record-key>` paths in a Merkle Search Tree whose root is a signed
commit; "the repository MST is a key/value mapping where the keys are non-empty
byte arrays, and the values are CID links to records" (repository spec, §4-1).
Identity is one of two blessed DID methods — "did:plc: a self-authenticating DID
method developed specifically for use with atproto" and did:web; "the intention is
to keep the 'blessed' set as small as possible" (DID spec, §4-3).

**Gap.** None structural. The crate's Ed25519 fixture personas are the declared
stand-in for exactly this (RUN-ATTEST-01 §3); the envelope's author key becomes
the account's signing key from the DID document.

### Row 2 — Content addressing for cross-reference → **Native, with one honest analysis**

**Need.** Antecedent citation by content address (the R7 shape): vouches cite
transaction/ceremony facts, halves join on the shared core.

**ATProto.** CIDs over DAG-CBOR with SHA-256: "the blessed CID format described in
Data Model is used for references to commit objects, MST node objects, and
records" (repository spec, §4-1). Same machinery family as the crate's
BLAKE3-over-dag-cbor object ids — a codec/hash swap, not a model change.

**The honest analysis the co-signed edge requires.** A *true* bidirectional CID
cross-reference is impossible: if half A's bytes contained half B's CID and half
B's bytes contained half A's CID, each hash would depend on the other —
circularity. So the edge realizes on ATProto as: **half A published first; half B
citing A's CID; and BOTH halves carrying the shared core hash** (both persona
ids, edge nonce, consent mode, ceremony refs — `EdgeCore`). The **core-hash
equality is the real join** — exactly T-AT1.2's rule, where an edge exists iff two
halves co-signed by both personas reference the same canonical core. The CID
citation from B to A is one-directional *convenience* (a locator, an antecedent
hint for indexers), never the identity of the edge. This is the difference between
the crate's model and a naive "two records point at each other": the naive version
cannot exist, and anything that half-implements it (only B cites A) silently makes
the edge asymmetric. The crate's fold never traverses a CID to establish the edge;
it compares core hashes. An ATProto realization must keep that rule.

### Row 3 — Closed vocabularies → **Native**

**Need.** Closed enums everywhere a value could smuggle substrate or open the
antecedent class (T-AT6.1, T-A3.2): predicate kinds, methods, consent modes,
`AntecedentKind`.

**ATProto.** Lexicon is "a schema definition language used to describe atproto
records, HTTP endpoints (XRPC), and event stream messages" (lexicon spec, §4-2).
One precision matters: lexicon string `knownValues` are OPEN — "a set of suggested
or common values for this field. Values are not limited to this set" — while
`enum` is CLOSED — "a closed set of allowed values" (lexicon spec, §4-2). The
draft lexicons in `attest-family/lexicons/` therefore use `enum`, never
`knownValues`, for every closed set; using `knownValues` would silently reopen
the T-AT6.1/T-A3.2 compile boundary at the schema layer. Validation posture
matches the crate's ("Protocol implementations should generally consider data
which fails to validate against the Lexicon to be entirely invalid").

### Row 4 — Author-sovereign delete/amend → **Native, with V2's claim honestly bounded**

**Need.** V2's claw-back: the author removes the canonical copy of a withdrawn
attestation from their own PDS; amend = whole-record replace. Never proactive
network pull-back.

**ATProto.** Record delete and same-rkey replace are native: "Record deletion is
supported without leaving a trace or 'tombstone' of previous contents"
(repository spec, §4-1) — the exact no-residue shape T-A3.6 proves at the
Drystone tier — and updating a record-key path yields a new signed commit
(same-rkey replace = amend).

**The bounding paragraph (no hedging, no overclaim).** Deletion removes the
authoritative copy, and the repository itself retains no trace. But the firehose
emits the delete as an event — "#commit events — individual record operations
(create, update, delete)" (sync spec, §4-4) — and any subscriber or AppView that
already ingested the record physically retains what it took. The protocol's
posture is compliance, not cryptography: mirrors "must respect repository updates
(eg, record deletion) and account status changes... in a timely manner (within
seconds or minutes)," and "static repository snapshots should not be
redistributed publicly in bulk form" (sync spec, §4-4). So V2's no-residue claim
holds **at the authoritative layer and in compliant views**; it is **not network
amnesia**. A non-compliant archiver keeps the bytes, and nothing in the protocol
prevents that. "No active trace" is the accurate phrasing; "gone from the
network" would be an overclaim, and the design must never promise it.

### Row 5 — Aggregation / corroboration queries → **Native pattern**

**Need.** The one query in the model: (viewer, subject, scope) → corroboration
structure, viewer-relative, with resolvability filtering (T-AT3.3: absent, not
redacted-but-counted).

**ATProto.** The AppView pattern is exactly this: services consume the firehose
("a repository event stream ('firehose') provides real-time updates about changes
to repository state" — sync spec, §4-4) and serve indexed views over XRPC
("HTTP APIs for client-server and server-server requests in atproto use a set of
common conventions called 'Lexicon RPC', or XRPC for short" — xrpc spec, §4-8).
Viewer-relative filtering requires an **authed** AppView — the viewer's identity
must be verified before the resolvability filter can be applied. The in-house
precedent already exists: RUN-14's Stellin EXP-A serves a viewer-gated profile
view behind real atproto **service auth** (`verify_service_jwt` against the
DID-doc key, then the `app.stellin.getProfileView` viewer-gate), and the
appview-infra kit (RUN-14/D-series) is the hosting substrate for exactly such a
service. Gap: none in kind; the corroboration AppView is an application to build,
not a protocol ability to wait for.

### Row 6 — Subject notice (`unilateral_notice`) → **Gap named; product-layer close**

**Need.** T-AT5.2: folding a review deterministically emits a notice fact
addressed to the subject; delivery is out of scope in the crate but the mode's
name promises notice.

**ATProto.** No native notification primitive exists at the protocol layer — the
event stream carries repo/identity/account events, not user-addressed notices
(sync spec, §4-4; the Spring 2026 roadmap discusses no notice primitive either,
§4-6). Notifications in the ecosystem are **derived by consumers**: an AppView
watches the firehose, sees a record referencing a subject, and materializes a
notification in its own index.

**Where the gap closes.** Product layer: the corroboration AppView (row 5)
derives notice facts from folded reviews — which is precisely the crate's shape
(the notice fact is fold-derived, deterministic, delivery-external). The gap is
named, not blocking: the design never required the substrate to deliver notice,
only to make notice derivable, which the firehose does.

### Row 7 — Issuer predicates + commitment lineage → **Native-shaped**

**Need.** RUN-ATTEST-02's issuer model: credential and per-epoch commitment
records are public-safe by design (blinded commitments, no persona identifiers);
the status check is a read-side solicitation, not a registry.

**ATProto.** The issuer is an account like any other: it publishes credential
records and per-epoch `commitmentEpoch` records in its own repo (rows 1–3
machinery). The status check maps to an **XRPC endpoint, not records** — a
query ("query (GET, cacheable)" — xrpc spec, §4-8) on the issuer's service
answering current/superseded/unknown from its own assertion lineage, signed,
exactly the OCSP shape T-PA6.3 proves. Publishing status as records would
invert the design (a registry); the XRPC mapping preserves "solicited read"
semantics. Gap: none structural — this row is native-shaped end to end.

### Row 8 — Persona resolvability / scoped disclosure → **NOT native; the two-tier boundary as a table cell**

**Need.** T-AT4.1: a persona's own policy governs who may resolve it; an
unresolvable attester's attestation is absent from responses.

**ATProto.** Repos are public today. The Spring 2026 roadmap (2026-03-24) is
explicit that public sync is the finished part ("broadly complete, with multiple
implementations deployed and interoperating in the network") and that
**permissioned data** is the direction: "shipping Permissioned Data will require
updates to PDS implementations, SDKs, written specifications, moderation tooling,
and more," expected to be "a major focus for the Bluesky protocol team through
the summer" (roadmap, §4-6). The community Private Data WG carries the parallel
track; the current Bluesky sketch is "namespaces" gated "to the user's client
(via auth scopes); and to other users (by DID)," with "nothing... encrypted at
rest or E2EE, only in transit" (WG notes, §4-7).

**Where the gap is held.** The Drystone tier holds resolvability until then: the
public tier publishes only what is resolvable-to-all; per-viewer disclosure is
served by the authed AppView (row 5) from Drystone-tier policy state, and
policy-restricted attestations simply do not reach the public repo. This row IS
the two-tier boundary stated as a table cell — when ATProto's permissioned data
ships, some of this migrates; nothing in the design has to change shape for it.

### Row 9 — Persona anchors / sybil floor → **Anchor credential supplies the floor — and the PLC-correlator analysis (F-AT-6)**

**Need.** RUN-ATTEST-02's reality anchor: a vetting ceremony and fee behind each
anchor persona, sibling personas unlinkable in public objects (T-PA2.1).

**ATProto.** DIDs are free to create — nothing in the DID spec (§4-3) imposes
cost or friction — so **the DID is not the sybil floor; the anchor credential
is**. That part carries over unchanged. What does NOT carry over silently is
sibling unlinkability, because `did:plc` has a public operations log:

**The PLC-correlator analysis.** "The full history of DID operations and
updates, including timestamps, is permanently publicly accessible," including
"the full history of handle updates and PDS locations (URLs) over time," and
"the set of all identifiers is enumerable" (PLC spec, §4-5). So: creation
timestamps correlate (sibling personas created the same day cluster), rotation
operations correlate (same-day key rotations across "unrelated" DIDs), and PDS
hosting choice is visible (siblings on the same small PDS share an
infrastructure fingerprint). These correlators live in infrastructure the design
does not control — the T-PA2.1 floor (no shared identifier in published
*attestation* objects) survives, but the identity layer underneath it leaks
metadata the fixture-keypair model never had. Filed as **F-AT-6** in FINDINGS.md
(residual correlators, ATProto edition), sibling to F-AT-1/F-PA-1; mitigations
named there (distinct PDS hosts, staggered creation, the epoch-coarseness
practice from F-PA-3) — recorded, not solved.

### Row 10 — No-scalar constraint survival → **Discipline the substrate permits but does not enforce**

**Need.** T-AT0.2/T-AT3.1: no numeric trust/score/rating field exists anywhere,
compile-boundary enforced; corroboration is structure, never aggregate.

**ATProto.** Nothing in ATProto forces an aggregate — but nothing prevents one
either. The constraint therefore lives in two places the design controls: (a)
**the lexicons** — no score field exists in any `ing.croft.attest.*` schema
(the T-AT0.2 invariant expressed as schema absence; integers appear only inside
date claims), and lexicon validation makes a conforming record unable to carry
one ("data which fails to validate against the Lexicon [is] entirely invalid" —
lexicon spec, §4-2); and (b) **AppView behavior** — the corroboration endpoint
returns structure, and the T-AT5.4-style allowlist discipline must follow the
serving code. Named honestly: any third-party AppView can compute scores over
public attestation records, and the protocol will not stop it. What the design
guarantees is that no *first-party surface* emits a scalar and no *record
format* can carry one — the same boundary the crate proves, relocated.

## 3. Verdict summary

| # | Ability | Verdict |
|---|---|---|
| 1 | Signed durable records / stable identity | Native (repo + DID) |
| 2 | Content addressing / cross-reference | Native — core-hash equality is the join; CID cite is one-directional convenience |
| 3 | Closed vocabularies | Native (lexicon `enum`, never `knownValues`) |
| 4 | Author-sovereign delete/amend | Native — no-residue holds at authoritative layer + compliant views; NOT network amnesia |
| 5 | Corroboration queries | Native pattern (authed AppView; RUN-14 precedent) |
| 6 | Subject notice | Gap named; product-layer close (fold-derived from firehose) |
| 7 | Issuer predicates + commitment lineage; status via XRPC | Native-shaped |
| 8 | Resolvability / scoped disclosure | NOT native; Drystone tier holds it; permissioned-data direction on roadmap + WG |
| 9 | Persona anchors / sybil floor | Credential supplies the floor; PLC correlators filed as F-AT-6 |
| 10 | No-scalar survival | Discipline: schema absence + AppView behavior; substrate permits, does not enforce |

## 4. Anchors (all fetched in-session, 2026-07-18)

| ref | source | anchoring |
|---|---|---|
| §4-1 | https://atproto.com/specs/repository | records/MST/CID; "Record deletion is supported without leaving a trace or 'tombstone' of previous contents"; same-rkey replace commits |
| §4-2 | https://atproto.com/specs/lexicon | `knownValues` open ("Values are not limited to this set") vs `enum` "a closed set of allowed values"; invalid-data posture |
| §4-3 | https://atproto.com/specs/did | blessed methods did:plc/did:web; minimal blessed set |
| §4-4 | https://atproto.com/specs/sync | firehose event types incl. delete ops; mirrors "must respect repository updates (eg, record deletion)... in a timely manner (within seconds or minutes)"; no-bulk-snapshot guidance |
| §4-5 | https://web.plc.directory/spec/v0.1/did-plc | "full history of DID operations and updates, including timestamps, is permanently publicly accessible"; PDS-location history; enumerable identifier set |
| §4-6 | https://atproto.com/blog/2026-spring-roadmap (2026-03-24) | permissioned-data status: sketch design published; "a major focus... through the summer"; public sync "broadly complete" |
| §4-7 | https://atproto.wiki/en/working-groups/private-data + https://notes.commonscomputer.com/s/atproto-private-data-wg | Private Data WG scope; "namespaces" sketch, access "gated... to other users (by DID)", "nothing is encrypted at rest" |
| §4-8 | https://atproto.com/specs/xrpc | XRPC query/procedure conventions; lexicon-defined endpoints |

In-house precedent (repo-local, not re-fetched): RUN-14-SUMMARY.md (Stellin EXP-A
service-auth viewer-gate; appview-infra kit).

## 5. Appendix pointer — draft lexicons (non-normative)

`attest-family/lexicons/` holds DRAFT JSON sketches for
`ing.croft.attest.edgeHalf`, `.vouch`, `.review`, `.reviewReply`, `.credential`,
`.commitmentEpoch` — schema-mirrors of the crate's canonical payloads, including
the closed `AntecedentKind` from V1 (as lexicon `enum`) and **no numeric score
field anywhere** (the T-AT0.2 invariant as schema absence). Marked DRAFT,
non-normative; the crate's canonical dag-cbor remains the source of truth. The
fields that deliberately have NO lexicon home (resolvability policy, ceremony
session privates, seam-typed issuer state) are enumerated mechanically by
T-A3.8 (`atproto_map`), each with the tier that holds it.
