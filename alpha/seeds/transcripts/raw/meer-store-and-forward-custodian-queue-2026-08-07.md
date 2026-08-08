# Raw: the MLS store-and-forward meer becomes a custodian queue over CISS (2026-08-07)

**Preservation status: user turns preserved VERBATIM (live session, not a reconstructed paste —
including voice-dictation artifacts, kept as-is). Assistant turns preserved CONTENT-FAITHFUL — all
load-bearing technical content, diagrams, tables, corrections, and reversals retained; purely
connective prose tightened. NOT byte-pristine on the assistant side. Tool invocations (greps, file
reads, web searches) are summarized as bracketed notes rather than reproduced. PLAYBOOK §4.**

**Session:** Claude Code (Opus 5), working directory `CroftC/`, 2026-08-07.

**Why this is filed:** the session moved the meer from "a service with its own store" to "a thin
gateway over CISS custodian chains," which is a structural change to a Phase-10 component and to
CISS's authorization model. The *path* matters as much as the conclusion — several assistant claims
were corrected mid-session, and two design reversals happened in dialogue.

**Distilled into:**
- `alpha/thinking/meer-as-custodian-queue.md` (the hypothesis / conceptual alignment)
- `alpha/experiments/meer-queue/SPIKE-SPEC.md` (the Phase-0 experiment)
- `alpha/plans/2026-08-07-meer-lane.md` (the lane this experiment is Phase 0 of)

**Supersedes the storage half of** `alpha/thinking/meer-superpeer-design.md` (2026-06-16); that doc's
roles, confidentiality tiers, and anti-entrenchment analysis still stand.

---

## Turn 1 — user

> can you look at our croft-stack setup and look at the MLS store-and-forward server expectations
> defined in drystone and for croft ecosystem
>
> I want to start building out that MLS store and forward server and deploy it to our VPS but we need
> to walk it out first and what it entails

## Turn 1 — assistant

*[Read croft-stack CONTRACT.md/README/TODO, services manifests, broker README, RUNBOOK ADD-A-TENANT;
drystone Part 2 §5.4, §6.6, §6.8.5, §6.9–6.11; delivery-layer 01-delivery-architecture.md in full;
mls-hardcases-and-posture.md; plans/croft-stack/10-drystone-layer.md; located the meer P0/P1 prototype
at `experiments/alpha/iroh/crates/relay-loadtest/src/meer.rs`.]*

Lead finding: **the corpus describes two servers, not one**, and they get conflated.

- **The meer** (D-meer, Part 2 §6.6.2): "hold Bob's undelivered sealed bytes until he dials home."
  Delivery-bounded retention. Knows nothing about MLS — opaque bytes plus a have/want digest set.
  Spec status settled, `Verified`, no open `[confirm]`s.
- **The history store** (§6.8.5): archival, keeps full sealed history. Needs a mirror group G-hist,
  nested double-sealing, a reconciliation envelope. Envelope byte layout is `[gates-release]`; the
  G-hist resumption-PSK binding is `[confirm]`.

Phase 10 gates the *convergence server* on "drystone's fold/MLS becoming real" and warns against
pre-building against a reference-model fold. **That gate binds the history store, not the meer** —
because the per-author index is sealed (§6.6.2), the meer cannot order and does not need to, so it
needs zero fold knowledge.

Normative constraints on the meer: byte-identical forwarding (a re-sealed copy could induce ratchet
key or nonce reuse); never issued a key; no ordering role; not a Group principal; redundant and
rotatable; retention delivery-bounded, and since gossip gives no per-recipient ack (§6.10.4), drop
decisions must ride dial-home acks.

Reuse inventory: the meer prototype (366 lines, already has `Publish`/`Sync{have}`/`Export`/`Import`/
`Stats`, admission allowlist, `meer_payload_keys_held=0`); CISS live at `ciss.croft.ing`; `ciss-iroh`;
the live iroh relay as the precedent for a non-HTTP UDP service; `mls-welcome-over-iroh`.

Two cautions on the prototype: its `BlobMeta` carries cleartext `ts` and `namespace`, but §6.8.5 says
the envelope MUST NOT carry wall-clock and the subspace must be hashed — June-era spec drift. And the
delivery-layer round-2 experiment crates (e6–e11) **do not exist anywhere under `CroftC/`**, so there
is no live MLS traffic to point a meer at.

Deploy shape: contract-fit is satisfiable, UDP port as the relay's documented precedent, `data_profile`
= blobs (content-addressed → `blobs_immutable`) + canonical (roster, cursors).

Four decisions put to the user: scope (meer only), storage substrate (own store vs CISS), repo home,
admission model.

## Turn 2 — user

> teh history sotre is one of the ciss functions
>
> the meer naming shifted but landed on the name we gave to our store-and-forward mls server bc it's
> not doing ordering, meerly delivery
>
> there should be MLS servers even written in rust already that we can look at to either just use with
> ignoring ordering or commeent our ordering or I'm not srue
>
> and yes we are building the server to then start meaningfuly testing clients
>
> so a basic MLS client we can use to test woudl be useful
>
> I'm not understanding teh sqlite, isn't tis just a queue, is that the queue part?
>
> and set aside lightstream for now, mor eor less I thikn we want, queue and sotre for 14 days and if
> not invalidate and purge
>
> 1) yes meer (ciss is already the store)
>
> 2) I guess we could adapt ciss to be the queue for the meer but let's discuss
>
> 3) sure, but again the meer is effectively an MLS store and forward server that dos not do ordering,
> we soudl look at MLS reference impelemtnations first and foremost
>
> 4) so we awnat to know how clients auth to teh meer?

## Turn 2 — assistant

*[Read cairn/mls-and-mimi.md; web searches on OpenMLS delivery service and Phoenix R&D; fetched
github.com/phnx-im, openmls/delivery-service/ds, mls-assist.]*

**The SQLite is the queue, and that's all it is.** Two pieces of state, which the previous turn
conflated:

```
   ┌─ the bytes ────────────────────────────────────────────┐
   │  sealed MLS PrivateMessage, content-addressed          │
   │  one blob, stored once, fanned to N recipients          │  ← CISS
   └────────────────────────────────────────────────────────┘
   ┌─ the queue ────────────────────────────────────────────┐
   │  (recipient, blob_digest, enqueued_at)                  │
   │  ~64 bytes a row. no content. no ordering.              │  ← the meer
   │  + a per-recipient purge watermark                      │
   └────────────────────────────────────────────────────────┘
```

An index, not a store. Phoenix R&D's homeserver splits the same way — a **delivery service** and a
separate **queue service** — independent convergence on the same seam.

On Litestream: agreed to set aside, **and there is a principled reason**. Part 2 §5.4 commits to
"losing a meer costs availability, never data." Backing up a meer would quietly contradict the
design's own claim. What 14-day expiry does need: **purge must be visible** (§2.2 permits loss,
forbids invisible loss) — keep the per-recipient purge watermark after the bytes are gone.

**Prior art: there is no MLS server to adopt-minus-ordering.**

| What | License | What it is |
|---|---|---|
| OpenMLS `delivery-service/ds` + `ds-lib` + `cli` | MIT | Explicitly "a proof-of-concept for an MLS delivery service that can be used for testing" |
| phnx-im/air (was `phnx`) | **AGPL-3.0** | The real one. DS + QS split, absorbed `mls-assist` (archived Oct 2025, moved into `air`) |
| MIMI | — | Per the cairn survey, "reintroduces a hub": one provider per room orders and enforces policy |

Structural finding: **a real MLS DS is big not because of ordering but because it holds group state
and validates commits.** Strip ordering *and* group-state validation and what remains is a queue with
blind blob fan-out. There is nothing left to fork. The prior art pays off as three things instead:
OpenMLS's `cli` as the test client, `ds-lib`'s message vocabulary as a sanity check, and `air` as a
read-don't-vendor reference.

**CISS as the queue — the discussion.** CISS is the store, the meer is the queue, and they shouldn't
merge; CISS's model is DID-keyed owned manifests with rent, receipts, seal and grace, all statements
about a customer's own data. Mail in transit isn't that. But a better relationship is available:
**the meer as a CISS customer** — one DID, mail as its keep-set, 14-day churn, dedup for free, and the
cost twin makes "what does a meer cost" readable.

**Auth is two questions.** Deposit (spam/exhaustion vector) and drain (traffic analysis, and if you
can ack you can delete their mail). Drain has a clean answer needing no new credential: the iroh QUIC
connection is already authenticated to an EndpointId, so "you may drain only the queue keyed to your
own EndpointId" — self-scoping the way croft-stack's own-data API addendum requires.

## Turn 3 — user

> I've been thinking a lot about the meer service just being a shim on ciss and keeping the durable
> queue there, with a chain per receiver maybe, then a ciss backup and ha model is complete

## Turn 3 — assistant

*[Read CISS SYNC-MODEL.md, SECURITY-POSTURE.md §7 (B1–B5), gated-reads plan, blobstore.rs.]*

The shape works, and the "chain per receiver" is machinery CISS already shipped — but there's an
ironic catch.

**The namespace decision determines everything.** Manifest writes are owner-signed (`derive_id(key) ==
did`, B1), so a sender cannot write into a receiver's namespace. The queue must live in the meer's own
namespace with the meer as signer. Not a compromise — the chain is meer-attested, tamper-evident
against a *successor* meer, which is already the trust posture.

Chain-per-receiver is the M3 frontier renamed: `heads: device_id → cid(DeviceHead)` becomes
`slot → cid(QueueHead)` with **zero server change**, since B1 says the server validates the owner
signature and seq-monotonicity and stores bytes while the fold stays client-side. And the gated-reads
plan's own Problem Statement names "**history-convergence** backend" as a motivating consumer.

**Three that bite:**

1. **Seq-CAS contention.** Slot discipline makes commits non-lossy, but every deposit still bumps one
   `seq` on one manifest — a single ordered slot, reintroducing at the storage layer exactly the
   serialization declined at the protocol layer. Fix: decouple blob PUT from manifest commit; the
   manifest is the retention anchor, not the delivery path.
2. **The leak profile has conditions.** §6.8.5 requires a pseudonymous slot key (never a raw
   EndpointId), padded sizes, no wall-clock. Get the slot key wrong and it's a spec violation.
3. **The 2 MiB cap is hard.** `MAX_OBJECT_BYTES`, refused on put *and* get. Welcome/GroupInfo for
   large groups are the risk; the cairn survey flags exactly this as the deployer's problem.

**On backup/HA:** not contradicting the earlier "don't back up the meer" — the invariant is that the
*group must not depend* on meer durability, not that the meer must be fragile. Better: **invariant B6
(exit-exempt) becomes the anti-entrenchment guard** — the meer can always drag its full queue out,
even under an exhausted ceiling.

## Turn 4 — user

> well we could have a chain mode in ciss that allows for a custodian, who can keep chains per did on
> behalf of and then we get the cost in transit and at rest for each did included and ciss is our own
> thing and I wouldn't want to mix queue mode ciss with durable store mode in most scale cases so it
> really becomes a scaling model

## Turn 4 — assistant

*[Grepped CISS for existing custodian/delegated-write; found delegation exists only on the read side.]*

Better than the prior proposal, and worth saying why: **it dissolves the seq-CAS problem outright** —
N receivers get N independent manifests with N independent `seq` counters. The single ordered slot
stops existing.

Custodian mode is the **write-side mirror of gated reads** — same record shape, same two signing
models, same Z7 anti-rollback.

**The one rule that makes it safe:** the custodial chain must be a **separate record with its own
seq**, not a mutation of the owner's manifest. If a custodian could write R's manifest it could delete
R's keep-set, inflate R's rent, or roll it back. Separate record means B1 and B3 keep their existing
proofs.

**Transit vs at-rest falls out honestly:** in-transit bytes rest on the custodian's attestation;
accepted-into-your-own-manifest bytes are a pure function of your own signed document.

**The tension, named:** per-DID cost attribution and pseudonymous slots pull against each other, since
custodian mode makes the DID the chain key. Resolution: CISS's `id:<64-hex>` is self-certifying, so a
persona can mint a **per-meer-relationship identity** — costs stay legible, the meer learns a durable
identifier that doesn't link to the public DID.

**The scaling model is the strongest part** — queue mode (high write rate, small objects, 14-day
churn, no backup) and store mode (low write rate, large keep-sets, indefinite, backup essential) want
different boxes. One binary, runtime mode, two croft-stack tenants — matching the croft-groups
resolution (deployment axis isolated instance, code axis mode-vs-shared-lib).

**The cost, stated plainly:** this is the most security-sensitive change available to CISS — the first
time a non-owner causes a write in an owner's namespace.

## Turn 5 — user

> I would want to gate this by chain type like we have history convergence, file sync, etc, this would
> be a queue type and it woudl be the only one where custodial write is possible and it would be a
> quick easy failure check and impossible to bleed over

## Turn 5 — assistant

*[Verified the `kind` self-tag in ciss-sync: "the leading `kind` self-tag domain-separates the hashed
pre-image."]*

Already CISS's idiom rather than a new invention. Four rules make the gate real:

1. **Type in the preimage** (already the idiom).
2. **Type immutable at genesis** — otherwise file-sync → queue → file-sync lets custodial writes be
   reinterpreted as owner-authored content.
3. **Per-type prefix, not just a per-type field** — a signature for one type becomes *structurally*
   invalid on another.
4. **Fail closed on unknown**, including types that don't exist yet.

**The honest cost:** today the server "never interprets a head." Custodian mode breaks that
necessarily — a client-side check on a custodial write is not a check. So make it exactly one field:
read the tag, compare, refuse. Naming the erosion beats pretending it didn't happen.

**Making the claim mechanical:** the load-bearing test is exhaustive over the chain-type enum, so a
new type breaks the suite until someone states its custodial posture.

Scope note: the gate bounds bleed-over *between* types, not custodian misbehaviour *within* a queue.

## Turn 6 — user

> ok, talk to me about chain kind now if it's not part of a head

## Turn 6 — assistant

**Two different things are called `kind`,** and conflating them is the bug:

- **Blob-format tag** — `DeviceHead` carries `kind: DEVICE_HEAD_KIND`; format identity, client-side.
- **Chain kind** — a property of the *slot*, not of any blob in it.

**Why it can't ride in the head:** it would be self-authorizing (the custodian declares its own kind on
the write being evaluated); it would cost a fetch and a parse in the write path; a slot needs a kind
before it has any heads; and immutability would reduce to comparing against the previous head, making
a chain reset a retyping vector.

**Where it goes** — the slot declaration in the owner's manifest:

```
   Manifest (owner-signed, seq, B1/B3)
     └─ slots: { slot_id → { kind, custodian?, head_cid } }
        preimage: …:slots=<sha256>        ← exactly how :heads=<sha256> was added
```

**This revises the previous turn.** The split should be: **declaration** (kind + custodian) in the
owner's manifest, **content** (deposits) in a separate custodian-signed record. Drops a second record
and the cross-record consistency question. Revocation becomes an ordinary manifest update inheriting
B3 anti-rollback.

## Turn 7 — user

> ok, talk through that model in plain english and how it works

## Turn 7 — assistant

*[Narrative walkthrough: Bob sets up a mailbox — one line in his signed inventory saying "pigeonhole
#7 is a mailbox, the meer may drop things in it." Alice sends while Bob sleeps; the meer stores bytes
in CISS and appends to the mailbag, never touching the inventory. Bob wakes, drains over a connection
already tied to his device identity, then *accepts* mail into his own signed inventory — which is where
the transit/at-rest billing distinction comes from. Nobody comes for it: purge at 14 days, leave the
note, because the design permits losing messages and forbids losing them invisibly. Bob fires the meer
by deleting one line; his mail doesn't move, because it was in his account the whole time.]*

Capability table: the meer can read nothing (never held a key), add only to pigeonhole #7, touch
nothing else (wrong kind), delete nothing (can't sign the inventory).

## Turn 8 — user

> this is pretty good, and bob's queue chain coudl be on the same ciss as his file chain or another,
> makes no difference and bob really needs to request his messages with his mls identity not his iroh
> one right? but either way

## Turn 8 — assistant

Agreed on the first — the model is indifferent, though two reasons favour *separate* (workload
profiles; metadata compartmentalization) and one cost (accept-into-at-rest becomes a cross-server
copy).

**On MLS identity: I think it's neither, and that's better.** The right answer is the **CISS account
identity**. Three reasons MLS identity is wrong:

- **Wrong granularity** — one mailbox holds mail from all groups; an MLS leaf in Group A says nothing
  about Group B.
- **It rotates** — epoch-bound credentials would lock Bob out of his own mailbox on rekey.
- **It leaks the thing the meer is designed not to know** — presenting group credentials to a blind
  store tells it which groups you're in.

Underlying principle: **entitlement is enforced by the seal, not the drain gate.** A meer holding mail
for someone who can't decrypt it has wasted bytes, not leaked anything — which is precisely why it can
be blind at all.

**One thing that follows, unasked:** if mail lands in Bob's account, **Bob pays transit rent on
it** — spam costs the victim money. The meer is the writer, so the deposit gate lives at its admission
policy, and Bob can cap the slot. Worth an explicit invariant: *a custodian must never grow a slot past
the owner's declared ceiling.*

## Turn 9 — user

> we could choose to track and not charge in the meer case, just bc we have good accounting doesn't
> mean we use it to bill, it's also a scaling and intelligene asset, but yes, I think bob having a
> chain queue that has allowed a meer custodian is appropriate, and the contenst of that chain at-rest
> and in transit meter are 2 questions for outcome

## Turn 9 — assistant

Metering-without-billing resolves the spam case: transit metered but not charged means the ceiling
does the defending and the meter only watches.

**The intelligence asset has a specific name in the corpus.** `meer-superpeer-design.md` names the
**offline-data fraction** as the only number that sizes a meer fleet, invisible in relay metrics, and
lists it under open edges as *unmeasured*. The transit meter **is** that measurement — it closes a
specific open item as a byproduct.

**The caution that comes with it.** Same doc, item 6: the meer sees metadata members don't; surface it
and keep it minimal. Which surfaces a gap nobody had decided: **the mail purges at 14 days, the meter
doesn't, unless you say so.** The meer's most sensitive artifact would be the one thing that never
expires. Two things to settle: bounded retention on the meter (rolling aggregates answer fleet-sizing
as well as a full ledger), and surfacing it (if the meer keeps a number about Bob, Bob should read the
same number).

## Turn 10 — user

> My thinking is that's like one shape of the mirror cue, you know, and I think it's likely the one
> that we want to start with. Can we I'm surprised that there's not an MLS store in deliver server we
> can just you know copy but I guess really we just need a a kind of pub sub shim over the the sys
> server.

## Turn 10 — assistant

*[Wrote the memory file capturing the architecture decision.]*

**Why there's nothing to copy — the absence is structural.** RFC 9750 deliberately doesn't specify the
DS, so there's no spec to implement and every DS is bespoke to its product. And a DS in practice *is*
the server half of a messenger: identity, group creation, KeyPackage hosting, Welcome distribution,
room policy, ordering. OpenMLS ships a PoC; Phoenix ships a real one inside a whole AGPL application;
MIMI standardized the role by putting an ordering hub in the middle. **Everyone else's DS is their
product, and we're building only the part that isn't.**

**Pub/sub is half right, and the half matters:**

```
  DEPOSIT side  — pub/sub          DRAIN side — mailbox
  one publish, many recipients     each recipient, own cursor,
  blob stored once (dedup)         own pace, own retention window
```

Classic pub/sub is fan-out-to-connected; the whole point here is fan-out-to-absent.

The shim is five operations: accept a publish; PUT the blob once; append to each recipient's chain;
serve a drain (have/want, ack, prune); sweep expired and leave the watermark.

**The de-risking move:** take OpenMLS's `cli`, stand up two members, put one offline, prove the drain
and decrypt. Turns the remaining uncertainty into a yes or no for the price of a day.

## Turn 11 — user

> Right, and part of the interesting thing is is that we don't have to keep a cursor because ordering
> is monotonic, right? Like where we are is where we are. Okay, so we need a service that sits on top
> of Sis, that accepts a publish, puts the blob to SIS, pedited tree to each recipient's Q chain.
>
> Serve a drain. Sweep expired. Leave the watermark. No ordering. Right? No group state. No key
> handling. No storage layer. No backup story. Yep. Yeah, I think you're right the spike just put the
> thing together and see what's what but the other thing that I'm thinking is is like it's really
> interesting what happens once you start thinking in like cryptographic storage objects and I'm
> thinking maybe this won't be the last sort of thing that's like sis plus whatever.
>
> So I'm wondering if we could even write this as like a I don't know a module or a plugin or something
> because like Sys already listens on the network already serves up storage records like the difference
> between this thing and the history convergence server is almost ideological rather than functional,
> you know.

## Turn 11 — assistant

**Right conclusion on the cursor, refined reason:** it's not monotonicity, it's the **have/want diff**.
Content-addressing removes the cursor; monotonicity only makes the diff cheap enough to range rather
than enumerate. §6.8.1 says the same thing from the protocol side — "the cursor and the gap-detector
are the same object, not two."

**Multi-device is the one thing that could drag a cursor back:** ack-prune per device is a cursor;
prune-when-all-acked is a cursor; **don't prune on ack at all** keeps cursor-freedom but costs bytes.

**On "this won't be the last CISS-plus-whatever"** — the corpus already applies this pattern (AppView
tiers as "policy on one engine"; croft-groups as an AppView with a different write policy). But
pushing back on "almost ideological":

| | meer | history store |
|---|---|---|
| network service over CISS storage | same | same |
| holds what it can't read | same | same |
| have/want diff over content-addressed blobs | same | same |
| retention | 14 days | indefinite |
| who writes | custodian | owner/members |
| sealing | pass-through MLS bytes | **mirror group + nested double-sealing** |

Top half is one thing; bottom half is a real functional difference.

**Recommended factoring:** not "the meer is a CISS plugin" but **CISS grows a typed-chain substrate**
(kinds, custodial write, ceilings, metering) with **thin consumers outside** (meer via iroh ALPN,
history store via G-hist). Protocol adapters stay out because CISS speaks HTTP and a QUIC listener
inside the thing holding everyone's data widens its exposure.

**Sharpened spike:** run against plain CISS as it exists today — no custodian mode, no chain kinds —
so the substrate is informed by the spike rather than guessed at.

## Turn 12 — user

> Yeah, I wasn't thinking m monotonicity. I was thinking like it's basically a range query, right? like
> here's where I'm at, what's ahead of me. you know, that kind of thing. multi-device if act then Bob's
> phone drains right I see you're saying if it's cursor if it's not per device okay so act prune per
> device device.
>
> That's a cursor. Now store a set of per device positions. Yeah, you're right. Prune when all enrolled
> devices have act. Also set of positions. also a cursor, yep. Or yeah you're right make them but if
> you try to keep everything for 14 days that's just a lot of unnecessary bloat and seems like it just
> makes everything more difficult, you know, especially when you're trying to troubleshoot or maintain
> cue sizes or god knows what like and you definitely don't want to be susceptible to like you know
> huge fluctuations and I mean I guess in the per device prune you're already just storing it until all
> devices.
>
> So I guess yeah, you could really I think you could choose. You could say, you know, delivered once is
> fine for me, and then the history convergence layer on their side takes care of it. you know, because
> really you're delivering to their device group, right? And then their device group also shares.
>
> So the sort of end-to-end of our design thinking here does give us some compensating factors. But I
> think like you enroll so I think users enrol a device to drain and if they enroll two devices then you
> you get, you know, I guess you could do per device or race.
>
> And I mean that's just a toggle setting. So to me it I think this is one of those dial things. Or
> yeah, to your point, you could just not prune it all and say whatever the whole duration is, that's
> the whole duration. Then you would also have to stack on a you'd have to I mean the only way I'd be
> comfortable with that is if you stacked on you know some kind of size or rate or like the point at
> which you're saying I'm gonna keep everything for 14 days no matter what now I start to feel nervous
> because that's a much harder guarantee than for 14 days or until a drain at least to my mind maybe
> maybe not now I'm really thinking about it but honestly I don't know that the cursor's that expensive
> server side and I guess I suppose if there's a Yeah, it just seems to me like it's a range query, but
> then what if you're further than 14 days out, well then you either have to have had history from your
> own device group, in which case you can come back up to like where the 14 days starts or you know you
> have to basically it's like a a key warning on an SSH server like whoa, are you sure that this is the
> thing that you want?
>
> You know what I mean? Okay, so not like plugins, but companion processes that are made to I guess, but
> then you're well i'm just thinking like how can we preserve metering two reasons to keep the protocol
> adapters outside we speak http mir speaks I wrote quick. Putting a quick listener inside the metered
> store meaningfully widens its explo exposure.
>
> That's true. I mean, I thought the I suppose right now I think we do have an Iro listener inside the
> SIS because that was part of the file chain type and the file history you know convergence backup so
> I think we already have it but your point that kind of transport and storage, you know, that deep a
> coupling starts to get ugly fast.
>
> So maybe we should you would have a mirror history store, a file store, chat store. And they would be
> kind of thin shaping sort of framing services effectively like giving kind of shape and context and
> like a you know it's like they're gateway services almost And yeah, I think that we should test this
> for sure. I think the next plan is to take all of this thinking, clean it up and persist it to like
> the hypothesis file, right? Here's what we're thinking, here's what we're trying to do. Now we then we
> create the experiment file of like let's go kick the tires and see if we can make it happen and what
> we learn from it.
>
> But like in a small controlled way. 'Cause we're really thinking through conceptual alignment

## Turn 12 — assistant

*[Verified: `ciss-iroh` is depended on only by `ciss-cli`; the `Endpoint::builder` is in the crate,
driven by the CLI. M4's own note was "Server change: none."]*

**Correction: the CISS server has no iroh listener.** The peer-fetch is client-to-client; the server
still speaks only HTTP. The exposure concern is unpaid, and the instinct was right.

**The device-group point retires the multi-device problem.** §6.6.5 guarantees that if any one enrolled
device receives a message, every enrolled device eventually sees it — so **deliver-once is correct by
design**, and the meer has no business duplicating the fan-out. Caveat: it makes the meer's sufficiency
depend on the device group being enrolled and running, so the right default is "deliver-once when a
device group is present, race otherwise" — a detectable condition rather than a preference.

**Agreed that unconditional retention is the scarier promise.** "14 days no matter what" is a floor you
must hold under adversarial load and makes queue size a function of traffic rather than delivery.
Ceiling, not commitment. The beyond-the-window answer is the SSH-host-key shape — loud, visible.

**On preserving metering:** custodian mode is the answer and it closes the loop. A gateway acting as
itself bills everything to its own account; acting as custodian, bytes meter to the person they belong
to. The thing invented to hold mail is what lets any shaping service front CISS honestly.

*[Wrote `alpha/thinking/meer-as-custodian-queue.md`.]*

## Turn 13 — user

> Explain the two megabit cap issue max object bytes. Let's scope it to offline member drains and
> decrypts and also exercise, yep, the bite identifical forward. Yeah, let's let's really walk it out.
> Like this is the first time we're working through this thing in practice, not just in theory.
>
> So let's kind of set ourselves up to learn what we need to to understand the shape of the problem,
> which to me means put it through the kind of user stories and workflows that we know about to learn
> how they shape up, you know.

## Turn 13 — assistant

*[Found OpenMLS 0.8.1 already in the workspace via `mls-replant`, and `mls-welcome-over-iroh` already
building a real OpenMLS group with a Welcome over a real iroh connection.]*

**The 2 MiB cap** (mebibytes): `MAX_OBJECT_BYTES = 2 * 1024 * 1024`, refused on put *and* get, plus
axum's `DefaultBodyLimit`. **Not arbitrary** — it came from the 2026-08-03 security review, where a
512 MiB upload was fully buffered in RAM against `MemoryMax=384M`, so one unauthenticated request could
restart the service. Raising it re-opens a closed vulnerability unless streaming replaces buffering.

Collision with MLS: application messages are small, commits grow with log N, but **`GroupInfo` with the
embedded ratchet-tree extension is O(N)**, so `Welcome` is the object at risk. The meer can't chunk by
re-encoding (M2 forbids it). And the corpus already resolved this from the protocol side — Part 2
§6.9.1 mandates the broadcast tier ship the tree out of band. **What nobody has is the number.**

*[Wrote the spike spec: two must-pass claims (M1 offline drain-and-decrypt, M2 byte-identical with a
negative re-framing arm) and eight shape-learning scenarios, three registered SPEC-DELTA stand-ins,
falsification criteria written before the run.]*

**The find that de-risks it:** the spike isn't "write an MLS client" — it's "extend
`mls-welcome-over-iroh`, which already does the hard half." Fidelity note recorded: round-2 delivery
experiments ran against `mls-rs 0.55.2`, which isn't in this workspace; this runs against OpenMLS 0.8.1.

## Turn 14 — user

> Yes, let's go ahead and walk all this out as a experiment and also this whole transcript really needs
> to be filed. Itself is a raw transcript. It's meaningful enough. So we should get that. And we should
> get the experiment and we should get a file about you know kind of the the general plan of which this
> experiment is basically phase zero right so this is a discovery phase practically and then it kind of
> tells us what the shape of the things that we've mostly sketched out but need some direction on right
> so let's go ahead and get all the documents together and commit everything first here.

## Turn 14 — assistant

*[Discovered the spike spec had been misfiled to the frozen standalone `experiments/` repo (last commit
2026-07-29) rather than the folded, active corpus at `discovery/alpha/experiments/` (PLAYBOOK §3b:
"Commit in `discovery` (the single active repo; the folded `Proofs`/`experiments` corpora live under
`alpha/` and their standalone repos are frozen and archived)"). Moved it. Then wrote this transcript,
the lane plan, and the index registrations.]*

---

## Corrections and reversals in this session (the reason it is worth filing)

1. **Assistant, turn 2 → turn 1.** Turn 1 listed the meer's queue and its blob store together as one
   "state" bullet, obscuring that the queue is an index and CISS is the store. Corrected on the user's
   "isn't this just a queue" challenge.
2. **Assistant, turn 6 → turn 5.** Proposed the custodial grant as a separate `PolicyRecord`; revised
   to putting the declaration (kind + custodian) in the owner's manifest and only the *content* in a
   separate record. Fewer moving parts, no cross-record consistency question.
3. **Assistant, turn 8, against the user's premise.** The user's "Bob needs to request with his MLS
   identity" was answered "neither — the CISS account identity," on the grounds that presenting group
   credentials to a blind store leaks group membership.
4. **Assistant, turn 11 → turn 11.** The user's "no cursor because ordering is monotonic" was refined:
   content-addressing removes the cursor, monotonicity only makes the diff cheap. The user then clarified
   they meant a range query, which is the same object from the other side.
5. **Assistant, turn 12.** Retracted the multi-device concern raised in turn 11 — the device-Group
   (§6.6.5) is the compensating mechanism, so deliver-once is correct by design.
6. **Assistant, turn 12.** Corrected the user's belief that CISS already binds an iroh listener; it does
   not (`ciss-iroh` is client-side only).
7. **Assistant, turn 14.** Corrected its own misfiling of the spike spec into the frozen `experiments/`
   repo.

## Open items carried out of this session

- Who mints the custodian grant: per-persona enrolment versus Group-collective.
- Meter retention policy (aggregate shape and window).
- `Welcome`/`GroupInfo` versus the 2 MiB cap — the measurement is S8 in the spike.
- Custody-dial default per Group, and whether bootstrap always implies the pooled mode.
