# Meer queue spike — Phase 0 (Discovery) findings

`Run: 2026-08-08` · `Plan: ../../plans/2026-08-07-2-plan-meer-queue-spike.md` · `Spec: ./SPIKE-SPEC.md`

Phase 0 is discovery, so the **Discovery Exemption** applies: these probes produce knowledge, not
production code. No TDD, no wiring tests. Each probe declares a disposition; the `promote` ones name
the phase that will re-implement them test-first.

Probes live in `src/bin/d*.rs` and are runnable: `cargo run --bin <name>`.

## Resolved versions (this run)

```
rustc 1.97.1 (8bab26f4f 2026-07-14) (Homebrew)   cargo 1.97.1
ciss 0.6.0 (path ../../../../CISS)                mls-replant 0.1.0 (path ../mls-replant)
openmls =0.8.1   openmls_rust_crypto =0.5.1   openmls_basic_credential =0.5.0
openmls_traits =0.5.0   tls_codec 0.4
iroh =1.0.0   iroh-relay =1.0.0   axum 0.8.9   reqwest 0.13.4
```

---

## D1 — cross-repo path dependency to CISS · **CONFIRMED**

`ciss = { path = "../../../../CISS" }` builds. `App`, `Blobs`, `Db`, `Limits` are all reachable from
outside the crate, and `App::router()` returns.

**Finding that changed the plan:** `Limits` has **no `Default`**, but its fields are `pub`
(`CISS/src/server.rs:165–170`), so the spike constructs it by literal with explicit values. That is
better than a `default()` would have been — the ceiling is stated in the spike, not inherited.

```
D1 OK: ciss path dep builds; App::with_limits + router() reachable.
  provider_id = id:665ff2f7d1eeaff1e5ecb493cd89099d2b68133f9aa7098a585f5aded1894ca9
```

Disposition: `promote` → `src/ciss_harness.rs` (Phase 1).

## D2 — real `PUT`/`GET` over loopback HTTP · **CONFIRMED**

Auth is the `id:` signed session: headers `x-croft-pubkey` + `x-croft-session`, signing the challenge
`ciss-session/v1/{did}`. Identity via `ciss::crypto::derive_keypair` + `ciss::identity::derive_id`.

```
[1] small PUT      -> 200 {"bytes":18,"cid":"33b07e42…","receipt_mode":"unilateral"}
[2] GET 33b07e42… -> 200, 18 bytes, round-trips = true
[3] PUT exactly 2 MiB   -> 200
    PUT 2 MiB + 1       -> 413 Failed to buffer the request body: length limit exceeded
[4] same bytes twice -> same cid = true
[5] GET /du -> 200 {"objects":[…3 objects…],"total_bytes":2097188}
[6] blob files on disk = 3   (blocks/{did}/{cid})
```

**Findings that changed the plan:**

1. **The over-cap refusal is axum's, not CISS's.** `2 MiB + 1` is rejected at **413** by axum's
   `DefaultBodyLimit` with the message `Failed to buffer the request body: length limit exceeded` —
   the request never reaches the blobstore, so `blobstore.rs`'s `ObjectTooLarge` is a second line of
   defence that the HTTP path never exercises. Phase 1's wiring test said "refused with CISS's own
   error"; it must assert **413 at the HTTP boundary** instead. The layering is sound (defence in
   depth), but the plan named the wrong enforcer.
2. **The boundary is exact.** Exactly 2 MiB is accepted; 2 MiB + 1 is refused. The Pass-3 both-edges
   assertion is satisfiable as written.
3. **Dedup is real and observable two ways** — identical bytes yield the same cid, and the on-disk
   layout is `blocks/{did}/{cid}`, so S2 can count files as an independent check against `du`.
4. `du` returns a per-object list of `{cid, bytes}` plus `total_bytes` — exactly the accounting S2
   needs, no additional endpoint required.

Disposition: `promote` → `src/ciss_harness.rs` (Phase 1).

## D3 — the re-frame · **THE SPEC'S NEGATIVE-ARM HYPOTHESIS IS FALSIFIED**

Two findings, and the second one reshapes M2.

**(a) In a default build, a forwarder cannot re-frame at all.** Both conversions that would allow
decode-then-re-encode are compiled out unless `test-utils` is on, and openmls says why in its own
source:

```rust
// The following two `From` implementations break abstraction layers and MUST
// NOT be made available outside of tests or "test-utils".
#[cfg(any(feature = "test-utils", test))]
impl From<MlsMessageIn> for MlsMessageOut { … }
```
`openmls-0.8.1/src/framing/message_out.rs:195–211`; the same gate and an equivalent comment on
`From<PrivateMessageIn> for PrivateMessage`, `src/framing/private_message_in.rs:263–277`.

So the library independently enforces the property M2's `MUST` describes. The spike reaches the
forbidden path only via an explicit, named `reframe` feature (`reframe = ["openmls/test-utils"]`),
which exists solely to construct the forbidden thing deliberately.

**(b) When forced open, a re-encode is BYTE-IDENTICAL.** This is the falsification.

```
application message:
  original   189 bytes  7d07055e216f7975d27f3ee8dd1b53a213e42970d39a55f447c570e61e925c72
  re-encoded 189 bytes  7d07055e216f7975d27f3ee8dd1b53a213e42970d39a55f447c570e61e925c72
  BYTES IDENTICAL: true
commit (PrivateMessage):
  original   490 bytes  cbac9a5b3adcf095db3437c705cc7092c8055e8ca18d59f85a3eb6c9f5883823
  re-encoded 490 bytes  cbac9a5b3adcf095db3437c705cc7092c8055e8ca18d59f85a3eb6c9f5883823
  BYTES IDENTICAL: true
```

The spike spec's stated hypothesis was:

> **Hypothesis (negative):** a deliberately re-framed copy — decode and re-encode the `MlsMessage`
> without changing semantic content — is **detectably different** at the byte level

**It is not.** TLS-codec serialization is canonical, so decode→re-encode is a faithful round-trip for
both application messages and commits. An assertion that "the digest differs" would fail.

**What this means (to be folded back in Phase 13).** The `MUST` is not weakened — the *risk model
behind it* was mis-stated. The hazard the spec names (ratchet-key / nonce reuse) comes from
**re-sealing** (encrypting again), not from **re-framing** (decode/encode). And re-sealing requires a
key the meer does not have. So:

- a blind forwarder cannot produce a semantically-equivalent-but-byte-different copy at all;
- the only transformation available to it is byte-preserving;
- the transformation that *would* be dangerous is cryptographically out of reach.

The `MUST` therefore has teeth for a **stronger and simpler** reason than the spec gave, and M2's
negative arm needs rewriting rather than deleting. Phase 6 is restructured accordingly.

Disposition: `promote` → M2's negative arm (Phase 6), in its corrected form.

## D4 — mls-replant composes; real application messages work · **CONFIRMED**

```
group stamped: members = 2
bob joined:    epoch = 1
sealed:        189 bytes, wire_format = PrivateMessage
plaintext in ciphertext: false
bob decrypted: "the conversation stays alive while you sleep" — matches = true
```

`Persona`'s fields are `pub`, so `create_message(&p.provider, &p.signer, msg)` works directly — the
"does the spike need its own persona type" branch (closed during Pass 3) is confirmed closed in
practice. Pass 3's stronger seal assertion (`wire_format() == PrivateMessage`) is satisfiable.

Disposition: `promote` → `src/mls.rs` (Phase 2).

## D5 — real iroh over a real loopback relay · **CONFIRMED**

```
relay ports (ephemeral): RelayPorts { http: 63160, https: 63161, quic: 63162, metrics: 63163 }
relay up: https://127.0.0.1:63161/
bytes sent      = 41
server received = 41 (identical: true)
server saw peer = c0aec995…
drain-scope check: server can identify the caller by EndpointId = true
```

The relay-spawn fallback the plan feared was **not needed** — a real relay comes up on loopback and
carries the connection.

**Findings that changed the plan:**

1. **Dial by `Endpoint::addr()`, not by bare `EndpointId`.** `presets::Minimal` configures no DNS
   discovery, so a bare id fails with `No addressing information available / No address lookup
   configured`. The ancestor does the same (`mls-welcome-over-iroh/src/main.rs:65,103`).
2. **The `meer-spike-drain-auth` stand-in is viable as specified** — the responder reads the caller's
   `EndpointId` off the authenticated connection, which is what Phase 4's negative half needs.
3. Ports: the copied `relay.rs` gained `RelayPorts::ephemeral()` (the original's fixed
   3340/3343/3478/9090 would collide across concurrent runs). `ALPN` is now `croft/meer-queue/0`.

Disposition: `promote` → `src/relay.rs`, `src/node.rs`, `src/transport.rs` (Phase 4).

## D6 — construction cost at scale · **NO PATHOLOGICAL CURVE**

`mls_replant::stamp`, release mode. **Tree extension OFF** (that is what `stamp` hardcodes).

|     N | stamp ms | commit B | welcome B | welcome B/mbr |
|------:|---------:|---------:|----------:|--------------:|
|     2 |      1.2 |      688 |       378 |         378.0 |
|    10 |      2.6 |     3023 |      1594 |         177.1 |
|    50 |     20.0 |    14291 |      7674 |         156.6 |
|   200 |     45.8 |    56463 |     30476 |         153.1 |
|   500 |    126.0 |   140797 |     76076 |         152.5 |

Roughly linear; 500 members in 126 ms. The full S8 sweep is cheap — no strategy change needed at
high N. Per-member Welcome converges to ~152.5 B, **matching E12.1's 152–155 B/member** and
confirming that the prior figure is the tree-OFF case.

Disposition: `throwaway`.

## D7 — is S8's construction available, and what does the tree cost? · **CONFIRMED + FIRST DATA**

Added during Phase 0. Pass 3 established that `mls_replant::stamp` cannot serve S8 (it hardcodes
`MlsGroupCreateConfig::default()` and discards the `GroupInfo`), but created no probe to confirm the
replacement path exists. It does:
`MlsGroupCreateConfig::builder().use_ratchet_tree_extension(bool).ciphersuite(CS).build()`, and
`add_members` returns `(MlsMessageOut, MlsMessageOut, Option<GroupInfo>)`.

**These are the corpus's first measurements of the tree-ON case.**

|     N | ext | commit B | welcome B | group_info B | welcome B/mbr |
|------:|:---:|---------:|----------:|-------------:|--------------:|
|     2 | off |      688 |       378 |         None |         378.0 |
|     2 | ON  |      688 |       792 |          650 |         792.0 |
|    10 | off |     3023 |      1594 |         None |         177.1 |
|    10 | ON  |     3023 |      3612 |         2254 |         401.3 |
|    50 | off |    14291 |      7674 |         None |         344.2 |
|    50 | ON  |    14291 |     16868 |         9430 |         344.2 |
|   200 | off |    56463 |     30476 |         None |         153.1 |
|   200 | ON  |    56463 |     66314 |        36072 |         333.2 |

Reading (directional — **the crossover is Phase 11's to establish, not this probe's**):

- The tree extension roughly **doubles** per-member Welcome cost: ~153 → ~333 B/member.
- `GroupInfo` with the tree runs ~180 B/member at N=200.
- **`commit` is unaffected by the extension** — identical bytes at every N, tree ON or off.
- Straight-line extrapolation puts the 2 MiB crossover in the **several-thousand-member** range for
  every O(N) object (Welcome-with-tree the earliest, order ~6k). Nothing crosses at conversational
  group sizes.

If Phase 11's real sweep bears this out, the spike spec's catastrophic branch — "application messages
or ordinary commits crossing 2 MiB → CISS needs streaming before it can be the meer's substrate at
all" — is off the table. **That is an extrapolation from four points, not a result.** S8 measures it.

Disposition: `promote` → the config-parameterized builder in `src/mls.rs` (Phase 11).

---

## What Phase 0 changes in the plan

| # | Change | Where |
|---|---|---|
| 1 | M2's negative arm is falsified before running; Phase 6 restructured around what a blind forwarder can actually do | Phase 6, Phase 13 |
| 2 | Over-cap refusal is axum's 413, not CISS's `ObjectTooLarge`; wiring test corrected | Phase 1 |
| 3 | `Limits` built by literal (no `Default`); explicit ceiling | Phase 1 |
| 4 | Dial by `Endpoint::addr()`, not bare `EndpointId` | Phase 4 |
| 5 | `RelayPorts::ephemeral()` added to the copied relay; ALPN renamed | Phase 4 |
| 6 | S8's construction path confirmed; first tree-ON data recorded | Phase 11 |
| 7 | A `reframe` cargo feature exists solely for M2's negative arm | Phase 6 |

**Nothing here invalidates the spike's premise.** The design is testable as scoped; one hypothesis
inside M2 was wrong, and being wrong early is what Phase 0 is for.
