//! Minimal atproto *read* helpers: DID-document / PDS resolution, repo record
//! listing (`com.atproto.repo.listRecords`) with cursor pagination, and single
//! record fetch (`com.atproto.repo.getRecord`).
//!
//! This is the half of atproto the firehose phases never touched: reading a repo
//! directly from its PDS, and resolving identity from the PLC directory.

use anyhow::{anyhow, Context, Result};
use serde_json::{json, Value};

/// PLC directory — resolves `did:plc:*` to a DID document.
pub const PLC_DIRECTORY: &str = "https://plc.directory";

/// The bsky entryway, used for createSession and (proxied) writes.
pub const ENTRYWAY: &str = "https://bsky.social";

/// An authenticated session (access JWT held in memory only).
pub struct Session {
    pub did: String,
    pub handle: String,
    pub jwt: String,
}

/// `com.atproto.server.createSession` (app/account-password auth).
pub async fn create_session(
    http: &reqwest::Client,
    base: &str,
    identifier: &str,
    password: &str,
) -> Result<Session> {
    let v: Value = http
        .post(format!("{base}/xrpc/com.atproto.server.createSession"))
        .json(&json!({ "identifier": identifier, "password": password }))
        .send()
        .await?
        .error_for_status()
        .context("createSession failed (check identifier / password)")?
        .json()
        .await?;
    Ok(Session {
        did: v["did"].as_str().ok_or_else(|| anyhow!("no did"))?.to_string(),
        handle: v["handle"].as_str().unwrap_or("<unknown>").to_string(),
        jwt: v["accessJwt"].as_str().ok_or_else(|| anyhow!("no accessJwt"))?.to_string(),
    })
}

/// `com.atproto.repo.createRecord` → returns the new record's at:// URI.
pub async fn create_record(
    http: &reqwest::Client,
    base: &str,
    jwt: &str,
    did: &str,
    collection: &str,
    record: Value,
) -> Result<String> {
    let v: Value = http
        .post(format!("{base}/xrpc/com.atproto.repo.createRecord"))
        .bearer_auth(jwt)
        .json(&json!({ "repo": did, "collection": collection, "record": record }))
        .send()
        .await?
        .error_for_status()
        .context("createRecord failed")?
        .json()
        .await?;
    v["uri"].as_str().map(str::to_string).ok_or_else(|| anyhow!("no uri: {v}"))
}

/// `com.atproto.repo.deleteRecord`.
pub async fn delete_record(
    http: &reqwest::Client,
    base: &str,
    jwt: &str,
    did: &str,
    collection: &str,
    rkey: &str,
) -> Result<()> {
    http.post(format!("{base}/xrpc/com.atproto.repo.deleteRecord"))
        .bearer_auth(jwt)
        .json(&json!({ "repo": did, "collection": collection, "rkey": rkey }))
        .send()
        .await?
        .error_for_status()
        .context("deleteRecord failed")?;
    Ok(())
}

pub fn client() -> reqwest::Client {
    reqwest::Client::builder()
        .user_agent("appview-validation-experiment/0.1")
        .build()
        .expect("build reqwest client")
}

/// Fetch the DID document for a `did:plc:*` from the PLC directory.
pub async fn resolve_did_doc(http: &reqwest::Client, did: &str) -> Result<Value> {
    Ok(http
        .get(format!("{PLC_DIRECTORY}/{did}"))
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?)
}

/// The account's PDS service endpoint from its DID document.
pub fn pds_endpoint(did_doc: &Value) -> Option<String> {
    did_doc
        .get("service")?
        .as_array()?
        .iter()
        .find(|s| s.get("id").and_then(Value::as_str) == Some("#atproto_pds"))
        .and_then(|s| s.get("serviceEndpoint"))
        .and_then(Value::as_str)
        .map(str::to_string)
}

/// The account's primary handle from its DID document (`alsoKnownAs: at://handle`).
pub fn handle_from_doc(did_doc: &Value) -> Option<String> {
    did_doc
        .get("alsoKnownAs")?
        .as_array()?
        .iter()
        .filter_map(Value::as_str)
        .find_map(|aka| aka.strip_prefix("at://"))
        .map(str::to_string)
}

/// Resolve a DID to (pds_endpoint, handle) in one DID-document fetch.
pub async fn resolve_identity(http: &reqwest::Client, did: &str) -> Result<(String, String)> {
    let doc = resolve_did_doc(http, did).await?;
    let pds = pds_endpoint(&doc).ok_or_else(|| anyhow!("no #atproto_pds in DID doc for {did}"))?;
    let handle = handle_from_doc(&doc).unwrap_or_else(|| "<unknown>".to_string());
    Ok((pds, handle))
}

/// One page of `listRecords`: returns the records array and the next cursor (if any).
pub async fn list_records_page(
    http: &reqwest::Client,
    pds: &str,
    did: &str,
    collection: &str,
    limit: u32,
    cursor: Option<&str>,
) -> Result<(Vec<Value>, Option<String>)> {
    let limit = limit.to_string();
    let mut q: Vec<(&str, &str)> =
        vec![("repo", did), ("collection", collection), ("limit", &limit)];
    if let Some(c) = cursor {
        q.push(("cursor", c));
    }
    let v: Value = http
        .get(format!("{pds}/xrpc/com.atproto.repo.listRecords"))
        .query(&q)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;
    let records = v
        .get("records")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let next = v.get("cursor").and_then(Value::as_str).map(str::to_string);
    Ok((records, next))
}

/// Fetch a single record. Returns the `record` body (not the envelope).
pub async fn get_record(
    http: &reqwest::Client,
    pds: &str,
    did: &str,
    collection: &str,
    rkey: &str,
) -> Result<Value> {
    let v: Value = http
        .get(format!("{pds}/xrpc/com.atproto.repo.getRecord"))
        .query(&[("repo", did), ("collection", collection), ("rkey", rkey)])
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;
    Ok(v.get("value").cloned().unwrap_or(v))
}

/// Fetch an actor's profile (`app.bsky.actor.profile/self`) → (displayName, description).
/// Returns Nones if the account has no profile record.
pub async fn get_profile(
    http: &reqwest::Client,
    pds: &str,
    did: &str,
) -> (Option<String>, Option<String>) {
    match get_record(http, pds, did, "app.bsky.actor.profile", "self").await {
        Ok(v) => (
            v.get("displayName").and_then(Value::as_str).map(str::to_string),
            v.get("description").and_then(Value::as_str).map(str::to_string),
        ),
        Err(_) => (None, None),
    }
}

/// Parse an at:// URI into (did, collection, rkey).
pub fn parse_at_uri(uri: &str) -> Option<(String, String, String)> {
    let rest = uri.strip_prefix("at://")?;
    let mut parts = rest.splitn(3, '/');
    let did = parts.next()?.to_string();
    let collection = parts.next()?.to_string();
    let rkey = parts.next()?.to_string();
    Some((did, collection, rkey))
}
