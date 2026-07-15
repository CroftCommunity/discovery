# Verifiable Identity Binding (Phase 8)

Closes the **two-headed identity** issue surfaced in Phase 6: a group member's
MLS-derived `did:key` and their PDS `did:plc` were distinct identifiers for the
same principal with no cryptographic link. This binds them.

```
cargo run     # create -> verify -> reject forgery -> reject tampering
cargo test
```

## The binding

A single statement naming both DIDs is **mutually signed** — by the account key
*and* by the real MLS group signing key (openmls `SignatureKeyPair`, which for
Ed25519 produces raw ed25519 signatures, so `ed25519-dalek` verifies them
directly):

```
statement = "atproto-mls-identity-binding|v1|account=<did:plc>|group=<did:key>"
sig_account = sign(account_key,  statement)
sig_group   = sign(MLS_key,      statement)
```

Published as a record (`org.croftc.experiment.identity.binding`, carries
`$type`) in the account's repo.

## Why it's sound for an AppView

Verification needs only the **account's verification key** (which an AppView
gets from the `did:plc` DID document — the trust root); the **group key is
embedded in the `did:key` itself**, so the verifier extracts it from the binding.
Both signatures must check out over the statement naming both DIDs. Therefore,
given a record under `did:plc:X` plus this binding, an AppView proves the same
principal controls group key `did:key:Y` — with no trust beyond the DID document.

It's **bidirectional**, which is what defeats forgery: claiming a binding in
either direction requires a key the attacker doesn't have.

## Lifecycle (all checks PASS)

1+2. A valid binding verifies (both signatures).
3. **Attack rejected** — Mallory forges a binding claiming Alice's `did:key` under her own account; she can sign with her own keys but not Alice's MLS key, so the group signature fails.
4. **Tampering rejected** — flipping a byte of the group signature fails verification.
5. The cross-identity claim is provable end to end.

## Issues surfaced

1. **Trust root is the DID document.** The verifier must still resolve the
   `did:plc` (plc.directory) to obtain the account key; the binding proves the
   link *given* that key. (Egress-blocked here, but it's a standard resolve.)
2. **No revocation/expiry.** A compromised key needs the binding revocable —
   add `notBefore`/`notAfter` and a tombstone record. Not modeled.
3. **Discovery.** The binding must be published in the account's repo and an
   AppView must fetch + check it before trusting any cross-identity claim.
4. **`did:key` form.** We use reversible `did:key:z<hex>`; real atproto uses
   multibase/multicodec (`did:key:z6Mk…`), equally reversible — swap the codec.

## Resolved versions

rustc 1.94.1 · openmls 0.8.1 · ed25519-dalek 2.2.0 · serde_json 1.0.150.
