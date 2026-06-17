# The "geer" — an opt-in gating peer that CAN see content, for moderation only

date: 2026-06-17
status: thinking (design exploration). Problem / Approach / Reasoning. A deliberately non-blind,
opt-in, disclosed, revocable moderation role — the consented exception to blind-by-default. Explores
whether content-visible moderation can exist without becoming the extractive choke or a backdoor.
Sibling to the blind meer (`meer-superpeer-design.md`); a dial within the confidentiality tiers.

## Problem

Blind-by-default is Croft's strength and its limit: a blind meer **cannot proactively moderate
content** (it can't read it). Reactive moderation (member reports → revocation, the immune-signal +
threshold model) handles a lot, but three real needs push past it:

1. **App-store / legal reach (the Rave problem).** A community that wants App-Store distribution or to
   operate at semi-public scale may *need* demonstrable content moderation to survive a gatekeeper.
2. **Wanted moderation.** A moderated space is a *feature* — people join moderated communities
   *because* they are moderated. Self-policing by members doesn't scale for a large active community.
3. **Proactive vs reactive.** Reports catch what a member already saw and chose to flag; some harms
   (CSAM, spam floods) a community may want caught *before* a member has to witness and report them.

The question: is there a **thesis-consistent** way to offer content-visible moderation as an explicit
*choice* — without making it the default, covert, central, or unaccountable (i.e., without becoming
Telegram or a backdoor)?

## Approach — the geer as a consented, disclosed, scoped, revocable role

A **geer** (gating peer) is a peer a group **consensually admits** that **can decrypt content for the
limited purpose of moderation** (detect/flag/remove content, support bans/blocks/policy). It is the
opt-in exception to blind-by-default. What separates a geer from a backdoor is four properties, all
required:

- **Opt-in by group governance.** A group *votes a geer in* the way it admits any member (the threshold
  dial, `revocation-authority.md`). No group has a geer unless it chose one. The system default stays
  blind.
- **Disclosed.** Every member is informed the group has a content-visible moderator — it is a *named,
  visible role*, surfaced in the UI, not a silent capability. Informed consent is the line.
- **Scoped as narrowly as the use case allows** (the visibility dial, below) — ideally far less than
  "reads everything."
- **Accountable + revocable.** A geer is a member with a special role; the group can revoke it
  (MD-G5), and its actions are attributable. It is an elected officer, not an unaccountable watcher.

This is, deliberately, **how real clubs already work** — a club has moderators/officers the members
consented to, who can see what happens in the club and enforce its rules. The geer is the protocol
expression of an elected moderator, not of a state wiretap.

### The visibility dial — a geer need not see everything

"Content-visible" is itself a spectrum; a geer should sit at the *least* invasive rung that serves the
need:

1. **Report-gated geer (least invasive).** The geer holds **no group key**. It sees only the specific
   items a member **reports** — the reporter discloses that one item for adjudication. The geer
   adjudicates reports; it never reads the stream. (Closest to thesis; mostly what reactive moderation
   already needs, with a dedicated adjudicator.)
2. **Classifier-gated geer.** Clients run a local matcher (e.g. perceptual-hash matching against a
   known-bad set — the narrow, known-CSAM case) and surface **only matches** to the geer/authorities;
   clean content is never disclosed. More proactive, far narrower than general scanning — but it is the
   contested "client-side scanning" territory and must be treated as such.
3. **Full-key geer (Tier 2, most invasive).** The geer holds the group key and reads everything. This
   is the lab spec's **Tier-2 semantic meer** with a moderation purpose. Most capable, most invasive;
   only for a group that explicitly wants a fully-moderated space.

A group picks the rung; the default group has no geer at all (rung 0).

### What ban/block needs vs what content-visibility adds

Crucial distinction: **banning/blocking does NOT require content visibility.** A blind meer + member
governance can already ban/block on **reports + member attestation + behavioral metadata** (rate,
fan-out, the failed-op/immune-signal model). What content-visibility specifically adds is **proactive
detection** — catching bad content without a member having to see and report it first. So a geer is
justified *only* for the proactive-detection need; the reactive ban/block path stays blind. Don't reach
for a content-visible geer to do what governance + reports already do blind.

### Who holds a geer

- **Best: member-moderators.** The geer role is held by a member (or a few) the group elected — content
  visibility stays *inside* the community (self-governance with eyes), not outsourced. This is the
  club-officer model and the most thesis-aligned.
- **Acceptable by consent: a delegated moderation co-op.** A community may *choose* to delegate to a
  trusted external moderation service (a "moderation-as-a-co-op"), disclosed and revocable.
- **Never: an imposed, global, default geer.** A geer imposed on all groups by a central party is the
  extractive choke we refuse — that is Telegram, not Croft.

## Refinement (2026-06-17) — an elected *role* (rights, not capability), and a *labeler*, not an enforcer

Two refinements that make the geer markedly more faithful.

### It is an elected role with rights, not a capability of a resourced box

The geer is a **role in the governance/roles layer** (the threshold dial over MLS membership), granted
by election, the way a club elects an officer. The content-decryption capability is **downstream of the
granted right** and is **revoked with the role** — it is not inherent to being a beefy peer. In the
usual case the role *lands on* a **resourced, high-availability peer** (meer-class) because moderation
wants an always-on adjudicator — but that peer is just the chosen *host*; the **right** is what makes it
a geer, and any peer the group elects could hold it. This is the line the user drew, and it is the one
that keeps "rights vs capabilities" honest: **no peer sees content because it *can*; it sees content
because the group *granted the role*.** And it is **when-needed, not every-group** — a content gate a
community elects when it has a reason (app-store reach, scale, a wanted-moderation norm, compliance),
not a standing fixture.

**Crucially, the group can stand up a *different* geer and elect it instead.** The grant is not bound
to a particular box or party: because the geer software is open and the role is a re-issuable grant, a
group that distrusts or wants to replace its content-visible moderator can **stand up a fresh geer
(different hardware, different party) and elect it in place of the incumbent** — re-issuing the
moderation key to the new holder and revoking the old. This is the everyday, low-drama check on an
entrenched or curious geer (distinct from the trap-door fork, which is the adversarial backstop), and
it is what makes the geer's authority a genuine *delegation* rather than a de-facto office: a
content-visible role is only as concentrated as the group's inability to replace it, and the group can
always replace it. (Caveat: a *former* geer at rung 2/3 already *saw* past content — replacement stops
future visibility, it does not un-see history; rung-1 report-gated geers minimize even that.)

### Its output is *labels*, not bans — the Bluesky/Ozone intersection

Rather than a geer that *directly* removes/bans, model its output as **labels** — the proven,
composable atproto moderation primitive (Bluesky's **Ozone** + independent **labelers** that publish
labels via `com.atproto.label.subscribeLabels`; stackable, mix-and-match, advisory metadata clients
interpret *per user preference*; usable beyond hide/block — for curation and verification too). Casting
the geer as **a labeler with elected, group-scoped content access** gives three faithful properties:

- **Separation of powers.** The geer *labels* (sees content by elected right, emits a judgment); the
  **group's governance, or each member's client policy, decides the *action*** (hide / blur / warn /
  remove / ban / nothing). The geer cannot unilaterally enforce — it advises; enforcement stays with
  governance. That is a real separation the simple "geer bans" model lacks.
- **Composable / pluggable, no single all-seer.** A group can elect **multiple** labeler-geers (e.g. a
  known-bad-hash labeler + a spam labeler + a community-norms labeler), and members can **choose which
  labels to honor** — moderation as a *market of elected labelers*, not one all-seeing authority. This
  is exactly Bluesky's stackable model, and it is far more thesis-aligned than a monolithic moderator.
- **Labels do more than remove.** Curation, verification, content-warnings — the same primitive that
  gates abuse can *enrich* (the "label layer" from the social-graph design: opt-in labels over a
  structural layer). A geer is then not only a gate but a curator a community elected.

**Scope across lanes (S2 caution):** *account-level* labels (a judgment about a DID) can be portable —
shared reputation across the public (Lane 3 atproto) and private worlds, since identity (the DID) is
shared. *Content-level* labels inside a private group **must stay in-group** — exporting them would
leak the existence/shape of private content (the S2 structure-leak). So: account labels may travel;
content labels are group-scoped.

**What this does and does not fix:** it improves separation-of-powers, composability, and aligns with a
proven non-extractive model — but it does **not** change the compellability calculus for content-visible
rungs: a labeler at rung 2/3 still *sees* content, so what it sees remains compellable. The report-gated
rung-1 labeler (no key, sees only disclosed items) is the one that also shrinks the compellable surface.

## The argument AGAINST (the part that must not be waved away)

The strongest objection is **compellability**, and it is serious:

> Signal's real power is that it *cannot* comply — there is no content to hand over, so it cannot be
> *ordered* to. The moment a content-visible geer *can* exist in the system, the "we cannot" shield is
> gone: a court can order a geer to be installed, can compel an existing geer to hand over content, or
> can mandate that all groups run one. Capability invites compulsion. A blind system is protected by
> impossibility; a geer-capable system is back in Telegram's position — *you can, therefore you must.*

This is the camel's nose: "moderation visibility" is the exact justification every backdoor uses, and
once the affordance exists, the legal and product pressure is toward making it default and then
mandatory. The mitigation — **keep the system default firmly blind and the geer strictly per-group
opt-in + disclosed + revocable** — preserves "the *system* cannot, only groups that *chose* it can,"
which is a meaningfully stronger position than Telegram's "we can, everywhere." But it does not fully
escape the slope: a regulator could mandate geers, and a geer-capable codebase makes that order
*possible to obey*, where a purely-blind one does not. **This tension is the heart of the geer idea and
should be decided deliberately, ideally with legal input, not resolved by engineering optimism.**

## Reasoning

- **Why it's worth having as an option:** it directly solves the Rave/App-Store problem and the
  wanted-moderation need *for the groups that choose it*, while the privacy-max default stays blind.
  That is a genuinely better product answer than "all-blind-no-moderation" (which can't reach app
  stores) or "all-readable" (Telegram). Moderation becomes a **governance dial**, consistent with
  Croft's whole "policy is a per-group choice" posture.
- **Why disclosure + consent + revocability + minimal-scope are non-negotiable:** they are precisely
  what distinguishes an *elected club moderator* (legitimate) from a *covert/mandated backdoor*
  (the thing we exist to refuse). Drop any one and a geer becomes a backdoor.
- **Why the default must stay blind:** the system's protective value (and its honesty) is the
  blind default. A geer is the exception a group opts into, never the rule — and the compellability
  argument means the *less* the system can do by default, the safer everyone is.
- **Why ban/block stays blind:** conflating "moderation" with "content visibility" over-reaches —
  most enforcement (ban/block/remove on report) needs no content, only governance. Reserve the
  geer's eyes for the narrow proactive-detection case, at the least-invasive rung that works.

## Open edges

- **Compellability is the load-bearing open question** — needs legal review: does offering an opt-in
  geer materially weaken the "cannot comply" shield for *all* Croft users, and can that be bounded?
- **Cryptographically-enforced scope, not just policy:** can a report-gated or classifier-gated geer's
  limited view be *enforced by the crypto* (it literally only gets the disclosed items) rather than
  trusted by policy? That would make rung 1/2 much stronger than "we promise it only looks at reports."
- **Rogue / curious geer:** a geer with the key (rung 3) is a Tier-2 trust concentration; how does the
  group detect a geer exceeding its mandate (reading beyond reports, leaking)? Ties to the
  prove-blindness problem in `meer-superpeer-design.md` and the immune-signal model.
- **Classifier governance (rung 2):** who defines the known-bad set, how is false-positive harm
  bounded, and does it become a censor — the full client-side-scanning debate, inherited.
- **Lane fit:** a geer makes most sense on **Lane 2 clubs** and public-ish communities, least sense on
  **Lane 1** intimate groups (which should stay blind/self-moderated). Confirm a geer is never even
  offered for the most-private lane.
- **Naming:** "geer" (gating peer) as sibling to "meer" (always-on peer) — keep the meer blind by
  definition; the geer is the explicitly-non-blind role, so the names stay honest.
- **Labeler composition (Bluesky intersection):** how a private-group geer's labels compose with
  public atproto labelers under one DID; the label vocabulary; whether members can subscribe to
  *different* label sets within one group without fragmenting the shared view; and keeping
  content-level labels from leaking out of the private lane (S2). Bluesky refs:
  [stackable moderation](https://bsky.social/about/blog/03-12-2024-stackable-moderation),
  [Ozone](https://github.com/bluesky-social/ozone).
