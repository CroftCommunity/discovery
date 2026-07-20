//! Shared test scaffolding: fixture loading + a mock resolver.
#![allow(dead_code)]

use std::collections::HashMap;
use std::path::PathBuf;

use lexicon_community::attest::Resolver;
use lexicon_community::schema::Registry;

pub fn fixtures() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures")
}

pub fn load(rel: &str) -> serde_json::Value {
    let p = fixtures().join(rel);
    let s = std::fs::read_to_string(&p).unwrap_or_else(|e| panic!("read {}: {e}", p.display()));
    serde_json::from_str(&s).unwrap_or_else(|e| panic!("parse {}: {e}", p.display()))
}

/// Load every vendored + candidate lexicon into a registry.
pub fn registry() -> Registry {
    let mut reg = Registry::new();
    let dir = fixtures().join("lexicons");
    for entry in std::fs::read_dir(&dir).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().and_then(|e| e.to_str()) == Some("json") {
            let doc: serde_json::Value =
                serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
            reg.load(&doc).unwrap();
        }
    }
    reg
}

/// A resolver over an in-memory record + key table. No network.
#[derive(Default)]
pub struct MockResolver {
    pub records: HashMap<String, serde_json::Value>,
    pub keys: HashMap<String, Vec<String>>,
}

impl MockResolver {
    pub fn with_record(mut self, uri: &str, v: serde_json::Value) -> Self {
        self.records.insert(uri.to_string(), v);
        self
    }
    pub fn with_key(mut self, did: &str, did_key: &str) -> Self {
        self.keys.entry(did.to_string()).or_default().push(did_key.to_string());
        self
    }
}

impl Resolver for MockResolver {
    fn get_record(&self, at_uri: &str) -> Result<serde_json::Value, String> {
        self.records
            .get(at_uri)
            .cloned()
            .ok_or_else(|| format!("no record at {at_uri}"))
    }
    fn authorized_keys(&self, did: &str) -> Vec<String> {
        self.keys.get(did).cloned().unwrap_or_default()
    }
}

/// Derive the reference fixture's P-256 public did:key from its published
/// private did:key (the CLI omits the public `key` — a finding in itself).
pub fn reference_public_did_key() -> String {
    use lexicon_community::didkey::PubKey;
    let raw = bs58::decode("42tv1pb3Dzog28Q1udyieg1YJP3x1Un5vraE1bttXeCDSpW")
        .into_vec()
        .unwrap();
    // strip the 2-byte multicodec private prefix (0x1306 = p256-priv).
    let sk = p256::ecdsa::SigningKey::from_slice(&raw[2..]).unwrap();
    PubKey::P256(*sk.verifying_key()).to_did_key()
}
