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

### Normative-text flags

Raised here, **not** applied — spec edits are not a spike's call.

- **Part 2 §6.6.2** — the reasoning offered for the byte-identical `MUST` attributes the hazard to
  re-framing. The requirement should stand; the rationale should be corrected to name re-sealing,
  and may note that OpenMLS makes re-framing unavailable in a production build.
- No other normative text is implicated by M1 or M2.

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

## S1, S4–S8

Not yet run. S4 (multi-device) Phase 8; S5/S6 Phase 9; S7 Phase 10; S8 Phase 11; S1 (enrollment,
Rung C static) Phase 12.
