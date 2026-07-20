//! `HttpLeg` — the live impl of `LiveLegTrait` against a bsky-hosted PDS
//! (also works against any spec-conforming PDS if the env is pointed at one).
//!
//! Cred inputs, env-only:
//!   HIST_PDS_HOST      — entry host, e.g. https://bsky.social
//!   HIST_PDS_HANDLE    — the account handle (informational; the session call
//!                        uses the login identifier below)
//!   HIST_PDS_LOGIN     — identifier (handle OR email) for createSession
//!   HIST_PDS_APP_PASSWORD — app password (NEVER main account password)
//!
//! Every call is metered through the shared `Budget` and single-flighted at
//! ≤1 rps through the shared `Pacer`.  Rate-limit headers on any 4xx/2xx are
//! captured; a 429 or explicit RateLimitExceeded surfaces as
//! `XrpcError::Xrpc { rate_limit_headers, .. }` — the caller MUST halt.

use crate::budget::{Budget, BudgetCaps, Pacer};
use crate::did::DidDoc;
use crate::leg::*;
use serde::Deserialize;
use std::sync::Mutex;
use std::time::Duration;
use ureq::{Agent, AgentBuilder};
use std::sync::Arc as StdArc;

pub struct HttpLeg {
    pub budget: Budget,
    pub pacer: Pacer,
    entry_host: String,
    login: String,
    password: String,
    agent: Agent,
    session: Mutex<Option<SessionTokens>>,
    /// Cached DID doc lookups (did → doc JSON).
    did_docs: Mutex<std::collections::HashMap<String, serde_json::Value>>,
    /// Most-recent rate-limit headers seen on ANY response (for E8's passive
    /// observation).  Never used for control flow.
    last_rate_limit: Mutex<RateLimitHeaders>,
}

impl HttpLeg {
    /// Build from env.  Returns `Err` if the required env vars are missing.
    pub fn from_env(caps: BudgetCaps) -> Result<Self, String> {
        let entry_host = std::env::var("HIST_PDS_HOST")
            .map_err(|_| "HIST_PDS_HOST unset".to_string())?;
        let login = std::env::var("HIST_PDS_LOGIN")
            .or_else(|_| std::env::var("HIST_PDS_HANDLE"))
            .map_err(|_| "HIST_PDS_LOGIN unset".to_string())?;
        let password = std::env::var("HIST_PDS_APP_PASSWORD")
            .map_err(|_| "HIST_PDS_APP_PASSWORD unset".to_string())?;
        Ok(HttpLeg::new(entry_host, login, password, caps))
    }

    pub fn new(entry_host: String, login: String, password: String, caps: BudgetCaps) -> Self {
        // Load the environment's CA bundle (honors SSL_CERT_FILE if set, or
        // falls back to the well-known bundle path used by the agent proxy).
        let mut roots = rustls::RootCertStore::empty();
        let bundle_path = std::env::var("SSL_CERT_FILE")
            .unwrap_or_else(|_| "/root/.ccr/ca-bundle.crt".to_string());
        if let Ok(f) = std::fs::File::open(&bundle_path) {
            let mut reader = std::io::BufReader::new(f);
            for cert in rustls_pemfile::certs(&mut reader).flatten() {
                let _ = roots.add(cert);
            }
        }
        // If the bundle path is missing, we still proceed — the request will
        // fail with a clear TLS error which is more useful than silently
        // falling back to a stale set of roots.
        // Install ring as the CryptoProvider (harmless if already installed).
        let _ = rustls::crypto::ring::default_provider().install_default();
        let tls_cfg = rustls::ClientConfig::builder()
            .with_root_certificates(roots)
            .with_no_client_auth();

        let mut builder = AgentBuilder::new()
            .timeout_connect(Duration::from_secs(10))
            .timeout(Duration::from_secs(30))
            .user_agent("hist_live/0.1 (RUN-HIST-02 rev B synthetic spike)")
            .tls_config(StdArc::new(tls_cfg));
        // Honor HTTPS_PROXY / https_proxy if set.
        for var in ["HTTPS_PROXY", "https_proxy"] {
            if let Ok(p) = std::env::var(var) {
                if let Ok(proxy) = ureq::Proxy::new(&p) {
                    builder = builder.proxy(proxy);
                    break;
                }
            }
        }
        let agent: Agent = builder.build();
        HttpLeg {
            budget: Budget::new(caps),
            pacer: Pacer::one_rps(),
            entry_host,
            login,
            password,
            agent,
            session: Mutex::new(None),
            did_docs: Mutex::new(std::collections::HashMap::new()),
            last_rate_limit: Mutex::new(RateLimitHeaders::default()),
        }
    }

    pub fn last_rate_limit_headers(&self) -> RateLimitHeaders {
        self.last_rate_limit.lock().unwrap().clone()
    }

    fn ensure_session(&self) -> Result<SessionTokens, XrpcError> {
        {
            let g = self.session.lock().unwrap();
            if let Some(s) = &*g {
                return Ok(s.clone());
            }
        }
        // First: createSession against the entry host.
        let url = format!("{}/xrpc/com.atproto.server.createSession", self.entry_host);
        let body = serde_json::json!({
            "identifier": self.login,
            "password": self.password,
        });
        let resp = self.post_json_metered_read(&url, None, &body)?;
        #[derive(Deserialize)]
        struct S {
            did: String,
            handle: String,
            #[serde(rename = "accessJwt")]
            access_jwt: String,
            #[serde(rename = "refreshJwt")]
            refresh_jwt: String,
        }
        let s: S = serde_json::from_value(resp).map_err(|e| XrpcError::Decode(e.to_string()))?;
        // Fetch DID doc to find the actual PDS endpoint.
        let doc_json = self.resolve_did_doc_inner(&s.did)?;
        let doc: DidDoc =
            serde_json::from_value(doc_json).map_err(|e| XrpcError::Decode(e.to_string()))?;
        let pds = doc
            .pds_endpoint()
            .ok_or_else(|| XrpcError::Decode("no PDS service in did doc".into()))?
            .to_string();
        let tok = SessionTokens {
            did: s.did,
            handle: s.handle,
            access_jwt: s.access_jwt,
            refresh_jwt: s.refresh_jwt,
            pds_endpoint: pds,
        };
        *self.session.lock().unwrap() = Some(tok.clone());
        Ok(tok)
    }

    fn resolve_did_doc_inner(&self, did: &str) -> Result<serde_json::Value, XrpcError> {
        {
            let g = self.did_docs.lock().unwrap();
            if let Some(v) = g.get(did) {
                return Ok(v.clone());
            }
        }
        let url = if did.starts_with("did:plc:") {
            format!("https://plc.directory/{}", did)
        } else if did.starts_with("did:web:") {
            let rest = did.trim_start_matches("did:web:");
            format!("https://{}/.well-known/did.json", rest)
        } else {
            return Err(XrpcError::Decode(format!("unsupported did method: {}", did)));
        };
        let v = self.get_json_metered_read(&url, None)?;
        self.did_docs
            .lock()
            .unwrap()
            .insert(did.to_string(), v.clone());
        Ok(v)
    }

    fn capture_rate_limit(&self, resp: &ureq::Response) {
        let mut rl = RateLimitHeaders::default();
        rl.limit = resp.header("ratelimit-limit").map(str::to_string);
        rl.remaining = resp.header("ratelimit-remaining").map(str::to_string);
        rl.reset = resp.header("ratelimit-reset").map(str::to_string);
        rl.policy = resp.header("ratelimit-policy").map(str::to_string);
        if rl.limit.is_some() || rl.remaining.is_some() {
            *self.last_rate_limit.lock().unwrap() = rl;
        }
    }

    fn xrpc_error_from(&self, resp: ureq::Response) -> XrpcError {
        let status = resp.status();
        let mut rl = RateLimitHeaders::default();
        rl.limit = resp.header("ratelimit-limit").map(str::to_string);
        rl.remaining = resp.header("ratelimit-remaining").map(str::to_string);
        rl.reset = resp.header("ratelimit-reset").map(str::to_string);
        rl.policy = resp.header("ratelimit-policy").map(str::to_string);
        *self.last_rate_limit.lock().unwrap() = rl.clone();
        let body = resp.into_string().unwrap_or_default();
        let (error, message) = serde_json::from_str::<serde_json::Value>(&body)
            .ok()
            .map(|v| {
                (
                    v.get("error").and_then(|e| e.as_str()).map(String::from),
                    v.get("message").and_then(|m| m.as_str()).map(String::from),
                )
            })
            .unwrap_or((None, Some(body)));
        if status == 429
            || error.as_deref() == Some("RateLimitExceeded")
        {
            self.budget.note_rate_limit_signal();
        }
        XrpcError::Xrpc {
            http_status: status,
            error,
            message,
            rate_limit_headers: rl,
        }
    }

    fn get_json_metered_read(
        &self,
        url: &str,
        auth: Option<&str>,
    ) -> Result<serde_json::Value, XrpcError> {
        let _g = self.pacer.acquire();
        self.budget.charge_read();
        let mut req = self.agent.get(url);
        if let Some(a) = auth {
            req = req.set("Authorization", &format!("Bearer {}", a));
        }
        match req.call() {
            Ok(resp) => {
                self.capture_rate_limit(&resp);
                let v: serde_json::Value = resp
                    .into_json()
                    .map_err(|e| XrpcError::Decode(e.to_string()))?;
                Ok(v)
            }
            Err(ureq::Error::Status(_, resp)) => Err(self.xrpc_error_from(resp)),
            Err(e) => Err(XrpcError::Http(e.to_string())),
        }
    }

    fn get_bytes_metered_read(
        &self,
        url: &str,
        auth: Option<&str>,
    ) -> Result<Vec<u8>, XrpcError> {
        let _g = self.pacer.acquire();
        self.budget.charge_read();
        let mut req = self.agent.get(url);
        if let Some(a) = auth {
            req = req.set("Authorization", &format!("Bearer {}", a));
        }
        match req.call() {
            Ok(resp) => {
                self.capture_rate_limit(&resp);
                let mut buf = Vec::new();
                resp.into_reader()
                    .read_to_end(&mut buf)
                    .map_err(|e| XrpcError::Http(e.to_string()))?;
                Ok(buf)
            }
            Err(ureq::Error::Status(_, resp)) => Err(self.xrpc_error_from(resp)),
            Err(e) => Err(XrpcError::Http(e.to_string())),
        }
    }

    fn post_json_metered_read(
        &self,
        url: &str,
        auth: Option<&str>,
        body: &serde_json::Value,
    ) -> Result<serde_json::Value, XrpcError> {
        let _g = self.pacer.acquire();
        self.budget.charge_read();
        let mut req = self.agent.post(url).set("Content-Type", "application/json");
        if let Some(a) = auth {
            req = req.set("Authorization", &format!("Bearer {}", a));
        }
        match req.send_json(body.clone()) {
            Ok(resp) => {
                self.capture_rate_limit(&resp);
                let v: serde_json::Value = resp
                    .into_json()
                    .map_err(|e| XrpcError::Decode(e.to_string()))?;
                Ok(v)
            }
            Err(ureq::Error::Status(_, resp)) => Err(self.xrpc_error_from(resp)),
            Err(e) => Err(XrpcError::Http(e.to_string())),
        }
    }
}

use std::io::Read;

impl LiveLegTrait for HttpLeg {
    fn session(&self) -> Result<SessionTokens, XrpcError> {
        self.ensure_session()
    }

    fn describe_repo(&self, did: &str) -> Result<serde_json::Value, XrpcError> {
        let sess = self.ensure_session()?;
        let url = format!(
            "{}/xrpc/com.atproto.repo.describeRepo?repo={}",
            sess.pds_endpoint, did
        );
        self.get_json_metered_read(&url, Some(&sess.access_jwt))
    }

    fn apply_writes(&self, ops: Vec<ApplyWritesOp>) -> Result<ApplyWritesResp, XrpcError> {
        let n: usize = ops.iter().map(|o| o.write_count()).sum();
        // Charge the budget BEFORE the pacer/network — if the cap denies, no
        // request goes out.  This is asserted in a stub test.
        self.budget.charge_writes(n)?;
        let sess = self.ensure_session()?;
        let url = format!("{}/xrpc/com.atproto.repo.applyWrites", sess.pds_endpoint);
        let body = serde_json::json!({
            "repo": sess.did,
            "writes": ops,
        });
        // The post_json_metered_read already charges one READ per pacer slot;
        // we've already charged writes above.  Bypass the read charge here by
        // directly running the request.
        let _g = self.pacer.acquire();
        let req = self
            .agent
            .post(&url)
            .set("Content-Type", "application/json")
            .set("Authorization", &format!("Bearer {}", sess.access_jwt));
        let resp = match req.send_json(body) {
            Ok(r) => r,
            Err(ureq::Error::Status(_, r)) => return Err(self.xrpc_error_from(r)),
            Err(e) => return Err(XrpcError::Http(e.to_string())),
        };
        self.capture_rate_limit(&resp);
        let v: serde_json::Value = resp
            .into_json()
            .map_err(|e| XrpcError::Decode(e.to_string()))?;
        serde_json::from_value(v).map_err(|e| XrpcError::Decode(e.to_string()))
    }

    fn get_record(&self, collection: &str, rkey: &str) -> Result<GetRecordResp, XrpcError> {
        let sess = self.ensure_session()?;
        let url = format!(
            "{}/xrpc/com.atproto.repo.getRecord?repo={}&collection={}&rkey={}",
            sess.pds_endpoint, sess.did, collection, rkey
        );
        let v = self.get_json_metered_read(&url, Some(&sess.access_jwt))?;
        serde_json::from_value(v).map_err(|e| XrpcError::Decode(e.to_string()))
    }

    fn sync_get_record(
        &self,
        collection: &str,
        rkey: &str,
    ) -> Result<SyncRecordResp, XrpcError> {
        let sess = self.ensure_session()?;
        let url = format!(
            "{}/xrpc/com.atproto.sync.getRecord?did={}&collection={}&rkey={}",
            sess.pds_endpoint, sess.did, collection, rkey
        );
        let car = self.get_bytes_metered_read(&url, Some(&sess.access_jwt))?;
        let parsed = crate::car::parse_car(&car).map_err(XrpcError::Decode)?;
        // Find the leaf: the block whose dag-cbor decodes to a map with $type
        // == our expected type.
        let mut leaf = None;
        for b in &parsed.blocks {
            if let Ok(v) = serde_ipld_dagcbor::from_slice::<ipld_core::ipld::Ipld>(&b.data) {
                if let ipld_core::ipld::Ipld::Map(m) = &v {
                    if let Some(ipld_core::ipld::Ipld::String(t)) = m.get("$type") {
                        if t == crate::record::HIST_ENTRY_TYPE {
                            leaf = Some((b.cid, b.data.clone()));
                            break;
                        }
                    }
                }
            }
        }
        Ok(SyncRecordResp {
            car_bytes: car,
            leaf_cid: leaf.as_ref().map(|(c, _)| *c),
            leaf_bytes: leaf.map(|(_, d)| d),
        })
    }

    fn list_records(
        &self,
        collection: &str,
        limit: u32,
        cursor: Option<&str>,
        reverse: bool,
    ) -> Result<ListRecordsPage, XrpcError> {
        self.list_records_range(collection, limit, cursor, reverse, None, None)
    }

    fn list_records_range(
        &self,
        collection: &str,
        limit: u32,
        cursor: Option<&str>,
        reverse: bool,
        rkey_start: Option<&str>,
        rkey_end: Option<&str>,
    ) -> Result<ListRecordsPage, XrpcError> {
        let sess = self.ensure_session()?;
        let mut url = format!(
            "{}/xrpc/com.atproto.repo.listRecords?repo={}&collection={}&limit={}",
            sess.pds_endpoint, sess.did, collection, limit
        );
        if let Some(c) = cursor {
            url.push_str(&format!("&cursor={}", urlencoding_encode(c)));
        }
        if reverse {
            url.push_str("&reverse=true");
        }
        if let Some(s) = rkey_start {
            url.push_str(&format!("&rkeyStart={}", urlencoding_encode(s)));
        }
        if let Some(e) = rkey_end {
            url.push_str(&format!("&rkeyEnd={}", urlencoding_encode(e)));
        }
        let v = self.get_json_metered_read(&url, Some(&sess.access_jwt))?;
        serde_json::from_value(v).map_err(|e| XrpcError::Decode(e.to_string()))
    }

    fn sync_get_blocks(&self, did: &str, cids: &[String]) -> Result<Vec<u8>, XrpcError> {
        let sess = self.ensure_session()?;
        let mut url = format!(
            "{}/xrpc/com.atproto.sync.getBlocks?did={}",
            sess.pds_endpoint, did
        );
        for c in cids {
            url.push_str(&format!("&cids={}", urlencoding_encode(c)));
        }
        self.get_bytes_metered_read(&url, Some(&sess.access_jwt))
    }

    fn upload_blob(&self, mime: &str, bytes: Vec<u8>) -> Result<BlobRef, XrpcError> {
        self.budget.charge_blob(bytes.len())?;
        let sess = self.ensure_session()?;
        let url = format!("{}/xrpc/com.atproto.repo.uploadBlob", sess.pds_endpoint);
        let _g = self.pacer.acquire();
        let req = self
            .agent
            .post(&url)
            .set("Content-Type", mime)
            .set("Authorization", &format!("Bearer {}", sess.access_jwt));
        let resp = match req.send_bytes(&bytes) {
            Ok(r) => r,
            Err(ureq::Error::Status(_, r)) => return Err(self.xrpc_error_from(r)),
            Err(e) => return Err(XrpcError::Http(e.to_string())),
        };
        self.capture_rate_limit(&resp);
        let v: serde_json::Value = resp
            .into_json()
            .map_err(|e| XrpcError::Decode(e.to_string()))?;
        serde_json::from_value(v).map_err(|e| XrpcError::Decode(e.to_string()))
    }

    fn sync_get_blob(&self, did: &str, cid: &str) -> Result<Option<Vec<u8>>, XrpcError> {
        let sess = self.ensure_session()?;
        let url = format!(
            "{}/xrpc/com.atproto.sync.getBlob?did={}&cid={}",
            sess.pds_endpoint, did, cid
        );
        match self.get_bytes_metered_read(&url, Some(&sess.access_jwt)) {
            Ok(b) => Ok(Some(b)),
            Err(XrpcError::Xrpc {
                http_status,
                ref error,
                ..
            }) if http_status == 404
                || matches!(
                    error.as_deref(),
                    Some("NotFound") | Some("BlobNotFound") | Some("RecordNotFound")
                ) =>
            {
                Ok(None)
            }
            Err(e) => Err(e),
        }
    }

    fn sync_get_repo(&self, did: &str) -> Result<Vec<u8>, XrpcError> {
        self.sync_get_repo_since(did, None)
    }

    fn sync_get_repo_since(
        &self,
        did: &str,
        since: Option<&str>,
    ) -> Result<Vec<u8>, XrpcError> {
        let sess = self.ensure_session()?;
        let mut url = format!(
            "{}/xrpc/com.atproto.sync.getRepo?did={}",
            sess.pds_endpoint, did
        );
        if let Some(s) = since {
            url.push_str(&format!("&since={}", urlencoding_encode(s)));
        }
        self.get_bytes_metered_read(&url, Some(&sess.access_jwt))
    }

    fn sync_get_latest_commit(
        &self,
        did: &str,
    ) -> Result<(String, String), XrpcError> {
        let sess = self.ensure_session()?;
        let url = format!(
            "{}/xrpc/com.atproto.sync.getLatestCommit?did={}",
            sess.pds_endpoint, did
        );
        let v = self.get_json_metered_read(&url, Some(&sess.access_jwt))?;
        let cid = v
            .get("cid")
            .and_then(|x| x.as_str())
            .ok_or_else(|| XrpcError::Decode("no cid".into()))?
            .to_string();
        let rev = v
            .get("rev")
            .and_then(|x| x.as_str())
            .ok_or_else(|| XrpcError::Decode("no rev".into()))?
            .to_string();
        Ok((cid, rev))
    }

    fn resolve_did_doc(&self, did: &str) -> Result<serde_json::Value, XrpcError> {
        self.resolve_did_doc_inner(did)
    }

    fn budget_snapshot(&self) -> crate::budget::BudgetLedger {
        self.budget.snapshot()
    }

    fn last_rate_limit_headers(&self) -> RateLimitHeaders {
        self.last_rate_limit_headers()
    }
}

fn urlencoding_encode(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        if c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.' | '~') {
            out.push(c);
        } else {
            for b in c.to_string().as_bytes() {
                out.push_str(&format!("%{:02X}", b));
            }
        }
    }
    out
}
