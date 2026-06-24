# Croft test corpus — the consolidated catalog

date: 2026-06-17

purpose: **one catalog of every test/scenario across all three repos**, so the whole corpus is
visible in a single place and can seed a scenario test bed (long-tail conformity + regression). This
is the *catalog* (what exists, where the code lives, which invariant it asserts). It is **not** the
status tracker and **not** the reasoning record:

- **status** (green-real / green-model / spec) → `proof-ledger.md` is the source of truth.
- **why each test was run, what it tells us** → `test-narrative.md` (33 entries).
- **conformance vectors definition** → `conformance-suite.md`; **model experiment specs** →
  `thinking/experiment-suite.md` (groups A–I).

When status and catalog disagree, the **ledger wins** — update this catalog to match.

## The three layers

```
 ┌─ discovery/ ─────────────  thinking + synthesis: specs, ledger, narrative, this catalog
 │
 ├─ Proofs/ ───────────────   durable proofs (Rust + TS model). The "keep" tier.
 │    lineage-groups/             core protocol: 133 #[test] fns, conformance vectors, TS model
 │    encrypted-local-first-atproto/   14 proof crates: identity / atproto / local-first slices
 │
 └─ experiments/ ──────────   code-forward spikes (live transport, media, meer, mobile)
      iroh/                       relay-lab + faithful-wire + media + meer + 15 dated run records
      (android-p2p-app, automerge-partial-reconstruction, encrypted-blob-share,
       public-roundtrip, appview-validation)
```

ID families currently tracked in the ledger:
`I1–I10 · E1.1–1.4 · E2.1–2.16 · E3.1–3.4 · E0/E5/E6/E7/E10/E11/E12 · AR1–AR6 · C1–C10 · MD-G5 ·
T1/T3/T5/T8/T9 · F2 · D3 · V1–V9 · S1–S4 · P0` — plus the spec'd-not-run **H1–H8 / I1–I8** (this round).

---

## A. Protocol invariants (I1–I10) — the trust properties

Asserted throughout the Rust + model suites; canonical list in `proof-ledger.md` §"Protocol invariants".
I1 genesis immutability · I2 threshold soundness · I3 provenance/standing from signed data alone ·
I4 forward-key linearity (one live epoch) · I5 deterministic survivor selection · I6 no silent
membership contradiction (hard-stop) · I7 history never corrupts · I8 backfill verifiability · I9
fold/unfold lossless · I10 convergence after no-conflict heal.

**New invariant helpers (this round, exercised by groups H/I):** INV-MEMBERSHIP-FRESH ·
INV-THRESHOLD-SATISFIABLE · INV-FLOOR-NO-BRICK · INV-ROLE-REVOCABLE (`experiment-suite.md`).

## B. Durable Rust proofs — `Proofs/lineage-groups/crates` (133 `#[test]` fns)

| Crate / test file | Families | Covers |
|---|---|---|
| `lineage-core/tests/e2_governance.rs` (6) | E2.1–2.8 | under-threshold reject, forged-genesis reject, partition heal, contradiction hard-stop, reformation, reconcile branches |
| `lineage-core/tests/e2_10_lineage_thresholds.rs` (3) | E2.9/E2.10 | multi-device fold; one-lineage-can't-make-quorum |
| `lineage-core/tests/authority.rs` + `adversarial.rs` + `conflict_corpus.rs` | E2.x / AR / C1–C10 | revoke-authority (k-of-n by lineage), adversarial passes, reconcile/conflict corpus |
| `lineage-core/tests/multidevice_adversarial.rs` + `multidevice_asymmetry.rs` (4) | MD / E2.9 | per-device keys under one lineage; device-count ≠ actor-count |
| `lineage-core/tests/ar2_malicious_sequencer.rs` (3) | AR2 | blind sequencer reorder / drop / inject |
| `lineage-core/tests/t3_threshold_checkpoint.rs` (4) | T3 / F2 | threshold-signed checkpoint; single-authority checkpoint rejected |
| `lineage-core/tests/t9_merkle_trust_proof.rs` (4) | T9 | offline Merkle inclusion proofs; forged leaf/path/root rejected |
| `lineage-history/tests/e2_history.rs` + `e2_12_self_sync.rs` | E2.12/2.13 | history retention; multi-device self-sync |
| `lineage-history/tests/ar3_backfill_dos.rs` + `backfill_adversarial.rs` | AR3 / I8 | bounded-cost backfill; forged-branch reject |
| `lineage-mls/tests/e1_crypto_feasibility.rs` (4) | E1.1–1.4 | PCS, external commit, fresh genesis, queued-remove rekey (openmls 0.8.1) |
| `lineage-mls/tests/t1_lineage_credential.rs` (6) | T1 | signed LineageClaim rides the real MLS leaf; forged claims rejected |
| `lineage-mls/tests/ar5_tree_scaling.rs` (1) | AR5 | embedded ratchet-tree = O(N) commits (characterization → broadcast tier disables it) |
| `lineage-iroh/tests/e3_end_to_end.rs` (5) | E3.1–3.4 | real iroh: NAT path, namespace shard converge, blind broker sees ciphertext+routing only |
| `lineage-sim/tests/phase0_trivial.rs` | P0 | scaffold / reproducibility |

## C. Conformance vectors — `Proofs/lineage-groups/conformance` (**66 pass / 0 fail**)

Language-neutral JSON emitted from the real `lineage-core`/`lineage-history` API; a second impl must
pass them. Categories present: **cat1** derivations (incl. §2 tagged wire derivations) · **cat2**
signing (+ one-bit-flip reject) · **cat3** fold · **cat5** revocation mechanics · **cat5b**
revoke-AUTHORITY (k-of-n Ed25519, lineage-counted) · **cat6** reconcile C1–C10 · **cat7** adversarial
AR-1/2/3/6 · **cat8** visibility V1–V9+S2 · **cat9** freshness E2.16. (`conformance-suite.md` for the
definition; `revocation-authority-PLACEHOLDER` is fillable now the co-sign ordering is decided.)

## D. TS model — `lineage-group-model` (the authoritative model runner)

V1–V9 visibility (regime born-at-genesis, outward-propagation-depth enforced by every verifier),
S1–S4 social safety (S2 structure-only-share unrepresentable; S3 quiet-membership / S4
multi-identity gated on G5), E2.14/E2.15 reconcile-as-branches, **E2.16a/b/c freshness**
(no-false-current). *green-model; the conformance cat8/cat9 vectors are emitted from here.*

## E. Live transport / media / meer — `experiments/iroh` (15 dated run records)

| Crate | Run record(s) | Covers |
|---|---|---|
| `relay-loadtest` | E0-memwall, E0-crosshost, E1-vertical, E2-placement, E3-namespace-sync, E5-cgroup-accounting, E6-tc-shaping, E7-placement-churn | relay capacity wall, placement authority, namespace converge, per-tenant cgroup billing/isolation, shaping degrades-visibly, re-home churn window |
| `altdrive-spike-faithful-sync` | **C-faithful-revoke-2026-06-17** | real k-of-n SignedOp over live iroh-gossip; AUTHORIZED accept / UNDER-THRESHOLD reject (MAC retired) |
| `mls-welcome-over-iroh` | **C-mls-welcome-2026-06-17** | real openmls Welcome over iroh → joiner derives same exporter secret + same lineage-folded standing |
| `media-sframe-spike` | E12-sframe-mls | SFrame keyed off MLS epoch; out-of-order decrypt + replay reject; revoke rotates epoch; blind SFU recovers zero plaintext |
| (RoQ datagram rig) | E10-roq-netem | datagram CC characterized: transparent under loss/delay; AIMD estimator backs off on RTT trend (TC-CC2); media/bulk flow isolation (TC-CC3) |
| `moq-lazy-spike` | E11-moq-lazy | broadcast: lazy egress, fan-out linear in N, blind relay, metadata-only admission |
| `altdrive-core` + meer | **E9-meer-tier0-2026-06-17** | blind Tier-0 meer: offline member syncs+converges 5/5; `payload_keys_held=0`; admission denies non-listed; export→import→re-home (anti-entrenchment) |
| `altdrive-spike-{iroh,gossip,irohdocs,lineage-sync}` | (sandbox-transcripts) | transport spikes feeding the faithful path |

(MD-G5 revocation-over-NAT lives with the transport spikes; the marker MAC is retired by C-faithful-revoke.)

## F. Identity / atproto / local-first — `Proofs/encrypted-local-first-atproto` (14 crates)

`identity-binding`, `binding-lifecycle`, `stable-record-identity` (PLC/DID resilience, T-series) ·
`concurrent-membership`, `membership-sequencer`, `removal-redaction` (membership + AR2 over atproto) ·
`encrypted-sync-slice`, `end-to-end-slice`, `local-first-lexicon-app` (encrypted local-first slices) ·
`public-private-split` (V3 republish boundary, T8) · `local-appview`, `jetstream-appview`,
`local-pds-bridge`, `live-bsky-validate` (appview + live Bluesky round-trip, T10 gated on egress).

## G. Other experiment spikes

`android-p2p-app` (T13 mobile, gated on iOS/Android build host) · `automerge-partial-reconstruction`
(roll-up / partial reconstruction, F/G scaling) · `encrypted-blob-share` · `public-roundtrip` ·
`appview-validation`.

## H. Planned — decided 2026-06-17, **specs written, not yet run** (`experiment-suite.md` groups H/I)

- **Group H — membership freshness (MEMBERSHIP-FRESH).** H1 cold-rejoin+corroboration · H2
  missed-membership-change · H3 stale-co-sign-rejected · H4 single-beacon-insufficiency · **H5
  fresh-but-wrong-partition→hard-stop** · H6 withholding-fail-safe · H7 stale-flood-fail-safe · H8
  partial-chain-sync. (H5/H6/H7 need a real transport.)
- **Group I — admin floor + role ladder (ADMIN FLOOR).** I1 satisfiability(solo⇒k=1) · I2
  genesis-roster-born-matured · I3 raise-only-when-n≥new_k · **I4 floor-reject+replace-not-remove** ·
  I5 no-downgrade-by-shrink · I6 legitimate-quorum-accepted(capture≠brick) · I7 role-ladder ·
  I8 ladder-generalizes-to-meer/geer(strip→detach, history+provenance preserved).
- **First to prove (unrecoverable-if-wrong): H5, I4.**

---

## The scenario / regression bed (forward-looking)

This catalog is also the **scenario source** for a driven test bed. The corpus partitions into what a
single client/harness can replay deterministically vs what needs the fabric:

```
 MODEL-FIRST (no boxes)          LIVE-FABRIC (node-1/2/3 per runbook)
 ──────────────────────          ───────────────────────────────────
 A invariants, B Rust units,     E relay-lab + faithful + media + meer,
 C conformance vectors,          H5/H6/H7 (partition + liveness attacks),
 D TS model (V/S/E2.16),         live MD-G5 / beacon-over-wire follow-ons
 H1–H4/H8, I1–I8
```

A CLI group-chat client (see session discussion) would sit on the green-real Rust stack
(`lineage-core` + `lineage-history` + `altdrive-spike-faithful-sync` + the Tier-0 meer) and let a
human or a script drive: create/join, send, add/remove under a threshold, raise/lower `k`, strip a
role, partition/heal, go-offline-and-rejoin. Each scenario maps to one or more rows above, so the same
script doubles as **regression** (re-run the corpus) and **conformity** (assert the spec'd outcome).
The long-tail comes from composing these primitives — the bed records every composed scenario as a
replayable fixture.
