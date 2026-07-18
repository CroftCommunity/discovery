# RUN-17 — The tier proof: the group model built and run end to end, red-first

`Run summary, 2026-07-18. Branch claude/tier-proof-run-17-u31gt1, rebased onto main at the RUN-16 merge (913a31b — the model text this run puts under test is now in main). Rust 1.94.1. Experiment lands at alpha/experiments/tier-proof/ (+ steward-seal/ sub-crate for the P6 real-MLS half). TDD red-first per the standing directive: every part's acceptance criteria committed as failing tests before implementation; live-wire predictions written as constants before the first (blocked) live call. Live legs (P2, P5) are BLOCKED on ATP_TEST_* credentials this environment does not carry — reported, never pretended (guardrail 4); the multi-party halves run behind the same interfaces against local keypairs. The croft-group group-seal MLS crate builds cleanly via the proxy, so P6's sealed steward group runs against real openmls at loopback grade. 52 tests green across 11 test files. Stop rules honoured: no owner decisions taken (Variant A/B open; no boundary number; interval-widening asserted default).`

## Environment preflight

| Check | Result |
|---|---|
| `cargo` / `rustc` | 1.94.1 |
| `ATP_TEST_HANDLE` / `_PASSWORD` | **unset** → live legs BLOCKED (guardrail 4) |
| `ATP_TEST_HANDLE_2/_3` | unset → multi-party halves use local keypairs (stand-in) |
| croft-group `group-seal` (openmls) | **builds clean** via proxy → P6 sealed half runs loopback |
| Outbound egress to bsky.social/PLC | not exercised (no creds) |

## Per-part status (red → green evidence table)

| Part | What | RED | GREEN | Tests | Grade |
|---|---|---|---|---|---|
| P1 | envelope canonical dag-cbor, ed25519/`did:key`, `H(envelope)` identity goldens (§A.5) | `7438f7a` | `e775e72` | 7 | **component** |
| P2 | open tier: fold catalogue+roster, self-reg leave, backfill+tail reconstructability | `2f79333` | `71be607` | 5 | **component** (live BLOCKED) |
| P3 | write-policy axis (newsletter/forum) + validate-before-relay (§A.8) | `e738619` | `ff44601` | 6 | **component** |
| P4 | gated two-sided facts, co-sign threshold, causal revocation cut, silence≠verdict, intervals, archive rebuild (§A.3) | `8e062a5` | `4fe5cb9` | 7 | **component** (multiparty local) |
| P5 | device-key delegation verify + revoke-by-deletion (event-driven) (§A.5) | `e57932d` | `0d85847` | 6 | **component** (live BLOCKED) |
| P6a | blinded roster: unlinkability, self-proof, salt rotation, forward-blindness | `8f806d6` | `ba2a435` | 4 | **component** |
| P6b | real MLS steward group: sealed reasoning → public verdict (croft-group `group-seal`) | — (int.) | `10512b4` | 3 | **loopback** |
| P7 | tier transition as re-plant: continuous identity, historical registrations, banner | `dfd87d1` | `21e1722` | 3 | **component** |
| P8 | delivery roles: reconciliation + interval backfill + three co-hosted processes, failure isolation (§A.7) | `207fbce` | `9177a9a` | 7 | **component** |
| P9 | scale rehearsal: 100k roster, log-N proof, verify throughput, churn curve | `b5e82a9` | `1e93d33` | 4 | **component** (P9b model) |

_Grades (A.9 when-in-doubt: stay at the lower grade): **component** = pure logic,
no network · **loopback** = real MLS over in-proc transport · **live** = real
bsky.social + Jetstream (none reached this run — all BLOCKED, not downgraded
silently). P6b is integration-over-existing-crates (no new production `src`), so
its red state is "crate absent"; the blinded-roster half P6a carries the proper
red→green._

## Predicted-vs-actual (every live prediction, per guardrail 3)

| # | Prediction | Verdict | Observed |
|---|---|---|---|
| **P2-1** | self-registration `create` propagates as a Jetstream `commit` frame, `operation:"create"`, record under `commit.record` | **BLOCKED (live)** | no credentials; live call not made (guardrail 4). The frame shape is the RUN-14-proven path; unverified here. |
| **P2-2** | record `delete` propagates as a `commit` frame, `operation:"delete"`, `rkey` only, no body | **BLOCKED (live)** | as above |
| **P2-3** | the repo-commit signature chains to the author's DID-doc key → own signing proves self-registration | **CONFIRMED (component)** | the registration envelope verifies on the author's key alone, no second party (`own_signature_proves_self_registration…`) |
| **P5-1** | delegated device key is a `did:key` whose multibase body begins `z6Mk` (ed25519 = `0xed01`) | **CONFIRMED (component)** | `dev.did()` begins `did:key:z6Mk` |
| **P5-2** | DID-doc key encoding round-trips (reversible, canonical) | **CONFIRMED (component)** | `did_key_from_verifying(verifying_from_did_key(d)) == d` |
| **P5-3** | deleting the attestation rejects the next device envelope; the flip is event-driven | **CONFIRMED (component)** | the delete EVENT alone flips the verdict; no TTL path exists (`cache_invalidation_is_event_driven_not_ttl`) |
| **P5-live** | account resolves via a real `did:plc` document over the PLC directory | **BLOCKED (live)** | no egress/creds; `DidKeyResolver` stands in behind the same `DidResolver` interface |

No prediction DIVERGED; the ones that could not run are BLOCKED, not glossed.

## The declared stand-ins (registered, not hidden)

| SPEC-DELTA | Kind | Site | Stands in for | Path back |
|---|---|---|---|---|
| `run17-live-source` | declared-stand-in | `src/source.rs` `MemSource` / `live_open_tier_leg` | live Jetstream firehose ingest (RUN-14 path) | swap `MemSource` for the live source behind `RecordSource` when creds+egress land |
| `run17-multiparty` | declared-stand-in | `tests/p4_gated_tier.rs` (local steward/member keypairs) | genuinely multi-account steward/member/outsider | supply `ATP_TEST_*_2/_3`; the grant/request/revoke shapes are unchanged |
| `run17-did-resolver` | declared-stand-in | `src/delegation.rs` `DidKeyResolver` | real `did:plc` doc resolution over PLC | swap the resolver impl behind `DidResolver`; the verify+delegate logic is unchanged |
| `run17-swarm-local` | declared-stand-in | `src/bin/swarm_peer.rs` (local TCP) | the iroh overlay transport | a real two-peer iroh exchange upgrades the tag; the transport is behind a socket seam |
| `run17-rbsr` | declared-stand-in | `src/roles.rs` `converge` | the RUN-12 RBSR set-reconciliation | replace the hash-set diff with RBSR; the dedup-by-`H(envelope)` contract is unchanged |
| `run17-mls-loopback` | declared-stand-in | `steward-seal/tests/steward_group.rs` | MLS deliberation over a real network transport | the crypto is real openmls; only the transport is in-proc (croft-group harness convention) |
| `run17-churn-model` | declared-stand-in | `src/scale.rs` `simulate_churn` | the croft-group MLS churn harness at scale | drive real `group-seal` epoch rolls; epoch-roll cost is already a measured O(N) quantity |

All seven are `SPEC-DELTA[run17-… | declared-stand-in]` tags at the site and rows
in `SPEC-DIVERGENCE-REGISTER.md` (same change). None is a silent downgrade.

## Named non-goals (honest gap register — brief §4)

- **No real lexicon publication** — records use experiment-local NSID-equivalent
  collection names (guardrail 5); no schema records written.
- **No production serving / OVH / R2** — RUN-15's staged Phase 1.5/2 concern.
- **No Variant A/B decision** — the gated tier stays fork-agnostic behind the
  RUN-15 `GroupStore` shape.
- **No boundary-number decision** — P9 informs; the owner decides. No number chosen.
- **No interval-widening decision** — default-interval asserted (P4/P8), the dial
  is owner-governed.
- **No recovery / I9 / OAuth-client / WebTransport-in-browser** — WebTransport is
  a Phase-2/product concern (the DS serves plain sockets here, a named non-goal
  not a delta).
- **No real iroh required** — one build attempt was in-scope; the tagged
  local-socket stand-in is the swarm transport.
- **No edits to the reviewed spec** — this run touches only `alpha/experiments/`.

## What each part demonstrated (mapping to the RUN-16 model, for `(evidence: …, RUN-17)` citations)

- **§A.5 (identity)** — P1: `H(envelope)` is byte-stable, dedups two delivery
  paths, distinguishes same-payload/different-antecedents as two messages, and a
  re-signed copy changes the hash (the signature is *inside* the hashed bytes).
- **§A.2 + open tier** — P2: an open/open genesis + a one-signature
  self-registration fold to a catalogue + roster; own signing proves membership;
  delete = leave; catalogue+roster rebuild byte-identically from backfill+tail.
- **§A.8 (write policy)** — P3: newsletter(open/single) vs forum(open/open) from
  one machinery; validate-before-relay drops a forged envelope and never re-emits.
- **§A.3 (gated tier + intervals)** — P4: a steward grant citing the request hash
  admits from the grant's causal position; co-sign reaches a threshold; a
  revocation is a causal cut (before-cut messages verify, after-cut rejected);
  silence stays `pending`; leave ≠ revoke; membership intervals = grant→cut;
  archive rebuild re-verifies signatures.
- **§A.5 (delegation)** — P5: a device key is delegated by attestation, verified
  against the resolved DID + attestation, and revoked by deletion — event-driven.
- **P6** — the blinded roster (unlinkable to outsiders, self-provable by members,
  salt-rotatable, forward-blind) and a **real MLS** steward group whose sealed
  reasoning yields a public verdict shaped exactly like P4's grant.
- **P7** — a tier transition is a re-plant: one continuous catalogue identity,
  historical self-registrations preserved (not silently regraded), a plain banner.
- **§A.7 (delivery roles)** — P8: three co-hosted processes dedup by
  `H(envelope)` across transports; interval backfill offers exactly the proven
  window (offering-side refusal); sealed store is ciphertext-only; failure is
  isolated.
- **P9** — measured: ~20k single-core verifications/sec, a 544 B log-N
  light-client proof at 100k, and an ~linear epoch-roll curve. Numbers, no boundary.

## How to reproduce

```bash
cd alpha/experiments/tier-proof
cargo test                        # P1–P9 component/loopback/stand-in suite (~20s; P9 dominates)
cargo clippy --all-targets        # clean (pedantic-adjacent; type aliases, no unwrap in src)
cargo run --release --bin measure # regenerates MEASUREMENTS.md
make roles-up                     # P8: three delivery-role processes; kill one, two keep serving
cd steward-seal && cargo test     # P6 real-MLS half (compiles openmls once, then 3 tests)
# With credentials, the live legs run instead of BLOCKED:
ATP_TEST_HANDLE=… ATP_TEST_PASSWORD=… cargo test   # P2/P5 live probes flip to Ran
```

Live record identifiers: **none** — no live records were written (BLOCKED). When
the live legs run, their record URIs will be preserved here per guardrail 5.

## Remaining items and what unblocks them (one line each)

- **P2/P5 live legs** — set `ATP_TEST_*` (+ `_2/_3` for P4 multi-party); the
  interfaces are unchanged, so only the source/resolver impls swap.
- **P8 real iroh** — a clean iroh build via the proxy upgrades `run17-swarm-local`
  to a real two-peer exchange behind the same socket seam.
- **P9b real churn** — drive `group-seal` epoch rolls at scale to replace the
  local model; epoch-roll cost is already measured O(N).
- **Owner calls** — Variant A/B, the boundary number, the interval-widening dial:
  P9 informs, none is taken here.
