//! FND-R19-1 pin (native, so the finding is provably NOT wasm-specific):
//! a `group-seal` member admitted by Welcome cannot itself admit a newcomer —
//! its joined group lacks the ratchet-tree extension
//! (`MlsGroupJoinConfig::default()` in `lineage-mls::Device::join_from_welcome`
//! does not enable `use_ratchet_tree_extension`), so the Welcome it produces
//! is unjoinable (`MissingRatchetTree`). Surfaced by the P2 cross-build
//! transcript; filed as a croft-group backlog gap, not fought here
//! (guardrail 2's not-fought discipline). If this test ever FAILS, the gap
//! has been fixed upstream — retire the backlog row and widen the P2/P5
//! transcripts to founder-independent invites.

use group_seal::Sealer;

#[test]
fn welcome_joined_member_cannot_yet_invite() {
    let mut alice = Sealer::found("did:example:alice").expect("found");
    let mut bob = Sealer::enroll("did:example:bob").expect("enroll");
    let (_c, welcome) = alice
        .invite(&[bob.key_package().expect("kp")])
        .expect("invite");
    bob.accept_welcome(&welcome).expect("welcome");

    // Bob (Welcome-joined) invites carol; carol cannot join his Welcome.
    let mut carol = Sealer::enroll("did:example:carol").expect("enroll");
    let (_c2, welcome2) = bob
        .invite(&[carol.key_package().expect("kp")])
        .expect("bob's invite commit itself succeeds");
    let joined = carol.accept_welcome(&welcome2);
    assert!(
        joined.is_err(),
        "FND-R19-1 fixed upstream? Welcome-joined member's Welcome became \
         joinable — retire the backlog row"
    );
    assert!(
        format!("{:?}", joined.unwrap_err()).contains("MissingRatchetTree"),
        "expected MissingRatchetTree"
    );
}
