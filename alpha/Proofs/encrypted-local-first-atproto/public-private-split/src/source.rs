//! The AppView ingest contract (same shape as Phases 3a/3b's
//! `source::{RecordEvent, RecordSource}`) plus a `PublicRepoSource` over the
//! public projection. This closes the loop: the public side of the split feeds
//! the *same* AppView ingest contract the prior phases built — a public AppView
//! would index exactly these events, and only these (never private records).

use serde_json::Value;

use crate::public_repo::PublicRepo;

#[derive(Clone, Debug)]
pub struct RecordEvent {
    pub action: &'static str, // "create" for a one-shot projection replay
    pub did: String,
    pub collection: String,
    pub rkey: String,
    pub record: Value,
}

impl RecordEvent {
    pub fn uri(&self) -> String {
        format!("at://{}/{}/{}", self.did, self.collection, self.rkey)
    }
}

pub trait RecordSource {
    fn events(&mut self) -> Vec<RecordEvent>;
}

/// Emits ingest events from the public repo — what a public AppView consumes.
pub struct PublicRepoSource<'a> {
    repo: &'a PublicRepo,
}

impl<'a> PublicRepoSource<'a> {
    pub fn new(repo: &'a PublicRepo) -> Self {
        Self { repo }
    }
}

impl RecordSource for PublicRepoSource<'_> {
    fn events(&mut self) -> Vec<RecordEvent> {
        self.repo
            .records()
            .into_iter()
            .map(|(did, collection, rkey, record)| RecordEvent {
                action: "create",
                did,
                collection,
                rkey,
                record: record.clone(),
            })
            .collect()
    }
}
