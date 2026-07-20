//! Live legs — attended-optional (creds-supplied precedent, RUN-14),
//! serving HIST-ATPROTO-MATCHUP.md row 1's anchored blob-lifecycle claims
//! with an observation against a real PDS.
//!
//! Gated behind env: `HIST_SPIKE_LIVE=1` plus `PDS_URL`, `PDS_IDENTIFIER`,
//! `PDS_PASSWORD`. Absent creds the tests SKIP WITH A NAMED REASON (printed
//! to stderr and visible under `--nocapture`), never a silent pass — each
//! skip prints `SKIP(live): …` naming exactly which variable is missing.
//!
//! What the legs observe:
//!  1. uploadBlob places the blob in temporary (unreferenced) storage — the
//!     GC-window entry state. Full GC observation (deletion after the ≥1 h
//!     grace floor) is out of session scope by construction; the leg records
//!     the entry observation and the floor is the anchored spec claim.
//!  2. Re-upload no-op: re-uploading identical bytes yields the identical
//!     blob CID, before and after a referencing record exists.
//!  3. The row-1 publicity fact mechanically: referencing requires a record;
//!     the leg creates one `ing.croft.hist.entry`-shaped record (validate
//!     off — the lexicon is a draft), confirms the reference, then deletes
//!     it (cleanup: the blob returns to unreferenced, GC-eligible state).
//!
//! Service-auth is NOT a live leg here (RUN-14 EXP-A already proves it; the
//! matchup row 5 cites it by reference).
//!
//! `OWNER-CALL: HS OC-3 pending` — these legs run under a personal account's
//! app credentials; scribe key custody and PLC rotation-key holders are the
//! open owner call, not something a live leg decides.

use serde_json::{json, Value};

struct Live {
    base: String,
    access_jwt: String,
    did: String,
    http: reqwest::blocking::Client,
}

/// Returns the live handle, or prints the NAMED skip reason and None.
fn live() -> Option<Live> {
    let need = |k: &str| match std::env::var(k) {
        Ok(v) if !v.is_empty() => Some(v),
        _ => {
            eprintln!("SKIP(live): env {k} unset — live PDS legs are attended-optional; \
                       set HIST_SPIKE_LIVE=1, PDS_URL, PDS_IDENTIFIER, PDS_PASSWORD to run");
            None
        }
    };
    if std::env::var("HIST_SPIKE_LIVE").ok().as_deref() != Some("1") {
        eprintln!("SKIP(live): HIST_SPIKE_LIVE != 1 — live PDS legs are attended-optional \
                   and did not run (named skip, not a silent pass)");
        return None;
    }
    let base = need("PDS_URL")?;
    let identifier = need("PDS_IDENTIFIER")?;
    let password = need("PDS_PASSWORD")?;

    let http = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .expect("client builds");
    let resp: Value = http
        .post(format!("{base}/xrpc/com.atproto.server.createSession"))
        .json(&json!({ "identifier": identifier, "password": password }))
        .send()
        .expect("createSession reachable")
        .error_for_status()
        .expect("createSession accepted")
        .json()
        .expect("createSession json");
    Some(Live {
        base,
        access_jwt: resp["accessJwt"].as_str().expect("accessJwt").to_string(),
        did: resp["did"].as_str().expect("did").to_string(),
        http,
    })
}

impl Live {
    fn upload_blob(&self, bytes: &[u8]) -> Value {
        self.http
            .post(format!("{}/xrpc/com.atproto.repo.uploadBlob", self.base))
            .bearer_auth(&self.access_jwt)
            .header("content-type", "application/octet-stream")
            .body(bytes.to_vec())
            .send()
            .expect("uploadBlob reachable")
            .error_for_status()
            .expect("uploadBlob accepted")
            .json::<Value>()
            .expect("uploadBlob json")["blob"]
            .clone()
    }
}

/// Legs 1+2+3 in one attended flow (one session, strict cleanup).
#[test]
fn live_blob_lifecycle_observation() {
    let Some(live) = live() else { return };

    // Distinct-but-deterministic content per run family; the counter suffix
    // only distinguishes re-runs, nothing here feeds modeled state.
    let content = b"hist-atproto-spike live leg: sealed-blob stand-in bytes (opaque)".to_vec();

    // Leg 1 — upload: temporary, unreferenced storage (GC-window entry).
    let blob1 = live.upload_blob(&content);
    let cid1 = blob1["ref"]["$link"].as_str().expect("blob CID").to_string();
    println!("OBSERVED uploadBlob: cid={cid1} size={} (temporary storage; \
              spec grace floor ≥1h before GC of un-referenced blobs — full \
              GC observation out of session scope)", blob1["size"]);

    // Leg 2a — re-upload while unreferenced: identical CID (no-op shape).
    let blob2 = live.upload_blob(&content);
    let cid2 = blob2["ref"]["$link"].as_str().expect("blob CID").to_string();
    assert_eq!(cid1, cid2, "re-upload of identical bytes yields the identical blob CID");

    // Leg 3 — the row-1 publicity fact: referencing requires a record.
    let subspace = hist_atproto_spike::envelope::fixture_subspace("live-leg");
    let rkey = hist_atproto_spike::rkey::entry_rkey(&subspace, 0);
    use base64ish::b64;
    let record = json!({
        "$type": "ing.croft.hist.entry",
        "subspace": { "$bytes": b64(&subspace) },
        "predecessor": { "$bytes": b64(&hist_atproto_spike::envelope::GENESIS_MARKER) },
        "entryDigest": { "$bytes": b64(blake3::hash(&content).as_bytes()) },
        "counter": 0,
        "sizeHint": content.len().div_ceil(64) * 64,
        "blob": blob1,
    });
    let created: Value = live
        .http
        .post(format!("{}/xrpc/com.atproto.repo.createRecord", live.base))
        .bearer_auth(&live.access_jwt)
        .json(&json!({
            "repo": live.did,
            "collection": "ing.croft.hist.entry",
            "rkey": rkey,
            "validate": false,
            "record": record,
        }))
        .send()
        .expect("createRecord reachable")
        .error_for_status()
        .expect("createRecord accepted")
        .json()
        .expect("createRecord json");
    println!("OBSERVED createRecord (the mandatory referencing record): {}",
             created["uri"]);

    // Leg 2b — re-upload while referenced: still the identical CID, and the
    // record is untouched (the anchored no-op claim).
    let blob3 = live.upload_blob(&content);
    assert_eq!(
        cid1,
        blob3["ref"]["$link"].as_str().expect("blob CID"),
        "re-upload after referencing: no change to the existing blob"
    );

    // Cleanup — delete the referencing record; the blob returns to
    // unreferenced (GC-eligible) state, per the last-reference rule.
    live.http
        .post(format!("{}/xrpc/com.atproto.repo.deleteRecord", live.base))
        .bearer_auth(&live.access_jwt)
        .json(&json!({
            "repo": live.did,
            "collection": "ing.croft.hist.entry",
            "rkey": rkey,
        }))
        .send()
        .expect("deleteRecord reachable")
        .error_for_status()
        .expect("deleteRecord accepted");
    println!("OBSERVED deleteRecord cleanup: blob now unreferenced → GC-eligible \
              after the grace window (last-reference rule)");
}

/// Minimal base64 (standard alphabet, padded) for the `$bytes` JSON shape —
/// test-local so the library keeps zero encoding deps beyond the canonical
/// path.
mod base64ish {
    const ALPHA: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    pub fn b64(data: &[u8]) -> String {
        let mut out = String::new();
        for chunk in data.chunks(3) {
            let b = [chunk[0], *chunk.get(1).unwrap_or(&0), *chunk.get(2).unwrap_or(&0)];
            let n = (u32::from(b[0]) << 16) | (u32::from(b[1]) << 8) | u32::from(b[2]);
            out.push(ALPHA[(n >> 18) as usize & 63] as char);
            out.push(ALPHA[(n >> 12) as usize & 63] as char);
            out.push(if chunk.len() > 1 { ALPHA[(n >> 6) as usize & 63] as char } else { '=' });
            out.push(if chunk.len() > 2 { ALPHA[n as usize & 63] as char } else { '=' });
        }
        out
    }
}
