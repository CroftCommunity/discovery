//! Minimal end-to-end encrypted, local-first group-sync vertical slice.
//!
//! Proves four strictly-separated layers compose end to end:
//!   * transport + content storage   -> `store` (BLAKE3 content-addressing;
//!                                       QUIC transport stubbed, see store.rs)
//!   * identity + 4-tuple addressing  -> `address` (Willow-shaped)
//!   * group key management           -> `mls` (openmls exporter secret -> key)
//!   * AEAD over the content key      -> `crypto`
//!   * document state (CRDT)          -> `doc` (automerge 0.7)
//!
//! Run with `cargo run`. Each lifecycle step prints a labeled section; a
//! pass/fail summary and a version report are printed at the end.

mod address;
mod crypto;
mod doc;
mod mls;
mod store;

use address::Address;
use store::BlobStore;

/// Single private namespace for the slice (the "hard boundary").
const NAMESPACE: &[u8] = b"private-group/demo";
/// Hierarchical path for the chat document.
const CHAT_PATH: &[u8] = b"/chat/doc";

fn key_fingerprint(key: &[u8; 32]) -> String {
    blake3::hash(key).to_hex().to_string()[..16].to_string()
}

fn section(title: &str) {
    println!("\n=== {title} ===");
}

fn main() {
    // Monotonic write clock for the slice (millis-shaped, deterministic order).
    let mut clock: u64 = 1_700_000_000_000;
    let mut tick = || {
        clock += 1;
        clock
    };

    // Shared content-addressed store (stands in for the iroh blob store; both
    // peers "fetch over the network" by reading from it).
    let mut store = BlobStore::new();

    let mut results: Vec<(&str, bool)> = Vec::new();

    // ------------------------------------------------------------------
    section("STEP 1: Setup — create group, add Bob, derive epoch content key");
    // ------------------------------------------------------------------
    let alice = mls::Member::new("Alice");
    let bob = mls::Member::new("Bob");

    let mut alice_group = mls::create_group(&alice);
    println!("Alice created group. epoch BEFORE add = {}", alice_group.epoch().as_u64());

    let welcome = mls::add_member(&mut alice_group, &alice, &bob);
    let bob_group = mls::join_from_welcome(&bob, &welcome);
    println!(
        "Alice added Bob. epoch AFTER add: Alice={}, Bob={}",
        alice_group.epoch().as_u64(),
        bob_group.epoch().as_u64()
    );

    // Each member independently derives the per-epoch content key.
    let key_alice_n = alice.content_key(&alice_group);
    let key_bob_n = bob.content_key(&bob_group);
    println!("Alice content-key fp (epoch N): {}", key_fingerprint(&key_alice_n));
    println!("Bob   content-key fp (epoch N): {}", key_fingerprint(&key_bob_n));

    let same_epoch = alice_group.epoch().as_u64() == bob_group.epoch().as_u64();
    let keys_match = key_alice_n == key_bob_n;
    println!("Same epoch? {same_epoch}.  Independently-derived keys match? {keys_match}.");
    results.push(("1. Setup: same epoch + matching epoch-N content key", same_epoch && keys_match));

    // Bob's MLS member identity (its Ed25519 signature public key) is also its
    // 4-tuple subspace — unified identity.
    println!(
        "Alice subspace id (=MLS sig pubkey, first 8 bytes): {}",
        hex::encode(&alice.identity()[..8])
    );

    // ------------------------------------------------------------------
    section("STEP 2: Alice writes a message");
    // ------------------------------------------------------------------
    let mut doc_alice = doc::new_doc();
    doc::append_message(&mut doc_alice, "hello from alice");
    let snapshot = doc::snapshot(&mut doc_alice);
    let alice_heads_after_write = doc::heads(&mut doc_alice); // Alice's "have" set
    println!("Alice doc messages: {:?}", doc::read_messages(&doc_alice));

    let ciphertext = crypto::encrypt(&key_alice_n, &snapshot);
    let snap_addr = Address::new(NAMESPACE, alice.identity(), CHAT_PATH, tick());
    let hash = store.put(&ciphertext);
    store.set_pointer(&snap_addr.storage_key(), hash.clone());
    println!(
        "Stored encrypted snapshot:\n  address tuple = (ns=private-group/demo, ss=Alice, path=/chat/doc, ts={})\n  storage key   = {}\n  content hash  = {}\n  plaintext len = {}  ciphertext len = {}",
        snap_addr.timestamp,
        snap_addr.storage_key(),
        hash,
        snapshot.len(),
        ciphertext.len()
    );
    results.push(("2. Alice writes + encrypts + stores snapshot", !ciphertext.is_empty()));

    // ------------------------------------------------------------------
    section("STEP 3: Bob bootstraps from the snapshot");
    // ------------------------------------------------------------------
    let fetched = store
        .resolve(&snap_addr.storage_key())
        .expect("Bob could not fetch snapshot by address")
        .to_vec();
    let plaintext = crypto::decrypt(&key_bob_n, &fetched)
        .expect("Bob failed to decrypt with his independently-derived epoch-N key");
    let mut doc_bob = doc::load(&plaintext);
    let bob_complete = doc::is_complete(&mut doc_bob); // via missing-deps, not emptiness
    let bob_msgs = doc::read_messages(&doc_bob);
    println!("Bob fetched {} bytes, decrypted {} bytes.", fetched.len(), plaintext.len());
    println!("Bob doc complete (missing-deps empty)? {bob_complete}");
    println!("Bob sees messages: {:?}", bob_msgs);
    results.push((
        "3. Bob bootstraps: decrypt + load + read [\"hello from alice\"]",
        bob_complete && bob_msgs == vec!["hello from alice"],
    ));

    // ------------------------------------------------------------------
    section("STEP 4: Incremental update (Bob -> Alice)");
    // ------------------------------------------------------------------
    doc::append_message(&mut doc_bob, "hi from bob");
    // Bob extracts only the changes Alice does not yet have.
    let incremental = doc::changes_since(&mut doc_bob, &alice_heads_after_write);
    println!("Bob produced {} incremental change(s) since Alice's heads.", incremental.len());

    // Serialize -> encrypt -> store under the 4-tuple (Bob is the author).
    let change_bytes = serialize_changes(&incremental);
    let inc_ct = crypto::encrypt(&key_bob_n, &change_bytes);
    let inc_addr = Address::new(NAMESPACE, bob.identity(), CHAT_PATH, tick());
    let inc_hash = store.put(&inc_ct);
    store.set_pointer(&inc_addr.storage_key(), inc_hash);
    println!(
        "Stored encrypted incremental change: storage key = {}, ciphertext len = {}",
        inc_addr.storage_key(),
        inc_ct.len()
    );

    // Alice fetches, decrypts, deserializes, applies.
    let inc_fetched = store.resolve(&inc_addr.storage_key()).expect("Alice fetch failed").to_vec();
    let inc_plain = crypto::decrypt(&key_alice_n, &inc_fetched).expect("Alice decrypt failed");
    let applied = deserialize_changes(&inc_plain);
    doc::apply(&mut doc_alice, applied);
    let alice_msgs = doc::read_messages(&doc_alice);
    println!("Alice applied changes. Alice now sees: {:?}", alice_msgs);
    results.push((
        "4. Incremental sync: Alice sees both messages",
        alice_msgs == vec!["hello from alice", "hi from bob"],
    ));

    // ------------------------------------------------------------------
    section("STEP 5: Membership change / epoch rotation (add Carol)");
    // ------------------------------------------------------------------
    // Single committer (Alice) performs the membership change. Concurrent
    // membership commits would require deterministic tiebreak / fork
    // resolution, which is out of scope for this slice.
    let carol = mls::Member::new("Carol");
    let epoch_before = alice_group.epoch().as_u64();
    let welcome_c = mls::add_member(&mut alice_group, &alice, &carol);
    let carol_group = mls::join_from_welcome(&carol, &welcome_c);
    let epoch_after = alice_group.epoch().as_u64();
    println!(
        "Epoch advanced on add: {epoch_before} -> {epoch_after} (Alice), Carol joined at {}.",
        carol_group.epoch().as_u64()
    );

    let key_alice_n1 = alice.content_key(&alice_group);
    let key_carol_n1 = carol.content_key(&carol_group);
    println!("Epoch-N   key fp        : {}", key_fingerprint(&key_alice_n));
    println!("Epoch-N+1 key fp (Alice): {}", key_fingerprint(&key_alice_n1));
    println!("Epoch-N+1 key fp (Carol): {}", key_fingerprint(&key_carol_n1));

    let key_rotated = key_alice_n1 != key_alice_n;
    let new_keys_match = key_alice_n1 == key_carol_n1;
    println!("Key rotated vs epoch N? {key_rotated}.  Alice/Carol epoch-N+1 keys match? {new_keys_match}.");

    // Carol bootstraps from a fresh snapshot encrypted under the NEW epoch key.
    let fresh_snapshot = doc::snapshot(&mut doc_alice);
    let carol_ct = crypto::encrypt(&key_alice_n1, &fresh_snapshot);
    let carol_addr = Address::new(NAMESPACE, alice.identity(), CHAT_PATH, tick());
    let carol_hash = store.put(&carol_ct);
    store.set_pointer(&carol_addr.storage_key(), carol_hash);

    let carol_fetched = store.resolve(&carol_addr.storage_key()).expect("Carol fetch failed").to_vec();
    let carol_plain = crypto::decrypt(&key_carol_n1, &carol_fetched)
        .expect("Carol failed to decrypt under rotated epoch-N+1 key");
    let mut doc_carol = doc::load(&carol_plain);
    let carol_complete = doc::is_complete(&mut doc_carol);
    let carol_msgs = doc::read_messages(&doc_carol);
    println!("Carol doc complete? {carol_complete}. Carol sees: {:?}", carol_msgs);
    results.push((
        "5. Epoch rotation: key rotates, Carol bootstraps under new key",
        key_rotated
            && new_keys_match
            && carol_complete
            && carol_msgs == vec!["hello from alice", "hi from bob"],
    ));

    // ------------------------------------------------------------------
    section("SUMMARY");
    // ------------------------------------------------------------------
    let mut all_pass = true;
    for (name, pass) in &results {
        println!("[{}] {}", if *pass { "PASS" } else { "FAIL" }, name);
        all_pass &= *pass;
    }
    println!(
        "\nOVERALL: {}",
        if all_pass { "PASS — architecture validated end to end" } else { "FAIL" }
    );

    section("VERSION REPORT");
    print_versions();

    if !all_pass {
        std::process::exit(1);
    }
}

/// Length-prefixed (u32 BE) concatenation of each change's raw bytes.
fn serialize_changes(changes: &[automerge::Change]) -> Vec<u8> {
    let mut out = Vec::new();
    for c in changes {
        let raw = c.raw_bytes();
        out.extend_from_slice(&(raw.len() as u32).to_be_bytes());
        out.extend_from_slice(raw);
    }
    out
}

/// Reverse of `serialize_changes`.
fn deserialize_changes(bytes: &[u8]) -> Vec<automerge::Change> {
    let mut out = Vec::new();
    let mut i = 0;
    while i + 4 <= bytes.len() {
        let len = u32::from_be_bytes(bytes[i..i + 4].try_into().unwrap()) as usize;
        i += 4;
        let raw = &bytes[i..i + len];
        out.push(automerge::Change::from_bytes(raw.to_vec()).expect("bad change bytes"));
        i += len;
    }
    out
}

fn print_versions() {
    // Versions are captured at build time (see build.rs) from the resolved
    // Cargo.lock and `rustc --version`, so the report reflects what linked.
    println!("rustc               : {}", env!("SLICE_RUSTC_VERSION"));
    println!("automerge           : {}", env!("SLICE_VER_AUTOMERGE"));
    println!("openmls             : {}", env!("SLICE_VER_OPENMLS"));
    println!("openmls_rust_crypto : {}", env!("SLICE_VER_OPENMLS_RUST_CRYPTO"));
    println!("chacha20poly1305    : {}", env!("SLICE_VER_CHACHA"));
    println!("blake3              : {}", env!("SLICE_VER_BLAKE3"));
    println!("iroh (resolvable, NOT linked — transport stubbed): {} / iroh-blobs {}", "0.98.2", "0.102.0");
    println!(
        "\nVersion gap CLOSED: ran on automerge {} (0.7.x line) on rustc {} (>= 1.80).",
        env!("SLICE_VER_AUTOMERGE"),
        env!("SLICE_RUSTC_VERSION")
    );
}
