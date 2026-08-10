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

## S1–S8

Not yet run. S1 (enrollment, Rung C static) lands in Phase 12; S2–S8 in Phases 7–11.
