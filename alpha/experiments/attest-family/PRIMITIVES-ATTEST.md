# PRIMITIVES-ATTEST — the attestation family's primitive language

`Status: DRAFT (living doc, alpha side). Landed by RUN-ATTEST-01 next to the
attest-family experiment crate. Home DECIDED (V10, 2026-07-18, owner-confirmed
in chat): this document stays in alpha beside the crate. GRADUATION TRIGGER
(named condition): it graduates to the beta spec tree at the attest lane's
RELEASE PASS, defined as (a) the RUN-ATTEST-04 settlement rider landed, (b)
grades re-evaluated, (c) the lexicon drafts promoted or explicitly versioned.
No thin beta summary is ever created — one design, one document (the fold_auth
lesson).`

The model in one paragraph: there is **one attestation family** with two axes.
**Subject type**: persona, or thing (business, product, work). **Consent mode**:
`mutual` (co-signed edge, the friend case), `unilateral_notice` (subject notified,
signed reply allowed, no countersign required — the review case), `unilateral_private`
(note to self). Everything else — scope labels, supersede-never-revoke, per-viewer
resolvability, corroboration structure, provenance grades — is shared machinery. No
trust score exists anywhere. Queries return corroboration structure, viewer-relative;
clients do the weighting. All of this is a consequence of the founding razor: the
protocol computes provenance (consistent + corroborated); it never computes utility
(true/right), which is left to humans at the edges. (evidence: attest-family crate,
RUN-ATTEST-01, Modeled)

## Vocabulary

Each term: one sentence of definition, one sentence of what it is NOT.

- **attestation** — A signed, content-addressed claim by one persona about a subject
  (persona or thing), carried in the family's single envelope shape and folded as a
  fact. It is NOT a verdict, a rating, or anything the protocol itself evaluates for
  truth. (evidence: `src/types.rs` `Envelope`/`Payload`, T-AT0.1, RUN-ATTEST-01, Modeled)

- **edge** — The mutual-mode relationship that exists exactly when two halves
  co-signed by both personas reference the same canonical shared core (both persona
  ids, edge id, consent mode, ceremony facts). It is NOT either half alone, and NOT a
  claim about the relationship's quality. (evidence: T-AT1.1–T-AT1.3, RUN-ATTEST-01,
  Modeled)

- **half** — One side's signed statement of an edge: the shared core plus that side's
  side-local label. It is NOT an edge, and a lone half is never partially one —
  it folds to pending. (evidence: T-AT1.1, T-AT1.4, RUN-ATTEST-01, Modeled)

- **vouch** — A separate, later, unilateral claim by one persona about another, in a
  named scope, standing on at least one resolvable qualifying antecedent from the
  closed antecedent class (co-signed edge, transaction attestation, or ceremony
  fact — OC-2 DECIDED (V1, 2026-07-18): option B) and superseding independently of
  its antecedents. It is NOT part of any edge, and NOT valid with zero qualifying
  antecedents — it folds to pending. (evidence: T-AT2.1–T-AT2.3, T-A3.1–T-A3.4,
  RUN-ATTEST-01/03, Modeled)

- **antecedent kind** — The closed class of provenance mechanisms that can stand a
  vouch up: `co_signed_edge` (bidirectional trust bound), `transaction`, and
  `ceremony` (unidirectional bounds), closed at the compile boundary and governed
  as a quorum register on the reused R7 machinery. (Quorum note, V7:
  **governance counts member handles — one per vetted ID at the group's own
  chosen vetting level; personas sign.**) It is NOT an open vocabulary
  (no string escape hatch — adding a kind is a source change; widening the
  qualifying list is a quorum act with lineage, not a code edit), and it is NOT a
  grade — each kind merely maps to one (`edge_backed`, `transaction_backed`,
  `ceremony_backed`). (evidence: `src/types.rs` `AntecedentKind`, T-A3.2, T-A3.3,
  T-A3.4, RUN-ATTEST-03, Modeled)

- **review** — A `unilateral_notice` attestation about a subject that stands with only
  the author's signature, deterministically emits a notice fact to the subject, and
  accepts the subject's signed reply as a peer object. It is NOT subject-approved
  content — requiring countersignature is the failure mode it exists to avoid.
  (evidence: T-AT5.1–T-AT5.4, RUN-ATTEST-01, Modeled)

- **predicate** — An issuer's process-backed claim about a persona ("over_18"),
  inseparable from the issuer identity and process-provenance metadata, whose
  substrate (ID number, card number) is unrepresentable in the payload type. It is
  NOT the verified datum itself, and NOT a bare boolean detached from who asserted it
  and how. (evidence: T-AT6.1–T-AT6.3, RUN-ATTEST-01, Modeled)

- **scope** — The named context a vouch or review speaks to ("would hire as
  contractor"), matched by exact equality in queries. It is NOT a category tree, NOT
  fuzzy-matched, and adjacent scopes never bleed. (evidence: T-AT3.2, RUN-ATTEST-01,
  Modeled)

- **consent mode** — The family's second axis: `mutual`, `unilateral_notice`, or
  `unilateral_private`, fixed inside the signed payload. It is NOT a moderation
  state and NOT changeable in place (a different mode is a different, superseding
  object). `unilateral_private` is deferred from v1 (OC-4 DECIDED (V3,
  2026-07-18)): when it ships, it ships as a private-substrate artifact (an
  MLS-group-of-one) under the private tier's own logic, never as a fourth public
  consent mode — zero tests remains the deliberate statement. (evidence:
  `src/types.rs` `ConsentMode`, T-AT1.3, RUN-ATTEST-01/03, Modeled)

- **ceremony fact** — A participant's signed statement of a shared co-presence
  session, referenced from an edge core; one from each participant over the same
  session grounds the `in_person` grade. It is NOT a countersignature on the edge
  and NOT required for the edge to exist. (evidence: T-AT1.7, RUN-ATTEST-01, Modeled)

- **grade** — Provenance metadata (`in_person`, `remote`, `transaction_backed`)
  recording how an attestation's ceremony/antecedent structure was formed. It is NOT
  a score and NOT an ordering input: the type implements no comparison, and no code
  path consumes it except serialization/display. (evidence: T-AT1.7, T-AT2.4,
  compile_fail doc-tests on `types::Grade`, RUN-ATTEST-01, Modeled)

- **corroboration structure** — The query result for (viewer, subject, scope): the
  set of standing, scope-matching attestations whose attester is resolvable to that
  viewer, with grades, markers, lineage pointers, and replies, in canonical-hash
  order. It is NOT an aggregate, NOT a ranking, and contains no numeric field beyond
  date claims. (evidence: T-AT3.1–T-AT3.4, T-AT0.2, RUN-ATTEST-01, Modeled)

- **resolvability** — Whether a persona is traversable for a given viewer, governed
  exclusively by that persona's own current policy fact (supersede-lineaged, like
  everything else). It is NOT grantable by an edge holder about the far end, and an
  unresolvable attester's attestation is absent from responses, not redacted-but-
  counted. (evidence: T-AT4.1, T-AT4.2, T-AT3.3, RUN-ATTEST-01, Modeled)

- **graded resolvability default (V4)** — With no policy act ever taken, a persona
  resolves exactly to the counterparts of its own STANDING co-signed edges; every
  other viewer receives cardinality only (the mutual count is the stranger-facing
  tier), and silence IS the graded posture — zero configuration. It is NOT
  resolvable-to-all (the retired RUN-ATTEST-01 stand-in) and NOT a publication
  gate: OPEN is a deliberate, reversible per-persona policy supersede with lineage
  (the motivating case: workplace personas for reading structure); reviews assert
  experience, not relationship — a review grants no resolution, and public-tier
  discoverability of published records is untouched by this Drystone-tier
  traversal default. This default and the CONTACT.md contact-policy family are
  siblings — per-scope consent dials where silence renders pending, never assent;
  a stranger who wants more than cardinality has a lawful path, and it is the
  knock. (evidence: T-A4.1–T-A4.5, RUN-ATTEST-04, V4 DECIDED (2026-07-18,
  owner-confirmed in chat), Modeled)

### Anchor-persona vocabulary (RUN-ATTEST-02)

- **anchor persona** — A holder-controlled persona root keypair carrying a
  co-op-minted credential, one of possibly several a member holds, with no
  public object designating any of them as first, primary, or preferred. It is
  NOT an account hierarchy: no default exists, and an observer's total
  knowledge is "this persona carries the predicate; that one does not".
  (evidence: T-PA1.1, T-PA1.2, RUN-ATTEST-02, Modeled)

- **reality anchor** — The signal a `vetted_holder` credential carries: a
  vetted human stands behind this persona, at the cost of a vetting ceremony
  and a fee, while the personas' graphs stay hard-split. It is NOT a link
  between the personas: accountability attaches per persona; unity of the
  human stays private. (evidence: T-PA2.1, T-PA5.2, RUN-ATTEST-02, Modeled)

- **credential** — The anchor-persona mint unit: a single-predicate,
  issuer-signed object with a fresh per-mint nonce, standing only on a
  vetting-event antecedent and verified from its bytes and the issuer key
  alone. It is NOT a bundle — combining predicates happens only
  presentation-side, as a subset the persona chooses to show. (evidence:
  T-PA3.1, T-PA4.4, T-PA1.3, RUN-ATTEST-02, Modeled)

- **`vetted_holder`** — The reality-anchor predicate: "a vetted human stands
  behind this persona." It is NOT proof of unique personhood —
  one human may hold several anchor personas, and no operation, query, or
  derivable value answers whether two personas share a holder. (evidence:
  T-PA5.3, RUN-ATTEST-02, Modeled)

- **`sole_anchor(context)`** — REJECTED (V7, 2026-07-18, owner-confirmed in
  chat; not deferred — a recorded rejection, kept findable with its
  reasoning, the same treatment full-issuance-facts received). The rationale
  of record: only one-credential-per-ID yields real uniqueness and anything
  short of that is gameable, so uniqueness is **group-local membership vetting
  under local authority** — a co-op's quorum counts member handles,
  one per vetted ID at its own chosen vetting level, and personas are signing
  instruments — never a portable credential. Portable proof-of-personhood is
  the escalation the design refuses ("eating the spider to eat the fly"); the
  valuable, less invasive question — is one real vetted human behind this
  persona? — is `vetted_holder`, already built. It is NOT built, will NOT be,
  and it is NOT `vetted_holder` — conflating them would silently turn the
  reality anchor into a uniqueness registry. (evidence: vocabulary only, RUN-ATTEST-02)

- **commitment (V5 revision)** — The keyed per-issuance value `HMAC(key_e,
  credential_id)` under the era's confidential commitment key, existing
  publicly only as a tree leaf behind a Merkle root (and, on supersession, in
  a superseded set). It is NOT locatable or even confirmable from the
  credential without the era key (dictionary-resistant, T-A4.8), and NOT
  ordered — leaves sort canonically by commitment value, so mint adjacency
  does not exist (T-A4.7). (evidence: T-A4.6–T-A4.8, RUN-ATTEST-04, V5
  DECIDED (2026-07-18, owner-confirmed in chat), Modeled)

- **tree head (V5)** — The issuer's ENTIRE per-epoch publication: ONE signed
  head (CT/RFC-9162 shape) carrying the Merkle root over the era's keyed
  commitments, the leaf count, the superseded-set root (set published
  beside), and the governance-era anchor. It is NOT a receipt pile — nothing
  per-credential is published (T-A4.6) — and its value claim is bounded:
  publication proves consistency, holder self-verifiability, and epoch-grain
  volume; it cannot and does not prove noncoercion or that vetting truly
  happened — process honesty lives in governance and the covenant. Head
  cadence is the seed-rule shape (epoch roll OR N facts, N a rule on the
  reused R7 machinery — T-A4.9); no wall-clock input exists anywhere in the
  pipeline. (evidence: T-A4.6, T-A4.9, T-A4.13, RUN-ATTEST-04, Modeled)

- **staple (V5)** — The holder-presented confirmation replacing the status
  check: commitment + issuer binding + inclusion proof, verified against a
  published head by a pure function — the verifier NEVER contacts the co-op,
  so the (verifier, subject) capture leak is structurally gone (T-A4.11,
  T-A4.12). It is NOT a status endpoint (that machinery is deleted), and a
  superseded credential cannot staple a fresh-head proof (T-A4.10);
  freshness requirements remain app policy, fail-closed as always.
  (evidence: T-A4.10–T-A4.12, RUN-ATTEST-04, Modeled)

- **era-reissue (V5/V6 refinement)** — On a governance change, a holder MAY
  request reissue under the new era with ONLY the holder signing the request;
  the issuer's reissue supersedes the old credential, chains the ORIGINAL
  vetting event (no new vetting antecedent — also V8's free-reissue
  structural pin), and mints a fresh commitment under the new era key,
  cross-era-unlinkable in the public sweep (T-A4.15). It is NOT an
  expiry-renewal: old-era credentials are valid facts forever, re-affirmation
  is voluntary, and silence carries no penalty. Membership is thereby
  era-graded — "meaningful but factual": the protocol supplies the factual
  half (issued-under, last-reissued-under, holder-requested — era facts
  only, no standing computation, T-A4.16); meaning is human. (evidence:
  T-A4.14–T-A4.16, RUN-ATTEST-04, Modeled)

- **seam boundary** — The named type (`SeamBoundary`) holding the issuer's
  payment-bookkeeping stand-in: the ONE place where member↔anchor-count
  linkage may live, visible in the type system so it cannot silently spread.
  It is NOT serializable, NOT queryable, and holds member handles, never
  persona identifiers. (evidence: T-PA6.1, T-PA3.3, RUN-ATTEST-02, Modeled)

Supporting vocabulary (machinery, not new axes): **supersede** — the only way any
attestation is retired: a later object cites and replaces it while the prior object's
bytes remain retrievable unchanged; never revoke-in-place (T-AT0.3). **notice fact** —
the deterministic, fold-derived fact addressed to a review's subject; delivery is out
of scope (T-AT5.2). **marker** — presentation metadata (`edge_superseded`, `stale`)
that annotates and never removes, hides, or reorders (T-AT2.3, T-AT5.5); the
superseded-antecedent marker is kind-specific per V2 — only a co-signed edge has an
ended state, so the marker is unrepresentable for transaction- and ceremony-backed
vouches (T-A3.5). **withdrawal (Drystone tier)** — an author-superseded (withdrawn)
vouch or review is ABSENT from every corroboration structure — no tombstone field,
no count — while lineage retains the object's bytes (T-AT0.3, T-A3.6); the
authoritative-layer claw-back (the author removing the canonical record from their
own PDS) is the public/ATProto tier's mechanism (V2; see
ATTEST-ATPROTO-MATCHUP.md), never proactive network pull-back, and amend =
whole-record replace, with no wall-clock anywhere.
**transaction attestation** — the verified-purchase analog cited as antecedent for the
`transaction_backed` grade; a fixture fact in this run, no payment rail (§3 stand-in).

The era-anchoring frame (V6, RUN-ATTEST-04): **the co-op is literally a
Drystone group — its governance lineage is the era spine; issuer epochs, key
rotations, and register changes are era'd governance facts; its co-signed tree
heads are horizon checkpoints of its own group.** Issuer operational time is
governance time; no other clock exists. And the sibling-batching posture (V6):
**ceremony spacing is the user's informed choice — the co-op informs ("minting
sibling personas in one sitting clusters them in the PLC log and shares an
epoch") and honors either choice without friction**; the ledger itself already
publishes nothing finer than epoch grain (T-A4.6/T-A4.7), so spacing is
guidance, never a mechanism. (evidence: T-A4.7, T-A4.9, RUN-ATTEST-04, V6
DECIDED (2026-07-18, owner-confirmed in chat), Modeled)

The fee posture (V8, RUN-ATTEST-04): fees have **no protocol surface** — they
are co-op policy. The fee attaches to the **vetting event and the mint act**
(the seam and dial, T-PA3.2); **era-reissues are free**, and the freedom is
structural, not policy: a reissue requires no new vetting antecedent and never
reaches the mint seam (T-A4.14). Guidance of record: publish the fee schedule
in the co-op's own governance lineage (the same era spine); bootstrap with a
simple generous offering — third-party or light-tier vetting is honest because
process provenance names the mechanism, and re-vet + reissue upgrades later.
(evidence: T-A4.14, T-PA3.2, RUN-ATTEST-04, V8 DECIDED (2026-07-18,
owner-confirmed in chat), Modeled)

The graduated ordering rule (V9, RUN-ATTEST-04, generalizing F-PA-3): **any
monotonic, published, infrastructure-side ordering is a correlator;
infrastructure publishes at era grain or not at all.** F-PA-3 (FINDINGS.md)
discovered the rule's first instance — a per-mint lamport counter republishing
mint order through a side field — and stays untouched as the finding of
record; this is its graduation into the primitive language. Instances now
governed by it: mint lamports are epoch-coarse (F-PA-3), tree leaves sort by
commitment value (T-A4.7), heads publish at epoch grain under era anchors
(T-A4.6), and the PLC-timing practice family (F-AT-6). (evidence: F-PA-3,
F-AT-6, T-A4.6/T-A4.7, RUN-ATTEST-04, V9 DECIDED (2026-07-18, owner-confirmed
in chat))

Cross-encoder identity (the RUN-ATTEST-03 audit's owed sentence, landed here
and cross-referenced from ATTEST-ATPROTO-MATCHUP.md row 2): **the crate's
canonical dag-cbor form is the source of truth for core hashes; lexicon
records embed core content rather than crate-computed hashes, so no
cross-encoder hash equality is ever required; atproto CIDs are locators,
never joins.**

## Declared stand-ins (RUN-ATTEST-01 §3, RUN-ATTEST-02 §3)

Personas are fixture Ed25519 keypairs (no DID/atproto — §9 non-goal). The
resolvability policy surface is the in-memory folded table, with policy facts giving
it lineage. The co-op issuer is a fixture persona with an `issuer` role marker. The
transaction attestation is a fixture fact. Default resolvability with no policy fact
on record is resolvable-to-all in this crate — a stand-in default, not a decided
posture. RUN-ATTEST-02 adds: the vetting event is a fixture fact (no real vetting);
the payment bookkeeping is the `SeamBoundary` stand-in (no payment rail); the
holder↔persona linkage lives only in fixture bookkeeping; the anchor-count dial
register reuses the substrate's rule_key 0 (`add_member_threshold`) as a declared
reinterpretation, alongside T-AT6.4's rule_key 1 covenant register. The
resolvable-to-all default flagged as a stand-in in the RUN-ATTEST-01 deviation
note (that note is frozen; this pointer is the live record) is RETIRED — DECIDED
(V4, 2026-07-18, owner-confirmed in chat): the default is now the graded posture
(see **graded resolvability default** above; T-A4.1's red was captured against
the old default). RUN-ATTEST-03
adds: the qualifying-antecedent-kind register reuses rule_key 2
(`role_change_threshold`) as a declared reinterpretation (bitmask; 7 = the full V1
class), and the crate's fold only mirrors the folded value
(`AntecedentRegister`), never governs it. RUN-ATTEST-04 adds: the issuer's
confidential commitment secret is a fixture constant (`coop_secret`); the era
key derivation `key_e = KDF(coop_secret, era_anchor_e)` is a MODELED choice
flagged owner-revisitable in-code (not an OC); the genesis era anchor is a
fixture constant standing in for the governance fact that opened the co-op's
first era; and the head-cadence register reuses rule_key 0 within the co-op's
own ISSUER-OPERATIONS group (a distinct fixture GroupId — the era-spine group)
as a declared reinterpretation, mirrored via `set_cadence` (T-A4.9).

## Owner calls (RUN-ATTEST-01 series)

- **OC-1** — DECIDED (V10, 2026-07-18, owner-confirmed in chat): this document
  stays in alpha beside the crate; the graduation trigger is the NAMED
  condition in this document's header (the attest lane's release pass). No
  thin beta summary is ever created — one design, one document (the fold_auth
  lesson).
- **OC-2** — DECIDED (V1, 2026-07-18, owner-confirmed in chat): option B — a vouch
  requires a qualifying antecedent from a **closed class** (co-signed edge,
  transaction attestation, or ceremony fact), not an edge specifically. Rationale
  of record: these are shapes of one provenance mechanism; what varies is the kind
  of trust bound — bidirectional (edge) or unidirectional (transaction, ceremony) —
  and both are valid. Settled by RUN-ATTEST-03 (T-A3.1–T-A3.4).
- **OC-3** — DECIDED (V2, 2026-07-18, owner-confirmed in chat): persist-with-marker
  RATIFIED for vouches whose base edge is superseded, with the marker made
  kind-specific (`edge_superseded`) and the tier boundary clarified: claw-back
  means the author removing the record from their own PDS (the canonical copy) on
  the public/ATProto tier — never proactive network pull-back; amend =
  whole-record replace; no wall-clock anywhere. Private review/remediation is a
  different mechanism, parked. Settled by RUN-ATTEST-03 (T-A3.5, T-A3.6).
- **OC-4** — DECIDED (V3, 2026-07-18, owner-confirmed in chat): `unilateral_private`
  is deferred from v1. When it ships, it ships as a private-substrate artifact (an
  MLS-group-of-one), never as a fourth public consent mode. Zero tests remains the
  deliberate statement.

RUN-ATTEST-02 owner calls (its §8 numbering; tagged at their test sites) —
ALL DECIDED by the 2026-07-18 walk (RUN-ATTEST-04 settlement; zero pending
OWNER-CALL tags remain in the attest lane):

- **OC-1 (PA)** — DECIDED (V5, 2026-07-18, owner-confirmed in chat):
  per-epoch signed tree heads over keyed commitments replace the
  per-credential receipt pile; confirmation = holder-stapled inclusion
  proofs (the verifier never contacts the co-op); revocation = the per-epoch
  superseded set; the publication value claim re-graded (consistency +
  self-verifiability + epoch-grain volume, never noncoercion or process
  truth). Tag at T-PA1.3; settled by T-A4.6–T-A4.13.
- **OC-2 (PA)** — DECIDED (V6, 2026-07-18, owner-confirmed in chat): ledger
  posture inherited from V5 (canonical-order leaves, epoch grain, nothing
  finer); ceremony spacing is USER GUIDANCE (the co-op informs and honors
  either choice); plus the era-anchoring move — issuer operational time is
  governance time. Tag moved with T-PA1.4's successor T-A4.7; the
  epoch-membership residue stays F-PA-1's last bullet.
- **OC-3 (PA)** — DECIDED (V7, 2026-07-18, owner-confirmed in chat):
  `sole_anchor(context)` REJECTED, not deferred — see the recorded rejection
  in the vocabulary above. Tag at T-PA5.3, whose unaskable pin now stands
  unqualified.
- **OC-4 (PA)** — DECIDED (V8, 2026-07-18, owner-confirmed in chat): fee
  semantics have no protocol surface; the fee attaches to the vetting event
  and the mint act; era-reissues are free (structural pin: T-A4.14);
  schedule published in the co-op's own governance lineage; bootstrap with a
  simple generous offering. Tag at T-PA3.2.
