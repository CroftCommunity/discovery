//! Lexicon record types.
//!
//! Each record serializes to JSON matching its lexicon, including a `$type`
//! field set to the **bare NSID** (atproto rule: `main` types are referenced in
//! `$type` as just the NSID, with no `#main` suffix; record objects must always
//! carry `$type`).
//!
//! Records are hand-written rather than generated: there are only two, and a
//! code generator would be more machinery than the interop question needs. (No
//! lexicon→Rust codegen crate resolved cleanly for the current lexicon spec on
//! this toolchain; see README.)

use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

/// Bare NSID of the post record type.
pub const POST_NSID: &str = "org.croftc.experiment.feed.post";
/// Bare NSID of the reaction record type.
pub const REACTION_NSID: &str = "org.croftc.experiment.feed.reaction";

/// A `feed.post` record.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Post {
    #[serde(rename = "$type")]
    pub type_: String,
    pub text: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub langs: Option<Vec<String>>,
}

impl Post {
    pub fn new(text: impl Into<String>, langs: Option<Vec<String>>) -> Self {
        Self {
            type_: POST_NSID.to_string(),
            text: text.into(),
            created_at: now_iso8601(),
            langs,
        }
    }
}

/// A strongRef-style pointer to a specific record (local analogue of
/// `com.atproto.repo.strongRef`).
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StrongRef {
    pub uri: String,
    pub cid: String,
}

/// A `feed.reaction` record.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Reaction {
    #[serde(rename = "$type")]
    pub type_: String,
    pub subject: StrongRef,
    pub emoji: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

impl Reaction {
    pub fn new(subject: StrongRef, emoji: impl Into<String>) -> Self {
        Self {
            type_: REACTION_NSID.to_string(),
            subject,
            emoji: emoji.into(),
            created_at: now_iso8601(),
        }
    }
}

/// Build an `at://` URI for a record: `at://<authority>/<collection>/<rkey>`.
/// `authority` here is the group-scoped author identity (a DID would go here in
/// a published deployment).
pub fn at_uri(authority: &str, collection: &str, rkey: &str) -> String {
    format!("at://{authority}/{collection}/{rkey}")
}

/// Content identifier for a record's canonical JSON.
///
/// This phase computes the REAL atproto CID (CIDv1, DAG-CBOR codec, SHA-256),
/// replacing the prior `b3-…` stand-in — see `content_id.rs`. The record JSON is
/// parsed and re-encoded as DAG-CBOR (records here contain only strings/arrays/
/// objects, so the round-trip is lossless).
pub fn cid_for(record_json: &str) -> String {
    let value: serde_json::Value =
        serde_json::from_str(record_json).expect("record is valid JSON");
    crate::content_id::record_cid(&value)
}

// ---------------------------------------------------------------------------
// TID record keys
// ---------------------------------------------------------------------------

const B32_SORTABLE: &[u8; 32] = b"234567abcdefghijklmnopqrstuvwxyz";
static LAST_TID: AtomicU64 = AtomicU64::new(0);

/// Generate an atproto-style TID: a 13-character, lexicographically-sortable,
/// base32-sortable encoding of a 64-bit value `(micros << 10) | clock_id`.
/// Strictly monotonic within the process so rkeys sort in creation order.
pub fn new_tid() -> String {
    let micros = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_micros() as u64;
    let clock = rand::random::<u64>() & 0x3ff;
    let mut val = (micros << 10) | clock;

    // Bump to keep TIDs strictly increasing even if the clock doesn't advance.
    loop {
        let prev = LAST_TID.load(Ordering::Relaxed);
        if val <= prev {
            val = prev + 1;
        }
        if LAST_TID
            .compare_exchange(prev, val, Ordering::Relaxed, Ordering::Relaxed)
            .is_ok()
        {
            break;
        }
    }
    s32_encode(val)
}

fn s32_encode(mut v: u64) -> String {
    let mut out = [0u8; 13];
    for slot in out.iter_mut().rev() {
        *slot = B32_SORTABLE[(v & 0x1f) as usize];
        v >>= 5;
    }
    String::from_utf8(out.to_vec()).unwrap()
}

// ---------------------------------------------------------------------------
// Datetimes
// ---------------------------------------------------------------------------

/// Current time as an atproto-conformant datetime string, e.g.
/// `2026-06-13T11:18:53.000Z` (UTC, milliseconds, `Z` timezone).
pub fn now_iso8601() -> String {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let secs = now.as_secs() as i64;
    let millis = now.subsec_millis();
    let days = secs.div_euclid(86_400);
    let tod = secs.rem_euclid(86_400);
    let (y, m, d) = civil_from_days(days);
    let (hh, mm, ss) = (tod / 3600, (tod % 3600) / 60, tod % 60);
    format!("{y:04}-{m:02}-{d:02}T{hh:02}:{mm:02}:{ss:02}.{millis:03}Z")
}

/// Howard Hinnant's days→civil-date algorithm (proleptic Gregorian).
fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as i64; // [0, 146096]
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365; // [0,399]
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100); // [0,365]
    let mp = (5 * doy + 2) / 153; // [0,11]
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32; // [1,31]
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32; // [1,12]
    (if m <= 2 { y + 1 } else { y }, m, d)
}
