//! Panels contributed into slots. `render_panel` is the contribution registry:
//! the frame iterates the layout's panels and dispatches each by kind, never
//! hard-coding the feed — adding a panel kind means one more arm here plus a
//! component, with no change to the frame, slots, or layout machinery.

use crate::runtime::Runtime;
use app_core::{project, Avatar, FeedView, Footer, Intent};
use design::tokens::*;
use design::*;
use leptos::prelude::*;
use shell::PanelKind;
use wasm_bindgen::JsCast;

/// Render one layout panel. `on_pin` receives an item's address when pinned.
pub fn render_panel(kind: &PanelKind, on_pin: Callback<String>) -> AnyView {
    match kind {
        PanelKind::Feed { actor } => {
            view! { <FeedPanel actor=actor.clone() on_pin=on_pin /> }.into_any()
        }
    }
}

#[component]
fn FeedPanel(#[prop(into)] actor: String, on_pin: Callback<String>) -> impl IntoView {
    let rt = Runtime::new(actor);
    let model = rt.model();

    // Cold-load once after mount.
    {
        let rt = rt.clone();
        Effect::new(move |_| rt.dispatch(Intent::OpenFeed));
    }

    // Scroll near the bottom -> ask for more (a no-op unless Loaded{Some}).
    let on_scroll = {
        let rt = rt.clone();
        move |ev: leptos::ev::Event| {
            if let Some(el) = ev.target().and_then(|t| t.dyn_into::<web_sys::Element>().ok()) {
                if (el.scroll_height() - (el.scroll_top() + el.client_height())) < 600 {
                    rt.dispatch(Intent::FeedReachedEnd);
                }
            }
        }
    };

    let rt_render = rt.clone();
    let scroller = "height:100%;overflow-y:auto;".to_string();
    let column = format!(
        "max-width:{MEASURE_COLUMN};margin:0 auto;padding:{SPACE_6} {SPACE_4};\
         display:flex;flex-direction:column;gap:{SPACE_4};"
    );

    view! {
        <div style=scroller on:scroll=on_scroll>
            <div style=column>
                {move || render_feed(project(&model.get()), rt_render.clone(), on_pin)}
            </div>
        </div>
    }
}

/// Map a `FeedView` onto design primitives, attaching a pin affordance to each
/// card. The single seam between core view models and the design system.
fn render_feed(view: FeedView, rt: Runtime, on_pin: Callback<String>) -> AnyView {
    match view {
        FeedView::Loading => view! { <LoadingView /> }.into_any(),
        FeedView::Empty { message } => view! { <EmptyView message=message /> }.into_any(),
        FeedView::Error { reason, .. } => {
            let retry = retry_cb(rt);
            view! { <ErrorView reason=reason on_retry=retry /> }.into_any()
        }
        FeedView::Feed { posts, footer } => {
            let cards = posts
                .into_iter()
                .map(|c| {
                    let avatar = match c.avatar {
                        Avatar::Url(url) => Some(url),
                        Avatar::NoAvatar => None,
                    };
                    let id = c.id.clone();
                    let pin = Callback::new(move |_| on_pin.run(id.clone()));
                    let wrap = "position:relative;".to_string();
                    let pin_pos = format!("position:absolute;top:{SPACE_3};right:{SPACE_3};");
                    view! {
                        <div style=wrap>
                            <PostCard
                                author_display_name=c.author_display_name
                                handle=c.handle
                                body=c.body
                                timestamp=c.timestamp
                                avatar=avatar
                            />
                            <div style=pin_pos>
                                <PinButton on_press=pin />
                            </div>
                        </div>
                    }
                })
                .collect_view();
            let foot = render_footer(footer, rt);
            view! {
                {cards}
                {foot}
            }
            .into_any()
        }
    }
}

fn render_footer(footer: Footer, rt: Runtime) -> AnyView {
    match footer {
        Footer::MoreAvailable => view! { <FooterMore /> }.into_any(),
        Footer::LoadingMore => view! { <FooterLoadingMore /> }.into_any(),
        Footer::EndReached => view! { <FooterEnd /> }.into_any(),
        Footer::InlineError { reason, .. } => {
            let retry = retry_cb(rt);
            view! { <FooterInlineError reason=reason on_retry=retry /> }.into_any()
        }
    }
}

fn retry_cb(rt: Runtime) -> Callback<()> {
    Callback::new(move |_| rt.dispatch(Intent::RetryRequested))
}
