//! atproto-style repository model layered on the Automerge document.
//!
//! A PDS lays out a repo as records keyed by `collection` (an NSID) + `rkey`.
//! We mirror that exactly: the Automerge ROOT is a map keyed by collection NSID,
//! and each collection is a map of `rkey -> record JSON`. So the local document
//! is structurally a mini-repo, and a record's local position
//! (`collection`/`rkey`) already matches where it would live in a PDS repo.
//!
//! Records are stored as their canonical JSON serialization (a string scalar).
//! Modeling each record as native nested Automerge objects is possible but
//! unnecessary for the interop question — the record travels as an opaque,
//! lexicon-valid JSON blob, which is precisely what gets POSTed to `createRecord`.
//!
//! How this sits alongside the 4-tuple addressing: the two are complementary.
//! The Willow-shaped `(namespace, subspace, path, timestamp)` address names an
//! *encrypted sync payload* (a snapshot or delta) in the blob store; the
//! `collection`/`rkey` model organizes *records within* the decrypted document.
//! Addressing moves ciphertext between peers; the repo model is the plaintext
//! structure they converge on.

use automerge::transaction::Transactable;
use automerge::{AutoCommit, ObjType, ReadDoc, ROOT};

/// Insert or overwrite a record at `collection`/`rkey`, creating the collection
/// map on first use. The record is stored as its JSON string.
pub fn put_record(doc: &mut AutoCommit, collection: &str, rkey: &str, record_json: &str) {
    let coll = match doc.get(ROOT, collection).expect("read failed") {
        Some((_, id)) => id,
        None => doc
            .put_object(ROOT, collection, ObjType::Map)
            .expect("failed to create collection map"),
    };
    doc.put(&coll, rkey, record_json).expect("failed to put record");
    doc.commit();
}

/// Fetch a single record's JSON by `collection`/`rkey`.
pub fn get_record(doc: &AutoCommit, collection: &str, rkey: &str) -> Option<String> {
    let (_, coll) = doc.get(ROOT, collection).expect("read failed")?;
    let (value, _) = doc.get(&coll, rkey).expect("read failed")?;
    value.into_string().ok()
}

/// All records in a collection as `(rkey, json)`, sorted by rkey. Because rkeys
/// are TIDs, this is chronological (creation) order.
pub fn list_collection(doc: &AutoCommit, collection: &str) -> Vec<(String, String)> {
    let Some((_, coll)) = doc.get(ROOT, collection).expect("read failed") else {
        return Vec::new();
    };
    let mut out: Vec<(String, String)> = doc
        .keys(&coll)
        .filter_map(|rkey| {
            doc.get(&coll, &rkey)
                .expect("read failed")
                .and_then(|(v, _)| v.into_string().ok())
                .map(|json| (rkey, json))
        })
        .collect();
    out.sort_by(|a, b| a.0.cmp(&b.0));
    out
}

/// All collection NSIDs present in the repo, sorted.
pub fn collections(doc: &AutoCommit) -> Vec<String> {
    let mut c: Vec<String> = doc.keys(ROOT).collect();
    c.sort();
    c
}
