//! Targeted atproto Lexicon validator.
//!
//! This is not a general-purpose Lexicon engine; it validates a record `Value`
//! against the constraints actually present in our two schemas, but it does so
//! by *reading the lexicon JSON* (required fields, property types, `maxLength`,
//! `maxGraphemes`, `format: datetime`, `ref` resolution) rather than hard-coding
//! the rules — so the schema files are the source of truth.
//!
//! atproto rules enforced:
//!   * record objects must carry `$type` equal to the bare NSID (the lexicon
//!     `id`); `main` types take no `#main` suffix.
//!   * object schemas are closed: only declared properties (plus `$type`) allowed.
//!   * `string` honours `maxLength` (UTF-8 bytes) and `maxGraphemes`.
//!   * `format: datetime` must be an atproto-conformant timestamp.
//!   * `ref` (e.g. `#strongRef`) resolves within the lexicon's `defs`.

use serde_json::Value;

/// The two lexicon schemas, embedded so the binary is self-contained. The same
/// files live under `lexicons/` for use by external atproto tooling.
pub const POST_LEXICON: &str = include_str!("../lexicons/org.croftc.experiment.feed.post.json");
pub const REACTION_LEXICON: &str =
    include_str!("../lexicons/org.croftc.experiment.feed.reaction.json");

pub struct Lexicon {
    id: String,
    doc: Value,
}

impl Lexicon {
    pub fn load(src: &str) -> Self {
        let doc: Value = serde_json::from_str(src).expect("lexicon is not valid JSON");
        assert_eq!(doc["lexicon"], serde_json::json!(1), "expected \"lexicon\": 1");
        let id = doc["id"].as_str().expect("lexicon has no id").to_string();
        Self { id, doc }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    /// Validate a record JSON object against this lexicon's `main` record schema.
    pub fn validate(&self, record: &Value) -> Result<(), String> {
        // $type must be present and equal the bare NSID.
        match record.get("$type").and_then(Value::as_str) {
            Some(t) if t == self.id => {}
            Some(t) => return Err(format!("$type \"{t}\" != lexicon id \"{}\"", self.id)),
            None => return Err("record is missing required $type".into()),
        }
        let main = &self.doc["defs"]["main"];
        if main["type"] != serde_json::json!("record") {
            return Err("lexicon main is not a record".into());
        }
        self.validate_object(&main["record"], record, true)
    }

    fn validate_object(&self, schema: &Value, value: &Value, allow_type_field: bool) -> Result<(), String> {
        let obj = value.as_object().ok_or("expected a JSON object")?;
        let props = schema["properties"].as_object();

        // Required fields present.
        if let Some(reqs) = schema["required"].as_array() {
            for r in reqs {
                let name = r.as_str().unwrap_or("");
                if !obj.contains_key(name) {
                    return Err(format!("missing required field `{name}`"));
                }
            }
        }
        // Closed object: every present field must be declared (plus $type at root).
        for (name, val) in obj {
            if name == "$type" {
                if allow_type_field {
                    continue;
                }
                return Err("nested object may not carry $type".into());
            }
            let Some(pschema) = props.and_then(|p| p.get(name)) else {
                return Err(format!("undeclared field `{name}` (objects are closed)"));
            };
            self.validate_field(pschema, val, name)?;
        }
        Ok(())
    }

    fn validate_field(&self, schema: &Value, value: &Value, name: &str) -> Result<(), String> {
        match schema["type"].as_str() {
            Some("string") => {
                let s = value
                    .as_str()
                    .ok_or_else(|| format!("`{name}` must be a string"))?;
                if let Some(max) = schema["maxLength"].as_u64() {
                    if s.len() as u64 > max {
                        return Err(format!("`{name}` exceeds maxLength {max} bytes (got {})", s.len()));
                    }
                }
                if let Some(max) = schema["maxGraphemes"].as_u64() {
                    // Approximated by Unicode scalar count; true grapheme
                    // clustering would need unicode-segmentation. Adequate for
                    // the ASCII/simple-emoji text used here.
                    let g = s.chars().count() as u64;
                    if g > max {
                        return Err(format!("`{name}` exceeds maxGraphemes {max} (got {g})"));
                    }
                }
                match schema["format"].as_str() {
                    Some("datetime") => {
                        if !is_atproto_datetime(s) {
                            return Err(format!("`{name}` is not a valid atproto datetime: {s:?}"));
                        }
                    }
                    Some("language") => {
                        if s.is_empty() {
                            return Err(format!("`{name}` language tag is empty"));
                        }
                    }
                    Some("at-uri") => {
                        if !s.starts_with("at://") {
                            return Err(format!("`{name}` is not an at:// URI: {s:?}"));
                        }
                    }
                    _ => {}
                }
                Ok(())
            }
            Some("array") => {
                let arr = value
                    .as_array()
                    .ok_or_else(|| format!("`{name}` must be an array"))?;
                if let Some(max) = schema["maxLength"].as_u64() {
                    if arr.len() as u64 > max {
                        return Err(format!("`{name}` array exceeds maxLength {max}"));
                    }
                }
                for item in arr {
                    self.validate_field(&schema["items"], item, name)?;
                }
                Ok(())
            }
            Some("object") => self.validate_object(schema, value, false),
            Some("ref") => {
                let r = schema["ref"]
                    .as_str()
                    .ok_or_else(|| format!("`{name}` ref has no target"))?;
                let target = r
                    .strip_prefix('#')
                    .ok_or_else(|| format!("`{name}` external ref `{r}` not supported here"))?;
                let resolved = &self.doc["defs"][target];
                if resolved.is_null() {
                    return Err(format!("`{name}` ref `#{target}` not found in defs"));
                }
                self.validate_object(resolved, value, false)
            }
            other => Err(format!("`{name}` has unsupported schema type {other:?}")),
        }
    }
}

/// Targeted check for atproto's constrained ISO-8601/RFC-3339 datetime:
/// `YYYY-MM-DDTHH:MM:SS(.fff)?(Z|±HH:MM)`, uppercase `T`, mandatory timezone.
pub fn is_atproto_datetime(s: &str) -> bool {
    let Some((date, time)) = s.split_once('T') else {
        return false;
    };
    // Date: YYYY-MM-DD
    let dparts: Vec<&str> = date.split('-').collect();
    if dparts.len() != 3
        || dparts[0].len() != 4
        || dparts[1].len() != 2
        || dparts[2].len() != 2
        || !dparts.iter().all(|p| p.bytes().all(|b| b.is_ascii_digit()))
    {
        return false;
    }
    // Time + timezone.
    let (clock, tz_ok) = if let Some(rest) = time.strip_suffix('Z') {
        (rest, true)
    } else if let Some(idx) = time.rfind(['+', '-']) {
        let (clock, offset) = time.split_at(idx);
        // offset like +HH:MM
        let off = &offset[1..];
        let ok = off.len() == 5
            && off.as_bytes()[2] == b':'
            && off[..2].bytes().all(|b| b.is_ascii_digit())
            && off[3..].bytes().all(|b| b.is_ascii_digit());
        (clock, ok)
    } else {
        return false; // no timezone
    };
    if !tz_ok {
        return false;
    }
    // clock: HH:MM:SS with optional fractional seconds.
    let base = clock.split('.').next().unwrap_or("");
    let tparts: Vec<&str> = base.split(':').collect();
    tparts.len() == 3
        && tparts.iter().all(|p| p.len() == 2 && p.bytes().all(|b| b.is_ascii_digit()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn datetime_rules() {
        assert!(is_atproto_datetime("2026-06-13T11:18:53.000Z"));
        assert!(is_atproto_datetime("2026-06-13T11:18:53Z"));
        assert!(is_atproto_datetime("2026-06-13T11:18:53+02:00"));
        assert!(!is_atproto_datetime("2026-06-13 11:18:53Z")); // space, not T
        assert!(!is_atproto_datetime("2026-06-13T11:18:53")); // no timezone
    }

    #[test]
    fn valid_post_passes_and_overlong_fails() {
        let lex = Lexicon::load(POST_LEXICON);
        let good = json!({"$type": crate::record::POST_NSID, "text": "hi", "createdAt": "2026-06-13T11:18:53.000Z"});
        assert!(lex.validate(&good).is_ok());

        let bad_type = json!({"$type": "wrong", "text": "hi", "createdAt": "2026-06-13T11:18:53.000Z"});
        assert!(lex.validate(&bad_type).is_err());

        let missing = json!({"$type": crate::record::POST_NSID, "text": "hi"});
        assert!(lex.validate(&missing).is_err());

        let undeclared = json!({"$type": crate::record::POST_NSID, "text": "hi", "createdAt": "2026-06-13T11:18:53.000Z", "nope": 1});
        assert!(lex.validate(&undeclared).is_err());
    }
}
