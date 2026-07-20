# ENGAGE-LEX: brief for engaging the Lexicon Community attestation work

`Engagement brief + experiment package, 2026-07-19. Suggested landing:
alpha/experiments/attest-family/ENGAGE-LEX.md (companion to PRIMITIVES-ATTEST.md and
ATTEST-ATPROTO-MATCHUP.md). Grounding: lexicon.community site and governance repo,
lexicon-community org Discussion #8 ("Attestation Lexicon"), Gerakines' published
ATProtocol attestation specification and attested.network posts, and Bluesky discussion
#3338 on third-party lexicon adoption. Everything proposed here is STAGED; the
experiment package mandates test-first throughout. Open calls tagged EL OC-N.`

## 0. One sentence

The attestation conversation we planned to start already exists with a published spec at
its center; the winning engagement is not a rival proposal but independent
implementation, adversarial fixtures, and one genuinely missing piece we have already
built: issuer status without verifier callback.

## 1. State of the field (grounded, July 2026)

- **The thread.** lexicon-community org Discussion #8, "Attestation Lexicon" (opened
  ~January 2025), discusses a proposal where a signature is generated over the DAG-CBOR
  representation of a record and verified via a "verification method" resolved through
  an `issuer` element. Open questions raised in-thread and not visibly settled:
  how verification-method resolution works, ambiguous field naming (a bare `did` whose
  referent is unclear), whether the signature lives inline or as a separate record, and
  a scoping comment that anything requiring shared records/repos is out of scope for
  lexicon.community until ATProto itself moves (shared repos being an "ATProto v2"
  aspiration).

- **The spec.** Gerakines has since published a formal ATProtocol attestation
  specification with two complementary patterns: **inline** attestations embedding
  signatures directly in records, and **remote** attestations storing proof in separate
  repository records; replay prevention via repository binding; integrity via CID-based
  content addressing. Companion work: attested.network, an open spec for decentralized
  proof-of-payments formalizing a **three-party attestation model** (learned from
  atprotofans.com), and a deep-dive "unforgeable endorsement" write-up with CID
  computation steps, firehose processing, validation algorithms, and lexicon
  definitions. Acudo's organizer-signed RSVPs follow this line. He is also
  standards-minded (IETF participation) and runs lexicon.garden.

- **The platform context.** Bluesky PBC states there are no hard blockers to
  third-party lexicons in the flagship app and that the Lexicon Resolution protocol
  design is complete but tooling/SDK support is unimplemented (discussion #3338). So
  community lexicons are pre-tooling: exactly the moment when fixtures, validators, and
  independent implementations carry outsized weight.

Consequence for posture: we are not first and should not pretend to be. The
contribution surface is (a) second-implementation feedback on a young spec, (b) test
corpus discipline nobody has yet supplied, and (c) the status/freshness layer the
published spec does not cover, where RUN-ATTEST-04's stapling result is a genuine
missing piece.

## 2. Shared concerns (theirs mapped to ours)

| Their concern (from the spec, thread, and Acudo) | Our matching work | Alignment |
|---|---|---|
| Signature over canonical record bytes (DAG-CBOR + CID) | Envelope hash identity; canonical-by-value leaves (RUN-ATTEST-04) | Same instinct: sign content, not location |
| Replay prevention via repository binding | Repo-bound context in the matchup patterns | Same threat model |
| Inline vs remote attestation placement | Core-embedded-as-content-not-hash pattern decision (matchup) | We already chose inline-with-content for the same reasons |
| Verification-method resolution (open question in #8) | Issuer key discovery via DID doc; era-keyed issuer commitments | We arrive with a worked answer to their open question |
| Attestation freshness / status (absent from the spec) | Era tree heads, holder-stapled inclusion proofs, superseded-set staple block, zero verifier callback (RUN-ATTEST-04, V5-V6) | The differentiating contribution: status without issuer capture or verifier surveillance |
| Three-party attestation (payer/recipient/attester) | Ceremony-graded co-signed edges; organizer-signed co-presence standing | Convergent shapes; theirs payments-first, ours presence-first |
| Field ambiguity, naming discipline (thread complaint) | Enum-never-knownValues and explicit-referent conventions (matchup) | Small but immediately useful review capital |
| Out-of-scope discipline (no shared-repo dependence) | Everything we would bring rides single-author records | Clean fit with their stated scope line |

What we deliberately do NOT bring to the shared table: chaining/membership semantics,
graded resolvability, vouch qualification classes, ceremony-grade vocabulary beyond an
optional open slot, anything scoring-shaped. Those are Drystone-side or razor-excluded.
The shared lexicon should stay a minimal signature-and-status core; our extensions live
under our namespace and compose.

## 3. The proposal we bring

Three planks, smallest first:

1. **Independent implementation + fixture corpus (the credibility plank).** A clean-room
   verifier for the published inline and remote patterns, built from the spec text
   alone, with a corpus of golden and adversarial fixtures (bad CID, wrong repo
   binding, mutated DAG-CBOR, stale key, malformed verification method). Every
   ambiguity the clean-room build hits becomes a respectful spec-feedback note in the
   thread. This is the contribution young specs need most and get least.

2. **Status stapling as an extension record (the missing-piece plank).** A candidate
   lexicon adding freshness to remote attestations: issuer publishes one signed tree
   head per era over keyed commitments; holders staple inclusion proofs into
   presentations; verifiers check with zero callback to the issuer. Pitch line: OCSP
   taught the web that status-by-callback centralizes and surveils; stapling is the
   known cure, and we bring it pre-tested (91 red-first tests in the lane, stapling
   replacing the callback design at V5-V6). Explicitly framed as an optional layer on
   their spec, not a change to it.

3. **A working-group charter or thread revival (the process plank).** Per their own
   process: start on Discourse, point at the builders already in the space (Acudo,
   Smoke Signal, attested.network, us), name what a shared schema unlocks (verifiable
   attendance, tickets, receipts, and credentials that any AppView can check without
   phoning an issuer), and ask the TSC to designate a sponsor. If Discussion #8 has a
   living owner, we join it rather than fork it; the charter ask is the fallback, not
   the opener.

## 4. Evidence we arrive with (grades stated)

- Attestation lane: RUN-ATTEST-01..04 merged, 91 tests green red-first,
  zero open owner calls; HolderBinding, era-keyed commitments, frozen-era freshness,
  stapled status (Modeled grade, loopback environments).

- Substrate context if asked: tier proof and reception machinery (RUN-17/18,
  demonstrated end to end), MLS-in-WASM over real QUIC/WebTransport (RUN-19).

- ATProto seam analysis: the matchup doc bounding which abilities ATProto natively
  supplies, the lossless mapping spike, and the F-AT-6 correlator findings (PLC op-log
  timing, hosting, enumerability), which we disclose rather than discover-in-thread.

- Nothing field-deployed; we say so. The offer is implementation and test rigor, not
  production war stories.

## 5. Engagement mechanics (their process, our order)

1. Read before writing: full Discussion #8 history, the published spec end to end, the
   attested.network spec, and the Polite Goshawk (Lenses) WG repo.

2. Run the experiment package below so the first post ships with artifacts, not
   intentions.

3. First Discourse post: short, artifact-led. Independent-implementation report,
   fixture corpus link (MIT), two or three concrete spec-feedback items phrased as
   questions, and one paragraph introducing stapled status with the demo. No Drystone
   evangelism; layer-scoped language per the comms rule (protocol: no one's; records:
   personal; infra: co-op).

4. Offer: reference verifier + fixtures donated to lexicon-community under MIT;
   stapling as a candidate WG deliverable; our time for review rotation.

5. Only after traction: the Lenses conversation for the calendar projection seam, and
   much later, era-transparency as a pattern the co-op issuer will run in production.

## 6. Claude Code experiment package (RUN-LEX-01, red-first mandatory)

Standing directive applies in full: acceptance criteria land as failing tests before
implementation, fixtures before features, red-to-green order evidenced in the run
summary. Rust-first to reuse lane crates; TypeScript shims only where @atproto tooling
is the validator of record.

- **EXP-LEX-01: fixture corpus + schema validation harness.**
  Vendor the lexicon JSONs we depend on (calendar event, RSVP, location core, the
  attestation spec's lexicon definitions). Failing tests first: every vendored schema
  validates under the official Lexicon tooling; every golden fixture record validates
  against its schema; every adversarial fixture (Incident-1-class invalid type, wrong
  enum casing, missing required, unknown-field round-trip) fails for the stated
  reason. Artifact: `fixtures/` + a one-command validation gate.

- **EXP-LEX-02: live-network consumption proof.**
  Recorded-fixture tests first (listRecords/getRecord captures checked in), live fetch
  behind a flag. Prove: parse real public calendar event and RSVP records from
  arbitrary PDSs; resolve strong refs; re-serialize to byte-identical DAG-CBOR and
  recompute matching CIDs. Artifact: "we consume the live network losslessly" report
  with per-record CID receipts.

- **EXP-LEX-03: clean-room verifier for the published attestation spec.**
  From spec text alone: inline and remote patterns, repository binding, CID
  addressing, verification-method resolution via DID doc. Failing tests first from
  the spec's own examples, then our adversarial set (replayed-to-other-repo, mutated
  payload, issuer key rotated mid-lineage). Every point where the spec underdetermines
  behavior becomes a logged AMBIGUITY entry; the run summary's ambiguity table IS the
  feedback post. Then the show-and-tell: an organizer-signed attendance attestation
  over a real public RSVP record, verified by the clean-room verifier in one command.

- **EXP-LEX-04: stapled-status extension prototype.**
  Failing tests first: verifier accepts a remote attestation with a valid era staple
  and no network access; rejects superseded (staple from a frozen era against a newer
  head); rejects forged inclusion. Port the lane's tree-head and staple machinery over
  the spec's remote-attestation record shape; draft the candidate extension lexicon.
  Benchmark table: staple bytes and verify time vs a simulated callback check.
  Artifact: the differentiating demo plus the draft schema.

- **EXP-LEX-05: lens seam spike (stretch).**
  Read the Polite Goshawk lens shape first; if a definition format exists, express the
  Croft envelope-projection → calendar-event transform as a lens with round-trip tests
  on EXP-LEX-02's fixtures. If the WG has no concrete format yet, downgrade to a
  worked-example document for their thread.

Drop order under pressure: 05, then 04's benchmark table (keep the staple tests), then
02's live flag (keep recorded fixtures). 01 and 03 are the spine; without them there is
no credible first post.

Definition of show-and-tell ready: EXP-LEX-01+03 green with a non-empty ambiguity
table, the one-command attendance-attestation demo, and the fixtures repo publishable
under MIT the same day the Discourse post goes up.

## 7. Risks and postures

- **Not-invented-here in reverse.** The spec author is prolific and central; the
  stapling plank must read as "your spec, plus the layer it says nothing about," never
  as competing architecture. Plank 1 before plank 2, always.

- **Namespace politics.** Candidate extension proposed for `community.lexicon.*` via
  their process; nothing lands there by our hand alone. `ing.croft.*` stays the home
  for everything Drystone-specific.

- **Scope line.** Their stated out-of-scope (shared records/repos) is our friend:
  everything we bring is single-author records. Say so explicitly in the first post.

- **Young-schema churn.** Pin vendored schema versions; the fixture harness is also
  our early-warning system for upstream changes.

- **Correlator honesty.** Anything we publish on ATProto inherits F-AT-6 exposure;
  disclosure sentence in the demo README rather than a discovered embarrassment.

## 8. Open calls

- EL OC-1: whether the first post targets Discussion #8 revival or a fresh Discourse
  thread citing it (depends on read of #8's liveness; decide after step 1).

- EL OC-2: staple extension naming (status vs freshness vs staple) and whether the era
  head record lives in the issuer's repo as a public record (visibility trade).

- EL OC-3: whether EXP-LEX-03's demo attests a consenting test event we create or a
  real third-party RSVP (consent optics; leaning: our own test event on a real PDS).

- EL OC-4: TypeScript companion verifier (their ecosystem's language) as a follow-up
  run, for adoption friction.

- EL OC-5: who speaks: the engagement lands under a personal persona, a Croft org
  persona, or both; layer-scoped comms rule applies either way.
