//! **S17 — nested sealing (E96): does an outer seal still route?**
//!
//! S7 measured the leak: `group_id`, `epoch` and `content_type` sit **beside** the ciphertext in
//! RFC 9420's `PrivateMessage` framing and are readable with **no key**. So a carrier can partition
//! its store by conversation and watch epochs advance — and rotating queue names do not help,
//! because the `group_id` inside the envelope is constant across every rotation.
//!
//! The corpus already owns the fix (the history-convergence store is specified with nested
//! double-sealing) and E96 was parked with one blocking objection: **an outer seal hides the
//! routing metadata, so how would the carrier route?** That objection died with the addressed
//! model. The queue name is derived from `export_secret`, not from the envelope, so the meer has a
//! routing handle that was never inside the seal in the first place.
//!
//! This measures whether that reasoning survives contact:
//!
//! 1. Is `group_id` actually recoverable from a bare envelope, and actually gone under an outer
//!    seal? (Not "should be" — grep the bytes.)
//! 2. Does the meer still route, still dedup, and still stay blind?
//! 3. **Does the catch-up walk survive?** This is the real question: each hop is wrapped with the
//!    key of the epoch whose queue carries it, and a returner reaches epoch N+1 only by opening
//!    something in queue N. If the wrapping breaks that induction, nested sealing costs the walk.
//! 4. What does it cost in bytes?
//!
//! Fidelity: **Rung A (real-lib)** — real OpenMLS AEAD (the ciphersuite's own, via the provider's
//! crypto), real CISS, real queue.

use std::sync::Arc;

use meer_queue::ciss_harness::CissHarness;
use meer_queue::meer::{Meer, RecipientId};
use meer_queue::outer_seal;
use meer_queue::{init_tracing, mls};
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

fn group_config() -> MlsGroupCreateConfig {
    MlsGroupCreateConfig::builder()
        .ciphersuite(mls_replant::CS)
        .use_ratchet_tree_extension(true)
        .build()
}

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

/// Does `haystack` contain `needle` as a contiguous run? The blunt instrument on purpose: a claim
/// about what a carrier can *see* should be tested the way a carrier would look.
fn contains_bytes(haystack: &[u8], needle: &[u8]) -> bool {
    !needle.is_empty() && haystack.windows(needle.len()).any(|w| w == needle)
}

/// **Question 1 — the leak, and its closure, measured in bytes.**
#[test]
fn the_group_id_is_readable_in_a_bare_envelope_and_absent_under_the_outer_seal() {
    let bob = Persona::new("bob");
    let alice = Persona::new("alice");
    let (mut bobs, _alices) = seat(&bob, &alice);

    let group_id = bobs.group_id().as_slice().to_vec();
    let inner = mls::seal(
        &mut bobs,
        &bob,
        b"the payload nobody should be able to bucket",
    )
    .expect("seal");

    // --- S7, reproduced as the baseline. No key involved: just look at the bytes. ---
    assert!(
        contains_bytes(&inner, &group_id),
        "S7's finding must still hold or this test is measuring the wrong thing"
    );
    // And the public API hands it over without even that much work.
    let parsed: ProtocolMessage = MlsMessageIn::tls_deserialize_exact(&inner)
        .expect("parse")
        .try_into_protocol_message()
        .expect("protocol");
    assert_eq!(parsed.group_id().as_slice(), group_id.as_slice());

    // --- The outer seal. ---
    let wrapped = outer_seal::wrap(&bobs, &bob, &inner).expect("wrap");
    assert!(
        !contains_bytes(&wrapped, &group_id),
        "the outer seal must remove the conversation identifier a carrier buckets on"
    );
    assert!(
        MlsMessageIn::tls_deserialize_exact(&wrapped).is_err(),
        "and the wrapped object must not parse as MLS at all"
    );

    let overhead = wrapped.len() - inner.len();

    // "Flat" is a claim about a function, not about one point on it — so measure a second point.
    let big_inner = mls::seal(&mut bobs, &bob, &vec![0x42; 64 * 1024]).expect("seal big");
    let big_wrapped = outer_seal::wrap(&bobs, &bob, &big_inner).expect("wrap big");
    assert_eq!(
        big_wrapped.len() - big_inner.len(),
        overhead,
        "the overhead must not scale with payload size"
    );

    println!(
        "S17 CONFIRMED (real-lib): `group_id` ({} bytes) appears verbatim in the bare MLS envelope \
         — S7 reproduced — and is ABSENT from the outer-sealed object, which does not parse as MLS \
         at all. The cleartext-`group_id` linkability E96 filed is closed by nested sealing, \
         measured rather than argued. Cost: {} bytes inner → {} wrapped, **{overhead} bytes \
         overhead** (a {}-byte nonce plus a {}-byte AEAD tag) — measured flat: a 64 KiB payload \
         pays the same {overhead} bytes. [{}]",
        group_id.len(),
        inner.len(),
        wrapped.len(),
        mls_replant::CS.aead_nonce_length(),
        mls_replant::CS.mac_length(),
        mls::resolved_versions()
    );
}

/// **Question 2 — does the meer still route, dedup and stay blind?**
#[tokio::test]
async fn the_meer_routes_the_wrapped_object_by_queue_name_and_stays_blind() {
    init_tracing();

    let bob = Persona::new("bob");
    let alice = Persona::new("alice");
    let (mut bobs, mut alices) = seat(&bob, &alice);

    let ciss = Arc::new(CissHarness::spawn().await);
    let mut meer = Meer::new(Arc::clone(&ciss));

    // The routing handle comes from `export_secret`, NOT from the envelope — which is exactly why
    // the seal can cover the envelope entirely.
    let q = queue_name(&bobs, &bob);
    let inner = mls::seal(&mut bobs, &bob, b"nested and delivered").expect("seal");
    let wrapped = outer_seal::wrap(&bobs, &bob, &inner).expect("wrap");

    meer.publish(&wrapped, std::slice::from_ref(&q))
        .await
        .expect("publish wrapped");
    assert_eq!(meer.queue_len(&q), 1);
    assert_eq!(
        meer.key_inventory().group_keys,
        0,
        "the meer holds no group key, and now cannot even name the conversation"
    );

    let drained = meer.drain(&q, &[]).await.expect("drain");
    assert_eq!(drained.len(), 1);
    assert_eq!(
        drained[0], wrapped,
        "byte-identical forwarding still holds — M2's property is unchanged by the extra layer"
    );

    // Alice unwraps, then opens. The inner bytes must survive exactly, or M2's digest chain breaks.
    let unwrapped = outer_seal::unwrap(&alices, &alice, &drained[0]).expect("unwrap");
    assert_eq!(
        unwrapped, inner,
        "the inner envelope is recovered byte-identically"
    );
    let plaintext = mls::open(&mut alices, &alice, &unwrapped).expect("open");
    assert_eq!(plaintext, b"nested and delivered");

    // Dedup is unaffected: content addressing is over the wrapped bytes, which are still stable.
    let again = meer
        .publish(&wrapped, std::slice::from_ref(&q))
        .await
        .expect("republish");
    assert_eq!(
        meer.queue_len(&q),
        1,
        "the same wrapped object queued twice is still one entry"
    );
    assert!(!again.as_str().is_empty());

    println!(
        "S17 CONFIRMED (real-lib): a nested-sealed object routes end to end over the real queue — \
         published under a name derived from `export_secret`, forwarded BYTE-IDENTICALLY, drained, \
         unwrapped and opened. **The E96 objection is answered:** the meer never needed anything \
         inside the envelope to route, because the queue name was never in the envelope. Dedup and \
         M2's byte-identity survive unchanged, since both are properties of the outermost bytes."
    );

    ciss.shutdown().await;
}

/// **Question 2b — the seal is a real access boundary, not an obfuscation.**
#[test]
fn a_non_member_cannot_unwrap_even_holding_the_bytes() {
    let bob = Persona::new("bob");
    let alice = Persona::new("alice");
    let carol = Persona::new("carol");
    let (mut bobs, _alices) = seat(&bob, &alice);

    // Carol has her own group — she is a stranger to this one, and S7 already measured that she
    // cannot open the inner message. The question is whether the OUTER layer holds too.
    let carols = MlsGroup::new(
        &carol.provider,
        &carol.signer,
        &group_config(),
        carol.cwk.clone(),
    )
    .expect("carol's own group");

    let inner = mls::seal(&mut bobs, &bob, b"not for carol").expect("seal");
    let wrapped = outer_seal::wrap(&bobs, &bob, &inner).expect("wrap");

    let err = outer_seal::unwrap(&carols, &carol, &wrapped)
        .expect_err("a non-member must not be able to unwrap");

    // And she cannot even learn what she is holding.
    assert!(!contains_bytes(&wrapped, bobs.group_id().as_slice()));
    let _ = carols.epoch();

    println!(
        "S17 CONFIRMED (real-lib): a non-member holding the wrapped bytes is refused at the outer \
         layer — `{err}` — and the refusal is a **decryption** failure, not a routing check. That \
         is a stronger negative than S7's, where the library declined on a group-id mismatch \
         *before* attempting decryption: there, confidentiality held but was never exercised. Here \
         the AEAD tag is what says no."
    );
}

/// **The wrapping rule, verified from the failing side.**
///
/// The rule is only worth stating if breaking it actually breaks something. Wrap the commit that
/// closes epoch N with epoch N+1's key — the natural mistake, since that is what the group holds
/// once the commit is merged — and measure what the returning member gets.
#[tokio::test]
async fn wrapping_a_commit_at_the_epoch_it_opens_deadlocks_the_walk() {
    let bob = Persona::new("bob");
    let alice = Persona::new("alice");
    let (mut bobs, alices) = seat(&bob, &alice);

    let ciss = Arc::new(CissHarness::spawn().await);
    let mut meer = Meer::new(Arc::clone(&ciss));

    let here = queue_name(&bobs, &bob);
    let (_, commit) = mls_replant::commit(&mut bobs, &bob);
    let commit_bytes = commit.tls_serialize_detached().expect("ser");

    // **The mistake:** bobs has already merged, so this wraps at epoch N+1 — while the object is
    // deposited into epoch N's queue.
    let mis_wrapped = outer_seal::wrap(&bobs, &bob, &commit_bytes).expect("wrap at N+1");
    meer.publish(&mis_wrapped, std::slice::from_ref(&here))
        .await
        .expect("publish");

    // Alice arrives at hop N holding epoch N, and cannot get in.
    let objects = meer.drain(&here, &[]).await.expect("drain");
    assert_eq!(objects.len(), 1);
    let refused = outer_seal::unwrap(&alices, &alice, &objects[0]);
    let err = refused
        .expect_err("the member at epoch N must NOT be able to open an object wrapped at N+1");

    println!(
        "S17 MEASURED (real-lib): a commit wrapped at the epoch it OPENS rather than the one it \
         CLOSES is unopenable by the member who needs it — `{err}`. She holds epoch N, the object \
         wants epoch N+1, and the only way to reach epoch N+1 is to process the object she cannot \
         open. **The deadlock is real, it is silent, and it looks exactly like a corrupt object.** \
         So the wrapping rule is a genuine constraint on any implementation, not a stylistic note: \
         wrap at the epoch of the QUEUE, and for the closing commit derive that key before merging."
    );

    ciss.shutdown().await;
}

/// **Question 3 — the one that could have killed it: does the catch-up walk still induct?**
///
/// The walk is: open something in queue N → learn the commit → advance to N+1 → derive queue N+1.
/// Under nested sealing each hop is wrapped with the key of the epoch whose queue carries it, so
/// the induction now has two steps per hop instead of one. If the outer key for hop N were only
/// derivable *after* processing hop N, the chain would deadlock.
#[tokio::test]
async fn the_catch_up_walk_still_induction_steps_under_nested_sealing() {
    let bob = Persona::new("bob");
    let alice = Persona::new("alice");
    let (mut bobs, alices) = seat(&bob, &alice);

    let ciss = Arc::new(CissHarness::spawn().await);
    let mut meer = Meer::new(Arc::clone(&ciss));

    // Bob runs three epochs while Alice is away. Every object — chatter and commit alike — is
    // wrapped with the key of the epoch whose queue carries it.
    const HOPS: usize = 3;
    for round in 0..HOPS {
        let here = queue_name(&bobs, &bob);
        // **The wrapping rule in practice:** capture this epoch's outer key BEFORE committing, and
        // hold it across the commit. OpenMLS exports the current epoch only, so once the commit is
        // merged this key is unreachable — and it is the one the returning member will hold when
        // she arrives at this hop.
        let key_here = outer_seal::outer_key(&bobs, &bob).expect("derive this epoch's outer key");

        let chatter = mls::seal(&mut bobs, &bob, format!("hop {round}").as_bytes()).expect("seal");
        let wrapped_chatter =
            outer_seal::wrap_with(&key_here, &bob.provider, &chatter).expect("wrap chatter");
        meer.publish(&wrapped_chatter, std::slice::from_ref(&here))
            .await
            .expect("publish chatter");

        let (_, commit) = mls_replant::commit(&mut bobs, &bob);
        // The commit closes this epoch, so it is wrapped with THIS epoch's key — the last thing a
        // member at this epoch can still open, and the thing that carries them to the next.
        let commit_bytes = commit.tls_serialize_detached().expect("ser");
        let wrapped_commit =
            outer_seal::wrap_with(&key_here, &bob.provider, &commit_bytes).expect("wrap commit");
        meer.publish(&wrapped_commit, std::slice::from_ref(&here))
            .await
            .expect("publish commit");
    }

    // Alice returns and walks. She holds only her own group state.
    let mut hers = alices;
    let mut read = Vec::new();
    for hop in 0..HOPS {
        let here = queue_name(&hers, &alice);
        let objects = meer.drain(&here, &[]).await.expect("drain hop");
        assert_eq!(objects.len(), 2, "hop {hop}: one chatter and one commit");

        // She must unwrap BEFORE she can dispatch — which means the outer layer has to be openable
        // with what she holds on arrival at this hop, not with what she learns from it.
        let mut staged = None;
        for object in objects {
            let plain = outer_seal::unwrap(&hers, &alice, &object)
                .expect("hop's outer seal opens with the key she already holds");
            let protocol: ProtocolMessage = MlsMessageIn::tls_deserialize_exact(&plain)
                .expect("parse")
                .try_into_protocol_message()
                .expect("protocol");
            // The client contract still applies: dispatch on the cleartext content_type — which is
            // cleartext to HER, inside the outer seal, and to nobody outside it.
            match protocol.content_type() {
                ContentType::Application => {
                    read.push(mls::open(&mut hers, &alice, &plain).expect("open"));
                }
                ContentType::Commit => staged = Some(plain),
                other => panic!("unexpected {other:?}"),
            }
        }

        let commit_bytes = staged.expect("every hop carries the commit that closes it");
        let protocol: ProtocolMessage = MlsMessageIn::tls_deserialize_exact(&commit_bytes)
            .expect("parse")
            .try_into_protocol_message()
            .expect("protocol");
        match hers
            .process_message(&alice.provider, protocol)
            .expect("process commit")
            .into_content()
        {
            ProcessedMessageContent::StagedCommitMessage(sc) => {
                hers.merge_staged_commit(&alice.provider, *sc)
                    .expect("merge");
            }
            other => panic!("expected a staged commit, got {other:?}"),
        }
    }

    assert_eq!(
        read,
        (0..HOPS)
            .map(|r| format!("hop {r}").into_bytes())
            .collect::<Vec<_>>(),
        "she read every message across the walk, in order"
    );
    assert_eq!(
        queue_name(&hers, &alice),
        queue_name(&bobs, &bob),
        "and lands exactly current"
    );

    println!(
        "S17 CONFIRMED (real-lib): the catch-up walk survives nested sealing across {HOPS} hops — \
         she read every message and landed at the current epoch. The induction holds because the \
         outer key for hop N is derived from the epoch she is ALREADY at when she arrives at hop N, \
         and the commit that closes epoch N is wrapped at epoch N. **There is no deadlock and no \
         extra round trip**: the two-step per hop is unwrap-then-process, both local."
    );
    println!(
        "S17 CONSEQUENCE: nested sealing costs a flat per-object overhead and one local AEAD \
         operation per object. It does NOT cost a hop, a fetch, or a change to the queue, the \
         watermark, dedup or byte-identity. The one discipline it adds is a **wrapping rule**: an \
         object must be wrapped at the epoch of the queue it is deposited into, which for the \
         commit that CLOSES an epoch is the epoch it closes, not the one it opens. Get that \
         backwards and the chain deadlocks at the first hop — the member cannot open the thing \
         that would let her derive the key to open it."
    );

    ciss.shutdown().await;
}
