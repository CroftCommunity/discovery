//! **The admission arc through the Session API (E117 P6, increment A).**
//!
//! The P4 machinery reaches the client surface: token issuance at join, the
//! §7.6.4 departure (standing intact), and the token return decided by the
//! CORE's `evaluate_admission` — the session assembles claims and context
//! from its own chain (issuance view derived from the log, standing from the
//! folded state) and deposits the Admission fact only when the decision
//! mints the approval. No MLS at this grade: the "commit" whose content
//! address identifies the admission event is the returner's opaque request
//! frame — the governance-plane arc, honestly labeled (the keylayer joins at
//! the product shells; the fold semantics are identical).
//!
//! Freshness is a caller input here (the HeadAck wiring is E112's standing
//! rung); the session documents it and the demo passes k. Modeled grade.

mod common;

use common::{has_member, replicate};
use local_storage_projection::types::{PrincipalId, Role};
use social_graph_core::{Identity, RemovalKind, Session, SessionError, TokenId};

fn setup_pair() -> (
    tempfile::TempDir,
    tempfile::TempDir,
    Session,
    Session,
    PrincipalId,
) {
    let dir_o = tempfile::tempdir().expect("dir o");
    let dir_b = tempfile::tempdir().expect("dir b");
    let id_o = Identity::from_seed([0x21; 32]);
    let id_b = Identity::from_seed([0x22; 32]);
    let b = PrincipalId::new(id_b.principal_id().0);
    let sess_o = Session::open(&dir_o.path().join("o.redb"), &id_o).expect("open O");
    let sess_b = Session::open(&dir_b.path().join("b.redb"), &id_b).expect("open B");
    for s in [&sess_o, &sess_b] {
        s.trust_peer(id_o.device_id(), id_o.principal_id());
        s.trust_peer(id_b.device_id(), id_b.principal_id());
    }
    (dir_o, dir_b, sess_o, sess_b, b)
}

/// **The full arc: join with an at-join token, depart, return by token —
/// and both stores converge on the re-opened span.**
#[tokio::test]
async fn token_return_arc_via_session_api() {
    let (_do, _db, sess_o, sess_b, b) = setup_pair();

    // O founds, enrols B, and mints B's re-entry token at join (§11.7
    // issuance setting 1) — a chain fact, replicated like any other.
    let group = sess_o.create_group().await.expect("create group");
    sess_o.add_member(&group, b, Role::Member).await.expect("add B");
    let token = TokenId::new([0x77; 32]);
    sess_o
        .issue_token(&group, token, b)
        .await
        .expect("at-join issuance");
    replicate(&sess_o, &sess_b, &group);
    assert!(has_member(&sess_b, &group, &b), "B is seated and knows it");

    // B departs — the §7.6.4 departure: self-authored, no quorum, standing
    // intact. O folds it.
    sess_b.depart(&group).await.expect("the exit floor");
    replicate(&sess_b, &sess_o, &group);
    assert!(!has_member(&sess_o, &group, &b), "B is off the hot roster at O");

    // B returns: an opaque request frame whose content address identifies
    // the admission event. O decides via the core and deposits the fact.
    let request = b"return request from B, riding the bus";
    sess_o
        .admit_return(&group, request, token, b, 1)
        .await
        .expect("the cross-check admits B");
    replicate(&sess_o, &sess_b, &group);

    assert!(has_member(&sess_o, &group, &b), "the span re-opened at O");
    assert!(has_member(&sess_b, &group, &b), "and B's own fold agrees");
}

/// **Bytes are not facts, at the session surface too: a token never issued
/// on this chain admits no one.**
#[tokio::test]
async fn a_token_never_issued_never_admits() {
    let (_do, _db, sess_o, sess_b, b) = setup_pair();
    let group = sess_o.create_group().await.expect("create group");
    sess_o.add_member(&group, b, Role::Member).await.expect("add B");
    replicate(&sess_o, &sess_b, &group);
    sess_b.depart(&group).await.expect("depart");
    replicate(&sess_b, &sess_o, &group);

    let unissued = TokenId::new([0x99; 32]);
    let err = sess_o
        .admit_return(&group, b"request", unissued, b, 1)
        .await
        .expect_err("no issuance fact on the chain");
    assert!(
        matches!(err, SessionError::AdmissionRefused(_)),
        "typed refusal, not a storage error: {err:?}"
    );
    assert!(!has_member(&sess_o, &group, &b), "B stays out");
}

/// **The ceiling holds at the session surface: a banned lineage's genuine
/// token does not re-admit; only a readmission decision could.**
#[tokio::test]
async fn a_banned_returner_is_refused_on_the_ceiling() {
    let (_do, _db, sess_o, sess_b, b) = setup_pair();
    let group = sess_o.create_group().await.expect("create group");
    sess_o.add_member(&group, b, Role::Member).await.expect("add B");
    let token = TokenId::new([0x77; 32]);
    sess_o.issue_token(&group, token, b).await.expect("issue");
    replicate(&sess_o, &sess_b, &group);

    // O bans B (threshold 1 in this fixture — the artifact kind is what
    // stamps the ceiling, not the quorum size).
    sess_o
        .propose_remove_member(&group, b, RemovalKind::Ban, vec![])
        .await
        .expect("ban");

    let err = sess_o
        .admit_return(&group, b"request", token, b, 1)
        .await
        .expect_err("the ceiling refuses");
    assert!(matches!(err, SessionError::AdmissionRefused(_)));
    assert!(!has_member(&sess_o, &group, &b), "banned stays out");
}
