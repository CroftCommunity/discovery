//! Jetstream wire types — just enough to parse real firehose commit frames.
//! (The local phases also wrap these in a `RecordSource`; here we only parse.)

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JetstreamEvent {
    pub did: String,
    pub time_us: u64,
    pub kind: String,
    #[serde(default)]
    pub commit: Option<Commit>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Commit {
    #[serde(default)]
    pub rev: String,
    #[serde(default)]
    pub operation: String,
    pub collection: String,
    pub rkey: String,
    #[serde(default)]
    pub record: Option<Value>,
    #[serde(default)]
    pub cid: Option<String>,
}
