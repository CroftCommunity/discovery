//! P20 — hard-stop on contradiction (Proof B), headless.
//!
//! Earns/bounds: Part 2 §7.6 / §7.6.1 — hard-stop on a Contradiction: no silent merge of two genesis claims for one group.
//!
//! Two devices each claim a genesis for the *same* group id — a contradictory
//! governance fact at the same gov slot. The substrate must flag the fork
//! (§7.6: no silent merge), `get_group_summary` must surface it, and the app must
//! render a blocking banner rather than presenting a silent winner.
//!
//! The fork is built through the public substrate + ed25519 adapters (the real
//! types), then a `Session`/`App` is opened over that store and rendered.

mod common;

use std::sync::Arc;

use common::signed_genesis;
use croft_chat::app::App;
use croft_chat::ui;
use local_storage_projection::fold_derived::DerivedFold;
use local_storage_projection::tables::Db;
use local_storage_projection::GroupId;
use ratatui::backend::TestBackend;
use ratatui::Terminal;
use social_graph_core::{Ed25519Verifier, Identity, RegistryCredentialResolver, Session};

#[tokio::test]
async fn contradiction_hard_stops_with_a_banner() {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("forked.redb");
    let group = GroupId::new([0x90; 32]);

    let id_a = Identity::from_seed([0x90; 32]);
    let id_b = Identity::from_seed([0x91; 32]);

    // Build the fork directly into the store: one fold (stateless ed25519
    // verifier + a resolver that trusts both devices) ingests two contradictory
    // genesis for the same group.
    {
        let db = Arc::new(Db::open(&path).expect("open db"));
        let resolver = RegistryCredentialResolver::new();
        resolver.register(id_a.device_id(), id_a.principal_id());
        resolver.register(id_b.device_id(), id_b.principal_id());
        let fold = DerivedFold::new(Arc::clone(&db), Ed25519Verifier, resolver);

        fold.ingest(&signed_genesis(&id_a, group, 1)).expect("genesis A");
        // The contradictory second genesis is applied with a fork recorded (not
        // silently rejected) — that is the honest behavior the demo proves.
        fold.ingest(&signed_genesis(&id_b, group, 1)).expect("genesis B (fork)");
    }

    // The substrate surfaces the contradiction.
    let session = Session::open(&path, &id_a).expect("open session");
    let summary = session.get_group_summary(&group).expect("summary");
    assert!(
        summary.fork_status.starts_with("forked_from"),
        "substrate must flag the contradiction, got {:?}",
        summary.fork_status
    );

    // The app hard-stops: a blocking banner, no silent winner.
    let mut app = App::new(session);
    app.select_group(group);

    let backend = TestBackend::new(80, 14);
    let mut terminal = Terminal::new(backend).expect("terminal");
    terminal
        .draw(|frame| ui::draw(frame, &app.view(), app.focus()))
        .expect("draw");
    let text = ui::buffer_to_string(terminal.backend().buffer());
    assert!(
        text.contains("FORK DETECTED") && text.contains("convergence halted"),
        "app must render the hard-stop banner: {text}"
    );
}
