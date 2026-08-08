# Meer queue spike — does the pub/sub-in, mailbox-out shape hold against real MLS traffic?

`Status: specified, not yet run`

`Hypothesis doc: discovery/alpha/thinking/meer-as-custodian-queue.md (claims under test cite it)`

`Methodology: bound by discovery/beta/impl/delivery-layer/08-experiment-methodology.md`

`Class: spike (PLAYBOOK §1 — "does it work / is this reachable"), not a proof`

---

## Why this spike exists

The meer design (`meer-as-custodian-queue.md`) is coherent on paper and rests on one unexercised
claim: that a store-and-forward node which **does no ordering, holds no group state, and holds no
key** is sufficient to carry a real MLS conversation across an absence. Everything downstream — the
CISS custodian-chain substrate, chain kinds, ceilings, the gateway-service pattern — is built on that
being true, and the substrate carries the security-review cost. So we test the shape before we pay
for the substrate.

This is the first time the thinking is put through practice rather than theory. The spike is
therefore scoped to two **must-pass** claims and a set of **shape-learning** scenarios that we expect
to teach us something regardless of whether they pass.

## How this is bound by the methodology

Restated from `08-experiment-methodology.md` because these are the rules that make a result
admissible:

1. **State the fidelity rung in every verdict line.** `CONFIRMED (real-lib)`,
   `CONFIRMED (model-form: <stand-in named>)`, or `CONFIRMED (static)`. A bare `CONFIRMED` is
   inadmissible.
2. **Never substitute a stand-in for the exact component a claim is about.** The canonical forbidden
   move is XOR-as-MLS. Every claim here touching the seal runs against real OpenMLS.
3. **Pin and print exact resolved versions** in every result.
4. **Do not assert an API shape from memory.** Read the crate docs or run `cargo doc` first.
5. **A FALSIFIED result is a success.** Record it loudly and name the branch it reshapes.

## Libraries and versions

Pin and print at run time. The workspace's runnable MLS is **OpenMLS**, already used by
`mls-replant` (`openmls = "=0.8.1"`, `openmls_rust_crypto = "=0.5.1"`,
`openmls_basic_credential = "=0.5.0"`, `openmls_traits = "=0.5.0"`).

**Reuse, do not rebuild:** `iroh/crates/mls-welcome-over-iroh` already creates a real OpenMLS group and
carries a real Welcome across a real iroh connection homed on a real relay, with the joiner deriving
the same exporter secret. That is the ancestor of this spike; extend it rather than starting fresh.
`mls-replant` is the second reuse source for group construction.

**Fidelity note for the record:** the round-2 delivery experiments
(`delivery-layer/10-experiments-round2.md`) ran against `mls-rs 0.55.2`, but that code does not exist
in this workspace. This spike runs against OpenMLS 0.8.1 because that is what is actually runnable
here. Results are real-lib either way; the library differs from round 2 and results should not be
cross-compared without noting it.

## Registered stand-ins (SPEC-DELTA)

The spike runs against **plain CISS as it exists today**. The following are deliberate stand-ins and
must be registered in `SPEC-DIVERGENCE-REGISTER.md`, with `SPEC-DELTA` markers in the code:

- `SPEC-DELTA[meer-spike-namespace | stand-in]` — no custodian chain mode exists yet, so the meer owns
  one CISS namespace and queues are slots within it. The spec target is per-DID queues under a
  custodial grant. This changes *who signs*, not the delivery shape under test.
- `SPEC-DELTA[meer-spike-kind-gate | absent]` — chain kinds and the queue-only custodial-write gate do
  not exist yet; nothing enforces them in this run.
- `SPEC-DELTA[meer-spike-drain-auth | stand-in]` — drain is scoped by iroh EndpointId (available free
  from the authenticated QUIC connection) rather than by CISS account identity. The spec target is
  account identity; the spike does not exercise multi-device-per-account auth.

Nothing about the **seal** is stood in. Every claim touching confidentiality or byte-identity runs
against real OpenMLS.

## The cast

Following the corpus cast so the beats stay legible:

- **Alice** — sender, online throughout.
- **Bob** — recipient, offline for the window that matters. Has a phone and a laptop.
- **Carol** — a node that carries but is not in the group (holds no leaf key).
- **The meer** — always-on, blind, holds no key.

---

## Must-pass claims

### M1 — An offline member drains and decrypts

**Claim under test:** a member offline during a message's live window recovers it from the meer and
decrypts it, with the meer never holding a key (hypothesis doc §"What the meer does"; Part 2 §6.6.2).

**Hypothesis:** Alice seals an application message to a real OpenMLS group while Bob is disconnected.
The meer stores the sealed bytes and an entry in Bob's queue. Bob reconnects, drains via a have/want
digest diff, and `process_incoming_message` yields the correct plaintext. The meer, asked for its key
count, reports zero.

**Method:**

1. Build a real two-member group (Alice, Bob) reusing `mls-welcome-over-iroh`'s construction path.
2. Disconnect Bob's endpoint.
3. Alice seals with the real OpenMLS application-message API and publishes to the meer.
4. Meer `PUT`s the blob to CISS, appends to Bob's queue slot.
5. Reconnect Bob. Bob sends his have-set (empty), receives the want-set, fetches blobs, acks.
6. Bob runs the sealed bytes through real `process_incoming_message`. Assert the plaintext.
7. Assert `meer_payload_keys_held == 0`, and that the meer process never held group key material.

**Fidelity rung intended:** A (real-lib). No stand-in for the seal.

**Print:** `M1 CONFIRMED (real-lib): offline member drained N blobs and decrypted; meer keys held = 0.`
or `M1 FALSIFIED (real-lib): <what happened>.`

### M2 — Byte-identical forwarding, and the negative case

**Claim under test:** the meer stores and forwards sealed bytes **unchanged**; a re-sealed or re-framed
copy is the one way a blind forwarder could break an MLS guarantee (Part 2 §6.6.2, the `MUST` on
byte-identical storage; ratchet-key / nonce reuse is the hazard).

**Hypothesis (positive):** `bytes_out == bytes_in`, exactly, for every queued object, including
across the CISS content-address round trip.

**Hypothesis (negative):** a deliberately re-framed copy — decode and re-encode the `MlsMessage`
without changing semantic content — is **detectably different** at the byte level, and feeding it to
Bob produces a real library-level failure rather than silent success. This is the test that proves the
`MUST` has teeth rather than being a comment.

**Method:**

1. Capture `sha256(bytes)` at the moment Alice produces the sealed message.
2. Assert the same digest after the meer's `PUT`, after CISS's re-verify-on-read, and at Bob's
   receive, before any decode.
3. Then, in a separate arm, have the meer round-trip the message through decode/encode and forward the
   re-encoded bytes. Assert the digest differs, and record precisely what OpenMLS does when Bob
   processes it — error, or accepted-but-different.
4. Record the failure site and error type verbatim. Do not assert the API shape from memory.

**Fidelity rung intended:** A (real-lib).

**Print:** `M2 CONFIRMED (real-lib): digest stable across store+serve; re-framed copy differs at <site>
and is rejected at <error>.` or `M2 FALSIFIED (real-lib): <what happened>.`

**Note on why the negative arm matters:** if a re-framed copy is *accepted* by OpenMLS, the `MUST` is
weaker than the spec text implies and the hypothesis doc needs correcting. That is a valuable
falsification, not a failure of the spike.

---

## Shape-learning scenarios

These are here because the ask is to learn the shape of the problem, not only to pass two assertions.
Each may falsify something in the hypothesis doc; that is the point.

### S1 — Enrollment: what does pointing a meer at your queue actually require?

Walk the workflow even though custodian mode does not exist. What does Bob have to hold, sign, and
hand over before the meer can accept mail for him? Record the sequence and every piece of state it
implies. **Learning goal:** whether the "one line in your inventory" story survives contact, and
whether enrollment needs anything the hypothesis doc does not mention.

### S2 — Fan-out and dedup

One message, five recipients. Assert the blob is stored **once** in CISS (content-addressed) and
referenced five times. Measure the actual storage cost of a fan-out versus the naive per-recipient
copy. **Learning goal:** confirm the dedup claim is real at the CISS boundary, not just in theory.

### S3 — Dual delivery

Bob receives the same message twice — once carried live, once drained from the meer. Assert it
deduplicates on content hash to a single entry, and that MLS applies it idempotently (Part 2 §6.6.2:
a duplicate commit no longer matches current state and is dropped). **Learning goal:** whether the
racing story (§6.6.4) is free in practice as the spec claims.

### S4 — Multi-device and deliver-once

Bob's phone and laptop, both enrolled. With deliver-once + prune-on-ack, does the laptop starve?
Run it **without** a device group first (expected: the laptop starves — this should falsify naive
deliver-once), then reason about what the device-group presence check needs to detect. **Learning
goal:** the hypothesis doc claims deliver-once is correct *because* §6.6.5 fans out; this scenario is
where that dependency becomes concrete rather than asserted.

### S5 — Expiry and the watermark

Age a queue past its window with no drain. Assert the bytes are gone and the watermark remains, and
that Bob's client can render an honest "here is what you missed and it is gone." **Learning goal:**
whether "loud, visible gap" is actually constructible from what the meer retains, or whether it needs
more state than the watermark.

### S6 — Revocation and re-point

Bob points at a second meer and stops using the first. Assert no mail is lost and nothing needs to be
migrated. **Learning goal:** the "it never left home" claim is the design's strongest story; this is
where it either holds or reveals a hidden dependency on the incumbent.

### S7 — Carol carries and learns nothing

Carol's node handles the sealed bytes and, given them directly, cannot decrypt (real OpenMLS failure,
not garbage-out). Assert what Carol *can* observe: digest, length, timing. **Learning goal:** state
the real observed metadata set rather than the assumed one, so §6.4's leak profile is grounded in a
measurement.

### S8 — Object sizes against the 2 MiB cap

**This is the scenario most likely to change the design.** See below.

---

## S8 in full: the `MAX_OBJECT_BYTES` question

**The constraint.** CISS refuses any object over `MAX_OBJECT_BYTES = 2 * 1024 * 1024` (2 MiB, in
`src/blobstore.rs`), on **both** put and get, and the HTTP boundary independently caps request bodies
at 2 MiB via axum's `DefaultBodyLimit`.

**It is not an arbitrary constant.** It came from a real finding in the 2026-08-03 security review: a
512 MiB upload was fully buffered in RAM against a `MemoryMax=384M` unit, so a single unauthenticated
request could restart the service at will. "Just raise the cap" re-opens that vulnerability unless
streaming replaces buffering first. Treat the number as load-bearing.

**Why it collides with MLS.** Most MLS objects are small — an application message is payload plus
framing, and a commit's `UpdatePath` grows only with the log of the group size. **The exception is
`Welcome` and the `GroupInfo` it carries**, because a `GroupInfo` that embeds the ratchet-tree
extension is **O(N) in group size**: every leaf contributes credential, signature key, HPKE key,
capabilities, and a signature. The corpus already knows this from two directions — Part 2 §6.9.1 says
the broadcast tier **MUST** disable the embedded ratchet tree and ship it out of band "because at
broadcast scale the per-commit O(N) tree cost is the binding constraint," and the cairn survey notes
RFC 9750 leaves "the storage and serving of the large `Welcome` and `GroupInfo` objects" to the
deployer.

**And the meer cannot chunk by re-framing.** M2 forbids it: the meer must return byte-identical bytes,
so any splitting must be transparent transport-level chunking that reassembles exactly, never a
re-encode.

**What we do not know is the actual number.** Nobody has measured where a real OpenMLS group crosses
2 MiB. That is the measurement.

**Method:** build real OpenMLS groups at increasing sizes (suggested: 2, 10, 50, 200, 1000, and as
high as the harness sustains). For each, serialize and record the byte length of:

| object | expected growth | measured at N = … |
|---|---|---|
| application message (`PrivateMessage`) | flat | |
| commit (add / remove / update) | ~log N | |
| `GroupInfo` **with** ratchet-tree extension | **O(N)** | |
| `GroupInfo` **without** the extension | flat-ish | |
| `Welcome` (1 joiner) | O(N) via GroupInfo | |
| `Welcome` (k joiners) | O(N) + k | |

**Print:** `S8 MEASURED: <object> crosses 2 MiB at N = <n>; <object> stays under at N = <max tested>.`

**The decision this feeds.** Three options, and the measurement picks between them rather than taste:

1. **Out of scope for v0** — the meer queues application messages and commits only; `Welcome`
   delivery uses another path. Cheapest, and defensible if Welcomes are rare and usually deliverable
   to an online joiner.
2. **Transparent chunking** — split at the transport boundary, reassemble byte-identically. `ciss-sync`
   already has FastCDC chunking and a manifest mapping a logical object to chunk CIDs, so the
   machinery exists. Costs a chunk-manifest layer inside the queue entry.
3. **Ship the tree out of band** — which Part 2 §6.9.1 already **mandates** for the broadcast tier.
   If the measurement shows the crossover lands below the group sizes we care about, this stops being
   a meer problem and becomes the group-tier decision it already is in the spec.

Option 3 is the interesting one: the cap may simply be re-discovering, from the storage side, a
constraint the spec already resolved from the protocol side.

---

## What would falsify the design

Recorded up front so we cannot rationalize afterward:

- **M1 fails** → store-and-forward without group state is insufficient; the meer needs to understand
  something about MLS, and the "it's just a mailbox" framing is wrong.
- **M2's negative arm shows a re-framed copy is accepted** → the byte-identical `MUST` is weaker than
  the spec states; correct the hypothesis doc and Part 2's reasoning.
- **S4 shows deliver-once starves a second device even *with* a device group** → the device-group
  fan-out is not a sufficient compensating mechanism and per-device cursors come back.
- **S6 shows re-pointing loses mail** → "it never left home" is false under the stand-in, and the
  claim depends on custodian mode in a way the hypothesis doc does not admit.
- **S8 shows application messages or ordinary commits crossing 2 MiB** → the cap is a general problem,
  not a `Welcome` problem, and CISS needs streaming before it can be the meer's substrate at all.

## Out of scope

- Custodian chain mode, chain kinds, ceilings (the substrate — informed by this spike, not tested by
  it).
- Metering, billing, and the offline-data-fraction measurement.
- The history-convergence store (gated separately; needs G-hist and nested sealing).
- Push / wake (P-push), gossip carriage (C-swarm), and D-peer reconciliation.
- Performance, throughput, and fleet sizing.

## On completion

- Verdict lines with fidelity rungs into a `TEST-LOG.md` here.
- Stand-ins registered in `../SPEC-DIVERGENCE-REGISTER.md`.
- Falsifications folded back into `discovery/alpha/thinking/meer-as-custodian-queue.md` — and, if any
  touch normative text, flagged for `beta/drystone-spec/`.
- Register in `../EXPERIMENT-BACKLOG.md` and `../MASTER-INDEX.md` (transport track).
