# Raw transcript — Drystone Part 1+2 consolidation (p9), apex clarification, proof-of-personhood research (2026-07-06)

**Preservation status: preserved-condensed (cleaned-paste, content-faithful — §4).** User-pasted tail of a
multi-thread claude.ai session (batch nine). UI render chrome and repeated edit/search narration condensed
to substance. Content-faithful, not a byte-pristine export. The durable outputs are the consolidated spec
set (filed to `beta/drystone-spec/`, document-pass-6) and its changelogs; the detailed consolidation
reasoning lives in `part-1-changelog.md` / `part-2-changelog.md` and `conventions-and-decisions.md`. This
file preserves the session's framing and the research (apex, proof-of-personhood) that did not become docs.

---

## Thread A — the consolidation goal (what this session was)

The user framed it: two top-level docs (Part 1 = conceptual/principled grounding; Part 2 = protocol design
spec, written as "here are the requirements by section/function, here's the realization, here's the
alignment", like the MLS hardcases alignment table). The ordering to fold: the prior Part 1+2, then the
transport/delivery-logistics discussion, then the deep MLS specifics/alignment discussion, all into a
self-contained consistent Part 1 + Part 2 with all the thinking cohesive. Watch for terms later sharpened
that need back-porting (e.g. persona's transport-design definition is sharper than the old Part 2; the MLS
`client` is the leaf-key-pair + author identity, a keying context, not "software" as earlier conflated).
Some MLS hard cases were expected to resolve once the broader context was pulled together. Deliberate note
from the user (batch nine): the web agent had been **losing/missing content** across iterations, so items
thought solved sometimes had not actually landed, hence the consolidation and the need for a later audit.

## Thread B — what the consolidation produced (the p9 set)

Definitive synthesis: Part 1 890→975 lines, Part 2 2751→3961 lines. Transport (§6) and deep-MLS design
folded fully into Part 2. Identity model integrated as §5.2 (Principal, client, persona); open items /
interop params / `ENABLING` encodings as Appendix B; external-fact confirmations as Appendix C; term
definitions as Appendix D. Terminology back-port: **`PrincipalSet` retired in favor of `Group Role Set`**
(a named, pinned, Group-recognized bundle of Group Roles); `client = leaf key pair + author identity`
consistent with MLS throughout. Primary-source verification pass, Part 1↔Part 2 consistency read, versioned
upstream reference sections added to both, and cross-document reference disambiguation (bare `§2.x` in Part
2 → `Part 1 §2.x`; bare `§4–§8` in Part 1 → `Part 2 §x`). Two corrections caught in verification: **RoQ is
an Internet-Draft, not RFC 9714**; **Sigstore's primitive is signature transparency, not "countersigning."**
Verified against primaries: transcript-hash §8.2 (verbatim), resumption PSK §8.6, epoch-metadata leak (RFC
9750), Matrix Hydra (CVE-2025-54315, CVE-2025-49090, MSC4289/4291/4297), CALM, RBSR (arXiv:2212.13567 /
SRDS 2023), CRDT (SSS 2011 / INRIA RR-7506), RFC 8446 padding. Deliverables named "p9-" as the current,
only-pending-design-review set. Remaining open work is design decisions + `ENABLING` encodings (Appendix B),
not editorial cleanup.

## Thread C — "apex" clarified

**User:** what does apex mean here? [in: "both camps, under pressure, conclude you need an apex" — Matrix
folds authority + a wall-clock into ordering; blockchains reach global consensus on a canonical chain and
treat forks as failures.]

**Assistant:** apex = a single top-level authority, the peak of a pyramid, the one point where final
ordering/truth gets decided by appeal to something privileged rather than by symmetric agreement among
equals. Matrix's apex is the power-level + timestamp tiebreaker (an asymmetric arbiter one side can't
contest as an equal); blockchains' apex is the canonical chain (one true history, forks are losers by
definition). Both reach the same shape under contention: install something above the peers to adjudicate.
Drystone's contrast is a design that refuses an apex, resolving conflict without electing a privileged point
of final authority. Caveat flagged: "apex" is the author's coinage for this argument, not a distributed-
systems term of art like "leader" or "coordinator."

## Thread D — proof-of-personhood research (prior-art, not distilled into a doc)

Directly relevant to the persona/peer/personhood vocabulary and the governance/activism layers. Preserved
as candidate `philosophy/prior-art` or `ECOSYSTEM` material.

- **The clean layering (from a prior turn, restated):** the rights-holding actor is a **principal**
  (standards term, covers humans/devices/agents/services; supports on-own-behalf vs on-behalf-of-another via
  **delegation**, per OAuth token-exchange; impersonation is distinct); the one-per-human governance weight
  unit is **proof of personhood** (property = Sybil resistance; unit = personhood credential). Principal is
  identity-of-actor and silent on weight; personhood is weight-counting and silent on identity. The DID
  layer maps: DID **subject** (the human), **controller** (proves control / makes changes), **delegate** (a
  device granted acting-permission via a verification method). The attack of collecting disproportionate
  weight by multiplying presence is the **Sybil attack** (Douceur).
- **Survey #1 — Shilina (Paradigm):** a field-mapping literature review of PoP as a category (history,
  advantages/challenges, use cases, verification mechanisms, comparison). Web3-oriented, taxonomy-and-
  comparison, not a proposal.
- **Survey (more rigorous) — Siddarth, Ivliev, Siri & Berman, "Who Watches the Watchmen? A Review of
  Subjective Approaches for Sybil-Resistance in Proof of Personhood Protocols" (Frontiers in Blockchain,
  2020).** Disclosed COI (authors built Democracy Earth's Equality Protocol / Proof of Humanity / Idena).
  Thesis: the best PoP tech does not abstract away subjectivity but embraces it. PoW = one-CPU-one-vote,
  PoS = one-dollar-one-vote, both collapse to plutocracy; governance needs one-person-one-vote → reliably
  signal unique humans. Identity must be **unique** (no two share an id) and **singular** (one person can't
  get more than one). The **Decentralized Identity Trilemma**: Sybil-resistance + self-sovereignty + privacy
  (framed as a trade-off, but sourced to blog posts, not proven — a lens, not an impossibility result). PoP
  needs a subjective substrate ("human entropy": voting, interpreting, being present, interacting; easy to
  produce once, hard to produce twice, hard for AI to replicate) plus an objective incentive (a crypto UBI
  making honest membership worth more than selling your credential). Comparison axes: decentralization,
  privacy preservation, scalability. Primitives: reverse Turing tests (decaying as CAPTCHA trains AI; must
  be human-generated; fail against human-generated attacks), pseudonym parties (physical presence, high
  anonymity + accountability, but non-permanent and needs re-sync), **web of trust** (certificates signed by
  other users; directly the GnuPG/PGP model, with PGP's historical failures named: trust levels not
  quantifiable, only first-degree fully trusted, remote/low-infra users can't get signatures),
  intersectional identity (Simmel: sociality/intersectionality/redundancy, no single chokepoint), TCR, DAOs.
  Seven projects (Idena the standout: FLIPs + virtual pseudonym party, fully decentralized + privacy-
  preserving, but high coordination cost and unproven AI-hardness; Humanity DAO terminated Jan 2020;
  Kleros Proof of Humanity; Upala; BrightID; Duniter; Equality Protocol). Takeaways: the taxonomy is
  artificial (real systems are hybrids); web-of-trust core is unproven at scale (Facebook removes ~2B fake
  accounts/quarter via ML); perfect Sybil-resistance may be the wrong target (bounded penetration is
  tolerable); these systems grow at a "human rate" (each project ~500–4,000 verified identities in 2020).
  Staleness flagged: 2020 snapshot, predates Worldcoin/World ID biometrics, the 2024 MIT/OpenAI personhood-
  credentials reframing, and Idena's 2022 puppeteer-pool crisis (which its own "marketplace for false
  identities" worry predicted).
- **Foundational papers:** Ford & Strauss (2008) pseudonym parties (uniqueness from one-body-in-one-place,
  no identity required); Borge et al. (2017) coined "proof of personhood" (accountable pseudonyms; one-
  human-one-unit as a PoW alternative); Adler et al. / MIT-OpenAI (2024) "personhood credentials" (same
  offline-physical-anchor bet, reframed for an AI adversary). Consistent goal: one-human-one-credential
  without identity disclosure; consensus (per the 2024 paper and Buterin): no ideal PoP, combine methods.

## Thread E — licensing the protocol (from an earlier turn in the paste)

No AGPL-for-prose exists because copyright can't reach the ideas/name. The stack the careful open-protocol
efforts converge on: spec text under **CC BY (or BY-SA)**; reference implementation under **AGPL-3.0**; a
**patent non-assertion** covering implementers; a **registered word mark on the name** + a one-page
conformance/usage policy so the name only denotes conforming implementations (this is the real lever against
"rebrand and misrepresent" and against false conformance claims); nominative fair use still lets others
truthfully refer to the protocol. The license makes the text open; the mark protects the meaning. (Not legal
advice; trademark is jurisdiction-specific.) This aligns with the spec README's existing Noria-foundation /
CC0-text framing and is relevant to the governance layer's IP-stewardship decisions.

---

*End of preserved tail. The next session (Claude Code, filing) landed the consolidation as document-pass-6:
overwrote `part-1-reasoning-underpinnings.md` and `part-2-certifiable-design.md` with the p9 content, filed
`conventions-and-decisions.md` + the two per-part changelogs, updated the canonical `impl/doc-writing-method.md`
to the newest version, and flagged the now-superseded standalone companions (persona-definition, open-items,
bounded-contexts, review-handoff, the SVGs) as retained-pending-retirement. Per the user, a content audit of
the consolidated Part 1/Part 2 against all previously-incorporated material follows, to confirm the web
agent's known content-loss did not drop anything in the consolidation.*
