//! Phase 11: binding lifecycle — validity windows, revocation, key rotation.

mod binding;
#[allow(dead_code)]
mod mls;

use binding::{create_binding, create_revocation, verify, Account};

fn section(t: &str) {
    println!("\n=== {t} ===");
}
fn pass(r: &mut Vec<(&'static str, bool)>, name: &'static str, ok: bool) {
    println!("  -> {}: {}", name, if ok { "PASS" } else { "FAIL" });
    r.push((name, ok));
}

fn main() {
    let mut results: Vec<(&'static str, bool)> = Vec::new();
    let none: &[binding::Revocation] = &[];

    let alice = mls::Member::new("Alice");
    let account = Account::new();
    let nbf = "2026-01-01T00:00:00Z";
    let naf = "2027-01-01T00:00:00Z";
    let b = create_binding(&account, &alice.signer, nbf, naf);
    println!("binding window: [{nbf}, {naf})");

    // ------------------------------------------------------------------
    section("STEP 1: Valid inside the window");
    // ------------------------------------------------------------------
    let now = "2026-06-13T13:00:00Z";
    let r = verify(&b, &account.verifying_key(), now, none);
    println!("  now={now} -> {r:?}");
    pass(&mut results, "1. Binding valid within its window", r.is_ok());

    // ------------------------------------------------------------------
    section("STEP 2: Expired (now >= not_after) is rejected");
    // ------------------------------------------------------------------
    let r = verify(&b, &account.verifying_key(), "2027-02-01T00:00:00Z", none);
    println!("  -> {r:?}");
    pass(&mut results, "2. Expired binding rejected", r.is_err());

    // ------------------------------------------------------------------
    section("STEP 3: Not yet valid (now < not_before) is rejected");
    // ------------------------------------------------------------------
    let r = verify(&b, &account.verifying_key(), "2025-12-01T00:00:00Z", none);
    println!("  -> {r:?}");
    pass(&mut results, "3. Not-yet-valid binding rejected", r.is_err());

    // ------------------------------------------------------------------
    section("STEP 4: Tamper — extend not_after, signature fails");
    // ------------------------------------------------------------------
    let mut tampered = b.clone();
    tampered.not_after = "2099-01-01T00:00:00Z".to_string(); // try to live forever
    let r = verify(&tampered, &account.verifying_key(), now, none);
    println!("  attacker set not_after=2099 -> {r:?}");
    pass(&mut results, "4. Extending the window breaks the signature", r.is_err());

    // ------------------------------------------------------------------
    section("STEP 5: Revocation supersedes a still-in-window binding");
    // ------------------------------------------------------------------
    let rev = create_revocation(&account, &b.group_did, "2026-03-01T00:00:00Z", "key compromised");
    let revs = vec![rev];
    let before = verify(&b, &account.verifying_key(), "2026-02-01T00:00:00Z", &revs); // before revocation
    let after = verify(&b, &account.verifying_key(), "2026-06-13T13:00:00Z", &revs); // after revocation
    println!("  before revocation date -> {before:?}");
    println!("  after  revocation date -> {after:?}");
    pass(&mut results, "5. Revocation invalidates the binding from its effective date", before.is_ok() && after.is_err());

    // ------------------------------------------------------------------
    section("STEP 6: Key rotation — revoke old binding, issue a new one");
    // ------------------------------------------------------------------
    // Compromise: Alice rotates to a fresh MLS key. The old binding is revoked;
    // a new binding for the new key is issued and verifies.
    let alice_rotated = mls::Member::new("Alice"); // fresh MLS signing key
    let new_b = create_binding(&account, &alice_rotated.signer, "2026-03-01T00:00:00Z", "2027-03-01T00:00:00Z");
    let revoke_old = create_revocation(&account, &b.group_did, "2026-03-01T00:00:00Z", "rotated");
    let all_revs = vec![revoke_old];
    let now2 = "2026-06-13T13:00:00Z";
    let old_ok = verify(&b, &account.verifying_key(), now2, &all_revs);
    let new_ok = verify(&new_b, &account.verifying_key(), now2, &all_revs);
    println!("  old binding ({}) -> {old_ok:?}", &b.group_did[..18]);
    println!("  new binding ({}) -> {new_ok:?}", &new_b.group_did[..18]);
    pass(&mut results, "6. After rotation: old binding rejected, new binding valid", old_ok.is_err() && new_ok.is_ok());

    // ------------------------------------------------------------------
    section("SUMMARY");
    // ------------------------------------------------------------------
    let mut all = true;
    for (name, ok) in &results {
        println!("[{}] {}", if *ok { "PASS" } else { "FAIL" }, name);
        all &= *ok;
    }
    println!("\nOVERALL: {}", if all { "PASS — bindings expire, revoke, and rotate verifiably" } else { "FAIL" });

    section("ISSUES SURFACED / RESOLVED");
    println!("  RESOLVED: the Phase 8 gap (no expiry/revocation) is closed — windows are signed");
    println!("    (un-extendable), and account-signed revocations supersede bindings from a date.");
    println!("  1. Revocation discovery is the hard part: an AppView must FIND revocations. Needs a");
    println!("     canonical, monotonic place to publish them (the account repo) + a freshness/");
    println!("     caching policy; a missed revocation = trusting a stale binding.");
    println!("  2. Clock trust: window + revocation checks assume a trusted 'now'. Skew or a lying");
    println!("     clock weakens both; real systems pin to signed timestamps / short windows.");
    println!("  3. Revocation here is account-signed; if the ACCOUNT key is compromised, revocation");
    println!("     itself is at risk — real did:plc uses separate rotation keys for exactly this.");

    section("VERSION REPORT");
    println!("rustc {} | openmls {} | ed25519-dalek {} | serde_json {}",
        env!("SLICE_RUSTC_VERSION"), env!("SLICE_VER_OPENMLS"), env!("SLICE_VER_DALEK"), env!("SLICE_VER_SERDE_JSON"));

    if !all {
        std::process::exit(1);
    }
}
