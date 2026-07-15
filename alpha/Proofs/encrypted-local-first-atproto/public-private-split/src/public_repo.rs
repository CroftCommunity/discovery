//! The public atproto space: an **unencrypted, world-readable** repo, laid out
//! like the private group doc (`did → collection → rkey → record`) but in plain
//! cleartext — this is what would live on a public PDS / be served by a public
//! AppView. Mirroring writes selected records here; nothing else ever does.
//!
//! `BTreeMap` is used throughout for deterministic ordering (stable JSON for the
//! non-leakage assertion).

use std::collections::BTreeMap;

use serde_json::Value;

#[derive(Default)]
pub struct PublicRepo {
    repos: BTreeMap<String, BTreeMap<String, BTreeMap<String, Value>>>,
}

impl PublicRepo {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn put(&mut self, did: &str, collection: &str, rkey: &str, record: Value) {
        self.repos
            .entry(did.to_string())
            .or_default()
            .entry(collection.to_string())
            .or_default()
            .insert(rkey.to_string(), record);
    }

    /// All public records as `(did, collection, rkey, &record)`.
    pub fn records(&self) -> Vec<(String, String, String, &Value)> {
        let mut out = Vec::new();
        for (did, colls) in &self.repos {
            for (coll, rkeys) in colls {
                for (rkey, rec) in rkeys {
                    out.push((did.clone(), coll.clone(), rkey.clone(), rec));
                }
            }
        }
        out
    }

    pub fn record_count(&self) -> usize {
        self.repos
            .values()
            .flat_map(|c| c.values())
            .map(|r| r.len())
            .sum()
    }

    /// The entire public repo as one JSON value (everything that is world-readable).
    pub fn to_json(&self) -> Value {
        serde_json::to_value(&self.repos).unwrap()
    }

    /// Does any byte of the public projection contain `needle`? The core
    /// non-leakage check: private content must never appear here.
    pub fn leaks(&self, needle: &str) -> bool {
        self.to_json().to_string().contains(needle)
    }
}
