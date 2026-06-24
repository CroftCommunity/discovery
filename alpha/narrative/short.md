# Croft: the short narrative

date: 2026-06-15

status: drafting. The max-3-page version of the story. The full version is `long-form.md`; the
standalone topic narratives are in `verticals/`.

---

## The problem

There is an old cycle. People build community on a networked tool. The community creates the value
— posts, moderation, the social graph, years of memory. Then the value is extracted, enclosed, or
erased: acquisition, API enclosure, monetization-of-the-user, repurposing, or a shutdown that
deletes decades of collective memory at a company's convenience. Usenet, GeoCities, MySpace, Yahoo
Groups, Reddit, Twitter — the mechanism varies, the shape recurs.

Enshittification describes the behavior. The cause is structural: venture funding needs an exit, an
exit needs an extraction story, and extraction is the rug-pull. Lock-in — a captured graph,
non-portable identity, data you do not hold — is the leverage that lets it happen without losing the
users. It is enclosure: land worked in common, fenced and rented back, now made of Terms of Service.

So the antidote cannot be a nicer company. It has to be structural, and committed at inception,
because you cannot retrofit it after taking growth capital.

## What Croft is

**Croft** — named for the small worked plot a crofter could not be cleared off once they had
tended it (security of tenure, the "Magna Carta of Gaeldom"). The name is unromantic on purpose:
the croft began as engineered dependency and the historic win was real but partial. That is exactly
the right the project builds into software — you cultivate your place here, and no one clears you
off it. (Kinship with *craft*; "by Croft" as the provenance mark.)

- A place to gather and talk: private encrypted group messaging, and a local-social feed of your
  circles in chronological order — no engagement algorithm.

- Owned by its members through a cooperative, not by investors who must eventually extract.

- On open protocols: your identity, graph, and data are yours to keep and take. Public content can
  reach the world via AT Protocol (Bluesky); private messaging stays on an encrypted P2P path the
  infrastructure cannot read.

The technical heart is the **lineage-group protocol**: groups are encrypted, append-only,
hash-linked histories that members hold. Provenance is the dual primitive — the ancestry that
proves who said what also lets diverged groups reconcile. The fork is the recovery primitive, not a
failure: complementary divergence converges automatically; contradictory divergence is detected and
presented with full attribution but never auto-resolved, because that is a human judgment; and the
trap door (fork and start fresh) always exists, so no failure path is a dead end. Encryption is MLS,
shared state is Automerge, identity is a DID, transport is P2P with an optional always-on broker.
The governing rule: **all peers are equal in rights, never in capabilities** — a bigger peer does
more, it never decides more.

## How it lasts

The software is mostly assembly of existing pieces, not new research. What makes Croft durable is
the **cooperative** — member-owned like a credit union, not a club; not a nonprofit (grant-dependent)
and not a startup (must extract). The coop runs the optional always-on nodes (each a member's
backup, content server, push relay, and Bluesky PDS in one box), employs the developers, and can run
one of several competing, replaceable ad brokers. Its energy is custodial, not insurgent — a seawall
against forces that never go away. The aim is not to win; it is to still be here, unchanged in what
it owes you, in twenty years.

This space is littered with maintenance-phase casualties — founders move on, projects get archived.
The cooperative is the maintenance plan, designed to answer "who is still here in year seven, and
why":

- A governance lifecycle: founder stewardship early (with an 80% member break-glass), a defined
  transition to member governance, separated decision domains so early agility survives.

- A founder royalty, not an exit: a small revenue percentage, paid only after sustainability, that
  never squeezes — legally a licensing fee, compatible with cooperative norms.

- Anti-enshittification as binding charter: 6-month deprecation cycles, an 18-month LTS interface,
  no dark patterns, guaranteed data portability, an endowment cap, public ad-revenue transparency.

- A structural anti-rug-pull guarantee: not permanence but a *guaranteed graceful exit*. If Croft
  shuts down, your data is already on your devices; a pre-funded static encrypted archive stays
  downloadable for a bounded window, held by a bankruptcy-remote steward; the code is open for a
  successor. You decrypt with your own key, which the coop never held.

The economics invert the attention economy: an opt-in ad exchange where users declare the ad types
they will see (intent, not surveillance), matching is local, and advertisers pay users directly via
Lightning. Brokers are chosen consumer-side, so unethical ones get dropped. No new token.

## The honest part

This is the alignment of pieces from separate worlds plus a structure that makes continuation
possible. The durable survivors (Wikimedia, Signal) endured by solving funding and governance, not
by shipping better software. Croft's bet is the under-occupied middle: decentralized values
delivered as one cohesive, stable product, not a protocol with twelve clients.

Named risks: identity/key recovery (the hardest seam, held open), mobile reliability, the web bridge,
and a deliberately slow cooperative adoption curve. The first thing to settle is what product a user
opens first; everything else flows from it.

The point is not to win. It is to persist as an antidote for those who want it. Every revolution has
a maintenance phase. Croft is the maintenance plan.
