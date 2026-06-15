# Governance & Survivability: a structural anti-rug-pull guarantee

author: design dialogue (distilled)

date: 2026-06-15

status: thinking / our design, evolving. Distilled from the 2026-06-13/14 design dialogue
(the governance + survivability + cheap-durable-archive turns). Not yet a proof or a spec.

---

## Problem

The platform-cycle history (see `../research/social-platform-cycle.md`) shows community value is
repeatedly extracted or *erased* — GeoCities and Yahoo Groups deleted decades of collective
memory because users never held custody. The diagnosis there is a **capital-structure** one: VC
funding requires an exit, an exit requires extraction, extraction is the rug-pull. Open protocol +
portable identity + user-owned data remove the *leverage* for a rug-pull; they do not remove the
*motive*. Only funding-and-governance committed **at inception** removes the motive, because the
history is clear you cannot retrofit it after taking growth capital.

This note develops a specific, differentiating commitment: a **structural guarantee that user
data survives the operator's death** for a bounded window — turning the trust problem inside out.
Users don't have to trust us not to rug-pull; the survivability is guaranteed by a mechanism we
don't control. That directly attacks the "why try yet another thing" hesitancy: the downside is
capped by design.

## The reframe that makes it fundable: graceful exit, not permanence

The guarantee is **not** "we keep your data forever" (an unfundable forever-liability someone must
pay for in perpetuity). It is: **"if we die, there is a guaranteed, pre-funded runway — ~90 days
to a year — during which you can retrieve and relocate your data and community before it's gone."**

That is a finite, pre-fundable, bankruptcy-remote obligation, and it is the more honest promise.
Framing it as a *guaranteed graceful exit* rather than permanence is both truthful and achievable,
and our architecture already half-satisfies it (see "Architecture does the legal work" below).

## Precedent (what exists, and the gap)

- **Software/data escrow** is mature but aims at the *wrong beneficiary*: it protects a business
  *licensee* from a vendor's collapse, releasing code/assets to the paying enterprise client — never
  to the individual end users. The novelty we'd add is the *trigger* (operator insolvency/
  dissolution/extended outage) and the *beneficiary* (a successor steward acting for users).
- **"Exit to Open"** (responsible nonprofit wind-down — preserve intangible assets for continued
  benefit) is the nearest philosophical precedent, but it's a wind-down *ethic*, not a founding,
  user-facing, time-bounded guarantee.
- **Nonprofit dissolution law** helps: a tax-exempt entity's assets cannot be sold to members and
  must transfer to a similar nonprofit; a plan of dissolution can name a successor steward as the
  asset recipient. The "where does the data go when we die" answer naturally points to another
  mission-aligned nonprofit or trust, not a fire-sale.
- **Data trusts / RadicalxChange "data escrow"** supply the right legal posture: the steward as a
  **fiduciary to the data producers** (the users), not to the operating company.
- **No social/community project** has productized a founding-time, user-facing, time-bounded
  data-survivability guarantee. We'd be early — opportunity and warning both. For sensitive private
  data there is *no* existing stewardship infrastructure to copy (Internet Archive = public web;
  GitHub = code; nothing for private nonprofit datasets).

## Structural options (compose, don't compete)

1. **Standing data-escrow agreement** with a release-to-users trigger. Reuse the mature escrow
   industry; rewrite the release condition (operator insolvency/dissolution/outage) and the
   released-to party (a designated successor steward, not a business licensee). Cheapest; weakest
   on "who actually serves the data" (escrow releases a deposit, it doesn't run a service).
2. **A bankruptcy-remote entity holding the survivability function** — a sibling nonprofit or a
   **purpose trust** whose sole charter is "preserve and make user data available for N days after
   a trigger," funded by a restricted endowment the operating entity cannot touch. This is the
   structural core. The operating nonprofit's plan of dissolution names this trust as recipient.
3. **A restricted wind-down reserve** — money raised and legally restricted to the survivability
   runway only, segregated from operating funds. Pair with (2). Treat it as a first-class funded
   obligation, not an afterthought ("a priority to fund an availability pattern that is very robust").
4. **Architecture-as-guarantee (our stack's superpower).** Because the system is local-first and
   users already hold their own data (encrypted, identity via portable DID, content in Automerge
   docs they possess), the rug-pull is partially defanged *before* any legal structure engages.
   Escrow the *code* and *relay/superpeer config* so a successor can run the infrastructure; the
   user data largely lives with users already. This turns a costly legal promise into a
   mostly-already-true technical fact. Strongest version of the pitch.
5. **Open protocol + portability as the ultimate backstop.** Portable identity (atproto DID) and
   open formats make "leave with your community intact" structurally possible regardless of the
   entity's fate. The legal guarantee just covers the window during which users exercise it.

## The cheap, durable, "static yet safe" archive (the technical half)

The cheapest durable storage pattern and the most survivable one are the **same thing**: as little
running as possible. No app server, no database, no compute to keep alive — cheapest to operate and
most likely to outlive the org. The "don't try to be foolproof" instinct is load-bearing: foolproof
adds operational complexity that destroys survivability.

- **Storage:** warm object storage with **zero egress** (Backblaze B2 + Cloudflare Bandwidth
  Alliance, or Cloudflare R2). *Not* Glacier deep-archive — its retrieval penalty (~$20/TB, ~12h)
  is exactly wrong when users need to retrieve during a wind-down. Cheap-storage-+-zero-egress-+-
  instant-retrieval beats cheapest-storage-+-punishing-retrieval. Cost is low hundreds to low
  thousands/yr for meaningful scale — small enough to **pre-fund**.
- **Layout:** **one bundled encrypted blob per user** (or per-user-per-period), not thousands of
  tiny per-message objects (~40KB metadata overhead per object makes tiny objects wasteful). This
  aligns with the Automerge snapshot model — one packed encrypted snapshot per user is cheaper and
  simpler. Plus a **fixed private index** mapping identity → object key (itself encrypted, or use
  keys derived from user secrets so there's no central plaintext map).
- **Encryption is the access control.** Don't rely on storage being private; rely on contents being
  useless without the key. Each archive is client-side encrypted under a key the user controls
  (password-derived, or better, the identity key material they already hold). Even a fully-leaked
  bucket yields only opaque ciphertext.
- **Slow KDF = "wait for CPU to catch up."** Argon2id with high cost parameters so brute-forcing a
  weak password is infeasible offline. The attacker's CPU pays, not our servers. Safe-enough by
  making attacks expensive, not by being unbreakable — exactly the "feasible, not foolproof" balance.
- **Auth degrades to encryption-only.** While alive, a light gate (Cloudflare Worker / pre-signed
  URLs tied to identity) sits in front. On operator death the auth layer *vanishes* and security
  still holds because encryption was always doing the real work. That degradation — from
  "auth + encryption" to "encryption alone" — is the key to survivability, because auth needs a
  running service and encryption does not.
- **What the user needs to recover:** (1) knowledge their archive exists + where, (2) their object
  key or the ability to derive it from identity, (3) their password / key material. Two of the
  three live with the user, not our infra — that's what makes it survivable.

## How the two halves fuse

The survivability fund need only keep a **static bucket** alive and readable for the window — not a
running platform. That's the difference between an unfundable forever-liability and a trivially
pre-fundable one. Concrete, honest guarantee: *"If we shut down, (a) your data already lives on your
devices in our local-first architecture, (b) a pre-funded static encrypted archive remains
downloadable for ≥ N months from object storage we've pre-paid, held by a separate trust our
insolvency cannot touch, and (c) the code and bucket config are escrowed so a successor can take
over. You decrypt with your own key, which we never held."* Every clause is cheap and survivable
precisely because nothing in it requires us to be running anything.

## Honest caveats

- **The index is a metadata leak.** A plaintext identity→object map reveals *who has an archive*
  (membership) even if contents stay encrypted. Encrypt the index or derive keys from user secrets;
  accept that perfect metadata privacy and "static serverless" are in tension. For the dead-operator
  state, some metadata exposure is a reasonable trade — but name it.
- **Password loss is unrecoverable by design** (the E2EE recovery problem again — see
  `[[open-considerations]]` and the messaging-landscape research). Optionally hand users a separate
  recovery key at signup; do **not** build server-side key custody.
- **"Infeasible to brute force" is a moving target** over decades. Fine for a bounded 90-day-to-1-yr
  window; don't oversell as eternal.

## Status / next

The crux is the same as the platform-cycle crux: the survivability fund must be **pre-funded and
outside the operating entity**, and the steward must be a **bankruptcy-remote entity with a
fiduciary duty to users**. A dedicated legal-options research brief (purpose trusts, restricted
endowments, escrow-with-user-release trigger, dissolution-plan mechanics, data-trust governance,
costed) remains **outstanding** — it was offered in the dialogue but never written.

Related: `plc-identity-resilience.md` (identity that roots both the public and private sides),
`group-privacy-lanes-design-note.md` (public/private lanes), `../research/social-platform-cycle.md`
(why this matters).
