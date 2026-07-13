//! Custom-lexicon *comprehension*: turning opaque stored JSON into a typed,
//! validated view.
//!
//! The publish phase proved a custom NSID (`org.owasp.validation.note`) *propagates*
//! on the network. Propagation is not comprehension: the indexer stored it as raw
//! JSON without understanding it. Comprehension means a consumer that knows the
//! lexicon schema can deserialize records into a typed shape and *reject* ones that
//! don't conform — exactly what an AppView does to build a trustworthy typed view.

use serde::Deserialize;
use serde_json::Value;

pub const NSID: &str = "org.owasp.validation.note";

/// The typed view of an `org.owasp.validation.note` record.
///
/// This is the schema a comprehending consumer enforces. `serde` handles the
/// required-field / type checks; anything that fails to deserialize is rejected.
#[derive(Debug, Deserialize)]
pub struct ValidationNote {
    /// Required free-text note.
    pub note: String,
    /// Required integer payload.
    pub n: i64,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

/// Attempt to comprehend a stored record as a `ValidationNote`.
///
/// Returns the typed value on success, or a human-readable rejection reason. We
/// also sanity-check `$type` matches the NSID (a real AppView would).
pub fn comprehend(record_json: &str) -> Result<ValidationNote, String> {
    let value: Value =
        serde_json::from_str(record_json).map_err(|e| format!("not JSON: {e}"))?;

    match value.get("$type").and_then(Value::as_str) {
        Some(t) if t == NSID => {}
        Some(other) => return Err(format!("$type mismatch: expected {NSID}, got {other}")),
        None => return Err("missing $type".to_string()),
    }

    serde_json::from_value::<ValidationNote>(value)
        .map_err(|e| format!("schema violation: {e}"))
}
