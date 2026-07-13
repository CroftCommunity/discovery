//! V4′ — the reconcile hard-stop fires on under-determination, not only
//! contradiction (Battery 6, Rung B).
//!
//! §7.6.1's escalation set has two members. P20 built the first — contradiction,
//! "too many valid claims" (two genesis → `ForkStatus::ForkedFrom` → banner). This
//! builds the second — under-determination, "too few": a required role vacant with
//! no admissible successor. Before this, `ForkStatus` was `{Clean, ForkedFrom}`, so
//! a group that lost its last Owner folded on *silently* as a headless group — the
//! exact failure a contradiction-only watcher makes.
//!
//! Scenario (through the real Replicator + Session + fold):
//!   O:  genesis (lamport 1)                 — O becomes Owner
//!   O:  MembershipAdd A2 as Admin (2)
//!   A2: MembershipRemove O (1)              — Admin removes the sole Owner
//!
//! After the removal the group holds one Admin and no Owner. No member can change
//! rules or grant roles (both require Owner), and none can promote a successor —
//! under-determined. The fold must hard-stop with a state distinct from both
//! `clean` and `forked_from`.
//!
//! Falsifies if: the group is reported `clean` (silent headless fold) or is only
//! representable as a fork. Corroborates if it reports the distinct
//! under-determination hard-stop.

mod common;

use common::{base, drive, frame, genesis_payload, membership_add_payload, sign};
use croft_chat::app::App;
use croft_chat::ui;
use local_storage_projection::types::envelope_hash;
use local_storage_projection::{AssertionType, DeviceId, GroupId, PrincipalId};
use ratatui::backend::TestBackend;
use ratatui::Terminal;
use social_graph_core::Identity;

/// MembershipRemove payload: subject principal(32).
fn remove_payload(subject: PrincipalId) -> Vec<u8> {
    subject.as_bytes().to_vec()
}

#[tokio::test]
async fn under_determination_hard_stops() {
    let dir = tempfile::tempdir().expect("tempdir");
    let group = GroupId::new([0xAD; 32]);

    let id_o = Identity::from_seed([0x60; 32]); // owner
    let id_a2 = Identity::from_seed([0x61; 32]); // admin
    let o_principal = PrincipalId::new(id_o.principal_id().0);
    let a2_principal = PrincipalId::new(id_a2.principal_id().0);
    let o_device = DeviceId::new(id_o.device_id().0);

    let genesis = sign(
        &id_o,
        base(&id_o, group, AssertionType::GroupGenesis, 1, vec![], genesis_payload(&o_device)),
    );
    let grant_a2 = sign(
        &id_o,
        base(&id_o, group, AssertionType::MembershipAdd, 2, vec![], membership_add_payload(a2_principal, 1)),
    );
    // A2 (Admin) removes O (the sole Owner). Authorized — MembershipRemove needs
    // Owner OR Admin — and it leaves the group with no Owner.
    let remove_o = sign(
        &id_a2,
        base(
            &id_a2,
            group,
            AssertionType::MembershipRemove,
            1,
            vec![envelope_hash(&grant_a2)],
            remove_payload(o_principal),
        ),
    );

    let authors = [&id_o, &id_a2];

    // Baseline: before the removal the group is clean and O is Owner.
    let sess_before = drive(
        &dir.path().join("before.redb"),
        &id_o,
        &authors,
        vec![frame(&genesis), frame(&grant_a2)],
    );
    let sum_before = sess_before.get_group_summary(&group).expect("summary before");
    assert_eq!(sum_before.fork_status, "clean", "with an Owner present, the group is clean");

    // After the removal: no Owner remains → under-determined hard-stop.
    let sess_after = drive(
        &dir.path().join("after.redb"),
        &id_o,
        &authors,
        vec![frame(&genesis), frame(&grant_a2), frame(&remove_o)],
    );
    let sum_after = sess_after.get_group_summary(&group).expect("summary after");

    // The Owner is gone, an Admin remains — the group is not empty, just headless.
    assert!(
        !sum_after.members.iter().any(|m| m.principal == o_principal),
        "O was removed"
    );
    assert!(
        sum_after.members.iter().any(|m| m.principal == a2_principal),
        "A2 (Admin) remains — the group is headless, not empty"
    );

    // THE HARD-STOP: distinct from both clean and forked_from.
    assert_eq!(
        sum_after.fork_status, "under_determined",
        "a required role (Owner) is vacant with no admissible successor — the fold \
         must hard-stop on the 'too few' escalation shape, not fold on silently"
    );
    assert_ne!(sum_after.fork_status, "clean", "must not silently continue headless");
    assert!(
        !sum_after.fork_status.starts_with("forked_from"),
        "under-determination is a distinct shape, not a fork"
    );

    // The app surfaces the legible picture: a blocking banner naming the shape,
    // not the fork wording (P20 symmetry, distinct headline).
    let mut app = App::new(sess_after);
    app.select_group(group);
    let backend = TestBackend::new(90, 14);
    let mut terminal = Terminal::new(backend).expect("terminal");
    terminal
        .draw(|frame| ui::draw(frame, &app.view(), app.focus()))
        .expect("draw");
    let text = ui::buffer_to_string(terminal.backend().buffer());
    assert!(
        text.contains("UNDER-DETERMINED") && text.contains("convergence halted"),
        "app must render the under-determination hard-stop banner: {text}"
    );
    assert!(
        !text.contains("FORK DETECTED"),
        "under-determination must not be mislabelled as a fork"
    );

    eprintln!(
        "V4′ RESULT (corroboration): removing the sole Owner leaves a headless group \
         (Admin remains, no Owner); the fold reports fork_status=under_determined and \
         the app renders a distinct hard-stop banner — the second §7.6.1 escalation \
         shape. A contradiction-only watcher would have reported 'clean' and folded on."
    );
}
