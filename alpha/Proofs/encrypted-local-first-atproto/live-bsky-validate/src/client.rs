//! Real atproto XRPC client (app-password auth) against a live PDS. Same wire
//! shape as the local-PDS phases; only the base URL + real TLS differ.

use reqwest::{Client, StatusCode};
use serde_json::{json, Value};

pub struct Session {
    pub did: String,
    pub handle: String,
    pub access_jwt: String,
}

/// `com.atproto.server.createSession` with an app password.
pub async fn create_session(client: &Client, base: &str, identifier: &str, password: &str) -> Result<Session, String> {
    let resp = client
        .post(format!("{base}/xrpc/com.atproto.server.createSession"))
        .json(&json!({ "identifier": identifier, "password": password }))
        .send().await.map_err(|e| e.to_string())?;
    let status = resp.status();
    let body: Value = resp.json().await.unwrap_or(Value::Null);
    if !status.is_success() {
        return Err(format!("createSession {status}: {body}"));
    }
    Ok(Session {
        did: body["did"].as_str().unwrap_or_default().to_string(),
        handle: body["handle"].as_str().unwrap_or_default().to_string(),
        access_jwt: body["accessJwt"].as_str().unwrap_or_default().to_string(),
    })
}

/// `com.atproto.repo.createRecord` with an explicit rkey (pinned identity).
/// Returns (status, body) so the caller can report exactly how the live PDS
/// responded to a custom-NSID record.
pub async fn create_record(
    client: &Client, base: &str, jwt: &str, repo: &str, collection: &str, rkey: &str, record: &Value,
) -> (StatusCode, Value) {
    let resp = client
        .post(format!("{base}/xrpc/com.atproto.repo.createRecord"))
        .bearer_auth(jwt)
        .json(&json!({ "repo": repo, "collection": collection, "rkey": rkey, "record": record }))
        .send().await;
    match resp {
        Ok(r) => {
            let s = r.status();
            (s, r.json().await.unwrap_or(Value::Null))
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, json!({ "transport_error": e.to_string() })),
    }
}

fn pct(s: &str) -> String {
    let mut out = String::new();
    for b in s.bytes() {
        if b.is_ascii_alphanumeric() || matches!(b, b'-' | b'.' | b'_' | b'~') {
            out.push(b as char);
        } else {
            out.push_str(&format!("%{b:02X}"));
        }
    }
    out
}

/// `com.atproto.repo.getRecord` (public read; no auth needed).
pub async fn get_record(client: &Client, base: &str, repo: &str, collection: &str, rkey: &str) -> (StatusCode, Value) {
    let url = format!(
        "{base}/xrpc/com.atproto.repo.getRecord?repo={}&collection={}&rkey={}",
        pct(repo), pct(collection), pct(rkey)
    );
    let resp = client.get(url).send().await;
    match resp {
        Ok(r) => {
            let s = r.status();
            (s, r.json().await.unwrap_or(Value::Null))
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, json!({ "transport_error": e.to_string() })),
    }
}

/// `com.atproto.repo.deleteRecord` — cleanup so the test accounts stay tidy.
pub async fn delete_record(client: &Client, base: &str, jwt: &str, repo: &str, collection: &str, rkey: &str) -> StatusCode {
    client
        .post(format!("{base}/xrpc/com.atproto.repo.deleteRecord"))
        .bearer_auth(jwt)
        .json(&json!({ "repo": repo, "collection": collection, "rkey": rkey }))
        .send().await
        .map(|r| r.status())
        .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
}
