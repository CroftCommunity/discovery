//! Activity builders — Follow / Undo Follow / Delete(Actor).
//!
//! Byte-fidelity to Mastodon's serializer emit (see specimens under
//! `tests/specimens/mastodon-*-observed-shape.md`). Key-order is the
//! serializer's insertion order; single-line compact JSON; no BOM,
//! no trailing newline.

use crate::actor::ShimActor;

/// Build a Follow activity body (`FED-SHIM.md §1 row 3`, specimen
/// `mastodon-follow-observed-shape.md`).
///
/// Key-order: `@context, id, type, actor, object`.
pub fn build_follow(actor: &ShimActor, object_url: &str, activity_id: &str) -> Vec<u8> {
    format!(
        r#"{{"@context":"https://www.w3.org/ns/activitystreams","id":"{id}","type":"Follow","actor":"{actor}","object":"{obj}"}}"#,
        id = activity_id,
        actor = actor.actor_url,
        obj = object_url,
    )
    .into_bytes()
}

/// Build an Undo Follow activity body (`FED-SHIM.md §1 row 4`, specimen
/// `mastodon-undo-follow-observed-shape.md`).
///
/// Outer key-order: `@context, id, type, actor, object` (object here is
/// the nested Follow activity). Nested Follow: `id, type, actor, object`
/// (no `@context` on the nested — outer covers).
pub fn build_undo_follow(
    actor: &ShimActor,
    follow_id: &str,
    follow_object: &str,
    activity_id: &str,
) -> Vec<u8> {
    format!(
        r#"{{"@context":"https://www.w3.org/ns/activitystreams","id":"{aid}","type":"Undo","actor":"{actor}","object":{{"id":"{fid}","type":"Follow","actor":"{actor}","object":"{fobj}"}}}}"#,
        aid = activity_id,
        actor = actor.actor_url,
        fid = follow_id,
        fobj = follow_object,
    )
    .into_bytes()
}

/// Build a Delete(Actor) activity body (`FED-SHIM.md §1 row 5`, specimen
/// `mastodon-delete-actor-observed-shape.md`).
///
/// Key-order: `@context, id, type, actor, object, to`. `actor == object`
/// (account-delete semantic). `to = ["https://www.w3.org/ns/activitystreams#Public"]`
/// so the delete fans out to every follower's inbox.
pub fn build_delete_actor(actor: &ShimActor, activity_id: &str) -> Vec<u8> {
    format!(
        r#"{{"@context":"https://www.w3.org/ns/activitystreams","id":"{aid}","type":"Delete","actor":"{actor}","object":"{actor}","to":["https://www.w3.org/ns/activitystreams#Public"]}}"#,
        aid = activity_id,
        actor = actor.actor_url,
    )
    .into_bytes()
}
