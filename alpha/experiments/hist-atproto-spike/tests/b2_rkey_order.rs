//! B2 — rkey order property (RUN-HIST-01).
//!
//! Serves history-durability.md §J (range reconciliation over (subspace,
//! counter)) via HIST-ATPROTO-MATCHUP.md row 2: the rkey encoding
//! (hashed-subspace prefix, zero-padded counter) sorts lexicographically
//! identically to the (subspace, counter) total order, property-tested
//! across counter magnitudes INCLUDING the padding boundary. RED-able: an
//! unpadded encoding fails at the 9→10 boundary ("10" < "9" lexically) —
//! the staged red IS the unpadded form, so the red run proves the failure
//! mode exists before the padded form goes green.
//!
//! The padding width itself is a byte choice, `[gates-release]`-adjacent;
//! the 20-digit width here is spike-local (src/rkey.rs).

use hist_atproto_spike::envelope::fixture_subspace;
use hist_atproto_spike::rkey::entry_rkey;

/// Counter magnitudes crossing every decimal-width boundary a u64 can hit,
/// plus the ceiling.
fn boundary_counters() -> Vec<u64> {
    let mut v = vec![0, 1, 2];
    let mut p: u64 = 1;
    loop {
        // 9→10, 99→100, … each padding boundary.
        v.push(p * 10 - 1);
        v.push(p * 10);
        v.push(p * 10 + 1);
        match p.checked_mul(10) {
            Some(next) if next <= 1_000_000_000_000_000_000 => p = next,
            _ => break,
        }
    }
    v.push(u64::MAX - 1);
    v.push(u64::MAX);
    v.sort_unstable();
    v.dedup();
    v
}

#[test]
fn rkey_lexicographic_order_equals_subspace_counter_order() {
    // Distinct fixture subspaces (hash-derived, so prefixes differ).
    let subspaces = ["b2-a", "b2-b", "b2-c", "b2-d"].map(fixture_subspace);
    let counters = boundary_counters();

    let mut keyed: Vec<(([u8; 32], u64), String)> = Vec::new();
    for s in &subspaces {
        for &c in &counters {
            keyed.push(((*s, c), entry_rkey(s, c)));
        }
    }

    // Property: for every pair, tuple order ≡ rkey lexicographic order.
    // (String comparison is byte comparison for this ASCII alphabet, which
    // is the MST's left-to-right lexical sort.)
    for (i, (ta, ka)) in keyed.iter().enumerate() {
        for (tb, kb) in keyed.iter().skip(i + 1) {
            assert_eq!(
                ta.cmp(tb),
                ka.cmp(kb),
                "rkey order must equal (subspace, counter) order for \
                 counters {} vs {} — the padding boundary is where the \
                 unpadded form fails (9→10)",
                ta.1,
                tb.1,
            );
        }
    }
}

#[test]
fn rkey_stays_inside_the_record_key_contract() {
    // Record-key spec (matchup §5-2): alphabet A-Za-z0-9.-_:~, 1..=512
    // chars; ≤80-char practice bound.
    let s = fixture_subspace("b2-contract");
    for c in [0u64, 9, 10, u64::MAX] {
        let k = entry_rkey(&s, c);
        assert!(!k.is_empty() && k.len() <= 80, "within the practice bound: {k}");
        assert!(
            k.bytes()
                .all(|b| b.is_ascii_alphanumeric() || b"-._:~".contains(&b)),
            "inside the record-key alphabet: {k}"
        );
    }
    // Same subspace, same counter → same rkey (deterministic index).
    assert_eq!(entry_rkey(&s, 7), entry_rkey(&s, 7));
}
