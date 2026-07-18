# tier-proof — RUN-17: the group model, demonstrated end to end

**Goal.** Build and run the RUN-16 model (the specification under test): show the
tier system working *for real* — the open tier's one-signature self-registration,
the gated tier's two-sided membership facts with causal revocation, device-key
delegation, the sealed steward group, catalogue reconstructability, the
write-policy axis, the blinded roster, the delivery-roles rehearsal, and a
measured scale rehearsal. TDD red-first per part; predictions about live wire
behaviour written as constants before the first live call.

**Approach.** A self-contained Rust crate. One mechanism — a signed **envelope**
(canonical dag-cbor over scope + author + antecedents + payload; identity is
`H(envelope)`) — carries every typed **record** (genesis, self-registration,
request, grant, leave, revocation, role grant/revoke, device attestation,
supersession) and every message. A **fold** replays an ordered event stream into
a **catalogue** (both policy fields) and a **roster** as membership **intervals**
(`[grant, cut)`), verifying every signature before folding. Live atproto legs
(P2, P5) run against real bsky.social + Jetstream when `ATP_TEST_*` credentials
are present; without them the live leg reports **BLOCKED** (never pretended) and
a stand-in runs behind the *same* interface. The sealed steward group (P6) uses
the real croft-group `group-seal` MLS crate at loopback grade
(`steward-seal/`, its own openmls-carrying workspace).

**Result.** See [`../RUN-17-SUMMARY.md`](../RUN-17-SUMMARY.md) for the red→green
table, predicted-vs-actual, the SPEC-DELTA register, and the per-part mapping to
the RUN-16 model sections. Measured scale numbers are in
[`MEASUREMENTS.md`](MEASUREMENTS.md).

## Layout

| Path | Part | What |
|---|---|---|
| `src/canonical.rs` | P1 | deterministic (RFC 8949 §4.2.1) CBOR key ordering |
| `src/envelope.rs` · `src/identity.rs` | P1 | envelope seal/verify, `H(envelope)`, ed25519 `did:key` |
| `src/records.rs` | P1 | the record shapes (§A.2–A.5) |
| `src/source.rs` | P2/P5 | `RecordSource` interface; live-leg BLOCKED probe; `MemSource` stand-in |
| `src/fold.rs` | P2/P3/P4/P7 | catalogue + interval roster, gated grant/co-sign, causal cut, archive rebuild, transition banner |
| `src/relay.rs` | P3 | validate-before-relay + write-policy gate (§A.8) |
| `src/delegation.rs` | P5 | device-key attestation verify, event-driven revocation; `DidResolver` seam |
| `src/blind.rs` | P6 | blinded roster (`hash(salt‖DID)` commitments) |
| `src/roles.rs` · `src/bin/{ds,swarm_peer,convergence}.rs` | P8 | reconciliation, interval backfill, three co-hosted processes (`make roles-up`) |
| `src/scale.rs` · `src/bin/measure.rs` | P9 | 100k roster, log-N inclusion proof, verify throughput, churn curve |
| `steward-seal/` | P6 | real MLS steward group (openmls via croft-group `group-seal`): sealed reasoning → public verdict |
| `src/chain.rs` (+ `fold`/`relay` extensions) | B1–B5 (RUN-18) | per-author chaining in write-restricted scopes: chain heads + genesis anchors in the fold, the relay `Unchained` check, the reader-side detector (gap detection, the `complete as of` claim, three-way existence classification, the vanilla contrast) |
| `src/audit.rs` | B6 (RUN-18) | auditable reach: independent refold of the served roster count; unsupported assertions detected |

## Run it

```bash
cargo test                 # P1–P9 component/loopback/stand-in suite (fast)
cargo run --release --bin measure > MEASUREMENTS.md   # P9 numbers
make roles-up              # P8: three delivery-role processes, co-hosted
cd steward-seal && cargo test   # P6 real-MLS half (compiles openmls once)
```

## Grades (honesty contract)

- **component** — pure logic, no network: P1, P3, P7, P8 (matrix), P9a.
- **live** — real bsky.social + Jetstream: P2, P5. **BLOCKED here** (no
  credentials); the multi-party half runs behind the same interface against
  local keypairs, tagged `SPEC-DELTA[run17-… | declared-stand-in]`.
- **loopback** — croft-group MLS over an in-proc transport: P6 sealed half; P9b
  is a local epoch model (`SPEC-DELTA[run17-churn-model]`).

Every stand-in is a greppable `SPEC-DELTA[run17-…]` tag at its site and a row in
[`../SPEC-DIVERGENCE-REGISTER.md`](../SPEC-DIVERGENCE-REGISTER.md).

## RUN-18 extension — reception completeness + auditable reach (B1–B6)

The crate now also proves the GROUPS.md A.2 reception paragraph and the
PUBLICATIONS.md delta table (`../RUN-18-SUMMARY.md` for the red→green table):
**B1** chaining validation (fold + relay), **B2** gap detection/repair from the
chain alone, **B3** the honest tail claim (`complete as of <newest held>`,
never currency; the swarm path advances it), **B4** chaining × interval
composition, **B5** the never-existed / retracted / withheld three-way
distinction with the vanilla current-state contrast (live deletion BLOCKED,
`SPEC-DELTA[run18-retraction-local | stand-in]`), **B6** auditable reach.
All component grade; `cargo test` runs them with the P-suite.
