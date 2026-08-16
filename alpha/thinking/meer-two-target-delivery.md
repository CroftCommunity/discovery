# Two-target delivery: the group queue and the personal inbox

date: 2026-08-12
status: **design synthesis, WALKED OUT at Rung A (S9–S17).** Both targets are now measured against
real OpenMLS, real CISS and real iroh — not proposed. **One capability is missing and it is named:
custodial write** (a stranger's deposit into the owner's namespace is refused HTTP 403). Everything
else in both paths works today. Problem / Approach / Reasoning.
**Amended 2026-08-13 by S15–S17** — three changes, marked inline: a **third gap** (nothing serves
`GroupInfo` to a returner, E105), a **correction** to the limbo claim, and **E96 moved from proposed
to measured**.
**Supersedes the delivery shape** in `meer-as-custodian-queue.md` (2026-08-07), whose meer was
*addressed* — a divergence the Phase-0 spike registered late as `meer-spike-addressed-deposit`. The
custody-dial, metering and anti-entrenchment reasoning there still stands.

**Handoff / resume point:** `alpha/experiments/meer-queue/STATE-AND-NEXT.md`

**Grounded in:** `alpha/experiments/meer-queue/TEST-LOG.md` (M1, M2, S1–S12, all Rung A except S1) ·
`S8-RESULTS.md` · Part 2 §5.4, §5.9, §6.6.2 · `alpha/ROADMAP_TODO.md` E91–E97 ·
`CISS/docs/plans/2026-08-11-object-lifecycle.md`

---

## Where this stands (2026-08-12)

| | mechanism | status |
|---|---|---|
| **group queue** — name | `export_secret("croft/meer-queue/v1")` | **measured** (S9) — members agree, non-members cannot derive, rotates per epoch |
| **group queue** — drain | `OP_DRAIN_QUEUE`, name *is* the capability | **built + measured** (S10) over real iroh |
| **group queue** — catch-up | serial chain, one hop per missed epoch | **measured** (S10) — 124 ms for 10 epochs, ~12 ms/hop |
| **inbox** — necessity | no queue name without group state | **measured** (S12) |
| **inbox** — read gate | `read_class: owner` | **measured** (S12) — owner 200, stranger 404, anon 404 |
| **inbox** — handshake | publish → fetch → validate → invite → deposit → join → handover | **measured end to end** (S12) |
| **inbox** — write | a stranger deposits under a custodial grant | **MISSING — HTTP 403** (S12). meer-lane Phase 1 |
| **retention** — expunge | 14 days, then the bytes are gone | **MISSING** (S5) — CISS has no object `DELETE`. E95 |
| **re-entry** — `GroupInfo` | a returner fetches current group state | **UNWIRED SEAM, not missing** — Part 2 §11.6/§11.11 already names the **history-convergence node**. E105 |
| **re-entry** — credential | §11.7's attestation + resumption PSK | **NOT IMPLEMENTABLE AS WRITTEN** (S16) — an external PSK replaces both halves. E106 |
| **linkability** — `group_id` | outer seal over the MLS envelope | **BUILT + MEASURED** (S17) — 28 flat bytes, breaks nothing. E96 |
| **removal** — durability | a removed member stays removed | **ONLY AS DURABLE AS `GroupInfo` DISTRIBUTION** (S18) — but refusal holds at two layers. E107 |

**Three blockers, all scoped:** custodial write (meer-lane Phase 1), object lifecycle (E95), and the
**`GroupInfo` channel** (E105 — new, and the only one this design did not previously know about).

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
- **Built and measured over the real transport** (S10). `OP_DRAIN_QUEUE` drains **by name** —
  possessing it *is* the entitlement, and the caller's identity never enters the decision. It
  supersedes the spike's `EndpointId` scoping (`meer-spike-drain-auth`), which identified the
  *device* and would have let the meer build a **device→groups map** across every queue it serves.
  `EndpointId` survives only for **rate limiting**, which needs a handle to count against rather
  than an identity.

### Target 2 — the personal inbox

**Keyed by identity, held in the owner's own namespace**, resolvable through their DID document.

Only one thing lands here: a **`Welcome`** — the sole object in MLS addressed to a *person* rather
than a group. Everything else is group-addressed and self-locating.

> **Correction (S14).** This section originally implied *all* joins arrive via the inbox. They do
> not. **First contact** needs it — a stranger must reach someone with whom they share no secret.
> **Re-entry by a former member does not:** §11.7's self-service path is an **external commit** from
> a current `GroupInfo`, measured working at Rung A (no `Welcome`, no active member's help). So the
> inbox carries *introductions*, not *returns* — which narrows what third-party deposit has to
> support.

- **It works for a stranger.** B can message A having never interacted with A before: resolve A's
  DID → find A's storage → deposit. No prior shared secret, which is precisely what target 1 cannot
  offer.
- **Per-DID is right here.** S2's fan-out cost is irrelevant for an object that is genuinely
  one-per-person and rare.
- **This is what custodian mode is for.** A revocable grant letting a helper append to one slot in
  your namespace is over-engineering for group traffic and exactly right for invitations.
- **History before the join is unreachable *by addressing*, not just by decryption** (S13). A
  joiner cannot name the queues of epochs she was not in, so she never requests them. **The MLS
  privacy boundary and the queue-addressing boundary are the same boundary** — which is why
  history-before-join needs no separate access rule.
- **It is necessary, not merely convenient** (S12). A queue name derives **only from group state**,
  so holding the owner's public KeyPackage — everything a stranger can legitimately obtain — yields
  nothing. A stranger has *no group-queue path at all*.
- **The handover happens exactly once, at first contact.** After the join, the inbox is idle and the
  group queue carries everything. That is what makes the split cheap: the inbox can be per-DID,
  low-volume and expensive-per-item **without any of that costing anything at scale**, because it is
  touched once per *relationship* rather than once per *message*. S2's fan-out cost never arises.

### Inbox authorization: read is solved today, write is the real work

The inbox address is **public by necessity** — a stranger must be able to find it. So the address
must carry **no** authority, and read/write must be gated separately.

**Read: `read_class: owner`. Shipped, and now measured (S12).** Owner `200`, authenticated stranger
`404`, anonymous `404`, against real CISS. So a public address yields a **write** target and nothing
else, and the harvest-now-decrypt-later concern collapses: an attacker cannot obtain the ciphertext
at all, let alone hold it for a future break.

> **The mutation matters more than the result.** Skip the policy write and the stranger reads with
> **`200`** — the world-readable PDS-compat default. **An inbox that forgets to set `read_class` is
> world-readable, silently.** So the policy write is part of **provisioning**, not optional
> hardening, and whatever creates an inbox should refuse to call it created until the policy is set.
> This is the kind of default that ships wrong once and stays wrong.

**Write: genuinely open, and not yet possible — measured, not cited (S12).** A stranger's deposit
into the owner's namespace is refused **HTTP 403**. The spec matches: *"Writes are unchanged —
owner-only… delegated writes are a [PLANNED] extension, not v1."* So B cannot deposit into A's
namespace today at all, and **this is the design's one genuine blocker.** That is the custodial-write gap (meer-lane Phase 1), and it is the piece that
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

**Measured end to end (S12):** published → fetched by a stranger → **`validate()`d** → group created
→ `Welcome` deposited and retrieved **byte-identically** → joined → both parties derive the same
group queue name. Only the deposit is stood in
(`SPEC-DELTA[meer-spike-owner-write-standin]`).

**A note on `validate()`.** The fetched KeyPackage goes through `KeyPackageIn::validate()` — the real
receiver path. The bare `From<KeyPackageIn> for KeyPackage` conversion is `test-utils`-gated
*precisely because it skips that check*, the same shape as the M2 re-frame gating. OpenMLS is
consistent about putting the unsafe shortcut behind a feature flag, and a client that reached for the
convenient conversion would be accepting unvalidated key material from a stranger.

### Retention must be ≥ the Group's liveness window

**The constraint (2026-08-12).** The meer's retention and Part 2 §11.6's **liveness window** are
different knobs deciding the same thing, and nothing currently forces them to agree. If retention is
**shorter** than liveness, a member in between lands in **limbo**: still live in the hot Group, unable
to catch up from the meer, and not yet migrated to cold.

> **Correction (S15, 2026-08-13).** This section originally continued *"so §11.7's re-entry path is
> not open either"*. **That is too strong, and S14 said it too.** Walked at Rung A: a
> stranded-but-live member **did** re-enter by external commit — openmls does not distinguish "cold"
> from "stranded", so §11.7's path is open to anyone holding a current `GroupInfo`.
>
> **But the escape needs a `GroupInfo`, and neither delivery target carries one** — the group queue
> is unnameable to her by construction, the inbox carries `Welcome`s, and a `GroupInfo` is not a
> queued object at all. So §11.7's return is self-service in COST only: it needs a live party to
> answer.
>
> **Corrected 2026-08-14 after reading Part 2 §11.6–§11.11:** *this design* has no such channel, but
> **the spec does** — the **history-convergence node** supplies `GroupInfo` and ratchet-tree
> conveyance (§11.11 item 6), and §11.6 already names history-convergence nodes as community
> infrastructure. **E105 is an unwired seam between this doc and §11, not a missing capability.** It
> also dissolves limbo without a new component: an always-on HCS in the returner's **own device
> pool** never falls out of the liveness window, so it can always answer.
>
> Net effect: limbo is *recoverable*, not *terminal* — but only once E105 exists. Until then the
> ordering constraint below is doing all the work.

**So: `meer retention ≥ liveness window`.** Then "cannot catch up" and "migrated to cold" coincide and
there is exactly one recovery path. **Now enforced in code**, not prose: `Meer::sweep_with_retention`
takes the window as an argument and `sweep()` merely defaults it, because §11.6's windows are set per
Group and a service constant can only ever suit the most aggressive band (S15).

§11.6's windows tighten with group size — **90 days at 250–1k down to 14 days at 7–10k** — so
retention is **bounded below by a per-Group governance policy**, not free. A fixed service-wide
constant is correct only for the largest, most aggressive band. **This argues E95's declared-expiry
axis belongs to the Group, not the service.**

### Retention: 30 days as the working figure, then expunge

**Working figure: 30 days** (owner, 2026-08-12) — which sits at §11.6's *modest* window for 1–3k and
its *aggressive* window for 7–10k, so it satisfies the constraint above for most realistic bands.
**Precedent (owner-checked, `[UNVERIFIED]` here): Threema holds undelivered messages ~14 days and
expunges.** Signal's posture is comparable. So a bounded hold with
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

**Why the write gate cannot be a capability (S11).** The appealing idea — make consuming a published
KeyPackage the write token — **fails, and inverts.** The single-use property is real but sits on the
*recipient's* side: Alice can join once per package. Anyone who can **read** a published KeyPackage
can build a valid `Welcome` against it, because a KeyPackage is public key material and inviting a
stranger is what it is *for*. So "mark it spent on deposit" lets a passer-by **burn the owner's whole
published supply and deny legitimate invitations** — the bound lands on the owner's *reachability*,
not the attacker's *effort*. Also measured: a stranger **can** seat you in a group you never asked to
join, which is MLS working as specified. So an unwanted invitation is **not cryptographically
preventable**, and the gate can only bound *volume* and make it *attributable*: a verified depositor
DID plus an owner-declared ceiling. **The inbox can be bounded and accountable, not
unsolicited-free** — where email landed, for the same reason.

**Why `EndpointId` is for rate limiting, never authorization.** Authorizing on it would let the meer
build a device→groups map across every queue it serves — re-introducing exactly the correlation the
capability design avoids. Abuse control needs a handle to count against, not an identity.

**Why the catch-up walk is acceptable.** S10 measured **124 ms for ten missed epochs** over real
CISS and real iroh — ~12 ms per hop, pipelined, no user interaction. And N is bounded twice: it
counts **governance events, not messages** (measured: 50 application messages leave the epoch
unchanged), and the retention window caps it absolutely, so a member back after six months pays the
same walk as one back after two weeks.

*Correcting an earlier claim in this line of work:* it was written here that "a group that rotates
keys aggressively makes returns slower", implying chat drove the cost. **Chat volume is irrelevant to
N.** Only commits advance the epoch.

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
  meer something to route on that is not the MLS framing. **Built and measured (S17, 2026-08-13):**
  `group_id` is absent under the outer seal, the object no longer parses as MLS, the cost is **28
  flat bytes**, and routing, dedup, byte-identity and the catch-up walk are all unaffected. Adoption
  is still the owner's call; the estimate is now a measurement.
- **Retention beats absence, silently, for invitations.** Offline past the window and the invitation
  expires. A cannot be told about a gap in a group A does not know exists — the one place the
  "loud, visible gap" story cannot reach.
- **Custodial write is required again, and it is the single blocker.** The fabric model made it look
  unnecessary; the personal inbox needs it. Measured as **HTTP 403** (S12). Security-review-heavy
  (meer-lane Phase 1). **This design does not reduce what CISS must build — it relocates it**, from
  group queues where it was over-engineering to the inbox where it is load-bearing.
- **The read gate's default is the wrong way round.** Unset, a namespace is **world-readable**
  (PDS-compat). An inbox is only private because someone remembered to set `read_class: owner`.
  Provisioning must enforce it; documentation will not.
- **A gap anywhere in the chain orphans everything after it** (S13). The walk is oldest-first and the
  oldest queue is closest to expiry, so if any link expires every later queue becomes **unnameable**.
  Loss is **total from the break forward**, not proportional to what expired. **This is the designed
  boundary, not a defect:** Part 2 §11.6 migrates a client that misses the **liveness window** to
  cold, and §11.7 defines re-entry *"at its own cost"*. Losing the epoch thread and needing
  readmission is the intended outcome.
- **An unwanted invitation cannot be prevented, only bounded** (S11). A stranger can seat you in a
  group you never asked to join — MLS as specified.
- **A removal is only as durable as `GroupInfo` distribution** (S18). A deliberately removed member
  re-seated herself on a current `GroupInfo` alone. **Refusal does hold** — a refuser's traffic is
  both undecryptable and unaddressable to her — but that is a per-member act, and its cost is a
  fork. **The admission surface is the ratchet tree**, so withholding the tree is the real dial.
- **A fork is invisible in the epoch counter** (S18). Two branches that each advanced once agree on
  the epoch number and share no secrets. Nothing in the delivery layer surfaces this; the only
  symptom is peers silently failing to read each other. **A client needs a fork signal that is not
  the epoch number.**

## Client contract (small rules with sharp edges)

- **Dispatch on the cleartext `content_type` *before* processing.** `process_message` consumes the
  message key, so try-decrypt-then-fall-back destroys group state — the second call hits
  `SecretReuseError`. S10 was written that way first and failed exactly so.
- **Preserve typed MLS errors.** `SecretReuseError` (benign duplicate) and `TooDistantInThePast`
  (unrecoverable loss) are one variant apart and mean opposite things (S3b).
- **Dedup on content hash before processing**, with `SecretReuseError` as the repair path when the
  in-memory cache is lost.
- **Validate a fetched KeyPackage** — `KeyPackageIn::validate()`, never the `test-utils` conversion,
  which exists to skip exactly that check. It is unvalidated key material from a stranger.
- **Set `read_class: owner` when creating an inbox, and treat it as part of creation.** The default
  is world-readable.
- **Consult the watermark before concluding you are caught up.** A swept queue and an empty queue
  return an identical empty drain (S13); only the watermark separates them. An empty drain alone is
  evidence of nothing.
- **Walk the epoch chain in order; do not attempt to skip.** OpenMLS refuses a commit whose
  predecessors were not seen, and the missed plaintexts would be unreadable anyway.
- **If nested sealing is adopted: wrap at the epoch of the QUEUE, not the epoch you are at** (S17).
  The commit that *closes* an epoch is wrapped with that epoch's key — derive it **before**
  committing and hold it across, because OpenMLS exports the current epoch only. Get it backwards and
  the walk deadlocks silently, indistinguishably from a corrupt object.
- **Refuse an external commit that carries no PSK you recognise** (S16). MLS admits a total stranger
  on a `GroupInfo` alone. Check `psk_proposals()`, the AAD attestation and the joiner's credential on
  the `ProcessedMessage` *before* `merge_staged_commit` — all three are available there.
- **Make that refusal a group-wide rule, not a local one** (S16). Declining moves only you; a member
  who merged is at a different epoch. **A policy each member evaluates differently is a partition.**

## What this changes elsewhere

| | was | now |
|---|---|---|
| custody dial | one global choice | **per queue type** — pooled for groups, per-DID for the inbox |
| E92 (deliver-once) | a dial needing §6.6.5 | **dissolved** — no per-recipient queue to starve |
| E94 (graph leak) | an inherent leak | **artifact** of the addressed model |
| E97 (announcement) | an unbuilt channel | **resolved** — groups are self-locating; the inbox is DID-resolvable |
| custodian mode | for group queues | **for the personal inbox** — and it is the one blocker |
| drain authorization | `EndpointId` (device-identifying) | **queue name as capability**; `EndpointId` demoted to rate limiting |
| write gate | a KeyPackage capability | **refuted (S11)** — verified depositor DID + owner ceiling instead |
| KeyPackage distribution | no story at all | **the owner's own namespace**, measured end to end |

## What to build, in order

Everything below is either measured-working or measured-missing; nothing here is a guess.

1. **Custodial write** (meer-lane Phase 1) — the single blocker. A stranger deposits into the
   owner's namespace under a revocable grant, gated by a **verified depositor DID** (any DID, but
   authenticated, so abuse is attributable and rate-limitable) and bounded by the **owner-declared
   ceiling**. Retires `meer-spike-owner-write-standin`.
2. **Object lifecycle** (E95, `CISS/docs/plans/2026-08-11-object-lifecycle.md`) — so "14 days, then
   expunge" is true rather than aspirational. A (manifest-driven reclamation, owed by atproto-compat
   regardless) then B (declared expiry as a seventh `KindSpec` axis).
3. **Inbox provisioning that sets `read_class: owner` as part of creation** — small, and the failure
   mode is silent world-readability.
4. **The group-context nomination** (E97) — an MLS `GroupContextExtensions` entry naming the meer a
   Group relies on. An **endorsement, not a permission**: the meer sees the fabric regardless.
5. **Nested sealing** (E96) — no longer merely unblocked: **built and measured (S17)**. Closes the
   cleartext-`group_id` linkability for **28 flat bytes**, with routing, dedup, byte-identity and the
   catch-up walk all measured unaffected. What remains is an adoption decision plus the wrapping rule
   in the client contract above.
6. **Wire the history-convergence node in as the `GroupInfo` server** (E105), **which is the same
   decision as removal durability** (E107, S18): whatever serves `GroupInfo` decides both who can come
   back and whether a removal sticks. Serve it **without the ratchet tree by default** — S18 measured
   the tree as the actual admission surface, and the export flag is per call, not per group config.
   **Record the trust asymmetry:** a meer holds no keys; an HCS in your pool holds group secrets, so
   one operator running both is a deliberate choice rather than a convenience.
7. **Readmission as a group-context policy** (E107) — never a per-member prompt, because S18 measured
   that disagreement forks the group and **the fork is invisible in the epoch counter**. Without it §11.7's self-service re-entry
   cannot execute, so a stranded member has no path at all. **Decide who serves it and whether serving
   it is a membership disclosure** — S16 measured that a `GroupInfo` alone admits a total stranger, so
   this channel is an admission surface, not a convenience.
8. **Rewrite §11.7's credential** (E106 — new, S16) around a **governance-issued external PSK** plus a
   group-context-extension policy. The resumption PSK the section names **cannot be attached to an
   external commit** on openmls 0.8.1; the external PSK carries both halves and works today.

**Deliberately not on this list:** anything that would make an unwanted invitation impossible (S11
shows it cannot be), and anything that reduces `N` in the catch-up walk (S10 shows it does not need
reducing).
