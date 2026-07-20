//! A minimal Lexicon record validator — enough to gate the fixture corpus.
//!
//! This is NOT a full Lexicon implementation; the *validator of record* is the
//! official `@atproto/lexicon` TypeScript tooling, driven by
//! `scripts/gate.mjs` (EXP-LEX-01). This Rust validator exists so the red-first
//! tests can run inside the crate with no Node dependency, and it deliberately
//! mirrors the two properties the corpus turns on:
//!
//!   * `enum` is CLOSED, `knownValues` is OPEN (the T-AT6.1 / matchup Row 3
//!     distinction) — a value outside a `knownValues` set is VALID;
//!   * required-field presence and primitive-type agreement are enforced;
//!   * unknown fields are rejected in strict mode (the closed-object posture).
//!
//! Refs and unions are checked shallowly (kind only) — the official gate carries
//! the deep check.

use std::collections::BTreeMap;

/// A loaded lexicon document (`{lexicon, id, defs}`).
#[derive(Debug, Clone)]
pub struct Lexicon {
    pub id: String,
    pub defs: serde_json::Map<String, serde_json::Value>,
}

/// A registry of lexicons keyed by NSID, able to resolve `#frag` refs.
#[derive(Debug, Default, Clone)]
pub struct Registry {
    lexicons: BTreeMap<String, Lexicon>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchemaError(pub String);

impl std::fmt::Display for SchemaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "schema: {}", self.0)
    }
}
impl std::error::Error for SchemaError {}

type R<T> = Result<T, SchemaError>;

fn e<T>(m: impl Into<String>) -> R<T> {
    Err(SchemaError(m.into()))
}

impl Registry {
    pub fn new() -> Self {
        Registry::default()
    }

    /// Load one lexicon JSON document.
    pub fn load(&mut self, doc: &serde_json::Value) -> R<()> {
        let id = doc
            .get("id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| SchemaError("lexicon missing id".into()))?
            .to_string();
        let defs = doc
            .get("defs")
            .and_then(|v| v.as_object())
            .ok_or_else(|| SchemaError("lexicon missing defs".into()))?
            .clone();
        self.lexicons.insert(id.clone(), Lexicon { id, defs });
        Ok(())
    }

    fn def(&self, nsid: &str, frag: &str) -> Option<&serde_json::Value> {
        self.lexicons.get(nsid)?.defs.get(frag)
    }

    /// Validate a record against `<nsid>#main` (a `record` def).
    pub fn validate_record(&self, nsid: &str, record: &serde_json::Value) -> R<()> {
        let main = self
            .def(nsid, "main")
            .ok_or_else(|| SchemaError(format!("no main def for {nsid}")))?;
        if main.get("type").and_then(|v| v.as_str()) != Some("record") {
            return e(format!("{nsid}#main is not a record"));
        }
        let rec_schema = main
            .get("record")
            .ok_or_else(|| SchemaError("record def missing `record`".into()))?;
        self.validate_object(nsid, rec_schema, record)
    }

    fn validate_object(&self, nsid: &str, schema: &serde_json::Value, val: &serde_json::Value) -> R<()> {
        let obj = val.as_object().ok_or_else(|| SchemaError("expected object".into()))?;
        let props = schema
            .get("properties")
            .and_then(|v| v.as_object())
            .cloned()
            .unwrap_or_default();
        // required
        if let Some(reqs) = schema.get("required").and_then(|v| v.as_array()) {
            for req in reqs {
                let name = req.as_str().unwrap_or("");
                if !obj.contains_key(name) {
                    return e(format!("missing required field `{name}`"));
                }
            }
        }
        // unknown-field rejection (closed-object posture). $type is always allowed.
        for k in obj.keys() {
            if k == "$type" {
                continue;
            }
            if !props.contains_key(k) {
                return e(format!("unknown field `{k}` not in schema"));
            }
        }
        // per-field
        for (name, fschema) in &props {
            if let Some(fval) = obj.get(name) {
                self.validate_field(nsid, name, fschema, fval)?;
            }
        }
        Ok(())
    }

    fn validate_field(
        &self,
        nsid: &str,
        name: &str,
        fschema: &serde_json::Value,
        fval: &serde_json::Value,
    ) -> R<()> {
        let ty = fschema.get("type").and_then(|v| v.as_str()).unwrap_or("");
        match ty {
            "string" => {
                let s = fval
                    .as_str()
                    .ok_or_else(|| SchemaError(format!("field `{name}`: expected string")))?;
                // enum is CLOSED; knownValues is OPEN (not enforced).
                if let Some(en) = fschema.get("enum").and_then(|v| v.as_array()) {
                    if !en.iter().any(|e| e.as_str() == Some(s)) {
                        return e(format!("field `{name}`: `{s}` not in closed enum"));
                    }
                }
                if let Some(maxg) = fschema.get("maxGraphemes").and_then(|v| v.as_u64()) {
                    if s.chars().count() as u64 > maxg {
                        return e(format!("field `{name}`: exceeds maxGraphemes"));
                    }
                }
                Ok(())
            }
            "integer" => fval
                .as_i64()
                .map(|_| ())
                .ok_or_else(|| SchemaError(format!("field `{name}`: expected integer"))),
            "boolean" => fval
                .as_bool()
                .map(|_| ())
                .ok_or_else(|| SchemaError(format!("field `{name}`: expected boolean"))),
            "array" => {
                let arr = fval
                    .as_array()
                    .ok_or_else(|| SchemaError(format!("field `{name}`: expected array")))?;
                if let Some(items) = fschema.get("items") {
                    for it in arr {
                        self.validate_field(nsid, name, items, it)?;
                    }
                }
                Ok(())
            }
            "object" => self.validate_object(nsid, fschema, fval),
            "blob" => {
                // atproto blob ref: {$type:"blob", ref:{$link}, mimeType, size}
                if fval.get("ref").is_some() || fval.get("$type").and_then(|v| v.as_str()) == Some("blob") {
                    Ok(())
                } else {
                    e(format!("field `{name}`: expected blob"))
                }
            }
            "ref" => {
                let target = fschema.get("ref").and_then(|v| v.as_str()).unwrap_or("");
                self.validate_ref(nsid, name, target, fval)
            }
            "union" => {
                // Shallow: value must be an object; if it carries $type it should
                // be one of the refs (open unions in atproto are allowed, so a
                // missing/foreign $type is not fatal here — the official gate
                // carries the strict check).
                if !fval.is_object() {
                    return e(format!("field `{name}`: union member must be an object"));
                }
                Ok(())
            }
            "unknown" | "" => Ok(()),
            other => {
                let _ = other;
                Ok(())
            }
        }
    }

    fn validate_ref(&self, nsid: &str, name: &str, target: &str, fval: &serde_json::Value) -> R<()> {
        // Resolve target: "#frag", "nsid", or "nsid#frag".
        let (tn, tf) = if let Some(frag) = target.strip_prefix('#') {
            (nsid.to_string(), frag.to_string())
        } else if let Some((n, f)) = target.split_once('#') {
            (n.to_string(), f.to_string())
        } else {
            (target.to_string(), "main".to_string())
        };
        // com.atproto.repo.strongRef is a well-known external ref (uri + cid).
        if tn == "com.atproto.repo.strongRef" {
            let o = fval
                .as_object()
                .ok_or_else(|| SchemaError(format!("field `{name}`: strongRef must be object")))?;
            for req in ["uri", "cid"] {
                if !o.get(req).map(|v| v.is_string()).unwrap_or(false) {
                    return e(format!("field `{name}`: strongRef missing `{req}`"));
                }
            }
            return Ok(());
        }
        match self.def(&tn, &tf) {
            Some(def) => self.validate_field(nsid, name, def, fval),
            None => Ok(()), // unresolved external ref — the official gate resolves it.
        }
    }
}
