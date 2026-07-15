# Binding Lifecycle: Expiry, Revocation, Rotation (Phase 11)

Closes the Phase 8 gap (a binding with no expiry or revocation). Adds the
controls a real identity system needs, all cryptographically enforced.

```
cargo run
```

## What's added

- **Validity window** — the binding carries `not_before`/`not_after`, and both
  are *inside the signed statement*, so they cannot be silently extended.
- **Revocation** — a separate account-signed revocation record supersedes a
  binding for `(account_did, group_did)` from an effective date; an AppView
  checks revocations before trusting a binding.
- **Key rotation** — a compromised MLS key is handled by revoking the old
  binding and issuing a fresh one for the new key.

`verify(binding, account_pub, now, revocations)` checks: both signatures over
the windowed statement, `now ∈ [not_before, not_after)`, and that no valid
revocation effective at/before `now` supersedes it. (atproto datetimes are
RFC-3339 UTC, so lexicographic compare == chronological.)

## Lifecycle (all 6 checks PASS)

1. Valid inside the window.
2. Expired (`now ≥ not_after`) → rejected.
3. Not yet valid (`now < not_before`) → rejected.
4. **Tamper** — extending `not_after` breaks the signature (the window is signed).
5. **Revocation** — a still-in-window binding is rejected from the revocation's effective date.
6. **Rotation** — after rotating the MLS key, the old binding is revoked/rejected and the new binding verifies.

## Issues surfaced / resolved

- **RESOLVED:** Phase 8's no-expiry/no-revocation gap — windows are signed and
  un-extendable; account-signed revocations supersede bindings.
- **Revocation discovery is the hard part.** An AppView must *find* revocations:
  a canonical, monotonic publish location (the account repo) plus a freshness/
  caching policy. A missed revocation = trusting a stale binding.
- **Clock trust.** Window and revocation checks assume a trusted `now`; skew or a
  lying clock weakens both. Real systems use signed timestamps / short windows.
- **Revocation authority.** Here revocations are account-signed; if the *account*
  key is compromised, revocation itself is at risk — real `did:plc` uses separate
  rotation keys precisely for this. A production design should sign revocations
  with a dedicated rotation key, not the everyday account key.

## Resolved versions

rustc 1.94.1 · openmls 0.8.1 · ed25519-dalek 2.2.0 · serde_json 1.0.150.
