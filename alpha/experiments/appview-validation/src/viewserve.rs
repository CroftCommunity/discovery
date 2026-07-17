//! Viewer-aware serving: an XRPC route whose response *shape* depends on who is
//! verified as asking. RUN-14 EXP-A steps 2–3.
//!
//! `app.stellin.getProfileView` is an **experiment-local** method name — NOT a
//! published lexicon. We deliberately publish no `app.stellin.*` schema record in
//! this run (the namespace decision is pending domain purchase); the name is a
//! route string only.
//!
//! The gate: the field `openToWork` is served ONLY when the verified viewer DID
//! is in the `recruiters` table AND the viewer's `affiliation` differs from the
//! subject's `employer`. Everyone else — non-recruiters, recruiters at the
//! subject's own employer, anonymous callers — gets the public view with no such
//! field. A present-but-invalid token is a 401, never a silently degraded view.
//!
//! SPEC-DELTA[run14-A2 | declared-stand-in]: the `recruiters` table stands in for
//! recruiter-admission governance (R7 territory, out of scope this run) — Register:
//! alpha/experiments/SPEC-DIVERGENCE-REGISTER.md

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

use crate::serviceauth::{verify_service_jwt, KeyResolver, ServiceAuthClaims, VerifyError};

/// The route method names (experiment-local, unpublished).
pub const GET_PROFILE_VIEW: &str = "app.stellin.getProfileView";
pub const GET_PROFILE_VIEWS: &str = "app.stellin.getProfileViews";

/// Server state: the index, the identity resolver, this service's audience DID,
/// and a fixed clock (the live bin passes real unix-time).
#[derive(Clone)]
pub struct ProfileState {
    pub db: Arc<Mutex<Connection>>,
    pub resolver: Arc<dyn KeyResolver + Send + Sync>,
    pub aud: String,
    pub now: i64,
}

/// Create the disposable profile index: subject profiles, the recruiter roster
/// (the declared stand-in), and the telemetry log (added in step 3).
pub fn open_profile_db(path: &str) -> anyhow::Result<Connection> {
    let _ = std::fs::remove_file(path); // disposable projection
    let conn = Connection::open(path)?;
    init_profile_schema(&conn)?;
    Ok(conn)
}

/// Create the profile/roster/telemetry tables on an already-open connection
/// (used for in-memory test databases).
pub fn init_profile_schema(conn: &Connection) -> anyhow::Result<()> {
    conn.execute_batch(
        r#"
        CREATE TABLE profiles (
            did          TEXT PRIMARY KEY,
            display_name TEXT,
            employer     TEXT,
            open_to_work INTEGER NOT NULL DEFAULT 0  -- server-side state, NOT a stored record field
        );
        -- Declared stand-in for recruiter-admission governance (SPEC-DELTA run14-A2).
        CREATE TABLE recruiters (
            did         TEXT PRIMARY KEY,
            affiliation TEXT
        );
        -- Telemetry: observed state, part of the disposable index (step 3).
        CREATE TABLE profile_views (
            viewer_did  TEXT NOT NULL,
            subject_did TEXT NOT NULL,
            ts          INTEGER NOT NULL
        );
        "#,
    )?;
    Ok(())
}

/// Seed a subject profile (test/live setup helper).
pub fn seed_profile(conn: &Connection, did: &str, display: &str, employer: &str, open: bool) {
    conn.execute(
        "INSERT INTO profiles (did, display_name, employer, open_to_work) VALUES (?1,?2,?3,?4)",
        rusqlite::params![did, display, employer, open as i64],
    )
    .unwrap();
}

/// Seed a recruiter into the roster stand-in.
pub fn seed_recruiter(conn: &Connection, did: &str, affiliation: &str) {
    conn.execute(
        "INSERT INTO recruiters (did, affiliation) VALUES (?1,?2)",
        rusqlite::params![did, affiliation],
    )
    .unwrap();
}

pub fn router(state: ProfileState) -> Router {
    Router::new()
        .route(&format!("/xrpc/{GET_PROFILE_VIEW}"), get(get_profile_view))
        .route(&format!("/xrpc/{GET_PROFILE_VIEWS}"), get(get_profile_views))
        // A generic record-echo route — the FALSIFY guard: it must NOT become a
        // side door onto the gated field.
        .route("/xrpc/com.atproto.repo.getRecord", get(get_record))
        .with_state(state)
}

#[derive(Deserialize)]
pub struct ActorParam {
    actor: String,
}

/// Pull and verify the bearer token, if any.
///   - No Authorization header        → Ok(None)  (anonymous)
///   - Present but invalid            → Err(VerifyError)  (→ 401)
///   - Present and valid              → Ok(Some(claims))
fn verify_bearer(
    headers: &HeaderMap,
    st: &ProfileState,
    lxm: &str,
) -> Result<Option<ServiceAuthClaims>, VerifyError> {
    let Some(auth) = headers.get("authorization") else {
        return Ok(None);
    };
    let raw = auth
        .to_str()
        .map_err(|_| VerifyError::Malformed("non-ascii authorization header".into()))?;
    let token = raw
        .strip_prefix("Bearer ")
        .ok_or_else(|| VerifyError::Malformed("authorization is not a Bearer token".into()))?;
    verify_service_jwt(token, &st.aud, Some(lxm), st.now, st.resolver.as_ref()).map(Some)
}

/// A verified viewer's role against a subject, derived from the roster stand-in.
struct ViewerRole {
    did: String,
    is_recruiter: bool,
    affiliation: Option<String>,
}

fn viewer_role(conn: &Connection, viewer_did: &str) -> ViewerRole {
    let affiliation: Option<String> = conn
        .query_row(
            "SELECT affiliation FROM recruiters WHERE did = ?1",
            [viewer_did],
            |r| r.get(0),
        )
        .ok();
    ViewerRole {
        did: viewer_did.to_string(),
        is_recruiter: affiliation.is_some(),
        affiliation,
    }
}

/// The gate: openToWork is offered only to a recruiter at a *different* employer.
fn may_see_open_to_work(viewer: &ViewerRole, subject_employer: &Option<String>) -> bool {
    viewer.is_recruiter && viewer.affiliation.as_ref() != subject_employer.as_ref()
}

async fn get_profile_view(
    State(st): State<ProfileState>,
    headers: HeaderMap,
    Query(p): Query<ActorParam>,
) -> (StatusCode, Json<Value>) {
    // A present-but-invalid token is refused; absent header = anonymous.
    let claims = match verify_bearer(&headers, &st, GET_PROFILE_VIEW) {
        Ok(c) => c,
        Err(_) => return (StatusCode::UNAUTHORIZED, Json(json!({ "error": "AuthRequired" }))),
    };

    let conn = st.db.lock().unwrap();
    let row = conn
        .query_row(
            "SELECT display_name, employer, open_to_work FROM profiles WHERE did = ?1",
            [&p.actor],
            |r| {
                Ok((
                    r.get::<_, Option<String>>(0)?,
                    r.get::<_, Option<String>>(1)?,
                    r.get::<_, i64>(2)? != 0,
                ))
            },
        )
        .ok();
    let (display, employer, open_to_work) = match row {
        Some(x) => x,
        None => return (StatusCode::NOT_FOUND, Json(json!({ "error": "ProfileNotFound" }))),
    };

    // Public view — the fields everyone gets.
    let mut view = json!({
        "did": p.actor,
        "displayName": display,
        "employer": employer,
    });

    // The viewer-gated field. Only a verified recruiter at a different employer.
    if let Some(c) = &claims {
        let role = viewer_role(&conn, &c.iss);
        if may_see_open_to_work(&role, &employer) {
            view["openToWork"] = json!(open_to_work);
        }
        // STEP-3 telemetry write lands here in the next green commit.
        let _ = &role;
    }

    (StatusCode::OK, Json(view))
}

async fn get_profile_views(
    State(st): State<ProfileState>,
    headers: HeaderMap,
) -> (StatusCode, Json<Value>) {
    // STEP-3 RED STUB — implemented in the step-3 green commit.
    let _ = (&st, &headers);
    (StatusCode::NOT_IMPLEMENTED, Json(json!({ "stub": true })))
}

async fn get_record(
    State(st): State<ProfileState>,
    Query(p): Query<GetRecordParam>,
) -> (StatusCode, Json<Value>) {
    // The generic record echo. It serves ONLY the stored public record fields —
    // openToWork is a viewer-dependent computed projection and is stored in a
    // separate column that this route never reads, so it cannot leak here.
    let conn = st.db.lock().unwrap();
    let row = conn
        .query_row(
            "SELECT display_name, employer FROM profiles WHERE did = ?1",
            [&p.rkey],
            |r| Ok((r.get::<_, Option<String>>(0)?, r.get::<_, Option<String>>(1)?)),
        )
        .ok();
    match row {
        Some((display, employer)) => (
            StatusCode::OK,
            Json(json!({
                "uri": format!("at://{}/app.bsky.actor.profile/self", p.rkey),
                "value": { "displayName": display, "employer": employer }
            })),
        ),
        None => (StatusCode::NOT_FOUND, Json(json!({ "error": "RecordNotFound" }))),
    }
}

#[derive(Deserialize)]
pub struct GetRecordParam {
    #[allow(dead_code)]
    collection: String,
    rkey: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serviceauth::fixtures::{secp256k1_ident, StubResolver, TestIdent};
    use std::collections::HashMap;

    const AUD: &str = "did:web:appview.stellin.test";
    const NOW: i64 = 1_800_000_000;

    // A subject and three viewers with distinct roles.
    struct World {
        subject: TestIdent,
        recruiter_other: TestIdent, // recruiter, different employer → sees the field
        recruiter_same: TestIdent,  // recruiter, subject's employer → does not
        plain: TestIdent,           // verified but not a recruiter
    }

    fn world() -> (World, ProfileState) {
        let subject = secp256k1_ident("did:plc:subject", [11u8; 32]);
        let recruiter_other = secp256k1_ident("did:plc:recruiter-other", [12u8; 32]);
        let recruiter_same = secp256k1_ident("did:plc:recruiter-same", [13u8; 32]);
        let plain = secp256k1_ident("did:plc:plain", [14u8; 32]);

        let mut keys = HashMap::new();
        for id in [&subject, &recruiter_other, &recruiter_same, &plain] {
            keys.insert(id.did.clone(), id.multibase.clone());
        }

        // In-memory, per-test, so parallel tests never share a disposable file.
        let db = Connection::open_in_memory().unwrap();
        init_profile_schema(&db).unwrap();
        seed_profile(&db, "did:plc:subject", "Subject", "Acme", true);
        seed_recruiter(&db, "did:plc:recruiter-other", "Globex");
        seed_recruiter(&db, "did:plc:recruiter-same", "Acme");

        let st = ProfileState {
            db: Arc::new(Mutex::new(db)),
            resolver: Arc::new(StubResolver { keys }),
            aud: AUD.to_string(),
            now: NOW,
        };
        (
            World { subject, recruiter_other, recruiter_same, plain },
            st,
        )
    }

    async fn serve(st: ProfileState) -> String {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move {
            axum::serve(listener, router(st)).await.unwrap();
        });
        format!("http://{addr}")
    }

    async fn get(base: &str, path: &str, bearer: Option<&str>) -> (u16, Value) {
        let client = reqwest::Client::new();
        let mut req = client.get(format!("{base}{path}"));
        if let Some(b) = bearer {
            req = req.header("authorization", format!("Bearer {b}"));
        }
        let resp = req.send().await.unwrap();
        let code = resp.status().as_u16();
        let body = resp.json::<Value>().await.unwrap_or(json!(null));
        (code, body)
    }

    fn tok(id: &TestIdent) -> String {
        id.mint(AUD, NOW + 60, Some(GET_PROFILE_VIEW))
    }

    // Anonymous: public view, no openToWork, 200.
    #[tokio::test]
    async fn anonymous_gets_public_view_no_field() {
        let (_w, st) = world();
        let base = serve(st).await;
        let (code, body) = get(&base, "/xrpc/app.stellin.getProfileView?actor=did:plc:subject", None).await;
        assert_eq!(code, 200);
        assert_eq!(body["did"], "did:plc:subject");
        assert!(body.get("openToWork").is_none(), "anonymous must not see openToWork");
    }

    // Verified non-recruiter: no field.
    #[tokio::test]
    async fn non_recruiter_sees_no_field() {
        let (w, st) = world();
        let base = serve(st).await;
        let (code, body) = get(
            &base,
            "/xrpc/app.stellin.getProfileView?actor=did:plc:subject",
            Some(&tok(&w.plain)),
        )
        .await;
        assert_eq!(code, 200);
        assert!(body.get("openToWork").is_none(), "non-recruiter must not see openToWork");
    }

    // Recruiter at a DIFFERENT employer: sees the field.
    #[tokio::test]
    async fn recruiter_at_other_employer_sees_field() {
        let (w, st) = world();
        let base = serve(st).await;
        let (code, body) = get(
            &base,
            "/xrpc/app.stellin.getProfileView?actor=did:plc:subject",
            Some(&tok(&w.recruiter_other)),
        )
        .await;
        assert_eq!(code, 200);
        assert_eq!(body["openToWork"], true, "recruiter at a different employer must see openToWork");
    }

    // Recruiter at the SUBJECT'S employer: no field.
    #[tokio::test]
    async fn recruiter_at_same_employer_sees_no_field() {
        let (w, st) = world();
        let base = serve(st).await;
        let (code, body) = get(
            &base,
            "/xrpc/app.stellin.getProfileView?actor=did:plc:subject",
            Some(&tok(&w.recruiter_same)),
        )
        .await;
        assert_eq!(code, 200);
        assert!(
            body.get("openToWork").is_none(),
            "recruiter at the subject's own employer must not see openToWork"
        );
    }

    // Malformed/invalid token: 401, not a degraded 200.
    #[tokio::test]
    async fn malformed_token_is_401_not_degraded_view() {
        let (_w, st) = world();
        let base = serve(st).await;
        let (code, _body) = get(
            &base,
            "/xrpc/app.stellin.getProfileView?actor=did:plc:subject",
            Some("not-a-valid-jwt"),
        )
        .await;
        assert_eq!(code, 401, "a present-but-invalid token must be refused, not degraded");
    }

    // A token minted for the WRONG lxm is also 401 (route binds the method).
    #[tokio::test]
    async fn wrong_lxm_token_is_401() {
        let (w, st) = world();
        let base = serve(st).await;
        let bad = w.recruiter_other.mint(AUD, NOW + 60, Some("app.stellin.somethingElse"));
        let (code, _body) = get(
            &base,
            "/xrpc/app.stellin.getProfileView?actor=did:plc:subject",
            Some(&bad),
        )
        .await;
        assert_eq!(code, 401);
    }

    // FALSIFY guard: the gated field must not leak through the generic getRecord
    // route for ANY viewer (it is a computed projection, not a stored field).
    #[tokio::test]
    async fn getrecord_does_not_bypass_the_gate() {
        let (w, st) = world();
        let base = serve(st).await;
        // Even the recruiter who WOULD see openToWork via the view route…
        let bearer = w.recruiter_other.mint(AUD, NOW + 60, Some("com.atproto.repo.getRecord"));
        let (_code, body) = get(
            &base,
            "/xrpc/com.atproto.repo.getRecord?collection=app.bsky.actor.profile&rkey=did:plc:subject",
            Some(&bearer),
        )
        .await;
        assert!(
            body.to_string().find("openToWork").is_none(),
            "openToWork must never appear via the generic getRecord route: {body}"
        );
    }
}
