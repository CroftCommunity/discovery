# Drystone revision changelog

`Scope: changes applied to Part 1 (Reasoning Underpinnings) and Part 2 (The Certifiable Design) from this discussion's conclusions.`

`Revision posture: full pass — reworked wherever the discussion's conclusions touch, including framing and intros. Citation posture: inline [confirm before publish] flags retained AND a new consolidated prior-art section (Part 2 Appendix C) added.`

This changelog is organized by theme rather than by line, because several changes touch multiple sections. Each entry states what changed, where, and why.

---

## document-pass-5 (2026-07-06): transport/identity/encryption integration + iroh 1.0 pass + RFC 9420 §16.4 correction

**What changed, in one line.** Part 2 §6 was expanded from a thin three-subsection stub into a full
transport/identity/encryption section (§6.1–6.8, +516 lines), flags were resolved against the released
iroh 1.0, and a real error in the RFC 9420 §16.4 metadata analysis was corrected. **Part 1 unchanged**
(its only iroh reference is a version-agnostic pointer to §10).

**Part 2 §6 (was "Transport", 3 subsections; now "Transport, Identity Planes, and the Encryption
Stack", §6.1–6.8):** the old §6.1–6.3 content is reorganized, not lost, into: §6.1 two identity planes
(peer = iroh EndpointId authenticating a channel; group = MLS leaf authorizing an actor in a scope; the
seam that reachability and membership fail independently); §6.2 the two-layer encryption stack (Layer A
iroh QUIC/TLS 1.3 hop-by-hop; Layer B MLS PrivateMessage end-to-end; why both, what neither gives);
§6.3 connection establishment + interaction tiers; §6.4 discovery (DNS/Pkarr soft-center,
Pkarr-on-mainline-DHT, mDNS); §6.5 delivery (direct, relay-assisted, the meer as additive offline
durability); §6.6 the gossip overlay (HyParView + PlumTree, and what it does/does not guarantee); §6.7
the two deployment modes; §6.8 real-time media. Two figures added: `drystone-exposure.svg` (the
trust/exposure map) and `drystone-catchup-flow.svg` (returning-member governance catch-up).

**RFC 9420 §16.4 correction (a real error caught, not a polish).** The prior draft claimed a
`PrivateMessage` exposes a per-sender **generation counter** and that a gap reveals a missed message to
an observer. Verified against §16.4 verbatim: **that claim was wrong and is removed.** `generation` lives
inside the AEAD-encrypted `SenderData` (§6.3.2), and §16.3 exists precisely to hide the sender, so it is
not visible in the framing; the "gap reveals non-delivery" assertion has no home in the RFC. Group ID (a)
and epoch (b) are verified but **rescoped**: they leak through the *cleartext `PrivateMessage` header* and
are unprotected against the *DS specifically*, not "any observer of the ciphertext." Membership inference
(d) verified at full strength with its RFC-named mitigation (pseudonymous credentials, frequent key
rotation), which dovetails with the §2.3 multiple-persona argument.

**iroh 1.0 differentiation pass.** iroh core reached 1.0 (wire + API stable; consistent with the
FACTCHECK SoT, iroh `1.0.0`). Resolved to Verified where 1.0 is load-bearing: EndpointId = Ed25519 public
key as TLS identity; the post-handshake seam (remote identity known only after the authenticated
handshake, `Connection::remote_id()` infallible); direct-first hole-punch with stateless content-blind
relay; `subscribe(TopicId, bootstrap_peers)`. Correctly **kept `[confirm]`, rescoped** to separately
versioned pre-1.0 crates: `iroh-gossip` internals (event surface, view sizes, PRUNE/GRAFT) and the
discovery crates (`iroh-mainline-address-lookup`, `iroh-mdns-address-lookup`); the old "is mDNS mature?"
flag resolved (it is a shipped crate now). §6 balance: 15 Verified / 12 `[confirm]`, each remaining flag
confirmed legitimately open (gossip-crate internals, Pkarr record spec, RFC 8446 traffic-analysis, RFC
9420 §16.9). **Still `[confirm before publish]`:** pin the three iroh subcrate versions in a build
manifest; pull the Pkarr spec, RFC 8446 §5.4, and RFC 9420 §16.9; lift the canonical "Barnes et al., July
2023" running-header line from rfc-editor.org for the §16.4 citation (the extraction came from a PDF
carrying Datatracker chrome).

**Companion added:** `bounded-contexts-and-vocabulary.md` (spec-layer design/language note): the DDD
bounded-context rationale for why terms like *peer* legitimately carry different meanings in different
parts of the design, and the test for when overload is fine vs. when to rename (as *peer* → *persona*
was). Process artifacts (the integration diff and summary) frozen to
`../../alpha/seeds/drystone-transport-integration/`; the messaging-layer research prompt filed to
`../../alpha/seeds/generated-prompts/`; the `drystone-transport-section.md` draft is superseded by the
merged §6 and not kept.

---

## document-pass-4 (2026-07-06): persona/peer vocabulary migration + identity model

**What changed, in one line.** The word *peer* was sharpened to name only the **relation** at an edge;
the **entity** a human is manifested as, a key pair by which a system represents a person, is now
**persona** (plural **personae**, Latin form used strictly). A new companion `persona-definition.md`
carries the vocabulary of record, and Part 2 gains Appendix D (the term lattice and invariants) plus
identity-model sections.

**The load-bearing distinction (verbatim-consistent across all docs):** *peer is the relation, persona is
the entity.* Persona is defined in **both** parts at each part's register, consistency of concept without
duplication of text: Part 1 (the "why") defines persona as the commitment form (the human layer's
manifestation, the entity rights and weight attach to, standing in peer relation), with a labeled
definition note added in §1 *before* the term's first load-bearing use; Part 2 (the "certifiable design")
defines it as the mechanical form (principal by virtue of a key pair, one rooting key pair, lineage fold,
flat weight, in §5.2). Neither competes to be the source of truth; the appendix defers to §5 if they ever
differ.

**Part 2 additions:** Appendix D (term lattice + six invariants, the vocabulary of record, before
References); §4.5 (multi-client fold: client-count and device-count ≠ persona-count); §5.2 (principal /
client / persona identity model); §5.5 (role, capability, PrincipalSet, delegation, the governance and
data-access planes). A pointer from §5.2 to Appendix D.

**The etymological congruence (ideological alignment, not a Latin lesson):** *persona* is per + sonare,
"to sound through", the mask an actor's voice sounds through; and **voice** is one of the three
fundamental rights (voice, tenure, exit). The word's root names the very right it carries: the thing that
manifests a human is, at the root, the thing through which the human's voice sounds. Stated in
`persona-definition.md` Note 2 and as a tight parenthetical in Part 1 §3 where voice is first made a
right. The prior seam-note that hedged "the root doesn't imply the mechanics" was corrected: the
congruence is *exact* for the voice right, while the other properties (rotation, lineage, multiplicity,
flat weight) remain spec-assigned, not etymological, so the alignment is claimed only where it is real.

**Persona multiplicity:** a human may hold multiple personae across multiple systems (a work graph, a
volleyball team, a school district), same key pair or one-per, a personal-utility choice; the persona is
not a proof-of-personhood and does not attempt one.

**Companion filed:** `persona-definition.md` (vocabulary of record) added to the spec directory. Process
artifacts (the delta, migration plan, session summary, per-part diffs, and the peer-inventory worksheet)
are frozen under `../../alpha/seeds/drystone-persona-migration/`, not in the spec tree.

---

## document-pass-3 (2026-07-06): voice/field-integrity bridge, RFC 9420 §16.4 reconciliation

**What changed, in one line.** A new Part 1 §2.6 links the `voice` right to a field-integrity precondition (the joint between the protocol and the companion argument about who may own the substrate), and the RFC 9420 §16.4 metadata claims were reconciled against the spec's verbatim text, correcting one claim.

**Part 1 — new §2.6 (voice requires field-integrity):** a bridging subsection after §2.5 (P-Durable-Enablement). It states one dependency: `voice` (§2.3 P-Peer-Equality) is a right only if the field a peer asserts into is not authored by a party whose interest the peer does not share and cannot see (call it field-integrity). It names three properties of legitimate ordering (peer-governed, legible, exitable), maps each to an existing mechanism (§2.2 no-silent-mutation, §2.4 fork, Part 2 §7.4 silence-is-not-currency, §7.6 fork), and marks the endemic-ordering point as a `[tension]` (the center-free field is not unshaped; it removes the structurally-adverse curator and returns ordering to peer governance). Adds **no new principle and no new wire obligation**; explicitly not a fifth peer-property. The empirical and ownership-form grounding is external, in the separately-maintained companion set (now filed under `../governance/` and `../activism/`), never a spec dependency for a mechanism.

**Part 2 — §7.4 back-reference + RFC 9420 correction.** §7.4 gained one sentence tying its silence-is-not-currency rule to §2.6's legibility property (refusing to render silence or a stale view as currency is the protocol declining to present a partial slice as the whole field). The RFC 9420 §16.4 metadata-exposure claims were checked against the RFC's verbatim text: group ID and epoch are cleartext in the `PrivateMessage` header and unprotected against the DS (verified, scoped to the DS rather than a generic ciphertext observer); group membership is inferable (verified as written); the **per-sender generation counter claim was corrected** — `generation` lives inside the AEAD-encrypted `SenderData`, so it is *not* visible in the framing to an observer, and "a gap reveals a missed message to an observer" is unsourced in RFC 9420 and was removed/narrowed.

**open-items.md:** two notes added — the §2.6 addition (recorded so it is not lost; no ruling needed) and a companion-narrative-tracked-separately note carrying one time-sensitive item: the **Project Mercury** allegations (Nov 2025 litigation filing, hearing set Jan 26 2026) surfaced no post-hearing ruling as of late June 2026, so PACER must be pulled directly before the companion is published externally. Does not gate the spec.

**Not filed:** the standalone `drystone-part1-voice-bridge.md` draft is superseded by the merged §2.6 and kept only as dropoff scratch.

---

## document-pass-2 (2026-07-06): model corrections, em-dash removal, open-items companion

**What changed, in one line.** Seven model corrections from a structured review pass, three citation fixes, and a complete em-dash removal (562 em-dashes → 0 across both parts). A new companion file `open-items.md` was added.

**Model corrections applied:**

- **A2 (peer/member/lineage).** "Group-recognized peer" removed. A group recognizes *members* (clients, in the MLS sense); lineage resolves a group's member-clients to one peer. Weight and threshold language now reads "one per distinct peer (by lineage)." The personhood judgment (whether that peer is one human) is kept separate as the group's contextual call.

- **A3/A4/A5 (meer and DS).** Meer is a retained colloquialism for a blind store-and-forward node (the earlier model where it was a full group member with no local history is abandoned). Verified against RFC 9750: a central Delivery Service is not required; clients can communicate directly peer-to-peer. The meer is optional offline-persistence (MLS store-and-forward layer); the iroh relay is separate (transport-layer NAT forwarder). §10.5 now says Drystone removes the **ordering** center.

- **B1 (rights floor).** `share` dropped as a right. The floor is three: tenure, voice, exit. Where share has substance it is ownership of a Meadowcap communal namespace (data layer). The one remaining open check is tenure-under-rekey (Appendix B, test shape written in).

- **B2 (group-principal).** Open seam noted: the Meadowcap/MLS alignment (can group-associated assets fork/merge sanely?) is the decisive next step before committing the communal-namespace construction. Rotation scheme and primary-vs-secondary question recorded as unworked.

- **B5 (escalation tolerance).** "Value" → "default value"; spec deliberately declines to pick the default; knob granularity and shipped defaults are deployment tuning.

- **B6 (capability mechanism).** Track A (Meadowcap) vocabulary preserved deliberately; Keyhive (Track B) recorded as preferred on revocation immediacy; deferral correct pending needs definition.

- **B8 (grounds of authority).** Position recorded: rights floor is variety-enabling → system-sustaining; rights-negation is self-amplifying toward collapse; human-to-peer binding (mint-and-bind) is contextual (family = simpler/higher-trust, large disconnected groups = harder).

**Citation fixes (D1–D3):** RFC 9750 reference corrected §3.5→§6.4; Matrix "trusting servers" line softened to paraphrase; duplicated Appendix line removed.

**Em-dash removal:** 219 (Part 1) + 343 (Part 2) = 562 em-dashes removed. Replacements by grammatical role: bullet labels/headings → colons, appositives → commas, independent-clause joins → semicolons, parentheticals → commas or parentheses.

**New companion file:** `open-items.md` — read-and-decide ledger (settled-this-round + genuinely-open items + deferred-no-action-needed).

---

## 0000000. Timestamp-ordering claim made architecture-relative; Matrix tiebreak confirmed (newest, this round)

**What changed, in one line.** Two refinements to the timestamp/governance-ordering material: (1) the Matrix State-Resolution-v2 timestamp tiebreak is now confirmed verbatim against primary sources and its marker cleared; (2) the claim that the authority-deciding path must exclude forgeable ordering inputs is reframed as **architecture-relative, not a universal law** — it is forced by Drystone's specific properties, and a system like Matrix can rationally tolerate what Drystone cannot.

**The Matrix confirmation.** State-Resolution-v2 sorts events by `power_level`, then `origin_server_ts` (the sender's claimed wall-clock), then a lexicographic `event_id` fallback — so a forgeable timestamp is a live discriminator in the governance-ordering path. The Matrix authors state the cost themselves in the MSC1442 discussion: timestamps are the tiebreak whenever the auth DAG implies no ordering, this is the **common case**, the system is "basically trusting servers not to lie about the time," and damage is mitigated only after the fact by an admin starting a new epoch. Confirmed against the Matrix.org stateres-v2 description and the MSC1442 PR thread; the §7.3.1 marker for this specific claim is cleared.

**The architecture-relativity refinement (the reviewer's point).** The earlier text risked implying "the authority path must never depend on a forgeable value" as a universal. It is not universal — it follows from two properties of Drystone that are two sides of one coin:

- **No authority tier above the peer.** Every node holds its own canonical view, so a corrupted governance resolution **cannot be overridden in place** — there is no higher authority to do it.

- **Fork as the only remedy.** Drystone's sole correction for a bad governance outcome is exit (the fork, §7.6), not in-place override.

A system *with* an authority tier (Matrix's homeservers and admins) can correct a bad resolution cheaply and in place ("start a new epoch," de-op, ban), and can therefore rationally tolerate a routinely-gameable ordering input. Drystone cannot: with the fork as its only remedy, a forgeable input would mean a heavyweight schism every time it is gamed. So the constraint is "a system whose nodes each hold their own canonical view and whose only remedy is exit rather than in-place override must exclude forgeable ordering inputs" — which is Drystone's case and that of any design sharing those two properties, **not** a claim about systems we are not building. The Matrix contrast is now explicitly a difference of **architecture and available remedy, not of correctness**.

**Where.**

- **Part 1 §2.0.1** — the "right tool for the job" paragraph extended with the architecture-relativity (no-tier + fork-as-only-remedy as one coin; scope bounded to systems sharing those properties; Matrix's tolerance named as rational for its architecture).

- **Part 2 §7.3.1** — the Matrix contrast block: tiebreak fields pinned, the authors' admission quoted, the marker cleared, and "this is a difference of architecture, not of correctness" added with the two-sides-of-one-coin framing.

**Why.** Stating the constraint as universal would be an overclaim Drystone can't back across the whole design space, and would unfairly read as "Matrix got it wrong" when Matrix made a defensible choice for its architecture. The honest claim is narrower and stronger: the exclusion is *forced* for our architecture, by the cost of our only remedy, and we needn't speak for systems we aren't building.

---

## 000000. Equality model corrected — two equalities, two inequalities; rights are always equal (this round)

**What changed, in one line.** The "three orthogonal axes (resource / right / weight)" framing carried a conceptual error: it treated **rights** as something that could be unequal ("one peer is an admin by a delegated role, another is not"), which conflates a *right* with a *role*. The model is now stated correctly as **four properties: two necessarily equal (rights, weight) and two legitimately unequal (resources, roles)** — and **capability** is demoted from a peer-property to the data-access mechanism beneath roles.

**The correction.** A right is what a principal *inherently holds* and is **always equal and unremovable** — the proof being that exit/fork survives even when every role is stripped and even when a quorum captures the group (participation persists as the standing to leave with your state). Being an admin is a **role**, not a right; it is granted, scoped, and revocable. So the inequality the old text attributed to "rights" was always a role inequality. Rights do not vary; roles do.

**Weight is equal "by necessity," not by separate decree.** The prior text called weight "the axis the design clamps," which made flat weight sound like an arbitrary designer's choice. It is not chosen — it *follows from* equal rights: if standing-to-participate is equal (the right), then standing-to-be-counted is equal (the weight). Weight is the governance image of the rights floor. This is now stated as a derivation, not a stipulation.

**Capability is not a peer-property.** Under "three axes plus two more nouns," capability (the Meadowcap data-access grant) sat awkwardly alongside resource/right/weight. It is now placed correctly: it is the **mechanism a role operates through** (a role may carry the authority to issue capabilities), a data-plane token (§7.1, §10.4) one level *below* the question of how peers differ. The peer-equality model is four-part; capability is underneath roles, not beside resources.

**The canonical sentence, everywhere:** *peers are equal in rights and (by necessity) weight, and unequal in resources and revocable roles.*

**Where.**

- **Part 2 §5 intro + §5.0** rewritten from "three orthogonal axes, two layers" to "two equalities, two inequalities, two layers." The four properties are stated as two equal (right, weight) and two unequal (resource, role); capability is explicitly placed beneath roles, not as a fifth property; weight is derived from rights "by necessity."

- **Part 2 §5.3** heading changed from "Rights: inherent to the peer, or delegated-as-role" to "Rights: the inherent, equal floor — never delegated, never unequal," and the opening corrected so a right is *only* inherent (the delegated thing is a role).

- **Part 2 §5.6** weight intro changed from "Weight is the third axis... the axis the spec deliberately clamps" to "the second of the two equalities... equal by necessity, not by separate decree," derived from the rights floor.

- **Part 1 §2.3** heading changed from "equal in weight; unequal in resources and (delegated) rights" to "equal in rights and (by necessity) weight; unequal in resources and revocable roles." Bullets restructured into two equalities + two inequalities; the "Rights are allowed to be unequal" line removed; the "Reasoning" paragraph updated so weight reads as the governance image of the right rather than a third clamped thing; the vocabulary note updated so capability sits beneath roles.

**Why.** The reviewer (Chase) caught that the axis framing still confused roles and rights: it named "right" as an axis and then illustrated inequality with a role (admin). The four-part model resolves it cleanly — rights and weight are the two equalities, resources and roles the two inequalities — and the relabel forces capability into its correct place as a data-access mechanism rather than a peer-property, which is *more* coherent than the prior "three axes + two planes." An axis that cannot vary (an always-equal "right") was never really an axis; dropping the axis metaphor removes that contradiction.

---

## 00000. Peer-vs-personhood keystone, the device-group dial, and the dial-discipline principle (this round)

**What changed, in one line.** Three connected refinements: (1) the **keystone distinction** that a peer is a *provenance object* and personhood is a *social judgment* — different kinds of thing, the binding an adjudication not a lookup; (2) grounding "cryptographic trust always bottoms out in social judgment" with **three irrefutable examples** (TLS CA, PGP web-of-trust, SSH TOFU); (3) correcting an overstated line so **even one's own device group is a dial, not a fixed high-trust truth**, plus a new **dial-discipline principle** (80/20 defaults, the 20% must stay representable).

**The keystone (Part 2 §5.2).** A peer is technically representable — the root of a cryptographic-provenance lineage the protocol can verify and count. Personhood — whether that lineage is a distinct human — has no technical representation, because it was never the protocol's to hold. So the binding (*this lineage is one person*) is an **adjudication, not a lookup**, which is exactly why it is a seat of social-utility judgment and why "peer" and "personhood" are distinct words rather than synonyms. Collapsing them is the same category error as "the network can certify truth." This makes explicit the *why* under the §5.6 contextual-personhood material.

**The three examples (Part 2 §5.6).** Replaced the thin single-CA mention with the full case that this is not a Drystone quirk: TLS/X.509 (chain airtight, anchored to the CA's social/institutional reputation; trust-store is a social artifact), PGP web-of-trust (validity by transitivity of human attestations, user-set trust thresholds, and explicitly *not* designed to verify personhood or resist Sybil — the canonical decentralized system drawing Drystone's exact line), and SSH TOFU (cryptography guarantees continuity, the human makes the founding "is this the right host" decision). The throughline: crypto proves *this key signed this / same key as before / chain valid*; it never proves *right person / distinct human / trustworthy*. The TLS-CA **seam is marked**: a CA is centralized (what Drystone refuses); the analogy holds at "certainty terminates in a social decision about an attester," and Drystone distributes and makes-exitable (forkable valuation edge) what PKI centralizes.

**The device-group dial correction (Part 1 §2.3).** The prior text asserted "your own device pool is a high-trust composition edge that needs **no** Byzantine defense." Corrected: it needs little Byzantine defense **by default**, but even this is a dial — a person whose threat model includes a device being seized or coerced (activist, journalist, unsafe household) may rationally want Byzantine suspicion within their own device group. Hardcoding "device group = trusted" prunes that case, the variety failure in miniature, biting hardest when stakes are highest. *(This corrects a line that was in the document, not merely something said in conversation.)*

**The dial-discipline principle (Part 1 §2.3, new).** Once everything is a dial, *if everything is a dial nothing is usable.* The discipline: default the common case hard (most groups never touch it), keep the uncommon case representable without ceremony, and never let a default calcify into a structural assumption that forecloses the alternative. Variety is preserved by the 20% being **expressible, not foregrounded** — explicitly **not a footnote**, because a default that forecloses the minority's needed setting quietly becomes the very center the design refuses.

**Why.** The peer/personhood distinction is the conceptual keystone the whole resource/capability/role/right/weight vocabulary rests on, and it was implicit; stating it makes the identity-layer instance of the provenance/utility split explicit. The three examples let the reader accept "this binding is social, not technical" from systems they already trust, and move on. The device-group correction and dial-discipline principle close a real hole: the spec had hardcoded a trust assumption it elsewhere insists must be a dial, and naming the usability/variety tradeoff prevents "everything is a dial" from becoming its own failure.

**Grounding.** PGP web-of-trust and SSH/GnuPG TOFU trust-model descriptions, and the proof-of-personhood literature's statement that web-of-trust was not designed for personhood/Sybil resistance, were web-verified this round (GnuPG trust-model docs; arXiv proof-of-personhood survey). TLS/X.509 CA framing is standard (RFC 5280). All carry `[confirm before publish]` pending pinning to the primary references.

---

## 0000. Personhood reframed as contextual group judgment, not an unforgeability assumption (this round)

**What changed, in one line.** The previous round stated the anti-capture property as resting on a *threat-model assumption that personhood is unforgeable*. That was itself a provenance/utility conflation — the very error the spec exists to prevent. This round reframes it: **the protocol guarantees provenance (flat, conserved, non-inflatable weight per recognized peer); the group judges personhood, contextually, at its own confidence.** Sybil resistance is stated as contextual, not global. Grounded in the Spritely Institute / ActivityPub lineage.

**The correction.** "Personhood is unforgeable" is both false (in a low-binding broadcast scope one human can mint many lineages, and no protocol prevents it) and undesirable (enforcing one-key-one-human would forbid legitimate multiple self-presentation). The honest split:

- *Protocol guarantee (provenance, technical):* messages from a key-lineage are provably from it; weight is flat per recognized peer and conserved under delegation, never minted by clients or resources.

- *Group judgment (personhood, social, contextual):* whether a recognized peer is a distinct person is a utility judgment the group makes at its confidence, on the trust-to-do gradient — high (QR-scan family scope), medium-and-anonymous (a verifiable-credential service enforcing one-per-ID without revealing the ID), or low (open broadcast). The protocol deliberately does not technicalize this, because the key-to-person binding is the kind of truth §2.0 says the system cannot certify, and because identity-presentation variety is part of the social substrate (the Ashby variety argument applied to identity).

So the load-bearing claim is not "you cannot forge personhood" but *given the group's recognition of its peers, weight is flat and uninflatable by resources.*

**A second conflation corrected (credential services).** Delegating the personhood check to a third-party attester is itself a *utility judgment to accept*; if that attester turns adversarial the remedy is withdraw-trust-and-fork (§7.6), not a technical revocation primitive. "Revocable" here does not mean a protocol off-switch — the fork is the revocation. Treating the credential service as a structural dependency needing a primitive would re-collapse utility into provenance; it is a valuation edge (Part 1 §2.3), trusted by choice and exited by fork. *(This corrects a caveat the assistant itself raised in the prior turn, which had imported a technical frame onto a social judgment.)*

**Where.**

- **Part 2 §5.6** conservation/anti-capture blocks rewritten: protocol-guarantee-vs-group-judgment split; contextual Sybil resistance with the three gradient examples (QR-scan / anonymous-credential / broadcast); credential-service-as-forkable-valuation-edge; the variety-applied-to-identity argument with Spritely grounding; the old "personhood-verification mechanism" open seam reframed from a threat-hole-to-plug into a contextual judgment the protocol deliberately leaves to groups.

- **Part 2 §5.0** weight-axis definition: anchored the meaning of "personhood-verified" as "recognized by the group at its contextual confidence, not a global protocol guarantee," so all later uses inherit it.

- **Part 1 §2.3** anti-capture claim rewritten from "design rule + unforgeable-personhood assumption" to "protocol guarantee (provenance) + group judgment (contextual personhood)," with the multiple-presentation / variety point.

- **Part 1 References** governance-frontier section: added the Spritely Institute / Lemmer-Webber (ActivityPub) reference, grounding contextual identity and the don't-overclaim principle, plus the petname tradition for naming.

**Why.** The earlier framing made peer-equality depend on an unsolved (and unsolvable, and undesirable-to-solve) problem — a weakness on its own terms and a layer-collapse on the spec's terms. The reframing rests the guarantee only on what the protocol actually provides (provenance) and hands personhood to contextual group judgment, which is both more defensible and faithful to the provenance/utility split. It is a recognition of reality, not a cop-out: make the tooling useful and honest rather than claim a global guarantee it cannot provide.

**Grounding.** Spritely Institute design principles (no global town square; contextual flows; "we should not pretend we can prevent what we cannot") and the W3C ActivityPub lineage (Lemmer-Webber, lead author) were web-verified this round. Christine Lemmer-Webber's "How decentralized is Bluesky really?" (2024) was read for the Petname / Zooko's-triangle naming material (relevant to a future naming treatment, not over-applied to the personhood point). All quotations carry `[confirm before publish]` pending pinning to the primary pages.

---

## 000. Resource / capability / role correction + group-as-principal (this round)

**What changed, in one line.** The previous round renamed the delegated-authority engine to "role" and kept "capability" for device facility. This round corrects that: **"capability" reverts to Meadowcap's exact meaning** (a read/write data-access grant), the **device-facility layer is renamed "resource"**, and **"role" is narrowed to in-group governance authority** — three nouns at three distinct planes, none colliding with the prior art. A new **group-as-principal** subsection (Part 2 §5.10) and a matching Part 1 reasoning illustration were added.

**Why the change from last round.** Grounding Meadowcap against its primary spec showed its "capability" is scoped tightly and specifically — an unforgeable token bestowing **read or write access to data** in a namespace, issued by the data's owner. That is a *data-access* primitive, not the device-facility thing the spec had been calling "capability," and not the same as an in-group governance authority. Reusing "capability" for the device layer fought the prior art Drystone intends to adopt; the cleaner separation is **resource** (device), **capability** (Meadowcap data access), **role** (governance authority). "Resource" also names the device fact without inviting the false slide from "able to" to "entitled to" — a device with more resources can *do* more, never *count* for more.

**The final five-noun model (across both parts).**

- **Resource** — what a *device* has (storage, uptime, reachability, radio). Intrinsic, descriptive, unequal, non-delegable. *(Was "capability" last round.)*

- **Capability** — a *data-access grant* (read/write a namespace area), Meadowcap's sense kept verbatim, issued by member consent, attenuating.

- **Role** — an *in-group governance authority* (admin, moderator, gating, the act-for-the-group authority, the authority to issue capabilities). The layer MLS deliberately leaves to the application.

- **Right** — inherent peer floor (voice, tenure, exit). Never delegated.

- **Weight** — flat governance count, one per personhood-verified peer.

**MLS grounding for the role layer.** RFC 9750 states plainly that **MLS enforces no access control on group operations** — any member can add or evict — and that **the application is responsible for its own access-control policy** (the "if only the administrator may change members, the application must define and communicate that" example). So the "parent can invite, child cannot" case is exactly the slot MLS intentionally leaves empty; "role" is Drystone's word to define, colliding with no MLS primitive. The enforcement MLS punts to "the application" lives in Drystone's governance fold: a child's `Remove(parent)` is a well-formed MLS message that honest peers reject as unauthorized from replicated role policy — convergent agreement that the op is unauthorized, not cryptographic impossibility of emitting it (now stated as an honest seam in §5.5/§5.7).

**Group-as-principal (Part 2 §5.10, new).** A group is a principal: a communal-namespace identity, living **above MLS** (MLS is the communication-and-safety substrate; the group-principal is the application-layer ownership/governance construct represented in the artifacts). Resolves the previously-open "group-as-principal identity" seam by giving it a concrete shape — a Meadowcap **communal** namespace (horizontal authority, no apex), which is `P-Peer-Equality` at the data layer. Includes: the **forked-artifact ownership** mechanism (both layers own it, both forks carry the whole artifact, like an open-source fork — the data-layer face of fork-not-verdict); **composition** (a group-principal can be a member of another, nesting user→community→federation); the **act-for-the-group role** (a group cannot sign, so acting for it is a governed, revocable role under the same anti-entrenchment ladder); and the **recursion bottoming out at flat-weight peers** so composition cannot launder governance weight. Maps Meadowcap **communal vs owned** onto Drystone's **peer-equal vs apex**: communal for group governance, owned reserved for the single-author-sub-content case (the `share`/`tenure` seam).

**Where (Part 2).**

- **§5 provenance note + §5.0** rewritten to the three-plane model (resource / capability / role) with MLS and Meadowcap vocabularies both adopted as-is.

- **§5.4** retitled "Capabilities" → "**Resources**" (device facilities).

- **§5.5** rewritten to separate **role** (governance authority) from **capability** (Meadowcap data-access grant) as two distinct planes, with delegation, PeerSet, and the meer example updated (a meer holds an availability-serve *role*, expects availability/reachability *resources*, holds an empty *capability* set).

- **§5.6** weight wording updated (resources/capabilities/roles do not add weight).

- **New §5.10** group-as-principal (above).

- **§5.3** open-check note: removed the now-wrong "Meadowcap capability = Drystone role" line; points to §5.10 communal model.

- **§7.2** retitled "role-and-revocation" → "**grant-and-revocation interface**," covering both roles and capabilities (both are unforgeable, attenuating, revocable governance facts); R1–R6 reworded to "grant," R2 notes Meadowcap confinement, R6 stays capability-specific (data-access acceptance).

- **§10.4 / §10.5** "Role mechanism (ocap capability)" corrected back to "**Capability mechanism** (Meadowcap data-access)," with the role layer noted as sitting above it.

- **Appendix C** Meadowcap/Keyhive line corrected: capability is Meadowcap's word (kept), roles are the governance layer above.

**Where (Part 1).**

- **§2.3 retitled** "...unequal in capability and (delegated) rights" → "**...unequal in resources and (delegated) rights**" and updated throughout (device axis is "resource"; configuration formula is `floor + [roles] + [implied capabilities] + [expected resources]`; vocabulary note corrected so Meadowcap's capability is unchanged and the device fact is a resource).

- **§2.3 forked-artifact reasoning illustration** added (three peers, auto-merge, fork, both layers own, both forks keep the whole) with the mechanism cross-referenced to Part 2 §5.10.

- **§2.3 open-seams note** updated: group-as-principal is now shaped (communal namespace) rather than fully open; remaining open items are the communal-namespace key construction and cross-group grants.

**Two seams still explicitly open (propagated, not resolved):** the **communal-namespace key construction** for a group-principal (establishment and rotation under membership change), and **cross-group** grants/references (the valuation-vs-composition edge wire encoding). Both flagged `[confirm before publish]` / `ENABLING` and carried to Appendix B.

**Grounding.** Meadowcap primary docs (the capability definition as a read/write data-access token; communal vs owned namespace models and their explicit governance framing) and RFC 9420/9750 (MLS enforces no group-operation access control; application owns the policy; the administrator example) were web-verified this round. Exact quotations carry `[confirm before publish]` pending pinning to the primary text.

---

## 00. Identity & authority vocabulary overhaul — principal / peer / client and the three axes (this round)

**What changed, in one line.** The spec previously overloaded "peer" to mean identity, rights-holder, and governance unit at once, and used "capability" for the delegated-authority thing. This round separates those into a precise model: **three orthogonal axes (capability / right / weight)** and **two layers (identity: principal+client; governance: peer+weight)**, with a deep rename so **capability = device-intrinsic** and the delegated-authority engine becomes **role**.

**The model now written into both parts.**

- **Capability** — what a *client/device* can do (storage, uptime, reachability, radio). Intrinsic, descriptive, **unequal**, **not delegated**.

- **Right** — what a *principal* may do. An inherent floor every peer holds, plus authority delegated **as a revocable role**. Allowed to be unequal.

- **Weight** — how much a *peer* counts in governance. **Flat: one per personhood-verified peer.** Default one-peer-one-vote; liquid delegation, elected admins, and broadcast-only are explicit variations on top. Conserved under delegation (allocated one-per-personhood at source, moved never minted).

- **Principal** — a role-holding entity identified by one key-lineage (the genus). Kinds: peer, group, mere-peer/blind-broker (meer), and delegate (a *state*, not a species).

- **Peer** — the kind of principal carrying the rights floor and local canonical state; source of one unit of weight.

- **Client** — a single device: one MLS leaf, one signature key, one credential. A principal is realized by one or more clients. MLS addresses clients; governance addresses principals.

**The governance-integrity spine.** Governance quorums/thresholds count **peers by personhood-verified identity, never clients.** Adding devices adds capability, never rights and never weight. This is the anti-capture property, stated as two separate commitments: a *design rule* Drystone controls (weight from personhood, conserved under delegation) and a *threat-model assumption* it must defend (personhood is unforgeable). Motivated explicitly by the history of resource-weighted crypto-governance takeovers being painful to unwind.

**The deep capability→role rename (decided deliberately over the split-names alternative).** What the object-capability literature and Meadowcap call a "capability," Drystone now calls a **role**; "capability" is reserved for device-intrinsic facility. Because this collides with the ocap/Meadowcap lineage by design, a reconciliation note is placed at every prior-art touchpoint (§5.5, §7.2, §10.4, §10.5, Part 1 §2.3) so the collision is explicit, not accidental.

**Where (Part 2).**

- **§5 retitled and substantially rewritten** ("Identity, Principals, Rights, Roles, and Capabilities"): new §5.0 (three axes + two layers), §5.1 (local canonical, now principal/client), §5.2 (principal/client/peer identity model + the governance-integrity spine + the group-as-principal open seam), §5.3 (rights floor, inherent vs delegated-as-role), §5.4 (capabilities = device-intrinsic, **inverted** from the old "delegated capability" framing), §5.5 (role / delegation / PeerSet permission layer + mutual-exclusion-fails-loud + the meer worked example rewritten), **new §5.6 (Weight** — flat default, the multi-model variations, the conservation invariant, the personhood open seam), §5.7 (membership/threshold, now counting peers-not-clients), §5.8 (revocation future-not-past, role language), §5.9 (exitability, role language).

- **§7.2 renamed** "capability-and-revocation interface" → "**role-and-revocation interface**"; R1–R6 kept verbatim in meaning, reworded to role, with the ocap reconciliation note.

- **§4.5.1** tied "device" to "**client**" (MLS leaf) at first use and cross-referenced the §5.2 spine (each client a distinct member, but governance resolves to principal before counting).

- **§10.2** added a named, accepted **device-level metadata-leak tradeoff** (per RFC 9750 §8.2.4: others can see which device sent what and when devices are added/removed; Drystone's identity layer is the "separate user-level mechanism" the RFC describes) — stated as a cost accepted, not eliminated.

- **§10.4 / §10.5** "Capability mechanism" → "**Role mechanism**" with the ocap note in the primitives prose and the ledger row.

**Where (Part 1).**

- **§2.3 retitled** "equal in rights, not in capabilities" → "**equal in weight; unequal in capability and (delegated) rights**" and rewritten under the three axes. Corrects the specific conflation the review caught: the old text called "a vote weight," "a role, a delegated authority, a moderation power" all *capabilities*; these are now correctly the **weight** axis and the **role** (permission) layer respectively. Adds the two-commitments anti-capture statement, the ocap vocabulary note, the group-as-principal open seam, and updates the recursion paragraph (a principal can be a group; a user is a group of clients).

- **§2.4** "delegated capability" → "delegated role."

**Why.** A governance spec cannot let "peer" mean three things or let "capability" mean both a device fact and a delegated permission; the conflation is exactly the kind of first-landing error that is expensive to unlearn. Grounded in the MLS specs, which settle the terminology: RFC 9420/9750 define **client** as the per-device participant, **member** as an in-group client, and state that multiple devices of one user operate as distinct clients, that duplicate signature keys in a group are forbidden, and that "when two credentials represent the same user" is an explicit **application policy** above MLS — which is precisely the identity layer Drystone owns.

**Two seams left explicitly open (propagated around, not resolved):** the **group-as-principal identity** (what a collective's key-lineage is), and the **personhood-verification mechanism** (the contested seam the anti-capture property depends on). Both are flagged in-place with `[confirm before publish]` and carried to Appendix B.

**Grounding.** RFC 9420 §5.3.3 (uniquely identifying clients; multiple devices as distinct clients), RFC 9750 §2.1 (client/member/group definitions), §8.2.4 (multi-device operation and the metadata-correlation tradeoff and mitigations), and the architecture's "policy for when two credentials represent the same client/user" were web-verified this round. Exact quotations carry `[confirm before publish]` pending pinning to the primary text.

---

## 0. Part 1 distributed-systems grounding + canonical References (added this round)

**What changed.**

Part 1 §3 ("Why these principles are corroborated, not invented") gained a new **distributed-systems subsection, placed first** among the fields, grounding the formal spine the section was previously missing. The §3 field count moved from five to six (distributed systems now sits alongside ethics, economics, systems science, epistemology, political science). A new **canonical References section** was appended to the bottom of Part 1 (it previously had none; only Part 2 carried references). The Ostrom citation was split correctly between the 1990 primary and the 2013 generalization, resolving a standing open flag.

**Where.**

- Part 1 §3: new distributed-systems subsection (Lamport 1978, CAP/Gilbert-Lynch 2002, CALM/Hellerstein-Alvaro 2020, CRDTs/Shapiro et al. 2011), each with an explicit "what it grounds / where the seam is" note and a per-quote verification flag.

- Part 1 §3 intro and the "why this convergence persisted unassembled" box: updated to six fields and to state plainly that this is not a technical-supremacy claim but a claim that established results *force* a humane shape.

- Part 1 §3 Ostrom paragraph: principles 6 and 7 now grounded to *Governing the Commons* (1990) verbatim; subsidiarity correctly attributed to Wilson/Ostrom/Cox (2013), with an explicit do-not-conflate note. This closes the prior "[confirm] — 2013 wording, not 1990" flag by separating the two.

- Part 1 "The convergence is the corroboration" close: reworked to pair the distributed-systems theorems (shape is *forced*) with the human-sciences arguments (shape is *humane*), and to restate the scoped claim.

- Part 1 new **References (Part 1)** section: organized by field, each entry stating what it grounds, the seam, and a *Verified* / **[confirm]** flag; framed up front as acknowledgment of prior art, explicitly *not* a priority or supremacy claim; cross-referenced to Part 2 Appendix C (mechanism lineage) and §10 (substrate requirement-vs-realization).

**Why.**

The distributed-systems lineage is the field the work most directly lives in, yet §3 grounded it only by forward-reference to Part 2. Adding Lamport/CAP/CALM/CRDTs as the first field makes the central Part 1 claim — local-first is *derived, not chosen* — rest on the field's own impossibility results rather than on assertion: no global clock (Lamport), no consistent-and-available global state under partition (CAP), coordination-free only in the monotonic fragment (CALM), convergence-without-a-coordinator proven for that fragment (CRDTs). The references section serves the stated goal: transparency and acknowledgment of connected and prior art, making explicit that the contribution is synthesis and humane delivery, not invention.

**Grounding.** Lamport 1978, Gilbert & Lynch 2002 (formalizing Brewer 2000), Hellerstein & Alvaro CALM 2020 (PODS 2010 conjecture; Ameloot/Neven/Van den Bussche proof), Shapiro et al. CRDTs 2011, and Ostrom 1990 principles 6/7 plus Wilson-Ostrom-Cox 2013 subsidiarity were all web-verified to primary or near-primary sources this round. Exact quote *wording* for the distributed-systems sources carries **[confirm before publish]** until pulled from the papers themselves; the *statements* are confirmed. The three recent arXiv "category mistake in logical clocks" preprints were deliberately **not** cited — they align with §2.0.1 but are recent, non-canonical, single-author preprints, and leaning on them would weaken the grounding by association; §2.0.1 stands on Lamport's established result plus Drystone's own argument.

---

## 1. Terminology: "serverless" removed in favor of "center-free"

**What changed.**

"Serverless" is removed as the identity of the protocol everywhere it appeared as an authority claim. Replaced with **center-free** for the authority property and **peer-to-peer** reserved strictly for the transport layer.

**Where.**

- Part 1: new naming note in §1; the word "serverless" eliminated; §1.2 and the §2.x consequences now say "center-free" or "no node holds privileged or canonical state."

- Part 2: new "Naming: center-free, not serverless" callout near the top; §3.1 gains a paragraph explaining why topology (p2p) is not the load-bearing property; §4.5.1, §6, and Appendix A.1 retitled/reworded from "serverless" to "center-free."

**Why.**

Your objection: "serverless" has a fixed, very different meaning (AWS/Lambda managed ephemeral compute) and is guaranteed to confuse the exact technical audience you want. "P2P" is not wrong but describes topology, not the authority property — and is frequently used by systems (blockchains) that build the global apex you reject. "Center-free" names the property you actually mean (no privileged/canonical node) without the baggage, and pairs cleanly with p2p-for-transport. This also reinforces the §3.1 "wires lie, authority topology tells the truth" argument: the distinguishing word is deliberately *not* in the topology.

**Note.** I did not search-verify that "center-free" is an unclaimed term of art. It is used here descriptively, not as a coined brand. If you want a verified-distinct coined name, that is a separate naming pass.

---

## 2. Time-is-an-assertion: the §2.0.1 upgrade (the largest conceptual change)

**What changed.**

The justification for excluding wall-clock timestamps was upgraded from a *defensive* argument (peers can lie about clocks) to a *structural* one (a timestamp is not a fact even from an honest node, because time discernment is a locally-represented shared construct and no node can prove when an event occurred on another node — so a timestamp fails corroboration at the root and is an assertion, never provenance).

**Where.**

- Part 1: **new subsection §2.0.1** ("Time is an assertion, never a fact — the razor's sharpest edge"), placed immediately after the identity discussion in §2.0 because it is the same assertion/provenance cut applied to time.

- Part 2: **§7.3.1 substantially rewritten** — the "timestamps appear nowhere" paragraph now leads with the structural reason and explicitly distinguishes it from the weak deception reason; adds the social-engineering-vector-even-with-membership-gating point.

- Part 2: **§4.2** — the `timestamp` field **removed** from the content-id canonical pre-image, with a note explaining why and how an application may record a "claimed time" as ordinary payload instead. (This is a real wire change and is now also flagged in Appendix B `ENABLING` encodings.)

- Part 2: **§4.5.1** — logical clock section reworded to emphasize per-device `lamport` is *strictly logical, never a wall-clock*, as a load-bearing instance of §2.0.1.

- Part 2: **§4.3** — note added that `seq` is a per-branch counter, not a clock.

- Part 2: **§7.2 R4** — the stale-authority bound respecified as epoch/generation, explicitly **not** a wall-clock interval (a time-expressed bound would rest on the same uncorroborable clock).

- Part 2: **§8** — new "wall-clock as an attack surface" cross-reference paragraph; the metadata-surface paragraph corrected (a relay observes *arrival order at the relay as its own local observation*, not an authored "timestamp").

- Part 2: **Appendix A** — the Matrix-rejection entry now separates the timestamp-tiebreak rejection (uncorroborable, §2.0.1) from the power-in-comparator rejection (apparent-cycle), as two distinct reasons.

**Why.**

Your framing: timestamping is an assertion, never corroborable — a node can be objectively wrong about its own clock and not know, and can never prove when an event happened on another node. Crucially you noted this is a social-engineering vector *even when membership is gated*, because gating checks *who*, not *whether their clock is real*. This is a stronger and more fundamental claim than "clocks are gameable," and it converts the timestamp-free ordering from an anti-attacker measure into a direct consequence of the razor. The content-id `timestamp` removal follows necessarily: if time is not provenance, it cannot sit in an identity-bearing computation.

---

## 3. Fork-not-verdict: the forced terminus made explicit (Part 1 §2.5, Part 2 §7.6)

**What changed.**

The conclusion that the non-monotonic residue is intrinsically a utility judgment, and therefore must terminate in a human-adjudicated fork rather than a computed verdict, was promoted from implicit to explicit and stated as *derived, not chosen*.

**Where.**

- Part 1: **new subsection §2.5** ("The forced terminus — why fork, not verdict"), explicitly framed as new emphasis, not a new principle. States the CALM-boundary argument (a total social order over concurrent non-monotonic ops is itself non-monotonic, so no coordination-free determinate resolution), the intrinsic-utility argument (no fact of the matter; the people constitute the answer), and the fork-not-verdict conclusion (a verdict presupposes an answer the loser should accept; a fork presupposes there was none).

- Part 1: **§3 Mill paragraph** extended to connect Mill explicitly to the forced terminus.

- Part 2: **§7.6 substantially extended** — new "why the terminus is a fork and not a verdict" paragraph; explicit framing of the hard-stop as the realization of Part 1 §2.5.

- Part 2: **§7.3.2** — new boundary note distinguishing "deterministically tiebroken" conflicts (content-address, provenance, non-arbitrary) from "must escalate" contradictions (utility, §7.6).

**Why.**

This was the through-line we converged on: the residue is non-empty by CALM (logic can't close it), and where it exists it is a value question by nature (the razor), so it is not uncomputable-by-this-machine but not-a-computation-at-all. The honest sharpening you insisted on — that "all conflicts → humans" is wrong, only the provably-non-empty residue is escalated — is captured in the "what this is not" note. The legitimacy point (the protocol technicalizes only provenance and *mechanizes the refusal* to technicalize utility) is stated in Part 1 §2.5 and Part 2 Appendix C.5.

---

## 4. Conflicted-subgraph closure added to §7.5.2 (the highest-divergence-risk fix)

**What changed.**

§7.5.2 now requires the resolution input set to be closed under **two** distinct relations, labeled by the property each guards: **frontier-closure** (authorization-guard, backward-reachability/ancestors — already present, now named) and **conflicted-subgraph-closure** (convergence-guard, the events causally *between* two conflicting facts — new). The subgraph rule is explicitly adopted from Matrix State Resolution v2.1 (MSC4297), with the SCC characterization and forward-backward-intersection computation credited to them, while keeping Drystone's own tiebreak.

**Where.**

- Part 2: **§7.5.2** — new two-closures requirement block; new `ENABLING` note crediting MSC4297 and stating that adopting their closure does not adopt their ordering; new "failure mode specific to Drystone's monotonic fold" callout (an incomplete closure manifests as a false trip of the human channel, not as Matrix's reversion).

- Part 2: **§9** — conformance section updated to name "frontier-closure-and-subgraph-closure" as the likeliest divergence point.

- Part 2: **Appendix B** — `ENABLING` encodings entry updated to "frontier-closure-and-subgraph-closure before sort."

**Why.**

The MSC4297 deep-dive confirmed the gap precisely: your §7.5.2 closed only under frontier-naming (ancestors), but Matrix's v2.1 proved you also need the between-conflicts subgraph for convergence, and gave it a formal name (SCC of the contracted conflict supernode) and a reference algorithm. We also established the failure mode is *worse* for you than for Matrix — because your monotonic fold can't revert, an incomplete closure shows up as two honest peers at different heads, which your §7.6 would escalate as a false contradiction, eroding the trust the algedonic channel depends on. That is why this is now normative and byte-specified rather than best-effort.

---

## 5. Capped-root steelman and the design-philosophy reframe (Part 1 §2.3, Part 2 §5.6, §7.3, Appendix B)

**What changed.**

The MSC4289 uncapped-root conclusion is now stated as an explicit steelman against P-Peer-Equality, with Drystone's response (the apex was forced by their wall-clock inputs, not by the problem; Drystone removes the wall-clock so the backdating surface that forced their apex has no purchase) — and, importantly, reframed as a **design-philosophy difference**: Matrix *prevents* capture with an apex; Drystone *permits* capture by a legitimate quorum and makes the §7.6 fork the *remedy*. The soundness comparison is stated as argued-not-proven, with a precise testing-coverage rubric.

**Where.**

- Part 1: **§2.3** — new callout naming the Matrix steelman and Drystone's wager and the philosophy difference.

- Part 2: **§5.6** — "Capture ≠ brick" now explicitly tied to the uncapped-root steelman as Drystone's answer.

- Part 2: **§7.3** — the central-design-bet callout now carries both the *supporting case* (CVE-2025-49090, monotonic fold) and the *steelman against* (MSC4289), with the two-distinct-claims separation (monotonic fold defends the reset class; timestamp-free order defends the backdating surface). The unconflictable-root paragraph notes the structural root-forgery closure (analogue of MSC4291).

- Part 2: **Appendix B** — capped-vs-uncapped-root promoted to "the priority open security item," with the (a)/(b)/(c) attack-component decomposition and the instruction to **state the actual test coverage** rather than a bare "tested/open," plus the exit-as-remedy-for-capture framing as the cleaner test target.

- Part 2: **§8.1** — added to the honesty-boundaries list as item (g).

**Why.**

You flagged that I had overstated the causal link ("timestamp tiebreaks produced the CVE and drove MSC4289") — that was wrong and is corrected throughout (see item 7 below). Separately, you said you believe you *tested* the capped-root soundness and need to surface or refine it. The changelog rubric in Appendix B is built so you can check exactly which of the three attack components (backdating; root-forgery; entrenchment-trap) and which compositions your existing tests covered, and state that coverage precisely — which is a far stronger and more falsifiable claim than either "tested" or "open." The exit-as-remedy-for-capture reframe (your "capture ≠ brick" already implied it) is the cleaner thing to test and to assert.

---

## 6. False-positive escalation tolerance as a governed utility judgment (new §7.4.1)

**What changed.**

New subsection §7.4.1 establishing that the machine computes corroborable provenance signals (concurrency vs causal-dependence, liveness-over-window, frontier-divergence shape) but the *escalation tolerance* over those signals is a per-scope governed policy, not a hardcoded constant — because the benign-sync-artifact vs genuine-contradiction distinction is ultimately a utility judgment and is vulnerable to alarm-fatigue. Two normative guardrails: the auto-reconcile boundary must stay cryptographic/causal (never heuristic), and genuinely ambiguous cases default to escalate.

**Where.**

- Part 2: **new §7.4.1** ("The false-positive tolerance is a governed utility judgment, not a constant").

- Part 2: **§7.4** — new "what freshness can and cannot establish" note (no node can know "most current," but liveness-over-window and causal-independence *are* corroborable provenance), tied to §2.0.1.

- Part 2: **§8.1** — added as honesty-boundary item (f).

- Part 2: **Appendix B** — new open-item entry for the byte-level signal definitions and policy-fact format.

**Why.**

Your reasoning: the false-trip problem arises because no node can know what's most current in a center-free design — but they *can* know who is active over a window, and can present the risk with a tolerance tuned to use case, temperament, and need, since it's an honest-misunderstanding-vs-deception question that is ultimately a social utility judgment vulnerable to fatigue. The §7.4.1 design keeps this razor-clean: the machine surfaces verifiable signals; the humans (via governed policy) set the sensitivity. The guardrail against a too-loose tolerance silently auto-resolving a real contradiction (re-opening the manufactured-resolution surface) is added because that is the obvious way the feature could be abused.

---

## 7. Correction of the overstated Matrix causal claim

**What changed.**

The claim that "timestamp tiebreaks produced CVE-2025-49090 and drove MSC4289's infinite-creator-power" is **removed and corrected** everywhere. The accurate decomposition is now used consistently: CVE-2025-49090's root cause was starting-state and replay-scope (fixed by v2.1's empty-base + conflicted-subgraph), **not** the tiebreak; MSC4289's uncapped root was driven by backdating + create-event-uniqueness, not the tiebreak. These are now presented as *separate* points: the CVE supports the monotonic-fold choice (§7.3); the timestamp exclusion is a separate structural matter (§7.3.1); the power-in-comparator rejection is a third separate matter (Appendix A).

**Where.**

- Part 2: **§7.3** (supporting case explicitly says "root cause was starting-state and replay-scope, not the tiebreak"), **§7.3.1** (contrast-with-Matrix note explicitly says the CVE was rooted in starting-state/replay-scope, not the timestamp tiebreak), **Appendix A** (two-distinct-reasons separation), **Appendix C.2** (the three Matrix citations sorted into adopted / cautionary / steelman).

- Part 1: **§2.2 callout** frames the CVE only as evidence for the monotonic-fold, not for the timestamp point.

**Why.**

You asked me to fact-check the bold claim, and it did not survive: v2.1 fixed the reset *without* removing power-ordering or timestamp tiebreaks, so "timestamp tiebreaks produced the CVE" is false. The corrected version is actually stronger — it shows Drystone defends against *two distinct* Matrix failure modes (reset via monotonic fold; backdating via timestamp-free order) rather than collapsing them into one shootable claim.

---

## 8. New consolidated prior-art section (Part 2 Appendix C) and up-front positioning (Part 1 §1.2)

**What changed.**

A full Related Work / positioning section was added consolidating CALM, CRDTs/local-first, Willow/Meadowcap/Keyhive, Matrix State Resolution and the 2025 CVEs, decentralized-MLS (now correctly split into the two distinct drafts), blockchain governance, and Modular Politics/Ostrom — each with the precise relationship (borrowed / adopted / cautionary / steelman / neighbor). A shorter positioning subsection was added near the top of Part 1.

**Where.**

- Part 1: **new §1.2** ("Where Drystone sits") — the two-camps framing, the Modular Politics neighbor, and the scoped novelty claim (synthesis + terminus, unoccupied-against-closest-neighbors, not first-ever).

- Part 1: **new callout in §3** explaining *why* the cross-disciplinary convergence persisted unassembled (the five-fields-rarely-meet structural reason).

- Part 2: **new Appendix C** (C.1 data layer; C.2 governance/resolution layer; C.3 cryptographic group-state; C.4 governance-as-protocol frontier / Modular Politics; C.5 cross-disciplinary grounding) and an expanded References section.

- Part 2: **Appendix A.1** corrected to split `draft-kohbrok-mls-dmls`/FREEK from `draft-xue-distributed-mls` (these were conflated in the prior draft).

**Why.**

You asked for both inline flags and a consolidated section. The consolidated section lets a reviewer see the whole landscape at once and lets the novelty claim be stated honestly: the mechanisms are prior art (cite them as foundation, not rivals); what is novel is the vertical synthesis and the fork-not-verdict terminus, and the claim is "unoccupied against the closest published neighbors," explicitly *not* "first ever" (absence-of-evidence, not proof-of-emptiness). The Modular Politics entry (C.4) is the single strongest piece of positioning, because the closest governance-as-protocol group shares your premises and stops at the door Part 2 walks through.

---

---

## 8.5. New §10 — Substrate Requirements and Reference Realizations (added this round)

**What changed.**

A new consolidated **Section 10** was added to Part 2, applying the requirement-vs-realization discipline to the bundled substrate stack. For each component it states the abstract requirement, a compliance table, the disqualifiers, and the reference realization with a "why it currently wins" note. The existing in-place mechanism-neutral text is kept and cross-referenced (per your "consolidated section PLUS keep in-place" choice).

**Where.**

- Part 2: **new §10** with subsections — §10.1 (how to read), §10.2 (messaging backplane / group key agreement — the MLS treatment), §10.3 (transport and overlay — the iroh treatment and the topology question), §10.3.1 (the "local-first p2p" category definition), §10.4 (primitives — signature/hash/capability/data-model), §10.5 (the dependency-vs-realization summary ledger).

- Part 2: cross-reference notes inserted into **§4.1** (→ §10.4), **§4.5.1** (→ §10.2, naming K5), and **§6** (→ §10.3).

- Part 2: **§3** roadmap updated to list §10.

**The MLS treatment (§10.2), per your "table + disqualifiers + why MLS wins" choice.**

- Defines each property precisely against RFC 9420/9750: **FS**, **PCS**, **membership agreement**, **asynchronous operation**, and the Drystone-specific **FS-under-concurrency** (the property standard server-ordered MLS does *not* have to satisfy and Drystone does, because there is no Delivery Service to order Commits).

- Compliance table **K1–K8** and an explicit disqualifier list (trusted online ordering authority; `O(N²)` sender-keys; no per-device membership; silent membership divergence; wall-clock dependency).

- Positions MLS as **reference, not requirement**, and states the critical divergence: **MLS assumes a trusted AS and an ordering Delivery Service; Drystone has no DS** (it is center-free, §3.1), so MLS-as-standardized is *necessary but not sufficient* — K7 must be met by a FREEK-shaped puncturable-PRF (`draft-kohbrok-mls-dmls`), the `draft-xue` Send-Groups approach, or an explicit retention budget. The **AS** role is filled center-free by the user-principal-as-its-own-CA (§4.5.1).

**The iroh / topology treatment (§10.3), per your "abstract requirements + iroh reference + compliant divergent topologies + local-first-p2p category + human-adjudication note" choice.**

- Compliance table **T1–T7** (public-key identity; e2e encryption with blind intermediaries; direct path with bypassable relay fallback; blind packet-forwarding relay; reliable-stream + unreliable-datagram modes; member-discovery overlay; no wall-clock) and a disqualifier list (IP-as-identity; content-reading intermediary; mandatory-permanent relay hop; reliable-only transport).

- iroh named as reference with the grounded "why it wins" — public-key `EndpointId` as TLS identity, **stateless blind relay**, datagram mode — verified against iroh 1.0 primary docs.

- The **topology question stated surgically**: the requirement is **reachability, not topology**; **pure mesh is fully compliant but not required**; the reference is point-to-point-with-blind-relay-fallback because pure mesh is `O(N²)`, churn-intolerant, and expensive without giving the reconciliation model anything it needs. Mesh is *permitted, not required*; the reference *avoids* mesh deliberately.

- **§10.3.1 defines "local-first peer-to-peer" as a category** — defining trait: *no shared global state by design* — notes it is unsuitable for genuinely-global-ordering applications but well-suited to a significant class (messaging, collaboration, governance), and includes your point that **every convergent system is ultimately human-managed** (centralized ones just hide the humans behind an operator), so the human-adjudication requirement *reinforces* rather than devalues the crypto-style convergent lineage.

**The primitives (§10.4) and the ledger (§10.5).**

- Signature (Ed25519), hash (SHA-256/BLAKE3 open item), capability (§7.2 R1–R6, already requirement-first), data model (Willow-shaped, already requirement-first) each given the compliance bar and reference.

- §10.5 is a single summary table mapping every component to {requirement bar, reference realization, why/divergence}.

**Why.**

You asked to codify, with provenance, exactly what the messaging backplane *requires* (PFS, PCS, etc.) versus what MLS *currently realizes*, and to give the same treatment to iroh and the rest of the bundled stack. The structure separates dependency from realization everywhere, so a reader sees what Drystone is committed to versus what it happens to use, and a future substrate swap has an explicit compliance bar to meet. The MLS "trusted DS" divergence and the iroh "mesh permitted-not-required" point are the two places this most sharpens the spec, because both are exactly where the center-free premise forces Drystone off the standard realization's assumptions.

**Grounding.** RFC 9420/9750 (MLS FS/PCS definitions, the trusted-AS/untrusted-ordering-DS model) and iroh 1.0 primary docs (public-key identity, stateless blind relay, hole-punch-then-relay-fallback, datagram mode) were web-verified this round. All external claims in §10 carry `[confirm before publish]` against those primaries.

---

## 9. Smaller consistency edits

- Part 1 closing section ("What Part 1 establishes") and Part 2 honesty-boundaries (§8.1) both updated to reflect the new §2.0.1, §2.5, §7.4.1, and the capped-root open item, and to state the scoped novelty claim.

- "Distributed system of peers" / "center-free" language harmonized across both documents.

- Hayek paragraph (Part 1 §3) gains a sentence guarding the boundary (utility is irreducibly local; provenance is exactly the part that *can* be global) so the citation isn't misread as "everything is local."

- Ostrom paragraph (Part 1 §3) gains a sentence naming the value-to-mechanism gap explicitly (Ostrom is corroboration of values, not mechanics).

- BLAKE3 reconciliation item (Part 2 Appendix B) gains the note that Willow uses BLAKE3, making the §7 choice convergent with the data model.

---

## Items deliberately NOT changed (and why)

- **The conformance suite count (66/0)** is left as-is — I have no basis to change a measured number, and the new §7.3–§7.5 vectors are correctly noted as not-yet-in-suite.

- **The green-real claims for §4/§6** are untouched — they describe what was actually run on real crypto/transport, and nothing in this discussion bears on them.

- **The capped-root claim was not upgraded to "tested"** — only reframed and given a coverage rubric, because you said you *believe* you tested it but it has not been surfaced here; the honest move is to state coverage, not assert a result I cannot see.

- **No new mechanisms were invented.** Everything added (subgraph-closure, the tolerance, the §2.0.1 framing) is either an adoption from cited prior art (MSC4297) or a sharpening of a principle already latent in the documents.

---

## Open flags carried forward (the [confirm before publish] set)

Before any external publication, confirm against primary sources: CALM statement (Hellerstein & Alvaro paper); Willow data model + Meadowcap communal/owned + BLAKE3 (Willow/Earthstar specs); Matrix State Resolution v2/v2.1 mechanics, CVE-2025-49090 root cause, CVE-2025-54315/MSC4291, MSC4289 uncapped-root, MSC4297 conflicted-subgraph (the MSC primary texts, not the blog/implementer-guide summaries currently relied on); decentralized-MLS drafts and FREEK; MLS deployment status; Modular Politics quotations (CSCW 2021 paper); Beer verbatim and Cybersyn/OGAS figures; Ostrom subsidiarity wording (2013 generalization). The group-key/MLS device-as-member facts (§4.5.1) also remain to be confirmed against the primary MLS specification.
