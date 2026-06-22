# The Social Layer: A Graph You Hold, Not a Graph Held About You

author: ISaT / Product Security

date: 2026-06-13

status: early thesis, separate from the group-chat protocol spec (different threat model)

relationship: rides on top of the lineage-groups protocol (THESIS.md, MULTI_DEVICE.md) but is its own opt-in system with its own threat model

covers: the intimate social graph (sections 1-7) and the scale axis of broadcast/topic/civic groups (sections 8-10)

---

## 1. The inversion (the actual thesis)

Today, someone else builds the graph of your life. You don't see it, you don't control it, it's monetized, and it's built whether you consent or not (shadow profiles). The thing to invert is not a feature gap. It is the ownership.

The inversion: **you hold the graph of your life, you see it, you shape it, no one monetizes it, and it is invisible to anyone you have not chosen to show it to.**

The reason this does not already exist is not technical difficulty. It is that there is no business model in a graph you cannot extract from. The absence of an extraction model is not a missing feature here. It is the entire point. The graph can be honest precisely because no one is selling it.

This ties the social layer to the same thesis as the rest of the work: non-extractive infrastructure that serves the person holding it rather than the party operating it.

## 2. The unmet need

Work has mailing lists, org charts, distribution groups. Real life has nothing equivalent. There is no easy way to say "message all of my grandparents' descendants except this one person" to reach the aunts and uncles. There is no "the parents of the kid my kid is staying with tonight" as a durable, nameable, reachable group. The structure of real social life is rich and we have no tool that reflects it, because every attempt to build one (Google+ Circles is the canonical corpse) was built by an extractor who needed the graph for their own purposes, so it felt invasive and died.

The opportunity is a grounded, real-life social graph that exists to serve the person, not to be mined.

## 3. The layer separation (the design move that makes it safe)

Two distinct layers. Only one of them is social. Keeping them separate is what makes the structure safe to hold.

- **The graph layer (structural fact).** Who is connected to whom, derived from real group membership in the underlying protocol. This is what it is. It lives on your own devices, local-first.

- **The label layer (opt-in social).** Names, roles, and visibility riding on top of the graph: "this is my family," "this connection is advertised," "show this group to people two hops out." This layer is entirely opt-in and is where all the identifying, advertisable, human-meaningful information lives.

Most social platforms fuse these: the graph IS the labeled, advertised, visible thing, and that fusion is the invasiveness. Holding them as separate layers means you can have the useful structure without being forced to surface the identifying labels. You can advertise a connection or not, label a group or not, expose a role or not, each independently.

## 4. Safety invariants (build these in from the beginning, not bolted on)

These exist to keep at-risk and marginalized people safe by default. They are not settings to find; they are the default posture.

**S1 - Freeze by default.** Nothing enters your view without your pull. Trust is transitive only as far as you personally walk it. Your social world does not grow because someone else's did. This converts web-of-trust's fatal flaw (automatic transitive trust, clean up the mess after) into "you decide what enters, nothing arrives uninvited." Promote this from a setting to the core invariant.

**S2 - Scoped visibility, not opaque structure.** You cannot offer "visible structure with hidden identities," because graph topology deanonymizes: the shape of someone's connections is close to a fingerprint, and "six people, two connecting to the Henderson family group, in a town of 4,000" re-identifies fast. What you can offer is visibility scoped to what people have consented to be seen by someone at your distance, at the resolution they chose. Honest version of the Kevin-Bacon-distance idea.

**S3 - Asymmetric / quiet membership.** Someone can be in your life without being visible in the graph. Be reachable without being mapped. Be in a group without that group exposing all your other edges. Real relationships have this texture (you can be close to someone who does not know who else you are close to); a graph that flattens it into mutual visibility is the invasive version even with no company involved. This is the unsolved one and it is where the inside-adversary problem lives.

**S4 - Multi-identity as a fact of life, not an edge case.** A person legitimately presents differently across contexts and may need hard separation between them (the estranged relative, the person who left a community, the custody situation, the marginalized person who cannot let context A see context B). Multiple identities under a person's control, with no forced linkage between them, has to be a first-class assumption baked in early, not a feature retrofitted.

**S5 - Private must be structural, not a runtime gate (the Twitter Circles lesson).** A visibility "clause" attached to content that physically lives on a public/broadcast plane is not privacy; it is a promise the platform can break the moment its ranking or indexing logic changes. **Twitter Circles** is the billion-user proof: posts gated to ~150 people sat on the public timeline plane with a gating clause, and when the ranking logic was overhauled they **leaked into strangers' "For You" feeds** — the feature was killed within a year. Communities (a public sub-forum) failed similarly; Group DMs/XChat survived by being a hard binary (fully public, or fully E2EE-isolated). The invariant: **never overlay a semi-private gate on a public plane — make private data structurally private (off-repo / encrypted / never broadcast), or it will leak.** This is the social-graph statement of the lineage-groups "structural, not runtime, enforcement" rule and of "content is born into a visibility regime and cannot silently change it" (see `../crystallized/principles.md`; lineage-group-model V3). On an AT-Proto-style stack the corollary is concrete: keep private records *off the public repo/firehose* (a private namespace / off-repo store) rather than gating a published record. Source + corroboration: `../seeds/transcripts/raw/croft-atproto-sovereign-appview-open-social-dialogue-2026-06-22.md` (fact-checked); `../research/atproto-sovereign-appview-club.md`; COHESION §29.

## 5. The adversary model (why this layer is its own threat model)

The group-chat protocol's threat model is mostly about adversaries outside the conversation. This layer adds adversaries the protocol does not:

- **The scanner / fraudster.** The same structural map that helps a grandmother helps a scammer. Calibration: this exposes roughly what a phone contact list already exposes, but makes it useful, and danger scales with usefulness, so safeguards scale with it too. The safeguards are not a tax on the feature; they decide which user you built it for.

- **The inside adversary.** The person who vouched for you can see you. The family group that contains you knows you are in it. Pruning lets you cut forward but does not un-show what was shown. S3 (quiet membership) and S4 (multi-identity) are the defenses, and they must be present before this is usable by anyone at risk.

- **Deanonymization by topology.** Addressed by S2. The structure itself leaks; only consented, scoped sharing is safe.

## 6. The contention (the trade-off to hold)

It has to be composable and smooth enough to contain the subtle case that respects safety and humanity, while also being usable enough to actually build up meaningful adoption. Too safe-but-clunky and no one uses it, so it protects no one. Too smooth-but-exposing and it becomes the scanner's tool. The honest trade-offs live in this tension and should be made deliberately, case by case, not resolved by pretending one side does not exist.

The messaging is the point as much as the mechanism: let people build their own social world for themselves, at their own pace and comfort, with safety for at-risk people built in from the start rather than discovered after harm.

## 7. Open design questions (for the dream/spec phase)

- Per-connection advertise controls: allow-list / deny-list, per-group, with sensible non-exposing defaults.

- Metadata on top (names, roles, "is family," default reach) as opt-in skin, without constraining use cases.

- How quiet membership (S3) is actually expressed in the protocol: can a leaf be in a group for reachability while withholding its other edges from that group's view?

- How multi-identity (S4) composes with the lineage model: distinct lineages with deliberately no provable linkage, versus one lineage with scoped facets.

- An early concrete target: a bare-bones "Facebook 2010" experience built on these primitives, as the usability proving ground.
