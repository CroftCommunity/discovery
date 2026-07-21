# Croft: the long-form narrative

date: 2026-06-15

status: drafting. The full why → what → how story for humans, synthesized from the dossier, the
thinking docs, the crystallized principles, the research, and the proofs. Cross-links the six
verticals in `verticals/`. Name decision: `../NAMING.md`.

---

## Why

There is a cycle, and it is old. People use a networked tool to build community. The community
creates real value through its own posts, moderation, social graph, and accumulated memory. Then
that value is extracted, enclosed, or simply erased — by acquisition, by API enclosure, by
monetization-of-the-user, by repurposing for an owner's agenda, or by a shutdown that deletes
decades of collective memory at a company's convenience. Usenet, GeoCities, MySpace, Yahoo Groups,
Google Reader, Vine, Reddit, Twitter. The mechanism varies; the shape recurs. (The full history is
in `../research/social-platform-cycle.md`.)

The usual diagnosis is Doctorow's enshittification — platforms are good to users, then abuse users
to favor business customers, then claw back all the value, then die. That describes the behavior.
It does not explain it. The deeper diagnosis is a capital-structure one: venture funding requires
an exit, an exit requires an extraction story, and extraction is the rug-pull. The technology of
lock-in — a captured social graph, non-portable identity, data you do not hold — supplies the
leverage that makes extraction possible without losing the users. The cycle is the marriage of an
extraction imperative to an inability to leave.

This is not new. It is enclosure. For centuries, land that was held and worked in common was fenced
into private property, and the people who had worked it were cleared off. The platform that invites
you in, lets you build your home and your audience on its land, then fences it and rents it back —
that is enclosure, six hundred years on, made of Terms of Service instead of hedgerows. The
honest reading of the commons is not Hardin's "tragedy" (a theory adopted to justify privatization)
but Ostrom's: communities have stably governed shared resources for centuries when the structure
let them. (Civic grounding: `verticals/the-civic-why.md`.)

So the antidote cannot be "a nicer company." A nicer company is still a company with an exit. The
antidote has to be structural: an ownership form that legally cannot be sold out, identity and data
the user actually holds, an open protocol, and — the part most attempts skip — a user experience
and brand stable enough that a normal person will actually use it. You need both halves, and you
have to commit to the governance half at inception, because the history is clear that you cannot
retrofit it after taking growth capital.

## What

**Croft** is the name and the center of gravity. A croft is a small worked plot of land. For
centuries crofters tended ground they did not legally own and could be cleared off at a landlord's
whim — until they organized, held their ground, and won *security of tenure*: the right to stay on
the land they had improved, to pass it on, and to be compensated for what they built. The 1886 Act
that granted it was called "the Magna Carta of Gaeldom." The name earns its power by being
unromantic: the croft began as engineered dependency (plots deliberately too small to live on, to
tie labor to the landlord), and the win was real but partial (tenancy, not ownership; a floor, not
a destination). That is exactly the shape of a user's relationship to a platform, and exactly the
right the project builds into software — you cultivate your place here, and no one gets to clear
you off it. The kinship with *craft* (worked skill, made by hand, the maker not the product) is the
voice; "by Croft" is the provenance mark. (Full rationale: `../NAMING.md`.)

What Croft is, concretely:

- A place to gather and talk — private group messaging, and a local-social feed of the people in
  your circles, in the order they posted, with no engagement algorithm deciding what you see.

- Owned by the people who use it, through a cooperative, not by investors who will eventually need
  to extract from it.

- Built on open protocols, so your account, your social graph, and your data are yours to keep and
  to take with you. Public content can flow to the wider world through AT Protocol (Bluesky); private
  group messaging stays on an encrypted peer-to-peer path that the infrastructure cannot read.

The load-bearing technical idea is the **lineage-group protocol**. Groups are not rows in a
company's database; they are encrypted, append-only, hash-linked histories that members hold. The
design resolved, through hard reasoning (`../thinking/design-notes-addendum.md`,
`verticals/lineage-group-protocol.md`), into a small set of moves:

- **Provenance is the dual primitive.** The same cryptographic ancestry that proves who said what
  is also what lets diverged groups find their common ancestor and reconcile. One mechanism
  (Merkle-style ancestry + replay-forward) handles multi-device sync, clean group merges, and
  splinters — they are all "two histories that share an ancestor reconciling."

- **The fork is the recovery primitive, not a failure.** Complementary divergence (nobody made
  contradictory claims) converges automatically. Contradictory divergence (someone ejected on one
  branch, re-added on another) is *detected and presented with full attribution*, never
  auto-resolved, because that is a social judgment, not something a tree can compute. And under
  everything sits the trap door: fork and start new history, always available, with the local-first
  view keeping even a divergent state searchable and livable. Every failure path ends somewhere
  legitimate.

- **Hard breaks are a social problem, not a cryptographic one.** Cryptography cannot prevent an
  undesired outcome; it can only make the response legible, attributable, and cheap. The honest
  guarantee is: any two members can always determine their common ancestor and converge where their
  histories are complementary, and can always see a true account of where and why they are not — but
  the system does not promise they end up in the same group, because that is theirs to decide.

The encryption is MLS (per-epoch group keys, forward secrecy), the shared state is Automerge CRDTs,
large media are encrypted content-addressed blobs, identity is an AT Protocol DID, and the transport
is a peer-to-peer stack with an optional always-on broker. Critically: **all peers are equal in
rights, but not in capabilities.** A bigger peer can do more (storage, uptime, relaying); it can
never *decide* more. The moment a capability difference becomes a rights difference, the design has
leaked. (Identity and the unsolved recovery problem: `verticals/multi-device-and-identity.md`. The
graph you hold, non-extraction, visibility regimes: `verticals/social-graph-you-hold.md`.)

## How

The technical substrate is mostly assembly of pieces from separate worlds, not new research. The
honest feasibility pass (`../thinking/open-considerations.md`, and the origin dialogue in
`seeds/transcripts/raw/p2p-architecture-origin-dialogue.md`) is clear about what is proven and what
is hard:

- Proven: peer-to-peer transport, gossip, content-addressed blobs, a self-hosted PDS, MLS via
  openmls, Lightning settlement. The encrypted-sync core was built and verified end-to-end
  (`../Proofs/encrypted-local-first-atproto/`).

- Hard, and named honestly: cross-platform mobile (iOS and Android background delivery need push;
  no P2P stack has a turnkey iOS story — the pragmatic path is a thin Rust core compiled to native);
  the web client (a browser cannot be a persistent peer — the coop provides a blind bridge); true
  metadata-blind sync (research); and the one weakest seam, **identity/key recovery**, which is the
  hardest usability-vs-security collision in the whole space and is held open, not pretended solved.

The thing that makes Croft durable is not the software. It is the **cooperative**. Not a nonprofit
(grant-dependent, dies on the funding cycle) and not a startup (investor-funded, must eventually
extract). A cooperative — member-owned, like a credit union, not a club. You are a member because
you use the utility, not because you joined a community. The coop runs the optional always-on HA
nodes (each one a member's backup, content server, prekey/push relay, and Bluesky PDS in one box),
employs the developers, and can run one of several competing, replaceable ad brokers. Its character
is custodial, not insurgent: the energy is *resistance-as-maintenance* — an eternal brace against
forces that never go away, the seawall not the raised fist. The point is not to be the only place;
it is to be the place that is still here, unchanged in what it owes you, in twenty years.

The cooperative is the maintenance plan. The survey of prior attempts is littered with
maintenance-phase casualties — founders move on, projects get archived, the revolution was the easy
part (`../research/p2p-founder-motivations-adoption.md`). So the structure is designed to answer the
question that kills these projects: *who is still here in year seven, and why?* That means:

- A **governance lifecycle**: founder stewardship early (with an 80% member break-glass), a defined
  transition to member governance on a compound trigger, and a separation of technical / product /
  governance / mission decisions so early agility is not strangled by democracy.

- A **founder royalty**, not an exit: a small percentage of revenue, paid only after sustainability,
  proportional to success but muted, that never squeezes (legally a licensing fee, compatible with
  cooperative limited-return norms).

- **Anti-enshittification as binding charter**, not a blog promise: 6-month deprecation cycles, an
  18-month LTS interface, no dark patterns, guaranteed data portability, an endowment cap (surplus
  past it is returned to members or spent on defined mission purposes), and public ad-revenue
  transparency. (The cooperative in full: `verticals/the-cooperative.md`.)

And the strongest, most differentiating commitment — the structural anti-rug-pull guarantee
(`../thinking/governance-and-survivability.md`): not a promise of permanence, but a **guaranteed
graceful exit**. If Croft ever shuts down, your data already lives on your own devices; a pre-funded
static encrypted archive remains downloadable for a bounded window from storage the coop pre-paid,
held by a bankruptcy-remote steward the coop's insolvency cannot touch; and the code is open so a
successor can take over. You decrypt with your own key, which the coop never held. The economics of
a static encrypted bucket are cheap enough to actually pre-fund — which is what turns the promise
from marketing into something a separate trust can hold.

The economic layer (`verticals/substrate-and-economics.md`) inverts the attention economy rather
than reproducing it: an opt-in, consumer-driven ad exchange where users declare the ad types they
will see (declared intent, not surveillance), the match happens locally, and advertisers pay the
user directly via Lightning. Brokers are replaceable services chosen consumer-side, so unethical
brokers get dropped. Recognition and reputation are namespace-scoped, countersigned, append-only —
honest by construction, with no rewind. No new token; an existing respected currency for settlement.

## The honest verdict

This is not much new technology. It is the alignment of needed pieces from separate worlds, plus a
legal and financial structure that makes the continuation of the thing possible. The survivors in
this space (Wikimedia, Signal) endured not because they shipped better software but because they
solved funding and governance in a way that removed the extraction imperative. Croft's bet is that
coupling that durable structure with the user experience and brand stability the community side has
historically failed to deliver is a real, under-occupied position — decentralized values delivered
as one cohesive, stable product rather than a fragmented protocol with twelve clients.

The remaining risks are honest and named: identity recovery, mobile reliability, the web bridge, and
the adoption curve — a cooperative grows slowly by nature, and that is a feature (aligned incentives)
as much as a constraint. The open design questions live in `../thinking/open-considerations.md` and
`../crystallized/conclusions.md`. The single most important one — *what is the actual product a user
opens first* — is the thing to settle before the rest, because everything downstream flows from it.

The point is not to win. It is to persist as an option, an antidote available to those who want it.
Every revolution has a maintenance phase. Croft is the maintenance plan.
