# ENGAGE-LEX — engaging the Lexicon Community attestation work

`Engagement brief, 2026-07-19. Companion to PRIMITIVES-ATTEST.md and
ATTEST-ATPROTO-MATCHUP.md. Grounding: lexicon.community site + governance repo,
lexicon-community Discussion #8 ("Attestation Lexicon"), Gerakines' published
ATProtocol (CID-First) attestation specification (badge.blue) + the
atproto-attestation reference crate, and Bluesky discussion #3338 on third-party
lexicon adoption. Everything proposed here is STAGED; test-first throughout. Open
calls tagged EL OC-N.`

> **STATUS (2026-07-20): the §6 experiment package RUN-LEX-01 is EXECUTED** —
> `../lexicon-community/` (33 acceptance tests green red-first, official
> `@atproto/lexicon` gate green, clean-room verifier proven INTEROPERABLE with the
> reference implementation, the stapled-status differentiator, and the ten-row
> feedback table `../lexicon-community/AMBIGUITIES.md`). Run ledger:
> `../lexicon-community/RUN-LEX-01-SUMMARY.md`. The engagement itself (posting to
> the thread) is NOT yet done — that is the owner's call (EL OC-1/OC-5). This brief
> is the plan; the lane is the artifacts the plan said to arrive with.

## 0. One sentence

The attestation conversation we planned to start already exists with a published
spec at its center; the winning engagement is not a rival proposal but independent
implementation, adversarial fixtures, and one genuinely missing piece — issuer
status without verifier callback — which RUN-LEX-01 has now built.

## 1. State of the field (grounded, July 2026)

- **The thread.** lexicon-community Discussion #8, "Attestation Lexicon" (active,
  owner @ngerakines engaged, most recent activity 2025-02-10): a signature over the
  DAG-CBOR of a record, verified via a verification method resolved through an
  `issuer`. Open in-thread and unsettled there: verification-method resolution
  ("is there a default algorithm? how do I compute it?"), an ambiguous bare `did`
  field, `sigs`-inline vs separate record, and the scope line that cross-repo
  writes are not enabled in atproto v1 (so attestations live in the issuer's own
  repo).
- **The spec.** Gerakines' formal **CID-First Attestation Specification**
  (badge.blue) with two patterns — **inline** (signatures embedded in a
  `signatures` array) and **remote** (a separate proof record + strongRef); replay
  prevention via **repository binding**; integrity via **CID-based content
  addressing**; ECDSA over three blessed curves, low-S. Reference crate:
  `atproto-attestation`. Companion: attested.network (three-party proof-of-payments).
- **The platform context.** Third-party lexicons are pre-tooling (Lexicon
  Resolution designed, SDK support unimplemented — discussion #3338). Exactly the
  moment fixtures, validators, and independent implementations carry outsized weight.

Posture: we are not first and do not pretend to be. Contribution surface = (a)
second-implementation feedback, (b) test-corpus discipline nobody has supplied,
(c) the status/freshness layer the spec does not cover.

## 2. Shared concerns (theirs → ours)

| Their concern | Our matching work | Alignment |
|---|---|---|
| Sign canonical record bytes (DAG-CBOR + CID) | Envelope-hash identity; canonical-by-value leaves | same instinct: sign content, not location |
| Replay prevention via repository binding | Repo-bound context in the matchup patterns | same threat model |
| Inline vs remote placement | Core-embedded-as-content pattern decision | we chose inline-with-content for the same reasons |
| Verification-method resolution (open in #8) | Issuer key discovery via DID doc; era-keyed commitments | we arrive with a worked answer — AND a demonstrated gap (A-1) |
| Attestation freshness/status (absent from the spec) | Era tree heads, holder-stapled inclusion proofs, zero verifier callback | THE differentiating contribution |
| Three-party attestation | Ceremony-graded co-signed edges | convergent; theirs payments-first, ours presence-first |
| Field ambiguity, naming discipline | enum-never-knownValues; explicit-referent names | immediately useful review capital |
| Out-of-scope discipline (no shared repos) | Everything rides single-author records | clean fit with their scope line |

Deliberately NOT brought to the shared table: chaining/membership semantics,
graded resolvability, vouch classes, ceremony vocabulary, anything scoring-shaped.
Those are Drystone-side (`ing.croft.*`) or razor-excluded. The shared lexicon
stays a minimal signature-and-status core; our extensions live under our namespace
and compose.

## 3. The proposal (three planks, smallest first)

1. **Independent implementation + fixture corpus (credibility).** A clean-room
   verifier for the published inline + remote patterns, from the spec text alone,
   with golden + adversarial fixtures. Every clean-room ambiguity becomes a
   respectful spec-feedback note. **← RUN-LEX-01 EXP-LEX-01/02/03. Done, and it goes
   further than planned: the verifier is proven interoperable with the reference
   implementation's own output.**
2. **Status stapling as an extension record (the missing piece).** Issuer publishes
   one signed tree head per era; holders staple inclusion proofs; verifiers check
   with zero issuer callback. Framed as an OPTIONAL layer on their spec. **←
   EXP-LEX-04. Done, pre-tested; benchmarked vs a callback.**
3. **A working-group charter or thread revival (process).** Join Discussion #8 if it
   has a living owner (it does), else the charter ask. **← open (EL OC-1).**

## 4. Evidence we arrive with (grades stated)

- RUN-LEX-01: clean-room verifier INTEROPERABLE with the reference impl (inline +
  remote); DAG-CBOR/CID path byte- and CID-identical to real PDS records; stapled
  status (Modeled, loopback); a ten-row grounded ambiguity table with two
  security-relevant entries; official `@atproto/lexicon` gate green.
- Attestation lane context if asked: RUN-ATTEST-01..04 (91 tests, HolderBinding,
  era-keyed commitments, stapled status) — the design this lane's staple ports.
- ATProto seam analysis: ATTEST-ATPROTO-MATCHUP.md and the F-AT-6 correlator
  findings, disclosed rather than discovered-in-thread.
- Nothing field-deployed; we say so. The offer is implementation and test rigor.

## 5. Engagement mechanics (their process, our order)

1. Read before writing: full Discussion #8, the spec end to end, attested.network,
   the Lenses WG. **← done.**
2. Run the experiment package so the first post ships with artifacts. **← done
   (RUN-LEX-01).**
3. First Discourse/thread post: short, artifact-led — independent-implementation
   report, fixture corpus link (MIT), the ambiguity table's questions, one
   paragraph on stapled status + the demo. Layer-scoped language. **← drafted-ready;
   posting is the owner's call.**
4. Offer: reference verifier + fixtures under MIT; stapling as a candidate WG
   deliverable; our time for review rotation.
5. Only after traction: the Lenses conversation (EXP-LEX-05 worked example is
   ready), and much later era-transparency in production.

## 6. Experiment package — see RUN-LEX-01

The full §6 package (EXP-LEX-01..05, red-first mandatory) is executed in
`../lexicon-community/`. Highlights: the clean-room build empirically resolved the
spec's underspecified signing input (36-byte binary CID, SHA-256 ECDSA, low-S) by
reproducing the reference implementation's signature, and demonstrated that inline
attestations authenticate a key, not an issuer, until a DID-document binding check
is added (shipped). See `../lexicon-community/RUN-LEX-01-SUMMARY.md` and
`AMBIGUITIES.md`.

## 7. Risks & postures

- **Not-invented-here in reverse.** The stapling plank reads as "your spec, plus
  the layer it says nothing about," never competing architecture. Plank 1 before
  plank 2, always.
- **Namespace politics.** Candidate extensions proposed for `community.lexicon.*`
  via their process; nothing lands there by our hand alone. `ing.croft.*` stays the
  Drystone home.
- **Scope line.** Their out-of-scope (shared repos) is our friend — everything we
  bring is single-author records. Said explicitly.
- **Young-schema churn.** Vendored schemas pinned; the fixture harness is our
  early-warning system.
- **Correlator honesty.** Anything published on ATProto inherits F-AT-6 exposure;
  disclosure sentence in the demo README, not a discovered embarrassment.

## 8. Open calls

- **EL OC-1** — first post targets Discussion #8 revival (active) or a fresh thread
  citing it. Leaning: join #8.
- **EL OC-2** — staple extension naming (status/freshness/staple) + head-record
  visibility.
- **EL OC-3** — demo attests a consenting test event we create vs a real
  third-party RSVP (leaning: our own test event on a real PDS for anything public;
  the run's CID/verify proof used a real public RSVP).
- **EL OC-4** — TypeScript companion verifier (their ecosystem's language) as a
  follow-up run.
- **EL OC-5** — who speaks: personal persona, Croft org persona, or both;
  layer-scoped comms rule applies either way.
