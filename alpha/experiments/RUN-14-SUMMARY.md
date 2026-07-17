# RUN-14 — Stellin AppView caller-identity spikes: service-auth serving, sealed offer-gating, the helper seam

`Run summary, 2026-07-17. Branch claude/experiments-run-14, from main at the RUN-13 merge (10fe6d3).
Three code experiments (EXP-A/B/C), each TDD red-first, sequenced by leverage. The through-line: the
one unproven capability behind every Stellin differentiator — the AppView has never known who its
caller is. No Part 2 or social-mapping edit; the evidence-target status-moves are STAGED in
proposed-changes-2026-07-experiment-reconciliation.md (RUN-14 section, PROPOSED, not landed). The
AppView-provisioned scope key was left untouched (stop rule 5b).`

## Environment preflight

| Check | Result |
|---|---|
| `rustc --version` | **1.94.1** (e408947bf 2026-03-25) — ≥ 1.94 ✅ |
| `wss://jetstream2.us-east.bsky.network` via proxy | reachable (HTTP 200 on the base) |
| `https://bsky.social` via proxy | reachable (`describeServer` 200) |
| `https://plc.directory` via proxy | reachable (302 base; DID docs resolve — P-A3 confirmed live) |
| `ATP_TEST_HANDLE` / `ATP_TEST_PASSWORD` | initially unset (P-A1/P-A2 BLOCKED); **credentials later supplied by the owner → the live token leg ran and P-A1/P-A2 CONFIRMED** (see the predicted-vs-actual table). Creds never entered code/fixtures/logs/commits — passed as env vars for one run only |
| Credentials in code/fixtures/logs/commits | none (fixture keys are deterministic in-test scalars) |

## Per-part status (red → green evidence table)

| Part | What | RED commit | GREEN commit | Verdict | SPEC-DELTA |
|---|---|---|---|---|---|
| EXP-A step 1 | `verify_service_jwt` — real ECDSA verify vs DID-doc key, distinct error variants | `dad1a25` | `3d1175f` | ✅ PASS (10 tests) | — |
| EXP-A step 2 | `app.stellin.getProfileView` viewer-gate (openToWork by verified recruiter) | `9ad42b7` | `58939be` | ✅ PASS (7 tests) | `run14-A2` |
| EXP-A step 3 | verified-read telemetry in the disposable index | `ad2805e` | `4b3845e` | ✅ PASS (11 tests total) | — |
| EXP-A step 4 | live confirmation (`authserve`) | — (live driver) | `18998cf` | ✅ PASS — P-A1/A2/A3 all CONFIRMED live (real token verified end-to-end) | `run14-A4` |
| EXP-B | sealed offer-gating (§H hybrid serve half), `sealed` bin | `73897af` | `61ee86a` | ✅ PASS (4 tests, `--features client-seal`) | — |
| EXP-C | the helper seam, crate `helper-seam` (real openmls) | `22dede0` | `9749e71` | ✅ PASS (3 tests) | — |

Clippy cleanup for EXP-A: `4e549d1`. Order discipline: every part's `test(run-14): red …` commit precedes its `feat(run-14): green …` commit (step 4 is a live driver whose acceptance is the live matrix, BLOCKED on creds; its testable pure helper is covered green in the step-1/step-4 suites).

## Predicted-vs-actual (EXP-A live predictions)

| # | Prediction | Verdict | Observed |
|---|---|---|---|
| **P-A1** | `getServiceAuth` returns `{ "token": <compact JWT> }` | **CONFIRMED (live)** | a real 3-segment compact JWT returned under the `token` field |
| **P-A2** | JWT claims include `iss`, `aud`, `exp`, and `lxm` when a method is requested | **CONFIRMED, with a divergence** | all four predicted claims present; `exp − iat = 60s` confirms "short, ~a minute". **DIVERGED (present-but-unmodeled):** the real token *also* carries `iat` (issued-at) and `jti` (a per-token nonce / JWT-id) that the prediction did not model — a real AppView should tolerate (and may want to check `jti` for replay) |
| **P-A3** | signature verifies against the `#atproto` method in the issuer's DID doc (secp256k1/p256), resolved via PLC over HTTPS | **CONFIRMED (live)** | `@bsky.app` → `did:plc:z72i7hdynmk6r22z27h6tvur`; `#atproto` = secp256k1, 33-byte compressed SEC1; real `zQ3sh…` multibase decoded by `decode_multikey` and round-tripped through the verifier |

**End-to-end, fully live.** With owner-supplied creds, a real bsky.social account created a session, called `getServiceAuth` (self-issued, `aud` = own DID per stand-in `run14-A4`, `lxm` = `app.stellin.getProfileView`), and the resulting **real** token was verified by `verify_service_jwt` against the issuer's **real** DID-document key — `LIVE VERIFY OK`. The verifier is now proven green against minted fixtures (both curves) AND a real issued token. The one remaining unproven leg is interactive OAuth/DPoP (the PWA client-login flow), which is a different mechanism (named non-goal).

## The two declared stand-ins (registered, not hidden)

| SPEC-DELTA | Kind | Site | Stands in for | Path back |
|---|---|---|---|---|
| `run14-A2` | declared-stand-in | `viewserve.rs` `recruiters` table | recruiter-**admission** governance (R7) | admission becomes a governed projection (§7.2 R7 membership); the verify+gate half is unchanged |
| `run14-A4` | declared-stand-in | `authserve.rs` service-DID `aud` | a provisioned Stellin service DID | provision a real `did:web:…` (domain purchase pending); no verifier change. **Dormant** — not exercised (creds unset) |

Both are rows in `SPEC-DIVERGENCE-REGISTER.md` (Active table). Neither weakens a proven mechanism.

## Named non-goals (honest gap register)

- **Interactive OAuth/DPoP — the PWA client-login leg.** Requires a browser hop this environment lacks; explicitly not attempted (attended-run territory). Recorded in `EXPERIMENT-BACKLOG.md §6b`. Distinct from atproto **service auth** (server-to-server), which EXP-A proves.
- **The AppView-provisioned scope key.** A design decision (how the audience scope key is provisioned, granted, rotated) — untouched per stop rule 5b. Backlog `§6b`, parked.

## What each experiment demonstrated

- **EXP-A** — the AppView learns its caller. A compact atproto service-auth JWT is verified with real ECDSA (k256/p256) against the issuer's `#atproto` DID-document key; `getProfileView` serves the recruiter-gated `openToWork` field only to a *verified* recruiter at a different employer; anonymous callers get the public view, malformed/wrong-`lxm` tokens get 401 (never a degraded view), and the generic `getRecord` route cannot leak the computed field; verified reads emit telemetry into the disposable index. **All three live predictions confirmed** (creds supplied): a real `getServiceAuth` token verified end-to-end against a real DID-doc key, with `iat`/`jti` observed beyond the predicted claim set.
- **EXP-B** — the §H hybrid **serve half**. A content-blind store offers ciphertext only to a *verified roster member*; non-member/anonymous/nonexistent-group are one indistinguishable 403. Blindness is a **compilation boundary**: `cargo tree` shows the seal/open AEAD crate absent from the `sealed` binary's default dependency graph, present only under `--features client-seal`; `SealedState` holds no key. Roster removal stops future offering while already-fetched ciphertext + a retained key still reads — §H's offering-vs-reading distinction made executable.
- **EXP-C** — the content-helper seam over **real MLS**. A helper admitted by a real Welcome (`group-seal`, croft-group L2a) decrypts group messages as any member does, normalizes them through the provenance-copied `NormalizedEvent` boundary, and feeds the *same* index/serve path a public source feeds (one search returns both). Revocation (`remove_member` + epoch roll) makes the helper forward-blind (MLS forward secrecy → no post-roll rows; pre-revocation rows remain). The helper holds no authority surface.

## Mapping back to the evidence targets (so the staging entries write themselves)

- **§H hybrid topology, serve half — `Design → experiment-earned`.** EXP-B demonstrates offer-gating against a real verified viewer identity (EXP-A), with content-blindness enforced at a compilation boundary and offering-vs-reading shown. Staged as proposed-changes **H-A** (`ready`). The confidentiality guarantee still rests on encryption (unchanged); what is newly earned is that an AppView gates *offering* by verified identity without holding the key.
- **social-mapping helper delegation — `Design → experiment-earned`.** EXP-C indexes group content by grant and goes forward-blind on revocation, over real openmls, with no authority. Staged as proposed-changes **H-B** (`ready`).
- **The unproven record stays honest.** Interactive OAuth/DPoP (the PWA client-login leg) is named as still-unproven and attended-run territory. Staged as proposed-changes **H-C** (`caveat`, `ready`); backlog `§6b`.

## How to reproduce

```bash
cd alpha/experiments/appview-validation
cargo test --lib serviceauth viewserve                 # EXP-A: 22 tests
cargo run  --bin authserve                             # EXP-A step 4: live P-A3 (P-A1/A2 need creds)
cargo test --lib --features client-seal sealed         # EXP-B: 4 tests
cargo tree -e no-dev -i chacha20poly1305               # EXP-B boundary: absent by default
cargo run  --bin sealed                                # EXP-B: server refuses anonymous (flat 403)

cd ../helper-seam
cargo test                                             # EXP-C: 3 tests on real openmls
```

## Remaining items and what unblocks them (one line each)

- **EXP-A P-A1/P-A2** — ✅ **now confirmed** (owner supplied creds; `authserve` self-issued with `aud`=own-DID, SPEC-DELTA `run14-A4`, and the real token verified end-to-end).
- **Interactive OAuth/DPoP** — still open; unblocks in an attended session with a browser (a different mechanism from service auth).
- **The AppView-provisioned scope key** — unblocks with an owner design decision (not a run).
