//! Consumer-path coverage of the fold's **I5 Vouch payload gate**
//! (`local_storage_projection::fold_derived::check_authorization`, the `Vouch` arm,
//! `fold_derived.rs:447-477`). A hand-crafted `Vouch` envelope is driven through the real
//! `DerivedFold` (the path `surface::LocalStore` builds), and the accept/reject decision is
//! read back through **folded state** — the derived VOUCHES edge surfaced by
//! `LocalStore::get_trust_signals` — never by inspecting the gate function or private tables.
//!
//! Serves: closes backlog §2d (the Vouch payload-validation coverage residual RUN-07's X3
//! sweep recorded — 10 justified-survivor mutants in `fold_derived.rs`'s Vouch checks, uncovered
//! by both suites). This is a coverage residual, not a spec claim: no status tag moves.
//!
//! Earns/bounds: not a spec-earning test — it pins the I5 Vouch payload gate against the RUN-07
//! mutation survivors (X3-AUTOMATED-SWEEP.md), so the 10 previously-justified Vouch survivors die
//! to the consumer suite. The R7 trust claim (§7.2) is unrelated and untouched.
//!
//! Wire (VouchPayload): `subject(32) || ctx_len(4, BE) || ctx_bytes || strength(1)`. The gate
//! rejects: `len < 37` (MalformedEnvelope, too short); `ctx_len == 0` (AuthorizationFailed, empty
//! context); `len < 32 + 4 + ctx_len + 1` (MalformedEnvelope, truncated); `strength > 2`
//! (AuthorizationFailed, invalid strength). Trailing bytes past `required` are accepted.

mod common;

use std::sync::Arc;

use common::{base, sign};
use local_storage_projection::fold_derived::{DerivedFold, FoldError, IngestResult};
use local_storage_projection::surface::{Directness, LocalStore, TrustSignal};
use local_storage_projection::tables::Db;
use local_storage_projection::types::{ContextTag, VouchStrength};
use local_storage_projection::{AssertionEnvelope, AssertionType, GroupId, PrincipalId};
use social_graph_core::{Ed25519Verifier, Identity, MonotonicLamport, RegistryCredentialResolver};

// --- payload builders -------------------------------------------------------

/// A well-formed VouchPayload: `subject || ctx_len(=ctx.len()) || ctx || strength`.
fn vouch_payload(subject: &PrincipalId, ctx: &[u8], strength: u8) -> Vec<u8> {
    let mut p = Vec::with_capacity(32 + 4 + ctx.len() + 1);
    p.extend_from_slice(subject.as_bytes());
    p.extend_from_slice(&(ctx.len() as u32).to_be_bytes());
    p.extend_from_slice(ctx);
    p.push(strength);
    p
}

/// A VouchPayload whose **declared** `ctx_len` may differ from the ctx bytes actually
/// supplied — lets a test build a truncated or over-declared context.
fn vouch_payload_declared(
    subject: &PrincipalId,
    declared_ctx_len: u32,
    ctx: &[u8],
    strength: u8,
) -> Vec<u8> {
    let mut p = Vec::with_capacity(32 + 4 + ctx.len() + 1);
    p.extend_from_slice(subject.as_bytes());
    p.extend_from_slice(&declared_ctx_len.to_be_bytes());
    p.extend_from_slice(ctx);
    p.push(strength);
    p
}

// --- harness ----------------------------------------------------------------

/// The fixed voucher (author). One seed, registered per ingest.
fn voucher() -> Identity {
    Identity::from_seed([0xA1; 32])
}

/// Sign a `Vouch` envelope for `author` over the zero (global) GroupId with `payload`.
fn vouch_env(author: &Identity, payload: Vec<u8>) -> AssertionEnvelope {
    sign(
        author,
        base(author, GroupId::new([0u8; 32]), AssertionType::Vouch, 1, vec![], payload),
    )
}

/// Ingest one envelope through a fresh `DerivedFold` (the consumer path
/// `surface::LocalStore` constructs) and return the fold outcome plus the shared db (and its
/// backing tempdir, kept alive by the caller), so folded state can be read back with
/// [`signals_for`].
fn ingest(
    author: &Identity,
    env: &AssertionEnvelope,
) -> (Result<IngestResult, FoldError>, Arc<Db>, tempfile::TempDir) {
    let dir = tempfile::tempdir().expect("tempdir");
    let db = Arc::new(Db::open(&dir.path().join("vouch.redb")).expect("open db"));
    let resolver = RegistryCredentialResolver::new();
    resolver.register(author.device_id(), author.principal_id());
    let fold = DerivedFold::new(Arc::clone(&db), Ed25519Verifier, resolver);
    let outcome = fold.ingest(env);
    (outcome, db, dir)
}

/// The derived trust signals inbound to `subject` — the **folded state** a Vouch produces.
/// Empty when the vouch did not fold (was rejected by the gate).
fn signals_for(db: &Arc<Db>, subject: &PrincipalId) -> Vec<TrustSignal> {
    let store = LocalStore::new(
        Arc::clone(db),
        Ed25519Verifier,
        RegistryCredentialResolver::new(),
        MonotonicLamport::new(),
        *subject,
    );
    store
        .get_trust_signals(subject, None)
        .expect("read trust signals")
        .signals
}

// ===========================================================================
// ACCEPT — a well-formed Vouch folds and is visible as a derived trust signal.
// ===========================================================================

#[test]
fn wellformed_vouch_folds_and_is_visible_in_trust_signals() {
    let author = voucher();
    let subject = PrincipalId::new([0x22; 32]);
    // strength 1 == Moderate, non-empty readable context.
    let env = vouch_env(&author, vouch_payload(&subject, b"engineering", 1));

    let (outcome, db, _dir) = ingest(&author, &env);
    assert!(outcome.is_ok(), "well-formed vouch must fold: {outcome:?}");

    let signals = signals_for(&db, &subject);
    assert_eq!(signals.len(), 1, "the accepted vouch must surface as one trust signal");
    let s = &signals[0];
    assert_eq!(s.voucher, PrincipalId::new(author.principal_id().0), "voucher is the author");
    assert_eq!(s.context, ContextTag("engineering".to_string()), "context folds through");
    assert!(matches!(s.strength, VouchStrength::Moderate), "strength byte 1 == Moderate");
    assert!(matches!(s.directness, Directness::Direct), "a first-hop vouch is Direct");
}

/// A valid vouch carrying **extra trailing bytes** past `required` is still accepted
/// (the gate bounds the minimum length, not the maximum). Pins `462 < with >`.
#[test]
fn oversized_trailing_bytes_are_accepted() {
    let author = voucher();
    let subject = PrincipalId::new([0x23; 32]);
    let mut payload = vouch_payload(&subject, b"ops", 2); // valid, len == required
    payload.extend_from_slice(&[0xDE, 0xAD, 0xBE, 0xEF]); // trailing slack

    let (outcome, db, _dir) = ingest(&author, &env_of(&author, payload));
    assert!(outcome.is_ok(), "trailing bytes past required must not be rejected: {outcome:?}");
    assert_eq!(signals_for(&db, &subject).len(), 1, "the oversized-but-valid vouch folds");
}

// ===========================================================================
// REJECT — malformed / empty-context / truncated / bad-strength do NOT fold.
// ===========================================================================

/// Empty declared context at exactly the minimum length (37): rejected as
/// **AuthorizationFailed** (empty context), *not* as too-short. Pins both `449` mutants
/// (`< with ==`, `< with <=`): each would reject this at 449 as MalformedEnvelope instead.
#[test]
fn empty_context_rejected_as_unauthorized_not_too_short() {
    let author = voucher();
    let subject = PrincipalId::new([0x24; 32]);
    // subject(32) || ctx_len=0 (4) || <no ctx> || strength(1)  => len 37.
    let payload = vouch_payload_declared(&subject, 0, b"", 1);
    assert_eq!(payload.len(), 37, "exercises the 449 minimum-length boundary");

    let (outcome, db, _dir) = ingest(&author, &env_of(&author, payload));
    assert!(
        matches!(outcome, Err(FoldError::AuthorizationFailed(_))),
        "empty context must reject as AuthorizationFailed (the 449 mutants reject it too early, as MalformedEnvelope): {outcome:?}"
    );
    assert!(signals_for(&db, &subject).is_empty(), "a rejected vouch does not fold");
}

/// A payload shorter than the 32-byte subject: rejected as too short, never indexing past
/// the buffer. Pins the min-length guard's protective role.
#[test]
fn too_short_payload_rejected() {
    let author = voucher();
    let subject = PrincipalId::new([0x25; 32]);
    let payload = vec![0u8; 20]; // < 32
    let (outcome, db, _dir) = ingest(&author, &env_of(&author, payload));
    assert!(
        matches!(outcome, Err(FoldError::MalformedEnvelope(_))),
        "a 20-byte payload must reject as MalformedEnvelope: {outcome:?}"
    );
    assert!(signals_for(&db, &subject).is_empty(), "rejected vouch does not fold");
}

/// A payload one byte short of the length its own `ctx_len` requires: rejected as truncated.
/// Pins the four `461` `required`-arithmetic mutants (`32 + 4 + ctx_len + 1`): any of them
/// shrinks `required` below the actual length, so the truncation check passes and the strength
/// read at `payload[36 + ctx_len]` runs off the end — a panic the clean gate refuses by
/// rejecting first.
#[test]
fn one_byte_short_of_declared_context_rejected() {
    let author = voucher();
    let subject = PrincipalId::new([0x26; 32]);
    // declare ctx_len 4, supply only "abc" (3) + strength => len = 32+4+3+1 = 40; required = 41.
    let payload = vouch_payload_declared(&subject, 4, b"abc", 1);
    assert_eq!(payload.len(), 40, "one byte short of the required 41");

    let (outcome, db, _dir) = ingest(&author, &env_of(&author, payload));
    assert!(
        matches!(outcome, Err(FoldError::MalformedEnvelope(_))),
        "a truncated context must reject as MalformedEnvelope: {outcome:?}"
    );
    assert!(signals_for(&db, &subject).is_empty(), "rejected vouch does not fold");
}

/// A context declared far larger than the bytes supplied: rejected as truncated.
#[test]
fn over_declared_context_length_rejected() {
    let author = voucher();
    let subject = PrincipalId::new([0x27; 32]);
    let payload = vouch_payload_declared(&subject, 100, b"tiny", 1); // required = 137, len = 41
    let (outcome, db, _dir) = ingest(&author, &env_of(&author, payload));
    assert!(
        matches!(outcome, Err(FoldError::MalformedEnvelope(_))),
        "an over-declared context must reject as MalformedEnvelope: {outcome:?}"
    );
    assert!(signals_for(&db, &subject).is_empty(), "rejected vouch does not fold");
}

/// A strength byte above the max (Strong == 2): rejected. Pins `470 > with <`, which would
/// instead reject the *valid* low strengths and admit this one.
#[test]
fn invalid_strength_byte_rejected() {
    let author = voucher();
    let subject = PrincipalId::new([0x28; 32]);
    let payload = vouch_payload(&subject, b"team", 5); // strength 5 > 2
    let (outcome, db, _dir) = ingest(&author, &env_of(&author, payload));
    assert!(
        matches!(outcome, Err(FoldError::AuthorizationFailed(_))),
        "strength byte 5 must reject as AuthorizationFailed: {outcome:?}"
    );
    assert!(signals_for(&db, &subject).is_empty(), "rejected vouch does not fold");
}

/// The strength byte is read at `payload[32 + 4 + ctx_len]`. Mutant `469:48` (`32 - 4`) reads
/// `payload[28 + ctx_len]` instead; with `ctx_len == 1` that is `payload[29]`, a subject byte we
/// set to `0xFF` — the mutant then sees an out-of-range strength and rejects a valid vouch.
#[test]
fn strength_index_offset_low_is_pinned() {
    let author = voucher();
    // subject[29] = 0xFF so the mis-indexed strength read (28 + ctx_len == 29) sees > 2.
    let mut sb = [0x2Au8; 32];
    sb[29] = 0xFF;
    let subject = PrincipalId::new(sb);
    let payload = vouch_payload(&subject, b"x", 1); // ctx_len 1, real strength at payload[37] = 1

    let (outcome, db, _dir) = ingest(&author, &env_of(&author, payload));
    assert!(
        outcome.is_ok(),
        "the clean gate reads strength at 36 + ctx_len (a valid 1) and accepts: {outcome:?}"
    );
    assert_eq!(signals_for(&db, &subject).len(), 1, "the valid vouch folds");
}

/// Mutant `469:52` (`4 - ctx_len`) reads `payload[36 - ctx_len]`; with a large `ctx_len` that
/// index underflows `usize` and panics — a divergence from the clean gate, which reads the real
/// strength at `36 + ctx_len` and accepts. A `ctx_len` of 40 places the real strength safely in
/// bounds while `36 - 40` underflows.
#[test]
fn strength_index_offset_high_is_pinned() {
    let author = voucher();
    let subject = PrincipalId::new([0x29; 32]);
    let ctx = vec![b'z'; 40];
    let payload = vouch_payload(&subject, &ctx, 1); // ctx_len 40, real strength at payload[76]

    let (outcome, db, _dir) = ingest(&author, &env_of(&author, payload));
    assert!(
        outcome.is_ok(),
        "the clean gate reads strength at 36 + ctx_len and accepts a valid strength: {outcome:?}"
    );
    assert_eq!(signals_for(&db, &subject).len(), 1, "the valid vouch folds");
}

// --- small helper: build+sign a Vouch envelope for a raw payload ------------

fn env_of(author: &Identity, payload: Vec<u8>) -> AssertionEnvelope {
    vouch_env(author, payload)
}
