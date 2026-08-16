//! **S21 — the group shared secret vs "keying for a member", and where governance actually fits.**
//!
//! The owner's model (2026-08-16), which this file exists to check rather than illustrate:
//!
//! > *A, B, C. C can invite D through a crypto package, or attempt to from an MLS tier, but A and B
//! > still need to accept the invite from a group governance perspective and agree to key for D —
//! > right?*
//!
//! **Half right, and the wrong half matters.** MLS has **one shared secret per epoch**, not
//! per-member keying. There is no "encrypt to A and B but not D" — you encrypt to the epoch, and
//! membership of the epoch is what decides who can read. So "agreeing to key for D" is not a
//! per-member act; it is **the decision to merge the commit that creates the epoch D is in.**
//!
//! Which raises the real question: is that decision a *thing a group can take*, or does it happen to
//! them? MLS separates **proposal** from **commit** — and that separation is exactly where a
//! governance gate belongs. This measures whether it works that way, and why an **external commit**
//! is the problem case: it is self-committing, so it skips the phase where the gate would sit.
//!
//! Fidelity: **Rung A (real-lib)** — real OpenMLS.

use meer_queue::mls;
use mls_replant::{join, Persona};
use openmls::messages::group_info::VerifiableGroupInfo;
use openmls::prelude::*;
use tls_codec::{Deserialize as _, Serialize as _};

const QUEUE_LABEL: &str = "croft/meer-queue/v1";

fn epoch_secret(group: &MlsGroup, who: &Persona) -> String {
    hex::encode(
        group
            .export_secret(who.provider.crypto(), QUEUE_LABEL, &[], 32)
            .expect("export_secret"),
    )
}

fn group_config() -> MlsGroupCreateConfig {
    MlsGroupCreateConfig::builder()
        .ciphersuite(mls_replant::CS)
        .use_ratchet_tree_extension(true)
        .build()
}

fn wire(m: &MlsMessageOut) -> Vec<u8> {
    m.tls_serialize_detached().expect("ser")
}

fn protocol(bytes: &[u8]) -> ProtocolMessage {
    MlsMessageIn::tls_deserialize_exact(bytes)
        .expect("parse")
        .try_into_protocol_message()
        .expect("protocol")
}

/// Found A + B + C.
fn found_abc() -> (Vec<Persona>, Vec<MlsGroup>) {
    let a = Persona::new("A");
    let b = Persona::new("B");
    let c = Persona::new("C");
    let mut ga = MlsGroup::new(&a.provider, &a.signer, &group_config(), a.cwk.clone())
        .expect("create");
    let (_c, welcome_out, _g) = ga
        .add_members(&a.provider, &a.signer, &[b.key_package(), c.key_package()])
        .expect("add b and c");
    ga.merge_pending_commit(&a.provider).expect("merge");
    let tree: RatchetTreeIn = ga.export_ratchet_tree().into();
    let wb = wire(&welcome_out);
    let extract = || match MlsMessageIn::tls_deserialize_exact(&wb)
        .expect("de")
        .extract()
    {
        MlsMessageBodyIn::Welcome(w) => w,
        _ => panic!("welcome"),
    };
    let gb = join(&b, extract(), tree.clone());
    let gc = join(&c, extract(), tree);
    (vec![a, b, c], vec![ga, gb, gc])
}

/// **The core mechanic.** There is ONE secret per epoch, and everyone in the epoch holds it.
#[test]
fn there_is_one_shared_secret_per_epoch_not_per_member_keying() {
    let (p, g) = found_abc();
    let secrets: Vec<String> = g.iter().zip(&p).map(|(gr, pr)| epoch_secret(gr, pr)).collect();

    assert!(
        secrets.windows(2).all(|w| w[0] == w[1]),
        "A, B and C must derive the IDENTICAL epoch secret"
    );

    println!(
        "S21 CONFIRMED (real-lib): A, B and C all derive the **identical** epoch secret. MLS keys the \
         **epoch**, not the member — there is no per-recipient key and therefore no such operation as \
         'encrypt to A and B but not C'. **So 'agreeing to key for D' is not a per-member act.** The \
         only decision available is whether to be in an epoch that contains D. [{}]",
        mls::resolved_versions()
    );
}

/// **Where the governance gate actually fits: MLS separates PROPOSE from COMMIT.**
///
/// C proposes adding D. The proposal does **not** seat D. It sits pending until some member commits
/// it — which is the two-phase decide-then-enact split, available in the protocol.
#[test]
fn a_proposal_to_add_does_not_seat_anyone_until_a_member_commits_it() {
    let (p, mut g) = found_abc();
    let d = Persona::new("D");
    let before = g[0].members().count();

    // C proposes. C is a member in good standing; this is the legitimate invite path.
    let (proposal, _ref) = g[2]
        .propose_add_member(&p[2].provider, &p[2].signer, &d.key_package())
        .expect("C proposes adding D");
    let proposal_wire = wire(&proposal);

    // A and B receive it. **Nothing is seated.** They store it as pending.
    for i in [0usize, 1] {
        let processed = g[i]
            .process_message(&p[i].provider, protocol(&proposal_wire))
            .expect("process proposal");
        match processed.into_content() {
            ProcessedMessageContent::ProposalMessage(qp) => {
                g[i].store_pending_proposal(p[i].provider.storage(), *qp)
                    .expect("store pending");
            }
            other => panic!("expected a proposal, got {other:?}"),
        }
        assert_eq!(
            g[i].members().count(),
            before,
            "a PROPOSAL must not change membership at member {i}"
        );
    }

    println!(
        "S21 CONFIRMED (real-lib): C's Add **proposal** for D left the roster at {before} members at \
         every recipient. A proposal is a **request that changes nothing** — it seats nobody, rolls \
         no epoch, and grants no keys. **This is the phase a governance gate belongs in**, and it \
         exists in MLS today: propose → (governance decides) → commit. It is the protocol-level form \
         of the spec's decide-then-enact split."
    );

    // Now A commits the pending proposal — the ENACTMENT, and the moment D gets keys.
    let (commit, welcome, _gi) = g[0]
        .commit_to_pending_proposals(&p[0].provider, &p[0].signer)
        .expect("A enacts");
    g[0].merge_pending_commit(&p[0].provider).expect("merge");
    let commit_wire = wire(&commit);

    for i in [1usize, 2] {
        let processed = g[i]
            .process_message(&p[i].provider, protocol(&commit_wire))
            .expect("process commit");
        match processed.into_content() {
            ProcessedMessageContent::StagedCommitMessage(sc) => {
                g[i].merge_staged_commit(&p[i].provider, *sc).expect("merge")
            }
            other => panic!("expected commit, got {other:?}"),
        }
    }
    assert_eq!(g[0].members().count(), before + 1);

    // D joins from the Welcome and lands in the SAME epoch secret as everyone else.
    let tree: RatchetTreeIn = g[0].export_ratchet_tree().into();
    let welcome_in = match MlsMessageIn::tls_deserialize_exact(wire(&welcome.expect("welcome")).as_slice())
        .expect("de")
        .extract()
    {
        MlsMessageBodyIn::Welcome(w) => w,
        _ => panic!("welcome"),
    };
    let gd = join(&d, welcome_in, tree);

    let all: Vec<String> = g
        .iter()
        .zip(&p)
        .map(|(gr, pr)| epoch_secret(gr, pr))
        .chain(std::iter::once(epoch_secret(&gd, &d)))
        .collect();
    assert!(
        all.windows(2).all(|w| w[0] == w[1]),
        "after the commit, A B C and D all hold ONE secret"
    );

    println!(
        "S21 CONFIRMED (real-lib): once A **committed** the proposal, A, B, C **and D** all derive \
         the identical epoch secret. **The commit is the whole of the admission**: before it D had \
         nothing, after it D has exactly what everyone has. There is no intermediate state where A \
         and B are 'keying for each other but not D' — that state does not exist in MLS."
    );
}

/// **The consequence the owner's model needs.** Once merged, refusing to key for D means leaving.
#[test]
fn after_the_commit_you_cannot_key_for_the_group_without_keying_for_d() {
    let (p, mut g) = found_abc();
    let d = Persona::new("D");

    let (_prop, _r) = g[2]
        .propose_add_member(&p[2].provider, &p[2].signer, &d.key_package())
        .expect("propose");
    // A commits its own pending proposal set directly (C's proposal is delivered below in the
    // simplest form: A re-proposes and commits, which is the same enactment shape).
    let (commit, welcome, _gi) = g[0]
        .add_members(&p[0].provider, &p[0].signer, &[d.key_package()])
        .expect("A enacts the add");
    g[0].merge_pending_commit(&p[0].provider).expect("merge");
    let commit_wire = wire(&commit);

    // B merges — B has accepted D.
    let processed = g[1]
        .process_message(&p[1].provider, protocol(&commit_wire))
        .expect("process");
    match processed.into_content() {
        ProcessedMessageContent::StagedCommitMessage(sc) => {
            g[1].merge_staged_commit(&p[1].provider, *sc).expect("merge")
        }
        other => panic!("expected commit, got {other:?}"),
    }

    let tree: RatchetTreeIn = g[0].export_ratchet_tree().into();
    let welcome_in = match MlsMessageIn::tls_deserialize_exact(wire(&welcome).as_slice())
        .expect("de")
        .extract()
    {
        MlsMessageBodyIn::Welcome(w) => w,
        _ => panic!("welcome"),
    };
    let mut gd = join(&d, welcome_in, tree);

    // B now speaks to "the group". D reads it — because there is only one epoch secret.
    let msg = mls::seal(&mut g[1], &p[1], b"intended for the original three").expect("seal");
    let d_read = mls::open(&mut gd, &d, &msg).expect("D reads it");
    assert_eq!(d_read, b"intended for the original three");

    println!(
        "S21 CONFIRMED (real-lib): after B merged the commit seating D, a message B sealed **for the \
         group** was read by **D**, in cleartext. B cannot address the original three; the epoch is \
         the audience. **So the owner's 'A and B agree to key for D' is real but lives entirely at \
         MERGE TIME:** merge and D is in everything you send; decline and you are not in the epoch \
         at all. To exclude D after merging requires a new commit REMOVING D — there is no lesser \
         move."
    );
}

/// **And why an external commit is the problem case: it skips the proposal phase entirely.**
#[test]
fn an_external_commit_is_self_committing_so_there_is_no_proposal_phase_to_gate() {
    let (p, mut g) = found_abc();
    let outsider = Persona::new("outsider");

    let gi_bytes = g[0]
        .export_group_info(p[0].provider.crypto(), &p[0].signer, true)
        .expect("gi")
        .tls_serialize_detached()
        .expect("ser");
    let gi: VerifiableGroupInfo = match MlsMessageIn::tls_deserialize_exact(&gi_bytes)
        .expect("de")
        .extract()
    {
        MlsMessageBodyIn::GroupInfo(gi) => gi,
        _ => panic!("gi"),
    };

    let (_og, bundle) = MlsGroup::external_commit_builder()
        .with_config(MlsGroupJoinConfig::default())
        .build_group(&outsider.provider, gi, outsider.cwk.clone())
        .expect("build")
        .load_psks(outsider.provider.storage())
        .expect("psks")
        .build(
            outsider.provider.rand(),
            outsider.provider.crypto(),
            &outsider.signer,
            |_| true,
        )
        .expect("build commit")
        .finalize(&outsider.provider)
        .expect("finalize");

    // What arrives at A is a COMMIT, not a proposal. There is no pending-proposal step at which a
    // governance decision could have been taken.
    let arrived = g[0]
        .process_message(&p[0].provider, protocol(&wire(bundle.commit())))
        .expect("A processes");
    let is_commit = matches!(
        arrived.content(),
        ProcessedMessageContent::StagedCommitMessage(_)
    );
    let is_external = matches!(arrived.sender(), Sender::NewMemberCommit);
    assert!(is_commit && is_external);

    println!(
        "S21 MEASURED (real-lib): an external join arrives as a **StagedCommit from \
         `NewMemberCommit`** — a COMMIT, never a proposal. **There is no pending-proposal phase to \
         gate**, because the joiner performed both halves itself. That is the structural difference \
         between C-invites-D (propose → govern → commit) and an outsider seating herself \
         (commit, take it or leave it)."
    );
    println!(
        "S21 CONSEQUENCE: the two admission paths need **different gates, in different places**. The \
         invite path can be gated in the protocol's own proposal phase — cheap, explicit, and the \
         group decides before anything changes. The external-join path has **no such phase**, so its \
         only gates are the two S19/S20 identified: **who is served a GroupInfo**, and a **merge-time \
         policy every member evaluates identically**. Conflating them is how the readmission \
         discussion got confusing: 'members must agree' is straightforwardly available for invites \
         and structurally unavailable, as a request, for external joins."
    );
}
