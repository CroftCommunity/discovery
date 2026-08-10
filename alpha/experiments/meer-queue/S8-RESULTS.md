# S8 — object sizes against the 2 MiB cap

`Run: 2026-08-10, release mode, 49.6 s` · `Code: tests/s8_object_sizes.rs`, `src/mls.rs::measure_group`
· `Rung: A (real-lib)`

**Reproduce** (the sweep is `#[ignore]`d by default — the N = 8000 rungs are ~50 s in release and
minutes in debug, which would make the ordinary suite unusable):

```
cargo test --release --test s8_object_sizes -- --ignored --nocapture --test-threads=1
```

```
rustc 1.97.1 · openmls =0.8.1 · openmls_rust_crypto =0.5.1
openmls_basic_credential =0.5.0 · openmls_traits =0.5.0 · tls_codec 0.4
MAX_OBJECT_BYTES = 2 097 152 (2 MiB), refused on put and get; axum DefaultBodyLimit at the same figure.
```

**The sweep ran to the cap.** It stopped at N = 8000 because objects crossed 2 MiB there, not because
a budget ran out — Open Question 3 pre-authorised the full run with no time ceiling.

## Measured

| N | ext | app_msg | commit_add | commit_update | commit_remove | welcome | group_info |
|---:|:--:|---:|---:|---:|---:|---:|---:|
| 2 | off | 181 | 688 | 492 | 349 | 378 | – |
| 2 | ON | 181 | 688 | 492 | 349 | 792 | 650 |
| 10 | off | 181 | 3 023 | 1 253 | 1 143 | 1 594 | – |
| 10 | ON | 181 | 3 023 | 1 253 | 1 143 | 3 612 | 2 254 |
| 50 | off | 181 | 14 291 | 4 603 | 4 493 | 7 674 | – |
| 50 | ON | 181 | 14 291 | 4 603 | 4 493 | 16 868 | 9 430 |
| 200 | off | 181 | 56 463 | 16 977 | 16 867 | 30 476 | – |
| 200 | ON | 181 | 56 463 | 16 977 | 16 867 | 66 314 | 36 072 |
| 500 | off | 181 | 140 797 | 41 614 | 41 504 | 76 076 | – |
| 500 | ON | 181 | 140 797 | 41 614 | 41 504 | 165 082 | 89 240 |
| 1000 | off | 181 | 281 331 | 82 651 | 82 541 | 152 076 | – |
| 1000 | ON | 181 | 281 331 | 82 651 | 82 541 | 329 650 | 177 808 |
| 2000 | off | 181 | 563 365 | 164 688 | 164 578 | 304 076 | – |
| 2000 | ON | 181 | 563 365 | 164 688 | 164 578 | 659 718 | 355 876 |
| 4000 | off | 181 | 1 127 399 | 328 725 | 328 615 | 608 076 | – |
| 4000 | ON | 181 | 1 127 399 | 328 725 | 328 615 | 1 319 786 | 711 944 |
| **8000** | off | 181 | **2 255 433** | 656 762 | 656 652 | 1 216 076 | – |
| **8000** | ON | 181 | **2 255 433** | 656 762 | 656 652 | **2 639 854** | 1 424 012 |

Bold = over the cap.

## Growth, and where each object crosses 2 MiB

Per-member cost is **stable from N = 200 onward** (e.g. welcome-off: 153.1 → 152.5 → 152.2 → 152.1 →
152.1 → 152.0), and the fit is essentially pure linear — `welcome_ON ≈ 330.02·N + 48`. So the
extrapolations below are grounded, not guesses.

| object | growth | B/member | crosses 2 MiB at |
|---|---|---:|---:|
| **application message** | **FLAT** | — (181 B total) | **never** |
| commit — self-update | linear | 82.1 | ≈ 25 540 |
| commit — remove | linear | 82.1 | ≈ 25 550 |
| `GroupInfo` (tree ON) | linear | 178.0 | ≈ 11 780 |
| `Welcome` (tree **off**) | linear | 152.0 | ≈ 13 790 |
| commit — add-all | linear | 282.0 | ≈ 7 440 |
| **`Welcome` (tree ON)** | linear | **330.0** | **≈ 6 350 — first to cross** |

## What this settles

**1. The spike spec's "commit ~log N" is falsified — all three commit types are linear.**
`mls-replant` had already measured a *sparse self-update* commit at O(N); this extends that to **add**
and **remove**, and pins the rates: 82 B/member for update and remove, 282 B/member for the add-all
commit. Nothing here is logarithmic.

**2. Application messages never approach the cap.** Flat at 181 bytes from N = 2 to N = 8000. The
object the meer actually carries in steady state is nowhere near the constraint.

**3. The catastrophic branch is HALF off the table, and the half that survives matters.**

The spec pre-registered this falsification:

> **S8 shows application messages or ordinary commits crossing 2 MiB** → the cap is a general
> problem, not a `Welcome` problem, and CISS needs streaming before it can be the meer's substrate
> at all.

- **Application messages: never cross.** That half is closed.
- **Ordinary commits: do cross, at ≈ 25 500 members** (self-update and remove). The add-all commit
  crosses far earlier, at ≈ 7 440 — though that one is a group-*creation* artifact, not traffic.

So CISS does **not** need streaming to be the meer's substrate for conversational groups. It **would**
need something at broadcast scale — which is the tier §6.9.1 already treats separately.

**4. The tree extension roughly doubles `Welcome`, and shipping it out of band roughly doubles the
viable group size** — crossover moves from ≈ 6 350 to ≈ 13 790. That is the largest single lever
measured.

## The design decision this feeds

The spec offered three options. The measurement selects, and one of them turns out not to be a
change at all:

| option | verdict |
|---|---|
| **3 — ship the ratchet tree out of band** | **Already the status quo.** `mls_replant::stamp` returns `ratchet_tree` separately and `join()` takes it separately, so the corpus already does this — arrived at incidentally, not decided. Part 2 §6.9.1 already **mandates** it for the broadcast tier. It buys ~2× headroom for free. |
| **1 — `Welcome` out of scope for v0** | **Defensible and cheap.** With the tree out of band, `Welcome` crosses at ≈ 13 790; below that it is an ordinary object the meer can carry. Deferring it costs little because it is rare and usually deliverable to an online joiner. |
| **2 — transparent chunking** | **Not needed at conversational scale.** `ciss-sync` already has FastCDC chunking and a manifest, so the machinery exists — but nothing under ~6 000 members requires it, and M2 forbids the meer re-framing, so any chunking must reassemble byte-identically at the transport layer. Hold it for broadcast tier. |

**Recommended reading of the result: the cap binds at group sizes in the thousands, not at
conversational sizes.** For any group under ~6 000 members, no object the meer carries crosses 2 MiB.
Above that, the order of failure is: `Welcome`-with-tree (≈ 6 350) → add-all commit (≈ 7 440) →
`GroupInfo` (≈ 11 780) → `Welcome`-without-tree (≈ 13 790) → ordinary commits (≈ 25 500).

**This is the storage side re-deriving a constraint the protocol side already resolved.** §6.9.1
mandates disabling the embedded ratchet tree at broadcast scale "because at broadcast scale the
per-commit O(N) tree cost is the binding constraint." S8 arrives at the same boundary from the
opposite direction, and lands in the same place.

## Honest limits of this measurement

- **`Welcome (k joiners)` was not varied independently of N.** In this harness every non-planter is a
  joiner, so `k = N − 1` and the two dimensions move together. Separating them needs incremental adds
  and belongs to the substrate work, not the spike.
- **One ciphersuite** (`MLS_128_DHKEMX25519_AES128GCM_SHA256_Ed25519`). A larger signature scheme
  would raise per-member cost and move every crossover down.
- **`BasicCredential` only.** Real credentials (X.509, or lineage-bearing) are larger per leaf, which
  again moves crossovers down — plausibly a lot. **The figures here are a best case.**
