//! Group-document layout for AppView ingest — lives BEHIND the `RecordSource`
//! boundary (this module touches Automerge; the indexer and server never do).
//!
//! Design note / deviation from the prior phase: the lexicon-app phase stored
//! records as a single `collection -> rkey` map, implicitly one author. An
//! AppView, however, inherently indexes records from *many* authors, and
//! atproto's unit of authorship is the per-DID repo. So here the shared CRDT
//! document is laid out as a **multi-repo**:
//!
//!     ROOT -> <authorDid> (Map) -> <collectionNsid> (Map) -> <rkey> -> recordJson
//!
//! This extends (does not contradict) the prior `collection/rkey` layout — it
//! prefixes the author DID, so each author's records form their own sub-repo,
//! and synthesizing an `at://<did>/<collection>/<rkey>` URI is direct. The
//! author DID corresponds to the 4-tuple `subspace` (the member identity key).

use automerge::transaction::Transactable;
use automerge::{AutoCommit, ObjType, ReadDoc, ROOT};

/// Write a record into `did`'s sub-repo at `collection/rkey`.
pub fn put_record(doc: &mut AutoCommit, did: &str, collection: &str, rkey: &str, record_json: &str) {
    let repo = ensure_map(doc, ROOT, did);
    let coll = ensure_map(doc, repo, collection);
    doc.put(&coll, rkey, record_json).expect("failed to put record");
    doc.commit();
}

fn ensure_map(
    doc: &mut AutoCommit,
    parent: impl AsRef<automerge::ObjId>,
    key: &str,
) -> automerge::ObjId {
    match doc.get(parent.as_ref(), key).expect("read failed") {
        Some((_, id)) => id,
        None => doc
            .put_object(parent.as_ref(), key, ObjType::Map)
            .expect("failed to create map"),
    }
}

/// Every record across all authors, as `(did, collection, rkey, json)`, sorted
/// deterministically by `(did, collection, rkey)`.
pub fn list_all(doc: &AutoCommit) -> Vec<(String, String, String, String)> {
    let mut out = Vec::new();
    let mut dids: Vec<String> = doc.keys(ROOT).collect();
    dids.sort();
    for did in dids {
        let (_, repo) = doc.get(ROOT, &did).expect("read failed").expect("repo");
        let mut colls: Vec<String> = doc.keys(&repo).collect();
        colls.sort();
        for coll in colls {
            let (_, cmap) = doc.get(&repo, &coll).expect("read failed").expect("coll");
            let mut rkeys: Vec<String> = doc.keys(&cmap).collect();
            rkeys.sort();
            for rkey in rkeys {
                if let Some((v, _)) = doc.get(&cmap, &rkey).expect("read failed") {
                    if let Ok(json) = v.into_string() {
                        out.push((did.clone(), coll.clone(), rkey, json));
                    }
                }
            }
        }
    }
    out
}
