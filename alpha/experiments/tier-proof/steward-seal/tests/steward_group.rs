//! P6 (sealed steward group half) — real MLS deliberation → public verdict.
//!
//! Loopback grade (`SPEC-DELTA[run17-mls-loopback | declared-stand-in]` for the
//! transport; the crypto is real openmls 0.8 via the proven `lineage-mls`
//! Device wrapper). A small real MLS steward group deliberates under seal — a
//! steward's reasoning is MLS ciphertext, never public — and then emits a
//! VERDICT as a PUBLIC fact: a Grant matching P4's exact shape, which folds into
//! the roster on the open catalogue. Sealed reasoning, public ruling.

use group_core::ChatMessage;
use group_seal::Sealer;

use tier_proof::fold::Fold;
use tier_proof::identity::Signer;
use tier_proof::records::{Genesis, MembershipPolicy, Record, WritePolicy};
use tier_proof::source::{MemSource, RecordSource};

const SCOPE: &str = "scope:tribunal";

/// Found a real MLS steward group of three over a real Welcome (the croft-group
/// L2a add/Welcome path), all in one epoch.
fn steward_group() -> (Sealer, Sealer, Sealer) {
    let mut founder = Sealer::found("steward-founder").expect("found steward group");
    let mut alice = Sealer::enroll("steward-alice").expect("enroll alice");
    let mut bob = Sealer::enroll("steward-bob").expect("enroll bob");
    let kps = [
        alice.key_package().expect("alice kp"),
        bob.key_package().expect("bob kp"),
    ];
    let (_commit, welcome) = founder.invite(&kps).expect("invite");
    alice.accept_welcome(&welcome).expect("alice joins");
    bob.accept_welcome(&welcome).expect("bob joins");
    (founder, alice, bob)
}

#[test]
fn deliberation_is_sealed_reasoning() {
    let (mut founder, mut alice, _bob) = steward_group();

    // A steward's private reasoning about a case.
    let reasoning = ChatMessage {
        sender: "steward-founder".to_string(),
        text: "admit applicant: vouched by two existing members".to_string(),
    };
    let sealed = founder.seal(&reasoning).expect("seal deliberation");

    // The reasoning is real MLS ciphertext — the plaintext never appears.
    assert!(
        !sealed.windows(6).any(|w| w == b"admit "),
        "deliberation is sealed: the reasoning is not public",
    );
    // A co-steward in the same epoch opens it back — sealed to the group only.
    assert_eq!(
        alice.open(&sealed).expect("co-steward opens"),
        reasoning,
        "the sealed reasoning is readable inside the steward group",
    );
}

#[test]
fn verdict_is_a_public_grant_matching_p4_shape() {
    // The steward group deliberated under seal (above); the RULING is public.
    let (mut founder, _alice, _bob) = steward_group();
    let _sealed = founder.seal(&ChatMessage {
        sender: "steward-founder".to_string(),
        text: "verdict: grant".to_string(),
    });

    // The public fact: a gated scope, an applicant's request, and the steward's
    // GRANT (P4's exact Record::Grant shape, citing the request hash).
    let steward = Signer::from_seed([80u8; 32]);
    let applicant = Signer::from_seed([81u8; 32]);
    let mut src = MemSource::new();
    src.put_record(
        &steward,
        Record::Genesis(Genesis {
            scope: SCOPE.to_string(),
            title: "The Tribunal".to_string(),
            write_policy: WritePolicy::Open,
            membership_policy: MembershipPolicy::Gated,
            steward_set: vec![steward.did()],
            threshold: 1,
        }),
    );
    let req = src.put_record(&applicant, Record::Request { scope: SCOPE.to_string() });
    src.put_record_with_antecedents(
        &steward,
        vec![req],
        Record::Grant { scope: SCOPE.to_string(), subject: applicant.did() },
    );

    // The public verdict folds into the roster — sealed reasoning, public ruling.
    let state = Fold::run(&src.all()).expect("fold");
    assert!(
        state.roster_members(SCOPE).contains(&applicant.did()),
        "the public grant admits the applicant (verdict is executable)",
    );
}

#[test]
fn removal_from_the_steward_group_advances_the_epoch() {
    // Forward-blindness at the MLS layer: removing a steward re-keys the group
    // (epoch advances), so a removed steward's old epoch keys no longer open new
    // sealed deliberation. (The roster-level forward-blindness is the blinded
    // roster's salt rotation; this is its cryptographic complement.)
    let (mut founder, _alice, _bob) = steward_group();
    let before = founder.epoch().expect("epoch before");
    let _commit = founder.remove_member("steward-bob").expect("remove bob");
    let after = founder.epoch().expect("epoch after");
    assert!(after > before, "removal advances the MLS epoch (re-key)");
}
