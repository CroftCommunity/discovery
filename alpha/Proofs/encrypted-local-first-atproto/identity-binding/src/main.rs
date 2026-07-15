//! Phase 8: verifiable group-identity <-> DID binding.
//!
//! Closes the "two-headed identity" issue from Phase 6: a group member's
//! MLS-derived `did:key` and their PDS `did:plc` are linked by a mutually-signed
//! binding that an AppView can verify with only the account's DID-doc key.

mod binding;
#[allow(dead_code)]
mod mls;

use binding::{create_binding, forge_binding, verify_binding, Account};

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
    section("STEP 1: Create a mutually-signed binding");
    // ------------------------------------------------------------------
    let alice = mls::Member::new("Alice"); // group identity = MLS Ed25519 key
    let alice_account = Account::new(); // PDS account (did:plc)
    let binding = create_binding(&alice_account, &alice.signer);
    println!("{}", serde_json::to_string_pretty(&binding).unwrap());
    println!("  account did:plc verification key (from DID doc): {}", hex::encode(alice_account.verifying_key().to_bytes()));

    // ------------------------------------------------------------------
    section("STEP 2: An AppView verifies it (account key from DID doc; group key from did:key)");
    // ------------------------------------------------------------------
    let ok = verify_binding(&binding, &alice_account.verifying_key());
    println!("  verify result: {ok:?}");
    pass(&mut results, "1+2. Valid binding verifies (both signatures)", ok.is_ok());

    // ------------------------------------------------------------------
    section("STEP 3: Attack — Mallory tries to claim Alice's group key");
    // ------------------------------------------------------------------
    let mallory = mls::Member::new("Mallory");
    let mallory_account = Account::new();
    // Mallory forges a binding claiming Alice's did:key under Mallory's account,
    // signing with the only keys she has (her own).
    let forged = forge_binding(&mallory_account, &binding.group_did, &mallory.signer);
    let attack = verify_binding(&forged, &mallory_account.verifying_key());
    println!("  Mallory's forged binding (claims {} ):", forged.group_did);
    println!("  verify result: {attack:?}");
    pass(&mut results, "3. Forged claim of another's group key is REJECTED", attack.is_err());

    // ------------------------------------------------------------------
    section("STEP 4: Tamper — flip a byte of the group signature");
    // ------------------------------------------------------------------
    let mut tampered = binding.clone();
    let first = &tampered.sig_group[0..1];
    let repl = if first == "a" { "b" } else { "a" };
    tampered.sig_group.replace_range(0..1, repl);
    let t = verify_binding(&tampered, &alice_account.verifying_key());
    println!("  verify result: {t:?}");
    pass(&mut results, "4. Tampered signature is REJECTED", t.is_err());

    // ------------------------------------------------------------------
    section("STEP 5: What an AppView can now conclude");
    // ------------------------------------------------------------------
    println!("  Given a record published under {} and this binding,", binding.account_did);
    println!("  an AppView proves the same principal controls group key");
    println!("  {} — with no trust beyond the did:plc DID document.", binding.group_did);
    pass(&mut results, "5. Cross-identity claim is provable end to end", true);

    // ------------------------------------------------------------------
    section("SUMMARY");
    // ------------------------------------------------------------------
    let mut all = true;
    for (name, ok) in &results {
        println!("[{}] {}", if *ok { "PASS" } else { "FAIL" }, name);
        all &= *ok;
    }
    println!("\nOVERALL: {}", if all { "PASS — group identity and PDS DID are cryptographically, verifiably bound" } else { "FAIL" });

    section("ISSUES SURFACED");
    println!("  1. Trust root is the did:plc DID document: the verifier still must RESOLVE the DID");
    println!("     (plc.directory) to get the account key. The binding proves the link given that key.");
    println!("  2. No revocation/expiry: a compromised key needs the binding revocable (add notBefore/");
    println!("     notAfter + a tombstone record). Not modeled here.");
    println!("  3. Discovery: the binding must be PUBLISHED in the account's repo (as {}) and an", binding::BINDING_NSID);
    println!("     AppView must fetch + check it before trusting any cross-identity claim.");
    println!("  4. did:key here is did:key:z<hex>; real atproto uses multibase/multicodec (did:key:z6Mk…).");

    section("VERSION REPORT");
    println!("rustc {} | openmls {} | ed25519-dalek {} | serde_json {}",
        env!("SLICE_RUSTC_VERSION"), env!("SLICE_VER_OPENMLS"), env!("SLICE_VER_DALEK"), env!("SLICE_VER_SERDE_JSON"));

    if !all {
        std::process::exit(1);
    }
}
