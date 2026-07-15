//! Phase 4: the public/private split — selective mirroring from the encrypted
//! private group into a public atproto space.
//!
//! The architectural keystone the prior phases kept deferring: records authored
//! in the encrypted group are projected into a public, cleartext atproto repo
//! ONLY when explicitly marked public, never otherwise, and never in a way that
//! leaks the existence of a private record (no dangling references). What
//! crosses is valid atproto lexicon data under the author's public DID, and it
//! plugs into the same AppView ingest contract from Phases 3a/3b.
//!
//! Fully offline (no live PDS/network), consistent with every prior phase.

#[allow(dead_code)]
mod content_id;
mod crypto;
mod groupdoc;
mod lexicon;
mod mirror;
mod mls;
mod source;
mod visibility;
mod public_repo;
// Carried modules whose full surface isn't exercised here (e.g. doc::load,
// record::cid_for, content_id::is_atproto_cid).
#[allow(dead_code)]
mod doc;
#[allow(dead_code)]
mod record;

use lexicon::Lexicon;
use record::{Post, Reaction, StrongRef, POST_NSID, REACTION_NSID};
use serde_json::Value;
use source::RecordSource;
use visibility::{MirrorPolicy, Visibility};

/// Public DID derived from a member's MLS identity (same bytes as the 4-tuple
/// subspace) — the identity bridge between private group and public network.
fn did_of(m: &mls::Member) -> String {
    format!("did:plc:{}", hex::encode(&m.identity()[..14]))
}

fn section(t: &str) {
    println!("\n=== {t} ===");
}

fn pass(results: &mut Vec<(&'static str, bool)>, name: &'static str, ok: bool) {
    println!("  -> {}: {}", name, if ok { "PASS" } else { "FAIL" });
    results.push((name, ok));
}

fn main() {
    let mut results: Vec<(&'static str, bool)> = Vec::new();
    let post_lex = Lexicon::load(lexicon::POST_LEXICON);
    let reaction_lex = Lexicon::load(lexicon::REACTION_LEXICON);

    // ------------------------------------------------------------------
    section("STEP 1: Encrypted private group with mixed-visibility records");
    // ------------------------------------------------------------------
    let alice = mls::Member::new("Alice");
    let bob = mls::Member::new("Bob");
    let mut alice_group = mls::create_group(&alice);
    let welcome = mls::add_member(&mut alice_group, &alice, &bob);
    let bob_group = mls::join_from_welcome(&bob, &welcome);
    let key_alice = alice.content_key(&alice_group);
    let key_bob = bob.content_key(&bob_group);
    let (alice_did, bob_did) = (did_of(&alice), did_of(&bob));

    let mut group_doc = doc::new_doc();
    let mut policy = MirrorPolicy::new();

    // post_public (Alice): meant for the world.
    let pub_rkey = record::new_tid();
    let pub_post = Post::new("Announcing our public launch 🚀", Some(vec!["en".into()]));
    let pub_post_json = serde_json::to_string(&pub_post).unwrap();
    groupdoc::put_record(&mut group_doc, &alice_did, POST_NSID, &pub_rkey, &pub_post_json);
    let pub_post_uri = record::at_uri(&alice_did, POST_NSID, &pub_rkey);
    policy.set(&pub_post_uri, Visibility::Public);

    // post_private (Alice): sensitive, group-only.
    const SECRET: &str = "internal: Q3 budget is 50000 USD";
    let priv_rkey = record::new_tid();
    let priv_post = Post::new(SECRET, None);
    groupdoc::put_record(&mut group_doc, &alice_did, POST_NSID, &priv_rkey, &serde_json::to_string(&priv_post).unwrap());
    let priv_post_uri = record::at_uri(&alice_did, POST_NSID, &priv_rkey);
    policy.set(&priv_post_uri, Visibility::Private);

    // reaction_pub (Bob): public, on the PUBLIC post -> fine to mirror.
    let rpub_rkey = record::new_tid();
    let reaction_pub = Reaction::new(
        StrongRef { uri: pub_post_uri.clone(), cid: content_id::record_cid(&pub_post) },
        "🎉",
    );
    groupdoc::put_record(&mut group_doc, &bob_did, REACTION_NSID, &rpub_rkey, &serde_json::to_string(&reaction_pub).unwrap());
    policy.set(&record::at_uri(&bob_did, REACTION_NSID, &rpub_rkey), Visibility::Public);

    // reaction_to_priv (Bob): tagged public BUT references the PRIVATE post ->
    // must be redacted, or it leaks the private post's existence + AT-URI.
    let rpriv_rkey = record::new_tid();
    let reaction_priv = Reaction::new(
        StrongRef { uri: priv_post_uri.clone(), cid: content_id::record_cid(&priv_post) },
        "👀",
    );
    groupdoc::put_record(&mut group_doc, &bob_did, REACTION_NSID, &rpriv_rkey, &serde_json::to_string(&reaction_priv).unwrap());
    policy.set(&record::at_uri(&bob_did, REACTION_NSID, &rpriv_rkey), Visibility::Public);

    // Encrypt at rest (the private space is genuinely encrypted) + confirm the
    // ciphertext does not contain the secret in cleartext.
    let snapshot = doc::snapshot(&mut group_doc);
    let ciphertext = crypto::encrypt(&key_alice, &snapshot);
    let ct_has_secret = String::from_utf8_lossy(&ciphertext).contains(SECRET);
    let private_count = groupdoc::list_all(&group_doc).len();
    println!("private group holds {private_count} records (2 posts + 2 reactions), all encrypted at rest");
    println!("epoch keys match? {} ; ciphertext leaks secret in cleartext? {ct_has_secret}", key_alice == key_bob);
    // Bob can decrypt (sanity that this is the real stack, not a fake).
    let _ = crypto::decrypt(&key_bob, &ciphertext).expect("bob decrypts");
    pass(&mut results, "1. Private group: 4 mixed-visibility records, encrypted at rest",
        private_count == 4 && key_alice == key_bob && !ct_has_secret);

    // ------------------------------------------------------------------
    section("STEP 2: Mirror into the public space (the split)");
    // ------------------------------------------------------------------
    let (public, stats) = mirror::mirror(&group_doc, &policy, &post_lex, &reaction_lex);
    println!("{stats:?}");
    println!("public repo ({} records):\n{}", public.record_count(),
        serde_json::to_string_pretty(&public.to_json()).unwrap());
    pass(&mut results, "2. Mirror publishes only public records (default-deny)",
        stats.mirrored == 2 && stats.kept_private == 1 && stats.redacted_refs == 1);

    // ------------------------------------------------------------------
    section("STEP 3: Non-leakage assertion (the keystone)");
    // ------------------------------------------------------------------
    let leaks_secret = public.leaks(SECRET);
    let leaks_priv_uri = public.leaks(&priv_post_uri);
    let leaks_priv_rkey = public.leaks(&priv_rkey);
    println!("public projection contains the private secret text? {leaks_secret}");
    println!("public projection contains the private post AT-URI? {leaks_priv_uri}");
    println!("public projection contains the private rkey?       {leaks_priv_rkey}");
    pass(&mut results, "3. Non-leakage: no private content/URI/rkey in the public space",
        !leaks_secret && !leaks_priv_uri && !leaks_priv_rkey);

    // ------------------------------------------------------------------
    section("STEP 4: Public records are valid atproto + identity bridge");
    // ------------------------------------------------------------------
    let mut all_valid = true;
    let mut public_uris = Vec::new();
    for (did, coll, rkey, rec) in public.records() {
        let lex = if coll == POST_NSID { &post_lex } else { &reaction_lex };
        let ok = lex.validate(rec).is_ok();
        all_valid &= ok;
        let uri = record::at_uri(&did, &coll, &rkey);
        println!("  public {uri}  valid={ok}");
        public_uris.push((did, uri));
    }
    // Identity bridge: every public record's author DID is a known member DID.
    let bridge_ok = public_uris.iter().all(|(did, _)| *did == alice_did || *did == bob_did);
    println!("  author DIDs bridge to MLS members (Alice/Bob)? {bridge_ok}");
    pass(&mut results, "4. Public records lexicon-valid; author DID bridges to MLS identity",
        all_valid && bridge_ok);

    // ------------------------------------------------------------------
    section("STEP 5: Public projection plugs into the AppView ingest contract");
    // ------------------------------------------------------------------
    // The same RecordSource/RecordEvent contract from Phases 3a/3b: a public
    // AppView would ingest exactly these events — and only these.
    let mut src = source::PublicRepoSource::new(&public);
    let events = src.events();
    for ev in &events {
        println!("  ingest event: {} {}", ev.action, ev.uri());
    }
    let none_private = events.iter().all(|e| {
        let v: &Value = &e.record;
        !v.to_string().contains(SECRET)
    });
    pass(&mut results, "5. Public side emits AppView ingest events; none carry private data",
        events.len() == 2 && none_private);

    // ------------------------------------------------------------------
    section("SUMMARY");
    // ------------------------------------------------------------------
    let mut all = true;
    for (name, ok) in &results {
        println!("[{}] {}", if *ok { "PASS" } else { "FAIL" }, name);
        all &= *ok;
    }
    println!("\nOVERALL: {}", if all { "PASS — public/private split holds; private data never crosses the boundary" } else { "FAIL" });

    section("VERSION REPORT");
    println!("rustc               : {}", env!("SLICE_RUSTC_VERSION"));
    println!("automerge           : {}", env!("SLICE_VER_AUTOMERGE"));
    println!("openmls             : {}", env!("SLICE_VER_OPENMLS"));
    println!("chacha20poly1305    : {}", env!("SLICE_VER_CHACHA"));
    println!("cid                 : {}", env!("SLICE_VER_CID"));
    println!("serde_ipld_dagcbor  : {}", env!("SLICE_VER_DAGCBOR"));
    println!("serde_json          : {}", env!("SLICE_VER_SERDE_JSON"));
    println!("iroh (resolvable, NOT linked — transport stubbed): 0.98.2 / iroh-blobs 0.102.0");

    if !all {
        std::process::exit(1);
    }
}
