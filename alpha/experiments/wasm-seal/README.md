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

**Result.** P1–P5 green, red-first. Headline: **no cross-build asymmetry** —
the crypto provider behaves identically under wasm (epoch secrets
byte-identical across half-wasm/half-native transcripts). The full loop runs
over real QUIC with the removed member offered-but-unable-to-read. The
`run19-bare-openmls` fallback never fired: the real croft-group L2a stack is
what runs in wasm. See [`../RUN-19-SUMMARY.md`](../RUN-19-SUMMARY.md) —
red→green table with grades, predicted-vs-actual, pinned pairs, measurements,
findings (FND-R19-1/2), and the register rows.

## Prediction pins (written RED-first, per the prediction-first directive)

| ID | Prediction | Where tested |
|---|---|---|
| PRED-CS | the MTI suite `MLS_128_DHKEMX25519_AES128GCM_SHA256_Ed25519` is available under wasm via the pure-Rust provider | `p1_seal_in_wasm.rs::pred_cs_ciphersuite_is_mti` |
| PRED-RNG | module entropy is getrandom's `js` backend (`crypto.getRandomValues`); building **without** openmls's `js` feature fails at compile time (getrandom refuses the bare wasm target) | test `pred_rng_two_geneses_differ` + a recorded compile probe (summary) |
| PRED-API | the `group-seal` API surface is unchanged under wasm (no cfg-gated signatures) | the P1 test source is byte-identical to what a native consumer writes |
| PRED-WT (P4) | wtransport self-signed + `server_certificate_hashes` trust, one bidi stream per request, CONNECT-established session on localhost QUIC | `blind-ds`/`ds-client` predictions module (P4) |

## P3 — the browser mapping and the hazards, documented (not built)

The in-environment storage backing is a file / the Node host's filesystem
(`SPEC-DELTA[run19-storage-shim]`). The **product (browser) mapping** the
drill stands in for: the member's MLS state rests in **IndexedDB or OPFS**
only ever as the AES-GCM blob (`nonce || ciphertext`, AAD-domain-separated —
the same bytes this drill produces); the at-rest key is held as a
**WebCrypto non-extractable key** (wrapped, never serialized into page
memory as raw bytes), so a storage dump without the key is ciphertext.
Restore = fetch blob → unwrap key → `restore()` inside the module.

**Multi-writer hazard, named:** MLS state is strictly **single-writer** —
two tabs advancing the same ratchet from one blob fork the hash chain and
brick the member. The product must elect one writer (Web Locks API /
tab-leader election, a SharedWorker, or a service-worker owner); this run
demonstrates the single-writer discipline (one host process owns the blob)
and does **not** build the election.

**Eviction honesty:** destroying the blob is destroying the member — there
is no self-restore path (forward secrecy is not overridden by any recovery
mechanism here); re-entry is a fresh add via Welcome, provably blind to the
gap. Key recovery/escrow is I9 territory and firewalled out (non-goal).

## Custody posture (DRAFT — FOR OWNER REVIEW)

> Drafted by RUN-19 P6 for review, NOT ratified (guardrail 6). The staged
> spec-facing form lives in the proposed-changes doc (RUN-19 section).

A browser sealed member's caveats are custody-shaped, not transport-shaped:
group keys live in wasm linear memory while the page runs, so **XSS is the
threat model** (a scripted page can drive the module's own API; wasm memory
is not an enclave) and the sealed surface carries the strictest
no-third-party-script/CSP posture. The **blast radius is bounded by
device-key delegation** (RUN-17 P5): the browser member holds a delegated
device key, never the account root — compromise burns one device key;
**revocation is attestation deletion** (event-driven) plus the MLS removal
re-key this run demonstrates across the wire. **Eviction is honest**: a
destroyed blob has no self-restore; re-entry is a fresh Welcome, blind to
the gap. **Single-writer**: one tab-leader owns the ratchet (Web Locks in
product). **Draft-status pinning**: WebTransport server/client must ship
from matching revisions (wtransport =0.7.1 both sides here).

## Layout

| Path | Part | What |
|---|---|---|
| `crates/seal-wasm` | P1/P2/P3 | the seal stack as a wasm module (re-exports of croft-group `group-seal`), the in-wasm P1 suite, and the JS APIs (`JsSealer`, `JsPersistSealer`) |
| `crates/seal-wire` | P2 | KeyPackage across the process/build boundary (tls_codec + validate) |
| `crates/seal-native` | P2 | the NATIVE build as an ndjson peer (`seal-peer`) + the FND-R19-1 pin |
| `crates/seal-persist` | P3 | the persistence-capable member (provenance-headed Device copy + AES-GCM snapshot/restore + `BlobStore`) |
| `crates/ds-proto` | P4 | the crypto-free DS wire protocol (the blindness boundary) |
| `crates/blind-ds` | P4 | the content-blind DS over wtransport QUIC (own process) |
| `crates/ds-client` | P4/P5 | the native browser-parity WebTransport client + `ds-cli` |
| `node/` | P2/P3/P5 | the Node hosts and drivers: `interop.mjs`, `wasm-peer.mjs`, `resume.mjs`, `loop.mjs` |

## Run it

```bash
cd alpha/experiments/wasm-seal
cargo test -p seal-wasm --target wasm32-unknown-unknown  # P1: 8 MLS tests inside wasm (Node runner)
cargo test                                               # native suites (wire, persist, QUIC matrix)
make p2-interop                                          # P2 cross-build goldens
make p3-resume                                           # P3 host-kill + eviction drill
make p5-loop                                             # P5 full loop over real QUIC
make p4-blindness                                        # DS dependency-graph blindness evidence
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
