# Open Considerations: what the feasibility work surfaced but didn't resolve

author: design dialogue (distilled)

date: 2026-06-15

status: thinking / live design questions. Distilled from the closing design-review of the
2026-06-13/14 dialogue. These are not blockers and not yet decisions — they are the questions the
feasibility work made visible. The architecture's hard seams are proven; feasibility is no longer
the binding constraint, so several of these are now the binding constraints instead.

---

The substrate is unusually complete — transport (iroh), encryption (MLS/openmls), CRDT sync
(Automerge), blobs (iroh-blobs), Android wiring, the atproto public path, and now governance +
survivability have all been validated or designed. What follows is what that completeness threw
into relief.

## 1. What is the actual product?

The substrate is proven; the thing a user opens and *why they'd choose it* has never been named.
Every competitive analysis (Germ, X Chat, Discord, Matrix, the social-platform history) circles
this without landing it. Risk: building a beautiful foundation for a building whose floor plan
doesn't exist. **Decide concretely:** Discord competitor? group chat? forum? a 2010-Facebook feed?
a protocol others build on? Each implies very different next moves — and gates most of the
considerations below. **This is the one to settle first.**

## 2. Identity / key recovery is the load-bearing unsolved problem

It recurs everywhere: MLS multi-device, the messaging-landscape (SSB's "lose your device, lose your
identity" cautionary tale), and the survivability archive (password loss is unrecoverable by
design — see `governance-and-survivability.md`). We have a good answer for *data* survivability but
**no answer for identity/key recovery**, the hardest usability-vs-security collision in the space.
Signal solved it with a PIN + registration lock; SSB never did and it contributed to their stall.
A DID is portable but a lost key is still catastrophic. Treat this as a first-class design problem
on par with the closed architecture seams — it's the one most likely to make the product unusable
for normal people while staying invisible in technical validation. See `multi-device.md`,
`plc-identity-resilience.md`.

## 3. Possible over-application of Automerge

Surfaced in the Willow and AppView turns: most social objects (posts, likes, reactions, membership
records) are write-once or last-writer-wins and **don't need a CRDT at all**. Automerge earns its
place only for genuinely concurrent collaborative editing. If the product is chat-and-feed rather
than collaborative-documents, we may be carrying CRDT complexity (history growth, snapshot
management, Android memory profile) for a small fraction of cases. **Audit honestly** how much data
truly needs merge semantics before committing the whole stack to it. (Willow's LWW-per-path model
already matches the non-collaborative majority — see `group-privacy-lanes-design-note.md`.)

## 4. The superpeer is load-bearing, not an "optional optimization"

Trace it: membership-commit orderer (avoiding MLS forks), always-on rendezvous (so sleeping phones
can sync at all), durable queue (offline delivery), push-notification sender, and implicitly part of
survivability. That's infrastructure that, if down, degrades the product to "barely works." We've
been honest that superpeer-absent mode is best-effort — but name plainly that **real-world
reliability is the superpeer's reliability**, i.e. the "decentralized" system has a centralization
point that matters more than the framing admits. Fine (Delta Chat rides existing infra too), but
the superpeer's funding, uptime, and governance are **core, not peripheral**.

## 5. Moderation & abuse under a blind broker is unaddressed — and brutal

The Matrix and Discord briefs gesture at it; nothing in the design tackles it. A broker that can't
read content cannot do server-side spam/CSAM detection or abuse response, and "private groups with
multiple admins" + "we can't see anything" is a combination that has put platforms in serious legal
and ethical trouble. Given the commissioner's day job (product security, child safety), this needs
explicit early thought: **what is the abuse-handling story when we've deliberately blinded
ourselves?** Real techniques exist (client-side reporting that reveals specific messages,
metadata-based rate limiting, reputation) but must be designed in, not bolted on.

## 6. The strongest strategic insight is underexploited

Recurring across the Discord, social-cycle, and survivability threads: the **unoccupied market
position** is "decentralized values delivered as one cohesive, branded, stable product rather than a
fragmented protocol-with-twelve-clients." Discord won on zero-friction + coherence; FLOSS
alternatives lose on fragmentation. Our architecture + the survivability guarantee could be the
thing with open-protocol values **and** product coherence **and** a structural anti-rug-pull
promise. That's more defensible than any single feature. Consider making it the **organizing
thesis**, not a conclusion we keep re-deriving. See `../research/discord-dominance.md`.

## 7. Encrypt-then-content-address kills cross-user dedup

Lighter, but real: same photo + different nonces ⇒ different ciphertext hashes ⇒ no cross-user
dedup. For media-heavy use this has storage-cost implications that interact with survivability
funding (which was costed on small archives). Media changes that math. See
`../../experiments/encrypted-blob-share/`.

---

## Priority

Act on **(1) product definition first** — every other decision (how much CRDT, how reliable the
superpeer must be, what moderation is needed, what the survivability fund covers) flows from knowing
what we're building and for whom. Feasibility is no longer the binding constraint; **product
clarity is.**
