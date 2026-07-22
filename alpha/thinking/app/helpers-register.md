# The helpers register — optional enhancement services and their baseline degradation

date: 2026-07-22 · status: draft register (first pass; extend as helpers surface). Sibling to
`estate-architecture-and-browser-constraints.md` and `client-architecture-adr.md`.

A **helper** is an optional service that *enhances* the experience but is **not required for acceptable
usage**. The product must be fully usable at an honest baseline with **zero helpers running**; each helper
is progressive enhancement on top. This is the same discipline as the survivability / anti-enclosure
stance — because no helper is load-bearing, nothing can be taken away (or rug-pulled, or subpoenaed, or
priced) to break the baseline. It is `P-Durable-Enablement` applied to the server surface: *only ship a
guarantee you can keep enabled by default; anything too costly to run uniformly becomes a helper, not a
requirement.*

## The helper contract (the rule every helper must satisfy)

1. **Optional.** The baseline works with the helper absent. If something breaks when a helper is off, it
   is not a helper — it is a dependency, and it must be redesigned or reclassified.
2. **Declared degradation.** Each helper names *exactly what the baseline does without it* (below). No
   silent failure; the app detects-and-degrades.
3. **Isolated trust surface.** A helper is a trust and attack surface, so it is minimal, runs on its own
   origin/domain (never sharing the estate session origin), and holds the least state possible.
4. **Replaceable / self-hostable.** No specific centralized instance is required; a user or co-op can run
   their own. Swapping the helper does not change the baseline.
5. **Reads stay direct.** Public reads never route through a helper; only the enhancement path does.

## Register

| Helper | Enhances | Baseline WITHOUT it (the honest degradation) | Trust surface / where | Status · source |
|---|---|---|---|---|
| **Custom AppView** | rich read / index / feeds / search over the network | public AppView (`public.api.bsky.app`) + direct `getRecord`/`listRecords`; **pdsview proves AppView-less browsing works** | read/index authority; own infra | live pattern · `ECOSYSTEM.md` §5f, `research/atproto-sovereign-appview-club.md`, pdsview |
| **BFF (token)** | sign-in-**once SSO** across separately-hosted apps + DPoP key held server-side | **per-app atproto OAuth login** (fully serverless; the live pads do this) | holds session + DPoP key (highest-value) → tiny, isolated at `account.croft.ing` | optional, deferrable · `estate-architecture-and-browser-constraints.md`, `beta/OPEN-THREADS.md` T55 |
| **AP-followers restriction** | "followers-level" / roster-gated *private-but-not-E2EE* sharing (offer gated by verified roster membership) | **public** (bare atproto records) or **sealed** (MLS E2EE) — the two ends of the privacy ladder | gatekeeper appview gates *offering*, not crypto | designed · RUN-16, `experiments/appview-infra/GROUPS.md`+`PUBLICATIONS.md`, ROADMAP_TODO E43 |
| **Web Push helper** | recipient notifications when the app is closed | in-app / on-open only (no background push) | holds push subscriptions (encrypted to keys); the "shrunk helper" = push + session-attestation only | designed · card-maker/push raw (T1), Web Push |
| **Content-blind ingest shim** | **anonymous / no-login multi-writer** append (collaborative card, guestbooks) | **named-set (logged-in) writers only** — no anonymous append | time-boxed, content-blind writer; the PDS holds state | designed · `ponds/virtual-cards-and-guestbooks.md`, E43/E45, T56 |
| **iroh relay** | cross-network / behind-NAT peer connection (and browser peers, which cannot hole-punch) | **same-network (LAN) direct P2P** — the base-base case needs no relay; relay only extends reach beyond the local network | traffic broker (content-blind); swappable/self-hostable (n0 default → co-op-run) | designed · E11, `ECOSYSTEM.md` §1, relay lab |
| **Deep-link resolver** | smooth one-tap join / share (and the whole acquisition/growth path) | **manual join by pasting a ticket/code** (the claim-code "one-more-tap") | catalog + link broker | designed · `ponds/build-order.md` §0.2 — *strategically* huge for growth, not a functional requirement |
| **meer / Delivery Service** (Drystone) | store-and-forward when a peer is offline; async delivery to absent members | **direct delivery to online peers** only; offline peers catch up on reconnect | the accepted *unequal peer* role (Drystone); bounded, exitable, content-blind | designed · Drystone spec (DS role), `thinking/meer-superpeer-design.md` |

## The base-base case (the floor)

**Two people, same network, a direct connection, a manually-shared join code — no relay, no resolver, no
AppView, no BFF, no delivery service, no server of any kind.** That is "acceptable usage." Every row in the
register above is enhancement on top of this floor, and each names how it degrades back to it. If a
proposed component cannot be removed while leaving *this* floor intact, it is not a helper — it is a
requirement, and must be justified as one (or redesigned). *(Browser-peer caveat: browsers cannot
hole-punch, so cross-network browser peers lean on the relay; the LAN floor is most direct for native
peers. The floor is same-network direct; the relay extends reach.)*

The resolver makes one distinction sharp: **strategic importance ≠ functional need.** The resolver is the
growth engine and matters enormously for adoption, yet the product still *works* without it (manual code
join). This register classifies by the functional floor, not strategic weight — otherwise "growth depends
on it" quietly smuggles components back into the required set.

## Carried forward from Drystone (protocol → product)

This helper discipline is not new to Croft — it is the **product-layer inheritance of Drystone's
protocol-layer principles**, and keeping that lineage visible should inform the thinking:

- **Peer-equality floor.** Drystone's peers are equal; the base-base case (two direct, equal peers) is that
  floor made concrete at the product layer. Helpers are the *bounded, accepted unequal-peer roles* layered
  above it — exactly Drystone's stance that an unequal peer (the MLS Delivery Service / meer) is accepted
  only as a bounded, exitable role, never the thing the whole system depends on.
- **`P-Durable-Enablement`.** "Only ship a guarantee you can keep enabled by default; a feature too costly
  to implement uniformly becomes one users route around." A helper is precisely a capability too costly to
  guarantee for everyone, so it is made optional and route-around-able rather than required.
- **Exitability.** Every helper being replaceable/self-hostable is the product expression of Drystone's
  exitability backstop — no privileged role can hold the system hostage.

So the register is Croft's concrete accounting of Drystone's "which privileged roles exist, and are they
all bounded and route-around-able?" — checked helper by helper, at the product layer.

## Why tie them together

- **One contract, uniformly applied.** Every future server-side component is designed to the five rules
  above, or it is honestly reclassified as a dependency (and then justified). This keeps the serverless /
  local-first baseline real rather than aspirational.
- **The trust/attack surface is enumerated and isolated.** The helpers are exactly the places that hold
  state or keys or gate access — so listing them is also the security map (each on its own origin, minimal,
  replaceable). The BFF and AppView are the highest-value; the register makes that explicit.
- **It is the anti-enclosure proof, made checkable.** "No one can take X away and break the product" is
  only true if every X is a helper with a declared degradation. The register is where that claim is kept
  honest, helper by helper.

## Cross-links / principles

Grounds in `crystallized/principles.md` (`P-Durable-Enablement`; compute-provenance-not-utility;
no-right-to-remove-rights → survivability/exit) and the "moat from not having things" framing in the app
body. Estate origin/trust topology: `estate-architecture-and-browser-constraints.md`. Open: as each helper
is built, verify its degradation path actually holds (detect-and-degrade tested), and keep this register
current — a new helper without a declared baseline degradation is the smell that it is really a dependency.
