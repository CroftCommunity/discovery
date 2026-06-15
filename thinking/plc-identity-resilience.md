# PLC / Identity Resilience: did method choice for the MLS root, and a validating PLC read-replica

**Context:** Our public side integrates with atproto; the same DID roots our private encrypted side (MLS credentials / group membership keyed to DID). This document covers two coupled decisions:

1. What DID method should back identities that serve as the MLS root.

2. How to build a validating PLC read-replica (offline archive) sized to whatever `did:plc` identities remain in the critical path after decision 1.

**Status:** Research + design. Crypto is delegated to existing crates rather than hand-rolled. The Rust sketch in the appendix is illustrative and has NOT been compiled (no toolchain in the authoring environment). Items not confirmed against current sources are marked `[UNVERIFIED]`.

---

## Part 1 — DID method choice for the MLS root

### What an MLS root actually requires

MLS group membership is long-lived, and add/remove/re-add of a member is a cryptographic operation, not a UI toggle. The properties the rooting DID needs, in priority order:

1. **Durable resolvability** — resolvable to current keys even if a third party disappears.

2. **Recoverable from key compromise** — rotate a leaked key without destroying the identity (destroying it orphans every MLS group that member is in).

3. **Portable across hosts** — changing PDS/provider doesn't change the identifier.

4. **Self-authenticating** — verifiable from a signed history, not from trusting a server.

### Scorecard

| Property | did:plc | did:web | did:webvh |
|---|---|---|---|
| Durable resolvability (no central directory) | Weak: needs PLC directory live for resolution | Strong: resolves from your domain `/.well-known/did.json` | Strong: resolves over the web like did:web |
| Recoverable from key compromise | Strong: 72h recovery window, priority-ordered rotation keys | None: no rotation/recovery mechanism | Restored: carries key + rotation history |
| Portable across hosts | Strong: identifier is host-independent | None: identifier IS the domain, cannot migrate | Strong: SCID + history move with you ("credible exit") |
| Self-authenticating | Strong: signed, hash-chained op-log | Weak: trust current domain control | Strong: verifiable history |
| atproto support today | Blessed method | Blessed method (hostname-level only) | `[UNVERIFIED]` — discussion, not confirmed shipped across PDS/relay/AppView |

### did:plc detail

Recovery is real and well-specified. There is a 72-hour recovery window in which a higher-priority (lower-index) rotation key can sign a new operation pointing at the last valid operation (the fork point), overriding an operation signed by a lower-priority key. Standard hardening: enroll a backup rotation key, held off-PDS by you, with higher priority than any key the PDS holds, so a compromised or hostile PDS cannot seize the identity.

Where did:plc fails *us* specifically: durable resolvability depends on the PLC directory. An archive (Part 2) mitigates **reads**. It cannot mitigate **writes** — new rotations/recoveries must append to the single canonical chain, and with PLC gone there is no authority the network agrees on to accept them. Under "PLC disappears," a did:plc identity is frozen: resolvable from archive, but unable to rotate keys. For a long-lived MLS root, frozen-and-unrotatable is a genuine failure mode.

Privacy note to communicate to users: the full PLC operation history is permanently public, including every past handle and PDS URL, not redactable even after deactivation. Any PII in handles is exposed forever.

### did:web detail

Excellent durable resolvability with no central directory. But the spec is explicit that did:web "does not provide a mechanism for migration or recovering from loss of control of the domain name." No rotation chain, no recovery window. The identifier is the domain; lose the domain (lapse, seizure, registrar action, DNS compromise) and the identity is unrecoverable, with every MLS group rooted on it orphaned. Acceptable only for a controlled set on a domain you operate, with eyes open. atproto supports hostname-level did:web only (no path-based DIDs); same TLD restrictions as handles.

### did:webvh detail (the option that fits best)

did:webvh ("web + verifiable history") resolves over the web like did:web (no central directory in the path) but adds a self-certifying ID (SCID) plus a verifiable history of keys and rotations — restoring recoverability, portability, and self-authentication that plain did:web lacks. It was framed in the atproto discussion as a "credible exit" enabler: take your identifier and rotation history with you across services. Conceptually: did:plc's recoverable/portable/self-authenticating history, but resolved from a domain you control rather than from Bluesky's directory.

Caveat, marked clearly: `[UNVERIFIED]` native atproto support across PDS/relay/AppView. As of available sources this was a discussion, not confirmed shipped support; the two officially "blessed" methods are did:plc and did:web. Verify current stack support before committing. It also still roots the *current* endpoint in domain control, though the verifiable history softens the catastrophic-loss case versus plain did:web.

### Recommendation

For identities that root MLS membership and cannot be lost:

- **Control the namespace** (internal users, your domain, your PDS): prefer **did:webvh on a domain you operate** *if and when* atproto support is confirmed, because it removes the PLC dependency from the critical path while keeping key rotation/recovery. Until confirmed, did:web is the fallback only with full acceptance of the domain-loss risk.

- **Users bring their own identity / max network interop today:** stay on **did:plc with an independent high-priority backup rotation key held off-PDS**, plus the Part 2 archiver for read survival. Document that writes do not survive a full PLC disappearance.

- **Do not** use plain did:web for an MLS root unless domain-loss-equals-unrecoverable-identity is explicitly accepted.

Decide now: changing a member's DID after MLS groups exist forces re-establishing their credential across every group. The choice is effectively permanent per identity.

This resizes Part 2: if high-value identities are did:web(vh), the PLC archiver is no longer existential for our users and becomes purely about resolving outside-set did:plc identities we interact with.

---

## Part 2 — Validating PLC read-replica (offline archive)

Scope after Part 1: resolve the did:plc identities we depend on, survive a PLC outage for **reads**, and detect directory misbehavior (rollback/equivocation).

### Principle

Store **operations**, not resolved documents. The DID document is a fold over the signed op-log; the log is the trust root. A cache stores answers; a replica stores and re-validates the evidence. We want the replica.

### Data model

`plc_op` (the log):

- `cid` TEXT PK — operation content hash (primary identity of an op)

- `did` TEXT — subject DID

- `seq` BIGINT — sequence number from `/export` (pagination cursor + secondary unique id)

- `created_at` TIMESTAMP — advisory only, never trusted for ordering

- `prev_cid` TEXT NULL — CID this op chains from (null = genesis)

- `operation` BLOB — raw DAG-CBOR (or canonical JSON) of the op

- `sig` TEXT — signature

- `nullified` BOOL — set when a later valid recovery forks past this op

- `validated_at` TIMESTAMP NULL — when chain/sig validation passed

`plc_did_state` (derived fast-read path, rebuildable from `plc_op`):

- `did` TEXT PK

- `head_cid` TEXT — current valid tip after replay

- `doc_json` BLOB — resolved document (alsoKnownAs, verificationMethods, rotationKeys, services)

- `last_seq_applied` BIGINT

- `updated_at` TIMESTAMP

`sync_cursor` (resumable pagination):

- `id` INT PK CHECK(id=1) — single row

- `last_seq` BIGINT

Index `plc_op(did)` and `plc_op(prev_cid)` for replay; key inserts on `cid` for idempotency.

### Sync loop

Phase 1 — backfill: page `/export` by sequence number from 0, insert ops idempotently. Rate limits are generous; scraping the full log is explicitly expected not to hit them.

Phase 2 — live tail: hold the `/export/stream` websocket for real-time sync; on drop, fall back to polling `/export` from `last_seq`.

Correctness trap (from the Jan 2026 export update): operations can be first-seen in a different order than timestamp sort order, due to concurrency in the PLC service. Therefore:

- Never order or validate by timestamp. Order by the chain (`prev_cid`); use `seq` only as a pagination cursor, not a trusted total order.

- Insert idempotently on `cid`. An op whose `prev_cid` is not yet present goes to a pending set and is retried when its parent arrives. Do not assume parents precede children.

### Validation (what makes it a replica, not a cache)

Per the did:plc spec, before an op counts as valid:

- Signature verifies against a key in the rotationKeys set as-of the parent op's resulting state.

- Hash chain holds: `prev` points at the CID of the prior valid op.

- Fork/recovery rule: a competing op is a valid recovery only if signed by a lower-index (higher-priority) rotation key than the op it supersedes AND submitted within the 72h window. If valid, mark the superseded branch `nullified`; else reject.

- Rotation keys must be k256 or p256. Signing keys (verificationMethods) may be any did:key but confer no identity control unless also in rotationKeys.

Real-time verification (rejecting bad hashes/sigs/timestamps) is what the atproto "PLC Read Replicas" design (Feb 2026) specifies, and it is what catches a primary that deletes an update to roll back a DID: the replica retains both branches, so the rollback is detectable rather than silent.

Do not hand-roll the crypto. Wrap the Rust `atproto-identity` crate (genesis/update/tombstone op types with signing, verification, chain validation); `did-method-plc` is the reference implementation.

### Serving / routing

The resolver backend returns the same DID-document shape the client already consumes. TTL discipline per Bluesky guidance: max 24h for core identity metadata, ~5min for cached failures; bust the cache on an invalid repo signature (signals a signing-key rotation). Under a PLC outage, serve from `plc_did_state`, flag staleness, keep working for reads. Writes remain the known-unrecoverable part.

### Open items to verify before building

- `[UNVERIFIED]` current atproto did:webvh support across PDS/relay/AppView (affects whether Part 1's preferred option is available).

- `[UNVERIFIED]` whether Blacksky or another party runs a production validating PLC replica we could sync from instead of standing up our own.

- `[UNVERIFIED]` current PLC governance handoff status (spec states Bluesky is "enthusiastic about" moving governance/registry operation out of its sole control; no evidence it has happened yet).

---

## Appendix — illustrative Rust validator sketch (NOT compiled)

Delegates primitives to `atproto-identity` / `did-method-plc`. Types are simplified to show the fork/recovery check and the replay fold; field names are indicative, not API-accurate.

```rust
// ILLUSTRATIVE ONLY — not compiled, not API-accurate.
// Real signature/CID/chain primitives come from the atproto-identity crate.

use std::collections::HashMap;

/// A PLC operation as stored. `raw` is the canonical bytes the CID is computed over.
struct PlcOp {
    cid: String,
    did: String,
    seq: i64,
    prev_cid: Option<String>,
    raw: Vec<u8>,
    sig: String,
    // Parsed fields (from `raw`) relevant to identity control:
    rotation_keys: Vec<String>,      // did:key values, priority order (index 0 = highest)
    verification_methods: HashMap<String, String>,
    also_known_as: Vec<String>,
    services: HashMap<String, (String, String)>, // id -> (type, endpoint)
    created_at: String,              // advisory only
    is_tombstone: bool,
}

/// Resolved state after folding the valid chain.
struct DidState {
    head_cid: String,
    rotation_keys: Vec<String>,
    verification_methods: HashMap<String, String>,
    also_known_as: Vec<String>,
    services: HashMap<String, (String, String)>,
    deactivated: bool,
}

#[derive(Debug)]
enum ValidateError {
    BadSignature,
    BrokenChain,
    InvalidRotationKeyType,
    RecoveryWindowExpired,
    RecoveryNotHigherPriority,
    UnknownParent,
}

const RECOVERY_WINDOW_SECS: i64 = 72 * 3600;

/// Verify one op against the state produced by its parent.
/// `prev_state` is None only for a genesis op.
fn validate_op(
    op: &PlcOp,
    prev_state: Option<&DidState>,
) -> Result<(), ValidateError> {
    // 1. Chain linkage.
    match (&op.prev_cid, prev_state) {
        (None, None) => { /* genesis */ }
        (Some(prev), Some(state)) if *prev == state.head_cid => { /* ok */ }
        (Some(_), Some(_)) => return Err(ValidateError::BrokenChain),
        _ => return Err(ValidateError::UnknownParent),
    }

    // 2. Rotation-key types must be k256 or p256 (delegate the actual parse/check).
    if !op.rotation_keys.iter().all(is_k256_or_p256) {
        return Err(ValidateError::InvalidRotationKeyType);
    }

    // 3. Signature must verify against a rotation key valid in the PARENT state
    //    (genesis verifies against its own declared rotation keys).
    let signer_keys: &[String] = match prev_state {
        Some(s) => &s.rotation_keys,
        None => &op.rotation_keys,
    };
    let signer_index = verify_sig_return_key_index(&op.raw, &op.sig, signer_keys)
        .ok_or(ValidateError::BadSignature)?;

    // Record signer_index for the fork-resolution step (see resolve_fork).
    let _ = signer_index;
    Ok(())
}

/// Decide whether `challenger` legitimately supersedes `incumbent` at a fork.
/// Both share the same prev_cid. Returns Ok(true) if challenger wins (incumbent
/// branch should be nullified), Ok(false) if challenger is rejected.
fn resolve_fork(
    incumbent: &PlcOp,
    challenger: &PlcOp,
    parent_state: &DidState,
) -> Result<bool, ValidateError> {
    let incumbent_idx = signer_key_index(incumbent, &parent_state.rotation_keys)
        .ok_or(ValidateError::BadSignature)?;
    let challenger_idx = signer_key_index(challenger, &parent_state.rotation_keys)
        .ok_or(ValidateError::BadSignature)?;

    // Lower index = higher priority. Challenger must be strictly higher priority.
    if challenger_idx >= incumbent_idx {
        return Err(ValidateError::RecoveryNotHigherPriority);
    }
    // And within the 72h window measured from the op being invalidated.
    if seconds_between(&incumbent.created_at, &challenger.created_at)? > RECOVERY_WINDOW_SECS {
        return Err(ValidateError::RecoveryWindowExpired);
    }
    Ok(true)
}

/// Fold the validated chain for one DID into resolved state.
/// `ops_in_chain_order` must already be ordered by prev-linkage, not timestamp,
/// with nullified branches removed by resolve_fork.
fn replay(ops_in_chain_order: &[PlcOp]) -> Result<DidState, ValidateError> {
    let mut state: Option<DidState> = None;
    for op in ops_in_chain_order {
        validate_op(op, state.as_ref())?;
        state = Some(DidState {
            head_cid: op.cid.clone(),
            rotation_keys: op.rotation_keys.clone(),
            verification_methods: op.verification_methods.clone(),
            also_known_as: op.also_known_as.clone(),
            services: op.services.clone(),
            deactivated: op.is_tombstone,
        });
    }
    state.ok_or(ValidateError::BrokenChain)
}

// --- delegated to atproto-identity / did-method-plc (signatures shown for intent) ---
fn is_k256_or_p256(_did_key: &String) -> bool { unimplemented!() }
fn verify_sig_return_key_index(_raw: &[u8], _sig: &str, _keys: &[String]) -> Option<usize> { unimplemented!() }
fn signer_key_index(_op: &PlcOp, _keys: &[String]) -> Option<usize> { unimplemented!() }
fn seconds_between(_a: &str, _b: &str) -> Result<i64, ValidateError> { unimplemented!() }
```

Notes on the sketch:

- The fork/recovery check is the security-critical part: priority compared against the **parent** state's rotation-key ordering, and the 72h window measured from the op being invalidated. Get either wrong and you either accept an illegitimate takeover or reject a legitimate recovery.

- Ordering for `replay` comes from `prev_cid` linkage, never timestamps, consistent with the out-of-order delivery caveat.

- Everything cryptographic is left to the maintained crates on purpose. This file should not be the place secp256k1/P-256 verification or CID computation gets reimplemented.
