# Two-target delivery: the group queue and the personal inbox

date: 2026-08-12
status: thinking (design synthesis). Problem / Approach / Reasoning.
**Supersedes the delivery shape** in `meer-as-custodian-queue.md` (2026-08-07), whose meer was
*addressed* — a divergence the Phase-0 spike registered late as `meer-spike-addressed-deposit`. The
custody-dial, metering and anti-entrenchment reasoning there still stands.

**Grounded in:** `alpha/experiments/meer-queue/TEST-LOG.md` (M1, M2, S1–S10, all Rung A) ·
Part 2 §5.4, §5.9, §6.6.2 · `alpha/ROADMAP_TODO.md` E91–E97

---

## Problem

The meer spike proved the central claim — a blind store-and-forward node with no ordering, no group
state and no key carries a real MLS conversation across an absence (M1, M2). But it tested an
**addressed** meer, and Part 2 §5.4 describes something else: a meer *"participates in a Group's
delivery scope the way any swarm node does, in the gossip fabric."* Correcting that raised the
question the addressed model never had to answer: **what entitles you to drain, and how do you find
your mail?**

Working it through produced a shape neither doc had: **delivery has two targets, not one, and they
want different homes.**

## Approach

### Target 1 — the group queue

**Keyed by a secret only members can derive.** `export_secret(label = "croft/meer-queue/v1")` yields
a name every member computes identically, no non-member can compute, and **the meer cannot compute
either** — so it needs nothing in advance and stores under an opaque name it is handed (S9, Rung A).

- **Possessing the name *is* the entitlement.** No membership check, no credential presented, no
  identity disclosed. The meer learns "someone who can derive this asked."
- **Rotation is free.** The secret is epoch-bound, so the name changes on every commit. Nobody tells
  the meer anything; it is simply a new queue.
- **The meer holds no per-device state.** The have-set lives on the device, so two devices of one
  persona get different answers from one store by asking different questions. §6.8.1's *"the cursor
  and the gap-detector are the same object"*, arriving for free.
- **Storage is pooled**, not per-DID. S2 measured that dedup does not cross a CISS namespace, so
  per-DID queues cost storage linear in fan-out. Group traffic is exactly where that hurts.

### Target 2 — the personal inbox

**Keyed by identity, held in the owner's own namespace**, resolvable through their DID document.

Only one thing lands here: a **`Welcome`** — the sole object in MLS addressed to a *person* rather
than a group. Everything else is group-addressed and self-locating.

- **It works for a stranger.** B can message A having never interacted with A before: resolve A's
  DID → find A's storage → deposit. No prior shared secret, which is precisely what target 1 cannot
  offer.
- **Per-DID is right here.** S2's fan-out cost is irrelevant for an object that is genuinely
  one-per-person and rare.
- **This is what custodian mode is for.** A revocable grant letting a helper append to one slot in
  your namespace is over-engineering for group traffic and exactly right for invitations.

### Inbox authorization: read is solved today, write is the real work

The inbox address is **public by necessity** — a stranger must be able to find it. So the address
must carry **no** authority, and read/write must be gated separately.

**Read: `read_class: owner`. Shipped, configure it.** CISS's gated reads (v0.4.0, invariants Z4–Z8)
authorize on verified DID ownership — `allow ⇔ caller == owner` — checked against the DID document's
key via service-auth JWT, or an `id:` session. So a public address yields a **write** target and
nothing else, and the harvest-now-decrypt-later concern collapses: an attacker cannot obtain the
ciphertext at all, let alone hold it for a future break.

**Write: genuinely open, and not yet possible.** The spec is explicit — *"Writes are unchanged —
owner-only… delegated writes are a [PLANNED] extension, not v1."* So B cannot deposit into A's
namespace today at all. That is the custodial-write gap (meer-lane Phase 1), and it is the piece that
must be **designed**, not configured. Open writes into someone's own namespace also mean **spam costs
the victim rent**, which is the concrete form of the abuse problem.

#### The KeyPackage-as-write-token idea, and why it fails (S11, Rung A)

MLS offers an appealing candidate: B cannot invite A without consuming one of A's published
KeyPackages, and KeyPackages are single-use by design. Make consumption the write capability, and
invitations are bounded by a supply the owner controls.

**Measured, and it does not hold.** Two findings, and the second kills it:

1. **The single-use property is real, but on the wrong side.** Alice could join the first group and
   **not** the second — the private half is consumed on join. So one published package does seat her
   at most once.
2. **But anyone who can *read* a published KeyPackage can build a valid `Welcome` against it.** Two
   independent parties each produced one from the same package, and nothing at the crypto layer
   objected — a KeyPackage is *public key material*, and inviting a stranger is precisely what it is
   for.

Together those invert the intended effect. "Mark it spent on deposit" lets any passer-by **burn the
owner's entire published supply** and deny legitimate invitations. **The bound lands on the wrong
party: it limits the owner's reachability, not the attacker's effort.** Rejected.

#### What the write gate can and cannot be

Also measured: **a stranger can seat A in a group she never asked to join.** That is MLS working as
specified. So an unwanted invitation is **not cryptographically preventable**, and the gate can only
bound *volume* and make it *attributable* — never prevent the first one.

Which leaves two mechanisms, neither novel:

- **An authenticated depositor DID.** Not pre-authorized — any DID — but *verified*, so abuse is
  attributable and rate-limitable per identity. This is CISS's existing identity plane.
- **An owner-declared ceiling**, so total damage is bounded by a number the owner chose. Already in
  the design; the hypothesis doc's *"the owner-declared ceiling does the defending and the meter only
  watches"* is exactly this case.

**Neither is a capability, and that is the honest position:** the inbox cannot be made
unsolicited-free, only bounded and accountable — the same posture email settled on, for the same
reason.

### And it closes the KeyPackage gap

The corpus has **no KeyPackage distribution story** (searched 2026-08-12). But B needs A's KeyPackage
to create the group at all, so B is already fetching something from A. If A publishes KeyPackages to
the same namespace, **A's namespace becomes A's contact point: KeyPackages out, invitations in.** One
location, one resolution step, two gaps closed.

### Retention: 14 days, then expunge

**Precedent (owner, 2026-08-12, `[UNVERIFIED]` — owner-checked, not re-verified here): Threema holds
undelivered messages ~14 days and expunges.** Signal's posture is comparable. So a bounded hold with
real deletion is *industry-normal*, not an eccentric position — which matters, because the honest
version of this design has to say "and then it is gone" and mean it.

**Today it cannot mean it.** S5 measured that CISS has no object `DELETE`, so sweeping ends *serving*
and not *holding*. E95 is the fix; the owner's decision is both halves (A: manifest-driven
reclamation for atproto-compat; B: declared expiry as a seventh `KindSpec` axis).

### What each target reaches, and what it does not

```
   B sends to A                    A comes back online
   ─────────────                   ───────────────────
   already in a group?             for each group I hold:
     └─► group queue                 └─► derive name, drain, walk the chain
         H(exporter secret)              (~12 ms/hop, N = governance events)

   new conversation?               poll my personal inbox:
     └─► personal inbox              └─► one stable, identity-derived location
         in A's own namespace
```

**Push is a wake signal only.** It never names a group — a push that did would leak membership to the
push provider. A polls what it already knows it holds.

## Reasoning

**Why the split is forced rather than chosen.** MLS gives one object addressed to a person
(`Welcome`) and everything else addressed to a group. A single delivery target would have to serve
both, and the two have opposite requirements: group traffic wants unguessable, rotating, pooled;
invitations must be reachable by someone who shares no secret with you and therefore *cannot* be
unguessable. The split is the shape of the underlying protocol, not a design preference.

**Why `EndpointId` is for rate limiting, never authorization.** Authorizing on it would let the meer
build a device→groups map across every queue it serves — re-introducing exactly the correlation the
capability design avoids. Abuse control needs a handle to count against, not an identity.

**Why the catch-up walk is acceptable.** S10 measured **124 ms for ten missed epochs** over real
CISS and real iroh — ~12 ms per hop, pipelined, no user interaction. And N is bounded twice: it
counts **governance events, not messages** (50 application messages leave the epoch unchanged), and
the retention window caps it absolutely, so a member back after six months pays the same walk as one
back after two weeks.

**Why "skip ahead" is not an alternative.** OpenMLS refuses a commit whose predecessors were not
seen, and even if it did not, the missed plaintexts stay unreadable because their epoch secrets were
never derived. The walk catches up **and delivers**; skipping abandons. Comparing their costs
compares delivery against non-delivery.

## Honest limits

- **The personal inbox address is not secret.** Identity-derived and non-rotating, so an observer can
  tell *that A received an invitation* — not from whom or to what, since it is sealed to A's
  KeyPackage. **Structural:** a stranger must be able to address you before you share any secret.
- **`group_id` is cleartext** in every MLS envelope (S7), so the meer can link a group's traffic
  across epochs despite rotating queue names. **The opaque name buys access control, not privacy.**
  E96 (nested sealing) is the fix — and the queue name is what makes it *possible*, by giving the
  meer something to route on that is not the MLS framing.
- **Retention beats absence, silently, for invitations.** Offline past the window and the invitation
  expires. A cannot be told about a gap in a group A does not know exists — the one place the
  "loud, visible gap" story cannot reach.
- **Custodial write is required again.** The fabric model made it look unnecessary; the personal
  inbox needs it. That is the security-review-heavy piece (meer-lane Phase 1).

## Client contract (small rules with sharp edges)

- **Dispatch on the cleartext `content_type` *before* processing.** `process_message` consumes the
  message key, so try-decrypt-then-fall-back destroys group state — the second call hits
  `SecretReuseError`. S10 was written that way first and failed exactly so.
- **Preserve typed MLS errors.** `SecretReuseError` (benign duplicate) and `TooDistantInThePast`
  (unrecoverable loss) are one variant apart and mean opposite things (S3b).
- **Dedup on content hash before processing**, with `SecretReuseError` as the repair path when the
  in-memory cache is lost.

## What this changes elsewhere

| | was | now |
|---|---|---|
| custody dial | one global choice | **per queue type** — pooled for groups, per-DID for the inbox |
| E92 (deliver-once) | a dial needing §6.6.5 | **dissolved** — no per-recipient queue to starve |
| E94 (graph leak) | an inherent leak | **artifact** of the addressed model |
| E97 (announcement) | an unbuilt channel | **resolved** — groups are self-locating; the inbox is DID-resolvable |
| custodian mode | for group queues | **for the personal inbox** |
