# HIST-ATPROTO-MATCHUP — a PDS repo as the history store's durable dataset backend

`Class: read-and-report abilities brief (RUN-HIST-01 Part A). Own lane (HIST),
renumberable to mainline later; attest-lane precedent
(ATTEST-ATPROTO-MATCHUP.md, RUN-ATTEST-03 Part B). No spec, register, crate, or
frozen record is edited by this file. It maps; it decides nothing. All five HS
OC tags (§6) are surfaced and left pending for the owner walk.`

`Serves: ../../beta/impl/drystone-design/history-durability.md — §E mirror-group
transport, §G reconciliation envelope, §I/§J convergence, §L pruning and
checkpoints — and ../../beta/impl/drystone-design/rbsr-construction.md (req. 3
stateless responder, req. 5 omission resistance). Scope: whether and how an
ATProto PDS repo can serve as the durable dataset backend of a history store S,
with a thin fetch-and-caching layer in front (the MLS terminator plus cache).
Does NOT re-prove RUN-14 service-auth (cited by reference), does NOT pin wire
encodings ([gates-release] untouched), does NOT decide any HS OC.`

`Form: what does the history-store design need from a durable backend → here is
ATProto's current version of it → gap → which tier the ability lands in (the
public repo tier, or the thin layer in front of it). Every claim about current
ATProto state carries a primary-source anchor fetched in-session on 2026-07-20
(§5). Quotes are verbatim from those anchors. Status flags per the A.9 ladder:`
`Established` `(anchored this session),` `[confirm]` `(load-bearing, not
verifiable this session — the exact open question written out),` `Design` `(this
lane's own binding proposal),` `Synthesis` `(assembled across sources).`

---

## 1. The frame this brief assumes (given, not open)

The history store S (history-durability.md §D–§J) is a content-blind helper: it
holds pairs of (cleartext-to-G-hist reconciliation envelope, G-sealed content
blob), serves range reconciliation over (subspace, counter) with `entry_digest`
fingerprints, and proves nothing — backfill admission at member endpoints is
standing plus contiguity (Proofs Phase 2.5 A2.3). This brief maps the option of
swapping S's local disk for an ATProto PDS repo. Carried in as given:

- **Posture/path split.** Posture (is the envelope published on a public repo)
  is a per-scope consent dial. Path (how a member fetches) is a client-suited
  delivery preference. Both paths are offered; neither is a capability
  fallback (RUN-19: MLS-in-WASM over WebTransport — no wasm/native asymmetry).

- **Three-rung fetch menu**, all serving identical sealed bytes, differing only
  in metadata exposure: (1) G-hist over iroh, meer-level, member IP never
  touches the PDS; (2) service-auth XRPC to the thin layer — the RUN-14 EXP-A
  mechanism, cited by reference, not re-proven here; (3) raw public sync
  (getRepo / CAR), the audit rung, public-posture groups only.

- **The ordering rider (written once, referenced from rows 2 and 9).** Repo
  commit order, firehose sequence, and rkey sort are delivery artifacts;
  folding from any of them is a MUST-NOT; convergence ordering is the
  in-payload predecessor chain (`predecessor_digest`), only ever. This is the
  delivery-cursor rule of GROUPS.md v2 (A.7: "any sequence numbers a role
  assigns are delivery cursors, never order") applied to the three orderings a
  PDS backend introduces. The spike's B4 assertion renders it as a test.
  `Design.`

- **Helper with capability, never standing.** The scribe (whoever signs the
  repo) holds a delegated capability. Removal is supersession — PLC rotation
  plus successor re-hydration from CAR — never revoke-in-place (row 8).

## 2. The required-abilities inventory

Each row: what the design requires (with its §-pointer) → what ATProto
currently provides (anchored) → the gap, if any → which tier it lands in
(**public repo tier** = the PDS repo itself; **thin-layer tier** = the
fetch-and-caching service in front, the MLS terminator).

### Row 1 — Content-addressed blob storage with reference-based retention → **Native, with one publicity fact stated plainly**

**Need.** `entry_digest` addresses the sealed blob (history-durability.md §G:
"`entry_digest`: the content digest of this entry, and the address of its
sealed blob").

**ATProto.** Blobs are CID-addressed — "blobs are universally content addressed
(by CID)" — but retention is reference-based: "After a successful upload, blobs
are placed in temporary storage… Servers should 'garbage collect' (delete)
un-referenced temporary blobs after an appropriate time span" (blob spec,
§5-3), with "at least one hour a firm lower bound" of grace time before
collection (blob spec, §5-3). And deletion follows the last reference: "When a
record referencing blobs is deleted, the server checks if any other current
records from the same repository reference the blob. If not, the blob is
deleted along with the record" (blob spec, §5-3). Re-upload of an
already-referenced blob is a no-op: "Re-uploading a blob which has already been
stored and referenced results in no change to the existing blobs or records"
(blob spec, §5-3). `Established.`

**The load-bearing publicity fact of the whole lane, stated without
softening.** If sealed blobs live on the PDS, a referencing record is
mandatory — an unreferenced blob is garbage-collected — **so some index record
is public repo content.** There is no PDS-resident-blob configuration in which
nothing about the dataset is publishable repo state. What that index record
minimally reveals is the envelope surface of §G (hashed subspace, digests,
counter, padded size) or, at minimum (row 11 option b), blob count, sizes, and
append rhythm. The posture dial (§1) chooses how much; it cannot choose none
while blobs are PDS-resident. `Synthesis` (blob-lifecycle anchor ∧ §G).

**Tier.** Public repo tier (the blob store and its mandatory referencing
records). The GC grace window and the re-upload no-op are the live-leg
observations (Part B, attended-optional).

### Row 2 — Sorted index over (subspace, counter) → **Native: the MST is the reconciliation index**

**Need.** §J range reconciliation over the envelope axes: "S reconciles over
the (subspace, counter) space with `entry_digest` fingerprints"
(history-durability.md §J).

**ATProto.** Repo records live at `collection/rkey` paths in a Merkle Search
Tree: "the repository MST is a key/value mapping where the keys are non-empty
byte arrays, and the values are CID links to records", and "The sort order is
from **left** (lexically first) to **right** (lexically latter)" (repository
spec, §5-1). Record keys allow "alphanumeric (`A-Za-z0-9`), period, dash,
underscore, colon, or tilde (`.-_:~`)" and "must have at least 1 and at most
512 characters", with the practice guidance "try to keep key paths to under 80
characters" (record-key spec, §5-2). `Established.`

**The construction.** An rkey of the form `<hashed-subspace-prefix>-<zero-
padded counter>` makes the repo tree itself the sorted (subspace, counter)
index: lexicographic rkey order equals the (subspace, counter) total order
exactly when the counter field is fixed-width. **The padding width is a byte
choice** — `[gates-release]`-adjacent, deferred with the other wire pins. The
spike picks a spike-local width (20 decimal digits, the u64 ceiling) and says
so in its doc-comments; B2 property-tests the order equivalence including the
unpadded failure mode at the 9→10 boundary. `Design;` width `[gates-release]`-
adjacent. The ordering rider (§1) applies: rkey sort is a delivery artifact,
never fold order. (evidence: B2 `rkey_order.rs`, RUN-HIST-01, Modeled)

**Tier.** Public repo tier (the index is repo structure). Note the row-1
consequence: in the full-envelope posture these index records ARE the public
envelope; sealed-posture alternatives are row 11.

### Row 3 — Cursored, stateless range reads → **Native shape; explicit range bounds are a named gap**

**Need.** rbsr-construction req. 3: a responder that "SHOULD be stateless on
the responder side, because a history store serves many members and holds no
session" (rbsr-construction.md §A.3).

**ATProto.** `com.atproto.repo.listRecords` is an unauthenticated query taking
`repo`, `collection`, `limit` (1–100, default 50), `cursor`, and `reverse`
("Flag to reverse the order of the returned records") — verified against the
current lexicon JSON, not assumed (listRecords lexicon, §5-4). Cursor
conventions are the standard XRPC shape: "If a cursor is included in a
response, the next batch of responses can be fetched by including that value in
a follow-on, continuing until the cursor is not included any longer" (xrpc
spec, §5-11). `Established.`

**Gap, named.** The current lexicon carries **no `rkeyStart`/`rkeyEnd`-style
bound parameters** — earlier parameter names of that shape do not exist in the
fetched lexicon, so bounded range reads over the public rung realize as cursor
iteration with a client-side stop at the end bound (wasteful when the span is a
small slice of a large collection), and true bounded-span serving lands on the
**thin-layer tier** (a lexicon of this lane's own, or server-side filtering in
the thin layer). The mock responder in Part B (B3) therefore implements the
listRecords *shape* — cursored pages, no session state — and the member-side
consumer does its own bounding. `Established` (parameter absence, against the
fetched lexicon); `Design` (thin-layer close).

**Tier.** Public repo tier for cursor-paged reads; thin-layer tier for bounded
spans and for anything the sealed posture withholds from the repo (row 11).

### Row 4 — Whole-dataset export and re-hydration → **Native (CAR + migration machinery)**

**Need.** Store replacement and audit: §D's helper-removal posture and this
lane's supersession story (scribe removal = rotate, re-hydrate, continue —
row 8).

**ATProto.** "Full repository exports can be fetched from the account's PDS
host using the `com.atproto.sync.getRepo` XRPC endpoint (HTTP GET)"; "This
endpoint is not authenticated, and returns all repo records, MST nodes, and the
current signed commit object, all in a single CAR file" (sync spec, §5-5). The
repository spec's CAR language carries the import caution the design must
honor: "When importing CAR files, note that there may existing dangling CID
references" and "care should be taken when injecting CAR contents directly in
to backend block storage, to ensure resources are not wasted on un-referenced
blocks" (repository spec, §5-1). The migration machinery is the supersession
rehearsal in miniature: "A CAR file can be imported to the new PDS using the
authenticated `com.atproto.repo.importRepo` endpoint"; "Blobs (media files) are
download and re-uploaded one by one"; "Larger repo exports may fail on import
and give unexpected errors" (account-migration guide, §5-9). `Established.`

**Gap.** None structural. The design-side duty the caution imposes: a
re-hydration MUST treat the CAR as untrusted input and rebuild the envelope
index by verification, naming any missing block rather than silently accepting
an incomplete store — B5's assertion. (evidence: B5 `car_rehydrate.rs`,
RUN-HIST-01, Modeled)

**Tier.** Public repo tier (export); thin-layer tier (re-hydration and index
rebuild).

### Row 5 — Authenticated service reads → **Proven in-house; cited, not re-proven**

**Need.** Rung 2 of the fetch menu (§1): a member fetches over authenticated
XRPC from the thin layer, without touching iroh and without the public rung's
exposure.

**ATProto.** Service-auth JWTs verified against the DID document key. Evidence:
**RUN-14 EXP-A** — `verify_service_jwt` with real ECDSA against the issuer's
`#atproto` DID-document key, viewer-gated serving, and the live leg confirmed
with real credentials (evidence: RUN-14 EXP-A, `Verified` at its stated
bounds). This row deliberately carries **no new anchor and no re-prove**; the
mechanism is already in-house at a higher grade than a fetched quote would
supply.

**Tier.** Thin-layer tier (the thin layer is the verifying service).

### Row 6 — Public unauthenticated sync rung → **Native by design**

**Need.** The audit path: rung 3 of the fetch menu, and the substrate on which
omission detection (B6) runs — any party in a public-posture group can pull the
full dataset without the thin layer's cooperation.

**ATProto.** The `com.atproto.sync.*` endpoints are unauthenticated by design
(the getRepo quote in row 4; sync spec, §5-5). The mirror posture is
compliance, not cryptography: "It is important for such mirrors to respect
repository updates (eg, record deletion) and account status changes (eg,
account deactivation or deletion) in a timely manner (within seconds or
minutes)", and "static repository snapshots should not be redistributed
publicly in bulk form (eg, archival datasets or torrent files)" (sync spec,
§5-5) — the same timing language ATTEST-ATPROTO-MATCHUP §4-4 already carries.
`Established.`

**Why this rung matters to *this* design specifically.** rbsr-construction
req. 5 wants omission resistance at the content-blind-store seam. The public
sync rung adds a structural backstop the local-disk backend never had: a
responder that withholds an entry can be caught by any auditor replaying the
public CAR against the predecessor chain — the member-side detection (B6) plus
an unauthenticated export path makes omission *visible*, not merely resisted.
`Synthesis` (req. 5 ∧ sync-spec unauthenticated export). (evidence: B6
`omission.rs`, RUN-HIST-01, Modeled)

**Tier.** Public repo tier. Public-posture groups only (§1 posture dial).

### Row 7 — Deletion without tombstone, bounded by mirror visibility → **Native; the hist-specific bound stated plainly**

**Need.** §L pruning below a checkpoint: the store drops index records and
blobs for the pruned prefix.

**ATProto.** "Record deletion is supported without leaving a trace or
'tombstone' of previous contents" (repository spec, §5-1); the firehose emits
the delete as an event ("`action` (string, required)… One of: 'create',
'update', or 'delete'" — sync spec, §5-5); mirrors retain within their policy,
bounded by the compliance-not-cryptography posture quoted in row 6.
`Established.` The general delete-visibility bounding is
ATTEST-ATPROTO-MATCHUP row 4's paragraph, reused here by pointer: no-residue
holds at the authoritative layer and in compliant views; it is NOT network
amnesia.

**The hist-specific bound, no hedging, no overclaim.** What a non-compliant
mirror retains from this lane's repo is **ciphertext plus envelope** — G-sealed
blobs the mirror cannot decrypt, and §G envelope fields (hashed subspace ids,
digests, counters, padded sizes). So the harm of retained residue is
**metadata-shaped, not content-shaped**: chain existence, lengths, append
rhythm, padded sizes — the §G leak profile, frozen at prune time, held by
whoever mirrored before the prune. That is a real, permanent metadata exposure
and the design owns it; it is not a content exposure, and no phrasing here
should promise the metadata ever becomes retractable. Pruning on this backend
is a forward-exclusion operation exactly as §L's removal-is-not-redaction
paragraph already requires. `Synthesis` (§G leak profile ∧ sync-spec mirror
posture).

**Tier.** Public repo tier (deletion mechanics); the §L checkpoint *gate* is
design machinery (B7 tests the gate; the checkpoint construction itself stays
open per §L — see the lexicon sketch, §3).

### Row 8 — Signing-key rotation as store supersession → **Native (PLC); the enumerability cost filed as F-HIST-1**

**Need.** Scribe removal = supersession: rotate the identity, re-hydrate a
successor from CAR, continue — never revoke-in-place (§1; history-durability
§D helpers-with-capability).

**ATProto.** did:plc separates control from signing: "Control over a `did:plc`
identity rests in a set of reconfigurable rotation keys pairs"; "Keys are
listed in the `rotationKeys` field of operations in order of descending
authority"; "Best practice is to maintain separation between rotation keys and
atproto signing keys"; and "The PLC server provides a 72hr window during which
a higher authority rotation key can 'rewrite' history" (PLC spec, §5-8). So
scribe supersession is native shape: rotation-key holders (HS OC-3) sign an
operation replacing the repo signing key and PDS location, and the successor
re-hydrates from the CAR export (row 4). `Established.`

**The cost, extended from F-AT-6.** The PLC log is a public, permanent,
enumerable correlator surface: "The full history of DID operations and updates,
including timestamps, is permanently publicly accessible"; "the set of all
identifiers is enumerable"; including "the full history of handle updates and
PDS locations (URLs) over time" (PLC spec, §5-8). F-AT-6 (attest lane) already
files the persona-side correlators. The **hist-specific extension is new**:
under the per-group-DID option of HS OC-1, **creating a per-group DID announces
the group's existence to a public, enumerable directory** — group creation
time, rotation events (scribe supersessions!), and PDS hosting choice all
become permanently public facts about the *group*, not any persona. Filed as
**F-HIST-1** in this lane's FINDINGS.md
(`hist-atproto-spike/FINDINGS.md`), not by editing the attest lane's file.
`Established` (the log properties); the correlator analysis `Synthesis`.

**Tier.** Public repo tier (identity layer). Custody of rotation keys is
HS OC-3, surfaced in §6, not decided here.

### Row 9 — Firehose as cache-freshness signal → **Native; the cursor discipline sentence carried**

**Need.** The thin layer stays warm: it learns "the repo changed" without
polling, so cached envelope indexes and blob fetches stay fresh.

**ATProto.** `com.atproto.sync.subscribeRepos` is the repository event stream
(sync spec, §5-5). Its sequencing is explicitly a delivery mechanism:
"Sequence numbers are always positive integers (non-zero), and increase
monotonically, but otherwise have flexible semantics"; "The scope for sequence
numbers is the combination of service provider (hostname) and endpoint
(NSID)"; on reconnect, a "`cursor` is in roll-back window: server sends any
persisted messages with greater-or-equal `seq` number", and a too-old cursor
yields an info message "then starts at the oldest available `seq`"; the
backfill window's "intent… is to ensure reliable delivery of events following
disruptions during a reasonable time window (eg, hours or days)" (event-stream
spec, §5-7). `Established.`

**The cursor discipline sentence (MUST carry, per the ordering rider, §1).**
Firehose `seq` is a delivery cursor, never order, never folded: the thin layer
MAY use it to resume consumption and to invalidate caches, and MUST NOT use it
— or commit order, or rkey sort — as convergence ordering, which is the
in-payload `predecessor_digest` chain only ever. A too-old cursor is a cache
staleness event (re-list, re-hydrate), never a history gap: gaps are named by
bounding digests (§I), not by sequence arithmetic. `Design.` (evidence: B4
`fold_order.rs`, RUN-HIST-01, Modeled)

**Tier.** Thin-layer tier (the consumer); public repo tier emits it.

### Row 10 — Size and rate bounds → **Protocol is mostly silent; operational limits are real and operator-documented**

**Need.** §G: `size_hint` is "the padded byte length of the sealed blob…
Padded so true payload length does not leak"; envelope records are small;
blobs are bounded by chunking (§L's open granularity item).

**ATProto, protocol layer.** The repository and data-model specs state **no
record-size limit** at the protocol level ("Best practices for validating and
limiting the size and structure of generic atproto data are described in a
Data Validation guide" — data-model spec, §5-10); blob constraints are
lexicon-declared per record type ("`maxSize` (integer, optional): maximum size
in bytes" — lexicon spec, §5-6) plus operator policy ("Servers may have their
own generic limits and policies for blobs… maximum blob sizes; content
policies; etc." — blob spec, §5-3). `Established` (as absence/delegation).

**ATProto, operational layer.** The Bluesky-hosted-PDS values are operator
documentation, not protocol constants, and are marked accordingly: blob upload
limit "52,428,800 bytes (50 MByte)"; content writes rate-limited on a points
system, "5,000 points per hour" and "35,000 points per day" with CREATE 3 /
UPDATE 2 / DELETE 1 points (≈1,666 creates/hour); general requests "3000 per
5 minutes" per IP (docs.bsky.app rate-limits, §5-12). `[confirm:` these are
current Bluesky-operated-infrastructure values from operator docs; a
self-hosted or third-party PDS sets its own; re-check against the deployment
target before any capacity plan rests on them.`]`

**Consequence for the design.** Envelope records (§G's five small fields plus
a blob ref) sit far below any plausible record limit; the binding constraints
are blob size (drives §L's open chunking choice — a >50 MB sealed entry does
not fit the hosted-PDS default) and write rate (bounds append rhythm per
scribe; also a correlator — the padded-size and rhythm leak of §G exists at
the PDS operator too). `Synthesis.`

**Tier.** Public repo tier (limits live at the PDS); thin-layer tier absorbs
burst-shaping if append rhythm must be smoothed.

### Row 11 — Sealed-posture backend shape → **Option space laid out; HS OC-5, no verdict**

**Need.** A sealed-membership group (GROUPS.md v2) may still want durable
storage without a public envelope. Row 1's publicity fact forces a real choice
here; this row lays out the space and decides nothing.

- **(a) Forbid the PDS backend for sealed posture.** Sealed groups use the
  local-disk store S unchanged; the PDS backend is a public/gated-posture
  feature. Leak accepted: none new. Cost: sealed groups forgo the durability
  and audit rungs entirely — the availability story stays §J's.

- **(b) Blobs on PDS with minimal opaque-rkey reference records; envelope
  index kept inside the thin layer only.** The repo shows records whose rkeys
  are opaque (no subspace prefix, no counter) and whose content is only a blob
  ref. Leak accepted, stated plainly: **blob count, padded blob sizes, and
  append rhythm are still public** (row 1 — the referencing record is
  mandatory), plus repo-level commit timing; chain structure, subspace
  cardinality, and counters are not. The MST-as-index ability (row 2) is lost;
  reconciliation index lives wholly in the thin layer.

- **(c) Envelope fields sealed under the G-hist key and stored as opaque
  record content.** The record body is ciphertext to everyone but G-hist
  members (the store included — it holds the G-hist key, so the thin layer
  can still index). Leak accepted: as (b) — count, sizes, rhythm — with the
  envelope recoverable by G-hist members from the repo itself (self-contained
  re-hydration), unlike (b) where the index is thin-layer state. MST-as-index
  is lost the same way (opaque rkeys).

Shared residual for (b) and (c): the PLC/hosting correlators of row 8 and
F-HIST-1 apply to the repo's existence regardless of record opacity.
**Tagged `HS OC-5`; surfaced, not decided.** `Design` (option space).

## 3. Draft lexicons (non-normative sketches)

DRAFT, non-normative, attest-lane rules applied: closed sets would use `enum`
never `knownValues` ("a closed set of allowed values" vs "Values are not
limited to this set" — lexicon spec, §5-6); core content is embedded, not
hash-referenced; the canonical encoding of record bodies is not pinned here
(`[gates-release]` untouched). JSON files live beside the spike at
`hist-atproto-spike/lexicons/`; the shapes:

**`ing.croft.hist.entry`** — the §G envelope as a record, plus the blob ref
(row 1: the referencing record is mandatory).

| field | type | §G source |
|---|---|---|
| `subspace` | bytes (32 max) | hashed device-subspace id (hash-not-expose, per Willow e2e guidance already adopted in §G) |
| `predecessor` | bytes (32 max) | `predecessor_digest`, or the genesis/checkpoint marker |
| `entryDigest` | bytes (32 max) | `entry_digest` — the reconciliation identity; its relation to the blob CID is **HS OC-2**, so both are carried in the sketch rather than fused |
| `counter` | integer (≥ 0) | the per-subspace logical counter; also rendered fixed-width in the rkey (row 2) — carried in-body so the record, not the key encoding, is authoritative |
| `sizeHint` | integer (≥ 1) | padded byte length of the sealed blob |
| `blob` | blob (`$type: blob`, raw CID) | the G-sealed content blob |

**`ing.croft.hist.checkpoint`** — a marker record naming a corroborated
pruned-prefix commitment. **Shape only: §L's checkpoint construction is open,
and this sketch does not close it** — the `commitment` bytes are a placeholder
for whatever the §L construction pins, and the B7 guard tests the *gate*
(prune only above a present, verifiable checkpoint), never the construction.

| field | type | note |
|---|---|---|
| `subspace` | bytes (32 max) | which chain is pruned |
| `belowCounter` | integer (≥ 1) | prune boundary: entries with counter < this are prunable once this record verifies |
| `prefixHead` | bytes (32 max) | digest of the last pruned entry — the anchor reverse validation terminates at (§L) |
| `commitment` | bytes | the corroborated commitment to the pruned prefix — construction OPEN (§L), placeholder shape |

**Fields with NO lexicon home, listed mechanically** (the atproto_map
discipline, T-A3.8 precedent): the Willow `path` (inside the sealed content,
§G exclusion); wall-clock time (presentation content inside the sealed
payload, §K); Meadowcap capabilities / AuthorisationTokens (inside the sealed
payload, verified at member endpoints, §J); the raw (unhashed) `subspace_id`
(§G exclusion); the G content key and every MLS secret (asset-keying.md;
never serializable to this tier); the member frontier map (§I — session-shaped
exchange state, not a record); the thin layer's cache state (thin-layer tier).

## 4. What Part B asserts (pointer)

The mechanical-seam spike (`hist-atproto-spike/`, this run's Part B) lands
B1–B7 red-first: envelope↔record lossless round-trip (B1), rkey order property
including the padding boundary (B2), chain-gap detection over cursored pages
(B3), fold-from-repo-order rejection — the ordering rider as a test (B4), CAR
re-hydration with named incompleteness (B5), omission detection under a
withholding responder (B6), and the checkpoint prune gate (B7). Live legs
(uploadBlob GC window, re-upload no-op) are attended-optional and skip with a
named reason absent credentials. Evidence rows live in
`beta/drystone-spec/EVIDENCE-MAP.md` (HIST band).

## 5. Anchors (all fetched in-session, 2026-07-20)

| ref | source | anchoring |
|---|---|---|
| §5-1 | https://atproto.com/specs/repository | MST key/value mapping, byte-array keys, lexical sort; "Record deletion is supported without leaving a trace or 'tombstone' of previous contents"; CAR v1 export format; import cautions (dangling CID references; un-referenced blocks) |
| §5-2 | https://atproto.com/specs/record-key | allowed characters `A-Za-z0-9` + `.-_:~`; 1–512 chars; ≤80-char practice; TIDs: loose temporal ordering, "should not trust TID timestamps" |
| §5-3 | https://atproto.com/specs/blob | CID-addressed blobs; temporary storage + GC of un-referenced blobs, ≥1 hour grace floor; last-reference deletion; re-upload no-op; operator size/policy limits |
| §5-4 | https://raw.githubusercontent.com/bluesky-social/atproto/main/lexicons/com/atproto/repo/listRecords.json | current parameters exactly: `repo`, `collection`, `limit` (1–100, default 50), `cursor`, `reverse`; **no rkey-bound parameters** |
| §5-5 | https://atproto.com/specs/sync | getRepo unauthenticated full-CAR export; firehose `#commit` create/update/delete; mirrors "must respect repository updates (eg, record deletion)… in a timely manner (within seconds or minutes)"; no-bulk-snapshot guidance |
| §5-6 | https://atproto.com/specs/lexicon | `enum` "a closed set of allowed values" vs `knownValues` open; `bytes` `maxLength`; blob `accept`/`maxSize`; invalid-data posture |
| §5-7 | https://atproto.com/specs/event-stream | `seq` positive, monotonic, "otherwise have flexible semantics"; scope = hostname + NSID; cursor roll-back window and too-old behavior; backfill window "hours or days" |
| §5-8 | https://web.plc.directory/spec/v0.1/did-plc | rotation keys distinct from signing keys, descending authority, 72 h rewrite window; op log "permanently publicly accessible" incl. timestamps; identifier set enumerable; PDS-location history visible |
| §5-9 | https://atproto.com/guides/account-migration | getRepo → importRepo CAR flow; blobs re-uploaded one by one; large-import caution; PLC op signed by old PDS |
| §5-10 | https://atproto.com/specs/data-model | blessed CID: CIDv1, dag-cbor 0x71 / raw 0x55, sha-256; blob node `{$type, ref, mimeType, size}`; no protocol-level record size limit (delegated to validation guide) |
| §5-11 | https://atproto.com/specs/xrpc | cursor pagination convention; 429/Retry-After; inter-service JWT section exists (row 5 rests on RUN-14, not on this anchor) |
| §5-12 | https://docs.bsky.app/docs/advanced-guides/rate-limits | hosted-PDS operational values (blob 50 MB; write points 5,000/h, 35,000/day; 3,000 req/5 min per IP) — operator documentation, `[confirm]`-graded in row 10 |

In-house precedent (repo-local, not re-fetched): RUN-14-SUMMARY.md (EXP-A
service auth, row 5); ATTEST-ATPROTO-MATCHUP.md (rows 4/9 pointers, anchor
discipline); Proofs Phase 2.5 A2.3 (`lineage-history::backfill_import` —
standing plus contiguity, reused by B6); GROUPS.md v2 A.7 (delivery-cursor
rule); RUN-19-SUMMARY.md (no wasm/native asymmetry, §1).

## 6. Owner-call register (surfaced, tagged, NOT decided — the I9-firewall posture)

- **HS OC-1 — Repo ownership.** Service DID (one repo per thin-layer service,
  groups as collections) vs per-group DID (the communal namespace rendered
  literally — one repo IS one group's dataset). The per-group option pays the
  F-HIST-1 enumerability cost (row 8): group existence, creation time, scribe
  rotations, and hosting become permanent public directory facts. `OWNER-CALL: pending`.
- **HS OC-2 — Reconciliation identity.** `entry_digest` ≡ blob CID (one
  identity, but the digest is then coupled to the blessed CID multicodec/hash
  choice — sha-256/raw today, §5-10) vs a separate digest carried in the
  record (decoupled from CID choices, but two identities to keep consistent).
  Either way the coupling is to a hash choice that is `[gates-release]`
  territory. `OWNER-CALL: pending`.
- **HS OC-3 — Scribe key custody.** Who holds the repo signing key
  (the thin layer? a member device? k-of-n?) and who holds the PLC rotation
  keys that can supersede the scribe (row 8). The 72 h rewrite window (§5-8)
  makes rotation-key custody a real recovery-authority question — I9-adjacent.
  `OWNER-CALL: pending`.
- **HS OC-4 — Envelope posture default.** Public envelope opt-in vs opt-out,
  per scope (§1's posture dial): which way the default points when a group
  says nothing. `OWNER-CALL: pending`.
- **HS OC-5 — Sealed-posture backend shape.** Row 11's option space (forbid /
  opaque-reference / sealed-envelope-in-record), each with its named leak.
  `OWNER-CALL: pending`.
