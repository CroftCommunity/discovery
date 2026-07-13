//! The runtime loop: the architecture in miniature. Apply an intent via the
//! pure `update`, perform each returned effect, turn each result into a
//! follow-up intent, and repeat until no effects remain.

use crate::effects::perform_effect;
use crate::executor::block_on;
use app_core::{update, Intent, Model};
use bluesky::port::BlueskyPort;
use std::collections::VecDeque;

/// Drive the core from a starting model and an initial intent until quiescent
/// (no more effects to perform). Returns the settled model.
pub fn drive<P: BlueskyPort>(mut model: Model, initial: Intent, port: &P) -> Model {
    let mut pending: VecDeque<Intent> = VecDeque::new();
    pending.push_back(initial);

    while let Some(intent) = pending.pop_front() {
        let (next, effects) = update(model, intent);
        model = next;
        for effect in &effects {
            // The shell performs the effect and feeds the result back as a new
            // intent. The core stays out of this entirely.
            let follow_up = block_on(perform_effect(effect, port));
            pending.push_back(follow_up);
        }
    }
    model
}
