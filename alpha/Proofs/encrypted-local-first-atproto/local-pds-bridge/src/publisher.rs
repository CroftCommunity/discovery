//! The publishing client — talks the real XRPC HTTP wire to a PDS. This is the
//! exact code path that would target a live PDS (bsky.social / a custom PDS);
//! only `base` changes. App-password auth via `createSession`; `createRecord`
//! with a Bearer token.

use reqwest::Client;
use serde_json::{json, Value};

/// `com.atproto.server.createSession` -> (did, accessJwt).
pub async fn create_session(client: &Client, base: &str, identifier: &str, password: &str) -> (String, String) {
    let resp: Value = client
        .post(format!("{base}/xrpc/com.atproto.server.createSession"))
        .json(&json!({ "identifier": identifier, "password": password }))
        .send()
        .await
        .expect("createSession request failed")
        .json()
        .await
        .expect("createSession response not JSON");
    (
        resp["did"].as_str().expect("no did").to_string(),
        resp["accessJwt"].as_str().expect("no accessJwt").to_string(),
    )
}

/// `com.atproto.repo.createRecord` -> (uri, cid). Panics with the server's error
/// body on non-success, so validation rejections are visible.
pub async fn create_record(
    client: &Client,
    base: &str,
    token: &str,
    repo: &str,
    collection: &str,
    record: &Value,
) -> (String, String) {
    let resp = client
        .post(format!("{base}/xrpc/com.atproto.repo.createRecord"))
        .header("Authorization", format!("Bearer {token}"))
        .json(&json!({ "repo": repo, "collection": collection, "record": record }))
        .send()
        .await
        .expect("createRecord request failed");
    let status = resp.status();
    let body: Value = resp.json().await.unwrap_or(Value::Null);
    if !status.is_success() {
        panic!("createRecord failed ({status}): {body}");
    }
    (
        body["uri"].as_str().expect("no uri").to_string(),
        body["cid"].as_str().expect("no cid").to_string(),
    )
}
