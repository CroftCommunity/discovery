//! EXP-LEX-01 — fixture corpus + schema validation harness.
//!
//! Acceptance criteria (red-first): every vendored schema loads; every golden
//! fixture validates against its schema; every adversarial fixture FAILS for the
//! stated reason; the enum(closed) vs knownValues(open) distinction holds.

mod common;
use common::{load, registry};

fn ty(v: &serde_json::Value) -> String {
    v.get("$type").and_then(|x| x.as_str()).unwrap().to_string()
}

#[test]
fn all_vendored_schemas_load() {
    // 9 lexicons: calendar event/rsvp, 4 location, 3 candidate attest.
    let reg = registry();
    // A record type resolves.
    reg.validate_record("community.lexicon.calendar.event", &load("golden/event_valid.json"))
        .expect("event schema present + golden valid");
}

#[test]
fn golden_records_validate() {
    let reg = registry();
    for f in ["golden/event_valid.json", "golden/rsvp_valid.json"] {
        let rec = load(f);
        reg.validate_record(&ty(&rec), &rec).unwrap_or_else(|e| panic!("{f}: {e}"));
    }
}

#[test]
fn known_values_are_open() {
    // A status outside the rsvp knownValues set MUST still validate (open set).
    let reg = registry();
    let rec = load("golden/rsvp_novel_status_open.json");
    reg.validate_record("community.lexicon.calendar.rsvp", &rec)
        .expect("knownValues is OPEN — a novel status is valid");
}

#[test]
fn enum_is_closed() {
    // Our candidate inclusionStaple uses a CLOSED enum for `alg`; a value outside
    // it MUST fail. (The contrast to known_values_are_open is the whole point.)
    let reg = registry();
    let rec = load("adversarial/staple_bad_alg.json");
    let err = reg
        .validate_record("community.lexicon.attest.inclusionStaple", &rec)
        .expect_err("closed enum rejects RS256");
    assert!(err.0.contains("closed enum"), "reason was: {}", err.0);
}

#[test]
fn adversarial_missing_required_fails() {
    let reg = registry();
    let rec = load("adversarial/event_missing_name.json");
    let err = reg
        .validate_record("community.lexicon.calendar.event", &rec)
        .expect_err("missing required `name`");
    assert!(err.0.contains("required") && err.0.contains("name"), "reason: {}", err.0);
}

#[test]
fn adversarial_wrong_type_fails() {
    // Incident-1 class: a field carrying the wrong primitive type.
    let reg = registry();
    let rec = load("adversarial/event_name_wrong_type.json");
    let err = reg
        .validate_record("community.lexicon.calendar.event", &rec)
        .expect_err("`name` is an integer, not a string");
    assert!(err.0.contains("expected string"), "reason: {}", err.0);
}

#[test]
fn adversarial_unknown_field_fails() {
    // A smuggled scalar in an unknown field — rejected by the closed-object posture.
    let reg = registry();
    let rec = load("adversarial/rsvp_unknown_field.json");
    let err = reg
        .validate_record("community.lexicon.calendar.rsvp", &rec)
        .expect_err("unknown field rejected");
    assert!(err.0.contains("unknown field"), "reason: {}", err.0);
}

#[test]
fn adversarial_strongref_missing_cid_fails() {
    let reg = registry();
    let rec = load("adversarial/rsvp_subject_missing_cid.json");
    let err = reg
        .validate_record("community.lexicon.calendar.rsvp", &rec)
        .expect_err("subject strongRef missing cid");
    assert!(err.0.contains("cid"), "reason: {}", err.0);
}
