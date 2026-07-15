//! Phase 12: removal != redaction.
//!
//! Phase 7 noted that removing a member rotates the MLS epoch, but the removed
//! member keeps the keys of epochs it belonged to (forward secrecy protects
//! FUTURE content, not PAST content). So "remove member" does NOT revoke access
//! to content already encrypted under past epochs. This proves the gap and the
//! mitigation (re-encryption), and is honest about the mitigation's limits.

mod crypto;
#[allow(dead_code)]
mod mls;

fn section(t: &str) {
    println!("\n=== {t} ===");
}
fn pass(r: &mut Vec<(&'static str, bool)>, name: &'static str, ok: bool) {
    println!("  -> {}: {}", name, if ok { "PASS" } else { "FAIL" });
    r.push((name, ok));
}

fn main() {
    let mut results: Vec<(&'static str, bool)> = Vec::new();

    // ------------------------------------------------------------------
    section("STEP 1: Group {Alice, Bob, Mallory}; content under the epoch-N key");
    // ------------------------------------------------------------------
    let alice = mls::Member::new("Alice");
    let bob = mls::Member::new("Bob");
    let mallory = mls::Member::new("Mallory");

    let mut ag = mls::create_group(&alice);
    let w_bob = mls::add_member(&mut ag, &alice, &bob);
    let mut bg = mls::join_from_welcome(&bob, &w_bob);
    // Alice adds Mallory; Bob must apply the commit to stay in sync.
    let (commit_m, w_mal) = mls::stage_add(&mut ag, &alice, &mallory);
    mls::merge_own(&mut ag, &alice);
    mls::apply_commit(&mut bg, &bob, &commit_m);
    let mg = mls::join_from_welcome(&mallory, &w_mal);

    let epoch_n = ag.epoch().as_u64();
    let k_n_alice = alice.content_key(&ag);
    let k_n_mallory = mallory.content_key(&mg);
    println!("epoch N = {epoch_n}; members = Alice, Bob, Mallory");
    println!("all derive the same epoch-N key? {}", k_n_alice == k_n_mallory);

    // Durable content authored at epoch N.
    const PLAN: &[u8] = b"the secret plan: launch on the 14th";
    let ct_n = crypto::encrypt(&k_n_alice, PLAN);
    let mallory_reads_old = crypto::decrypt(&k_n_mallory, &ct_n).map(|p| p == PLAN).unwrap_or(false);
    println!("Mallory (a member) can read epoch-N content? {mallory_reads_old}");
    pass(&mut results, "1. Members share the epoch-N key; Mallory can read epoch-N content", k_n_alice == k_n_mallory && mallory_reads_old);

    // ------------------------------------------------------------------
    section("STEP 2: Remove Mallory -> epoch N+1; key rotates");
    // ------------------------------------------------------------------
    let commit_rm = mls::remove_member(&mut ag, &alice, b"Mallory");
    mls::apply_commit(&mut bg, &bob, &commit_rm);
    let epoch_n1 = ag.epoch().as_u64();
    let k_n1_alice = alice.content_key(&ag);
    let k_n1_bob = bob.content_key(&bg);
    // Mallory's group is stale at epoch N; she keeps k_n_mallory and CANNOT get k_{n+1}.
    let key_rotated = k_n1_alice != k_n_alice && k_n1_alice == k_n1_bob;
    let members_after: Vec<Vec<u8>> = mls::member_identities(&ag);
    let mallory_gone = !members_after.contains(&b"Mallory".to_vec()) && members_after.len() == 2;
    println!("epoch N+1 = {epoch_n1}; members = {:?}", members_after.iter().map(|m| String::from_utf8_lossy(m)).collect::<Vec<_>>());
    println!("key rotated (Alice==Bob, != epoch-N)? {key_rotated}; Mallory removed? {mallory_gone}");
    pass(&mut results, "2. Removal rotates the epoch key; Mallory is out of the group", key_rotated && mallory_gone);

    // ------------------------------------------------------------------
    section("STEP 3: Forward secrecy — Mallory cannot read NEW (epoch N+1) content");
    // ------------------------------------------------------------------
    const PLAN2: &[u8] = b"post-removal plan: rotate all credentials";
    let ct_n1 = crypto::encrypt(&k_n1_alice, PLAN2);
    // Mallory only has k_n_mallory (epoch N). Try it on the epoch-N+1 ciphertext.
    let mallory_reads_new = crypto::decrypt(&k_n_mallory, &ct_n1).is_ok();
    let bob_reads_new = crypto::decrypt(&k_n1_bob, &ct_n1).map(|p| p == PLAN2).unwrap_or(false);
    println!("Mallory can read epoch-N+1 content with her old key? {mallory_reads_new}");
    println!("Bob (current member) can read it? {bob_reads_new}");
    pass(&mut results, "3. Forward secrecy: removed member cannot read post-removal content", !mallory_reads_new && bob_reads_new);

    // ------------------------------------------------------------------
    section("STEP 4: THE GAP — removal != redaction (Mallory still reads OLD content)");
    // ------------------------------------------------------------------
    // The epoch-N ciphertext is unchanged; Mallory still holds k_n.
    let mallory_still_reads_old = crypto::decrypt(&k_n_mallory, &ct_n).map(|p| p == PLAN).unwrap_or(false);
    println!("Mallory STILL reads the old epoch-N ciphertext after removal? {mallory_still_reads_old}");
    println!("  => removing a member does NOT revoke access to content from epochs it belonged to.");
    pass(&mut results, "4. Gap confirmed: removal alone leaves old content readable by ex-member", mallory_still_reads_old);

    // ------------------------------------------------------------------
    section("STEP 5: Redaction — re-encrypt old content under the new key");
    // ------------------------------------------------------------------
    // Alice retained k_n (she was an epoch-N member), so she recovers the old
    // content and re-encrypts it under k_{n+1}, replacing the stored ciphertext.
    let plain_old = crypto::decrypt(&k_n_alice, &ct_n).expect("Alice recovers old content");
    let ct_redacted = crypto::encrypt(&k_n1_alice, &plain_old); // re-keyed copy replaces ct_n in the store
    let mallory_reads_redacted = crypto::decrypt(&k_n_mallory, &ct_redacted).is_ok();
    let bob_reads_redacted = crypto::decrypt(&k_n1_bob, &ct_redacted).map(|p| p == PLAN).unwrap_or(false);
    println!("after re-encrypting old content under epoch-N+1 key:");
    println!("  Mallory can read the re-keyed copy? {mallory_reads_redacted}");
    println!("  Bob can read the re-keyed copy?     {bob_reads_redacted}");
    pass(&mut results, "5. Redaction: re-encryption revokes ex-member access to the stored copy", !mallory_reads_redacted && bob_reads_redacted);

    // ------------------------------------------------------------------
    section("SUMMARY");
    // ------------------------------------------------------------------
    let mut all = true;
    for (name, ok) in &results {
        println!("[{}] {}", if *ok { "PASS" } else { "FAIL" }, name);
        all &= *ok;
    }
    println!("\nOVERALL: {}", if all { "PASS — removal rotates keys (forward secrecy); redaction needs re-encryption" } else { "FAIL" });

    section("ISSUES SURFACED / CONCLUSIONS");
    println!("  CONCLUSION: 'remove member' and 'revoke access to history' are DIFFERENT operations.");
    println!("    - Removal (MLS epoch rotation) gives forward secrecy: the ex-member cannot read");
    println!("      content encrypted AFTER removal.");
    println!("    - It does NOT redact the past: content already encrypted under epochs the member");
    println!("      belonged to stays readable with the keys they retain.");
    println!("  MITIGATION: to revoke access to specific past content, current members must");
    println!("    RE-ENCRYPT it under the new epoch key and replace the stored ciphertext.");
    println!("  HARD LIMIT (honest): re-encryption only controls the STORED copy. If the ex-member");
    println!("    already copied the plaintext or kept the old ciphertext + key, nothing can retract");
    println!("    that. True deletion is impossible against an adversary who already had access.");
    println!("  IMPLICATION: a 'delete from history' product feature must (a) re-encrypt/rotate and");
    println!("    (b) be honest that it bounds FUTURE access to the stored copy, not past exposure.");

    section("VERSION REPORT");
    println!("rustc {} | openmls {} | chacha20poly1305 {}", env!("SLICE_RUSTC_VERSION"), env!("SLICE_VER_OPENMLS"), env!("SLICE_VER_CHACHA"));

    if !all {
        std::process::exit(1);
    }
}
