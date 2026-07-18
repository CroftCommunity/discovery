# RUN-19 — Sealed tier in the browser's shape: MLS in WASM exchanging messages over real QUIC

`Run summary, 2026-07-18. Branch claude/wasm-seal-run-19-o60nr3. Rust 1.94.1, Node 22.22.2.
Experiment lands at alpha/experiments/wasm-seal/ (own workspace, 7 crates + Node hosts). TDD
red-first per part; prediction-first for every wire and build-behavior claim. Sequence gate
honored: RUN-16/17 merged before this run (P6 amends their landed docs via staging only).
Numbering: RUN-18 (the consolidated
publications/reception amendment) landed on main while this run was in flight; RUN-19 stands as
briefed, rebased onto it (the backlog section renumbered to §6g — RUN-18 took §6f). The run's grounding claims, cited:
OpenMLS lists wasm32-unknown-unknown as a supported CI-BUILT target with a js feature and a
pure-Rust provider, but its CI builds and does NOT test that target — P1 supplies the missing
test evidence for our stack; WebTransport reached cross-browser Baseline in March 2026
(Safari 26.4 joining Chrome 97+ 2022-01, Edge 98+ 2022-02, Firefox 114+ 2023-06), so the
browser transport leg is product-promiseable. No credentials, no money, no network beyond
crates-via-proxy and localhost QUIC. The run19-bare-openmls fallback NEVER FIRED — the real
croft-group L2a stack is what runs in wasm.`

## HEADLINE (the P2 interop verdict)

**No asymmetry between the builds.** Ciphertext sealed in wasm unseals in the native build and
vice versa; a half-wasm half-native transcript (Welcome join, messages both directions, a wasm
add-commit folded natively, a native removal evicting a wasm member, a wasm removal evicting the
native member) folds to **byte-identical exported epoch secrets and epochs at every step** (the
I4 comparator). The crypto provider behaves identically under wasm. Falsification did not fire.

## Environment preflight

| Check | Result |
|---|---|
| `rustc` / `cargo` | 1.94.1 |
| `wasm32-unknown-unknown` target | installed via proxy |
| wasm-bindgen-cli | 0.2.126 (matches crate pin =0.2.126) |
| wasm-pack | 0.15.0 (npm binary) |
| Node | 22.22.2 (`/opt/node22`) |
| chromedriver / chromium | present (147 / pw-1194) but **unusable**: no display, no IPv6 |
| IPv6 | absent (`os error 97` — consistent with MASTER-INDEX §5) |

## Per-part status (red → green evidence table)

| Part | What | RED | GREEN | Tests | Grade |
|---|---|---|---|---|---|
| P1 | croft-group MLS seal stack RUNS in wasm: create, Welcome add, seal/unseal, in-epoch msgs, epoch roll, forward-blindness — all inside the module | `3c78d3f` (8 fail under the Node runner) | `bf7905f` | 8 (in-wasm) | **wasm-node** |
| P2 | cross-build goldens: wasm↔native unseal both ways; half-and-half transcript byte-compared; cross-build removals both directions | `d089b1b` (`make p2-interop` FAIL) | `442f0fe` | 2 native + 14 driver checks | **component / wasm-node** |
| P3 | AES-GCM state at rest; SIGKILL the wasm host mid-conversation → restore → next-epoch decryption; eviction drill | `9a812d2` (4 fail + drill FAIL) | `6b6fed7` | 4 native + 8 drill checks | **component / wasm-node** |
| P4 | real QUIC: wtransport blind DS (own process) + native browser-parity client; offer-gating matrix; blindness at the dependency boundary | `1ce099f` (5 fail) | `11a3082` | 6 (1 always-green source guard) | **quic-native** |
| P5 | the full loop: two (then three) wasm members through QUIC + blind DS, commits and removal riding the wire; offered-but-cannot-read | `11e39b4` (driver absent) | `487799a` | 9 driver checks | **wasm-node + quic-native** |
| P6 | staged doc consequence (proposed-changes RUN-19 section + reviews-log row, same commit; README custody DRAFT; backlog §6g) | — (docs) | `99ed43d` | site gate | **staged, needs-call** |

_Grades: **component** = native pure logic · **wasm-node** = the real module under the
wasm-bindgen Node host (`SPEC-DELTA[run19-node-runner]`; the one guardrail-4 headless-chrome
attempt was made and failed environmentally — chromedriver dies at
`CreatePlatformSocket() … os error 97`, IPv6 absent, before ever reaching the browser) ·
**quic-native** = real QUIC/WebTransport on localhost, the client a native Rust WebTransport
client speaking the browser-identical protocol (the literal browser page is a named non-goal,
guardrail 4)._

## Predicted-vs-actual (every prediction, written red-first)

| # | Prediction | Verdict | Observed |
|---|---|---|---|
| PRED-CS | the MTI suite `MLS_128_DHKEMX25519_AES128GCM_SHA256_Ed25519` is available under wasm via the pure-Rust provider | **CONFIRMED** | `pred_cs_ciphersuite_is_mti` green in-module |
| PRED-RNG (build half) | compiling the stack to wasm WITHOUT openmls's `js` feature fails at getrandom ("target is not supported") | **CONFIRMED** | getrandom 0.2.17 emits the predicted `compile_error!` verbatim (probe recorded in the P1 Cargo.toml comment) |
| PRED-RNG (runtime half) | entropy inside the module is getrandom's js backend (`crypto.getRandomValues`) — distinct groups/secrets from identical inputs | **CONFIRMED** | `pred_rng_two_geneses_differ` green |
| PRED-RNG (nuance) | — | **CONFIRMED with a finding** | openmls's `js` feature covers only its OWN getrandom (0.3, `wasm_js`); the lineage pin rand =0.8.6 carries getrandom 0.2, which needs its `js` feature enabled separately by the consumer (the standard feature-union idiom). Without it the build still fails after `js` is on. |
| PRED-API | the `group-seal` API surface is unchanged under wasm (no cfg-gated signatures) | **CONFIRMED** | the P1 test source is byte-identical to what a native consumer writes; `seal.rs` GREEN is pure re-exports |
| PRED-WT1 | HTTP/3-CONNECT session over QUIC; client connects ONLY on the pinned SHA-256 cert hash; wrong hash refused at TLS before any request | **CONFIRMED** | `connect_requires_pinned_cert_hash` green (flipped-byte hash refused) |
| PRED-WT2 | one bidi stream per request; u32-LE framed JSON header + payload both ways | **CONFIRMED** | by construction + the matrix exercising it; a FIN/STOP_SENDING race surfaced (see nuances) |
| PRED-WT3 | `Identity::self_signed` mints an exactly-14-day certificate — the browser `serverCertificateHashes` ≤2-week cap, browser-parity by construction | **CONFIRMED** | wtransport 0.7.1 source: `.validity_days(14)` (pinned crate, cited in the test header) |

Environment nuances recorded (not divergences): the WebTransport client must bind IPv4
explicitly (`with_bind_default` binds `[::]` → os error 97 — the same IPv6 absence
MASTER-INDEX §5 records for iroh); the client's stream FIN is best-effort because the server
may STOP_SENDING once it has read a complete request (benign race; the length-prefixed framing
delimits requests).

## Findings (pinned, filed — not fought)

- **FND-R19-1 — a Welcome-joined `group-seal` member cannot itself invite.**
  `Device::join_from_welcome` uses `MlsGroupJoinConfig::default()`, which lacks
  `use_ratchet_tree_extension`, so the Welcome such a member produces is
  MissingRatchetTree-unjoinable. Surfaced by the P2 transcript; pinned **native** in
  `seal-native/tests/finding_ratchet_tree.rs` (proving it is not wasm-specific; the pin flips
  when fixed upstream). Backlog §6g row; transcripts route adds through the founder.
- **FND-R19-2 — the seal stack has no state-at-rest surface.** `group-seal`/`lineage-mls`
  deliberately hide the openmls provider, so MLS state cannot be serialized through them.
  Worked around by `seal-persist` (provenance-headed copy of the Device pattern extended with
  snapshot/restore); the upstream fix is a persistence seam (openmls supports it: the public
  storage map + `MlsGroup::load` + `SignatureKeyPair::read`). Backlog §6g row.

## The declared stand-ins (registered, not hidden)

| SPEC-DELTA | Kind | Site | Stands in for | Path back |
|---|---|---|---|---|
| `run19-node-runner` | declared-stand-in | `wasm-seal/.cargo/config.toml` (+ the Node hosts) | a real browser page hosting the module | product-side manual browser verification (backlog §6g); module/API/tests unchanged, only the host swaps |
| `run19-storage-shim` | declared-stand-in | `seal-persist/src/lib.rs` `BlobStore` + `node/wasm-peer.mjs` | IndexedDB/OPFS + WebCrypto-wrapped at-rest key (single-writer via Web Locks) | implement `BlobStore` over the browser substrate; blob format unchanged |

`run19-bare-openmls` (the guardrail-2 conditional) **never fired** — the real croft-group L2a
stack compiles and runs on the wasm target, so no register row exists for it.

## Pinned toolchain and crate pairs (guardrail 5)

| Piece | Pin | Pair rule |
|---|---|---|
| wasm-bindgen (crate) | =0.2.126 | must equal wasm-bindgen-cli **0.2.126** (installed) |
| wasm-bindgen-test | =0.3.76 | the 0.2.126-matched harness |
| wasm-pack | 0.15.0 | npm binary; used for pkg build + the one headless attempt |
| openmls / openmls_rust_crypto / _traits / _basic_credential | =0.8.1 / =0.5.1 / =0.5.0 / =0.5.0 | the lineage-groups/croft-group pins, unchanged |
| wtransport | =0.7.1 | **server and client from the same crate revision** — the draft-tracking WebTransport ecosystem's match rule |
| getrandom (wasm) | 0.2.17 `js` + 0.3.4 `wasm_js` (via openmls `js`) | the PRED-RNG nuance above |

## Measurements (sanity magnitudes, not benchmarks)

- **QUIC roundtrip**: put+fetch of a 192 B sealed frame over localhost WebTransport, n=50:
  **median ≈ 0.56 ms**, p90 ≈ 27 ms (p90 dominated by first-connection warmup in a cold spawn).
- **wasm module size**: 6.8 MB — a `--dev --no-opt` build (no wasm-opt pass, debug info
  retained); a release/opt product build is expected substantially smaller, not measured here.
- **In-wasm suite wall time**: 8 MLS tests in ~0.13 s under the Node runner (crypto at
  `opt-level 1`).

## What each part demonstrated (mapping to the model, for `(evidence: …, RUN-19)` citations)

- **GROUPS.md A.8 (transports / the browser leg)** — P1+P5: the sealed tier's browser shape
  needs no overlay bridge; MLS in wasm over WebTransport to the content-blind DS carries the
  whole loop (staged A.8 revision, P6).
- **GROUPS.md A.7 (the DS role)** — P4: the DS as a separate, authority-free process — opaque
  blobs, offer-gating by roster, one flat refusal, seq as delivery cursor never order, no
  unseal capability in its shipped dependency graph (`make p4-blindness`).
- **A.9 sealed row (removal semantics: cryptographic, next epoch)** — P5: the removed member is
  still OFFERED post-roll ciphertext it provably cannot READ — offering vs reading demonstrated
  across the wire (the §H/EXP-B distinction, now on real QUIC).
- **The upstream-evidence claim** — P1: OpenMLS wasm32 is CI-built but not CI-tested upstream;
  our stack now has in-module test evidence (8 green), plus the two feature-union facts a
  product build needs (PRED-RNG nuance).
- **State custody (P6 staging)** — P3: encryption-at-rest via the provider's AEAD, resumption
  across a real host kill into the next epoch, eviction honesty (no self-restore; rejoin blind
  to the gap) — the custody-posture DRAFT's load-bearing evidence.

## How to reproduce

```bash
cd alpha/experiments/wasm-seal
cargo test -p seal-wasm --target wasm32-unknown-unknown  # P1: 8 MLS tests inside wasm (Node runner)
cargo test                                               # native suites (13: wire, persist, QUIC matrix)
make p2-interop                                          # P2 goldens: wasm↔native, byte-compared
make p3-resume                                           # P3 drill: SIGKILL the wasm host, restore, evict
make p5-loop                                             # P5: the full loop over real QUIC
make p4-blindness                                        # the DS dependency-graph blindness evidence
cargo clippy --all-targets                               # clean
```

## Named non-goals (one line each, brief §4)

- **No real browser page** — one headless attempt made and recorded; manual product-side
  verification filed (backlog §6g).
- **No iroh** — its absence on this path IS the point (the overlay loads only for sealed
  steward governance per A.8; this run's leg is web-native end to end).
- **No custody-policy ratification** — drafted FOR review (P6, `needs-call`); nothing landed.
- **No key-recovery / I9** — eviction shows the boundary and stops; no recovery path built.
- **No performance benchmarking** — sanity magnitudes only.
- **No serving infrastructure** — the DS here is the experiment's blind store, not the RUN-15
  kit.
- **No lexicon publication** — no schema records anywhere in the run.

## Remaining items and what unblocks them (one line each)

- **The real page** — a browser session (product-side): load `seal-wasm/pkg`, IndexedDB/OPFS
  `BlobStore`, Web Locks tab-leader; upgrades both run19 stand-ins.
- **FND-R19-1 / FND-R19-2** — small croft-group/lineage-mls API changes (join-config parity;
  a persistence seam); the pins flip green→retired when fixed.
- **The A.8 revision + custody posture** — the owner's `needs-call` on the staged RUN-19
  section.
