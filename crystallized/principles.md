# Principles (Design + Civic)

date: 2026-06-15

purpose: the recurring, settled principles distilled from both bodies of thinking — the
Sovereign Commons dossier (civic/values/economic) and the lineage-groups design
(technical). These read as decided, not tentative. Open questions live in conclusions.md.

Two tiers: **values** (why we build, what we refuse) and **design** (how the system
behaves). They are linked: the dialogue repeatedly finds that the ethical choice and the
technical strength are the same choice.

---

## The unifying observation

> Refusing extraction forces decentralization; decentralization is what delivers
> resilience and privacy. The ethical choice and the technical strength keep turning out to
> be the same choice — not by accident, but because "knowing reliably where something came
> from" (provenance) is simultaneously the security invariant and the social-legibility
> invariant.

This is the thread that ties the civic vision to the protocol design.

---

## The deeper foundation (2026-06-20): provenance not utility, local-first as the unit

source: the design-imperative dialogue (`seeds/transcripts/raw/croft-architecture-design-dialogue-2026-06-20.md`),
its essay `narrative/lineage-of-a-design-imperative.md`, and the architecture distillation
`thinking/local-first-as-design-imperative.md`. These ground the principles below in a cross-field,
cross-millennium lineage (Socrates → Mill → Peirce/Popper → Hayek → Ostrom → Ashby → Beer → Scott) —
read as *settled foundation*, the bedrock the other tiers stand on.

- **The razor: compute provenance, never utility.** The system establishes what is *consistent and
  corroborated* (provenance — a closed, determinate question: does the chain verify); humans supply what
  is *true and right* (utility — open in principle, living in plural revisable values). A distributed
  system sees assertions and agreement patterns, never the world the assertions are about, so "is this
  true" is structurally outside its reach; the only answerable questions are local consistency and
  alignment, **neither of which is a truth claim**. Any feature that needs the system to judge worth is
  miscast. The honest mission is not truth but **trustworthy disagreement**. (Generalizes Tier-2 "never
  algorithmically adjudicate a social dispute.")

- **Local-first state is the generative premise — and the architecture's epistemology.** "The primary
  copy lives with the unit; the network reconciles" is the *same sentence* as "truth is local and
  corroborated across peers, never certified from a center." The design earns belief the way it
  describes belief being earned. Two halves of one theorem: **(1)** respecting humanity *entails*
  local-first, because the person is the unit of composability (central primary state has already made
  the person secondary); **(2)** central-truth design is *necessarily* faulty, expressing as permanent
  **friction**, because by requisite variety the center is always below the variety it governs and must
  force reality down to fit. **Necessary, not sufficient:** local-first is the required *form* of a
  humanity-respecting system, not a guarantee of one (the edge can be wrong too — variety preserved ≠
  humanity served). **Friction is diagnostic:** honest friction (real disagreement between real units)
  vs. manufactured friction (a center forcing variety it can't hold).

- **Plurality is a survival condition, not a preference (requisite variety).** "Only variety can absorb
  variety" (Ashby): a regulator that collapses plural perspectives into one certified truth is brittle
  by formal law and will be overwhelmed by the reality it oversimplified. Preserving plural
  perspectives, allowing forks, refusing to collapse divergence is requisite variety made architectural.
  *Plurality is the robustness.* The monoculture-forest failure (Scott) is the cautionary case: optimize
  for one legible metric (consensus-yield), erase the variety that gave resilience, collapse in the
  second generation. **The design job is to not be the blight** — variety is generated and selected, not
  designed; you refrain from collapsing it.

- **Design for the conditions, then get out of the way.** The conditions are knowable and finite even
  though the variety they permit is unknowable and unbounded — *you can specify the soil without
  specifying the forest.* The enabling set: secure standing, a real (cheap, dignified) exit, an honest
  non-equivocating record, accessible resolution that defers judgment, refusal to optimize toward a
  single legible value. Mostly conditions and restraints, almost no features — **the negative space is
  the design.** This is also the falsifiability discipline: giving up the authority to decide the
  outcome means you can be corrected, observably, when the variety dies.

- **No right to remove the rights of others (the one principled boundary).** Variety is permitted in
  everything except the removal of another's *standing to hold variety* — a fork creates, a clearance
  destroys. **Rights** (standing: tenure, exit, voice, share — the precondition of a legitimate
  collective) are not the collective's to remove; **roles** (governed delegation) move freely. The
  **wolf test:** any action that, generalized, would remove the conditions of its own contestation is
  illegitimate by nature (the tell is self-cancellation). Rights-removal is the *only self-amplifying
  move toward collapse* (it lowers the variety available to resist the next removal — the monoculture
  mechanism in a polity), so the rights-floor is a **consistency requirement and an equality
  requirement, not a moral overlay** (equal standing is *forced*, not asserted — unequal standing lets
  some clear others and the system dies). It is the backstop against the system becoming **incestuous**.

- **Inverse-correlation of protection: maximal freedom where exit protects you, maximal protection of
  rights where exit cannot.** Where contestation is cheap (discussion/fork/merge/lineage), get out of
  the way — the conditions self-defend, because anyone cleared just forks. Where decisions are
  **irreversible / singleton-bound**, exit-after-the-fact can't heal them, so that is where
  constitutional rigidity (highest thresholds, strictest plane, rights-removal prohibited most
  absolutely) bites hardest. The crofting frame: everything flexible except *tenure*, the irreversible
  singleton whose loss fork can't heal.

- **Equal in rights, not capabilities — applied to the equal possibility of all shapes.** The
  collective is the peer primitive at a larger scale, and its internal form is a free variable
  (household / co-op / foundation / township). Equality of rights *generates* variety of form precisely
  because it refuses to privilege one form's standing — mandating a single shape would smuggle a
  monoculture in at the structural level. (Lifts the Tier-1 equal-rights principle to collectives.)

---

## Tier 1 — Values / civic principles

source: SOVEREIGN-COMMONS-DOSSIER.md §2, §3, §7, §8

- **The enemy is centralization *capture*, not centralization itself.** "It's not central
  resources that are an issue, it's centralization capture and thus control."

- **Delegated authority, never imposed; revocable in fact, not just in form.** Roles in a group
  (admin, moderator, a content-gating `geer`, an always-on `meer`) are fine — *as long as they are
  not immutable or forced.* Equal peers may **elect a representative** and grant it rights for ease
  and consistency; the difference from an imposed authority is that the grant is a **revocable
  delegation**, reversible by the same peers who made it. That difference is significant in design
  *and* ethos even when a role is, for practical reasons, de-facto persistent. The hard part is that
  **de-jure revocability is necessary but not sufficient:** a resourced, always-on peer can accrue
  *outsized weight through sheer circumstance* (it holds the state, it is always present, few others
  can run it), so "reversible" hollows out if exit/replacement is impractical. The design must keep
  delegation **materially** reversible, not just formally — the decisive guard being that **a group can
  stand up a *different* holder (meer/geer) and elect it in place of the incumbent** (the role is a
  re-issuable grant, not bound to a box or party), backed by **state portability** (no data hostage:
  re-host on the new holder cheaply), **blindness** (a blind meer can't accrue *content* power),
  **label-not-enforce separation** (a geer advises; governance acts), the **trap-door / re-formation
  backstop** (a captured role can always be forked away from, minus the incumbent), **scoped
  non-creeping rights**, and **rotation-friendly defaults**. Routine replacement is the normal check;
  the fork is the adversarial backstop. See
  `thinking/meer-superpeer-design.md` (anti-entrenchment) and `thinking/geer-gating-peer.md`.

- **Non-extraction is the point, not a missing feature.** The reason a graph-you-hold or a
  civic notice board doesn't already exist is that there's no business model in something
  you can't extract from. The absence of an extraction model is what lets the thing be
  honest.

- **Refuse "credibly decentralized but operationally centralized."** Cryptographic
  portability that is technically real but economically meaningless because aggregation
  re-centralizes. Must survive as small self-hosted nodes.

- **The recurring inversion.** Take an extractive stateful intermediary → reduce it to
  stateless / content-blind / optional → wrap it in an institution that reinforces rather
  than extracts. Applied at five scales: relay→stateless rendezvous; relay→optional
  superpeer; routing server→content-blind mule; ad platform→consumer-side broker;
  compellable operator→cooperative.

- **Credit union, not a club.** A social utility structured to reinforce, not extract.
  Member-owned, dues-funded, surplus reinvested. The co-op is the maintenance plan.

- **All peers are equal in rights, but not capabilities.** (Refined from the dossier's
  "equal in ability, not capacity.") The protocol grants every peer the same *rights* — what
  it is permitted to do; *capabilities* — what its hardware/uptime let it do — differ. An
  always-on superpeer has more capability but must never acquire more rights by virtue of it.
  Proven by F5 (availability-as-rights-escalation): no governance right is escrowed to the
  superpeer's presence.

- **User-need-first, never data-extraction-first.** The Google+ lesson.

- **Commons are real and governable** (Ostrom, not Hardin): clear boundaries, local rules,
  collective choice — the governance DNA.

- **Free-tier mandate.** "You can be part of this if all you have is a phone with CPU,
  memory, storage, and an internet connection."

- **Different, not weaker.** Security posture is tiered and claims properties centralized
  systems can't (transparent offline, no operator to compel) — guarded by a per-tier
  properties matrix so "different" never rationalizes "weaker."

## Tier 2 — Design principles (lineage-groups)

source: thinking/thesis-lineage-groups.md, multi-device.md, social-layer.md

- **Provenance is the dual-purpose primitive.** Common ancestry / reliable fork-provenance
  is both the safety primitive and the social-legibility primitive.

- **Forward-key convergence ≠ history reconciliation.** Never conflate them. The MLS epoch
  key is single and linear; history is data and never merges into one transcript. No
  timestamp interleaving ("six tapes in a room").

- **Forks are a feature.** Under partition, contradictory-but-valid commits are inevitable.
  The right resolution is a clean, attributable, non-insulting fork: heal silently on no
  conflict; hard-stop and escalate to a human on real conflict. Never algorithmically
  adjudicate a social dispute.

- **Immutable genesis grounds the recursion.** The group's constitution (per-operation
  thresholds) is fixed at birth and is not renegotiable by the normal machinery. Stops the
  "who decides who decides" regress.

- **Per-operation thresholds track blast radius.** leave-self = 1, add = low, boot =
  higher, dissolve = highest. Lenient no-surprise defaults; strictness behind an advanced
  menu or a single "how strict?" question.

- **Keys are not identity; thresholds count lineages, not leaves.** A person is a DID
  lineage; each device is a distinct key/member; "same person" is recovered one layer up.
  Two signatures from one lineage count once — the defense against manufacturing a quorum
  from your own devices.

- **Reconciliation is consensual backfill within a lineage.** A member gifts a branch; the
  recipient verifies it chains to a shared genesis and chooses to absorb it. Self-sync
  across your own devices is the same operation. No server-side source of truth.

- **Fail early, fail clearly.** Stale must be visible; unavailable and murky are not
  allowed. The no-broker tier is the degraded tier and must say so — never silently.

- **Freshness is a first-class signal; absence-of-news is not evidence of currency.** Every
  "stale is visible" guarantee was comparative — it needed a peer ahead of you to compare
  against. A peer that hears from *no one* cannot tell it is behind, and silence must never be
  rendered as currency. The mechanism is a periodic signed **tip beacon** (head/epoch/seq +
  routing only — content-free, blind-broker-safe, ties to AR-4) plus a local *time-since-last-
  heard* clock (liveness is measured locally, never from peers' wall-clocks). The invariant is
  **no-false-current**: a view may show "current" only if it both matches the best-seen tip *and*
  was confirmed within the tier's freshness horizon; otherwise it degrades visibly to "behind" or
  "unverified." Freshness also **gates authority** — do not act on a membership/revocation decision
  authorized against a group view you cannot prove is current. (See `thinking/freshness-signal.md`;
  tested by E2.16.)

- **Structural, not runtime, enforcement for safety limits.** A violating share is
  unrepresentable / rejected by every verifier — never merely warned against on the
  sender's client. The safety case is exactly the hostile sender.

- **Layer separation makes the social graph safe.** Structural graph layer vs. opt-in label
  layer. Most platforms fuse them; that fusion is the invasiveness.

- **Freeze by default; quiet membership; multi-identity as a fact of life** (S1, S3, S4).
  Scoped visibility, not opaque structure, because topology deanonymizes (S2).

- **Content is born into a visibility regime and cannot silently change it.** Crossing
  intimate→public is a deliberate new authored republish, never a forward.

- **Openness caps propagation depth.** A large public group is a visibility sink, not a
  conduit. Inward visibility and outward propagation are independent parameters.

- **The broker is de-facto-mandatory for mainstream UX; "optional" means graceful
  degradation, not the common path.** State this plainly (from the research synthesis).

- **Never trade away forward secrecy for decentralization convenience** (Session's mistake;
  MLS gives PFS/PCS by design).

- **Group size is three products, not one knob.** Interactive / quiet-large / broadcast —
  guarantees degrade by behaviour, not a hard size cap. Type is chosen at creation, never an
  in-place mode switch. Real-time signals (presence/typing) are opt-in, peer-to-peer gossip
  between live peers (broker-blind), with battery as the visible cost. (See
  thinking/interaction-tiers.md.)

- **The privacy-preserving default is the free one; convenience is the opt-in that costs
  something visible.** Forward-only history is free, backfill is work; on-device keys are the
  default, multi-device sync is the expensive thing; presence costs battery, not metadata.
  Lean into the features where privacy and ease coincide; treat imported-from-Signal/iMessage
  conveniences as deliberate, scoped design problems, not assumed table stakes.

## Tier 3 — Product / durability principles

source: the Germ/X Chat durable-product dialogue (thinking/interaction-tiers.md; raw in
seeds/transcripts/raw/germ-xchat-design-dialogue.md).

- **Settings serve three audiences, not two.** Not basic/advanced (which conflates "most
  people never touch this" with "this is simple"). The three are defined by *relationship to
  the system*: (1) those who never change anything — the defaults *are* the product, and
  their quality is the whole verdict; (2) those who'll tune one or two findable things;
  (3) those who want the full surface (disproportionately early adopters / fiercest
  defenders). Name sections by **intent, not depth** (depth-names like "advanced" always
  collapse). The middle is a curated short list of what's actually worth a normal person's
  attention; the bottom is the unfiltered surface.

  - *Composable-interface note (2026-06-16):* the three-audiences split is likely realized
    through a **composable interface** — the progression from audience (1) → (2) → (3) is users
    *reaching up into owning their own experience* and refining it by composition, not by
    flipping ever-more-obscure toggles. The audiences are not three fixed tiers so much as
    positions on a ramp of self-authorship: the interface exposes composable units a person
    assembles as far as they care to. This binds the three-audiences principle to the
    shapeability/LTS principle below — composability is the *how* of climbing, stability is what
    keeps the climb masterable rather than extractive. (Filed as a forward note; no proof/spec
    attached yet — a product-track concern under ROADMAP M2, not a validation-track item.)

- **Shapeability is only valuable paired with stability; constant UI change is quietly
  extractive.** When interfaces shift constantly, people never build a durable mental model
  or reach fluency, and "change it back" friction becomes an engagement lever. An
  engagement-maximizing product benefits from keeping users off-balance; a member-serving one
  benefits from user mastery. **LTS-for-interfaces:** alpha/beta/stable channels, a long
  (~3yr) stable window, opt-in change "trains" on a regular cadence (~6mo), the learned
  surface (layout, names, where things live) left alone otherwise. Hold the *learned surface*
  stable while shipping improvements behind it; security changes are the exception and must be
  over-communicated. Honest cost to budget: multiple live UI generations = real
  documentation/support load — name it, or the principle dies in year two.
