//! Phase 3 (core): tier claim -> per-connection rate bucket.

use croft_admit::tier::{bucket_for, RateBucket, Tier};

#[test]
fn coordination_tier_is_capped() {
    let b = bucket_for(Tier::Coordination);
    assert!(
        b.bytes_per_second.is_some(),
        "coordination must carry a finite cap"
    );
    assert!(b.max_burst_bytes.is_some());
}

#[test]
fn broker_tier_is_unlimited() {
    assert_eq!(bucket_for(Tier::Broker), RateBucket::UNLIMITED);
    assert!(bucket_for(Tier::Broker).bytes_per_second.is_none());
}

#[test]
fn coordination_cap_sits_far_below_media_bitrate() {
    // The whole point: coordination headroom for disco frames, nowhere near a
    // usable audio/video bitrate. A conservative floor for "media" is ~24 kB/s
    // (low-bitrate audio); coordination must be well under that.
    let b = bucket_for(Tier::Coordination);
    let bps = b.bytes_per_second.unwrap();
    assert!(
        bps < 24_000,
        "coordination cap {bps} B/s should starve sustained media"
    );
}

#[test]
fn coordination_burst_covers_a_disco_exchange() {
    // Burst must clear a handful of small holepunch frames in one go, else even
    // coordination fails. (Sanity bound on the placeholder; real number is the
    // SPEC-DELTA calibration item.)
    let b = bucket_for(Tier::Coordination);
    assert!(b.max_burst_bytes.unwrap() >= 4 * 1024);
}
