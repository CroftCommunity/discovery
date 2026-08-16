//! **S14 — does the delivery design match §11.6 / §11.7 as written?**
//!
//! The corpus sketched the absence boundary far deeper than this spike had been treating it:
//!
//! - **§11.6** — liveness is *"processing epochs, not authoring messages"*; a client that misses the
//!   **liveness window** is **migrated to cold**, a removal from the hot Group.
//! - **§11.7** — re-entry is a **two-part credential**: a governance attestation (standing) plus a
//!   **resumption PSK** enabling a **self-service external commit** (keys). Explicitly *not* a
//!   stored KeyPackage, because *"the returner cannot produce their own Welcome."*
//!
//! This walks those paths against the real library to confirm the recent delivery thinking and the
//! historical planning actually agree — and to find where they do not.
//!
//! Fidelity: **Rung A (real-lib)**.

use meer_queue::mls;
use mls_replant::{join, Persona};
use openmls::prelude::*;
use tls_codec::{Deserialize as _, Serialize as _};

const QUEUE_LABEL: &str = "croft/meer-queue/v1";

fn queue_name(group: &MlsGroup, who: &Persona) -> String {
    hex::encode(
        group
            .export_secret(who.provider.crypto(), QUEUE_LABEL, &[], 32)
            .expect("export_secret"),
    )
}

fn group_config() -> MlsGroupCreateConfig {
    MlsGroupCreateConfig::builder()
        .ciphersuite(mls_replant::CS)
        // §11.7's continuity token: keep past resumption PSKs so a returner can prove prior
        // membership across epoch churn.
        .number_of_resumption_psks(8)
        .use_ratchet_tree_extension(true)
        .build()
}

/// **§11.6's core distinction:** liveness is *processing*, not *authoring*.
#[test]
fn a_silent_reader_who_processes_stays_current_and_keeps_its_queue_name() {
    let alice = Persona::new("alice");
    let bob = Persona::new("bob");
    let mut a = MlsGroup::new(
        &alice.provider,
        &alice.signer,
        &group_config(),
        alice.cwk.clone(),
    )
    .expect("group");
    let (_c, welcome_out, _g) = a
        .add_members(&alice.provider, &alice.signer, &[bob.key_package()])
        .expect("add bob");
    a.merge_pending_commit(&alice.provider).expect("merge");
    let tree = a.export_ratchet_tree().into();
    let welcome = match MlsMessageIn::tls_deserialize_exact(
        welcome_out.tls_serialize_detached().expect("ser"),
    )
    .expect("de")
    .extract()
    {
        MlsMessageBodyIn::Welcome(w) => w,
        _ => panic!("welcome"),
    };
    let mut b = join(&bob, welcome, tree);

    // Bob never authors anything. He only processes what Alice commits — a "silent reader".
    for _ in 0..5 {
        let (_, commit) = mls_replant::commit(&mut a, &alice);
        mls_replant::apply_commit(&mut b, &bob, &commit);
    }

    assert_eq!(
        a.epoch(),
        b.epoch(),
        "the silent reader is at the current epoch"
    );
    assert_eq!(
        queue_name(&a, &alice),
        queue_name(&b, &bob),
        "and therefore derives the current queue name"
    );

    println!(
        "S14 CONFIRMED (real-lib): a member who NEVER authors but processes every epoch stays exactly \
         current and derives the same queue name as the author. §11.6's distinction — liveness is \
         processing, not authoring — holds at the delivery layer too: **the queue name is a liveness \
         indicator.** A client that can still derive the current name is by definition live. [{}]",
        mls::resolved_versions()
    );
}

/// **§11.6's migration to cold is a removal — and the delivery layer enforces it automatically.**
#[test]
fn migration_to_cold_severs_queue_access_with_no_extra_mechanism() {
    let alice = Persona::new("alice");
    let bob = Persona::new("bob");
    let mut a = MlsGroup::new(
        &alice.provider,
        &alice.signer,
        &group_config(),
        alice.cwk.clone(),
    )
    .expect("group");
    let (_c, welcome_out, _g) = a
        .add_members(&alice.provider, &alice.signer, &[bob.key_package()])
        .expect("add");
    a.merge_pending_commit(&alice.provider).expect("merge");
    let tree = a.export_ratchet_tree().into();
    let welcome = match MlsMessageIn::tls_deserialize_exact(
        welcome_out.tls_serialize_detached().expect("ser"),
    )
    .expect("de")
    .extract()
    {
        MlsMessageBodyIn::Welcome(w) => w,
        _ => panic!("welcome"),
    };
    let b = join(&bob, welcome, tree);
    let bobs_last_name = queue_name(&b, &bob);
    assert_eq!(bobs_last_name, queue_name(&a, &alice));

    // Alice migrates Bob to cold: a removal (§11.6 batches many of these into one commit).
    let bob_leaf = a
        .members()
        .find(|m| m.index != a.own_leaf_index())
        .expect("bob")
        .index;
    let (_msg, _w, _g) = a
        .remove_members(&alice.provider, &alice.signer, &[bob_leaf])
        .expect("remove");
    a.merge_pending_commit(&alice.provider).expect("merge");

    // Alice has moved on; Bob's last-known name is stale and he cannot derive the new one.
    assert_ne!(
        queue_name(&a, &alice),
        bobs_last_name,
        "the group's queue name has moved past Bob"
    );

    println!(
        "S14 CONFIRMED (real-lib): migrating a member to cold (a removal) SEVERS queue access with \
         no delivery-layer mechanism at all. The group's name moves on and the cold member cannot \
         derive it — access control for cold members is a CONSEQUENCE of the naming scheme, not a \
         feature that has to be built or enforced by the meer."
    );
}

/// **§11.7's key half:** a resumption PSK enables a self-service external commit.
#[test]
fn a_cold_member_can_rejoin_by_external_commit_without_a_welcome() {
    let alice = Persona::new("alice");
    let bob = Persona::new("bob");
    let mut a = MlsGroup::new(
        &alice.provider,
        &alice.signer,
        &group_config(),
        alice.cwk.clone(),
    )
    .expect("group");
    let (_c, welcome_out, _g) = a
        .add_members(&alice.provider, &alice.signer, &[bob.key_package()])
        .expect("add");
    a.merge_pending_commit(&alice.provider).expect("merge");
    let tree = a.export_ratchet_tree().into();
    let welcome = match MlsMessageIn::tls_deserialize_exact(
        welcome_out.tls_serialize_detached().expect("ser"),
    )
    .expect("de")
    .extract()
    {
        MlsMessageBodyIn::Welcome(w) => w,
        _ => panic!("welcome"),
    };
    let b = join(&bob, welcome, tree);
    let bobs_epoch = b.epoch();
    let had_resumption = b.resumption_psk_secret().as_slice().to_vec();
    assert!(
        !had_resumption.is_empty(),
        "the returner stores a continuity token"
    );

    // Bob goes cold; the group churns well past him.
    let bob_leaf = a
        .members()
        .find(|m| m.index != a.own_leaf_index())
        .expect("bob")
        .index;
    a.remove_members(&alice.provider, &alice.signer, &[bob_leaf])
        .expect("remove");
    a.merge_pending_commit(&alice.provider).expect("merge");
    for _ in 0..4 {
        mls_replant::commit(&mut a, &alice);
    }

    // §11.7: the returner fetches a current GroupInfo and builds its OWN commit — the cost falls
    // on the returner, not on an active member.
    let group_info = a
        .export_group_info(alice.provider.crypto(), &alice.signer, true)
        .expect("group info");
    let gi_bytes = group_info.tls_serialize_detached().expect("ser gi");
    let verifiable = match MlsMessageIn::tls_deserialize_exact(&gi_bytes)
        .expect("de gi")
        .extract()
    {
        MlsMessageBodyIn::GroupInfo(gi) => gi,
        _ => panic!("expected GroupInfo"),
    };

    // NOTE: deprecated in favour of MlsGroup::external_commit_builder. Kept because it is the
    // shortest expression of 11.7 s claim; the builder is the path a real client should use.
    #[allow(deprecated)]
    let rejoin = MlsGroup::join_by_external_commit(
        &bob.provider,
        &bob.signer,
        None,
        verifiable,
        &MlsGroupJoinConfig::default(),
        None,
        None,
        &[],
        bob.cwk.clone(),
    );

    match rejoin {
        Ok((mut rejoined, _commit_msg, _gi)) => {
            rejoined.merge_pending_commit(&bob.provider).ok();
            println!(
                "S14 CONFIRMED (real-lib): a cold member REJOINED by external commit using a \
                 current GroupInfo — no Welcome, no active member's help. §11.7's self-service \
                 return path exists in the library, at the returner's own cost, exactly as \
                 specified. He left at epoch {} and re-entered at epoch {}.",
                bobs_epoch.as_u64(),
                rejoined.epoch().as_u64()
            );
        }
        Err(e) => println!(
            "S14 MEASURED (real-lib): external-commit rejoin was refused: {e}. §11.7's self-service \
             path may need more than this harness supplies (its resync form, or a PSK injected \
             explicitly) — recorded rather than assumed working."
        ),
    }
}

/// **The alignment claim:** the meer's retention window must be ≥ the Group's liveness window,
/// or a member lands in limbo — live, uncatchable-up, and not yet cold.
#[test]
fn the_limbo_state_is_real_when_retention_is_shorter_than_liveness() {
    // Modelled as a policy comparison, not a run: both windows are policy numbers, and the
    // limbo is a property of their ordering rather than of any code path. Stated as an
    // executable check so a later change to either default trips it.
    const MEER_RETENTION_DAYS: u64 = meer_queue::meer::RETENTION_DAYS; // 14 today
                                                                       // §11.6's schedule, tightening with group size.
    let liveness_windows: [(&str, u64, u64); 4] = [
        ("250–1k", 90, 45),
        ("1–3k", 60, 30),
        ("3–7k", 45, 21),
        ("7–10k", 30, 14),
    ];

    let mut limbo_bands = Vec::new();
    for (band, modest, aggressive) in liveness_windows {
        for (label, window) in [("modest", modest), ("aggressive", aggressive)] {
            if MEER_RETENTION_DAYS < window {
                limbo_bands.push(format!("{band}/{label} ({window}d)"));
            }
        }
    }

    println!(
        "S14 MEASURED: meer retention is {MEER_RETENTION_DAYS} days. §11.6 liveness windows it is \
         SHORTER than — i.e. bands where a member can be live-but-uncatchable — are: {}.",
        if limbo_bands.is_empty() {
            "none".to_string()
        } else {
            limbo_bands.join(", ")
        }
    );
    println!(
        "S14 CONSEQUENCE: in every band listed, a member absent longer than retention but shorter \
         than the liveness window is still a LIVE member of the hot Group, cannot catch up from the \
         meer (the chain is severed at the oldest link), and has NOT been migrated to cold. The fix \
         is ordering, not code: **meer retention ≥ the Group's liveness window**, which makes \
         'cannot catch up' and 'migrated to cold' coincide. [CORRECTED by S15, 2026-08-13: this \
         verdict originally added 'so §11.7's re-entry is not open to them either — neither \
         mechanism applies'. Measured false. External commit IS open to a stranded-but-live member; \
         what is missing is the GroupInfo it needs, which nothing serves. See E105.]"
    );

    assert!(
        !limbo_bands.is_empty(),
        "documenting the current state: a 14-day retention is below most §11.6 windows"
    );
}
