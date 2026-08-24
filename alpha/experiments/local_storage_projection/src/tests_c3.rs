//! **C3 — the HeadAck head-currency primitive (E112).**
//!
//! HeadAck as a §7.3.4 sign-the-state object: identity is the state attested; signatures union as
//! corroboration; freshness = distinct **lineages** (never devices) attesting the node's head.
//!
//! - **Arm 1 — request/response (modeled loopback).** A node solicits corroboration-on-latest; k
//!   distinct-lineage peers reply with signed HeadAcks of the node's head; the acks cross a wire
//!   (serialize → re-parse), are verified, and folded — freshness reaches k and the §7.4 gate
//!   flips from stall to admit. Ties C3 to C2: the seeded integer is replaced by a real count.
//! - **Arm 2 — corroborated-fresh threshold.** Below k the node stays not-fresh (gate stalls); at
//!   k it is fresh (gate admits).
//! - **Arm 3 — union.** Two acks of one head from two lineages = one object, two vouchers
//!   (freshness 2). Two devices of ONE lineage = one voucher (freshness 1) — §5.7, never clients.
//! - **Arm 4 — adversarial.** A forged/tampered ack fails the signature (and cannot be recorded —
//!   typestate). An ack naming an unknown head is a detected GAP (converge before trusting),
//!   never counted toward freshness (§7.4.3: a locator, not an authorization input).
//!
//! Fidelity: **Modeled / loopback grade.** Real signature binding and a real wire crossing; the
//! transport is in-process (the FANOUT-M1 iroh bus carries opaque frames — a mechanical upgrade).

#[cfg(test)]
mod c3 {
    use crate::completeness_ahead::quorum_k;
    use crate::head_ack::{AckOutcome, FreshnessTracker, HeadAck, HeadAckError};
    use crate::head_currency::{admits_membership_origination, HeadCurrency, Stalled};
    use crate::traits::mocks::MockCredentialResolver;
    use social_tree_core::ports::ed25519::{Ed25519Signer, Ed25519Verifier};
    use crate::traits::{DeviceId as TDeviceId, PrincipalId as TPrincipalId, Signer};
    use crate::types::{DeviceId, GroupId, Hash, PrincipalId};

    /// A peer: one device (its own MockSigner) attesting on behalf of one lineage (principal).
    struct Peer {
        signer: Ed25519Signer,
        lineage: PrincipalId,
    }
    impl Peer {
        fn new(device_seed: u8, lineage_seed: u8) -> Self {
            Self {
                signer: Ed25519Signer::from_seed([device_seed; 32]),
                lineage: PrincipalId::new([lineage_seed; 32]),
            }
        }
        fn device(&self) -> DeviceId {
            DeviceId::new(self.signer.device_id().0)
        }
        fn ack(&self, group: GroupId, head: Hash, generation: u64) -> HeadAck {
            HeadAck::mint(&self.signer, self.lineage, group, head, generation)
        }
    }

    /// A credential resolver that trusts every peer's (device, lineage) pair.
    fn cred_for(peers: &[&Peer]) -> MockCredentialResolver {
        let mut cred = MockCredentialResolver::new();
        for p in peers {
            cred.register(TDeviceId(p.device().as_bytes().to_owned()), TPrincipalId(p.lineage.as_bytes().to_owned()));
        }
        cred
    }

    /// Verify one ack the way a receiver would: cross the wire, then verify with
    /// the stateless ed25519 verifier — the ack's embedded device IS the key, so
    /// no per-device reconstruction exists to get wrong. Typed error on failure.
    fn receive(
        ack: &HeadAck,
        cred: &MockCredentialResolver,
    ) -> Result<crate::head_ack::VerifiedHeadAck, HeadAckError> {
        let wire = ack.to_bytes();
        let parsed = HeadAck::from_bytes(&wire)?;
        parsed.verify(&Ed25519Verifier, cred)
    }

    const GROUP: [u8; 32] = [0x99; 32];
    const HEAD: [u8; 32] = [0x42; 32];
    const GEN: u64 = 7;

    /// **Arm 1 — request/response over a modeled loopback bus lifts the §7.4 gate.**
    #[test]
    fn arm1_corroboration_on_latest_reaches_freshness_over_the_wire() {
        let group = GroupId::new(GROUP);
        let head = Hash::new(HEAD);
        let member_count = 5u64;
        let k = quorum_k(member_count); // = 3

        // Three distinct-lineage peers respond to the corroboration-on-latest solicitation.
        let peers = [
            Peer::new(0x10, 0xA1),
            Peer::new(0x20, 0xB2),
            Peer::new(0x30, 0xC3),
        ];
        let cred = cred_for(&peers.iter().collect::<Vec<_>>());

        let mut tracker = FreshnessTracker::new(group, head, GEN);
        let currency = HeadCurrency::new(); // not behind

        // Below k: the gate stalls.
        assert_eq!(
            admits_membership_origination(&currency, tracker.freshness(), member_count),
            Err(Stalled::NotCorroboratedFresh { have: 0, need: k })
        );

        for (i, peer) in peers.iter().enumerate() {
            let ack = peer.ack(group, head, GEN);
            let verified = receive(&ack, &cred).expect("a genuine ack verifies");
            assert_eq!(tracker.record(&verified), AckOutcome::CorroboratesHead);
            let expect_fresh = (i as u64 + 1) >= k;
            let gate = admits_membership_origination(&currency, tracker.freshness(), member_count);
            assert_eq!(gate.is_ok(), expect_fresh, "freshness={} k={k}", tracker.freshness());
        }

        assert_eq!(tracker.freshness(), k, "reached exactly k distinct-lineage vouchers");
        assert_eq!(
            admits_membership_origination(&currency, tracker.freshness(), member_count),
            Ok(()),
            "corroborated-fresh at k: the §7.4 gate admits"
        );

        println!(
            "C3 arm 1 MEASURED (Modeled/loopback): {k} distinct-lineage HeadAcks of the node's head \
             crossed a wire (serialize→re-parse), each verified and unioned; freshness rose 0→{k} \
             and the §7.4 origination gate flipped stall→admit exactly at k. This is the real \
             freshness source that replaces C2's seeded integer."
        );
    }

    /// **Arm 2 — the threshold is exactly k: k-1 stalls, k admits.**
    #[test]
    fn arm2_below_k_stays_behind_at_k_is_fresh() {
        let group = GroupId::new(GROUP);
        let head = Hash::new(HEAD);
        let member_count = 4u64;
        let k = quorum_k(member_count); // = 2
        let currency = HeadCurrency::new();

        let peers = [Peer::new(0x11, 0xA1), Peer::new(0x22, 0xB2)];
        let cred = cred_for(&peers.iter().collect::<Vec<_>>());
        let mut tracker = FreshnessTracker::new(group, head, GEN);

        // k - 1 vouchers: still stalled.
        let v0 = receive(&peers[0].ack(group, head, GEN), &cred).unwrap();
        tracker.record(&v0);
        assert_eq!(tracker.freshness(), k - 1);
        assert_eq!(
            admits_membership_origination(&currency, tracker.freshness(), member_count),
            Err(Stalled::NotCorroboratedFresh { have: k - 1, need: k })
        );

        // The k-th distinct lineage: fresh.
        let v1 = receive(&peers[1].ack(group, head, GEN), &cred).unwrap();
        tracker.record(&v1);
        assert_eq!(tracker.freshness(), k);
        assert_eq!(
            admits_membership_origination(&currency, tracker.freshness(), member_count),
            Ok(())
        );

        println!("C3 arm 2 MEASURED (Modeled): threshold is exactly k={k}; k-1 stalls, k admits.");
    }

    /// **Arm 3 — union counts distinct lineages, never devices.**
    #[test]
    fn arm3_union_counts_lineages_not_clients() {
        let group = GroupId::new(GROUP);
        let head = Hash::new(HEAD);

        // Two distinct lineages attesting one head → freshness 2, one head entry.
        let a = Peer::new(0x11, 0xA1);
        let b = Peer::new(0x22, 0xB2);
        let cred_ab = cred_for(&[&a, &b]);
        let mut t = FreshnessTracker::new(group, head, GEN);
        t.record(&receive(&a.ack(group, head, GEN), &cred_ab).unwrap());
        t.record(&receive(&b.ack(group, head, GEN), &cred_ab).unwrap());
        assert_eq!(t.freshness(), 2, "two lineages, two vouchers, one object");

        // TWO DEVICES OF ONE LINEAGE attesting the same head → still ONE voucher (§5.7).
        let dev1 = Peer { signer: Ed25519Signer::from_seed([0x31; 32]), lineage: PrincipalId::new([0xC3; 32]) };
        let dev2 = Peer { signer: Ed25519Signer::from_seed([0x32; 32]), lineage: PrincipalId::new([0xC3; 32]) };
        let cred_c = cred_for(&[&dev1, &dev2]);
        let mut t2 = FreshnessTracker::new(group, head, GEN);
        t2.record(&receive(&dev1.ack(group, head, GEN), &cred_c).unwrap());
        t2.record(&receive(&dev2.ack(group, head, GEN), &cred_c).unwrap());
        assert_eq!(t2.freshness(), 1, "two clients of one persona are one voucher, never two");

        // Idempotence: the SAME ack recorded twice is still one voucher (union, not a rival).
        let mut t3 = FreshnessTracker::new(group, head, GEN);
        let once = a.ack(group, head, GEN);
        t3.record(&receive(&once, &cred_ab).unwrap());
        t3.record(&receive(&once, &cred_ab).unwrap());
        assert_eq!(t3.freshness(), 1, "the same state attested twice unions to one voucher");

        println!(
            "C3 arm 3 MEASURED (Modeled): freshness counts distinct LINEAGES — two personae give 2, \
             two devices of one persona give 1, and a re-heard ack unions to 1. §5.7 upheld."
        );
    }

    /// **Arm 4 — adversarial: forged/tampered acks fail; an unknown head is a gap, not authority.**
    #[test]
    fn arm4_forged_fails_and_unknown_head_is_a_detected_gap() {
        let group = GroupId::new(GROUP);
        let head = Hash::new(HEAD);
        let a = Peer::new(0x11, 0xA1);
        let cred = cred_for(&[&a]);

        // (a) Forged signature: mint a genuine ack, then corrupt the signature.
        let mut forged = a.ack(group, head, GEN);
        forged.sig[0] ^= 0xFF;
        assert!(matches!(receive(&forged, &cred), Err(HeadAckError::BadSignature)),
            "a forged signature fails verification and cannot be recorded (typestate)");

        // (b) Tampered head: sign over one head, present another. The signature no longer matches.
        let mut tampered = a.ack(group, head, GEN);
        tampered.head = Hash::new([0xEE; 32]);
        assert!(matches!(receive(&tampered, &cred), Err(HeadAckError::BadSignature)),
            "changing the attested state after signing breaks the signature");

        // (c) Unknown-head ack: genuinely signed by a peer, but attesting a head this node does not
        // hold, at a NEWER generation. It is a detected gap — never counted toward freshness.
        let peer_head = Hash::new([0x77; 32]);
        let genuine_but_different = a.ack(group, peer_head, GEN + 5);
        let verified = receive(&genuine_but_different, &cred).expect("it IS validly signed");
        let mut tracker = FreshnessTracker::new(group, head, GEN);
        let outcome = tracker.record(&verified);
        assert_eq!(
            outcome,
            AckOutcome::DetectedGap { peer_head, peer_generation: GEN + 5, ahead: true },
            "an unknown head is a locator that says converge, never an authorization input"
        );
        assert_eq!(tracker.freshness(), 0, "a gap contributes NOTHING to freshness (§7.4.3)");

        println!(
            "C3 arm 4 MEASURED (Modeled): a forged or tampered ack fails the signature and cannot \
             enter the count (typestate gate); a validly-signed ack for an UNKNOWN head is a \
             detected gap (ahead=true → converge), contributing zero to freshness. §7.4.3 upheld: \
             an ack is a locator, not an authorization input."
        );
    }
}
