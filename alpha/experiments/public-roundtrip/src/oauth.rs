//! Extension #1: drive the *real* atproto OAuth flow as far as a headless
//! machine can — discovery, PKCE, a DPoP keypair, and a pushed authorization
//! request (PAR) — then emit the authorization URL and listen on a loopback
//! callback. Everything here runs server-side with no human; the only step that
//! cannot be automated in this environment is the browser consent screen.
//!
//! Measuring where the wall actually is (PAR succeeds → only consent remains)
//! is the point: it turns "OAuth is hard locally" into a concrete finding.

use anyhow::{bail, Context, Result};
use base64::Engine;
use p256::ecdsa::{signature::Signer, Signature, SigningKey};
use serde_json::{json, Value};
use std::time::Duration;

use crate::xrpc::XrpcClient;

const B64: base64::engine::general_purpose::GeneralPurpose = base64::engine::general_purpose::URL_SAFE_NO_PAD;

/// A DPoP proof signer: an ephemeral P-256 key plus its public JWK.
struct DpopKey {
    signing: SigningKey,
    jwk: Value,
}

impl DpopKey {
    fn new() -> Self {
        let signing = SigningKey::random(&mut rand::rngs::OsRng);
        let vk = signing.verifying_key();
        let pt = vk.to_encoded_point(false);
        let x = B64.encode(pt.x().expect("x coord"));
        let y = B64.encode(pt.y().expect("y coord"));
        let jwk = json!({ "kty": "EC", "crv": "P-256", "x": x, "y": y });
        DpopKey { signing, jwk }
    }

    /// Build a DPoP proof JWT for `htm htu`, optionally bound to a server nonce.
    fn proof(&self, htm: &str, htu: &str, nonce: Option<&str>) -> String {
        let header = json!({ "typ": "dpop+jwt", "alg": "ES256", "jwk": self.jwk });
        let iat = now_secs();
        let mut claims = json!({
            "jti": rand_token(16),
            "htm": htm,
            "htu": htu,
            "iat": iat,
        });
        if let Some(n) = nonce {
            claims["nonce"] = json!(n);
        }
        let signing_input = format!(
            "{}.{}",
            B64.encode(serde_json::to_vec(&header).unwrap()),
            B64.encode(serde_json::to_vec(&claims).unwrap())
        );
        let sig: Signature = self.signing.sign(signing_input.as_bytes());
        format!("{signing_input}.{}", B64.encode(sig.to_bytes()))
    }
}

fn now_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn rand_token(bytes: usize) -> String {
    use rand::RngCore;
    let mut buf = vec![0u8; bytes];
    rand::rngs::OsRng.fill_bytes(&mut buf);
    B64.encode(&buf)
}

/// PKCE verifier + S256 challenge.
fn pkce() -> (String, String) {
    use sha2::{Digest, Sha256};
    let verifier = rand_token(32);
    let challenge = B64.encode(Sha256::digest(verifier.as_bytes()));
    (verifier, challenge)
}

/// Run the OAuth flow up to (and including) the authorization URL. `port` is the
/// loopback callback port. Returns nothing; prints a friction report.
pub async fn run(handle: &str, port: u16) -> Result<()> {
    let http = reqwest::Client::builder()
        .user_agent("public-roundtrip-experiment/0.1")
        .timeout(Duration::from_secs(15))
        .build()?;
    let xrpc = XrpcClient::new()?;
    let mut steps = 0u32;

    // 1. handle -> DID -> PDS
    let did = xrpc.resolve_handle(handle).await.context("resolveHandle")?;
    steps += 1;
    let pds = xrpc.resolve_pds_via_plc(&did).await.context("PDS via PLC")?;
    steps += 1;
    println!("✓ identity: @{handle} -> {did} -> {pds}");

    // 2. PDS protected-resource metadata -> authorization server
    let prm: Value = http
        .get(format!("{pds}/.well-known/oauth-protected-resource"))
        .send()
        .await
        .context("protected-resource metadata")?
        .error_for_status()?
        .json()
        .await?;
    steps += 1;
    let auth_server = prm
        .get("authorization_servers")
        .and_then(|a| a.as_array())
        .and_then(|a| a.first())
        .and_then(|v| v.as_str())
        .context("no authorization_servers in protected-resource metadata")?
        .trim_end_matches('/')
        .to_string();
    println!("✓ authorization server: {auth_server}");

    // 3. authorization server metadata
    let asm: Value = http
        .get(format!("{auth_server}/.well-known/oauth-authorization-server"))
        .send()
        .await
        .context("auth-server metadata")?
        .error_for_status()?
        .json()
        .await?;
    steps += 1;
    let par_endpoint = asm
        .get("pushed_authorization_request_endpoint")
        .and_then(|v| v.as_str())
        .context("auth server does not advertise PAR endpoint")?
        .to_string();
    let authorization_endpoint = asm
        .get("authorization_endpoint")
        .and_then(|v| v.as_str())
        .context("no authorization_endpoint")?
        .to_string();
    println!("✓ PAR endpoint: {par_endpoint}");

    // 4. PKCE + 5. DPoP key + loopback client identifiers.
    // `_verifier` is held only conceptually here: it is sent at the token
    // exchange, which is past the consent wall this headless run cannot cross.
    let (_verifier, challenge) = pkce();
    let dpop = DpopKey::new();
    let state = rand_token(16);
    let redirect_uri = format!("http://127.0.0.1:{port}/callback");
    let scope = "atproto";
    // atproto loopback/development client: client_id is http://localhost with
    // redirect_uri + scope encoded as query params.
    let client_id = format!(
        "http://localhost?redirect_uri={}&scope={}",
        urlencode(&redirect_uri),
        urlencode(scope)
    );

    // 6. PAR (with the DPoP nonce handshake: first attempt may 400 asking for a nonce)
    let form = [
        ("client_id", client_id.as_str()),
        ("redirect_uri", redirect_uri.as_str()),
        ("response_type", "code"),
        ("scope", scope),
        ("state", state.as_str()),
        ("code_challenge", challenge.as_str()),
        ("code_challenge_method", "S256"),
        ("login_hint", handle),
    ];
    let mut nonce: Option<String> = None;
    let mut request_uri = String::new();
    for attempt in 1..=2 {
        let proof = dpop.proof("POST", &par_endpoint, nonce.as_deref());
        let resp = http
            .post(&par_endpoint)
            .header("DPoP", proof)
            .form(&form)
            .send()
            .await
            .context("PAR request")?;
        steps += 1;
        let server_nonce = resp
            .headers()
            .get("DPoP-Nonce")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());
        let status = resp.status();
        let body: Value = resp.json().await.unwrap_or(Value::Null);
        if status.is_success() {
            request_uri = body
                .get("request_uri")
                .and_then(|v| v.as_str())
                .context("PAR succeeded but returned no request_uri")?
                .to_string();
            println!("✓ PAR accepted (attempt {attempt}): request_uri={request_uri}");
            break;
        }
        // Expected once: server demands a DPoP nonce. Retry with it.
        let err = body.get("error").and_then(|v| v.as_str()).unwrap_or("");
        if err == "use_dpop_nonce" && server_nonce.is_some() && attempt == 1 {
            println!("• PAR asked for a DPoP nonce (normal); retrying with it");
            nonce = server_nonce;
            continue;
        }
        bail!("PAR failed: HTTP {status} — {body}");
    }
    if request_uri.is_empty() {
        bail!("PAR did not yield a request_uri");
    }

    // 7. Authorization URL — the browser/consent wall.
    let auth_url = format!(
        "{authorization_endpoint}?client_id={}&request_uri={}",
        urlencode(&client_id),
        urlencode(&request_uri)
    );

    println!("\n=== OAuth friction report ===");
    println!("Back-channel steps completed with NO human: {steps}");
    println!("  (handle->DID, DID->PDS, protected-resource meta, auth-server meta, PAR[+nonce])");
    println!("DPoP: ephemeral P-256 key, ES256 proof accepted by the auth server ✓");
    println!("PKCE: S256 challenge sent ✓");
    println!("\nTHE WALL — open this URL in a browser, log in, and approve:");
    println!("{auth_url}");
    println!("\nThe loopback listener below would capture the redirect with ?code=...,");
    println!("after which a DPoP-bound token exchange yields the session.");

    // 8. Loopback listener (bounded) — proves the callback path is wired, and in a
    // real (human-present) run would receive the authorization code.
    listen_for_callback(port, Duration::from_secs(8)).await?;
    Ok(())
}

/// Minimal loopback HTTP listener: waits up to `wait` for a single GET to
/// /callback and reports the captured `code`/`error`, then returns.
async fn listen_for_callback(port: u16, wait: Duration) -> Result<()> {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    let listener = tokio::net::TcpListener::bind(("127.0.0.1", port))
        .await
        .with_context(|| format!("binding loopback callback on 127.0.0.1:{port}"))?;
    println!("\nLoopback callback listening on 127.0.0.1:{port} (waiting {:?})...", wait);
    match tokio::time::timeout(wait, listener.accept()).await {
        Ok(Ok((mut socket, _))) => {
            let mut buf = [0u8; 2048];
            let n = socket.read(&mut buf).await.unwrap_or(0);
            let req = String::from_utf8_lossy(&buf[..n]);
            let first = req.lines().next().unwrap_or("");
            println!("• callback received: {first}");
            let _ = socket
                .write_all(b"HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nExperiment callback received. You can close this tab.")
                .await;
        }
        Ok(Err(e)) => println!("• callback accept error: {e}"),
        Err(_) => println!("• no callback within the window (expected in a headless run — no browser to redirect)"),
    }
    Ok(())
}

fn urlencode(s: &str) -> String {
    // Conservative percent-encoding for query-component values.
    let mut out = String::with_capacity(s.len());
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => out.push(b as char),
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}
