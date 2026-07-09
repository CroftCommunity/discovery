# Modular Politics: Source-Grounded Analysis

author: Claude (prepared for Chase Pettet)

date: 2026-06-29

source: Schneider, N., De Filippi, P., Frey, S., Tan, J. Z., Zhang, A. X. (2021). "Modular Politics: Toward a Governance Layer for Online Communities." Proc. ACM Hum.-Comput. Interact. 5, CSCW1, Article 16. arXiv:2005.13701v3

epistemic status: Grounded in the arXiv v3 full text, retrieved this session. Claims about the paper are cited to section number. Synthesis and interpretation are labeled inline as such and are not attributed to the authors.

---

## Summary

Modular Politics is a 2021 conceptual paper proposing a paradigm for online community governance built from composable computational parts, with a call for an open standard. It defines a vocabulary and a design-goals checklist but, by the authors' own statement, is a prologue rather than a specification. Two pieces are left unbuilt and are directly relevant to peer-symmetric protocol work: authority is rooted in a platform operator, and the cryptographic and wire-level resolution layers are absent.

The shorthand "it drew the map and left the territory" is fair, and the authors say as much themselves (§1).

## 1. What the paper is

The paper proposes a governance paradigm assembled from modular, composable parts, and calls for the development of an open standard for networked governance (§1).

It states four design goals (§1):

- Modularity: systems built from composable parts, bottom-up.

- Expressiveness: able to implement as wide a range of processes as possible.

- Portability: tools built for one platform reusable on another.

- Interoperability: governance systems on different platforms able to interact.

The intellectual root is the Ostroms' Institutional Analysis and Development (IAD) framework. The paper adopts IAD's nested, bottom-up conception of institutions and its basic unit, the *action situation*, a game-like setting in which participants make individual and collective choices (§2.3).

The framing problem: offline governance has juries, term limits, and elections, but online platforms mostly assign unchecked power to admins and moderators, so monarchic and oligarchic patterns emerge more readily than democratic ones (§1).

The authors are explicit about status. They describe the model as preliminary and provisional, and the paper as a conceptual overview intended as a prologue for future theoretical, technical, and empirical work, not a specification or theory (§1).

## 2. The permission model is operator-rooted

This is the load-bearing structural constraint, and it is confirmed in the text.

The architecture nests as Platform, then Instance, then Org, then Module. Authority flows top-down.

Three terms the claim depends on:

**Platform** is the host system Modular Politics runs inside (a game, social network, or blockchain protocol). The paper states that Modular Politics is not standalone and must be implemented within an underlying Platform (§3.1). The Platform is the substrate, not part of the model itself.

**Instance** is one implementation of Modular Politics inside a Platform. It defines the interface between the model and the Platform, specifying which Platform entities have a governance role and what actions they can take (§3.1). Decisively: in defining the Instance, platform operators determine who has access to Modular Politics and what it can govern (§3.1).

**Permissions** are the configuration policies specifying what users can do. The paper states that all permissions ultimately derive from what platform administrators specify at the level of the Instance (§3.5). Permissions then inherit downward: restrictions set at an Instance or Org apply automatically to every Org nested within it (§3.5).

Synthesis (my interpretation, not the authors'): the system is portable and composable above the Platform line, but the platform operator is the unbudging root of trust. There is no peer-symmetric authority anywhere in the model. Every capability a user holds traces back to an operator's Instance configuration. For peer-symmetric design, this is precisely the thing left unbuilt. The metaphor of a "governance layer" has a seam here: the layer floats on top of a substrate it does not control and cannot make symmetric.

## 3. Cryptographic resolution and wire encodings

This needs a careful split between what the paper defers explicitly and what it simply never reaches, because those are different epistemic objects.

Explicitly deferred. The paper states that its overview does not consider matters such as security and database structures, which it says will be vital at the design stage (§1). Security mechanism is named as out of scope.

Deliberately uncommitted on form. The paper weighs three implementation strategies (§3.8):

- A single central cloud service called over an API.

- A decentralized application on a distributed-ledger technology (DLT).

- An open standard implemented through open-source libraries.

The authors favor the open standard, on the grounds that it would define features and behaviors while allowing differing implementations to interoperate (§3.8). On cryptography specifically, they argue an open standard removes any need to depend on a single organization's API or on the "cryptoeconomics" that DLT systems require, while noting both remain possible to implement (§3.8).

Never reached. The phrase "wire encodings" does not appear in the paper. What the text actually leaves undefined is the concrete software form altogether: the authors state plainly that they have not specified the particular form it should take in software (§3.8). Cross-Instance interaction is described only as "API calls" whose shape, serialization, and protocol-level encoding are never pinned down (§3.2).

Precision note for any downstream write-up: framing this as "cryptographic resolution and wire encodings as future work" slightly over-credits the paper. The crypto and security piece is explicitly deferred. The wire-encoding piece is never reached at all. Both land as "not here," but only one is named as such.

## 4. Notable specifics worth carrying

The two worked examples are the most concrete the paper gets, and both are hypothetical with no prototype built.

- Example 1 (§4.1): a social-media group for sculpture that evolves from majority-vote rules into a randomly-selected five-member jury for content moderation, then adds a statistics bot that surfaces which rules juries find ambiguous.

- Example 2 (§4.2): an open-source mobile-OS project whose *benevolent dictator for life* builds a three-branch elected government hosted on a blockchain-based repository called BitGit, where voting requires a staking token earned by a merged contribution within the past year.

These illustrate expressiveness. They are not evaluations.

The authors are candid about bias in the artifact. They acknowledge the formulation surely carries built-in political biases they have not yet interrogated, and that the model's pliability can implement oppressive regimes as readily as democratic ones (§6). Their defense is empirical rather than normative: visible, explicit governance is argued to beat a *tyranny of structurelessness*.

Lineage detail for context: the paper traces online governance tooling back to the 1982 text-based game Nomic and the VOTEMGR software for the early FidoNet bulletin board system (§1), placing the work in a roughly forty-year arc rather than as a blockchain-era novelty.

## 5. Relevance to peer-symmetric protocol design

Synthesis (my assessment, not the authors'):

If the target is peer-symmetric protocol design with cryptographic resolution at the wire level, the gap is structural rather than incidental.

- Modular Politics is asymmetric by construction, because permissions are operator-rooted (§3.5).

- It is substrate-agnostic by choice, with crypto and security explicitly deferred (§1, §3.8).

The paper hands over a usable vocabulary (Instance, Org, Module, Monitor, Policy, Permission) and a design-goals checklist. The two layers it does not provide are exactly the ones a peer-symmetric protocol must define:

- A resolution layer: who adjudicates conflicting state across Instances without a privileged operator.

- An encoding layer: what actually goes over the wire.

The map is genuinely useful. The territory is genuinely empty exactly where peer-symmetric work happens.

## 6. Sourcing caveat

Everything above is drawn from the arXiv v3 full text (12 March 2021), which matches the published CSCW version by DOI (10.1145/3449090).

I have not separately checked whether later work revises any of these specific claims. In particular, Schneider's 2024 book *Governable Spaces* cites this paper, and the operator-rooted permission model or the deferred-crypto stance could have shifted in subsequent writing. That check was not performed in this session.

## Source

Schneider, N., De Filippi, P., Frey, S., Tan, J. Z., Zhang, A. X. (2021). Modular Politics: Toward a Governance Layer for Online Communities. Proceedings of the ACM on Human-Computer Interaction, 5(CSCW1), Article 16. https://doi.org/10.1145/3449090 (preprint: arXiv:2005.13701v3)
