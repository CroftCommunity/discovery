//! The DOM runtime loop — the same shape as the CLI's, in a browser, now scoped
//! to one feed (one actor) so several panels can each drive their own core.
//! Hold the Model in a signal; apply an intent via `update`; set the signal;
//! perform each effect off-thread (spawn_local) and feed results back as
//! intents.

use crate::effects;
use app_core::{update, Effect, Intent, Model};
use leptos::prelude::*;
use leptos::task::spawn_local;

/// A cheap-to-clone handle to one running feed.
#[derive(Clone)]
pub struct Runtime {
    model: RwSignal<Model>,
    actor: String,
}

impl Runtime {
    pub fn new(actor: impl Into<String>) -> Self {
        Runtime {
            model: RwSignal::new(Model::new()),
            actor: actor.into(),
        }
    }

    pub fn model(&self) -> RwSignal<Model> {
        self.model
    }

    /// Apply an intent, then perform any effects it produced.
    pub fn dispatch(&self, intent: Intent) {
        let (next, effects) = update(self.model.get_untracked(), intent);
        self.model.set(next);

        for effect in effects {
            match effect {
                Effect::FetchFeed { cursor } => {
                    let actor = self.actor.clone();
                    let this = self.clone();
                    spawn_local(async move {
                        let follow_up = effects::fetch_feed(&actor, cursor).await;
                        this.dispatch(follow_up);
                    });
                }
            }
        }
    }
}
