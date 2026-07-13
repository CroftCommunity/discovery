//! A deterministic gallery of every required view state, rendered from fixed
//! sample data (no network). This is what the per-state snapshot harness
//! captures: every state has a section with a stable `data-snapshot` id, so an
//! undesigned/missing state is a failing snapshot test (spec 1.5).

use design::tokens::*;
use design::*;
use leptos::prelude::*;

/// A no-op retry handler — the gallery never performs effects.
fn noop() -> Callback<()> {
    Callback::new(|_: ()| {})
}

const AVATAR: &str =
    "https://cdn.bsky.app/img/avatar/plain/did:plc:z72i7hdynmk6r22z27h6tvur/bafkreihwihm6kpd6zuwhhlro75p5qks5qtrcu55jp3gddbfjsieiv7wuka@jpeg";

fn sample_card() -> impl IntoView {
    view! {
        <PostCard
            author_display_name="Bluesky"
            handle="bsky.app"
            body="v1.125 is live — you can now reply to specific messages in group chats and DMs."
            timestamp="2026-06-17T21:29:11.000Z"
            avatar=Some(AVATAR.to_string())
        />
    }
}

fn sample_card_2() -> impl IntoView {
    view! {
        <PostCard
            author_display_name="Bluesky"
            handle="bsky.app"
            body="This was one of the top requests we heard after launching Group Chats."
            timestamp="2026-06-17T21:01:15.786Z"
            avatar=Some(AVATAR.to_string())
        />
    }
}

/// One labeled, id'd section wrapping a state at the reading column width.
#[component]
fn Section(id: &'static str, label: &'static str, children: Children) -> impl IntoView {
    let wrap = format!("padding:{SPACE_5} {SPACE_4};border-bottom:{HAIRLINE} solid {COLOR_BORDER};");
    let lbl = format!(
        "margin:0 0 {SPACE_3};font-family:{FONT_MONO};font-size:{TEXT_CAPTION_SIZE};\
         color:{COLOR_TEXT_FAINT};"
    );
    let col = format!(
        "max-width:{MEASURE_COLUMN};margin:0 auto;display:flex;flex-direction:column;gap:{SPACE_4};"
    );
    view! {
        <section data-snapshot=id style=wrap>
            <p style=lbl>{label}</p>
            <div style=col>{children()}</div>
        </section>
    }
}

/// The full gallery, in the same order as the spec's required snapshot set.
#[component]
pub fn Gallery() -> impl IntoView {
    let bg = format!("background:{COLOR_SURFACE};min-height:100vh;");
    view! {
        <div style=bg>
            // --- feed states ---
            <Section id="feed-loading" label="feed · loading (cold)">
                <LoadingView />
            </Section>
            <Section id="feed-more" label="feed · loaded, more available">
                {sample_card()}
                {sample_card_2()}
                <FooterMore />
            </Section>
            <Section id="feed-end" label="feed · loaded, end reached">
                {sample_card()}
                <FooterEnd />
            </Section>
            <Section id="feed-loading-more" label="feed · loading more">
                {sample_card()}
                <FooterLoadingMore />
            </Section>
            <Section id="feed-empty" label="feed · empty">
                <EmptyView message="Nothing here yet." />
            </Section>
            <Section id="feed-error-cold" label="feed · error (cold)">
                <ErrorView reason="couldn't reach the feed" on_retry=noop() />
            </Section>
            <Section id="feed-error-appended" label="feed · error while appended">
                {sample_card()}
                <FooterInlineError reason="couldn't reach the feed" on_retry=noop() />
            </Section>

            // --- card states ---
            <Section id="card-standard" label="card · standard">
                {sample_card()}
            </Section>
            <Section id="card-no-avatar" label="card · missing avatar">
                <PostCard
                    author_display_name="Ada Lovelace"
                    handle="ada.example.com"
                    body="No avatar set — a designed placeholder, never a broken image."
                    timestamp="2026-06-17T18:08:55.638Z"
                    avatar=None
                />
            </Section>
            <Section id="card-long-text" label="card · very long text">
                <PostCard
                    author_display_name="Bluesky"
                    handle="bsky.app"
                    body="A very long post. Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident."
                    timestamp="2026-06-17T16:10:58.799Z"
                    avatar=Some(AVATAR.to_string())
                />
            </Section>
            <Section id="card-long-name" label="card · long name + handle (truncation)">
                <PostCard
                    author_display_name="A Remarkably Long Display Name That Should Truncate Cleanly"
                    handle="an-extremely-long-handle-that-keeps-going.bsky.social"
                    body="The author line truncates; the timestamp stays pinned at the end."
                    timestamp="2026-06-17T16:26:54.600Z"
                    avatar=Some(AVATAR.to_string())
                />
            </Section>

            // --- Phase 2 shell surfaces ---
            <Section id="shell-pinned-empty" label="pinned strip · empty">
                <EmptyPinnedStrip />
            </Section>
            <Section id="shell-pinned-items" label="pinned strip · with pins">
                <PinnedStrip>
                    <PinnedItem
                        author="Bluesky"
                        snippet="v1.125 is live — reply to specific messages in group chats and DMs."
                        on_unpin=noop()
                    />
                    <PinnedItem
                        author="Ada Lovelace"
                        snippet="A pinned thought worth keeping at the top."
                        on_unpin=noop()
                    />
                </PinnedStrip>
            </Section>
            <Section id="shell-pinned-degraded" label="pinned strip · degraded (gone target)">
                <PinnedStrip>
                    <PinnedItem author="Bluesky" snippet="Still here." on_unpin=noop() />
                    <DegradedPinItem on_unpin=noop() />
                </PinnedStrip>
            </Section>
            <Section id="shell-panel-in-slot" label="column slot · a contributed feed panel">
                {sample_card()}
                {sample_card_2()}
                <FooterMore />
            </Section>
        </div>
    }
}
