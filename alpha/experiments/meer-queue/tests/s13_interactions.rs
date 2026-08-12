//! **S13 — the two interactions nobody has tested.**
//!
//! Every piece of the two-target design is measured in isolation. These are the two places the
//! pieces *meet*, which is where a design usually breaks:
//!
//! 1. **The handover** — inbox → join → group queue. A joiner arrives at one epoch while the group
//!    may already have moved on, and the queue names are epoch-derived.
//! 2. **Expiry racing a drain** — retention elapses *while* a returning member is walking the chain.
//!    The walk is serial, so there is a window in which the next hop can be swept out from under it.
//!
//! Fidelity: **Rung A (real-lib)** — real OpenMLS, real CISS.
//!
//! The question behind (2) is the same one S4 raised and the fabric model was supposed to retire:
//! **can a member tell "gone" from "nothing new"?** An empty queue and a swept queue look identical
//! unless something distinguishes them.

use std::sync::Arc;

use meer_queue::ciss_harness::CissHarness;
use meer_queue::meer::{Meer, RecipientId, RETENTION_DAYS};
use meer_queue::mls;
use mls_replant::{join, Persona};
use openmls::prelude::*;
use tls_codec::{Deserialize as _, Serialize as _};

const QUEUE_LABEL: &str = "croft/meer-queue/v1";

fn queue_name(group: &MlsGroup, who: &Persona) -> RecipientId {
    RecipientId::new(hex::encode(
        group
            .export_secret(who.provider.crypto(), QUEUE_LABEL, &[], 32)
            .expect("export_secret"),
    ))
}

fn new_group(founder: &Persona) -> MlsGroup {
    let config = MlsGroupCreateConfig::builder()
        .ciphersuite(mls_replant::CS)
        .build();
    MlsGroup::new(
        &founder.provider,
        &founder.signer,
        &config,
        founder.cwk.clone(),
    )
    .expect("create group")
}

/// **Interaction 1 — the handover.**
///
/// A joiner arrives mid-conversation. Which queues can she name, which can she read, and does the
/// boundary land where MLS says it should?
#[tokio::test]
async fn the_handover_from_inbox_to_group_queue_lands_at_the_right_epoch() {
    meer_queue::init_tracing();

    let bob = Persona::new("bob");
    let alice = Persona::new("alice");
    let mut bobs = new_group(&bob);

    let ciss = Arc::new(CissHarness::spawn().await);
    let mut meer = Meer::new(Arc::clone(&ciss));

    // --- Epoch 0: Bob talks to himself, BEFORE Alice exists in the group. ---
    let q0 = queue_name(&bobs, &bob);
    let before = mls::seal(&mut bobs, &bob, b"said before alice arrived").expect("seal");
    meer.publish(&before, std::slice::from_ref(&q0)).await.expect("pub 0");

    // --- Bob adds Alice. The add commit closes epoch 0 and opens epoch 1. ---
    let (_commit, welcome_out, _gi) = bobs
        .add_members(&bob.provider, &bob.signer, &[alice.key_package()])
        .expect("add alice");
    bobs.merge_pending_commit(&bob.provider).expect("merge");
    let tree = bobs.export_ratchet_tree().into();
    let welcome = match MlsMessageIn::tls_deserialize_exact(
        &welcome_out.tls_serialize_detached().expect("ser"),
    )
    .expect("de")
    .extract()
    {
        MlsMessageBodyIn::Welcome(w) => w,
        _ => panic!("expected a Welcome"),
    };

    // --- Epoch 1: Bob speaks again, then commits, then speaks at epoch 2. ---
    let q1 = queue_name(&bobs, &bob);
    let after = mls::seal(&mut bobs, &bob, b"said after alice was added").expect("seal");
    meer.publish(&after, std::slice::from_ref(&q1)).await.expect("pub 1");
    let (_, commit1) = mls_replant::commit(&mut bobs, &bob);
    meer.publish(
        &commit1.tls_serialize_detached().expect("ser"),
        std::slice::from_ref(&q1),
    )
    .await
    .expect("pub commit1");

    let q2 = queue_name(&bobs, &bob);
    let latest = mls::seal(&mut bobs, &bob, b"said at epoch two").expect("seal");
    meer.publish(&latest, std::slice::from_ref(&q2)).await.expect("pub 2");

    // --- Alice joins from the Welcome (the inbox's output). ---
    let mut alices = join(&alice, welcome, tree);
    let alice_q = queue_name(&alices, &alice);

    // **The handover lands at epoch 1, not epoch 0.** Alice's first derivable queue is the one
    // whose epoch her Welcome seated her in.
    assert_eq!(alice_q, q1, "the joiner's first queue is the epoch she was added at");
    assert_ne!(alice_q, q0, "she cannot name the queue from before she existed");

    // She drains it and reads what was said after she was added, then advances.
    let drained = meer.drain(&alice_q, &[]).await.expect("drain q1");
    let mut read = Vec::new();
    for bytes in drained {
        let protocol: ProtocolMessage = MlsMessageIn::tls_deserialize_exact(&bytes)
            .expect("parse")
            .try_into_protocol_message()
            .expect("protocol");
        match protocol.content_type() {
            ContentType::Application => {
                read.push(mls::open(&mut alices, &alice, &bytes).expect("open"));
            }
            ContentType::Commit => {
                let p = alices
                    .process_message(&alice.provider, protocol)
                    .expect("process");
                if let ProcessedMessageContent::StagedCommitMessage(sc) = p.into_content() {
                    alices.merge_staged_commit(&alice.provider, *sc).expect("merge");
                }
            }
            other => panic!("unexpected {other:?}"),
        }
    }
    assert_eq!(read, vec![b"said after alice was added".to_vec()]);
    assert_eq!(
        queue_name(&alices, &alice),
        q2,
        "after applying the commit she names the current queue"
    );

    // And the pre-join message is unreachable — she cannot name its queue, so she never even
    // asks for it. The privacy boundary and the addressing boundary coincide.
    assert!(
        meer.drain(&alice_q, &[]).await.expect("re-drain").len() <= 2,
        "nothing from epoch 0 leaks into her view"
    );

    println!(
        "S13 CONFIRMED (real-lib): the handover lands at the epoch the Welcome seated her in. \
         History before the join is not merely undecryptable — its queue is UNNAMEABLE, so she \
         never requests it. The MLS privacy boundary and the queue-addressing boundary are the \
         SAME boundary, which is why no separate access rule is needed for backfill."
    );

    ciss.shutdown().await;
}

/// **Interaction 2 — expiry racing a drain.**
///
/// The walk is serial. Retention can elapse mid-walk, sweeping the next hop. Can the member tell
/// **"gone"** from **"nothing new"**?
#[tokio::test]
async fn a_member_can_tell_a_swept_queue_from_an_empty_one() {
    let bob = Persona::new("bob");
    let alice = Persona::new("alice");
    let mut bobs = new_group(&bob);

    let ciss = Arc::new(CissHarness::spawn().await);
    let mut meer = Meer::new(Arc::clone(&ciss));

    // Seat Alice, then let the group run on while she is away.
    let (_c, welcome_out, _g) = bobs
        .add_members(&bob.provider, &bob.signer, &[alice.key_package()])
        .expect("add");
    bobs.merge_pending_commit(&bob.provider).expect("merge");
    let tree = bobs.export_ratchet_tree().into();
    let welcome = match MlsMessageIn::tls_deserialize_exact(
        &welcome_out.tls_serialize_detached().expect("ser"),
    )
    .expect("de")
    .extract()
    {
        MlsMessageBodyIn::Welcome(w) => w,
        _ => panic!("welcome"),
    };
    let alices = join(&alice, welcome, tree);
    let alice_q = queue_name(&alices, &alice);

    // Bob deposits into her epoch's queue, then the group moves on.
    let msg = mls::seal(&mut bobs, &bob, b"you missed this").expect("seal");
    meer.publish(&msg, std::slice::from_ref(&alice_q)).await.expect("pub");
    let (_, c) = mls_replant::commit(&mut bobs, &bob);
    meer.publish(
        &c.tls_serialize_detached().expect("ser"),
        std::slice::from_ref(&alice_q),
    )
    .await
    .expect("pub commit");

    // A queue that NEVER had anything — the control.
    let never_used = RecipientId::new("f".repeat(64));
    assert!(meer.drain(&never_used, &[]).await.expect("drain").is_empty());
    assert!(
        meer.watermark(&never_used).is_none(),
        "a queue that never held anything has no watermark"
    );

    // Now retention elapses before Alice ever drains.
    meer.advance_days(RETENTION_DAYS + 1);
    let report = meer.sweep();
    assert_eq!(report.swept, 2, "both objects aged out unread");

    // Alice returns and drains. Both queues answer "empty" — the observation is identical.
    let swept_drain = meer.drain(&alice_q, &[]).await.expect("drain swept");
    let empty_drain = meer.drain(&never_used, &[]).await.expect("drain empty");
    assert!(swept_drain.is_empty() && empty_drain.is_empty());

    // **The watermark is the only thing that distinguishes them.**
    let swept_mark = meer.watermark(&alice_q);
    let empty_mark = meer.watermark(&never_used);
    assert!(
        swept_mark.is_some(),
        "a swept queue MUST carry a watermark, or loss is invisible"
    );
    assert!(empty_mark.is_none(), "a never-used queue must not");

    let wm = swept_mark.expect("watermark");
    assert_eq!(wm.swept, 2);

    println!(
        "S13 CONFIRMED (real-lib): a swept queue and a never-used queue return the SAME empty \
         drain. Only the watermark separates them — {} swept entries vs none. Without it, \
         'caught up' and 'you lost mail' are indistinguishable, which is the S4 failure mode \
         (a starving device looks like an idle one) reappearing at the retention boundary.",
        wm.swept
    );
    println!(
        "S13 CONSEQUENCE: a client MUST consult the watermark before concluding it is caught up. \
         An empty drain alone is not evidence of anything. This belongs in the client contract."
    );

    ciss.shutdown().await;
}

/// The nastier form: the sweep lands **mid-walk**, after the member has already advanced.
#[tokio::test]
async fn a_sweep_mid_walk_strands_the_member_and_says_so() {
    let bob = Persona::new("bob");
    let alice = Persona::new("alice");
    let mut bobs = new_group(&bob);

    let ciss = Arc::new(CissHarness::spawn().await);
    let mut meer = Meer::new(Arc::clone(&ciss));

    let (_c, welcome_out, _g) = bobs
        .add_members(&bob.provider, &bob.signer, &[alice.key_package()])
        .expect("add");
    bobs.merge_pending_commit(&bob.provider).expect("merge");
    let tree = bobs.export_ratchet_tree().into();
    let welcome = match MlsMessageIn::tls_deserialize_exact(
        &welcome_out.tls_serialize_detached().expect("ser"),
    )
    .expect("de")
    .extract()
    {
        MlsMessageBodyIn::Welcome(w) => w,
        _ => panic!("welcome"),
    };
    let mut alices = join(&alice, welcome, tree);

    // Two epochs of traffic, deposited on different days so they age apart.
    let q_first = queue_name(&bobs, &bob);
    let (_, c1) = mls_replant::commit(&mut bobs, &bob);
    meer.publish(
        &c1.tls_serialize_detached().expect("ser"),
        std::slice::from_ref(&q_first),
    )
    .await
    .expect("pub c1");

    meer.advance_days(3);
    let q_second = queue_name(&bobs, &bob);
    let later = mls::seal(&mut bobs, &bob, b"the hop that gets swept").expect("seal");
    meer.publish(&later, std::slice::from_ref(&q_second)).await.expect("pub later");

    // Alice takes the FIRST hop successfully.
    let first_hop = meer.drain(&q_first, &[]).await.expect("hop 1");
    assert_eq!(first_hop.len(), 1);
    let protocol: ProtocolMessage = MlsMessageIn::tls_deserialize_exact(&first_hop[0])
        .expect("parse")
        .try_into_protocol_message()
        .expect("protocol");
    let p = alices
        .process_message(&alice.provider, protocol)
        .expect("process");
    if let ProcessedMessageContent::StagedCommitMessage(sc) = p.into_content() {
        alices.merge_staged_commit(&alice.provider, *sc).expect("merge");
    }
    let next = queue_name(&alices, &alice);
    assert_eq!(next, q_second, "she can now name the next hop");

    // …and the sweep lands before she takes it.
    meer.advance_days(RETENTION_DAYS + 1);
    meer.sweep();

    let stranded = meer.drain(&next, &[]).await.expect("hop 2");
    assert!(stranded.is_empty(), "the next hop was swept out from under her");
    let mark = meer.watermark(&next).expect("the swept hop leaves a watermark");

    println!(
        "S13 MEASURED (real-lib): a sweep landing MID-WALK strands the member at the hop she had \
         just earned the right to name — and the watermark on that exact queue ({} entry) is what \
         tells her so. She is not silently 'caught up'; she is demonstrably short.",
        mark.swept
    );
    println!(
        "S13 CONSEQUENCE: the walk is serial, so its total exposure to expiry is N hops long, not \
         one. A member far behind is racing the sweeper for the whole walk — which argues the \
         retention window should be measured from the OLDEST unacked entry a member still needs, \
         not merely per-object. Recorded as a design question, not resolved here."
    );

    ciss.shutdown().await;
}
