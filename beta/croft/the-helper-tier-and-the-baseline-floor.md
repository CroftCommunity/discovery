# The helper tier and the baseline floor: nothing server-side is load-bearing

date: 2026-07-22

**What this establishes.** How Croft's client and server surface is structured so that the product stays
un-enclosable: there is an honest **baseline floor** that runs with **zero servers and zero helpers**, and
every server-side component above it is an **optional helper** that only *enhances* — each with a declared
way it degrades back to the floor. This is the product-layer expression of the neutral protocol's
principles (peer equality, durable-enablement, exitability); it sits beside the garden-of-ponds product
shape (`product-the-garden-of-ponds.md`) and the build sequence (`build-order-and-ponds-roadmap.md`).

## The baseline floor

**Two people, same network, a direct connection, a manually-shared join code — no relay, no resolver, no
AppView, no login broker, no delivery service, no server of any kind.** That is "acceptable usage." A pair
of peers on one network can find each other, connect directly, and interact, with nothing in the middle.
Everything richer is built on top of that floor and names how it falls back to it.

The floor is not a degraded mode grudgingly supported; it is the definition of the product working. If a
proposed component cannot be removed while leaving the floor intact, it is not a helper — it is a
requirement, and it must be justified as one or redesigned. That test is what keeps the local-first,
un-enclosable claim honest rather than aspirational.

## The helper contract

A **helper** is an optional service that enhances the experience but is not required for the floor. Every
helper satisfies five rules:

1. **Optional** — the floor works with the helper absent.
2. **Declared degradation** — the helper names exactly what the baseline does without it, and the app
   detects-and-degrades rather than failing silently.
3. **Isolated trust surface** — a helper is a trust and attack surface, so it is minimal, runs on its own
   origin or domain (never sharing the session origin), and holds the least state possible.
4. **Replaceable and self-hostable** — no specific centralized instance is required; a person or a
   cooperative can run their own, and swapping it does not change the floor.
5. **Reads stay direct** — public reads never route through a helper; only the enhancement path does.

Because the helpers are exactly the components that hold state, hold keys, or gate access, the register of
them is also the security map: listing every helper is listing every privileged surface, each isolated and
route-around-able.

## The register of helpers

| Helper | Enhances | Baseline without it (the degradation) |
|---|---|---|
| Custom AppView | rich read / index / feeds / search over the network | the public AppView plus direct record reads (a records browser works with no custom AppView at all) |
| Login broker (BFF) | one sign-in shared across separately-hosted apps, and the signing key held server-side | each app signs in on its own (fully serverless) |
| Followers-level restriction | roster-gated "private but not end-to-end encrypted" sharing | public records, or the end-to-end-encrypted sealed tier — the two ends of the privacy ladder |
| Push helper | notifications when the app is closed | in-app and on-open only |
| Content-blind ingest | anonymous, no-login, multi-writer contribution (a group card, a guestbook) | named, signed-in writers only |
| Relay | connection across networks and behind NAT (and for browser peers, which cannot hole-punch) | same-network direct connection — the floor needs no relay; the relay only extends reach |
| Deep-link resolver | one-tap join and share, and the whole growth path | manual join by pasting a ticket or code |
| Delivery service (the unequal peer) | store-and-forward to peers who are offline | direct delivery to peers who are online; the offline peer catches up on reconnect |

Two distinctions keep the register honest. **Untrusted data versus untrusted code:** an app that merely
*renders* others' content (a feed window) can be first-party code handling untrusted data safely, and does
not need isolation; only an app that *executes* a stranger's code (an open-any-artifact renderer) must be a
separate, sandboxed origin. **Strategic importance versus functional need:** the deep-link resolver is the
growth engine and matters enormously for adoption, yet the product still works without it, so it is a
helper — classifying by the functional floor prevents "growth depends on it" from smuggling components back
into the required set.

## Where the helpers run (the origin shape)

The isolation rule has a concrete consequence, and one part of it was settled by measurement rather than
assumption: browser storage is shared across pages of the **same origin** on every engine, but a store
shared across **subdomains** is partitioned on WebKit (and therefore on all iOS browsers). So the estate
cannot lean on a single shared-storage origin embedded across subdomains. The resulting shape:

- **First-party, low-risk pads** may share **one origin** (path-based), where a shared client store and a
  single session work on every engine.
- **Higher-risk or untrusted-input pads** take their **own origin** for blast-radius containment.
- **Anything running untrusted code** takes a **separate registrable domain**, sandboxed and message-passing
  only, never touching the session.
- The **user's own data store is the authority** across all of them; coherence between separately-hosted
  pads flows through it.

A login broker (helper) is the only way one sign-in reaches across those isolated origins on every engine,
because a session bound to a per-request key cannot be shared through browser storage alone; without the
broker, each isolated app signs in for itself. That trade — one small optional server for shared sign-in,
or no server and per-app sign-in — is a helper decision, not a floor decision. The still-open mechanics of
that broker, and the propagation-latency question for cross-device coherence, are tracked as open threads
rather than asserted here.

## Why this is carried forward from the protocol

The helper tier is the product-layer inheritance of the neutral protocol's principles, and reading them
together is the point:

- **Peer equality** (the protocol's floor: peers are equal) becomes the baseline floor of two direct, equal
  peers. Helpers are the bounded, accepted *unequal-peer roles* layered above it — the delivery service is
  the canonical one the protocol already treats as an accepted, bounded, exitable role, never the thing the
  system depends on.
- **Durable enablement** (only ship a guarantee you can keep enabled by default; a capability too costly to
  provide uniformly becomes one people route around) is exactly what makes a capability a helper rather than
  a requirement.
- **Exitability** (no privileged role can hold the system hostage) is why every helper is replaceable and
  self-hostable.

So this register is the product's concrete accounting of the protocol's standing question — which
privileged roles exist, and are they all bounded and route-around-able — checked helper by helper. It is
also where the anti-enclosure claim is kept checkable: "no one can take a piece away and break the product"
is true only because every server-side piece is a helper with a declared degradation back to a floor that
needs no one's permission.

## Status

The floor, the helper contract, the untrusted-data-versus-code and strategic-versus-functional distinctions,
and the origin shape are settled. The still-open, deferrable items live in the open-threads queue: the
login-broker mechanics and whether shared sign-in works on WebKit, the exact storage backend on Safari, and
cross-device coherence latency. None of them blocks the floor or any single helper from shipping.

---

Provenance: consolidated 2026-07-22 from the account-kernel spike and its capability analysis
(`../../alpha/spike/account-kernel/FINDINGS-AND-PIVOT.md`, `KC0-RESULTS.md`, `K1-SPIKE-RESULTS.md`;
`../../alpha/thinking/app/helpers-register.md`, `estate-architecture-and-browser-constraints.md`), staged
in `../OPEN-THREADS.md` T55 and traced in `../../alpha/LAYER-ROLLUP.md`. The protocol principles it inherits
are the neutral spec's; the iroh/atproto facts cite the FACTCHECK source of truth.
