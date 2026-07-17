//! Sealed offer-gating — the §H hybrid **serve half**. RUN-14 EXP-B.
//!
//! social-mapping §H invariant: *encryption is the confidentiality boundary in
//! every topology; an AppView gates offering (who is handed ciphertext), never
//! reading (who can decrypt).* This module is that sentence made executable: a
//! content-blind store that holds opaque ciphertext keyed by `(group_id, seq)`
//! and a roster, and a `getSealedRecords` route that offers ciphertext ONLY to a
//! verified roster member. Non-members and anonymous callers get a flat 403 with
//! no length/existence signal.
//!
//! The blindness is a **compilation fact**: the seal/open code (and the AEAD
//! crate itself) live behind `#[cfg(feature = "client-seal")]`, which the
//! `sealed` server binary never enables. [`SealedState`] carries no key material
//! and no seal capability — there is no configuration path that hands the server
//! a key. See EXP-B step 2.

use std::sync::{Arc, Mutex};

use axum::{
    extract::{Query, State},
    http::{HeaderMap, StatusCode},
    routing::get,
    Json, Router,
};
use rusqlite::Connection;
use serde::Deserialize;
use serde_json::{json, Value};

use crate::serviceauth::{verify_service_jwt, KeyResolver, VerifyError};

pub const GET_SEALED_RECORDS: &str = "app.stellin.getSealedRecords";

/// Server state. Deliberately holds NO key material and NO seal/open capability —
/// only the content-blind store, the identity resolver, the audience, the clock.
#[derive(Clone)]
pub struct SealedState {
    pub db: Arc<Mutex<Connection>>,
    pub resolver: Arc<dyn KeyResolver + Send + Sync>,
    pub aud: String,
    pub now: i64,
}

/// Create the disposable sealed store: opaque ciphertext blobs and the roster.
pub fn open_sealed_db(path: &str) -> anyhow::Result<Connection> {
    let _ = std::fs::remove_file(path);
    let conn = Connection::open(path)?;
    init_sealed_schema(&conn)?;
    Ok(conn)
}

pub fn init_sealed_schema(conn: &Connection) -> anyhow::Result<()> {
    conn.execute_batch(
        r#"
        -- Opaque ciphertext. The server stores bytes it cannot read: a nonce and
        -- a ciphertext blob, keyed by (group_id, seq). No plaintext column exists.
        CREATE TABLE sealed_records (
            group_id TEXT NOT NULL,
            seq      INTEGER NOT NULL,
            nonce    BLOB NOT NULL,
            ciphertext BLOB NOT NULL,
            PRIMARY KEY (group_id, seq)
        );
        -- The roster: who may be OFFERED ciphertext. This gates offering, not reading.
        CREATE TABLE roster (
            group_id   TEXT NOT NULL,
            member_did TEXT NOT NULL,
            PRIMARY KEY (group_id, member_did)
        );
        "#,
    )?;
    Ok(())
}

/// Store one opaque sealed record (the server never inspects the bytes).
pub fn store_sealed(conn: &Connection, group_id: &str, seq: i64, nonce: &[u8], ciphertext: &[u8]) {
    conn.execute(
        "INSERT INTO sealed_records (group_id, seq, nonce, ciphertext) VALUES (?1,?2,?3,?4)",
        rusqlite::params![group_id, seq, nonce, ciphertext],
    )
    .unwrap();
}

/// Add a member to a group's roster (offer policy).
pub fn add_to_roster(conn: &Connection, group_id: &str, member_did: &str) {
    conn.execute(
        "INSERT OR IGNORE INTO roster (group_id, member_did) VALUES (?1,?2)",
        rusqlite::params![group_id, member_did],
    )
    .unwrap();
}

/// Remove a member from a group's roster — stops FUTURE offering only.
pub fn remove_from_roster(conn: &Connection, group_id: &str, member_did: &str) {
    conn.execute(
        "DELETE FROM roster WHERE group_id = ?1 AND member_did = ?2",
        rusqlite::params![group_id, member_did],
    )
    .unwrap();
}

fn is_member(conn: &Connection, group_id: &str, member_did: &str) -> bool {
    conn.query_row(
        "SELECT 1 FROM roster WHERE group_id = ?1 AND member_did = ?2",
        rusqlite::params![group_id, member_did],
        |_| Ok(()),
    )
    .is_ok()
}

pub fn router(state: SealedState) -> Router {
    Router::new()
        .route(&format!("/xrpc/{GET_SEALED_RECORDS}"), get(get_sealed_records))
        .with_state(state)
}

#[derive(Deserialize)]
pub struct SealedParam {
    group_id: String,
    #[serde(default)]
    since: Option<i64>,
}

/// The flat refusal — identical for non-member, anonymous, and nonexistent
/// group, so the error leaks no length/existence signal beyond itself.
fn forbidden() -> (StatusCode, Json<Value>) {
    (StatusCode::FORBIDDEN, Json(json!({ "error": "NotOffered" })))
}

#[allow(dead_code)]
async fn get_sealed_records(
    State(st): State<SealedState>,
    headers: HeaderMap,
    Query(p): Query<SealedParam>,
) -> (StatusCode, Json<Value>) {
    // STEP-1 RED STUB — implemented in the green commit. (verify_bearer,
    // is_member, forbidden are exercised by the green implementation.)
    let _ = (&st, &headers, &p);
    let _ = (verify_bearer(&headers, &st), is_member, forbidden);
    (StatusCode::NOT_IMPLEMENTED, Json(json!({ "stub": true })))
}

/// Verify a bearer service-auth token bound to this route's method.
fn verify_bearer(
    headers: &HeaderMap,
    st: &SealedState,
) -> Result<Option<String>, VerifyError> {
    let Some(auth) = headers.get("authorization") else {
        return Ok(None);
    };
    let raw = auth
        .to_str()
        .map_err(|_| VerifyError::Malformed("non-ascii authorization".into()))?;
    let token = raw
        .strip_prefix("Bearer ")
        .ok_or_else(|| VerifyError::Malformed("not a Bearer token".into()))?;
    let claims = verify_service_jwt(
        token,
        &st.aud,
        Some(GET_SEALED_RECORDS),
        st.now,
        st.resolver.as_ref(),
    )?;
    Ok(Some(claims.iss))
}

// ─────────────────────────────────────────────────────────────────────────────
//  Client-side seal/open — the ONLY decryption path, behind a feature the server
//  never enables. This is the mechanical content-blind boundary.
// ─────────────────────────────────────────────────────────────────────────────
#[cfg(feature = "client-seal")]
pub mod client_seal {
    use chacha20poly1305::{
        aead::{Aead, KeyInit},
        ChaCha20Poly1305, Key, Nonce,
    };

    /// Seal plaintext under a 32-byte group key + 12-byte nonce (real AEAD).
    pub fn seal(key: &[u8; 32], nonce: &[u8; 12], plaintext: &[u8]) -> Vec<u8> {
        let cipher = ChaCha20Poly1305::new(Key::from_slice(key));
        cipher
            .encrypt(Nonce::from_slice(nonce), plaintext)
            .expect("seal")
    }

    /// Open ciphertext; `None` if the key/nonce is wrong or the tag fails.
    pub fn open(key: &[u8; 32], nonce: &[u8; 12], ciphertext: &[u8]) -> Option<Vec<u8>> {
        let cipher = ChaCha20Poly1305::new(Key::from_slice(key));
        cipher.decrypt(Nonce::from_slice(nonce), ciphertext).ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serviceauth::fixtures::{secp256k1_ident, StubResolver, TestIdent};
    use std::collections::HashMap;

    const AUD: &str = "did:web:appview.stellin.test";
    const NOW: i64 = 1_800_000_000;
    const GROUP: &str = "group:alpha";

    struct Env {
        member: TestIdent,
        outsider: TestIdent,
        st: SealedState,
    }

    fn env() -> Env {
        let member = secp256k1_ident("did:plc:member", [21u8; 32]);
        let outsider = secp256k1_ident("did:plc:outsider", [22u8; 32]);
        let mut keys = HashMap::new();
        for id in [&member, &outsider] {
            keys.insert(id.did.clone(), id.multibase.clone());
        }
        let db = Connection::open_in_memory().unwrap();
        init_sealed_schema(&db).unwrap();
        add_to_roster(&db, GROUP, "did:plc:member");
        // Two opaque records (arbitrary bytes — the store never reads them).
        store_sealed(&db, GROUP, 0, b"nonce-aaaaaa", b"CIPHERTEXT-0-opaque");
        store_sealed(&db, GROUP, 1, b"nonce-bbbbbb", b"CIPHERTEXT-1-opaque");
        let st = SealedState {
            db: Arc::new(Mutex::new(db)),
            resolver: Arc::new(StubResolver { keys }),
            aud: AUD.to_string(),
            now: NOW,
        };
        Env { member, outsider, st }
    }

    async fn serve(st: SealedState) -> String {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move { axum::serve(listener, router(st)).await.unwrap() });
        format!("http://{addr}")
    }

    async fn get(base: &str, path: &str, bearer: Option<&str>) -> (u16, Value) {
        let client = reqwest::Client::new();
        let mut req = client.get(format!("{base}{path}"));
        if let Some(b) = bearer {
            req = req.header("authorization", format!("Bearer {b}"));
        }
        let resp = req.send().await.unwrap();
        (resp.status().as_u16(), resp.json::<Value>().await.unwrap_or(json!(null)))
    }

    fn tok(id: &TestIdent) -> String {
        id.mint(AUD, NOW + 60, Some(GET_SEALED_RECORDS))
    }

    // Step 1: a roster member is offered the ciphertext.
    #[tokio::test]
    async fn member_is_offered_ciphertext() {
        let e = env();
        let member_tok = tok(&e.member);
        let base = serve(e.st).await;
        let (code, body) = get(
            &base,
            &format!("/xrpc/{GET_SEALED_RECORDS}?group_id={GROUP}&since=0"),
            Some(&member_tok),
        )
        .await;
        assert_eq!(code, 200);
        let recs = body["records"].as_array().expect("records array");
        assert_eq!(recs.len(), 2, "member sees both sealed records: {body}");
        // What comes back is ciphertext, not plaintext (base64 of the opaque blob).
        assert!(recs[0]["ciphertext"].is_string());
    }

    // Step 1: non-member, anonymous, and nonexistent-group all get the SAME 403 —
    // no length/existence leak beyond the error itself.
    #[tokio::test]
    async fn non_member_anonymous_and_missing_group_are_indistinguishable_403() {
        let e = env();
        let outsider_tok = tok(&e.outsider);
        let base = serve(e.st).await;

        let (c_out, b_out) = get(
            &base,
            &format!("/xrpc/{GET_SEALED_RECORDS}?group_id={GROUP}&since=0"),
            Some(&outsider_tok),
        )
        .await;
        let (c_anon, b_anon) =
            get(&base, &format!("/xrpc/{GET_SEALED_RECORDS}?group_id={GROUP}&since=0"), None).await;
        let (c_missing, b_missing) = get(
            &base,
            &format!("/xrpc/{GET_SEALED_RECORDS}?group_id=group:does-not-exist&since=0"),
            Some(&outsider_tok),
        )
        .await;

        assert_eq!((c_out, c_anon, c_missing), (403, 403, 403));
        // Identical bodies: an outsider cannot tell a group they're barred from
        // apart from one that does not exist.
        assert_eq!(b_out, b_anon);
        assert_eq!(b_out, b_missing);
    }

    // Step 2: the served bytes decrypt ONLY in the client harness that holds the
    // group key — the server never had it. (Runs under --features client-seal.)
    #[cfg(feature = "client-seal")]
    #[tokio::test]
    async fn served_ciphertext_decrypts_only_client_side() {
        use super::client_seal::{open, seal};

        // The client seals under a group key the server never sees.
        let key = [7u8; 32];
        let nonce = [3u8; 12];
        let plaintext = b"open to work: staff platform eng";
        let ct = seal(&key, &nonce, plaintext);

        // Stand up a store holding ONLY that ciphertext, roster = [member].
        let member = secp256k1_ident("did:plc:member", [21u8; 32]);
        let mut keys = HashMap::new();
        keys.insert(member.did.clone(), member.multibase.clone());
        let db = Connection::open_in_memory().unwrap();
        init_sealed_schema(&db).unwrap();
        add_to_roster(&db, GROUP, "did:plc:member");
        store_sealed(&db, GROUP, 0, &nonce, &ct);
        let st = SealedState {
            db: Arc::new(Mutex::new(db)),
            resolver: Arc::new(StubResolver { keys }),
            aud: AUD.to_string(),
            now: NOW,
        };
        let member_tok = tok(&member);
        let base = serve(st).await;

        let (code, body) = get(
            &base,
            &format!("/xrpc/{GET_SEALED_RECORDS}?group_id={GROUP}&since=0"),
            Some(&member_tok),
        )
        .await;
        assert_eq!(code, 200);
        let rec = &body["records"][0];
        let got_ct = base64_decode(rec["ciphertext"].as_str().unwrap());
        let got_nonce = base64_decode(rec["nonce"].as_str().unwrap());
        let nonce12: [u8; 12] = got_nonce.try_into().unwrap();

        // The server's bytes are opaque; only the key-holder opens them.
        let opened = open(&key, &nonce12, &got_ct).expect("client with the key opens it");
        assert_eq!(opened, plaintext);
        // A different key does NOT open it — the store's bytes really are sealed.
        assert!(open(&[0u8; 32], &nonce12, &got_ct).is_none());
    }

    // Step 3: roster gates OFFERING; encryption alone gates READING. Remove the
    // member → future offers 403; but ciphertext already fetched + a retained key
    // still decrypts. Revocation is forward-only on offering.
    #[cfg(feature = "client-seal")]
    #[tokio::test]
    async fn roster_gates_offering_not_reading() {
        use super::client_seal::{open, seal};
        let key = [9u8; 32];
        let nonce = [1u8; 12];
        let ct = seal(&key, &nonce, b"payload");

        let member = secp256k1_ident("did:plc:member", [21u8; 32]);
        let mut keys = HashMap::new();
        keys.insert(member.did.clone(), member.multibase.clone());
        let db = Connection::open_in_memory().unwrap();
        init_sealed_schema(&db).unwrap();
        add_to_roster(&db, GROUP, "did:plc:member");
        store_sealed(&db, GROUP, 0, &nonce, &ct);
        let st = SealedState {
            db: Arc::new(Mutex::new(db)),
            resolver: Arc::new(StubResolver { keys }),
            aud: AUD.to_string(),
            now: NOW,
        };
        let db_handle = st.db.clone();
        let member_tok = tok(&member);
        let base = serve(st).await;
        let url = format!("/xrpc/{GET_SEALED_RECORDS}?group_id={GROUP}&since=0");

        // Member fetches while on the roster.
        let (code, body) = get(&base, &url, Some(&member_tok)).await;
        assert_eq!(code, 200);
        let fetched_ct = base64_decode(body["records"][0]["ciphertext"].as_str().unwrap());

        // Remove from roster → future OFFERING stops.
        remove_from_roster(&db_handle.lock().unwrap(), GROUP, "did:plc:member");
        let (code2, _b2) = get(&base, &url, Some(&member_tok)).await;
        assert_eq!(code2, 403, "removal stops future offering");

        // But the ALREADY-FETCHED ciphertext + retained key still READS. Offering
        // was gated; reading was never the AppView's to gate (§H).
        let nonce12: [u8; 12] = nonce;
        assert_eq!(open(&key, &nonce12, &fetched_ct).unwrap(), b"payload");
    }

    #[cfg(feature = "client-seal")]
    fn base64_decode(s: &str) -> Vec<u8> {
        use base64::{engine::general_purpose::STANDARD, Engine};
        STANDARD.decode(s).unwrap()
    }
}
