//! Library surface for the AppView validation experiment, so multiple binaries
//! (the `appview-validation` ingest/index/serve demo and the `publish` loop test)
//! can share the ingest, index and reporting code.

pub mod atproto;
pub mod index;
pub mod jetstream;
pub mod lexicon;
pub mod record_source;
pub mod report;
pub mod server;
pub mod serviceauth;
