//! Capability probes for the collaborative-ingest model (E43). Each test answers one "what can/can't
//! this model do, and what does it trust" question, so the reasoning is executable, not asserted in
//! prose. Findings are summarized in ../CAPABILITIES.md.
//!
//! These are characterization tests over the built behavior (CAP-1/2/3/5) plus the authorship
//! capability from card-sign (CAP-4).

use card_service::{
    content_address, Bearer, Ciphertext, Collection, Ingest, InMemoryStore, Reveal, ViewerId,
    WriteTarget,
};

const K1: [u8; 32] = [11u8; 32];
const K2: [u8; 32] = [22u8; 32];

fn coll() -> Collection {
    Collection("ing.croft.card.entry".into())
}
fn seal_env(key: &[u8; 32], nonce: [u8; 12], pt: &[u8]) -> Ciphertext {
    let mut env = nonce.to_vec();
    env.extend_from_slice(&card_seal::seal(key, &nonce, pt).expect("seal"));
    Ciphertext(env)
}

// CAP-1 -- What does a content-blind writer STILL learn from what it stores?
// Answer: entry count, per-entry byte length (=> message-length band), and byte-identical
// duplicate detection (identical ciphertext -> identical content address). It cannot learn content.
// Nonce discipline controls the plaintext-duplicate leak: identical plaintext under a UNIQUE nonce
// yields different ciphertext, so it does NOT collide; identical plaintext under a REUSED nonce does.
#[test]
fn cap1_metadata_residue_of_a_blind_writer() {
    let ingest = Ingest::new(Bearer("link".into()));
    let mut store = InMemoryStore::new();

    let a = seal_env(&K1, [1u8; 12], b"congrats!");
    let b_reused_nonce = seal_env(&K1, [1u8; 12], b"congrats!"); // same plaintext, SAME nonce
    let c_unique_nonce = seal_env(&K1, [2u8; 12], b"congrats!"); // same plaintext, DIFFERENT nonce
    let d = seal_env(&K1, [3u8; 12], b"a much longer heartfelt message");

    for e in [&a, &b_reused_nonce, &c_unique_nonce, &d] {
        ingest.append(&mut store, &Bearer("link".into()), &coll(), e).expect("append");
    }

    // Duplicate-by-reused-nonce collapses to one content address: the blind writer learns a==b.
    assert_eq!(content_address(&a), content_address(&b_reused_nonce), "reused nonce -> ref collision (leak)");
    assert_ne!(content_address(&a), content_address(&c_unique_nonce), "unique nonce -> no collision");

    let listed = store.list(&coll());
    // Count is observable (3 distinct records: a==b collapsed).
    assert_eq!(listed.len(), 3, "the writer learns how many distinct contributions exist");
    // Per-entry length is observable (=> a message-length band; mitigate by padding).
    let lens: Vec<usize> = listed.iter().map(|(_, ct)| ct.0.len()).collect();
    assert!(lens.iter().max() > lens.iter().min(), "byte lengths differ and are observable");
    // But content is NOT observable: no plaintext survives in what the writer holds.
    for (_, ct) in &listed {
        assert!(!contains(&ct.0, b"congrats"), "the writer never holds plaintext");
        assert!(!contains(&ct.0, b"heartfelt"), "the writer never holds plaintext");
    }
}

// CAP-2 -- What does the bearer link authorize? Append only. Content addressing binds ref<->content,
// so a link holder cannot clobber another entry (different bytes always get a different ref), and
// re-appending identical bytes is idempotent. There is no edit/delete on the surface at all.
#[test]
fn cap2_bearer_authorizes_append_only_content_addressed() {
    let ingest = Ingest::new(Bearer("link".into()));
    let mut store = InMemoryStore::new();

    let x = seal_env(&K1, [1u8; 12], b"entry X");
    let rx = ingest.append(&mut store, &Bearer("link".into()), &coll(), &x).expect("append X");

    // Appending different content yields a DIFFERENT ref; it cannot overwrite X's content.
    let y = seal_env(&K1, [2u8; 12], b"entry Y");
    let ry = ingest.append(&mut store, &Bearer("link".into()), &coll(), &y).expect("append Y");
    assert_ne!(rx, ry);
    assert_eq!(store.get(&coll(), &rx), Some(x.clone()), "X is untouched by appending Y (immutable by ref)");

    // Re-appending identical bytes is idempotent: same ref, store does not grow.
    let before = store.list(&coll()).len();
    let rx2 = ingest.append(&mut store, &Bearer("link".into()), &coll(), &x).expect("re-append X");
    assert_eq!(rx2, rx);
    assert_eq!(store.list(&coll()).len(), before, "idempotent re-append, no growth");

    // The wrong bearer authorizes nothing.
    assert!(ingest.append(&mut store, &Bearer("guess".into()), &coll(), &x).is_err());
    // (Edit/delete are absent from the Ingest surface by construction: the link is an append
    // capability. Delete over a real PDS collection would be a separate scope the shim must withhold.)
}

// CAP-3 -- What does the reveal TRUST? The clock source. The gate is now >= reveal_at; whoever
// supplies `now` controls it. If a viewer could supply their own `now`, they bypass the reveal.
// So `now` MUST come from a trusted authority (the service clock), never a viewer claim. The gate is
// also monotonic: once open it stays open for that reveal_at (un-reveal requires a new Reveal, an
// organizer capability).
#[test]
fn cap3_reveal_trusts_the_clock_source() {
    let ingest = Ingest::new(Bearer("link".into()));
    let mut store = InMemoryStore::new();
    ingest.append(&mut store, &Bearer("link".into()), &coll(), &seal_env(&K1, [1u8; 12], b"surprise")).unwrap();
    let recipient = ViewerId("did:recipient".into());
    let reveal = Reveal::new(100, recipient.clone());

    // Authority-supplied time gates correctly.
    assert!(reveal.offer(&store, &coll(), 50, &recipient).is_err(), "too early: withheld");
    assert!(reveal.offer(&store, &coll(), 100, &recipient).is_ok(), "at reveal: offered");

    // The trust dependency, made explicit: the gate is a pure function of the `now` it is handed. If
    // that `now` were a VIEWER claim, the viewer could pass reveal_at and open early. This is why the
    // clock must be authority-supplied, not viewer-supplied.
    let viewer_claimed_now = 100; // a malicious viewer claiming it is already reveal time
    let real_time = 50;
    assert!(reveal.offer(&store, &coll(), viewer_claimed_now, &recipient).is_ok());
    assert!(reveal.offer(&store, &coll(), real_time, &recipient).is_err());

    // Monotonic: still open at any later authority time.
    assert!(reveal.offer(&store, &coll(), 100_000, &recipient).is_ok());
}

// CAP-4 -- Authorship is an OPT-IN capability, orthogonal to confidentiality. A signature over the
// ciphertext binds authorship and is verifiable WITHOUT the decryption key (content-blind-safe). The
// no-login/bearer path has no such binding, so any link holder can append content under any name.
#[test]
fn cap4_authorship_is_opt_in_and_content_blind_verifiable() {
    let ingest = Ingest::new(Bearer("link".into()));
    let mut store = InMemoryStore::new();

    let alice = card_sign::Author::from_seed(&[1u8; 32]);
    let mallory = card_sign::Author::from_seed(&[9u8; 32]);

    // Alice seals a contribution and signs the CIPHERTEXT (not the plaintext).
    let env = seal_env(&K1, [1u8; 12], b"from Alice");
    let sig = alice.sign(&env.0);
    ingest.append(&mut store, &Bearer("link".into()), &coll(), &env).expect("append");

    // A reader verifies authorship over the stored ciphertext with NO decryption key -> content-blind-safe.
    let (_, stored) = store.list(&coll()).into_iter().next().unwrap();
    assert!(card_sign::verify(&alice.public_key(), &stored.0, &sig), "genuine authorship verifies over ciphertext");

    // Forgery fails: Mallory cannot pass off her signature as Alice's, and a tampered ciphertext breaks it.
    assert!(!card_sign::verify(&alice.public_key(), &stored.0, &mallory.sign(&stored.0)));
    let mut tampered = stored.0.clone();
    tampered[13] ^= 0xff;
    assert!(!card_sign::verify(&alice.public_key(), &tampered, &sig));

    // The bearer/no-login path carries no authorship: the service accepts any link-authorized append
    // regardless of who "wrote" it, so an unsigned entry is impersonatable by construction.
    let impersonation = seal_env(&K1, [7u8; 12], b"also from Alice (actually Mallory)");
    assert!(
        ingest.append(&mut store, &Bearer("link".into()), &coll(), &impersonation).is_ok(),
        "bearer append is accepted with no authorship check -> impersonation is possible without signatures"
    );
}

// CAP-5 -- Revocation in the symmetric-key tier is ALL-OR-NOTHING. Rotating the key makes prior
// ciphertext unreadable under the new key, and there is no operation that revokes ONE holder while
// keeping others: the only lever is rotate-for-all + redistribute the link. Selective revocation is
// what escalates the design to the MLS-sealed tier.
#[test]
fn cap5_rotation_is_all_or_nothing_no_selective_revocation() {
    let nonce = [1u8; 12];
    let env = seal_env(&K1, nonce, b"members-only note");
    let (n, ct) = env.0.split_at(12);
    let n: [u8; 12] = n.try_into().unwrap();

    // Holders of K1 can read.
    assert!(card_seal::open(&K1, &n, ct).is_ok());
    // After rotating to K2, the old ciphertext is unreadable under the new key: rotation invalidates
    // everything sealed under the old key at once.
    assert_eq!(card_seal::open(&K2, &n, ct), Err(card_seal::SealError::Open));
    // There is no key that reads for holder-1 but not holder-2 among K1 holders: both hold K1, so
    // revoking one necessarily means moving to K2 (which neither old link can derive) and re-issuing.
    // (Demonstrated by the absence of any per-holder key operation: the capability simply is not here;
    // it lives in the MLS-sealed tier, which can remove one member with forward secrecy.)
}

fn contains(haystack: &[u8], needle: &[u8]) -> bool {
    haystack.windows(needle.len()).any(|w| w == needle)
}
