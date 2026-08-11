# meer-queue — Phase-0 spike for the MLS store-and-forward meer

`Status: RUN 2026-08-08/11. Both must-pass claims settled; all eight scenarios run.`
`Rung: A (real-lib) throughout, except S1 (Rung C, static inspection).`

Tests whether a mailbox that **does no ordering, holds no group state, and holds no key** is
sufficient to carry a real MLS conversation across an absence — against real OpenMLS, a real CISS
server, and a real iroh transport over a real relay.

| | |
|---|---|
| Spec | `SPIKE-SPEC.md` (M1, M2, S1–S8) |
| Plan | `../../plans/2026-08-07-2-plan-meer-queue-spike.md` |
| Hypothesis | `../../thinking/meer-as-custodian-queue.md` |
| Lane | `../../plans/2026-08-07-meer-lane.md` (this is Phase 0) |
| Bound by | `beta/impl/delivery-layer/08-experiment-methodology.md` (the fidelity ladder) |
| **Results** | **`TEST-LOG.md`** — verdicts with rungs · **`S8-RESULTS.md`** — the size sweep · **`PHASE-0-FINDINGS.md`** — discovery |

## The headline

**The central claim held.** M1 CONFIRMED (real-lib): a member offline for a message's entire live
window drained it from the meer and decrypted it through real `process_message`, with the meer
holding zero group keys. M2's positive arm CONFIRMED: bytes are byte-identical across store and
serve.

**Six subsidiary claims did not**, and each is now measured rather than assumed. In rough order of
consequence:

- **S7** — "the meer learns nothing" is false. `group_id`, `epoch` and `content_type` are cleartext
  in MLS framing, so a carrier can link messages by conversation with no key. (E96)
- **S5** — "it is gone after 14 days" is false. CISS has no object `DELETE`; sweeping ends *serving*,
  not *holding*. (E95)
- **S8** — the 2 MiB cap binds **in the thousands**, not at conversational sizes. Application
  messages are flat at 181 bytes; everything that grows is linear, not `~log N`. (closed)
- **S4** — naive deliver-once starves a second device, as predicted; and racing is nearly free, so
  the dependency on §6.6.5 buys less than the design implies. (E92)
- **S2** — dedup is **per-namespace**, so "stored once" holds for a pooled meer and fails for the
  per-DID default. The unconditional saving is transit. (E91)
- **S3** — MLS does **not** apply a duplicate idempotently; it errors. Dedup must precede
  processing. (E93)
- **S1** — enrollment is not two-party: senders need an announcement nobody has specified, and
  revocation is one-sided as a result. (E97)

**M2's negative arm was falsified before it was written** (Phase 0, D3): a re-frame is
*byte-identical*, so the spec's stated hazard was wrong. The `MUST` survives on stronger grounds —
the dangerous operation is re-*sealing*, which needs a key the meer lacks, and OpenMLS makes
re-framing unavailable in a production build anyway.

## Running it

```sh
cargo test                      # 34 tests, seconds
cargo clippy --all-targets      # clean

# M2's negative arm — deliberately enables openmls `test-utils` to construct the forbidden
# re-frame. Never on for the positive path.
cargo test --features reframe --test m2_byte_identity -- --nocapture

# S8's size sweep — #[ignore]d by default; ~50 s in release, minutes in debug.
cargo test --release --test s8_object_sizes -- --ignored --nocapture --test-threads=1

# The Phase-0 discovery probes (kept runnable as evidence).
cargo run --bin d1_ciss_dep      # cross-repo path dep to CISS
cargo run --bin d2_ciss_put      # real PUT/GET, the 2 MiB boundary, dedup, du
cargo run --bin d3_d4_mls        # real seal/open; the re-frame's absence
cargo run --bin d5_iroh          # real relay + endpoints + drain scope
cargo run --release --bin d6_scale d7_tree_ext
```

## Layout

| file | what |
|---|---|
| `src/ciss_harness.rs` | real CISS on loopback. `SPEC-DELTA[meer-spike-ciss-inproc]` |
| `src/mls.rs` | the seal — real OpenMLS. Also S8's config-parameterized construction |
| `src/meer.rs` | the five operations. **Names no MLS type** — that is M2's structural arm. `SPEC-DELTA[meer-spike-namespace, meer-spike-kind-gate]` |
| `src/queue.rs` | have/want diff, ack, sweep, watermark. `SPEC-DELTA[meer-spike-clock]` |
| `src/transport.rs` | real iroh. Drain scoped by `EndpointId`; **no recipient field on the wire**. `SPEC-DELTA[meer-spike-drain-auth]` |
| `src/relay.rs`, `src/node.rs` | copied from `../iroh/crates/mls-welcome-over-iroh`, ports made ephemeral |
| `tests/w0–w3` | wiring tests, one per layer |
| `tests/m1, m2` | the must-pass claims |
| `tests/s2–s8` | the shape-learning scenarios |

## Stand-ins

Five, all tagged in code and enumerated in `../SPEC-DIVERGENCE-REGISTER.md`. Correspondence is
checked: every tag has a row, every row has a tag. **Nothing about the seal is stood in.**

`meer-spike-namespace` · `meer-spike-kind-gate` · `meer-spike-drain-auth` · `meer-spike-clock` ·
`meer-spike-ciss-inproc`

## Reading the results honestly

- Every verdict states its **fidelity rung**. A bare `CONFIRMED` is inadmissible here.
- S6 is `CONFIRMED-WITH-STAND-IN` — it passes for a **weaker reason** than the design claims,
  because under `meer-spike-namespace` the mail lives in the meer's namespace, not the recipient's.
- S8's figures are a **best case**: one ciphersuite, `BasicCredential` only. Real credentials move
  every crossover down.
- S4's with-device-group arm is **not tested** and says so — §6.6.5 is not built, and standing in
  for it would substitute for the exact mechanism the claim is about.

## Followups

`ROADMAP_TODO.md` **E92** (device-group arm) · **E93** (Part 2 §6.6.2 corrections) · **E94**
(unlinkable deposit) · **E95** (CISS object lifecycle) · **E96** (nested sealing / group-linkability)
· **E97** (enrollment announcement + two-sided revocation). Lane-scoped findings are appended to
**E91**.
