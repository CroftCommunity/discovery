//! Phase 1: enrollment + the deny-closed access-check contract.

mod common;
use common::*;

use croft_admit::access::{decide, AccessDecision};
use croft_admit::enroll::{verify_and_bind, EnrollError};
use croft_admit::pds::PdsError;
use croft_admit::registry::Registry;

// --- enrollment: DID-control proof binds; everything else denies -------------

#[test]
fn enroll_binds_when_pds_record_names_the_endpoint() {
    let reg = Registry::new();
    let d = did("did:plc:alice");
    let ep = endpoint_from_seed(1);
    let mut pds = MockPds::new();
    pds.publish(&d, ep);

    let r = verify_and_bind(&reg, &pds, &d, ep);

    assert!(r.is_ok());
    assert!(reg.is_enrolled(&ep));
    assert_eq!(reg.did_for(&ep).as_ref(), Some(&d));
}

#[test]
fn enroll_denies_and_writes_nothing_on_endpoint_mismatch() {
    let reg = Registry::new();
    let d = did("did:plc:alice");
    let published = endpoint_from_seed(1);
    let claimed = endpoint_from_seed(2); // a *different* endpoint
    let mut pds = MockPds::new();
    pds.publish(&d, published);

    let r = verify_and_bind(&reg, &pds, &d, claimed);

    assert_eq!(r, Err(EnrollError::EndpointMismatch));
    assert!(!reg.is_enrolled(&claimed));
    assert!(!reg.is_enrolled(&published));
    assert!(reg.is_empty(), "a mismatch must never bind anything");
}

#[test]
fn enroll_denies_closed_when_pds_times_out() {
    let reg = Registry::new();
    let d = did("did:plc:alice");
    let ep = endpoint_from_seed(1);
    let mut pds = MockPds::new();
    pds.fail(&d, PdsError::Timeout);

    let r = verify_and_bind(&reg, &pds, &d, ep);

    assert_eq!(r, Err(EnrollError::PdsUnavailable(PdsError::Timeout)));
    assert!(!reg.is_enrolled(&ep), "timeout must deny, never fail-open");
}

#[test]
fn enroll_denies_when_did_has_no_record() {
    let reg = Registry::new();
    let d = did("did:plc:nobody");
    let ep = endpoint_from_seed(1);
    let pds = MockPds::new(); // publishes nothing

    let r = verify_and_bind(&reg, &pds, &d, ep);

    assert_eq!(r, Err(EnrollError::PdsUnavailable(PdsError::NotFound)));
    assert!(reg.is_empty());
}

// --- access check: parse header, ask registry, deny on any doubt -------------

#[test]
fn access_allows_enrolled_endpoint() {
    let reg = Registry::new();
    let ep = endpoint_from_seed(7);
    reg.bind(ep, did("did:plc:alice"));

    let decision = decide(&reg, Some(&ep.to_hex()));

    assert_eq!(decision, AccessDecision::Allow);
    assert!(decision.is_allow());
}

#[test]
fn access_denies_wellformed_but_unenrolled_endpoint() {
    let reg = Registry::new();
    let ep = endpoint_from_seed(7); // never enrolled

    let decision = decide(&reg, Some(&ep.to_hex()));

    assert_eq!(decision, AccessDecision::DenyNotEnrolled);
    assert!(!decision.is_allow());
}

#[test]
fn access_denies_missing_header() {
    let reg = Registry::new();
    assert_eq!(decide(&reg, None), AccessDecision::DenyMalformedId);
}

#[test]
fn access_denies_malformed_header() {
    let reg = Registry::new();
    // not hex
    assert_eq!(decide(&reg, Some("zzzz")), AccessDecision::DenyMalformedId);
    // right alphabet, wrong length
    assert_eq!(decide(&reg, Some("dead")), AccessDecision::DenyMalformedId);
    // empty
    assert_eq!(decide(&reg, Some("")), AccessDecision::DenyMalformedId);
}

#[test]
fn end_to_end_only_enrolled_endpoint_is_admitted() {
    // The Phase 1 acceptance shape, app-side: enroll A, then A is admitted and
    // an unenrolled B is refused.
    let reg = Registry::new();
    let a = endpoint_from_seed(0xA);
    let b = endpoint_from_seed(0xB);
    let da = did("did:plc:a");
    let mut pds = MockPds::new();
    pds.publish(&da, a);

    verify_and_bind(&reg, &pds, &da, a).expect("A enrolls");

    assert_eq!(decide(&reg, Some(&a.to_hex())), AccessDecision::Allow);
    assert_eq!(
        decide(&reg, Some(&b.to_hex())),
        AccessDecision::DenyNotEnrolled
    );
}
