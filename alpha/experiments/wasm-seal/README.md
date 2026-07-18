# wasm-seal — RUN-19: the sealed tier in the browser's shape

**Goal.** Prove the model behind the sealed tier's browser story: an MLS client
compiled to `wasm32-unknown-unknown`, sealing and unsealing real group traffic,
exchanged over actual QUIC (WebTransport's transport) through the content-blind
DS role. Grounding for the bet: OpenMLS lists `wasm32-unknown-unknown` as a
supported CI-built target with a `js` feature and a pure-Rust crypto provider
(`openmls_rust_crypto`), but its CI **builds and does not test** that target —
this run supplies the missing test evidence for our stack. WebTransport reached
Baseline in March 2026 (Safari 26.4 joining Chrome 97+, Edge 98+, Firefox
114+), so the browser leg is product-promiseable.

**Approach.** One workspace, parts landed red→green (`test(wasm):` /
`feat(wasm):` per part). P1: the croft-group L2a seal stack (`group-seal` →
`lineage-mls` → openmls 0.8.1 + pure-Rust provider) compiled to wasm and driven
through group create / add / seal-unseal / epoch roll / forward-blindness
**inside** the module, under the wasm-bindgen **Node** runner
(`SPEC-DELTA[run19-node-runner]`; one `wasm-pack test --headless --chrome`
attempt per guardrail 4). P2: cross-build goldens — sealed-in-wasm unsealed
native and vice versa, transcript folds byte-compared. P3: state at rest
through a storage trait (AES-GCM blob; `SPEC-DELTA[run19-storage-shim]`),
resumption after host kill, and the eviction drill (destroyed blob ⇒ rejoin via
fresh Welcome only). P4: a WebTransport (real QUIC) content-blind DS storing
offer-gated opaque ciphertext (the EXP-B pattern) + a native WebTransport
client with certificate-hash dev trust. P5: two wasm members through the full
loop, including a removal commit and the offering-vs-reading distinction across
the wire.

**Effort.** ~1 session (RUN-19), unattended; crates via the proxy only; no
credentials, no live network beyond localhost QUIC.

**Result.** See [`../RUN-19-SUMMARY.md`](../RUN-19-SUMMARY.md) — red→green
table with grades, predicted-vs-actual, pinned toolchain pairs, the P2 interop
verdict, measurements, and the register rows. (In progress until the summary
lands.)

## Prediction pins (written RED-first, per the prediction-first directive)

| ID | Prediction | Where tested |
|---|---|---|
| PRED-CS | the MTI suite `MLS_128_DHKEMX25519_AES128GCM_SHA256_Ed25519` is available under wasm via the pure-Rust provider | `p1_seal_in_wasm.rs::pred_cs_ciphersuite_is_mti` |
| PRED-RNG | module entropy is getrandom's `js` backend (`crypto.getRandomValues`); building **without** openmls's `js` feature fails at compile time (getrandom refuses the bare wasm target) | test `pred_rng_two_geneses_differ` + a recorded compile probe (summary) |
| PRED-API | the `group-seal` API surface is unchanged under wasm (no cfg-gated signatures) | the P1 test source is byte-identical to what a native consumer writes |
| PRED-WT (P4) | wtransport self-signed + `server_certificate_hashes` trust, one bidi stream per request, CONNECT-established session on localhost QUIC | `blind-ds`/`ds-client` predictions module (P4) |

## Layout

| Path | Part | What |
|---|---|---|
| `crates/seal-wasm` | P1 | the seal stack as a wasm module + the in-wasm test suite |

(Grows per part; final table in the summary.)

## Run it

```bash
cd alpha/experiments/wasm-seal
cargo test -p seal-wasm --target wasm32-unknown-unknown   # P1 suite in wasm (Node runner)
```

## Grades (honesty contract)

- **component / wasm-node** — the module runs under the wasm-bindgen Node
  runner, not a browser page (`SPEC-DELTA[run19-node-runner]`).
- **quic-native** — real QUIC via WebTransport on localhost; the client is a
  native Rust WebTransport client speaking the identical protocol a browser
  would (the literal browser page is a named non-goal in this environment;
  guardrail 4).

Every stand-in is a greppable `SPEC-DELTA[run19-…]` tag at its site and a row
in [`../SPEC-DIVERGENCE-REGISTER.md`](../SPEC-DIVERGENCE-REGISTER.md).
