//! End-to-end: many signers contribute to one card through the content-blind ingest, the recipient
//! is blind until the scheduled reveal, and only the key holder (via the URL-fragment key) can read.
//!
//! The client role uses `card-seal` (a DEV dependency). The service (`card-service`) has no such
//! dependency on its normal edges, which is what makes content-blindness a compile fact. The
//! ciphertext envelope stored by the service is `nonce (12 bytes) || AEAD ciphertext` — carrying the
//! nonce but never the key, exactly the "ref carries hash + nonce, no key" shape the encrypted-blob-
//! share spike validated.

use card_service::{Bearer, Ciphertext, Collection, Ingest, Reveal, ViewerId, WriteTarget};

const K: [u8; 32] = [42u8; 32]; // the URL-fragment key; lives only with link holders, never the server

fn coll() -> Collection {
    Collection("ing.croft.card.entry".into())
}

/// Client side: seal plaintext into the stored envelope (nonce || ciphertext).
fn client_seal(nonce: [u8; 12], plaintext: &[u8]) -> Ciphertext {
    let mut env = nonce.to_vec();
    env.extend_from_slice(&card_seal::seal(&K, &nonce, plaintext).expect("seal"));
    Ciphertext(env)
}

/// Client side: split the envelope and open with the key.
fn client_open(env: &Ciphertext) -> Result<Vec<u8>, card_seal::SealError> {
    let (nonce, ct) = env.0.split_at(12);
    let nonce: [u8; 12] = nonce.try_into().expect("nonce len");
    card_seal::open(&K, &nonce, ct)
}

#[test]
fn card_flow_content_blind_and_blind_until_reveal() {
    let bearer = Bearer("2f9c-unguessable-card-link".into());
    let ingest = Ingest::new(bearer.clone());
    let mut store = card_service::InMemoryStore::new();

    // Two signers add entries with no login — they just hold the link (bearer). Each seals
    // client-side; the service only ever sees ciphertext.
    let e1 = client_seal([1u8; 12], b"happy birthday, Carol!");
    let e2 = client_seal([2u8; 12], b"we miss you at the shop");
    let r1 = ingest.append(&mut store, &bearer, &coll(), &e1).expect("signer 1 appends");
    let r2 = ingest.append(&mut store, &bearer, &coll(), &e2).expect("signer 2 appends");
    assert_ne!(r1, r2);

    // What the service holds is opaque ciphertext, not the messages.
    for (_, stored) in store.list(&coll()) {
        assert!(!contains(&stored.0, b"birthday"), "service must not hold plaintext");
        assert!(!contains(&stored.0, b"shop"), "service must not hold plaintext");
    }

    let reveal = Reveal::new(100, ViewerId("did:plc:recipient".into()));

    // Blind until reveal: too early is withheld...
    assert!(reveal.offer(&store, &coll(), 50, &ViewerId("did:plc:recipient".into())).is_err());
    // ...and a wrong viewer at reveal time is withheld with the same opaque error (no leak).
    assert!(reveal.offer(&store, &coll(), 100, &ViewerId("did:plc:intruder".into())).is_err());

    // At the reveal time, the recipient is offered the ciphertext (still encrypted).
    let offered = reveal
        .offer(&store, &coll(), 100, &ViewerId("did:plc:recipient".into()))
        .expect("recipient offered at reveal");
    assert_eq!(offered.len(), 2);

    // Only the key holder can read. The recipient holds K (it was in the link).
    let mut messages: Vec<Vec<u8>> = offered.iter().map(|(_, e)| client_open(e).expect("open")).collect();
    messages.sort();
    assert_eq!(messages[0], b"happy birthday, Carol!");
    assert_eq!(messages[1], b"we miss you at the shop");
}

#[test]
fn offered_ciphertext_is_useless_without_the_key() {
    // Even if a party obtains the offered ciphertext, without K it cannot be read. This is why the
    // reveal offering ciphertext (never plaintext) is safe: the gate controls *delivery*, the key
    // controls *reading*, and they are independent layers.
    let bearer = Bearer("link".into());
    let ingest = Ingest::new(bearer.clone());
    let mut store = card_service::InMemoryStore::new();
    let env = client_seal([9u8; 12], b"secret note");
    ingest.append(&mut store, &bearer, &coll(), &env).expect("append");

    let stolen = store.get(&coll(), &card_service::content_address(&env)).expect("ciphertext");
    // Tamper the key: a non-holder trying to read.
    let (nonce, ct) = stolen.0.split_at(12);
    let nonce: [u8; 12] = nonce.try_into().unwrap();
    let wrong_key = [0u8; 32];
    assert_eq!(
        card_seal::open(&wrong_key, &nonce, ct),
        Err(card_seal::SealError::Open),
        "ciphertext is unreadable without the fragment key"
    );
}

fn contains(haystack: &[u8], needle: &[u8]) -> bool {
    haystack.windows(needle.len()).any(|w| w == needle)
}
