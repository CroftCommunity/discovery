# Identity and data-ownership poles: where Croft sits between Solid and DSNP

`Status: cairn layer (Layer 3, the open field). Register: survey / positioning. Resolution: library — the
two identity-and-data-ownership poles Croft sits between, and why it relates to each the way it does. All
external facts were web-verified 2026-06-22; versions, products, and commercial arrangements drift, so
volatile rows are marked "refresh before external use." No user-decision gates are resolved here.`

## Overview

Two mature, standards-backed answers already exist to the question Croft also answers — "who owns a
person's identity and social data, and how do applications reach it?" They sit at opposite ends of a
spectrum, and naming them precisely is what lets Croft state its own position without hand-waving.

At one end, **Solid** (WebID, Solid-OIDC, DPoP) makes each person's data a *private-by-default* store the
person hosts, which applications read from and write to *directly*. At the other, **DSNP** (with its
reference chain, *Frequency*) makes the social graph a *public utility* — identity and graph decoupled from
any single application and anchored on a shared ledger. Solid's centre of gravity is the individual store;
DSNP's is the shared, portable graph.

Croft sits *between* these poles and belongs to neither. It carries an end-to-end-encrypted private layer
that is neither a directly-readable personal store (Solid) nor a publicly-anchored graph (DSNP). It shares
DSNP's ambition to unbundle the social web and its delegation stance — acting on a user's behalf without
holding the user's master keys — while rejecting the blockchain that DSNP's reference chain relies on. The
load-bearing point of this document is that Croft's position is a *deliberate third location*, and the
reasoning for each relationship (homage to both, alignment with DSNP's goals, rejection of the chain)
travels with it so the position can be re-derived, not merely asserted.

## Charter: what this document covers

- **In scope:** the two identity/data-ownership poles as external prior art — Solid + WebID + Solid-OIDC +
  DPoP on one side, DSNP + Frequency on the other — and the reason Croft credits each, aligns with DSNP's
  goals, and rejects the chain.
- **Out of scope (and where it lives):** the AT Protocol ecosystem and Croft's complement-not-competitor
  positioning against it (`atproto-ecosystem.md`); the substrate, transport, and CRDT/MLS prior art
  (`substrate-prior-art.md`); MLS and the decentralized-MLS siblings (`mls-and-mimi.md`); cross-platform
  identity provenance and DID methods (the identity-provenance cairn note).
- **Boundary call:** this is the "which poles, and why between them" register for the *ownership model* —
  private-store versus public-graph. It is not the register for *how* Croft's private layer is built (that
  is the substrate and protocol material) and not the register for *identity provenance mechanics* such as
  DID methods and rotation (that is the identity-provenance note). Where a fact touches both, it is carried
  here only to the depth needed to place the pole.

## Solid — the private-by-default, direct-access pole

**Solid** (the project led from Tim Berners-Lee's group; *Inrupt* is the company commercializing it)
models each person's data as a **Pod**: a general-purpose, user-owned web store. Applications do not hold a
copy of the user's data behind their own walls; they read from and write to the Pod **directly**, subject
to the user's permissions. Data is expressed as RDF (Turtle / JSON-LD) over HTTP, so a Pod is
general-purpose web storage rather than a social-app-shaped database.

Identity in Solid is a **WebID** — a profile URL that identifies the person. Authentication is
**Solid-OIDC**, where the identity provider is discovered from the WebID itself (the `solid:oidcIssuer`
pointer in the profile), and tokens are bound to the client with **DPoP** — Demonstrating
Proof-of-Possession, standardized as **RFC 9449** — so a stolen bearer token cannot be replayed by a party
that does not hold the matching key. Access is governed by **granular per-file ACLs**, and the stance is
**private-by-default**: nothing is public unless the owner grants it.

Why this is a pole and not just a neighbour: Solid answers the ownership question by making the *store*
personal and having apps reach into it directly, so the user's control is expressed as control over one
place they host. That is the opposite end from a publicly-anchored graph, and it is also distinct from the
public-indexed-pipeline model of the broader indexed social web. Croft's relationship is **homage and
learn-from**: Croft takes the private-by-default posture and the "the user holds the data, not the app"
principle seriously, but does not adopt direct-read-from-a-personal-store as its mechanism — Croft's
private layer is end-to-end encrypted, so no application (and no store operator) can read it by reaching in,
which is a stronger confidentiality stance than ACL-gated direct access where the store operator can read.

`[verified: web 2026-06-22.]` Solid and its specs are W3C-standards-based and the project is live; the
DPoP binding is fixed by RFC 9449. The commercial arrangement (Inrupt commercializing Solid) and product
maturity are the volatile facts here — refresh before external use.

## DSNP + Frequency — the social-graph-as-public-utility pole

**DSNP** (the Decentralized Social Networking Protocol, from *Project Liberty*, Frank McCourt's initiative)
answers the ownership question the other way: it treats the **social graph as a public utility**,
deliberately **decoupling the graph and the data from the application layer** so that no single app owns the
relationships. The pieces are:

- **Identity is a keypair**, and it is **platform-unrevokable** — because the identity is anchored on a
  shared ledger rather than in a company's user table, no platform can take it away.
- The **social graph is a set of portable, on-chain events**, so the graph moves with the person across
  applications instead of being trapped inside one.
- **Content** is handled as on-chain announcements that point to off-chain media, keeping bulk data off the
  ledger.
- There is **no built-in cryptocurrency token in the core protocol** — a deliberate choice that separates
  the "public utility for the social graph" idea from token speculation.
- **Delegation** lets user-agents act on a person's behalf **without surrendering the person's master
  keys** — the everyday-app key can be delegated and revoked while the root of the identity stays with the
  user.

The reference chain is **Frequency**, a **Polkadot parachain** whose economic model uses **capacity and
staking rather than per-transaction fees**, so ordinary social actions do not incur a per-message toll.

Why this is a pole and not just a neighbour: DSNP answers the ownership question by making the *graph*
public and shared and anchoring identity on a chain, so the user's control is expressed as portability
across any app that speaks the protocol. Croft's relationship is **homage and learn-from, with a specific
rejection**: Croft shares two of DSNP's goals directly — **unbundle the social web** (the graph and data
should not be captive to an application) and **delegation without surrendering master keys** (act on the
user's behalf with a scoped, revocable key while the root stays with the user). Croft **rejects the chain**:
it does not anchor identity or the social graph on a blockchain ledger, because that reintroduces a global
consensus layer and a shared public substrate, which is at odds with Croft's private-by-encryption,
member-scoped design. The goals are shared; the mechanism (a ledger) is not.

`[verified: web 2026-06-22 — dsnp.org and secondary reporting.]` DSNP is live with a published whitepaper
and the Frequency parachain. The parachain's economic details (capacity/staking model) and the
Polkadot-parachain relationship are the volatile facts — refresh before external use. `[confirm: the
"no built-in token in the core protocol" claim against the current DSNP core spec, since ecosystem token
arrangements around a protocol can change independently of the core.]`

## Where Croft sits — the third location

The two poles are not a line Croft slides along; they are two different answers, and Croft is a third.

```
   Solid pole                    Croft                       DSNP pole
   private-by-default            E2EE private layer          social graph as
   personal store;              (neither a directly-         public utility;
   apps read/write              readable store nor a         identity+graph on
   DIRECTLY into the Pod        publicly-anchored graph)     a shared chain
   ────────────────────────────────────────────────────────────────────────▶
   control = your store         control = your keys          control = portable graph
   operator CAN read            no one but members can read  graph is PUBLIC by design
```

- **Against Solid:** Croft keeps the private-by-default posture but replaces direct-read-from-a-store with
  end-to-end encryption. In Solid a permitted application (and the Pod operator) can read the data; in Croft
  a non-member cannot read a record because they lack the keys, not because a service refused them. Croft's
  confidentiality is cryptographic, not access-controlled.
- **Against DSNP:** Croft keeps the unbundle-the-social-web and delegation-without-master-keys goals but
  removes the ledger. There is no global chain anchoring identity or the graph; Croft's private state is
  member-scoped and encrypted rather than publicly portable-by-anchoring.

The reason to carry both relationships explicitly is the anti-rollup rule: the decision (Croft is a third
location, neither pole) is only trustworthy if the reasons (E2EE instead of direct read; delegation and
unbundling kept, but the chain rejected) mature alongside it. A future reader who is told only "Croft is
between Solid and DSNP" cannot reconstruct why; a reader given the two rejections and two alignments can.

## What this establishes (and does not)

Establishes that two mature, standards-backed poles already answer the identity-and-data-ownership question
— Solid (private-by-default, apps read/write directly to a user-owned Pod; WebID + Solid-OIDC + DPoP/RFC
9449; per-file ACLs) and DSNP (social graph as a public utility on a shared ledger; keypair identity,
portable on-chain graph, no core token, delegation without master keys, Frequency as the reference
Polkadot parachain). Establishes that Croft is a deliberate third location: an end-to-end-encrypted private
layer that is neither pole, sharing DSNP's unbundle-and-delegate goals while rejecting the chain, and taking
Solid's private-by-default posture while replacing direct-store-access with cryptographic confidentiality —
with the reasoning for each relationship carried so the position can be re-derived.

Does **not** cover the AT Protocol ecosystem or Croft's complement-not-competitor stance toward it
(`atproto-ecosystem.md`), the substrate/transport/CRDT/MLS prior art (`substrate-prior-art.md`), or the DID
methods and identity-provenance mechanics (the identity-provenance cairn note). Does **not** re-verify the
volatile facts — commercial arrangements, product maturity, and the parachain economic model drift and are
flagged for a refresh pass against primary sources before external use. Does **not** resolve any
user-decision gate.
