//! **S15 — the limbo state, walked end to end.**
//!
//! S14 asserted limbo as a **policy comparison**: meer retention (14 days) is shorter than seven of
//! §11.6's eight liveness windows, so a member absent longer than retention but shorter than
//! liveness is live-but-uncatchable. That was an ordering claim about two numbers. It was never
//! *walked* — nobody put a member in that state against the real library and asked what she can
//! actually do.
//!
//! This walks it. The question is not "does the ordering hold" (arithmetic settles that) but:
//!
//! 1. **What can a stranded-but-live member observe?** Does she learn she is stranded, or does she
//!    look caught up?
//! 2. **Is limbo actually inescapable?** S14 measured §11.7's external-commit path working for a
//!    *cold* member. A live member is not cold — but the library does not know that. If the path
//!    is open to her, limbo is a recoverable state and the retention constraint softens.
//! 3. **If it is open, what does it cost, and does the delivery design supply what it needs?**
//!
//! Fidelity: **Rung A (real-lib)** — real OpenMLS, real CISS.

use std::sync::Arc;

use meer_queue::ciss_harness::CissHarness;
use meer_queue::meer::{Meer, RecipientId, RETENTION_DAYS};
use meer_queue::mls;
use mls_replant::{join, Persona};
use openmls::prelude::*;
use tls_codec::{Deserialize as _, Serialize as _};

const QUEUE_LABEL: &str = "croft/meer-queue/v1";

/// §11.6's *modest* window for a 1–3k Group — the band the corpus's 30-day working figure sits at.
/// Chosen because it is the smallest window that is still larger than today's 14-day retention:
/// the tightest genuine limbo, not the most flattering one.
const LIVENESS_WINDOW_DAYS: u64 = 30;

fn queue_name(group: &MlsGroup, who: &Persona) -> RecipientId {
    RecipientId::new(hex::encode(
        group
            .export_secret(who.provider.crypto(), QUEUE_LABEL, &[], 32)
            .expect("export_secret"),
    ))
}

fn group_config() -> MlsGroupCreateConfig {
    MlsGroupCreateConfig::builder()
        .ciphersuite(mls_replant::CS)
        .number_of_resumption_psks(8)
        .use_ratchet_tree_extension(true)
        .build()
}

/// Seat `joiner` in a fresh group founded by `founder`, returning both views.
fn seat(founder: &Persona, joiner: &Persona) -> (MlsGroup, MlsGroup) {
    let mut f = MlsGroup::new(
        &founder.provider,
        &founder.signer,
        &group_config(),
        founder.cwk.clone(),
    )
    .expect("create group");
    let (_c, welcome_out, _g) = f
        .add_members(&founder.provider, &founder.signer, &[joiner.key_package()])
        .expect("add");
    f.merge_pending_commit(&founder.provider).expect("merge");
    let tree = f.export_ratchet_tree().into();
    let welcome = match MlsMessageIn::tls_deserialize_exact(
        welcome_out.tls_serialize_detached().expect("ser"),
    )
    .expect("de")
    .extract()
    {
        MlsMessageBodyIn::Welcome(w) => w,
        _ => panic!("expected a Welcome"),
    };
    let j = join(joiner, welcome, tree);
    (f, j)
}

/// **The premise, walked.** A member absent 15 days — past retention, inside liveness — is
/// simultaneously a live member of the hot Group and unable to name anything but her own stale
/// queue.
#[tokio::test]
async fn a_member_past_retention_but_inside_liveness_is_stranded_while_still_live() {
    meer_queue::init_tracing();

    let bob = Persona::new("bob");
    let alice = Persona::new("alice");
    let (mut bobs, alices) = seat(&bob, &alice);

    let ciss = Arc::new(CissHarness::spawn().await);
    let mut meer = Meer::new(Arc::clone(&ciss));

    // Alice's last derivable queue — the epoch her Welcome seated her in.
    let q1 = queue_name(&alices, &alice);
    assert_eq!(q1, queue_name(&bobs, &bob), "she starts current");

    // Bob runs the group for three epochs while Alice is away. Each epoch's queue carries the
    // commit that closes it, which is the link a returner needs to name the next one.
    let mut chain = vec![q1.clone()];
    for round in 0..3u8 {
        let here = queue_name(&bobs, &bob);
        let said =
            mls::seal(&mut bobs, &bob, format!("epoch chatter {round}").as_bytes()).expect("seal");
        meer.publish(&said, std::slice::from_ref(&here))
            .await
            .expect("publish chatter");
        let (_, commit) = mls_replant::commit(&mut bobs, &bob);
        meer.publish(
            &commit.tls_serialize_detached().expect("ser"),
            std::slice::from_ref(&here),
        )
        .await
        .expect("publish commit");
        chain.push(queue_name(&bobs, &bob));
    }

    // Fifteen days pass: past the 14-day retention, well inside the 30-day liveness window.
    let absent_days = RETENTION_DAYS + 1;
    assert!(
        absent_days > RETENTION_DAYS && absent_days < LIVENESS_WINDOW_DAYS,
        "the absence must land strictly inside the limbo band or this test proves nothing"
    );
    meer.advance_days(absent_days);
    let report = meer.sweep();
    assert!(report.swept > 0, "the whole chain aged out unread");

    // --- 1. She is still a LIVE member. Nobody removed her; §11.6's migration never ran. ---
    let alices_leaf = alices.own_leaf_index();
    assert!(
        bobs.members().any(|m| m.index == alices_leaf),
        "at 15 days she is inside the liveness window, so the hot Group still holds HER leaf"
    );
    assert_eq!(
        bobs.members().count(),
        2,
        "and the group is still exactly the two of them — no removal happened"
    );

    // --- 2. She learns she is stranded — the watermark, not the empty drain, tells her. ---
    let drained = meer.drain(&q1, &[]).await.expect("drain her last queue");
    assert!(drained.is_empty(), "everything she needed was swept");
    let mark = meer
        .watermark(&q1)
        .expect("a swept queue MUST carry a watermark or the loss is silent");
    assert_eq!(mark.swept, 2, "one chatter message and one commit");

    // --- 3. And the chain is severed at the FIRST link, so loss is total from there forward. ---
    // There is nothing to process, so she cannot advance: the one name she can derive is still
    // the stale one, and it is the only member of her reachable set.
    assert_eq!(
        queue_name(&alices, &alice),
        q1,
        "she can name exactly one queue, and it is the stale one"
    );
    for later in &chain[1..] {
        assert_ne!(
            *later, q1,
            "every later queue has a different name she cannot derive"
        );
        assert!(
            meer.drain(later, &[]).await.expect("drain").is_empty()
                || meer.watermark(later).is_some(),
            "and even if she could name it, it is swept too"
        );
    }

    println!(
        "S15 MEASURED (real-lib): a member absent {absent_days} days — PAST the {RETENTION_DAYS}-day \
         retention, INSIDE a {LIVENESS_WINDOW_DAYS}-day liveness window — is in all three states at \
         once: (a) still seated in the hot Group, so §11.6's migration to cold has not run; (b) \
         holding a watermark of {} lost entries, so she knows she is short; (c) able to name exactly \
         ONE queue, the stale one, because the commit that would have named the next was in the \
         entries that were swept. Limbo is not an ordering argument about two constants. It is a \
         reachable state, and this is what it looks like from inside. [{}]",
        mark.swept,
        mls::resolved_versions()
    );

    ciss.shutdown().await;
}

/// **Is limbo escapable?** S14 measured §11.7's external-commit path working for a *cold* member.
/// The library does not know "cold" from "stranded". If the path is open to a live member, limbo
/// is recoverable and the retention constraint is a cost argument, not a correctness one.
#[tokio::test]
async fn a_stranded_live_member_can_re_enter_by_external_commit_but_needs_a_groupinfo_nobody_serves(
) {
    let bob = Persona::new("bob");
    let alice = Persona::new("alice");
    let (mut bobs, alices) = seat(&bob, &alice);

    let ciss = Arc::new(CissHarness::spawn().await);
    let mut meer = Meer::new(Arc::clone(&ciss));

    let q1 = queue_name(&alices, &alice);
    let stranded_epoch = alices.epoch().as_u64();

    // The group runs on; everything Alice needed is deposited and then swept.
    for _ in 0..3 {
        let here = queue_name(&bobs, &bob);
        let (_, commit) = mls_replant::commit(&mut bobs, &bob);
        meer.publish(
            &commit.tls_serialize_detached().expect("ser"),
            std::slice::from_ref(&here),
        )
        .await
        .expect("publish commit");
    }
    meer.advance_days(RETENTION_DAYS + 1);
    meer.sweep();
    assert!(meer.drain(&q1, &[]).await.expect("drain").is_empty());

    // --- What can she address? Exactly one queue, and it is empty. ---
    // A GroupInfo is not an MLS message that travels in a queue: it is produced on demand by a
    // member holding current group state. Neither delivery target carries one — the group queue
    // she cannot name, and the personal inbox carries Welcomes.
    let everything_she_can_reach = meer.drain(&q1, &[]).await.expect("drain");
    assert!(
        everything_she_can_reach.is_empty(),
        "the delivery layer offers her nothing at all"
    );

    // --- Grant her the GroupInfo out of band and the path opens. ---
    let gi_bytes = bobs
        .export_group_info(bob.provider.crypto(), &bob.signer, true)
        .expect("export group info")
        .tls_serialize_detached()
        .expect("ser gi");
    let verifiable = match MlsMessageIn::tls_deserialize_exact(&gi_bytes)
        .expect("de gi")
        .extract()
    {
        MlsMessageBodyIn::GroupInfo(gi) => gi,
        _ => panic!("expected GroupInfo"),
    };

    let (rejoined, _bundle) = MlsGroup::external_commit_builder()
        .with_config(MlsGroupJoinConfig::default())
        .build_group(&alice.provider, verifiable, alice.cwk.clone())
        .expect("build external commit group")
        .load_psks(alice.provider.storage())
        .expect("load psks")
        .build(
            alice.provider.rand(),
            alice.provider.crypto(),
            &alice.signer,
            |_| true,
        )
        .expect("build commit")
        .finalize(&alice.provider)
        .expect("finalize");

    assert!(
        rejoined.epoch().as_u64() > stranded_epoch,
        "she re-entered ahead of where she was stranded"
    );

    println!(
        "S15 MEASURED (real-lib): a STRANDED-BUT-LIVE member re-entered by external commit — she \
         left at epoch {stranded_epoch} and re-entered at epoch {}. The library does not \
         distinguish 'cold' from 'stranded'; §11.7's path is open to anyone holding a current \
         GroupInfo. **So limbo is escapable, and S14's 'neither mechanism applies' is too strong.**",
        rejoined.epoch().as_u64()
    );
    println!(
        "S15 CONSEQUENCE — and this is the finding that matters: the escape needs a **current \
         GroupInfo**, and NEITHER delivery target carries one. The group queue is unnameable to \
         her by construction (that is what being stranded means), and the personal inbox carries \
         Welcomes. A GroupInfo is not a queued object at all — it is produced on demand by a member \
         holding live group state. So §11.7's 'self-service' return is self-service in COST only: \
         it still requires a live member to answer, over a channel this design does not have. \
         **The limbo fix is therefore not only 'retention >= liveness'. It is also: something must \
         serve GroupInfo to a returner.** Recorded as an open question, not resolved here."
    );

    ciss.shutdown().await;
}

/// **The constructive half.** Retention as a per-Group governance value, not a service constant:
/// set it at or above the Group's liveness window and the limbo band is empty by construction.
#[tokio::test]
async fn retention_at_the_liveness_window_leaves_no_limbo_band() {
    let bob = Persona::new("bob");
    let alice = Persona::new("alice");
    let (mut bobs, alices) = seat(&bob, &alice);

    let ciss = Arc::new(CissHarness::spawn().await);
    let mut meer = Meer::new(Arc::clone(&ciss));

    let q1 = queue_name(&alices, &alice);
    let (_, commit) = mls_replant::commit(&mut bobs, &bob);
    meer.publish(
        &commit.tls_serialize_detached().expect("ser"),
        std::slice::from_ref(&q1),
    )
    .await
    .expect("publish");

    // The same 15-day absence that stranded her above.
    meer.advance_days(RETENTION_DAYS + 1);

    // …but this Group governs its meer with retention == its liveness window.
    let report = meer.sweep_with_retention(LIVENESS_WINDOW_DAYS);
    assert_eq!(
        report.swept, 0,
        "nothing aged out: 15 days < 30-day retention"
    );

    let drained = meer.drain(&q1, &[]).await.expect("drain");
    assert_eq!(drained.len(), 1, "her next hop is still there");
    assert!(meer.watermark(&q1).is_none(), "and she lost nothing");

    // She takes the hop and is current again — no re-entry, no GroupInfo, no cost to anyone else.
    let mut hers = alices;
    let protocol: ProtocolMessage = MlsMessageIn::tls_deserialize_exact(&drained[0])
        .expect("parse")
        .try_into_protocol_message()
        .expect("protocol");
    let processed = hers
        .process_message(&alice.provider, protocol)
        .expect("process");
    match processed.into_content() {
        ProcessedMessageContent::StagedCommitMessage(sc) => {
            hers.merge_staged_commit(&alice.provider, *sc)
                .expect("merge");
        }
        other => panic!("expected a commit, got {other:?}"),
    }
    assert_eq!(
        queue_name(&hers, &alice),
        queue_name(&bobs, &bob),
        "she is current again by walking, which is the cheap path"
    );

    println!(
        "S15 CONFIRMED (real-lib): with retention set to the Group's liveness window \
         ({LIVENESS_WINDOW_DAYS}d), the SAME {}-day absence that stranded her above costs nothing — \
         she drains her next hop and is current. The limbo band is empty by construction, not by \
         luck. This is the constructive form of the constraint 'meer retention >= liveness window', \
         and it is why retention is a **per-Group governance value**: the Group that sets the \
         liveness window is the only party that knows what retention has to clear.",
        RETENTION_DAYS + 1
    );

    ciss.shutdown().await;
}
