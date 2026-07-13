//! Minimal AT Protocol XRPC client over plain HTTP/JSON.
//!
//! We talk to the real endpoints directly (`com.atproto.*`) rather than going
//! through `atrium`. These XRPC methods are stable, simple JSON calls, and
//! avoiding the higher-level client removes the API-churn risk the brief warns
//! about while keeping the round-trip fully under our control. The OAuth value
//! `atrium-oauth` would have added is moot here anyway (see README friction log:
//! a headless container cannot satisfy the OAuth public-callback requirement).

use anyhow::{anyhow, bail, Context, Result};
use serde::Deserialize;
use serde_json::{json, Value};

const PUBLIC_RESOLVER: &str = "https://bsky.social";
const PLC_DIRECTORY: &str = "https://plc.directory";

/// An authenticated session bound to a specific PDS.
#[derive(Debug, Clone)]
pub struct Session {
    pub did: String,
    pub handle: String,
    pub pds: String,
    pub access_jwt: String,
}

pub struct XrpcClient {
    http: reqwest::Client,
}

#[derive(Deserialize)]
struct ResolveHandleResp {
    did: String,
}

#[derive(Deserialize)]
struct CreateSessionResp {
    did: String,
    handle: String,
    #[serde(rename = "accessJwt")]
    access_jwt: String,
    #[serde(rename = "didDoc")]
    did_doc: Option<Value>,
}

#[derive(Debug, Deserialize)]
pub struct CreateRecordResp {
    pub uri: String,
    pub cid: String,
    #[serde(rename = "validationStatus")]
    pub validation_status: Option<String>,
}

impl XrpcClient {
    pub fn new() -> Result<Self> {
        let http = reqwest::Client::builder()
            .user_agent("public-roundtrip-experiment/0.1")
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .context("building HTTP client")?;
        Ok(Self { http })
    }

    /// Resolve a handle to a DID using the public resolver (entryway).
    pub async fn resolve_handle(&self, handle: &str) -> Result<String> {
        let url = format!("{PUBLIC_RESOLVER}/xrpc/com.atproto.identity.resolveHandle");
        let resp = self
            .http
            .get(&url)
            .query(&[("handle", handle)])
            .send()
            .await
            .context("resolveHandle request")?;
        if !resp.status().is_success() {
            bail!("resolveHandle failed: HTTP {} — {}", resp.status(), resp.text().await.unwrap_or_default());
        }
        Ok(resp.json::<ResolveHandleResp>().await?.did)
    }

    /// Fetch the full DID document from the PLC directory.
    pub async fn fetch_plc_doc(&self, did: &str) -> Result<Value> {
        let url = format!("{PLC_DIRECTORY}/{did}");
        Ok(self
            .http
            .get(&url)
            .send()
            .await
            .context("PLC directory request")?
            .error_for_status()
            .context("PLC directory status")?
            .json()
            .await?)
    }

    /// Query a labeler's `com.atproto.label.queryLabels` for labels on the given
    /// URI patterns (a pattern may be a DID, an AT-URI, or `*`).
    pub async fn query_labels(&self, labeler_endpoint: &str, patterns: &[&str], limit: u32) -> Result<Value> {
        let url = format!("{labeler_endpoint}/xrpc/com.atproto.label.queryLabels");
        let mut q: Vec<(String, String)> = patterns.iter().map(|p| ("uriPatterns".to_string(), p.to_string())).collect();
        q.push(("limit".to_string(), limit.to_string()));
        Ok(self
            .http
            .get(&url)
            .query(&q)
            .send()
            .await
            .context("queryLabels request")?
            .error_for_status()
            .context("queryLabels status")?
            .json()
            .await?)
    }

    /// Fetch the PLC audit log (signed operation history) for a DID.
    pub async fn fetch_plc_audit(&self, did: &str) -> Result<Value> {
        let url = format!("{PLC_DIRECTORY}/{did}/log/audit");
        Ok(self
            .http
            .get(&url)
            .send()
            .await
            .context("PLC audit request")?
            .error_for_status()
            .context("PLC audit status")?
            .json()
            .await?)
    }

    /// Resolve a `did:plc` to its PDS service endpoint via the PLC directory.
    /// Demonstrates the DID-document → `#atproto_pds` service lookup.
    pub async fn resolve_pds_via_plc(&self, did: &str) -> Result<String> {
        let url = format!("{PLC_DIRECTORY}/{did}");
        let doc: Value = self
            .http
            .get(&url)
            .send()
            .await
            .context("PLC directory request")?
            .error_for_status()
            .context("PLC directory status")?
            .json()
            .await?;
        pds_from_did_doc(&doc).ok_or_else(|| anyhow!("no #atproto_pds service in DID doc for {did}"))
    }

    /// App-password session via `com.atproto.server.createSession`.
    ///
    /// This is the headless-friendly auth path. It is NOT full OAuth — see the
    /// README friction log for why OAuth is infeasible in this environment.
    /// `host` is the entryway/PDS to authenticate against (e.g. https://bsky.social).
    pub async fn create_session(&self, host: &str, identifier: &str, app_password: &str) -> Result<Session> {
        let url = format!("{host}/xrpc/com.atproto.server.createSession");
        let resp = self
            .http
            .post(&url)
            .json(&json!({ "identifier": identifier, "password": app_password }))
            .send()
            .await
            .context("createSession request")?;
        if !resp.status().is_success() {
            // Never echo the password; surface only the server's error body.
            bail!("createSession failed: HTTP {} — {}", resp.status(), resp.text().await.unwrap_or_default());
        }
        let s: CreateSessionResp = resp.json().await.context("parsing createSession response")?;
        // Prefer the PDS endpoint advertised in the returned DID document; the
        // entryway (bsky.social) is not necessarily the repo's actual PDS.
        let pds = s
            .did_doc
            .as_ref()
            .and_then(pds_from_did_doc)
            .unwrap_or_else(|| host.to_string());
        Ok(Session {
            did: s.did,
            handle: s.handle,
            pds,
            access_jwt: s.access_jwt,
        })
    }

    /// Publish a record via `com.atproto.repo.createRecord`. `rkey` is omitted,
    /// so the PDS assigns a TID rkey and returns it in the AT-URI.
    pub async fn create_record(&self, s: &Session, collection: &str, record: &Value) -> Result<CreateRecordResp> {
        let url = format!("{}/xrpc/com.atproto.repo.createRecord", s.pds);
        let resp = self
            .http
            .post(&url)
            .bearer_auth(&s.access_jwt)
            .json(&json!({ "repo": s.did, "collection": collection, "record": record }))
            .send()
            .await
            .context("createRecord request")?;
        if !resp.status().is_success() {
            bail!("createRecord failed: HTTP {} — {}", resp.status(), resp.text().await.unwrap_or_default());
        }
        Ok(resp.json().await.context("parsing createRecord response")?)
    }

    /// Update (or create) a record at a specific rkey via `putRecord`.
    pub async fn put_record(&self, s: &Session, collection: &str, rkey: &str, record: &Value) -> Result<CreateRecordResp> {
        let url = format!("{}/xrpc/com.atproto.repo.putRecord", s.pds);
        let resp = self
            .http
            .post(&url)
            .bearer_auth(&s.access_jwt)
            .json(&json!({ "repo": s.did, "collection": collection, "rkey": rkey, "record": record }))
            .send()
            .await
            .context("putRecord request")?;
        if !resp.status().is_success() {
            bail!("putRecord failed: HTTP {} — {}", resp.status(), resp.text().await.unwrap_or_default());
        }
        Ok(resp.json().await.context("parsing putRecord response")?)
    }

    /// List records in a collection via `com.atproto.repo.listRecords`.
    /// Returns `(uri, cid)` pairs.
    pub async fn list_records(&self, s: &Session, collection: &str) -> Result<Vec<(String, String)>> {
        let url = format!("{}/xrpc/com.atproto.repo.listRecords", s.pds);
        let resp = self
            .http
            .get(&url)
            .bearer_auth(&s.access_jwt)
            .query(&[("repo", s.did.as_str()), ("collection", collection), ("limit", "100")])
            .send()
            .await
            .context("listRecords request")?
            .error_for_status()
            .context("listRecords status")?;
        let body: Value = resp.json().await?;
        let mut out = Vec::new();
        if let Some(records) = body.get("records").and_then(|r| r.as_array()) {
            for r in records {
                if let (Some(uri), Some(cid)) = (r.get("uri").and_then(|v| v.as_str()), r.get("cid").and_then(|v| v.as_str())) {
                    out.push((uri.to_string(), cid.to_string()));
                }
            }
        }
        Ok(out)
    }

    /// Public (unauthenticated) `listRecords` returning full record bodies,
    /// for AppView backfill. `listRecords` is a public read on the PDS.
    pub async fn list_records_full_public(
        &self,
        pds: &str,
        did: &str,
        collection: &str,
    ) -> Result<Vec<(String, String, Value)>> {
        let url = format!("{pds}/xrpc/com.atproto.repo.listRecords");
        let body: Value = self
            .http
            .get(&url)
            .query(&[("repo", did), ("collection", collection), ("limit", "100")])
            .send()
            .await
            .context("listRecords (public) request")?
            .error_for_status()
            .context("listRecords (public) status")?
            .json()
            .await?;
        let mut out = Vec::new();
        if let Some(records) = body.get("records").and_then(|r| r.as_array()) {
            for r in records {
                if let (Some(uri), Some(cid), Some(value)) = (
                    r.get("uri").and_then(|v| v.as_str()),
                    r.get("cid").and_then(|v| v.as_str()),
                    r.get("value"),
                ) {
                    out.push((uri.to_string(), cid.to_string(), value.clone()));
                }
            }
        }
        Ok(out)
    }

    /// Upload a blob via `com.atproto.repo.uploadBlob`; returns the `blob` object.
    pub async fn upload_blob(&self, s: &Session, bytes: Vec<u8>, mime: &str) -> Result<Value> {
        let url = format!("{}/xrpc/com.atproto.repo.uploadBlob", s.pds);
        let resp = self
            .http
            .post(&url)
            .bearer_auth(&s.access_jwt)
            .header("Content-Type", mime)
            .body(bytes)
            .send()
            .await
            .context("uploadBlob request")?;
        if !resp.status().is_success() {
            bail!("uploadBlob failed: HTTP {} — {}", resp.status(), resp.text().await.unwrap_or_default());
        }
        let body: Value = resp.json().await?;
        body.get("blob").cloned().context("uploadBlob: no blob in response")
    }

    /// Fetch a blob's bytes via `com.atproto.sync.getBlob`.
    pub async fn get_blob(&self, pds: &str, did: &str, cid: &str) -> Result<Vec<u8>> {
        let url = format!("{pds}/xrpc/com.atproto.sync.getBlob");
        let resp = self
            .http
            .get(&url)
            .query(&[("did", did), ("cid", cid)])
            .send()
            .await
            .context("getBlob request")?
            .error_for_status()
            .context("getBlob status")?;
        Ok(resp.bytes().await?.to_vec())
    }

    /// Fetch the full repo as a CAR file via `com.atproto.sync.getRepo`.
    pub async fn get_repo_car(&self, pds: &str, did: &str) -> Result<Vec<u8>> {
        let url = format!("{pds}/xrpc/com.atproto.sync.getRepo");
        let resp = self
            .http
            .get(&url)
            .query(&[("did", did)])
            .send()
            .await
            .context("getRepo request")?
            .error_for_status()
            .context("getRepo status")?;
        Ok(resp.bytes().await?.to_vec())
    }

    /// Public (unauthenticated) `getRecord` → `(cid, value)` for one record.
    pub async fn get_record_public(
        &self,
        pds: &str,
        did: &str,
        collection: &str,
        rkey: &str,
    ) -> Result<(String, Value)> {
        let url = format!("{pds}/xrpc/com.atproto.repo.getRecord");
        let body: Value = self
            .http
            .get(&url)
            .query(&[("repo", did), ("collection", collection), ("rkey", rkey)])
            .send()
            .await
            .context("getRecord request")?
            .error_for_status()
            .context("getRecord status")?
            .json()
            .await?;
        let cid = body.get("cid").and_then(|v| v.as_str()).unwrap_or_default().to_string();
        let value = body.get("value").cloned().unwrap_or(Value::Null);
        Ok((cid, value))
    }

    /// Delete a record via `com.atproto.repo.deleteRecord`.
    pub async fn delete_record(&self, s: &Session, collection: &str, rkey: &str) -> Result<()> {
        let url = format!("{}/xrpc/com.atproto.repo.deleteRecord", s.pds);
        let resp = self
            .http
            .post(&url)
            .bearer_auth(&s.access_jwt)
            .json(&json!({ "repo": s.did, "collection": collection, "rkey": rkey }))
            .send()
            .await
            .context("deleteRecord request")?;
        if !resp.status().is_success() {
            bail!("deleteRecord failed: HTTP {} — {}", resp.status(), resp.text().await.unwrap_or_default());
        }
        Ok(())
    }
}

/// Extract the `#atproto_pds` service endpoint from a DID document.
fn pds_from_did_doc(doc: &Value) -> Option<String> {
    let services = doc.get("service")?.as_array()?;
    for svc in services {
        let id = svc.get("id").and_then(|v| v.as_str()).unwrap_or_default();
        let typ = svc.get("type").and_then(|v| v.as_str()).unwrap_or_default();
        if id.ends_with("#atproto_pds") || typ == "AtprotoPersonalDataServer" {
            if let Some(ep) = svc.get("serviceEndpoint").and_then(|v| v.as_str()) {
                // The DID document is externally controlled, so only accept an
                // https endpoint with a host. This avoids SSRF: without it, a
                // malicious/compromised DID doc could redirect our authenticated
                // createRecord/deleteRecord calls to file:// or an internal URL.
                match url::Url::parse(ep) {
                    Ok(u) if u.scheme() == "https" && u.has_host() => {
                        return Some(ep.trim_end_matches('/').to_string());
                    }
                    _ => return None,
                }
            }
        }
    }
    None
}

/// Parse an AT-URI `at://did/collection/rkey` into its parts.
pub fn parse_at_uri(uri: &str) -> Option<(String, String, String)> {
    let rest = uri.strip_prefix("at://")?;
    let mut parts = rest.splitn(3, '/');
    let did = parts.next()?.to_string();
    let collection = parts.next()?.to_string();
    let rkey = parts.next()?.to_string();
    Some((did, collection, rkey))
}
