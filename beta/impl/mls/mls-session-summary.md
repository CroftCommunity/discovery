# Session Summary: MLS Study for Drystone

`Status: session notes`

`Date context: working session grounding MLS (RFC 9420 / 9750) against the Drystone design`

`Companion docs produced or updated this session: mls-overview-and-terms.md, mls-hardcases-and-posture.md, 12-side-histories-and-threading.md, and a new practice added to 11-doc-method.md`

This summarizes what we worked through, in the order we built it, so the reasoning chain is recoverable. It records both the conclusions and the corrections made along the way, because several early framings were tightened or retracted and the corrections are part of the value.

---

## 1. What we set out to do

Pin the loose MLS vocabulary (client, leaf, device, and related terms) to concrete normative definitions, then build up through the architecture to the specific places MLS is likely to trip up a center-free design like Drystone. The work stayed grounded in the two RFCs directly, with the peer-reviewed analysis of the standard used only for corroboration.

---

## 2. Vocabulary pinned (detail in mls-overview-and-terms.md)

The core confusion was leaf versus client versus device. Resolved as:

- Client: the atomic unit, defined by the keys it holds. Not the user, not the device.

- Member: a client viewed relative to a group it has joined.

- Leaf: a position in a group's ratchet tree. LeafNode: the signed, group-scoped object at that position, which holds keys. A leaf is not "a key," and the LeafNode is distinct from the stable client identity beneath it.

- Device: not an MLS concept at all. Lives only at the application layer. MLS fixes one cardinality (one client, one leaf per group) and leaves user-to-client and device-to-client mapping to the application.

- Identity, keys, credential: three separate things. The credential binds an identity to a signature key, which is precisely what lets one change while the other holds (enabling key rotation and multi-device). The user's initial instinct that identity and keys were the same was the thing corrected here.

The user's original mental model was largely correct and needed two tightenings: leaf names a position not a key, and identity is separable from keys via the credential.

---

## 3. Architecture walked

- MLS is a group key agreement protocol, not a messaging app. Output is a shared group secret.

- Ratchet tree: members at the leaves; you know a node's secret iff you sit beneath it; re-keying costs log(N) by encrypting to copath resolutions up the tree.

- Blank nodes (keyless holes from removal) and unmerged leaves (freshly-added members not yet knowing an ancestor's key) both degrade log(N) toward N, and heal only when members commit. Tree efficiency is a function of recent commit activity, not a fixed guarantee.

- Two abstract services: the Authentication Service (highly trusted for identity) and the Delivery Service (untrusted for confidentiality, but a metadata and availability adversary). The DS's hidden essential job is imposing a single order on the linear epoch chain.

---

## 4. The center-free trip hazards (detail in mls-hardcases-and-posture.md)

The unifying insight: MLS provides a local cryptographic check nearly everywhere it needs agreement, and assumes a coordinator supplies the global agreement. Specific hazards:

- The linear epoch chain cannot represent a fork. The transcript hash is a hash-linked chain of exactly one commit sequence.

- The confirmation tag proves agreement with the committer, not global agreement among all members. Detecting whole-group consistency needs out-of-band comparison of the epoch_authenticator.

- The committer role is a rotating, contestable sequencer. Center-free does not eliminate the coordinator function; it makes it rotate and contestable.

- The per-epoch membership key means a member who missed one commit falls off the key chain and must rejoin via external commit or re-add, not just "catch up."

A second sweep of the RFC 9750 security catalogue (§8) and untouched RFC 9420 sections surfaced further hazards, all now sectioned in the hardcases doc:

- External-join recovery (hardcases §5). A rejoining client trusts the GroupInfo it is served; a stale GroupInfo can defeat post-compromise security or block rejoin. In a center-free mesh any peer can serve it. Resolved in Drystone because GroupInfo is a claim corroborated against the governance chain, not an authority, with the monotonic fold self-forking a bad asserter and the cosign threshold quantifying attack cost per posture.

- Insider replay and nonce reuse on restore (hardcases §6). MLS provides no insider-replay protection and carries a reuse_guard for state reverts. Both are the same shape (old bytes re-entering the live stream) and both are isolated by out-of-band history convergence, provided live epoch secrets are never restored in place.

- ReInit non-atomicity (hardcases §9). Committing a ReInit freezes the old group instantly but re-forming the new one is a separate step, so a committer that strands mid-operation leaves the group frozen. This is the grounded form of the escalation-window question; candidate resolution is the governance chain recording re-plant intent before the freeze.

- Concept alignment (hardcases §10). A map of where Drystone concepts have native MLS representations (direct, partial, Drystone-only, or underused), so overlapping ideas fold rather than fight.

---

## 5. How Drystone's design reframed the hazards

The user's uploaded docs (Part 1 principles, delivery architecture, provenance) reframed several hazards from problems into designed behavior:

- Forks are a first-class escalation signal, not a failure. Part 1 Section 2.5 derives fork-not-verdict from the CALM boundary applied to governance: a total social order over concurrent non-monotonic operations is itself non-monotonic, so it has no coordination-free determinate resolution. The residue is not uncomputable; it is not a computation at all, because the question is a value, not a fact.

- The MLS linear chain is a coordination mechanism, so leaning on it for governance would take the coordinate horn the razor forbids. The chain is fine for the (monotonic) dataplane and disqualified for the (non-monotonic) governance residue. This is the chain doing its job, not a limitation.

- The MLS group is subordinate, disposable key-distribution infrastructure. A conversation ("besties") persists across an arbitrary sequence of MLS groups. Fork, heal, and re-plant are the same primitive at different arity: read authoritative membership, plant fresh group(s), atomically repoint conversation pointer(s).

Corrections made to earlier framings in this session:

- Retracted the "FS versus durable history" tension as originally stated. Forward secrecy is defined against ciphertext retention; retaining sealed bytes is FS-safe. The real friction is key retention under reordering (the documented DMLS problem) and the persistently-offline member.

- Clarified that MLS has no self-deleting-message feature. Its deletion schedule operates on keys to produce forward secrecy; disappearing messages are an application-layer content policy that MLS neither implements nor conflicts with.

---

## 6. New design conclusions reached this session

- The escalation set generalizes. Part 1 Section 2.5 covers contradiction (too many valid claims). Role-delegation gridlock is a second, distinct member: under-determination (a needed role vacant, no agreed grant). Same terminus (fork), different detection. Proposed for explicit naming in Part 2 Section 7.6, pending confirmation against that text.

- Rights versus roles resolves the hardest hygiene case. Pruning a stale device for key hygiene touches resources and roles, never the rights floor. The only-device-holding-a-needed-role case degrades capacity without touching standing, and adapts by re-delegation, running role-less, or forking. This is explicitly a social-utility judgment, explicitly not a technical solution, and that is correct by design.

- Staleness, not event-count, is the honest removal trigger. Per-member key age is a deterministic, coordinator-free, cross-corroborable predicate over the shared tree. Event-count measures activity, is loosest where risk is highest, and is acceptable only as a secondary poke.

- The codifiable/non-codifiable boundary. The staleness predicate and thresholds are codifiable into the governance chain (provenance). The decision to act on a firing, and how, stays human (utility). Codifying the action along with the predicate re-installs a small center and must be avoided; the response is a governed per-scope dial with auto-prune available but not default.

- Drystone's re-plant family has native MLS representation. ReInit (close and re-form over the same members) and branching (a subset splits off, original survives) are the MLS-native shapes of the re-key and fork arities, linked by resumption PSK. This is external validation that the design converged on the same shape as the standard, and it lowers implementation risk. It also surfaced the ReInit non-atomicity hazard.

- Threading and side histories are three tiers, not one (doc 12). A subid into the existing dataplane tree (tier 1, free, the common case), a separate-but-inherited side history under the same keys (tier 2, candidate, the vacation/guestbook case), or a real subgroup branch with its own entitlement (tier 3, expensive, only when access must narrow). The selector is whether the side history needs different keys or just different structure.

- The concept-alignment pass found one likely reinvention. The MLS epoch_authenticator is a native per-epoch value for out-of-band whole-group consistency comparison, which Drystone may be duplicating with a separately-built check; flagged as a candidate to fold rather than parallel-build, pending confirmation of whether same-MLS-state and same-governance-state are the same question.

---

## 7. DMLS/FREEK assessment

Pulled and assessed draft-kohbrok-mls-decentralized-mls-00 (March 2025), built on FREEK (eprint 2023/394). It targets exactly the key-retention seam. Verdict: preliminary, not a dependency. Its introduction and security sections are empty TODOs, and its consolidation procedure assumes two coordinating servers, the opposite of Drystone's premise. Two ideas worth adopting independently: content-derived epoch identifiers, and the PPRF approach to forward-secure retained init secrets. Track for maturation; do not adopt its consolidation.

---

## 8. Open items carried forward

The hardcases doc holds these in full; condensed here for the session record.

- Confirm the under-determination generalization against Part 2 Section 7.6.

- The KeyPackage-exhaustion seating trilemma: fresh-FS, offline-seatability, and avoiding the external-join path cannot all hold; likely a posture dial, but the external-join corner's safety depends on the §5 GroupInfo-validation defense. Flagged to talk out.

- The §5 residual: whether a node far behind on the governance chain can always distinguish a recent-but-superseded GroupInfo from a current one.

- The §6 residual: whether the recovery design ever restores live epoch secrets in place versus always re-planting fresh.

- The ReInit non-atomicity resolution (§9): whether the governance chain records re-plant intent before the freeze, letting any member complete a stranded re-plant.

- The re-plant seating default: Welcome-seating versus external-commit-seating, now reweighted by §5 (the external path carries a PCS hazard the Welcome path does not).

- The epoch_authenticator overlap (§10 underused): whether Drystone's consistency check can use it directly rather than a separate build.

- The resumption-PSK cross-group linking (§10 underused): whether re-plant could use its cross-group PCS-carrying property to link healing across a persona's groups.

- The epoch-number metadata leak versus re-plant frequency.

- The communal-namespace key construction under membership change (Part 1 Section 2.3; Part 2 Section 5.10).

- Tenure-under-re-key: whether the survivor/re-key path can strand a persona's tenure.

- Whether tier-2 side histories (doc 12) deserve a first-class named construct or are an emergent use of the existing data model.

- Re-evaluate DMLS/FREEK once it matures.

---

## 9. Primary sources used

- RFC 9420, The Messaging Layer Security (MLS) Protocol, July 2023.

- RFC 9750, The Messaging Layer Security (MLS) Architecture, April 2025.

- draft-kohbrok-mls-decentralized-mls-00, Decentralized Messaging Layer Security, March 2025.

- FREEK: Alwen, Mularczyk, Tselekounis, Fork-Resilient Continuous Group Key Agreement, eprint 2023/394.

- draft-tian-quic-quicmls-00, Securing QUIC with MLS (informative, for the ReInit replay-immunity point only).

- ETK: External-Operations TreeKEM, eprint 2025/229 (noted as the relevant analysis of external operations; not read in depth).

- Peer-reviewed analysis of the MLS standard, used for corroboration of the key-schedule and transcript-hash mechanics only.

Note on citation discipline: several mechanics in the companion docs (transcript hash construction, confirmation tag semantics, key-schedule derivation) were corroborated against the analysis literature rather than quoted from the RFC section directly. Before any of these docs is frozen into normative Drystone text, the load-bearing mechanics should be checked against the specific RFC 9420 section numbers (Section 7 for the tree, Section 8 for the key schedule, Section 9 for the secret tree and deletion schedule, Section 12 for proposals and commits).
