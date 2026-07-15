# Removal ≠ Redaction (Phase 12)

Validates the subtle security property Phase 7 flagged: **removing a member is
not the same as revoking their access to history.**

```
cargo run
```

## What it shows

1. Group `{Alice, Bob, Mallory}` at epoch N; all derive the same epoch-N key and
   can read content encrypted under it.
2. **Remove Mallory → epoch N+1**, key rotates (Alice == Bob, ≠ epoch-N). Mallory's
   group is stale at epoch N; she keeps her epoch-N key and cannot obtain N+1.
3. **Forward secrecy**: Mallory cannot read content encrypted *after* removal
   (epoch N+1) — she lacks the new key. Bob can.
4. **The gap**: the epoch-N ciphertext is unchanged, and Mallory still holds the
   epoch-N key, so she **still reads the old content**. Removal alone did not
   revoke access to the past.
5. **Redaction mitigation**: a current member recovers the old content (with the
   epoch-N key they retain) and **re-encrypts it under the epoch-N+1 key**,
   replacing the stored ciphertext. Now Mallory cannot read the re-keyed copy;
   Bob can.

## Conclusions (the deliverable)

- **"Remove member" and "revoke access to history" are different operations.**
  - Removal (MLS epoch rotation) gives **forward secrecy** — no access to content
    encrypted *after* removal.
  - It does **not** redact the past — content already encrypted under epochs the
    member belonged to stays readable with the keys they retain.
- **Mitigation**: to revoke access to specific past content, current members must
  **re-encrypt it under the new key** and replace the stored ciphertext.
- **Hard limit (honest)**: re-encryption only controls the *stored* copy. If the
  ex-member already copied the plaintext or kept the old ciphertext + key, nothing
  can retract that — true deletion is impossible against an adversary who already
  had access.
- **Product implication**: a "delete from history" feature must (a) re-encrypt /
  rotate the stored data, and (b) be honest that it bounds *future* access to the
  stored copy, not past exposure. This shapes what "leave group and erase me" can
  truthfully promise.

## Resolved versions

rustc 1.94.1 · openmls 0.8.1 · chacha20poly1305 0.10.1.
