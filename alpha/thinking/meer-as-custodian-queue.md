# The meer as a custodian queue over CISS — conceptual alignment

date: 2026-08-07
status: **hypothesis, SPIKED 2026-08-08/11. DELIVERY SHAPE SUPERSEDED 2026-08-12 by
`meer-two-target-delivery.md`** — the meer here is *addressed*; the spec (Part 2 §5.4) has it observe
the fabric, and delivery turns out to have **two targets** (group queue + personal inbox), not one.
The custody-dial, metering and anti-entrenchment reasoning below still stands. Phase 0 of the meer lane ran every claim below against
real OpenMLS, real CISS and real iroh. **The central claim held; six subsidiary claims did not.** Each
correction is marked inline as `[SPIKE 2026-08-11: …]` with the scenario that found it; the full
record is `alpha/experiments/meer-queue/TEST-LOG.md`. Problem / Approach / Reasoning. Supersedes the
storage half of
`meer-superpeer-design.md` (2026-06-16), which assumed the meer owned its own store; the roles,
tiers, and anti-entrenchment analysis there still stand. Gates the spike (see "What we test next").

## Problem

The meer — Drystone's blind store-and-forward node (Part 2 §6.6.2, D-meer) — is the single gating
unbuilt component for "the conversation stays alive while you sleep." Two things blocked it, and both
turn out to be misframings rather than obstacles.

**First: there is no MLS delivery service to copy, and that is structural.** RFC 9750 deliberately
leaves the DS unspecified, so there is no spec to implement against and every shipped DS is bespoke to
its product. In practice a DS *is* the server half of a messenger — identity, group creation,
KeyPackage hosting, Welcome distribution, room policy, and ordering. OpenMLS ships one and labels it a
proof-of-concept for testing; Phoenix R&D ships a real one inside a whole application under AGPL-3.0;
MIMI standardises the role by putting a hub in the middle that orders, which is precisely what
Drystone declines. Strip ordering and strip group-state validation and what remains is a mailbox.
Nobody ships a mailbox standalone because on its own it is not a messenger. **The reason there is
nothing to copy is that everyone else's DS is their product, and we are building only the part that
is not.**

**Second: the meer was scoped as a service with its own storage**, which imported a storage design, a
backup design, a durability promise, and an entrenchment problem — none of which it needs, because
CISS already exists, is live, and holds content-addressed bytes it re-verifies on read.

The Phase-10 plan (`plans/croft-stack/10-drystone-layer.md`) gates the *history-convergence server* on
the fold and MLS becoming real, and warns against pre-building against a reference-model fold. That
gate binds the history store. It does not bind the meer: because the per-author index lives inside the
seal (§6.6.2), the meer cannot order and does not need to, so it needs zero fold knowledge. The meer
is unblocked today.

## Approach

**The meer is a thin gateway service over CISS. CISS holds the queue.**

### The custodian chain

CISS gains a **custodian chain mode**, so a helper can append to a chain in someone else's namespace
without any power over the rest of it.

- A chain's **kind** (`queue`, `file-sync`, `history-convergence`, …) is declared in the **owner's
  manifest slot declaration** — never in a head blob. Declaring it in the head would be
  self-authorising: the custodian would assert its own kind on the write being evaluated. It would
  also cost a fetch and a DAG-CBOR parse in the write path, and it could not describe a slot that has
  no heads yet.
- Kind is **fixed at genesis** and bound into the signed preimage, the way `heads` already appends
  `:heads=<sha256>`. Retyping is refused independently of `seq`, because a chain that could go
  file-sync → queue → file-sync would let custodial writes be re-read as owner-authored content.
- Per-kind prefixes in the preimage (`ciss/v1/chain:queue:…`) make a signature for one kind
  *structurally* invalid on another, so cross-kind reuse is impossible rather than merely checked.
- **`queue` is the only custodially-writable kind, and unknown kinds fail closed.** A mis-scoped write
  dies on one enum comparison and cannot bleed into a file-sync or history chain. The load-bearing
  test is exhaustive over the kind enum, so a newly added kind breaks the suite until someone states
  its custodial posture deliberately.
- The slot declaration carries **kind + custodian + an owner-declared ceiling**. Revocation is
  clearing the custodian field and bumping `seq` — an ordinary owner write that inherits B3
  anti-rollback for free.
- **Custodial writes go to a separate custodian-signed record** with its own `seq`. The custodian never
  writes the owner's manifest, so CISS invariants B1 and B3 keep their existing proofs and the new
  surface is bounded to a record the owner can revoke.

### What the meer does

**[SUPERSEDED 2026-08-12 — the operations below describe an *addressed* meer, and the spec does not.**
Part 2 §5.4 has a meer *"participate in a Group's delivery scope **the way any swarm node does, in the
gossip fabric**, carrying and seeing the sealed envelope and its routing metadata as it passes."* It
**observes**; it is never told who the recipients are. The spike built the addressed model and
registered it late as `meer-spike-addressed-deposit`. Two of its findings — the
`(depositor → recipients)` graph leak (E94) and multi-device starvation (E92) — are artifacts of that
shape, not properties of a meer. The corrected operations are below; the originals are kept because
the spike's results are recorded against them.]**

### The operations, corrected (2026-08-12)

The meer is a **swarm participant**, so it is never addressed and holds a **group's** traffic, not a
person's. **MLS forces this**: an application message is one ciphertext encrypted to the Group's epoch
key, and every member decrypts the same bytes. There is no per-recipient copy to hold.

1. **carry** — be in the Group's gossip topic, and keep the sealed envelopes that pass
2. **store** — `PUT` each distinct envelope once to CISS (content-addressed)
3. **serve a drain** — a member states the digests it holds; hand back the difference
4. **expire** — drop what is past the Group's declared window, leaving a watermark

**Four, not five, and the one that disappeared is the interesting one.** "Append an entry to each
recipient's queue" is gone: there are no per-recipient queues. **The meer holds no per-device state at
all** — the have-set lives on the device, so two devices of one persona get different answers from the
same store simply by asking different questions. That is §6.8.1's "the cursor and the gap-detector are
the same object", arriving for free rather than by design effort.

**What this dissolves.** Deliver-once versus race (§6.6.5, E92) stops being a dial and becomes a
non-question: with no per-recipient queue there is nothing to starve, no prune-on-ack race, and no
dependency on device-group fan-out. The ack still exists but it is *local* — a device pruning its own
have-set — not a signal the meer acts on.

**The one genuine exception: `Welcome`.** It is encrypted to a specific joiner's KeyPackage and is the
only object in the flow addressed to a *person*. It is also — per spike S8 — the largest object and the
first to cross CISS's 2 MiB cap (≈6,350 members with the ratchet tree embedded). So the one thing
needing per-recipient handling is the one thing with a size problem, which is an argument for keeping
`Welcome` out of the meer's v0 scope: without it the meer needs no notion of "user" whatsoever.

### The original (addressed) operations, retained for the record

Five operations, and nothing else:

1. accept a publish (sealed blob + recipient set)
2. `PUT` the blob once to CISS (content-addressed) — **[SPIKE 2026-08-11 (S2): the deposit-once
   claim holds; "stored once" is conditional.** Dedup is **per-namespace** (`blocks/{did}/{cid}`), so
   fifty recipients is one stored object under a meer-owned pool and **fifty under per-DID queues,
   the stated default below.** The saving that is unconditional is **transit**, which is also the
   metered quantity.]
3. append an entry to each recipient's queue chain
4. serve a drain — have/want diff, hand back blobs, ack, prune
5. sweep expired, leave a watermark — **[SPIKE 2026-08-11 (S5): sweeping ends *serving*, not
   *holding*.** CISS's object plane is `PUT`/`GET` with **no `DELETE`**, so swept bytes remain in the
   namespace indefinitely. "Here is what is gone" is false as written against today's substrate;
   "we stopped serving it" is true. Fix designed in **E95** (owner lean: retention declared on the
   `queue` chain kind).]

No ordering, no group state, no key handling, no storage layer, no backup story.

The deposit side is pub/sub (one publish, many recipients); the drain side is a mailbox (each
recipient, own pace, own retention). Both halves are true and they carry the two distinct auth
questions: deposit is gated at the meer's admission policy, drain at CISS account identity.

### Cursors and delivery

**There is no stored offset on either side.** The recipient states the digests it holds; the meer
answers with the difference. Content-addressing is what removes the cursor; monotonicity only makes
the diff cheap enough to range rather than enumerate. This is §6.8.1 arriving from the storage side —
"the cursor and the gap-detector are the same object, not two."

**[SPIKE 2026-08-11 (S4): FALSIFIED as stated — without a device group the second device starves
(measured: it drains its own queue and receives zero). And the framing overstates the trade: racing
across two devices costs **1 deposit, 1 stored object, 2 queue entries** — nearly free at the meer.
The real trade is metadata (racing reveals device count) and retention, versus a dependency on a
§6.6.5 fan-out that is not built. Tracked as **E92**.]** ~~Deliver-once is correct, not a
compromise.~~ **Deliver-once is one side of a dial whose cost was mis-stated.** §6.6.5 guarantees that if any one of a persona's
enrolled devices receives a message, every enrolled device eventually sees it, so the device-Group is
the fan-out and the meer must not duplicate it. Prune on ack. The dial: deliver-once when a device
group is present, race across enrolled devices when it is not — a detectable condition, not a
preference.

Retention is **14 days as a ceiling, not a floor** — "14 days or until drained," never "14 days no
matter what." The unconditional form is a strictly harder promise, makes queue size a function of
traffic rather than of delivery, and has to be held under adversarial load. Past the window the
recipient gets the watermark: a loud, visible, SSH-host-key-shaped "here is what is gone," which is
the no-invisible-loss rule (Part 1 §2.2) doing its job.

### Custody is a dial

Per-DID and meer-owned queues are both valid; the choice is by function. This is a **custody dial**,
sibling to the existing confidentiality dial (Tier 0/1/2/no-mirror) — what the meer can *see* and who
*owns* what it holds are different questions.

- **Per-DID (default).** The mail is the recipient's the whole time. Costs are legible to the person
  paying them. Each recipient has an independent `seq`, so a meer serving hundreds is not contending
  on one lock.
- **Meer-owned pool.** Wins for bootstrap (someone with no account cannot own a queue), for
  idle-heavy scale (one namespace, dedup across everyone, no per-member provisioning), and for anyone
  who would rather the meer not know them by a durable identifier.

The trade is symmetrical and cannot be maximised on both sides: **per-DID buys ownership and legible
accounting; meer-owned buys a meer that knows less about you.**

### Metering and billing are separate decisions

Meter both transit and at-rest; decide independently whether either bills. They rest on different
evidence, which is why they can carry different policy: at-rest is verifiable from the owner's own
signed manifest, transit rests on the custodian's attestation.

Metering without billing also resolves the abuse case. Because mail lands in the recipient's
namespace, spam would otherwise cost the victim money; if transit is metered but not charged, the
owner-declared ceiling does the defending and the meter only watches.

The transit meter is not generic telemetry — it *is* the **offline-data fraction**, the number
`meer-superpeer-design.md` names as the only thing that sizes a meer fleet and lists under open edges
as unmeasured. It falls out of the accounting rather than needing instrumentation.

**Meter retention must be bounded.** Otherwise mail purges at 14 days while a per-account profile of
who receives how much, when, and in what sizes never expires — the meer's most sensitive artifact
being the one thing that outlives everything. Rolling aggregates answer fleet-sizing as well as a full
ledger does. And whatever the meer keeps about a person, that person should be able to read (the
metadata-transparency guard, `meer-superpeer-design.md` item 6).

### The generalisation: substrate and gateway services

The reusable thing is not the meer. It is the **typed-chain substrate** — kinds, custodial write,
ceilings, per-chain metering. Above it sit thin **gateway services** that give shape and context:
mirror-history, file, chat, meer. They are not plugins and not products; they are framing.

Protocol adapters stay outside CISS. The meer speaks iroh QUIC and CISS speaks HTTP; the CISS server
today binds **no** iroh listener (`ciss-iroh` is depended on only by `ciss-cli`), and putting a QUIC
listener inside the thing that holds everyone's data and signs everyone's receipts is a real widening
of its exposure. Deployment is separate regardless, since queue-mode and store-mode want different
boxes (high write rate / tiny objects / 14-day churn / no backup, versus low write rate / large
keep-sets / indefinite retention / backup essential), and separate instances also buy AR-4
compartmentalisation. This is the croft-groups resolution reused: **deployment axis is an isolated
instance; code axis is a shared substrate.**

**Custodian mode is what makes the gateway pattern meter honestly.** A gateway acting as itself would
bill everything to its own account; acting as custodian, bytes meter to the person they belong to.

## Reasoning

**Why the meer is unblocked while the history store is not.** Sealing the per-author index is a single
decision with two consequences: a member can detect gaps, and a blind carrier cannot order or
attribute. "Being blind and not ordering are the same fact." So the meer needs no fold, no envelope
structure, and no mirror group. The history store needs G-hist membership, nested double-sealing, and
an envelope whose byte layout is still `[gates-release]` — genuinely gated, and genuinely a different
component despite sharing the substrate.

**Why kind-gating rather than per-write authorization.** A type check on an owner-signed,
genesis-fixed field is a structural gate; a grant lookup is a policy gate that can be misconfigured.
Two independent conditions must hold, and the cheap one runs first and fails closed. It also bounds
the security review to "can a non-queue chain ever be written custodially," which is a tractable
question, rather than "is every custodial write path correctly authorized," which is not.

**Why per-DID is the friendlier default.** The anti-entrenchment guard in `meer-superpeer-design.md`
is state portability — "a meer that becomes the sole custodian of recoverable state is the
entrenchment failure." Per-DID queues do not satisfy that guard, they **retire** it: there is nothing
to port because it never left home. Portability is a promise that a helper will give your data back;
ownership is not having handed it over. The user-facing form: *the always-on helper that catches your
messages while you sleep never holds them — it has a one-line, revocable permission to add to one
pigeonhole on your own plot, it cannot read what it puts there, and it cannot touch anything else.*
That is a materially different claim from every self-hosted or federated messenger, where the messages
live in the server's database and "you can export them" is the mitigation.

**Why drain authorizes on account identity, never MLS identity.** Presenting group credentials to a
blind store would tell it which groups you are in — metadata the blindness exists to prevent.
**[SPIKE 2026-08-11 (S7): the gate is right, this rationale is not.** `group_id`, `epoch` and
`content_type` are **cleartext** in the MLS `PrivateMessage` framing, so the meer already learns
which groups you are in from every message it stores — measured: two messages to one group are
linkable with no key. Refusing MLS identity avoids **adding** a credential-based disclosure; it does
not prevent group-linkability. The corpus owns the fix (nested double-sealing, specified for the
history store) and has not applied it here. Tracked as **E96**.]** MLS
identity is also the wrong granularity (one mailbox holds mail from all groups) and epoch-bound (a
rekey would lock a persona out of their own mailbox). Entitlement is enforced by the seal, not by the
drain gate: a meer holding mail for someone who cannot decrypt it has wasted bytes, not leaked
anything. That is the same property that lets the meer be blind at all.

**The honest limits.** A member can validate everything the meer *asserts* — the chain is hash-linked,
entries are signed, and the mailbag is checkable against what was actually drained, so over-reporting,
under-delivery, and reordering are all caught. A member cannot detect **withholding**: something handed
to the meer that never appears. That is undetectable by construction and is why D-peer corroboration
exists as an independent source. And the kind gate bounds bleed-over *between* chains; it says nothing
about a custodian misbehaving *within* a queue, which is what the ceiling, revocation, and the
untrusted-by-assumption posture are for.

## What we test next (the spike)

**[SPIKE 2026-08-11: RUN. This section is kept as written for the record; the corrections are below.]**

Against **plain CISS as it exists today** — no custodian mode, no chain kinds. The meer owns one
namespace, mail goes in slots, ~~two OpenMLS clients (the MIT `cli` PoC)~~ **two real OpenMLS clients
built in-process from `mls-replant` and `mls-welcome-over-iroh`** (the PoC line was stale — those
ancestors are ours, already Rung A, and adding a third-party delivery service to test a design whose
premise is that everyone else's DS is their product would have been backwards), one offline; prove
the drain and the decrypt.

**Result: the central claim HELD.** M1 CONFIRMED (real-lib) — a member offline for a message's entire
live window drained it from the meer and decrypted it through real `process_message`, with the meer
holding zero group keys. M2's positive arm CONFIRMED — bytes are byte-identical across store and
serve. **A store-and-forward node that does no ordering, holds no group state and holds no key is
sufficient to carry a real MLS conversation across an absence.**

Six subsidiary claims were corrected (S2, S3, S4, S5, S7, S1) and are marked inline above and below.
None of them threatens the shape; all of them were assumptions the design had not measured.

This tests the thing we are actually unsure about — whether the pub/sub-in, mailbox-out shape holds up
against real MLS traffic — without paying for the substrate first. The substrate carries the security
review cost, so it should be informed by the spike rather than guessed at ahead of it.

## Open

- **Who mints the custodian grant:** per-persona enrolment out of band (lean: matches §5.4, "a
  persona's *use* of a meer is a per-persona decision") versus riding Group governance so a Group can
  point its members at a meer collectively.
- **Meter retention policy** — aggregate shape and window.
- ~~**Welcome and GroupInfo versus the 2 MiB object cap**~~ — **[SPIKE 2026-08-11 (S8): MEASURED,
  this item is closed.** Application messages are **flat at 181 bytes** at every group size and never
  approach the cap. Everything that grows is **linear**, not `~log N` as assumed. Crossover order:
  `Welcome`-with-tree ≈ 6 350 members → add-all commit ≈ 7 440 → `GroupInfo` ≈ 11 780 →
  `Welcome`-without-tree ≈ 13 790 → ordinary commits ≈ 25 500. **The cap binds in the thousands, not
  at conversational sizes**, so CISS needs no streaming rewrite to be the meer's substrate — though
  ordinary commits *do* cross at broadcast scale, which is the tier §6.9.1 already treats separately.
  Chunking is not needed below ~6 000 members. **Shipping the ratchet tree out of band is already the
  corpus's de-facto behaviour** (`mls_replant::stamp` returns it separately) and buys ~2× headroom —
  arrived at incidentally rather than decided, and currently undocumented. **Best case only:** one
  ciphersuite, `BasicCredential`; real credentials move every crossover down. Full table:
  `alpha/experiments/meer-queue/S8-RESULTS.md`.]**

- **[SPIKE 2026-08-11 (S1, S3b, S5): new open items the spike surfaced.]**
  - **How do senders learn which meer to deposit at?** Enrollment is framed as Bob ↔ meer, but Alice
    needs a resolvable announcement and none is specified. Revocation is consequently one-sided —
    clearing the custodian field stops the meer *writing*, not senders *depositing*. **E97.**
  - **Duplicate handling is settled** (owner, 2026-08-10): keep an in-memory delivered-hash cache as
    the fast path and `SecretReuseError` as its repair path, so the cache needs no persistence and the
    processed-but-not-acked crash window closes for free. Safe because `SecretReuseError` (benign
    duplicate) and `TooDistantInThePast` (unrecoverable loss) are **distinct variants** — a client
    must match the variant, never "any processing failure."
  - **Object lifecycle in CISS.** No `DELETE` exists, so retention cannot be honoured at the storage
    layer. Two designs in **E95**; owner lean is **B**, retention declared on the `queue` chain kind
    and enforced without the owner online.
- **Custody-dial default per Group**, and whether bootstrap always implies the pooled mode.

## References

`beta/drystone-spec/part-2-certifiable-design.md` §5.4, §6.6.2, §6.6.5, §6.8.1, §6.8.5 ·
`beta/impl/delivery-layer/01-delivery-architecture.md` · `beta/cairn/mls-and-mimi.md` ·
`thinking/meer-superpeer-design.md` (predecessor) · `plans/croft-stack/10-drystone-layer.md` ·
`CISS/docs/SECURITY-POSTURE.md` (B1/B3/B6/Z7), `CISS/docs/plans/2026-08-05-gated-reads-authorization.md`
(the read-side mirror of custodial write) · ROADMAP_TODO E82, E90.
