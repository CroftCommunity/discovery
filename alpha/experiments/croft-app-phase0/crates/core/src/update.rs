//! The update function: `(Model, Intent) -> (Model, Vec<Effect>)`.
//!
//! Pure and synchronous. No async, no I/O, no clock. This signature is
//! load-bearing for DECISION 1: an awaited port call could not return from it,
//! which is exactly why the port lives in the shell, not here.

use crate::effect::Effect;
use crate::intent::Intent;
use crate::model::{FeedStatus, Model};

pub fn update(model: Model, intent: Intent) -> (Model, Vec<Effect>) {
    let Model { posts, status } = model;

    let result = match intent {
        // --- Group A: opening ---
        Intent::OpenFeed => match status {
            // A1: cold open kicks off the first fetch.
            FeedStatus::Idle => (
                Model {
                    posts,
                    status: FeedStatus::LoadingCold,
                },
                vec![Effect::FetchFeed { cursor: None }],
            ),
            // A2/A3 and any other state: no wasteful re-fetch, no duplicate load.
            other => no_change(posts, other),
        },

        // --- Group C (load-more trigger) ---
        Intent::FeedReachedEnd => match status {
            // C1: more available -> load from the carried cursor.
            FeedStatus::Loaded {
                cursor: Some(cursor),
            } => (
                Model {
                    posts,
                    status: FeedStatus::LoadingMore {
                        cursor: cursor.clone(),
                    },
                },
                vec![Effect::FetchFeed {
                    cursor: Some(cursor),
                }],
            ),
            // C3 (true end), C4 (already loading more), and others: do nothing.
            other => no_change(posts, other),
        },

        // --- Groups B & C: a page arrived ---
        Intent::FeedPageLoaded { posts: new, next_cursor } => match status {
            // B1/B2: cold load resolves; the page becomes the feed.
            FeedStatus::LoadingCold => (
                Model {
                    posts: new,
                    status: FeedStatus::Loaded { cursor: next_cursor },
                },
                vec![],
            ),
            // C2: append after existing, never replace. The cursor we were
            // loading *from* is spent and dropped; the new one is the intent's
            // (BUILD-SPEC §3a, the one place a cursor is discarded).
            FeedStatus::LoadingMore { .. } => {
                let mut all = posts;
                all.extend(new);
                (
                    Model {
                        posts: all,
                        status: FeedStatus::Loaded { cursor: next_cursor },
                    },
                    vec![],
                )
            }
            // A page arriving in any other state is unexpected; ignore it
            // rather than corrupt state.
            other => no_change(posts, other),
        },

        // --- Groups B & C: a fetch failed ---
        Intent::FeedLoadFailed { reason } => match status {
            // B3: cold load failed, no posts. No auto-retry.
            FeedStatus::LoadingCold => (
                Model {
                    posts,
                    status: FeedStatus::ErrorCold { reason },
                },
                vec![],
            ),
            // C5: load-more failed. Keep existing posts and preserve the cursor
            // in the variant so retry can always resume (DECISION 4).
            FeedStatus::LoadingMore { cursor } => (
                Model {
                    posts,
                    status: FeedStatus::ErrorWhileAppended { reason, cursor },
                },
                vec![],
            ),
            other => no_change(posts, other),
        },

        // --- Group D: retry ---
        Intent::RetryRequested => match status {
            // D1: retry a cold failure from None.
            FeedStatus::ErrorCold { .. } => (
                Model {
                    posts,
                    status: FeedStatus::LoadingCold,
                },
                vec![Effect::FetchFeed { cursor: None }],
            ),
            // D2: retry an append failure from the preserved cursor. Cannot fail
            // for lack of a cursor: the variant guarantees it (DECISION 4).
            FeedStatus::ErrorWhileAppended { cursor, .. } => (
                Model {
                    posts,
                    status: FeedStatus::LoadingMore {
                        cursor: cursor.clone(),
                    },
                },
                vec![Effect::FetchFeed {
                    cursor: Some(cursor),
                }],
            ),
            other => no_change(posts, other),
        },
    };

    // 0c (Phase 1): mechanically catch a future transition that violates the
    // cursor invariant. The type system already guarantees the cursor-mandatory
    // variants *carry* a cursor; this also asserts it is a real (non-empty)
    // token, so a transition that ever constructed one of these states with a
    // junk cursor — the way a careless future edit might — trips loudly in
    // debug. (The C2 cursor discard is intentional: the new cursor comes from
    // the intent, see §3a.) Debug-only, so the pure release path is untouched.
    debug_assert_cursor_invariant(&result.0);

    result
}

/// Assert the cursor invariant on a freshly produced model (debug builds only).
fn debug_assert_cursor_invariant(model: &Model) {
    match &model.status {
        FeedStatus::LoadingMore { cursor }
        | FeedStatus::ErrorWhileAppended { cursor, .. }
        | FeedStatus::Loaded {
            cursor: Some(cursor),
        } => {
            debug_assert!(
                !cursor.0.is_empty(),
                "cursor invariant (BUILD-SPEC §3a / 0c): {:?} must carry a non-empty cursor",
                model.status,
            );
        }
        FeedStatus::Idle | FeedStatus::LoadingCold | FeedStatus::ErrorCold { .. } | FeedStatus::Loaded { cursor: None } => {}
    }
}

/// Reassemble the model unchanged with no effects.
fn no_change(posts: Vec<bluesky::types::Post>, status: FeedStatus) -> (Model, Vec<Effect>) {
    (Model { posts, status }, vec![])
}
