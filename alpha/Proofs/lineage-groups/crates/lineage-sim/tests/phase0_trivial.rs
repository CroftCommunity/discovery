//! Phase 0 gate — a trivial scenario runs green under the harness and is
//! reproducible across runs (at the logical layer).

use lineage_sim::Scenario;

/// Build the trivial scenario: one device, genesis, one message. MLS forbids
/// a sender decrypting its own message, so we assert the message is produced
/// (a non-empty ciphertext) rather than self-delivering it.
fn run_trivial(seed: u64) -> (Scenario, Vec<u8>) {
    let mut s = Scenario::new(seed);
    s.add_device("alice", "alice").unwrap();
    s.device_mut("alice").create_group().unwrap();
    let ct = s.device_mut("alice").send(b"genesis: hello").unwrap();
    // Stamp the send on the logical clock so the digest reflects the op.
    s.clock.tick();
    (s, ct)
}

#[test]
fn phase0_trivial_scenario_green() {
    let (s, ciphertext) = run_trivial(0xC0FFEE);
    assert!(!ciphertext.is_empty(), "genesis message produced no ciphertext");
    assert_eq!(s.device("alice").member_count().unwrap(), 1);
    assert_eq!(s.device("alice").epoch().unwrap(), 0);
}

#[test]
fn phase0_trivial_scenario_is_logically_reproducible() {
    // Same seed -> identical logical digest across independent runs. (MLS
    // ciphertext is not bit-identical; see lineage-core::rng honesty boundary.)
    let (a, _) = run_trivial(0xC0FFEE);
    let (b, _) = run_trivial(0xC0FFEE);
    assert_eq!(
        a.logical_digest(),
        b.logical_digest(),
        "scenario is not reproducible for a fixed seed"
    );
}
