//! The composable frame: the top-level pinned strip above the column of
//! contributed panels, both driven by the persisted layout document. The frame
//! does not know about the feed — it iterates the layout's panels and dispatches
//! each through the panel registry. Pinning/unpinning updates the layout signal
//! and persists it, so the workspace survives a restart.

use crate::layout_store;
use crate::panel::render_panel;
use crate::pinned::PinnedStripView;
use design::tokens::*;
use leptos::prelude::*;
use shell::{Pin, PinType, PondId, SlotId};

#[component]
pub fn Frame() -> impl IntoView {
    let layout = RwSignal::new(layout_store::load());

    let on_pin = Callback::new(move |address: String| {
        layout.update(|l| l.add_pin(Pin::new(PondId::Bluesky, address, PinType::Post)));
        layout_store::save(&layout.get_untracked());
    });
    let on_unpin = Callback::new(move |address: String| {
        layout.update(|l| l.remove_pin(&address));
        layout_store::save(&layout.get_untracked());
    });

    let frame_style = format!(
        "display:flex;flex-direction:column;height:100vh;background:{COLOR_SURFACE};"
    );
    let column_slot = "flex:1 1 auto;min-height:0;".to_string();

    view! {
        <div style=frame_style>
            <PinnedStripView layout=layout on_unpin=on_unpin />
            <div style=column_slot>
                {move || {
                    let l = layout.get();
                    l.panels_in(SlotId::Column)
                        .map(|p| render_panel(&p.kind, on_pin))
                        .collect_view()
                }}
            </div>
        </div>
    }
}
