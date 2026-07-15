//! Phase 2: a minimal local-first "microblog + reactions" group feed whose data
//! respects AT Protocol lexicon schemas, built on the proven encrypted-CRDT core.
//!
//! The question this answers: does a local-first app whose records validate
//! against real atproto lexicons compose cleanly with the encrypted CRDT stack,
//! such that the same records could later be published to a PDS *without
//! reshaping the data*? No live PDS / OAuth / DID / relay is touched — only the
//! data-model interop is proven (Step 6 is the keystone assertion).
//!
//! Layers reused unchanged from `experiments/encrypted-sync-slice/`:
//!   store (BLAKE3 content addressing; QUIC transport still stubbed),
//!   address (Willow-shaped 4-tuple), mls (openmls exporter -> epoch key),
//!   crypto (ChaCha20-Poly1305), doc (automerge 0.7 plumbing).
//! New here: lexicon schemas + validator, record types, atproto repo model.

mod address;
mod crypto;
mod doc;
mod lexicon;
mod mls;
mod record;
mod repo;
mod store;

use std::collections::HashMap;

use address::Address;
use lexicon::Lexicon;
use record::{Post, Reaction, StrongRef, POST_NSID, REACTION_NSID};
use serde_json::Value;
use store::BlobStore;

/// The private group's namespace (the 4-tuple "hard boundary").
const NAMESPACE: &[u8] = b"org.croftc.experiment.group/demo";

fn key_fingerprint(key: &[u8; 32]) -> String {
    blake3::hash(key).to_hex().to_string()[..16].to_string()
}

fn section(title: &str) {
    println!("\n=== {title} ===");
}

/// 4-tuple path for a collection's encrypted sync payloads.
fn repo_path(collection: &str) -> Vec<u8> {
    format!("/repo/{collection}").into_bytes()
}

/// Validate a record value against a lexicon, printing the outcome.
fn check(lex: &Lexicon, record: &Value, label: &str) -> bool {
    match lex.validate(record) {
        Ok(()) => {
            println!("  [lexicon OK] {label} validates against {}", lex.id());
            true
        }
        Err(e) => {
            println!("  [lexicon FAIL] {label}: {e}");
            false
        }
    }
}

/// Plain-text rendering of the group feed from a reconstructed document.
fn render_feed(doc: &automerge::AutoCommit) -> String {
    let posts = repo::list_collection(doc, POST_NSID);
    let reactions = repo::list_collection(doc, REACTION_NSID);

    // Group reaction emojis by the rkey of the post they reference.
    let mut by_subject: HashMap<String, Vec<String>> = HashMap::new();
    for (_rk, json) in &reactions {
        let r: Reaction = serde_json::from_str(json).unwrap();
        let subject_rkey = r.subject.uri.rsplit('/').next().unwrap_or("").to_string();
        by_subject.entry(subject_rkey).or_default().push(r.emoji);
    }

    let mut out = String::new();
    for (rkey, json) in &posts {
        let p: Post = serde_json::from_str(json).unwrap();
        out.push_str(&format!("  • {}  ({})\n", p.text, p.created_at));
        if let Some(emojis) = by_subject.get(rkey) {
            out.push_str(&format!("      reactions: {}\n", emojis.join(" ")));
        }
    }
    out
}

fn main() {
    let mut clock: u64 = 1_700_000_000_000;
    let mut tick = || {
        clock += 1;
        clock
    };
    let mut store = BlobStore::new();
    let mut results: Vec<(&str, bool)> = Vec::new();

    let post_lex = Lexicon::load(lexicon::POST_LEXICON);
    let reaction_lex = Lexicon::load(lexicon::REACTION_LEXICON);

    // ------------------------------------------------------------------
    section("STEP 1: Group setup (reuse proven MLS path)");
    // ------------------------------------------------------------------
    let alice = mls::Member::new("Alice");
    let bob = mls::Member::new("Bob");
    let mut alice_group = mls::create_group(&alice);
    let welcome = mls::add_member(&mut alice_group, &alice, &bob);
    let bob_group = mls::join_from_welcome(&bob, &welcome);

    let key_n_alice = alice.content_key(&alice_group);
    let key_n_bob = bob.content_key(&bob_group);
    println!("epoch (Alice/Bob) = {}/{}", alice_group.epoch().as_u64(), bob_group.epoch().as_u64());
    println!("content-key fp Alice={} Bob={}", key_fingerprint(&key_n_alice), key_fingerprint(&key_n_bob));
    let setup_ok = key_n_alice == key_n_bob;
    println!("epoch-N keys match? {setup_ok}");
    results.push(("1. Group setup: matching epoch-N content key", setup_ok));

    // Author authority for at:// URIs. A real DID would go here; we use the
    // member's MLS signature public key (the same bytes as the 4-tuple subspace).
    let authority = format!("did:key:z{}", hex::encode(&alice.identity()[..10]));

    // ------------------------------------------------------------------
    section("STEP 2: Alice posts a status (lexicon-validated)");
    // ------------------------------------------------------------------
    let mut doc_alice = doc::new_doc();

    let post = Post::new("local-first, lexicon-shaped, end-to-end encrypted 👋", Some(vec!["en".into()]));
    let post_json = serde_json::to_value(&post).unwrap();
    let post_ok = check(&post_lex, &post_json, "Alice's post");

    let post_rkey = record::new_tid();
    let post_json_str = serde_json::to_string(&post).unwrap();
    repo::put_record(&mut doc_alice, POST_NSID, &post_rkey, &post_json_str);
    println!("  stored at {POST_NSID}/{post_rkey}");

    let snap = doc::snapshot(&mut doc_alice);
    let alice_heads_after_post = doc::heads(&mut doc_alice);
    let ct = crypto::encrypt(&key_n_alice, &snap);
    let post_addr = Address::new(NAMESPACE, alice.identity(), repo_path(POST_NSID), tick());
    let h = store.put(&ct);
    store.set_pointer(&post_addr.storage_key(), h);
    println!("  encrypted snapshot stored: path=/repo/{POST_NSID} key={} ct_len={}", post_addr.storage_key(), ct.len());
    results.push(("2. Alice posts: lexicon-valid + encrypted + stored", post_ok));

    // ------------------------------------------------------------------
    section("STEP 3: Bob syncs, reads, and re-validates");
    // ------------------------------------------------------------------
    let fetched = store.resolve(&post_addr.storage_key()).expect("Bob fetch failed").to_vec();
    let plain = crypto::decrypt(&key_n_bob, &fetched).expect("Bob decrypt failed");
    let mut doc_bob = doc::load(&plain);
    let bob_complete = doc::is_complete(&mut doc_bob);
    let got = repo::get_record(&doc_bob, POST_NSID, &post_rkey).expect("post missing");
    let got_value: Value = serde_json::from_str(&got).unwrap();
    println!("  Bob reconstructed record:\n{}", serde_json::to_string_pretty(&got_value).unwrap());
    let revalidate_ok = check(&post_lex, &got_value, "received post");
    println!("  Bob doc complete (missing-deps)? {bob_complete}");
    results.push(("3. Bob syncs + re-validates received post", bob_complete && revalidate_ok && got_value["$type"] == serde_json::json!(POST_NSID)));

    // ------------------------------------------------------------------
    section("STEP 4: Bob reacts (incremental sync back to Alice)");
    // ------------------------------------------------------------------
    let subject = StrongRef {
        uri: record::at_uri(&authority, POST_NSID, &post_rkey),
        cid: record::cid_for(&post_json_str),
    };
    let reaction = Reaction::new(subject, "🎉");
    let reaction_json = serde_json::to_value(&reaction).unwrap();
    let reaction_ok = check(&reaction_lex, &reaction_json, "Bob's reaction");

    let reaction_rkey = record::new_tid();
    repo::put_record(&mut doc_bob, REACTION_NSID, &reaction_rkey, &serde_json::to_string(&reaction).unwrap());
    println!("  stored at {REACTION_NSID}/{reaction_rkey} -> subject {}", reaction.subject.uri);

    let changes = doc::changes_since(&mut doc_bob, &alice_heads_after_post);
    let inc_ct = crypto::encrypt(&key_n_bob, &doc::serialize_changes(&changes));
    let inc_addr = Address::new(NAMESPACE, bob.identity(), repo_path(REACTION_NSID), tick());
    let ih = store.put(&inc_ct);
    store.set_pointer(&inc_addr.storage_key(), ih);
    println!("  {} incremental change(s) encrypted+stored", changes.len());

    let inc_fetched = store.resolve(&inc_addr.storage_key()).expect("Alice fetch failed").to_vec();
    let inc_plain = crypto::decrypt(&key_n_alice, &inc_fetched).expect("Alice decrypt failed");
    doc::apply(&mut doc_alice, doc::deserialize_changes(&inc_plain));
    println!("  Alice's feed after applying Bob's reaction:\n{}", render_feed(&doc_alice));
    let step4_ok = reaction_ok
        && repo::list_collection(&doc_alice, POST_NSID).len() == 1
        && repo::list_collection(&doc_alice, REACTION_NSID).len() == 1;
    results.push(("4. Bob reacts: lexicon-valid + incremental sync to Alice", step4_ok));

    // ------------------------------------------------------------------
    section("STEP 5: Membership change / epoch rotation (add Carol)");
    // ------------------------------------------------------------------
    let carol = mls::Member::new("Carol");
    let welcome_c = mls::add_member(&mut alice_group, &alice, &carol);
    let carol_group = mls::join_from_welcome(&carol, &welcome_c);
    let key_n1_alice = alice.content_key(&alice_group);
    let key_n1_carol = carol.content_key(&carol_group);
    println!("epoch now {} ; key rotated? {} ; Alice/Carol new keys match? {}",
        alice_group.epoch().as_u64(),
        key_n1_alice != key_n_alice,
        key_n1_alice == key_n1_carol);

    // Carol bootstraps from a fresh snapshot under the NEW epoch key.
    let fresh = doc::snapshot(&mut doc_alice);
    let carol_ct = crypto::encrypt(&key_n1_alice, &fresh);
    let carol_addr = Address::new(NAMESPACE, alice.identity(), repo_path("snapshot"), tick());
    let ch = store.put(&carol_ct);
    store.set_pointer(&carol_addr.storage_key(), ch);

    let carol_fetched = store.resolve(&carol_addr.storage_key()).expect("Carol fetch failed").to_vec();
    let carol_plain = crypto::decrypt(&key_n1_carol, &carol_fetched).expect("Carol decrypt failed");
    let doc_carol = doc::load(&carol_plain);

    // Re-validate every record Carol received, across the epoch boundary.
    let mut all_valid = true;
    for (rk, json) in repo::list_collection(&doc_carol, POST_NSID) {
        let v: Value = serde_json::from_str(&json).unwrap();
        all_valid &= check(&post_lex, &v, &format!("post {rk}"));
    }
    for (rk, json) in repo::list_collection(&doc_carol, REACTION_NSID) {
        let v: Value = serde_json::from_str(&json).unwrap();
        all_valid &= check(&reaction_lex, &v, &format!("reaction {rk}"));
    }
    println!("  Carol's repo collections (mini-repo layout): {:?}", repo::collections(&doc_carol));
    println!("  Carol's feed:\n{}", render_feed(&doc_carol));
    let step5_ok = key_n1_alice != key_n_alice
        && key_n1_alice == key_n1_carol
        && all_valid
        && repo::list_collection(&doc_carol, POST_NSID).len() == 1
        && repo::list_collection(&doc_carol, REACTION_NSID).len() == 1;
    results.push(("5. Epoch rotation: Carol bootstraps + records re-validate", step5_ok));

    // ------------------------------------------------------------------
    section("STEP 6: INTEROP ASSERTION — ready for com.atproto.repo.createRecord");
    // ------------------------------------------------------------------
    // Take the post as Carol reconstructed it (locally authored -> validated ->
    // encrypted -> CRDT-synced -> epoch-rotated -> decrypted) and show it is the
    // exact `record` payload a createRecord call would carry.
    let (rkey, json) = repo::list_collection(&doc_carol, POST_NSID)
        .into_iter()
        .next()
        .expect("no post");
    let record_payload: Value = serde_json::from_str(&json).unwrap();
    let create_record_body = serde_json::json!({
        "repo": authority,
        "collection": POST_NSID,
        "rkey": rkey,
        "record": record_payload,
    });
    println!("The `record` payload (what POSTs to a PDS, unchanged):");
    println!("{}", serde_json::to_string_pretty(&record_payload).unwrap());
    println!("\nFull com.atproto.repo.createRecord input body (sketch):");
    println!("{}", serde_json::to_string_pretty(&create_record_body).unwrap());

    let interop_ok = post_lex.validate(&record_payload).is_ok()
        && record_payload["$type"] == serde_json::json!(POST_NSID);
    if interop_ok {
        println!(
            "\n>>> This record is a valid atproto lexicon record and could be published to a PDS\n>>> unchanged; only transport + auth (OAuth/identity, the createRecord call) would be added."
        );
    } else {
        println!("\n>>> INTEROP FAILED: reconstructed record did not validate.");
    }
    results.push(("6. Interop: synced record is createRecord-ready lexicon JSON", interop_ok));

    // ------------------------------------------------------------------
    section("SUMMARY");
    // ------------------------------------------------------------------
    let mut all_pass = true;
    for (name, pass) in &results {
        println!("[{}] {}", if *pass { "PASS" } else { "FAIL" }, name);
        all_pass &= *pass;
    }
    println!("\nOVERALL: {}", if all_pass { "PASS — local-first/atproto data-model interop validated" } else { "FAIL" });

    section("VERSION REPORT");
    println!("rustc               : {}", env!("SLICE_RUSTC_VERSION"));
    println!("automerge           : {}", env!("SLICE_VER_AUTOMERGE"));
    println!("openmls             : {}", env!("SLICE_VER_OPENMLS"));
    println!("openmls_rust_crypto : {}", env!("SLICE_VER_OPENMLS_RUST_CRYPTO"));
    println!("chacha20poly1305    : {}", env!("SLICE_VER_CHACHA"));
    println!("blake3              : {}", env!("SLICE_VER_BLAKE3"));
    println!("serde_json          : {}", env!("SLICE_VER_SERDE_JSON"));
    println!("iroh (resolvable, NOT linked — transport stubbed): 0.98.2 / iroh-blobs 0.102.0");
    println!("\nBuilt on real automerge {} / openmls {} on rustc {} (>= 1.80).",
        env!("SLICE_VER_AUTOMERGE"), env!("SLICE_VER_OPENMLS"), env!("SLICE_RUSTC_VERSION"));

    if !all_pass {
        std::process::exit(1);
    }
}
