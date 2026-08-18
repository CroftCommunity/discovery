//! **C2 — behind-detection from traffic (E112).**
//!
//! §7.4 requires a member originating or co-signing a membership act to be corroborated-fresh.
//! This experiment runs the precondition against the real fold, RED-first.
//!
//! - **Arm 1 (detection, RED-first).** A node receives a *governance* fact whose antecedents name
//!   a head it has not folded (`FoldError::MissingAntecedents`) → it marks itself behind, must not
//!   render current, and must refuse to originate or co-sign a membership op. The RED step watched
//!   the origination gate admit a behind node (the precondition un-wired) before it was wired in.
//! - **Arm 2 (quiet-group negative, expected-fail, kept).** With only ordinary traffic (applied
//!   governance + applied data-plane), the behind-via-traffic signal never fires — so a node that
//!   IS behind cannot tell. Only the fail-closed freshness leg (freshness `0 < k`) keeps it from
//!   originating, and freshness can only be *raised* by the ack primitive (C3). This proves the
//!   ack primitive is needed rather than assuming it.
//!
//! Fidelity: **Modeled / loopback grade.** Real redb fold; the freshness count is a seeded integer
//! standing in for real attested values (C3 replaces it with HeadAck union).

#[cfg(test)]
mod c2 {
    use crate::completeness_ahead::quorum_k;
    use crate::fold_derived::{DerivedFold, FoldError, IngestResult};
    use crate::head_currency::{admits_membership_origination, HeadCurrency, Stalled};
    use crate::tables::Db;
    use crate::traits::mocks::{MockCredentialResolver, MockSigner};
    use crate::traits::{
        DeviceId as TraitsDeviceId, PrincipalId as TraitsPrincipalId, Signer, VerifyError, Verifier,
    };
    use crate::types::{
        AssertionEnvelope, AssertionType, DeviceId, GroupId, Hash, PrincipalId, Role,
    };
    use std::sync::Arc;

    /// A verifier that knows several devices, delegating to the matching per-device `MockSigner`.
    struct MultiVerifier {
        signers: Vec<MockSigner>,
    }
    impl Verifier for MultiVerifier {
        fn verify(
            &self,
            device_id: &TraitsDeviceId,
            message: &[u8],
            signature: &[u8],
        ) -> Result<(), VerifyError> {
            for s in &self.signers {
                if s.device_id().0 == device_id.0 {
                    return Verifier::verify(s, device_id, message, signature);
                }
            }
            Err(VerifyError::UnknownDevice(*device_id))
        }
    }

    fn role_byte(r: &Role) -> u8 {
        match r {
            Role::Owner => 0,
            Role::Admin => 1,
            Role::Member => 2,
            Role::Observer => 3,
        }
    }

    fn genesis_payload(device: &DeviceId, threshold: u32) -> Vec<u8> {
        let mut p = Vec::new();
        p.extend_from_slice(&1u16.to_be_bytes()); // policy_version
        p.extend_from_slice(&threshold.to_be_bytes()); // add_member_threshold
        p.extend_from_slice(&threshold.to_be_bytes()); // remove_member_threshold
        p.extend_from_slice(&threshold.to_be_bytes()); // role_change_threshold
        p.extend_from_slice(&threshold.to_be_bytes()); // rule_change_threshold
        p.extend_from_slice(device.as_bytes());
        p
    }

    fn membership_add_payload(principal: &PrincipalId, role: &Role) -> Vec<u8> {
        let mut p = Vec::new();
        p.extend_from_slice(principal.as_bytes());
        p.push(role_byte(role));
        p
    }

    fn message_payload(body: &str) -> Vec<u8> {
        let mut p = Vec::new();
        let b = body.as_bytes();
        p.extend_from_slice(&(b.len() as u32).to_be_bytes());
        p.extend_from_slice(b);
        p.extend_from_slice(&0u32.to_be_bytes()); // no reply_to
        p
    }

    /// Build and sign an envelope for `signer`.
    fn signed(
        signer: &MockSigner,
        principal: PrincipalId,
        group: GroupId,
        atype: AssertionType,
        antecedents: Vec<Hash>,
        lamport: u64,
        payload: Vec<u8>,
    ) -> AssertionEnvelope {
        let mut env = AssertionEnvelope {
            version: 0x01,
            assertion_type: atype,
            author_device: DeviceId::new(signer.device_id().0),
            author_principal: principal,
            group,
            antecedents,
            lamport,
            timestamp: 1_700_000_000 + lamport,
            payload,
            signature: vec![],
        };
        env.signature = signer.sign(&env.canonical_bytes());
        env
    }

    struct World {
        fold: DerivedFold<MultiVerifier, MockCredentialResolver>,
        owner: MockSigner,
        peer: MockSigner,
        p_owner: PrincipalId,
        p_peer: PrincipalId,
        group: GroupId,
    }

    /// Boot a two-owner group (threshold 1, self-authorizing) folded at the owner's node.
    fn boot_two_owner_group() -> World {
        let owner = MockSigner::new([1u8; 32]);
        let peer = MockSigner::new([2u8; 32]);
        let p_owner = PrincipalId::new([0x11; 32]);
        let p_peer = PrincipalId::new([0x22; 32]);
        let group = GroupId::new([0x99; 32]);

        let verifier = MultiVerifier {
            signers: vec![MockSigner::new([1u8; 32]), MockSigner::new([2u8; 32])],
        };
        let mut cred = MockCredentialResolver::new();
        cred.register(TraitsDeviceId([1u8; 32]), TraitsPrincipalId(*p_owner.as_bytes()));
        cred.register(TraitsDeviceId([2u8; 32]), TraitsPrincipalId(*p_peer.as_bytes()));
        let fold = DerivedFold::new(Arc::new(Db::create_in_memory().unwrap()), verifier, cred);

        let owner_dev = DeviceId::new([1u8; 32]);
        fold.ingest(&signed(
            &owner, p_owner, group, AssertionType::GroupGenesis, vec![], 1,
            genesis_payload(&owner_dev, 1),
        ))
        .expect("genesis");
        fold.ingest(&signed(
            &owner, p_owner, group, AssertionType::MembershipAdd, vec![], 2,
            membership_add_payload(&p_owner, &Role::Owner),
        ))
        .expect("add owner");
        fold.ingest(&signed(
            &owner, p_owner, group, AssertionType::MembershipAdd, vec![], 3,
            membership_add_payload(&p_peer, &Role::Owner),
        ))
        .expect("add peer as owner");

        World { fold, owner, peer, p_owner, p_peer, group }
    }

    /// **Arm 1 — a behind node refuses to originate or co-sign a membership op.**
    #[test]
    fn arm1_behind_via_traffic_refuses_membership_origination() {
        let w = boot_two_owner_group();
        let member_count = 2u64;
        let k = quorum_k(member_count); // = 1

        // Ordinary state: not behind, and (seeded) corroborated-fresh at k. Nothing stalls.
        let mut currency = HeadCurrency::new();
        assert!(currency.may_render_current());
        assert_eq!(
            admits_membership_origination(&currency, k, member_count),
            Ok(()),
            "a fresh, not-behind node may originate"
        );

        // Traffic arrives: the peer authors a governance fact whose antecedent is a head this node
        // has NEVER folded. The fold holds it back — the behind-via-traffic signal.
        let unseen_head = Hash::new([0xEE; 32]);
        let traffic = signed(
            &w.peer, w.p_peer, w.group, AssertionType::MembershipAdd,
            vec![unseen_head], 100,
            membership_add_payload(&PrincipalId::new([0x33; 32]), &Role::Member),
        );
        let outcome = w.fold.ingest(&traffic);
        assert!(
            matches!(outcome, Err(FoldError::MissingAntecedents { have: 0, need: 1 })),
            "the incoming governance fact names an unseen head; got {outcome:?}"
        );
        currency.observe_ingest(&outcome);

        // Detection.
        assert!(currency.is_behind(), "the node knows it is behind, from traffic alone");
        assert!(!currency.may_render_current(), "a behind node must not render current");

        // The gate: even with freshness seeded AT k (so the freshness leg would admit), the behind
        // flag alone must stall origination AND co-signature. This is the RED assertion — with the
        // precondition un-wired, the gate admits and this fails.
        assert_eq!(
            admits_membership_origination(&currency, k, member_count),
            Err(Stalled::BehindViaTraffic),
            "a behind node must refuse to originate a membership op (fail-closed, §7.4)"
        );
        assert_eq!(
            admits_membership_origination(&currency, k, member_count),
            Err(Stalled::BehindViaTraffic),
            "a behind node must refuse to co-sign, too"
        );

        // Recovery: once caught up, origination is admitted again (freshness still at k).
        currency.note_caught_up();
        assert_eq!(
            admits_membership_origination(&currency, k, member_count),
            Ok(()),
            "after catching up, a fresh node may originate again"
        );

        println!(
            "C2 arm 1 MEASURED (Modeled/loopback): a governance fact naming an unseen head made \
             the fold return MissingAntecedents; the node marked itself behind and its \
             origination/co-sign gate fail-closed refused (BehindViaTraffic) even though freshness \
             was seeded at k={k}. Reads stayed available. Recovery cleared the flag."
        );
    }

    /// **Arm 2 — the quiet group: ordinary traffic yields no detection (expected).**
    #[test]
    fn arm2_quiet_group_cannot_detect_behind_from_ordinary_traffic() {
        let w = boot_two_owner_group();
        let member_count = 3u64; // a third member exists at a head this node hasn't seen
        let k = quorum_k(member_count); // = 2

        // Only ORDINARY traffic — an applied governance fact and an applied data-plane message,
        // both fully present. Neither names an unseen head.
        let mut currency = HeadCurrency::new();
        let gov = signed(
            &w.owner, w.p_owner, w.group, AssertionType::MembershipAdd, vec![], 4,
            membership_add_payload(&PrincipalId::new([0x44; 32]), &Role::Member),
        );
        let r1 = w.fold.ingest(&gov);
        assert!(matches!(r1, Ok(IngestResult::Applied { .. })), "ordinary governance applies: {r1:?}");
        currency.observe_ingest(&r1);

        let msg = signed(
            &w.peer, w.p_peer, w.group, AssertionType::Message, vec![], 101,
            message_payload("hello"),
        );
        let r2 = w.fold.ingest(&msg);
        assert!(matches!(r2, Ok(IngestResult::Applied { .. })), "ordinary message applies: {r2:?}");
        currency.observe_ingest(&r2);

        // Genuinely behind (a third member was seated at a head it never received), but ordinary
        // traffic gave it no way to know: the behind-via-traffic signal never fired.
        assert!(
            !currency.is_behind(),
            "EXPECTED SILENCE: ordinary traffic does not reveal a governance gap"
        );
        assert!(currency.may_render_current(), "so it would (wrongly) believe itself current");

        // The ONLY thing keeping it from originating is the fail-closed freshness leg.
        assert_eq!(
            admits_membership_origination(&currency, 0, member_count),
            Err(Stalled::NotCorroboratedFresh { have: 0, need: k }),
            "fail-closed: no positive corroboration means no origination, detection or not"
        );

        // The only way to lift freshness to k is positive head-attestations — the ack primitive
        // (C3). Seeded here to show the gate would then admit.
        assert_eq!(
            admits_membership_origination(&currency, k, member_count),
            Ok(()),
            "corroborated-fresh at k lifts the stall — and only HeadAcks can supply that"
        );

        println!(
            "C2 arm 2 MEASURED (Modeled/loopback): under ordinary traffic the behind-via-traffic \
             signal is SILENT (data-plane facts are optimistically accepted, §2.0.1 razor; no \
             unseen-head governance fact arrived), so a genuinely-behind node believes itself \
             current. The fail-closed freshness leg (freshness 0 < k={k}) is what actually \
             prevents an unsafe origination — and only HeadAcks (C3) can raise freshness. This is \
             the Appendix-B unreferenced-tail case, and it PROVES the ack primitive is required."
        );
    }
}
