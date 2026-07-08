# Proof of personhood and the identity layering: why "not the protocol's job" is a conclusion

`Status: philosophy layer (Layer 2), prior-art register. Register: survey / grounding for a spec
position. Resolution: library — the full proof-of-personhood prior art and the identity-layering
distinctions that the spec's persona vocabulary rests on. This material was surfaced in an AI
discussion session; every author and paper attribution below carries [UNVERIFIED, confirm before
publish] and must be checked against primary sources before any external use. Terms of art coined by
their originators are italicized on first use.`

## Overview

Drystone Part 2 §5.2 (Principal, client, persona: the identity model) takes a deliberate stance: whether
a given key-lineage corresponds to a distinct human being is a judgment the *Group* makes for its own
purposes, never a fact the protocol computes. The spec states that position flatly and moves on. It does
not show the prior art that makes the position defensible rather than evasive.

This document is that prior art. It registers the *proof-of-personhood* research literature — the body of
work that has spent two decades trying, and repeatedly failing, to make one-human-one-unit a thing a
protocol can decide — and it registers the clean identity-layering vocabulary (principal, personhood,
the DID subject / controller / delegate triad, and OAuth token-exchange's delegation-versus-impersonation
split) that grounds how the spec talks about personae at all. The load-bearing conclusion is a single
sentence: personhood is a social-utility judgment the protocol deliberately does not compute, and the
reason that is a *conclusion* and not a *cop-out* is that the best minds who tried to compute it concluded
it cannot be computed by a protocol alone. The survey is the receipt for the spec's one-line position.

## Charter: what this document covers

- **In scope:** the proof-of-personhood literature and its verdict; the mechanisms surveyed and why each
  is partial; the identity-layering distinctions (principal / personhood, the DID triad, OAuth
  token-exchange) that ground the spec's persona vocabulary.
- **Out of scope (and where it lives):** the spec's own persona / personhood / weight design, which is
  fixed in Drystone Part 2 §5.2 and §5.6 and is **not re-argued here**; the peer-standing and cooperative-
  form arguments, which live in the peer-standing philosophy note; the substrate and transport prior art,
  which lives in the cairn substrate register.
- **Boundary call:** this is the "why is 'out of scope' a defensible answer" register. It shows the field
  the spec's one-line stance stands on. It does not restate the stance, and it does not extend it.

## The problem the survey circumscribes: Sybil, and Douceur's result

The whole field exists because of one attack. A *Sybil attack* is the strategy of collecting
disproportionate weight in a system by manufacturing many apparent identities that in fact answer to one
actor. John Douceur's result is the anchor: in an open system with no trusted central authority to vouch
for a one-identity-one-entity binding, a sufficiently resourced actor can always present as many distinct
identities, so identity multiplicity cannot be ruled out by the network alone. `[UNVERIFIED, confirm
before publish: Douceur, "The Sybil Attack," IPTPS 2002.]`

This is the exact shape of the problem the spec confronts and declines to solve at the protocol layer:
the system counts key-lineages, and nothing the protocol can do turns "this is one lineage" into "this is
one person." Douceur is why. Proof of personhood is the name of every subsequent attempt to get around
Douceur without reinstalling a central authority.

## The proof-of-personhood lineage

The lineage runs through four load-bearing works, each restating the same bet — that uniqueness among
humans can be anchored to something *physical and offline* that a digital adversary cannot cheaply
duplicate — for a successively harder adversary.

- **Ford & Strauss (2008), pseudonym parties.** The founding mechanism: uniqueness derived from
  one-body-in-one-place-at-one-time. People physically gather at a synchronized event; each attendee can
  obtain exactly one pseudonymous credential because a body cannot be in two rooms at once. No real-world
  identity is disclosed — the guarantee is *one per person*, not *who the person is*. `[UNVERIFIED,
  confirm before publish: Ford & Strauss, "An Offline Foundation for Online Accountable Pseudonyms,"
  2008.]`

- **Borge et al. (2017), the coinage.** This work coined the term *proof of personhood* and framed it as
  an alternative to proof-of-work: accountable pseudonyms yielding one-human-one-unit as the basis for
  democratic weight rather than one-CPU-one-vote. `[UNVERIFIED, confirm before publish: Borge, Kokoris-
  Kogias, Jovanovic, Gasser, Gailly & Ford, "Proof-of-Personhood: Redemocratizing Permissionless
  Cryptocurrencies," IEEE EuroS&P Workshops 2017.]`

- **Siddarth, Ivliev, Siri & Berman (2020), the rigorous review.** "Who Watches the Watchmen? A Review of
  Subjective Approaches for Sybil-Resistance in Proof of Personhood Protocols" (Frontiers in Blockchain,
  2020) is the field-mapping survey. Its thesis, restated rather than quoted: the best proof-of-personhood
  technology does not abstract subjectivity away — it embraces it. The argument runs that proof-of-work
  collapses to one-CPU-one-vote and proof-of-stake to one-dollar-one-vote, both plutocratic, so any
  governance aiming at one-person-one-vote must reliably signal *unique humans*, which requires a
  subjective human substrate rather than a purely objective computation. The substrate the review names is
  *human entropy* — acts like voting, interpreting, being present, and interacting that are cheap to produce
  once but hard to produce twice and hard for a machine to replicate — paired with an objective incentive
  (a crypto-UBI-style reward that makes honest membership worth more than selling one's credential), so the
  scarce human signal is not simply purchased back. Identity in this frame must be both **unique** (no two
  people share one identifier) and **singular** (no one person can obtain more than one). The authors
  disclose a conflict of interest — they built systems in the space — so the review is informed but not
  disinterested. `[UNVERIFIED, confirm before publish: authors, title, venue, year, the COI disclosure, and
  the "human entropy" / objective-incentive framing.]`

- **Adler et al. / MIT–OpenAI (2024), personhood credentials.** The most recent restatement takes the
  same offline-physical-anchor bet and reframes it for an AI adversary: as machines get better at producing
  human-looking behavior at scale, the *personhood credential* becomes the way to signal a real person is
  behind an action without disclosing who. The reported consensus of this work — and of commentary around
  it — is that there is no single ideal proof-of-personhood mechanism, and that robust schemes must combine
  methods. `[UNVERIFIED, confirm before publish: Adler et al., "Personhood Credentials," 2024, and the
  named consensus.]`

The through-line across sixteen years is one goal stated four ways: **one human, one credential, without
identity disclosure** — and a converging verdict that no single mechanism achieves it cleanly.

## The decentralized-identity trilemma

The Siddarth review frames a *decentralized-identity trilemma*: a decentralized identity system is pulled
between three goods — **Sybil-resistance**, **self-sovereignty** (the person, not an authority, controls
the identity), and **privacy** — and cannot maximize all three at once. Verification flag on the status of
the claim itself: as surfaced, the trilemma is sourced to blog-post-level material rather than to a proof,
so it is a *lens* for reasoning about trade-offs, not an established impossibility result. `[UNVERIFIED,
confirm before publish: the trilemma's framing and its provenance; do not present it as a proven theorem.]`

The trilemma matters here because it is the general form of the specific choice the spec makes. A protocol
that tried to compute personhood would have to pick a corner — sacrifice privacy for a strong biometric
binding, or sacrifice Sybil-resistance for self-sovereign anonymity, and so on. The spec's move is to
decline the trilemma at the protocol layer and hand the corner-choice to the Group, whose *recognition*
standard can sit anywhere on the gradient the Group's own function warrants.

## Mechanisms surveyed, and why each is only partial

The review catalogues the mechanisms and, for each, the reason it does not close the problem alone. The
pattern across all of them is the anti-rollup point: every mechanism buys a piece of the property at a
stated cost, and none delivers the whole thing, which is precisely why a protocol cannot pick one and call
personhood solved.

- **Reverse Turing tests (CAPTCHA-family).** Decaying by construction: each generation trains the machines
  meant to be excluded, and human-generated attacks (paid solvers) defeat them regardless.

- **Pseudonym parties.** Strong on the uniqueness-plus-anonymity pairing (physical presence, no identity
  disclosed), but non-permanent — the guarantee is tied to an event and needs periodic re-synchronization —
  and operationally heavy.

- **Web of trust — the PGP model, and why it failed.** The *web of trust* is the GnuPG / PGP design:
  identities are vouched for by certificates other users sign, trust propagating through the graph. The
  survey names its historical failure modes directly. Trust levels were never quantifiable — the model
  wanted scalar, comparable trust and could not produce it. Only first-degree relationships were ever
  *fully* trusted, so trust did not compose transitively the way the design needed. And remote or
  low-infrastructure users could not get their keys signed at all, so the graph excluded exactly the
  people who most needed inclusion. The general lesson the spec inherits: **transitive, scalar trust does
  not work as a substitute for personhood** — you cannot add up "somewhat trusted by several" into "is one
  distinct person." `[UNVERIFIED, confirm before publish: the specific PGP failure modes as attributed.]`

- **The projects.** *Idena* is the survey's standout — FLIP puzzles plus a virtual pseudonym party,
  fully decentralized and privacy-preserving — but with high coordination cost and unproven AI-hardness
  (and a later real-world crisis its own designers' "marketplace for false identities" worry had
  anticipated). *BrightID* (a social-graph analysis approach) and *Proof of Humanity* (a Kleros-adjudicated
  registry with a web-of-trust-and-deposit flavor) are the other reference points. Verification flag: the
  review is a 2020 snapshot and predates the biometric-registry wave and the 2024 reframing, so treat
  project-level detail as dated. `[UNVERIFIED, confirm before publish: project attributions and the
  2020-snapshot staleness.]`

The synthesis the survey itself offers — restated, not quoted — is that the taxonomy is somewhat
artificial because real systems are hybrids, that perfect Sybil-resistance may be the wrong target
(bounded, tolerable penetration is the realistic aim), and that these systems grow only at a "human rate,"
each verifying hundreds-to-thousands of identities in its early years. That human rate is what makes the
wrong-target point bite: the adversary operates at industrial scale — the large platforms purge fake
accounts by the billions each quarter, and by machine learning rather than by any web of hand-signed
vouches. The order-of-magnitude gap between a billions-per-quarter attack and a thousands-per-year verifier
is itself the argument that no hand-built personhood graph closes the problem at scale. `[UNVERIFIED,
confirm before publish: the billions-per-quarter platform figure.]` The practical upshot is that personhood
is expensive, contextual, and never absolute — which is the empirical backing for treating it as a
Group-level utility judgment rather than a protocol guarantee.

## The identity layering that grounds the persona vocabulary

Underneath the personhood question sits a cleaner set of distinctions that the spec's vocabulary depends
on. These are not contested research frontiers; they are settled standards-world layering, and getting
them straight is what lets the spec keep "identity-of-actor" and "weight-counting" from collapsing into
each other.

- **Principal versus personhood — two different questions.** A *principal* is the standards term for the
  rights-holding actor: it covers humans, devices, agents, and services, and it answers *who is acting*. A
  principal can act **on its own behalf** or **on behalf of another** via **delegation**. *Proof of
  personhood* answers a different question entirely: *how many distinct humans are here*, the one-per-human
  weight unit whose property is Sybil-resistance. The clean split: **principal is identity-of-actor and is
  silent on weight; personhood is weight-counting and is silent on identity.** Conflating them is the same
  category error the spec warns against — asking the identity layer to deliver a headcount it structurally
  does not hold.

- **The DID subject / controller / delegate triad.** The decentralized-identifier layer supplies three
  roles that map cleanly onto the actor question. The **subject** is the entity the identifier is about
  (the human). The **controller** is the party that can prove control of the identifier and make changes to
  it. The **delegate** is an entity — typically a device — granted permission to act via a verification
  method. This triad is why "one human, several devices, one standing" is expressible at all: the human is
  the subject, holds control, and delegates acting-permission to devices without any device becoming a
  separate person. `[UNVERIFIED, confirm before publish: the subject / controller / delegate definitions
  against the W3C DID specification.]`

- **OAuth token-exchange: delegation is not impersonation.** The OAuth token-exchange model draws the
  distinction the whole delegation story needs: **delegation** is A acting *on behalf of* B with the
  relationship visible — the resulting authority is marked as "A-for-B," and the "for-B" is legible —
  whereas **impersonation** is A acting *as* B with the relationship erased, so the system cannot tell A
  from B. The spec's persona / device / client story is delegation, never impersonation: a device acts for
  its persona with the lineage visible, which is what keeps a folded multi-device principal one countable
  standing rather than a way to launder extra weight. `[UNVERIFIED, confirm before publish: the delegation-
  versus-impersonation distinction against RFC 8693, OAuth 2.0 Token Exchange.]`

## Why "not the protocol's job" is a conclusion, not an evasion

The spec's position in Part 2 §5.2 — that the protocol counts lineages and the Group decides which
lineages it recognizes as distinct persons — is not a gap the design left unfilled. It is where two decades
of proof-of-personhood research land. Douceur says the network alone cannot rule out identity multiplicity.
Ford & Strauss through the 2024 personhood-credentials work say the only known anchors are physical,
offline, and partial, and that no single mechanism suffices. The trilemma says you cannot have
Sybil-resistance, self-sovereignty, and privacy all maximized, so someone has to choose the trade-off. The
web-of-trust failure says transitive, scalar trust is not a workable substitute. Put together, the field's
verdict is that personhood is a contextual, expensive, standard-dependent judgment — exactly the kind of
call a Group makes at its own confidence and a protocol cannot make for everyone.

The anti-rollup point, stated so a future reader can reconstruct it: the spec's one line rests on this
whole register, and the register is *why* the line is safe to keep short. Delete the register and the line
reads as hand-waving; keep it and the line reads as the disciplined refusal to overclaim that the design's
provenance-versus-utility split requires. The prior art is the receipt.

## What this establishes (and does not)

Establishes that the spec's "personhood is a Group utility judgment, not a protocol computation" stance is
grounded in a real and convergent research literature: that Sybil multiplicity is unremovable by the
network alone (Douceur); that the proof-of-personhood lineage (Ford & Strauss, Borge et al., Siddarth et
al., the 2024 MIT–OpenAI work) has repeatedly concluded no single mechanism delivers one-human-one-unit
cleanly; that the decentralized-identity trilemma forces a trade-off someone must own; that transitive,
scalar web-of-trust failed for stated reasons; and that the principal / personhood split, the DID subject /
controller / delegate triad, and OAuth token-exchange's delegation-versus-impersonation line give the
spec's persona vocabulary a clean standards footing.

Does **not** re-argue or extend the spec's persona, weight, or recognition design (that is fixed in
Drystone Part 2 §5.2 and §5.6 and is cited here, not restated); does **not** endorse any one
proof-of-personhood mechanism for the project to adopt; does **not** treat the decentralized-identity
trilemma as a proven impossibility result; and does **not** certify any attribution above — every author,
paper, venue, year, and RFC reference carries an [UNVERIFIED] flag and must be checked against primary
sources before external use.
