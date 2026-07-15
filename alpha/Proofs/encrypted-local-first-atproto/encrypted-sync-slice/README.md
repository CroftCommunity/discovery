# Encrypted Local-First Sync — Minimal Vertical Slice

A smallest-possible Rust program that proves an encrypted, local-first
group-sync architecture end to end. This is a **vertical slice**, not a
product: two/three simulated peers in a single process, exercising every
architectural seam once.

```
cargo run     # runs the full lifecycle and prints a pass/fail summary
cargo test    # unit tests for the Willow conflict-resolution comparator
```

## The four layers (kept strictly separate)

| Concern | Crate / approach | Status |
|---|---|---|
| Transport + content storage | BLAKE3 content-addressed store (`store.rs`) | **stub** (real BLAKE3 addressing; QUIC transport simulated) |
| Identity + addressing | Willow-shaped 4-tuple `(namespace, subspace, path, timestamp)` (`address.rs`) | **real** |
| Group key management | `openmls` (MLS / RFC 9420), exporter secret → per-epoch content key (`mls.rs`) | **real** |
| Encryption | `chacha20poly1305` AEAD keyed by the exporter-derived content key (`crypto.rs`) | **real** |
| Document state (CRDT) | `automerge` 0.7 (`doc.rs`) | **real** |

Key design point: MLS application-message encryption is **not** used for stored
data. MLS manages only the rotating per-epoch group key; durable payloads are
encrypted by us with an AEAD keyed by MLS's *exporter secret*.

Member identity is unified: a member's MLS Ed25519 signature public key is used
directly as the `subspace` of the 4-tuple address.

## Lifecycle demonstrated

1. **Setup** — Alice creates the MLS group, adds Bob; both derive the *same*
   epoch-N content key independently (key fingerprints compared).
2. **Alice writes** — Automerge doc + message → `save()` snapshot → AEAD encrypt
   under epoch-N key → store addressed by the 4-tuple.
3. **Bob bootstraps** — fetch by address → decrypt with his independently-derived
   key → `load()` → reads `["hello from alice"]`.
4. **Incremental sync** — Bob appends, extracts changes since Alice's heads,
   encrypts/stores; Alice fetches, decrypts, `apply_changes` → sees both messages.
5. **Epoch rotation** — add Carol (single committer); epoch advances, content key
   rotates; Carol bootstraps from a fresh snapshot under the **new** epoch key.

Document completeness is probed via `get_missing_deps`, never by checking whether
the `messages` list is empty (an incomplete and a legitimately-empty doc both
read as empty).

## Resolved versions (from `cargo run`'s version report)

- `rustc` 1.94.1 (≥ 1.80 — the MSRV gate **passes**)
- `automerge` **0.7.4** — closes the prior version gap (a previous attempt was
  stuck on `automerge 0.6.1` / Rust 1.75)
- `openmls` 0.8.1, `openmls_rust_crypto` 0.5.1, `openmls_traits` 0.5.0,
  `openmls_basic_credential` 0.5.0
- `chacha20poly1305` 0.10.1, `blake3` 1.8.5
- `iroh` 0.98.2 / `iroh-blobs` 0.102.0 — *resolvable but not linked* (see below)

## Deviations from the build brief

- **iroh transport stubbed (permitted).** `iroh`/`iroh-blobs` pull a large async
  dependency tree with a high-churn API. Per the brief's explicit allowance, the
  transport is simulated with a shared in-process store. Content addressing is
  still **real** (BLAKE3 — the same primitive iroh's blob store uses). Swapping
  in real `iroh-blobs` + an `iroh::Endpoint` would not change any layer above the
  store. Current resolvable versions are reported, not linked.
- **`automerge` 0.7 vs. current stable 0.10.** Current stable is `0.10.0`, but
  the brief's #1 success criterion is literally closing the *0.7* gap, so the
  dependency is pinned to `0.7` (resolves to `0.7.4`).
- **`get_missing_deps`/`get_changes`/`get_heads` take `&mut self` on
  `AutoCommit` in 0.7.4**, not `&self`. The brief stated `get_missing_deps` takes
  `&self` in 0.7 — that applies to the lower-level `Automerge` type, not
  `AutoCommit`. `get_changes` does return *owned* `Vec<Change>` as the brief
  said. (See note in `doc.rs`.)
- **openmls companion-crate versions.** `openmls 0.8.1` requires the `0.5.x` line
  of `openmls_rust_crypto` (0.5.1), `openmls_traits` (0.5.0), and
  `openmls_basic_credential` (0.5.0); `cargo add` initially picked stale `0.4.x`,
  which produced a duplicate-`openmls_traits` trait-mismatch error.
- **Welcome extraction.** Both `MlsMessageOut::into_welcome` and
  `MlsMessageIn::into_welcome` are `#[cfg(test-utils)]` gated in 0.8.1. The
  production path serializes the Welcome to bytes, deserializes as
  `MlsMessageIn`, and uses the ungated `extract()` → `MlsMessageBodyIn::Welcome`.

## The two outcomes that matter most

1. Compiled and ran against **real `automerge 0.7.4` on Rust 1.94.1 (≥ 1.80)** —
   version gap closed.
2. The epoch content key derived **independently by each member matches**
   (epoch-N: Alice == Bob; epoch-N+1: Alice == Carol), it **rotates** across the
   membership change, and a **new member bootstraps correctly from a snapshot
   under the rotated key**. The private-group crypto core is validated end to end.
