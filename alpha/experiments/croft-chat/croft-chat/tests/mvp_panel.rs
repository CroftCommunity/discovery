//! **The truthful panel and the mute register, wired through the real App
//! (E117 P6, increment B).**
//!
//! Client-level pins over a real session and the real shell state machine:
//! the membership panel carries the fold's truth (a banned lineage renders
//! "admission voided", never a shorter list); the `/mute` command — the
//! lightest of E116's three registers — resolves a hex prefix, marks the
//! author's lines instead of dropping them, and persists as local truth
//! that survives an app restart and never rides the wire.

mod common;

use croft_chat::app::{App, Focus};
use croft_chat::input::Action;
use local_storage_projection::types::{PrincipalId, Role};
use social_graph_core::{Identity, RemovalKind, Session};

fn hex(p: &PrincipalId) -> String {
    p.as_bytes().iter().map(|b| format!("{b:02x}")).collect()
}

#[tokio::test]
async fn the_panel_tells_the_truth_and_mute_persists() {
    let dir = tempfile::tempdir().expect("dir");
    let id_o = Identity::from_seed([0x31; 32]);
    let id_b = Identity::from_seed([0x32; 32]);
    let id_c = Identity::from_seed([0x33; 32]);
    let b = PrincipalId::new(id_b.principal_id().0);
    let c = PrincipalId::new(id_c.principal_id().0);

    let store = dir.path().join("o.redb");
    let muted_file = dir.path().join("muted.txt");

    let group = {
        let sess = Session::open(&store, &id_o).expect("open");
        sess.trust_peer(id_o.device_id(), id_o.principal_id());
        let group = sess.create_group().await.expect("group");
        sess.add_member(&group, b, Role::Member).await.expect("add B");
        sess.add_member(&group, c, Role::Member).await.expect("add C");
        // C is banned: the §7.6.4 ceiling — the panel must SAY so.
        sess.propose_remove_member(&group, c, RemovalKind::Ban, vec![])
            .await
            .expect("ban C");
        sess.send_message(&group, "hello from O", None).await.expect("send");
        group
    };

    // ---- The panel: seated rows + the voided row, from the fold's truth.
    let sess = Session::open(&store, &id_o).expect("reopen");
    let mut app = App::new(sess).with_muted_file(muted_file.clone());
    app.refresh();
    app.select_group(group);

    let view = app.view();
    let rows = &view.members.rows;
    assert_eq!(rows.len(), 3, "O seated, B seated, C voided — never a shorter list");
    let c_row = rows
        .iter()
        .find(|r| r.principal == c)
        .expect("the banned lineage is ON the panel");
    assert_eq!(c_row.standing_label, "admission voided");
    assert!(rows
        .iter()
        .filter(|r| r.standing_label.is_empty())
        .count()
        == 2);

    // ---- The mute register: "/mute <hex-prefix of B>" from the input line.
    app.perform(Action::ToggleFocus).await;
    assert_eq!(app.focus(), Focus::Input);
    for ch in format!("/mute {}", &hex(&b)[..8]).chars() {
        app.perform(Action::Input(ch)).await;
    }
    app.perform(Action::Submit).await;

    let view = app.view();
    let b_row = view
        .members
        .rows
        .iter()
        .find(|r| r.principal == b)
        .expect("B on the panel");
    assert!(b_row.muted, "the mute marker rides B's row");
    assert!(view.draft.is_empty(), "the command cleared the draft");

    // Local truth persisted: hex lines in the muted file.
    let persisted = std::fs::read_to_string(&muted_file).expect("mute file written");
    assert!(persisted.contains(&hex(&b)), "B's principal persisted");

    // ---- Restart: the mute survives as seeded local truth.
    drop(app);
    let sess = Session::open(&store, &id_o).expect("reopen 2");
    let mut app = App::new(sess).with_muted_file(muted_file);
    app.refresh();
    app.select_group(group);
    let view = app.view();
    assert!(
        view.members.rows.iter().find(|r| r.principal == b).expect("B").muted,
        "the mute survived the restart"
    );
}
