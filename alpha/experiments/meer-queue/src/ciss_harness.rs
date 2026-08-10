//! The spike's storage boundary: the **real** CISS server, driven over real loopback HTTP.
//!
//! SPEC-DELTA[meer-spike-ciss-inproc | test-hermeticization]: CISS ships no server binary
//! (`ciss-cli` declares the only `[[bin]]`, and it is a *client*), so the spike spawns CISS's
//! real axum router in-process on an ephemeral port — which is how CISS's own suite drives
//! itself. Real handler chain, real HTTP, real content-address round trip, real
//! re-verify-on-read, real 2 MiB cap. **Not** a deployed instance (no systemd, TLS, or
//! operator). — Register: `alpha/experiments/SPEC-DIVERGENCE-REGISTER.md`
//!
//! Two deliberate construction choices, both from Phase 0:
//!
//! - **`App::with_limits`, never `App::new`.** `App::new` calls `Limits::from_env()` and reads
//!   `CISS_MAX_STORE_BYTES` / `CISS_MAX_DID_BYTES` from the ambient environment, which would
//!   let a developer's shell perturb S2's fan-out or S8's sizes. `Limits` has no `Default`,
//!   but its fields are `pub`, so the ceiling is stated here rather than inherited.
//! - **`Blobs::Fs`, not `Blobs::Memory`.** The filesystem backend lays objects out as
//!   `blocks/{did}/{cid}`, so "stored once" is countable on disk — an accounting source
//!   independent of both our bookkeeping and CISS's own `du`.

use std::path::PathBuf;

use ciss::server::{App, Blobs, Db, Limits};
use tokio::sync::oneshot;

/// CISS's per-object ceiling (`CISS/src/blobstore.rs`), restated so tests can sit on it.
///
/// Load-bearing: it came from a real memory-exhaustion finding in the 2026-08-03 security
/// review (a 512 MiB upload buffered in RAM against a 384 MiB unit). It does not get raised
/// to make a test pass.
pub const MAX_OBJECT_BYTES: usize = 2 * 1024 * 1024;

/// The whole-store ceiling this spike pins. Ample for the spike; stated, not inherited.
const SPIKE_STORE_CEILING: u64 = 1024 * 1024 * 1024;

/// A caller identity: the keypair whose derived `id:` DID it acts as.
///
/// CISS's `id:` plane authenticates a signed session — `x-croft-pubkey` plus
/// `x-croft-session`, the latter signing the challenge `ciss-session/v1/{did}`.
pub struct Identity {
    keypair: ciss::crypto::Keypair,
    did: String,
}

impl Identity {
    /// This identity's DID — also its CISS namespace.
    #[must_use]
    pub fn did(&self) -> &str {
        &self.did
    }

    /// The `(pubkey, session-signature)` header pair proving control of the key.
    fn session_headers(&self) -> (String, String) {
        let challenge = format!("ciss-session/v1/{}", self.did);
        (
            self.keypair.public_key_hex(),
            self.keypair.sign_message(&challenge),
        )
    }
}

/// One HTTP exchange with the server: status plus raw body.
pub struct Outcome {
    /// HTTP status code.
    pub status: u16,
    /// Raw response body.
    pub body: Vec<u8>,
}

impl Outcome {
    /// The body as text, for assertion messages.
    #[must_use]
    pub fn body_text(&self) -> String {
        String::from_utf8_lossy(&self.body).into_owned()
    }

    /// The content address from a `PUT` response, if the body carries one.
    ///
    /// CISS answers a successful object `PUT` with
    /// `{"bytes":N,"cid":"<sha256 hex>","receipt_mode":"..."}`.
    #[must_use]
    pub fn cid(&self) -> Option<String> {
        let text = self.body_text();
        let rest = text.split("\"cid\":\"").nth(1)?;
        let cid = rest.split('"').next()?;
        (cid.len() == 64 && cid.chars().all(|c| c.is_ascii_hexdigit())).then(|| cid.to_owned())
    }
}

/// A running CISS server on an ephemeral loopback port, with its blob root on disk.
pub struct CissHarness {
    base: String,
    client: reqwest::Client,
    blob_root: PathBuf,
    shutdown: Option<oneshot::Sender<()>>,
    handle: Option<tokio::task::JoinHandle<()>>,
    /// Held so the temp dir outlives the server; dropped last.
    _dir: tempfile::TempDir,
}

impl CissHarness {
    /// Spawn the real CISS router on `127.0.0.1:0` and return a handle to it.
    ///
    /// # Panics
    /// If the server cannot be built or the port cannot be bound — both are environment
    /// faults a spike should fail loudly on, not paper over.
    pub async fn spawn() -> Self {
        let dir = tempfile::tempdir().expect("tempdir");
        let blob_root = dir.path().to_path_buf();

        let app = App::with_limits(
            "meer-spike-provider",
            Blobs::Fs(blob_root.clone()),
            Db::Memory,
            Limits {
                store_ceiling: SPIKE_STORE_CEILING,
                did_cap: None,
            },
        )
        .expect("build CISS app");

        let router = app.router();
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("bind ephemeral loopback port");
        let addr = listener.local_addr().expect("local_addr");

        let (tx, rx) = oneshot::channel::<()>();
        let handle = tokio::spawn(async move {
            axum::serve(listener, router)
                .with_graceful_shutdown(async move {
                    let _ = rx.await;
                })
                .await
                .expect("serve");
        });

        Self {
            base: format!("http://{addr}"),
            client: reqwest::Client::new(),
            blob_root,
            shutdown: Some(tx),
            handle: Some(handle),
            _dir: dir,
        }
    }

    /// A caller identity named `name`, deterministic per name.
    #[must_use]
    pub fn identity(&self, name: &str) -> Identity {
        let keypair = ciss::crypto::derive_keypair("meer-spike", name);
        let did = ciss::identity::derive_id(&keypair.verifying_key());
        Identity { keypair, did }
    }

    /// `PUT /{did}/objects/{key}` — store an object in `who`'s namespace.
    pub async fn put_object(&self, who: &Identity, key: &str, bytes: &[u8]) -> Outcome {
        let (pubkey, session) = who.session_headers();
        let url = format!("{}/{}/objects/{key}", self.base, who.did());
        self.run(
            self.client
                .put(url)
                .header("x-croft-pubkey", pubkey)
                .header("x-croft-session", session)
                .body(bytes.to_vec()),
        )
        .await
    }

    /// `GET /{did}/objects/{cid}` — fetch an object by content address. CISS re-verifies the
    /// bytes against the address before serving them.
    pub async fn get_object(&self, who: &Identity, cid: &str) -> Outcome {
        let (pubkey, session) = who.session_headers();
        let url = format!("{}/{}/objects/{cid}", self.base, who.did());
        self.run(
            self.client
                .get(url)
                .header("x-croft-pubkey", pubkey)
                .header("x-croft-session", session),
        )
        .await
    }

    /// `GET /{did}/du` — CISS's own accounting for a namespace: a per-object list of
    /// `{cid, bytes}` plus `total_bytes`. S2 cross-checks this against [`Self::blob_files`].
    pub async fn du(&self, who: &Identity) -> Outcome {
        let (pubkey, session) = who.session_headers();
        let url = format!("{}/{}/du", self.base, who.did());
        self.run(
            self.client
                .get(url)
                .header("x-croft-pubkey", pubkey)
                .header("x-croft-session", session),
        )
        .await
    }

    /// Every blob file the server has actually written, relative to the blob root
    /// (`blocks/{did}/{cid}`). The accounting source that owes nothing to our own
    /// bookkeeping.
    #[must_use]
    pub fn blob_files(&self) -> Vec<String> {
        let mut out = Vec::new();
        let mut stack = vec![self.blob_root.clone()];
        while let Some(dir) = stack.pop() {
            let Ok(entries) = std::fs::read_dir(&dir) else {
                continue;
            };
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    stack.push(path);
                } else if let Ok(rel) = path.strip_prefix(&self.blob_root) {
                    out.push(rel.display().to_string());
                }
            }
        }
        out.sort();
        out
    }

    /// Shut the server down gracefully and wait for the task to finish, so a caller can
    /// observe that the port was released rather than leaked.
    pub async fn shutdown(mut self) {
        if let Some(tx) = self.shutdown.take() {
            let _ = tx.send(());
        }
        if let Some(handle) = self.handle.take() {
            let _ = handle.await;
        }
    }

    async fn run(&self, builder: reqwest::RequestBuilder) -> Outcome {
        let resp = builder.send().await.expect("request send");
        let status = resp.status().as_u16();
        let body = resp.bytes().await.expect("response body").to_vec();
        Outcome { status, body }
    }
}
