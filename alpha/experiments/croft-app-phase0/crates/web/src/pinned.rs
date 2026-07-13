//! The top-level pinned strip. Each pin is a reference; the strip hydrates it
//! live through the Bluesky module's hooks and degrades gracefully when the
//! target is gone. The shell owns the pin list (in the layout document); the
//! module owns "given this address, hydrate it".

use crate::effects::hydrate_pin;
use design::*;
use leptos::prelude::*;
use leptos::task::spawn_local;
use shell::Layout;

/// Render the strip from the layout's pins, or a quiet empty state.
#[component]
pub fn PinnedStripView(layout: RwSignal<Layout>, on_unpin: Callback<String>) -> impl IntoView {
    move || {
        let pins = layout.get().pins;
        if pins.is_empty() {
            view! { <EmptyPinnedStrip /> }.into_any()
        } else {
            let items = pins
                .into_iter()
                .map(|pin| {
                    let addr = pin.address.clone();
                    let unpin = Callback::new(move |_| on_unpin.run(addr.clone()));
                    view! { <PinView address=pin.address on_unpin=unpin /> }
                })
                .collect_view();
            view! { <PinnedStrip>{items}</PinnedStrip> }.into_any()
        }
    }
}

#[derive(Clone)]
enum PinState {
    Loading,
    Ready(String, String),
    Gone,
}

/// One pin: hydrates on mount, then renders live or degraded.
#[component]
fn PinView(#[prop(into)] address: String, on_unpin: Callback<()>) -> impl IntoView {
    let state = RwSignal::new(PinState::Loading);
    {
        let address = address.clone();
        Effect::new(move |_| {
            let address = address.clone();
            spawn_local(async move {
                match hydrate_pin(&address).await {
                    Some((author, snippet)) => state.set(PinState::Ready(author, snippet)),
                    None => state.set(PinState::Gone),
                }
            });
        });
    }

    move || match state.get() {
        PinState::Loading => {
            view! { <PinnedItem author="…" snippet="loading…" on_unpin=on_unpin /> }.into_any()
        }
        PinState::Ready(author, snippet) => {
            view! { <PinnedItem author=author snippet=snippet on_unpin=on_unpin /> }.into_any()
        }
        PinState::Gone => view! { <DegradedPinItem on_unpin=on_unpin /> }.into_any(),
    }
}
