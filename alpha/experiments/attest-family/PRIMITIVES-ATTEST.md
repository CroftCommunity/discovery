# PRIMITIVES-ATTEST — the attestation family's primitive language

`Status: DRAFT-PENDING (living doc, alpha side). Landed by RUN-ATTEST-01 next to the
attest-family experiment crate. Where this graduates (beta spec section vs alpha living
doc) is OC-1 — an owner call, not made by this run.`

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

- **vouch** — A separate, later, unilateral claim by one edge participant about the
  other, in a named scope, citing the base edge as antecedent and superseding
  independently of it. It is NOT part of the edge, and (this run's narrow option,
  OC-2) NOT valid standing alone without a base edge. (evidence: T-AT2.1–T-AT2.3,
  RUN-ATTEST-01, Modeled)

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
  object). Whether `unilateral_private` ships in v1 is OC-4 — it is defined here and
  deliberately has zero experiments in this run. (evidence: `src/types.rs`
  `ConsentMode`, T-AT1.3, RUN-ATTEST-01, Modeled)

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

- **`sole_anchor(context)`** — A vocabulary-only, scope-bound predicate for
  contexts that genuinely require one-persona-per-human (voting, one-account
  promotions), deliberately reintroducing linkage WITHIN that context and
  nowhere else. It is NOT built (OC-3 pending), and it is NOT `vetted_holder`
  — conflating them would silently turn the reality anchor into a uniqueness
  registry. (evidence: vocabulary only, RUN-ATTEST-02)

- **commitment** — The issuer's only public per-issuance trace: the hash of a
  credential id with a fresh salt, folded per epoch as an unordered set. It is
  NOT locatable from the credential (the salt blinds it) and NOT ordered —
  within an epoch, mint adjacency does not exist. (evidence: T-PA1.3,
  T-PA1.4, RUN-ATTEST-02, Modeled)

- **status check** — The OCSP-shaped read-side solicitation: a verifier
  submits a credential hash and the issuer answers current/superseded/unknown,
  signed and deterministic, from its own assertion lineage. It is NOT a
  registry lookup and staleness of an unanswered check is NOT a verdict — an
  app requiring a fresh answer fails closed by ITS policy, never by protocol
  timeout. (evidence: T-PA6.3, T-PA6.4, T-PA2.3, RUN-ATTEST-02, Modeled)

- **seam boundary** — The named type (`SeamBoundary`) holding the issuer's
  payment-bookkeeping stand-in: the ONE place where member↔anchor-count
  linkage may live, visible in the type system so it cannot silently spread.
  It is NOT serializable, NOT queryable, and holds member handles, never
  persona identifiers. (evidence: T-PA6.1, T-PA3.3, RUN-ATTEST-02, Modeled)

Supporting vocabulary (machinery, not new axes): **supersede** — the only way any
attestation is retired: a later object cites and replaces it while the prior object's
bytes remain retrievable unchanged; never revoke-in-place (T-AT0.3). **notice fact** —
the deterministic, fold-derived fact addressed to a review's subject; delivery is out
of scope (T-AT5.2). **marker** — presentation metadata (`antecedent_superseded`,
`stale`) that annotates and never removes, hides, or reorders (T-AT2.3, T-AT5.5).
**transaction attestation** — the verified-purchase analog cited as antecedent for the
`transaction_backed` grade; a fixture fact in this run, no payment rail (§3 stand-in).

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
reinterpretation, alongside T-AT6.4's rule_key 1 covenant register.

## Owner calls surfaced (not decided here)

- **OC-1** — where this document graduates (alpha living doc vs beta spec section).
- **OC-2** — whether a vouch may stand alone without a base edge; this run implements
  edge-antecedent-required (narrowest option; `// OWNER-CALL: OC-2 pending` at
  T-AT2.1); the transaction-antecedent grade path (T-AT2.4) hints at the alternative.
- **OC-3** — long-term semantics of vouches whose base edge is superseded:
  persist-with-marker (implemented; `// OWNER-CALL: OC-3 pending` at T-AT2.3) vs a
  re-affirmation window.
- **OC-4** — whether `unilateral_private` ships in v1; defined in vocabulary, zero
  experiments here — a deliberate statement, not an oversight.

RUN-ATTEST-02 owner calls (its §8 numbering; tagged at their test sites):

- **OC-1 (PA)** — issuer public-lineage content: blinded commitments
  (implemented, narrowest; tag at T-PA1.3) vs publishing nothing vs full
  issuance facts (the recorded rejected pole).
- **OC-2 (PA)** — sibling-batching mitigation: unordered per-epoch commitment
  folds (implemented; tag at T-PA1.4) vs ceremony-spacing policy vs both. The
  epoch-membership residue is F-PA-1's last bullet.
- **OC-3 (PA)** — whether `sole_anchor(context)` ever ships, and which
  contexts justify deliberate intra-context linkage (tag at T-PA5.3; NOT
  built).
- **OC-4 (PA)** — fee semantics: flat per-anchor vs vetting-tier pricing;
  pure policy over the T-PA3.2 dial, no protocol impact (tag at T-PA3.2).
