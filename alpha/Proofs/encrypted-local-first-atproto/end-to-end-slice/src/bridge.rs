//! The connective tissue between the private group and the public network: it
//! takes the decrypted group document + the visibility policy and *publishes*
//! the public records to the PDS via real `createRecord` calls — the step that
//! turns Phase 4's in-memory projection into actual publication (Phase 5).
//!
//! This is where the genuinely new integration issues surface:
//!   * **AT-URI identity changes at the boundary.** A record's private address is
//!     `at://<groupDid>/<coll>/<rkey>`; once published it lives at
//!     `at://<pdsDid>/<coll>/<pdsRkey>`. A public reaction's `subject` must be
//!     *rewritten* from the private URI to the published URI/CID, or it dangles.
//!   * **Identity mapping.** The group member identity (MLS credential) and the
//!     PDS account DID are *different identifiers for the same principal*; the
//!     bridge needs an explicit mapping (and, in production, a verifiable
//!     binding between them).
//!   * **Referential integrity.** A public reaction whose subject was never
//!     published (because it is private) is redacted — enforced here against
//!     real publication, not just an in-memory repo.

use std::collections::HashMap;

use reqwest::Client;
use serde_json::Value;

use crate::groupdoc;
use crate::publisher;
use crate::record::{POST_NSID, REACTION_NSID};
use crate::visibility::{MirrorPolicy, Visibility};
use automerge::AutoCommit;

/// Maps a group-side author DID to that principal's PDS session.
pub struct Identity {
    pub pds_did: String,
    pub token: String,
}

#[derive(Debug, Default)]
pub struct PublishStats {
    pub published_posts: usize,
    pub published_reactions: usize,
    pub kept_private: usize,
    pub redacted_refs: usize,
}

/// Publish the public-tagged records of `group_doc` to the PDS at `base`.
/// Returns the publish stats and the private→public URI map (for assertions).
pub async fn mirror_publish(
    client: &Client,
    base: &str,
    group_doc: &AutoCommit,
    policy: &MirrorPolicy,
    identities: &HashMap<String, Identity>,
    log: &mut Vec<String>,
) -> (PublishStats, HashMap<String, (String, String)>) {
    let mut stats = PublishStats::default();
    // private group URI -> (published public URI, published CID)
    let mut uri_map: HashMap<String, (String, String)> = HashMap::new();

    let all = groupdoc::list_all(group_doc);

    // Pass 1: publish public posts, recording the URI/CID they land at.
    for (group_did, collection, rkey, json) in &all {
        if collection != POST_NSID {
            continue;
        }
        let group_uri = crate::record::at_uri(group_did, collection, rkey);
        if policy.visibility(&group_uri) != Visibility::Public {
            stats.kept_private += 1;
            continue;
        }
        let identity = identities.get(group_did).expect("no PDS identity for author");
        let record: Value = serde_json::from_str(json).unwrap();
        let (pub_uri, pub_cid) =
            publisher::create_record(client, base, &identity.token, &identity.pds_did, collection, &record).await;
        log.push(format!("post  {group_uri}\n   -> {pub_uri}  (cid {})", &pub_cid[..24]));
        uri_map.insert(group_uri, (pub_uri, pub_cid));
        stats.published_posts += 1;
    }

    // Pass 2: publish public reactions, rewriting subject refs private->public.
    for (group_did, collection, rkey, json) in &all {
        if collection != REACTION_NSID {
            continue;
        }
        let group_uri = crate::record::at_uri(group_did, collection, rkey);
        if policy.visibility(&group_uri) != Visibility::Public {
            stats.kept_private += 1;
            continue;
        }
        let mut record: Value = serde_json::from_str(json).unwrap();
        let subject_uri = record["subject"]["uri"].as_str().unwrap_or("").to_string();

        // Referential integrity: the subject must itself have been published.
        let Some((pub_subject_uri, pub_subject_cid)) = uri_map.get(&subject_uri) else {
            log.push(format!("reaction {group_uri}\n   -> REDACTED (subject {subject_uri} is private/unpublished)"));
            stats.redacted_refs += 1;
            continue;
        };

        // Rewrite the strongRef from the private address to the public one.
        record["subject"]["uri"] = Value::String(pub_subject_uri.clone());
        record["subject"]["cid"] = Value::String(pub_subject_cid.clone());

        let identity = identities.get(group_did).expect("no PDS identity for author");
        let (pub_uri, _cid) =
            publisher::create_record(client, base, &identity.token, &identity.pds_did, collection, &record).await;
        log.push(format!("react {group_uri}\n   -> {pub_uri}  (subject rewritten -> {pub_subject_uri})"));
        stats.published_reactions += 1;
    }

    (stats, uri_map)
}
