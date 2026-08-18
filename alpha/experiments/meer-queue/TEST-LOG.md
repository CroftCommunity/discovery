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

## S10 — What catch-up actually costs (2026-08-12)

S9 established that a returning member walks the epoch chain. This measures whether that is
**expensive**, over real CISS and real iroh, and whether "skip ahead" is an alternative.

**Rung: A (real-lib).** **Code:** `tests/s10_catchup_cost.rs`.

```
S10 MEASURED (real-lib): catch-up across 10 missed epochs over real CISS + real iroh
= 124.085083ms total, 12.408508ms per hop, 10 message(s) recovered.

S10 MEASURED (real-lib): skipping ahead is REFUSED outright — a member cannot apply a commit
whose predecessors it has not seen: Message epoch differs from the group's epoch.
```

**Verdict: `S10 MEASURED (real-lib)` — the walk is cheap, and the alternative is not an
alternative.**

### 1. The walk costs ~12 ms per hop, and N is small

**~124 ms for ten missed epochs**, end to end, through the real storage boundary and a real relay.
Pipelined over one connection, with no user interaction.

And **N is bounded twice over**:

- **N counts governance events, not messages.** Measured: **50 application messages leave the epoch
  unchanged**; only a commit advances it. Earlier framing here said "a group that rotates keys
  aggressively makes returns slower" and implied chat drove it. **That was wrong** — chat volume is
  irrelevant to N.
- **The retention window caps it absolutely.** Past the window there is nothing to walk, so a member
  back after six months pays the *same* walk as one back after two weeks (owner, 2026-08-12).

So for a stable group the walk is a handful of hops at ~12 ms each. **The concern this test was
written to investigate does not survive it.**

### 2. "Skip ahead" is a category error, not a cheaper option

A member cannot apply a commit whose predecessors it has not seen — openmls refuses with *"Message
epoch differs from the group's epoch."* And even if it could, the missed plaintexts would remain
unreadable, because their epoch secrets were never derived.

**The walk and the skip are not two strategies for one goal.** The walk catches up **and delivers
what was missed**; skipping abandons it. Comparing their costs compares a delivery mechanism against
a non-delivery mechanism.

### 3. A delivery-semantics rule this test learned the hard way

**Dispatch on the cleartext `content_type` *before* processing. Never try-decrypt-then-fall-back.**

`process_message` **consumes the message key**, so a client that attempts an application decrypt and
falls back to commit handling destroys its own group state — the second call hits
`SecretReuseError`. This test was written that way first and failed exactly so.

S7 measured that `content_type` is readable with no key. **This is what that is for**: routing on the
cleartext type rather than on a failed decrypt. It is a small rule with a sharp edge, and it belongs
in the client contract rather than being rediscovered by whoever writes the next client.

### Implementation landed alongside

`OP_DRAIN_QUEUE` — **drain by name**, the S9 capability model. Possessing the name *is* the
entitlement. This supersedes `OP_DRAIN`'s `EndpointId` scoping (`meer-spike-drain-auth`), which
identified the *device* and would have let the meer build a device→groups map across every queue it
serves. `EndpointId` remains for **rate limiting**, which needs a handle to count against, not an
identity.

---

## S11 — Is a KeyPackage a one-time write token? (2026-08-12)

**Why.** The personal inbox must accept a deposit from a **stranger**, so its write side is open by
necessity — and open writes into someone's own namespace mean **spam costs the victim rent**. The
proposal: make consuming a published KeyPackage the write capability, bounding invitations by a
supply the owner controls.

**Rung: A (real-lib).** **Code:** `tests/s11_keypackage_write_token.rs`.

**Verdict: `S11 REFUTED (real-lib)` — the KeyPackage fails as a write token.**

| measured | result |
|---|---|
| one package seats the owner at most once | **yes** — Alice joined the first group, not the second; the private half is consumed on join |
| only a legitimate user can produce a `Welcome` against it | **no** — two independent parties each built a valid one from the same package |
| a stranger can seat the owner in an unasked-for group | **yes** — MLS working as specified |

**Why it inverts.** The single-use property is real but sits on the **recipient's** side. Anyone who
can *read* a published KeyPackage can build a valid `Welcome` against it, because a KeyPackage is
public key material and inviting a stranger is what it is for. So "mark it spent on deposit" lets a
passer-by **burn the owner's entire published supply and deny legitimate invitations.** The bound
lands on the wrong party: it limits the owner's **reachability**, not the attacker's **effort**.

**What survives.** An unwanted invitation is **not cryptographically preventable**, so the write gate
can only bound *volume* and make it *attributable*: an authenticated depositor DID (any DID, but
verified) plus an owner-declared ceiling. Neither is a capability — the inbox can be bounded and
accountable, not unsolicited-free. Same posture email reached, for the same reason.

**Read authorization, by contrast, needs no new work:** `read_class: owner` on CISS's shipped gated
reads (v0.4.0, Z4–Z8) means a public address yields a write target and nothing else.

---

## S12 — The personal inbox, walked out (2026-08-12)

The group half of the two-target design was Rung A end to end; **the personal half was design only.**
This closes that asymmetry. **Rung: A (real-lib)** — real OpenMLS, real CISS over loopback HTTP,
real Model-A self-signed assertions. **Code:** `tests/s12_personal_inbox.rs`.

| question | verdict |
|---|---|
| is the inbox necessary? | **yes** — a queue name derives only from group state |
| does `read_class: owner` hold? | **yes** — owner 200, authenticated stranger 404, anonymous 404 |
| is custodial write the gap? | **yes** — a stranger's deposit is refused **403** |
| does the stranger handshake work? | **yes, end to end**, with only the deposit stood in |

### 1. The inbox is necessary, not merely convenient

A queue name derives **only from group state**. Holding the owner's public KeyPackage — everything a
stranger can legitimately obtain — yields nothing. So a stranger has **no group-queue path at all**,
and first contact must land somewhere else.

### 2. Read gating holds, and the default does not

With `read_class: owner`: owner `200`, authenticated stranger `404`, anonymous `404`.

**Mutation-verified, and the mutation is the interesting part:** skip the policy write and the
stranger reads with **`200`** — the world-readable PDS-compat default. So the gate is doing real
work, and **an inbox that forgets to set it is world-readable.** That makes the policy write part of
provisioning, not an optional hardening step.

**This answers the harvest-now-decrypt-later concern completely:** the ciphertext is never
obtainable, so there is nothing to hold against a future break.

### 3. Custodial write is the one genuine blocker — measured, not cited

A stranger's deposit into the owner's namespace is refused **HTTP 403** (Z2; delegated write is
`[PLANNED]`, not v1). Registered as `meer-spike-owner-write-standin`. **Everything else in the
handshake already works today.**

### 4. The full stranger handshake, end to end

KeyPackage published to the owner's namespace → fetched by a stranger → **`validate()`d** (the real
receiver path; the bare `From<KeyPackageIn>` conversion is `test-utils`-gated precisely because it
would skip that) → group created → `Welcome` deposited and retrieved **byte-identically** → joined →
**both parties then derive the same group queue name.**

**The handover happens exactly once, at first contact.** After it, the inbox is idle and the group
queue carries everything — which is why the inbox can be low-volume, per-DID, and expensive-per-item
without that costing anything at scale.

---

## S13 — The two interactions (2026-08-12)

Every piece of the two-target design was measured in isolation. These are the two places the pieces
**meet**. **Rung: A (real-lib).** **Code:** `tests/s13_interactions.rs`.

### 1. The handover lands exactly where MLS's privacy boundary is

A joiner's first derivable queue is **the epoch her `Welcome` seated her in** — not the group's
first, and not its current one. Measured: `alice_q == q1`, `alice_q != q0`.

**The finding is why that is safe without any extra rule.** Messages sent before Alice joined are
undecryptable to her — standard MLS. But she also **cannot name their queue**, because the name
derives from an epoch secret she never held. So she never *requests* them.

> **The MLS privacy boundary and the queue-addressing boundary are the same boundary.**

That is not a coincidence to be relied on quietly — it is the reason no separate access rule is
needed for history-before-join, and it should be stated in the design rather than rediscovered.

### 2. A swept queue and an empty queue are indistinguishable — except for the watermark

Measured: both return an **identical empty drain**. Only the watermark separates them (2 swept
entries vs none), and mutation-verified — suppress the watermark and loss becomes invisible.

**This is the S4 failure mode reappearing at the retention boundary.** There, a starving device
looked like an idle one; here, a member who lost mail looks like one who is caught up.

> **Client contract: a client MUST consult the watermark before concluding it is caught up. An empty
> drain alone is evidence of nothing.**

### 3. A sweep mid-walk strands a member — and the walk's exposure is N hops long

The nastier form: the member takes hop 1 successfully, earns the right to name hop 2, and the sweep
lands before she takes it. Measured: hop 2 is empty and **carries a watermark on that exact queue**,
so she is demonstrably short rather than silently caught up.

**Corrected framing (2026-08-12, after checking the corpus).** I first wrote this up as a *race* —
the member racing the sweeper during the walk. That is the wrong shape and understates it in one way
while overstating it in another.

**The sharper statement:** the chain is walked oldest-first, and the oldest queue is the one closest
to expiry. If **any** link expires, every queue after it becomes **unnameable** — so the loss is not
proportional to what expired, it is **total from the break forward**, including messages from
minutes ago. A 14-day window does not mean "you lose messages older than 14 days"; it means "if you
are away past the window you may lose *everything*, because the chain is severed at the far end."

**But this is not a new problem, and the corpus already has the mechanism.** Part 2 **§11.6**
(hot/cold Groups, liveness-driven migration) defines exactly this boundary: **liveness is having
processed an epoch within the liveness window** — explicitly *"processing epochs, not authoring
messages"*, so a silent reader who syncs stays live. A client that misses the window is **migrated to
cold**, which is a removal from the hot Group, batched into one commit. **§11.7** then defines
re-entry: *"how a cold persona returns at its own cost."*

So "I lost the epoch thread and need readmission" is the **designed** outcome, not a failure mode
this spike discovered. And §11.6 already sets the windows, tightening with group size:

| band | modest | aggressive |
|---|---|---|
| 250–1k | 90 days | 45 days |
| 1–3k | 60 days | 30 days |
| 3–7k | 45 days | 21 days |
| 7–10k | 30 days | 14 days |

### What IS new: the two windows must not disagree

The meer's **retention window** and the Group's **liveness window** are different knobs that decide
the same thing, and nothing currently constrains them to agree. If retention (say 14 days) is
**shorter** than liveness (say 30 days), a member gone 20 days lands in a **limbo state**: still a
live member of the hot Group, but unable to catch up from the meer, and not yet migrated to cold — so
the designed recovery path (§11.7 re-entry) is not open to them either.

**The constraint that removes the limbo:** *meer retention **≥** the Group's liveness window.* Then
"I cannot catch up" and "I have been migrated to cold" coincide, and there is exactly **one**
recovery path rather than a gap between two.

**Consequence for E95:** the meer's retention is therefore not a free parameter — it is bounded below
by a Group-governance policy that varies by group size (14–90 days). A fixed 14-day constant would be
correct only for the 7–10k aggressive band and wrong everywhere else. **This should be a declared
per-Group value, not a service default** — which fits E95's declared-expiry axis well, and argues the
axis belongs to the *Group* rather than the service.

**Owner's position (2026-08-12):** falling behind and needing readmission is *"a per-user choice and
consequence"* — the risk a member accepts by being away. That is §11.6's live-experience versus
return-experience trade, seen from the member's side rather than the community's.

---

## S14 — Does the delivery design match §11.6 / §11.7 as written? (2026-08-12)

The corpus sketched the absence boundary far deeper than this spike had been treating it. **S13's
"open design question" was largely already answered**; this walks the specified paths to find where
recent thinking and historical planning actually agree.

**Rung: A (real-lib).** **Code:** `tests/s14_liveness_and_reentry.rs`.

**Verdict: three confirmations and one real gap.** The delivery design fits §11.6/§11.7 better than
expected — in two places the spec's properties fall out of the queue-naming scheme for free.

### 1. The queue name IS a liveness indicator

§11.6: liveness is *"processing epochs, not authoring messages"* — a silent reader who syncs stays
hot. Measured: a member who **never authors** but processes every epoch stays exactly current and
derives the **same queue name** as the author.

> **A client that can still derive the current queue name is, by definition, live.**

So the delivery layer needs no separate liveness signal. The thing §11.6 measures is the same thing
the queue name already encodes.

### 2. Migration to cold severs queue access with no mechanism at all

Measured: after a removal, the group's name moves on and the cold member cannot derive it.

**Access control for cold members is a *consequence* of the naming scheme, not a feature.** Nothing
in the meer enforces it; nothing can forget to. This is the same property S13 found for
history-before-join — the privacy boundary and the addressing boundary coincide — appearing again at
the other end of the membership lifecycle.

### 3. §11.7's self-service re-entry works, at Rung A

Measured: a cold member **rejoined by external commit** from a current `GroupInfo` — **no `Welcome`,
no active member's help** — left at epoch 1, re-entered at epoch 7.

This confirms §11.7's central claim, including its negative half: *"a pre-published KeyPackage does
not enable self-service return… the returner cannot produce their own Welcome."* The library
supports the specified path, and the cost falls on the returner as designed.

**Correction to this spike's own design doc:** `meer-two-target-delivery.md` says the personal inbox
carries `Welcome` because it is *"the sole object in MLS addressed to a person."* True — but it
implied **all** re-entry flows through the inbox. It does not: **first contact** needs the inbox,
while **re-entry by a former member** is self-service and needs only a `GroupInfo`. Two different
paths, and only one of them needs third-party deposit.

### 4. The gap: retention is below almost every liveness window

Measured against §11.6's schedule, with the spike's current `RETENTION_DAYS = 14`:

```
S14 MEASURED: meer retention is 14 days. §11.6 liveness windows it is SHORTER than — i.e. bands
where a member can be live-but-uncatchable — are: 250–1k/modest (90d), 250–1k/aggressive (45d),
1–3k/modest (60d), 1–3k/aggressive (30d), 3–7k/modest (45d), 3–7k/aggressive (21d),
7–10k/modest (30d).
```

**Seven of eight bands.** In each, a member absent longer than retention but shorter than the
liveness window is **live in the hot Group, unable to catch up from the meer, and not yet migrated to
cold** — so §11.7's re-entry is not open to them either. **Neither mechanism applies.**

> **SUPERSEDED IN PART by S15 (2026-08-13).** The last clause is wrong: §11.7's re-entry **is** open
> to a stranded-but-live member, because openmls does not distinguish "cold" from "stranded". What
> is actually missing is the **`GroupInfo`** that path needs, which nothing serves (E105). The
> ordering constraint below stands unchanged; only "neither mechanism applies" was too strong.

**The fix is ordering, not code: `meer retention ≥ the Group's liveness window`.** Then "cannot catch
up" and "migrated to cold" coincide, and there is exactly one recovery path. Since §11.6's windows
are per-band governance policy (14–90 days), **retention is not a service constant** — it is bounded
below by a Group decision, which is where E95's declared-expiry axis should live.

The check is written as an executable assertion so a later change to either default trips it.

---

## S15 — The limbo state, walked end to end (2026-08-13)

S14 asserted limbo as a **policy comparison** between two constants. Nobody had put a member in that
state against the real library and asked what she can actually do.

**Rung: A (real-lib).** **Code:** `tests/s15_limbo_walked.rs`.

**Verdict: the state is real and reachable — and S14's characterisation of it was too strong.**

### 1. What limbo looks like from inside

Measured, at 15 days absent (past the 14-day retention, inside a 30-day liveness window), the member
is in **all three states at once**:

- **still seated in the hot Group** — the leaf is there, the group is still exactly two members, so
  §11.6's migration to cold has not run;
- **holding a watermark of 2 lost entries** — she knows she is short, which is S13's finding doing
  its job at exactly the moment it matters;
- **able to name exactly ONE queue**, the stale one — because the commit that would have named the
  next was among the entries that were swept.

### 2. **Correction to S14: limbo is escapable.** "Neither mechanism applies" is too strong

S14 concluded that a stranded-but-live member has neither catch-up nor §11.7 re-entry available.
Measured here: **she re-entered by external commit.** The library does not distinguish "cold" from
"stranded" — §11.7's path is open to anyone holding a current `GroupInfo`, membership status
irrelevant.

### 3. **The new finding: nothing serves `GroupInfo`.**

The escape needs a **current `GroupInfo`**, and **neither delivery target carries one**:

- the **group queue** is unnameable to her by construction — that is what being stranded *means*;
- the **personal inbox** carries `Welcome`s;
- and a `GroupInfo` is **not a queued object at all** — it is produced on demand by a member holding
  live group state.

> **§11.7's "self-service" return is self-service in COST only.** It still requires a live member to
> answer, over a channel this design does not have.

**So the limbo fix is not only `retention ≥ liveness`. It is also: something must serve `GroupInfo`
to a returner.** Filed as **E105**; not resolved here.

### 4. The constructive half, and one production change

Measured: with retention set to the Group's liveness window (30d), **the same 15-day absence costs
nothing** — she drains her next hop and is current. The limbo band is empty by construction.

This drove the spike's one production change in this round: **`Meer::sweep_with_retention(days)`**,
with `sweep()` delegating to it at `RETENTION_DAYS`. The signature is the claim in code — §11.6's
windows are set per Group (90 days at 250–1k down to 14 at 7–10k), so **a single service constant
can only ever be correct for the most aggressive band. Retention is a Group governance value the meer
is told, not a property of the meer.**

---

## S16 — The governance-attestation half of §11.7's two-part credential (2026-08-13)

§11.7 defines re-entry as **governance attestation** (standing) + **resumption PSK** (keys). S14
measured the key half "working" — but that rejoin supplied **no PSK at all**, which means whatever
admitted her was not the credential §11.7 describes. This tests the halves properly.

**Rung: A (real-lib).** **Code:** `tests/s16_governance_attestation.rs`. No CISS — nothing here
touches storage.

**Verdict: §11.7's two-part credential is not implementable on openmls 0.8.1 as written. Both halves
fail, for different reasons, and a third mechanism carries both.**

### 1. MLS checks no standing whatsoever

Measured: a party who was **never a member**, never invited, holding **no group secret of any
epoch**, joined a live group by external commit using only a current `GroupInfo` — and the incumbent
**processed and merged it without objection**. 2 members → 3.

**This generalises S11 from the inbox to the group.** S11: a stranger can seat **you** in a group you
never asked to join. S16: a stranger can seat **herself** in a group that never asked for her. Both
are MLS working as specified; both say the same thing — **admission control is entirely
application-layer.**

### 2. The resumption PSK cannot be attached to an external commit at all

This arm was written expecting "it works but is optional". **It refuted that.** Attaching a
resumption PSK fails with `PskError(KeyNotFound)` even when the returner genuinely holds that epoch's
secret and has written it to her provider PSK store. The cause is structural:

- resumption PSKs resolve from the **group's own** `ResumptionPskStore`, never from provider storage
  — `schedule/psk.rs:530-537` sends the `Psk::Resumption` branch straight to
  `resumption_psk_store.get()`;
- a group built by external commit initialises that store **empty** — `ResumptionPskStore::new(32)`
  at `group/mls_group/commit_builder/external_commits.rs:290`;
- and the store's `add` is `pub(crate)`, so there is no public API to seed it.

**This is not a gap in our code. It is a gap between the spec text and the library, and the spec is
the thing that has to move.**

### 3. What does work: a governance-issued **external** PSK — and it carries both halves

Measured end to end: an external PSK resolves from provider storage (the `Psk::External` branch of
the same resolver), attaches to the external commit, is visible to the incumbent as a **countable
`psk_proposals()` entry before merging**, and the merge seats the returner.

> **One mechanism, both halves.** Possessing the token proves the governance issued it (**standing**);
> it binds into the commit's key schedule so it cannot be claimed without being held (**keys**).

Its one difference from a resumption PSK is the honest one: it proves **the governance vouched for
you**, not **that you were there**.

### 4. The policy hook exists, and it is pre-merge

Measured available on the `ProcessedMessage` **before** `merge_staged_commit`:

- **AAD survives to the incumbent**, byte-exact (60 bytes round-tripped) — the attestation's carrier;
- the sender is distinguishable as **`NewMemberCommit`** rather than a member's commit;
- the joiner's **credential is readable**, so the attestation can be checked *against an identity*.

Dropping the staged commit instead of merging left the group **unchanged** at its prior epoch and
member count. So the full check is expressible today: **read the token, read the AAD attestation,
verify both against the joiner's credential, then merge or drop.**

### Two honest limits, the second sharper

1. **The AAD is signed by the joiner's own new leaf key** — self-asserted. It authenticates the
   carrier, never the claim. The attestation must be a token *governance* issued and the member
   verifies; MLS supplies the envelope and nothing else.
2. **Refusal is not consensus.** One member declining moves only that member — anyone who merged is
   now at a different epoch, and **the group has forked.** So the attestation policy must be a
   **group-wide rule agreed in advance** (a group-context extension is the natural home), not a
   per-member judgement call. *A policy every member evaluates differently is a partition.*

---

## S17 — Nested sealing: does an outer seal still route? (2026-08-13)

E96 was parked on one blocking objection: an outer seal hides the routing metadata, so how would the
carrier route? **That objection died with the addressed model** — the queue name comes from
`export_secret`, which was never inside the envelope.

**Rung: A (real-lib)** — the group ciphersuite's own AEAD via the provider's crypto, real MLS
exporter output for the key, real CISS, real queue. **Code:** `tests/s17_nested_sealing.rs`,
`src/outer_seal.rs`.

**Verdict: E96's fix works, costs 28 flat bytes, and breaks nothing. It is unblocked in practice, not
just in principle.**

### 1. The leak closes, measured in bytes

`group_id` (16 bytes) appears **verbatim** in the bare MLS envelope — S7 reproduced by grepping the
bytes, not by citing it — and is **absent** from the outer-sealed object, which does not parse as MLS
at all.

**Cost: 188 → 216 bytes, `28 bytes` overhead** (12-byte nonce + 16-byte AEAD tag). **Measured flat:**
a 64 KiB payload pays the same 28 bytes.

### 2. Routing, dedup and byte-identity all survive

Measured end to end over the real queue: published under an `export_secret`-derived name, forwarded
**byte-identically**, drained, unwrapped, opened. The same wrapped object queued twice is still one
entry.

**The E96 objection is answered:** the meer never needed anything inside the envelope to route.
Dedup and M2's byte-identity are properties of the *outermost* bytes, so an extra layer is invisible
to both.

### 3. A stronger negative than S7's

A non-member holding the wrapped bytes is refused at the outer layer with an
**`AeadDecryptionError`** — a *decryption* failure, not a routing check. S7's refusal was a `group_id`
mismatch raised *before* decryption was attempted, so confidentiality held but was never exercised.
**Here the AEAD tag is what says no.**

### 4. The catch-up walk still inducts — and the wrapping rule that makes it work

Measured across 3 hops: she read every message and landed exactly current. The induction holds
because the outer key for hop N is derived from the epoch she is **already at** when she arrives.
**No deadlock, no extra round trip** — the two-step per hop is unwrap-then-process, both local.

> **The wrapping rule:** an object must be wrapped with the key of the epoch whose **queue** carries
> it. For the commit that **closes** an epoch, that is the epoch it closes, **not** the one it opens.

**Verified from the failing side**, because a rule is only worth stating if breaking it breaks
something: a commit wrapped at the epoch it *opens* is unopenable by the member who needs it
(`AeadDecryptionError`). She holds epoch N; the object wants N+1; the only route to N+1 is the object
she cannot open. **The deadlock is real, silent, and indistinguishable from a corrupt object.**

OpenMLS exports the **current epoch only**, so this is not a mistake the API can prevent — which is
why `outer_seal::OuterKey` is a first-class value a caller derives *before* committing and holds
across the commit, rather than something `wrap()` reaches for implicitly.

---

## S18 — How durable is a removal? (2026-08-14)

S16 measured that a party who was *never* a member can join on a `GroupInfo` alone. A **deliberately
removed** member is the same mechanism pointed at the case where getting it wrong hurts a real
person — a death, a divorce, a falling-out. The owner's framing, which this was built to check
rather than illustrate: *"being in the group is a multi-tiered constraint, and who you key your
responses for is the truest sense of your group — you can't be forced to do it."*

**Rung: A (real-lib).** **Code:** `tests/s18_removal_durability.rs`.

**Verdict: the side door is real, the refusal is real and stronger than expected, and the mitigation
is a different artifact than assumed.**

### 1. A removed member re-seated herself

Measured: removed at epoch 1 (3 members → 2), then **back at epoch 3 with 3 members** — using a
current `GroupInfo` alone. **No `Welcome`, no invitation, no PSK, and no member acting on her
behalf.** The member who performed the removal processed the re-entry without the library objecting.

> **A removal is exactly as durable as `GroupInfo` distribution, and not one bit more.**

**The re-entry channel and the removal side door are the same mechanism** — which means E105 cannot
be designed without deciding this, and the two cannot be given different answers.

### 2. But refusal holds, at two independent layers

Bob declined to merge; Carol re-seated anyway via Alice. Then:

- **Keys.** A message Bob sealed afterwards is unreadable to Carol — `An error occurred during AEAD
  decryption.`
- **Addressing.** Bob's queue name is not one Carol can derive, so his mail sits at an address she
  **cannot even ask for**. Her drain returns empty.

Neither layer needs the meer's cooperation, and neither needs Carol's. **The owner's framing is
literally true and mechanically enforced: who you key for IS your group, and nobody can force you to
key for someone.**

> **Methodological note.** The first version of this measured `Generation is too old to be
> processed` — a bookkeeping rejection, the same weakness S7's negative had. Corrected by having Bob
> commit once on his own branch first, so both sides sit at the **same epoch number** with divergent
> secrets. The failure is now a genuine AEAD decryption failure.

### 3. **The fork is invisible in the epoch counter**

The sharpest finding, and it came out of fixing the above. After Alice accepts and Bob declines and
each advances once, they hold the **same epoch number** and **different secrets** — measured by
deriving different queue names.

**So a client cannot detect a fork by comparing epochs.** The only symptom is that peers silently
stop being able to read each other.

**Carol did not split the group; the DISAGREEMENT about Carol split the group.** The mechanism is
sound and the **UX is the whole problem**: members must reach the same answer *in advance*, without
a negotiation round. **A dialog box asking each member "allow Carol back?" is a partition generator,
and one that hides its own damage.** This is why the readmission rule has to be a group-context
policy.

### 4. The mitigation is the ratchet tree, not the `GroupInfo`

Measured with a proper control: the **same** member, **same** removal, **same** epoch is *refused* on
a `GroupInfo` carrying no ratchet tree (`No ratchet tree available to build initial tree.`) and
*admitted* the moment the tree is bundled.

> **The admission surface is the ratchet tree.** Withhold it and re-entry needs two
> separately-distributed artifacts instead of one.

**An earlier version of this test measured nothing** and nearly recorded a false negative: it turned
`use_ratchet_tree_extension` **off in the group config** and then exported the `GroupInfo` with
`with_ratchet_tree: true`, handing the joiner the tree anyway. **The export flag is independent of
the group config** — an exporter chooses per call. So this must be enforced at whatever serves
`GroupInfo` (E105), *not* in the group's configuration.

**Bandwidth and governance want the same thing here.** S8 measured the tree extension roughly
doubling `Welcome` (330 vs 152 bytes/member) and crossing the 2 MiB cap first, at N ≈ 6,350. Ship the
tree deliberately and narrowly rather than by default.

---

## S19 — What does an epoch roll actually do, and what does it prevent? (2026-08-16)

The working model behind most of the planning, stated by the owner: *"an epoch roll is the literal
changing of the group encryption material, so a user left out of an epoch roll would have no way in
cryptographically."* S18's result appeared to contradict it. This separates the two claims.

**Rung: A (real-lib).** **Code:** `tests/s19_what_an_epoch_roll_does.rs`.

**Verdict: the working model is CORRECT about what it describes, and the two facts are consistent.
They are two different doors.**

### 1. The epoch roll really does lock her out — confirmed at the strong grade

Measured **at two grades, because the weaker one proves nothing**:

- While merely **behind**, she is refused on a **counter** check (`Message epoch differs from the
  group's epoch`) — which a member who merely *lagged* would also hit. That is bookkeeping.
- After advancing her own stale branch so both sides sit at the **same epoch number**, she is
  refused **on the key**: `An error occurred during AEAD decryption.`

She also cannot derive the new queue name, so she cannot even *find* post-roll mail.

**Each commit mixes new path entropy with the prior epoch's secret, and she holds neither the entropy
nor a leaf on the re-keyed path. Nothing she possesses computes the new epoch.**

*Side observation:* she **can** process the commit that removes her — it is addressed to the epoch
she still holds — so she learns she was removed.

### 2. But external join never derives from prior state, so losing it prevents nothing

Measured the sharpest available way: a returner on a **completely fresh provider** — no stored group
state, no prior epoch secrets, nothing carried over — joined the live group and the incumbent merged
it.

**Prior membership contributes nothing to the external-join path.** The joiner performs a KEM against
the **`external_pub`** key published *in the `GroupInfo`* to obtain the current epoch's `init_secret`,
then commits itself in.

> **The epoch roll's lock is on DERIVATION. The external-join path never derives.** Exclusion is
> *passive-reading* exclusion; re-entry is an *active protocol operation* gated on a published key,
> not on anything the roll destroyed.

Any planning that reads "locked out of the epoch" as implying "cannot get back in" is reading a
guarantee the protocol does not make — which is exactly why Part 2 §11.8 puts ban enforcement in an
**application-layer admission gate** rather than relying on the key layer.

### 3. There is no such thing as a "safe" `GroupInfo`

`export_group_info` offers **exactly one** option, `with_ratchet_tree`. Measured across both
settings: with the tree, a stranger gets in; without it, the refusal is specifically about the
missing **tree**, never a missing `external_pub`. And
`export_group_info_with_additional_extensions` documents that it *errors* if a `RatchetTreeExtension`
or `ExternalPubExtension` is supplied directly, so neither can be hand-managed.

> **Every `GroupInfo` a member can produce carries the external-join key.** There is no way to prove
> current group state without also admitting the holder.

**This closes the question S18 left open about which dial to reach for.** The admission surface
cannot be narrowed by exporting a weaker `GroupInfo` — only by **withholding the ratchet tree** and
by **controlling who is handed a `GroupInfo` at all**. Both are policies at the serving node, which
is why E105's channel and E107's removal durability are one decision.

---

## S20 — A governance ban at group scale (2026-08-16)

The owner's methodological challenge, and the right one to raise: *"I worry our testing right now is
'roll epoch and equally include all existing users including the ban prospect'… we need a way to say
10 people, 1 is banned by legit group governance, epoch roll, only include non-banned folks in new
group."*

S19 used a **two-person** group, which cannot distinguish "the removal excluded her" from "the group
is now one person and trivially disagrees with her". **At N = 10 the distinction is visible.**

**Rung: A (real-lib).** **Code:** `tests/s20_governance_removal_at_scale.rs`.

**Verdict: the base-layer model is confirmed, and two things nobody asked about turned up.**

### 1. The exclusion is genuine — nine agree, one is alone

All ten agreed on the derived queue name **before** the ban. After one governance removal commit the
**nine survivors all derive the same new key material**, the **banned member derives none of it** and
is stranded on the pre-ban key unchanged, and the roster goes to nine.

Measured at the strong grade: with her own branch advanced to the **same epoch number**, she is
refused on the **key** (`An error occurred during AEAD decryption`), not on a counter.

### 2. Three post-ban states, not two

| state | what she holds |
|---|---|
| never sees the removal | live object, stale key |
| **processes the removal** | **dead object — `UseAfterEviction`, no derivation at all** |
| rebuilds from a `GroupInfo` | fresh object, current keys |

**Nothing carries between them.** And note: **the well-behaved client gets the worst outcome** —
syncing and processing the commit is what marks the group inactive. Correct for a ban; a **defect for
a dormancy migration.**

### 3. Re-entry is self-admission, and the window is exactly the lagging member

She asks nobody for anything: she takes a `GroupInfo` from a member who has not synced the ban and
constructs a commit seating herself. **There is no request, so there is nothing to deny** — the gate
cannot be a permission prompt.

Measured: admitted by the lagging member; **not applicable** at a member already past the ban, whose
superseded epoch refuses it with no policy consulted.

> **The exposure window is not "anyone can let her back in". It is precisely the set of members whose
> view predates the ban.**

**So the gate is not at the ban and not at the re-entry — it is at the moment a `GroupInfo` is
served.** Enactment is instantaneous; *enforcement* is only as fast as the ban reaches whoever will
hand one out. This is why E105 and E107 are one decision, and it argues the `GroupInfo` server must
**resolve standing at head (§11.8) before serving**, not merely relay.

---

## S21 — The group shared secret, and where a governance gate can actually sit (2026-08-16)

The owner's model: *"C can invite D through a crypto package… but A and B still need to accept the
invite from a group governance perspective and agree to key for D — right?"*

**Rung: A (real-lib).** **Code:** `tests/s21_shared_secret_and_the_proposal_gate.rs`.

**Verdict: correct about the decision, wrong about the mechanism — and the difference decides where a
gate can live.**

### 1. One shared secret per epoch, not per-member keying

A, B and C derive the **identical** epoch secret. There is no operation that encrypts to A and B but
not C. **So "agreeing to key for D" is not a per-member act** — the only decision available is
whether to be in an epoch that contains D.

Measured downstream: after B merged the commit seating D, a message B sealed *for the group* was read
by **D** in cleartext. **To exclude D after merging requires a new commit removing D. There is no
lesser move.**

### 2. But the decision is real, and MLS already has the phase for it

C's Add **proposal** for D left the roster at 3 members **at every recipient**. A proposal seats
nobody, rolls no epoch, grants no keys. Only when A **committed** it did D get anything — and at that
moment A, B, C and D all held one secret.

> **propose → (governance decides) → commit.** The protocol-level form of the spec's decide-then-enact
> split (§7.3.6), available today. **This is where the invite gate belongs.**

### 3. And this is exactly what an external join lacks

An external join arrives as a `StagedCommit` from `NewMemberCommit` — a **commit, never a proposal**.
The joiner performed both halves itself, so **there is no pending-proposal phase to gate.**

| path | gate available | where |
|---|---|---|
| **C invites D** | **yes — the proposal phase** | in-protocol, before anything changes |
| **outsider seats herself** | **none exists** | only: who is served a `GroupInfo`, + a merge-time policy every member evaluates identically |

**Conflating these is what made the readmission discussion confusing.** "Members must agree" is
straightforwardly available for invites and structurally unavailable, *as a request*, for external
joins. **All-members-can-invite is a feature to manage, not a hole to prevent** — managed in the
proposal phase. The external-join path is the one that needs the dial.

---

## S22 — The readmission serving policy, when there is no server (2026-08-16)

Built to make the dial's **position 1** a thing rather than a label. **Written twice**: the first
version gated at a history-convergence *server*, which the owner corrected mid-build against Part 1.

> **Part 1 §2.4 (P-Durable-Enablement):** *"a Group MUST NOT structurally depend on any single
> persona's presence to act"*; the no-helper path MUST stay real. **A meer is optional; everything
> else is distributed.**

**Rung: A (real-lib)** for every MLS operation. Standing is
`SPEC-DELTA[groupinfo-serving-standing-stub]` — §7.3.1's fold is **not** reimplemented, and nothing
here is evidence about it or about gap-completeness. **Code:** `src/groupinfo_policy.rs`,
`tests/s22_serving_policy_when_there_is_no_server.rs`.

**Verdict: position 1 works, and is not sufficient — and the reason inverts the earlier
recommendation.**

### 1. Refusing to serve is the whole gate

A standing-checked peer **refused** a banned lineage. **The control proves it is the gate:** handed
the same `GroupInfo` it withheld, she re-enters immediately. So S18's *"a removal is only as durable
as `GroupInfo` distribution"* is not a defect to fix elsewhere — **it is a specification of where the
fix goes.**

### 2. The graceful path survives it

The **same peer, same chain, same epoch** that refused the banned lineage **served** a dormant member
in good standing, who returned immediately by external commit. **The key layer cannot make this
distinction** (§11.6/§11.8 use the identical removal commit); the standing chain can.

### 3. The tree is a second, independent gate

Same requester, same standing: served **bare** he is refused; served **with the tree** he is in. So a
peer has two dials — **who** it serves and **what** it releases. **The bare form recovers the one
property S19 said a `GroupInfo` cannot have:** proving current group state (for §7.4.2 corroboration)
*without* admitting its holder.

### 4. **The correction: every member is a serving peer**

Measured with **ten members** all holding current group state, ban reached nine: **nine refused
correctly, one stale peer served, and she needed exactly one yes.**

> **A negative check is only as good as the LEAST-synced member**, and Part 1 §2.4 guarantees there
> is no serving tier to shrink that set to.

**An earlier draft of this file claimed position 1 makes the residual "a small enumerable set a
community can watch". WITHDRAWN — an artifact of the server framing, and the server does not exist.**

### 5. And it inverts which position is robust

Under the **same** staleness, **position 2 refused at every peer.**

| | check | stale peer |
|---|---|---|
| position 1 | *negative* — "refuse if I know she is banned" | **fails OPEN** |
| position 2 | *positive* — "serve only on a token I recognise" | **fails CLOSED** |

**With a chokepoint, position 1 is the cheap right answer. Without one — by principle — position 1
degrades to the worst-synced member and position 2 is what holds.** Position 1 is not discarded: it
is the right shape for the **dormancy** path, where failing open is the *desired* behaviour. It is
simply not the ban defence.

**This also re-reads §11.8's eventual-consistency argument more favourably:** the spec never claimed
a chokepoint, it claimed re-keying backstops late propagation. **A positive credential is the
complement that claim needs.**

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

---

## S23 — the token ledger: PSK resolvability across membership change (2026-08-17)

`tests/s23_token_ledger.rs` · **Rung A (real-lib)**, real OpenMLS 0.8.1, no CISS (nothing here
touches storage). Part of E112 (the C-series + amended S-series). Plan:
`../../plans/2026-08-16-1-plan-token-reentry-proveout.md` (S23), amended by
`../../plans/2026-08-17-1-plan-head-currency-and-admission-fact.md`.

The constraint the readmission walk *derived* from RFC/S16 reading but never measured: because a
PSK secret is mixed into the commit's key schedule, **every incumbent that may process the return
commit must resolve the PSK from its own provider storage.** The token is therefore group state —
a "token ledger" — that must reach members who join after issuance and survive membership change.
Three arms, RED-first on arm 1.

### Arm 1 — the ledger constraint is real, and the failure mode is LOUD (RED-first)

RED step (watched fail): the optimistic no-ledger hypothesis — incumbent Bob, holding group state
but **not** R's PSK bytes, seats R anyway — was asserted and failed. So the constraint holds:
decision-2's ledger obligation does not dissolve. The durable test then characterizes the mode,
which the review (group D missed-issue 4) flagged as load-bearing for the strict-merge premise:

**Verdict: `S23 arm 1 MEASURED (real-lib)`: a missing ledger entry produces a clean, named
`process_message` error — `"The PSK could not be found in the store."` — at PSK resolution. Not a
silent drop, not a partial state change (Bob's epoch and member count are unmoved). A missing
ledger is therefore a loud production failure, which is what makes the strict-merge floor
enforceable.**

### Arm 2 — the ledger must reach members who join after issuance

R's token is issued while the group is {alice, bob}. Carol and Dave then join *after* issuance;
the ledger entry is transferred to Carol only (modeled as sealed app-layer state synced in-band,
deliberately **not** `GroupContextExtensions`, which leak into a served `GroupInfo`). R returns.

**Verdict: `S23 arm 2 MEASURED (real-lib)`: Carol (ledger transferred) resolves the PSK and seats
R; Dave (not transferred) fails at PSK resolution with the same `"PSK could not be found"` error.
The *transfer* is the load-bearing thing — the token cannot live only with the members present at
issue time.**

### Arm 3 — revocation is a chain fact, not a key-deletion race

Bob still *holds* the PSK bytes (the crypto would resolve — staging succeeds). But the issuance
fact, named in the returner's AAD attestation, is revoked in Bob's governance fold. Bob's policy
layer reads the attestation *before* `merge_staged_commit` and refuses.

**Verdict: `S23 arm 3 MEASURED (real-lib)`: with the PSK bytes present and staging SUCCEEDED, the
incumbent still refused the return because the AAD-named issuance fact is revoked. Revocation is a
policy decision over a chain fact, decided before merge — no key material is deleted and nothing
races. This is exactly the property decision-2 asserts.**

### What S23 does NOT discharge

Rung A / real-lib for the MLS mechanics only. The ledger *transport* (sealed app-layer sync to
late-joiners) is **modeled** by depositing bytes into a provider store — the real in-band sync
channel is unbuilt. Revocation's governance-fold source is modeled by an in-test `HashSet`; the
fold-to-position read that populates it is the fold side's job (croft-chat), not measured here.

---

## S24 — position 2 end-to-end, with the admission fact (2026-08-17)

`tests/s24_admission_decision.rs` + `src/admission.rs` + `tests/common/mod.rs` · **Rung A** for the
MLS half (real openmls 0.8.1); **Modeled** for the governance plane (issuance ledger, acceptance
chain, serve challenge-response — in-memory stand-ins). E112. Amends the base S24 (2026-08-16 plan)
with the **admission fact** and refusal arm **(d)**. 8 arms, all green.

The centerpiece: DECISION-2's amendment — *recognition is the merge, and the merge deposits the
admission fact*. `src/admission.rs` carries it: an `AdmissionFact` (R6-shaped acceptance record,
event = the merged commit's content address, chained at the acceptor's frontier F), an
`IssuanceLedger`, and `mint_or_refuse` — the merge-rule clause in one place (a merge that cannot
mint its fact does not happen).

- **Graceful** — `S24 graceful MEASURED (Rung A MLS / Modeled governance)`: serve released the
  tree to a recognised lineage; the returner redeemed and was **seated at a new epoch** (real MLS);
  and the merge **deposited the admission fact** — event = the commit's content address, chained,
  carrying frontier F. One fact, indexed by the event.
- **Refused-if-absent** — `MEASURED (Modeled)`: `mint_or_refuse` with no issuance returns
  `NoIssuanceFact` and deposits nothing; a caller that merges only on `Ok` seats no one. The
  merge-rule clause holds.
- **Arm (d) — the severed-fact arm (REVIEW gap 1)** — `MEASURED (Rung A MLS / Modeled)`: the
  incumbent HOLDS the PSK bytes (crypto would resolve, MLS would seat), but with **no issuance
  fact** the admission gate refuses (`NoIssuanceFact`) upstream of the merge. Holding bytes is not
  holding a fact.
- **Arm (a)** — `MEASURED (Modeled)`: a genuine token presented by a bearer whose credential is a
  different lineage than the issued-to one → `LineageMismatch` (the §11.7 credential half).
- **Arm (c)** — `MEASURED (Modeled)`: valid token + lineage but standing revoked refuses at BOTH
  gates — serve (`BannedAtHead`) and merge (`Revoked` issuance).
- **Serve s-i** — `MEASURED (Modeled)`: a replayed challenge-response is rejected — the P-generated
  nonce is single-use.
- **Serve s-ii** — `MEASURED (Modeled)`: a valid `psk_id` presented by a requester who cannot sign
  for the issued-to lineage is refused **at serve** (not only at merge).
- **Perishability** — `MEASURED (Rung A)`: a `GroupInfo` served at epoch E builds a commit the
  group **refuses** once it has rolled to E+1 (real MLS). Leaked serve artifacts decay per roll;
  the token is the only durable thing.

**What S24 does NOT discharge:** the governance plane is Modeled (issuance ledger + acceptance chain
+ serve challenge-response are in-memory; the serve signature is a keyed digest, not a real
lineage-rooted signature). The arrival-order / comparator-placement property of the admission fact
(never slot-competing) is **C4 arm 1a**, not here. Artifact-isolation (tree without a current
`GroupInfo`) is structural — `external_commit_builder().build_group` requires a `VerifiableGroupInfo`,
so a tree alone constructs no join; not separately measured.

---

## C4 — the Bob/Dana stale-admission end-to-end (2026-08-17)

`tests/c4_bob_dana_end_to_end.rs` (+ `src/admission.rs` projection). **Rung A** for the MLS half
(real openmls 0.8.1 — seat, exposure reads, re-key exclusion); **Modeled** for the governance
projection (standing-over-spans in `src/admission.rs`). E112. The review's Bob/Dana story run whole,
with the admission fact as the detection trigger. 4 arms, all green.

- **Arm 1 (same-branch, whole group stale)** — `MEASURED (Rung A MLS / Modeled projection)`: a
  stale group served + merged the returner (own valid token + lineage key; only standing stale);
  the merge minted the admission fact so the collision was **chain-visible at merge**, before any
  read failure. The exposure window was **counted** — the returner read all 3 messages sealed
  during her open span. Sync arrived; the projection read **standing over the span** → `Excluded`,
  span recorded, **no hard-stop**. The §11.8 re-fire (real MLS `remove_members`) re-keyed her out:
  a message sealed after the re-fire no longer opens for her (AEAD). The window was real, the record
  says so, nothing retroactively unmade.
- **Arm 1a (arrival-order permutation)** — `MEASURED (Modeled)`: admission-fact-then-ban and
  ban-then-admission-fact project **byte-identically** — span recorded, subject `Excluded`, never
  `CONTESTED`. This pins the comparator placement (the fact opens a span, it does not compete on the
  standing slot). Mutation target: an impl that treats the fact as a standing decision would contest
  or become order-dependent here — the arm would go red.
- **Arm 2 (diverged branch)** — `MEASURED (Rung A commit / Modeled governance)`: stale Bob mints +
  merges; synced Carol (issuance revoked in her fold) refuses and mints nothing. The two branches
  differ by a **chain-readable admission fact** (its event/content-address), not by a queue name
  only cross-fork traffic would ever reveal (S18's silent case). The admission fact names the fork.
- **Arm 3 (genuine-contradiction control)** — `MEASURED (Modeled)`: a readmission **quorum** racing
  the ban is two decisions on the standing slot → `CONTESTED` (order-independent hard-stop); an
  admission **fact** racing the same ban is enactment vs decision → `Excluded`, never contested.
  The routine/genuine line is pinned from both sides. (CONTESTED's own pinning stays croft-chat's,
  per E108.)

**What C4 does NOT discharge:** the projection and the fork-naming are Modeled (the real fold's
CONTESTED/contradiction machinery lives in croft-chat's `local_storage_projection`, E108). The
continuity carry-over (B1 / RUN-12 shapes) referenced in the diverged-branch story is not exercised
here. Bears on Appendix-B obligation (3) (a late pre-checkpoint fact cannot silently reverse
enforcement — the void-and-refire path) and (4) (fork composition: explicit heal), as evidence, not
proof.

---

## S25 — the stale-peer matrix, amended (2026-08-17)

`tests/s25_stale_peer_matrix.rs`. **Rung A** for the MLS mechanics reused from S24; **Modeled** for
the serve/merge posture and the HeadAck freshness (which arms 2–3 genuinely consume from C3, via a
`[dev-dependencies]` path to `local_storage_projection`). E112. 5 arms, all green.

- **Arm 1 (liberal serve / best-known merge)** — `MEASURED (Modeled)`: a lagging incumbent seats X
  while a synced one refuses (revoked issuance) — the S18 invisible fork made concrete, named by
  the admission fact's presence/absence.
- **Arm 2 (liberal serve / STRICT merge)** — `MEASURED (Rung-A merge / Modeled freshness via C3
  HeadAck)`: below k the strict gate STALLS (X not seated); at k the incumbent is corroborated-
  current, which means it has folded the ban, so it refuses. X is never seated — the
  strict-merge/liberal-serve middle holds.
- **Arm 3 (strict serve / strict merge)** — `MEASURED (Modeled freshness via C3 HeadAck)`: a stale
  P stalls the serve for X *and* for a dormant good returner alike — that indiscriminate stall is
  the liveness price of strict-serve; it lifts once P reaches k HeadAcks.
- **Arm 4 (no serve check, strict merge)** — `MEASURED (Modeled)`: a tokenless party is handed
  GroupInfo+tree, but the strict merge refuses (`NoIssuanceFact`). Net gain is **roster knowledge
  only, never admission** — the serve check protects the roster, the merge check protects the
  membership.
- **Banned-holder population arm (the amendment)** — `MEASURED (Modeled)`: "fails CLOSED at a stale
  peer" is **population-dependent**. A stranger fails closed at serve (S22); a **banned holder's own
  token** is *served* at a stale peer, and its admission dies later — at the strict merge / at fold
  (per C4), not at the serve. REVIEW verdict item 1's unqualified wording is retired.

**Does NOT discharge:** the freshness is C3's HeadAck at Modeled/loopback grade; the propagation
window is expressed as the k-of-N corroboration threshold, not a wall-clock number (no clocks —
horizons in generations).

## S26 — catch-up replay determinism: admission at-position (2026-08-17)

`tests/s26_catchup_at_position.rs`. **Modeled** (positioned governance history + the C4/admission
projection). E112. 3 arms, all green.

- **Arm 1 (convergence)** — `MEASURED (Modeled)`: a member replaying {join@X, ban@Y>X} from behind
  lands **byte-identical** with the live edge (the admission projection is order-independent). The
  join is applied at its position X; the span is recorded; standing at head excludes. A
  head-anchored evaluation could not reach this.
- **Arm 2 (position-anchoring pinned)** — `MEASURED (Modeled)`: at-position and at-head evaluation
  **disagree** for a join at X < ban-position; the rule must consult fold-at-position. A
  head-anchored mutation refuses the historically-valid join and self-exiles — this test catches it.
- **Arm 3 (stale-majority)** — `MEASURED (Modeled)`: a stale-majority at-position-**invalid** join
  is corrected **governance-forward** — M processes it structurally, reads `Excluded` from the fold
  (§7.6.12 phase 1), and converges via the §11.8 re-fire. No chain-refusal during catch-up (that is
  reserved for a deliberate fork).
