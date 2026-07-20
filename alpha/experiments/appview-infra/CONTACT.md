# First contact on the group substrate: postures, knocks, and the door

`Design document, 2026-07-18. Standalone corpus payload, written to be placed beside GROUPS.md
and PUBLICATIONS.md (suggested landing: alpha/experiments/appview-infra/CONTACT.md). It
records the derivation that began with "could Drystone underlie a dating, friendship, or
meetup app" and ended with a first-contact model. Companion to the group tier model
(GROUPS.md / RUN-16 §A) and the attestation family (PRIMITIVES-ATTEST.md,
RUN-ATTEST-01..03). Everything in this document is STAGED, not applied: Design grade unless
a sentence carries its own evidence parenthetical. Open calls are tagged CT OC-N and decide
nothing by their presence.`

## 0. One sentence

Meeting a stranger is the one social act the substrate has no shape for yet; this document
gives it one, built from parts already proven, with silence as a first-class outcome and
the operator holding a door, never a room.

## 1. Position in the corpus

This is not a fourth tier. The combination that motivated it, operator-readable payloads
with private membership, already exists as a coordinate of the A.2 model: the gated
backplane tier with the salt-blinding dial on (cleartext signed content, roster
salt-blinded, distribution DS-gated). What this document adds is exactly two moves and one
primitive family:

- **Move 1, posture as consent dial.** The sealed-vs-backplane choice, currently a scale
  parameter (`group_scale_boundary`, below which scopes seal), additionally becomes a
  per-scope choice the members make knowingly. A two-person scope MAY run the readable
  posture on purpose. The scale boundary remains the default; the dial is the exception a
  scope elects with the banner discipline of §9.

- **Move 2, path as delivery preference.** Which route a first-contact act travels
  (DS-queued or direct local) is an opt-in delivery preference distinct from the act
  itself, the same cut A.8 makes for swarm participation. Product decides what to surface;
  transparency decides what to disclose; protocol forbids neither path.

- **The primitive family.** Contact policy (§4), knock (§5), mutual reveal (§6), contact
  scope (§7), block (§8). All are records, folds, and gates; none is new cryptography.

The degeneration principle binds here as everywhere: every mechanism below must earn its
place by expressing something the existing substrate structurally cannot. Where a section
merely applies an existing rule to a new noun, it says so and cites the rule.

## 2. Definitions

- **Discovery scope**: a scope whose purpose is letting strangers find each other (the
  catalogue of a dating or friendship surface, the attendee surface of an event). Gated
  and blinded by default in this domain (§3.2, §11).

- **Contact policy**: a self-authored record on a persona stating how that persona may be
  approached (§4). Provenance, not preference text: the gate enforces it.

- **Knock**: a single structured envelope requesting contact, carrying bounded content and
  standing assertions (§5). A knock is one act, not a channel.

- **Standing assertion**: an attest-family artifact attached to a knock proving something
  about the sender without opening their graph: an anchor-persona credential, a scoped
  vouch with a qualifying antecedent (evidence: AntecedentKind closed enum,
  RUN-ATTEST-03, Modeled), a mutual-count-without-identity disclosure, or a co-presence
  fact (§5.2, §10.5).

- **Contact scope**: the dyadic scope minted by an accept, in which the two parties
  actually converse (§7).

- **Posture**: which of two protection models a scope runs. *Provider-visible*: content
  readable by the operator the members chose, hidden from the world, screened, and
  evidence-capable. *Sealed*: real MLS end-to-end encryption via group-seal (evidence:
  L2a group-seal crate, Verified/loopback), operator content-blind.

- **The gate**: the validate-before-relay checkpoint at the DS, extended in this document
  by one clause (§4.3).

## 3. The provider-visible posture

### 3.1 Naming rule (binding on all product surfaces and docs)

The readable posture is NEVER described as MLS, encrypted, or private messaging. Its name
in user-facing language is **provider-visible messaging**, and its framing is a helper
mode: basic safety and audit, provided by an operator the members chose. The word MLS
appears in the UX only where it is cryptographically true (the sealed posture). Envelope
semantics stay identical across postures (§3.3), so the *shape* of messaging is one thing
and the posture is a property of it, which is what permits honest naming.

### 3.2 Coherence condition

Signed cleartext envelopes name their authors. Therefore provider-visible content MUST
never touch a public path: no firehose, no public PDS records, no open catalogue listing
of the scope. Roster entries are salt-blinded per A.1; the DS offers content to roster
members only. "Private membership" in this posture means non-discernible from public
data, and "readable" means readable by the chosen operator, not by the world and not
confidential from the operator. The same rule governs knocks (§5.3) and discovery
profiles (§11.1): in this domain, records that describe people default to scoped
distribution, and publication to any public path is a separate, explicit, per-record act
with its own banner.

### 3.3 One envelope shape

All postures carry the same envelope: scope, antecedents, payload, signature, hash
identity (rule inherited from A.5, applied unchanged). A provider-visible message and a
sealed message differ only in whether the payload is ciphertext under the scope's MLS
epoch. Clients therefore render one messaging surface with a posture indicator, not two
products.

### 3.4 The RFC boundary

RFC 9420 requires that application messages be sent as PrivateMessage; only handshake
messages may travel as PublicMessage, and RFC 9750 explicitly anticipates the delivery
service observing handshakes to enforce policy (evidence: RFC 9420 §framing, RFC 9750,
Verified-RFC). Consequence: plaintext application data inside real MLS is off-spec, so
the provider-visible posture is built on backplane machinery (signed envelopes, roster
fold, capability grants), not on an MLS instance. MLS enters a contact scope exactly
when the scope goes sealed. §3.1's naming rule is therefore also technically accurate,
not merely honest marketing.

### 3.5 Safety rationale, stated honestly

Provider-visible and sealed are two different protection models, not a strong and a weak
version of one model:

- Provider-visible protects by **screening** (the operator can filter abuse before it
  reaches a person) and by **evidence** (a signed cleartext message is non-repudiable;
  reporting it breaks no seal). It does not protect confidentiality from the operator or
  from anyone who can compel the operator.

- Sealed protects **confidentiality** (forward secrecy, post-compromise security,
  operator content-blindness). It cannot screen, and reports from within it are
  assertions, not evidence.

Open social discovery is a context where screening-and-evidence frequently beats
confidentiality, which is why provider-visible MAY be the surface default (§3.6). The
choice is stated to users in exactly these terms; neither posture is described as
"more secure" without saying against what.

### 3.6 Deployment default and the jurisdiction sentence

The posture default is a **per-deployment parameter**, not a global constant. Deployments
serving audiences for whom operator-readable content is itself the threat (the
compellability risk clusters demographically and jurisdictionally) SHOULD default
sealed-first. Every provider-visible banner MUST include the jurisdiction sentence: a
plain statement that content readable by the operator is content obtainable from the
operator by legal process in the operator's jurisdiction, named.

### 3.7 Retention covenant

Provider-visible screening data expires on a short, stated window. Evidence is pinned
beyond that window only when a member of the scope files a report, and the pin covers the
reported material. The covenant is published as an R7-governed rule of the operator's
service scope, so a member can verify the stated policy is the governed one (rule
machinery evidence: R7 Verified for the count; the covenant instance itself: Design).
Window length is CT OC-1.

## 4. Contact policy

### 4.1 The record

Each persona MAY publish one contact-policy record: a self-authored fact, superseded by
replacement, never edited in place (rule inherited from the supersede-never-revoke seam
decision). Verbs, closed set for v1:

- `open`: any well-formed knock admitted.

- `requires_standing(X)`: knocks admitted only with assertions satisfying predicate X,
  where X is composed from the closed antecedent classes (evidence: AntecedentKind,
  RUN-ATTEST-03, Modeled) plus anchor-credential presence and co-presence facts.

- `requires_sealed`: no provider-visible first contact; acceptable approach is a sealed
  proposal only (the scope, if minted, begins sealed).

- `closed`: no queue exists for this persona. A persona with no published policy is
  `closed` (protective default; composes with freeze-by-default).

### 4.2 Path-aware predicates

X MAY discriminate on transport provenance and context: for example, `ds_screened_only`
(the discovery default) or `direct_ok_if(co_present(event_scope))` (the event-room
setting). Both are predicates over facts the fold already holds, keeping policy a pure
rendering of provenance.

### 4.3 Enforcement

Validate-before-relay gains one clause: a knock that does not satisfy the target
persona's current policy is dropped unpropagated. Policy checks are transparent
(rejection for an unmet published requirement MAY say so, since the policy is public to
whoever can see the card); screening verdicts are silent (§5.6). Enforcement lives at
the gate, not in the client, so a hostile client gains nothing by ignoring policy.

## 5. The knock

### 5.1 Structured shape, v1

A knock is one envelope with a fixed shape: bounded plain text (bound value CT OC-2),
zero media, standing assertions (§5.2), and an optional reference to what prompted it (a
profile card, an event, a mutual-reveal match). Media becomes possible only inside an
accepted contact scope, behind an explicit per-scope toggle. Reasoning on record: the
ping precedent (free text and free media turn a gesture back into messaging), and the
best-documented category of first-contact harm is unsolicited explicit media; no
legitimate first message requires an image.

### 5.2 Standing

Assertions attach by reference and MUST be verifiable without resolving the sender's
other personas (sibling unlinkability is inherited absolute from RUN-ATTEST-02, Modeled).
Recognized classes for v1: anchor-persona credential presence with status check; scoped
vouch carrying a qualifying antecedent per V1; mutual-count-without-identity;
co-presence fact (§10.5). Requiring standing is the recipient-chosen Sybil cost, the
same shape as fee-as-friction.

### 5.3 Paths

Two transports, one act:

- **DS-queued** (default for open discovery): the knock travels directly to the
  recipient's chosen DS, never through any public path (coherence condition §3.2 applied
  to knocks: "A knocked on B" is itself sensitive and MUST NOT be a public record; the
  queue is therefore operational DS state, not a rebuildable public projection, and that
  trade is stated in posture language).

- **Direct local** (co-presence contexts): peer-to-peer, no operator in the loop, no
  screening, no operator evidence trail; physical co-presence is its own verification
  and its own risk model. The receiving client MUST disclose which path a knock arrived
  by (§10.5).

Delivery durability is a helper on both paths, never a guarantee: the sender's client
holds the knock and MAY re-offer if a DS is replaced. What the protocol owes is that an
*accepted* knock's consequences are provable facts, not that any particular envelope
survives.

### 5.4 Queue semantics

Admission rule = the recipient's policy (that is where "at user discretion" operates).
Queue states: exists, superseded, accepted. No timeout, no auto-decline, no delivery or
read signal to the sender. Authored transitions only: accept (§7), decline (optional
courtesy record), sender withdrawal (supersede own knock; symmetric with leave
semantics, since the knock is the sender's record).

### 5.5 Silence

**Silence is never assent or approval, and it is a first-class outcome. Voice is
exercisable, not mandatory.** Consequences, binding:

- A recipient's non-response authors no record anywhere: nothing in their repo, nothing
  conscripting them into a relation with the sender, nothing the sender can cite.

- "Pending to stale" is the sender's client rendering on the sender's own tolerance
  dial; staleness is the sender's narrative about their own act, never information
  about the recipient.

- The sender's "pending" is honestly indistinguishable across screened-out, delivered
  but unseen, and seen and passed over. This indistinguishability is the protection and
  is stated in posture language as a feature.

- The strongest claim a sender's client ever renders is "sent." Everything past the
  transport receipt is silence-space.

(Rule lineage: A.3 pending-forever, decay-is-presentation-never-expiry,
no-verdict-by-timeout; this section is their affirmative statement.)

### 5.6 Acknowledgement semantics

Published-policy checks transparent, screening silent, no read or delivery receipts.
A gate rejection for unmet standing MAY carry the policy clause it failed; a screening
drop looks identical to silence, denying abusers an oracle.

## 6. Mutual reveal

The protective sibling of the knock, and the default gesture on dating surfaces: both
parties commit interest via fair-reveal (evidence: fair-reveal commit-reveal module,
per its run grade), the DS is an untrusted rendezvous, and interest is disclosed to the
counterparty and the operator only when mutual. A successful reveal feeds the same
accept flow as a knock (§7), with the reveal as the prompting reference. The knock
remains available as the deliberate, priced "walk up and say hello" act; the surface
prices the two gestures the way the physical world already does. Cohort scoping of
reveals (whole catalogue vs event cohort) is CT OC-3.

## 7. Accept and the contact scope

Accept mints a dyadic scope: genesis record, both membership facts two-sided (the
knock or reveal is the requester's side, the accept is the recipient's; conscription
impossible by the A.3 shape), roster blinded, no catalogue listing. Posture at minting
follows the recipient's policy and the surface default, with the banner shown to both.

The write capability is granted to the requester's **persona** and exercised through
**per-device delegated keys** (evidence: delegation machinery, RUN-17, per its grade),
yielding two revocation granularities that match how people think: revoke a device
(lost phone) without revoking the person; supersede the capability (done with the
person) regardless of devices.

## 8. Revocation and blocks

### 8.1 Pull-back

Capability supersession, never revoke-in-place (seam decision, inherited). Causal cut
enforced at the gate: on DS-mediated paths the cut is effectively immediate because the
DS stops accepting the superseded capability's envelopes. The revoked party has no
holding; their ability to write was a granted capability, not a standing right. In a
sealed scope, pull-back additionally removes the leaf and rolls the epoch
(cryptographic removal). Pre-upgrade provider-visible messages remain evidence held by
both parties either way.

### 8.2 Blocks

A block is a signed per-persona policy fact delivered to the persona's DS and held as
provider-visible policy state, consulted at the gate before queue admission. Never a
public record (the public-block-list failure is a known privacy harm; telling only the
operator is exactly the trust the readable posture already grants). Pull-back from a
contact scope writes a block automatically unless the actor opts out. In sealed scopes
a block is local rendering plus leaf removal. Blocks bind personas; the recidivist
question routes to §13, not to the gate.

## 9. Posture transitions and the banner catalogue

Both directions are re-plants under unchanged lineage (rule inherited from A.9,
including the mandatory plain-language statement), both require both-consent, and a
transition proposal from one side renders as a pending request where **silence
preserves the more protective state**: a sealed scope stays sealed, a proposed upgrade
to sealed does not lapse into readable. §5.5's principle applied to security posture.

Downgrade specifics: past sealed history stays sealed (keys do not retroactively open);
only new messages become provider-visible; the banner says both things.

Banner catalogue (the MUST-surface statements, collected):

- Provider-visible scope, at entry and on demand: operator named; readable for
  screening, safety review, and abuse evidence; not confidential from the operator;
  jurisdiction sentence (§3.6); retention covenant summary (§3.7); "you can move to
  sealed at any time."

- Upgrade to sealed: operator becomes content-blind; screening and report-with-evidence
  end for new messages; prior readable messages were readable and operator deletion of
  processed content is a covenant, not cryptography.

- Downgrade to provider-visible: everything in the provider-visible banner, plus "past
  sealed messages stay sealed."

- Direct-path knock, at receipt: "arrived directly, never screened, no operator record."

- Queue posture: delivery is best-effort; a lost queue loses pending knocks, not
  accepted contacts.

## 10. Events integration

### 10.1 Two event tiers (as proposed, now normative shape)

An event is a scope. Tier one: a catalogue marker exists, blinded (salt-hashed lineage,
outsiders cannot correlate which community) or plain, while roster and coordination are
gated or sealed. Tier two: purely a sealed artifact, no catalogue entry, no PDS trace,
invite = MLS Welcome, all delivery roles optional (A.7 replaceability, inherited; pure
P2P valid).

### 10.2 Attendance-disclosure dial

Per-scope, three positions: public RSVP records anyone can re-derive (evidence:
auditable-reach refold, RUN-18, per its grade); steward-co-signed aggregate only
("N attended," private reasoning, public rulings applied to attendance); nothing.
Default for dating-adjacent events: aggregate only. Default per broader context is
CT OC-4.

### 10.3 Permanence split by role

The organizer's announcement stream is chained (tamper-evident permanence is the product
there, owed to subscribers about content). Attendee participation defaults to open-tier
self-registration in a blinded scope, so leaving is deleting and the trace is
self-sovereign; "provably attended a singles night" is erasable by the person it
describes. Write-restricted chained participation is available where a community
knowingly wants it, with the banner saying what permanence means.

### 10.4 Late-binding venue

The venue is its own record, released in-scope to confirmed attendees at a configured
time (default near-day-before, exact default CT OC-5), operator-readable where the scope
is provider-visible and the banner says so. Copies the proven curated-dinner pattern at
the cost of one roster-gated envelope.

### 10.5 Co-presence standing and RSVP references

Attendance at an event is provable standing (an interval in the event scope), and an
in-person exchange is the top ceremony grade in the attestation family, so a direct
knock in the room can carry stronger standing than any screened knock. An RSVP envelope
carries the event envelope's hash in its antecedents, so "responded to this version" is
structural, not a feature.

## 11. Admission standing for discovery scopes

### 11.1 Profiles

Discovery profiles are records served roster-gated within the discovery scope, never
public PDS records (§3.2 applied). Interop with public event/RSVP lexicons is a
per-scope, per-record publish choice with a banner ("public in the open events
network"), right for community meetups, wrong by default for dating surfaces.

### 11.2 Age

Dating and adjacent discovery scopes carry verified-18 standing as a genesis admission
policy; the operator refuses to serve the catalogue without it; the co-op anchor
credential is the intended satisfier. The open tier structurally cannot stop
self-registration, so the check is standing at the door, encoded in genesis as an
auditable rule rather than a terms-of-service promise, placing liability with the
accountable entity on purpose.

## 12. Exclusions and non-goals

- **No matching, ranking, or compatibility scoring in the protocol.** The razor. The
  transparent baseline is user-ranked attribute assertions evaluated openly by either
  client; any advanced matcher is an opt-in helper at user discretion, at the app layer.

- **No person-targeted unilateral public reviews in this context, v1.** Consistent with
  V3 (unilateral_private deferred). Permitted person-directed assertions: the positive
  closed class (vouches with qualifying antecedents per V1) and reports to the operator,
  which are evidence, not publication. A negative unilateral claim about a person is a
  harm whose entire substance is truth, and the protocol cannot compute truth;
  provably-attributed accusation is provenance in service of defamation. Revisit only
  with a designed dispute process (parked with V3's private-remediation thread).

- **No media in knocks. No read receipts anywhere in first contact.**

- **No anonymity claims beyond what blinding gives.** Blinding hides which lineage and
  who is enrolled from the public; it does not hide traffic patterns from the chosen
  operator, and the posture language says so.

## 13. Recidivism, Sybil, and the honest cost

Sibling unlinkability stays absolute at every gate, forever: the moment any gate can
correlate anchor personas, unlinkability is dead for every honest user, and that
property is the product. Exclusion of bad actors therefore concentrates at the
**issuer**: on adjudicated abuse the co-op refuses or revokes credentials via the
status-check machinery (evidence: OCSP-style status check, RUN-ATTEST-02, Modeled),
under an R7-governed adjudication process (sealed deliberation, published verdict,
appeal path, covenant-bound). Fee friction remains the base rate limiter. The honest
cost, stated rather than hidden: a determined abuser who keeps paying gets N chances;
that is the price of not building centralized biometric identity, and deployments say
it out loud. Adjudication process design is CT OC-6.

## 14. The operator's chair

Holds: discovery catalogue and search index (disposable projections, incapable of
inventing a user or count that survives an audit refold); knock queues and contact-scope
rosters and policies (small operational and canonical state); readable payloads for
provider-visible scopes under the ratified write-path variant; ciphertext only for
sealed scopes. Does: gates offering by roster, screens knocks and provider-visible
traffic, operates report-evidence workflows, curates discovery at the display layer.
Structurally cannot: admit, expel, or hold anyone on a roster; keep a revoked party
writing; read a sealed scope; inflate a count; convert service into standing (plane
separation, inherited). Replaceable: scopes are lineages, not operator rows; the same
grants let another operator serve them.

## 15. Product sequencing note

Discovery launches as a feature of events, not beside them: event series, co-presence
knocks, and mutual reveal within event cohorts first; the standing catalogue later,
seeded by ceremony-graded edges. This converts the cold-start objection into the
sequencing already chosen (meetup/community as beachhead), and the event delivers value
when no match happens, so early users pay no network-effect tax. Product-layer detail
belongs in the Croft docs, not here; this section exists so the protocol pieces above
are read in their intended order of arrival.

## 16. Evidence and grades

Everything introduced by this document is **Design** grade. Sentences riding proven
machinery cite it inline above; the load-bearing imports are: R7 governance (Verified
for the count), delegation and roster machinery (RUN-17), chaining and auditable-count
refold (RUN-18), group-seal (Verified/loopback), fair-reveal (per its run), attestation
family and status check (RUN-ATTEST-01..03, Modeled), and the RFC 9420/9750 framing
rules (Verified-RFC, primary-source anchored). No claim above upgrades an import's
grade.

## 17. Executable evidence plan (RUN-CONTACT-01, sketch)

Test-first is mandatory throughout: acceptance criteria land as failing tests before any
implementation, fixtures before features, red-to-green order evidenced in the run
summary. Candidate proof set, in dependency order:

1. Contact-policy fold: closed verb set; unpublished policy folds `closed`; supersede
   replaces; path-aware predicate evaluates over folded facts only.

2. Gate clause: knock failing policy is dropped unpropagated; policy rejection
   transparent, screening drop silent and byte-identical to silence from the sender's
   view.

3. Structured knock: media-bearing knock rejected at validation; text bound enforced;
   standing verification without sibling resolution (negative test: correlation attempt
   fails).

4. Queue lifecycle: exists/superseded/accepted only; no state transition on elapsed
   time (adversarial clock test); withdrawal by sender supersedes.

5. Accept: scope minted with two-sided facts; conscription attempt cannot construct a
   valid roster; capability exercised via device delegation; device revocation vs
   capability supersession distinguished.

6. Pull-back: superseded capability's envelopes refused at gate; auto-block written;
   block consulted pre-admission; block never appears in any public projection.

7. Posture transition: re-plant preserves lineage; both-consent required; silence
   preserves sealed (pending upgrade proposal does not lapse); downgrade leaves prior
   ciphertext sealed.

8. Attendance dial: three positions fold correctly; aggregate position yields a
   co-signed count with no roster disclosure; open position survives the RUN-18 audit
   refold.

9. Banner surface: every transition and posture entry emits the catalogue string for
   its case (string-presence test, keeping language load-bearing).

## 18. Open calls (surfaced, not decided)

- CT OC-1: retention window value for provider-visible screening data.

- CT OC-2: knock text bound (value and unit).

- CT OC-3: mutual-reveal cohort scoping (catalogue-wide vs event-cohort vs both).

- CT OC-4: attendance-dial defaults per context class beyond dating-adjacent.

- CT OC-5: venue-release timing default.

- CT OC-6: issuer adjudication process design (who convenes, quorum shape, appeal path)
  under the covenant.

- CT OC-7: per-deployment posture presets (which audience classes flip the default to
  sealed-first, and the preset vocabulary).

- CT OC-8: rate-limit semantics for knocks (display-layer only vs gate-enforced, and
  per what key).

- CT OC-9: downgrade with partial device sets (a member whose devices straddle the
  transition; welcome/rejoin choreography).

- CT OC-10: interaction with sole_anchor(context) once that parked attestation item is
  decided.

## 19. Pointers

Tier mechanics: GROUPS.md (A.1 blinding, A.2 axes, A.3 membership facts, A.5 envelope,
A.7 roles, A.8 transports, A.9 transitions, A.10 owner decisions). Publications model
(the event-series-as-publication reading): PUBLICATIONS.md. Attestation family and
anchor personas: PRIMITIVES-ATTEST.md, RUN-ATTEST-01..03, ATTEST-ATPROTO-MATCHUP.md.
Executable lineage this plan extends: RUN-17 (tier proof), RUN-18
(reception/publications amendment).
