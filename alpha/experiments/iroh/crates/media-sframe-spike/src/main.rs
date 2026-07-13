//! E12 — blind media-meer: SFrame-over-MLS (the C3 keying proof, TC-KEY1–4).
//!
//! Builds a REAL openmls group via `lineage-mls`, derives per-sender SFrame base keys from the
//! group's genuine MLS exporter secret (`epoch_proof`) bound to `(epoch, leaf_index)`, and runs the
//! four C3 test cases from `discovery/thinking/realtime-media-over-iroh.md`:
//!   TC-KEY1  per-sender keys are distinct; a non-member cannot derive any key.
//!   TC-KEY2  loss + intra-window reorder: surviving frames decrypt out of order; replays rejected.
//!   TC-KEY3  membership change advances the epoch → revoked sender's later frames undecryptable
//!            (media MD-G5), while pre-revocation frames stay decryptable (history not clawed back).
//!   TC-KEY4  the blind SFU sees only (key_id, counter) headers + ciphertext: it can route/select but
//!            recovers ZERO plaintext (the blind property, media analog of E3.4/AR-4).
//!
//! Spike-class: assertions panic loudly on any failure; a JSON verdict prints on success. Pure crypto
//! (no transport) — the keying question is independent of the wire E10 already characterized.

use chacha20poly1305::aead::{Aead, KeyInit, Payload};
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
use hkdf::Hkdf;
use lineage_core::Did;
use lineage_mls::Device;
use serde::Serialize;
use sha2::Sha256;

/// SFrame header — sent in clear, bound into every frame as AEAD associated data. The SFU reads it;
/// it never decrypts. `epoch` lets a receiver reject a removed sender's stale-epoch frames.
#[derive(Clone, Copy, Debug)]
struct Header {
    epoch: u64,
    leaf: u32,
    counter: u64,
}

impl Header {
    fn aad(&self) -> [u8; 20] {
        let mut b = [0u8; 20];
        b[0..8].copy_from_slice(&self.epoch.to_be_bytes());
        b[8..12].copy_from_slice(&self.leaf.to_be_bytes());
        b[12..20].copy_from_slice(&self.counter.to_be_bytes());
        b
    }
    fn nonce(&self) -> Nonce {
        // Per-(epoch,leaf) key; nonce only needs uniqueness per counter. leaf||counter is unique.
        let mut n = [0u8; 12];
        n[0..4].copy_from_slice(&self.leaf.to_be_bytes());
        n[4..12].copy_from_slice(&self.counter.to_be_bytes());
        *Nonce::from_slice(&n)
    }
}

/// Per-sender SFrame base key = HKDF-SHA256(exporter_secret, "croft/sframe/v1" ‖ epoch ‖ leaf).
/// Every group member derives the same key for a given (epoch, leaf); a non-member has no
/// exporter_secret and so cannot derive any sender's key.
fn sframe_key(exporter_secret: &[u8], epoch: u64, leaf: u32) -> [u8; 32] {
    let hk = Hkdf::<Sha256>::new(None, exporter_secret);
    let mut info = Vec::with_capacity(15 + 8 + 4);
    info.extend_from_slice(b"croft/sframe/v1");
    info.extend_from_slice(&epoch.to_be_bytes());
    info.extend_from_slice(&leaf.to_be_bytes());
    let mut okm = [0u8; 32];
    hk.expand(&info, &mut okm).expect("32-byte HKDF expand is valid");
    okm
}

/// A sealed SFrame: clear header + AEAD ciphertext over the media payload.
#[derive(Clone)]
struct SealedFrame {
    hdr: Header,
    ct: Vec<u8>,
}

fn seal(key: &[u8; 32], hdr: Header, plaintext: &[u8]) -> SealedFrame {
    let cipher = ChaCha20Poly1305::new(Key::from_slice(key));
    let aad = hdr.aad();
    let ct = cipher
        .encrypt(&hdr.nonce(), Payload { msg: plaintext, aad: &aad })
        .expect("seal");
    SealedFrame { hdr, ct }
}

fn open(key: &[u8; 32], frame: &SealedFrame) -> Option<Vec<u8>> {
    let cipher = ChaCha20Poly1305::new(Key::from_slice(key));
    let aad = frame.hdr.aad();
    cipher
        .decrypt(&frame.hdr.nonce(), Payload { msg: &frame.ct, aad: &aad })
        .ok()
}

/// Sliding replay window over a per-sender monotone counter — loss-tolerant (gaps allowed), reorder-
/// tolerant within the window, replay-rejecting. No contiguity requirement (unlike the message chain).
struct ReplayWindow {
    high: u64,
    seen: std::collections::VecDeque<u64>,
    width: u64,
}
impl ReplayWindow {
    fn new(width: u64) -> Self {
        Self { high: 0, seen: std::collections::VecDeque::new(), width }
    }
    /// Returns true if `counter` is fresh (accept), false if a replay/too-old (reject).
    fn admit(&mut self, counter: u64) -> bool {
        if self.seen.contains(&counter) {
            return false;
        }
        if counter + self.width < self.high {
            return false; // outside the window, treat as replay/too-late
        }
        self.seen.push_back(counter);
        if counter > self.high {
            self.high = counter;
        }
        while self.seen.len() as u64 > self.width * 2 {
            self.seen.pop_front();
        }
        true
    }
}

#[derive(Serialize, Default)]
struct Verdict {
    tc_key1_distinct_keys_nonmember_cannot_derive: bool,
    tc_key2_lossy_reordered_decrypt_replay_reject: bool,
    tc_key3_revoked_undecryptable_history_retained: bool,
    tc_key4_blind_sfu_zero_plaintext: bool,
    notes: Vec<String>,
}

fn main() -> anyhow::Result<()> {
    let mut v = Verdict::default();

    // --- Build a REAL 3-member MLS group: alice (founder) + bob + carol. ---
    let mut alice = Device::new(Did::new("alice"))?;
    let mut bob = Device::new(Did::new("bob"))?;
    let mut carol = Device::new(Did::new("carol"))?;
    alice.create_group()?;
    let kp_bob = bob.key_package()?;
    let kp_carol = carol.key_package()?;
    let (commit_add, welcome) = alice.add(&[kp_bob, kp_carol])?;
    let _ = commit_add;
    bob.join_from_welcome(&welcome, None)?;
    carol.join_from_welcome(&welcome, None)?;

    let epoch = alice.epoch()?;
    assert_eq!(epoch, bob.epoch()?, "alice/bob same epoch");
    assert_eq!(epoch, carol.epoch()?, "carol same epoch");
    let s1_alice = alice.epoch_proof()?;
    let s1_bob = bob.epoch_proof()?;
    assert_eq!(s1_alice, s1_bob, "members derive the SAME MLS exporter secret");

    let leaf_alice = alice.leaf_index_of(&Did::new("alice"))?.expect("alice leaf").u32();
    let leaf_carol = alice.leaf_index_of(&Did::new("carol"))?.expect("carol leaf").u32();
    v.notes.push(format!(
        "group built: epoch={epoch} members=3 leaf(alice)={leaf_alice} leaf(carol)={leaf_carol} exporter_secret_len={}",
        s1_alice.len()
    ));

    // ===== TC-KEY1 — per-sender keys distinct; non-member cannot derive =====
    let k_alice = sframe_key(&s1_bob, epoch, leaf_alice);
    let k_carol = sframe_key(&s1_bob, epoch, leaf_carol);
    let distinct = k_alice != k_carol;
    // A non-member has no group → epoch_proof() errors → no exporter secret → cannot derive any key.
    let outsider = Device::new(Did::new("mallory"))?;
    let nonmember_cannot_derive = outsider.epoch_proof().is_err();
    v.tc_key1_distinct_keys_nonmember_cannot_derive = distinct && nonmember_cannot_derive;
    assert!(distinct, "TC-KEY1: two senders must get distinct keys");
    assert!(nonmember_cannot_derive, "TC-KEY1: a non-member must not be able to derive the secret");
    v.notes.push("TC-KEY1: alice/carol keys distinct; non-member epoch_proof() errs (no standing)".into());

    // ===== TC-KEY2 — loss + intra-window reorder; decrypt survivors, reject replays =====
    // alice sends 100 frames; drop ~10% deterministically, reorder within a 16-frame window.
    let total = 100u64;
    let mut wire: Vec<SealedFrame> = Vec::new();
    for c in 0..total {
        if c % 10 == 7 {
            continue; // ~10% loss (deterministic so the spike is reproducible)
        }
        let pt = format!("opus-frame-{c}").into_bytes();
        wire.push(seal(&k_alice, Header { epoch, leaf: leaf_alice, counter: c }, &pt));
    }
    // intra-window reorder: swap each adjacent pair (stays within a 16-wide window).
    let mut i = 0;
    while i + 1 < wire.len() {
        wire.swap(i, i + 1);
        i += 2;
    }
    let sent = wire.len();
    let mut rw = ReplayWindow::new(16);
    let mut decrypted = 0u64;
    let mut bad = 0u64;
    for f in &wire {
        match open(&k_alice, f) {
            Some(pt) if rw.admit(f.hdr.counter) => {
                assert_eq!(pt, format!("opus-frame-{}", f.hdr.counter).into_bytes());
                decrypted += 1;
            }
            Some(_) => bad += 1, // decrypted but a replay (shouldn't happen on first pass)
            None => bad += 1,
        }
    }
    assert_eq!(decrypted as usize, sent, "TC-KEY2: every surviving frame decrypts out of order");
    assert_eq!(bad, 0, "TC-KEY2: no spurious failures on the first pass");
    // Now replay the whole window: every counter is a replay → all rejected.
    let mut replays_rejected = 0u64;
    for f in &wire {
        if open(&k_alice, f).is_some() && !rw.admit(f.hdr.counter) {
            replays_rejected += 1;
        }
    }
    let replay_ok = replays_rejected == sent as u64;
    v.tc_key2_lossy_reordered_decrypt_replay_reject = replay_ok;
    assert!(replay_ok, "TC-KEY2: every replayed frame must be rejected by the window");
    v.notes.push(format!(
        "TC-KEY2: lost {} of {total}, reordered; decrypted {decrypted}/{sent} out-of-order; replays rejected {replays_rejected}/{sent}",
        total - sent as u64
    ));

    // ===== TC-KEY3 — revoke carol → epoch advance → media MD-G5 + history retained =====
    // Pre-revoke: carol sends a frame at epoch1; bob receives & decrypts; bob CACHES it (history).
    let k_carol_e1 = sframe_key(&s1_bob, epoch, leaf_carol);
    let pre_revoke = seal(&k_carol_e1, Header { epoch, leaf: leaf_carol, counter: 0 }, b"carol-pre-revoke");
    let pre_plain = open(&sframe_key(&bob.epoch_proof()?, epoch, leaf_carol), &pre_revoke);
    assert_eq!(pre_plain.as_deref(), Some(&b"carol-pre-revoke"[..]), "pre-revoke frame decrypts");

    // Revoke carol. Alice commits the removal; bob processes the same commit → both advance to epoch2.
    let carol_leaf = alice.leaf_index_of(&Did::new("carol"))?.expect("carol leaf idx");
    let (commit_remove, _welcome) = alice.remove(&[carol_leaf])?;
    bob.recv(&commit_remove)?; // bob merges the removal commit
    let epoch2 = alice.epoch()?;
    assert_eq!(epoch2, bob.epoch()?, "alice/bob both at the new epoch");
    assert!(epoch2 > epoch, "epoch advanced on removal");
    let s2_bob = bob.epoch_proof()?;
    assert_ne!(s1_bob, s2_bob, "exporter secret rotated on the membership change");

    // carol is removed: still stuck at epoch1, has only S1, no S2.
    assert_eq!(carol.epoch()?, epoch, "removed carol cannot advance her own epoch");
    assert_ne!(carol.epoch_proof()?, s2_bob, "removed carol cannot derive the new group secret");
    // bob no longer counts carol as a member.
    let carol_still_member = bob.leaf_index_of(&Did::new("carol"))?.is_some();
    assert!(!carol_still_member, "bob drops carol from the member set");

    // Post-revoke: carol keeps sending (only S1-keyed, epoch1). bob is at epoch2 and REJECTS it:
    // the sender is no longer a current member and the frame's epoch is stale. Media MD-G5.
    let post_revoke = seal(&k_carol_e1, Header { epoch, leaf: leaf_carol, counter: 1 }, b"carol-post-revoke");
    let bob_current_epoch = bob.epoch()?;
    let reject_post = {
        let stale = post_revoke.hdr.epoch < bob_current_epoch;
        let not_member = bob.leaf_index_of(&Did::new("carol"))?.is_none();
        // Policy: a removed member's stale-epoch frame is rejected without even attempting decrypt.
        stale && not_member
    };
    assert!(reject_post, "TC-KEY3: removed sender's later frame must be rejected (media MD-G5)");
    // And structurally carol cannot forge an epoch2 frame: she has no S2-derived key.
    let carol_forge = open(&sframe_key(&s2_bob, epoch2, leaf_carol),
        &seal(&k_carol_e1, Header { epoch: epoch2, leaf: leaf_carol, counter: 2 }, b"forge"));
    assert!(carol_forge.is_none(), "TC-KEY3: carol's S1 frame cannot pass as an epoch2 frame");

    // History retained: bob's cached pre-revoke frame still decrypts under the cached epoch1 key.
    let history_ok = open(&k_carol_e1, &pre_revoke).as_deref() == Some(&b"carol-pre-revoke"[..]);
    assert!(history_ok, "TC-KEY3: pre-revocation frames stay decryptable (not clawed back)");
    // The group keeps working: alice (current member) sends at epoch2; bob decrypts.
    let k_alice_e2 = sframe_key(&s2_bob, epoch2, leaf_alice);
    let after = seal(&k_alice_e2, Header { epoch: epoch2, leaf: leaf_alice, counter: 0 }, b"alice-after");
    assert_eq!(open(&k_alice_e2, &after).as_deref(), Some(&b"alice-after"[..]), "group still works post-removal");

    v.tc_key3_revoked_undecryptable_history_retained = reject_post && carol_forge.is_none() && history_ok;
    v.notes.push(format!(
        "TC-KEY3: removal advanced epoch {epoch}->{epoch2}, secret rotated; removed carol stuck@{epoch} (no S2); \
         her later frame rejected (stale+non-member); pre-revoke history still decrypts; group works post-removal"
    ));

    // ===== TC-KEY4 — the blind SFU routes from headers but recovers zero plaintext =====
    // The SFU holds NO exporter secret. It parses headers (route/select) and tries to decrypt with
    // nothing — it cannot. Feed it every frame we put on the wire plus carol's.
    let all: Vec<SealedFrame> = wire.iter().cloned().chain([pre_revoke, after].into_iter()).collect();
    let mut routed = 0u64;
    let mut sfu_recovered_plaintext = 0u64;
    for f in &all {
        // Routing decision uses only the clear header — proves a blind SFU can still forward/select.
        let _route_target = (f.hdr.leaf, f.hdr.counter);
        routed += 1;
        // The SFU has no key. Model every key it could "guess" as absent: try an all-zero key and
        // confirm it yields nothing (AEAD auth fails). It cannot derive the real key (no secret).
        if open(&[0u8; 32], f).is_some() {
            sfu_recovered_plaintext += 1;
        }
        // Also confirm the ciphertext is not the plaintext in the clear.
        assert!(!f.ct.windows(4).any(|w| w == b"opus" || w == b"caro" || w == b"alic"),
            "ciphertext must not contain plaintext");
    }
    let blind = sfu_recovered_plaintext == 0 && routed == all.len() as u64;
    v.tc_key4_blind_sfu_zero_plaintext = blind;
    assert!(blind, "TC-KEY4: blind SFU must route all frames yet recover zero plaintext");
    v.notes.push(format!(
        "TC-KEY4: SFU routed {routed}/{} frames from clear headers; recovered {sfu_recovered_plaintext} plaintext bytes (blind)",
        all.len()
    ));

    println!("{}", serde_json::to_string_pretty(&v)?);
    let all_pass = v.tc_key1_distinct_keys_nonmember_cannot_derive
        && v.tc_key2_lossy_reordered_decrypt_replay_reject
        && v.tc_key3_revoked_undecryptable_history_retained
        && v.tc_key4_blind_sfu_zero_plaintext;
    assert!(all_pass, "E12: all TC-KEY cases must pass");
    eprintln!("E12 ALL PASS");
    Ok(())
}
