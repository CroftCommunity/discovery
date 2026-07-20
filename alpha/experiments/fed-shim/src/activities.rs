//! Activity builders — Follow / Undo Follow / Delete(Actor).
//!
//! Byte-fidelity to Mastodon's serializer emit (see specimens under
//! `tests/specimens/mastodon-*-observed-shape.md`). Key-order is the
//! serializer's insertion order; single-line compact JSON; no BOM,
//! no trailing newline.

use crate::actor::ShimActor;

/// Build a Follow activity body (`FED-SHIM.md §1 row 3`, specimen
/// `mastodon-follow-observed-shape.md`).
pub fn build_follow(_actor: &ShimActor, _object_url: &str, _activity_id: &str) -> Vec<u8> {
    unimplemented!("fed-shim GREEN: Follow JSON body, key-order @context,id,type,actor,object")
}

/// Build an Undo Follow activity body (`FED-SHIM.md §1 row 4`, specimen
/// `mastodon-undo-follow-observed-shape.md`).
pub fn build_undo_follow(
    _actor: &ShimActor,
    _follow_id: &str,
    _follow_object: &str,
    _activity_id: &str,
) -> Vec<u8> {
    unimplemented!("fed-shim GREEN: Undo Follow JSON body, nested Follow with matching id")
}

/// Build a Delete(Actor) activity body (`FED-SHIM.md §1 row 5`, specimen
/// `mastodon-delete-actor-observed-shape.md`).
pub fn build_delete_actor(_actor: &ShimActor, _activity_id: &str) -> Vec<u8> {
    unimplemented!("fed-shim GREEN: Delete(Actor) JSON body, actor==object, to=[as:Public]")
}
