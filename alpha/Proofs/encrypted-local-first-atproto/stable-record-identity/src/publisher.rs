//! Publishing client for the record-identity demo. `create_record` always sends
//! an explicit `rkey` (pinned at creation); `put_record` edits in place;
//! `get_record` reads back to confirm CID changes. Returns the raw status so the
//! demo can show idempotency (200) vs. conflict (409).

use reqwest::{Client, StatusCode};
use serde_json::{json, Value};

pub async fn create_session(client: &Client, base: &str, identifier: &str, password: &str) -> (String, String) {
    let r: Value = client
        .post(format!("{base}/xrpc/com.atproto.server.createSession"))
        .json(&json!({ "identifier": identifier, "password": password }))
        .send().await.unwrap().json().await.unwrap();
    (r["did"].as_str().unwrap().to_string(), r["accessJwt"].as_str().unwrap().to_string())
}

/// Returns (status, body). Body has uri/cid/idempotent on success, or an error.
pub async fn create_record(
    client: &Client, base: &str, token: &str, repo: &str, collection: &str, rkey: &str, record: &Value,
) -> (StatusCode, Value) {
    let resp = client
        .post(format!("{base}/xrpc/com.atproto.repo.createRecord"))
        .header("Authorization", format!("Bearer {token}"))
        .json(&json!({ "repo": repo, "collection": collection, "rkey": rkey, "record": record }))
        .send().await.unwrap();
    let status = resp.status();
    let body = resp.json().await.unwrap_or(Value::Null);
    (status, body)
}

pub async fn put_record(
    client: &Client, base: &str, token: &str, repo: &str, collection: &str, rkey: &str, record: &Value,
) -> (StatusCode, Value) {
    let resp = client
        .post(format!("{base}/xrpc/com.atproto.repo.putRecord"))
        .header("Authorization", format!("Bearer {token}"))
        .json(&json!({ "repo": repo, "collection": collection, "rkey": rkey, "record": record }))
        .send().await.unwrap();
    let status = resp.status();
    let body = resp.json().await.unwrap_or(Value::Null);
    (status, body)
}

pub async fn get_record(client: &Client, base: &str, repo: &str, collection: &str, rkey: &str) -> Value {
    client
        .get(format!("{base}/xrpc/com.atproto.repo.getRecord?repo={repo}&collection={collection}&rkey={rkey}"))
        .send().await.unwrap().json().await.unwrap()
}
