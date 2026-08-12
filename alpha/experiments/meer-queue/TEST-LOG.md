# Meer queue spike — test log

`Spec: SPIKE-SPEC.md` · `Plan: ../../plans/2026-08-07-2-plan-meer-queue-spike.md` ·
`Discovery: PHASE-0-FINDINGS.md`

Format follows `beta/impl/delivery-layer/08-experiment-methodology.md` §5: per result, the claim
and the doc section it backs, exact resolved versions, the fidelity rung (stand-in named if Rung B),
the code, the raw output, a one-line verdict, and the design consequence.

**A bare `CONFIRMED` is inadmissible.** Every verdict states its rung.

## Resolved versions (all results below)

```
rustc 1.97.1 (8bab26f4f 2026-07-14) (Homebrew)   cargo 1.97.1
openmls =0.8.1   openmls_rust_crypto =0.5.1   openmls_basic_credential =0.5.0
openmls_traits =0.5.0   tls_codec 0.4
ciss 0.6.0 (path)   mls-replant 0.1.0 (path)   iroh =1.0.0   iroh-relay =1.0.0
axum 0.8.9   reqwest 0.13.4
```

## Registered stand-ins in force

`meer-spike-namespace`, `meer-spike-kind-gate`, `meer-spike-drain-auth`, `meer-spike-clock`,
`meer-spike-ciss-inproc` — all five in `../SPEC-DIVERGENCE-REGISTER.md`. **Nothing about the seal is
stood in;** every result below runs against real OpenMLS.

---

## M1 — An offline member drains and decrypts

**Claim.** A member offline during a message's live window recovers it from the meer and decrypts
it, with the meer never holding a key. Backs `meer-as-custodian-queue.md` §"What the meer does";
Part 2 §6.6.2.

**Rung: A (real-lib).** Real OpenMLS group + seal + `process_message`; real CISS storage boundary
over real loopback HTTP; real iroh transport over a real loopback relay.

**Code.** `tests/m1_offline_drain.rs`.

**Method note — "offline" is a real teardown.** Bob's iroh endpoint is closed for the whole send
window, and the test asserts reachability *before* (must succeed) and *after* (must fail). He
returns on the same secret key, so his `EndpointId` and therefore his queue survive the absence.

**Raw output.**
```
M1 CONFIRMED (real-lib): offline member drained 1 blob(s) and decrypted; meer group keys held = 0,
storage credentials = 1. [openmls =0.8.1, openmls_rust_crypto =0.5.1,
openmls_basic_credential =0.5.0, openmls_traits =0.5.0, tls_codec 0.4]
```

**Verdict: `M1 CONFIRMED (real-lib)`.**

**Design consequence.** The pub/sub-in, mailbox-out shape carries a real MLS conversation across a
real absence. The store-and-forward node needs no group state, no ordering, and no key — the first
direct evidence for the hypothesis doc's central claim.

**One honest refinement to the claim as stated.** The spec asked for `meer_payload_keys_held == 0`.
Reporting a bare zero would be both a tautology (the meer module cannot name an MLS type) and an
overstatement. The meer holds **exactly one credential** — its own CISS namespace key, without which
it could not write the mail anywhere. *Blind to content is not the same as credential-less.* The
verdict reports `group_keys = 0, storage_credentials = 1`.

---

## M2 — Byte-identical forwarding, and the negative case

**Claim.** The meer stores and forwards sealed bytes unchanged. Backs Part 2 §6.6.2, the `MUST` on
byte-identical storage.

**Rung: A (real-lib).**

**Code.** `tests/m2_byte_identity.rs`. Negative arm runs under `--features reframe`.

### Positive arm

**Raw output.**
```
M2 positive arm CONFIRMED (real-lib): digest 4884ed5ac3d3eb44 stable at production, after PUT,
after CISS re-verify-on-read, and at receive.
```

Note a property worth recording: **CISS's content address *is* the sha256 of the stored bytes**, so
the address it returns is directly comparable to the digest computed at production. The chain is
checked at four points and never touches the MLS layer.

**Verdict: `M2 positive arm CONFIRMED (real-lib)`.**

### Negative arm — **FALSIFIED AS SPECIFIED**

The spec hypothesised:

> a deliberately re-framed copy — decode and re-encode the `MlsMessage` without changing semantic
> content — is **detectably different** at the byte level

**It is not.** A forced decode/re-encode is **byte-identical**, for application messages (189 B) and
commits (490 B) alike, because TLS-codec serialization is canonical.

**Raw output.**
```
M2 negative arm FALSIFIED-AS-SPECIFIED (real-lib): a forced decode/re-encode is byte-identical
(4884ed5ac3d3eb44), so a re-framed copy is NOT detectably different. The MUST stands; its stated
rationale does not — the hazard is re-SEALING, which needs a key the meer lacks.

M2 structural (real-lib): re-frame unreachable in a default build — openmls gates both conversions
behind `test-utils` with an explicit MUST NOT.
```

**Verdict: `M2 negative-arm hypothesis FALSIFIED (real-lib)`; `M2's MUST upheld on stronger
grounds (real-lib)`.**

**Design consequence — the `MUST` survives, its rationale does not.** Three things hold, and
together they are a stronger result than the spec claimed:

1. **A default build cannot re-frame at all.** openmls gates
   `From<MlsMessageIn> for MlsMessageOut` and `From<PrivateMessageIn> for PrivateMessage` behind
   `test-utils`, each carrying *"break abstraction layers and MUST NOT be made available outside of
   tests"* (`framing/message_out.rs:195–211`, `framing/private_message_in.rs:263–277`). The library
   enforces the property independently of our discipline.
2. **Forced open, the re-encode is byte-preserving** — so re-framing is not a route to a
   different-but-valid copy either.
3. **The operation that would break the seal is re-sealing**, which needs a group key the meer does
   not hold (M1: `group_keys = 0`).

A blind forwarder therefore has **no route** to a semantically-equivalent-but-byte-different copy.
The spec's stated hazard (ratchet-key / nonce reuse) is real but arises from re-sealing, not from
re-framing.

### Normative-text flags (consolidated — all scenarios)

Raised here, **not** applied. Spec edits are not a spike's call; each is tracked in the backlog.

| spec text | what the measurement says | tracked |
|---|---|---|
| **Part 2 §6.6.2** — rationale for the byte-identical `MUST` | The hazard is named as re-framing. A re-frame is **byte-identical**; the hazard is **re-sealing**, which needs a key a blind forwarder lacks. Requirement stands, rationale is wrong. OpenMLS also makes re-framing unavailable in a production build, so the `MUST` is *stronger* than stated. | **E93** |
| **Part 2 §6.6.2** — "a duplicate … is dropped" | Holds for **commits**. **False for application messages** — openmls errors (`SecretReuseError`), because the per-message secret is destroyed after first use. Dedup must precede MLS processing. | **E93** |
| **Part 2 §6.4** — the leak profile | Must be grounded in the measured set, which includes **cleartext `group_id`, `epoch`, `content_type`** and the `(depositor → recipients)` graph. "Learns nothing" is false; "learns nothing about the content" is true. | **E96**, **E94** |
| **Part 2 §6.6.5** — device-group fan-out as the justification for deliver-once | Not built, therefore **not tested**. The without-arm falsifies naive deliver-once. The measured cost of the alternative (racing) makes the dependency look weaker than the framing implies. | **E92** |
| **Part 2 §5.4 / §6.6.2** — the meer's shape | **The spike tested a topology the spec does not describe.** §5.4 has the meer *observe* the fabric as a swarm node; the spike **addresses** it with an explicit recipient set. Registered late as `meer-spike-addressed-deposit`. §5.4 also gains a **third layer** the spec does not yet have: a Group-level **nomination** of the meer it relies on (MLS `GroupContextExtensions`), sitting between fabric-level presence and per-persona use — an **endorsement, not a permission**. | **E97** |
| **Part 2 §6.9** — discovery | Acknowledged in-spec as *\"treated more lightly than it will ultimately need.\"* Confirmed it does **not** cover meer discovery: §6.9 resolves an `EndpointId` to a network location, a different question. In the fabric model no meer discovery is needed at all. | E97 (closed) |
| **Part 2 §6.9.1** — broadcast tier must disable the embedded ratchet tree | **Corroborated from the storage side.** S8 reaches the same boundary independently; the mandate is well-founded, and the tree is already shipped out of band in practice. | — (no change needed) |

### A limitation stated rather than implied

The plan called for asserting the meer's blindness with `cargo tree`. **That mechanism does not
exist as specified:** `cargo tree` resolves *crates*, and the meer is a *module* in a crate that
also contains the MLS layer. The test therefore uses a source-level lint (no `openmls`,
`mls_replant`, `crate::mls`, or `MlsMessage` reference in `meer.rs`/`queue.rs`), which is a lint and
not a toolchain guarantee.

**Recommendation for the meer lane's Phase 2:** split the meer into its own crate with no openmls
dependency in its manifest. The gateway service is a separate process there anyway, so the structural
guarantee is free at that point — and it converts this lint into something the compiler enforces.

---

## S2 — Fan-out and dedup

**Claim.** The blob is stored **once** in CISS and referenced N times — `meer-as-custodian-queue.md`
§"What the meer does": *"`PUT` the blob once to CISS (content-addressed, so a message to fifty
recipients is stored once)"*.

**Rung: A (real-lib).** Real OpenMLS seal, real CISS storage boundary.

**Code.** `tests/s2_fanout_dedup.rs`.

**Raw output.**
```
S2 MEASURED (real-lib): fan-out to 5 = 1 deposit(s), 1 stored object(s), 173 sealed bytes.
Naive per-recipient = 5 deposit(s), still 1 stored object(s).
Dedup saves TRANSIT (5x), not at-rest storage.

S2 MEASURED (real-lib): identical bytes stored under 2 namespaces = 2 object(s) on disk. 
Dedup is per-namespace.
```

**Verdict: `S2 CONFIRMED (real-lib)` for the meer's store-once; `S2 FALSIFIES the unconditional
form of the dedup claim (real-lib)`.**

**Design consequence 1 — the saving is transit, not storage.** A naive per-recipient meer leaves
*the same one object on disk*, because the store is content-addressed. What five deposits actually
cost is 5x the **transit**. Since the design meters transit — and the transit meter *is* the
offline-data fraction that sizes a meer fleet — "stored once" is the less interesting half of the
claim. "Deposited once" is the half that has a price attached.

**Design consequence 2 — dedup does not cross a namespace, which bounds the claim to one custody
mode.** CISS lays objects out as `blocks/{did}/{cid}`. Identical bytes under two DIDs are **two**
stored objects (same content address, different paths). So:

| custody mode | one message to 50 recipients |
|---|---|
| meer-owned pool (**the spike's stand-in**) | 1 stored object |
| per-DID queues (**the design's stated default**) | **50 stored objects** |

The hypothesis doc states the dedup claim unconditionally in §"What the meer does", while
§"Custody is a dial" separately lists "dedup across everyone" as a *pooled-mode advantage*. Those two
passages disagree, and the measurement settles it: **dedup is per-namespace.** The unconditional
sentence needs qualifying, and the custody dial gains a cost dimension it does not currently name —
per-DID buys ownership and legible accounting, and it costs at-rest storage linear in fan-out.

---

## S3 — Dual delivery

**Claim.** Bob receives the same message twice — once carried live, once drained — and it
deduplicates to a single entry, with MLS applying it idempotently (Part 2 §6.6.2). The hypothesis
doc calls the racing story §6.6.4 *free in practice*.

**Rung: A (real-lib).** Live carriage is a real iroh connection straight to Bob's endpoint,
bypassing the meer; the second copy goes through the meer.

**Code.** `tests/s3_dual_delivery.rs`.

**Raw output.**
```
S3 MEASURED (real-lib): a duplicate application message is REJECTED by openmls 0.8.1 at the
second application: process_message refused: The requested secret was deleted to preserve
forward secrecy.

S3 CONFIRMED (real-lib): live + drained delivery of one object dedups to 1 entry on content
hash; declaring the digest suppresses re-send.
```

**Verdict: `S3 CONFIRMED (real-lib)` for the dedup shape; `S3 FALSIFIES "MLS applies it
idempotently" for application messages (real-lib)`.**

**Design consequence — dedup is required, not an optimisation.** openmls 0.8.1 does **not** apply a
duplicate application message idempotently. It **errors**: the per-message secret is deleted after
first use to preserve forward secrecy, so the second application fails with *"The requested secret
was deleted to preserve forward secrecy."*

Part 2 §6.6.2's idempotence language describes **commits** ("a duplicate commit no longer matches
current state and is dropped"), and that reasoning does not carry to application messages. The
practical consequence is sharp:

- A client that feeds both copies to `process_message` gets a **hard error on the second**, and that
  error is indistinguishable at a glance from tampering or a decryption attack.
- Therefore **content-hash dedup must happen before MLS processing**, not after and not as an
  optimisation. The racing story is free *only if you dedup first*; done in the wrong order it
  manufactures alarming errors on a completely normal delivery race.

**Scope note, stated so the result is not over-read.** S3's dedup assertion models *client*
behaviour (two identical byte strings have one digest — trivially true). The production assertions
it exercises are the queue's want-diff and prune, already mutation-verified in Phases 3 and 4. **S3's
real contribution is the measurement**, not a new assertion.

---

## S3b — Is a duplicate distinguishable from genuine loss?

**Why this exists.** S3 found that openmls errors on a duplicate rather than applying it
idempotently. That leaves two candidate designs: the client **keeps state** (a set of delivered
content hashes) and dedups before processing, or the client **treats the error as a benign
duplicate signal** — a no-op costing only bandwidth and an error branch.

The second is safe **only if** the duplicate error is distinguishable from the error a genuinely
lost message produces. Forward secrecy deletes a message key after use *and* as the ratchet advances
past it, so "I already read this" and "I can never read this" could plausibly surface identically.
If they did, treating the error as benign would silently swallow unrecoverable loss — exactly what
the no-invisible-loss rule (Part 1 §2.2) forbids and the watermark exists to prevent.

**Rung: A (real-lib).** **Code.** `tests/s3b_duplicate_vs_loss.rs`.

**Raw output.**
```
S3b MEASURED (real-lib):
  duplicate (already read)   -> process_message refused: The requested secret was deleted to
                                preserve forward secrecy.
  lost (ratchet moved past)  -> process_message refused: Generation is too old to be processed.
  DISTINGUISHABLE BY ERROR: true
```

**Verdict: `S3b MEASURED (real-lib): the two conditions are DISTINGUISHABLE`.**

They are distinct variants of `SecretTreeError` (`openmls-0.8.1/src/tree/secret_tree.rs:14–40`):

| condition | variant | meaning |
|---|---|---|
| already read | `SecretReuseError` | benign — a duplicate delivery |
| ratchet moved past | `TooDistantInThePast` | **unrecoverable loss** — this message is gone |
| too far ahead | `TooDistantInTheFuture` | out-of-order beyond the window |

**Design consequence — both designs are viable, and the choice is now informed rather than
forced.** A client may treat `SecretReuseError` as a no-op without hiding loss. Dedup-before-process
remains the tidier option (no error path at all, and it saves the decrypt attempt), but it is no
longer *required for correctness* — which was the open question S3 left.

**Two conditions on taking the error-driven route**, both concrete:

1. **Match on the variant, never on "any processing failure."** `TooDistantInThePast` sitting one
   variant away is precisely the loss the no-invisible-loss rule requires be surfaced. A catch-all
   `Err(_) => ignore` would swallow it.
2. **Preserve the typed error to the point of decision.** This spike's `MlsError::Process(String)`
   flattens the variant into a message, which is fine for a result log and **wrong for a client** —
   string-matching on an upstream error message is a silent breakage waiting for a library bump. A
   production client should carry the `SecretTreeError` variant through.

### Decision (owner, 2026-08-10): keep both, and let them compose

Not "pick one." The two mechanisms have different jobs and the second repairs the first:

- **In-memory cache of delivered content hashes** — the fast path. Dedup before processing, so
  the ordinary duplicate costs nothing: no decrypt attempt, no error branch.
- **`SecretReuseError`** — the *repair* path. It is what tells a client "already read" when the
  cache is gone.

**The cache is memory-only and deliberately not persisted.** It does not need to be, because it
rebuilds lazily through use: start empty, and each duplicate that arrives identifies itself via
`SecretReuseError`, which re-populates the entry. Nothing has to be reconstructed eagerly at
startup, and there is no on-disk structure to keep consistent, migrate, or corrupt.

**What this buys concretely — the crash window closes for free.** There is a gap between
*processed* and *acked*: a client that reads a message and dies before acking leaves the meer still
holding it. On restart the meer re-serves it, the client re-processes, and:

```
  cache empty (restarted)
        │
        ├── process duplicate ──► SecretReuseError
        │                            │
        │                            └── "already read" ──► ack ──► meer prunes
        │                                     │
        └────────────────── cache repopulated ┘
```

Without the error the client would need **persistent** delivered-hash state purely to survive that
window. With it, the window is closed by a condition MLS already reports. This is why both are kept:
the cache makes the common case free, and the error makes the cache disposable.

**Safety of acking on `SecretReuseError`.** Sound, and worth stating explicitly: the secret tree is
local per-member-per-generation state, so the variant means *this member consumed this generation*.
It cannot be induced by a third party — forging a message at that generation requires the group key.
So "I saw this error, therefore I have read it, therefore acking is correct" holds.

**A third thing this buys, unprompted:** `TooDistantInThePast` is a *detector for the gap the
watermark describes*. A client can tell "the meer swept this before I drained it" from "I already
have it" without asking the meer anything — which is the same have/want reasoning applied to the
ratchet rather than to the queue.

---

## S4 — Multi-device and deliver-once

**Claim.** From `meer-as-custodian-queue.md` §"Cursors and delivery": *"Deliver-once is correct, not
a compromise. §6.6.5 guarantees that if any one of a persona's enrolled devices receives a message,
every enrolled device eventually sees it, so the device-Group is the fan-out and the meer must not
duplicate it."*

**Two arms, two different rungs — stated up front so the result is not over-read.**

| arm | rung | why |
|---|---|---|
| without a device group | **A (real-lib)** | real group, real transport, real prune-on-ack |
| with a device group | **NOT TESTED** | §6.6.5 fan-out is not built; standing in for it would substitute for the exact mechanism the claim is about |

**Code.** `tests/s4_multi_device.rs`.

**Raw output.**
```
S4 FALSIFIED-AS-EXPECTED (real-lib, without-device-group arm): the phone received and acked;
the laptop drained its own queue and got 0 messages. Naive deliver-once starves a second
enrolled device. The compensating mechanism (§6.6.5 device-group fan-out) is NOT BUILT and is
NOT TESTED here.

S4 MEASURED (real-lib): racing across 2 enrolled devices costs 1 deposit(s), 1 stored
object(s), and 2 queue entries.
```

**Verdict: `S4 FALSIFIED (real-lib)` for naive deliver-once; `S4 with-device-group arm NOT TESTED —
Rung-A follow-up filed as ROADMAP_TODO E92`.**

### Design consequence — the second measurement matters more than the first

The starvation is what the plan predicted, so on its own it changes little. **The measurement beside
it does change something:** racing across two enrolled devices costs **one deposit, one stored
object, and two queue entries.** The blob is shared; only the queue reference is duplicated.

So the fallback the doc treats as the lesser option is **nearly free at the meer**, and the §6.6.5
dependency is buying less than the framing implies. The doc argues deliver-once is *correct rather
than a compromise* **because** the device group fans out. But if the alternative costs one extra
queue entry per device and no extra storage or transit, correctness is not what the argument is
really about.

**The honest trade, restated from the measurement:**

| | deliver-once (+ device group) | race across enrolled devices |
|---|---|---|
| meer cost | 1 queue entry, prune on 1 ack | N queue entries, prune per device |
| storage | 1 object | 1 object (**unchanged**) |
| transit | 1 deposit | 1 deposit (**unchanged**) |
| metadata to the meer | device count hidden | **device count revealed** |
| dependency | **needs §6.6.5 fan-out to exist** | none |

That is a materially better-framed dial than "deliver-once is correct." The real cost of racing is
**metadata** — the meer learns how many devices a persona has, which the blindness posture exists to
minimise — plus longer retention, since entries persist until every device acks. The real cost of
deliver-once is a dependency on a mechanism that is not built.

**Neither is obviously right, and the spike does not resolve it** — that is E92's job, once §6.6.5
exists and the with-arm can be run at Rung A.

### The with-device-group arm, reasoned (NOT measured)

Recorded as reasoning, explicitly not evidence. For deliver-once to be safe, the meer must **detect**
whether a device group is present — the doc calls this "a detectable condition, not a preference."
Two things that detection would have to be true of, both visible from this arm's failure:

1. **It must be observable to the meer without reading the seal.** The meer cannot inspect group
   membership — that is the blindness. So presence has to be asserted out of band, at enrolment,
   which makes it a property of the *custodial grant* rather than of the message.
2. **It must fail closed.** If presence is asserted but the fan-out silently stops working, the
   result is exactly this test: a device that never learns it is missing anything. Nothing in the
   drain path would surface it, because a starved device's queue is *legitimately* empty — it looks
   identical to having nothing waiting. **A starving device and an idle device are indistinguishable
   at the meer**, which is why the dial cannot be left to a default.

---

## Standing finding — the meer's leak profile, measured rather than assumed

**Framing (owner, 2026-08-10):** using a meer is a choice, and *nothing can queue for delivery to an
absent recipient without learning the shape of that task.* That is inherent to the function, not a
defect. **The obligation is to be clear about it, not to run from it** — which means an enumerated
profile, in a form the subject can read (the metadata-transparency guard,
`meer-superpeer-design.md` item 6), rather than a minimised one.

The scenarios so far have been accumulating this incidentally. Collected here; S7 (Phase 10)
completes the carrier's column and this becomes §6.4's grounding.

| the meer observes | necessarily? | evidence |
|---|---|---|
| that mail exists for a recipient | **yes** — it is the queue | S2, S4 |
| deposit time | **yes** — it is the retention clock | Phase 3 (`deposit_days`) |
| object size | **yes** — it stores the bytes | S2 (173 B sealed) |
| fan-out width (recipient count) | **yes** — one deposit names N | S2 |
| drain time (when you came back) | **yes** — it serves the drain | M1 |
| **depositor → recipient edges** | **NO — see below** | S2, S4 |
| device count per persona | only when racing | S4 |
| message content | **never** — sealed | M1, **S7** |
| ~~which group a message belongs to~~ **which group a message belongs to** | ~~never~~ **YES — `group_id` is cleartext in the MLS framing** | **S7 — this row was assumed and is now FALSIFIED** |
| epoch (and therefore that membership changed) | **yes** — cleartext | **S7** |
| content type (application vs handshake) | **yes** — cleartext | **S7** |
| ordering / causality | **never** — the per-author index is inside the seal | design |

### The one that is not inherent, and is not in the doc: the communication graph

A deposit names its recipients explicitly, and the depositor is identified by the authenticated
connection. **So the meer learns `(depositor → recipients)` edges — the communication graph.**

That is worth separating from the rest of the table because:

1. **It is arguably more sensitive than what the design carefully protects.** The doc reasons at
   length that drain must authorize on *account* identity and never MLS identity, because
   "presenting group credentials to a blind store would tell it which groups you are in — metadata
   the blindness exists to prevent." The deposit side hands over who-talks-to-whom directly, which
   is a superset of that concern for most threat models.
2. **It is not enumerated anywhere.** §"Cursors and delivery" covers the drain side; deposit gets one
   clause — *"deposit is gated at the meer's admission policy"* — about authorization, not about
   what admission reveals.
3. **Unlike the rest of the table, it is not forced by the function.** The meer must know *who to
   queue for*. It does **not** need to know *who deposited*. The depositor identity is currently
   supplied by the transport (`meer-spike-drain-auth`'s `EndpointId` comes free off the QUIC
   connection) rather than by necessity — so an unlinkable deposit is a design option, not a
   contradiction.

**Consequence for S4's dial.** This shrinks the marginal cost I attributed to racing. If the meer
already holds the graph, deposit times, sizes, and fan-out width, then *device count* is one more
row in a table that is already substantial — not a categorical change in what it knows. That weakens
"racing leaks device count" as an argument against racing, and correspondingly weakens the case for
depending on §6.6.5. Noted against **E92**.

**Not resolved here.** Whether deposit should be unlinkable is a design question with real costs
(admission policy, abuse control, and the metering story all currently lean on knowing the
depositor). Flagged, not decided.

---

## S5 — Expiry and the watermark

**Claim.** Retention is *"14 days as a ceiling, not a floor — 14 days **or until drained**"*, and
past the window the recipient gets a watermark: *"a loud, visible, SSH-host-key-shaped 'here is what
is gone'"* (`meer-as-custodian-queue.md` §"Cursors and delivery"; Part 1 §2.2 no-invisible-loss).

**Learning goal (spike spec).** Whether a loud, visible gap is **constructible from what the meer
retains**, or needs more state than a watermark.

**Rung: A (real-lib)** for the storage boundary; time is SPEC-DELTA[meer-spike-clock].

**Code.** `tests/s5_expiry_watermark.rs`. **Method note:** the watermark is deliberately minimal —
a count and a day range, **no digests** — so the test cannot flatter its own answer by retaining
enough state to guarantee it.

**Verdict: `S5 CONFIRMED (real-lib)` for the gap being constructible; `S5 FALSIFIES "it is gone"
as a storage claim (real-lib)`.**

### The gap is constructible, and bounded

The minimal watermark renders:

> *"You were away. 3 message(s) arrived between day 0 and day 2 and are no longer available from
> this meer."*

That satisfies no-invisible-loss: loud, counted, time-bounded. Also confirmed at the boundary —
an entry **at** day 14 is still served, at day 15 it is swept (the ceiling means served *for* its
fourteenth day) — and confirmed that **drained mail leaves no watermark**, since a gap marker for
successfully delivered mail would be a false alarm, the opposite of the rule's intent.

**What it does not support is recovery.** With no digests retained, a client can say *how much* it
missed but cannot **name** it to a peer — so D-peer corroboration cannot be pointed at the gap.
Retaining digests would enable that, and would leave a **per-recipient content-address log
outliving the mail it describes**: the same shape as the concern the design already raises about
meter retention ("the meer's most sensitive artifact being the one thing that outlives
everything"). The trade is real and is not resolved here.

### The falsification: "gone" is a serving claim, not a storage claim

```
S5 MEASURED (real-lib): after sweep, queue serves 0 entries but CISS still holds 1 object(s).
```

**CISS's object plane is `PUT`/`GET` with no `DELETE`** (`src/server.rs` routes: objects, manifest,
assertion, receipt-countersign, meter, du — no delete on any). The meer therefore has **no
mechanism to remove what it stored**. Sweeping ends *service*; the bytes remain in the meer's
namespace indefinitely.

Three consequences, none of which the design currently states:

1. **The 14-day promise is about serving, not holding.** "Here is what is gone" is false as
   written; "we stopped serving it" is true. The user-facing wording should not claim deletion the
   substrate cannot perform.
2. **Storage grows monotonically.** Every message ever deposited stays. The design's framing of a
   queue as *"high write rate / tiny objects / 14-day churn / no backup"* assumes a churn that does
   not happen — the deployment sizing follows from a false premise.
3. **Ciphertext outlives its window.** Sealed, so not readable — but it is a harvest-now-decrypt-
   later surface and a durable metadata surface (sizes, timing, counts, and the graph from E94)
   that the retention window was supposed to bound.

**What would close it — owner's call 2026-08-10 is to build it.** Two designs, and they are
complementary rather than alternatives (detail in **E95**):

- **A — manifest-driven reclamation.** No DELETE endpoint is needed, because the manifest already
  *is* the owner's signed statement of what should exist: it binds every leaf (B1) and carries a
  monotonic `seq` refused on rollback (B3, `CISS/src/server.rs:1299`). Signing manifest `N+1`
  without a leaf is an authenticated, replay-proof "I no longer claim this"; the server reclaims
  what no manifest references. This is **independently owed by the PDS-compat claim** — CISS ships
  `uploadBlob`/`getBlob`/`listBlobs` with no record surface, so nothing can ever *become*
  unreferenced, whereas atproto collects blobs once their referencing records are deleted.
  **Limitation:** it needs the owner online, which the meer case cannot assume.
- **B — owner-declared retention on the `queue` chain kind.** The planned slot declaration already
  carries *kind + custodian + ceiling*; add a retention window. The owner pre-authorises expiry
  **once, at enrolment**, and the server enforces it with no per-delete signature and no owner
  presence. The policy lives in the owner's signed, genesis-fixed declaration, so it inherits B3
  anti-rollback for free. **It also sharpens the kind itself:** `queue` becomes *the kind whose
  contents expire by owner-declared policy*, which separates it from `file-sync` on a real axis
  instead of an authorization footnote.

---

## S6 — Revocation and re-point

**Claim.** *"There is nothing to port because it never left home."* The design's strongest story.

**Rung: A (real-lib)** for the mechanism, **bounded by SPEC-DELTA[meer-spike-namespace]**.

**Code.** `tests/s6_repoint.rs`. Two independent meers over two independent CISS instances.

**Raw output.**
```
S6 CONFIRMED-WITH-STAND-IN (real-lib): re-pointing lost no mail and migrated nothing — each meer
served only its own era, and Bob's queue address survived the move because it is his identity.
```

**Verdict: `S6 CONFIRMED-WITH-STAND-IN (real-lib)` — the mechanism holds; the claim that makes it
interesting is UNTESTED.**

Bob keeps one identity across the move (the secret key *is* the queue address), era-1 mail stays
readable at the old meer, era-2 arrives at the new one, both decrypt against the same group, and
neither meer is asked to hand anything over.

**The limit, stated because the verdict would otherwise overstate itself.** Under the namespace
stand-in the mail sits in each **meer's** CISS namespace, not Bob's. So *"nothing to migrate"* holds
here because the two meers are **independent**, not because Bob **owned** the bytes. The design's
actual claim — mail never left home, and the meer held only a revocable grant to write into Bob's
own namespace — requires custodian mode and **cannot be tested in this spike**. A stronger S6
belongs in meer lane Phase 1.

This is the scenario the plan flagged as most likely to pass for the wrong reason, and it did.

---

## S7 — Carol carries and learns nothing

**Claim.** A node that handles the sealed bytes but is not in the group cannot decrypt them.
Learning goal: *state the real observed metadata set rather than the assumed one, so §6.4's leak
profile is grounded in a measurement.*

**Rung: A (real-lib).** **Code.** `tests/s7_carol_carries.rs`.

**Raw output.**
```
S7 MEASURED (real-lib) — what a carrier observes with NO key:
  byte length     : 169
  sha256          : 3a098701d7d675eb...
  wire format     : PrivateMessage
  group_id        : 818801a8fea6b21533c84b2f57d0e6af   <-- CLEARTEXT
  epoch           : 1                                  <-- CLEARTEXT
  content_type    : Application                        <-- CLEARTEXT
  plaintext       : NOT AVAILABLE — Message group ID differs from the group's group ID.
```

**Verdict: `S7 CONFIRMED (real-lib)` for confidentiality; `S7 FALSIFIES "learns nothing"
(real-lib)`.**

### The content is safe. "Learns nothing" is not true.

`group_id`, `epoch` and `content_type` sit **beside** the ciphertext in the MLS `PrivateMessage`
framing, not inside it (`openmls-0.8.1/src/framing/private_message.rs:33–38`), and are readable
through the public API with **no key at all**. This is RFC 9420 framing, not a CISS or meer choice.

**And the refusal is a routing check, not cryptography.** Carol is turned away with *"Message group
ID differs from the group's group ID"* — the library declines **before attempting decryption**,
because the cleartext group_id does not match her state. That is a different security story from
"tried and failed": the confidentiality guarantee is real, but nothing about this refusal exercises
it.

### The consequence for the design's stated reasoning

`meer-as-custodian-queue.md` §Reasoning argues:

> **Why drain authorizes on account identity, never MLS identity.** Presenting group credentials to
> a blind store would tell it which groups you are in — metadata the blindness exists to prevent.

**That mitigation is defeated by the payload it protects.** Refusing MLS identity at the drain gate
does not stop the meer learning which groups you are in — `group_id` is in every message it stores.
Measured: two messages to one group are **linkable by a carrier** with no key.

So a meer can, without breaking any seal:

- partition its store **by conversation**;
- count **per-conversation** traffic per recipient;
- watch **epoch advancement**, which signals commits — i.e. that membership or keys changed.

The account-identity drain gate is still right (it avoids *adding* a second, credential-based
disclosure), but it should not be presented as preventing group-linkability. It does not.

**The corpus already owns the fix, and has not applied it here.** The history-convergence store is
specified with **nested double-sealing**; the meer is not. An outer seal over the MLS message would
close this, at the cost the history store already pays. **Filed as E96.**

---

## S8 — Object sizes against the 2 MiB cap

Full table, extrapolations, and the design decision in **`S8-RESULTS.md`** (kept separate because
the measurement table would crowd this log). Summary:

**Verdict: `S8 MEASURED (real-lib)`. `S8 FALSIFIES "commit ~log N" (real-lib)`. The pre-registered
catastrophic branch is HALF closed.**

- **Application messages are flat at 181 bytes**, N = 2 → 8000. The object the meer carries in
  steady state never approaches the cap.
- **All three commit types are linear**, not logarithmic — 82 B/member (self-update, remove),
  282 B/member (add-all). The spec's "~log N" row is wrong.
- **Crossover order:** `Welcome`-with-tree ≈ 6 350 → add-all commit ≈ 7 440 → `GroupInfo` ≈ 11 780 →
  `Welcome`-without-tree ≈ 13 790 → ordinary commits ≈ 25 500.
- **Half the catastrophic branch fires.** Application messages never cross, so CISS needs no
  streaming rewrite for conversational groups. **Ordinary commits do cross**, at ≈ 25 500 — so the
  concern is real at broadcast scale, the tier §6.9.1 already treats separately.
- **Shipping the ratchet tree out of band is already the corpus's de-facto behaviour** and buys ~2×
  headroom. It was arrived at incidentally, not decided, and is currently undocumented.

**Best-case caveat, stated because the numbers invite over-reading:** one ciphersuite,
`BasicCredential` only. Real credentials are larger per leaf, so **every crossover moves down**. The
*shape* is the finding; the figures are an upper bound.

---

## S9 — The queue name as a capability (2026-08-12, post-reshape)

**Why this exists.** The fabric reshape (Part 2 §5.4) raised a question the addressed model never
had to answer: **what entitles you to drain a group's queue?** Membership is the natural answer, but
the meer is blind and cannot evaluate MLS membership.

**The proposal:** derive the queue name from the group's exporter secret
(`export_secret(label = "croft/meer-queue/v1")`). Every member derives the same name; a non-member
cannot; the meer, holding no key, cannot either — so **it needs nothing in advance**.

**Rung: A (real-lib).** **Code:** `tests/s9_queue_name_capability.rs`.

**Verdict: `S9 CONFIRMED (real-lib)` — the scheme works, including the part that could have
deadlocked.**

| result | measured |
|---|---|
| members agree on the name | yes; a foreign group derives a different one |
| rotates per epoch | yes — so rotation needs no coordination with the meer |
| catch-up across N missed epochs | **N serial round trips, in order — does not deadlock** |
| hides the group from the meer | **no** — see the limit below |

### The finding that matters: catch-up chains, and why it does not deadlock

The name is epoch-bound, so a member offline across N commits **cannot name the newest queue**. The
scheme works only because of an ordering property worth confirming rather than assuming: **the commit
closing epoch E is sent *during* epoch E**, so it lands in the queue the returning member can already
name. Catch-up is: drain E → apply → derive E+1 → drain E+1 → …

**Measured: 5 missed epochs = 5 serial round trips.** Verified by mutation — filing commits under the
epoch they *open* rather than *close* breaks the chain on the first hop, which is exactly the
deadlock the design would have had if the ordering ran the other way.

**Cost:** a member returning after N commits pays **N sequential fetches** before it can read
anything newer. That ties epoch churn directly to return latency, and connects to S8's O(N) commits
and §11.11's liveness-window tuning: **a group that rotates keys aggressively makes returns slower.**

### The limit: access control, not privacy

The queue name is **not** the `group_id`. The name rotates and is unguessable; `group_id` is
**cleartext in the envelope** (S7) and stable for the group's life, so the meer can still link across
epochs via the payload. **The opaque name buys access control and nothing else** — recorded because
the opacity invites the stronger reading.

### The unlooked-for consequence: this unblocks E96

Nested sealing was blocked by an unstated dependency — an outer seal blinds the meer completely,
leaving nothing to route on. **The queue name gives the meer a routing handle that is not the MLS
framing**, so a nested-sealed envelope stays deliverable. E96 becomes "add an outer seal *once the
queue name carries routing*."

### Decisions recorded alongside

- **Routing metadata carries the name in the clear.** The meer cannot compute it, so the sender
  attaches it; §5.4 already anticipates the slot. Consequence: visible to every swarm participant —
  acceptable (the topic is Group-derived) but stated.
- **`EndpointId` is for rate limiting, never authorization.** Authorizing on it would let the meer
  build a device→groups map across every queue it serves — re-introducing the correlation the
  capability design avoids. Abuse control needs a handle to count against, not an identity.

---

## S1 — Enrollment: what does pointing a meer at your queue actually require?

**Rung: C (static).** Inspection and enumeration, **not** a run. Custodian mode does not exist, so
there is nothing to execute; writing code that *simulated* enrollment would produce a green test
proving nothing and would retire a question that is still open. Recorded as an inspection and
labelled as one.

**Learning goal.** Whether the *"one line in your inventory"* story survives contact, and whether
enrollment needs anything the hypothesis doc does not mention.

### The sequence, walked

| # | step | state it implies | in the doc? |
|---|---|---|---|
| 1 | Bob has a CISS namespace | an account — already true for a Croft user | yes |
| 2 | Bob learns the meer's identity | which identity? CISS account (for the custodian field) **and** iroh `EndpointId` (to reach it) — **two** identifiers, bound together by nothing | partly |
| 3 | Bob declares a queue slot | `kind = queue`, `custodian = <meer>`, owner-declared `ceiling`, and (per E95-B) `retention` | yes |
| 4 | Bob signs the manifest at `seq + 1` | one signed write, inheriting B3 anti-rollback | yes |
| 5 | The meer learns it has a grant | push or pull? unspecified — the meer must discover it can write | **no** |
| 6 | **Senders learn where to deposit for Bob** | an announcement Alice can resolve | **NO — see below** |
| 7 | Device-group presence is asserted | required by S4, and only assertable here | **no** |

### Finding 1 — enrollment is not two-party, and the missing party is the interesting one

The doc frames enrollment as Bob ↔ meer: *"a one-line, revocable permission to add to one pigeonhole
on your own plot."* That is accurate for the **grant**. It is not the whole enrollment, because
**Alice has to know which meer to deposit at**, and nothing in the design says how she finds out.

In this spike the question was invisible: the test handed Alice the meer's address directly. In
reality that is a **discovery problem** with at least three candidate homes — a service endpoint in
Bob's DID document (atproto-native, and the natural fit), a KeyPackage extension inside MLS, or out
of band. Each has different revocation and privacy properties. **The doc addresses none of them.**

### Finding 2 — revocation is two-sided, and only one side is designed

Revocation is described as *"clearing the custodian field and bumping `seq`"* — an ordinary owner
write inheriting B3 for free. That correctly stops **the meer writing**. It does nothing about
**senders still depositing**, because they hold the old announcement.

So a revoked meer keeps receiving deposits it can no longer act on, and Alice's messages go nowhere
until she re-resolves. The window is however long announcement propagation takes — which is
unspecified, because announcement is unspecified. **S6 tested re-pointing; it did not and could not
test revocation propagation**, because there is no announcement channel to propagate through.

### Finding 3 — "one line" is true of the grant and understates the enrollment

Counting what Bob must hold or publish: a queue slot declaration (four fields), the meer's two
identifiers, an announcement senders can resolve, and a device-group assertion. The *grant* is one
line. The **enrollment** is a small record plus a published pointer, and the published pointer is the
part with the interesting failure modes.

This does not damage the user-facing story — *"a revocable permission to add to one pigeonhole"*
remains true and is the part a user would care about. It does mean the **implementation** has an
unbuilt component the design has not named. **Filed as E97.**

---

## Stand-in register correspondence (Phase 12 check)

`grep -rn "SPEC-DELTA\[" src tests Cargo.toml` yields exactly five tagged ids —
`meer-spike-ciss-inproc`, `meer-spike-clock`, `meer-spike-drain-auth`, `meer-spike-kind-gate`,
`meer-spike-namespace` — and `../SPEC-DIVERGENCE-REGISTER.md` carries exactly those five rows.
**Every tag has a row; every row has a tag.** (One further mention, `SPEC-DELTA[...]` in
`src/lib.rs`, is the convention being referenced in prose, not a tagged site.)

Not yet run. S7 Phase 10; S8 Phase 11; S1 (enrollment,
Rung C static) Phase 12.
