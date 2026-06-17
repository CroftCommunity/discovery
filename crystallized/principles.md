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
