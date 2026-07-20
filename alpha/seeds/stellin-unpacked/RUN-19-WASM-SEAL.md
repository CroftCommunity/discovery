# RUN-19: Sealed tier in the browser's shape — MLS in WASM exchanging messages over real QUIC

`Status: runnable brief, 2026-07-17. Proves the model: an MLS client compiled to
wasm32-unknown-unknown, sealing and unsealing real group traffic, exchanged over actual QUIC
(WebTransport's transport) through the content-blind DS role. Grounding for the bet, cite in
the summary: OpenMLS lists wasm32-unknown-unknown as a supported CI-built target with a `js`
feature and a pure-Rust crypto provider, but its CI builds and does NOT test that target —
this run supplies the missing test evidence for our stack; WebTransport reached Baseline in
March 2026 (Safari 26.4 joining Chrome 97+, Edge 98+, Firefox 114+), so the browser leg is
product-promiseable. Executes in the discovery repo under house rules. TDD red-first per the
standing directive; prediction-first for every wire and build-behavior claim. Numbering:
assumes RUN-18 (the consolidated publications/reception amendment); confirm against
MASTER-INDEX and renumber consistently if taken. Independent of RUN-18's merge (different
surfaces); sequence-gate only on RUN-16/17 being merged, since P6 amends their landed docs
via staging.`

## 0. Owner pre-steps

None. No credentials, no money, no live network required (crates via the proxy only). One
optional flag: if you want the browser-manual step attempted, say so at session start;
default is the graded stand-in per guardrail 4.

## 1. Where it lands

`alpha/experiments/wasm-seal/` — own crate/workspace, house experiment style (README:
goal/approach/effort/result). Path-deps on croft-group crates where they compile to the wasm
target; provenance-headed copies where self-containment wins. Registers: backlog +
MASTER-INDEX rows; summary at `alpha/experiments/RUN-19-SUMMARY.md`.

## 2. Guardrails

1. Branch `claude/wasm-seal-run-19`; red then green commit per part (`test(wasm):` /
   `feat(wasm):`); summary shows order and grade per green (component / wasm-node /
   quic-native).

2. Honesty contract: every stand-in tagged `SPEC-DELTA[run19-…]` with a register row, same
   commit. The named expected stand-ins: `run19-node-runner` (wasm exercised under the
   wasm-bindgen Node test runner, not a real browser page — same module, different host) and,
   if croft-group's seal crate will not compile to wasm within the time-box,
   `run19-bare-openmls` (bare openmls + RustCrypto behind the same seal interface, with the
   croft-group gap filed as a backlog item, not fought).

3. Never edit the reviewed spec or frozen summaries; P6's doc changes ride the
   proposed-changes staging doc + reviews-log row.

4. The BROWSER itself is out of environmental reach (no display, no chromedriver assumed):
   attempt `wasm-pack test --headless --chrome` ONCE; on failure fall to the Node runner and
   tag. The QUIC leg uses a native Rust WebTransport client speaking the identical protocol a
   browser would; the literal browser page is a named non-goal for this environment, verified
   manually in product work. BLOCKED beats pretended.

5. Toolchain: add the wasm32-unknown-unknown target and wasm-bindgen/wasm-pack via the proxy;
   pin versions in the crate (the WebTransport ecosystem is draft-tracking and server/client
   crate revisions must match — record the pinned pair). ~30 min time-box per stuck install;
   BLOCKED with the unblock line if the toolchain will not land.

6. No lexicon publication, no owner decisions (custody policy wording is drafted FOR review,
   not ratified), no production serving.

## 3. The queue

### P1 — MLS compiles AND RUNS in wasm (the missing-test-evidence part)

Red first (wasm-bindgen tests written against the seal interface), then: compile the seal
stack (croft-group L2a preferred; bare openmls + RustCrypto fallback per guardrail 2) to
wasm32-unknown-unknown with the `js` feature and run the suite under the Node runner: group
create, add member, seal/unseal roundtrip, application messages within an epoch, commit and
epoch roll, removed-member forward-blindness — all INSIDE the wasm module. Prediction-first
constants: expected ciphersuite availability (the MTI X25519/Ed25519 suite), getrandom/js
entropy behavior, and any API surface that differs under wasm; report CONFIRMED/DIVERGED.
PASS: the suite is green under wasm. This green is, to our knowledge, evidence the upstream
CI does not produce for this target; say so in the summary with the citation.

### P2 — Cross-build interop goldens (the correctness claim that matters)

Red first, then: ciphertext SEALED IN WASM is unsealed by the NATIVE build of the same stack,
and vice versa; a full transcript (adds, messages, commits, a removal) generated half-in-wasm
half-native folds to the identical group state on both sides, byte-compared via the canonical
encodings. FALSIFY: any asymmetry between the builds — that would be a stop-the-line finding
about the crypto provider under wasm, reported, not papered.

### P3 — State at rest, resumption, and eviction honesty

Red first, then: MLS state serialized through a storage trait to an encrypted-at-rest blob
(AES-GCM via the provider; the browser mapping — WebCrypto-wrapped key over IndexedDB/OPFS —
documented, with the trait's in-environment backing a file/Node KV shim, tagged
`run19-storage-shim`); kill the wasm host mid-conversation, reload from the blob, and the
member still decrypts the next epoch's traffic. EVICTION drill: destroy the blob entirely and
assert the documented recovery path — the member cannot self-restore (forward secrecy is not
overridden), and re-entry is a fresh add via Welcome, blind to the gap. Multi-writer hazard
named in the doc (single-writer MLS state; Web Locks/tab-leader for the product), not built.

### P4 — Real QUIC through the content-blind DS

Red first (predicted handshake, stream framing, cert-hash trust setup), then: a WebTransport
server (wtransport or h3 stack; pinned) extending the DS role — stores and offer-gates
OPAQUE ciphertext blobs per the EXP-B pattern, blindness at the dependency boundary — and a
NATIVE WebTransport client exchanging the P1 module's sealed messages with it over actual
QUIC on localhost (self-signed cert via certificate hashes, the browser-parity dev-trust
mechanism). Assert: member offered, non-member refused, server dependency graph contains no
unseal capability. Record a small latency/throughput number for the summary (not a benchmark,
a sanity magnitude).

### P5 — The full loop: two wasm members through QUIC

Red first, then the model in one motion: wasm member A (Node-hosted module) seals →
WebTransport/QUIC → blind DS store → offer-gated fetch by wasm member B → unseals in wasm;
a commit (add/remove) travels the same path and both members advance epochs; the removed
member's post-roll fetch still yields ciphertext it provably cannot decrypt (offering vs
reading, demonstrated across the wire). PASS: the sealed tier's browser-shaped architecture,
end to end, with the only substitutions being the two tagged host stand-ins.

### P6 — The doc consequence: the deferred sentence gets upgraded (staged)

The landed transports language (RUN-16 A.8 as merged) defers sealed-in-browser as "WASM MLS
plus a relay bridge; native apps for now." This run's evidence revises it: the browser sealed
client needs NO overlay bridge — MLS in wasm over WebTransport to the content-blind DS is
the path, with custody-shaped (not transport-shaped) caveats. Stage, never edit: append to
the proposed-changes doc a short amendment (the revised sentence; the Baseline support
matrix with dates — Safari 26.4 March 2026, Chrome 97, Edge 98, Firefox 114; the custody
posture paragraph: keys in wasm memory, XSS as the threat model, device-key delegation as
the blast-radius bound, revocation by attestation deletion; eviction = rejoin-via-Welcome;
single-writer/tab-leader; draft-status pinning note), reviews-log row same commit, and a
DRAFT custody-posture paragraph in the experiment README marked FOR OWNER REVIEW. Backlog
rows: browser-manual verification (the real page, product-side); croft-group wasm gap if the
fallback fired.

### P7 — Summary and registers

`RUN-19-SUMMARY.md`: red→green table with grades; predicted-vs-actual for P1/P4 predictions;
pinned toolchain and crate pairs; the P2 interop verdict stated as the run's headline; the
SPEC-DELTA register rows; measurements; the paragraph mapping parts to model sections so
evidence tags can cite `(evidence: …, RUN-19)`. Site gate green.

## 4. Explicit non-goals

No real browser page (one headless attempt, then the tagged Node runner; manual product-side
verification filed); no iroh (the point is its absence on this path); no custody-policy
ratification (drafted for review); no key-recovery/I9 work (eviction shows the boundary and
stops); no performance benchmarking beyond sanity magnitudes; no serving infrastructure (the
DS here is the experiment's blind store, not the RUN-15 kit); no lexicon publication. One
line each in the summary.
