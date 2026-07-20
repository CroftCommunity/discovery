# RUN-FED-SHIM-01 summary — the fediverse-wire conformance shim

`Own-lane run (RUN-ATTEST-01 precedent), executed 2026-07-20 as a
sequenced follow-on to RUN-AP-01 (branch: claude/ap-ambassador-
receipt-lane-sjokor, restarted from main after PR #27 merged). Layer
placement: Croft product lane at alpha/experiments/fed-shim/ — a new
sub-project sibling to ap-ambassador. Sequencing gate: RUN-AP-01
merged (f0183dd). Retroactively upgrades RUN-AP-01's fixture leg to
a specimen-anchored shim leg.`

## Governing principle (canonical, from FED-SHIM.md §0)

**fed-shim is a WIRE-CONFORMANCE surface, not a Mastodon replica.**
Every wire behavior modeled is anchored to a captured specimen or a
spec citation; behaviors that can't be faithfully modeled (retry
queues, media pipelines, Sidekiq semantics, PG consistency,
Ruby-side quirks) are NOT approximated — they are firm non-goals
that the attended live leg closes. Silent partial fidelity is the
failure mode this charter is designed to prevent.

## What landed

- **`alpha/experiments/fed-shim/`** — new standalone crate. Pure
  workspace, own lockfile, no network deps. `rsa` (verify+sign,
  `pem` feature enabled for actor-document PEM), `sha2`, `base64`,
  `blake3`, `rand_chacha` as main deps; `ap-ambassador` as dev-dep
  only.
- **`FED-SHIM.md`** — the charter. §0 governing principle
  (wire-conformance ≠ replica). §1 fidelity table: 9 modeled
  capabilities, each row citing a specimen or spec section. §3
  firm non-goals (11 named categories). §4 attended live leg (two
  shapes — Mastodon docker-compose or GoToSocial single-binary).
  §6 declared stand-ins (RSA-1024 for test speed; static Date
  header; in-memory KeyResolver). §7 grade `Modeled`.
- **`tests/specimens/`** — five dated wire specimens with source
  citations (Mastodon `follow_serializer.rb`,
  `undo_follow_serializer.rb`, `delete_actor_serializer.rb`,
  `actor_serializer.rb`, `app/lib/request.rb`) + a README
  enforcing the "single source of truth" discipline.
- **Library surface** — `ShimActor::generate` (deterministic
  keypair + PEM + SPKI DER from `blake3(seed_id)`); `actor_document`
  (JSON-LD byte-fidelity); `build_follow` / `build_undo_follow` /
  `build_delete_actor` (activity JSON byte-fidelity); `build_inbox_post`
  (signed inbox POST byte-fidelity + determinism claim §1 row 9);
  `AcceptedKind` + `ShimAcceptError` (§1 row 7 firm accept-set).
- **`FINDINGS-FED-SHIM.md`** — scaffold; no findings this run
  (F-AP-2 was surfaced by the shim leg and filed under
  `ap-ambassador/FINDINGS-AP.md`).

**Status tag:** `Modeled`. Live-Mastodon round-trip is the attended
live leg (§4); the shim's own tests + the cross-implementation
roundtrip against `ap-ambassador::verify` are the current grade.

## Reuse (RUN-ATTEST-01's discipline honored)

- **`ap-ambassador::verify` — DEV-DEP ONLY.** The shim library does
  NOT depend on ap-ambassador; only its tests do. This keeps the
  shim usable outside the ap-ambassador world (e.g. by
  xmtp-ambassador later, if a shared verify surface materializes).
- **`ap-ambassador::verify::verify_ap_http_signature` reused
  UNCHANGED** — the roundtrip test (`tests/
  shim_ambassador_roundtrip.rs`) drives the ambassador's verify
  path against the shim's signed-request output.
- **Crypto pins** — `rsa 0.9`, `sha2 0.10`, `base64 0.22`,
  `blake3 1`, `rand_chacha 0.3`. Same versions ap-ambassador
  already used (no dep proliferation).
- **Deterministic RNG** — same pattern as ap-ambassador's
  fixtures.rs: `rand_chacha` seeded from `blake3(seed_id)`.

## Red → green evidence

RED capture digest (scratchpad `fed-shim-red.txt`):
sha256 `799870a4b2fe951b44f603f63fb4f771f8948b64863602df1e0973abe8a29f1a`
— 11 tests failed as designed (10 wire-conformance + 1 roundtrip).
RED commit: `de1ef52`.

GREEN capture: 11/11 green (10 wire_conformance + 1
shim_ambassador_roundtrip + 0 doctests).
GREEN commit: `9a3dd13`.

Retroactive shim-leg commit (adds 3 tests + F-AP-2 fix in
`ap-ambassador::verify::parse_json_object`): `0cda8d7`.

Combined test tally on the branch:
- `fed-shim`: 11/11 green (10 wire-conformance + 1 roundtrip).
- `ap-ambassador`: 40/40 green (37 previous + 3 shim_leg).
- Total: **51 tests green across two crates.**

## Fidelity discipline in practice — the two independent implementations

The wire-conformance claim rests on more than shim-vs-itself
self-consistency. `tests/shim_ambassador_roundtrip.rs` and
`ap-ambassador/tests/shim_leg.rs` both drive:

  fed-shim ShimActor
    ─(build_follow)─▶ Mastodon-shape JSON body
      ─(build_inbox_post)─▶ signed inbox POST (draft-cavage RSA-SHA256)
        ─(convert to SignedRequest)─▶
          ap-ambassador::verify::verify_ap_http_signature
            ─▶ Ok(VerifiedActivity)

fed-shim implements the SIGN direction. ap-ambassador implements the
VERIFY direction. They share ONLY the `rsa` / `sha2` / `base64` /
`serde` crate dependencies — no shared canonical-signing-string code,
no shared header-parser code. Their agreement on the wire bytes is
the load-bearing evidence.

## Test map

| Test file | Tests | Result |
|---|---|---|
| `fed-shim/tests/wire_conformance.rs` | T-FS1.1 ShimActor deterministic · T-FS1.2 PEM is SPKI · T-FS2.1 Follow specimen shape · T-FS2.2 Follow no extra fields · T-FS3.1 Undo Follow specimen shape · T-FS4.1 Delete(Actor) specimen shape · T-FS5.1 Actor document specimen shape · T-FS6.1 Signature header key-order · T-FS6.2 Digest = SHA-256(body) · T-FS6.3 Signed post is deterministic | 10/10 green |
| `fed-shim/tests/shim_ambassador_roundtrip.rs` | T-FS-RT.1 shim Follow verifies through ap-ambassador (two independent implementations agree on wire bytes) | 1/1 green |
| `ap-ambassador/tests/shim_leg.rs` (retroactive) | T-AP1S.1 shim Follow verifies + mints receipt · T-AP1S.2 shim Undo closes Follow interval · T-AP1S.3 shim Delete(Actor) redacts held receipts (surfaced F-AP-2) | 3/3 green |

## FIX vs FINDING classifications

- **FINDING (F-AP-2, filed against `ap-ambassador/FINDINGS-AP.md`)** —
  the AP JSON parser refused arrays; extended to skip balanced `[…]`
  as it already skipped balanced `{…}`. Surfaced by the shim leg's
  Delete(Actor) specimen (`to: [as:Public]`). The RUN-AP-01 fixture
  leg was fit-but-incomplete against real Mastodon shapes; a
  specimen-anchored leg surfaced it immediately. No status tag moves.

## Deviations from expected shape

1. **`rsa` needed `pem` feature.** Initial Cargo.toml didn't enable it;
   compile failure at `to_public_key_pem` → added feature. One-liner.
2. **The shim's determinism seed** (`blake3(key_id ‖ peer_inbox_url ‖
   date ‖ body)`) is a shim-specific claim, not a Mastodon-fidelity
   claim. Real Mastodon uses OS randomness for RSA-blinding. This is
   §1 row 9 in the charter and is why the roundtrip test is
   reproducible.
3. **RSA-1024 keys** for fixture speed. Not deployment-grade; declared
   as a stand-in (§6). The verify path is size-agnostic.

## OWNER-CALL / OC tags surfaced, NOT decided this run

- **FS-SHIM-OC-1** — sharedInbox emit. When a Delete(Actor) fans out
  to a follower set, real Mastodon batches via sharedInbox
  endpoints. The shim's `build_inbox_post` produces ONE POST per
  peer; sharedInbox awareness is a delivery-run concern (AP-OC-6).
- **FS-SHIM-OC-2** — additional AP kinds. `Announce`, `Like`,
  `Update`, `Note` — the shim returns `NotAShim` for all (§1 row 7).
  A future run may add them if downstream ambassadors need them (AP-OC-9).
- **FS-SHIM-OC-3** — algorithms beyond RSA-SHA256. Some fediverse
  servers now emit `hs2019` / Ed25519 signatures. Fixture leg is
  RSA-only; adding Ed25519 verify to fed-shim + ap-ambassador is a
  future run's scope.

## Definition of green — checklist

- Charter's §0 governing principle stated: **yes** (FED-SHIM.md).
- Every §1 row cites a specimen or spec section: **yes** (5
  specimens + 4 spec citations).
- §3 firm non-goals enumerated (no silent approximation): **yes** (11
  named categories).
- §4 attended live leg recorded (not built): **yes** (two shapes
  named + predictions).
- Red-first per part: **yes** (11 tests failing at
  RED commit `de1ef52`, all green at GREEN commit `9a3dd13`; RUN-AP-01
  shim leg additional at `0cda8d7`).
- Cargo test clean: **yes** — 11/11 fed-shim + 40/40 ap-ambassador
  (across both crates the branch's combined test tally is 51/51
  green, 0 doctests).
- Clippy clean on new code: **yes**.
- Reuse discipline: **yes** — ap-ambassador::verify path REUSED
  UNCHANGED as a dev-dep; the shim library itself does NOT depend on
  ap-ambassador.
- RUN-AP-01 leg upgraded: **yes** — `ap-ambassador/tests/shim_leg.rs`
  drives full inbound-flow (Follow → verify → mint; Undo → fold
  closes; Delete → redact) through specimen-anchored fed-shim bytes.
