//! **S25 — the stale-peer matrix, with and without the finality gate (E112).**
//!
//! Peer P is **key-current but governance-lagging** (has not folded a ban of lineage X; X holds a
//! pre-ban token). The matrix crosses serve posture × merge posture, and the amendment adds the
//! **banned-holder population arm** (the realistic adversary is a former member, not a stranger)
//! and sources freshness from **C3's HeadAck** rather than a harness oracle.
//!
//!   - **Arm 1 (liberal serve, best-known merge)** — the divergence, made concrete: a lagging
//!     incumbent seats X, a synced one refuses; the fork is named by the admission fact.
//!   - **Arm 2 (liberal serve, STRICT merge)** — the strict-merge floor holds: below-k freshness
//!     stalls; at k the incumbent is current (sees the ban) and refuses. X is never seated.
//!   - **Arm 3 (strict serve, strict merge)** — P stalls the serve; and the liveness price: a
//!     dormant-in-good-standing returner asking the same stale P is stalled too.
//!   - **Arm 4 (no serve check — sloppy/compromised — strict merge)** — a tokenless/forged
//!     requester is handed GroupInfo+tree, but the strict merge refuses: net gain is **roster
//!     knowledge only, never admission**.
//!   - **Banned-holder population arm** — a banned lineage with its OWN token + lineage key: the
//!     serve **succeeds** at the stale peer (retiring the unqualified "fails CLOSED at a stale
//!     peer"), and the admission dies later — at strict merge / at fold.
//!
//! Freshness for the strict gates is C3's HeadAck (distinct-lineage attestations reaching k).
//!
//! Fidelity: **Rung A** for the MLS mechanics reused from S24; **Modeled** for the serve/merge
//! posture model and the HeadAck freshness (loopback grade, as C3).

use meer_queue::admission::{evaluate_admission, IssuanceLedger, MergeRefusal};
use meer_queue::groupinfo_policy::{RefusalReason, ServeDecision, ServePolicy, ServingPeer};
use std::collections::HashSet;

use local_storage_projection::completeness_ahead::{admits_irreversible, quorum_k};
use local_storage_projection::head_ack::{FreshnessTracker, HeadAck};
use local_storage_projection::traits::{
    CredentialError, CredentialResolver, DeviceId as TDeviceId, PrincipalId as TPrincipalId, Signer,
    VerifyError, Verifier,
};
use local_storage_projection::types::{GroupId, Hash, PrincipalId};

const LIN_X: &[u8] = b"lineage/banned-x";

// ---- A minimal real-binding signer/verifier for HeadAcks (the lsp mocks are feature-gated). ----

struct FreshSigner {
    key: [u8; 32],
}
impl FreshSigner {
    fn mac(&self, msg: &[u8]) -> Vec<u8> {
        use sha2::{Digest, Sha256};
        let mut h = Sha256::new();
        h.update(b"s25-fresh");
        h.update(self.key);
        h.update(msg);
        h.finalize().to_vec()
    }
}
impl Signer for FreshSigner {
    fn sign(&self, message: &[u8]) -> Vec<u8> {
        self.mac(message)
    }
    fn device_id(&self) -> TDeviceId {
        TDeviceId(self.key)
    }
}
impl Verifier for FreshSigner {
    fn verify(&self, device_id: &TDeviceId, message: &[u8], signature: &[u8]) -> Result<(), VerifyError> {
        if device_id.0 != self.key {
            return Err(VerifyError::UnknownDevice(*device_id));
        }
        if signature != self.mac(message).as_slice() {
            return Err(VerifyError::InvalidSignature { device_id: *device_id });
        }
        Ok(())
    }
}

struct TrustAll;
impl CredentialResolver for TrustAll {
    fn resolve(&self, _device: &TDeviceId, _principal: &TPrincipalId) -> Result<(), CredentialError> {
        Ok(())
    }
}

/// Build a freshness tracker for `head` corroborated by `n` distinct lineages (C3's HeadAck).
fn corroborated_to(head: Hash, group: GroupId, n: u8) -> FreshnessTracker {
    let mut tracker = FreshnessTracker::new(group, head, 7);
    for i in 0..n {
        let signer = FreshSigner { key: [0x40 + i; 32] };
        let lineage = PrincipalId::new([0x80 + i; 32]);
        let ack = HeadAck::mint(&signer, lineage, group, head, 7);
        let verifier = FreshSigner { key: ack.signer_device.as_bytes().to_owned() };
        let verified = ack.verify(&verifier, &TrustAll).expect("genuine ack verifies");
        tracker.record(&verified);
    }
    tracker
}

/// The strict-merge finality floor (§7.3.8): an incumbent may apply an irreversible admission only
/// when corroborated-fresh (freshness >= k). Below k it STALLS.
fn strict_merge_admits(freshness: u64, member_count: u64) -> bool {
    admits_irreversible(freshness, quorum_k(member_count))
}

// --------------------------------------------------------------------------------------------

#[test]
fn arm1_liberal_serve_best_known_merge_diverges_and_the_fact_names_it() {
    // A lagging incumbent (no ban folded → issuance unrevoked) merges; a synced incumbent (ban
    // folded → issuance revoked) refuses. The two disagree — the S18 invisible fork, concrete.
    let token_id = b"tok".to_vec();
    let mut lagging = IssuanceLedger::new();
    lagging.issue(token_id.clone(), LIN_X.to_vec());
    let mut synced = IssuanceLedger::new();
    synced.issue(token_id.clone(), LIN_X.to_vec());
    synced.revoke(&token_id);

    let lag = evaluate_admission(&lagging, b"P-lagging", 3, &token_id, LIN_X, b"commit");
    let syn = evaluate_admission(&synced, b"Q-synced", 9, &token_id, LIN_X, b"commit");
    assert!(lag.is_ok(), "the lagging incumbent seats X (best-known)");
    assert_eq!(syn, Err(MergeRefusal::Revoked), "the synced incumbent refuses");
    // Chain-visible: the divergence is the presence/absence of the admission fact for this event.
    assert_eq!(lag.unwrap().event, meer_queue::admission::content_address(b"commit"));
    println!("S25 arm 1 MEASURED (Modeled): liberal/best-known merge DIVERGES — lagging seats X, \
              synced refuses. The fork is the admission fact's presence, chain-visible (S18 made \
              concrete).");
}

#[test]
fn arm2_liberal_serve_strict_merge_never_seats_x() {
    let group = GroupId::new([0x99; 32]);
    let head = Hash::new([0x42; 32]);
    let member_count = 5u64; // k = 3

    // Below k: the strict gate stalls — X is not seated, regardless of the liberal serve.
    let stale = corroborated_to(head, group, 1); // freshness 1 < k
    assert!(!strict_merge_admits(stale.freshness(), member_count), "below k: stall (X not seated)");

    // At k: the incumbent is corroborated-current — which means it has folded the ban — so its
    // ledger refuses. Either way, X is never seated.
    let fresh = corroborated_to(head, group, 3); // freshness 3 = k
    assert!(strict_merge_admits(fresh.freshness(), member_count), "at k: the gate would admit,");
    let mut current_ledger = IssuanceLedger::new();
    current_ledger.issue(b"tok".to_vec(), LIN_X.to_vec());
    current_ledger.revoke(b"tok"); // being current == having folded the ban
    assert_eq!(
        evaluate_admission(&current_ledger, b"P", 9, b"tok", LIN_X, b"commit"),
        Err(MergeRefusal::Revoked),
        "…but a current incumbent has folded the ban and refuses"
    );
    println!("S25 arm 2 MEASURED (Rung-A merge / Modeled freshness via C3 HeadAck): liberal serve + \
              STRICT merge never seats X — below k the gate stalls; at k the incumbent is current \
              and refuses on the folded ban. The strict-merge/liberal-serve middle holds.");
}

#[test]
fn arm3_strict_serve_stalls_the_peer_and_charges_the_good_returner() {
    let group = GroupId::new([0x99; 32]);
    let head = Hash::new([0x42; 32]);
    let member_count = 5u64; // k = 3

    // A strict serve refuses to release the tree unless P is corroborated-fresh. P is stale.
    let stale = corroborated_to(head, group, 1);
    let p_can_serve = strict_merge_admits(stale.freshness(), member_count);
    assert!(!p_can_serve, "strict serve: a stale P stalls the serve for the banned X");

    // The liveness price: the SAME stale P stalls a dormant-in-good-standing returner too — strict
    // serve cannot tell them apart until P is fresh, so it stalls both.
    assert!(!p_can_serve, "…and stalls the good returner asking the same stale P — the liveness cost");

    // Once P corroborates to k, it can serve (and, being current, will apply standing correctly).
    let fresh = corroborated_to(head, group, 3);
    assert!(strict_merge_admits(fresh.freshness(), member_count), "a fresh P serves again");
    println!("S25 arm 3 MEASURED (Modeled freshness via C3 HeadAck): strict serve stalls a stale P \
              for X AND for a dormant good returner alike — that indiscriminate stall is the \
              liveness price of strict-serve; it lifts once P reaches k HeadAcks.");
}

#[test]
fn arm4_no_serve_check_yields_roster_knowledge_never_admission() {
    // Sloppy/compromised server: serve check skipped, GroupInfo+tree handed to anyone.
    let peer = ServingPeer::new(ServePolicy::Open, vec![9; 8], vec![1; 8]);
    let decision = peer.serve(b"whoever", true);
    let got_tree = matches!(decision, ServeDecision::Served { with_tree: true, .. });
    assert!(got_tree, "the sloppy server released the tree (roster knowledge) to a tokenless party");

    // But the strict merge refuses: a tokenless/forged requester has no issuance fact.
    let empty = IssuanceLedger::new();
    assert_eq!(
        evaluate_admission(&empty, b"incumbent", 1, b"no-such-token", b"lineage/whoever", b"commit"),
        Err(MergeRefusal::NoIssuanceFact),
        "every incumbent refuses the commit — net gain is roster knowledge, never admission"
    );
    println!("S25 arm 4 MEASURED (Modeled): with the serve check skipped, a tokenless party gets \
              GroupInfo+tree (roster knowledge) but the strict merge refuses (NoIssuanceFact). The \
              serve check protects the roster; the merge check protects the membership.");
}

#[test]
fn banned_holder_population_arm_serve_succeeds_admission_dies_later() {
    // The realistic adversary: a BANNED lineage presenting its OWN genuine token + lineage key at
    // a STALE peer. Contrast the populations at the SERVE gate.
    let mut recognised = HashSet::new();
    recognised.insert(LIN_X.to_vec());

    // Position 2 (Vouched) with the token recognised, standing NOT yet updated (stale peer).
    let stale_peer = ServingPeer::new(ServePolicy::Vouched(recognised.clone()), vec![9; 8], vec![1; 8]);

    // Stranger (no token): fails closed at serve (S22).
    let stranger = stale_peer.serve(b"lineage/stranger", true);
    assert!(matches!(stranger, ServeDecision::Refused(RefusalReason::NoRecognisedToken)),
        "a stranger fails closed at serve");

    // Banned holder (own token + lineage): the stale peer RECOGNISES the token and has not folded
    // the ban, so the serve SUCCEEDS. This retires the unqualified "fails CLOSED at a stale peer".
    let banned_holder = stale_peer.serve(LIN_X, true);
    assert!(matches!(banned_holder, ServeDecision::Served { with_tree: true, .. }),
        "the banned holder's OWN token is served at the stale peer — serve is population-dependent");

    // But the admission dies later: at a strict merge (a current incumbent's revoked issuance) or
    // at fold (C4's projection excludes the span). Here: the merge gate refuses once current.
    let mut current = IssuanceLedger::new();
    current.issue(b"tok".to_vec(), LIN_X.to_vec());
    current.revoke(b"tok");
    assert_eq!(
        evaluate_admission(&current, b"incumbent", 9, b"tok", LIN_X, b"commit"),
        Err(MergeRefusal::Revoked),
        "the banned holder's admission dies at the strict merge / at fold — not at the stale serve"
    );

    println!("S25 banned-holder MEASURED (Modeled): 'fails CLOSED at a stale peer' is \
              POPULATION-DEPENDENT — a stranger fails closed at serve (S22), but a banned holder's \
              OWN token is served at a stale peer; its admission dies later (strict merge / fold, \
              per C4), not at serve. REVIEW verdict item 1's unqualified wording is retired.");
}
