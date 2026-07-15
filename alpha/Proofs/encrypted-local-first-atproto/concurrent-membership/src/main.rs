//! Phase 7: concurrent membership commits + CRDT fork resolution — the
//! highest-risk unknown, open since Phase 1.
//!
//! Two concurrency stories, deliberately separated because they behave very
//! differently:
//!
//!   * **MLS membership is single-writer per epoch.** Two members committing
//!     against the same epoch FORK the group; MLS does not auto-resolve. A total
//!     order (a delivery service / sequencer) must pick a winner; the loser
//!     aborts its commit and re-proposes against the new epoch. We model that
//!     with a deterministic tiebreak.
//!   * **Automerge content is a CRDT.** Concurrent edits MERGE automatically and
//!     deterministically — no fork, no lost writes. Ordering is not required.
//!
//! So: order membership, merge content. This program proves both, and that they
//! compose (converged content re-keys under the resolved epoch).

mod crypto;
#[allow(dead_code)] // carried chat-doc helpers; not all exercised here
mod doc;
mod mls;

fn section(t: &str) {
    println!("\n=== {t} ===");
}
fn pass(r: &mut Vec<(&'static str, bool)>, name: &'static str, ok: bool) {
    println!("  -> {}: {}", name, if ok { "PASS" } else { "FAIL" });
    r.push((name, ok));
}
fn kfp(k: &[u8; 32]) -> String {
    hex::encode(&k[..8])
}

fn main() {
    let mut results: Vec<(&'static str, bool)> = Vec::new();

    // ------------------------------------------------------------------
    section("STEP 0: Group at epoch 1 (Alice + Bob)");
    // ------------------------------------------------------------------
    let alice = mls::Member::new("Alice");
    let bob = mls::Member::new("Bob");
    let carol = mls::Member::new("Carol");
    let dave = mls::Member::new("Dave");
    let mut alice_group = mls::create_group(&alice);
    let welcome = mls::add_member(&mut alice_group, &alice, &bob);
    let mut bob_group = mls::join_from_welcome(&bob, &welcome);
    println!("epoch = {} ; members = Alice, Bob", alice_group.epoch().as_u64());

    // ------------------------------------------------------------------
    section("STEP 1: CONCURRENT membership commits at epoch 1 (Alice+Carol vs Bob+Dave)");
    // ------------------------------------------------------------------
    // Both stage an add against epoch 1 — a fork. Neither merges yet.
    let (commit_a, welcome_a) = mls::stage_add(&mut alice_group, &alice, &carol);
    let (commit_b, welcome_b) = mls::stage_add(&mut bob_group, &bob, &dave);

    // Deterministic tiebreak (stand-in for a delivery-service total order): the
    // lexicographically-least commit message wins. Both peers compute the same
    // winner from the same bytes, with no coordination.
    let alice_wins = commit_a <= commit_b;
    println!(
        "two commits raced at epoch 1; deterministic winner = {}",
        if alice_wins { "Alice (adds Carol first)" } else { "Bob (adds Dave first)" }
    );

    // Resolve to epoch 2 (winner's add), then the loser re-proposes -> epoch 3.
    let (carol_group, dave_group) = if alice_wins {
        mls::merge_own(&mut alice_group, &alice); // epoch 2: {A,B,Carol}
        mls::clear_own(&mut bob_group, &bob); // abort add-Dave
        mls::apply_commit(&mut bob_group, &bob, &commit_a);
        let mut carol_group = mls::join_from_welcome(&carol, &welcome_a);
        let _stale = &welcome_b; // Bob's welcome was generated at the superseded epoch
        println!("  resolved to epoch {} with Carol; Bob's add-Dave aborted (welcome discarded)", alice_group.epoch().as_u64());

        // Loser Bob re-adds Dave at epoch 2 -> epoch 3 (existing: Alice, Bob, Carol).
        let (commit_d, welcome_d) = mls::stage_add(&mut bob_group, &bob, &dave);
        mls::apply_commit(&mut alice_group, &alice, &commit_d);
        mls::apply_commit(&mut carol_group, &carol, &commit_d);
        mls::merge_own(&mut bob_group, &bob);
        let dave_group = mls::join_from_welcome(&dave, &welcome_d);
        (carol_group, dave_group)
    } else {
        mls::merge_own(&mut bob_group, &bob); // epoch 2: {A,B,Dave}
        mls::clear_own(&mut alice_group, &alice); // abort add-Carol
        mls::apply_commit(&mut alice_group, &alice, &commit_b);
        let mut dave_group = mls::join_from_welcome(&dave, &welcome_b);
        let _stale = &welcome_a;
        println!("  resolved to epoch {} with Dave; Alice's add-Carol aborted (welcome discarded)", alice_group.epoch().as_u64());

        // Loser Alice re-adds Carol at epoch 2 -> epoch 3 (existing: Alice, Bob, Dave).
        let (commit_c, welcome_c) = mls::stage_add(&mut alice_group, &alice, &carol);
        mls::apply_commit(&mut bob_group, &bob, &commit_c);
        mls::apply_commit(&mut dave_group, &dave, &commit_c);
        mls::merge_own(&mut alice_group, &alice);
        let carol_group = mls::join_from_welcome(&carol, &welcome_c);
        (carol_group, dave_group)
    };

    // All four members must agree: same epoch, same membership, same key.
    let epochs = [
        alice_group.epoch().as_u64(),
        bob_group.epoch().as_u64(),
        carol_group.epoch().as_u64(),
        dave_group.epoch().as_u64(),
    ];
    let members_agree = {
        let a = mls::member_identities(&alice_group);
        [&bob_group, &carol_group, &dave_group].iter().all(|g| mls::member_identities(g) == a) && a.len() == 4
    };
    let ka = alice.content_key(&alice_group);
    let kb = bob.content_key(&bob_group);
    let kc = carol.content_key(&carol_group);
    let kd = dave.content_key(&dave_group);
    let keys_agree = ka == kb && kb == kc && kc == kd;
    println!("after re-propose: epochs = {epochs:?}");
    println!("membership (all 4 agree on {{Alice,Bob,Carol,Dave}})? {members_agree}");
    println!("epoch-3 content-key fps: A={} B={} C={} D={}", kfp(&ka), kfp(&kb), kfp(&kc), kfp(&kd));
    pass(&mut results, "1. Concurrent commits resolved: all converge to one epoch/membership/key",
        epochs == [3, 3, 3, 3] && members_agree && keys_agree);

    // ------------------------------------------------------------------
    section("STEP 2: CONCURRENT CRDT content edits converge (no fork, no lost writes)");
    // ------------------------------------------------------------------
    // Common base shared by two members.
    let mut doc_a = doc::new_doc();
    doc::append_message(&mut doc_a, "base");
    let base = doc::snapshot(&mut doc_a);
    let base_heads = doc::heads(&mut doc_a);
    let mut doc_b = doc::load(&base);

    // Concurrent edits from the same base.
    doc::append_message(&mut doc_a, "from alice");
    doc::append_message(&mut doc_b, "from bob");

    // Exchange only the changes since the common base, then apply crosswise.
    let changes_a = doc::changes_since(&mut doc_a, &base_heads);
    let changes_b = doc::changes_since(&mut doc_b, &base_heads);
    doc::apply(&mut doc_a, changes_b);
    doc::apply(&mut doc_b, changes_a);

    let msgs_a = doc::read_messages(&doc_a);
    let msgs_b = doc::read_messages(&doc_b);
    let converged = doc::heads(&mut doc_a) == doc::heads(&mut doc_b) && msgs_a == msgs_b;
    let no_lost_writes = msgs_a.len() == 3
        && msgs_a.contains(&"base".to_string())
        && msgs_a.contains(&"from alice".to_string())
        && msgs_a.contains(&"from bob".to_string());
    println!("Alice sees {msgs_a:?}");
    println!("Bob   sees {msgs_b:?}");
    println!("converged (equal heads + messages)? {converged}; all 3 writes present? {no_lost_writes}");
    pass(&mut results, "2. Concurrent CRDT edits auto-merge to identical state", converged && no_lost_writes);

    // ------------------------------------------------------------------
    section("STEP 3: Compose — converged content re-keyed under the resolved epoch");
    // ------------------------------------------------------------------
    // The merged content is opaque to epoch churn; encrypt it under the epoch-3
    // key and have the newly-added member (Dave) read it back.
    let snapshot = doc::snapshot(&mut doc_a);
    let ciphertext = crypto::encrypt(&ka, &snapshot);
    let plain = crypto::decrypt(&kd, &ciphertext).expect("Dave decrypts under epoch-3 key");
    let dave_view = doc::read_messages(&doc::load(&plain));
    println!("Dave (added via re-propose) reads converged content: {dave_view:?}");
    pass(&mut results, "3. Converged content encrypts under resolved key; new member reads it",
        dave_view == msgs_a);

    // ------------------------------------------------------------------
    section("SUMMARY");
    // ------------------------------------------------------------------
    let mut all = true;
    for (name, ok) in &results {
        println!("[{}] {}", if *ok { "PASS" } else { "FAIL" }, name);
        all &= *ok;
    }
    println!("\nOVERALL: {}", if all { "PASS — concurrent membership resolves via ordering; CRDT content merges" } else { "FAIL" });

    section("ISSUES SURFACED");
    println!("  1. MLS forks are NOT self-healing: two commits on one epoch diverge, and openmls");
    println!("     rejects a commit from a superseded epoch. A total order (delivery service /");
    println!("     sequencer) is REQUIRED; this slice used a lexicographic tiebreak as a stand-in.");
    println!("  2. The losing committer must abort (clear_pending_commit) and RE-PROPOSE against");
    println!("     the new epoch; its Welcome and any proposals/app-messages from the losing epoch");
    println!("     are invalidated and must be re-sent.");
    println!("  3. KeyPackages are single-use; a re-proposed add needs a fresh key package (handled");
    println!("     here by generating one per add).");
    println!("  4. Automerge content needs NO ordering — it converges — so the design split is:");
    println!("     order membership (MLS), merge content (CRDT). They compose cleanly.");
    println!("  5. Ciphertext is epoch-bound but CRDT state is epoch-agnostic: post-rotation writes");
    println!("     must re-encrypt under the new key, and forward secrecy means a removed member can");
    println!("     still read content encrypted under epochs it belonged to (removal != redaction).");

    section("VERSION REPORT");
    println!("rustc {} | automerge {} | openmls {} | chacha20poly1305 {}",
        env!("SLICE_RUSTC_VERSION"), env!("SLICE_VER_AUTOMERGE"), env!("SLICE_VER_OPENMLS"), env!("SLICE_VER_CHACHA"));

    if !all {
        std::process::exit(1);
    }
}
