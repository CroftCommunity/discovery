# Injected-Trait Contracts

`local_storage_projection` receives all external capabilities through five
traits defined in `src/traits.rs`.  This document states the exact semantics
each real implementation must satisfy, what each mock does and what it
deliberately simplifies, and which invariants the fold engine relies on.

---

## Verifier

```rust
pub trait Verifier: Send + Sync {
    fn verify(
        &self,
        device_id: &DeviceId,
        message: &[u8],
        signature: &[u8],
    ) -> Result<(), VerifyError>;
}
```

### Real-Implementation Semantics

- `verify` MUST return `Ok(())` if and only if `signature` was produced by the
  private key bound to `device_id` over `message`, under the agreed signing
  scheme (e.g. Ed25519).
- `verify` MUST be **deterministic**: the same `(device_id, message, signature)`
  triple must always return the same result.  The fold engine calls `verify`
  inside the validation path before any write; if the result were non-
  deterministic, replaying assertions during `rebuild()` could reach a different
  state than the original ingest.
- `verify` MUST return `VerifyError::UnknownDevice` if `device_id` is not
  registered in the verifier's key store.
- `verify` MUST return `VerifyError::InvalidSignature` if the device is known
  but the signature does not match.
- `verify` MUST NOT perform I/O that can block indefinitely or panic on
  well-formed inputs.
- `verify` is called on `canonical_bytes()` (i.e., the envelope bytes **without**
  the signature), never on `canonical_bytes_with_sig()`.

### What the Mock Does

`MockSigner` implements both `Signer` and `Verifier`.  It produces a 64-byte
deterministic "signature" by XOR-folding message bytes against its 32-byte key
and a position counter, then appending the bitwise complement.  Verification
re-derives the expected signature and compares byte-for-byte.

**Deliberate simplifications:**

- There is no asymmetric cryptography; the same key is used to sign and verify.
  A `MockSigner` can only verify signatures it produced itself.
- Key derivation is trivially guessable from the seed byte.
- The output is not collision-resistant; two distinct messages may produce the
  same signature if the XOR pattern happens to collide (unlikely in tests with
  short messages but not impossible).

### Invariants the Fold Relies On

- **Determinism**: `ingest` checks the signature before writing; `rebuild`
  re-runs the same check from stored bytes.  If `verify` returns different
  results for identical inputs across runs, `rebuild` may reject valid
  assertions.
- **Totality**: `verify` must not panic; a panic inside the validation path
  would violate invariant I5 (no partial writes on failure) because the redb
  write transaction would remain open.

---

## Signer

```rust
pub trait Signer: Send + Sync {
    fn sign(&self, message: &[u8]) -> Vec<u8>;
    fn device_id(&self) -> DeviceId;
}
```

### Real-Implementation Semantics

- `sign` MUST produce a byte string that `verify(device_id(), message, sig)`
  accepts as `Ok(())`.
- `device_id` MUST return the stable identifier bound to the signing key.  It
  MUST be the same value across calls and across process restarts.
- `sign` MUST NOT mutate state visible to `device_id` or `verify`.
- `sign` is called on `canonical_bytes()`, never on the full `canonical_bytes_with_sig()`.

### What the Mock Does

`MockSigner::from_seed(seed)` builds a key `[seed; 32]`.  `device_id()` returns
`DeviceId([seed; 32])`, making the device identity trivially predictable from
the seed.  The mock is `Clone` so it can be used as both a `Signer` and a
`Verifier` in the same test.

**Deliberate simplifications:**

- No private-key security; the key is the device identity.
- Signing is CPU-only and infallible; there is no HSM, keychain, or async path.

### Invariants the Fold Relies On

`Signer` is not used by the fold engine directly — the surface layer (`surface.rs`)
calls `signer.sign` when constructing outbound assertions.  The fold engine only
calls `Verifier`.  However, for round-trip correctness, the `Signer` used to
construct an assertion and the `Verifier` used to validate it must be paired
(i.e., they must agree on the signing scheme).

---

## CredentialResolver

```rust
pub trait CredentialResolver: Send + Sync {
    fn resolve(
        &self,
        device: &DeviceId,
        principal: &PrincipalId,
    ) -> Result<(), CredentialError>;
}
```

### Real-Implementation Semantics

- `resolve` MUST return `Ok(())` if the `(device, principal)` binding is
  currently active — i.e., the device is enrolled under that principal and
  neither has been revoked.
- `resolve` MUST return `CredentialError::NotFound` if the pair was never
  registered.
- `resolve` MUST return `CredentialError::Revoked` if the pair was registered
  but subsequently revoked.
- `resolve` MUST be **deterministic** for a given stored state: the same pair
  must return the same result within a single process run.  (Dynamic revocation
  between runs is acceptable; within a single `ingest` call it must not change.)
- `resolve` is called after signature verification and before authorization.  A
  `CredentialError` causes the fold to return `FoldError::CredentialInvalid`
  with zero writes (invariant I5).

### What the Mock Does

`MockCredentialResolver` maintains an in-memory `HashMap<(DeviceId, PrincipalId), bool>`.
Pairs inserted with `register()` return `Ok(())`; all other pairs return
`CredentialError::NotFound`.

**Deliberate simplifications:**

- There is no `Revoked` path; the mock only models the allow-list case.
- The backing store is not persistent; a new `MockCredentialResolver` starts
  empty.
- There is no TTL, no external lookup, and no async I/O.

### Invariants the Fold Relies On

- **Determinism within a fold call**: The fold must not observe a transition
  from `Ok` to `Err` (or vice versa) between the credential check and the write
  transaction.  Because `CredentialResolver` is called once per `ingest` call
  and the result is not re-checked inside the write transaction, this constraint
  is satisfied as long as the real implementation does not race-mutate its state
  concurrently with an in-flight `ingest`.

---

## LamportSource

```rust
pub trait LamportSource: Send + Sync {
    fn next_lamport(&self) -> u64;
}
```

### Real-Implementation Semantics

- `next_lamport` MUST return a value **strictly greater than** the last value
  it returned, even across concurrent calls from multiple threads.
- `next_lamport` MUST NOT repeat a value within the lifetime of a single device
  key.  Repeating a value would cause a `LamportViolation` error on the second
  assertion from the same device.
- `next_lamport` should ideally start at `1` or at `last_stored_lamport + 1`
  after a restart to avoid gaps.  Gaps are allowed (the fold only checks that
  new > stored max); they are not considered violations.
- The fold engine enforces per-device monotonicity at ingest time by scanning
  `AUTH_ASSERTIONS_BY_DEVICE` for the maximum lamport seen for the device.

### What the Mock Does

`MockLamportSource` wraps an `AtomicU64` starting at `1` and increments it with
`fetch_add(1, SeqCst)` on every call.  The first call returns `1`, the second
returns `2`, and so on.

**Deliberate simplifications:**

- The counter resets to `1` every time a new `MockLamportSource` is constructed.
  In tests that create multiple fold instances sharing the same database, this
  can cause lamport collisions across instances.  Tests work around this by
  tracking lamport values manually and constructing envelopes with explicit
  lamport values.
- There is no persistence across process restarts.

### Invariants the Fold Relies On

`LamportSource` is consumed by the surface layer (`LocalStore::next_lamport`) to
stamp outbound assertions.  The fold engine validates inbound Lamport values
against `AUTH_ASSERTIONS_BY_DEVICE`.  The invariant is:

> For any device D, the lamport value of every successfully ingested assertion
> from D must be strictly greater than all previously ingested lamport values
> for D.

If `LamportSource` ever produces a repeated or decreasing value, the second
assertion will be rejected with `FoldError::LamportViolation`.

---

## BlobPresence

```rust
pub trait BlobPresence: Send + Sync {
    fn is_present(&self, hash: &Hash) -> bool;
}
```

### Real-Implementation Semantics

- `is_present` MUST return `true` if the raw blob identified by `hash` is
  available in local storage and can be read without I/O failure.
- `is_present` MUST return `false` if the blob is not locally available,
  regardless of whether it exists remotely.
- `is_present` SHOULD be cheap; it is intended as a synchronous in-process
  check, not a network call.
- The fold engine does **not** call `BlobPresence::is_present` in the current
  implementation; blob presence is tracked separately in `STATE_BLOB_PRESENCE`
  via `request_fetch`.  `BlobPresence` is defined in `traits.rs` as a
  capability boundary for future use by validation rules that gate on blob
  availability.

### What the Mock Does

`MockBlobPresence` maintains a `HashSet<Hash>`.  Hashes inserted with `insert()`
return `true` from `is_present`; all others return `false`.

**Deliberate simplifications:**

- No I/O, no file-system check, no network.
- The set is in-memory and not persistent.

### Invariants the Fold Relies On

Currently none — `BlobPresence` is not called by the fold or surface layer in
this version.  Future validation rules that check `is_present` before accepting
an `AttachmentAdd` assertion would require:

- `is_present` must not panic.
- `is_present` must be consistent within a single `ingest` call (the blob must
  not disappear between the check and the write transaction).
