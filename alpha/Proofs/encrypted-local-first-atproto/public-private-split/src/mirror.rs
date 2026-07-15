//! The mirror: projects public-tagged records from the private encrypted group
//! document into the public cleartext repo. This is the public/private split.
//!
//! Two privacy rules are enforced here:
//!   1. **Default-deny**: only records explicitly marked `Public` are mirrored.
//!   2. **No dangling references to private records**: a public reaction whose
//!      `subject` points at a record that is NOT public is *redacted* (dropped),
//!      because publishing it would leak the existence and AT-URI of a private
//!      record. This is a real cross-boundary integrity rule, not cosmetic.
//!
//! Public records are validated against their lexicon before publishing — what
//! crosses the boundary must be conformant atproto data.

use std::collections::HashSet;

use automerge::AutoCommit;
use serde_json::Value;

use crate::groupdoc;
use crate::lexicon::Lexicon;
use crate::public_repo::PublicRepo;
use crate::record;
use crate::visibility::{MirrorPolicy, Visibility};

#[derive(Debug, Default)]
pub struct MirrorStats {
    pub considered: usize,
    pub mirrored: usize,
    pub kept_private: usize,
    pub redacted_refs: usize,
}

pub fn mirror(
    private_doc: &AutoCommit,
    policy: &MirrorPolicy,
    post_lex: &Lexicon,
    reaction_lex: &Lexicon,
) -> (PublicRepo, MirrorStats) {
    let mut public = PublicRepo::new();
    let mut stats = MirrorStats::default();

    let all = groupdoc::list_all(private_doc);

    // Which private URIs are public? Needed to check reaction subjects.
    let public_uris: HashSet<String> = all
        .iter()
        .filter_map(|(did, coll, rkey, _)| {
            let uri = record::at_uri(did, coll, rkey);
            (policy.visibility(&uri) == Visibility::Public).then_some(uri)
        })
        .collect();

    for (did, collection, rkey, json) in &all {
        stats.considered += 1;
        let uri = record::at_uri(did, collection, rkey);

        // Rule 1: default-deny.
        if policy.visibility(&uri) != Visibility::Public {
            stats.kept_private += 1;
            continue;
        }

        let value: Value = serde_json::from_str(json).expect("record is valid JSON");

        // Rule 2: a public reaction must not reference a non-public subject.
        if collection == reaction_lex.id() {
            let subject = value["subject"]["uri"].as_str().unwrap_or("");
            if !public_uris.contains(subject) {
                stats.redacted_refs += 1;
                continue; // redact: dropping avoids leaking the private subject
            }
        }

        // What crosses the boundary must be valid atproto.
        let lex = if collection == post_lex.id() { post_lex } else { reaction_lex };
        lex.validate(&value).expect("public record must be lexicon-valid");

        public.put(did, collection, rkey, value);
        stats.mirrored += 1;
    }

    (public, stats)
}
