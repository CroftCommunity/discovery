//! Standalone experiment: encrypted file/photo share in a private group over
//! real `iroh-blobs`.
//!
//! The single most important outcome: a real photo, encrypted under an
//! MLS-exporter epoch key, stored and transferred byte-for-byte over real
//! `iroh-blobs` QUIC **as ciphertext**, referenced from an Automerge document by
//! its **ciphertext** BLAKE3 hash, is fetched by a second group member and
//! decrypted back to a plaintext whose hash matches the original exactly — with
//! the blob store and transport never seeing plaintext.
//!
//! ## The encrypt-then-content-address property
//!
//! Because we encrypt *before* the bytes enter the content-addressed store, the
//! BLAKE3 hash is of the **ciphertext**, not the plaintext. Consequences,
//! demonstrated below:
//!   * the store/transport only ever see ciphertext;
//!   * the AEAD nonce must travel in the CRDT reference record (the hash alone is
//!     not enough to recover plaintext);
//!   * the key never travels anywhere — members hold it via MLS;
//!   * two members encrypting the same photo under different nonces produce
//!     different ciphertext hashes, so cross-user dedup is lost. That is the
//!     genuine tradeoff of doing encryption right.

mod aead;
mod blobnet;
mod doc;
mod lexicon;
mod mls;

use anyhow::Context;
use blobnet::{Fetcher, Provider};
use doc::{AttachmentAddress, AttachmentRef, GroupDoc};

const GROUP_ID: &str = "private-group-001";
const MIME: &str = "image/png";

fn hex_encode(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        s.push_str(&format!("{b:02x}"));
    }
    s
}

fn hex_decode_nonce(s: &str) -> anyhow::Result<[u8; aead::NONCE_LEN]> {
    anyhow::ensure!(s.len() == aead::NONCE_LEN * 2, "nonce hex wrong length");
    let mut out = [0u8; aead::NONCE_LEN];
    for (i, byte) in out.iter_mut().enumerate() {
        *byte = u8::from_str_radix(&s[i * 2..i * 2 + 2], 16).context("bad nonce hex")?;
    }
    Ok(out)
}

fn blake3_hex(bytes: &[u8]) -> String {
    blake3::hash(bytes).to_hex().to_string()
}

fn now_unix() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("=== Encrypted file/photo share in a private group over REAL iroh-blobs ===\n");

    // ----------------------------------------------------------------------
    // CHECKPOINT: trivial iroh-blobs add/get round-trip BEFORE the feature.
    // ----------------------------------------------------------------------
    println!("[checkpoint] trivial iroh-blobs add/get round-trip");
    {
        let provider = Provider::spawn().await.context("spawn checkpoint provider")?;
        let h = provider
            .add_ciphertext(b"hello iroh-blobs".to_vec())
            .await?;
        let addr = provider.addr().await;
        let fetcher = Fetcher::spawn().await?;
        let got = fetcher.fetch(addr, h).await?;
        anyhow::ensure!(got == b"hello iroh-blobs", "round-trip bytes mismatch");
        println!(
            "  add/get OK: stored & fetched {} bytes, hash {}\n",
            got.len(),
            h.to_hex()
        );
        fetcher.shutdown().await?;
        provider.shutdown().await?;
    }

    // ----------------------------------------------------------------------
    // STEP 1: Group setup (Alice creates group, adds Bob; both derive key).
    // ----------------------------------------------------------------------
    println!("[step 1] MLS group setup + per-epoch content key derivation");
    let alice = mls::Member::new("alice")?;
    let bob = mls::Member::new("bob")?;

    let mut alice_group = mls::create_group(&alice)?;
    let bob_kp = bob.key_package()?;
    let add = mls::add_member(&mut alice_group, &alice, bob_kp)?;
    let bob_group = mls::join_from_welcome(&bob, &add.welcome)?;

    let alice_key = alice.content_key(&alice_group)?;
    let bob_key = bob.content_key(&bob_group)?;
    let alice_key_hash = blake3_hex(&alice_key);
    let bob_key_hash = blake3_hex(&bob_key);
    println!("  members: {} (creator), {}", alice.name, bob.name);
    println!("  epoch: {}", alice_group.epoch().as_u64());
    println!("  {} content-key hash: {alice_key_hash}", alice.name);
    println!("  {}   content-key hash: {bob_key_hash}", bob.name);
    anyhow::ensure!(
        alice_key == bob_key,
        "epoch content keys differ between members!"
    );
    println!("  PASS: both members derived an identical epoch content key\n");

    // ----------------------------------------------------------------------
    // STEP 2: Take a real binary (the photo).
    // ----------------------------------------------------------------------
    println!("[step 2] load real binary");
    let asset = format!("{}/assets/sample-photo.png", env!("CARGO_MANIFEST_DIR"));
    let plaintext = std::fs::read(&asset).with_context(|| format!("read {asset}"))?;
    let plaintext_hash = blake3_hex(&plaintext);
    println!("  source: real PNG asset ({asset})");
    println!("  plaintext size: {} bytes", plaintext.len());
    println!("  plaintext BLAKE3: {plaintext_hash}\n");

    // ----------------------------------------------------------------------
    // STEP 3: Encrypt under the epoch content key with a fresh nonce.
    // ----------------------------------------------------------------------
    println!("[step 3] encrypt under epoch content key (ChaCha20-Poly1305)");
    let (nonce, ciphertext) = aead::encrypt(&alice_key, &plaintext)?;
    println!("  nonce: {}", hex_encode(&nonce));
    println!("  ciphertext size: {} bytes (plaintext + 16-byte AEAD tag)\n", ciphertext.len());

    // ----------------------------------------------------------------------
    // STEP 4: Store ciphertext in real iroh-blobs MemStore.
    // ----------------------------------------------------------------------
    println!("[step 4] store CIPHERTEXT in real iroh-blobs (MemStore)");
    let provider = Provider::spawn().await.context("spawn provider")?;
    let ct_hash = provider.add_ciphertext(ciphertext.clone()).await?;
    let ct_hash_hex = ct_hash.to_hex();
    println!("  ciphertext BLAKE3 (from store): {ct_hash_hex}");
    anyhow::ensure!(
        ct_hash_hex != plaintext_hash,
        "ciphertext hash unexpectedly equals plaintext hash!"
    );
    println!("  PASS: ciphertext hash != plaintext hash (encrypt-then-address)\n");

    // ----------------------------------------------------------------------
    // STEP 5: Author the attachment reference record in Automerge.
    // ----------------------------------------------------------------------
    println!("[step 5] author attachment reference record (Automerge + lexicon)");
    let rkey = "3l5xqphoto001";
    let att = AttachmentRef::new(
        ct_hash_hex.to_string(),
        hex_encode(&nonce),
        MIME,
        plaintext.len() as u64,
        Some("sample-photo.png".to_string()),
    );
    lexicon::validate(&att).context("lexicon validation rejected record")?;
    let addr_4tuple = AttachmentAddress::new(GROUP_ID, "alice", rkey, now_unix());
    let mut group_doc = GroupDoc::new()?;
    group_doc.put_attachment(rkey, &att)?;
    println!(
        "  4-tuple address: namespace={} subspace={} path={} timestamp={}",
        addr_4tuple.namespace, addr_4tuple.subspace, addr_4tuple.path, addr_4tuple.timestamp
    );
    println!("  record (atproto-blob-style; ref=ciphertext hash, +nonce, no key):");
    println!("{}\n", serde_json::to_string_pretty(&att)?);

    // ----------------------------------------------------------------------
    // STEP 6: Transfer the ciphertext over REAL iroh QUIC.
    // ----------------------------------------------------------------------
    println!("[step 6] transfer ciphertext over REAL iroh QUIC");
    let provider_addr = provider.addr().await;
    println!("  provider endpoint id: {}", provider_addr.id);
    println!(
        "  provider direct addresses ({}): {:?}",
        provider_addr.addrs.len(),
        provider_addr.addrs
    );
    println!("  connection: Bob dials Alice's full EndpointAddr directly (no DNS/relay needed)");
    let fetcher = Fetcher::spawn().await.context("spawn fetcher")?;
    let fetched_ct = fetcher
        .fetch(provider_addr, ct_hash)
        .await
        .context("fetch ciphertext over iroh")?;
    let fetched_ct_hash = blake3_hex(&fetched_ct);
    println!("  fetched {} bytes", fetched_ct.len());
    anyhow::ensure!(
        fetched_ct_hash == ct_hash_hex,
        "fetched bytes hash != stored ciphertext hash!"
    );
    println!("  PASS: transfer integrity confirmed (BLAKE3-verified streaming): {fetched_ct_hash}\n");

    // ----------------------------------------------------------------------
    // STEP 7: Bob reads the reference, decrypts, recovers the plaintext.
    // ----------------------------------------------------------------------
    println!("[step 7] Bob reconstructs the file");
    // Hand the document to Bob directly (the one simplification — see README).
    let doc_bytes = group_doc.save();
    let bob_doc = GroupDoc::load(&doc_bytes)?;
    let bob_att = bob_doc.get_attachment(rkey)?;
    lexicon::validate(&bob_att).context("Bob: lexicon validation failed")?;
    anyhow::ensure!(bob_att == att, "Bob read back a different record");
    let bob_nonce = hex_decode_nonce(&bob_att.nonce)?;
    let recovered = aead::decrypt(&bob_key, &bob_nonce, &fetched_ct)
        .context("Bob failed to decrypt")?;
    let recovered_hash = blake3_hex(&recovered);
    println!("  recovered {} bytes", recovered.len());
    println!("  recovered BLAKE3: {recovered_hash}");
    anyhow::ensure!(
        recovered_hash == plaintext_hash,
        "CORE ASSERTION FAILED: recovered plaintext hash != original!"
    );
    anyhow::ensure!(
        recovered.len() == plaintext.len(),
        "recovered size mismatch"
    );
    // Sanity: valid PNG of the original size (magic bytes).
    anyhow::ensure!(
        recovered.starts_with(&[0x89, b'P', b'N', b'G', 0x0d, 0x0a, 0x1a, 0x0a]),
        "recovered bytes are not a valid PNG"
    );
    println!("  PASS (CORE): recovered plaintext hash == original, valid PNG, byte-for-byte\n");

    // ----------------------------------------------------------------------
    // STEP 8: Epoch rotation on membership change (add Carol).
    // ----------------------------------------------------------------------
    println!("[step 8] epoch rotation: add Carol, rotate content key");
    let carol = mls::Member::new("carol")?;
    let carol_kp = carol.key_package()?;
    let add2 = mls::add_member(&mut alice_group, &alice, carol_kp)?;
    // Existing member Bob processes the commit to stay in sync (real epoch sync).
    let mut bob_group = bob_group;
    mls::process_commit(&mut bob_group, &bob, &add2.commit)?;
    let carol_group = mls::join_from_welcome(&carol, &add2.welcome)?;

    let new_alice_key = alice.content_key(&alice_group)?;
    let new_bob_key = bob.content_key(&bob_group)?;
    let new_carol_key = carol.content_key(&carol_group)?;
    println!("  new epoch: {}", alice_group.epoch().as_u64());
    println!("  new content-key hash: {}", blake3_hex(&new_alice_key));
    anyhow::ensure!(new_alice_key != alice_key, "epoch key did not rotate!");
    anyhow::ensure!(
        new_alice_key == new_carol_key && new_alice_key == new_bob_key,
        "members disagree on the new epoch key!"
    );
    println!("  PASS: key rotated, and Alice/Bob/Carol all derived the SAME new key");

    // New-epoch share: encrypt a fresh attachment under the new key; Carol fetches + decrypts.
    let (nonce2, ciphertext2) = aead::encrypt(&new_alice_key, &plaintext)?;
    let ct2_hash = provider.add_ciphertext(ciphertext2.clone()).await?;
    let provider_addr2 = provider.addr().await;
    let carol_fetcher = Fetcher::spawn().await?;
    let fetched_ct2 = carol_fetcher.fetch(provider_addr2, ct2_hash).await?;
    let carol_recovered = aead::decrypt(&new_carol_key, &nonce2, &fetched_ct2)
        .context("Carol failed to decrypt new-epoch blob")?;
    anyhow::ensure!(
        blake3_hex(&carol_recovered) == plaintext_hash,
        "Carol's new-epoch decrypt did not match original"
    );
    println!("  PASS: Carol fetched + decrypted a blob shared under the NEW epoch key");
    println!("  old-epoch rule: a blob encrypted under epoch N's key requires that retained key");
    println!("    to decrypt. Carol (joined at epoch N+1) cannot derive epoch N's exporter key,");
    println!("    so she could only open older attachments if the old content key were explicitly");
    println!("    retained and shared to her — the same forward-secrecy tradeoff that governs");
    println!("    message history.");

    // Demonstrate the old-epoch rule concretely: Carol's new key cannot open the
    // step-3 ciphertext (encrypted under the original epoch key).
    let carol_tries_old = aead::decrypt(&new_carol_key, &nonce, &ciphertext);
    anyhow::ensure!(
        carol_tries_old.is_err(),
        "Carol unexpectedly decrypted an old-epoch blob with the new key"
    );
    println!("  (verified: Carol's new-epoch key cannot decrypt the old-epoch ciphertext)\n");

    // Cleanup.
    carol_fetcher.shutdown().await?;
    fetcher.shutdown().await?;
    provider.shutdown().await?;

    println!("=== ALL STEPS PASSED ===");
    Ok(())
}
